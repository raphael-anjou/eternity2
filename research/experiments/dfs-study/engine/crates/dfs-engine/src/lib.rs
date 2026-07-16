//! The composable depth-first backtracker and the variant registry.
//!
//! # The one idea: a variant is a declared delta over a parent
//!
//! Every algorithm in the study is the SAME recursive DFS, parameterised by
//! four independently-toggleable strategy choices:
//!
//! ```text
//! Variant = { path: PathOrder, value: ValueOrder, propagate: Propagator, breaks: BreakPolicy }
//! ```
//!
//! A [`Spec`] names those four choices plus a `parent` and a one-line `delta`
//! ("BORDER-FIRST + forward-check"). The registry ([`registry`]) is the list of
//! specs; the site's "what stacks on what" matrix is generated from the deltas,
//! never hand-maintained. Adding a variant is adding one [`Spec`] to the
//! registry — no new search code, unless it introduces a genuinely new strategy
//! piece.
//!
//! This module is deliberately small and readable. The performance-engineered
//! baseline (`NAIVE-CODEGEN`) lives behind the same [`Spec`] interface so it can
//! be measured head-to-head with the clean one.

#![forbid(unsafe_code)]

mod geometry;
mod search;
mod spec;
mod strategy;

pub mod registry;

#[cfg(test)]
mod tests;

pub use geometry::Geometry;
pub use search::{run, RunConfig, RunResult};
pub use spec::{Spec, SpecKind};
pub use strategy::breaks::{BreakPolicy, BreakStep, BLACKWOOD_SCHEDULE, VERHAARD_SLIP_SCHEDULE};
pub use strategy::path::PathOrder;
pub use strategy::propagate::Propagator;
pub use strategy::value::ValueOrder;

pub use registry::{find, all_specs};
