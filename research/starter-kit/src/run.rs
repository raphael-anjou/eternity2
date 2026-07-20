//! Reproducible, self-describing run directories.
//!
//! Every sweep writes one directory under `runs/<name>-<stamp>/` holding three
//! files, so a run can be re-read, compared, or shipped without the code that
//! produced it:
//!
//! * `config.json` — the exact parameters (solver name, seeds, board shape,
//!   pins, budget). Re-running the same config reproduces the same results,
//!   because the seeds are deterministic.
//! * `results.jsonl` — one [`CellResult`] per line: seed, score, breaks, nodes,
//!   elapsed, and the board's eternity2.dev URL so you can open any cell.
//! * `summary.json` — the aggregate [`Summary`]: n, mean, median, best, and the
//!   standard deviation, because a score delta means nothing without its spread.
//!
//! The shape mirrors what `research/experiments/*` write at fleet scale, so a
//! promising kit solver can graduate into a full experiment later without a
//! format change.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// The exact parameters of a sweep. Written to `config.json`; re-running an
/// identical config reproduces the run byte-for-byte (seeds are deterministic).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunConfig {
    /// The solver's `name()`.
    pub solver: String,
    /// Board side length (16 for the official puzzle).
    pub size: u8,
    /// Interior colour count used to generate the boards.
    pub colors: u8,
    /// Whether boards were generated with frame-restricted colours.
    pub framed: bool,
    /// The seeds swept, in order.
    pub seeds: Vec<u32>,
    /// How many solution cells were pinned as hints on each board.
    pub pins: u32,
    /// Per-instance wall-clock budget, seconds.
    pub budget_s: f64,
}

/// One instance's result within a sweep. One JSON object per line in
/// `results.jsonl`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellResult {
    /// The generation seed for this board.
    pub seed: u32,
    /// Canonical matched-edge score (rim excluded).
    pub score: u32,
    /// `max_score - score`: unmatched interior edges.
    pub breaks: u32,
    /// Nodes/placements the solver reported, if any (0 if it doesn't track them).
    pub nodes: u64,
    /// Wall-clock seconds this instance took.
    pub elapsed_s: f64,
    /// The board's canonical eternity2.dev viewer URL.
    pub url: String,
}

/// Aggregate statistics over a sweep's cells. Written to `summary.json`.
///
/// `sd` is the population standard deviation of the per-cell scores — always
/// report it: on this puzzle the score spread across seeds is large (roughly
/// 12–20 points for a whole-board solver), so a one- or two-point mean
/// difference between two runs is usually noise until `n` is at least ~40.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    pub solver: String,
    pub n: usize,
    pub mean: f64,
    pub median: f64,
    pub best: u32,
    pub worst: u32,
    pub sd: f64,
    /// Total wall-clock seconds across all cells.
    pub total_s: f64,
}

impl Summary {
    /// Compute the aggregate over a set of cell results.
    #[must_use]
    pub fn of(solver: &str, cells: &[CellResult]) -> Self {
        let n = cells.len();
        if n == 0 {
            return Self {
                solver: solver.to_string(),
                n: 0,
                mean: 0.0,
                median: 0.0,
                best: 0,
                worst: 0,
                sd: 0.0,
                total_s: 0.0,
            };
        }
        let scores: Vec<f64> = cells.iter().map(|c| f64::from(c.score)).collect();
        let mean = scores.iter().sum::<f64>() / n as f64;
        let var = scores.iter().map(|s| (s - mean).powi(2)).sum::<f64>() / n as f64;
        let mut sorted: Vec<u32> = cells.iter().map(|c| c.score).collect();
        sorted.sort_unstable();
        let median = if n % 2 == 0 {
            f64::from(sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
        } else {
            f64::from(sorted[n / 2])
        };
        Self {
            solver: solver.to_string(),
            n,
            mean,
            median,
            best: *sorted.last().unwrap(),
            worst: sorted[0],
            sd: var.sqrt(),
            total_s: cells.iter().map(|c| c.elapsed_s).sum(),
        }
    }
}

/// A run directory on disk: `config.json`, `results.jsonl`, `summary.json`.
pub struct RunDir {
    root: PathBuf,
}

impl RunDir {
    /// Create `runs_root/<name>` (creating parents), ready to write into.
    ///
    /// `name` should already carry a stamp for uniqueness (the runner builds one
    /// from the solver name plus a caller-supplied stamp). If the directory
    /// exists it is reused.
    pub fn create(runs_root: impl AsRef<Path>, name: &str) -> std::io::Result<Self> {
        let root = runs_root.as_ref().join(name);
        std::fs::create_dir_all(&root)?;
        Ok(Self { root })
    }

    /// The directory path, e.g. for printing where a run landed.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.root
    }

    /// Write `config.json`.
    pub fn write_config(&self, config: &RunConfig) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(config).expect("config serializes");
        std::fs::write(self.root.join("config.json"), json)
    }

    /// Write `results.jsonl` (one object per line) and the derived
    /// `summary.json`.
    pub fn write_results(&self, solver: &str, cells: &[CellResult]) -> std::io::Result<Summary> {
        let mut jsonl = String::new();
        for c in cells {
            jsonl.push_str(&serde_json::to_string(c).expect("cell serializes"));
            jsonl.push('\n');
        }
        std::fs::write(self.root.join("results.jsonl"), jsonl)?;
        let summary = Summary::of(solver, cells);
        let json = serde_json::to_string_pretty(&summary).expect("summary serializes");
        std::fs::write(self.root.join("summary.json"), json)?;
        Ok(summary)
    }

    /// Read a run directory's results back (for the compare tool).
    pub fn read_results(dir: impl AsRef<Path>) -> std::io::Result<Vec<CellResult>> {
        let text = std::fs::read_to_string(dir.as_ref().join("results.jsonl"))?;
        let mut out = Vec::new();
        for line in text.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let cell: CellResult = serde_json::from_str(line)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            out.push(cell);
        }
        Ok(out)
    }
}
