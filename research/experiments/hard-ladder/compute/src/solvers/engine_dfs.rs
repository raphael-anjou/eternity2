//! The main engine's step-able path DFS (`engine/src/solver.rs`), the plain
//! DFS of `research/topics/hardness-peak`. Its `Report::nodes` counts one per
//! placement that fits the rim and every placed neighbour: exactly the common
//! metric. Stepping in chunks of at most `cap - nodes` steps (one step is one
//! placement or one backtrack) keeps the count at or below the cap.

use crate::counter::NodeCounter;
use crate::solver::{LadderSolver, Outcome};
use eternity2_engine::{build_path, Puzzle, Solver, Status};

pub struct EngineDfs {
    pub path_kind: &'static str,
}

impl LadderSolver for EngineDfs {
    fn solve(&mut self, puzzle: &Puzzle, counter: &mut NodeCounter) -> Outcome {
        let path = build_path(self.path_kind, puzzle.width, puzzle.height, 0)
            .unwrap_or_else(|| panic!("unknown path kind {}", self.path_kind));
        let mut s = Solver::new(puzzle, &path, false, false, 0).expect("valid puzzle");
        let cells = puzzle.cell_count() as u32;
        let cap = counter.cap();
        loop {
            let rep = s.report();
            let nodes = rep.nodes as u64;
            counter.set_nodes(nodes);
            counter.backtracks = rep.backtracks as u64;
            counter.fit_tests = rep.attempts as u64;
            counter.native_nodes = Some(nodes);
            match rep.status {
                Status::Solved => return Outcome::Solved(s.board().to_vec()),
                Status::Exhausted => return Outcome::Exhausted,
                Status::Running => {}
            }
            if nodes >= cap {
                if rep.placed == cells {
                    // Full board: the next step only records the solution.
                    s.step(1);
                    continue;
                }
                return Outcome::CapReached;
            }
            let k = (cap - nodes).min(1 << 24) as u32;
            s.step(k);
        }
    }
}
