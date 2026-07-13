//! GAColor + AC-3 + NS-1 deficit-invariant propagator.
use crate::SiteInstance;
use eternity2_solver_engine::EngineSolver;

use super::{engine_common::run_engine, AlgoRun, RunParams};

pub fn run(inst: &SiteInstance, p: RunParams) -> AlgoRun {
    run_engine(inst, p, Box::new(EngineSolver::gacolor_ac3_ns1()))
}
