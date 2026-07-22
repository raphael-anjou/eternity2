//! n13-encoding-cliff — heuristic arm + instance exporter.
//!
//!   cargo run --release -- sweep  --ns 10,11,12,13,14 --gen-seeds 1,2,3,4,5 --budget-secs 20
//!   cargo run --release -- export --ns 10,...,16 --gen-seeds 1,2,3,4,5 --out instances
//!
//! `sweep` measures the heuristic side of the claim: a restarting randomized
//! depth-first search that places only exact matches, on planted framed
//! instances with five pinned solution clues. Below the cliff it full-solves in
//! milliseconds; at the cliff its score pins far below target and stays there
//! regardless of budget. One JSON line per (size, seed) cell on stdout; a human
//! table on stderr.
//!
//! `export` writes the identical instances as JSON for the exact arm
//! (`cpsat_cliff.py`), so both arms measure the same boards.
//!
//! Everything canonical comes from the starter kit: the generator, the clue
//! pinning, the scorer, the viewer URL. Nothing here re-implements scoring.

use e2_kit::{
    fit, generator, instance_from_generated, pin_solution_hints, Board, Budget, Instance, Pieces,
    SolveOutcome, Solver, XorShift, BORDER,
};

const HINTS: u32 = 5;

/// The kit's `Board` and `score_cells` are fixed 16x16 (stride 16). A smaller
/// NxN instance is embedded at the top-left: cell (x, y) of the sub-board
/// lives at `y * 16 + x`. Empty cells never score and a sub-board rim edge is
/// colour 0 (which `score_cells` excludes), so the canonical scorer counts
/// exactly the NxN interior matches and nothing else. `pin_solution_hints`
/// returns N-wide positions, so hints are remapped through this before they
/// touch a `Board`.
const STRIDE: usize = 16;

fn embed(pos_n: usize, n: usize) -> usize {
    (pos_n / n) * STRIDE + pos_n % n
}

/// The seeded start board with the instance's hints at embedded positions.
fn embedded_seed_board(instance: &Instance) -> Board {
    let n = usize::from(instance.width);
    let mut b = Board::new();
    for h in &instance.hints {
        b.place(embed(usize::from(h.pos), n), h.piece, h.rot);
    }
    b
}

// ---------------------------------------------------------------------------
// The heuristic instrument: restarting randomized exact-match DFS.
// ---------------------------------------------------------------------------

/// Restarting randomized DFS, the `cdfs_edges` analogue from the source
/// measurement. Chronological search in row-major order over the empty cells;
/// a candidate is any unused (piece, rotation) whose every constrained side
/// matches exactly (rim colour 0, placed neighbours equal). Candidate order is
/// shuffled per node; each restart gets a node cap and a fresh shuffle stream.
/// The best (deepest) partial board across restarts is returned, so the score
/// is honest when the search never completes.
struct RestartingDfs {
    solver_seed: u32,
    restart_nodes: u64,
}

struct Search<'a> {
    pieces: &'a Pieces,
    /// Sub-board side length (positions are 16-stride embedded, see [`embed`]).
    n: usize,
    slots: &'a [usize],
    board: Board,
    used: Vec<bool>,
    rng: XorShift,
    nodes: u64,
    restart_start: u64,
    restart_cap: u64,
    best_depth: usize,
    best_board: Board,
    budget: Budget,
}

