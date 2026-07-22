//! ledger-prune reproduction: a break-budget row-major DFS with the LEDGER
//! global colour-ledger prune (deficit + parity vs the remaining break budget).
//!
//! Modes:
//!   cargo run --release -- --mode soundness [--seed S]
//!   cargo run --release -- --mode ab [--seed S] [--suffix K] [--budget R] [--cap-secs T]
//!
//! Soundness replays a generated board's perfect tail at zero slack and asserts
//! the ledger never fires along the real line. AB exhausts a fixed-prefix
//! suffix twice (prune off / prune on), counting ALL completions within the
//! break budget; completions must be identical between arms and the node ratio
//! is the reproduced quantity. See PLAN.md for the claim and its provenance.

#![forbid(unsafe_code)]

use std::time::{Duration, Instant};

use e2_kit::{generator, instance_from_generated, Board, Budget, Instance, SolveOutcome, Solver};

/// Colour ids fit comfortably below 32 on the official shape (22 interior + 5
/// frame colours + grey 0).
const MAXC: usize = 32;

struct Ctx {
    w: usize,
    h: usize,
    n: usize,
    /// Per piece: its URDL edges at each of the 4 rotations.
    rots: Vec<[[u8; 4]; 4]>,
}

impl Ctx {
    fn new(instance: &Instance) -> Self {
        let w = usize::from(instance.width);
        let h = usize::from(instance.height);
        let rots = instance
            .pieces
            .iter()
            .map(|(_, p)| [p.rotated(0), p.rotated(1), p.rotated(2), p.rotated(3)])
            .collect();
        Self { w, h, n: w * h, rots }
    }
}

struct Search<'a> {
    ctx: &'a Ctx,
    /// Placed edge quads, row-major; None = empty.
    cells: Vec<Option<[u8; 4]>>,
    /// Placed (piece, rot), parallel to `cells` (to rebuild a Board).
    assign: Vec<Option<(u16, u8)>>,
    used: Vec<bool>,
    /// S(c): half-edge supply over unused pieces (rotation-invariant multiset).
    supply: [i64; MAXC],
    ledger_on: bool,
    nodes: u64,
    fires: u64,
    completions: u64,
    first_completion: Option<Board>,
    deadline: Instant,
    timed_out: bool,
}

impl<'a> Search<'a> {
    /// Build search state from a row-major-contiguous prefix board.
    fn from_prefix(ctx: &'a Ctx, instance: &Instance, start: &Board) -> (Self, usize) {
        let mut cells: Vec<Option<[u8; 4]>> = vec![None; ctx.n];
        let mut assign: Vec<Option<(u16, u8)>> = vec![None; ctx.n];
        let mut used = vec![false; instance.pieces.len()];
        let mut supply = [0i64; MAXC];
        for (pid, p) in instance.pieces.iter() {
            if start_piece_pos(start, ctx.n, pid).is_none() {
                for &c in &p.edges {
                    supply[usize::from(c)] += 1;
                }
            } else {
                used[usize::from(pid)] = true;
            }
        }
        for pos in 0..ctx.n {
            if let Some((pid, r)) = start.piece_at(pos) {
                cells[pos] = Some(ctx.rots[usize::from(pid)][usize::from(r)]);
                assign[pos] = Some((pid, r));
            }
        }
        let first_empty = (0..ctx.n).find(|&p| cells[p].is_none()).unwrap_or(ctx.n);
        assert!(
            (first_empty..ctx.n).all(|p| cells[p].is_none()),
            "start board must be a row-major-contiguous prefix (non-contiguous \
             prefixes are a phase-2 item, see PLAN.md)"
        );
        (
            Self {
                ctx,
                cells,
                assign,
                used,
                supply,
                ledger_on: false,
                nodes: 0,
                fires: 0,
                completions: 0,
                first_completion: None,
                deadline: Instant::now() + Duration::from_secs(3600),
                timed_out: false,
            },
            first_empty,
        )
    }

