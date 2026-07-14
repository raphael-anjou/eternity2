#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod board;
pub mod path;
pub mod piece;
pub mod puzzle;

pub use board::{Board, CellAssignment};
pub use path::{Hint, Hints, PathPolicy};
pub use piece::{Color, Edges, Piece, PieceId, Rotation, BORDER};
pub use puzzle::{Position, Puzzle, PuzzleError};
