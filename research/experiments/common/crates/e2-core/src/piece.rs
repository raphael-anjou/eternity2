//! The piece set. A piece is four edge colors at rotation 0, in URDL order.

use crate::{rotated, Rot};

/// Sentinel piece id meaning "no piece" in a placement slot.
pub const EMPTY: u16 = u16::MAX;

/// One puzzle piece: its four edge colors at rotation 0, URDL.
///
/// A piece is identified by its index in [`Pieces`]; the id is not stored on
/// the piece itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    /// Edge colors at rotation 0, URDL = (up, right, down, left).
    pub edges: [u8; 4],
}

impl Piece {
    #[inline]
    #[must_use]
    pub const fn new(edges: [u8; 4]) -> Self {
        Self { edges }
    }

    /// This piece's edges after a clockwise rotation.
    #[inline]
    #[must_use]
    pub fn rotated(&self, r: Rot) -> [u8; 4] {
        rotated(self.edges, r)
    }

    /// Number of border-colored (0) edges at rotation 0. Two for a corner, one
    /// for an edge piece, zero for an interior piece — rotation-invariant, so it
    /// classifies the piece.
    #[inline]
    #[must_use]
    pub fn border_edge_count(&self) -> u8 {
        self.edges.iter().filter(|&&c| c == crate::BORDER).count() as u8
    }
}

/// The full piece set for an instance, indexed by piece id.
#[derive(Debug, Clone)]
pub struct Pieces {
    pieces: Vec<Piece>,
    /// Highest interior color seen, for reference. Border color 0 excluded.
    max_color: u8,
}

impl Pieces {
    #[must_use]
    pub fn new(edges: Vec<[u8; 4]>) -> Self {
        let mut max_color = 0;
        for e in &edges {
            for &c in e {
                max_color = max_color.max(c);
            }
        }
        let pieces = edges.into_iter().map(Piece::new).collect();
        Self { pieces, max_color }
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.pieces.len()
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.pieces.is_empty()
    }

    #[inline]
    #[must_use]
    pub fn get(&self, id: u16) -> Option<&Piece> {
        self.pieces.get(id as usize)
    }

    #[inline]
    #[must_use]
    pub fn edges(&self, id: u16) -> [u8; 4] {
        self.pieces[id as usize].edges
    }

    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[Piece] {
        &self.pieces
    }

    /// Highest color id present (border 0 excluded from meaning, but included in
    /// the max if a piece somehow carried it).
    #[inline]
    #[must_use]
    pub fn max_color(&self) -> u8 {
        self.max_color
    }

    pub fn iter(&self) -> impl Iterator<Item = (u16, &Piece)> {
        self.pieces
            .iter()
            .enumerate()
            .map(|(i, p)| (i as u16, p))
    }
}
