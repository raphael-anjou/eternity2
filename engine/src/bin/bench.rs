//! Single-core throughput benchmark, native (not WASM).
//! Usage: cargo run --release --bin bench
//!
//! Reports nodes/sec (placements, the community metric) measured on a stream
//! of generated puzzles that solve continuously, plus the raw fit-check rate
//! on the official 16x16 board for reference. On the official board the
//! search thrashes near the top and completes almost no net nodes, so it is
//! not a good place to measure nodes/sec.

use eternity2_engine::{build_path, generate, official_puzzle, Solver, Status};
use std::time::Instant;

fn main() {
    // ---- nodes/sec: solve a stream of 6x6 puzzles back to back -----------
    let path6 = build_path("snake", 6, 6, 0).unwrap();
    // warm up
    {
        let p = generate(6, 6, 0);
        Solver::new(&p, &path6, false, false, 0).unwrap().step(2_000_000);
    }

    let mut total_nodes = 0.0f64;
    let mut seed = 1u32;
    let t0 = Instant::now();
    while t0.elapsed().as_secs_f64() < 3.0 {
        let p = generate(6, 6, seed);
        seed += 1;
        let mut solver = Solver::new(&p, &path6, false, false, 0).unwrap();
        loop {
            let r = solver.step(5_000_000);
            if r.status != Status::Running {
                total_nodes += r.nodes;
                break;
            }
        }
    }
    let secs = t0.elapsed().as_secs_f64();
    let nodes_per_sec = total_nodes / secs;

    // ---- fit-check rate on the official 16x16 board (reference) ----------
    let official = official_puzzle();
    let path16 = build_path("row-major", 16, 16, 0).unwrap();
    let mut solver = Solver::new(&official, &path16, true, false, 0).unwrap();
    solver.step(2_000_000); // warm
    let c0 = Instant::now();
    let mut last = solver.report();
    while c0.elapsed().as_secs_f64() < 2.0 {
        last = solver.step(2_000_000);
    }
    let check_rate = last.attempts / c0.elapsed().as_secs_f64();

    println!("native single-core (release):");
    println!(
        "  {:.1}M nodes/s   (placements; measured over {:.0} solved 6x6 puzzles in {secs:.1}s)",
        nodes_per_sec / 1e6,
        (seed - 1) as f64,
    );
    println!(
        "  {:.0}M fit checks/s   (candidate tests, on the official 16x16 board)",
        check_rate / 1e6,
    );
}
