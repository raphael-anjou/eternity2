use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::piece::{PieceId, Rotation};
use crate::puzzle::{Position, Puzzle};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CellAssignment {
    pub position: Position,
    pub piece_id: PieceId,
    pub rotation: Rotation,
}

// In-memory board: dense array of Option<(piece_id, rotation)> indexed by
// position. Cheap to clone for snapshots in event streams.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Board {
    width: u32,
    height: u32,
    cells: Vec<Option<(PieceId, Rotation)>>,
}

impl Board {
    #[must_use]
    pub fn empty(puzzle: &Puzzle) -> Self {
        let n = (puzzle.width as usize) * (puzzle.height as usize);
        Self {
            width: puzzle.width,
            height: puzzle.height,
            cells: alloc::vec![None; n],
        }
    }

    #[must_use]
    pub fn width(&self) -> u32 { self.width }
    #[must_use]
    pub fn height(&self) -> u32 { self.height }
    #[must_use]
    pub fn cells(&self) -> &[Option<(PieceId, Rotation)>] { &self.cells }

    pub fn place(&mut self, pos: Position, piece_id: PieceId, rotation: Rotation) {
        self.cells[pos as usize] = Some((piece_id, rotation));
    }

    pub fn clear(&mut self, pos: Position) {
        self.cells[pos as usize] = None;
    }

    #[must_use]
    pub fn get(&self, pos: Position) -> Option<(PieceId, Rotation)> {
        self.cells[pos as usize]
    }

    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.cells.iter().all(Option::is_some)
    }

    #[must_use]
    pub fn assignments(&self) -> Vec<CellAssignment> {
        self.cells
            .iter()
            .enumerate()
            .filter_map(|(i, c)| c.map(|(id, rot)| CellAssignment {
                position: i as Position,
                piece_id: id,
                rotation: rot,
            }))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece::{Edges, Piece};

    #[test]
    fn empty_then_place_then_complete() {
        let pieces = (0..4).map(|i| Piece::new(i, Edges::new(0, 1, 1, 0))).collect();
        let puz = Puzzle::new(2, 2, 2, pieces).unwrap();
        let mut b = Board::empty(&puz);
        assert!(!b.is_complete());
        for pos in 0..4 {
            b.place(pos, pos as PieceId, Rotation::R0);
        }
        assert!(b.is_complete());
        assert_eq!(b.assignments().len(), 4);
    }
}
