//! A15 (round 3): non-chronological, bond-weighted unassignment on the hard
//! ladder. See `../PREREG.md`.

pub mod driver;
pub mod search;

pub use search::{grid, Rule, Stats, Unassign};
