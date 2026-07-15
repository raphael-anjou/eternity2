// EngineConfig factory methods (the "profiles" — 40+ const and fn
// factories that produce ready-to-go EngineConfig values for the
// registry).
//
// Each public profile here corresponds to a (solver_id, heuristic_profile)
// pair exposed in ListSolvers + server::service::instantiate. See
// proto/solver/v2/solver.proto for the registry comment.
//
// Moved here in T3 (split solver-engine/src/lib.rs into modules).

use crate::{
    EngineConfig, Parallelism, PathSkeleton, PropagatorConfig, ScanOrder,
    ValueOrder, VariableOrder,
};

impl EngineConfig {
    pub const BORDER_FIRST_LCV: Self = Self {
        variable_order: VariableOrder::BorderFirstMrv,
        value_order: ValueOrder::InsertionOrder,
        break_symmetry: false,
        parallelism: Parallelism::SingleThread,
        path_skeleton: None,
        scan_order: None,
        shuffle_within_blackwood_ties: false,
        propagators: PropagatorConfig {
            class_balance: true,
            parity: false,
            island: false,
            ac3: false,
            gacolor: false,
            multiset_equality: false,
            depth_threshold: None,
        },
    };

 /// Blackwood 2020 base profile (engine knobs only; the
    /// `BlackwoodSchedule` itself rides on `SolveOpts.blackwood_schedule`).
    /// Sets scan_order=RowMajorBottomUp + value_order=BlackwoodHeuristic.
    /// Pair with gacolor + AC-3 + (optionally) NS-1 propagation.
    pub const BLACKWOOD_BASE: Self = Self {
        value_order: ValueOrder::BlackwoodHeuristic,
        scan_order: Some(ScanOrder::RowMajorBottomUp),
        propagators: PropagatorConfig {
            gacolor: true,
            ac3: true,
            multiset_equality: true,
            ..Self::BORDER_FIRST_LCV.propagators
        },
        ..Self::BORDER_FIRST_LCV
    };

    pub const BLACKWOOD_BASE_PAR: Self = Self {
        parallelism: Parallelism::RootSplit { split_depth: 0 },
        ..Self::BLACKWOOD_BASE
    };

 /// "dumb Blackwood" profile suggested by external
    /// reviewer: bottom-up scan + heuristic value-order + schedule +
    /// break allowance + piece-uniqueness + simple edge forward-
    /// checking ONLY. Drops gacolor, AC-3, NS-1 because those exact-
    /// solution propagators are NOT obviously sound after a break-
    /// index allows a mismatch (gacolor's per-color alldiff
    /// feasibility, AC-3's support count, NS-1's multiset equality
    /// all assume exact-matching downstream). This is the
    /// minimum-viable Blackwood implementation we can run safely
    /// past the first break. Pair with `with_blackwood_schedule`.
    pub const BLACKWOOD_RAW: Self = Self {
        value_order: ValueOrder::BlackwoodHeuristic,
        scan_order: Some(ScanOrder::RowMajorBottomUp),
 //: class_balance also dropped. Profile showed 7.6%
        // self time in RAW. It IS break-sound (piece classes don't
        // change under a color mismatch), but the pruning value at
        // the depths Blackwood reaches (~80) is marginal. Better to
        // pay the cost where it earns it (full-pipeline profiles).
        propagators: PropagatorConfig {
            class_balance: false,
            ..Self::BORDER_FIRST_LCV.propagators
        },
        ..Self::BORDER_FIRST_LCV
    };

    pub const BLACKWOOD_RAW_PAR: Self = Self {
        parallelism: Parallelism::RootSplit { split_depth: 0 },
        ..Self::BLACKWOOD_RAW
    };

    // Experiment A: GAColor as the strong global propagator.
    pub const BORDER_FIRST_GACOLOR: Self = Self {
        propagators: PropagatorConfig {
            gacolor: true,
            ..Self::BORDER_FIRST_LCV.propagators
        },
        ..Self::BORDER_FIRST_LCV
    };

    pub const BORDER_FIRST_GACOLOR_PAR: Self = Self {
        parallelism: Parallelism::RootSplit { split_depth: 0 },
        propagators: PropagatorConfig {
            gacolor: true,
            ..Self::BORDER_FIRST_LCV.propagators
        },
        ..Self::BORDER_FIRST_LCV
    };

