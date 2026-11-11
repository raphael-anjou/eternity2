//! A4: MAP-Elites over basin descriptors versus parallel tempering.
//!
//! Subcommands:
//!   run     one (puzzle, seed, method) run; writes a JSON record with the
//!           best score, measured compute and the near-best boards it visited.
//!   basins  pools the run records of each puzzle, fixes best_known per puzzle,
//!           and counts distinct basins per run for every (k, delta) pair.
//!
//! Both methods share the generator, the scorer, the initial fill and the move
//! kernel (`run_sa_steps_fixed_temp`, bursts of `burst` steps at a temperature
//! from the same geometric ladder). They differ only in which board the next
//! burst starts from (PT: the replica at that temperature after exchanges;
//! MAP-Elites: a uniformly drawn elite of a descriptor grid) and in what is
//! kept (PT: 8 replicas; MAP-Elites: one elite per descriptor cell).

#![forbid(unsafe_code)]

use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use eternity2_core::{Board, Edges, Piece, Puzzle, Rotation};
use eternity2_localsearch::{run_sa_steps_fixed_temp, RngHandle, StateRef};
use serde_json::{json, Value};

pub const N_REPLICAS: usize = 8;
pub const T_MIN: f64 = 0.05;
pub const T_MAX: f64 = 2.0;
/// Candidates kept per run: score >= run_best - KEEP_BELOW_BEST (largest k analysed).
pub const KEEP_BELOW_BEST: u32 = 4;
pub const K_VALUES: [u32; 4] = [0, 1, 2, 4];
pub const DELTA_VALUES: [f64; 3] = [0.10, 0.25, 0.50];
pub const D1_BINS: usize = 4;
pub const D2_BINS: usize = 6;
pub const D3_BINS: usize = 4;

// ---------------------------------------------------------------- puzzle

pub fn make_puzzle(size: u8, colors: u8, seed: u32) -> Puzzle {
    let p = eternity2_engine::generate(size, colors, seed);
    let pieces: Vec<Piece> = p
        .pieces
        .iter()
        .enumerate()
        .map(|(i, e)| Piece::new(i as u16, Edges::new(e[0], e[1], e[2], e[3])))
        .collect();
    Puzzle::new(u32::from(size), u32::from(size), u32::from(p.num_colors) + 1, pieces)
        .expect("generated puzzle is valid")
}

pub fn ladder() -> Vec<f64> {
    let ratio = (T_MAX / T_MIN).powf(1.0 / (N_REPLICAS as f64 - 1.0));
    (0..N_REPLICAS).map(|i| T_MIN * ratio.powi(i as i32)).collect()
}

/// Same per-replica seeding as `run_pt_from`.
pub fn replica_rng(seed: u64, r: usize) -> RngHandle {
    RngHandle::new(seed.wrapping_add((r as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)))
}

pub fn encode(b: &Board) -> Vec<u16> {
    b.cells()
        .iter()
        .map(|c| {
            let (p, r) = c.expect("complete board");
            p * 4 + u16::from(r.as_u8())
        })
        .collect()
}

fn edges_at(puzzle: &Puzzle, code: u16) -> [u8; 4] {
    let pid = code / 4;
    let rot = Rotation::from_u8((code % 4) as u8).unwrap();
    puzzle.piece(pid).unwrap().edges.rotated(rot).as_array()
}

/// Matched interior edges, same convention as the local-search scorer
/// (equal colours and not the grey border).
pub fn score_cells(puzzle: &Puzzle, cells: &[u16]) -> u32 {
    let n = puzzle.width as usize;
    let mut m = 0;
    for y in 0..n {
        for x in 0..n {
            let e = edges_at(puzzle, cells[y * n + x]);
            if x + 1 < n {
                let r = edges_at(puzzle, cells[y * n + x + 1]);
                if e[1] == r[3] && e[1] != 0 {
                    m += 1;
                }
            }
            if y + 1 < n {
                let b = edges_at(puzzle, cells[(y + 1) * n + x]);
                if e[2] == b[0] && e[2] != 0 {
                    m += 1;
                }
            }
        }
    }
    m
}

// ---------------------------------------------------------------- descriptors

