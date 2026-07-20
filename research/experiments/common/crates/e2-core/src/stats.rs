//! Search statistics — first-class in the search state, not bolted on after.
//!
//! These are the numbers the study raises for every run: node throughput, how
//! deep the search reached, and (for break variants) how many edge mismatches
//! the best board carries. Counters are plain fields the hot loop increments;
//! throughput is derived at the end from `nodes` and wall time.

/// Live counters a DFS updates as it searches. One instance per run.
#[derive(Debug, Clone, Default)]
pub struct SearchStats {
    /// Search nodes visited: one per attempted placement into a cell. This is
    /// the denominator of node/sec. Labelled `search-nodes/s` and never
    /// cross-compared across engine families (a propagating node is not a naive
    /// node).
    pub nodes: u64,
    /// Backtracks: one per retreat out of a cell after exhausting its
    /// candidates.
    pub backtracks: u64,
    /// Deepest placement depth reached at any point (number of cells filled).
    /// The study's "how far did it get" axis.
    pub max_depth: u32,
    /// Depth of the search frontier when the time budget expired. For a
    /// backtracker that never completes, this says where it was stuck.
    pub depth_at_timeout: u32,
    /// Number of edge mismatches ("breaks") on the best board found. Zero for a
    /// strict (constraint-respecting) variant; for a break variant the
    /// published score is `MAX - breaks`.
    pub breaks: u32,
    /// Search nodes visited at the instant the best board FIRST reached the
    /// maximum matched-edge score (a full solution). `0` means no full solution
    /// was found within the budget. This is the deterministic "size of the tree
    /// explored to solve" — the quantity the hint-geometry study compares, free
    /// of wall-clock variance.
    pub nodes_to_solution: u64,
    /// Wall-clock seconds elapsed when the first full solution was found. `0.0`
    /// if none. Reported alongside `nodes_to_solution` for context only; the
    /// node count is the machine-independent metric.
    pub secs_to_solution: f64,
}

impl SearchStats {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Record reaching `depth` (cells filled). Cheap to call on every descent.
    #[inline]
    pub fn observe_depth(&mut self, depth: u32) {
        if depth > self.max_depth {
            self.max_depth = depth;
        }
    }

    /// Node throughput in search-nodes per second over `elapsed_s` wall time.
    #[must_use]
    pub fn nodes_per_sec(&self, elapsed_s: f64) -> f64 {
        if elapsed_s > 0.0 {
            self.nodes as f64 / elapsed_s
        } else {
            0.0
        }
    }
}
