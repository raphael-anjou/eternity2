//! Native-only stats generator for the website's Algorithms page.
//!
//! Emits JSON to stdout:
//!   - difficulty sweep: attempts-to-solve for generated n×n puzzles across
//!     sizes and color counts (multiple seeds, median/min/max, censored at a
//!     work cap),
//!   - path comparison: same puzzle, different visit orders,
//!   - search-space sizes: log10 of the raw placement space per board size.
//!
//! Usage: cargo run --release --bin stats > ../web/src/data/difficulty.json

use eternity2_engine::{build_path, generate, Solver, Status};

const WORK_CAP: u64 = 30_000_000; // attempts per instance before censoring
const SEEDS: u32 = 10;

struct RunResult {
    attempts: f64,
    solved: bool,
}

fn solve_instance(size: u8, colors: u8, seed: u32, path_kind: &str) -> RunResult {
    let puzzle = generate(size, colors, seed);
    let path = build_path(path_kind, size, size, 0).expect("path kind");
    let mut solver = Solver::new(&puzzle, &path, true, false, 0).expect("solver");
    loop {
        let r = solver.step(2_000_000);
        if r.status == Status::Solved {
            return RunResult { attempts: r.attempts, solved: true };
        }
        if r.status == Status::Exhausted {
            // Shouldn't happen (generator guarantees solvability) but stay honest.
            return RunResult { attempts: r.attempts, solved: false };
        }
        if r.attempts >= WORK_CAP as f64 {
            return RunResult { attempts: r.attempts, solved: false };
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
    let mut out = String::from("{\n");

    // ---- difficulty sweep -------------------------------------------------
    eprintln!("difficulty sweep...");
    out.push_str("  \"difficulty\": [\n");
    let mut rows: Vec<String> = Vec::new();
    for size in 3u8..=8 {
        let max_c = eternity2_engine::generator::max_colors(size).min(14);
        for colors in 2..=max_c {
            let mut attempts: Vec<f64> = Vec::new();
            let mut censored = 0u32;
            for seed in 1..=SEEDS {
                let r = solve_instance(size, colors, seed, "row-major");
                if !r.solved {
                    censored += 1;
                }
                attempts.push(r.attempts);
            }
            attempts.sort_by(|a, b| a.partial_cmp(b).unwrap());
            eprintln!(
                "  size {size} colors {colors}: median {:.0} (censored {censored})",
                median(&attempts)
            );
            rows.push(format!(
                "    {{\"size\": {size}, \"colors\": {colors}, \"median\": {:.0}, \"min\": {:.0}, \"max\": {:.0}, \"censored\": {censored}, \"seeds\": {SEEDS}}}",
                median(&attempts),
                attempts[0],
                attempts[attempts.len() - 1]
            ));
        }
    }
    out.push_str(&rows.join(",\n"));
    out.push_str("\n  ],\n");

    // ---- path comparison --------------------------------------------------
    eprintln!("path comparison...");
    out.push_str("  \"paths\": [\n");
    let mut rows: Vec<String> = Vec::new();
    for kind in eternity2_engine::PATH_KINDS {
        let mut attempts: Vec<f64> = Vec::new();
        let mut censored = 0u32;
        for seed in 1..=SEEDS {
            let puzzle = generate(6, 6, seed);
            let path = build_path(kind, 6, 6, 7).expect("path kind");
            let mut solver = Solver::new(&puzzle, &path, true, false, 0).expect("solver");
            let result = loop {
                let r = solver.step(2_000_000);
                if r.status == Status::Solved {
                    break RunResult { attempts: r.attempts, solved: true };
                }
                if r.status == Status::Exhausted || r.attempts >= WORK_CAP as f64 {
                    break RunResult { attempts: r.attempts, solved: false };
                }
            };
            if !result.solved {
                censored += 1;
            }
            attempts.push(result.attempts);
        }
        attempts.sort_by(|a, b| a.partial_cmp(b).unwrap());
        eprintln!("  {kind}: median {:.0} (censored {censored})", median(&attempts));
        rows.push(format!(
            "    {{\"kind\": \"{kind}\", \"size\": 6, \"colors\": 6, \"median\": {:.0}, \"min\": {:.0}, \"max\": {:.0}, \"censored\": {censored}, \"seeds\": {SEEDS}}}",
            median(&attempts),
            attempts[0],
            attempts[attempts.len() - 1]
        ));
    }
    out.push_str(&rows.join(",\n"));
    out.push_str("\n  ],\n");

    // ---- search-space sizes ----------------------------------------------
    // Raw placement space for an n×n puzzle: n²! orderings × 4^(n²) rotations.
    // Refined (corner/edge/interior classes, the way real solvers think):
    // 4!·(4(n-2))!·((n-2)²)!·4^((n-2)²); rim rotations are forced.
    out.push_str("  \"searchSpace\": [\n");
    let ln10 = std::f64::consts::LN_10;
    let mut rows: Vec<String> = Vec::new();
    for size in 2u32..=16 {
        let n = size * size;
        let naive = (ln_factorial(n) + f64::from(n) * 4f64.ln()) / ln10;
        let inner = (size - 2) * (size - 2);
        let rim_edges = 4 * (size - 2);
        let refined = (ln_factorial(4)
            + ln_factorial(rim_edges)
            + ln_factorial(inner)
            + f64::from(inner) * 4f64.ln())
            / ln10;
        rows.push(format!(
            "    {{\"size\": {size}, \"log10Naive\": {naive:.1}, \"log10Refined\": {refined:.1}}}"
        ));
    }
    out.push_str(&rows.join(",\n"));
    out.push_str("\n  ]\n}\n");

    println!("{out}");
}
