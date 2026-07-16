//! The restart / perturbation policy: what to do when the loop stops making
//! progress. This is the fourth axis. The vault's clearest negative result is
//! that a plain ALNS run flattens within dozens of iterations and never moves
//! again — so a policy that notices the stall and kicks the board is the natural
//! thing to test against a run that just keeps grinding the same basin.

use crate::state::{Rng, State};

/// What the loop does after a run of non-improving iterations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Restart {
    /// Never restart: grind the same basin for the whole budget. The control that
    /// shows how much (or how little) a run gains after it first stalls.
    None,
    /// After `patience` iterations with no new global best, apply a random *kick*:
    /// destroy and randomly refill `kick` cells (unconditionally, not scored), to
    /// bump the search out of the current basin without discarding it entirely.
    Kick { patience: u64, kick: usize },
    /// After `patience` stalled iterations, restart the working board from the
    /// best-so-far (not from scratch): revert any accumulated sideways drift and
    /// re-attack the incumbent. A softer perturbation than a kick.
    RevertToBest { patience: u64 },
}

impl Restart {
    #[must_use]
    pub const fn tag(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Kick { .. } => "kick",
            Self::RevertToBest { .. } => "revert-best",
        }
    }

    /// The stall patience, or 0 if this policy never fires.
    #[must_use]
    pub const fn patience(self) -> u64 {
        match self {
            Self::None => 0,
            Self::Kick { patience, .. } | Self::RevertToBest { patience } => patience,
        }
    }

    /// Apply a random kick to `st`: unplace `kick` random non-hint cells and
    /// refill them with their own pieces in random rotations/positions. Used only
    /// by [`Self::Kick`]. Returns the number of cells perturbed.
    pub fn apply_kick(kick: usize, st: &mut State, is_hint: &[bool], rng: &mut Rng) -> usize {
        use e2_core::{H, W};
        let mut pool: Vec<usize> = (0..W * H).filter(|&p| !is_hint[p] && !st.is_empty_at(p)).collect();
        rng.shuffle(&mut pool);
        pool.truncate(kick);
        let mut pieces: Vec<u16> = pool.iter().filter_map(|&p| st.clear(p).map(|(pid, _)| pid)).collect();
        rng.shuffle(&mut pieces);
        for (&pos, &pid) in pool.iter().zip(pieces.iter()) {
            let rot = (rng.next_u64() % 4) as u8;
            st.place(pos, pid, rot);
        }
        pool.len()
    }
}
