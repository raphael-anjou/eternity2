// Smoke benchmark: measure SA performance on representative puzzles.
// Marked #[ignore] so cargo test doesn't run it by default; invoke with
//   cargo test -p eternity2-localsearch --release -- --ignored --nocapture

use eternity2_generator::{generate, GeneratorConfig};
use eternity2_localsearch::{run_sa, SaConfig};

#[test]
#[ignore = "perf measurement; run with --ignored --nocapture"]
fn sa_8x8_60s() {
    let puzzle = generate(GeneratorConfig {
        size: 8,
        interior_colors: 7,
        seed: 42,
    })
    .expect("gen");
    let cfg = SaConfig {
        time_budget_ms: 60_000,
        ..Default::default()
    };
    let out = run_sa(&puzzle, &cfg);
    println!(
        "size=8 colors=7 budget=60s -> {} / {} edges matched ({}%); iters={}",
        out.best_score,
        out.total_edges,
        (out.best_score * 100) / out.total_edges,
        out.iterations,
    );
}

#[test]
#[ignore = "perf measurement; run with --ignored --nocapture"]
fn sa_8x8_5s() {
    let puzzle = generate(GeneratorConfig {
        size: 8,
        interior_colors: 7,
        seed: 42,
    })
    .expect("gen");
    let cfg = SaConfig {
        time_budget_ms: 5_000,
        ..Default::default()
    };
    let out = run_sa(&puzzle, &cfg);
    println!(
        "size=8 colors=7 budget=5s -> {} / {} edges matched ({}%); iters={}; elapsed={}us",
        out.best_score,
        out.total_edges,
        (out.best_score * 100) / out.total_edges,
        out.iterations,
        out.elapsed_us,
    );
    // Sanity bound: 8×8 has 7+7+8+8 = 30 + 30 = wait, let me recompute.
    // For w=h=8: (w-1)*h + w*(h-1) = 7*8 + 8*7 = 56 + 56 = 112 interior edges.
    assert_eq!(out.total_edges, 112);
}

#[test]
#[ignore = "perf measurement; run with --ignored --nocapture"]
fn sa_10x10_5s() {
    let puzzle = generate(GeneratorConfig {
        size: 10,
        interior_colors: 10,
        seed: 7,
    })
    .expect("gen");
    let cfg = SaConfig {
        time_budget_ms: 5_000,
        ..Default::default()
    };
    let out = run_sa(&puzzle, &cfg);
    println!(
        "size=10 colors=10 budget=5s -> {} / {} edges matched ({}%); iters={}",
        out.best_score,
        out.total_edges,
        (out.best_score * 100) / out.total_edges,
        out.iterations,
    );
}

#[test]
#[ignore = "perf measurement; run with --ignored --nocapture"]
fn sa_16x16_15s() {
    let puzzle = generate(GeneratorConfig {
        size: 16,
        interior_colors: 22,
        seed: 999,
    })
    .expect("gen");
    let cfg = SaConfig {
        time_budget_ms: 15_000,
        ..Default::default()
    };
    let out = run_sa(&puzzle, &cfg);
    println!(
        "size=16 colors=22 budget=15s -> {} / {} edges matched ({}%); iters={}",
        out.best_score,
        out.total_edges,
        (out.best_score * 100) / out.total_edges,
        out.iterations,
    );
}
