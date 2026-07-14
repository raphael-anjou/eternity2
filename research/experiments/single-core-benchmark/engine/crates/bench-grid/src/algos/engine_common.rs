//! Shared drive logic for every EngineSolver/Solver-trait algorithm. Each
//! per-algo module is reduced to one call: pick the solver, hand it here. All
//! the opts wiring, MaxScore objective, stats capture, and board extraction
//! live ONCE, here.

use crate::{core_board_to_vec, SiteInstance};
use eternity2_events::{EventBody, EventSink, FinalStats, SolverEvent};
use eternity2_solver_trait::{Objective, SolveMode, SolveOpts, SolveOutcome, Solver};

use super::{AlgoRun, RunParams};

/// Captures only the terminal `FinalStats`. No per-node work — the engine emits
/// ~a dozen category events per run, so this is throughput-neutral vs a null
/// sink.
#[derive(Default)]
struct StatsSink {
    final_stats: Option<FinalStats>,
}

impl EventSink for StatsSink {
    fn emit(&mut self, event: SolverEvent) {
        match event.body {
            EventBody::Solved { final_stats, .. }
            | EventBody::Exhausted { final_stats, .. }
            | EventBody::TimedOut { final_stats, .. }
            | EventBody::Cancelled { final_stats, .. } => {
                self.final_stats = Some(final_stats);
            }
            _ => {}
        }
    }
}

/// Drive any boxed `Solver` on a site instance for `budget_ms`, single-core,
/// maximizing matched edges. Returns the best board + native node stats. This
/// is the ONE place the engine-family algos share; a per-algo module is just
/// `run_engine(inst, seed, budget_ms, EngineSolver::preset())`.
pub fn run_engine(
    inst: &SiteInstance,
    params: RunParams,
    mut solver: Box<dyn Solver>,
) -> AlgoRun {
    let core_puzzle = inst.to_core_puzzle();
    let opts = SolveOpts {
        mode: SolveMode::FirstSolution,
        seed: params.seed,
        time_budget_ms: params.budget_ms,
        node_budget: params.node_budget,
        hints: inst.to_core_hints(),
        objective: Some(Objective::MaxScore),
        batch_hint_application: true,
        ..Default::default()
    };

    let mut sink = StatsSink::default();
    let outcome = solver.solve(&core_puzzle, &opts, &mut sink);

    let board = match outcome {
        SolveOutcome::Solved(b) => Some(b),
        SolveOutcome::TimedOut { best_partial, .. }
        | SolveOutcome::Cancelled { best_partial, .. } => Some(best_partial),
        SolveOutcome::AllSolutions(bs) => bs.into_iter().next(),
        SolveOutcome::Exhausted | SolveOutcome::Error(_) => None,
    };
    let board_vec = board
        .map(|b| core_board_to_vec(&b))
        .unwrap_or_else(|| vec![-1; inst.cell_count()]);

    let stats = sink.final_stats.unwrap_or_default();
    AlgoRun {
        board: board_vec,
        nodes: stats.nodes,
        backtracks: stats.backtracks,
        // All engine-family algos are DFS/CSP backtrack search.
        nps_unit: "search-nodes/s",
    }
}
