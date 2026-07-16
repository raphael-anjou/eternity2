//! The composable destroy-and-repair local searcher and the variant registry.
//!
//! # The one idea: a variant is a declared delta over a parent
//!
//! Every algorithm in the study is the SAME destroy-and-repair loop,
//! parameterised by five independently-toggleable strategy choices:
//!
//! ```text
//! Variant = {
//!   start:   StartBoard,  // the board the loop begins from
//!   destroy: Destroy,     // which cells to rip out each iteration
//!   repair:  Repair,      // how the hole is refilled
//!   accept:  Accept,      // whether a candidate replaces the working board
//!   restart: Restart,     // what to do on a stall
//! }
//! ```
//!
//! A [`Spec`] names those five choices plus a `parent` and a one-line `delta`
//! ("destroy the worst band instead of scattered cells"). The registry
//! ([`registry`]) is the list of specs; the site's "what stacks on what" matrix
//! is generated from the deltas, never hand-maintained. Adding a variant is
//! adding one [`Spec`] — no new loop code, unless it introduces a genuinely new
//! strategy piece.
//!
//! This is the deliberate sibling of the DFS study's engine. Both sit on the
//! shared `e2-core` / `e2-io` library: the board a repair loop polishes is the
//! same board a backtracker fills, scored by the same canonical scorer, so a
//! score from either study means the same thing.

#![forbid(unsafe_code)]

mod search;
mod spec;
mod state;
mod stats;
mod strategy;

pub mod registry;

#[cfg(test)]
mod tests;

pub use search::{run, RunConfig, RunResult};
pub use spec::{Family, Spec};
pub use stats::{RepairStats, CURVE_STRIDE};
pub use strategy::accept::Accept;
pub use strategy::destroy::Destroy;
pub use strategy::repair::Repair;
pub use strategy::restart::Restart;
pub use strategy::start::StartBoard;

pub use registry::{all_specs, find};
