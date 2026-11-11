//! The plug-in interface.

use crate::counter::NodeCounter;
use eternity2_engine::Puzzle;

/// Result of one search. Boards are row-major `piece * 4 + rotation`
/// (clockwise quarter turns, URDL edges), `-1` for an empty cell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// A complete board; the harness verifies it independently.
    Solved(Vec<i32>),
    /// The search space was exhausted without a solution.
    Exhausted,
    /// The node cap was reached (`NodeCounter::try_assign` returned false).
    CapReached,
}

/// A complete exact search. It receives only the scrambled puzzle (no hints,
/// no planted solution) and must call [`NodeCounter::try_assign`] once per
/// assignment, before making it, stopping when it returns `false`.
pub trait LadderSolver {
    fn solve(&mut self, puzzle: &Puzzle, counter: &mut NodeCounter) -> Outcome;
}

/// A named, runnable solver. `make` receives `run_seed`.
#[derive(Clone, Copy)]
pub struct SolverEntry {
    pub name: &'static str,
    pub description: &'static str,
    pub run_seed: u64,
    pub make: fn(u64) -> Box<dyn LadderSolver>,
}