    // Experiment D: GAColor + AC-3.
    pub const GACOLOR_AC3: Self = Self {
        propagators: PropagatorConfig {
            gacolor: true,
            ac3: true,
            ..Self::BORDER_FIRST_LCV.propagators
        },
        ..Self::BORDER_FIRST_LCV
    };

    // Experiment E — true LCV value ordering combined with our best
    // propagation (gacolor + AC-3).
    pub const GACOLOR_AC3_LCV: Self = Self {
        value_order: ValueOrder::LeastConstraining,
        propagators: PropagatorConfig {
            gacolor: true,
            ac3: true,
            ..Self::BORDER_FIRST_LCV.propagators
        },
        ..Self::BORDER_FIRST_LCV
    };

    pub const GACOLOR_AC3_LCV_PAR: Self = Self {
        value_order: ValueOrder::LeastConstraining,
        parallelism: Parallelism::RootSplit { split_depth: 0 },
        propagators: PropagatorConfig {
            gacolor: true,
            ac3: true,
            ..Self::BORDER_FIRST_LCV.propagators
        },
        ..Self::BORDER_FIRST_LCV
    };

    // Experiment B' — CHESS revisited with AC-3 propagation.
    pub const CHESS_GACOLOR_AC3: Self = Self {
        variable_order: VariableOrder::BorderFirstChess,
        propagators: PropagatorConfig {
            gacolor: true,
            ac3: true,
            ..Self::BORDER_FIRST_LCV.propagators
        },
        ..Self::BORDER_FIRST_LCV
    };

    pub const GACOLOR_AC3_PAR: Self = Self {
        parallelism: Parallelism::RootSplit { split_depth: 0 },
        propagators: PropagatorConfig {
            gacolor: true,
            ac3: true,
            ..Self::BORDER_FIRST_LCV.propagators
        },
        ..Self::BORDER_FIRST_LCV
    };

    // Same as GACOLOR_AC3_PAR (deterministic MRV variable-ordering, both
    // propagators) but with a seeded-random VALUE order: domain rows are
    // shuffled at each node. Different seeds produce different CP partials,
    // which is needed when downstream local search saturates because every
    // seed of CP+greedy_fill converges to the same canonical (and globally
    // wrong) prefix. Keeping MRV intact preserves CP's pruning power; only
    // tie-breaks between equally-good piece choices are randomised.
    pub const GACOLOR_AC3_RANDOM_PAR: Self = Self {
        value_order: ValueOrder::RandomShuffle,
        parallelism: Parallelism::RootSplit { split_depth: 0 },
        propagators: PropagatorConfig {
            gacolor: true,
            ac3: true,
            ..Self::BORDER_FIRST_LCV.propagators
        },
        ..Self::BORDER_FIRST_LCV
    };

    // Single-core counterpart of GACOLOR_AC3_RANDOM_PAR — same seeded-random
    // value order, one thread. Used by the single-core benchmark grid where a
    // seed-diverse strong engine is needed so rounds produce real variance
    // (the deterministic presets return identical boards for every seed).
    pub const GACOLOR_AC3_RANDOM: Self = Self {
        parallelism: Parallelism::SingleThread,
        ..Self::GACOLOR_AC3_RANDOM_PAR
    };

    // Experiment C: GAColor + symmetry breaking.
    pub const GACOLOR_SYMBREAK: Self = Self {
        break_symmetry: true,
        propagators: PropagatorConfig {
            gacolor: true,
            ..Self::BORDER_FIRST_LCV.propagators
        },
        ..Self::BORDER_FIRST_LCV
    };

    pub const GACOLOR_SYMBREAK_PAR: Self = Self {
        break_symmetry: true,
        parallelism: Parallelism::RootSplit { split_depth: 0 },
        propagators: PropagatorConfig {
            gacolor: true,
            ..Self::BORDER_FIRST_LCV.propagators
        },
        ..Self::BORDER_FIRST_LCV
    };

    // Experiment B: CHESS + GAColor.
    pub const CHESS_GACOLOR: Self = Self {
        variable_order: VariableOrder::BorderFirstChess,
        propagators: PropagatorConfig {
            gacolor: true,
            ..Self::BORDER_FIRST_LCV.propagators
        },
        ..Self::BORDER_FIRST_LCV
    };

    pub const CHESS_GACOLOR_PAR: Self = Self {
        variable_order: VariableOrder::BorderFirstChess,
        parallelism: Parallelism::RootSplit { split_depth: 0 },
        propagators: PropagatorConfig {
            gacolor: true,
            ..Self::BORDER_FIRST_LCV.propagators
        },
        ..Self::BORDER_FIRST_LCV
    };

