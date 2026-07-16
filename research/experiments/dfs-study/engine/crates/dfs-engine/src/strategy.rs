//! The four independently-toggleable strategy axes of the DFS. Each is a small
//! enum with cheap implementations; a [`crate::Spec`] picks one value from each.
//!
//! Keeping them as enums (not trait objects) means the search dispatches on a
//! `match` the optimiser can inline, and a variant is a plain `const` record —
//! which is what makes dozens of variants cheap and the registry declarative.

pub mod breaks;
pub mod path;
pub mod propagate;
pub mod value;
