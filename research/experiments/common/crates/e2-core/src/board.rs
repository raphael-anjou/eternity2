//! The board: a fixed 16x16 grid of placements. A placement is a piece id plus
//! a clockwise rotation; empty cells hold [`EMPTY`].

use crate::{Pieces, Rot, EMPTY, N, W};

/// A 16x16 board of placements, row-major. Each cell is either a placed
/// `(piece_id, rot)` or empty. Cheap to clone (it is `N` small structs).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    /// Piece id per cell, [`EMPTY`] when unplaced.
    piece: [u16; N],
    /// Clockwise rotation per cell (meaningless where the cell is empty).
    rot: [Rot; N],
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    #[must_use]
    pub fn new() -> Self {
        Self {
            piece: [EMPTY; N],
            rot: [0; N],
        }
    }

    #[inline]
    #[must_use]
    pub fn is_empty_at(&self, pos: usize) -> bool {
        self.piece[pos] == EMPTY
    }

    #[inline]
    #[must_use]
    pub fn piece_at(&self, pos: usize) -> Option<(u16, Rot)> {
        if self.piece[pos] == EMPTY {
            None
        } else {
            Some((self.piece[pos], self.rot[pos]))
        }
    }

    #[inline]
    pub fn place(&mut self, pos: usize, piece: u16, rot: Rot) {
        self.piece[pos] = piece;
        self.rot[pos] = rot;
    }

    #[inline]
    pub fn clear(&mut self, pos: usize) {
        self.piece[pos] = EMPTY;
    }

    /// The board as the site/benchmark row-major `piece*4 + rot` vector, `-1`
    /// for empty cells. This is the wire format `e2-io` renders to a bucas URL
    /// and the JSON output.
    #[must_use]
    pub fn to_cell_codes(&self) -> Vec<i32> {
        (0..N)
            .map(|pos| {
                if self.piece[pos] == EMPTY {
                    -1
                } else {
                    i32::from(self.piece[pos]) * 4 + i32::from(self.rot[pos])
                }
            })
            .collect()
    }

    /// Build a board from a row-major `piece*4 + rot` vector (`-1` empty).
    #[must_use]
    pub fn from_cell_codes(codes: &[i32]) -> Self {
        let mut b = Self::new();
        for (pos, &v) in codes.iter().enumerate().take(N) {
            if v >= 0 {
                b.place(pos, (v / 4) as u16, (v % 4) as u8);
            }
        }
        b
    }

    /// Per-cell URDL edge colors, resolving rotation. Empty cells become
    /// `[0;4]` (all border), which never scores. This is exactly the grid the
    /// canonical scorer consumes.
    #[must_use]
    pub fn to_edge_cells(&self, pieces: &Pieces) -> Vec<[u8; 4]> {
        (0..N)
            .map(|pos| {
                if self.piece[pos] == EMPTY {
                    [0u8; 4]
                } else {
                    pieces
                        .get(self.piece[pos])
                        .map_or([0u8; 4], |p| p.rotated(self.rot[pos]))
                }
            })
            .collect()
    }

    /// FNV-1a hash of the placement grid — the bit-identical A/B gate the
    /// harness uses to detect two runs that produced the same board.
    #[must_use]
    pub fn hash(&self) -> u64 {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        for pos in 0..N {
            for b in self.piece[pos].to_le_bytes() {
                h ^= u64::from(b);
                h = h.wrapping_mul(0x0000_0100_0000_01b3);
            }
            h ^= u64::from(self.rot[pos]);
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        }
        h
    }

    /// (row, col) of a cell, row-major.
    #[inline]
    #[must_use]
    pub fn rc(pos: usize) -> (usize, usize) {
        (pos / W, pos % W)
    }
}
