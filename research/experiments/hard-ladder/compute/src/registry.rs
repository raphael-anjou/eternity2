//! Built-in solver registry.

use crate::solver::{LadderSolver, SolverEntry};
use crate::solvers::engine_dfs::EngineDfs;
use crate::solvers::example::ExampleDfs;
use crate::solvers::scb::Scb;
use eternity2_solver_engine::EngineSolver;
use eternity2_solver_naive::NaiveSolver;
use eternity2_solver_trait::Solver;

macro_rules! engine_dfs {
    ($name:literal, $kind:literal, $desc:literal) => {
        SolverEntry {
            name: $name,
            description: $desc,
            run_seed: 0,
            make: |_| -> Box<dyn LadderSolver> { Box::new(EngineDfs { path_kind: $kind }) },
        }
    };
}

macro_rules! scb {
    ($name:literal, $seed:literal, $ctor:expr, $desc:literal) => {
        SolverEntry {
            name: $name,
            description: $desc,
            run_seed: $seed,
            make: |seed| -> Box<dyn LadderSolver> {
                Box::new(Scb { make: || -> Box<dyn Solver> { Box::new($ctor) }, seed })
            },
        }
    };
}

pub const BUILTIN: &[SolverEntry] = &[
    engine_dfs!("hp-dfs", "row-major", "main engine plain DFS, row-major path (hardness-peak baseline)"),
    engine_dfs!("engine-dfs-spiral-in", "spiral-in", "main engine plain DFS, spiral-in path"),
    engine_dfs!("engine-dfs-border-first", "border-first", "main engine plain DFS, border-first path"),
    engine_dfs!("engine-dfs-snake", "snake", "main engine plain DFS, snake path"),
    scb!("scb-naive_rowmajor", 0, NaiveSolver::row_by_row(), "single-core registry naive_rowmajor"),
    scb!("scb-naive_spiral", 0, NaiveSolver::spiral(), "single-core registry naive_spiral"),
    scb!("scb-border_first_lcv", 0, EngineSolver::border_first_lcv(), "border-first MRV + class balance"),
    scb!("scb-border_first_full", 0, EngineSolver::border_first_full(), "border_first_lcv + parity + island"),
    scb!("scb-border_first_random", 1, EngineSolver::border_first_random(), "border-first random ties"),
    scb!("scb-rare_color_first", 0, EngineSolver::rare_color_first(), "rare-colour-first variable order"),
    scb!("scb-gacolor_ac3", 0, EngineSolver::gacolor_ac3(), "border-first MRV + gacolor + AC-3"),
    scb!("scb-gacolor_ac3_ns1", 0, EngineSolver::gacolor_ac3_ns1(), "gacolor_ac3 + NS-1 multiset equality"),
    scb!("scb-gacolor_ac3_lcv", 0, EngineSolver::gacolor_ac3_lcv(), "gacolor_ac3 + least-constraining value"),
    scb!("scb-gacolor_ac3_random", 1, EngineSolver::gacolor_ac3_random(), "gacolor_ac3, random ordering"),
    SolverEntry {
        name: "example-dfs",
        description: "plug-in example: row-major DFS through the LadderSolver trait",
        run_seed: 0,
        make: |_| -> Box<dyn LadderSolver> { Box::new(ExampleDfs) },
    },
];

/// Calibration candidates (PREREG.md, "Baselines").
pub const CALIBRATION_CANDIDATES: &[&str] = &[
    "hp-dfs",
    "engine-dfs-spiral-in",
    "engine-dfs-border-first",
    "engine-dfs-snake",
    "scb-naive_rowmajor",
    "scb-naive_spiral",
    "scb-border_first_lcv",
    "scb-border_first_full",
    "scb-border_first_random",
    "scb-rare_color_first",
    "scb-gacolor_ac3",
    "scb-gacolor_ac3_ns1",
    "scb-gacolor_ac3_lcv",
    "scb-gacolor_ac3_random",
];

#[must_use]
pub fn find<'a>(extra: &'a [SolverEntry], name: &str) -> Option<&'a SolverEntry>
where
    'static: 'a,
{
    extra.iter().chain(BUILTIN.iter()).find(|e| e.name == name)
}
