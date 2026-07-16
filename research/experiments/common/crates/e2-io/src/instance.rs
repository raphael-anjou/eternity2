//! The one instance and output types every study algorithm shares.

use e2_core::{score_cells, Board, Pieces, Rot, MAX_SCORE_16};

use crate::format::{viewer_url, BoardDoc};

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
    /// render its canonical eternity2.dev viewer URL, and package it. The
    /// engine's own score is never used here — [`score_cells`] is the single
    /// source of truth.
    #[must_use]
    pub fn finish(&self, board: &Board) -> SolveOutput {
        let cells = board.to_edge_cells(&self.pieces);
        let score = score_cells(&cells);
        let codes = board.to_cell_codes();
        SolveOutput {
            score,
            breaks: MAX_SCORE_16.saturating_sub(score),
            url: viewer_url(&self.name, self.width, &cells, &codes),
            board_hash: board.hash(),
            cells,
            board: codes,
            name: self.name.clone(),
            size: self.width,
            max_score: self.max_score(),
        }
    }

    /// Rebuild a full board from decoded per-cell edge quads by matching each
    /// quad against this instance's piece set (recovering the piece id and
    /// rotation a board alone cannot carry), then package it with [`finish`].
    ///
    /// This is how an edge-only viewer URL becomes a *complete* canonical
    /// document: greedy first-fit matching, each piece used at most once — the
    /// same procedure the crate's cross-validation test uses to turn the known
    /// 469 board back into placements. An empty cell (`[0; 4]`) stays empty.
    #[must_use]
    pub fn match_board(&self, cells: &[[u8; 4]]) -> SolveOutput {
        let mut used = vec![false; self.pieces.len()];
        let mut board = Board::new();
        for (pos, &cell) in cells.iter().enumerate() {
            if cell == [0u8; 4] {
                continue; // empty / all-border cell
            }
            'find: for (pid, p) in self.pieces.iter() {
                if used[pid as usize] {
                    continue;
                }
                for r in 0..4 {
                    if p.rotated(r) == cell {
                        board.place(pos, pid, r);
                        used[pid as usize] = true;
                        break 'find;
                    }
                }
            }
        }
        self.finish(&board)
    }
}

/// What every solver returns, ready to serialize. `board` is the row-major
/// `piece*4 + rot` vector (`-1` empty). Its canonical serialized form is the
/// [`BoardDoc`] JSON produced by [`SolveOutput::to_doc`] — the one artifact
/// every algorithm on the site writes to disk.
#[derive(Debug, Clone)]
pub struct SolveOutput {
    /// Canonical matched-edge count. The single source of truth.
    pub score: u32,
    /// `MAX - score`: the number of unmatched interior edges on this board.
    /// For a full 256-piece board this is exactly the break count.
    pub breaks: u32,
    /// The canonical eternity2.dev viewer URL for this board.
    pub url: String,
    pub board_hash: u64,
    /// Row-major `piece*4 + rot` (`-1` empty).
    pub board: Vec<i32>,
    /// Per-cell URDL edge colors — kept so every derived format (JSON, CSV,
    /// bucas URL) can be produced without re-reading the piece set.
    pub cells: Vec<[u8; 4]>,
    /// Puzzle name carried into the URL and JSON.
    pub name: String,
    /// Board side length (square).
    pub size: u8,
    /// The instance's maximum matched-edge score, for `breaks` in the doc.
    pub max_score: u32,
}

impl SolveOutput {
    /// The canonical board document — the one JSON shape written to disk.
    #[must_use]
    pub fn to_doc(&self) -> BoardDoc {
        BoardDoc::new(
            &self.name,
            self.size,
            self.score,
            self.max_score,
            &self.cells,
            &self.board,
            self.board_hash,
        )
    }

    /// Serialize to the canonical pretty JSON.
    #[must_use]
    pub fn to_json(&self) -> String {
        self.to_doc().to_json()
    }

    /// Write the canonical `.json` — the single output artifact per board.
    pub fn write_json<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<()> {
        self.to_doc().write_json(path)
    }
}
