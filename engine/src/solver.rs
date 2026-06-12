//! Step-able backtracking DFS over an arbitrary cell-visit order.
//!
//! The solver is an explicit machine, not a recursive function: callers run
//! `step(budget)` repeatedly and read off the board between calls, which is
//! exactly what an animated browser UI needs. One "step" is one placement or
//! one backtrack; the candidate scan inside a step is bounded by
//! pieces × rotations.
//!
//! No clocks in here; wall-time is the caller's business (std::time::Instant
//! panics on wasm32-unknown-unknown).

use crate::types::{rotated, Color, Puzzle, BORDER};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Running,
    Solved,
    Exhausted,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    pub status: Status,
    /// Successful placements so far (f64: u32 overflows in long runs).
    pub nodes: f64,
    /// Candidate (piece, rotation) fit checks.
    pub attempts: f64,
    pub backtracks: f64,
    /// Pieces currently on the board, hints included.
    pub placed: u32,
    /// Deepest the search has ever been, hints included.
    pub best_placed: u32,
}

struct Frame {
    pos: u16,
    /// Next candidate row (piece_order_index * 4 + rotation) to try when we
    /// come back to this depth.
    cursor: u32,
    /// piece_order index placed here, or u32::MAX while empty.
    placed: u32,
}

pub struct Solver {
    width: usize,
    height: usize,
    n_pieces: usize,
    /// Rotated edges for every (piece, rotation), piece-id major.
    table: Vec<[Color; 4]>,
    /// False for rotations identical to a lower rotation of the same piece.
    distinct: Vec<bool>,
    /// Iteration order over pieces (optionally seed-shuffled).
    piece_order: Vec<u16>,
    /// Cell -> table row (piece_id * 4 + rot) or -1.
    board: Vec<i32>,
    used: Vec<bool>,
    /// One frame per non-hint cell, in visit order.
    frames: Vec<Frame>,
    depth: usize,
    hint_count: u32,
    status: Status,
    nodes: u64,
    attempts: u64,
    backtracks: u64,
    best_placed: u32,
    best_board: Vec<i32>,
}

impl Solver {
    pub fn new(
        puzzle: &Puzzle,
        path: &[u16],
        use_hints: bool,
        shuffle_pieces: bool,
        seed: u32,
    ) -> Result<Solver, String> {
        let n_cells = puzzle.cell_count();
        let n_pieces = puzzle.pieces.len();
        if n_pieces != n_cells {
            return Err(format!("puzzle has {n_pieces} pieces for {n_cells} cells"));
        }
        if path.len() != n_cells {
            return Err(format!("path covers {} of {n_cells} cells", path.len()));
        }
        let mut seen = vec![false; n_cells];
        for &c in path {
            if c as usize >= n_cells || seen[c as usize] {
                return Err("path is not a permutation of the cells".into());
            }
            seen[c as usize] = true;
        }

        let mut table = vec![[0u8; 4]; n_pieces * 4];
        let mut distinct = vec![true; n_pieces * 4];
        for (id, &e) in puzzle.pieces.iter().enumerate() {
            for r in 0..4u8 {
                let re = rotated(e, r);
                table[id * 4 + r as usize] = re;
                for prev in 0..r {
                    if table[id * 4 + prev as usize] == re {
                        distinct[id * 4 + r as usize] = false;
                        break;
                    }
                }
            }
        }

        let mut piece_order: Vec<u16> = (0..n_pieces as u16).collect();
        if shuffle_pieces {
            crate::generator::XorShift::new(seed).shuffle(&mut piece_order);
        }

        let mut board = vec![-1i32; n_cells];
        let mut used = vec![false; n_pieces];
        let mut hint_count = 0u32;
        if use_hints {
            for h in &puzzle.hints {
                let pos = h.pos as usize;
                if pos >= n_cells || h.piece as usize >= n_pieces || used[h.piece as usize] {
                    return Err(format!("invalid hint at position {pos}"));
                }
                board[pos] = (h.piece as i32) * 4 + i32::from(h.rot & 3);
                used[h.piece as usize] = true;
                hint_count += 1;
            }
        }

        let frames = path
            .iter()
            .filter(|&&c| board[c as usize] == -1)
            .map(|&c| Frame {
                pos: c,
                cursor: 0,
                placed: u32::MAX,
            })
            .collect();

        let best_board = board.clone();
        Ok(Solver {
            width: puzzle.width as usize,
            height: puzzle.height as usize,
            n_pieces,
            table,
            distinct,
            piece_order,
            board,
            used,
            frames,
            depth: 0,
            hint_count,
            status: Status::Running,
            nodes: 0,
            attempts: 0,
            backtracks: 0,
            best_placed: hint_count,
            best_board,
        })
    }

