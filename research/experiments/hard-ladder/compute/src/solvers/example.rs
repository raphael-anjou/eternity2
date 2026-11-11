//! Minimal plug-in example: row-major DFS written against [`LadderSolver`].
//! Same candidate order as the engine's plain DFS (piece index, then
//! rotation, identical orientations skipped), so its node counts must equal
//! `hp-dfs` exactly; `tests/ladder.rs` checks this.

use crate::counter::NodeCounter;
use crate::instance::consistent;
use crate::solver::{LadderSolver, Outcome};
use eternity2_engine::{rotated, Puzzle};

#[derive(Default)]
pub struct ExampleDfs;

struct State<'a> {
    puzzle: &'a Puzzle,
    board: Vec<i32>,
    used: Vec<bool>,
}

enum Step {
    Found,
    Failed,
    Cap,
}

fn dfs(st: &mut State, pos: usize, counter: &mut NodeCounter) -> Step {
    if pos == st.board.len() {
        return Step::Found;
    }
    for p in 0..st.puzzle.pieces.len() {
        if st.used[p] {
            continue;
        }
        for r in 0..4u8 {
            let e = rotated(st.puzzle.pieces[p], r);
            if (0..r).any(|q| rotated(st.puzzle.pieces[p], q) == e) {
                continue;
            }
            counter.fit_tests += 1;
            if !consistent(st.puzzle, &st.board, pos, e) {
                continue;
            }
            if !counter.try_assign() {
                return Step::Cap;
            }
            st.board[pos] = (p as i32) * 4 + i32::from(r);
            st.used[p] = true;
            match dfs(st, pos + 1, counter) {
                Step::Failed => {}
                done => return done,
            }
            st.board[pos] = -1;
            st.used[p] = false;
            counter.backtracks += 1;
        }
    }
    Step::Failed
}

impl LadderSolver for ExampleDfs {
    fn solve(&mut self, puzzle: &Puzzle, counter: &mut NodeCounter) -> Outcome {
        let cells = puzzle.cell_count();
        let mut st = State { puzzle, board: vec![-1; cells], used: vec![false; cells] };
        match dfs(&mut st, 0, counter) {
            Step::Found => Outcome::Solved(st.board),
            Step::Failed => Outcome::Exhausted,
            Step::Cap => Outcome::CapReached,
        }
    }
}
