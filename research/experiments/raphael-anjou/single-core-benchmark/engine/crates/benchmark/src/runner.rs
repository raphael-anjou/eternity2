use std::path::PathBuf;
use std::time::Instant;

use eternity2_events::{BufferSink, EventBody};
use eternity2_solver_engine::EngineSolver;
use eternity2_solver_naive::NaiveSolver;
use eternity2_solver_trait::{SolveOpts, SolveOutcome, Solver};
use serde::Serialize;

use crate::loader;

#[derive(Debug, Clone, Copy)]
pub struct ProfileSpec {
    pub solver_id: &'static str,
    pub heuristic_profile: &'static str,
}

pub const PROFILES: &[ProfileSpec] = &[
    ProfileSpec { solver_id: "naive",  heuristic_profile: "row_by_row" },
    ProfileSpec { solver_id: "naive",  heuristic_profile: "spiral" },
    ProfileSpec { solver_id: "engine", heuristic_profile: "border_first_lcv" },
    ProfileSpec { solver_id: "engine", heuristic_profile: "rare_color_first" },
    ProfileSpec { solver_id: "engine", heuristic_profile: "border_first_random" },
    ProfileSpec { solver_id: "engine", heuristic_profile: "border_first_parity" },
    ProfileSpec { solver_id: "engine", heuristic_profile: "border_first_full" },
    ProfileSpec { solver_id: "engine", heuristic_profile: "border_first_gacolor" },
    ProfileSpec { solver_id: "engine", heuristic_profile: "border_first_lcv_par" },
    ProfileSpec { solver_id: "engine", heuristic_profile: "border_first_full_par" },
    ProfileSpec { solver_id: "engine", heuristic_profile: "border_first_gacolor_par" },
    ProfileSpec { solver_id: "engine", heuristic_profile: "chess_gacolor" },
    ProfileSpec { solver_id: "engine", heuristic_profile: "chess_gacolor_par" },
    ProfileSpec { solver_id: "engine", heuristic_profile: "gacolor_symbreak" },
    ProfileSpec { solver_id: "engine", heuristic_profile: "gacolor_symbreak_par" },
    ProfileSpec { solver_id: "engine", heuristic_profile: "gacolor_ac3" },
    ProfileSpec { solver_id: "engine", heuristic_profile: "gacolor_ac3_par" },
    ProfileSpec { solver_id: "engine", heuristic_profile: "chess_gacolor_ac3" },
    ProfileSpec { solver_id: "engine", heuristic_profile: "gacolor_ac3_lcv" },
    ProfileSpec { solver_id: "engine", heuristic_profile: "gacolor_ac3_lcv_par" },
];