/// (d1, d2, d3) bins; all three are invariant under the four board rotations.
/// d1: share of mismatched seams with an endpoint on the outer ring, 4 bins.
/// d2: cyclic order of the four corner pieces modulo rotation, 6 classes.
/// d3: distinct colours on mismatched seams / colour count, 4 bins.
pub fn descriptor(puzzle: &Puzzle, cells: &[u16]) -> (usize, usize, usize) {
    let n = puzzle.width as usize;
    let colors = (puzzle.color_count - 1) as usize;
    let on_ring = |x: usize, y: usize| x == 0 || y == 0 || x == n - 1 || y == n - 1;
    let mut mism = 0usize;
    let mut mism_ring = 0usize;
    let mut seen = vec![false; colors + 1];
    let mut visit = |a: u8, b: u8, ring: bool, mism: &mut usize, mism_ring: &mut usize| {
        if !(a == b && a != 0) {
            *mism += 1;
            if ring {
                *mism_ring += 1;
            }
            seen[a as usize] = true;
            seen[b as usize] = true;
        }
    };
    for y in 0..n {
        for x in 0..n {
            let e = edges_at(puzzle, cells[y * n + x]);
            if x + 1 < n {
                let r = edges_at(puzzle, cells[y * n + x + 1]);
                visit(e[1], r[3], on_ring(x, y) || on_ring(x + 1, y), &mut mism, &mut mism_ring);
            }
            if y + 1 < n {
                let b = edges_at(puzzle, cells[(y + 1) * n + x]);
                visit(e[2], b[0], on_ring(x, y) || on_ring(x, y + 1), &mut mism, &mut mism_ring);
            }
        }
    }
    let quart = |f: f64, bins: usize| ((f * bins as f64).floor() as usize).min(bins - 1);
    let d1 = if mism == 0 { 0 } else { quart(mism_ring as f64 / mism as f64, D1_BINS) };
    let n_col = seen.iter().skip(1).filter(|s| **s).count();
    let d3 = if mism == 0 { 0 } else { quart(n_col as f64 / colors as f64, D3_BINS) };
    // Corners clockwise from top-left; rotate so the smallest piece id leads.
    let c = [cells[0] / 4, cells[n - 1] / 4, cells[n * n - 1] / 4, cells[n * (n - 1)] / 4];
    let lead = (0..4).min_by_key(|&i| c[i]).unwrap();
    let rest = [c[(lead + 1) % 4], c[(lead + 2) % 4], c[(lead + 3) % 4]];
    let mut sorted = rest;
    sorted.sort_unstable();
    let rank = |v: u16| sorted.iter().position(|&s| s == v).unwrap();
    let perm = [rank(rest[0]), rank(rest[1]), rank(rest[2])];
    let d2 = perm[0] * 2 + usize::from(perm[1] > perm[2]);
    (d1, d2, d3)
}

pub fn cell_index(d: (usize, usize, usize)) -> usize {
    (d.0 * D2_BINS + d.1) * D3_BINS + d.2
}

// ---------------------------------------------------------------- distance

/// Rotate a board 90 degrees clockwise: cell (x, y) moves to (n-1-y, x) and
/// each piece turns one quarter clockwise.
pub fn rotate_cw(cells: &[u16], n: usize) -> Vec<u16> {
    let mut out = vec![0u16; n * n];
    for y in 0..n {
        for x in 0..n {
            let c = cells[y * n + x];
            let (nx, ny) = (n - 1 - y, x);
            out[ny * n + nx] = (c / 4) * 4 + (c % 4 + 1) % 4;
        }
    }
    out
}

/// Fraction of cells holding a different (piece, rotation), minimised over the
/// four board rotations of `b`.
pub fn distance(a: &[u16], b_rots: &[Vec<u16>; 4]) -> f64 {
    let n2 = a.len();
    let mut best = n2;
    for br in b_rots {
        let d = a.iter().zip(br.iter()).filter(|(x, y)| x != y).count();
        best = best.min(d);
    }
    best as f64 / n2 as f64
}

pub fn all_rotations(cells: &[u16], n: usize) -> [Vec<u16>; 4] {
    let r1 = rotate_cw(cells, n);
    let r2 = rotate_cw(&r1, n);
    let r3 = rotate_cw(&r2, n);
    [cells.to_vec(), r1, r2, r3]
}

