use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::piece::{PieceId, Rotation};
use crate::puzzle::Position;

// Interpretation of a user-supplied path. See V2_DESIGN.md §"PathPolicy".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PathPolicy {
    Strict,
    OrderingPrior,
    PrefixConstraint { k: u32 },
    Ignored,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Hint {
    pub position: Position,
    pub piece_id: PieceId,
    pub rotation: Rotation,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Hints {
    pub hints: Vec<Hint>,
}

impl Hints {
    #[must_use]
    pub fn new(hints: Vec<Hint>) -> Self { Self { hints } }

    #[must_use]
    pub fn at(&self, pos: Position) -> Option<Hint> {
        self.hints.iter().copied().find(|h| h.position == pos)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool { self.hints.is_empty() }
}