    /// The two LEDGER necessary conditions at the node about to expand `pos`,
    /// with `r` charged breaks still available. Recomputed from scratch: sound
    /// and fire-identical to an incremental ledger, just slower per node.
    /// Hint-free reproduction, so the free-junction term u is 0.
    fn ledger_ok(&self, pos: usize, r: u32) -> bool {
        let (w, h) = (self.ctx.w, self.ctx.h);
        let mut demand = [0i64; MAXC];
        for e in pos..self.ctx.n {
            let (x, y) = (e % w, e / w);
            if y == 0 {
                demand[0] += 1;
            } else if let Some(q) = self.cells[e - w] {
                demand[usize::from(q[2])] += 1;
            }
            if x == w - 1 {
                demand[0] += 1;
            } else if let Some(q) = self.cells[e + 1] {
                demand[usize::from(q[3])] += 1;
            }
            if y == h - 1 {
                demand[0] += 1;
            } else if let Some(q) = self.cells[e + w] {
                demand[usize::from(q[0])] += 1;
            }
            if x == 0 {
                demand[0] += 1;
            } else if let Some(q) = self.cells[e - 1] {
                demand[usize::from(q[1])] += 1;
            }
        }
        let mut deficit = 0i64;
        let mut odd = 0u32;
        for c in 0..MAXC {
            let f = self.supply[c] - demand[c];
            if f < 0 {
                deficit -= f;
            }
            if f & 1 != 0 {
                odd += 1;
            }
        }
        deficit <= i64::from(r) && odd <= 2 * r
    }

    /// Exhaustive break-budget DFS from `pos`: counts every completion using at
    /// most `r` further charged breaks. Breaks are charged per edge (top/left
    /// junction mismatches); all four rim edges are hard (proper frame), so
    /// this is the per-edge, no-per-cell-cap convention (see PLAN.md scope
    /// note vs the vault engine's relaxed buckets).
    fn dfs(&mut self, pos: usize, r: u32) {
        if pos == self.ctx.n {
            self.completions += 1;
            if self.first_completion.is_none() {
                let mut b = Board::new();
                for (p, a) in self.assign.iter().enumerate() {
                    if let Some((pid, rot)) = a {
                        b.place(p, *pid, *rot);
                    }
                }
                self.first_completion = Some(b);
            }
            return;
        }
        self.nodes += 1;
        if self.nodes & 0x3FFF == 0 && Instant::now() >= self.deadline {
            self.timed_out = true;
        }
        if self.timed_out {
            return;
        }
        if self.ledger_on && !self.ledger_ok(pos, r) {
            self.fires += 1;
            return;
        }
        let (w, h) = (self.ctx.w, self.ctx.h);
        let (x, y) = (pos % w, pos / w);
        // In row-major order with a contiguous prefix, the up/left neighbours
        // of the frontier cell are always placed (unless on the rim). URDL:
        // the neighbour above exposes its Down edge (q[2]); the neighbour to
        // the left exposes its Right edge (q[1]).
        let up = if y > 0 { self.cells[pos - w].map(|q| q[2]) } else { None };
        let left = if x > 0 { self.cells[pos - 1].map(|q| q[1]) } else { None };
        for pid in 0..self.ctx.rots.len() {
            if self.used[pid] {
                continue;
            }
            for rot in 0..4u8 {
                let e = self.ctx.rots[pid][usize::from(rot)];
                // Proper frame: an edge is grey exactly when it lies on the rim.
                if (y == 0) != (e[0] == 0)
                    || (x == w - 1) != (e[1] == 0)
                    || (y == h - 1) != (e[2] == 0)
                    || (x == 0) != (e[3] == 0)
                {
                    continue;
                }
                let mut b = 0u32;
                if let Some(c) = up {
                    if e[0] != c {
                        b += 1;
                    }
                }
                if let Some(c) = left {
                    if e[3] != c {
                        b += 1;
                    }
                }
                if b > r {
                    continue;
                }
                self.cells[pos] = Some(e);
                self.assign[pos] = Some((pid as u16, rot));
                self.used[pid] = true;
                for &c in &self.ctx.rots[pid][0] {
                    self.supply[usize::from(c)] -= 1;
                }
                self.dfs(pos + 1, r - b);
                for &c in &self.ctx.rots[pid][0] {
                    self.supply[usize::from(c)] += 1;
                }
                self.used[pid] = false;
                self.cells[pos] = None;
                self.assign[pos] = None;
                if self.timed_out {
                    return;
                }
            }
        }
    }
}

