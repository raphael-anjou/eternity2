//! Stage-2 A/B ladder for the clue-corridors topic.
//!
//! Three arms differing only in a corridor phase, on seeded instances with
//! faithful clue geometry (the five official clue ratios scaled to N):
//!
//! * `control`  — clues pinned, max-contact greedy fill, restarts;
//! * `path1`    — 1-wide clue-to-clue L-routes laid first, then the same fill;
//! * `ribbon2`  — 2-wide ribbons laid first, then the same fill.
//!
//! All arms share the fill rule, the restart loop and the RNG discipline, so a
//! paired per-seed delta is attributable to the corridor phase alone. The
//! substrate's board size is compile-time, so run once per rung:
//!
//!   cargo run --release --bin ladder --features size-8 -- 8 12 20 8
//!   (args: size, paired seeds, seconds per run, worker threads)
//!
//! Prints one JSON object per run to stdout: per-arm per-seed canonical scores
//! plus one representative viewer URL per arm (seed 1's best board). No
//! wall-clock fields are emitted; the tier is seeded-statistical.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use e2_kit::analysis::canonical_key;
use e2_kit::{
    fit, generator, instance_from_generated, rotated, score_board, Board, Budget, Hint, Instance,
    SolveOutcome, Solver, XorShift,
};
use serde_json::json;

/// The five official clue cells as (x, y) on the 16x16 board; the ratio law
/// scales them as round(x * (N-1) / 15).
const OFFICIAL_CLUE_XY: [(usize, usize); 5] = [(2, 2), (13, 2), (7, 8), (2, 13), (13, 13)];

fn ratio_clue_cells(n: usize) -> Vec<usize> {
    OFFICIAL_CLUE_XY
        .iter()
        .map(|&(x, y)| {
            let sx = ((x * (n - 1)) as f64 / 15.0).round() as usize;
            let sy = ((y * (n - 1)) as f64 / 15.0).round() as usize;
            sy * n + sx
        })
        .collect()
}

/// Pin the solution pieces of a generated instance at the given cells — the
/// same mapping as the kit's `pin_solution_hints`, but at *chosen* cells
/// (the ratio-law clue cells) instead of seed-random ones.
fn pin_cells(mut instance: Instance, size: u8, colors: u8, seed: u32, cells: &[usize]) -> Instance {
    let solved = generator::generate_solved_framed(size, colors, seed, true);
    let mut by_key: std::collections::HashMap<[u8; 4], (u16, [u8; 4])> =
        std::collections::HashMap::new();
    for (pid, piece) in instance.pieces.iter() {
        by_key.insert(canonical_key(piece.edges), (pid, piece.edges));
    }
    let mut hints = Vec::with_capacity(cells.len());
    for &cell in cells {
        let want = solved.pieces[cell];
        let (pid, base) = by_key[&canonical_key(want)];
        let rot = (0..4).find(|&r| rotated(base, r) == want).expect("rotation exists");
        hints.push(Hint { pos: cell as u16, piece: pid, rot });
    }
    instance.hints = hints;
    instance
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Arm {
    Control,
    Path1,
    Ribbon2,
}

impl Arm {
    fn name(self) -> &'static str {
        match self {
            Arm::Control => "control",
            Arm::Path1 => "path1",
            Arm::Ribbon2 => "ribbon2",
        }
    }
}

/// Cells of an L-route from `a` to `b` (endpoints excluded), horizontal leg
/// first when `horiz_first`, in walking order from `a`.
fn l_route(a: (usize, usize), b: (usize, usize), horiz_first: bool, w: usize) -> Vec<usize> {
    let mut cells = Vec::new();
    let (ax, ay) = a;
    let (bx, by) = b;
    let xs = |from: usize, to: usize| -> Vec<usize> {
        if from <= to { (from..=to).collect() } else { (to..=from).rev().collect() }
    };
    if horiz_first {
        for x in xs(ax, bx) {
            cells.push(ay * w + x);
        }
        for y in xs(ay, by) {
            cells.push(y * w + bx);
        }
    } else {
        for y in xs(ay, by) {
            cells.push(y * w + ax);
        }
        for x in xs(ax, bx) {
            cells.push(by * w + x);
        }
    }
    cells.retain(|&c| c != ay * w + ax && c != by * w + bx);
    cells.dedup();
    cells
}

