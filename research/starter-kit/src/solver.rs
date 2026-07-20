//! The one extension point. Implement [`Solver`] and the rest of the kit — the
//! sweep runner, the run directories, the compare tool — works on your idea
//! unchanged.
//!
//! A solver takes an [`Instance`] (the piece set + board dimensions + any pinned
//! hints) and a [`Budget`] (how long it may run), and returns a filled
//! [`Board`]. It does **not** score itself: the kit re-scores every board
//! canonically through `e2-io`'s `Instance::finish`, so your reported number is
//! the same number the site and every other engine produce. Return the best
//! board you found; a partial board is fine and scores what it scores.
//!
//! See `examples/my_solver.rs` for a complete, working baseline to copy.

use std::time::{Duration, Instant};

use e2_core::Board;
use e2_io::Instance;

/// How long a solver may run, and a cheap way to ask "am I out of time?".
///
/// The sweep runner constructs one per cell from the configured per-instance
/// time budget. A solver should poll [`Budget::expired`] in its inner loop and
/// return the best board it has when the budget is gone — the kit measures wall
/// time single-core, so a solver that ignores the budget just runs long and
/// skews the comparison.
#[derive(Debug, Clone, Copy)]
pub struct Budget {
    start: Instant,
    limit: Duration,
}

impl Budget {
    /// A budget of `secs` wall-clock seconds, starting now.
    #[must_use]
    pub fn seconds(secs: f64) -> Self {
        Self {
            start: Instant::now(),
            limit: Duration::from_secs_f64(secs.max(0.0)),
        }
    }

    /// True once the wall-clock budget is spent.
    #[must_use]
    pub fn expired(&self) -> bool {
        self.start.elapsed() >= self.limit
    }

    /// Seconds elapsed since the budget started.
    #[must_use]
    pub fn elapsed_secs(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }

    /// Fraction of the budget consumed, clamped to `[0, 1]`.
    #[must_use]
    pub fn fraction(&self) -> f64 {
        if self.limit.is_zero() {
            1.0
        } else {
            (self.start.elapsed().as_secs_f64() / self.limit.as_secs_f64()).min(1.0)
        }
    }
}

/// A solver: your idea, in one method.
///
/// Implement `name` (used in run directories and reports) and `solve`. Keep any
/// per-run state on `self`; the runner calls `solve` once per instance and may
/// reuse the same solver across instances, so reset anything stateful at the top
/// of `solve` if you need to.
pub trait Solver {
    /// A short, stable identifier — it names the run directory and appears in
    /// reports. Prefer kebab-case, e.g. `"greedy-mrv"`.
    fn name(&self) -> String;

    /// Fill and return a board for `instance` within `budget`. Start from
    /// `instance.seed_board()` to honour pinned hints. Returning a partial board
    /// is allowed; it scores what it scores.
    fn solve(&mut self, instance: &Instance, budget: Budget) -> Board;
}
