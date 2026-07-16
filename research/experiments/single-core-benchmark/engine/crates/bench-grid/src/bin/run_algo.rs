//! Unified single-core runner for the native "engine-family" algorithms.
//!
//! A thin dispatcher: it loads a site-schema variant, looks the algorithm up in
//! `bench_grid::algos::registry()`, runs it once for a wall budget on one core,
//! and emits the best board as a bucas `.url` re-scored by the canonical scorer.
//! Each algorithm's actual entry point lives in its own file under `src/algos/`.
//!
//! Usage:
//!   run_algo --puzzle variant.json --algo NAME --seed S --budget-ms 60000 \
//!            --emit out.url [--json out.json]   |   run_algo --list

use std::path::PathBuf;
use std::time::Instant;

use bench_grid::{algos, algos::RunParams, SiteInstance};

fn main() {
    let mut puzzle: Option<PathBuf> = None;
    let mut algo = String::new();
    let mut seed: u64 = 1;
    let mut budget_ms: u64 = 60_000;
    let mut node_budget: u64 = 0; // 0 = wall-time; nonzero = fixed-work (A/B gate)
    let mut emit: Option<PathBuf> = None;
    let mut json_out: Option<PathBuf> = None;

    let args: Vec<String> = std::env::args().collect();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--puzzle" => { puzzle = Some(PathBuf::from(&args[i + 1])); i += 2; }
            "--algo" => { algo = args[i + 1].clone(); i += 2; }
            "--seed" => { seed = args[i + 1].parse().unwrap(); i += 2; }
            "--budget-ms" => { budget_ms = args[i + 1].parse().unwrap(); i += 2; }
            "--node-budget" => { node_budget = args[i + 1].parse().unwrap(); i += 2; }
            "--emit" => { emit = Some(PathBuf::from(&args[i + 1])); i += 2; }
            "--json" => { json_out = Some(PathBuf::from(&args[i + 1])); i += 2; }
            "--list" => {
                for (name, _) in algos::registry() {
                    println!("{name}");
                }
                return;
            }
            other => { eprintln!("unknown arg {other}"); std::process::exit(2); }
        }
    }

    let puzzle_path = puzzle.expect("--puzzle required");
    let raw = std::fs::read_to_string(&puzzle_path).expect("read puzzle json");
    let inst: SiteInstance = serde_json::from_str(&raw).expect("parse site instance");

    let algo_fn = algos::lookup(&algo).unwrap_or_else(|| {
        eprintln!("unknown algo {algo}; try --list");
        std::process::exit(2);
    });

    let t0 = Instant::now();
    let run = algo_fn(&inst, RunParams { seed, budget_ms, node_budget });
    let elapsed = t0.elapsed();

    // Board fingerprint for the bit-identical A/B optimization gate. A pure
    // speedup must reproduce this hash on a fixed (seed, node_budget) run.
    let board_hash = fnv1a(&run.board);
    let out = inst.finish(run.board);
    let secs = elapsed.as_secs_f64().max(1e-9);
    let nps = (run.nodes as f64 / secs) as u64;

    if let Some(p) = &emit {
        // Canonical board document (one .json with an eternity2.dev viewer URL).
        inst.to_doc(&out.board, board_hash)
            .write_json(p)
            .expect("write board json");
    }
    if let Some(p) = &json_out {
        std::fs::write(p, serde_json::to_string(&out).unwrap()).expect("write json");
    }

    println!(
        "RESULT algo={} seed={} node_budget={} score={} elapsed_ms={} nodes={} \
         backtracks={} nps={} nps_unit={} board_hash={:016x} url={}",
        algo,
        seed,
        node_budget,
        out.score,
        elapsed.as_millis(),
        run.nodes,
        run.backtracks,
        nps,
        run.nps_unit,
        board_hash,
        emit.as_ref().map(|p| p.display().to_string()).unwrap_or_default()
    );
}

/// FNV-1a over the board cells — a stable fingerprint for the A/B gate.
fn fnv1a(board: &[i32]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &c in board {
        for b in c.to_le_bytes() {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    h
}
