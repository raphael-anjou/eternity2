//! Border-first with a seeded random tie-break (seed-diverse baseline).
use crate::SiteInstance;
use eternity2_solver_engine::EngineSolver;

use super::{engine_common::run_engine, AlgoRun, RunParams};

pub fn run(inst: &SiteInstance, p: RunParams) -> AlgoRun {
    run_engine(inst, p, Box::new(EngineSolver::border_first_random()))
}
