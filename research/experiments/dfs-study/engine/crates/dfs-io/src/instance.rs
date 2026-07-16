//! The one instance and output types every study algorithm shares.

use dfs_core::{score_cells, Board, Pieces, Rot, MAX_SCORE_16};

use crate::bucas::board_to_bucas_url;

/// A pinned placement: piece `piece` at cell `pos`, clockwise rotation `rot`.
/// The five official clues and any corner pins are all just `Hint`s.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hint {
    pub pos: u16,
    pub piece: u16,
    pub rot: Rot,
}

/// A puzzle instance in the study's own shape: the piece set, the board
/// dimensions, and the pinned hints. Every converter targets or sources this
/// type, so it is the hub of the IO layer.
#[derive(Debug, Clone)]
pub struct Instance {
    /// A human name carried through to the bucas URL.
    pub name: String,
    pub width: u8,
    pub height: u8,
    /// Interior colors (border 0 excluded), for reference/reporting.
    pub num_colors: u8,
    /// The piece set, indexed by id.
    pub pieces: Pieces,
    /// Pinned placements.
    pub hints: Vec<Hint>,
}

impl Instance {
    /// Maximum matched-edge score: `2wh - w - h` (480 for 16x16).
    #[must_use]
    pub fn max_score(&self) -> u32 {
        let (w, h) = (u32::from(self.width), u32::from(self.height));
        2 * w * h - w - h
    }

    /// Apply the hints onto a fresh board — the starting point every search
    /// begins from.
    #[must_use]
    pub fn seed_board(&self) -> Board {
        let mut b = Board::new();
        for h in &self.hints {
            b.place(h.pos as usize, h.piece, h.rot);
        }
        b
    }

    /// The uniform output contract: canonically re-score a finished board,
    /// render its bucas URL, and package it. The engine's own score is never
    /// used here — [`score_cells`] is the single source of truth.
    #[must_use]
    pub fn finish(&self, board: &Board) -> SolveOutput {
        let cells = board.to_edge_cells(&self.pieces);
        let score = score_cells(&cells);
        SolveOutput {
            score,
            breaks: MAX_SCORE_16.saturating_sub(score),
            bucas_url: board_to_bucas_url(&self.name, self.width, self.height, &cells),
            board_hash: board.hash(),
            board: board.to_cell_codes(),
        }
    }
}

/// What every solver returns, ready to serialize. `board` is the row-major
/// `piece*4 + rot` vector (`-1` empty).
#[derive(Debug, Clone)]
pub struct SolveOutput {
    /// Canonical matched-edge count. The single source of truth.
    pub score: u32,
    /// `MAX - score`: the number of unmatched interior edges on this board.
    /// For a full 256-piece board this is exactly the break count.
    pub breaks: u32,
    pub bucas_url: String,
    pub board_hash: u64,
    pub board: Vec<i32>,
}
