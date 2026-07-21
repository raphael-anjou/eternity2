//! THE canonical scorer. Matched non-border adjacencies, counted right + down
//! per cell. Identical formula to the site engine's `score_board`, the
//! benchmark's `score_cells`, and `verify_bucas`. Every board in the study is
//! re-scored here; no engine's self-report is ever trusted.

use crate::{Board, Pieces, BORDER, H, W};

/// The maximum matched-edge score for the current board size:
/// `2*W*H - W - H` (= 480 at 16×16). Computed from the compile-time W/H, so it
/// is correct at every size the `size-N` features select.
pub const MAX_SCORE: u32 = (2 * W * H - W - H) as u32;

/// Deprecated name kept so the existing (16×16) consumers compile unchanged.
/// Prefer [`MAX_SCORE`]; this is the same value.
pub const MAX_SCORE_16: u32 = MAX_SCORE;

/// Score a grid of per-cell URDL edge colors. A right or down adjacency scores
/// 1 iff the two facing edges are equal AND non-border. This is the single
/// source of truth for board quality.
#[must_use]
pub fn score_cells(cells: &[[u8; 4]]) -> u32 {
    let mut score = 0;
    for y in 0..H {
        for x in 0..W {
            let a = cells[y * W + x];
            if x + 1 < W {
                let b = cells[y * W + x + 1];
                if a[1] == b[3] && a[1] != BORDER {
                    score += 1;
                }
            }
            if y + 1 < H {
                let b = cells[(y + 1) * W + x];
                if a[2] == b[0] && a[2] != BORDER {
                    score += 1;
                }
            }
        }
    }
    score
}

/// Score a board directly, resolving each placed piece's rotated edges.
#[must_use]
pub fn score_board(board: &Board, pieces: &Pieces) -> u32 {
    score_cells(&board.to_edge_cells(pieces))
}
