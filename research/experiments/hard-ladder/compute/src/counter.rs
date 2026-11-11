//! The common node metric.
//!
//! One node = one assignment of an oriented piece (piece, rotation) to an
//! empty cell made by the search, where the oriented piece agrees with every
//! already-assigned neighbour and with the rim (grey exactly on the outer
//! edges). It is counted when made, whether or not propagation later refutes
//! it. Rejected candidate tests, propagation prunes and undo steps are not
//! nodes; they go in the secondary counters.

/// Node counter handed to every solver, with the per-instance cap.
#[derive(Debug, Clone)]
pub struct NodeCounter {
    cap: u64,
    nodes: u64,
    /// Undo steps (solver-reported, secondary).
    pub backtracks: u64,
    /// Candidate edge tests including rejected ones (secondary, if reported).
    pub fit_tests: u64,
    /// Solver-native propagation work units (secondary, if reported).
    pub propagation: u64,
    /// Tried assignments that disagreed with an assigned neighbour or the rim.
    /// Must be zero for a solver to be eligible.
    pub inconsistent: u64,
    /// The wrapped solver's own node count, when it has one (equivalence tests).
    pub native_nodes: Option<u64>,
}

impl NodeCounter {
    #[must_use]
    pub const fn new(cap: u64) -> Self {
        Self {
            cap,
            nodes: 0,
            backtracks: 0,
            fit_tests: 0,
            propagation: 0,
            inconsistent: 0,
            native_nodes: None,
        }
    }

    /// Count one assignment. Returns `false` (and counts nothing) once the cap
    /// is reached: the solver must then stop and return
    /// [`crate::Outcome::CapReached`].
    #[inline]
    pub fn try_assign(&mut self) -> bool {
        if self.nodes >= self.cap {
            false
        } else {
            self.nodes += 1;
            true
        }
    }

    #[must_use]
    pub const fn nodes(&self) -> u64 {
        self.nodes
    }

    #[must_use]
    pub const fn cap(&self) -> u64 {
        self.cap
    }

    #[must_use]
    pub const fn remaining(&self) -> u64 {
        self.cap.saturating_sub(self.nodes)
    }

    /// For wrapped solvers that keep their own exact count of the common
    /// metric. A value above the cap makes the run censored.
    pub(crate) fn set_nodes(&mut self, nodes: u64) {
        self.nodes = nodes;
    }
}
