//! Per-cell edge constraints and candidate fitting — the placement arithmetic
//! every solver needs, promoted from the worked example so topics stop copying
//! it.
//!
//! The contract is the same one the canonical scorer uses: sides are URDL at
//! rotation 0, the rim wants [`BORDER`] (grey), and a neighbour's facing side
//! constrains this cell exactly when that neighbour is placed. `None` in a
//! constraint slot means "no constraint yet" (the neighbour cell is empty).

use e2_core::{Board, Pieces, BORDER};

/// What each URDL side of `(x, y)` must show for a *perfect* fit right now:
/// `Some(color)` where the rim or a placed neighbour constrains the side,
/// `None` where the neighbouring cell is still empty.
///
/// `width`/`height` are the board dimensions in cells; `x < width`,
/// `y < height`.
#[must_use]
pub fn edge_constraints(
    board: &Board,
    pieces: &Pieces,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> [Option<u8>; 4] {
    let at = |cx: usize, cy: usize| -> Option<[u8; 4]> {
        let pos = cy * width + cx;
        board
            .piece_at(pos)
            .map(|(pid, r)| pieces.get(pid).expect("piece id on board").rotated(r))
    };
    // Up: rim if y==0, else the down-edge of the piece above.
    let up = if y == 0 { Some(BORDER) } else { at(x, y - 1).map(|e| e[2]) };
    // Right: rim if x==width-1, else the left-edge of the piece to the right.
    let right = if x == width - 1 { Some(BORDER) } else { at(x + 1, y).map(|e| e[3]) };
    // Down: rim if y==height-1, else the up-edge of the piece below.
    let down = if y == height - 1 { Some(BORDER) } else { at(x, y + 1).map(|e| e[0]) };
    // Left: rim if x==0, else the right-edge of the piece to the left.
    let left = if x == 0 { Some(BORDER) } else { at(x - 1, y).map(|e| e[1]) };
    [up, right, down, left]
}

/// Score a candidate's edges `e` (already rotated) against the constraints:
/// `Some(matched_count)` if every constrained side matches, `None` if any
/// constrained side conflicts. Use for *hard-constraint* (perfect-fit) search.
#[must_use]
pub fn fit_score(e: &[u8; 4], want: &[Option<u8>; 4]) -> Option<u32> {
    let mut matched = 0;
    for (side, req) in want.iter().enumerate() {
        if let Some(c) = req {
            if e[side] == *c {
                matched += 1;
            } else {
                return None;
            }
        }
    }
    Some(matched)
}

/// Like [`fit_score`] but *soft*: counts matches and mismatches instead of
/// rejecting, for break-tolerant search. Returns `(matched, mismatched)` over
/// the constrained sides only.
#[must_use]
pub fn fit_counts(e: &[u8; 4], want: &[Option<u8>; 4]) -> (u32, u32) {
    let (mut ok, mut bad) = (0, 0);
    for (side, req) in want.iter().enumerate() {
        if let Some(c) = req {
            if e[side] == *c {
                ok += 1;
            } else {
                bad += 1;
            }
        }
    }
    (ok, bad)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::official_instance;

    #[test]
    fn official_solution_cells_fit_their_constraints() {
        // On the official instance, every pinned clue placed on an otherwise
        // empty board must satisfy its rim/neighbour constraints.
        let inst = official_instance(true);
        let board = inst.seed_board();
        let w = usize::from(inst.width);
        let h = usize::from(inst.height);
        for hint in &inst.hints {
            let pos = usize::from(hint.pos);
            let (x, y) = (pos % w, pos / w);
            let want = edge_constraints(&board, &inst.pieces, x, y, w, h);
            let e = inst.pieces.get(hint.piece).expect("hint piece").rotated(hint.rot);
            // The clue is already ON the seed board; its own cell reads as
            // occupied, so check the *edges agree with the rim* at least.
            for (side, req) in want.iter().enumerate() {
                if let Some(c) = req {
                    // A constraint present at a clue cell can only be the rim
                    // (clues are interior on the official board => no rim
                    // constraint) or a neighbouring clue; either way the
                    // placed solution edge must match it.
                    assert_eq!(e[side], *c, "clue at {} side {}", hint.pos, side);
                }
            }
        }
    }

    #[test]
    fn fit_score_rejects_conflicts_and_counts_matches() {
        let e = [1, 2, 3, 4];
        assert_eq!(fit_score(&e, &[None, None, None, None]), Some(0));
        assert_eq!(fit_score(&e, &[Some(1), None, Some(3), None]), Some(2));
        assert_eq!(fit_score(&e, &[Some(9), None, None, None]), None);
        assert_eq!(fit_counts(&e, &[Some(9), Some(2), None, None]), (1, 1));
    }
}
