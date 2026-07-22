//! The scaling ladder: planted, solvable N x N instances with a free proven
//! ceiling of 2N(N-1) matched internal edges, run under a family of solvers,
//! reading off each solver's score-over-ceiling curve as N grows.
//!
//! Board size is a compile-time feature of e2-core, so one invocation covers
//! ONE rung (N = e2_core::W, selected by `--features size-N`):
//!
//!   cargo run --release --features size-8  -- --seeds 1,2,3 --budget-s 12
//!   cargo run --release --features size-10 -- --seeds 1,2,3 --budget-s 12
//!   cargo run --release --features size-12 -- --seeds 1,2,3 --budget-s 12
//!   cargo run --release --features size-14 -- --seeds 1,2,3 --budget-s 12
//!
//! Emits one JSON row per (solver, seed) on stdout; concatenate the four rungs
//! into results/ladder.jsonl. Every board is re-scored through the canonical
//! rim-excluding scorer (`Instance::finish`); no solver self-report is ever
//! trusted. Before any solver runs on a rung, the planted board is re-scored
//! and asserted to reach exactly 2N(N-1), executing the vol-240 sect A.2
//! planted-optimality lemma.
//!
//! Two baseline solvers span the two families the finding separates:
//!   dfs-full-match   backtracking DFS placing only fully matching pieces
//!                    (simplest member of the propagating family)
//!   greedy-row-major greedy filler, no backtracking (naive family)
//!
//! The original 13-method registry (gacolor_ac3 family, verhaard_preferred,
//! joe_depth150, border_first family, naive_spiral) is NOT ported yet; see
//! ../PLAN.md for what these two baselines can and cannot reproduce.

use e2_kit::{
    generator, instance_from_generated, pin_solution_hints, Board, Budget, Instance, Pieces,
    SolveOutcome, Solver,
};

const HINTS: u32 = 5;

fn main() {
    let args = Args::parse();
    let n = e2_core::W as u8; // the rung this binary was built for

    for &seed in &args.seeds {
        let (instance, ceiling) = build_rung(n, args.colors, seed);
        for solver in &mut [
            Box::new(DfsFullMatch) as Box<dyn Solver>,
            Box::new(GreedyRowMajor) as Box<dyn Solver>,
        ] {
            run_cell(solver.as_mut(), &instance, n, seed, ceiling, args.budget_s);
        }
    }
}

/// Build one ladder rung: a framed, colour-balanced, scrambled instance with
/// five solution cells pinned as clues, plus its proven ceiling 2N(N-1).
///
/// The ceiling is certified here, at build time: the planted (solved) board is
/// re-scored through the canonical scorer and asserted to reach exactly
/// 2N(N-1). If this assert ever fires the generator is broken and no solver
/// number downstream would mean anything.
fn build_rung(n: u8, colors: u8, seed: u32) -> (Instance, u32) {
    let colors = colors.min(generator::max_colors(n));
    let name = format!("ladder-n{n}-c{colors}-s{seed}");

    let puzzle = generator::generate_framed(n, colors, seed, true);
    let instance = instance_from_generated(&name, &puzzle);
    let instance = pin_solution_hints(instance, n, colors, seed, true, HINTS);

    let ceiling = 2 * u32::from(n) * (u32::from(n) - 1);

    // Planted-optimality lemma, executed: identity placement of the solved
    // form must reach the internal-adjacency bound through the same scorer
    // every solver's board goes through.
    let solved = generator::generate_solved_framed(n, colors, seed, true);
    let solved_instance = instance_from_generated(&name, &solved);
    let mut board = Board::new();
    for i in 0..solved.pieces.len() {
        board.place(i, i as u16, 0);
    }
    let planted = solved_instance.finish(&board);
    assert_eq!(
        planted.score, ceiling,
        "planted board must score the 2N(N-1) internal-edge bound (rung n={n} seed={seed})"
    );

    eprintln!(
        "# rung n={n} colors={colors} seed={seed} ceiling={ceiling} planted_url={}",
        planted.url
    );
    (instance, ceiling)
}

