//! [`Spec`] — a repair variant declared as a delta over a parent. This is the
//! unit the registry lists and the site matrix is generated from, exactly
//! mirroring the DFS study's spec so the two studies read the same way.

use crate::strategy::{accept::Accept, destroy::Destroy, repair::Repair, restart::Restart, start::StartBoard};

/// A named repair variant: its five strategy choices, plus the parent it derives
/// from and the one-line delta it adds. The `delta` is what the site renders
/// into the "what stacks on what" matrix, so it must read as a single concrete
/// change over `parent`.
#[derive(Debug, Clone, Copy)]
pub struct Spec {
    /// Stable machine name, kebab-case (e.g. `greedy-mismatch`, `anneal-band`).
    pub name: &'static str,
    /// Display name, SCREAMING-CASE.
    pub display: &'static str,
    /// The variant this one is a delta over, by `name`. `None` for a root.
    pub parent: Option<&'static str>,
    /// One line: the single change this variant makes over `parent`.
    pub delta: &'static str,
    /// Which family the variant belongs to, for grouping on the page.
    pub family: Family,

    pub start: StartBoard,
    pub destroy: Destroy,
    pub repair: Repair,
    pub accept: Accept,
    pub restart: Restart,
}

/// The family a variant is grouped under on the study pages. Each family fixes
/// four axes and varies one, so its members are one change apart.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Family {
    /// The starting-board comparison (random vs greedy vs greedy-rare).
    Start,
    /// The destroy-operator comparison (random / mismatch / band / component).
    Destroy,
    /// The repair comparison (greedy / jittered / exact-small).
    Repair,
    /// The acceptance comparison (greedy / strict / annealing / late-acceptance).
    Accept,
    /// The restart / perturbation comparison (none / kick / revert-to-best).
    Restart,
}

impl Family {
    #[must_use]
    pub const fn tag(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Destroy => "destroy",
            Self::Repair => "repair",
            Self::Accept => "accept",
            Self::Restart => "restart",
        }
    }
}
