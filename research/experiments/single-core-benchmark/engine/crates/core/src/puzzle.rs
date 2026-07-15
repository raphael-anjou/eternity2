use alloc::vec::Vec;
use core::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::piece::{Color, Piece, PieceId, BORDER};

// Row-major board position: y * width + x.
pub type Position = u32;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Puzzle {
    pub width: u32,
    pub height: u32,
    pub color_count: u32,
    pieces: Vec<Piece>,
 /// `pieces_by_id[id as usize]` is the index into `pieces`
    /// where piece `id` lives. Built at construction so `Puzzle::piece(id)`
    /// is O(1) instead of O(n) linear search (the old `iter().find`).
    ///
    /// PieceId is a u16 in the canonical E2 instance (256 pieces).
    /// The Vec is sized to `max_id + 1`; gaps in the id space remain
    /// u32::MAX (sentinel).
    #[cfg_attr(feature = "serde", serde(default))]
    pieces_by_id: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PuzzleError {
    DimensionMismatch { expected: usize, got: usize },
    EmptyBoard,
    DuplicatePieceId(PieceId),
    ColorOutOfRange { piece: PieceId, color: Color, max: Color },
}

impl fmt::Display for PuzzleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DimensionMismatch { expected, got } => {
                write!(f, "dimension mismatch: expected {expected} pieces, got {got}")
            }
            Self::EmptyBoard => f.write_str("board must have non-zero dimensions"),
            Self::DuplicatePieceId(id) => write!(f, "duplicate piece id {id}"),
            Self::ColorOutOfRange { piece, color, max } => {
                write!(f, "piece {piece} uses color {color} but max is {max}")
            }
        }
    }
}


impl Puzzle {
    pub fn new(width: u32, height: u32, color_count: u32, pieces: Vec<Piece>) -> Result<Self, PuzzleError> {
        if width == 0 || height == 0 {
            return Err(PuzzleError::EmptyBoard);
        }
        let expected = (width as usize) * (height as usize);
        if pieces.len() != expected {
            return Err(PuzzleError::DimensionMismatch { expected, got: pieces.len() });
        }
        // u8::MAX is the max representable color; color_count <= 255 enforced.
        let max_color = u8::try_from(color_count.saturating_sub(1)).unwrap_or(u8::MAX);
        for piece in &pieces {
            for &c in &piece.edges.as_array() {
                if c != BORDER && c > max_color {
                    return Err(PuzzleError::ColorOutOfRange {
                        piece: piece.id,
                        color: c,
                        max: max_color,
                    });
                }
            }
        }
        // Duplicate id check (small N; nested loop fine).
        for (i, p) in pieces.iter().enumerate() {
            for q in &pieces[i + 1..] {
                if p.id == q.id {
                    return Err(PuzzleError::DuplicatePieceId(p.id));
                }
            }
        }
 // build the O(1) id->index map.
        let max_id = pieces.iter().map(|p| u32::from(p.id)).max().unwrap_or(0);
        let mut pieces_by_id: Vec<u32> = Vec::with_capacity((max_id as usize) + 1);
        pieces_by_id.resize((max_id as usize) + 1, u32::MAX);
        for (idx, p) in pieces.iter().enumerate() {
            pieces_by_id[usize::from(p.id)] = idx as u32;
        }
        Ok(Self { width, height, color_count, pieces, pieces_by_id })
    }

    #[must_use]
    pub fn pieces(&self) -> &[Piece] { &self.pieces }

 /// O(1) piece lookup by id (was O(n) linear search via
    /// `iter().find`; profile showed score_board's 3-call-per-cell
    /// pattern made this O(n²) on a fully-placed board).
    #[must_use]
    pub fn piece(&self, id: PieceId) -> Option<&Piece> {
        let idx = *self.pieces_by_id.get(usize::from(id))?;
        if idx == u32::MAX {
            return None;
        }
        self.pieces.get(idx as usize)
    }

    #[must_use]
    pub fn cell_count(&self) -> u32 { self.width * self.height }

    // Convert position to (x, y).
    #[must_use]
    pub fn xy(&self, pos: Position) -> (u32, u32) {
        (pos % self.width, pos / self.width)
    }

    #[must_use]
    pub fn position(&self, x: u32, y: u32) -> Position {
        y * self.width + x
    }

    // Which edges of a cell touch the gray border. Order: [top, right, bottom, left].
    #[must_use]
    pub fn border_mask(&self, pos: Position) -> [bool; 4] {
        let (x, y) = self.xy(pos);
        [y == 0, x == self.width - 1, y == self.height - 1, x == 0]
    }

    // Stable SHA-256-style hash of the piece catalog (using a tiny FNV
    // for no-std purity; collision risk is negligible for our needs and
    // upgrading to SHA-256 in the wire layer is straightforward).
    #[must_use]
    pub fn fingerprint(&self) -> u64 {
        let mut h: u64 = 0xcbf29ce484222325;
        let prime: u64 = 0x100000001b3;
        let mix = |h: &mut u64, v: u64| {
            *h ^= v;
            *h = h.wrapping_mul(prime);
        };
        let mut state = h;
        mix(&mut state, u64::from(self.width));
        mix(&mut state, u64::from(self.height));
        mix(&mut state, u64::from(self.color_count));
        for p in &self.pieces {
            mix(&mut state, u64::from(p.id));
            for c in p.edges.as_array() {
                mix(&mut state, u64::from(c));
            }
        }
        h = state;
        h
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece::Edges;
    use alloc::vec;

    fn p(id: PieceId, t: Color, r: Color, b: Color, l: Color) -> Piece {
        Piece::new(id, Edges::new(t, r, b, l))
    }

    #[test]
    fn rejects_dim_mismatch() {
        let err = Puzzle::new(2, 2, 3, vec![p(0, 0, 1, 1, 0)]).unwrap_err();
        assert!(matches!(err, PuzzleError::DimensionMismatch { .. }));
    }

    #[test]
    fn rejects_duplicate_id() {
        let pieces = vec![
            p(0, 0, 1, 1, 0), p(0, 0, 0, 1, 1),
            p(2, 1, 0, 0, 1), p(3, 1, 1, 0, 0),
        ];
        let err = Puzzle::new(2, 2, 2, pieces).unwrap_err();
        assert!(matches!(err, PuzzleError::DuplicatePieceId(0)));
    }

    #[test]
    fn xy_roundtrip() {
        let pieces = (0..9).map(|i| p(i, 1, 1, 1, 1)).collect();
        let puz = Puzzle::new(3, 3, 2, pieces).unwrap();
        for pos in 0..9 {
            let (x, y) = puz.xy(pos);
            assert_eq!(puz.position(x, y), pos);
        }
    }

    #[test]
    fn border_mask_corners() {
        let pieces = (0..9).map(|i| p(i, 1, 1, 1, 1)).collect();
        let puz = Puzzle::new(3, 3, 2, pieces).unwrap();
        assert_eq!(puz.border_mask(0), [true, false, false, true]);   // top-left
        assert_eq!(puz.border_mask(2), [true, true, false, false]);   // top-right
        assert_eq!(puz.border_mask(6), [false, false, true, true]);   // bot-left
        assert_eq!(puz.border_mask(8), [false, true, true, false]);   // bot-right
        assert_eq!(puz.border_mask(4), [false, false, false, false]); // center
    }
}