    #[inline]
    fn fits(&self, pos: usize, e: [Color; 4]) -> bool {
        let (w, h) = (self.width, self.height);
        let (x, y) = (pos % w, pos / w);
        let [top, right, bottom, left] = e;

        // Rim edges must be grey; interior-facing edges must not be.
        if (y == 0) != (top == BORDER) {
            return false;
        }
        if (y == h - 1) != (bottom == BORDER) {
            return false;
        }
        if (x == 0) != (left == BORDER) {
            return false;
        }
        if (x == w - 1) != (right == BORDER) {
            return false;
        }

        // Match whichever neighbors are already placed (the visit order is
        // arbitrary, so all four directions matter).
        if y > 0 {
            let n = self.board[pos - w];
            if n >= 0 && self.table[n as usize][2] != top {
                return false;
            }
        }
        if y < h - 1 {
            let n = self.board[pos + w];
            if n >= 0 && self.table[n as usize][0] != bottom {
                return false;
            }
        }
        if x > 0 {
            let n = self.board[pos - 1];
            if n >= 0 && self.table[n as usize][1] != left {
                return false;
            }
        }
        if x < w - 1 {
            let n = self.board[pos + 1];
            if n >= 0 && self.table[n as usize][3] != right {
                return false;
            }
        }
        true
    }

    /// Run up to `budget` steps (placements + backtracks). Returns the
    /// report after stopping; cheap to call in an animation loop.
    pub fn step(&mut self, budget: u32) -> Report {
        let mut remaining = budget;
        while remaining > 0 && self.status == Status::Running {
            remaining -= 1;

            if self.depth == self.frames.len() {
                self.status = Status::Solved;
                self.best_placed = self.placed();
                self.best_board.copy_from_slice(&self.board);
                break;
            }

            let frame = &self.frames[self.depth];
            let pos = frame.pos as usize;
            let mut cursor = frame.cursor;
            let limit = (self.n_pieces * 4) as u32;
            let mut placed_row = u32::MAX;

            while cursor < limit {
                let oi = (cursor / 4) as usize;
                let r = cursor % 4;
                cursor += 1;
                let pid = self.piece_order[oi] as usize;
                if self.used[pid] {
                    cursor = (cursor + 3) & !3; // skip remaining rotations
                    continue;
                }
                let row = pid * 4 + r as usize;
                if !self.distinct[row] {
                    continue;
                }
                self.attempts += 1;
                if self.fits(pos, self.table[row]) {
                    placed_row = row as u32;
                    break;
                }
            }

            if placed_row != u32::MAX {
                let pid = (placed_row / 4) as usize;
                self.board[pos] = placed_row as i32;
                self.used[pid] = true;
                let f = &mut self.frames[self.depth];
                f.cursor = cursor;
                f.placed = placed_row;
                self.depth += 1;
                self.nodes += 1;
                let placed = self.placed();
                if placed > self.best_placed {
                    self.best_placed = placed;
                    self.best_board.copy_from_slice(&self.board);
                }
            } else {
                // Dead end: reset this frame and undo the previous placement.
                let f = &mut self.frames[self.depth];
                f.cursor = 0;
                if self.depth == 0 {
                    self.status = Status::Exhausted;
                    break;
                }
                self.depth -= 1;
                let prev = &mut self.frames[self.depth];
                let row = prev.placed;
                prev.placed = u32::MAX;
                self.board[prev.pos as usize] = -1;
                self.used[(row / 4) as usize] = false;
                self.backtracks += 1;
            }
        }
        self.report()
    }

    pub fn placed(&self) -> u32 {
        self.hint_count + self.depth as u32
    }

    pub fn report(&self) -> Report {
        Report {
            status: self.status,
            nodes: self.nodes as f64,
            attempts: self.attempts as f64,
            backtracks: self.backtracks as f64,
            placed: self.placed(),
            best_placed: self.best_placed,
        }
    }

