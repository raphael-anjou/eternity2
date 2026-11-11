//! Hard-instance ladder: planted instances at n = 6..10 with Owen
//! one-expected-solution interior colour counts, a solver-independent node
//! counter, and a harness that runs any [`LadderSolver`] on the committed
//! instances under the preregistered caps. See `../PREREG.md`,
//! `../PROTOCOL.md` and `../README.md`.

pub mod cli;
pub mod counter;
pub mod instance;
pub mod registry;
pub mod runner;
pub mod solver;
pub mod solvers;

pub use counter::NodeCounter;
pub use eternity2_engine::Puzzle;
pub use instance::{verify_board, Ladder, LadderInstance, Role};
pub use runner::{run_all, run_one, RunRecord};
pub use solver::{LadderSolver, Outcome, SolverEntry};

/// Per-instance node cap for test runs (PREREG.md).
pub const TEST_CAP: u64 = 10_000_000;
/// Per-instance node cap for calibration runs (PREREG.md).
pub const CALIBRATION_CAP: u64 = 2_000_000;