    pub const BORDER_FIRST_LCV_PAR: Self = Self {
        parallelism: Parallelism::RootSplit { split_depth: 0 },
        ..Self::BORDER_FIRST_LCV
    };

    pub const BORDER_FIRST_FULL_PAR: Self = Self {
        parallelism: Parallelism::RootSplit { split_depth: 0 },
        propagators: PropagatorConfig {
            parity: true,
            island: true,
            ..Self::BORDER_FIRST_LCV.propagators
        },
        ..Self::BORDER_FIRST_LCV
    };

    pub const RARE_COLOR_FIRST: Self = Self {
        variable_order: VariableOrder::RareColorFirst,
        value_order: ValueOrder::InsertionOrder,
        ..Self::BORDER_FIRST_LCV
    };

    pub const BORDER_FIRST_RANDOM: Self = Self {
        variable_order: VariableOrder::BorderFirstRandom,
        value_order: ValueOrder::InsertionOrder,
        ..Self::BORDER_FIRST_LCV
    };

 //: gacolor + AC-3 + NS-1 multiset equality (Hopfer 2022).
    pub const GACOLOR_AC3_NS1: Self = Self {
        propagators: PropagatorConfig {
            gacolor: true,
            ac3: true,
            multiset_equality: true,
            ..Self::BORDER_FIRST_LCV.propagators
        },
        ..Self::BORDER_FIRST_LCV
    };

    pub const GACOLOR_AC3_NS1_PAR: Self = Self {
        parallelism: Parallelism::RootSplit { split_depth: 0 },
        propagators: PropagatorConfig {
            gacolor: true,
            ac3: true,
            multiset_equality: true,
            ..Self::BORDER_FIRST_LCV.propagators
        },
        ..Self::BORDER_FIRST_LCV
    };

 //: Joe-Saunders 2026 — gacolor + AC-3, but extras only fire
    // at depth ≥ 150. Tries to bridge our ~2k nodes/sec to McGavin's
    // ~300M/sec by skipping per-node Step-8 work during early search.
    pub const JOE_DEPTH150: Self = Self {
        propagators: PropagatorConfig {
            gacolor: true,
            ac3: true,
            multiset_equality: true,
            depth_threshold: Some(150),
            ..Self::BORDER_FIRST_LCV.propagators
        },
        ..Self::BORDER_FIRST_LCV
    };

    pub const JOE_DEPTH150_PAR: Self = Self {
        parallelism: Parallelism::RootSplit { split_depth: 0 },
        propagators: PropagatorConfig {
            gacolor: true,
            ac3: true,
            multiset_equality: true,
            depth_threshold: Some(150),
            ..Self::BORDER_FIRST_LCV.propagators
        },
        ..Self::BORDER_FIRST_LCV
    };

 /// #1 — Joe-depth150 baseline + edge-color BP marginals as
    /// value-order. Caller must populate `SolveOpts.edge_bp_marginals`;
    /// see `eternity2_solver_engine::load_edge_bp_marginals`.
    pub const JOE_DEPTH150_BP: Self = Self {
        value_order: ValueOrder::EdgeBpMarginals,
        propagators: PropagatorConfig {
            gacolor: true,
            ac3: true,
            multiset_equality: true,
            depth_threshold: Some(150),
            ..Self::BORDER_FIRST_LCV.propagators
        },
        ..Self::BORDER_FIRST_LCV
    };

    pub const JOE_DEPTH150_BP_PAR: Self = Self {
        value_order: ValueOrder::EdgeBpMarginals,
        parallelism: Parallelism::RootSplit { split_depth: 0 },
        propagators: PropagatorConfig {
            gacolor: true,
            ac3: true,
            multiset_equality: true,
            depth_threshold: Some(150),
            ..Self::BORDER_FIRST_LCV.propagators
        },
        ..Self::BORDER_FIRST_LCV
    };

 /// joe_depth150_bp_par + auto-built hint-rectangle skeleton
    /// path (places the 4 outer hints + center via PathPolicy first).
    /// Requires `opts.hints` to have ≥4 hint positions. Empirically the
    /// strongest non-warm-started canonical-E2 single-process profile.
    pub const JOE_DEPTH150_BP_REC_PAR: Self = Self {
        value_order: ValueOrder::EdgeBpMarginals,
        parallelism: Parallelism::RootSplit { split_depth: 0 },
        path_skeleton: Some(PathSkeleton::HintRectangle),
        propagators: PropagatorConfig {
            gacolor: true,
            ac3: true,
            multiset_equality: true,
            depth_threshold: Some(150),
            ..Self::BORDER_FIRST_LCV.propagators
        },
        ..Self::BORDER_FIRST_LCV
    };