/// Leader clustering: candidates in (score desc, cells asc) order; a candidate
/// within distance < delta of an existing leader joins it, otherwise it becomes
/// a new leader. Returns the number of leaders.
pub fn count_basins(cands: &[(u32, Vec<u16>)], n: usize, delta: f64) -> usize {
    let mut order: Vec<&(u32, Vec<u16>)> = cands.iter().collect();
    order.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
    let mut leaders: Vec<[Vec<u16>; 4]> = Vec::new();
    for (_, c) in order {
        if !leaders.iter().any(|l| distance(c, l) < delta) {
            leaders.push(all_rotations(c, n));
        }
    }
    leaders.len()
}

// ---------------------------------------------------------------- runs

pub struct RunOut {
    pub best_score: u32,
    pub best_cells: Vec<u16>,
    pub evals: u64,
    pub bursts: u64,
    pub first_solve_eval: Option<u64>,
    pub candidates: Vec<(u32, Vec<u16>)>,
    pub archive_filled: Option<usize>,
}

struct Pool {
    best: u32,
    seen: HashSet<Vec<u16>>,
    cands: Vec<(u32, Vec<u16>)>,
}

impl Pool {
    fn new() -> Self {
        Self { best: 0, seen: HashSet::new(), cands: Vec::new() }
    }
    fn offer(&mut self, score: u32, board: &Board) {
        if score + KEEP_BELOW_BEST < self.best {
            return;
        }
        let cells = encode(board);
        if !self.seen.insert(cells.clone()) {
            return;
        }
        self.cands.push((score, cells));
        if score > self.best {
            self.best = score;
            let floor = self.best.saturating_sub(KEEP_BELOW_BEST);
            self.cands.retain(|(s, _)| *s >= floor);
        }
    }
}

/// The repo's parallel tempering (`run_pt_from`, default path: greedy fill,
/// no Houdayer, repair, kick or forbidden-edge penalty), run sequentially and
/// without the early stop at a perfect board, recording every burst's best.
pub fn run_pt(puzzle: &Puzzle, seed: u64, burst: u64, evals: u64) -> RunOut {
    let state = StateRef::new(puzzle);
    let total = state.total_interior_edges();
    let temps = ladder();
    let n = N_REPLICAS;
    let empty = Board::empty(puzzle);
    let mut boards = Vec::with_capacity(n);
    let mut scores = Vec::with_capacity(n);
    let mut rngs = Vec::with_capacity(n);
    for r in 0..n {
        let mut rng = replica_rng(seed, r);
        let b = state.greedy_fill_initial(&empty, &mut rng);
        scores.push(state.score(&b));
        boards.push(b);
        rngs.push(rng);
    }
    let mut chain_best_scores = scores.clone();
    let mut chain_best_boards = boards.clone();
    let mut gbest = *chain_best_scores.iter().max().unwrap();
    let gi = chain_best_scores.iter().position(|s| *s == gbest).unwrap();
    let mut gboard = chain_best_boards[gi].clone();
    let mut swap_rng = RngHandle::new(seed ^ 0xDEAD_BEEF_CAFE_BABE);
    let mut pool = Pool::new();
    let (mut done, mut bursts, mut rounds) = (0u64, 0u64, 0u64);
    let mut first_solve = None;
    while done + burst * n as u64 <= evals {
        for i in 0..n {
            let mut bb = boards[i].clone();
            let mut bs = scores[i];
            run_sa_steps_fixed_temp(
                &state, &mut boards[i], &mut scores[i], &mut bb, &mut bs, &mut rngs[i], temps[i], burst,
            );
            done += burst;
            bursts += 1;
            pool.offer(bs, &bb);
            if bs == total && first_solve.is_none() {
                first_solve = Some(done);
            }
            if bs > chain_best_scores[i] {
                chain_best_scores[i] = bs;
                chain_best_boards[i] = bb;
            }
        }
        for i in 0..n {
            if chain_best_scores[i] > gbest {
                gbest = chain_best_scores[i];
                gboard = chain_best_boards[i].clone();
            }
        }
        let mut i = (rounds & 1) as usize;
        while i + 1 < n {
            let (blo, bhi) = (1.0 / temps[i], 1.0 / temps[i + 1]);
            let log_p = (f64::from(scores[i]) - f64::from(scores[i + 1])) * (bhi - blo);
            if log_p >= 0.0 || swap_rng.next_f64() < log_p.exp() {
                boards.swap(i, i + 1);
                scores.swap(i, i + 1);
                chain_best_boards.swap(i, i + 1);
                chain_best_scores.swap(i, i + 1);
            }
            i += 2;
        }
        rounds += 1;
    }
    RunOut {
        best_score: gbest,
        best_cells: encode(&gboard),
        evals: done,
        bursts,
        first_solve_eval: first_solve,
        candidates: pool.cands,
        archive_filled: None,
    }
}