/// Run one (solver, instance) cell under the budget and emit its verified row.
fn run_cell(
    solver: &mut dyn Solver,
    instance: &Instance,
    n: u8,
    seed: u32,
    ceiling: u32,
    budget_s: f64,
) {
    let start = instance.seed_board();
    let budget = Budget::seconds(budget_s);
    let outcome = solver.solve(instance, &start, budget);
    let wall_s = budget.elapsed_secs();

    // The canonical re-score: the row's `score` comes from `finish`, never
    // from anything the solver claimed.
    let out = instance.finish(&outcome.board);
    let gap = f64::from(out.score) / f64::from(ceiling);
    let cells = usize::from(instance.width) * usize::from(instance.height);
    let full = (0..cells).all(|pos| !outcome.board.is_empty_at(pos));

    let row = serde_json::json!({
        "solver": solver.name(),
        "n": n,
        "seed": seed,
        "score": out.score,
        "ceiling": ceiling,
        "max_score_reported": out.max_score,
        "gap": (gap * 1000.0).round() / 1000.0,
        "full_solve": full && out.score == ceiling,
        "outcome": format!("{:?}", outcome.kind),
        "nodes": outcome.nodes,
        "wall_s": (wall_s * 100.0).round() / 100.0,
        "url": out.url,
    });
    println!("{row}");
}

// ---------------------------------------------------------------------------
// Solver 1: backtracking DFS, full-match-only placements, row-major order.
// The simplest member of the propagating family: it never places a piece that
// breaks a known constraint, and backtracks when a cell has no candidate.
// TODO(port): the vault's gap=1.000-through-N=12 rows belong to AC-3 based
// solvers (gacolor_ac3 family); this DFS is the honest floor of that family,
// not a faithful port. Porting AC-3 domain propagation is the deep part.
// ---------------------------------------------------------------------------

struct DfsFullMatch;

impl Solver for DfsFullMatch {
    fn name(&self) -> String {
        "dfs-full-match".into()
    }

    fn solve(&mut self, instance: &Instance, start: &Board, budget: Budget) -> SolveOutcome {
        let w = usize::from(instance.width);
        let h = usize::from(instance.height);
        let cells = w * h;
        let pieces = &instance.pieces;

        let mut board = start.clone();
        let mut used = vec![false; pieces.len()];
        for pos in 0..cells {
            if let Some((pid, _)) = board.piece_at(pos) {
                used[pid as usize] = true;
            }
        }
        let open: Vec<usize> = (0..cells).filter(|&p| board.is_empty_at(p)).collect();

        let mut st = DfsState {
            instance,
            open,
            used,
            nodes: 0,
            best_depth: 0,
            best_board: board.clone(),
            budget,
            out_of_time: false,
        };
        let complete = st.dfs(&mut board, 0);

        let nodes = st.nodes;
        let outcome = if complete {
            SolveOutcome::complete(board)
        } else {
            SolveOutcome::improved(st.best_board)
        };
        outcome.with_nodes(nodes)
    }
}

struct DfsState<'a> {
    instance: &'a Instance,
    open: Vec<usize>,
    used: Vec<bool>,
    nodes: u64,
    best_depth: usize,
    best_board: Board,
    budget: Budget,
    out_of_time: bool,
}

impl DfsState<'_> {
    fn dfs(&mut self, board: &mut Board, depth: usize) -> bool {
        if depth == self.open.len() {
            return true;
        }
        if self.out_of_time {
            return false;
        }
        let pos = self.open[depth];
        let w = usize::from(self.instance.width);
        let want = edge_constraints(
            board,
            &self.instance.pieces,
            pos % w,
            pos / w,
            self.instance.width,
            self.instance.height,
        );

        let piece_ids: Vec<u16> = self.instance.pieces.iter().map(|(pid, _)| pid).collect();
        for pid in piece_ids {
            if self.used[pid as usize] {
                continue;
            }
            let base = self.instance.pieces.get(pid).unwrap().edges;
            for r in 0..4 {
                self.nodes += 1;
                if self.nodes % 4096 == 0 && self.budget.expired() {
                    self.out_of_time = true;
                    return false;
                }
                let e = e2_core::rotated(base, r);
                if !satisfies(&e, &want) {
                    continue;
                }
                board.place(pos, pid, r);
                self.used[pid as usize] = true;
                if depth + 1 > self.best_depth {
                    self.best_depth = depth + 1;
                    self.best_board = board.clone();
                }
                if self.dfs(board, depth + 1) {
                    return true;
                }
                board.clear(pos);
                self.used[pid as usize] = false;
                if self.out_of_time {
                    return false;
                }
            }
        }
        false
    }
}

