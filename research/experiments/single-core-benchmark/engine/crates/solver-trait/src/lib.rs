#![forbid(unsafe_code)]

use std::sync::Arc;

use eternity2_core::{Board, Hints, PathPolicy, Puzzle};
use eternity2_events::EventSink;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// Stable string identifiers. New solvers opt in by adding registry entries
// (see V2_DESIGN.md §"Resolved design decisions"). Strings are cheap to
// match in non-hot paths and avoid proto/registry coupling.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SolverId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HeuristicProfile(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SolveMode {
    FirstSolution,
    AllSolutions,
    CountOnly,
}

/// search objective. Distinct from `SolveMode` because `mode`
/// controls how many full solutions are collected; `Objective` controls
/// what we optimise *during* search. `None` = take whatever the search
/// produces (the behaviour). `Some(MaxScore)` = branch-and-bound
/// on matched-edge count, returning the best-scored board seen.
///
/// Under `MaxScore`, the engine:
///   1. never returns `Found` early on the first full board;
///   2. tracks `best_score_partial` across the whole search;
///   3. prunes any subtree whose `matched_count + remaining_upper_bound`
///      cannot beat the running best;
///   4. surfaces the best board via `SolveOutcome::Solved` on natural
///      exhaustion, or via `TimedOut.best_partial` on the budget cap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Objective {
    MaxScore,
}

#[derive(Debug, Clone)]
pub struct SolveOpts {
    pub mode: SolveMode,
    pub path: Vec<u32>,
    pub path_policy: PathPolicy,
    pub hints: Hints,
    pub seed: u64,
    pub time_budget_ms: u64,       // 0 = unlimited
    pub max_solutions: u32,        // 0 = unlimited (AllSolutions only)
    pub solver_run_id: u64,
    /// Pieces forbidden from use in this run. Empty = use all pieces.
    /// Set by `solver-verhaard` to defer "worst-performer" inner pieces
    /// during phase-1 scaffolding.
    pub excluded_pieces: Vec<eternity2_core::PieceId>,
    /// Pieces to prefer in value-ordering (only consulted when
    /// `EngineConfig.value_order == ValueOrder::PreferredFirst`).
    /// Rows whose piece-id is in this list are tried before others.
    /// Used by Verhaard's phase-1 to load "hard pieces" early.
    pub preferred_pieces: Vec<eternity2_core::PieceId>,
 /// Per-grid-edge color marginals from edge-color BP
    /// (`output/v12_bp/edge_bp_60i.json`). Flat layout
    /// `len = n_edges * 23` where `n_edges = 2*W*H + W + H` enumerated
    /// in the same row-major (y,x) order as the Python BP script:
    /// for each cell, claim N then S-if-last-row then W then
    /// E-if-last-col. Color 0 = BORDER. Wrapped in `Arc` so it is
    /// cheap to share across rayon workers when
    /// `Parallelism::RootSplit` clones `SolveOpts` per task.
    /// Only consulted when `EngineConfig.value_order ==
    /// ValueOrder::EdgeBpMarginals`.
    pub edge_bp_marginals: Option<Arc<Vec<f32>>>,
 /// when true, `apply_symmetry_and_hints` pins all hints
    /// first (in supplied order) and then runs one batched propagation
    /// pass. This avoids the issue where mid-application propagation
    /// removes a row that a later (still pending) hint needs, which
    /// happens routinely for hint sets ≥ ~50 cells extracted from a
    /// previous DFS partial. Used by the prune-restart driver.
    /// Default false (preserves existing per-hint propagation order).
    pub batch_hint_application: bool,
 /// when set, the engine searches for the board with the
    /// highest matched-edge count instead of returning the first valid
    /// completion. See [`Objective`] for the contract. Default `None`
 /// preserves FirstSolution behaviour. Honoured only by
    /// `EngineSolver`; legacy solvers ignore it.
    pub objective: Option<Objective>,
 /// records-prior value order. Per cell, a vector of
    /// `(piece_id, rotation, frequency)` from our 7 canonical-E2
    /// verified records. Length: `n_cells`. Each inner vec is sorted
    /// descending by frequency. Only consulted when
    /// `EngineConfig.value_order == ValueOrder::RecordsPrior`.
 /// Produced by `ml/structural_scan.py` (output/cell_value_order.json).
    /// Wrapped in `Arc` so it is cheap to share across rayon workers.
    pub records_prior_map: Option<Arc<Vec<Vec<(u16, u8, u32)>>>>,
 /// hard cap on engine node count (matching Joe's
    /// iteration-budgeted prune-restart triggering policy, msg
    /// #11725 in community corpus). 0 = unlimited (preserves
    /// existing behaviour). Engine honours this in the same
    /// check site as `time_budget_ms`.
    pub node_budget: u64,
}

impl Default for SolveOpts {
    fn default() -> Self {
        Self {
            mode: SolveMode::FirstSolution,
            path: Vec::new(),
            path_policy: PathPolicy::Ignored,
            hints: Hints::default(),
            seed: 0,
            time_budget_ms: 0,
            max_solutions: 0,
            solver_run_id: 0,
            excluded_pieces: Vec::new(),
            preferred_pieces: Vec::new(),
            edge_bp_marginals: None,
            batch_hint_application: false,
            objective: None,
            records_prior_map: None,
            node_budget: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SolveOutcome {
    Solved(Board),
    AllSolutions(Vec<Board>),
    Exhausted,
    TimedOut { best_partial: Board, best_depth: u32 },
    Cancelled { best_partial: Board, best_depth: u32, solutions_so_far: Vec<Board> },
    Error(String),
}

pub trait Solver: Send {
    fn id(&self) -> SolverId;
    fn heuristic_profile(&self) -> HeuristicProfile;

    fn supports_path_policy(&self, policy: &PathPolicy) -> bool;

    fn solve(
        &mut self,
        puzzle: &Puzzle,
        opts: &SolveOpts,
        sink: &mut dyn EventSink,
    ) -> SolveOutcome;
}