    /// Single-thread variant of JOE_DEPTH150_BP_REC_PAR.
    pub const JOE_DEPTH150_BP_REC: Self = Self {
        value_order: ValueOrder::EdgeBpMarginals,
        path_skeleton: Some(PathSkeleton::HintRectangle),
        propagators: PropagatorConfig {
            gacolor: true,
            ac3: true,
            multiset_equality: true,
            depth_threshold: Some(150),
            ..Self::BORDER_FIRST_LCV.propagators
        },
        ..Self::BORDER_FIRST_LCV
    };

 /// user-proposed: joe_depth150_bp + LAYERED rectangle skeleton.
    /// Path: rectangle perimeter → interior (centre-out) → annulus
    /// (row-by-row) → outer border. Engine is locked into this order
    /// for the entire search via PathPolicy::PrefixConstraint{k=256}.
    pub const JOE_DEPTH150_BP_REC_LAYERED_PAR: Self = Self {
        value_order: ValueOrder::EdgeBpMarginals,
        parallelism: Parallelism::RootSplit { split_depth: 0 },
        path_skeleton: Some(PathSkeleton::HintRectangleLayered),
        propagators: PropagatorConfig {
            gacolor: true,
            ac3: true,
            multiset_equality: true,
            depth_threshold: Some(150),
            ..Self::BORDER_FIRST_LCV.propagators
        },
        ..Self::BORDER_FIRST_LCV
    };

 /// user-proposed: joe_depth150_bp_par + X-skeleton path
    /// (two 3-cell-wide diagonals through the 5 canonical hints,
    /// then Chebyshev-outward fill from centre). Requires
    /// `opts.hints.len() >= 5`.
    pub const JOE_DEPTH150_BP_X_PAR: Self = Self {
        value_order: ValueOrder::EdgeBpMarginals,
        parallelism: Parallelism::RootSplit { split_depth: 0 },
        path_skeleton: Some(PathSkeleton::XSkeleton),
        propagators: PropagatorConfig {
            gacolor: true,
            ac3: true,
            multiset_equality: true,
            depth_threshold: Some(150),
            ..Self::BORDER_FIRST_LCV.propagators
        },
        ..Self::BORDER_FIRST_LCV
    };

    pub const JOE_DEPTH150_BP_REC_LAYERED: Self = Self {
        value_order: ValueOrder::EdgeBpMarginals,
        path_skeleton: Some(PathSkeleton::HintRectangleLayered),
        propagators: PropagatorConfig {
            gacolor: true,
            ac3: true,
            multiset_equality: true,
            depth_threshold: Some(150),
            ..Self::BORDER_FIRST_LCV.propagators
        },
        ..Self::BORDER_FIRST_LCV
    };

    // Step 8 profiles: baseline + extra propagators.
    pub const BORDER_FIRST_PARITY: Self = Self {
        propagators: PropagatorConfig {
            parity: true,
            ..Self::BORDER_FIRST_LCV.propagators
        },
        ..Self::BORDER_FIRST_LCV
    };

    pub const BORDER_FIRST_FULL: Self = Self {
        propagators: PropagatorConfig {
            parity: true,
            island: true,
            ..Self::BORDER_FIRST_LCV.propagators
        },
        ..Self::BORDER_FIRST_LCV
    };

    /// Verhaard 2008 — gacolor + AC-3 + PreferredFirst value ordering.
    /// Caller sets `SolveOpts.preferred_pieces` to (deferred ∪ worst-good)
    /// from phase-0 SA. See `solver-verhaard` crate.
    pub const VERHAARD_PREFERRED: Self = Self {
        value_order: ValueOrder::PreferredFirst,
        propagators: PropagatorConfig {
            gacolor: true,
            ac3: true,
            ..Self::BORDER_FIRST_LCV.propagators
        },
        ..Self::BORDER_FIRST_LCV
    };

    pub const VERHAARD_PREFERRED_PAR: Self = Self {
        value_order: ValueOrder::PreferredFirst,
        parallelism: Parallelism::RootSplit { split_depth: 0 },
        propagators: PropagatorConfig {
            gacolor: true,
            ac3: true,
            ..Self::BORDER_FIRST_LCV.propagators
        },
        ..Self::BORDER_FIRST_LCV
    };
}
