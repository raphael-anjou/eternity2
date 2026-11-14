//! The one verifier. Input: the solver-visible puzzle and a board in the main
//! engine's encoding (row-major `piece * 4 + rotation`, rotation as
//! `eternity2_engine::rotated`). Output: the matched interior seams, or an
//! error. Errors: wrong length, an empty cell (missing piece), a piece id out
//! of range or used twice, a grey edge off the rim or a coloured edge on it.

use eternity2_engine::{rotated, Puzzle, BORDER};

pub fn verify(puzzle: &Puzzle, board: &[i32]) -> Result<u32, String> {
    let (w, h) = (puzzle.width as usize, puzzle.height as usize);
    let cells = w * h;
    if board.len() != cells || puzzle.pieces.len() != cells {
        return Err(format!("board has {} cells, expected {cells}", board.len()));
    }
    let mut used = vec![false; cells];
    let mut edges = Vec::with_capacity(cells);
    for (pos, &v) in board.iter().enumerate() {
        if v < 0 {
            return Err(format!("missing piece: cell {pos} empty"));
        }
        let (p, r) = ((v / 4) as usize, (v % 4) as u8);
        if p >= cells {
            return Err(format!("piece {p} out of range at cell {pos}"));
        }
        if used[p] {
            return Err(format!("piece {p} reused at cell {pos}"));
        }
        used[p] = true;
        edges.push(rotated(puzzle.pieces[p], r));
    }
    let mut score = 0u32;
    for y in 0..h {
        for x in 0..w {
            let [t, r, b, l] = edges[y * w + x];
            if (y == 0) != (t == BORDER)
                || (y == h - 1) != (b == BORDER)
                || (x == 0) != (l == BORDER)
                || (x == w - 1) != (r == BORDER)
            {
                return Err(format!("rim violated at ({x},{y})"));
            }
            if x + 1 < w && r == edges[y * w + x + 1][3] && r != BORDER {
                score += 1;
            }
            if y + 1 < h && b == edges[(y + 1) * w + x][0] && b != BORDER {
                score += 1;
            }
        }
    }
    Ok(score)
}

/// Score of a partial board (empty cells allowed, each seam counted only when
/// both sides are placed). Secondary column for constructive engines; never
/// enters the primary metric.
#[must_use]
pub fn partial_score(puzzle: &Puzzle, board: &[i32]) -> u32 {
    eternity2_engine::score_board(puzzle, board)
}
