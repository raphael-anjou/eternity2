// Engine configuration types: variable-order, value-order, scan-order,
// path skeleton, parallelism, propagator toggles, BlackwoodSchedule,
// PropagatorConfig, and the EngineConfig struct itself.
//
// Factory methods for EngineConfig (border_first_lcv, joe_depth150_bp,
// etc.) live in profiles.rs.
//
// Moved here in vol-33 T3 (split solver-engine/src/lib.rs into modules).

use eternity2_core::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariableOrder {
    BorderFirstMrv,        // corners > edges > inner, ties by smallest domain
    Mrv,                   // pure smallest-domain
    RareColorFirst,        // border-first, ties by globally rare colors
    BorderFirstRandom,     // border-first, ties by seeded random
    // CHESS heuristic from Ansótegui et al. CP'08:
    //   1. corners
    //   2. border edges
    //   3. interior "black" cells (checkerboard parity 0) in a spiral
    //      from center outward
    //   4. interior "white" cells (parity 1) in the same spiral
    // Static order; ties broken by domain size.
    BorderFirstChess,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueOrder {
    InsertionOrder,        // try rows in domain insertion order
    LeastConstraining,     // LCV — pick value that prunes fewest neighbor rows (v2.1)
    RandomShuffle,         // shuffle the domain via the opts.seed RNG at each node
    /// Stable partition: rows whose piece-id is in
    /// `SolveOpts.preferred_pieces` first, then the rest. Used by
    /// Verhaard's phase-1 to load "hard pieces" (deferred + worst
    /// good-set members) into early placements where the search has
    /// maximum flexibility. groups.io 105190116 (Verhaard 2008-04-11).
    PreferredFirst,
    /// Score each candidate row by Σ over its 4 sides of
    /// `SolveOpts.edge_bp_marginals[edge_id(cell, side)][row.edges[side]]`;
    /// sort descending so higher-probability colors are tried first.
    /// Vol-14 #1, port of vol-12's edge-color BP measurement
    /// (`output/v12_bp/edge_bp_60i.json`, 18.84% interior-edge
    /// entropy reduction; beat random as value-order in Python A/B).
    /// Falls back silently to InsertionOrder if marginals absent
    /// (preserves correctness in tests / smoke runs).
    EdgeBpMarginals,
    /// Vol-15 — Blackwood 2020 heuristic-side value ordering. Score
    /// each candidate row by `Σ_{side} 1[row.edges[side] ∈ schedule.heuristic_sides]`
    /// and sort descending so heuristic-rich candidates are tried first.
    /// Falls back silently to InsertionOrder when no schedule is
    /// attached (preserves correctness in tests).
    BlackwoodHeuristic,
    /// Vol-26 — Learned imitation-policy value order. Score each
    /// candidate row by an external Python subprocess running a small
    /// neural network trained on expert search trajectories. The bridge
    /// is spawned in `EngineSolver::new` when this variant is selected.
    /// Silently falls back to `InsertionOrder` if the bridge isn't
    /// available — preserves correctness for tests that don't ship the
    /// model. Per-node overhead: ~1-3 ms (JSON + inference). Acceptable
    /// at small puzzle sizes; not intended for canonical E2.
    Learned,
    /// Vol-30 — hybrid value order. Score candidates by EdgeBpMarginals
    /// first; reorder only the top-k tied/near-tied candidates with the
    /// Learned scorer. Cheaper per-node than `Learned` (we call the NN
    /// only when a tie-break matters), and theoretically dominated by
    /// EdgeBpMarginals when there are no ties. Vol-29 measurement showed
    /// the learned model picks productive tie-breakers
    /// (−35% nodes / −37% backtracks at iso-depth); this variant tests
    /// whether that signal turns into Δ > 0 depth on canonical E2.
    /// Requires both `opts.edge_bp_marginals` and a loaded Learned model.
    LearnedOnTies,
    /// Vol-41 — records-prior value order. At each cell, rank candidates
    /// by how many of our 7 verified canonical-E2 records have that
    /// (piece_id, rotation) at that position. Concept: records are a
    /// biased sample of high-score local optima; their per-cell
    /// distribution is a useful prior on "good" placements.
    /// Requires `SolveOpts.records_prior_map` (Vec of (cell → ranked
    /// variants)). Falls back to InsertionOrder if absent.
    RecordsPrior,
}

/// Vol-15 — Blackwood 2020 algorithm parameters. The backtracker is
/// constrained by:
///   1. A piecewise-linear schedule that demands a minimum count of
///      "heuristic-colored" edges placed by each depth — branches
///      that fall behind are pruned.
///   2. A fixed list of cell indices (in scan order) where ONE edge
///      mismatch is permitted during placement.
///
/// With 12 break opportunities the maximum-feasible score is `480 −
/// (12 − 1) = 469` on canonical E2 — this is the algorithm that
/// reaches community SOTA. See `V15_BLACKWOOD_SPEC.md`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlackwoodSchedule {
    /// Edge colors that count toward the exhaustion schedule.
    pub heuristic_sides: Vec<Color>,
    /// Piecewise-linear schedule control points: `(depth, target_count)`,
    /// sorted ascending by depth. The required count at intermediate
    /// depths is linearly interpolated.
    pub exhaustion_targets: Vec<(u32, u32)>,
    /// Total occurrences of `heuristic_sides` colors across all 256
    /// piece edges (≈ 120 for a canonical 5-clue E2 instance).
    pub heuristic_pool_size: u32,
    /// Maximum depth at which the schedule applies; beyond this, no
    /// schedule check (the algorithm has "earned its way" past the
    /// heuristic phase).
    pub max_heuristic_index: u32,
    /// Sorted ascending list of board cell indices (in the engine's
    /// scan order, i.e., after any ScanOrder remapping) where one
    /// placed-neighbor edge mismatch is permitted.
    pub break_indexes_allowed: Vec<u32>,
}