// ---------------------------------------------------------------------------
// Solver 2: greedy row-major, no backtracking (naive family). Adapted from the
// kit's examples/my_solver.rs baseline.
// ---------------------------------------------------------------------------

struct GreedyRowMajor;

impl Solver for GreedyRowMajor {
    fn name(&self) -> String {
        "greedy-row-major".into()
    }

    fn solve(&mut self, instance: &Instance, start: &Board, budget: Budget) -> SolveOutcome {
        let w = usize::from(instance.width);
        let h = usize::from(instance.height);
        let cells = w * h;
        let pieces = &instance.pieces;

        let mut board = start.clone();
        let mut used = vec![false; pieces.len()];
        for pos in 0..cells {
            if let Some((pid, _)) = board.piece_at(pos) {
                used[pid as usize] = true;
            }
        }

        let mut nodes = 0u64;
        for pos in 0..cells {
            if budget.expired() {
                break;
            }
            if !board.is_empty_at(pos) {
                continue;
            }
            let want = edge_constraints(&board, pieces, pos % w, pos / w, instance.width, instance.height);
            let mut best: Option<(u16, u8, u32)> = None;
            for (pid, piece) in pieces.iter() {
                if used[pid as usize] {
                    continue;
                }
                for r in 0..4 {
                    nodes += 1;
                    let e = piece.rotated(r);
                    if satisfies(&e, &want) {
                        let matched = want.iter().filter(|c| c.is_some()).count() as u32;
                        if best.is_none_or(|(_, _, b)| matched > b) {
                            best = Some((pid, r, matched));
                        }
                    }
                }
            }
            if let Some((pid, r, _)) = best {
                board.place(pos, pid, r);
                used[pid as usize] = true;
            }
        }
        SolveOutcome::improved(board).with_nodes(nodes)
    }
}

// ---------------------------------------------------------------------------
// Shared constraint helpers (URDL, rim colour 0, placed neighbours are hard).
// ---------------------------------------------------------------------------

/// Per-side requirements for the piece at `(x, y)`, URDL. `Some(c)` means the
/// edge must be colour `c` (rim edges must be 0, placed neighbours must be
/// matched); `None` means free.
fn edge_constraints(
    board: &Board,
    pieces: &Pieces,
    x: usize,
    y: usize,
    width: u8,
    height: u8,
) -> [Option<u8>; 4] {
    let w = usize::from(width);
    let h = usize::from(height);
    let at = |cx: usize, cy: usize| -> Option<[u8; 4]> {
        board
            .piece_at(cy * w + cx)
            .map(|(pid, r)| pieces.get(pid).unwrap().rotated(r))
    };
    let up = if y == 0 { Some(0) } else { at(x, y - 1).map(|e| e[2]) };
    let right = if x == w - 1 { Some(0) } else { at(x + 1, y).map(|e| e[3]) };
    let down = if y == h - 1 { Some(0) } else { at(x, y + 1).map(|e| e[0]) };
    let left = if x == 0 { Some(0) } else { at(x - 1, y).map(|e| e[1]) };
    [up, right, down, left]
}

/// True when edges `e` satisfy every hard constraint in `want`.
fn satisfies(e: &[u8; 4], want: &[Option<u8>; 4]) -> bool {
    want.iter()
        .enumerate()
        .all(|(side, req)| req.is_none_or(|c| e[side] == c))
}

// ---------------------------------------------------------------------------
// Args
// ---------------------------------------------------------------------------

struct Args {
    seeds: Vec<u32>,
    colors: u8,
    budget_s: f64,
}

impl Args {
    fn parse() -> Self {
        let mut seeds = vec![1, 2, 3];
        let mut colors = 22u8;
        let mut budget_s = 12.0;
        let argv: Vec<String> = std::env::args().skip(1).collect();
        let mut i = 0;
        while i < argv.len() {
            match argv[i].as_str() {
                "--seeds" => seeds = list(&argv[i + 1]),
                "--colors" => colors = argv[i + 1].parse().expect("--colors u8"),
                "--budget-s" => budget_s = argv[i + 1].parse().expect("--budget-s f64"),
                other => panic!("unknown arg {other}; use --seeds --colors --budget-s (rung is picked by --features size-N)"),
            }
            i += 2;
        }
        Self { seeds, colors, budget_s }
    }
}

fn list<T: std::str::FromStr>(s: &str) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    s.split(',')
        .map(|p| p.trim().parse().expect("comma-separated list"))
        .collect()
}