/// Where (if anywhere) `pid` sits on `start`.
fn start_piece_pos(start: &Board, n: usize, pid: u16) -> Option<usize> {
    (0..n).find(|&p| start.piece_at(p).map(|(q, _)| q) == Some(pid))
}

/// The kit-facing skeleton: LEDGER break-budget DFS as a [`Solver`]. Handed a
/// row-major-contiguous prefix as the start board, it exhausts the suffix
/// within `breaks` charged mismatches and returns the first completion found
/// (or the untouched start when the space is empty), with the honest outcome
/// kind: `Exhausted` when the search ran to completion, `Improved` on a budget
/// cut-off.
struct LedgerBreakDfs {
    breaks: u32,
    ledger_on: bool,
}

impl Solver for LedgerBreakDfs {
    fn name(&self) -> String {
        if self.ledger_on { "ledger-break-dfs".into() } else { "noprune-break-dfs".into() }
    }

    fn solve(&mut self, instance: &Instance, start: &Board, budget: Budget) -> SolveOutcome {
        let ctx = Ctx::new(instance);
        let (mut s, first_empty) = Search::from_prefix(&ctx, instance, start);
        s.ledger_on = self.ledger_on;
        // Respect the kit budget: poll via a deadline derived from it.
        s.deadline = Instant::now() + Duration::from_secs_f64(remaining_secs(&budget));
        s.dfs(first_empty, self.breaks);
        let board = s.first_completion.clone().unwrap_or_else(|| start.clone());
        let out = if s.timed_out {
            SolveOutcome::improved(board)
        } else {
            SolveOutcome::exhausted(board)
        };
        out.with_nodes(s.nodes)
    }
}

/// Seconds left on a kit [`Budget`]. The kit exposes `expired`/`fraction` but
/// not the raw limit, so this reconstructs a conservative remainder.
/// (kit gap: a `Budget::remaining()` accessor would remove this dance.)
fn remaining_secs(budget: &Budget) -> f64 {
    if budget.expired() {
        return 0.0;
    }
    let frac = budget.fraction();
    if frac <= f64::EPSILON {
        return 3600.0;
    }
    (budget.elapsed_secs() / frac - budget.elapsed_secs()).clamp(0.0, 3600.0)
}

/// A generated official-shaped instance plus its known solution as cell codes.
fn generated_with_solution(seed: u32) -> (Instance, Vec<i32>) {
    let puzzle = generator::generate_framed(16, 22, seed, true);
    let instance = instance_from_generated(&format!("gen-16x16-c22-s{seed}"), &puzzle);
    let solved = generator::generate_solved_framed(16, 22, seed, true);
    let out = instance.match_board(&solved.pieces);
    assert_eq!(out.breaks, 0, "recovered solution must be perfect");
    println!("instance gen-16x16-c22-s{seed}  solution: {}/480", out.score);
    println!("solution board: {}", out.url);
    (instance, out.board)
}

fn prefix_board(codes: &[i32], keep: usize) -> Board {
    let mut b = Board::new();
    for (pos, &code) in codes.iter().enumerate().take(keep) {
        if code >= 0 {
            b.place(pos, (code / 4) as u16, (code % 4) as u8);
        }
    }
    b
}