impl Search<'_> {
    /// Required colour per URDL side of `pos`: `Some(BORDER)` on a sub-board
    /// rim side, `Some(c)` against a placed neighbour, `None` when
    /// unconstrained. The kit's [`fit::edge_constraints`] does the neighbour
    /// arithmetic on the 16-stride board; only the sub-board's right and
    /// bottom rims differ from the 16x16 rims (the embedding is top-left, so
    /// top and left coincide, and the cells past the sub-board edge are
    /// always empty), so those two sides are overridden to the rim colour.
    fn wants(&self, pos: usize) -> [Option<u8>; 4] {
        let n = self.n;
        let (x, y) = (pos % STRIDE, pos / STRIDE);
        let mut want = fit::edge_constraints(&self.board, self.pieces, x, y, STRIDE, STRIDE);
        if x == n - 1 {
            want[1] = Some(BORDER);
        }
        if y == n - 1 {
            want[2] = Some(BORDER);
        }
        want
    }

    /// True when the search must unwind (budget gone or restart cap reached).
    fn out_of_gas(&self) -> bool {
        self.nodes - self.restart_start >= self.restart_cap
            || (self.nodes % 256 == 0 && self.budget.expired())
    }

    fn dfs(&mut self, depth: usize) -> bool {
        if depth > self.best_depth {
            self.best_depth = depth;
            self.best_board = self.board.clone();
        }
        if depth == self.slots.len() {
            return true;
        }
        if self.out_of_gas() {
            return false;
        }
        let pos = self.slots[depth];
        let want = self.wants(pos);

        let mut cands: Vec<(u16, u8)> = Vec::new();
        for (pid, piece) in self.pieces.iter() {
            if self.used[pid as usize] {
                continue;
            }
            for r in 0..4 {
                if fit::fit_score(&piece.rotated(r), &want).is_some() {
                    cands.push((pid, r));
                }
            }
        }
        self.rng.shuffle(&mut cands);

        for (pid, r) in cands {
            self.nodes += 1;
            self.board.place(pos, pid, r);
            self.used[pid as usize] = true;
            if self.dfs(depth + 1) {
                return true;
            }
            self.board.clear(pos);
            self.used[pid as usize] = false;
            if self.out_of_gas() {
                return false;
            }
        }
        false
    }
}

impl Solver for RestartingDfs {
    fn name(&self) -> String {
        format!("restarting-dfs-s{}", self.solver_seed)
    }

    fn solve(&mut self, instance: &Instance, start: &Board, budget: Budget) -> SolveOutcome {
        let n_sub = usize::from(instance.width);
        // Row-major over the sub-board, in 16-stride embedded positions.
        let slots: Vec<usize> = (0..n_sub * n_sub)
            .map(|p| embed(p, n_sub))
            .filter(|&p| start.is_empty_at(p))
            .collect();
        let mut used = vec![false; instance.pieces.len()];
        for p in 0..STRIDE * STRIDE {
            if let Some((pid, _)) = start.piece_at(p) {
                used[pid as usize] = true;
            }
        }

        let mut s = Search {
            pieces: &instance.pieces,
            n: n_sub,
            slots: &slots,
            board: start.clone(),
            used,
            rng: XorShift::new(self.solver_seed),
            nodes: 0,
            restart_start: 0,
            restart_cap: self.restart_nodes,
            best_depth: 0,
            best_board: start.clone(),
            budget,
        };

        let mut restarts = 0u64;
        let solved = loop {
            s.restart_start = s.nodes;
            if s.dfs(0) {
                break true;
            }
            // Reset to the seeded start for the next restart; the shuffle
            // stream continues, so each restart explores a different order.
            s.board = start.clone();
            for u in &mut s.used {
                *u = false;
            }
            for p in 0..STRIDE * STRIDE {
                if let Some((pid, _)) = start.piece_at(p) {
                    s.used[pid as usize] = true;
                }
            }
            restarts += 1;
            if budget.expired() {
                break false;
            }
        };

        let nodes = s.nodes;
        let out = if solved {
            SolveOutcome::complete(s.board)
        } else {
            eprintln!("    ({restarts} restarts, best depth {}/{})", s.best_depth, slots.len());
            SolveOutcome::improved(s.best_board)
        };
        out.with_nodes(nodes)
    }
}

// ---------------------------------------------------------------------------
// Instance construction (shared by both modes).
// ---------------------------------------------------------------------------

fn build_instance(size: u8, colors: u8, gen_seed: u32) -> Instance {
    let effective = colors.min(generator::max_colors(size));
    let puzzle = generator::generate_framed(size, effective, gen_seed, true);
    let instance = instance_from_generated(
        &format!("cliff-{size}x{size}-c{effective}-s{gen_seed}"),
        &puzzle,
    );
    pin_solution_hints(instance, size, effective, gen_seed, true, HINTS)
}

