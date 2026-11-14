//! Partial-match ladder: planted boards on which score-maximising engines
//! stall below the optimum, one verifier, one work counter.
//!
//! - `instance`: the generator call (the hard ladder's framed generator and
//!   scramble, reused by path dependency) and the instance rows.
//! - `verify`: the only scorer whose number enters a result. A board with an
//!   empty cell, an out-of-range or reused piece, or a rim violation is an
//!   error, never a score.
//! - `work`: the one counter every entrant reports through.
//! - `entrants`: thin adapters around the repo's engines, unchanged.

#![forbid(unsafe_code)]

pub mod entrants;
pub mod instance;
pub mod verify;
pub mod work;