/// Widen a route to a 2-wide ribbon: after each route cell, its perpendicular
/// neighbour (pushed inward, staying on the board), walking order preserved.
fn ribbon_cells(route: &[usize], w: usize, h: usize) -> Vec<usize> {
    let mut out = Vec::with_capacity(route.len() * 2);
    for (i, &c) in route.iter().enumerate() {
        let (x, y) = (c % w, c / w);
        out.push(c);
        // Direction of travel decides the perpendicular.
        let next = route.get(i + 1).or_else(|| route.get(i.wrapping_sub(1))).copied().unwrap_or(c);
        let horizontal_step = next % w != x && next / w == y;
        let side = if horizontal_step {
            // Offset in y, inward.
            let ny = if y + 1 < h - 1 { y + 1 } else { y.saturating_sub(1) };
            ny * w + x
        } else {
            let nx = if x + 1 < w - 1 { x + 1 } else { x.saturating_sub(1) };
            y * w + nx
        };
        if side != c {
            out.push(side);
        }
    }
    let mut seen = std::collections::HashSet::new();
    out.retain(|&c| seen.insert(c));
    out
}

/// Kruskal MST over the clue cells under Manhattan distance: the nearest-pair-
/// first spanning set of clue-to-clue links the corridor arms lay.
fn clue_mst(clues: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let n = clues.len();
    let mut edges: Vec<(usize, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let d = clues[i].0.abs_diff(clues[j].0) + clues[i].1.abs_diff(clues[j].1);
            edges.push((d, i, j));
        }
    }
    edges.sort_unstable();
    let mut parent: Vec<usize> = (0..n).collect();
    fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] != x {
            let r = find(parent, parent[x]);
            parent[x] = r;
        }
        parent[x]
    }
    let mut out = Vec::new();
    for (_, i, j) in edges {
        let (ri, rj) = (find(&mut parent, i), find(&mut parent, j));
        if ri != rj {
            parent[ri] = rj;
            out.push((i, j));
        }
    }
    out
}

struct CorridorSolver {
    arm: Arm,
    seed: u32,
}

impl CorridorSolver {
    /// Best unused (piece, rot) for `pos` under the current constraints.
    /// `hard` demands a perfect fit (returns None on a dead end); soft mode
    /// maximises net matched minus mismatched edges and always places.
    fn best_candidate(
        board: &Board,
        instance: &Instance,
        used: &[bool],
        cand: &[u16],
        pos: usize,
        w: usize,
        h: usize,
        hard: bool,
    ) -> Option<(u16, u8)> {
        let want = fit::edge_constraints(board, &instance.pieces, pos % w, pos / w, w, h);
        let mut best: Option<(i64, u16, u8)> = None;
        for &pid in cand {
            if used[usize::from(pid)] {
                continue;
            }
            for r in 0..4u8 {
                let e = instance.pieces.get(pid).expect("piece").rotated(r);
                let key = if hard {
                    match fit::fit_score(&e, &want) {
                        Some(m) => i64::from(m),
                        None => continue,
                    }
                } else {
                    let (ok, bad) = fit::fit_counts(&e, &want);
                    i64::from(ok) - i64::from(bad)
                };
                if best.is_none_or(|(bk, _, _)| key > bk) {
                    best = Some((key, pid, r));
                }
            }
        }
        best.map(|(_, pid, r)| (pid, r))
    }

