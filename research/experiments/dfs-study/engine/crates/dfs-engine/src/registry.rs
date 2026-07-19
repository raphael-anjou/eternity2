//! The variant registry: the declarative list of every DFS the study runs.
//!
//! Each entry is a [`Spec`] naming its four strategy choices plus its parent and
//! the one-line delta it adds. The site's "what stacks on what" matrix is
//! generated from these deltas. Adding a variant is adding an entry here.
//!
//! The families are laid out so that adjacent variants differ by exactly one
//! choice — that single-cause discipline is what lets the study attribute a
//! score or speed change to the thing that moved.

use crate::spec::{Family, Spec, SpecKind};
use crate::strategy::breaks::{BreakPolicy, BLACKWOOD_SCHEDULE, VERHAARD_SLIP_SCHEDULE};
use crate::strategy::path::PathOrder;
use crate::strategy::propagate::Propagator;
use crate::strategy::value::ValueOrder;

/// Shorthand for a strict, non-propagating, insertion-order engine variant —
/// the shape most path-family entries share (they differ only in `path`).
const fn strict_engine(
    name: &'static str,
    display: &'static str,
    parent: Option<&'static str>,
    delta: &'static str,
    family: Family,
    path: PathOrder,
) -> Spec {
    Spec {
        name,
        display,
        parent,
        delta,
        family,
        kind: SpecKind::Engine,
        path,
        value: ValueOrder::Insertion,
        propagate: Propagator::None,
        breaks: BreakPolicy::Strict,
        external_note: "",
    }
}

/// A strict engine variant with an explicit value order and propagator — the
/// shape the heuristic family uses (they share a path and differ by one
/// propagation/ordering choice).
#[allow(clippy::too_many_arguments)]
const fn heuristic_engine(
    name: &'static str,
    display: &'static str,
    parent: Option<&'static str>,
    delta: &'static str,
    path: PathOrder,
    value: ValueOrder,
    propagate: Propagator,
) -> Spec {
    Spec {
        name,
        display,
        parent,
        delta,
        family: Family::Heuristic,
        kind: SpecKind::Engine,
        path,
        value,
        propagate,
        breaks: BreakPolicy::Strict,
        external_note: "",
    }
}

/// An external community engine, cited as reference DFS. Not executed by the
/// study (it hardcodes its own instance); its break policy is depth-gated
/// Blackwood, its path row-major — recorded for the matrix only.
const fn community_ref(name: &'static str, display: &'static str, note: &'static str) -> Spec {
    Spec {
        name,
        display,
        parent: Some("break-1"),
        delta: "the community's own record engine (run as published, cited not re-run here)",
        family: Family::Community,
        kind: SpecKind::External,
        path: PathOrder::RowMajor,
        value: ValueOrder::Insertion,
        propagate: Propagator::None,
        breaks: BreakPolicy::DepthGated {
            schedule: BLACKWOOD_SCHEDULE,
            max_adjacent: 1,
        },
        external_note: note,
    }
}