pub fn instantiate(spec: &ProfileSpec) -> Option<Box<dyn Solver>> {
    match (spec.solver_id, spec.heuristic_profile) {
        ("naive", "row_by_row") => Some(Box::new(NaiveSolver::row_by_row())),
        ("naive", "spiral") => Some(Box::new(NaiveSolver::spiral())),
        ("engine", "border_first_lcv") => Some(Box::new(EngineSolver::border_first_lcv())),
        ("engine", "rare_color_first") => Some(Box::new(EngineSolver::rare_color_first())),
        ("engine", "border_first_random") => Some(Box::new(EngineSolver::border_first_random())),
        ("engine", "border_first_parity") => Some(Box::new(EngineSolver::border_first_parity())),
        ("engine", "border_first_full") => Some(Box::new(EngineSolver::border_first_full())),
        ("engine", "border_first_gacolor") => Some(Box::new(EngineSolver::border_first_gacolor())),
        ("engine", "border_first_lcv_par") => Some(Box::new(EngineSolver::border_first_lcv_par())),
        ("engine", "border_first_full_par") => Some(Box::new(EngineSolver::border_first_full_par())),
        ("engine", "border_first_gacolor_par") => Some(Box::new(EngineSolver::border_first_gacolor_par())),
        ("engine", "chess_gacolor") => Some(Box::new(EngineSolver::chess_gacolor())),
        ("engine", "chess_gacolor_par") => Some(Box::new(EngineSolver::chess_gacolor_par())),
        ("engine", "gacolor_symbreak") => Some(Box::new(EngineSolver::gacolor_symbreak())),
        ("engine", "gacolor_symbreak_par") => Some(Box::new(EngineSolver::gacolor_symbreak_par())),
        ("engine", "gacolor_ac3") => Some(Box::new(EngineSolver::gacolor_ac3())),
        ("engine", "gacolor_ac3_par") => Some(Box::new(EngineSolver::gacolor_ac3_par())),
        ("engine", "chess_gacolor_ac3") => Some(Box::new(EngineSolver::chess_gacolor_ac3())),
        ("engine", "gacolor_ac3_lcv") => Some(Box::new(EngineSolver::gacolor_ac3_lcv())),
        ("engine", "gacolor_ac3_lcv_par") => Some(Box::new(EngineSolver::gacolor_ac3_lcv_par())),
        _ => None,
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PuzzleSpec {
    pub file: String,
    pub size: u32,
    pub colors: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct RunResult {
    pub solved: bool,
    pub outcome: String,             // "solved" | "exhausted" | "timed_out" | "cancelled" | "error"
    pub time_us: u128,
    pub nodes: u64,
    pub backtracks: u64,
    pub propagations: u64,
    pub domain_wipeouts: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProfileResult {
    pub solver_id: String,
    pub heuristic_profile: String,
    pub runs: Vec<RunResult>,
    pub median_time_us: u128,
    pub solved_count: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct PuzzleReport {
    pub puzzle: PuzzleSpec,
    pub profiles: Vec<ProfileResult>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BenchReport {
    pub cpu_brand: String,
    pub n_cores_physical: usize,
    pub n_cores_logical: usize,
    pub rayon_threads: usize,
    pub time_budget_ms: u64,
    pub runs_per_cell: u32,
    pub git_branch: String,
    pub puzzles: Vec<PuzzleReport>,
}

pub fn run_one(
    puzzle: &eternity2_core::Puzzle,
    spec: &ProfileSpec,
    time_budget_ms: u64,
) -> RunResult {
    let Some(mut solver) = instantiate(spec) else {
        return RunResult {
            solved: false,
            outcome: "error".into(),
            time_us: 0, nodes: 0, backtracks: 0, propagations: 0, domain_wipeouts: 0,
        };
    };
    let mut sink = BufferSink::new();
    let mut opts = SolveOpts::default();
    opts.time_budget_ms = time_budget_ms;
    let t0 = Instant::now();
    let outcome = solver.solve(puzzle, &opts, &mut sink);
    let elapsed_us = t0.elapsed().as_micros();

    let (solved, outcome_label) = match outcome {
        SolveOutcome::Solved(_) => (true, "solved"),
        SolveOutcome::AllSolutions(b) => (!b.is_empty(), "solved"),
        SolveOutcome::Exhausted => (false, "exhausted"),
        SolveOutcome::TimedOut { .. } => (false, "timed_out"),
        SolveOutcome::Cancelled { .. } => (false, "cancelled"),
        SolveOutcome::Error(_) => (false, "error"),
    };

    // Final stats from the last Stats/Solved/Exhausted/TimedOut event.
    let mut nodes = 0u64;
    let mut backtracks = 0u64;
    let mut propagations = 0u64;
    let mut domain_wipeouts = 0u64;
    for ev in sink.events.iter().rev() {
        match &ev.body {
            EventBody::Solved { final_stats, .. }
            | EventBody::TimedOut { final_stats, .. }
            | EventBody::Cancelled { final_stats, .. }
            | EventBody::Exhausted { final_stats, .. } => {
                nodes = final_stats.nodes;
                backtracks = final_stats.backtracks;
                propagations = final_stats.propagations;
                domain_wipeouts = final_stats.domain_wipeouts;
                break;
            }
            EventBody::Stats(s) => {
                nodes = s.nodes;
                backtracks = s.backtracks;
                propagations = s.propagations;
                domain_wipeouts = s.domain_wipeouts;
                break;
            }
            _ => {}
        }
    }

    RunResult {
        solved,
        outcome: outcome_label.into(),
        time_us: elapsed_us,
        nodes, backtracks, propagations, domain_wipeouts,
    }
}

pub fn median(mut xs: Vec<u128>) -> u128 {
    if xs.is_empty() { return 0; }
    xs.sort_unstable();
    let m = xs.len() / 2;
    if xs.len() % 2 == 1 { xs[m] } else { (xs[m - 1] + xs[m]) / 2 }
}

pub fn run_profile(
    puzzle: &eternity2_core::Puzzle,
    spec: &ProfileSpec,
    time_budget_ms: u64,
    runs: u32,
) -> ProfileResult {
    let mut results = Vec::with_capacity(runs as usize);
    for _ in 0..runs {
        results.push(run_one(puzzle, spec, time_budget_ms));
    }
    let times: Vec<u128> = results.iter().map(|r| r.time_us).collect();
    let med = median(times);
    let solved_count = results.iter().filter(|r| r.solved).count() as u32;
    ProfileResult {
        solver_id: spec.solver_id.into(),
        heuristic_profile: spec.heuristic_profile.into(),
        runs: results,
        median_time_us: med,
        solved_count,
    }
}

pub fn list_puzzles(dir: &PathBuf) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(dir).unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("csv"))
        .filter(|p| {
            // Filter out `results.csv` if it lives in the puzzles dir.
            p.file_name().and_then(|n| n.to_str())
                .map(|n| n != "results.csv")
                .unwrap_or(true)
        })
        .collect();
    paths.sort();
    paths
}

pub fn puzzle_spec(path: &std::path::Path, puzzle: &eternity2_core::Puzzle) -> PuzzleSpec {
    PuzzleSpec {
        file: path.file_name().and_then(|n| n.to_str()).unwrap_or("?").into(),
        size: puzzle.width,
        colors: puzzle.color_count - 1,
    }
}

pub fn parse_size_colors(filename: &str) -> Option<(u32, u32)> {
    // Files are named size_N_colors_M_hash.csv.
    let parts: Vec<&str> = filename.split('_').collect();
    if parts.len() < 4 { return None; }
    let size: u32 = parts[1].parse().ok()?;
    let colors: u32 = parts[3].parse().ok()?;
    Some((size, colors))
}

pub fn cpu_info() -> (String, usize, usize) {
    let brand = std::process::Command::new("sysctl")
        .args(["-n", "machdep.cpu.brand_string"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".into());
    let physical = std::process::Command::new("sysctl")
        .args(["-n", "hw.physicalcpu"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0);
    let logical = std::process::Command::new("sysctl")
        .args(["-n", "hw.logicalcpu"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0);
    (brand, physical, logical)
}

pub fn git_branch() -> String {
    std::process::Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".into())
}

pub fn _force_loader_use() { let _ = loader::load_puzzle::<&str>; }