    /// Current board: cell -> piece_id*4+rotation, or -1 when empty.
    pub fn board(&self) -> &[i32] {
        &self.board
    }

    /// Deepest board ever reached (by placed count).
    pub fn best_board(&self) -> &[i32] {
        &self.best_board
    }
}

/// Matched interior edges of a board encoded as cell -> piece*4+rot | -1.
/// Bucas convention: a grey-grey interior contact does NOT score.
pub fn score_board(puzzle: &Puzzle, board: &[i32]) -> u32 {
    let (w, h) = (puzzle.width as usize, puzzle.height as usize);
    let edges_of = |row: i32| -> Option<[Color; 4]> {
        if row < 0 {
            return None;
        }
        let pid = (row / 4) as usize;
        let r = (row % 4) as u8;
        Some(rotated(*puzzle.pieces.get(pid)?, r))
    };
    let mut score = 0;
    for y in 0..h {
        for x in 0..w {
            let here = edges_of(board[y * w + x]);
            if x + 1 < w {
                if let (Some(a), Some(b)) = (here, edges_of(board[y * w + x + 1])) {
                    if a[1] == b[3] && a[1] != BORDER {
                        score += 1;
                    }
                }
            }
            if y + 1 < h {
                if let (Some(a), Some(b)) = (here, edges_of(board[(y + 1) * w + x])) {
                    if a[2] == b[0] && a[2] != BORDER {
                        score += 1;
                    }
                }
            }
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::generate;
    use crate::paths::build_path;

    fn solve(puzzle: &Puzzle, path_kind: &str) -> Report {
        let path = build_path(path_kind, puzzle.width, puzzle.height, 0).unwrap();
        let mut s = Solver::new(puzzle, &path, true, false, 0).unwrap();
        loop {
            let r = s.step(5_000_000);
            if r.status != Status::Running {
                return r;
            }
            assert!(r.attempts < 5e9, "runaway search in test");
        }
    }

    #[test]
    fn solves_generated_puzzles_on_all_paths() {
        for kind in crate::paths::PATH_KINDS {
            let p = generate(4, 4, 11);
            let r = solve(&p, kind);
            assert_eq!(r.status, Status::Solved, "path {kind} failed");
            assert_eq!(r.placed, 16);
        }
    }

    #[test]
    fn solved_board_has_max_score() {
        let p = generate(5, 5, 3);
        let path = build_path("snake", 5, 5, 0).unwrap();
        let mut s = Solver::new(&p, &path, true, false, 0).unwrap();
        while s.step(1_000_000).status == Status::Running {}
        assert_eq!(score_board(&p, s.board()), p.max_score());
    }

    #[test]
    fn impossible_puzzle_exhausts() {
        // Color 2 appears on exactly one edge in the whole set, so it can
        // never be matched: no arrangement of these corners tiles the 2x2.
        let p = Puzzle {
            name: "impossible".into(),
            width: 2,
            height: 2,
            num_colors: 2,
            pieces: vec![[0, 1, 1, 0], [0, 0, 1, 2], [1, 1, 0, 0], [1, 0, 0, 1]],
            hints: vec![],
        };
        let path = build_path("row-major", 2, 2, 0).unwrap();
        let mut s = Solver::new(&p, &path, true, false, 0).unwrap();
        let r = s.step(1_000_000);
        assert_eq!(r.status, Status::Exhausted);
    }

    #[test]
    fn official_hints_preplace_and_search_advances() {
        let p = crate::official::official_puzzle();
        assert_eq!(p.pieces.len(), 256);
        assert_eq!(p.hints.len(), 5);
        assert_eq!(p.num_colors, 22);
        let path = build_path("row-major", 16, 16, 0).unwrap();
        let mut s = Solver::new(&p, &path, true, false, 0).unwrap();
        let r = s.step(2_000_000);
        assert_eq!(r.status, Status::Running);
        // Naive lexicographic DFS thrashes shallow on the official set; any
        // meaningful progress past the hint count (5) proves the machine runs.
        assert!(r.best_placed > 30, "best {}", r.best_placed);
        // Hinted cells stay pinned.
        for h in &p.hints {
            assert_eq!(
                s.board()[h.pos as usize],
                (h.piece as i32) * 4 + i32::from(h.rot)
            );
        }
    }
}
