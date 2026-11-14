//! The one work counter. Every entrant adapter charges its work here, at one
//! place per adapter, in the unit of its family:
//!
//! - `placement` (constructive engines): one placement of an oriented piece on
//!   an empty cell made by the search (`BreakSolver`'s own `nodes`, which is
//!   the hard ladder's node definition with a break licence).
//! - `move_eval` (local search): one proposed move whose score delta is
//!   evaluated and accepted or rejected (one iteration of the local-search
//!   crate's SA kernel; for ALNS, one step of its SA repair).
//!
//! The two units are not comparable to each other. Cross-family comparisons
//! use a pair of budgets fixed once by the exchange rate in CALIBRATION.md.

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Unit {
    Placement,
    MoveEval,
}

#[derive(Debug, Clone, Serialize)]
pub struct Work {
    pub unit: Unit,
    pub budget: u64,
    pub used: u64,
}

impl Work {
    #[must_use]
    pub const fn new(unit: Unit, budget: u64) -> Self {
        Self {
            unit,
            budget,
            used: 0,
        }
    }

    /// Charge `n` units. Panics if the budget would be exceeded: an adapter
    /// must ask for at most `remaining()` before doing the work.
    pub fn charge(&mut self, n: u64) {
        assert!(
            self.used + n <= self.budget,
            "work over budget: {} + {n} > {}",
            self.used,
            self.budget
        );
        self.used += n;
    }

    #[must_use]
    pub const fn remaining(&self) -> u64 {
        self.budget - self.used
    }
}
