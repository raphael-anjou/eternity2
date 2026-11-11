//! Hard-ladder runner with the `bb-*` solvers registered (plug-in interface).

use backdoor_branching::{learned_rule, single_rule, BbSolver, Rule, TIME_GUARD};
use hard_ladder::{LadderSolver, SolverEntry};

fn main() -> std::process::ExitCode {
    hard_ladder::cli::main_with(&[
        SolverEntry {
            name: "bb-mrv",
            description: "A3 propagation, dynamic fail-first (min |D|)",
            run_seed: 0,
            make: |_| -> Box<dyn LadderSolver> { Box::new(BbSolver { rule: Rule::Mrv, guard: TIME_GUARD }) },
        },
        SolverEntry {
            name: "bb-impact",
            description: "A3 propagation, Refalo-style impact (min sum exp(L_o - L))",
            run_seed: 0,
            make: |_| -> Box<dyn LadderSolver> { Box::new(BbSolver { rule: Rule::Impact, guard: TIME_GUARD }) },
        },
        SolverEntry {
            name: "bb-learned",
            description: "A3 propagation, learned backdoor score (V1)",
            run_seed: 0,
            make: |_| -> Box<dyn LadderSolver> { Box::new(BbSolver { rule: learned_rule(), guard: TIME_GUARD }) },
        },
        SolverEntry {
            name: "bb-single",
            description: "A3 propagation, best single non-domain feature (V2)",
            run_seed: 0,
            make: |_| -> Box<dyn LadderSolver> { Box::new(BbSolver { rule: single_rule(), guard: TIME_GUARD }) },
        },
    ])
}
