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
pub mod generator;
mod piece;
mod score;
mod stats;

pub use board::Board;
pub use generator::{generate, generate_framed, generate_solved, generate_solved_framed,
    max_colors, GeneratedPuzzle};
pub use piece::{Piece, Pieces, EMPTY};
pub use score::{score_board, score_cells, MAX_SCORE, MAX_SCORE_16};
pub use stats::SearchStats;

/// The border color. A border-facing edge carries color 0 and never scores.
pub const BORDER: u8 = 0;

/// Board width/height. Kept a compile-time constant so the hot loop uses
/// fixed-size arrays (`[T; N]`) and elided bounds checks — a runtime size
/// parameter would cost 10–30% on this backtracker. Multi-size support (for the
/// Hint Study's scaling axis) is therefore a **compile-time feature**: exactly
/// one of the `size-N` features selects the const values below, and a
/// size-specialised binary is built per size. The default (no feature, or
/// `size-16`) is the official 16×16 board, so every existing consumer
/// (dfs-study, repair-study) compiles unchanged with zero runtime cost.
///
/// The size features are mutually exclusive. Enabling more than one — INCLUDING
/// `size-16` (which otherwise selects nothing, since 16 is the fallback) — is a
/// hard compile error, so a defensive `--features size-16` unified with another
/// crate's `size-8` across the workspace cannot silently flip the board size.
#[cfg(any(
    all(feature = "size-8", feature = "size-10"),
    all(feature = "size-8", feature = "size-12"),
    all(feature = "size-8", feature = "size-14"),
    all(feature = "size-8", feature = "size-16"),
    all(feature = "size-10", feature = "size-12"),
    all(feature = "size-10", feature = "size-14"),
    all(feature = "size-10", feature = "size-16"),
    all(feature = "size-12", feature = "size-14"),
    all(feature = "size-12", feature = "size-16"),
    all(feature = "size-14", feature = "size-16"),
))]
compile_error!("at most one e2-core `size-N` feature may be enabled at a time");

#[cfg(feature = "size-8")]
pub const W: usize = 8;
#[cfg(feature = "size-10")]
pub const W: usize = 10;
#[cfg(feature = "size-12")]
pub const W: usize = 12;
#[cfg(feature = "size-14")]
pub const W: usize = 14;
#[cfg(not(any(feature = "size-8", feature = "size-10", feature = "size-12", feature = "size-14")))]
pub const W: usize = 16;

/// Board height. The study's boards are square, so `H == W` at every size.
pub const H: usize = W;
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
