//! Single-core throughput benchmark: the exact same measurement the website's
//! TimeToSolve component runs in the browser (official puzzle, row-major,
//! hints on), but native. Usage: cargo run --release --bin bench

use eternity2_engine::{build_path, official_puzzle, Solver};
use std::time::Instant;

fn main() {
    let puzzle = official_puzzle();
    let path = build_path("row-major", 16, 16, 0).unwrap();
    let mut solver = Solver::new(&puzzle, &path, true, false, 0).unwrap();

    // Warm up caches like the JIT-warmed browser run.
    solver.step(2_000_000);

    let t0 = Instant::now();
    let mut last = solver.report();
    while t0.elapsed().as_secs_f64() < 3.0 {
        last = solver.step(2_000_000);
    }
    let secs = t0.elapsed().as_secs_f64();
    println!(
        "native single-core: {:.1}M fit checks/s ({:.1}M steps/s) over {secs:.2}s",
        last.attempts / secs / 1e6,
        (last.nodes + last.backtracks) / secs / 1e6,
    );
}
