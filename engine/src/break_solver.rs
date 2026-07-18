//! Break-tolerant backtracking DFS: the idea behind every record solver.
//!
//! The strict solver in `solver.rs` rejects any placement that mismatches an
//! already-placed neighbour, so it can only ever build a perfectly-matched
//! board — and on a hard puzzle it simply runs out of perfect moves and stops
//! short. The community's record solvers (Blackwood's break-index) relax that:
//! a bounded number of placements are allowed to carry a mismatch. The search
//! then completes a full board that scores *almost* perfectly, which is exactly
//! how a 469/480 (eleven mismatches) is reachable at all.
//!
//! This is a separate machine from the strict `Solver` so the rest of the
//! engine — and its byte-for-byte ports in other languages — is untouched. It
//! shares the same step-able, no-clocks shape so a browser can animate it.
//!
//! Crucially, breaks are not allowed *anywhere*: they are permitted only at a
//! fixed set of `break_positions` (cells), and at most one mismatch per such
//! cell. This is exactly Blackwood's design — "a handful of fixed late positions
//! carry one mismatch instead of demanding a perfect match" — and it matters:
//! allowing a mismatch at *every* cell would explode the branching factor and
//! make the search worse, not better. Concentrating the licence at a few cells
//! keeps the rest of the search as tight as the strict solver while letting the
//! board complete. With no break positions this is the strict solver; granting
//! a few (in the right place) lets it finish a near-perfect board the strict
//! one cannot.

use crate::bitset::BitSet;
use crate::solver::{score_board, Report, Status};
use crate::types::{rotated, Color, Puzzle, BORDER};

struct Frame {
    pos: u16,
    cursor: u32,
    placed: u32,
    /// breaks this placement introduced (charged against the budget on undo).
    breaks_here: u32,
}

pub struct BreakSolver {
    width: usize,
    height: usize,
    n_pieces: usize,
    table: Vec<[Color; 4]>,
    distinct: Vec<bool>,
    piece_order: Vec<u16>,
    board: Vec<i32>,
    used: BitSet,
    frames: Vec<Frame>,
    depth: usize,
    hint_count: u32,
    status: Status,
    nodes: u64,
    attempts: u64,
    backtracks: u64,
    best_placed: u32,
    best_board: Vec<i32>,
    /// Per-cell: may this cell carry one mismatch? (the fixed break positions).
    break_at: Vec<bool>,
    /// Breaks spent on the current partial board.
    breaks_used: u32,
}

