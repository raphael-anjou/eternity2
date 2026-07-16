//! The shared substrate every DFS variant in the study is built on: the piece
//! set, the board, the ONE canonical scorer, and the search statistics.
//!
//! # Conventions (identical to the site engine and the sibling benchmark)
//!
//! Edges are **URDL** = (top/up, right, bottom/down, left). Rotation is a
//! clockwise quarter turn. A placed cell is `piece_id * 4 + rot`, row-major,
//! `-1` when empty. The **border color is 0**; a border-facing edge never
//! scores. These conventions are verified byte-for-byte against the benchmark's
//! `score_cells` by the parity test in `e2-io` (a known 469 board must score
//! 469 through this scorer too).
//!
//! Nothing here knows about files or search strategy — that is `e2-io` and
//! `dfs-engine` respectively. This crate is pure data + the scorer + the stat
//! counters, so the scorer can be trusted as the single source of truth.

#![forbid(unsafe_code)]

mod board;
mod piece;
mod score;
mod stats;

pub use board::Board;
pub use piece::{Piece, Pieces, EMPTY};
pub use score::{score_board, score_cells, MAX_SCORE_16};
pub use stats::SearchStats;

/// The border color. A border-facing edge carries color 0 and never scores.
pub const BORDER: u8 = 0;

/// Board width/height for the official puzzle. The study is 16x16 throughout;
/// keeping this a constant lets the hot loop use fixed-size arrays.
pub const W: usize = 16;
/// Board height (see [`W`]).
pub const H: usize = 16;
/// Cell count, `W * H`.
pub const N: usize = W * H;

/// A clockwise rotation in quarter turns, `0..=3`.
pub type Rot = u8;

/// Rotate URDL edges clockwise by `r` quarter turns. Matches the site engine's
/// `Edges::rotated` and the benchmark's `rotated`.
#[inline]
#[must_use]
pub fn rotated(e: [u8; 4], r: Rot) -> [u8; 4] {
    let [t, ri, b, l] = e;
    match r & 3 {
        0 => [t, ri, b, l],
        1 => [l, t, ri, b],
        2 => [b, l, t, ri],
        _ => [ri, b, l, t],
    }
}
