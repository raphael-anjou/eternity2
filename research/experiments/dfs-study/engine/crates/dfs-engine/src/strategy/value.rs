//! Value order — the order in which a cell's candidate pieces are tried. Changes
//! which board a time-boxed search reaches first, not the search's correctness.
//!
//! The project's prior finding is sobering: no static within-bucket ordering reliably
//! beats insertion order once a global prune dominates (`R7-ORDERING`,
//! `fanout-sort`). The study keeps the orders anyway so that verdict is
//! *measured here*, not just cited.

/// How to order the candidate pieces at a cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueOrder {
    /// Piece-id insertion order. The baseline; deterministic and free.
    Insertion,
    /// Rarest-color-first: try pieces carrying globally scarce colors earlier,
    /// so scarce colors are committed while options remain (Selby/Riordan).
    RareColorFirst,
    /// Seeded shuffle of the candidate list — the control that says whether any
    /// deterministic order beats chance.
    RandomShuffle,
}

impl ValueOrder {
    /// A short human tag for the site matrix.
    #[must_use]
    pub const fn tag(self) -> &'static str {
        match self {
            ValueOrder::Insertion => "insertion",
            ValueOrder::RareColorFirst => "rare-color-first",
            ValueOrder::RandomShuffle => "random",
        }
    }
}