// ---------------------------------------------------------------------------
// CLI.
// ---------------------------------------------------------------------------

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mode = args.first().map_or("sweep", String::as_str);
    let ns = list_flag(&args, "--ns").unwrap_or_else(|| vec![10, 11, 12, 13, 14]);
    let gen_seeds = list_flag(&args, "--gen-seeds").unwrap_or_else(|| vec![1, 2, 3, 4, 5]);
    let colors: u8 = flag(&args, "--colors").unwrap_or(22);
    match mode {
        "sweep" => {
            let budget_secs: f64 = flag(&args, "--budget-secs").unwrap_or(20.0);
            let solver_seed: u32 = flag(&args, "--solver-seed").unwrap_or(1);
            let restart_nodes: u64 = flag(&args, "--restart-nodes").unwrap_or(100_000);
            sweep(&ns, &gen_seeds, colors, budget_secs, solver_seed, restart_nodes);
        }
        "export" => {
            let out = str_flag(&args, "--out").unwrap_or_else(|| "instances".into());
            export(&ns, &gen_seeds, colors, &out);
        }
        other => {
            eprintln!("unknown mode `{other}` (expected `sweep` or `export`)");
            std::process::exit(2);
        }
    }
}

fn sweep(ns: &[u32], gen_seeds: &[u32], colors: u8, budget_secs: f64, solver_seed: u32, restart_nodes: u64) {
    eprintln!("heuristic arm: restarting exact-match DFS, {HINTS} pinned clues, budget {budget_secs} s");
    eprintln!("{:>3} {:>5} {:>7} {:>7} {:>6} {:>9} {:>8}", "N", "seed", "score", "target", "full", "secs", "nodes");
    for &n in ns {
        for &gs in gen_seeds {
            let size = n as u8;
            let instance = build_instance(size, colors, gs);
            let target = instance.max_score();
            let mut solver = RestartingDfs { solver_seed, restart_nodes };
            let start = embedded_seed_board(&instance);
            let budget = Budget::seconds(budget_secs);
            let outcome = solver.solve(&instance, &start, budget);
            let secs = budget.elapsed_secs();
            let out = instance.finish(&outcome.board);
            let full = out.score == target;
            eprintln!(
                "{:>3} {:>5} {:>7} {:>7} {:>6} {:>9.3} {:>8}",
                n, gs, out.score, target, full, secs, outcome.nodes
            );
            let line = serde_json::json!({
                "arm": "heuristic",
                "solver": solver.name(),
                "n": n,
                "colors": instance.num_colors,
                "gen_seed": gs,
                "hints": HINTS,
                "score": out.score,
                "target": target,
                "full_solve": full,
                "secs": secs,
                "nodes": outcome.nodes,
                "url": out.url,
            });
            println!("{line}");
        }
    }
}

fn export(ns: &[u32], gen_seeds: &[u32], colors: u8, out_dir: &str) {
    std::fs::create_dir_all(out_dir).expect("create export dir");
    for &n in ns {
        for &gs in gen_seeds {
            let size = n as u8;
            let instance = build_instance(size, colors, gs);
            let pieces: Vec<[u8; 4]> = instance.pieces.iter().map(|(_, p)| p.edges).collect();
            let hints: Vec<serde_json::Value> = instance
                .hints
                .iter()
                .map(|h| serde_json::json!({"pos": h.pos, "piece": h.piece, "rot": h.rot}))
                .collect();
            let doc = serde_json::json!({
                "name": instance.name,
                "size": instance.width,
                "num_colors": instance.num_colors,
                "target": instance.max_score(),
                "pieces": pieces,
                "hints": hints,
            });
            let path = format!("{out_dir}/inst_n{n}_c{}_s{gs}.json", instance.num_colors);
            std::fs::write(&path, serde_json::to_string(&doc).unwrap()).expect("write instance");
            eprintln!("wrote {path}");
        }
    }
}

// -- tiny arg helpers --------------------------------------------------------

fn str_flag(args: &[String], name: &str) -> Option<String> {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).cloned()
}

fn flag<T: std::str::FromStr>(args: &[String], name: &str) -> Option<T> {
    str_flag(args, name).and_then(|v| v.parse().ok())
}

fn list_flag(args: &[String], name: &str) -> Option<Vec<u32>> {
    str_flag(args, name).map(|v| v.split(',').filter_map(|s| s.trim().parse().ok()).collect())
}
