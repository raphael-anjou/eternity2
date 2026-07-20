//! Run a solver across a grid of seeds and write a reproducible run directory.
//!
//!   cargo run --release --example sweep                       # 20 seeds, 2s each
//!   cargo run --release --example sweep -- --n 40 --budget 3  # 40 seeds, 3s each
//!   cargo run --release --example sweep -- --pins 5           # pin 5 clue cells
//!
//! This is the iteration loop. It generates one board per seed, runs your
//! [`Solver`] on each single-core, re-scores canonically, and writes
//! `runs/<solver>-<cfg>-<stamp>/` with `config.json`, `results.jsonl`, and
//! `summary.json`. Change your solver, re-run, then:
//!
//!   cargo run --release --bin e2kit -- compare runs/<A> runs/<B>
//!
//! The solver below is the same greedy baseline as `examples/my_solver.rs`,
//! inlined so this file is self-contained. Swap it for yours (or `use` your own
//! module) and the runner, run directory, and compare tool all keep working.

use e2_kit::{Board, Budget, Instance, Pieces, SolveOutcome, Solver, SweepConfig};

/// The greedy baseline again — replace with your idea. See `my_solver.rs` for
/// the commented version.
struct GreedyRowMajor;

impl Solver for GreedyRowMajor {
    fn name(&self) -> String {
        "greedy-row-major".into()
    }

    fn solve(&mut self, instance: &Instance, start: &Board, budget: Budget) -> SolveOutcome {
        let n = usize::from(instance.width) * usize::from(instance.height);
        let s = usize::from(instance.width);
        let pieces = &instance.pieces;
        let mut board = start.clone();
        let mut used = vec![false; pieces.len()];
        for pos in 0..n {
            if let Some((pid, _)) = board.piece_at(pos) {
                used[pid as usize] = true;
            }
        }
        let mut nodes = 0u64;
        let mut ran_out = false;
        for pos in 0..n {
            if budget.expired() {
                ran_out = true;
                break;
            }
            if !board.is_empty_at(pos) {
                continue;
            }
            let (x, y) = (pos % s, pos / s);
            let want = constraints(&board, pieces, x, y, s);
            let mut best: Option<(u16, u8, u32)> = None;
            for (pid, piece) in pieces.iter() {
                if used[pid as usize] {
                    continue;
                }
                for r in 0..4 {
                    nodes += 1;
                    let e = piece.rotated(r);
                    if let Some(score) = fit(&e, &want) {
                        if best.is_none_or(|(_, _, b)| score > b) {
                            best = Some((pid, r, score));
                        }
                    }
                }
            }
            if let Some((pid, r, _)) = best {
                board.place(pos, pid, r);
                used[pid as usize] = true;
            }
        }
        // Honest outcome: Complete only if the board is actually full (see
        // my_solver.rs for the reasoning); otherwise it's a best-effort Improved.
        let full = (0..n).all(|pos| !board.is_empty_at(pos));
        if full && !ran_out {
            SolveOutcome::complete(board).with_nodes(nodes)
        } else {
            SolveOutcome::improved(board).with_nodes(nodes)
        }
    }
}

fn constraints(board: &Board, pieces: &Pieces, x: usize, y: usize, s: usize) -> [Option<u8>; 4] {
    let at = |cx: usize, cy: usize| -> Option<[u8; 4]> {
        board
            .piece_at(cy * s + cx)
            .map(|(pid, r)| pieces.get(pid).unwrap().rotated(r))
    };
    [
        if y == 0 { Some(0) } else { at(x, y - 1).map(|e| e[2]) },
        if x == s - 1 { Some(0) } else { at(x + 1, y).map(|e| e[3]) },
        if y == s - 1 { Some(0) } else { at(x, y + 1).map(|e| e[0]) },
        if x == 0 { Some(0) } else { at(x - 1, y).map(|e| e[1]) },
    ]
}

fn fit(e: &[u8; 4], want: &[Option<u8>; 4]) -> Option<u32> {
    let mut matched = 0;
    for (side, req) in want.iter().enumerate() {
        if let Some(c) = req {
            if e[side] == *c {
                matched += 1;
            } else {
                return None;
            }
        }
    }
    Some(matched)
}

fn main() {
    // Tiny flag parser: --n, --budget, --pins.
    let raw: Vec<String> = std::env::args().skip(1).collect();
    let mut n = 20u32;
    let mut budget_s = 2.0f64;
    let mut pins = 0u32;
    let mut i = 0;
    while i < raw.len() {
        match raw[i].as_str() {
            "--n" => {
                i += 1;
                n = raw.get(i).and_then(|s| s.parse().ok()).unwrap_or(n);
            }
            "--budget" => {
                i += 1;
                budget_s = raw.get(i).and_then(|s| s.parse().ok()).unwrap_or(budget_s);
            }
            "--pins" => {
                i += 1;
                pins = raw.get(i).and_then(|s| s.parse().ok()).unwrap_or(pins);
            }
            other => {
                eprintln!("unknown flag {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }

    let mut config = SweepConfig::official(1..=n, budget_s);
    config.pins = pins;

    let mut solver = GreedyRowMajor;
    match e2_kit::sweep(&mut solver, &config, "runs") {
        Ok((dir, summary)) => {
            println!(
                "swept {} seeds → mean {:.1}  sd {:.1}  best {}  (n_scored={})",
                summary.n, summary.mean, summary.sd, summary.best, summary.n_scored
            );
            if summary.bounds > 0 || summary.exhausted > 0 {
                println!(
                    "     ({} bound(s), {} exhaustion(s) not counted in the score stats)",
                    summary.bounds, summary.exhausted
                );
            }
            if summary.n_scored < 40 {
                println!(
                    "note: n_scored={} is small — score sd on this puzzle is large, so treat mean\n\
                     differences below ~sd as noise. Sweep --n 40+ before trusting a delta.",
                    summary.n_scored
                );
            }
            println!("run:  {}", dir.path().display());
            println!("compare two runs with:  cargo run --release --bin e2kit -- compare <A> <B>");
        }
        Err(e) => {
            eprintln!("sweep failed: {e}");
            std::process::exit(1);
        }
    }
}
