//! Single-core benchmark registry engines (`solver-engine` presets and
//! `solver-naive`), driven with the same `SolveOpts` as the benchmark's
//! `run_engine` (FirstSolution, MaxScore objective, batched hints, no time or
//! node budget) but through a sink that counts the common metric.
//!
//! Both engines emit one `ValueTried` event per value tried (immediately
//! before their own `stats.nodes += 1`). The sink rebuilds the current partial
//! assignment from the event depths, checks each tried value against the rim
//! and the assigned ancestors (a mismatch is counted in `inconsistent`), and
//! asks the solver to stop through `should_continue` once the count passes
//! the cap. The solvers poll `should_continue` once per recursion, so a run
//! may overshoot the cap by a few nodes; any run whose count exceeds the cap is
//! recorded as censored at the cap.

use std::panic::{self, AssertUnwindSafe};

use bench_grid::{core_board_to_vec, SiteInstance, SiteInstanceExt};
use eternity2_events::{EventBody, EventSink, FinalStats, SolverEvent};
use eternity2_solver_trait::{Objective, SolveMode, SolveOpts, SolveOutcome, Solver};

use crate::counter::NodeCounter;
use crate::instance::consistent;
use crate::solver::{LadderSolver, Outcome};
use eternity2_engine::{rotated, Puzzle};

/// Panic payload used if a solver ignores `should_continue` for too long.
pub struct CapOverrun;

const OVERRUN_SLACK: u64 = 1_000_000;

struct CapSink<'a> {
    puzzle: &'a Puzzle,
    cap: u64,
    count: u64,
    inconsistent: u64,
    board: Vec<i32>,
    stack: Vec<(u32, usize)>,
    final_stats: Option<FinalStats>,
}

impl EventSink for CapSink<'_> {
    fn emit(&mut self, event: SolverEvent) {
        match event.body {
            EventBody::ValueTried { position, piece_id, rotation } => {
                let d = event.depth;
                while let Some(&(dd, p)) = self.stack.last() {
                    if dd < d {
                        break;
                    }
                    self.board[p] = -1;
                    self.stack.pop();
                }
                let pos = position as usize;
                let e = rotated(self.puzzle.pieces[piece_id as usize], rotation.as_u8());
                if !consistent(self.puzzle, &self.board, pos, e) {
                    self.inconsistent += 1;
                }
                self.board[pos] = i32::from(piece_id) * 4 + i32::from(rotation.as_u8());
                self.stack.push((d, pos));
                self.count += 1;
                if self.count > self.cap + OVERRUN_SLACK {
                    panic::panic_any(CapOverrun);
                }
            }
            EventBody::Solved { final_stats, .. }
            | EventBody::Exhausted { final_stats, .. }
            | EventBody::TimedOut { final_stats, .. }
            | EventBody::Cancelled { final_stats, .. } => self.final_stats = Some(final_stats),
            _ => {}
        }
    }

    fn should_continue(&self) -> bool {
        self.count <= self.cap
    }
}

pub struct Scb {
    pub make: fn() -> Box<dyn Solver>,
    pub seed: u64,
}

impl LadderSolver for Scb {
    fn solve(&mut self, puzzle: &Puzzle, counter: &mut NodeCounter) -> Outcome {
        let site: SiteInstance =
            serde_json::from_value(serde_json::to_value(puzzle).expect("puzzle json")).expect("site json");
        let core = site.to_core_puzzle();
        let opts = SolveOpts {
            mode: SolveMode::FirstSolution,
            seed: self.seed,
            time_budget_ms: 0,
            node_budget: 0,
            hints: site.to_core_hints(),
            objective: Some(Objective::MaxScore),
            batch_hint_application: true,
            ..Default::default()
        };
        let mut solver = (self.make)();
        let mut sink = CapSink {
            puzzle,
            cap: counter.cap(),
            count: 0,
            inconsistent: 0,
            board: vec![-1; puzzle.cell_count()],
            stack: Vec::new(),
            final_stats: None,
        };
        let outcome = panic::catch_unwind(AssertUnwindSafe(|| solver.solve(&core, &opts, &mut sink)));
        counter.set_nodes(sink.count);
        counter.inconsistent = sink.inconsistent;
        if let Some(fs) = sink.final_stats {
            counter.backtracks = fs.backtracks;
            counter.propagation = fs.propagations;
            counter.native_nodes = Some(fs.nodes);
        }
        match outcome {
            Err(payload) if payload.is::<CapOverrun>() => Outcome::CapReached,
            Err(payload) => panic::resume_unwind(payload),
            Ok(SolveOutcome::Solved(b)) => Outcome::Solved(core_board_to_vec(&b)),
            Ok(SolveOutcome::Exhausted) => Outcome::Exhausted,
            Ok(SolveOutcome::AllSolutions(bs)) => bs
                .first()
                .map_or(Outcome::Exhausted, |b| Outcome::Solved(core_board_to_vec(b))),
            Ok(_) => Outcome::CapReached,
        }
    }
}
