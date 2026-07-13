//! One module per benchmarked algorithm. Every algo is a self-contained entry
//! `fn run(inst, seed, budget_ms) -> AlgoRun` over the SITE-schema puzzle, so it
//! reads as a single unit and is liftable to the public site independently.
//!
//! The heavy algorithm logic still lives in the shared library crates
//! (`solver-engine`, `solver-naive`); each module here is the thin, uniform
//! entry point that adapts that engine to the benchmark interface and reports
//! its native throughput unit.

use crate::SiteInstance;

pub mod border_first_full;
pub mod border_first_lcv;
pub mod border_first_random;
pub mod gacolor_ac3;
pub mod gacolor_ac3_lcv;
pub mod gacolor_ac3_ns1;
pub mod gacolor_ac3_random;
pub mod joe_depth150;
pub mod joe_depth150_bp;
pub mod naive_rowmajor;
pub mod naive_spiral;
pub mod rare_color_first;
pub mod verhaard_preferred;

mod engine_common;

/// What one algorithm run yields. `board` is row-major `piece*4 + rot` (`-1`
/// empty); nps is in the algo's NATIVE unit (labeled by `nps_unit`), never
/// cross-compared between families.
pub struct AlgoRun {
    pub board: Vec<i32>,
    pub nodes: u64,
    pub backtracks: u64,
    pub nps_unit: &'static str,
}

/// Run parameters shared by every algorithm entry point. `node_budget = 0`
/// means "unbounded — use the wall-clock `budget_ms`"; a non-zero `node_budget`
/// caps the search to a FIXED amount of work so the run is reproducible (the
/// bit-identical A/B gate for optimization uses this — wall-time runs are not
/// reproducible because a faster build does more work).
#[derive(Debug, Clone, Copy)]
pub struct RunParams {
    pub seed: u64,
    pub budget_ms: u64,
    pub node_budget: u64,
}

/// Signature every algo module exposes.
pub type AlgoFn = fn(&SiteInstance, RunParams) -> AlgoRun;

/// The registry: stable name -> entry point. Adding an algorithm = add a module
/// + one line here. Single-core only (no `_par` presets — this grid is
/// single-core apples to apples).
pub fn registry() -> &'static [(&'static str, AlgoFn)] {
    &[
        ("naive_rowmajor", naive_rowmajor::run),
        ("naive_spiral", naive_spiral::run),
        ("border_first_lcv", border_first_lcv::run),
        ("border_first_full", border_first_full::run),
        ("border_first_random", border_first_random::run),
        ("rare_color_first", rare_color_first::run),
        ("gacolor_ac3", gacolor_ac3::run),
        ("gacolor_ac3_ns1", gacolor_ac3_ns1::run),
        ("gacolor_ac3_lcv", gacolor_ac3_lcv::run),
        ("gacolor_ac3_random", gacolor_ac3_random::run),
        ("joe_depth150", joe_depth150::run),
        ("joe_depth150_bp", joe_depth150_bp::run),
        ("verhaard_preferred", verhaard_preferred::run),
    ]
}

pub fn lookup(name: &str) -> Option<AlgoFn> {
    registry().iter().find(|(n, _)| *n == name).map(|(_, f)| *f)
}