/// Every variant in the study, in presentation order.
#[must_use]
pub fn all_specs() -> Vec<Spec> {
    let mut v = vec![
        // ---- Baselines: the rawest DFS, and the speed question ----
        strict_engine(
            "naive-clean",
            "NAIVE-CLEAN",
            None,
            "the rawest depth-first backtracker: row-major, no heuristics, no breaks",
            Family::Baseline,
            PathOrder::RowMajor,
        ),
        // NAIVE-CODEGEN is declared here but executed by the specialised codegen
        // backtracker (added with the speed-baseline task). Same semantics as
        // NAIVE-CLEAN; different engine.
        Spec {
            kind: SpecKind::Codegen,
            name: "naive-codegen",
            display: "NAIVE-CODEGEN",
            parent: Some("naive-clean"),
            delta: "same algorithm, a 16×16-specialised unrolled hot loop",
            family: Family::Baseline,
            ..strict_engine(
                "naive-codegen",
                "NAIVE-CODEGEN",
                Some("naive-clean"),
                "",
                Family::Baseline,
                PathOrder::RowMajor,
            )
        },
        // ---- Path family: fix everything but the fill order ----
        strict_engine(
            "rowmajor",
            "ROWMAJOR",
            Some("naive-clean"),
            "the row-major control (same as NAIVE-CLEAN, named for the path study)",
            Family::Path,
            PathOrder::RowMajor,
        ),
        strict_engine(
            "rowmajor-bottomup",
            "ROWMAJOR-BOTTOMUP",
            Some("rowmajor"),
            "fill bottom-to-top instead of top-to-bottom (Blackwood's scan direction)",
            Family::Path,
            PathOrder::RowMajorBottomUp,
        ),
        strict_engine(
            "spiral-in",
            "SPIRAL-IN",
            Some("rowmajor"),
            "fill the outer ring inward instead of row-major (the cloister spiral)",
            Family::Path,
            PathOrder::SpiralIn,
        ),
        strict_engine(
            "spiral-out",
            "SPIRAL-OUT",
            Some("spiral-in"),
            "spiral from the centre outward instead of inward",
            Family::Path,
            PathOrder::SpiralOut,
        ),
        strict_engine(
            "border-first",
            "BORDER-FIRST",
            Some("rowmajor"),
            "fill the whole border ring first, then the interior",
            Family::Path,
            PathOrder::BorderFirst,
        ),
        strict_engine(
            "verhaard-comb",
            "VERHAARD-COMB",
            Some("rowmajor"),
            "a horizontal band then vertical teeth (Verhaard's COMB order)",
            Family::Path,
            PathOrder::VerhaardComb { horiz_rows: 10 },
        ),
        strict_engine(
            "clue-rows-first",
            "CLUE-ROWS-FIRST",
            Some("rowmajor"),
            "row-major, but the official-E2 clue rows (2,8,13) are swept first",
            Family::Path,
            PathOrder::ClueRowsFirst,
        ),
        // ---- Heuristic family: fix the path, add one propagator/order ----
        // The chain starts from border-first (the frame is where the rare
        // colours live) and adds, in order: dynamic MRV cell selection, then a
        // value order, then forward-checking, then arc-consistency, then
        // per-colour all-different. Each entry changes exactly one thing.
        heuristic_engine(
            "border-mrv",
            "BORDER-MRV",
            Some("border-first"),
            "choose the most-constrained empty cell dynamically (MRV) instead of a fixed order",
            PathOrder::Mrv,
            ValueOrder::Insertion,
            Propagator::None,
        ),
        heuristic_engine(
            "mrv-rare",
            "MRV-RARE",
            Some("border-mrv"),
            "try pieces carrying globally-rare colours first (Selby/Riordan rarity)",
            PathOrder::Mrv,
            ValueOrder::RareColorFirst,
            Propagator::None,
        ),
        heuristic_engine(
            "mrv-fc",
            "MRV-FC",
            Some("border-mrv"),
            "add forward-checking: reject a placement that empties any neighbour's domain",
            PathOrder::Mrv,
            ValueOrder::Insertion,
            Propagator::ForwardCheck,
        ),
        heuristic_engine(
            "mrv-ac3",
            "MRV-AC3",
            Some("mrv-fc"),
            "extend the look-ahead to arc-consistency (AC-3) over the frontier",
            PathOrder::Mrv,
            ValueOrder::Insertion,
            Propagator::Ac3,
        ),
        heuristic_engine(
            "mrv-gacolor",
            "MRV-GACOLOR",
            Some("mrv-ac3"),
            "add Régin per-colour all-different reasoning on the remaining supply",
            PathOrder::Mrv,
            ValueOrder::Insertion,
            Propagator::GacolorAc3,
        ),
        // ---- Break family: the elite axis (score = 480 − #breaks) ----
        // Breaks run row-major with no propagator (a local propagator is
        // unsound under a global break budget). Each variant differs by the
        // break schedule and the per-cell adjacency cap.
        Spec {
            name: "break-1",
            display: "BREAK-1",
            parent: Some("rowmajor"),
            delta: "allow ≤1 broken edge per cell on a depth schedule (Blackwood's ladder)",
            family: Family::Break,
            kind: SpecKind::Engine,
            path: PathOrder::RowMajor,
            value: ValueOrder::Insertion,
            propagate: Propagator::None,
            breaks: BreakPolicy::DepthGated {
                schedule: BLACKWOOD_SCHEDULE,
                max_adjacent: 1,
            },
            external_note: "",
        },
        Spec {
            name: "break-2",
            display: "BREAK-2",
            parent: Some("break-1"),
            delta: "allow up to 2 broken edges at one cell (double-breaks the community 460s use)",
            family: Family::Break,
            kind: SpecKind::Engine,
            path: PathOrder::RowMajor,
            value: ValueOrder::Insertion,
            propagate: Propagator::None,
            breaks: BreakPolicy::DepthGated {
                schedule: BLACKWOOD_SCHEDULE,
                max_adjacent: 2,
            },
            external_note: "",
        },
        Spec {
            name: "verhaard-slip",
            display: "VERHAARD-SLIP",
            parent: Some("break-1"),
            delta: "Verhaard's interior edge-slip schedule instead of Blackwood's ladder",
            family: Family::Break,
            kind: SpecKind::Engine,
            path: PathOrder::RowMajor,
            value: ValueOrder::Insertion,
            propagate: Propagator::None,
            breaks: BreakPolicy::DepthGated {
                schedule: VERHAARD_SLIP_SCHEDULE,
                max_adjacent: 1,
            },
            external_note: "",
        },
        Spec {
            name: "blackwood-comb-break",
            display: "BLACKWOOD-COMB-BREAK",
            parent: Some("break-1"),
            delta: "run the Blackwood break ladder on Verhaard's COMB fill order",
            family: Family::Break,
            kind: SpecKind::Engine,
            path: PathOrder::VerhaardComb { horiz_rows: 10 },
            value: ValueOrder::Insertion,
            propagate: Propagator::None,
            breaks: BreakPolicy::DepthGated {
                schedule: BLACKWOOD_SCHEDULE,
                max_adjacent: 1,
            },
            external_note: "",
        },
        // ---- Community reference DFS (run as published, NOT on our variants) ----
        // These are external programs, cited as the high-end of the family. They
        // are not executed by this study's harness: each hardcodes its own
        // instance/threads, so it cannot read our ten corner-pinned variants.
        // Our break reimplementations above are the runnable, comparable stand-ins.
        community_ref(
            "mcgavin-c",
            "MCGAVIN-C",
            "the fastest DFS the community has measured (~295M nodes/s): a codegen backtracker \
             with Blackwood's break scheme. Runs on its own puzzle build, not our variants.",
        ),
        community_ref(
            "blackwood-cs",
            "BLACKWOOD-CS",
            "Joshua Blackwood's own C# record engine. Public, builds on .NET, but hardcodes its \
             256 pieces and thread count, so it runs the plain puzzle, not our corner-pinned grid.",
        ),
    ];

    // A sound-configuration guard: no published variant may pair breaks with a
    // strict-only propagator. Panics loudly at construction if one does.
    for s in &v {
        assert!(s.is_sound(), "variant {} is unsound: breaks + strict propagator", s.name);
    }

    // Stable, deterministic order.
    v.sort_by(|a, b| family_rank(a.family).cmp(&family_rank(b.family)));
    v
}

const fn family_rank(f: Family) -> u8 {
    match f {
        Family::Baseline => 0,
        Family::Path => 1,
        Family::Heuristic => 2,
        Family::Break => 3,
        Family::Community => 4,
    }
}

/// Find a variant by its machine name.
#[must_use]
pub fn find(name: &str) -> Option<Spec> {
    all_specs().into_iter().find(|s| s.name == name)
}