/// Gate: replay the perfect tail at zero slack. At every prefix depth the
/// remaining budget along the real line is 0, so any ledger fire is unsound.
fn run_soundness(seed: u32) {
    let (instance, codes) = generated_with_solution(seed);
    let ctx = Ctx::new(&instance);
    let mut fires = 0u32;
    for depth in 0..=ctx.n {
        let start = prefix_board(&codes, depth);
        let (s, pos) = Search::from_prefix(&ctx, &instance, &start);
        assert_eq!(pos, depth);
        if !s.ledger_ok(depth, 0) {
            fires += 1;
            println!("UNSOUND: ledger fired at depth {depth} with r=0 on the real line");
        }
    }
    println!(
        "soundness replay seed {seed}: {} depths judged, {fires} fires (expected 0) -> {}",
        ctx.n + 1,
        if fires == 0 { "PASS" } else { "FAIL" }
    );
}

/// The A/B race: exhaust the suffix twice, prune off and on. Completions must
/// match exactly; the node ratio is the reproduced quantity.
fn run_ab(seed: u32, suffix: usize, breaks: u32, cap_secs: f64) {
    let (instance, codes) = generated_with_solution(seed);
    let ctx = Ctx::new(&instance);
    let keep = ctx.n - suffix;
    let start = prefix_board(&codes, keep);
    println!(
        "A/B suffix exhaustion: prefix D={keep}, suffix {suffix} cells, budget {breaks} breaks, cap {cap_secs}s/arm"
    );
    let mut rows = Vec::new();
    for ledger_on in [false, true] {
        let (mut s, pos) = Search::from_prefix(&ctx, &instance, &start);
        s.ledger_on = ledger_on;
        s.deadline = Instant::now() + Duration::from_secs_f64(cap_secs);
        let t0 = Instant::now();
        s.dfs(pos, breaks);
        let dt = t0.elapsed().as_secs_f64();
        println!(
            "  {}: nodes {}  completions {}  fires {}  {:.3}s{}",
            if ledger_on { "LEDGER " } else { "NoPrune" },
            s.nodes,
            s.completions,
            s.fires,
            dt,
            if s.timed_out { "  TIMEOUT (ratio not comparable)" } else { "" }
        );
        rows.push((s.nodes, s.completions, s.timed_out));
    }
    let (n0, c0, t0) = rows[0];
    let (n1, c1, t1) = rows[1];
    if !t0 && !t1 {
        assert_eq!(c0, c1, "arms disagree on completion count: prune is unsound or engine bug");
        println!(
            "  node ratio NoPrune/LEDGER = {:.2}x  (completions agree: {c0})",
            n0 as f64 / n1.max(1) as f64
        );
    }
}

/// The kit-integration path: the same search driven through the [`Solver`]
/// trait, continuing from a prefix board and finishing through the canonical
/// scorer, the way a sweep or a record re-measurement would consume it.
fn run_solver_demo(seed: u32, suffix: usize, breaks: u32, cap_secs: f64) {
    let (instance, codes) = generated_with_solution(seed);
    let n = usize::from(instance.width) * usize::from(instance.height);
    let start = prefix_board(&codes, n - suffix);
    let mut solver = LedgerBreakDfs { breaks, ledger_on: true };
    let outcome = solver.solve(&instance, &start, Budget::seconds(cap_secs));
    let out = instance.finish(&outcome.board);
    println!("solver:  {}", solver.name());
    println!("score:   {} / {}  breaks {}", out.score, out.max_score, out.breaks);
    println!("outcome: {:?}  nodes {}", outcome.kind, outcome.nodes);
    println!("board:   {}", out.url);
}

fn arg<T: std::str::FromStr>(name: &str, default: T) -> T {
    let args: Vec<String> = std::env::args().collect();
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn main() {
    let mode: String = arg("--mode", "all".to_string());
    let seed: u32 = arg("--seed", 1);
    let suffix: usize = arg("--suffix", 20);
    let breaks: u32 = arg("--budget", 1);
    let cap_secs: f64 = arg("--cap-secs", 25.0);
    match mode.as_str() {
        "soundness" => run_soundness(seed),
        "ab" => run_ab(seed, suffix, breaks, cap_secs),
        "solver" => run_solver_demo(seed, suffix, breaks, cap_secs),
        _ => {
            run_soundness(seed);
            run_ab(seed, suffix, breaks, cap_secs);
        }
    }
}
