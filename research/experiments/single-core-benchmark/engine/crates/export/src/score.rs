use eternity2_core::{Board, Puzzle, BORDER};

/// Count edge-matches between adjacent placed pieces on `board`.
/// Returns `(matched, total)` over horizontal + vertical neighbour pairs
/// that both have a piece placed. The canonical E2 maximum is 480
/// (240 horizontal + 240 vertical inner edges).
///
/// ** fix**: BORDER-BORDER matches between adjacent placed cells
/// do NOT count as `matched`. In a LEGAL board, BORDER edges face only
/// outside the puzzle; two adjacent placed cells with BORDER edges
/// touching would be illegal (the bf-bucket-bug class).
/// Excluding BORDER-BORDER from `matched` keeps the scorer honest for
/// illegal boards, matching the conservative behavior of
/// `eternity2_localsearch::alns::score_board`.
#[must_use]
pub fn score_board(puzzle: &Puzzle, board: &Board) -> (u32, u32) {
    let mut matched = 0u32;
    let mut total = 0u32;
    let w = puzzle.width;
    let h = puzzle.height;
    for y in 0..h {
        for x in 0..w {
            let pos = y * w + x;
            let Some((pid, rot)) = board.get(pos) else { continue; };
            let Some(p) = puzzle.piece(pid) else { continue; };
            let e = p.edges.rotated(rot).as_array();
            if x + 1 < w {
                if let Some((npid, nrot)) = board.get(y * w + (x + 1)) {
                    total += 1;
                    if let Some(np) = puzzle.piece(npid) {
                        let ne = np.edges.rotated(nrot).as_array();
                        if e[1] == ne[3] && e[1] != BORDER {
                            matched += 1;
                        }
                    }
                }
            }
            if y + 1 < h {
                if let Some((npid, nrot)) = board.get((y + 1) * w + x) {
                    total += 1;
                    if let Some(np) = puzzle.piece(npid) {
                        let ne = np.edges.rotated(nrot).as_array();
                        if e[2] == ne[0] && e[2] != BORDER {
                            matched += 1;
                        }
                    }
                }
            }
        }
    }
    (matched, total)
}

/// Total internal-edge count: every horizontal + vertical adjacency
/// inside the puzzle grid (regardless of whether the cells are placed).
/// On canonical 16×16 this is 480.
#[must_use]
pub const fn internal_edge_count(puzzle: &Puzzle) -> u32 {
    let w = puzzle.width;
    let h = puzzle.height;
    (w - 1) * h + w * (h - 1)
}

/// Number of cells with a piece placed on `board`.
#[must_use]
pub fn placed_count(board: &Board, puzzle: &Puzzle) -> u32 {
    let mut n = 0;
    for pos in 0..puzzle.cell_count() {
        if board.get(pos).is_some() {
            n += 1;
        }
    }
    n
}

/// Compact ASCII rendering of `board`. One line per row,
/// cell = `"piece_id:rot "` or `"----:- "` when empty.
#[must_use]
pub fn render_board(puzzle: &Puzzle, board: &Board) -> String {
    let mut out = String::new();
    for y in 0..puzzle.height {
        for x in 0..puzzle.width {
            let pos = y * puzzle.width + x;
            match board.get(pos) {
                Some((pid, rot)) => {
                    out.push_str(&format!("{:>4}:{} ", u32::from(pid), rot.as_u8()));
                }
                None => out.push_str("----:- "),
            }
        }
        out.push('\n');
    }
    out
}