impl BlackwoodSchedule {
    /// Verify the schedule is well-formed: `exhaustion_targets` is
    /// non-empty, depths are STRICTLY increasing, counts are
    /// non-decreasing, all counts fit in `heuristic_pool_size`. A
    /// schedule that violates strict-monotonicity in depth produces
    /// a "cliff" where `target_at(d)` jumps discontinuously — the
    /// vol-15 bug your friend spotted in run_1778604403.
    /// Returns `Err(reason)` if malformed; `Ok(())` otherwise.
    pub fn validate(&self) -> Result<(), String> {
        if self.exhaustion_targets.is_empty() {
            return Err("exhaustion_targets is empty".into());
        }
        for win in self.exhaustion_targets.windows(2) {
            let (d0, c0) = win[0];
            let (d1, c1) = win[1];
            if d0 >= d1 {
                return Err(format!(
                    "exhaustion_targets depths not strictly increasing: ({d0},{c0}) followed by ({d1},{c1})"
                ));
            }
            if c0 > c1 {
                return Err(format!(
                    "exhaustion_targets counts decrease: ({d0},{c0}) followed by ({d1},{c1})"
                ));
            }
        }
        for &(_d, c) in &self.exhaustion_targets {
            if c > self.heuristic_pool_size {
                return Err(format!(
                    "exhaustion target {c} exceeds heuristic_pool_size {}",
                    self.heuristic_pool_size
                ));
            }
        }
        if !self.break_indexes_allowed.windows(2).all(|w| w[0] <= w[1]) {
            return Err("break_indexes_allowed not sorted ascending".into());
        }
        Ok(())
    }

    /// Target heuristic-piece-occurrence count at `depth`, computed
    /// by piecewise-linear interpolation of `exhaustion_targets`.
    /// Saturates at the schedule endpoints.
    pub fn target_at(&self, depth: u32) -> u32 {
        if self.exhaustion_targets.is_empty() { return 0; }
        let xs = &self.exhaustion_targets;
        if depth <= xs[0].0 { return xs[0].1; }
        if depth >= xs[xs.len() - 1].0 { return xs[xs.len() - 1].1; }
        for w in xs.windows(2) {
            let (d0, c0) = w[0];
            let (d1, c1) = w[1];
            if depth >= d0 && depth <= d1 {
                if d1 == d0 { return c1; }
                let span = (d1 - d0) as u64;
                let dc = (c1 as i64) - (c0 as i64);
                let off = (depth - d0) as u64;
                let interp = (c0 as i64) + ((dc * off as i64) / span as i64);
                return interp.max(0) as u32;
            }
        }
        xs[xs.len() - 1].1
    }
}

