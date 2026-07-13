//! GAColor value order + AC-3 arc consistency.
use crate::SiteInstance;
use eternity2_solver_engine::EngineSolver;

use super::{engine_common::run_engine, AlgoRun, RunParams};

pub fn run(inst: &SiteInstance, p: RunParams) -> AlgoRun {
    run_engine(inst, p, Box::new(EngineSolver::gacolor_ac3()))
}
