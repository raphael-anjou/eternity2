//! The variant registry: the declarative list of every repair loop the study
//! runs. Each entry is a [`Spec`] naming its five strategy choices plus its
//! parent and the one-line delta it adds. The site's "what stacks on what"
//! matrix is generated from these deltas. Adding a variant is adding an entry.
//!
//! The families are laid out so that adjacent variants differ by exactly one
//! choice. A shared baseline anchors the tree: GREEDY-MISMATCH is a plain
//! destroy-and-repair loop (greedy start, mismatch-driven destroy, greedy
//! refill, keep-if-not-worse, no restart). Every other variant is that loop with
//! exactly one axis changed, so a score difference is attributable to the change.

use crate::spec::{Family, Spec};
use crate::strategy::accept::Accept;
use crate::strategy::destroy::Destroy;
use crate::strategy::repair::Repair;
use crate::strategy::restart::Restart;
use crate::strategy::start::StartBoard;

/// The shared anchor every family branches from: a greedy construction, then a
/// mismatch-driven destroy of a handful of cells, greedy refill, keep the result
/// unless it loses score, never restart. The plainest sensible repair loop.
const fn anchor() -> Spec {
    Spec {
        name: "greedy-mismatch",
        display: "GREEDY-MISMATCH",
        parent: None,
        delta: "the plain loop: greedy start, destroy mismatched cells, greedy refill, keep if not worse",
        family: Family::Destroy,
        start: StartBoard::GreedyConstruct,
        destroy: Destroy::MismatchedCells { k: 12 },
        repair: Repair::Greedy,
        accept: Accept::GreedyEqual,
        restart: Restart::None,
    }
}

/// Every variant in the study, in presentation order.
#[must_use]
pub fn all_specs() -> Vec<Spec> {
    let a = anchor();
    let mut v = vec![
        // ---- Destroy family: fix everything but WHERE we demolish ----
        a,
        Spec {
            name: "random-destroy",
            display: "RANDOM-DESTROY",
            parent: Some("greedy-mismatch"),
            delta: "destroy random cells instead of mismatched ones (geometry-blind control)",
            family: Family::Destroy,
            destroy: Destroy::RandomCells { k: 12 },
            ..a
        },
        Spec {
            name: "band-destroy",
            display: "BAND-DESTROY",
            parent: Some("greedy-mismatch"),
            delta: "destroy the worst band of two rows instead of scattered mismatched cells",
            family: Family::Destroy,
            destroy: Destroy::WorstBand { rows: 2 },
            ..a
        },
        Spec {
            name: "component-destroy",
            display: "COMPONENT-DESTROY",
            parent: Some("greedy-mismatch"),
            delta: "destroy one connected mismatch component plus a one-cell halo",
            family: Family::Destroy,
            destroy: Destroy::ComponentHalo { max: 16 },
            ..a
        },
        // ---- Repair family: fix the destroy, change HOW we rebuild ----
        Spec {
            name: "repair-jitter",
            display: "REPAIR-JITTER",
            parent: Some("greedy-mismatch"),
            delta: "break greedy-refill score ties with a seeded coin (controlled exploration)",
            family: Family::Repair,
            repair: Repair::GreedyJitterTies,
            ..a
        },
        // A small-hole greedy baseline: destroy at most six mismatched cells, so a
        // bounded exact refill can be a clean ONE-axis comparison against it (only
        // the repair changes; the destroy is identical and always inside the exact
        // size cap, so the exact solver actually runs every iteration).
        Spec {
            name: "repair-small",
            display: "REPAIR-SMALL",
            parent: Some("greedy-mismatch"),
            delta: "destroy at most six mismatched cells instead of twelve (small-hole baseline)",
            family: Family::Repair,
            destroy: Destroy::MismatchedCells { k: 6 },
            repair: Repair::Greedy,
            ..a
        },
        Spec {
            name: "repair-exact",
            display: "REPAIR-EXACT",
            parent: Some("repair-small"),
            delta: "rebuild the small hole exactly by bounded assignment instead of greedily",
            family: Family::Repair,
            destroy: Destroy::MismatchedCells { k: 6 },
            repair: Repair::ExactSmall { exact_max: 6 },
            ..a
        },
        // ---- Accept family: fix the loop, change WHEN we keep a move ----
        Spec {
            name: "accept-strict",
            display: "ACCEPT-STRICT",
            parent: Some("greedy-mismatch"),
            delta: "keep only strict improvements (no sideways moves)",
            family: Family::Accept,
            accept: Accept::StrictImprove,
            ..a
        },
        Spec {
            name: "accept-anneal",
            display: "ACCEPT-ANNEAL",
            parent: Some("greedy-mismatch"),
            delta: "accept worsening moves under a cooling simulated-annealing temperature",
            family: Family::Accept,
            accept: Accept::Annealing { t0: 3.0, t_end: 0.2 },
            ..a
        },
        Spec {
            name: "accept-late",
            display: "ACCEPT-LATE",
            parent: Some("greedy-mismatch"),
            delta: "accept against the score 40 iterations ago (late-acceptance hill climbing)",
            family: Family::Accept,
            accept: Accept::LateAcceptance { len: 40 },
            ..a
        },
        // ---- Restart family: fix the loop, change what happens on a STALL ----
        Spec {
            name: "restart-kick",
            display: "RESTART-KICK",
            parent: Some("greedy-mismatch"),
            delta: "after 400 stalled iterations, randomly kick 24 cells to escape the basin",
            family: Family::Restart,
            restart: Restart::Kick { patience: 400, kick: 24 },
            ..a
        },
        Spec {
            name: "restart-revert",
            display: "RESTART-REVERT",
            parent: Some("restart-kick"),
            delta: "on a stall, revert to the best board so far instead of kicking (softer perturbation)",
            family: Family::Restart,
            restart: Restart::RevertToBest { patience: 400 },
            ..a
        },
        // ---- Start family: fix the loop, change the STARTING board ----
        Spec {
            name: "start-random",
            display: "START-RANDOM",
            parent: Some("greedy-mismatch"),
            delta: "start from a random board instead of a greedy construction",
            family: Family::Start,
            start: StartBoard::Random,
            ..a
        },
        Spec {
            name: "start-rare",
            display: "START-RARE",
            parent: Some("greedy-mismatch"),
            delta: "start from a rarest-color-first greedy construction (Selby/Riordan rarity)",
            family: Family::Start,
            start: StartBoard::GreedyRareFirst,
            ..a
        },
        // The construct-then-refine pipeline: spend the first 20 s of the budget on
        // the DFS study's break-DFS, then repair its board for the remaining 40 s.
        // Answers whether repair adds anything on top of a strong backtracked board.
        Spec {
            name: "start-dfs",
            display: "START-DFS",
            parent: Some("greedy-mismatch"),
            delta: "start from a 20 s break-DFS board, then repair it (construct-then-refine)",
            family: Family::Start,
            start: StartBoard::FromDfsBoard { dfs_budget_ms: 20_000 },
            ..a
        },
    ];

    // Stable, deterministic order by family, then original insertion order.
    v.sort_by_key(|s| family_rank(s.family));
    v
}

const fn family_rank(f: Family) -> u8 {
    match f {
        Family::Start => 0,
        Family::Destroy => 1,
        Family::Repair => 2,
        Family::Accept => 3,
        Family::Restart => 4,
    }
}

/// Find a variant by its machine name.
#[must_use]
pub fn find(name: &str) -> Option<Spec> {
    all_specs().into_iter().find(|s| s.name == name)
}