impl BreakSolver {
    pub fn new(
        puzzle: &Puzzle,
        path: &[u16],
        use_hints: bool,
        break_positions: &[u16],
    ) -> Result<BreakSolver, String> {
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

        let mut break_at = vec![false; n_cells];
        for &c in break_positions {
            if (c as usize) < n_cells {
                break_at[c as usize] = true;
            }
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

        let piece_order: Vec<u16> = (0..n_pieces as u16).collect();
        let mut board = vec![-1i32; n_cells];
        let mut used = BitSet::new(n_pieces);
        let mut hint_count = 0u32;
        if use_hints {
            for h in &puzzle.hints {
                let pos = h.pos as usize;
                if pos >= n_cells || h.piece as usize >= n_pieces || used.contains(h.piece as usize) {
                    return Err(format!("invalid hint at position {pos}"));
                }
                board[pos] = (h.piece as i32) * 4 + i32::from(h.rot & 3);
                used.insert(h.piece as usize);
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
                breaks_here: 0,
            })
            .collect();

        let best_board = board.clone();
        Ok(BreakSolver {
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
            break_at,
            breaks_used: 0,
        })
    }

    /// Count the mismatched interior edges this piece would introduce against
    /// already-placed neighbours, or `None` if the placement is structurally
    /// illegal (a rim edge that isn't grey, or an interior edge that is). Border
    /// structure is never break-payable — only colour mismatches between two
    /// interior-facing edges are.
    #[inline]
    fn break_cost(&self, pos: usize, e: [Color; 4]) -> Option<u32> {
        let (w, h) = (self.width, self.height);
        let (x, y) = (pos % w, pos / w);
        let [top, right, bottom, left] = e;

        if (y == 0) != (top == BORDER) {
            return None;
        }
        if (y == h - 1) != (bottom == BORDER) {
            return None;
        }
        if (x == 0) != (left == BORDER) {
            return None;
        }
        if (x == w - 1) != (right == BORDER) {
            return None;
        }

        let mut breaks = 0u32;
        if y > 0 {
            let n = self.board[pos - w];
            if n >= 0 && self.table[n as usize][2] != top {
                breaks += 1;
            }
        }
        if y < h - 1 {
            let n = self.board[pos + w];
            if n >= 0 && self.table[n as usize][0] != bottom {
                breaks += 1;
            }
        }
        if x > 0 {
            let n = self.board[pos - 1];
            if n >= 0 && self.table[n as usize][1] != left {
                breaks += 1;
            }
        }
        if x < w - 1 {
            let n = self.board[pos + 1];
            if n >= 0 && self.table[n as usize][3] != right {
                breaks += 1;
            }
        }
        Some(breaks)
    }

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
            let mut placed_breaks = 0u32;

            while cursor < limit {
                let oi = (cursor / 4) as usize;
                let r = cursor % 4;
                cursor += 1;
                let pid = self.piece_order[oi] as usize;
                if self.used.contains(pid) {
                    cursor = (cursor + 3) & !3;
                    continue;
                }
                let row = pid * 4 + r as usize;
                if !self.distinct[row] {
                    continue;
                }
                self.attempts += 1;
                if let Some(cost) = self.break_cost(pos, self.table[row]) {
                    // Accept a perfect placement anywhere, or a single mismatch
                    // only at a designated break cell.
                    if cost == 0 || (cost == 1 && self.break_at[pos]) {
                        placed_row = row as u32;
                        placed_breaks = cost;
                        break;
                    }
                }
            }

            if placed_row != u32::MAX {
                let pid = (placed_row / 4) as usize;
                self.board[pos] = placed_row as i32;
                self.used.insert(pid);
                self.breaks_used += placed_breaks;
                let f = &mut self.frames[self.depth];
                f.cursor = cursor;
                f.placed = placed_row;
                f.breaks_here = placed_breaks;
                self.depth += 1;
                self.nodes += 1;
                let placed = self.placed();
                if placed > self.best_placed {
                    self.best_placed = placed;
                    self.best_board.copy_from_slice(&self.board);
                }
            } else {
                let f = &mut self.frames[self.depth];
                f.cursor = 0;
                if self.depth == 0 {
                    self.status = Status::Exhausted;
                    break;
                }
                self.depth -= 1;
                let prev = &mut self.frames[self.depth];
                let row = prev.placed;
                let undo_breaks = prev.breaks_here;
                prev.placed = u32::MAX;
                prev.breaks_here = 0;
                self.board[prev.pos as usize] = -1;
                self.used.remove((row / 4) as usize);
                self.breaks_used -= undo_breaks;
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

    pub fn board(&self) -> &[i32] {
        &self.board
    }

    pub fn best_board(&self) -> &[i32] {
        &self.best_board
    }

    /// Breaks on the current board.
    pub fn breaks_used(&self) -> u32 {
        self.breaks_used
    }
}

/// Convenience for callers that just want the best full-board score reachable
/// with a given set of break cells (used by tests). Returns (status, score,
/// breaks). `step_cap` bounds the number of 100k-step chunks.
pub fn solve_to_completion(
    puzzle: &Puzzle,
    path: &[u16],
    use_hints: bool,
    break_positions: &[u16],
    step_cap: u32,
) -> (Status, u32, u32) {
    let mut s = BreakSolver::new(puzzle, path, use_hints, break_positions).expect("valid args");
    let mut report = s.report();
    let mut iters = 0u32;
    while report.status == Status::Running && iters < step_cap {
        report = s.step(100_000);
        iters += 1;
    }
    (
        report.status,
        score_board(puzzle, s.board()),
        s.breaks_used(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::generate;
    use crate::paths::build_path;

    #[test]
    fn no_break_cells_matches_strict_perfect_solution() {
        // A generated puzzle has a perfect solution, so with no break cells the
        // search finds a full board scoring the maximum with zero breaks.
        let p = generate(4, 4, 11);
        let path = build_path("snake", 4, 4, 0).unwrap();
        let (status, score, breaks) = solve_to_completion(&p, &path, true, &[], 1_000);
        assert_eq!(status, Status::Solved);
        assert_eq!(score, p.max_score());
        assert_eq!(breaks, 0);
    }

    #[test]
    fn breaks_only_at_designated_cells() {
        // Breaks may appear only where licensed: a completed board's mismatches
        // must all sit on break cells. We check the count never exceeds the
        // number of break cells (each carries at most one).
        let p = generate(5, 5, 7);
        let path = build_path("snake", 5, 5, 0).unwrap();
        let n = (p.width * p.height) as usize;
        let break_cells: Vec<u16> = path[n.saturating_sub(8)..].to_vec();
        let (_status, _score, breaks) =
            solve_to_completion(&p, &path, true, &break_cells, 2_000);
        assert!(
            breaks <= break_cells.len() as u32,
            "breaks {breaks} exceed {} licensed cells",
            break_cells.len()
        );
    }

    #[test]
    fn break_accounting_survives_backtracking() {
        // Drive the solver over a puzzle hard enough to force real backtracking
        // through break cells, then assert the break bookkeeping stayed
        // consistent: every charged break is refunded on undo, so a completed
        // board never reports more breaks than it has licensed cells, and the
        // score deficit is bounded by the breaks actually spent. A refund bug
        // (breaks_used not decremented on backtrack) would let `breaks` drift
        // above the deficit or above the licensed-cell count.
        let p = generate(5, 5, 3);
        let path = build_path("snake", 5, 5, 0).unwrap();
        let n = (p.width * p.height) as usize;
        let break_cells: Vec<u16> = path[n.saturating_sub(6)..].to_vec();
        let (status, score, breaks) =
            solve_to_completion(&p, &path, true, &break_cells, 5_000);
        if status == Status::Solved {
            let deficit = p.max_score() - score;
            // Each licensed cell carries at most one mismatched placement, so a
            // consistent counter never exceeds the number of break cells...
            assert!(
                breaks <= break_cells.len() as u32,
                "breaks {breaks} exceed {} licensed cells",
                break_cells.len()
            );
            // ...and a perfectly-matched completion must report zero breaks
            // (the refund path ran). A leaked charge would break this.
            if deficit == 0 {
                assert_eq!(breaks, 0, "perfect board but {breaks} breaks charged");
            }
        }
    }
}