    /// Lay one clue-to-clue corridor. Tries the hard (perfect-fit) laying on
    /// one L-bend; on a dead end undoes and re-routes along the other bend; if
    /// both bends dead-end, lays the first bend softly. Ribbon arms widen the
    /// route to 2 cells.
    fn lay_route(
        &self,
        board: &mut Board,
        instance: &Instance,
        used: &mut [bool],
        cand: &[u16],
        a: (usize, usize),
        b: (usize, usize),
        w: usize,
        h: usize,
        rng: &mut XorShift,
    ) {
        let first_horiz = rng.next_below(2) == 0;
        let lay = |board: &mut Board, used: &mut [bool], horiz: bool, hard: bool| -> Option<Vec<usize>> {
            let mut route = l_route(a, b, horiz, w);
            if self.arm == Arm::Ribbon2 {
                route = ribbon_cells(&route, w, h);
            }
            let mut placed = Vec::new();
            for &pos in &route {
                if board.piece_at(pos).is_some() {
                    continue;
                }
                match Self::best_candidate(board, instance, used, cand, pos, w, h, hard) {
                    Some((pid, r)) => {
                        board.place(pos, pid, r);
                        used[usize::from(pid)] = true;
                        placed.push(pos);
                    }
                    None => {
                        for &p in &placed {
                            if let Some((pid, _)) = board.piece_at(p) {
                                used[usize::from(pid)] = false;
                            }
                            board.clear(p);
                        }
                        return None;
                    }
                }
            }
            Some(placed)
        };
        if lay(board, used, first_horiz, true).is_some() {
            return;
        }
        if lay(board, used, !first_horiz, true).is_some() {
            return;
        }
        // Both bends dead-end under perfect fit: commit the first bend softly.
        let _ = lay(board, used, first_horiz, false);
    }
}

impl Solver for CorridorSolver {
    fn name(&self) -> String {
        format!("corridor-{}", self.arm.name())
    }

