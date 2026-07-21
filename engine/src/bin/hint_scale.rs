//! hint_scale — the hint study's RUNTIME-SIZED solver, for the scaling axis.
//!
//! Unlike the DFS study's `run_dfs` (compiled for one fixed board size via a
//! cargo feature), this drives the site engine's runtime-sized backtracker
//! (`Solver`, board dimensions are fields, not consts), so ONE binary runs every
//! board size — 8×8 through 16×16 — with no per-size build and no chance of a
//! size-mismatch. It is slightly slower than a size-specialised build, which does
//! not matter here: the scaling axis compares score *ratios* across sizes, not
//! absolute throughput.
//!
//! It fills cells in a chosen path order (hint-aware, so `connect-hints-first`
//! works), honours the puzzle's pins, and reports one machine-readable RESULT
//! line matching `run_dfs`'s fields (score, max_depth, nodes, …) so the same
//! analysis reads both.
//!
//! Usage:
//!   hint_scale --puzzle P.json --path row-major --budget-s 8 [--seed 1]

use std::time::Instant;

use eternity2_engine::solver::{score_board, Solver, Status};
use eternity2_engine::{build_path_with_hints, Puzzle};

fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let flag = |name: &str| -> Option<String> {
        argv.iter().position(|a| a == name).and_then(|i| argv.get(i + 1)).cloned()
    };

    let Some(puzzle_path) = flag("--puzzle") else {
        eprintln!("usage: hint_scale --puzzle P.json --path row-major --budget-s 8 [--seed 1]");
        std::process::exit(2);
    };
    let path_kind = flag("--path").unwrap_or_else(|| "row-major".to_string());
    let budget_s: f64 = flag("--budget-s").and_then(|s| s.parse().ok()).unwrap_or(8.0);
    let seed: u32 = flag("--seed").and_then(|s| s.parse().ok()).unwrap_or(1);

    let text = match std::fs::read_to_string(&puzzle_path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("error: reading {puzzle_path}: {e}");
            std::process::exit(1);
        }
    };
    let puzzle: Puzzle = match serde_json::from_str(&text) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("error: parsing {puzzle_path}: {e}");
            std::process::exit(1);
        }
    };

    let hint_cells: Vec<u16> = puzzle.hints.iter().map(|h| h.pos).collect();
    let Some(path) = build_path_with_hints(&path_kind, puzzle.width, puzzle.height, seed, &hint_cells)
    else {
        eprintln!("error: unknown path kind {path_kind:?}");
        std::process::exit(1);
    };

    let mut solver = match Solver::new(&puzzle, &path, true, true, seed) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    };

    // Run to the time budget, stepping in chunks and tracking the best score.
    let start = Instant::now();
    let budget = std::time::Duration::from_secs_f64(budget_s);
    let mut best_score = score_board(&puzzle, solver.best_board());
    let mut last = Status::Running;
    loop {
        let r = solver.step(2_000_000);
        let s = score_board(&puzzle, solver.best_board());
        if s > best_score {
            best_score = s;
        }
        last = r.status;
        if r.status != Status::Running || start.elapsed() >= budget {
            break;
        }
    }
    let report = solver.step(0); // a zero-budget step reads current counters
    let elapsed = start.elapsed().as_secs_f64();

    println!(
        "RESULT path={} size={}x{} seed={} score={} solved={} best_placed={} placed={} \
         nodes={:.0} backtracks={:.0} elapsed_s={:.3} status={:?}",
        path_kind,
        puzzle.width,
        puzzle.height,
        seed,
        best_score,
        u8::from(last == Status::Solved),
        report.best_placed,
        report.placed,
        report.nodes,
        report.backtracks,
        elapsed,
        last,
    );
}
