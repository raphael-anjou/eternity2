//! Vanilla DFS with a spiral cell-visit order (path order is a first-class
use crate::SiteInstance;
use eternity2_solver_naive::NaiveSolver;

use super::{engine_common::run_engine, AlgoRun, RunParams};

pub fn run(inst: &SiteInstance, p: RunParams) -> AlgoRun {
    run_engine(inst, p, Box::new(NaiveSolver::spiral()))
}