/// MAP-Elites: same initial boards, same kernel, same ladder; the next burst
/// starts from a uniformly drawn occupied descriptor cell, at a temperature
/// drawn uniformly from the ladder. Both the burst's best board and its final
/// board are offered to the archive (a cell takes a board if empty or if the
/// board scores strictly higher than the incumbent).
pub fn run_me(puzzle: &Puzzle, seed: u64, burst: u64, evals: u64) -> RunOut {
    let state = StateRef::new(puzzle);
    let total = state.total_interior_edges();
    let temps = ladder();
    let n_cells = D1_BINS * D2_BINS * D3_BINS;
    let mut archive: Vec<Option<(u32, Board)>> = vec![None; n_cells];
    let mut occupied: Vec<usize> = Vec::new();
    let insert = |archive: &mut Vec<Option<(u32, Board)>>, occupied: &mut Vec<usize>, s: u32, b: &Board| {
        let idx = cell_index(descriptor(puzzle, &encode(b)));
        match &archive[idx] {
            None => {
                archive[idx] = Some((s, b.clone()));
                occupied.push(idx);
            }
            Some((inc, _)) if s > *inc => archive[idx] = Some((s, b.clone())),
            _ => {}
        }
    };
    let empty = Board::empty(puzzle);
    let mut gbest = 0u32;
    let mut gboard = empty.clone();
    for r in 0..N_REPLICAS {
        let mut rng = replica_rng(seed, r);
        let b = state.greedy_fill_initial(&empty, &mut rng);
        let s = state.score(&b);
        if s > gbest || r == 0 {
            gbest = s;
            gboard = b.clone();
        }
        insert(&mut archive, &mut occupied, s, &b);
    }
    let mut rng = RngHandle::new(seed ^ 0x0A4E_11E5_0A4E_11E5);
    let mut pool = Pool::new();
    let (mut done, mut bursts) = (0u64, 0u64);
    let mut first_solve = None;
    while done + burst <= evals {
        let pick = occupied[(rng.next_u64() % occupied.len() as u64) as usize];
        let t = temps[(rng.next_u64() % temps.len() as u64) as usize];
        let (mut s, mut b) = archive[pick].clone().unwrap();
        let mut bb = b.clone();
        let mut bs = s;
        run_sa_steps_fixed_temp(&state, &mut b, &mut s, &mut bb, &mut bs, &mut rng, t, burst);
        done += burst;
        bursts += 1;
        pool.offer(bs, &bb);
        if bs == total && first_solve.is_none() {
            first_solve = Some(done);
        }
        if bs > gbest {
            gbest = bs;
            gboard = bb.clone();
        }
        insert(&mut archive, &mut occupied, bs, &bb);
        insert(&mut archive, &mut occupied, s, &b);
    }
    RunOut {
        best_score: gbest,
        best_cells: encode(&gboard),
        evals: done,
        bursts,
        first_solve_eval: first_solve,
        candidates: pool.cands,
        archive_filled: Some(occupied.len()),
    }
}

fn cpu_seconds() -> f64 {
    // utime + stime of this process from /proc/self/stat (Linux), in seconds.
    let Ok(s) = fs::read_to_string("/proc/self/stat") else { return f64::NAN };
    let after = &s[s.rfind(')').unwrap() + 2..];
    let f: Vec<&str> = after.split_whitespace().collect();
    let ticks: f64 = f[11].parse::<f64>().unwrap() + f[12].parse::<f64>().unwrap();
    ticks / 100.0
}

