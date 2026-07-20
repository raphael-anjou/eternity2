//! The sweep runner: take a [`Solver`] and a grid of seeds, run every board
//! single-core, and write a reproducible run directory.
//!
//! This is the loop built for "many many iterations": change only your solver,
//! re-run `sweep`, and compare the new run directory against the old one. The
//! runner never touches your algorithm — it generates the boards, applies pins,
//! times each solve, re-scores canonically, and records the lot.

use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::run::{CellResult, RunConfig, RunDir, Summary};
use crate::solver::{Budget, Solver};
use crate::{generator, instance_from_generated, pin_solution_hints};

/// What to sweep: the board shape, the seeds, the pins, and the per-board
/// budget. Build one and hand it to [`sweep`] with your solver.
#[derive(Debug, Clone)]
pub struct SweepConfig {
    /// Board side length. Use 16 for the official puzzle.
    pub size: u8,
    /// Interior colour count. Use 22 for official-shaped boards.
    pub colors: u8,
    /// Generate with frame-restricted colours (the real-Eternity-II look).
    pub framed: bool,
    /// The generation seeds to sweep. Each yields one distinct board.
    pub seeds: Vec<u32>,
    /// How many solution cells to pin as hints on each board (0 = none).
    pub pins: u32,
    /// Per-board wall-clock budget, seconds.
    pub budget_s: f64,
}

impl SweepConfig {
    /// A sensible default sweep over the official 16×16/22-colour board: the
    /// given seed range, no pins, `budget_s` per board.
    #[must_use]
    pub fn official(seeds: impl IntoIterator<Item = u32>, budget_s: f64) -> Self {
        Self {
            size: crate::OFFICIAL_SIZE,
            colors: crate::OFFICIAL_COLORS,
            framed: true,
            seeds: seeds.into_iter().collect(),
            pins: 0,
            budget_s,
        }
    }
}

/// Run `solver` over every seed in `config`, writing a run directory under
/// `runs_root` and returning its [`Summary`].
///
/// The run directory is named `<solver>-<config-digest>-<stamp>` so repeated
/// runs of the same solver+config sit side by side and are easy to diff. The
/// generated boards themselves are fully determined by the seeds, so two runs of
/// an identical config produce identical per-cell scores — only the wall-clock
/// timings differ.
///
/// # Errors
/// Propagates any IO error from creating or writing the run directory.
pub fn sweep(
    solver: &mut impl Solver,
    config: &SweepConfig,
    runs_root: impl AsRef<Path>,
) -> std::io::Result<(RunDir, Summary)> {
    let mut cells = Vec::with_capacity(config.seeds.len());

    for &seed in &config.seeds {
        // A genuine (scrambled) puzzle for this seed.
        let puzzle = generator::generate_framed(config.size, config.colors, seed, config.framed);
        let name = format!("gen-{}x{}-c{}-s{}", config.size, config.size, config.colors, seed);
        let mut instance = instance_from_generated(&name, &puzzle);
        if config.pins > 0 {
            instance = pin_solution_hints(
                instance,
                config.size,
                config.colors,
                seed,
                config.framed,
                config.pins,
            );
        }

        let budget = Budget::seconds(config.budget_s);
        let board = solver.solve(&instance, budget);
        let elapsed_s = budget.elapsed_secs();

        // Canonical re-score — the solver's own opinion is never trusted here.
        let out = instance.finish(&board);
        cells.push(CellResult {
            seed,
            score: out.score,
            breaks: out.breaks,
            nodes: 0, // solvers that track nodes can extend this later
            elapsed_s,
            url: out.url,
        });
    }

    let solver_name = solver.name();
    let run_name = format!(
        "{}-{}-{}",
        slug(&solver_name),
        config_digest(config),
        stamp()
    );
    let dir = RunDir::create(runs_root, &run_name)?;
    dir.write_config(&RunConfig {
        solver: solver_name.clone(),
        size: config.size,
        colors: config.colors,
        framed: config.framed,
        seeds: config.seeds.clone(),
        pins: config.pins,
        budget_s: config.budget_s,
    })?;
    let summary = dir.write_results(&solver_name, &cells)?;
    Ok((dir, summary))
}

/// Nanoseconds since the Unix epoch, as a compact run-directory stamp. Not used
/// for any reproducibility-sensitive value — only to keep run directories
/// distinct, including two runs of the same config back to back.
fn stamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos())
}

/// A short, stable digest of the sweep parameters (not the timings), so two runs
/// of the same config share a recognisable middle segment in their directory
/// names.
fn config_digest(c: &SweepConfig) -> String {
    // Cheap FNV-1a over the parameter bytes; formatted short.
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut mix = |b: u64| {
        h ^= b;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    };
    mix(u64::from(c.size));
    mix(u64::from(c.colors));
    mix(u64::from(c.framed));
    mix(u64::from(c.pins));
    mix(c.budget_s.to_bits());
    for &s in &c.seeds {
        mix(u64::from(s));
    }
    format!("{:06x}", h & 0xff_ffff)
}

/// Turn a solver name into a filesystem-safe slug.
fn slug(name: &str) -> String {
    name.chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect()
}
