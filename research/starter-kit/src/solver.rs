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

    /// Seconds left before the budget expires (0 once spent). Handy for
    /// deriving an internal deadline in a recursive search.
    #[must_use]
    pub fn remaining_secs(&self) -> f64 {
        (self.limit.as_secs_f64() - self.start.elapsed().as_secs_f64()).max(0.0)
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

/// The kind of a bound, kept distinct so a *proven* bound can never be recorded
/// as if it were an *achieved* score.
///
/// A greedy relaxation, an LP upper bound, and a MIP upper bound are all numbers
/// that bound the optimum, not boards anyone placed. Carrying the kind on the
/// value at the type level is what stops the single most dangerous reporting
/// mistake on this puzzle: writing down a relaxation's number as if a solver had
/// reached it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BoundKind {
    /// An LP-relaxation upper bound.
    LpUb,
    /// A MIP upper bound (LP + integrality, still a bound, not a placement).
    MipUb,
    /// A combinatorial / greedy relaxation bound (e.g. sum of best local fits).
    GreedyRelaxed,
}

/// What a solve produced, beyond the board itself. This is the honest answer to
/// "what does this number mean?".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum OutcomeKind {
    /// The search finished on its own terms with a **full** board — every cell
    /// placed — and this is the answer it settled on. Use it only for a complete
    /// board; a search that stopped with holes or on the budget is `Improved`,
    /// and one that *proved* nothing better exists is `Exhausted`. Also the
    /// default when a stored result carries no explicit outcome.
    #[default]
    Complete,
    /// The search space (or the assigned sub-space) was **exhausted**: no better
    /// board exists there. The board is the best found; the *fact of exhaustion*
    /// is the result. This is what LEDGER / tail-enumeration work returns.
    Exhausted,
    /// The solver improved on its starting board and stopped on the budget. The
    /// board is the best-so-far; it is neither complete nor exhaustive.
    Improved,
    /// The result is a **bound**, not a placed board: `value` bounds the optimum
    /// from above, and `kind` says how it was obtained. The board field is
    /// whatever partial board the relaxation was read off (often empty).
    Bound {
        /// The bounding value (an upper bound on the achievable matched-edge score).
        value: u32,
        /// How the bound was obtained — never `Complete`/an achieved score.
        kind: BoundKind,
    },
}

/// Everything a solve returns: the board, what the result *means*, and the work
/// it took. The kit re-scores `board` canonically, so `OutcomeKind` describes
/// the search, never the score — the two are kept separate on purpose.
#[derive(Debug, Clone)]
pub struct SolveOutcome {
    /// The best board the solver found (may be partial, or empty for a pure bound).
    pub board: Board,
    /// What the search actually established.
    pub kind: OutcomeKind,
    /// Search work done — nodes/placements explored, `0` if not tracked.
    pub nodes: u64,
}

impl SolveOutcome {
    /// A completed solve returning `board`.
    #[must_use]
    pub fn complete(board: Board) -> Self {
        Self { board, kind: OutcomeKind::Complete, nodes: 0 }
    }

    /// An improved-but-unfinished solve (stopped on the budget).
    #[must_use]
    pub fn improved(board: Board) -> Self {
        Self { board, kind: OutcomeKind::Improved, nodes: 0 }
    }

    /// An exhaustive result: the (sub-)space held no better board than `board`.
    #[must_use]
    pub fn exhausted(board: Board) -> Self {
        Self { board, kind: OutcomeKind::Exhausted, nodes: 0 }
    }

    /// A bound result: `value` bounds the optimum from above, obtained via `kind`.
    #[must_use]
    pub fn bound(board: Board, value: u32, kind: BoundKind) -> Self {
        Self { board, kind: OutcomeKind::Bound { value, kind }, nodes: 0 }
    }

    /// Attach a node count (builder style).
    #[must_use]
    pub fn with_nodes(mut self, nodes: u64) -> Self {
        self.nodes = nodes;
        self
    }
}

/// A solver: your idea, in one method.
///
/// Implement `name` (for run directories and reports) and `solve`. `solve` is
/// handed a **starting board** to continue from. Through the [`sweep`] runner
/// that start is always `instance.seed_board()` (a fresh board carrying only the
/// pinned clues), because a sweep's job is one independent board per seed. To
/// continue from an *arbitrary* partial board — seed-and-grow, band/column, or
/// repair-a-known-board — call `solve` **directly** with your own start board
/// (see the continuation demo at the bottom of `examples/my_solver.rs`); you are
/// not limited to what the sweep passes.
///
/// Return a [`SolveOutcome`] so the result's *meaning* (complete, exhausted,
/// improved, or a bound) travels with the board.
///
/// Keep per-run state on `self`; the runner calls `solve` once per instance and
/// may reuse the solver, so reset anything stateful at the top of `solve`.
///
/// [`sweep`]: crate::sweep
pub trait Solver {
    /// A short, stable identifier — it names the run directory and appears in
    /// reports. Prefer kebab-case, e.g. `"greedy-mrv"`.
    fn name(&self) -> String;

    /// Search from `start` (already carries any pinned clues) within `budget`,
    /// and return what you found and what it means. A partial board is fine; a
    /// pure bound need not place anything.
    fn solve(&mut self, instance: &Instance, start: &Board, budget: Budget) -> SolveOutcome;
}
