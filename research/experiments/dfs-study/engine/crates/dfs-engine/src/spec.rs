//! [`Spec`] — a variant declared as a delta over a parent. This is the unit the
//! registry lists and the site matrix is generated from.

use crate::strategy::{breaks::BreakPolicy, path::PathOrder, propagate::Propagator, value::ValueOrder};

/// How a variant is executed. Most variants are the general composable engine
/// (`Engine`); the naive-codegen baseline is a separate 16×16-specialised hot
/// loop behind the same interface; community engines are external programs run
/// as published and reported with a "their code" caveat.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecKind {
    /// Run by the general composable DFS in this crate.
    Engine,
    /// Run by the dedicated, 16×16-specialised codegen backtracker. Same
    /// (path/value/propagate/breaks) semantics as its parent, but a hand-tuned
    /// hot loop that only makes sense for this fixed board — kept apart so the
    /// general engine stays readable.
    Codegen,
    /// An external community engine, run as published (its code, its number).
    /// Not executed by this crate; the harness shells out to it.
    External,
}

/// A named DFS variant: its four strategy choices, plus the parent it derives
/// from and the one-line delta it adds. The `delta` is what the site renders
/// into the "what stacks on what" matrix, so it must read as a single concrete
/// change over `parent`.
#[derive(Debug, Clone, Copy)]
pub struct Spec {
    /// Stable machine name, kebab-case (e.g. `border-first`, `break-1`).
    pub name: &'static str,
    /// Display name, SCREAMING-CASE (e.g. `BORDER-FIRST`).
    pub display: &'static str,
    /// The variant this one is a delta over, by `name`. `None` for a root
    /// (the rawest baseline).
    pub parent: Option<&'static str>,
    /// One line: the single change this variant makes over `parent`.
    pub delta: &'static str,
    /// Which family the variant belongs to, for grouping on the page.
    pub family: Family,
    pub kind: SpecKind,

    pub path: PathOrder,
    pub value: ValueOrder,
    pub propagate: Propagator,
    pub breaks: BreakPolicy,

    /// For `SpecKind::External`: a short note on how it is run and why it is not
    /// on the same axis (e.g. hardcoded pieces/cores). Empty otherwise.
    pub external_note: &'static str,
}

/// The family a variant is grouped under on the study pages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Family {
    /// The raw speed baselines (clean + codegen).
    Baseline,
    /// The path-order comparison (strict, minimal heuristics).
    Path,
    /// The heuristic comparison (fixed path, one propagator at a time).
    Heuristic,
    /// The break axis (strict vs depth-gated breaks; community reimpls).
    Break,
    /// External community engines, run as published.
    Community,
}

impl Family {
    #[must_use]
    pub const fn tag(self) -> &'static str {
        match self {
            Family::Baseline => "baseline",
            Family::Path => "path",
            Family::Heuristic => "heuristic",
            Family::Break => "break",
            Family::Community => "community",
        }
    }
}

impl Spec {
    /// Does this variant tolerate edge mismatches? Drives the "breaks" column
    /// the study raises for every row.
    #[must_use]
    pub fn allows_breaks(&self) -> bool {
        self.breaks.allows_breaks()
    }

    /// Guard: a variant that allows breaks must not also run a propagator that
    /// is only sound under strict placement. Checked once at registry build so a
    /// misconfigured variant fails fast rather than silently pruning reachable
    /// break branches.
    #[must_use]
    pub fn is_sound(&self) -> bool {
        !(self.allows_breaks() && self.propagate.requires_strict())
    }
}
