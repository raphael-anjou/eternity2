//! Break policy — whether the search may place a piece that mismatches an
//! already-placed neighbour ("break" an edge), and under what budget.
//!
//! Strict backtracking forbids all mismatches: a placement must match every
//! placed neighbour. The record community engines (Blackwood, Verhaard) instead
//! *release* mismatches on a depth schedule — at depth ≥ d, up to `budget`
//! cumulative interior mismatches are allowed, with a non-adjacency rule (no
//! cell incident to two broken edges). The score of a full board is then
//! `MAX - #breaks`.
//!
//! A local propagator is unsound under breaks (the budget is a global,
//! non-locally-spent resource), so break variants pair with `Propagator::None`.

/// One step of a depth-gated break schedule: at `depth >= at`, the cumulative
/// interior-break budget rises to `budget`.
#[derive(Debug, Clone, Copy)]
pub struct BreakStep {
    pub at: u32,
    pub budget: u32,
}

/// Whether and how the search tolerates edge mismatches.
#[derive(Debug, Clone, Copy)]
pub enum BreakPolicy {
    /// No mismatches ever. Score = matched edges. Keeps propagators sound.
    Strict,
    /// Depth-gated cumulative break budget with the non-adjacency rule. The
    /// schedule is a monotone list of `(depth, budget)` steps; `max_adjacent`
    /// caps how many broken edges may meet at one cell (1 = the classic
    /// non-adjacency rule; 2 = allow double-breaks, the axis the community
    /// 460/464 boards exploit). Border-facing edges never break.
    DepthGated {
        schedule: &'static [BreakStep],
        max_adjacent: u8,
    },
}

impl BreakPolicy {
    #[must_use]
    pub const fn allows_breaks(self) -> bool {
        matches!(self, BreakPolicy::DepthGated { .. })
    }

    /// The cumulative break budget permitted at `depth` under this policy.
    #[must_use]
    pub fn budget_at(self, depth: u32) -> u32 {
        match self {
            BreakPolicy::Strict => 0,
            BreakPolicy::DepthGated { schedule, .. } => {
                let mut b = 0;
                for step in schedule {
                    if depth >= step.at {
                        b = step.budget;
                    }
                }
                b
            }
        }
    }

    /// A short human tag for the site matrix.
    #[must_use]
    pub fn tag(self) -> &'static str {
        match self {
            BreakPolicy::Strict => "strict",
            BreakPolicy::DepthGated { max_adjacent: 1, .. } => "break (≤1/cell)",
            BreakPolicy::DepthGated { max_adjacent: 2, .. } => "break (≤2/cell)",
            BreakPolicy::DepthGated { .. } => "break",
        }
    }
}

/// Blackwood's decoded 469-feasible break schedule: mismatches unlock one at a
/// time on this depth ladder (`blackwood-algorithm`, verbatim break-index set
/// `[201,206,211,216,221,225,229,233,237,239,241,256]`, 12 breaks → max 469).
pub const BLACKWOOD_SCHEDULE: &[BreakStep] = &[
    BreakStep { at: 201, budget: 1 },
    BreakStep { at: 206, budget: 2 },
    BreakStep { at: 211, budget: 3 },
    BreakStep { at: 216, budget: 4 },
    BreakStep { at: 221, budget: 5 },
    BreakStep { at: 225, budget: 6 },
    BreakStep { at: 229, budget: 7 },
    BreakStep { at: 233, budget: 8 },
    BreakStep { at: 237, budget: 9 },
    BreakStep { at: 239, budget: 10 },
    BreakStep { at: 241, budget: 11 },
    BreakStep { at: 256, budget: 12 },
];

/// Verhaard's decoded interior edge-slip schedule (`R7-VERHAARD-RE`, recovered
/// slip array `{193:1, 202:2, 209:3, ... 240:12}`). Interior edges only; border
/// never slips.
pub const VERHAARD_SLIP_SCHEDULE: &[BreakStep] = &[
    BreakStep { at: 193, budget: 1 },
    BreakStep { at: 202, budget: 2 },
    BreakStep { at: 209, budget: 3 },
    BreakStep { at: 216, budget: 4 },
    BreakStep { at: 221, budget: 5 },
    BreakStep { at: 226, budget: 6 },
    BreakStep { at: 230, budget: 7 },
    BreakStep { at: 234, budget: 8 },
    BreakStep { at: 237, budget: 9 },
    BreakStep { at: 240, budget: 12 },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strict_never_allows_a_break() {
        assert_eq!(BreakPolicy::Strict.budget_at(0), 0);
        assert_eq!(BreakPolicy::Strict.budget_at(255), 0);
        assert!(!BreakPolicy::Strict.allows_breaks());
    }

    #[test]
    fn depth_gated_budget_is_monotone() {
        let p = BreakPolicy::DepthGated {
            schedule: BLACKWOOD_SCHEDULE,
            max_adjacent: 1,
        };
        assert_eq!(p.budget_at(200), 0);
        assert_eq!(p.budget_at(201), 1);
        assert_eq!(p.budget_at(205), 1);
        assert_eq!(p.budget_at(206), 2);
        assert_eq!(p.budget_at(256), 12);
        assert!(p.allows_breaks());
    }
}
