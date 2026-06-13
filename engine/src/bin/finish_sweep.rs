//! One-off: finish the difficulty sweep that the uncapped run couldn't.
//! Reads the 674 salvaged results + 86 missing jobs, runs the missing ones
//! with a 3-billion-node cap (the rest are unbounded for the naive engine),
//! merges, and writes web/src/data/difficulty.json.
//!
//! Usage: cargo run --release --bin finish_sweep

use eternity2_engine::{build_path, generate, Solver, Status};
use std::collections::HashMap;
use std::fs;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

const CAP: f64 = 3_000_000_000.0; // nodes before censoring
const THREADS: usize = 8;
const SEEDS: u32 = 10;

fn solve_capped(size: u8, colors: u8, seed: u32) -> (f64, bool) {
    let puzzle = generate(size, colors, seed);
    let path = build_path("row-major", size, size, 0).unwrap();
    let mut solver = Solver::new(&puzzle, &path, true, false, 0).unwrap();
    loop {
        let r = solver.step(20_000_000);
        if r.status == Status::Solved {
            return (r.nodes, false);
        }
        if r.status == Status::Exhausted || r.nodes + r.backtracks >= CAP {
            return (r.nodes, true); // censored / unsolved
        }
    }
}

fn median(sorted: &[f64]) -> f64 {
    let n = sorted.len();
    if n % 2 == 1 {
        sorted[n / 2]
    } else {
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
    }
}

fn ln_factorial(n: u32) -> f64 {
    (2..=n).map(|k| f64::from(k).ln()).sum()
}

fn main() {
    // ---- load the 674 salvaged results: size,colors,seed,nodes -----------
    // (nodes here are uncapped completions, all < CAP except none — every
    //  salvaged job actually finished, so none are censored.)
    let mut results: HashMap<(u8, u8, u32), (f64, bool)> = HashMap::new();
    for line in fs::read_to_string("/tmp/sweep_completed.csv").unwrap().lines() {
        let p: Vec<&str> = line.split(',').collect();
        let key = (p[0].parse().unwrap(), p[1].parse().unwrap(), p[2].parse().unwrap());
        // Clamp salvaged completions to the same cap as the capped re-run, so
        // every "too hard" seed reports the same ceiling (honest flat plateau,
        // not a spike where a few giants happened to finish overnight).
        let nodes: f64 = p[3].parse().unwrap();
        results.insert(key, (nodes.min(CAP), nodes >= CAP));
    }
    eprintln!("loaded {} completed results", results.len());

    // ---- run the 86 missing jobs, capped, in parallel --------------------
    let missing: Vec<(u8, u8, u32)> = fs::read_to_string("/tmp/sweep_missing.csv")
        .unwrap()
        .lines()
        .map(|l| {
            let p: Vec<&str> = l.split(',').collect();
            (p[0].parse().unwrap(), p[1].parse().unwrap(), p[2].parse().unwrap())
        })
        .collect();
    eprintln!("running {} missing jobs (cap {CAP:.0} nodes)...", missing.len());

    let next = AtomicUsize::new(0);
    let out = Mutex::new(Vec::<((u8, u8, u32), (f64, bool))>::new());
    std::thread::scope(|scope| {
        for _ in 0..THREADS {
            scope.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::Relaxed);
                if i >= missing.len() {
                    break;
                }
                let (s, c, seed) = missing[i];
                let t0 = std::time::Instant::now();
                let (nodes, censored) = solve_capped(s, c, seed);
                eprintln!(
                    "  {s}x{s} {c}c seed{seed}: {nodes:.0} nodes{} in {:.1}s",
                    if censored { " (CENSORED)" } else { "" },
                    t0.elapsed().as_secs_f64()
                );
                out.lock().unwrap().push(((s, c, seed), (nodes, censored)));
            });
        }
    });
    for (k, v) in out.into_inner().unwrap() {
        results.insert(k, v);
    }

    // ---- emit difficulty.json --------------------------------------------
    let mut s = String::from("{\n  \"difficulty\": [\n");
    let mut rows = Vec::new();
    for size in 3u8..=8 {
        let max_c = eternity2_engine::generator::max_colors(size).min(14);
        for colors in 2..=max_c {
            let mut nodes = Vec::new();
            let mut censored = 0u32;
            for seed in 1..=SEEDS {
                let (n, cens) = results[&(size, colors, seed)];
                nodes.push(n);
                if cens {
                    censored += 1;
                }
            }
            nodes.sort_by(|a, b| a.partial_cmp(b).unwrap());
            rows.push(format!(
                "    {{\"size\": {size}, \"colors\": {colors}, \"median\": {:.0}, \"min\": {:.0}, \"max\": {:.0}, \"censored\": {censored}, \"seeds\": {SEEDS}}}",
                median(&nodes), nodes[0], nodes[nodes.len() - 1]
            ));
        }
    }
    s.push_str(&rows.join(",\n"));
    s.push_str("\n  ],\n");

    // paths section: keep the existing (already-valid) data by re-reading the
    // current difficulty.json's paths + searchSpace blocks unchanged.
    let existing = fs::read_to_string("../web/src/data/difficulty.json").unwrap();
    let paths_start = existing.find("\"paths\"").unwrap();
    s.push_str("  ");
    s.push_str(&existing[paths_start..]);

    fs::write("../web/src/data/difficulty.json", &s).unwrap();
    let _ = ln_factorial; // (searchSpace already present in existing file)
    eprintln!("wrote difficulty.json");
}