// ---------------------------------------------------------------- CLI

fn arg<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).map(String::as_str)
}

fn req<T: std::str::FromStr>(args: &[String], name: &str) -> T {
    arg(args, name)
        .unwrap_or_else(|| panic!("missing {name}"))
        .parse()
        .unwrap_or_else(|_| panic!("bad {name}"))
}

fn cmd_run(args: &[String]) {
    let size: u8 = req(args, "--size");
    let colors: u8 = req(args, "--colors");
    let pseed: u32 = req(args, "--puzzle-seed");
    let rseed: u64 = req(args, "--run-seed");
    let evals: u64 = req(args, "--evals");
    let burst: u64 = req(args, "--burst");
    let method = arg(args, "--method").expect("--method pt|me");
    let out = PathBuf::from(arg(args, "--out").expect("--out"));
    let puzzle = make_puzzle(size, colors, pseed);
    let wall0 = Instant::now();
    let cpu0 = cpu_seconds();
    let r = match method {
        "pt" => run_pt(&puzzle, rseed, burst, evals),
        "me" => run_me(&puzzle, rseed, burst, evals),
        m => panic!("unknown method {m}"),
    };
    let wall = wall0.elapsed().as_secs_f64();
    let cpu = cpu_seconds() - cpu0;
    assert_eq!(score_cells(&puzzle, &r.best_cells), r.best_score, "scorer disagreement");
    for (s, c) in &r.candidates {
        assert_eq!(score_cells(&puzzle, c), *s, "candidate score disagreement");
    }
    let rec = json!({
        "size": size, "colors": colors, "puzzle_seed": pseed, "run_seed": rseed,
        "method": method, "burst": burst, "evals_budget": evals,
        "evals": r.evals, "bursts": r.bursts,
        "total_edges": 2 * u32::from(size) * (u32::from(size) - 1),
        "best_score": r.best_score, "first_solve_eval": r.first_solve_eval,
        "archive_filled": r.archive_filled,
        "wall_s": wall, "cpu_s": cpu,
        "best_cells": r.best_cells,
        "candidates": r.candidates.iter().map(|(s, c)| json!({"s": s, "c": c})).collect::<Vec<_>>(),
    });
    if let Some(p) = out.parent() {
        fs::create_dir_all(p).unwrap();
    }
    fs::write(&out, serde_json::to_string(&rec).unwrap()).unwrap();
}

fn read_run(p: &Path) -> Value {
    serde_json::from_str(&fs::read_to_string(p).unwrap()).unwrap()
}

