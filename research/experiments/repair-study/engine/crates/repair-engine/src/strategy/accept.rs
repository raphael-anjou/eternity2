//! The acceptance criterion: after a destroy-and-repair produces a candidate
//! score, do we keep it or revert? This is the third axis, and the one the
//! vault has the sharpest prior on: on Eternity II plateaus the landscape is
//! dominated by equal-score moves, so a simulated-annealing temperature turned
//! out to be inert across a tenfold range. This study measures that directly by
//! putting greedy, annealing and late-acceptance side by side.

use crate::state::Rng;

/// Whether a candidate score replaces the working score.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Accept {
    /// Keep the candidate iff it does not lose score (`>= current`). Hill-climb
    /// with sideways moves allowed; the simplest rule and a strong baseline.
    GreedyEqual,
    /// Keep only strict improvements (`> current`). Refuses sideways moves, so it
    /// locks onto the first basin — the control that shows what sideways
    /// wandering is worth.
    StrictImprove,
    /// Metropolis / simulated-annealing: always keep an improvement; keep a
    /// worsening move of size `d` with probability `exp(-d / T)`, `T` cooling
    /// geometrically from `t0` toward `t_end` over the run.
    Annealing { t0: f64, t_end: f64 },
    /// Late-acceptance hill climbing: keep the candidate if it is at least as good
    /// as the working score was `len` iterations ago. A memory of recent history
    /// rather than a temperature — often the best simple acceptance rule, and a
    /// clean contrast to annealing on a plateau-dominated landscape.
    LateAcceptance { len: usize },
}

impl Accept {
    #[must_use]
    pub const fn tag(self) -> &'static str {
        match self {
            Self::GreedyEqual => "greedy-equal",
            Self::StrictImprove => "strict-improve",
            Self::Annealing { .. } => "annealing",
            Self::LateAcceptance { .. } => "late-acceptance",
        }
    }

    /// The late-acceptance history length, or 0 if this rule keeps no history.
    #[must_use]
    pub const fn history_len(self) -> usize {
        match self {
            Self::LateAcceptance { len } => len,
            _ => 0,
        }
    }

    /// Decide whether `candidate` replaces `current`. `progress` is the fraction
    /// of the time budget elapsed, in `[0,1]`, used by annealing to cool.
    /// `hist_ref` is the working score `len` iterations ago, for late-acceptance.
    #[must_use]
    pub fn keep(
        self,
        current: u32,
        candidate: u32,
        progress: f64,
        hist_ref: u32,
        rng: &mut Rng,
    ) -> bool {
        match self {
            Self::GreedyEqual => candidate >= current,
            Self::StrictImprove => candidate > current,
            Self::LateAcceptance { .. } => candidate >= current || candidate >= hist_ref,
            Self::Annealing { t0, t_end } => {
                if candidate >= current {
                    return true;
                }
                let t = t0 * (t_end / t0).powf(progress.clamp(0.0, 1.0));
                let d = f64::from(current - candidate);
                rng.chance((-d / t.max(1e-6)).exp())
            }
        }
    }
}
