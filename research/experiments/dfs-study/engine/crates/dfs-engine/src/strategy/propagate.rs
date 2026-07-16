//! Propagation — the look-ahead a variant runs after each placement to prune
//! doomed branches before descending into them. The study's heuristic axis.
//!
//! The order here is one of increasing cost per node: `None` (pure edge match)
//! is the naive baseline; `ForwardCheck` rejects a placement that leaves any
//! empty frontier cell with zero candidates; `Ac3` additionally arc-reduces
//! neighbouring domains; `GacolorAc3` adds Régin per-color all-different
//! reasoning on the remaining supply. Each buys pruning at a throughput cost —
//! the trade-off the study measures (node count is not score).

/// Which propagator a variant runs at each node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Propagator {
    /// No look-ahead beyond the edge-match at the placed cell. Fastest per node.
    None,
    /// Forward-check: after placing, reject if any adjacent empty cell now has
    /// no fitting piece. One-ply, cheap, sound under strict placement.
    ForwardCheck,
    /// Arc-consistency (AC-3) over the placed cell's neighbourhood, on top of
    /// forward-checking.
    Ac3,
    /// AC-3 plus Régin per-color all-different on the remaining piece supply.
    GacolorAc3,
}

impl Propagator {
    /// A short human tag for the site matrix.
    #[must_use]
    pub const fn tag(self) -> &'static str {
        match self {
            Propagator::None => "none",
            Propagator::ForwardCheck => "forward-check",
            Propagator::Ac3 => "ac3",
            Propagator::GacolorAc3 => "gacolor+ac3",
        }
    }

    /// Whether this propagator is sound only under strict (no-break) placement.
    /// Under breaks, a local look-ahead can prune a branch the global break
    /// budget could still rescue, so break variants must use `None`
    /// (established in the project's break-lookahead analysis).
    #[must_use]
    pub const fn requires_strict(self) -> bool {
        !matches!(self, Propagator::None)
    }
}
