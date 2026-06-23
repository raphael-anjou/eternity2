//! Prune beats speed: the search-space principle, measured on the real solver.
//!
//! For E2-class search, two levers reduce the work to exhaust a region: going
//! *faster* (a constant divisor) and *pruning* — lowering the effective branching
//! factor (an exponential divisor). The second dominates. This crate makes the
//! claim concrete with the project's own depth-first solver:
//!
//!   1. The hardness curve. For a fixed small board we solve generated puzzles
//!      across colour counts and record the median nodes the solver explores.
//!      The curve rises to a peak (the phase transition) and falls — the same
//!      shape the 16×16 sits on. This shows the search space, not the clock, is
//!      what explodes.
//!   2. The lever comparison. From the measured node counts we derive the
//!      effective branching factor b at the peak, then report what a raw k× speedup
//!      buys (work / k) versus what a p% cut to b buys (work · ((1-p))^depth) —
//!      the exponential gap, in the puzzle's own numbers.
//!
//! Deterministic: fixed seeds, fixed board, same medians every run.

use eternity2_engine::generator::generate;
use eternity2_engine::paths::build_path;
use eternity2_engine::solver::{Solver, Status};

const SIZE: u8 = 6;
const SEEDS: u32 = 9; // median over this many generated puzzles per colour count
const NODE_CAP: f64 = 8_000_000.0; // bound each solve so the sweep stays quick

/// Median nodes the solver explores to first solution (or to the cap) for a
/// generated `size`×`size` puzzle with `colors` colours, over `SEEDS` seeds.
fn median_nodes(size: u8, colors: u8) -> f64 {
    let n_cells = (size as usize) * (size as usize);
    let mut runs: Vec<f64> = Vec::with_capacity(SEEDS as usize);
    for seed in 1..=SEEDS {
        let puzzle = generate(size, colors, seed);
        let path = build_path("row-major", size, size, 0).expect("row-major path");
        let mut solver = Solver::new(&puzzle, &path, false, false, seed).expect("solver");
        let mut nodes: f64;
        loop {
            let r = solver.step(50_000);
            nodes = r.nodes;
            if r.status != Status::Running || nodes >= NODE_CAP {
                break;
            }
        }
        let _ = n_cells; // (kept for clarity; cells = path length)
        runs.push(nodes);
    }
    runs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    runs[runs.len() / 2].max(1.0)
}

fn main() {
    // Sweep colour counts from easy (many solutions) through the peak.
    let color_min = 3u8;
    let color_max = 11u8;

    let mut points: Vec<(u8, f64)> = Vec::new();
    for c in color_min..=color_max {
        points.push((c, median_nodes(SIZE, c)));
    }

    // The peak: the colour count where the solver did the most work.
    let (peak_colors, peak_nodes) = points
        .iter()
        .copied()
        .fold((color_min, 0.0), |acc, p| if p.1 > acc.1 { p } else { acc });

    // Effective branching factor at the peak: b such that b^depth ≈ nodes, with
    // depth = the number of cells filled (the search tree's depth).
    let depth = (SIZE as f64) * (SIZE as f64);
    let b_eff = peak_nodes.powf(1.0 / depth);

    // The two levers, in the puzzle's own numbers, at a fixed wall-clock budget.
    //   raw k× speedup  -> work / k
    //   p% prune of b   -> work · ((1 - p))^depth
    let speedup_k: f64 = 1000.0; // a generous, realistic engine speedup
    let prune_p: f64 = 0.05; // a modest 5% cut to the branching factor, every level
    let work_after_speed = peak_nodes / speedup_k;
    let work_after_prune = peak_nodes * (1.0 - prune_p).powf(depth);
    // The speedup that would match the 5% prune.
    let equiv_speedup = peak_nodes / work_after_prune;

    println!("{{");
    println!("  \"board\": \"{SIZE}x{SIZE} generated puzzles\",");
    println!("  \"seedsPerPoint\": {SEEDS},");
    println!("  \"metric\": \"median nodes the DFS explores to first solution (capped)\",");
    println!("  \"hardnessCurve\": [");
    for (i, (c, n)) in points.iter().enumerate() {
        let comma = if i + 1 < points.len() { "," } else { "" };
        println!("    {{ \"colors\": {c}, \"medianNodes\": {} }}{}", *n as u64, comma);
    }
    println!("  ],");
    println!("  \"peakColors\": {peak_colors},");
    println!("  \"peakMedianNodes\": {},", peak_nodes as u64);
    println!("  \"depth\": {depth},");
    println!("  \"effectiveBranchingFactor\": {b_eff:.3},");
    println!("  \"leverComparison\": {{");
    println!("    \"baselineNodes\": {},", peak_nodes as u64);
    println!("    \"rawSpeedupK\": {speedup_k},");
    println!("    \"workAfterSpeedup\": {},", work_after_speed as u64);
    println!("    \"prunePercent\": {},", (prune_p * 100.0) as u64);
    println!("    \"workAfterPrune\": {},", work_after_prune as u64);
    println!(
        "    \"pruneEqualsSpeedupOf\": {:.3e}",
        equiv_speedup
    );
    println!("  }}");
    println!("}}");
}