/// Vol-15 — scan order for the engine. Default `RowMajorTopDown`
/// matches the engine's historic indexing `idx = y*W + x`.
/// `RowMajorBottomUp` matches Blackwood's `idx = (H-1-y)*W + x`
/// (index 0 = bottom-left). The scan order is materialised as an
/// auto-built `path_order` when no explicit user path or
/// `path_skeleton` is set; the engine then uses
/// `PathPolicy::OrderingPrior`-style tie-breaking against the cell's
/// scan-order index.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanOrder {
    RowMajorTopDown,
    RowMajorBottomUp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parallelism {
    SingleThread,
    // Root-level work-stealing: enumerate partial placements to depth K,
    // dispatch one rayon task per partial state. Workers replay the
    // prefix to rebuild domains, then run the standard sequential
    // search. Same algorithm as the legacy v3_par parallel_dlx_solver.
    //
    // split_depth=0 ⇒ auto-pick from puzzle size + rayon threads, matching
    // the legacy heuristic. WASM target always falls back to SingleThread.
    RootSplit { split_depth: u32 },
}

/// Vol-16 Cat-2 Stage A — orthogonal sub-config for the optional
/// propagator stack. Baseline edge-color matching + piece-uniqueness
/// are ALWAYS on inside the engine; this struct only controls the
/// additional propagators that may run after baseline succeeds.
///
/// Stays `Copy` so `EngineConfig` const profile slabs can use FRU.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropagatorConfig {
    /// Cheap; corner/edge/inner piece-class counting. On by default for
    /// most profiles. Note: in `BLACKWOOD_RAW` it is OFF (vol-16 Cat-4b
    /// dropped it because Blackwood's depth ~80 wall doesn't reach the
    /// regime where class_balance pays its cost).
    pub class_balance: bool,
    /// Checkerboard color-balance pruning. Weakly pruning end-state
    /// check today (~0% on a 6×6 sample); strong incremental version is
    /// future work.
    pub parity: bool,
    /// Connectivity check: every unplaced piece has a remaining home
    /// somewhere. ~3% node reduction on 6×6 sample.
    pub island: bool,
    /// Maintain arc consistency on neighbouring-cell domains after
    /// each placement. Re-checks edges across already-pruned domains
    /// until fixpoint. Strictly stronger than baseline forward-
    /// checking; expensive at deep search.
    pub ac3: bool,
    /// GAColor — symmetric-alldiff feasibility per color. Strictly
    /// tighter than `parity` on the same problem; intended replacement.
    pub gacolor: bool,
    /// NS-1 / Hopfer 2022 multiset-equality. Necessary condition for
    /// any full solution: multiset of inward-facing colors on the 56
    /// edge-class cells equals multiset of border-facing colors on
    /// the 56 14×14-perimeter interior cells.
    pub multiset_equality: bool,
    /// Joe-Saunders 2026 depth-gate: skip the expensive Step-8
    /// propagators below this depth. AC-3 and edge/uniqueness still
    /// run at every depth. `None` = always run; empirical canonical-E2
    /// sweet spot ~150.
    pub depth_threshold: Option<u32>,
}

impl PropagatorConfig {
    /// All optional propagators off; only baseline edge-color + piece-
    /// uniqueness will run. Used by `BLACKWOOD_RAW`.
    pub const OFF: Self = Self {
        class_balance: false,
        parity: false,
        island: false,
        ac3: false,
        gacolor: false,
        multiset_equality: false,
        depth_threshold: None,
    };