    fn solve(&mut self, instance: &Instance, start: &Board, budget: Budget) -> SolveOutcome {
        let w = usize::from(instance.width);
        let h = usize::from(instance.height);
        let pieces = &instance.pieces;
        let mut rng = XorShift::new(self.seed.wrapping_mul(0x9E37_79B9) ^ 0xC0FF_EE00);

        let clue_xy: Vec<(usize, usize)> = instance
            .hints
            .iter()
            .map(|hnt| (usize::from(hnt.pos) % w, usize::from(hnt.pos) / w))
            .collect();
        let mst = clue_mst(&clue_xy);

        let mut best = start.clone();
        let mut best_score = score_board(&best, pieces);
        let mut restarts = 0u64;

        while !budget.expired() {
            restarts += 1;
            let mut board = start.clone();
            let mut used = vec![false; pieces.len()];
            for pos in 0..w * h {
                if let Some((pid, _)) = board.piece_at(pos) {
                    used[usize::from(pid)] = true;
                }
            }
            // Fresh candidate order per restart: the only randomness, shared
            // verbatim by every arm's fill phase.
            let mut cand: Vec<u16> = (0..pieces.len() as u16).collect();
            rng.shuffle(&mut cand);

            // Corridor phase (arms only): nearest-pair-first MST routes.
            if self.arm != Arm::Control {
                for &(i, j) in &mst {
                    self.lay_route(
                        &mut board, instance, &mut used, &cand, clue_xy[i], clue_xy[j], w, h,
                        &mut rng,
                    );
                }
            }

            // Shared fill phase: max-contact greedy to a full board.
            let mut empty: Vec<usize> = (0..w * h).filter(|&p| board.piece_at(p).is_none()).collect();
            while !empty.is_empty() {
                let mut max_contact = 0u32;
                for &pos in &empty {
                    let want = fit::edge_constraints(&board, pieces, pos % w, pos / w, w, h);
                    let c = want.iter().flatten().count() as u32;
                    if c > max_contact {
                        max_contact = c;
                    }
                }
                let ties: Vec<usize> = empty
                    .iter()
                    .copied()
                    .filter(|&pos| {
                        let want = fit::edge_constraints(&board, pieces, pos % w, pos / w, w, h);
                        want.iter().flatten().count() as u32 == max_contact
                    })
                    .collect();
                let pos = ties[rng.next_below(ties.len() as u32) as usize];
                let (pid, r) =
                    Self::best_candidate(&board, instance, &used, &cand, pos, w, h, false)
                        .expect("soft placement always finds a piece");
                board.place(pos, pid, r);
                used[usize::from(pid)] = true;
                empty.retain(|&p| p != pos);
                if budget.expired() {
                    break;
                }
            }

            let s = score_board(&board, pieces);
            if s > best_score {
                best_score = s;
                best = board;
            }
        }
        SolveOutcome::improved(best).with_nodes(restarts)
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let size: u8 = args.next().and_then(|s| s.parse().ok()).unwrap_or(16);
    let n_seeds: u32 = args.next().and_then(|s| s.parse().ok()).unwrap_or(12);
    let budget_s: f64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(20.0);
    let threads: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(8);

    // The substrate's board size is compile-time; refuse a mismatched run.
    let n_cells = Board::new().to_cell_codes().len();
    assert_eq!(
        n_cells,
        usize::from(size) * usize::from(size),
        "binary compiled for {n_cells} cells; rebuild with --features size-{size}"
    );

    let cells = ratio_clue_cells(usize::from(size));
    if size == 16 {
        // The ratio law must reproduce the official clue cells exactly.
        assert_eq!(cells, vec![34, 45, 135, 210, 221], "ratio law broken at N=16");
    }

    let arms = [Arm::Control, Arm::Path1, Arm::Ribbon2];
    let seeds: Vec<u32> = (1..=n_seeds).collect();
    let jobs: Vec<(Arm, u32)> = arms
        .iter()
        .flat_map(|&a| seeds.iter().map(move |&s| (a, s)))
        .collect();

    let next = AtomicUsize::new(0);
    let results: Mutex<Vec<(Arm, u32, u32, u64, Option<String>)>> = Mutex::new(Vec::new());

    std::thread::scope(|scope| {
        for _ in 0..threads {
            scope.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::Relaxed);
                let Some(&(arm, seed)) = jobs.get(i) else { break };
                let puzzle = generator::generate_framed(size, 22, seed, true);
                let name = format!("corr-{size}x{size}-s{seed}");
                let instance =
                    pin_cells(instance_from_generated(&name, &puzzle), size, 22, seed, &cells);
                let mut solver = CorridorSolver { arm, seed };
                let out = solver.solve(&instance, &instance.seed_board(), Budget::seconds(budget_s));
                let finished = instance.finish(&out.board);
                let url = (seed == 1).then_some(finished.url);
                results.lock().unwrap().push((arm, seed, finished.score, out.nodes, url));
            });
        }
    });

    let mut rows = results.into_inner().unwrap();
    rows.sort_by_key(|&(arm, seed, ..)| (arm.name(), seed));

    let mut arms_json = serde_json::Map::new();
    for &arm in &arms {
        let mut scores = Vec::new();
        let mut url1 = None;
        for (a, seed, score, restarts, url) in &rows {
            if *a == arm {
                scores.push(json!({ "seed": seed, "score": score, "restarts": restarts }));
                if url.is_some() {
                    url1 = url.clone();
                }
            }
        }
        arms_json.insert(
            arm.name().to_string(),
            json!({ "per_seed": scores, "board_url_seed1": url1 }),
        );
    }
    let out = json!({
        "size": size,
        "colors": 22,
        "clue_cells": cells,
        "paired_seeds": seeds,
        "budget_s_per_run": budget_s,
        "max_score": 2 * u32::from(size) * u32::from(size) - 2 * u32::from(size),
        "arms": arms_json,
    });
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}
