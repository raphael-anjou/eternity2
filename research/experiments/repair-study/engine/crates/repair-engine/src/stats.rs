//! Search statistics for the repair loop — the numbers this study raises for
//! every run, first-class in the search state rather than bolted on after.
//!
//! A backtracker is measured by how deep it reached; a repair loop never has a
//! depth, so the axes are different. What matters here is how many
//! destroy-and-repair *iterations* the budget bought, what fraction of them the
//! acceptance rule kept, how many were genuine improvements, and — the axis the
//! whole "ALNS saturates early" story turns on — *when the best score stopped
//! moving*. Those are the counters below.

/// The best score after each block of iterations, for the convergence curve the
/// site draws. One sample every [`CURVE_STRIDE`] iterations keeps the vector
/// small while still showing the early-flattening shape.
pub const CURVE_STRIDE: u64 = 200;

/// Live counters a repair run updates as it iterates. One instance per run.
#[derive(Debug, Clone, Default)]
pub struct RepairStats {
    /// Destroy-and-repair iterations completed. The denominator of iters/sec and
    /// the study's "how much work did the budget buy" axis.
    pub iterations: u64,
    /// Iterations whose rebuilt board was *kept* (improved, or accepted sideways
    /// or downward by the acceptance rule). `accepts / iterations` is the accept
    /// rate the study reports.
    pub accepts: u64,
    /// Iterations that strictly improved the working score. Distinct from
    /// `accepts`: an annealing rule keeps non-improving moves too.
    pub improvements: u64,
    /// Iterations that improved the global best (a new best-so-far). The count of
    /// times the run actually moved its output forward.
    pub best_improvements: u64,
    /// Total cells destroyed across all iterations, so the mean destroy size can
    /// be reported (`destroyed_cells / iterations`).
    pub destroyed_cells: u64,
    /// Perturbation restarts fired (kick / random-restart). Zero for a variant
    /// with no restart policy.
    pub restarts: u64,
    /// The iteration at which the global best was last improved. The heart of the
    /// "improvement arrives early, then the loop stalls" finding: if this is far
    /// below `iterations`, the run spent most of its budget not moving.
    pub last_best_iter: u64,
    /// Score of the starting board, before any repair — so the study can report
    /// the lift the loop added over its construction, separately from the
    /// construction itself.
    pub start_score: u32,
    /// Best score sampled every [`CURVE_STRIDE`] iterations (the convergence
    /// curve). `curve[i]` is the best score after `i * CURVE_STRIDE` iterations.
    pub curve: Vec<u32>,
}

impl RepairStats {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Iterations per second over `elapsed_s` wall time. Labelled
    /// `repair-iters/s` and, like the DFS study's node rate, never cross-compared
    /// across families: an iteration that runs an exact refill is not the same
    /// unit of work as one that runs a greedy fill.
    #[must_use]
    pub fn iters_per_sec(&self, elapsed_s: f64) -> f64 {
        if elapsed_s > 0.0 {
            self.iterations as f64 / elapsed_s
        } else {
            0.0
        }
    }

    /// Accept rate in `[0, 1]`: kept iterations over total.
    #[must_use]
    pub fn accept_rate(&self) -> f64 {
        if self.iterations > 0 {
            self.accepts as f64 / self.iterations as f64
        } else {
            0.0
        }
    }

    /// Mean cells destroyed per iteration.
    #[must_use]
    pub fn mean_destroy(&self) -> f64 {
        if self.iterations > 0 {
            self.destroyed_cells as f64 / self.iterations as f64
        } else {
            0.0
        }
    }
}