fn cmd_basins(args: &[String]) {
    let dir = PathBuf::from(arg(args, "--runs").expect("--runs"));
    let out = PathBuf::from(arg(args, "--out").expect("--out"));
    let mut files: Vec<PathBuf> = fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    files.sort();
    let mut groups: BTreeMap<(u64, u64, u64), Vec<Value>> = BTreeMap::new();
    for f in &files {
        let v = read_run(f);
        let key = (v["size"].as_u64().unwrap(), v["colors"].as_u64().unwrap(), v["puzzle_seed"].as_u64().unwrap());
        groups.entry(key).or_default().push(v);
    }
    let mut lines = Vec::new();
    for ((size, colors, pseed), runs) in &groups {
        let best_known = runs.iter().map(|r| r["best_score"].as_u64().unwrap()).max().unwrap() as u32;
        let mut sorted: Vec<&Value> = runs.iter().collect();
        sorted.sort_by_key(|r| (r["method"].as_str().unwrap().to_string(), r["run_seed"].as_u64().unwrap()));
        for r in sorted {
            let cands: Vec<(u32, Vec<u16>)> = r["candidates"]
                .as_array()
                .unwrap()
                .iter()
                .map(|c| {
                    (
                        c["s"].as_u64().unwrap() as u32,
                        c["c"].as_array().unwrap().iter().map(|x| x.as_u64().unwrap() as u16).collect(),
                    )
                })
                .collect();
            let mut counts = serde_json::Map::new();
            for &k in &K_VALUES {
                let sel: Vec<(u32, Vec<u16>)> =
                    cands.iter().filter(|(s, _)| *s + k >= best_known).cloned().collect();
                for &d in &DELTA_VALUES {
                    let c = count_basins(&sel, *size as usize, d);
                    counts.insert(format!("k{k}_d{d:.2}"), json!(c));
                }
                counts.insert(format!("k{k}_boards"), json!(sel.len()));
            }
            lines.push(serde_json::to_string(&json!({
                "size": size, "colors": colors, "puzzle_seed": pseed,
                "method": r["method"], "run_seed": r["run_seed"],
                "best_known": best_known, "best_score": r["best_score"],
                "total_edges": r["total_edges"], "first_solve_eval": r["first_solve_eval"],
                "evals": r["evals"], "bursts": r["bursts"], "wall_s": r["wall_s"], "cpu_s": r["cpu_s"],
                "archive_filled": r["archive_filled"], "n_candidates": cands.len(),
                "basins": counts,
            })).unwrap());
        }
    }
    fs::write(&out, lines.join("\n") + "\n").unwrap();
    eprintln!("wrote {} rows to {}", lines.len(), out.display());
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("run") => cmd_run(&args[2..]),
        Some("basins") => cmd_basins(&args[2..]),
        _ => {
            eprintln!("usage: qd_basins run --size N --colors C --puzzle-seed P --run-seed R --method pt|me --evals B --burst L --out f.json");
            eprintln!("       qd_basins basins --runs DIR --out basins.jsonl");
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eternity2_localsearch::{run_pt_from, PtConfig};

    #[test]
    fn pt_matches_repo_run_pt_from() {
        for (size, colors, pseed) in [(8u8, 12u8, 7u32), (10, 14, 3)] {
            let puzzle = make_puzzle(size, colors, pseed);
            let rounds = 6u64;
            let burst = 5_000u64;
            let cfg = PtConfig {
                n_replicas: N_REPLICAS, t_min: T_MIN, t_max: T_MAX,
                inner_iters: burst, max_rounds: rounds, seed: 42, ..PtConfig::default()
            };
            let (repo, _) = run_pt_from(&puzzle, &Board::empty(&puzzle), &cfg);
            let ours = run_pt(&puzzle, 42, burst, rounds * burst * N_REPLICAS as u64);
            assert!(repo.best_score < repo.total_edges, "test needs an unsolved run");
            assert_eq!(ours.best_score, repo.best_score);
            assert_eq!(ours.best_cells, encode(&repo.best_board));
            assert_eq!(ours.evals, rounds * burst * N_REPLICAS as u64);
        }
    }

    #[test]
    fn rotation_invariants() {
        let puzzle = make_puzzle(8, 10, 1);
        let state = StateRef::new(&puzzle);
        let mut rng = RngHandle::new(5);
        let b = state.greedy_fill_initial(&Board::empty(&puzzle), &mut rng);
        let c = encode(&b);
        let r = rotate_cw(&c, 8);
        assert_eq!(score_cells(&puzzle, &c), state.score(&b));
        assert_eq!(score_cells(&puzzle, &c), score_cells(&puzzle, &r));
        assert_eq!(descriptor(&puzzle, &c), descriptor(&puzzle, &r));
        assert_eq!(distance(&c, &all_rotations(&r, 8)), 0.0);
        assert_eq!(rotate_cw(&rotate_cw(&rotate_cw(&r, 8), 8), 8), c);
    }

    #[test]
    fn planted_solution_scores_full() {
        // generate_solved puts piece i at cell i, rotation 0.
        let p = eternity2_engine::generate_solved_framed(8, 10, 9, false);
        let pieces: Vec<Piece> = p.pieces.iter().enumerate()
            .map(|(i, e)| Piece::new(i as u16, Edges::new(e[0], e[1], e[2], e[3]))).collect();
        let puzzle = Puzzle::new(8, 8, u32::from(p.num_colors) + 1, pieces).unwrap();
        let cells: Vec<u16> = (0..64u16).map(|i| i * 4).collect();
        assert_eq!(score_cells(&puzzle, &cells), 112);
        assert_eq!(count_basins(&[(112, cells.clone()), (112, rotate_cw(&cells, 8))], 8, 0.1), 1);
    }
}