    /// Default for most engine profiles: class_balance only.
    pub const CLASS_BALANCE_ONLY: Self = Self {
        class_balance: true,
        ..Self::OFF
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EngineConfig {
    pub variable_order: VariableOrder,
    pub value_order: ValueOrder,
    /// Optional propagator stack (vol-16 Cat-2 Stage A — extracted from
    /// 7 flat fields). Baseline edge-color + piece-uniqueness are
    /// always on regardless.
    pub propagators: PropagatorConfig,
    /// Break rotational symmetry by fixing the lowest-id corner piece at
    /// position (0,0) in its only valid rotation. Reduces search space
    /// by 4× on puzzles with 4 distinct corners (i.e., generated
    /// puzzles, which almost always satisfy this).
    pub break_symmetry: bool,
    pub parallelism: Parallelism,
    /// Vol-14 — automatic "skeleton path" that the engine pre-pins
    /// before falling back to its normal variable-ordering. When
    /// `Some(PathSkeleton::HintRectangle)`, `SearchState::new` builds
    /// a path that traces the rectangle through the 4 outermost hint
    /// positions + spokes to any remaining (interior) hints, then
    /// sets `path_order` + `path_index_of` so a `PathPolicy::PrefixConstraint`
    /// at `path.len()` is auto-injected. Empirically gives +44 placed cells
    /// vs default MRV on canonical E2 single-thread border_first_lcv. See
    /// `project_e2_vol14_rectangle_path_finding.md` (forthcoming).
    pub path_skeleton: Option<PathSkeleton>,
    /// Vol-15 — Blackwood 2020 scan order. `None` = inherit default
    /// `RowMajorTopDown` (engine's historic indexing). When set to
    /// `Some(RowMajorBottomUp)` and no explicit `opts.path` or
    /// `path_skeleton` is supplied, the engine auto-builds a full-board
    /// path that traverses cells in bottom-up row-major order; the
    /// schedule's `break_indexes_allowed` field then indexes into this
    /// path.
    pub scan_order: Option<ScanOrder>,
    /// Vol-17 (H22) — within-tie shuffle for BlackwoodHeuristic value
    /// order. When true, after sorting rows by heuristic-color count
    /// (descending), shuffle each run of equal-score rows using the
    /// seed-derived RNG. Gives schedule-invariant CP-partial diversity
    /// across seeds, without breaking Blackwood's score-descending
    /// invariant. Default: false (preserves existing deterministic
    /// tie-breaking).
    pub shuffle_within_blackwood_ties: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathSkeleton {
    /// Trace the rectangle through the 4 outermost hint positions
    /// (sorted by extremity: TL, TR, BR, BL), then a spoke from one
    /// rectangle side to any remaining hint cells (typically the
    /// center hint on canonical E2). The cells along the path are
    /// pinned in order via `PathPolicy::PrefixConstraint`.
    /// Requires `opts.hints` to have at least 4 hint positions.
    /// No-op if fewer than 4 hints.
    HintRectangle,
    /// Vol-14 user-proposed: same as HintRectangle but then continues
    /// the path by filling cells in layered order:
    ///   1. Rectangle skeleton (49 cells on canonical E2).
    ///   2. Interior of the rectangle, spiraling outward from center
    ///      (100 cells on canonical 16×16).
    ///   3. Annulus between rectangle and outer border, row-by-row
    ///      (~96 cells).
    ///   4. Outer border itself, corners → perimeter.
    /// The full path is `cell_count` long; PathPolicy effectively
    /// forces a specific scan order independent of MRV. Tests the
    /// hypothesis that maximum-constraint-first ordering improves
    /// search beyond the basic rectangle path.
    HintRectangleLayered,
    /// Vol-23 user-proposed: X-shape skeleton through the 5 canonical
    /// hints. Pre-commits cells along two 3-cell-wide diagonals that
    /// cross at the centre hint, so each subsequent diagonal cell has
    /// two already-placed neighbours (early constraint propagation).
    /// Order:
    ///   1. TL hint, then the 3-cell-wide diagonal band TL → centre.
    ///   2. Centre hint, then continue the same diagonal centre → BR.
    ///   3. TR hint, then the 3-cell-wide anti-diagonal TR → centre.
    ///   4. Centre → BL hint along the anti-diagonal.
    ///   5. Remaining cells: outward from centre (Chebyshev distance
    ///      ascending), within-tie row-major. Covers all `cell_count`
    ///      cells. Requires `opts.hints.len() >= 5`.
    /// Tests the hypothesis that two crossing constraint bands beat
    /// the rectangle's perimeter band on canonical E2.
    XSkeleton,
}
