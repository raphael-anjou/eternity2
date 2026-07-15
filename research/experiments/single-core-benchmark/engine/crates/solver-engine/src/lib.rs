// Unified search engine. Backtracking over per-position (piece, rotation)
// domains, with edge-color propagation and piece-uniqueness propagation
// always on. Variable and value ordering are configurable; additional
// propagators (parity, island, AC-3) will plug in here in Step 8.
//
// One engine + one Config covers everything that used to be split across
// "DLX" and "CSP" families. The two families' performance gap turned out
// to live entirely in propagation, not in the search strategy — so we
// model propagation as a first-class dimension of Config.

#![forbid(unsafe_code)]

#[cfg(all(not(target_arch = "wasm32"), feature = "learned-order"))]
mod bridge;

use std::sync::Arc;

use eternity2_time::Clock;
use eternity2_core::{Board, Color, PathPolicy, PieceId, Position, Puzzle, Rotation, BORDER};
use eternity2_events::{
    BacktrackCause, EventBody, EventSink, FinalStats, SelectionReason, SolverEvent,
};
use eternity2_propagators::{
    class_balance_check, gacolor_check, island_check, multiset_equality_check, parity_check,
    GaColorState, NeighborInfo,
    PlacementInfo, PropagatorContext, PropagatorResult,
};
use eternity2_solver_trait::{
    HeuristicProfile, Objective, SolveMode, SolveOpts, SolveOutcome, Solver, SolverId,
};

mod config;
pub use config::{
    BlackwoodSchedule, EngineConfig, Parallelism, PathSkeleton, PropagatorConfig, ScanOrder,
    ValueOrder, VariableOrder,
};

mod profiles;

pub struct EngineSolver {
    config: EngineConfig,
    solver_id: String,
    heuristic_profile: String,
 /// instrumentation: per-position backtrack counts from
    /// the most recent `solve()` call. Updated at end-of-solve.
    /// `take_pos_backtracks()` returns and clears this vector.
    last_pos_backtracks: std::sync::Mutex<Vec<u64>>,
 /// Blackwood 2020 schedule + break-index policy. Held
    /// on the solver (not in SolveOpts) so solver-trait does not
    /// have to know about Blackwood. Arc so rayon workers under
    /// `RootSplit` share a single allocation. Consulted when
    /// `config.value_order == ValueOrder::BlackwoodHeuristic`.
    pub(crate) blackwood_schedule: Option<Arc<BlackwoodSchedule>>,
}

impl EngineSolver {
    #[must_use]
    pub fn new(config: EngineConfig, solver_id: impl Into<String>, heuristic_profile: impl Into<String>) -> Self {
        Self {
            config,
            solver_id: solver_id.into(),
            heuristic_profile: heuristic_profile.into(),
            last_pos_backtracks: std::sync::Mutex::new(Vec::new()),
            blackwood_schedule: None,
        }
    }

 /// attach a Blackwood schedule to this solver. The
    /// schedule is read-only across the run; pass an `Arc` so rayon
    /// workers share the allocation. The engine consults the schedule
    /// only when `config.value_order == ValueOrder::BlackwoodHeuristic`.
    pub fn with_blackwood_schedule(mut self, schedule: Arc<BlackwoodSchedule>) -> Self {
        self.blackwood_schedule = Some(schedule);
        self
    }

 /// override the value-order of an existing engine profile.
    /// Useful for A/B testing alternate value-orders against a fixed
    /// variable-order + propagator stack.
    pub fn with_value_order(mut self, vo: ValueOrder) -> Self {
        self.config.value_order = vo;
        self
    }

    pub fn blackwood_schedule(&self) -> Option<Arc<BlackwoodSchedule>> {
        self.blackwood_schedule.clone()
    }

 /// Blackwood-mode constructor. Returns a solver with
    /// `BLACKWOOD_BASE_PAR` config and a Blackwood schedule attached.
    /// Mirrors `joe_depth150_bp_par()`'s pattern.
    #[must_use]
    pub fn blackwood_base_par(schedule: Arc<BlackwoodSchedule>) -> Self {
        Self::new(EngineConfig::BLACKWOOD_BASE_PAR, "engine", "blackwood_base_par")
            .with_blackwood_schedule(schedule)
    }

    #[must_use]
    pub fn blackwood_base(schedule: Arc<BlackwoodSchedule>) -> Self {
        Self::new(EngineConfig::BLACKWOOD_BASE, "engine", "blackwood_base")
            .with_blackwood_schedule(schedule)
    }

 /// "dumb Blackwood": no gacolor/AC-3/NS-1, just edge
    /// forward-checking + piece-uniqueness + schedule + breaks.
    /// Sound after break by construction (no exact-matching
    /// invariants).
    #[must_use]
    pub fn blackwood_raw_par(schedule: Arc<BlackwoodSchedule>) -> Self {
        Self::new(EngineConfig::BLACKWOOD_RAW_PAR, "engine", "blackwood_raw_par")
            .with_blackwood_schedule(schedule)
    }

    #[must_use]
    pub fn blackwood_raw(schedule: Arc<BlackwoodSchedule>) -> Self {
        Self::new(EngineConfig::BLACKWOOD_RAW, "engine", "blackwood_raw")
            .with_blackwood_schedule(schedule)
    }

 /// instrumentation hook: pull per-position backtrack
    /// counts from the most recent `solve()`. Cleared on read.
    /// Returns empty Vec if no run has completed.
    pub fn take_pos_backtracks(&self) -> Vec<u64> {
        let mut g = self.last_pos_backtracks.lock().unwrap();
        std::mem::take(&mut *g)
    }

    #[must_use]
    pub fn border_first_lcv() -> Self {
        Self::new(EngineConfig::BORDER_FIRST_LCV, "engine", "border_first_lcv")
    }

    #[must_use]
    pub fn rare_color_first() -> Self {
        Self::new(EngineConfig::RARE_COLOR_FIRST, "engine", "rare_color_first")
    }

    #[must_use]
    pub fn border_first_random() -> Self {
        Self::new(EngineConfig::BORDER_FIRST_RANDOM, "engine", "border_first_random")
    }

    #[must_use]
    pub fn border_first_parity() -> Self {
        Self::new(EngineConfig::BORDER_FIRST_PARITY, "engine", "border_first_parity")
    }

    #[must_use]
    pub fn border_first_full() -> Self {
        Self::new(EngineConfig::BORDER_FIRST_FULL, "engine", "border_first_full")
    }

    #[must_use]
    pub fn border_first_lcv_par() -> Self {
        Self::new(EngineConfig::BORDER_FIRST_LCV_PAR, "engine", "border_first_lcv_par")
    }

    #[must_use]
    pub fn border_first_full_par() -> Self {
        Self::new(EngineConfig::BORDER_FIRST_FULL_PAR, "engine", "border_first_full_par")
    }

    #[must_use]
    pub fn border_first_gacolor() -> Self {
        Self::new(EngineConfig::BORDER_FIRST_GACOLOR, "engine", "border_first_gacolor")
    }

    #[must_use]
    pub fn border_first_gacolor_par() -> Self {
        Self::new(EngineConfig::BORDER_FIRST_GACOLOR_PAR, "engine", "border_first_gacolor_par")
    }

    #[must_use]
    pub fn chess_gacolor() -> Self {
        Self::new(EngineConfig::CHESS_GACOLOR, "engine", "chess_gacolor")
    }

    #[must_use]
    pub fn chess_gacolor_par() -> Self {
        Self::new(EngineConfig::CHESS_GACOLOR_PAR, "engine", "chess_gacolor_par")
    }

    #[must_use]
    pub fn gacolor_symbreak() -> Self {
        Self::new(EngineConfig::GACOLOR_SYMBREAK, "engine", "gacolor_symbreak")
    }

    #[must_use]
    pub fn gacolor_symbreak_par() -> Self {
        Self::new(EngineConfig::GACOLOR_SYMBREAK_PAR, "engine", "gacolor_symbreak_par")
    }

    #[must_use]
    pub fn gacolor_ac3() -> Self {
        Self::new(EngineConfig::GACOLOR_AC3, "engine", "gacolor_ac3")
    }

    #[must_use]
    pub fn gacolor_ac3_par() -> Self {
        Self::new(EngineConfig::GACOLOR_AC3_PAR, "engine", "gacolor_ac3_par")
    }

    /// Same as `gacolor_ac3_par` but with a seeded random tiebreaker in the
    /// variable-order step. Different `opts.seed` values produce different
    /// CP partials, which is necessary when downstream search saturates
    /// because every seed of the deterministic pipeline converges to the
    /// same wrong prefix.
    #[must_use]
    pub fn gacolor_ac3_random_par() -> Self {
        Self::new(EngineConfig::GACOLOR_AC3_RANDOM_PAR, "engine", "gacolor_ac3_random_par")
    }

    #[must_use]
    pub fn gacolor_ac3_random() -> Self {
        Self::new(EngineConfig::GACOLOR_AC3_RANDOM, "engine", "gacolor_ac3_random")
    }

    #[must_use]
    pub fn chess_gacolor_ac3() -> Self {
        Self::new(EngineConfig::CHESS_GACOLOR_AC3, "engine", "chess_gacolor_ac3")
    }

 ///: gacolor + AC-3 + NS-1 multiset-equality propagator.
    #[must_use]
    pub fn gacolor_ac3_ns1() -> Self {
        Self::new(EngineConfig::GACOLOR_AC3_NS1, "engine", "gacolor_ac3_ns1")
    }

    #[must_use]
    pub fn gacolor_ac3_ns1_par() -> Self {
        Self::new(EngineConfig::GACOLOR_AC3_NS1_PAR, "engine", "gacolor_ac3_ns1_par")
    }

 ///: gacolor + AC-3 + NS-1 with Step-8 propagators gated to
    /// depth ≥ 150 (Joe-Saunders 2026 pruning policy).
    #[must_use]
    pub fn joe_depth150() -> Self {
        Self::new(EngineConfig::JOE_DEPTH150, "engine", "joe_depth150")
    }

    #[must_use]
    pub fn joe_depth150_par() -> Self {
        Self::new(EngineConfig::JOE_DEPTH150_PAR, "engine", "joe_depth150_par")
    }

    #[must_use]
    pub fn joe_depth150_bp() -> Self {
        Self::new(EngineConfig::JOE_DEPTH150_BP, "engine", "joe_depth150_bp")
    }

    #[must_use]
    pub fn joe_depth150_bp_par() -> Self {
        Self::new(EngineConfig::JOE_DEPTH150_BP_PAR, "engine", "joe_depth150_bp_par")
    }

    #[must_use]
    pub fn joe_depth150_bp_rec_par() -> Self {
        Self::new(EngineConfig::JOE_DEPTH150_BP_REC_PAR, "engine", "joe_depth150_bp_rec_par")
    }

    #[must_use]
    pub fn joe_depth150_bp_rec() -> Self {
        Self::new(EngineConfig::JOE_DEPTH150_BP_REC, "engine", "joe_depth150_bp_rec")
    }

    #[must_use]
    pub fn joe_depth150_bp_rec_layered_par() -> Self {
        Self::new(EngineConfig::JOE_DEPTH150_BP_REC_LAYERED_PAR,
                  "engine", "joe_depth150_bp_rec_layered_par")
    }

    #[must_use]
    pub fn joe_depth150_bp_x_par() -> Self {
        Self::new(EngineConfig::JOE_DEPTH150_BP_X_PAR,
                  "engine", "joe_depth150_bp_x_par")
    }

    #[must_use]
    pub fn joe_depth150_bp_rec_layered() -> Self {
        Self::new(EngineConfig::JOE_DEPTH150_BP_REC_LAYERED,
                  "engine", "joe_depth150_bp_rec_layered")
    }

    /// Verhaard-style value ordering: prefer the pieces listed in
    /// `SolveOpts.preferred_pieces`. Combined with gacolor + AC-3.
    /// Single-thread.
    #[must_use]
    pub fn verhaard_preferred() -> Self {
        Self::new(EngineConfig::VERHAARD_PREFERRED, "engine", "verhaard_preferred")
    }

    /// Parallel variant of verhaard_preferred.
    #[must_use]
    pub fn verhaard_preferred_par() -> Self {
        Self::new(EngineConfig::VERHAARD_PREFERRED_PAR, "engine", "verhaard_preferred_par")
    }

    #[must_use]
    pub fn gacolor_ac3_lcv() -> Self {
        Self::new(EngineConfig::GACOLOR_AC3_LCV, "engine", "gacolor_ac3_lcv")
    }

    #[must_use]
    pub fn gacolor_ac3_lcv_par() -> Self {
        Self::new(EngineConfig::GACOLOR_AC3_LCV_PAR, "engine", "gacolor_ac3_lcv_par")
    }
}

impl Solver for EngineSolver {
    fn id(&self) -> SolverId { SolverId(self.solver_id.clone()) }
    fn heuristic_profile(&self) -> HeuristicProfile {
        HeuristicProfile(self.heuristic_profile.clone())
    }

    fn supports_path_policy(&self, policy: &PathPolicy) -> bool {
        !matches!(policy, PathPolicy::Strict)
    }

    fn solve(
        &mut self,
        puzzle: &Puzzle,
        opts: &SolveOpts,
        sink: &mut dyn EventSink,
    ) -> SolveOutcome {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if matches!(self.config.parallelism, Parallelism::RootSplit { .. }) {
                return parallel::solve_parallel(self, puzzle, opts, sink);
            }
        }
 // instrumentation hook: capture pos_backtracks at
        // end-of-run. The state consumes itself in `run`; we use a
        // helper that returns both the outcome and the counters.
        let (outcome, pos_bt) = SearchState::new(puzzle, self, opts).run_with_diag(sink);
        if let Ok(mut g) = self.last_pos_backtracks.lock() {
            *g = pos_bt;
        }
        outcome
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod parallel;

mod paths;
pub use paths::{
    build_hint_rectangle_layered_path, build_hint_rectangle_path, build_x_skeleton_path,
};
use paths::compute_chess_rank;

mod schedule_builders;
pub use schedule_builders::{
    blackwood_schedule_469, blackwood_schedule_calibrated_v17a,
    blackwood_schedule_calibrated_v17a_p25, blackwood_schedule_calibrated_v17b,
    blackwood_schedule_calibrated_v17c, blackwood_schedule_calibrated_v17d,
    blackwood_schedule_calibrated_v17e, compute_heuristic_sides, count_color_occurrences,
};

/// Number of states per edge in the BP marginals file
/// (`output/v12_bp/edge_bp_60i.json`): color 0 = BORDER + 22 interior
/// colors = 23.
pub const EDGE_BP_NSTATE: usize = 23;


/// Build the (cell, side) -> edge_id map matching the Python BP
/// enumeration in `scripts/v12_edge_bp.py::build_grid_edges`. Layout:
/// `cell_side_edge[pos * 4 + side]` is the edge_id for that side
/// (side 0=N, 1=E, 2=S, 3=W). Internal edges are shared between two
/// cells; boundary edges have exactly one incident cell.
///
/// Total edge count is `2 * W * H + W + H` (= 544 for 16×16).
pub(crate) fn build_cell_side_edge(puzzle: &Puzzle) -> Vec<u32> {
    let w = puzzle.width as usize;
    let h = puzzle.height as usize;
    let n = w * h;
    let mut cse = vec![u32::MAX; n * 4];
    let mut next_eid: u32 = 0;
    for y in 0..h {
        for x in 0..w {
            let pos = y * w + x;
            // N (side 0)
            if cse[pos * 4 + 0] == u32::MAX {
                let eid = next_eid;
                next_eid += 1;
                cse[pos * 4 + 0] = eid;
                if y > 0 {
                    let npos = (y - 1) * w + x;
                    cse[npos * 4 + 2] = eid;
                }
            }
            // S (side 2) — only created here if last row (else assigned by next row's N pass)
            if cse[pos * 4 + 2] == u32::MAX && y == h - 1 {
                let eid = next_eid;
                next_eid += 1;
                cse[pos * 4 + 2] = eid;
            }
            // W (side 3)
            if cse[pos * 4 + 3] == u32::MAX {
                let eid = next_eid;
                next_eid += 1;
                cse[pos * 4 + 3] = eid;
                if x > 0 {
                    let npos = y * w + (x - 1);
                    cse[npos * 4 + 1] = eid;
                }
            }
            // E (side 1)
            if cse[pos * 4 + 1] == u32::MAX && x == w - 1 {
                let eid = next_eid;
                next_eid += 1;
                cse[pos * 4 + 1] = eid;
            }
        }
    }
    debug_assert!(cse.iter().all(|&e| e != u32::MAX));
    cse
}

/// Load edge-color BP marginals from a JSON file produced by
/// `scripts/v12_edge_bp.py`. Returns a flat `Arc<Vec<f32>>` of length
/// `n_edges * EDGE_BP_NSTATE` indexed as
/// `flat[edge_id * EDGE_BP_NSTATE + color]`. Order of edges in the
/// file is assumed to match `build_cell_side_edge`'s enumeration.
///
/// Errors: I/O, JSON parse, or unexpected schema (wrong NSTATE, ids
/// not contiguous starting at 0).
#[cfg(not(target_arch = "wasm32"))]
pub fn load_edge_bp_marginals(path: &std::path::Path) -> std::io::Result<Arc<Vec<f32>>> {
    let bytes = std::fs::read(path)?;
    let doc: serde_json::Value = serde_json::from_slice(&bytes)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    let edges = doc.get("edges").and_then(|v| v.as_array()).ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, "missing edges array")
    })?;
    let n = edges.len();
    let mut flat = vec![0f32; n * EDGE_BP_NSTATE];
    for (i, e) in edges.iter().enumerate() {
        let id = e.get("id").and_then(|v| v.as_u64()).unwrap_or(i as u64) as usize;
        if id != i {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("edge ids not contiguous at index {i} (got {id})"),
            ));
        }
        let m = e.get("marginal").and_then(|v| v.as_array()).ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "missing marginal")
        })?;
        if m.len() != EDGE_BP_NSTATE {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("edge {i} marginal len {} != {EDGE_BP_NSTATE}", m.len()),
            ));
        }
        for (c, v) in m.iter().enumerate() {
            flat[i * EDGE_BP_NSTATE + c] = v.as_f64().unwrap_or(0.0) as f32;
        }
    }
    Ok(Arc::new(flat))
}

/// Load per-cell records-prior value-order JSON.
/// Format produced by `ml/structural_scan.py`:
///   { "n_records": 7, "value_order": { "0": [{"piece_id", "rotation", "count"}, ...], ... } }
/// Returns Vec<Vec<(piece_id, rotation, freq)>> indexed by cell position (length 256 for canonical E2),
/// inner Vec sorted descending by freq.
pub fn load_records_prior_map(path: &std::path::Path) -> std::io::Result<Arc<Vec<Vec<(u16, u8, u32)>>>> {
    let bytes = std::fs::read(path)?;
    let doc: serde_json::Value = serde_json::from_slice(&bytes)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    let vo = doc.get("value_order").and_then(|v| v.as_object()).ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, "missing value_order object")
    })?;
    // Determine max position to allocate correctly.
    let max_pos = vo.keys()
        .filter_map(|k| k.parse::<usize>().ok())
        .max()
        .unwrap_or(255);
    let n_cells = max_pos + 1;
    let mut out: Vec<Vec<(u16, u8, u32)>> = vec![Vec::new(); n_cells];
    for (k, v) in vo.iter() {
        let pos: usize = k.parse().map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, format!("invalid pos key: {k}"))
        })?;
        let entries = v.as_array().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "value_order entry not an array")
        })?;
        let mut variants: Vec<(u16, u8, u32)> = entries.iter().filter_map(|e| {
            let pid = e.get("piece_id")?.as_u64()? as u16;
            let rot = e.get("rotation")?.as_u64()? as u8;
            let cnt = e.get("count")?.as_u64()? as u32;
            Some((pid, rot, cnt))
        }).collect();
        // Sort descending by count
        variants.sort_by(|a, b| b.2.cmp(&a.2));
        out[pos] = variants;
    }
    Ok(Arc::new(out))
}


#[derive(Debug, Clone, Copy)]
pub(crate) struct Row {
    pub(crate) piece_id: PieceId,
    pub(crate) edges: [Color; 4],
    pub(crate) rotation: u8,
    valid: bool,
}

/// Iterator that yields set bit positions of a u64 word, offset by `base`.
struct BitIter {
    cur: u64,
    base: u32,
}

impl Iterator for BitIter {
    type Item = u32;
    #[inline]
    fn next(&mut self) -> Option<u32> {
        if self.cur == 0 { return None; }
        let bit = self.cur.trailing_zeros();
        self.cur &= self.cur - 1;
        Some(self.base + bit)
    }
}

pub(crate) struct SearchState<'a> {
    pub(crate) puzzle: &'a Puzzle,
    pub(crate) opts: &'a SolveOpts,
    pub(crate) config: EngineConfig,
    pub(crate) solver_id: String,
    pub(crate) heuristic_profile: String,
    pub(crate) rows: Vec<Row>,
    /// Canonical domain representation as a flat row-id bitset.
    /// `domain_bits[pos * words_per_pos + word]` has bit `r_id % 64`
    /// of word `r_id / 64` set iff `r_id` is in the domain of `pos`.
 /// dropped the parallel `Vec<Vec<u32>>` rep (AUDIT_REPORT
    /// Step 6).
    pub(crate) domain_bits: Vec<u64>,
    /// Number of u64 words per position (ceil(n_rows / 64)).
    pub(crate) words_per_pos: usize,
    /// Precomputed: `side_color_mask[side * n_colors + color]` is a
    /// `words_per_pos`-word bitmask over row_ids whose
    /// `rows[r_id].edges[side] == color`. Used by place_and_propagate's
    /// 4-neighbor prune to do a single AND instead of a Vec scan.
    pub(crate) side_color_mask: Vec<u64>,
    /// Precomputed: `piece_mask[pid]` is a `words_per_pos`-word bitmask
    /// over row_ids belonging to piece `pid` (4 rotations). Used by
    /// piece-uniqueness propagation to mask off all 4 rotations of a
    /// placed piece at once.
    pub(crate) piece_mask: Vec<u64>,
    /// n_colors used to index side_color_mask.
    pub(crate) n_colors: usize,
    pub(crate) placed: Vec<Option<u32>>,
    pub(crate) path_order: Vec<Position>,
    pub(crate) path_index_of: Vec<u32>,
    pub(crate) used: Vec<bool>,
    pub(crate) rng_state: u64,
    pub(crate) started: Clock,
    pub(crate) node_id: u64,
    pub(crate) stats: FinalStats,
    pub(crate) best_depth: u32,
    pub(crate) best_partial: Option<Board>,
    pub(crate) gacolor: Option<GaColorState>,
    /// Precomputed static-rank for CHESS variable ordering.
    /// `chess_rank[pos]` is the position's priority (lower = picked
    /// sooner). Only consulted when `variable_order ==
    /// BorderFirstChess`. Empty for other orders.
    pub(crate) chess_rank: Vec<u32>,
    /// Static per-position priority class: 0 = corner, 1 = edge, 2 = inner.
    /// Read on every `select_position` call (hot — once per node); cached
    /// here so we don't redo the `border_mask` arithmetic every time.
    pub(crate) border_priority_cache: Vec<u32>,
    /// Reusable scratch for AC-3's hot-path count cache (Exp I).
    /// Shape is fixed at construction: `(n_pos * 4 * n_colors)` u16s for
    /// `ac3_count` and `(n_pos * words_per_pos)` u64s for `ac3_present`.
    /// Holding them on SearchState avoids per-call allocator round-trips.
    ///
 /// `ac3_count` is now maintained *across* AC-3 invocations
    /// via a dirty list. Mutation sites mark cells dirty via
    /// `mark_ac3_dirty(p)`; `propagate_ac3` rebuilds only dirty cells at
    /// entry. Replaces the 21% per-node full-rebuild loop.
    pub(crate) ac3_count: Vec<u16>,
    pub(crate) ac3_present: Vec<u64>,
    pub(crate) ac3_on_queue: Vec<bool>,
 /// positions whose `ac3_count` row is stale and must be
    /// rebuilt at the next `propagate_ac3` entry. Push-only during
    /// domain mutations; drained + cleared at AC-3 entry.
    pub(crate) ac3_dirty_list: Vec<u32>,
 /// per-position dirty flag, gating pushes into
    /// `ac3_dirty_list` to avoid duplicates. `true` iff position is in
    /// the dirty list pending rebuild.
    pub(crate) ac3_dirty_flag: Vec<bool>,
 /// `cell_side_edge[pos * 4 + side]` = edge_id in the BP
    /// marginals file. Empty unless value-order is
    /// `EdgeBpMarginals` *and* `opts.edge_bp_marginals` is set.
    /// Enumeration order matches `scripts/v12_edge_bp.py`
    /// `build_grid_edges`: scan (y,x) row-major; for each cell claim
    /// N then (S if y==H-1) then W then (E if x==W-1); shared sides
    /// reuse the neighbour's edge_id.
    pub(crate) cell_side_edge: Vec<u32>,
 /// transient instrumentation: backtracks bucketed by the
    /// cell whose value-choice exhausted. `pos_backtracks[pos]` =
    /// count of times `recurse` saw a Wipeout while attempting some
    /// row at `pos`. Exposed via `EngineSolver::take_pos_backtracks`
    /// for harness-side bucket analysis (corner / edge / interior).
    /// Always allocated; cheap (1 KB on canonical E2).
    pub(crate) pos_backtracks: Vec<u64>,
 /// `PathSkeleton::HintRectangle` auto-injected prefix length.
    /// When > 0, `select_position` walks the first `auto_skeleton_path_k`
    /// entries of `path_order` (which were populated from the hint
    /// rectangle) before falling back to the variable-order heuristic.
    /// Independent of `opts.path` / `opts.path_policy` — those still work
    /// if the user wants explicit control.
    pub(crate) auto_skeleton_path_k: u32,
 /// Blackwood schedule cloned from the solver.
    pub(crate) blackwood: Option<Arc<BlackwoodSchedule>>,
 /// set bitset over `heuristic_sides` for fast membership.
    /// `heuristic_color_mask[c as usize] = true` iff `c` is in
    /// `schedule.heuristic_sides`. Empty when no schedule.
    pub(crate) heuristic_color_mask: Vec<bool>,
 /// `break_indexes_allowed` lifted into a bitvec indexed
    /// by SCAN-ORDER INDEX (not engine traversal depth). At each
    /// recurse depth the engine fetches `pos`, then tests
    /// `is_break_index[scan_index_of_cell[pos]]`. This handles the
    /// case where hint cells are skipped from the engine traversal
    /// (depth ≠ scan_index) and HintRectangle composition (scan_index
    /// is still well-defined for each cell). Empty when no schedule.
    pub(crate) is_break_index: Vec<bool>,
 /// scan-order index of each cell. Built when scan_order
    /// is set; empty otherwise.
    pub(crate) scan_index_of_cell: Vec<u32>,
 /// running count of placed heuristic-color edge
    /// occurrences (Σ over placed rows of count of heuristic-colored
    /// edges in that row). Maintained incrementally by
    /// place_and_propagate / undo_place. 0 when no schedule.
    pub(crate) placed_heuristic_count: u32,
 /// Cat-4f — precomputed table for the AC-3 inner support
    /// loop. `same_piece_rots[row_id * 16 + side_a * 4 + side_b]` is a
    /// 4-bit mask (low nibble of a u8) where bit `r` is set iff
    /// rotation `r` of `row.piece_id` has the SAME color on side_b as
    /// `row.edges[side_a]`. AC-3's `same_piece` count becomes a single
    /// `(mask & present_4).count_ones()` instead of a 4-iteration loop
    /// with bounds-checked array reads on every rotation.
    ///
    /// Built once at SearchState::new from the static row table.
    /// Size: rows.len() * 16 bytes = ~16 KB on canonical E2.
    pub(crate) same_piece_rots: Vec<u8>,
 /// Cat-4g — reusable AC-3 work queue (was Vec::with_capacity(16)
    /// per invocation). LIFO discipline.
    pub(crate) ac3_queue: Vec<Position>,
 /// Cat-4 — single arena for all UndoEntry bit-diff buffers.
    /// `UndoEntry { pos, words_start }` references a contiguous slice
    /// `[words_start..words_start + words_per_pos]`. The arena grows
    /// monotonically within a `place_and_propagate_opts` call and is
    /// truncated back when `restore()` runs after the recurse child
 /// returns (LIFO discipline). Replaces ~42% of the earlier CPU
    /// spent in System::alloc/dealloc inside the `prune` closure.
    pub(crate) undo_words_arena: Vec<u64>,
 /// total internal grid edges = (w-1)*h + w*(h-1). Cached
    /// because the bound prune queries it on every node.
    pub(crate) total_internal_edges: u32,
 /// running count of internal edges with BOTH endpoints
    /// placed. Maintained incrementally: each `place_and_propagate_opts`
    /// adds one per placed neighbour; `undo_place` subtracts the same
    /// count (LIFO discipline guarantees correctness without snapshots).
    pub(crate) decided_edges: u32,
 /// running count of internal edges with both endpoints
    /// placed AND matching colors (excluding BORDER 0). The MaxScore
    /// objective. Same incremental discipline as `decided_edges`.
    pub(crate) matched_count: u32,
 /// best matched-edge count seen across the whole search.
    /// Only meaningful when `opts.objective == Some(Objective::MaxScore)`.
    pub(crate) best_score: u32,
 /// board snapshot at the moment we observed `best_score`.
    /// Cleared at search start; set the first time a candidate beats
    /// the running best.
    pub(crate) best_score_partial: Option<Board>,
 /// shared best-score atom across all RootSplit workers.
    /// Each worker reads it into the prune test (`max(local, shared)`)
    /// and CAS-bumps it whenever its leaf beats the running shared
    /// best. Single-threaded runs use `None` and rely on `best_score`
    /// only. The shared atom is the only mechanism that makes parallel
    /// branch-and-bound monotone-optimal under RootSplit; without it,
    /// workers can return strictly suboptimal local bests.
    pub(crate) shared_best_score: Option<Arc<std::sync::atomic::AtomicU32>>,
 /// in-process ONNX scorer for `ValueOrder::Learned`. Lazily
    /// constructed at the first value-ordering query; `None` until then
 /// or if model loading fails. Replaces the earlier stdio bridge.
    /// Not used when value_order != Learned. Present only with the
    /// `learned-order` feature (the ONNX runtime is optional).
    #[cfg(all(not(target_arch = "wasm32"), feature = "learned-order"))]
    pub(crate) learned_bridge: Option<crate::bridge::LearnedScorer>,
 /// set to `true` once we've attempted to load the scorer
    /// and failed, so we don't retry per-node.
    #[cfg(all(not(target_arch = "wasm32"), feature = "learned-order"))]
    pub(crate) learned_bridge_disabled: bool,
}

impl<'a> SearchState<'a> {
    pub(crate) fn new(puzzle: &'a Puzzle, solver: &EngineSolver, opts: &'a SolveOpts) -> Self {
        let pieces = puzzle.pieces();
        let max_piece_index = pieces.iter().map(|p| usize::from(p.id) + 1).max().unwrap_or(0);
        let mut rows = vec![
            Row { piece_id: 0, edges: [0; 4], rotation: 0, valid: false };
            max_piece_index * 4
        ];
        // SolveOpts.excluded_pieces (used by Verhaard phase-1 scaffold)
        // forbids these piece-ids from appearing in any domain. Mark
        // all four rows of an excluded piece as invalid below.
        let mut excluded = vec![false; max_piece_index];
        for &pid in &opts.excluded_pieces {
            let idx = usize::from(pid);
            if idx < excluded.len() {
                excluded[idx] = true;
            }
        }
        for piece in pieces {
            let pid = usize::from(piece.id);
            let is_excluded = excluded.get(pid).copied().unwrap_or(false);
            let mut seen: Vec<[Color; 4]> = Vec::with_capacity(4);
            for r in 0..4u8 {
                let rot = Rotation::from_u8(r).unwrap();
                let e = piece.edges.rotated(rot).as_array();
                let dup = seen.iter().any(|s| *s == e);
                if !dup { seen.push(e); }
                rows[pid * 4 + r as usize] = Row {
                    piece_id: piece.id,
                    edges: e,
                    rotation: r,
                    valid: !dup && !is_excluded,
                };
            }
        }

 // Cat-4f — precomputed same-piece rotation table for AC-3.
        // For each row r and (side_a, side_b) pair: which rotations of
        // r's piece have edges[side_b] == r.edges[side_a]? Stored as a
        // 4-bit mask in the low nibble of a u8. Reads turn AC-3's
        // 4-iteration support loop into a single mask + popcount.
        let mut same_piece_rots: Vec<u8> = vec![0; rows.len() * 16];
        for r_id in 0..rows.len() {
            let r = &rows[r_id];
            if !r.valid {
                continue;
            }
            let pid_base = usize::from(r.piece_id) * 4;
            for side_a in 0..4 {
                let required = r.edges[side_a];
                for side_b in 0..4 {
                    let mut mask: u8 = 0;
                    for rot in 0..4 {
                        let cand = pid_base + rot;
                        if cand >= rows.len() {
                            break;
                        }
                        let rc = &rows[cand];
                        if rc.valid && rc.edges[side_b] == required {
                            mask |= 1 << rot;
                        }
                    }
                    same_piece_rots[r_id * 16 + side_a * 4 + side_b] = mask;
                }
            }
        }

        let n_pos = puzzle.cell_count() as usize;
        let n_rows = max_piece_index * 4;
        let words_per_pos = n_rows.div_ceil(64).max(1);

        // Build domain_bits directly. Each cell admits rows whose
        // BORDER-edge mask matches the cell's border_mask.
        let mut domain_bits = vec![0u64; n_pos * words_per_pos];
        for pos in 0..puzzle.cell_count() {
            let mask = puzzle.border_mask(pos);
            let [on_top, on_right, on_bot, on_left] = mask;
            let base = (pos as usize) * words_per_pos;
            for (idx, row) in rows.iter().enumerate() {
                if !row.valid { continue; }
                let [t, ri, b, l] = row.edges;
                if on_top != (t == BORDER) { continue; }
                if on_right != (ri == BORDER) { continue; }
                if on_bot != (b == BORDER) { continue; }
                if on_left != (l == BORDER) { continue; }
                domain_bits[base + (idx >> 6)] |= 1u64 << (idx & 63);
            }
        }

        // Resolve path_order: prefer user-supplied opts.path, otherwise
        // auto-build from config.path_skeleton + config.scan_order.
        //
 // composition: when BOTH `path_skeleton` (e.g.
        // HintRectangle) and `scan_order` (Blackwood bottom-up) are
        // set, build the rectangle path FIRST and then append the
        // remaining cells in scan order. This way:
        //   - The engine's traversal depth always equals the
        //     position's index in `path_order`.
        //   - Blackwood break-indexes (the schedule's
        //     `break_indexes_allowed`) are interpreted as DEPTHS
        //     in the engine's traversal, not as raw cell labels.
        //   - The HintRectangle prefix still does its job (forces
        //     the 49 rectangle cells to be placed first).
        let order_key = |pos: Position, w: u32, h: u32, order: ScanOrder| -> u32 {
            let (x, y) = (pos % w, pos / w);
            match order {
                ScanOrder::RowMajorTopDown => y * w + x,
                ScanOrder::RowMajorBottomUp => (h - 1 - y) * w + x,
            }
        };
        let (path_order, auto_skeleton_path_k) = if !opts.path.is_empty() {
            (opts.path.clone(), 0u32)
        } else {
            let skel_path: Vec<Position> = match solver.config.path_skeleton {
                Some(PathSkeleton::HintRectangle) =>
                    build_hint_rectangle_path(puzzle, &opts.hints),
                Some(PathSkeleton::HintRectangleLayered) =>
                    build_hint_rectangle_layered_path(puzzle, &opts.hints),
                Some(PathSkeleton::XSkeleton) =>
                    build_x_skeleton_path(puzzle, &opts.hints),
                None => Vec::new(),
            };
            let n_cells = puzzle.cell_count();
            if !skel_path.is_empty() && solver.config.scan_order.is_some() {
                // Compose: rectangle prefix + remaining cells in scan order.
                let order = solver.config.scan_order.unwrap();
                let in_skel: std::collections::HashSet<Position> =
                    skel_path.iter().copied().collect();
                let mut tail: Vec<Position> = (0..n_cells)
                    .filter(|p| !in_skel.contains(p))
                    .collect();
                tail.sort_by_key(|&pos| order_key(pos, puzzle.width, puzzle.height, order));
                let mut p = skel_path;
                p.extend(tail);
                let k = p.len() as u32;
                (p, k)
            } else if !skel_path.is_empty() {
                let k = skel_path.len() as u32;
                (skel_path, k)
            } else if let Some(order) = solver.config.scan_order {
                let mut p: Vec<Position> = (0..n_cells).collect();
                p.sort_by_key(|&pos| order_key(pos, puzzle.width, puzzle.height, order));
                let k = p.len() as u32;
                (p, k)
            } else {
                (Vec::new(), 0u32)
            }
        };
        let mut path_index_of = vec![u32::MAX; n_pos];
        for (i, &p) in path_order.iter().enumerate() {
            if (p as usize) < n_pos {
                path_index_of[p as usize] = i as u32;
            }
        }

        // Precomputed side+color → rows bitmask. Used by the 4-neighbor
        // prune in place_and_propagate to AND off any row whose
        // edges[side]==required is not the required color.
        let n_colors = puzzle.color_count as usize;
        let mut side_color_mask = vec![0u64; 4 * n_colors * words_per_pos];
        for (r_id, row) in rows.iter().enumerate() {
            if !row.valid { continue; }
            for side in 0..4 {
                let c = row.edges[side] as usize;
                if c >= n_colors { continue; }
                let base = (side * n_colors + c) * words_per_pos;
                side_color_mask[base + r_id / 64] |= 1u64 << (r_id % 64);
            }
        }

        // Precomputed piece → rows bitmask. Used by piece-uniqueness
        // propagation: when piece `p` is placed, AND off all 4 rotations
        // of p from every unplaced cell's bitset.
        let mut piece_mask = vec![0u64; max_piece_index.max(1) * words_per_pos];
        for (r_id, row) in rows.iter().enumerate() {
            if !row.valid { continue; }
            let pid = usize::from(row.piece_id);
            if pid < max_piece_index {
                piece_mask[pid * words_per_pos + r_id / 64] |= 1u64 << (r_id % 64);
            }
        }

        Self {
            puzzle,
            opts,
            config: solver.config,
            solver_id: solver.solver_id.clone(),
            heuristic_profile: solver.heuristic_profile.clone(),
            rows,
            domain_bits,
            words_per_pos,
            side_color_mask,
            piece_mask,
            n_colors,
            placed: vec![None; n_pos],
            path_order,
            path_index_of,
            used: vec![false; max_piece_index.max(1)],
            rng_state: opts.seed.wrapping_add(0x9E37_79B9_7F4A_7C15),
            started: Clock::now(),
            node_id: 0,
            stats: FinalStats::default(),
            best_depth: 0,
            best_partial: None,
            gacolor: if solver.config.propagators.gacolor {
                Some(GaColorState::new(puzzle))
            } else {
                None
            },
            chess_rank: if matches!(solver.config.variable_order, VariableOrder::BorderFirstChess) {
                compute_chess_rank(puzzle)
            } else {
                Vec::new()
            },
            border_priority_cache: {
                let mut v = vec![0u32; n_pos];
                for pos in 0..puzzle.cell_count() {
                    let mask = puzzle.border_mask(pos);
                    let n = u32::from(mask[0]) + u32::from(mask[1])
                          + u32::from(mask[2]) + u32::from(mask[3]);
                    v[pos as usize] = match n { 2 => 0, 1 => 1, _ => 2 };
                }
                v
            },
            ac3_count: if solver.config.propagators.ac3 {
                vec![0u16; n_pos * 4 * puzzle.color_count as usize]
            } else {
                Vec::new()
            },
            ac3_present: if solver.config.propagators.ac3 {
                let n_rows = max_piece_index * 4;
                vec![0u64; n_pos * n_rows.div_ceil(64)]
            } else {
                Vec::new()
            },
            ac3_on_queue: if solver.config.propagators.ac3 {
                vec![false; n_pos]
            } else {
                Vec::new()
            },
 // initially every position is dirty (count is all
            // zeros at construction). First AC-3 entry rebuilds all
            // non-placed cells; subsequent entries only touch cells
            // that mutated since last rebuild.
            ac3_dirty_list: if solver.config.propagators.ac3 {
                (0..n_pos as u32).collect()
            } else {
                Vec::new()
            },
            ac3_dirty_flag: if solver.config.propagators.ac3 {
                vec![true; n_pos]
            } else {
                Vec::new()
            },
            cell_side_edge: if matches!(solver.config.value_order, ValueOrder::EdgeBpMarginals | ValueOrder::LearnedOnTies)
                && opts.edge_bp_marginals.is_some()
            {
                build_cell_side_edge(puzzle)
            } else {
                Vec::new()
            },
            pos_backtracks: vec![0u64; n_pos],
            auto_skeleton_path_k,
            blackwood: solver.blackwood_schedule.clone(),
            heuristic_color_mask: {
                if let Some(s) = &solver.blackwood_schedule {
                    let max_c = s.heuristic_sides.iter().copied().max().unwrap_or(0) as usize;
                    let mut m = vec![false; max_c + 1];
                    for &c in &s.heuristic_sides {
                        m[c as usize] = true;
                    }
                    m
                } else {
                    Vec::new()
                }
            },
            is_break_index: {
 // `break_indexes_allowed` interpreted as
                // engine-traversal DEPTHS (the i-th cell placed in
                // path_order). Composition rule in path-order
                // construction above guarantees: under pure bottom-up
                // scan, depth == bu_idx; under rectangle+scan, depth
                // is rectangle-then-scan position. Either way, "break
                // at depth k" matches the spec's intent.
                if let Some(s) = &solver.blackwood_schedule {
                    let mut v = vec![false; n_pos];
                    for &idx in &s.break_indexes_allowed {
                        if (idx as usize) < v.len() {
                            v[idx as usize] = true;
                        }
                    }
                    v
                } else {
                    Vec::new()
                }
            },
            scan_index_of_cell: {
                if let Some(order) = solver.config.scan_order {
                    let w = puzzle.width;
                    let h = puzzle.height;
                    let mut v = vec![0u32; n_pos];
                    for pos in 0..puzzle.cell_count() {
                        let (x, y) = (pos % w, pos / w);
                        let scan_idx = match order {
                            ScanOrder::RowMajorTopDown => y * w + x,
                            ScanOrder::RowMajorBottomUp => (h - 1 - y) * w + x,
                        };
                        v[pos as usize] = scan_idx;
                    }
                    v
                } else {
                    Vec::new()
                }
            },
            placed_heuristic_count: 0,
            same_piece_rots,
            ac3_queue: Vec::with_capacity(256),
 // Cat-4 — pre-reserve the arena for the worst-case
            // undo log: 4 prunes + n_pos piece-uniqueness entries per
            // place_and_propagate_opts call, * wpp words per entry,
            // * (n_pos) recursion depth. Conservative upper bound;
            // typical canonical-E2 usage is well under 10× smaller.
            undo_words_arena: Vec::with_capacity(
                (n_pos as usize) * (n_pos as usize + 4) * words_per_pos,
            ),
            total_internal_edges: (puzzle.width - 1) * puzzle.height
                + puzzle.width * (puzzle.height - 1),
            decided_edges: 0,
            matched_count: 0,
            best_score: 0,
            best_score_partial: None,
            shared_best_score: None,
            #[cfg(all(not(target_arch = "wasm32"), feature = "learned-order"))]
            learned_bridge: None,
            #[cfg(all(not(target_arch = "wasm32"), feature = "learned-order"))]
            learned_bridge_disabled: false,
        }
    }

 /// inject the cross-worker shared best-score atom. Called
    /// by `parallel::run_unit` once per worker; single-thread runs skip
    /// this. The atom is read on every node and CAS-bumped on every
    /// leaf in MaxScore mode.
    pub(crate) fn attach_shared_best_score(
        &mut self,
        atom: Arc<std::sync::atomic::AtomicU32>,
    ) {
        self.shared_best_score = Some(atom);
    }

    fn elapsed_us(&self) -> u64 { self.started.elapsed_us() }

    fn timed_out(&self) -> bool {
        self.opts.time_budget_ms != 0
            && self.elapsed_us() / 1000 >= self.opts.time_budget_ms
    }

    fn node_budget_exhausted(&self) -> bool {
        self.opts.node_budget != 0 && self.node_id >= self.opts.node_budget
    }

    /// Domain size for `pos`, computed from `domain_bits`.
    #[inline]
    fn domain_size(&self, pos: usize) -> u32 {
        let base = pos * self.words_per_pos;
        let mut n = 0u32;
        for w in 0..self.words_per_pos {
            n += self.domain_bits[base + w].count_ones();
        }
        n
    }

    /// Iterate set row_ids in domain of `pos`.
    fn domain_iter(&self, pos: usize) -> impl Iterator<Item = u32> + '_ {
        let base = pos * self.words_per_pos;
        let wpp = self.words_per_pos;
        (0..wpp).flat_map(move |w| {
            let word = self.domain_bits[base + w];
            BitIter { cur: word, base: (w as u32) * 64 }
        })
    }

    /// Snapshot a per-position bitset as a `Vec<u64>` of words.
    #[inline]
    fn snapshot_bits(&self, pos: usize) -> Vec<u64> {
        let base = pos * self.words_per_pos;
        self.domain_bits[base..base + self.words_per_pos].to_vec()
    }

    /// Restore a per-position bitset from a snapshot.
    #[inline]
    fn restore_bits(&mut self, pos: usize, snap: &[u64]) {
        let base = pos * self.words_per_pos;
        self.domain_bits[base..base + self.words_per_pos].copy_from_slice(snap);
 // domain[pos] changed; ac3_count[pos] is stale.
        self.mark_ac3_dirty(pos);
    }

    /// True iff domain[pos] is empty. Currently unused — the hot-loop
    /// callers fold this check into their own bitset walk via an
    /// OR-survival accumulator. Kept for tests and future helpers.
    #[allow(dead_code)]
    #[inline]
    fn domain_is_empty(&self, pos: usize) -> bool {
        let base = pos * self.words_per_pos;
        self.domain_bits[base..base + self.words_per_pos].iter().all(|&w| w == 0)
    }

    /// Mark a position's AC-3 row-count cache as stale. Called from
    /// every site that mutates `domain_bits[p]` outside the AC-3 inner
    /// loop. No-op when AC-3 isn't enabled (flag vec is empty).
 ///.
    #[inline]
    fn mark_ac3_dirty(&mut self, p: usize) {
        if let Some(flag) = self.ac3_dirty_flag.get_mut(p) {
            if !*flag {
                *flag = true;
                self.ac3_dirty_list.push(p as u32);
            }
        }
    }

    /// True iff row_id is in domain[pos].
    #[inline]
    fn domain_contains(&self, pos: usize, row_id: u32) -> bool {
        let base = pos * self.words_per_pos;
        let r = row_id as usize;
        (self.domain_bits[base + (r >> 6)] >> (r & 63)) & 1 == 1
    }

    /// Restrict domain[pos] to exactly `{row_id}`.
    #[inline]
    fn pin_to(&mut self, pos: usize, row_id: u32) {
        let base = pos * self.words_per_pos;
        for w in &mut self.domain_bits[base..base + self.words_per_pos] { *w = 0; }
        let r = row_id as usize;
        self.domain_bits[base + (r >> 6)] = 1u64 << (r & 63);
 // domain[pos] changed (almost everything dropped);
        // ac3_count[pos] is stale.
        self.mark_ac3_dirty(pos);
    }

    fn next_random(&mut self) -> u64 {
        self.rng_state = self.rng_state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.rng_state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn emit(&mut self, sink: &mut dyn EventSink, depth: u32, body: EventBody) {
        self.node_id += 1;
        let event = SolverEvent {
            schema_version: 1,
            solver_run_id: self.opts.solver_run_id,
            node_id: self.node_id,
            depth,
            timestamp_us: self.elapsed_us(),
            body,
        };
        sink.emit(event);
    }

    #[inline(always)]
    fn border_priority(&self, pos: Position) -> u32 {
        self.border_priority_cache[pos as usize]
    }

    fn position_score(&mut self, pos: Position) -> (u32, u32) {
        let d = self.domain_size(pos as usize);
        let bp = self.border_priority(pos);
        let path_tie = match self.opts.path_policy {
            PathPolicy::OrderingPrior => self.path_index_of[pos as usize],
            _ => 0,
        };
        match self.config.variable_order {
            VariableOrder::BorderFirstMrv => (bp, d.saturating_add(path_tie / 1000)),
            VariableOrder::Mrv => (d, path_tie),
            VariableOrder::RareColorFirst => (bp, d),  // placeholder; real impl in Step 8
            VariableOrder::BorderFirstRandom => (bp, (self.next_random() & 0xFFFF) as u32),
            // CHESS: primary key is the static precomputed rank; ties
            // broken by current domain size (smallest first, MRV style).
            VariableOrder::BorderFirstChess => {
                let r = self.chess_rank
                    .get(pos as usize)
                    .copied()
                    .unwrap_or(u32::MAX);
                (r, d)
            }
        }
    }

    fn selection_reason(&self) -> SelectionReason {
        match self.config.variable_order {
            VariableOrder::BorderFirstMrv
            | VariableOrder::BorderFirstRandom
            | VariableOrder::BorderFirstChess => SelectionReason::BorderFirst,
            VariableOrder::Mrv => SelectionReason::Mrv,
            VariableOrder::RareColorFirst => SelectionReason::RareColor,
        }
    }

    pub(crate) fn select_position(&mut self) -> Option<Position> {
        // 1. Explicit user path takes priority.
        if let PathPolicy::PrefixConstraint { k } = self.opts.path_policy {
            if self.stats.current_depth < k {
                for &p in &self.path_order {
                    if self.placed[p as usize].is_none() {
                        return Some(p);
                    }
                }
            }
        }
 // 2. auto-skeleton path (config.path_skeleton). Only
        // applies when no explicit user path was provided.
        if self.auto_skeleton_path_k > 0
            && self.opts.path.is_empty()
            && self.stats.current_depth < self.auto_skeleton_path_k
        {
            for &p in &self.path_order {
                if self.placed[p as usize].is_none() {
                    return Some(p);
                }
            }
        }
        let mut best: Option<(Position, (u32, u32))> = None;
        for pos in 0..self.puzzle.cell_count() {
            if self.placed[pos as usize].is_some() { continue; }
            let score = self.position_score(pos);
            match &best {
                None => best = Some((pos, score)),
                Some((_, bs)) if score < *bs => best = Some((pos, score)),
                _ => {}
            }
        }
        best.map(|(p, _)| p)
    }

    pub(crate) fn place_and_propagate(
        &mut self,
        sink: &mut dyn EventSink,
        depth: u32,
        pos: Position,
        row_id: u32,
    ) -> PropagationOutcome {
        self.place_and_propagate_opts(sink, depth, pos, row_id, None)
    }

 /// extended `place_and_propagate` that optionally skips
    /// edge-color propagation on one of the placed row's 4 sides (used
    /// at Blackwood break-indexes, where the placement intentionally
    /// mismatches a placed neighbour). `skip_side`:
 /// `None` ⇒ propagate normally (the engine's earlier behaviour).
    ///   `Some(s)`  ⇒ skip the edge-color prune on side `s` (0=N, 1=E,
    ///                2=S, 3=W). Piece-uniqueness, AC-3, and Step-8
    ///                propagators still run.
    pub(crate) fn place_and_propagate_opts(
        &mut self,
        sink: &mut dyn EventSink,
        depth: u32,
        pos: Position,
        row_id: u32,
        skip_side: Option<usize>,
    ) -> PropagationOutcome {
        let row = self.rows[row_id as usize];
        // Snapshot neighbor state BEFORE placement so we can update the
        // incremental GAColor state, and so a later unplace can be
        // reproduced from the same snapshot.
        let n_info = self.neighbor_info(pos);
        self.placed[pos as usize] = Some(row_id);
        let piece_idx = usize::from(row.piece_id);
        self.used[piece_idx] = true;
        if let Some(gc) = self.gacolor.as_mut() {
            gc.apply_place(&row.edges, &n_info);
        }
 // increment Blackwood heuristic-count.
        self.placed_heuristic_count += self.count_heuristic_in_row(&row.edges);
 // incremental score tracking for the MaxScore objective.
        // For each of the 4 sides, if the neighbour cell is also placed,
        // this edge has just become "decided". It counts as a match iff
        // the touching colors agree AND aren't BORDER (color 0). Walked
        // unconditionally — cost ~16 ALU ops per placement, dwarfed by
        // propagation; cheaper than gating on `opts.objective`.
        {
            let (px, py) = self.puzzle.xy(pos);
            let pw = self.puzzle.width;
            let ph = self.puzzle.height;
            let nbs: [(Option<Position>, usize, usize); 4] = [
                (if py > 0 { Some((py - 1) * pw + px) } else { None }, 0, 2),
                (if px + 1 < pw { Some(py * pw + (px + 1)) } else { None }, 1, 3),
                (if py + 1 < ph { Some((py + 1) * pw + px) } else { None }, 2, 0),
                (if px > 0 { Some(py * pw + (px - 1)) } else { None }, 3, 1),
            ];
            for (nb_opt, our_side, their_side) in nbs {
                let Some(nb) = nb_opt else { continue; };
                let Some(nb_row_id) = self.placed[nb as usize] else { continue; };
                self.decided_edges += 1;
                let nb_edges = self.rows[nb_row_id as usize].edges;
                let our = row.edges[our_side];
                if our != 0 && our == nb_edges[their_side] {
                    self.matched_count += 1;
                }
            }
        }
 // Cat-4b: pre-allocate so the `undo.push` chain doesn't
        // hit `RawVecInner::grow_amortized`. Worst case is one entry
        // per cell (piece-uniqueness over all unplaced positions) +
        // four neighbour prunes + ~few AC-3 cascades. Sized to
        // `n_pos + 8` to cover the common path without overshooting.
        let mut undo: Vec<UndoEntry> = Vec::with_capacity(self.puzzle.cell_count() as usize + 8);

        let (x, y) = self.puzzle.xy(pos);
        let w = self.puzzle.width;
        let h = self.puzzle.height;

        let prune = |this: &mut Self,
                     neighbor: Position,
                     neighbor_edge_idx: usize,
                     required: Color|
         -> PruneResult {
            if this.placed[neighbor as usize].is_some() {
                return PruneResult::Ok;
            }
            let wpp = this.words_per_pos;
            let bit_base = (neighbor as usize) * wpp;
            let n_colors = this.n_colors;
            let req_idx = required as usize;
            // Build keep mask:
            //   rows whose edges[neighbor_edge_idx]==required, MINUS
            //   rows belonging to the just-placed piece.
            let sc_base = (neighbor_edge_idx * n_colors + req_idx) * wpp;
            let pm_base = usize::from(row.piece_id) * wpp;
 // stack scratch + track survival in the same loop;
            // see piece-uniqueness path in place_and_propagate for the
            // motivation (eliminates bzero traffic + redundant
            // domain_is_empty rescan).
            const MAX_WPP: usize = 32;
            debug_assert!(wpp <= MAX_WPP);
            let mut drop_scratch = [0u64; MAX_WPP];
            let mut popcount: u32 = 0;
            let mut survived: u64 = 0;
            // Slice-based inner loop for bounds elision.
            let dom = &mut this.domain_bits[bit_base..bit_base + wpp];
            let pmask = &this.piece_mask[pm_base..pm_base + wpp];
            let sc_mask: &[u64] = if req_idx < n_colors {
                &this.side_color_mask[sc_base..sc_base + wpp]
            } else {
                &[]
            };
            let scratch = &mut drop_scratch[..wpp];
            for w in 0..wpp {
                let cur = dom[w];
                let keep = if req_idx < n_colors {
                    sc_mask[w] & !pmask[w]
                } else {
                    !pmask[w]
                };
                let drop = cur & !keep;
                let new = cur & keep;
                scratch[w] = drop;
                survived |= new;
                popcount += drop.count_ones();
                dom[w] = new;
            }
            if popcount > 0 {
                this.stats.propagations += popcount as u64;
                let arena_start = this.undo_words_arena.len();
                this.undo_words_arena.extend_from_slice(scratch);
                let entry = UndoEntry { pos: neighbor, words_start: arena_start as u32 };
 // domain[neighbor] changed; ac3_count[neighbor]
                // is stale until next AC-3 rebuild.
                this.mark_ac3_dirty(neighbor as usize);
                if survived == 0 {
                    PruneResult::Wipeout { entry }
                } else {
                    PruneResult::Removed(entry)
                }
            } else {
                PruneResult::Ok
            }
        };

        // Four neighbor prunes (edge-color propagation). Each entry
        // is (neighbour, neighbour-facing-edge-index, required-color,
        // our-side-index). When `skip_side == Some(our_side)` we
 // suppress propagation through that side ( Blackwood
        // break-index allowance).
        let neighbors: [(Option<Position>, usize, Color, usize); 4] = [
            (if y > 0 { Some((y - 1) * w + x) } else { None }, 2, row.edges[0], 0),
            (if x + 1 < w { Some(y * w + (x + 1)) } else { None }, 3, row.edges[1], 1),
            (if y + 1 < h { Some((y + 1) * w + x) } else { None }, 0, row.edges[2], 2),
            (if x > 0 { Some(y * w + (x - 1)) } else { None }, 1, row.edges[3], 3),
        ];
        for (maybe_np, edge_idx, required, our_side) in neighbors {
            if Some(our_side) == skip_side { continue; }
            let Some(np) = maybe_np else { continue; };
            match prune(self, np, edge_idx, required) {
                PruneResult::Wipeout { entry } => {
                    undo.push(entry);
                    self.stats.domain_wipeouts += 1;
                    self.emit(sink, depth, EventBody::DomainWipeout { position: np });
                    return PropagationOutcome::Wipeout { undo };
                }
                PruneResult::Removed(entry) => undo.push(entry),
                PruneResult::Ok => {}
            }
        }

        // Piece-uniqueness propagation (the "column" of classic DLX).
        // Bitset form: for every unplaced cell p, AND off any row in
        // piece_mask[just_placed_piece]. Save the dropped bits as a
        // bit-diff for the undo log — no per-row iteration.
 // Cat-4b: same capacity argument as `undo` above.
        let mut other_undo: Vec<UndoEntry> = Vec::with_capacity(self.puzzle.cell_count() as usize);
        let words_per_pos = self.words_per_pos;
        let pm_base = piece_idx * words_per_pos;
 // stack scratch buffer for the per-cell drop bits, sized
        // large enough for any realistic puzzle (16 words = 1024 rows =
        // canonical E2; 32 words = 2048 rows ⇒ 32×32). Avoids the
        // `resize(+wpp, 0)` zero-fill (bzero call site visible in
        // flamegraph) that previously dominated the arena push path.
        const MAX_WPP: usize = 32;
        debug_assert!(words_per_pos <= MAX_WPP, "words_per_pos {} exceeds MAX_WPP {}", words_per_pos, MAX_WPP);
        let mut drop_scratch = [0u64; MAX_WPP];
        for p in 0..self.puzzle.cell_count() {
            if p == pos || self.placed[p as usize].is_some() { continue; }
            let bit_base = (p as usize) * words_per_pos;
            let mut popcount: u32 = 0;
            // OR of post-mask domain words — equivalent to
            // `domain_is_empty(p)` after the loop without a second pass.
            let mut survived: u64 = 0;
            // Slice-based inner loop. Split borrows so dom + pmask + the
            // scratch buffer can all be sliced for bounds-elision.
            let dom = &mut self.domain_bits[bit_base..bit_base + words_per_pos];
            let pmask = &self.piece_mask[pm_base..pm_base + words_per_pos];
            let scratch = &mut drop_scratch[..words_per_pos];
            for w in 0..words_per_pos {
                let cur = dom[w];
                let drop = cur & pmask[w];
                let new = cur & !pmask[w];
                scratch[w] = drop;
                survived |= new;
                popcount += drop.count_ones();
                dom[w] = new;
            }
            if popcount > 0 {
                self.stats.propagations += popcount as u64;
                let arena_start = self.undo_words_arena.len();
                // Push the full slot in one shot — no zero-fill, no
                // conditional inner writes. extend_from_slice over a
                // statically-sized chunk lets the compiler vectorize.
                self.undo_words_arena.extend_from_slice(scratch);
                let entry = UndoEntry { pos: p, words_start: arena_start as u32 };
 // domain[p] changed; ac3_count[p] is stale.
                self.mark_ac3_dirty(p as usize);
                if survived == 0 {
                    other_undo.push(entry);
                    self.stats.domain_wipeouts += 1;
                    undo.extend(other_undo);
                    self.emit(sink, depth, EventBody::DomainWipeout { position: p });
                    return PropagationOutcome::Wipeout { undo };
                }
                other_undo.push(entry);
            }
        }
        undo.extend(other_undo);

        // AC-3 cascading propagation. Re-checks arc-consistency from the
        // freshly-placed cell outward until fixpoint. Each row r at
        // position a is "supported by" position b if some row r' in
        // domain[b] has matching color on the shared edge and r.piece !=
        // r'.piece. If support is lost we drop r from domain[a]. Updates
        // can cascade through the grid.
        if self.config.propagators.ac3 {
            match self.propagate_ac3(sink, depth, pos) {
                Ac3Outcome::Wipeout { removed } => {
                    undo.extend(removed);
                    return PropagationOutcome::Wipeout { undo };
                }
                Ac3Outcome::Ok { removed } => {
                    undo.extend(removed);
                }
            }
        }

        // Step 8 propagators: run only the ones enabled in config. They
        // are read-only over engine state (don't mutate domains) so the
        // undo log doesn't change. Order is cheapest-first.
        if self.run_extra_propagators(depth) == PropagatorResult::Wipeout {
            self.emit(sink, depth, EventBody::DomainWipeout { position: pos });
            return PropagationOutcome::Wipeout { undo };
        }

        PropagationOutcome::Ok { undo }
    }

    fn snapshot_placement_info(&self) -> Vec<Option<PlacementInfo>> {
        self.placed.iter().map(|p| p.map(|row_id| {
            let r = self.rows[row_id as usize];
            PlacementInfo { edges_after_rotation: r.edges }
        })).collect()
    }

    /// AC-3 cascading propagation. Maintains arc-consistency across
    /// neighbouring positions starting from the just-placed cell. Drops
    /// any row from an unplaced position's domain that has no supporting
    /// row in some neighbouring unplaced domain.
    ///
    /// Returns the list of (position, removed_rows) pairs so the caller
    /// can extend its undo log; or `Wipeout` if any domain emptied.
    fn propagate_ac3(
        &mut self,
        sink: &mut dyn EventSink,
        depth: u32,
        start_pos: Position,
    ) -> Ac3Outcome {
        let w = self.puzzle.width;
        let h = self.puzzle.height;
        let n_pos = self.puzzle.cell_count();
        let n_rows = self.rows.len();
        let n_colors = self.puzzle.color_count as usize;
        // Hot-path acceleration (Exp I, revised in Exp K):
        //   count[pos * stride_pos + side * n_colors + color] = #rows in
        //     domains[pos] whose edges[side]==color.
        //   present[pos] is a row-id bitset.
        // Buffers live on SearchState so we pay the alloc only once.
        //
 // `ac3_count` is now maintained *incrementally* via the
        // dirty list (see `mark_ac3_dirty`). At entry we rebuild only the
        // cells whose domain mutated since the last AC-3 call. Replaces
 // the full per-node rebuild (21% of joe runtime in the
        // pre-fix flamegraph).
        let stride_pos = 4 * n_colors;
        let words_per_pos = n_rows.div_ceil(64);
        let count_len = (n_pos as usize) * stride_pos;
        let present_len = (n_pos as usize) * words_per_pos;
        debug_assert!(self.ac3_count.len() == count_len);
        debug_assert!(self.ac3_present.len() == present_len);
        for v in &mut self.ac3_on_queue[..n_pos as usize] { *v = false; }
        // Pin a non-borrow timeout snapshot before we take mut-borrows
        // on the cache fields. self.started doesn't implement Copy so
        // we read elapsed once into a closure-ish form.
        let time_budget_ms = self.opts.time_budget_ms;
        let timeout_at_us: u64 = if time_budget_ms == 0 {
            u64::MAX
        } else {
            time_budget_ms.saturating_mul(1000)
        };
        // Bitset present mirror is already maintained incrementally on
        // self.domain_bits — just copy. Saves the per-row bit-set cost
        // during the entry rebuild loop.
        debug_assert!(self.domain_bits.len() == present_len,
            "domain_bits ({}) != present_len ({})", self.domain_bits.len(), present_len);
        self.ac3_present[..present_len].copy_from_slice(&self.domain_bits[..present_len]);
 // drain the dirty list. For each dirty position, zero
        // its count row, then rebuild from current domain bits. Clear
        // the dirty flag as we go. Placed cells are skipped (their
        // count row is never read).
        //
 // for each (side, color) we have a precomputed
        // `side_color_mask` u64 bitset (rows whose edges[side]==color).
        // count[p][s][c] = popcount(domain[p] & side_color_mask[s*nC+c]).
        // This is O(stride_pos × wpp) per position instead of
        // O(set_bits × 4); wins when domains are full (the common case
        // mid-search) and trades a small loss for very sparse domains
        // for the benefit of being autovectorizable.
        let dirty_len = self.ac3_dirty_list.len();
        if dirty_len > 0 {
            let dom_bits = self.domain_bits.as_slice();
            let scm = self.side_color_mask.as_slice();
            for i in 0..dirty_len {
                let p = self.ac3_dirty_list[i] as usize;
                self.ac3_dirty_flag[p] = false;
                if self.placed[p].is_some() { continue; }
                let base = p * words_per_pos;
                let pos_count_base = p * stride_pos;
                let dom_slice = &dom_bits[base..base + words_per_pos];
                let count_slice = &mut self.ac3_count[pos_count_base..pos_count_base + stride_pos];
                // count_slice[s * n_colors + c] = popcount(dom & scm[s,c])
                for sc in 0..stride_pos {
                    let scm_slice = &scm[sc * words_per_pos..sc * words_per_pos + words_per_pos];
                    let mut acc: u32 = 0;
                    for w in 0..words_per_pos {
                        acc += (dom_slice[w] & scm_slice[w]).count_ones();
                    }
                    count_slice[sc] = acc as u16;
                }
            }
            self.ac3_dirty_list.clear();
        }
        let count = &mut self.ac3_count;
        let present = &mut self.ac3_present;
        let on_queue = &mut self.ac3_on_queue;
 // Cat-4g — reuse the queue scratch across AC-3 calls.
        self.ac3_queue.clear();
        let queue = &mut self.ac3_queue;
        // Seed: all unplaced neighbours of start_pos (and their unplaced
        // neighbours).
        let seed_neighbors = |p: Position, w: u32, h: u32| -> [Option<Position>; 4] {
            let (x, y) = (p % w, p / w);
            [
                if y > 0 { Some((y - 1) * w + x) } else { None },
                if x + 1 < w { Some(y * w + (x + 1)) } else { None },
                if y + 1 < h { Some((y + 1) * w + x) } else { None },
                if x > 0 { Some(y * w + (x - 1)) } else { None },
            ]
        };
        for np in seed_neighbors(start_pos, w, h).iter().flatten() {
            if self.placed[*np as usize].is_none() && !on_queue[*np as usize] {
                queue.push(*np);
                on_queue[*np as usize] = true;
            }
        }
 // Cat-4b — bound the AC-3 cascade output by n_pos so we
        // skip grow_amortized work.
        let mut all_removed: Vec<UndoEntry> = Vec::with_capacity(self.puzzle.cell_count() as usize);

        let mut ac3_tick: u32 = 0;
        while let Some(a) = queue.pop() {
            ac3_tick = ac3_tick.wrapping_add(1);
            // Cooperative timeout check inside the AC-3 loop. On hard
            // puzzles a single AC-3 invocation can churn for seconds;
            // without this the outer per-N-nodes timeout check fires
 // late. — flamegraph showed `elapsed_us` cost was
            // 12.4% of joe runtime at 1/64 rate (mach_absolute_time +
            // Duration arithmetic on macOS is ~800% of an inner-loop
            // iteration). Drop to 1/4096; worst-case timeout latency is
            // still sub-millisecond on joe's 7k nps. Reading self.started
            // through an immutable borrow is fine here because count/
            // present/on_queue are disjoint fields (NLL).
            if ac3_tick & 0xfff == 0 && self.started.elapsed_us() >= timeout_at_us {
                return Ac3Outcome::Ok { removed: all_removed };
            }
            on_queue[a as usize] = false;
            if self.placed[a as usize].is_some() { continue; }

            // For each row in domain[a], check it has support across each
            // of its unplaced neighbours.
            // edge order: top=0, right=1, bottom=2, left=3
            //   neighbour-facing-edge index from a's side ↔ b's side:
            //   a-top(0)    ↔ b-bottom(2)
            //   a-right(1)  ↔ b-left(3)
            //   a-bottom(2) ↔ b-top(0)
            //   a-left(3)   ↔ b-right(1)
            let (ax, ay) = (a % w, a / w);
            let nb_info: [(Option<Position>, usize, usize); 4] = [
                (if ay > 0 { Some((ay - 1) * w + ax) } else { None }, 0, 2),
                (if ax + 1 < w { Some(ay * w + (ax + 1)) } else { None }, 1, 3),
                (if ay + 1 < h { Some((ay + 1) * w + ax) } else { None }, 2, 0),
                (if ax > 0 { Some(ay * w + (ax - 1)) } else { None }, 3, 1),
            ];

            // Accumulate the bit-diff for position `a` as words_per_pos
            // u64s — cheaper to OR back on restore than to iterate
 // individual row-ids. — stack scratch instead of the
            // arena's resize-then-truncate dance. The previous version
            // paid bzero on every queue pop (4.29% of joe runtime) and a
            // truncate on every no-diff pop. With scratch we extend the
            // arena exactly once at the end, only if there's a diff.
            const MAX_WPP: usize = 32;
            debug_assert!(words_per_pos <= MAX_WPP);
            let mut diff_scratch = [0u64; MAX_WPP];
            let mut removed_popcount: u32 = 0;
            let a_u = a as usize;
 // stack snapshot of domain_bits[a_u] for safe
            // iteration during in-place mutation. Replaces the
            // `ac3_to_check` Vec<u32> materialization (4.78% self-time
            // on `push`) plus its subsequent re-read. Now the bitset
            // walk and the support-check happen in one fused pass over
            // stack-resident words.
            let mut dom_snapshot = [0u64; MAX_WPP];
            {
                let base = a_u * words_per_pos;
                let src = &self.domain_bits[base..base + words_per_pos];
                dom_snapshot[..words_per_pos].copy_from_slice(src);
            }
 // bind read-only slices once so the inner loop
            // avoids re-dereffing self.* on every iteration. self.placed,
            // self.rows, self.same_piece_rots are all read-only here;
            // only count/present/domain_bits mutate (and those go
            // through distinct paths via `count`, `present`).
            let placed_slice = self.placed.as_slice();
            let rows_slice = self.rows.as_slice();
            let same_piece_rots_slice = self.same_piece_rots.as_slice();
            for w_idx in 0..words_per_pos {
                let mut word = dom_snapshot[w_idx];
                let bit_base = (w_idx as u32) * 64;
                while word != 0 {
                    let bit = word.trailing_zeros();
                    word &= word - 1;
                    let r_id = bit_base + bit;
                    let r = rows_slice[r_id as usize];
                    let mut supported = true;
                    let pid_base = usize::from(r.piece_id) * 4;
 // Cat-4f — precomputed 4-bit rotation mask per
                    // (row, side_a, side_b). Replaces the inner 4-iteration
                    // loop with a single AND + popcount over `present`.
                    let r_lut_base = (r_id as usize) * 16;
                    let present_word_idx = pid_base / 64;
                    let present_shift = pid_base % 64;
                    for (nb_opt, side_a, side_b) in nb_info.iter() {
                        let Some(nb) = nb_opt else { continue; };
                        if placed_slice[*nb as usize].is_some() { continue; }
                        let required = r.edges[*side_a] as usize;
                        let nb_u = *nb as usize;
                        let total = count[nb_u * stride_pos + side_b * n_colors + required];
                        // Precomputed: which rotations of r's piece satisfy
                        // edges[side_b] == r.edges[side_a]?
                        let rot_mask = same_piece_rots_slice[r_lut_base + side_a * 4 + side_b];
                        // The 4 rotation bits are at pid_base..pid_base+4
                        // in `present`. Since 4 ≤ 64 they always sit inside
                        // a single u64 word.
                        let present_word = present[nb_u * words_per_pos + present_word_idx];
                        let present_4 = ((present_word >> present_shift) & 0xF) as u8;
                        let same_piece = (rot_mask & present_4).count_ones() as u16;
                        if total <= same_piece {
                            supported = false;
                            break;
                        }
                    }
                    if !supported {
                        let bit_mask = 1u64 << (r_id as usize % 64);
                        diff_scratch[w_idx] |= bit_mask;
                        removed_popcount += 1;
                        present[a_u * words_per_pos + w_idx] &= !bit_mask;
                        self.domain_bits[a_u * words_per_pos + w_idx] &= !bit_mask;
                        for s in 0..4 {
                            let c = r.edges[s] as usize;
                            count[a_u * stride_pos + s * n_colors + c] -= 1;
                        }
                    }
                }
            }
            if removed_popcount > 0 {
                self.stats.propagations += removed_popcount as u64;
                let empty = {
                    let base = a_u * words_per_pos;
                    self.domain_bits[base..base + words_per_pos].iter().all(|&w| w == 0)
                };
                let arena_start = self.undo_words_arena.len();
                self.undo_words_arena.extend_from_slice(&diff_scratch[..words_per_pos]);
                all_removed.push(UndoEntry { pos: a, words_start: arena_start as u32 });
                if empty {
                    self.stats.domain_wipeouts += 1;
                    self.emit(sink, depth, EventBody::DomainWipeout { position: a });
                    return Ac3Outcome::Wipeout { removed: all_removed };
                }
                // Re-queue unplaced neighbours of a: their support in a
                // may have changed.
                for nb_opt in seed_neighbors(a, w, h).iter() {
                    let Some(nb) = nb_opt else { continue; };
                    if self.placed[*nb as usize].is_none() && !on_queue[*nb as usize] {
                        queue.push(*nb);
                        on_queue[*nb as usize] = true;
                    }
                }
            }
        }
        Ac3Outcome::Ok { removed: all_removed }
    }

    /// Build a 4-element neighbor snapshot for `pos`: [top, right, bot,
    /// left]. For placed neighbors we report their facing-edge color
    /// (the edge that touches `pos`). `placed_lookup` is consulted to
    /// determine which neighbors are currently placed.
    fn neighbor_info(&self, pos: Position) -> [Option<NeighborInfo>; 4] {
        let (x, y) = self.puzzle.xy(pos);
        let w = self.puzzle.width;
        let h = self.puzzle.height;
        let nps: [(Option<Position>, usize); 4] = [
            (if y > 0 { Some((y - 1) * w + x) } else { None }, 2),  // top: their bottom face
            (if x + 1 < w { Some(y * w + (x + 1)) } else { None }, 3), // right: their left
            (if y + 1 < h { Some((y + 1) * w + x) } else { None }, 0), // bot: their top
            (if x > 0 { Some(y * w + (x - 1)) } else { None }, 1),  // left: their right
        ];
        let mut out: [Option<NeighborInfo>; 4] = [None; 4];
        for (i, (np, facing_idx)) in nps.iter().enumerate() {
            let Some(np) = np else { continue; };
            let placed = self.placed[*np as usize].is_some();
            let facing_color = if placed {
                let row_id = self.placed[*np as usize].unwrap();
                self.rows[row_id as usize].edges[*facing_idx]
            } else {
                0 // arbitrary; consumer ignores facing_color when placed=false
            };
            out[i] = Some(NeighborInfo { placed, facing_color });
        }
        out
    }

    fn run_extra_propagators(&self, depth: u32) -> PropagatorResult {
        if !self.config.propagators.class_balance
            && !self.config.propagators.parity
            && !self.config.propagators.island
            && !self.config.propagators.gacolor
            && !self.config.propagators.multiset_equality
        {
            return PropagatorResult::Ok;
        }
        // Depth gate (Joe-Saunders 2026): suppress Step-8 propagators
        // below threshold so early-search throughput approaches the
        // bare-edge-equality + AC-3 inner-loop ceiling.
        if let Some(threshold) = self.config.propagators.depth_threshold {
            if depth < threshold {
                return PropagatorResult::Ok;
            }
        }
        let placed_info = self.snapshot_placement_info();
        let ctx = PropagatorContext {
            puzzle: self.puzzle,
            placed: &placed_info,
            used_pieces: &self.used,
            domain_bits: &self.domain_bits,
            words_per_pos: self.words_per_pos,
        };
        if self.config.propagators.class_balance
            && class_balance_check(&ctx) == PropagatorResult::Wipeout
        {
            return PropagatorResult::Wipeout;
        }
        // gacolor uses the incremental cache for O(color_count) checks;
        // a fresh full recompute via `gacolor_check` is available for
        // unit testing parity vs incremental correctness, but the hot
        // path goes through GaColorState.
        if self.config.propagators.gacolor {
            let _ = gacolor_check; // keep symbol live for tests
            if let Some(gc) = self.gacolor.as_ref() {
                if gc.feasible() == PropagatorResult::Wipeout {
                    return PropagatorResult::Wipeout;
                }
            }
        }
        if self.config.propagators.island
            && island_check(&ctx) == PropagatorResult::Wipeout
        {
            return PropagatorResult::Wipeout;
        }
        if self.config.propagators.parity
            && parity_check(&ctx) == PropagatorResult::Wipeout
        {
            return PropagatorResult::Wipeout;
        }
        // NS-1 multiset equality: cheapest after the border ring closes;
        // before that the supply terms are loose and rarely triggers.
        if self.config.propagators.multiset_equality
            && multiset_equality_check(&ctx) == PropagatorResult::Wipeout
        {
            return PropagatorResult::Wipeout;
        }
        PropagatorResult::Ok
    }

    pub(crate) fn restore(&mut self, undo: Vec<UndoEntry>) {
        let words_per_pos = self.words_per_pos;
        // Track the lowest arena offset we need to keep — everything
        // above it was reserved by these entries and is safe to drop.
        // Entries are pushed in order during place_and_propagate, so
        // the minimum is the first entry's words_start. Empty undo log
        // means nothing was reserved here.
        let arena_keep = undo.iter().map(|e| e.words_start as usize).min();
 // Cat-2 follow-up: take a non-overlapping view of the
        // arena (read-only borrow) and the domain_bits (mutable). For
        // each entry, slice both to a length-wpp window so the inner
        // OR loop is bounds-check-free (the compiler elides them when
        // length is known from the slice). This was 19.1% self time
        // in the post-Cat-4 profile.
        let arena_slice = self.undo_words_arena.as_slice();
        let dom_slice = self.domain_bits.as_mut_slice();
        for entry in &undo {
            let pos_u = entry.pos as usize;
            let bit_base = pos_u * words_per_pos;
            let ws = entry.words_start as usize;
            // SAFETY-NOTE: these indices are guaranteed in-bounds by
            // construction (pos < cell_count, words_start was pushed
            // into the arena with wpp zeros, so words_start + wpp <=
            // arena.len()). We rely on slice access + iter().zip()
            // to let the compiler elide the bounds-check.
            let dst = &mut dom_slice[bit_base..bit_base + words_per_pos];
            let src = &arena_slice[ws..ws + words_per_pos];
            for (d, s) in dst.iter_mut().zip(src.iter()) {
                *d |= *s;
            }
        }
 // mark every restored position's ac3_count stale. Done
        // after the OR loop so we drop the dom_slice/arena_slice borrows
        // before calling the (&mut self) mark helper.
        for entry in &undo {
            self.mark_ac3_dirty(entry.pos as usize);
        }
        drop(undo);
        if let Some(keep) = arena_keep {
            self.undo_words_arena.truncate(keep);
        }
    }

    fn board_from_state(&self) -> Board {
        let mut b = Board::empty(self.puzzle);
        for (i, p) in self.placed.iter().enumerate() {
            if let Some(row_id) = p {
                let r = self.rows[*row_id as usize];
                b.place(i as Position, r.piece_id, Rotation::from_u8(r.rotation).unwrap());
            }
        }
        b
    }

    // Apply symmetry-breaking pin (canonical corner at (0,0) when no user
    // hint touches it) and user-supplied hints (each hint position has its
    // domain restricted to the chosen (piece, rotation) and is committed
    // via place_and_propagate). Returns Err with the outcome if either
    // step is infeasible. Used by both the single-threaded run() and the
    // parallel root-split path so hints are honoured in both.
    pub(crate) fn apply_symmetry_and_hints(
        &mut self,
        sink: &mut dyn EventSink,
    ) -> Result<(), SolveOutcome> {
        // Symmetry-breaking. The board has D4 rotational symmetry: each
        // solution has 4 rotational copies (or 8 if we also count
        // reflection, which we don't here because pieces are
        // distinguishable). We pin the lowest-id corner piece at (0,0)
        // in its single rotation that puts BORDER on top and left. Any
        // valid solution under our pieces has *some* corner piece at
        // (0,0); by forcing the canonical one we visit exactly 1/4 of
        // the symmetric copies. If the puzzle's user_hints already pin
        // (0,0), we skip this (user knows best).
        if self.config.break_symmetry && self.opts.hints.hints.iter().all(|h| h.position != 0) {
            let canonical_corner = self.puzzle.pieces().iter()
                .filter(|p| p.is_corner())
                .min_by_key(|p| p.id);
            if let Some(piece) = canonical_corner {
                let pid = usize::from(piece.id);
                let canonical_row_id = (0..4u32).find_map(|r| {
                    let row = self.rows[pid * 4 + r as usize];
                    if !row.valid { return None; }
                    if row.edges[0] == BORDER && row.edges[3] == BORDER {
                        Some(pid as u32 * 4 + r)
                    } else { None }
                });
                if let Some(row_id) = canonical_row_id {
                    if self.domain_contains(0, row_id) {
                        self.pin_to(0, row_id);
                        if let PropagationOutcome::Wipeout { .. } =
                            self.place_and_propagate(sink, 0, 0, row_id)
                        {
                            return Err(SolveOutcome::Error(
                                "symmetry-breaking placement caused immediate wipeout".into()
                            ));
                        }
                    }
                }
            }
        }

 // batch hint application mode for prune-restart.
        //
        // In `per-hint` (default) mode: pin each hint and propagate
        // immediately. The propagation removes options from other cells'
        // domains; if a later hint's row was among those removed, the
        // engine rejects the hint set even when it's globally consistent.
        // This happens routinely for hint sets ≥ ~50 cells extracted from
        // a DFS partial (the DFS visited cells in MRV order, not the
        // order we pin them).
        //
        // In `batch` mode: pin all hint domains first (no propagation
        // between hints), then run propagation once at the end. Self-
        // consistent hint sets always succeed.
        if self.opts.batch_hint_application {
            // Phase 1: pin all hint domains + claim pieces, with no
            // propagation. Validate each row is plausible (in domain
            // BEFORE any hint was pinned — checked at construction).
            for h in self.opts.hints.hints.clone() {
                let row_id = u32::from(h.piece_id) * 4 + u32::from(h.rotation.as_u8());
                let pos_idx = h.position as usize;
                let n_pos = self.puzzle.cell_count() as usize;
                if pos_idx >= n_pos {
                    return Err(SolveOutcome::Error(format!(
                        "hint at position {} out of range", h.position
                    )));
                }
                // domain_contains is checked against the FRESHLY BUILT
                // domain_bits (only border-class filtered, no propagation
                // applied yet). Self-consistent hints pass.
                if !self.domain_contains(pos_idx, row_id) {
                    return Err(SolveOutcome::Error(format!(
                        "hint at position {} (pid={}, rot={}) has border-class mismatch",
                        h.position, h.piece_id, h.rotation.as_u8()
                    )));
                }
                // Sanity: piece must not already be used. Catches duplicate
                // hints for the same piece-id.
                let piece_idx = usize::from(h.piece_id);
                if piece_idx < self.used.len() && self.used[piece_idx] {
                    return Err(SolveOutcome::Error(format!(
                        "hint at position {} uses piece {} which is already pinned elsewhere",
                        h.position, h.piece_id
                    )));
                }
                self.pin_to(pos_idx, row_id);
                self.placed[pos_idx] = Some(row_id);
                self.used[piece_idx] = true;
 // update Blackwood heuristic count for this placement
                // (mirroring the increment in place_and_propagate).
                let row = self.rows[row_id as usize];
                let inc = self.count_heuristic_in_row(&row.edges);
                self.placed_heuristic_count = self.placed_heuristic_count.saturating_add(inc);
                // Also update gacolor occupancy if active.
                if self.gacolor.is_some() {
                    let n_info = self.neighbor_info(h.position);
                    if let Some(gc) = self.gacolor.as_mut() {
                        gc.apply_place(&row.edges, &n_info);
                    }
                }
            }
            // Phase 2: now propagate from all pinned cells. We trigger
            // propagation by re-pinning the LAST hint via
            // place_and_propagate; the propagators are global and will
            // see all already-pinned cells.
            //
            // Special case: if hints is empty, skip propagation entirely.
            if let Some(last) = self.opts.hints.hints.last().copied() {
                let row_id = u32::from(last.piece_id) * 4 + u32::from(last.rotation.as_u8());
                let pos_idx = last.position as usize;
                // Undo the last hint's pin/placed/used so place_and_propagate
                // can re-apply it cleanly; the bookkeeping side-effects (count,
                // gacolor) are idempotent under undo+redo.
                self.placed[pos_idx] = None;
                let piece_idx = usize::from(last.piece_id);
                self.used[piece_idx] = false;
                let row = self.rows[row_id as usize];
                let dec = self.count_heuristic_in_row(&row.edges);
                self.placed_heuristic_count = self.placed_heuristic_count.saturating_sub(dec);
                if self.gacolor.is_some() {
                    let n_info = self.neighbor_info(last.position);
                    if let Some(gc) = self.gacolor.as_mut() {
                        gc.apply_unplace(&row.edges, &n_info);
                    }
                }
                self.pin_to(pos_idx, row_id);
                if let PropagationOutcome::Wipeout { .. } =
                    self.place_and_propagate(sink, 0, last.position, row_id)
                {
                    return Err(SolveOutcome::Error(format!(
                        "batched hint propagation wiped out (final pin at position {})",
                        last.position
                    )));
                }
            }
        } else {
            for h in self.opts.hints.hints.clone() {
                let row_id = u32::from(h.piece_id) * 4 + u32::from(h.rotation.as_u8());
                let pos_idx = h.position as usize;
                let n_pos = self.puzzle.cell_count() as usize;
                if pos_idx >= n_pos || !self.domain_contains(pos_idx, row_id) {
                    return Err(SolveOutcome::Error(format!(
                        "hint at position {} is incompatible with constraints", h.position
                    )));
                }
                self.pin_to(pos_idx, row_id);
                if let PropagationOutcome::Wipeout { .. } = self.place_and_propagate(sink, 0, h.position, row_id) {
                    return Err(SolveOutcome::Error(format!(
                        "hint at position {} causes immediate wipeout", h.position
                    )));
                }
            }
        }
        Ok(())
    }

 /// wrapper: like `run` but also returns the per-position
    /// backtrack counters for downstream diagnostic analysis.
    pub(crate) fn run_with_diag(self, sink: &mut dyn EventSink) -> (SolveOutcome, Vec<u64>) {
        // Save a handle before run() consumes self; we'll repopulate
        // after by routing through `run`'s consumed body. The simplest
        // way is: replicate run() inline and snapshot at end. Since
        // run() is non-trivial we just call the existing run() via a
        // thin trampoline that exposes the counters.
        // Implementation: run consumes self, so we must replicate
        // the small wrapping. We achieve this by giving `run` the
        // option to return diag too.
        let (outcome, diag) = Self::run_inner(self, sink);
        (outcome, diag)
    }

    fn run_inner(state: Self, sink: &mut dyn EventSink) -> (SolveOutcome, Vec<u64>) {
        // Equivalent to `run` but yields pos_backtracks.
        let mut s = state;
        let outcome = s.run_body(sink);
        let diag = std::mem::take(&mut s.pos_backtracks);
        (outcome, diag)
    }

    /// Run body shared with `run_inner`. Renamed `run` to `run_body`.
    pub(crate) fn run_body(&mut self, sink: &mut dyn EventSink) -> SolveOutcome {
        self.run_impl(sink)
    }

    /// Backwards compat: old `run(self)` consumed self and returned outcome.
    pub(crate) fn run(mut self, sink: &mut dyn EventSink) -> SolveOutcome {
        self.run_impl(sink)
    }

    fn run_impl(&mut self, sink: &mut dyn EventSink) -> SolveOutcome {
        self.emit(sink, 0, EventBody::Started {
            solver_id: self.solver_id.clone(),
            heuristic_profile: self.heuristic_profile.clone(),
            puzzle_fingerprint: self.puzzle.fingerprint(),
            seed: self.opts.seed,
            started_wall_us: 0,
        });

        if let Err(e) = self.apply_symmetry_and_hints(sink) {
            return e;
        }

        let mut solutions = Vec::<Board>::new();
        let res = self.recurse(sink, 0, &mut solutions);

        self.stats.time_ms = self.elapsed_us() / 1000;
        match res {
            RecurseResult::Found if matches!(self.opts.mode, SolveMode::FirstSolution) => {
                self.stats.solutions_found = 1;
                let board = solutions.into_iter().next().unwrap();
                self.emit(sink, 0, EventBody::Solved {
                    board: board.clone(), final_stats: self.stats,
                });
                SolveOutcome::Solved(board)
            }
            RecurseResult::Found | RecurseResult::Exhausted => {
                self.stats.solutions_found = solutions.len() as u64;
                self.emit(sink, 0, EventBody::Exhausted {
                    final_stats: self.stats,
                    solutions_found: solutions.len() as u64,
                });
 // under MaxScore, surface the highest-score
                // FULL completion seen as `Solved`. If no completion was
                // reached (heavy pruning), fall back to Exhausted; the
                // caller can inspect `best_partial` via TimedOut paths.
                if matches!(self.opts.objective, Some(Objective::MaxScore)) {
                    if let Some(board) = self.best_score_partial.clone() {
                        self.emit(sink, 0, EventBody::Solved {
                            board: board.clone(), final_stats: self.stats,
                        });
                        return SolveOutcome::Solved(board);
                    }
                    return SolveOutcome::Exhausted;
                }
                if matches!(self.opts.mode, SolveMode::FirstSolution) || solutions.is_empty() {
                    SolveOutcome::Exhausted
                } else {
                    SolveOutcome::AllSolutions(solutions)
                }
            }
            RecurseResult::TimedOut => {
 // under MaxScore, prefer the highest-score full
                // completion seen over the depth-best partial. Cold-start
                // round-2 use case: a 412 completion is better than a
                // depth-297 partial that ALNS would have to repair.
                let best = if matches!(self.opts.objective, Some(Objective::MaxScore))
                    && self.best_score_partial.is_some()
                {
                    self.best_score_partial.clone().unwrap()
                } else {
                    self.best_partial.clone().unwrap_or_else(|| self.board_from_state())
                };
                let best_depth = self.best_depth;
                self.emit(sink, 0, EventBody::TimedOut {
                    final_stats: self.stats,
                    best_partial: best.clone(),
                    best_depth,
                });
                SolveOutcome::TimedOut { best_partial: best, best_depth }
            }
            RecurseResult::Cancelled => {
 // same surfacing rule as TimedOut.
                let best = if matches!(self.opts.objective, Some(Objective::MaxScore))
                    && self.best_score_partial.is_some()
                {
                    self.best_score_partial.clone().unwrap()
                } else {
                    self.best_partial.clone().unwrap_or_else(|| self.board_from_state())
                };
                let best_depth = self.best_depth;
                self.emit(sink, 0, EventBody::Cancelled {
                    final_stats: self.stats,
                    best_partial: best.clone(),
                    best_depth,
                    solutions_so_far: solutions.clone(),
                });
                SolveOutcome::Cancelled {
                    best_partial: best, best_depth,
                    solutions_so_far: solutions,
                }
            }
        }
    }

    // Phase-1 enumerator for RootSplit parallelism. Walks the search to
    // `target_depth`, pushing each valid partial placement (as a sequence
    // of (position, row_id)) to `units`. Caller resets engine state
    // between calls; restoration of placed/used/domains here mirrors the
    // standard recursion's backtracking. Limits to `cap_units` to avoid
    // pathological enumeration on already-easy puzzles.
    pub(crate) fn enumerate_units(
        &mut self,
        depth: u32,
        target_depth: u32,
        prefix: &mut Vec<(Position, u32)>,
        units: &mut Vec<Vec<(Position, u32)>>,
        cap_units: usize,
    ) {
        if units.len() >= cap_units { return; }
        if depth >= target_depth {
            units.push(prefix.clone());
            return;
        }
        let pos = match self.select_position() {
            Some(p) => p,
            None => {
                // Reached a complete state in the prefix — treat as a unit
                // even though there's nothing left to search; the worker
                // will discover the solution immediately.
                units.push(prefix.clone());
                return;
            }
        };
        let domain_snapshot: Vec<u32> = self.domain_iter(pos as usize).collect();
        let saved_bits = self.snapshot_bits(pos as usize);
        for &row_id in &domain_snapshot {
            if units.len() >= cap_units { return; }
            let row = self.rows[row_id as usize];
            let piece_idx = usize::from(row.piece_id);
            if self.used[piece_idx] { continue; }
            // Clear domain[pos] so place_and_propagate's prunes don't see
            // alternatives to the chosen row_id.
            let base = (pos as usize) * self.words_per_pos;
            for w in &mut self.domain_bits[base..base + self.words_per_pos] { *w = 0; }
 // domain[pos] changed; ac3_count[pos] is stale.
            self.mark_ac3_dirty(pos as usize);
            let mut null = eternity2_events::NullSink;
            match self.place_and_propagate(&mut null, depth, pos, row_id) {
                PropagationOutcome::Ok { undo } => {
                    prefix.push((pos, row_id));
                    self.enumerate_units(depth + 1, target_depth, prefix, units, cap_units);
                    prefix.pop();
                    self.restore(undo);
                }
                PropagationOutcome::Wipeout { undo } => {
                    self.restore(undo);
                }
            }
            self.undo_place(pos, row_id);
            self.restore_bits(pos as usize, &saved_bits);
            if units.len() >= cap_units { return; }
        }
    }

    pub(crate) fn recurse(
        &mut self,
        sink: &mut dyn EventSink,
        depth: u32,
        solutions: &mut Vec<Board>,
    ) -> RecurseResult {
        if self.node_id & 0xf == 0 {
            if !sink.should_continue() { return RecurseResult::Cancelled; }
            if self.timed_out() { return RecurseResult::TimedOut; }
            if self.node_budget_exhausted() { return RecurseResult::TimedOut; }
        }

        self.stats.current_depth = depth;
        if depth > self.best_depth {
            self.best_depth = depth;
            self.best_partial = Some(self.board_from_state());
            self.stats.max_depth_seen = depth;
        }

 // MaxScore objective: branch-and-bound on the matched-
        // edge count. `decided_edges` tracks edges with both ends placed;
        // every other internal edge is bounded by 1 match. If even the
        // optimistic completion can't beat the running best, prune the
        // whole subtree. `<=` (not `<`) so we never re-explore a subtree
        // that can only tie. O(1) test; safe to run unconditionally
        // when objective is set (`matches!` is a cheap discriminant cmp).
        //
        // Under RootSplit parallelism, `shared_best_score` (Arc<AtomicU32>)
        // makes the cutoff cross-worker: every worker reads the global
        // running best on each node and CAS-bumps it on each leaf. This
        // is the only way parallel BnB is monotone-optimal — without it,
        // each worker's `best_score` is local and workers can return
        // strictly suboptimal boards.
        if matches!(self.opts.objective, Some(Objective::MaxScore)) {
            let global = self.shared_best_score.as_ref()
                .map(|a| a.load(std::sync::atomic::Ordering::Relaxed))
                .unwrap_or(0);
            let cutoff = self.best_score.max(global);
            if self.matched_count + (self.total_internal_edges - self.decided_edges)
                <= cutoff
            {
                self.stats.backtracks += 1;
                return RecurseResult::Exhausted;
            }
        }

 // Blackwood schedule prune: BEFORE picking a position,
        // verify the running heuristic-color count is on-schedule for
        // the total number of placed cells (= depth + hint count).
        // The schedule is calibrated against TOTAL cells-on-board
        // (Blackwood includes hints in his cell index). Our engine's
        // `depth` is search-depth starting after hints; we add
        // `hint_offset` to make the schedule comparison total-cells-
        // accurate. If we've fallen behind, the branch can't recover:
        // only more cells get placed from here, each adding ≤ 4
        // heuristic-color edges (monotone non-decreasing).
        if let Some(sched) = self.blackwood.as_ref() {
            let hint_offset = self.opts.hints.hints.len() as u32;
            let total_placed = depth + hint_offset;
            let max_idx = sched.max_heuristic_index;
            if total_placed <= max_idx {
                let target = sched.target_at(total_placed);
                if self.placed_heuristic_count < target {
                    return RecurseResult::Exhausted;
                }
                if self.placed_heuristic_count >= sched.heuristic_pool_size
                    && total_placed < max_idx
                    && sched.target_at(max_idx) > sched.heuristic_pool_size
                {
                    return RecurseResult::Exhausted;
                }
            }
        }

        let pos = match self.select_position() {
            Some(p) => p,
            None => {
 // under MaxScore, a "complete" placement (no
                // more positions to fill) is just a candidate: capture
                // its score if it beats the running best, then return
                // Exhausted so the parent loop keeps trying alternatives
                // at the deepest still-meaningful node. `Found` would
                // unwind under FirstSolution mode, but MaxScore wants
                // to enumerate.
                if matches!(self.opts.objective, Some(Objective::MaxScore)) {
                    if self.matched_count > self.best_score {
                        self.best_score = self.matched_count;
                        self.best_score_partial = Some(self.board_from_state());
                    }
                    // Bump the shared cutoff so sibling workers prune
                    // against the new high-water mark immediately. Use
                    // fetch_max-via-CAS-loop (no fetch_max on AtomicU32
                    // in older toolchains, but stable since 1.45; use
                    // fetch_max directly).
                    if let Some(atom) = self.shared_best_score.as_ref() {
                        atom.fetch_max(self.matched_count,
                            std::sync::atomic::Ordering::Relaxed);
                    }
                    return RecurseResult::Exhausted;
                }
                solutions.push(self.board_from_state());
                return RecurseResult::Found;
            }
        };

        let mut domain_snapshot: Vec<u32> = self.domain_iter(pos as usize).collect();
        let reason = self.selection_reason();
        self.emit(sink, depth, EventBody::VariableSelected {
            position: pos,
            domain_size: domain_snapshot.len() as u32,
            score: 0.0,
            reason,
        });

        // LCV — Least Constraining Value. Score each candidate row by
        // the number of neighbour-domain rows it would eliminate, pick
        // ascending so we try the "safest" (most-future-options) values
        // first. Standard CP technique (Haralick & Elliott 1980).
        //
        // Cost: O(|D| × 4 × |D_nb|) once per node. Pays for itself when
        // it lets us avoid a deep wrong branch.
        if matches!(self.config.value_order, ValueOrder::LeastConstraining)
            && domain_snapshot.len() > 1
        {
            let (x, y) = self.puzzle.xy(pos);
            let w = self.puzzle.width;
            let h = self.puzzle.height;
            // (neighbour, our_side, their_side)
            let nbs: [(Option<Position>, usize, usize); 4] = [
                (if y > 0 { Some((y - 1) * w + x) } else { None }, 0, 2),
                (if x + 1 < w { Some(y * w + (x + 1)) } else { None }, 1, 3),
                (if y + 1 < h { Some((y + 1) * w + x) } else { None }, 2, 0),
                (if x > 0 { Some(y * w + (x - 1)) } else { None }, 3, 1),
            ];
            let mut scored: Vec<(u32, u32)> = domain_snapshot.iter().map(|&r_id| {
                let r = self.rows[r_id as usize];
                let mut prune_count = 0u32;
                for (nb_opt, our_side, their_side) in nbs.iter() {
                    let Some(nb) = nb_opt else { continue; };
                    if self.placed[*nb as usize].is_some() { continue; }
                    let needed = r.edges[*our_side];
                    for nb_r_id in self.domain_iter(*nb as usize) {
                        let nb_r = self.rows[nb_r_id as usize];
                        if nb_r.piece_id == r.piece_id || nb_r.edges[*their_side] != needed {
                            prune_count += 1;
                        }
                    }
                }
                (prune_count, r_id)
            }).collect();
            scored.sort_by_key(|(s, _)| *s);
            domain_snapshot.clear();
            domain_snapshot.extend(scored.into_iter().map(|(_, r)| r));
        }

        // RandomShuffle value order: shuffle the candidate-row order using
        // the opts.seed-derived RNG. Combined with deterministic
        // BorderFirstMrv variable ordering, this diversifies CP partials
        // across seeds while keeping the variable selection strong.
        if matches!(self.config.value_order, ValueOrder::RandomShuffle)
            && domain_snapshot.len() > 1
        {
            // Fisher-Yates with our seeded RNG.
            let n = domain_snapshot.len();
            for i in (1..n).rev() {
                let j = (self.next_random() as usize) % (i + 1);
                domain_snapshot.swap(i, j);
            }
        }

        // EdgeBpMarginals (and LearnedOnTies, which runs this first):
        // score each row by Σ over its 4 sides of the BP marginal mass
        // at the row's edge color. Sort descending so the engine tries
        // higher-mass colors first. Silently no-op if marginals or the
        // mapping table aren't populated.
        let is_edge_bp = matches!(self.config.value_order, ValueOrder::EdgeBpMarginals);
        let is_learned_on_ties = matches!(self.config.value_order, ValueOrder::LearnedOnTies);
        let mut bp_keys: Vec<u64> = Vec::new();  // parallel to domain_snapshot after sort
        if (is_edge_bp || is_learned_on_ties)
            && domain_snapshot.len() > 1
            && !self.cell_side_edge.is_empty()
            && self.opts.edge_bp_marginals.is_some()
        {
            let marg = self.opts.edge_bp_marginals.as_ref().unwrap();
            let base_side = (pos as usize) * 4;
            let eids = [
                self.cell_side_edge[base_side + 0] as usize,
                self.cell_side_edge[base_side + 1] as usize,
                self.cell_side_edge[base_side + 2] as usize,
                self.cell_side_edge[base_side + 3] as usize,
            ];
            let mut scored: Vec<(u64, u32)> = domain_snapshot.iter().map(|&r_id| {
                let r = self.rows[r_id as usize];
                let mut s = 0.0f32;
                for side in 0..4 {
                    let c = r.edges[side] as usize;
                    if c < EDGE_BP_NSTATE {
                        s += marg[eids[side] * EDGE_BP_NSTATE + c];
                    }
                }
                let q = (s * 1.0e6).max(0.0).min(4.0e6) as u64;
                (u64::MAX - q, r_id)
            }).collect();
            scored.sort_by_key(|(k, _)| *k);
            domain_snapshot.clear();
            domain_snapshot.extend(scored.iter().map(|(_, r)| *r));
            if is_learned_on_ties {
                bp_keys.extend(scored.iter().map(|(k, _)| *k));
            }
        }

 // LearnedOnTies: rerank only the top-k tied candidates
        // by Learned scores. "Tied" = within EPS of the top BP score.
        // Cheaper per-node than the full Learned variant (we call the
        // NN only when there's an ambiguous choice), and dominated by
        // EdgeBpMarginals when no ties exist.
        // Tunable: E2_LOT_EPS (BP-score units, default 0.05) and
        // E2_LOT_MAX_K (default 8). Read once per recurse — cheap and
        // lets us sweep without rebuilding.
        // Compiled only with `learned-order`; without it, LearnedOnTies keeps
        // the pure EdgeBpMarginals order (the tie-rerank block is absent).
        #[cfg(all(not(target_arch = "wasm32"), feature = "learned-order"))]
        if is_learned_on_ties && bp_keys.len() >= 2 {
            let eps_f: f32 = std::env::var("E2_LOT_EPS")
                .ok().and_then(|s| s.parse().ok()).unwrap_or(0.05);
            let eps_u: u64 = (eps_f * 1.0e6).clamp(0.0, 4.0e6) as u64;
            let max_tie_k: usize = std::env::var("E2_LOT_MAX_K")
                .ok().and_then(|s| s.parse().ok()).unwrap_or(8);
            let eps: u64 = eps_u;
            let max_tie_k_v: usize = max_tie_k;
            let top_key = bp_keys[0];
            // Count near-tied candidates (smaller u64 key = higher BP score
            // in the original space). bp_keys is sorted ascending.
            let mut tie_len = 1usize;
            while tie_len < bp_keys.len()
                && tie_len < max_tie_k_v
                && bp_keys[tie_len].saturating_sub(top_key) <= eps
            {
                tie_len += 1;
            }
 // T4 instrumentation: if E2_LOT_TRACE is set, emit
            // (depth, tie_len, dom_len) to stderr so the Python diagnostic
            // can compute per-depth fire-rate.
            if std::env::var("E2_LOT_TRACE").is_ok() {
                eprintln!("LOT_TRACE depth={} pos={} dom={} tie_len={} top_key={}",
                    depth, pos, bp_keys.len(), tie_len, top_key);
            }
            if tie_len >= 2 {
                let tie_slice = &domain_snapshot[..tie_len].to_vec();
                if let Some(scores) = self.learned_score_candidates(pos, tie_slice) {
                    // Sort the tie region by Learned score descending,
                    // preserving the rest of domain_snapshot as-is.
                    let mut paired: Vec<(u64, u32)> = tie_slice
                        .iter()
                        .zip(scores.iter())
                        .map(|(&r_id, &s)| {
                            let q = (s * 1.0e6f32).max(-1.0e9).min(1.0e9) as i64;
                            let key = (i64::MAX as i128 - q as i128) as u64;
                            (key, r_id)
                        })
                        .collect();
                    paired.sort_by_key(|(k, _)| *k);
                    for (i, (_, r_id)) in paired.into_iter().enumerate() {
                        domain_snapshot[i] = r_id;
                    }
                }
            }
        }

 // Learned imitation-policy value order. Reorders candidates by
        // ONNX-model scores. Compiled only with the `learned-order` feature;
        // without it, ValueOrder::Learned keeps the insertion-order fallback
        // (this block is simply absent, so the order is unchanged).
        #[cfg(all(not(target_arch = "wasm32"), feature = "learned-order"))]
        if matches!(self.config.value_order, ValueOrder::Learned)
            && domain_snapshot.len() > 1
        {
            if let Some(scores) = self.learned_score_candidates(pos, &domain_snapshot) {
                // scores[i] corresponds to domain_snapshot[i]. Sort
                // candidates by descending score using a fixed-point key.
                let mut paired: Vec<(u64, u32)> = domain_snapshot
                    .iter()
                    .zip(scores.iter())
                    .map(|(&r_id, &s)| {
                        let q = (s * 1.0e6f32).max(-1.0e9).min(1.0e9) as i64;
                        // i64 -> u64 descending key: subtract from a large
                        // anchor so larger s -> smaller u64 (sort ascending
                        // gives descending score).
                        let key = (i64::MAX as i128 - q as i128) as u64;
                        (key, r_id)
                    })
                    .collect();
                paired.sort_by_key(|(k, _)| *k);
                domain_snapshot.clear();
                domain_snapshot.extend(paired.into_iter().map(|(_, r)| r));
            }
            // If learned_score_candidates returned None (bridge
            // unavailable), keep the current order — preserves the
            // insertion-order fallback documented on ValueOrder::Learned.
        }

        // PreferredFirst: stable partition; preferred-piece rows first.
        // Verhaard 2008. Caller sets `opts.preferred_pieces`.
        if matches!(self.config.value_order, ValueOrder::PreferredFirst)
            && domain_snapshot.len() > 1
            && !self.opts.preferred_pieces.is_empty()
        {
            // O(|preferred|) bitset lookup keeps this cheap per node.
            let max_pid = self.opts.preferred_pieces.iter().map(|p| *p as usize).max().unwrap_or(0) + 1;
            let mut pref = vec![false; max_pid];
            for &pid in &self.opts.preferred_pieces {
                if (pid as usize) < pref.len() { pref[pid as usize] = true; }
            }
            // Stable partition: preferred first, others after; preserve
            // existing relative order within each bucket.
            let mut a: Vec<u32> = Vec::with_capacity(domain_snapshot.len());
            let mut b: Vec<u32> = Vec::new();
            for &r_id in domain_snapshot.iter() {
                let pid = self.rows[r_id as usize].piece_id as usize;
                if pid < pref.len() && pref[pid] {
                    a.push(r_id);
                } else {
                    b.push(r_id);
                }
            }
            a.extend(b);
            domain_snapshot.clear();
            domain_snapshot.extend(a);
        }

 // BlackwoodHeuristic value-order: rank rows by count
        // of heuristic-color edges (descending). Schedule-rich rows
        // tried first so the running heuristic count keeps pace with
        // the schedule's target_at(depth+1).
        //
 // (H22) — within-tie shuffle. The heuristic-color
        // count is an integer in 0..=4; many rows score equal. By
        // default sort_by_key is stable (preserves insertion order
        // within ties). With `config.shuffle_within_blackwood_ties`,
        // we shuffle the rows WITHIN each tie group using the
        // seed-derived RNG. This gives schedule-invariant CP-partial
        // diversity (different seeds → different shuffle → different
        // search trajectories) WITHOUT breaking Blackwood's score-
        // descending invariant.
        if matches!(self.config.value_order, ValueOrder::BlackwoodHeuristic)
            && !self.heuristic_color_mask.is_empty()
            && domain_snapshot.len() > 1
        {
            let mut scored: Vec<(u32, u32)> = domain_snapshot
                .iter()
                .map(|&r_id| {
                    let r = self.rows[r_id as usize];
                    let h = self.count_heuristic_in_row(&r.edges);
                    // Descending: invert the key.
                    (u32::MAX - h, r_id)
                })
                .collect();
            scored.sort_by_key(|(k, _)| *k);
            if self.config.shuffle_within_blackwood_ties {
                // Within each contiguous run of equal scores, Fisher-
                // Yates shuffle using seeded RNG.
                let n = scored.len();
                let mut i = 0;
                while i < n {
                    let mut j = i + 1;
                    while j < n && scored[j].0 == scored[i].0 { j += 1; }
                    // Shuffle range [i, j).
                    if j - i > 1 {
                        for k in (i + 1..j).rev() {
                            let r = (self.next_random() as usize) % (k - i + 1);
                            scored.swap(k, i + r);
                        }
                    }
                    i = j;
                }
            }
            domain_snapshot.clear();
            domain_snapshot.extend(scored.into_iter().map(|(_, r)| r));
        }

 // RecordsPrior value-order. Rank rows by record frequency
        // at this position (from opts.records_prior_map[pos]). Rows whose
        // (piece_id, rotation) appears in N/7 records get a key of u32::MAX - N,
        // so descending-frequency wins. Unranked rows get u32::MAX (lowest priority).
        // No-op if records_prior_map is None.
        if matches!(self.config.value_order, ValueOrder::RecordsPrior)
            && domain_snapshot.len() > 1
            && self.opts.records_prior_map.is_some()
        {
            let map = self.opts.records_prior_map.as_ref().unwrap();
            if (pos as usize) < map.len() {
                let cell_priors = &map[pos as usize];
                // Build (piece_id, rotation) → freq lookup. Linear scan
                // OK since cell_priors is typically ≤7 entries.
                let mut scored: Vec<(u32, u32)> = domain_snapshot
                    .iter()
                    .map(|&r_id| {
                        let r = self.rows[r_id as usize];
                        let freq = cell_priors.iter()
                            .find(|(p, rot, _)| {
                                u16::from(r.piece_id) == *p && r.rotation == *rot
                            })
                            .map(|(_, _, f)| *f)
                            .unwrap_or(0);
                        // Descending: invert key. Unranked (freq=0) → u32::MAX.
                        (u32::MAX - freq, r_id)
                    })
                    .collect();
                scored.sort_by_key(|(k, _)| *k);
                domain_snapshot.clear();
                domain_snapshot.extend(scored.into_iter().map(|(_, r)| r));
            }
        }

 // Blackwood break-index: at a break depth, the cell's
        // pruned domain may be empty or too narrow because a previously
        // placed neighbour pruned all rows with the "right" matching
        // color. Augment the candidate set with any row that:
        //   (a) matches this cell's border_mask pattern,
        //   (b) has piece-id not already used,
        //   (c) has ≤ 1 placed-neighbour edge mismatch.
        // The placed row will be flagged so place_and_propagate skips
        // the prune on the mismatched side. Stored in `break_candidates`
        // alongside their mismatched-side index (0..3) or u8::MAX for
        // exact match. Exact-match (mismatched-side = MAX) rows take
        // priority over relaxed ones; among relaxed, prefer rows that
        // satisfy more of the schedule.
        let break_candidates: Vec<(u32, u8)> = if !self.is_break_index.is_empty()
            && self.is_pos_break_index(pos)
        {
            let n_rows = self.rows.len() as u32;
            let mut admitted: Vec<(u32, u8)> = Vec::new();
            // First: include all currently-domained rows (exact match)
            // with side flag = MAX.
            let mut in_domain = std::collections::HashSet::new();
            for &r_id in &domain_snapshot {
                admitted.push((r_id, u8::MAX));
                in_domain.insert(r_id);
            }
            // Second: scan all rows for the relaxed-match candidates.
            for r_id in 0..n_rows {
                if in_domain.contains(&r_id) { continue; }
                let row = self.rows[r_id as usize];
                if !row.valid { continue; }
                if self.used[usize::from(row.piece_id)] { continue; }
                if !self.row_matches_border_pattern(pos, &row.edges) { continue; }
                let m = self.count_placed_neighbor_mismatches(pos, &row.edges);
                if m == 1 {
                    // Find the single mismatched side.
                    let (x, y) = self.puzzle.xy(pos);
                    let w = self.puzzle.width;
                    let h_ = self.puzzle.height;
                    let nbs: [(Option<Position>, usize, usize); 4] = [
                        (if y > 0 { Some((y - 1) * w + x) } else { None }, 0, 2),
                        (if x + 1 < w { Some(y * w + (x + 1)) } else { None }, 1, 3),
                        (if y + 1 < h_ { Some((y + 1) * w + x) } else { None }, 2, 0),
                        (if x > 0 { Some(y * w + (x - 1)) } else { None }, 3, 1),
                    ];
                    for (nb_opt, our_side, their_side) in nbs.iter() {
                        let Some(nb) = nb_opt else { continue; };
                        if let Some(nb_row_id) = self.placed[*nb as usize] {
                            let nb_edges = self.rows[nb_row_id as usize].edges;
                            if row.edges[*our_side] != nb_edges[*their_side] {
                                admitted.push((r_id, *our_side as u8));
                                break;
                            }
                        }
                    }
                }
            }
            admitted
        } else {
            domain_snapshot.iter().map(|&r| (r, u8::MAX)).collect()
        };

        for &(row_id, mismatch_side) in &break_candidates {
            let row = self.rows[row_id as usize];
            let piece_idx = usize::from(row.piece_id);
            if self.used[piece_idx] { continue; }
            self.emit(sink, depth, EventBody::ValueTried {
                position: pos,
                piece_id: row.piece_id,
                rotation: Rotation::from_u8(row.rotation).unwrap(),
            });
            self.stats.nodes += 1;
            let saved_bits = self.snapshot_bits(pos as usize);
            // Clear domain[pos] so place_and_propagate's prunes don't see
            // alternatives to the chosen row_id. At break depth, the
            // chosen row may not be currently in domain[pos]; we still
            // clear and let place_and_propagate set up the propagation
            // from the placed row's edges.
            let base = (pos as usize) * self.words_per_pos;
            for w in &mut self.domain_bits[base..base + self.words_per_pos] { *w = 0; }
 // domain[pos] changed; ac3_count[pos] is stale.
            self.mark_ac3_dirty(pos as usize);
            let skip_side = if mismatch_side == u8::MAX { None } else { Some(mismatch_side as usize) };
            let outcome = self.place_and_propagate_opts(sink, depth, pos, row_id, skip_side);
            match outcome {
                PropagationOutcome::Ok { undo } => {
                    match self.recurse(sink, depth + 1, solutions) {
                        RecurseResult::Found => {
                            if matches!(self.opts.mode, SolveMode::FirstSolution) {
                                return RecurseResult::Found;
                            }
                            if self.opts.max_solutions > 0
                                && solutions.len() as u32 >= self.opts.max_solutions
                            {
                                return RecurseResult::Found;
                            }
                        }
                        terminal @ (RecurseResult::TimedOut | RecurseResult::Cancelled) => {
                            self.restore(undo);
                            self.undo_place(pos, row_id);
                            self.restore_bits(pos as usize, &saved_bits);
                            return terminal;
                        }
                        RecurseResult::Exhausted => {}
                    }
                    self.restore(undo);
                }
                PropagationOutcome::Wipeout { undo } => {
                    self.restore(undo);
                    self.stats.backtracks += 1;
 // instrumentation (transient): count backtracks
                    // by the cell whose value-choice failed (pos at this
                    // depth). Inspect `self.pos_backtracks` from a sink.
                    if (pos as usize) < self.pos_backtracks.len() {
                        self.pos_backtracks[pos as usize] += 1;
                    }
                    self.emit(sink, depth, EventBody::Backtrack {
                        from_depth: depth + 1,
                        to_depth: depth,
                        cause: BacktrackCause::DomainWipeout,
                    });
                }
            }
            self.undo_place(pos, row_id);
            self.restore_bits(pos as usize, &saved_bits);
        }
        RecurseResult::Exhausted
    }

    /// Reverse the bookkeeping side-effects of a placement (NOT the
    /// domain pruning — that's `restore(undo)`'s job). Must be called
    /// while `placed[pos]` still references the row, before
    /// `domains[pos]` is rebuilt.
    pub(crate) fn undo_place(&mut self, pos: Position, row_id: u32) {
        let row = self.rows[row_id as usize];
        if self.gacolor.is_some() {
            let n_info = self.neighbor_info(pos);
            if let Some(gc) = self.gacolor.as_mut() {
                gc.apply_unplace(&row.edges, &n_info);
            }
        }
 // symmetric decrement of decided_edges / matched_count.
        // MUST run before `placed[pos] = None` so the neighbour scan can
        // still see this cell's placement (won't matter; we only check
        // the neighbour) — actually only the NEIGHBOUR's placement state
        // matters here, so order vs the line below is irrelevant. Kept
        // before for symmetry with place_and_propagate_opts.
        {
            let (px, py) = self.puzzle.xy(pos);
            let pw = self.puzzle.width;
            let ph = self.puzzle.height;
            let nbs: [(Option<Position>, usize, usize); 4] = [
                (if py > 0 { Some((py - 1) * pw + px) } else { None }, 0, 2),
                (if px + 1 < pw { Some(py * pw + (px + 1)) } else { None }, 1, 3),
                (if py + 1 < ph { Some((py + 1) * pw + px) } else { None }, 2, 0),
                (if px > 0 { Some(py * pw + (px - 1)) } else { None }, 3, 1),
            ];
            for (nb_opt, our_side, their_side) in nbs {
                let Some(nb) = nb_opt else { continue; };
                let Some(nb_row_id) = self.placed[nb as usize] else { continue; };
                self.decided_edges = self.decided_edges.saturating_sub(1);
                let nb_edges = self.rows[nb_row_id as usize].edges;
                let our = row.edges[our_side];
                if our != 0 && our == nb_edges[their_side] {
                    self.matched_count = self.matched_count.saturating_sub(1);
                }
            }
        }
        self.placed[pos as usize] = None;
        let piece_idx = usize::from(row.piece_id);
        self.used[piece_idx] = false;
 // decrement Blackwood heuristic-count.
        let dec = self.count_heuristic_in_row(&row.edges);
        self.placed_heuristic_count = self.placed_heuristic_count.saturating_sub(dec);
    }

 /// count edges in `edges` whose color is in the active
    /// `heuristic_color_mask`. Returns 0 when no schedule is attached.
    #[inline]
    fn count_heuristic_in_row(&self, edges: &[Color; 4]) -> u32 {
        if self.heuristic_color_mask.is_empty() {
            return 0;
        }
        let m = &self.heuristic_color_mask;
        let mut n = 0u32;
        for &c in edges {
            let ci = c as usize;
            if ci < m.len() && m[ci] {
                n += 1;
            }
        }
        n
    }

 /// count edges of a placed row `cand` that disagree with
    /// already-placed neighbours of `pos`. Returns u32::MAX if `cand`
    /// would violate piece-uniqueness (piece already used).
    fn count_placed_neighbor_mismatches(&self, pos: Position, cand_edges: &[Color; 4]) -> u32 {
        let (x, y) = self.puzzle.xy(pos);
        let w = self.puzzle.width;
        let h = self.puzzle.height;
        let mut n = 0u32;
        // (neighbor_pos, our_side, their_side)
        let nbs: [(Option<Position>, usize, usize); 4] = [
            (if y > 0 { Some((y - 1) * w + x) } else { None }, 0, 2),
            (if x + 1 < w { Some(y * w + (x + 1)) } else { None }, 1, 3),
            (if y + 1 < h { Some((y + 1) * w + x) } else { None }, 2, 0),
            (if x > 0 { Some(y * w + (x - 1)) } else { None }, 3, 1),
        ];
        for (nb_opt, our_side, their_side) in nbs.iter() {
            let Some(nb) = nb_opt else { continue; };
            if let Some(nb_row_id) = self.placed[*nb as usize] {
                let nb_edges = self.rows[nb_row_id as usize].edges;
                if cand_edges[*our_side] != nb_edges[*their_side] {
                    n += 1;
                }
            }
        }
        n
    }

 /// does this row also match the cell's border_mask
    /// pattern? Used to admit fresh candidates at break depth (where
    /// the cell's domain may have been pruned away by earlier
    /// neighbour-prunes).
    fn row_matches_border_pattern(&self, pos: Position, edges: &[Color; 4]) -> bool {
        let mask = self.puzzle.border_mask(pos);
        let [on_top, on_right, on_bot, on_left] = mask;
        on_top == (edges[0] == BORDER)
            && on_right == (edges[1] == BORDER)
            && on_bot == (edges[2] == BORDER)
            && on_left == (edges[3] == BORDER)
    }

 /// is the cell at `pos` a Blackwood break cell? Tests
    /// `is_break_index[scan_index_of_cell[pos]]`. Returns false when
    /// no schedule is attached or scan_order isn't set.
    #[inline]
    fn is_pos_break_index(&self, pos: Position) -> bool {
        if self.is_break_index.is_empty() || self.scan_index_of_cell.is_empty() {
            return false;
        }
        let scan_idx = self.scan_index_of_cell[pos as usize] as usize;
        scan_idx < self.is_break_index.len() && self.is_break_index[scan_idx]
    }

 /// call the in-process ONNX learned scorer to rank each
    /// candidate row at the given position. Returns `None` when the
    /// model is unavailable or the puzzle size doesn't match a v1
    /// model; the caller then preserves the existing order. Dispatches
    /// to v1 (fixed-size) or v2 (size-agnostic) based on the loaded
    /// scorer.
    #[cfg(all(not(target_arch = "wasm32"), feature = "learned-order"))]
    fn learned_score_candidates(
        &mut self,
        pos: Position,
        domain_snapshot: &[u32],
    ) -> Option<Vec<f32>> {
        use ndarray::{Array2, Array3};

        if self.learned_bridge_disabled {
            return None;
        }
        if self.learned_bridge.is_none() {
            match crate::bridge::LearnedScorer::spawn() {
                Some(s) => self.learned_bridge = Some(s),
                None => {
                    self.learned_bridge_disabled = true;
                    return None;
                }
            }
        }

        let w = self.puzzle.width as usize;
        let h = self.puzzle.height as usize;
        let n_cells = w * h;
        let is_v2 = self.learned_bridge.as_ref().map(|s| s.is_v2()).unwrap_or(false);

        // V1: refuse non-square or wrong-size puzzles up-front.
        if !is_v2 {
            if let Some(s) = self.learned_bridge.as_ref() {
                if s.grid_size() != Some(w) || w != h {
                    self.learned_bridge_disabled = true;
                    return None;
                }
            }
        }

        // Build the 13-D per-cell features. Same layout for v1 and v2.
        let mut feats = Array3::<f32>::zeros((1, n_cells, 13));
        for y in 0..h {
            for x in 0..w {
                let p = y * w + x;
                let (e0, e1, e2, e3, placed) = match self.placed[p] {
                    Some(r_id) => {
                        let r = self.rows[r_id as usize];
                        (r.edges[0] as f32, r.edges[1] as f32, r.edges[2] as f32, r.edges[3] as f32, 1.0_f32)
                    }
                    None => (0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32),
                };
                let nb_top = if y == 0 { 1.0_f32 } else if self.placed[(y - 1) * w + x].is_some() { 1.0 } else { 0.0 };
                let nb_right = if x == w - 1 { 1.0 } else if self.placed[y * w + x + 1].is_some() { 1.0 } else { 0.0 };
                let nb_bot = if y == h - 1 { 1.0 } else if self.placed[(y + 1) * w + x].is_some() { 1.0 } else { 0.0 };
                let nb_left = if x == 0 { 1.0 } else if self.placed[y * w + x - 1].is_some() { 1.0 } else { 0.0 };
                let bm_top = if y == 0 { 1.0_f32 } else { 0.0 };
                let bm_right = if x == w - 1 { 1.0 } else { 0.0 };
                let bm_bot = if y == h - 1 { 1.0 } else { 0.0 };
                let bm_left = if x == 0 { 1.0 } else { 0.0 };
                feats[[0, p, 0]] = e0;
                feats[[0, p, 1]] = e1;
                feats[[0, p, 2]] = e2;
                feats[[0, p, 3]] = e3;
                feats[[0, p, 4]] = nb_top;
                feats[[0, p, 5]] = nb_right;
                feats[[0, p, 6]] = nb_bot;
                feats[[0, p, 7]] = nb_left;
                feats[[0, p, 8]] = placed;
                feats[[0, p, 9]] = bm_top;
                feats[[0, p, 10]] = bm_right;
                feats[[0, p, 11]] = bm_bot;
                feats[[0, p, 12]] = bm_left;
            }
        }

        if is_v2 {
            // Build nb_idx (n_cells, 4) with off-board = n_cells sentinel.
            let mut nb_idx = Array2::<i64>::zeros((n_cells, 4));
            let mut valid_mask = Array2::<f32>::zeros((n_cells, 4));
            let pad = n_cells as i64;
            for y in 0..h {
                for x in 0..w {
                    let p = y * w + x;
                    if y > 0 { nb_idx[[p, 0]] = ((y - 1) * w + x) as i64; valid_mask[[p, 0]] = 1.0; } else { nb_idx[[p, 0]] = pad; }
                    if x < w - 1 { nb_idx[[p, 1]] = (y * w + x + 1) as i64; valid_mask[[p, 1]] = 1.0; } else { nb_idx[[p, 1]] = pad; }
                    if y < h - 1 { nb_idx[[p, 2]] = ((y + 1) * w + x) as i64; valid_mask[[p, 2]] = 1.0; } else { nb_idx[[p, 2]] = pad; }
                    if x > 0 { nb_idx[[p, 3]] = (y * w + x - 1) as i64; valid_mask[[p, 3]] = 1.0; } else { nb_idx[[p, 3]] = pad; }
                }
            }
            // Build cand_edges (1, C, 4) from the domain_snapshot rows
            // ROTATED according to row.rotation — the candidate's edges
            // as they would appear placed on the board.
            let c = domain_snapshot.len();
            let mut cand = Array3::<i64>::zeros((1, c, 4));
            for (i, &r_id) in domain_snapshot.iter().enumerate() {
                let r = self.rows[r_id as usize];
                for k in 0..4 {
                    cand[[0, i, k]] = i64::from(r.edges[k]);
                }
            }
            let scorer = self.learned_bridge.as_ref()?;
            match scorer.score_v2(&feats, &nb_idx, &valid_mask, pos, &cand) {
                Some(scores) if scores.len() == c => Some(scores),
                _ => {
                    self.learned_bridge = None;
                    self.learned_bridge_disabled = true;
                    None
                }
            }
        } else {
            // V1 path
            let candidates: Vec<(u16, u8)> = domain_snapshot
                .iter()
                .map(|&r_id| {
                    let r = self.rows[r_id as usize];
                    (r.piece_id, r.rotation)
                })
                .collect();
            let scorer = self.learned_bridge.as_ref()?;
            match scorer.score(&feats, pos, &candidates) {
                Some(scores) if scores.len() == domain_snapshot.len() => Some(scores),
                _ => {
                    self.learned_bridge = None;
                    self.learned_bridge_disabled = true;
                    None
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RecurseResult { Found, Exhausted, TimedOut, Cancelled }

/// Single undo entry: a bit-diff to OR back into `domain_bits[pos]` on
/// restore. The actual bit-diff lives in `SearchState::undo_words_arena`,
/// in the slot `[words_start..words_start + words_per_pos]`. Recurse-stack
/// discipline guarantees arena slots are popped LIFO with the entries
/// themselves, so no entry outlives its slot. ( Cat-4 perf
/// cleanup: dropped per-entry `Vec<u64>` allocation, was ~17.5% of CPU
/// in System::alloc_zeroed.)
#[derive(Clone, Copy)]
pub(crate) struct UndoEntry {
    pub pos: Position,
    pub words_start: u32,
}

pub(crate) enum PropagationOutcome {
    Ok { undo: Vec<UndoEntry> },
    Wipeout { undo: Vec<UndoEntry> },
}

pub(crate) enum Ac3Outcome {
    Ok { removed: Vec<UndoEntry> },
    Wipeout { removed: Vec<UndoEntry> },
}

enum PruneResult {
    Ok,
    Removed(UndoEntry),
    Wipeout { entry: UndoEntry },
}

#[cfg(test)]
mod tests {
    use super::*;
    use eternity2_core::{Edges, Piece};

    fn p(id: PieceId, t: Color, r: Color, b: Color, l: Color) -> Piece {
        Piece::new(id, Edges::new(t, r, b, l))
    }

    #[test]
    fn domain_bits_well_formed_on_construction() {
        // Every cell's domain must contain at least one row (otherwise
        // the puzzle is trivially unsat at construction). Total set
        // bits across all cells should equal the sum of expected domain
        // sizes by cell class.
        use eternity2_generator::{generate, GeneratorConfig};
        let puzzle = generate(GeneratorConfig { size: 5, interior_colors: 5, seed: 7 }).unwrap();
        let solver = EngineSolver::gacolor_ac3();
        let opts = SolveOpts::default();
        let state = SearchState::new(&puzzle, &solver, &opts);
        for pos in 0..puzzle.cell_count() as usize {
            let base = pos * state.words_per_pos;
            let popcount: u32 = state.domain_bits[base..base + state.words_per_pos]
                .iter().map(|w| w.count_ones()).sum();
            assert!(popcount > 0,
                "domain[{}] is empty at construction (popcount=0)", pos);
        }
    }

    #[test]
    fn domain_bits_construct_2x2_solves() {
        // 2×2 puzzle with 4 corner pieces. Validate that the solve still
        // works end-to-end with the bitset-only rep (no regressions vs
        // solves_2x2_trivial above, but exercised via the full pipeline
        // including SearchState::new → recurse → restore).
        let pieces = vec![
            p(0, 0, 1, 1, 0), p(1, 0, 0, 1, 1),
            p(2, 1, 1, 0, 0), p(3, 1, 0, 0, 1),
        ];
        let puzzle = Puzzle::new(2, 2, 2, pieces).unwrap();
        let mut s = EngineSolver::gacolor_ac3_ns1();
        let mut sink = eternity2_events::BufferSink::new();
        assert!(matches!(s.solve(&puzzle, &SolveOpts::default(), &mut sink),
            SolveOutcome::Solved(_)));
    }

    #[test]
    fn solves_2x2_trivial() {
        let pieces = vec![
            p(0, 0, 1, 1, 0),
            p(1, 0, 0, 1, 1),
            p(2, 1, 1, 0, 0),
            p(3, 1, 0, 0, 1),
        ];
        let puzzle = Puzzle::new(2, 2, 2, pieces).unwrap();
        let mut s = EngineSolver::border_first_lcv();
        let mut sink = eternity2_events::BufferSink::new();
        assert!(matches!(s.solve(&puzzle, &SolveOpts::default(), &mut sink), SolveOutcome::Solved(_)));
    }

 /// MaxScore must still find the unique perfect solution
    /// on a generator board where every internal edge matches by
    /// construction. Also acts as a smoke test that incremental
    /// `matched_count` / `decided_edges` accounting + the upper-bound
    /// prune don't break correctness.
    #[test]
    fn max_score_solves_generator_5x5() {
        use eternity2_generator::{generate, GeneratorConfig};
        let puzzle = generate(GeneratorConfig { size: 5, interior_colors: 5, seed: 11 }).unwrap();
        let mut s = EngineSolver::border_first_lcv();
        let mut sink = eternity2_events::BufferSink::new();
        let mut opts = SolveOpts::default();
        opts.time_budget_ms = 60_000;
        opts.objective = Some(Objective::MaxScore);
        let outcome = s.solve(&puzzle, &opts, &mut sink);
        let board = match outcome {
            SolveOutcome::Solved(b) => b,
            other => panic!("MaxScore got {other:?}, want Solved(_)"),
        };
        // Generator puzzles have a perfect solution; the returned
        // board's score must equal the total internal edge count.
        let w = puzzle.width;
        let h = puzzle.height;
        let total = (w - 1) * h + w * (h - 1);
        let mut matched = 0u32;
        for y in 0..h {
            for x in 0..w {
                let pos = y * w + x;
                let Some((pid, rot)) = board.get(pos) else { continue; };
                let edges = puzzle.piece(pid).unwrap().edges.rotated(rot).as_array();
                if x + 1 < w {
                    if let Some((rpid, rrot)) = board.get(y * w + (x + 1)) {
                        let re = puzzle.piece(rpid).unwrap().edges.rotated(rrot).as_array();
                        if edges[1] != 0 && edges[1] == re[3] { matched += 1; }
                    }
                }
                if y + 1 < h {
                    if let Some((bpid, brot)) = board.get((y + 1) * w + x) {
                        let be = puzzle.piece(bpid).unwrap().edges.rotated(brot).as_array();
                        if edges[2] != 0 && edges[2] == be[0] { matched += 1; }
                    }
                }
            }
        }
        assert_eq!(matched, total, "MaxScore should reach the perfect 5×5 solution");
    }

    #[test]
    fn solves_5x5_from_generator() {
        use eternity2_generator::{generate, GeneratorConfig};
        let puzzle = generate(GeneratorConfig { size: 5, interior_colors: 5, seed: 11 }).unwrap();
        for mut s in [
            EngineSolver::border_first_lcv(),
            EngineSolver::rare_color_first(),
            EngineSolver::border_first_random(),
        ] {
            let mut sink = eternity2_events::BufferSink::new();
            let mut opts = SolveOpts::default();
            opts.time_budget_ms = 60_000;
            let outcome = s.solve(&puzzle, &opts, &mut sink);
            let profile = s.heuristic_profile().0;
            assert!(matches!(outcome, SolveOutcome::Solved(_)), "{profile} got {outcome:?}");
        }
    }

    #[test]
    #[ignore = "performance comparison; run with --ignored"]
    fn parity_propagator_node_count_comparison_6x6() {
        // Propagators are pruning rules, but search order can be
        // sensitive to *which* state we backtrack from — a stronger
        // propagator occasionally forces an alternative path that
        // happens to take more nodes on a given seed. The honest
        // assertion is on averages, not per-seed monotonicity.
        use eternity2_events::{BufferSink, EventBody};
        use eternity2_generator::{generate, GeneratorConfig};
        let seeds = [3u64, 7, 13, 21, 42];
        let mut sum_a = 0u64;
        let mut sum_b = 0u64;
        let mut sum_c = 0u64;
        for seed in seeds {
            let puzzle = generate(GeneratorConfig { size: 6, interior_colors: 5, seed }).unwrap();
            let nodes = |s: &mut EngineSolver, puzzle: &Puzzle| -> u64 {
                let mut sink = BufferSink::new();
                let mut opts = SolveOpts::default();
                opts.time_budget_ms = 60_000;
                s.solve(puzzle, &opts, &mut sink);
                sink.events.iter().filter(|e| matches!(e.body, EventBody::ValueTried { .. })).count() as u64
            };
            let n_a = nodes(&mut EngineSolver::border_first_lcv(), &puzzle);
            let n_b = nodes(&mut EngineSolver::border_first_parity(), &puzzle);
            let n_c = nodes(&mut EngineSolver::border_first_full(), &puzzle);
            eprintln!("seed={seed} baseline={n_a} parity={n_b} full={n_c}");
            sum_a += n_a;
            sum_b += n_b;
            sum_c += n_c;
        }
        eprintln!("totals: baseline={sum_a} parity={sum_b} full={sum_c}");
        // On average across seeds, the stronger propagator set should
        // not be dramatically worse. Use a 50% headroom assertion to
        // catch real regressions while accepting per-seed variance.
        assert!(sum_b <= sum_a + sum_a / 2,
            "parity sum {sum_b} unexpectedly larger than baseline {sum_a}");
        assert!(sum_c <= sum_a + sum_a / 2,
            "full sum {sum_c} unexpectedly larger than baseline {sum_a}");
    }

    #[test]
    fn parity_propagator_reduces_or_equals_nodes() {
        // Compare node count: with parity propagator on, the engine
        // should never explore more nodes than without (parity is a
        // strict superset of edge_color pruning; in practice it should
        // explore fewer nodes on hard-ish puzzles, but the loose
        // assertion ("not more") is the safe contract).
        use eternity2_events::{BufferSink, EventBody};
        use eternity2_generator::{generate, GeneratorConfig};
        let puzzle = generate(GeneratorConfig { size: 5, interior_colors: 5, seed: 11 }).unwrap();

        fn nodes(sink: &BufferSink) -> u64 {
            sink.events.iter().filter(|e| matches!(e.body, EventBody::ValueTried { .. })).count() as u64
        }

        let mut baseline = EngineSolver::border_first_lcv();
        let mut sink_a = BufferSink::new();
        baseline.solve(&puzzle, &SolveOpts::default(), &mut sink_a);

        let mut with_parity = EngineSolver::border_first_parity();
        let mut sink_b = BufferSink::new();
        with_parity.solve(&puzzle, &SolveOpts::default(), &mut sink_b);

        let n_a = nodes(&sink_a);
        let n_b = nodes(&sink_b);
        eprintln!("baseline nodes: {n_a}, with parity: {n_b}");
        // Parity should be at least as strong as baseline (never more nodes).
        // Strict reduction depends on the puzzle — for this seed it should
        // be equal-or-less.
        assert!(n_b <= n_a, "parity should not increase node count; baseline={n_a} parity={n_b}");
    }

    #[test]
    fn propagator_enabled_profiles_solve_5x5() {
        use eternity2_generator::{generate, GeneratorConfig};
        let puzzle = generate(GeneratorConfig { size: 5, interior_colors: 5, seed: 11 }).unwrap();
        for mut s in [
            EngineSolver::border_first_parity(),
            EngineSolver::border_first_full(),
        ] {
            let mut sink = eternity2_events::BufferSink::new();
            let mut opts = SolveOpts::default();
            opts.time_budget_ms = 60_000;
            let outcome = s.solve(&puzzle, &opts, &mut sink);
            let profile = s.heuristic_profile().0;
            assert!(matches!(outcome, SolveOutcome::Solved(_)),
                "{profile} got {outcome:?}");
        }
    }

    #[test]
    #[ignore = "wall-time scaling check; run with --ignored"]
    fn root_split_scales_on_6x6() {
        use std::time::Instant;
        use eternity2_generator::{generate, GeneratorConfig};
        let puzzle = generate(GeneratorConfig { size: 6, interior_colors: 5, seed: 3 }).unwrap();
        let measure = |mut s: EngineSolver| -> u128 {
            let mut sink = eternity2_events::BufferSink::new();
            let mut opts = SolveOpts::default();
            opts.time_budget_ms = 120_000;
            let t = Instant::now();
            s.solve(&puzzle, &opts, &mut sink);
            t.elapsed().as_millis()
        };
        let seq_ms = measure(EngineSolver::border_first_lcv());
        let par_ms = measure(EngineSolver::border_first_lcv_par());
        eprintln!("seq: {seq_ms} ms, par: {par_ms} ms (threads={})",
                  rayon::current_num_threads());
    }

    #[test]
    fn root_split_solves_4x4_and_5x5() {
        use eternity2_generator::{generate, GeneratorConfig};
        for size in [4u32, 5] {
            let puzzle = generate(GeneratorConfig { size, interior_colors: size, seed: 17 }).unwrap();
            let mut s = EngineSolver::border_first_lcv_par();
            let mut sink = eternity2_events::BufferSink::new();
            let mut opts = SolveOpts::default();
            opts.time_budget_ms = 60_000;
            let outcome = s.solve(&puzzle, &opts, &mut sink);
            assert!(matches!(outcome, SolveOutcome::Solved(_)),
                "{size}x{size} parallel got {outcome:?}");
        }
    }

    #[test]
    fn root_split_emits_one_started_event() {
        use eternity2_events::EventBody;
        use eternity2_generator::{generate, GeneratorConfig};
        let puzzle = generate(GeneratorConfig { size: 4, interior_colors: 4, seed: 3 }).unwrap();
        let mut s = EngineSolver::border_first_lcv_par();
        let mut sink = eternity2_events::BufferSink::new();
        s.solve(&puzzle, &SolveOpts::default(), &mut sink);
        let started_count = sink.events.iter()
            .filter(|e| matches!(e.body, EventBody::Started { .. }))
            .count();
        assert_eq!(started_count, 1, "parallel run must emit exactly one Started");
    }

    #[test]
    fn engine_id_and_profile_in_started() {
        use eternity2_events::EventBody;
        use eternity2_generator::{generate, GeneratorConfig};
        let puzzle = generate(GeneratorConfig { size: 3, interior_colors: 3, seed: 1 }).unwrap();
        let mut s = EngineSolver::border_first_lcv();
        let mut sink = eternity2_events::BufferSink::new();
        s.solve(&puzzle, &SolveOpts::default(), &mut sink);
        let (sid, prof) = sink.events.iter().find_map(|e| match &e.body {
            EventBody::Started { solver_id, heuristic_profile, .. } => Some((solver_id.clone(), heuristic_profile.clone())),
            _ => None,
        }).unwrap();
        assert_eq!(sid, "engine");
        assert_eq!(prof, "border_first_lcv");
    }

    #[test]
    fn build_hint_rectangle_path_on_canonical_e2_hints() {
        // The 5 canonical E2 hints at (2,2), (13,2), (2,13), (13,13),
        // (7,8). Expected path: rectangle perimeter (44 cells) + spoke
        // (cells from x=2..=7 at y=8, minus the (2,8) already on left
        // col) = 49 unique cells total.
        use eternity2_generator::{generate, GeneratorConfig};
        use eternity2_core::{Hint, Hints, Rotation};
        let puzzle = generate(GeneratorConfig { size: 16, interior_colors: 22, seed: 1 }).unwrap();
        let r = Rotation::from_u8(0).unwrap();
        let pid = eternity2_core::PieceId::try_from(0u32).unwrap();
        let hints = Hints { hints: vec![
            Hint { position: 2 + 2*16, piece_id: pid, rotation: r },   // (2,2)
            Hint { position: 13 + 2*16, piece_id: pid, rotation: r },  // (13,2)
            Hint { position: 2 + 13*16, piece_id: pid, rotation: r },  // (2,13)
            Hint { position: 13 + 13*16, piece_id: pid, rotation: r }, // (13,13)
            Hint { position: 7 + 8*16, piece_id: pid, rotation: r },   // (7,8)
        ]};
        let path = build_hint_rectangle_path(&puzzle, &hints);
        // Expected length: top row 12 + right col 11 + bottom row 11
        // + left col 10 + spoke 5 (cells (3..=7, 8); (2,8) is already
        // on left col) = 49.
        assert_eq!(path.len(), 49, "expected 49 cells, got {}", path.len());
        // First cell must be (2,2).
        assert_eq!(path[0], 2 + 2*16);
        // No duplicates.
        let set: std::collections::HashSet<_> = path.iter().collect();
        assert_eq!(set.len(), path.len(), "path has duplicates");
    }

    #[test]
    fn build_hint_rectangle_layered_path_covers_all_cells() {
        use eternity2_generator::{generate, GeneratorConfig};
        use eternity2_core::{Hint, Hints, Rotation};
        let puzzle = generate(GeneratorConfig { size: 16, interior_colors: 22, seed: 1 }).unwrap();
        let r = Rotation::from_u8(0).unwrap();
        let pid = eternity2_core::PieceId::try_from(0u32).unwrap();
        let hints = Hints { hints: vec![
            Hint { position: 2 + 2*16, piece_id: pid, rotation: r },
            Hint { position: 13 + 2*16, piece_id: pid, rotation: r },
            Hint { position: 2 + 13*16, piece_id: pid, rotation: r },
            Hint { position: 13 + 13*16, piece_id: pid, rotation: r },
            Hint { position: 7 + 8*16, piece_id: pid, rotation: r },
        ]};
        let path = build_hint_rectangle_layered_path(&puzzle, &hints);
        assert_eq!(path.len(), 256, "expected 256 cells, got {}", path.len());
        // No duplicates
        let set: std::collections::HashSet<_> = path.iter().collect();
        assert_eq!(set.len(), 256, "duplicates in path");
        // First 49 must match the basic rectangle path
        let rect = build_hint_rectangle_path(&puzzle, &hints);
        assert_eq!(&path[..49], &rect[..]);
    }

    #[test]
    fn build_x_skeleton_path_covers_all_cells() {
        use eternity2_generator::{generate, GeneratorConfig};
        use eternity2_core::{Hint, Hints, Rotation};
        let puzzle = generate(GeneratorConfig { size: 16, interior_colors: 22, seed: 1 }).unwrap();
        let r = Rotation::from_u8(0).unwrap();
        let pid = eternity2_core::PieceId::try_from(0u32).unwrap();
        let canonical = [
            2 + 2 * 16u32,    // TL
            13 + 2 * 16u32,   // TR
            2 + 13 * 16u32,   // BL
            13 + 13 * 16u32,  // BR
            7 + 8 * 16u32,    // centre
        ];
        let hints = Hints { hints: canonical.iter().map(|&p| Hint {
            position: p, piece_id: pid, rotation: r,
        }).collect() };
        let path = build_x_skeleton_path(&puzzle, &hints);
        assert_eq!(path.len(), 256, "expected 256 cells, got {}", path.len());
        let set: std::collections::HashSet<_> = path.iter().collect();
        assert_eq!(set.len(), 256, "duplicates in path");
        // First cell must be the TL hint.
        assert_eq!(path[0], 2 + 2 * 16);
        // All 5 canonical hint positions must appear within the X-skeleton
        // prefix. Two 3-wide diagonals of length ~6 across canonical E2
        // give ~50-60 cells in the prefix; require all hints in first 90.
        let prefix: std::collections::HashSet<u32> = path[..90].iter().copied().collect();
        for &hp in &canonical {
            assert!(prefix.contains(&hp), "hint {hp} not in X-skeleton prefix");
        }
    }

    #[test]
    fn build_x_skeleton_path_too_few_hints() {
        use eternity2_generator::{generate, GeneratorConfig};
        let puzzle = generate(GeneratorConfig { size: 16, interior_colors: 22, seed: 1 }).unwrap();
        for n in 0..=4 {
            let hints = eternity2_core::Hints {
                hints: (0..n).map(|i| eternity2_core::Hint {
                    position: i,
                    piece_id: eternity2_core::PieceId::try_from(0u32).unwrap(),
                    rotation: eternity2_core::Rotation::from_u8(0).unwrap(),
                }).collect()
            };
            assert!(build_x_skeleton_path(&puzzle, &hints).is_empty(),
                "expected empty with {} hints", n);
        }
    }

    #[test]
    fn build_hint_rectangle_path_too_few_hints() {
        use eternity2_generator::{generate, GeneratorConfig};
        let puzzle = generate(GeneratorConfig { size: 16, interior_colors: 22, seed: 1 }).unwrap();
        // 0 hints, 3 hints — both should return empty.
        for n in 0..=3 {
            let hints = eternity2_core::Hints {
                hints: (0..n).map(|i| eternity2_core::Hint {
                    position: i,
                    piece_id: eternity2_core::PieceId::try_from(0u32).unwrap(),
                    rotation: eternity2_core::Rotation::from_u8(0).unwrap(),
                }).collect()
            };
            assert!(build_hint_rectangle_path(&puzzle, &hints).is_empty(),
                "expected empty with {} hints", n);
        }
    }

    #[test]
    fn cell_side_edge_counts_match_canonical_e2() {
        // The Python BP enumeration on the canonical 16×16 grid produces
        // 2*W*H + W + H = 544 distinct edges (64 boundary + 480 internal).
        // Use a generated 16×16 puzzle (any colors) — the mapping only
        // depends on grid geometry.
        use eternity2_generator::{generate, GeneratorConfig};
        let puzzle = generate(GeneratorConfig { size: 16, interior_colors: 22, seed: 1 }).unwrap();
        let cse = build_cell_side_edge(&puzzle);
        let n_pos = (puzzle.width * puzzle.height) as usize;
        assert_eq!(cse.len(), n_pos * 4);
        let max_eid = cse.iter().copied().max().unwrap();
        let n_edges = max_eid as usize + 1;
        assert_eq!(n_edges, 2 * 16 * 16 + 16 + 16, "expected 544 edges, got {n_edges}");
        // Internal-edge sharing: each non-corner side is referenced by 2
        // (pos, side) entries; boundary sides by 1. Count.
        let mut refs = vec![0u32; n_edges];
        for &e in &cse { refs[e as usize] += 1; }
        let boundary = refs.iter().filter(|&&r| r == 1).count();
        let internal = refs.iter().filter(|&&r| r == 2).count();
        assert_eq!(boundary, 2 * (16 + 16));
        assert_eq!(internal, 2 * 16 * 16 - 16 - 16);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn load_edge_bp_marginals_smoketest_v12_file() {
 // The 284 KB measurement is in the repo. If running under
        // a sandbox without it, skip cleanly.
        let path = std::path::Path::new("../../output/v12_bp/edge_bp_60i.json");
        if !path.exists() { return; }
        let m = load_edge_bp_marginals(path).expect("load");
        assert_eq!(m.len(), 544 * EDGE_BP_NSTATE);
        // Each edge's marginal must sum to ≈ 1.0.
        for e in 0..544 {
            let s: f32 = (0..EDGE_BP_NSTATE).map(|c| m[e * EDGE_BP_NSTATE + c]).sum();
            assert!((s - 1.0).abs() < 1e-3, "edge {e} sum = {s}");
        }
        // Boundary edges (those with refs=1 in cse) should have
        // argmax = 0 (BORDER). Spot-check edge 0 which is cell-0 north.
        let argmax_0 = (0..EDGE_BP_NSTATE)
            .max_by(|a, b| m[*a].partial_cmp(&m[*b]).unwrap())
            .unwrap();
        assert_eq!(argmax_0, 0, "boundary edge 0 should argmax to BORDER");
    }

 // ===== Blackwood tests =====

    #[test]
    fn blackwood_schedule_validate_catches_depth_cliff() {
        // Two control points at the same depth = cliff = exactly
 // the first-iteration bug. validate must reject.
        let bad = BlackwoodSchedule {
            heuristic_sides: vec![1],
            exhaustion_targets: vec![(0, 0), (60, 0), (60, 34), (100, 80)],
            heuristic_pool_size: 100,
            max_heuristic_index: 100,
            break_indexes_allowed: vec![],
        };
        assert!(bad.validate().is_err(), "duplicate-depth schedule must fail validate()");
    }

    #[test]
    fn blackwood_schedule_validate_catches_count_regression() {
        // Counts must be non-decreasing.
        let bad = BlackwoodSchedule {
            heuristic_sides: vec![1],
            exhaustion_targets: vec![(0, 0), (10, 50), (20, 30)],
            heuristic_pool_size: 100,
            max_heuristic_index: 100,
            break_indexes_allowed: vec![],
        };
        assert!(bad.validate().is_err(), "decreasing-count schedule must fail validate()");
    }

    #[test]
    fn blackwood_schedule_469_on_5x5_is_well_formed() {
        // Synthesise a small puzzle big enough for the affine remap
        // to give post_border < target_max. 5×5 has border_ring = 16,
        // n_pos = 25; target_max = 160 × 25 / 256 = 15. Too small —
        // post_border (17) > target_max (15). Per debug_assert this
        // would crash in dev; release-mode falls through.
        // For a 12×12: border_ring=44, n_pos=144, target_max=90,
        // post_border=45. Comfortable; schedule should validate.
        use eternity2_generator::{generate, GeneratorConfig};
        let puzzle = generate(GeneratorConfig {
            size: 12, interior_colors: 12, seed: 1,
        }).unwrap();
        let hints = eternity2_core::Hints::default();
        let s = blackwood_schedule_469(&puzzle, &hints)
            .expect("compute_heuristic_sides returned < 3 colors");
        s.validate().expect("blackwood_schedule_469 must produce a valid schedule");
        // First non-zero target lands STRICTLY past the border ring.
        let border_ring = 2 * 12 + 2 * 12 - 4;
        let first_nz = s.exhaustion_targets.iter().find(|(_, c)| *c > 0).copied();
        let (d_first, _) = first_nz.expect("schedule must have a non-zero target");
        assert!(d_first > border_ring,
            "first non-zero schedule depth {d_first} must be past border_ring {border_ring}");
    }

    #[test]
    fn blackwood_schedule_target_interp_endpoints() {
        let s = BlackwoodSchedule {
            heuristic_sides: vec![1, 2, 3],
            exhaustion_targets: vec![(0, 0), (100, 50), (200, 100)],
            heuristic_pool_size: 100,
            max_heuristic_index: 200,
            break_indexes_allowed: vec![],
        };
        assert_eq!(s.target_at(0), 0);
        assert_eq!(s.target_at(100), 50);
        assert_eq!(s.target_at(200), 100);
        // Linear interp at midpoint of first segment.
        assert_eq!(s.target_at(50), 25);
        // Midpoint of second.
        assert_eq!(s.target_at(150), 75);
        // Saturates below first and above last.
        assert_eq!(s.target_at(300), 100);
    }

    #[test]
    fn scan_order_bottom_up_indices() {
        // 4×4 puzzle, scan_order=RowMajorBottomUp:
        //   pos (x,y)=(0,3) → bu_idx 0
        //   pos (x,y)=(3,3) → bu_idx 3
        //   pos (x,y)=(0,0) → bu_idx 12
        //   pos (x,y)=(3,0) → bu_idx 15
        let pieces: Vec<Piece> = (0..16).map(|id| p(id as PieceId, 0, 0, 0, 0)).collect();
        let puzzle = Puzzle::new(4, 4, 1, pieces).unwrap();
        let mut cfg = EngineConfig::BORDER_FIRST_LCV;
        cfg.scan_order = Some(ScanOrder::RowMajorBottomUp);
        let solver = EngineSolver::new(cfg, "engine", "test_blackwood_scan");
        let opts = SolveOpts::default();
        let state = SearchState::new(&puzzle, &solver, &opts);
        // scan_index_of_cell[pos] = (H-1-y)*W + x
        let pos = |x: u32, y: u32| -> usize { (y * 4 + x) as usize };
        assert_eq!(state.scan_index_of_cell[pos(0, 3)], 0);
        assert_eq!(state.scan_index_of_cell[pos(3, 3)], 3);
        assert_eq!(state.scan_index_of_cell[pos(0, 0)], 12);
        assert_eq!(state.scan_index_of_cell[pos(3, 0)], 15);
        // path_order should follow scan order: first entry is pos(0,3)=12,
        // last is pos(3,0)=3.
        assert_eq!(state.path_order.first().copied(), Some(12u32));
        assert_eq!(state.path_order.last().copied(), Some(3u32));
        // auto_skeleton_path_k must cover all cells.
        assert_eq!(state.auto_skeleton_path_k, 16);
    }

    #[test]
    fn count_heuristic_in_row_basic() {
        // 2x2 puzzle, attach a schedule with heuristic colors {1, 3}.
        // Row (1,0,3,1) should have count = 3 (two 1s + one 3).
        let pieces = vec![
            p(0, 0, 1, 1, 0), p(1, 0, 0, 1, 1),
            p(2, 1, 1, 0, 0), p(3, 1, 0, 0, 1),
        ];
        let puzzle = Puzzle::new(2, 2, 2, pieces).unwrap();
        let schedule = Arc::new(BlackwoodSchedule {
            heuristic_sides: vec![1, 3],
            exhaustion_targets: vec![(0, 0), (4, 4)],
            heuristic_pool_size: 8,
            max_heuristic_index: 4,
            break_indexes_allowed: vec![],
        });
        let solver = EngineSolver::new(EngineConfig::BORDER_FIRST_LCV, "engine", "t")
            .with_blackwood_schedule(schedule);
        let opts = SolveOpts::default();
        let state = SearchState::new(&puzzle, &solver, &opts);
        assert_eq!(state.count_heuristic_in_row(&[1, 0, 3, 1]), 3);
        assert_eq!(state.count_heuristic_in_row(&[0, 0, 0, 0]), 0);
        assert_eq!(state.count_heuristic_in_row(&[3, 1, 3, 1]), 4);
    }

    #[test]
    fn compute_heuristic_sides_excludes_corners_and_start() {
        // 4-piece 2×2: corners use colors {0,1}. With no hints, no
        // start-piece constraint. compute_heuristic_sides should
        // return ZERO usable colors (all are on corners).
        let pieces = vec![
            p(0, 0, 1, 1, 0), p(1, 0, 0, 1, 1),
            p(2, 1, 1, 0, 0), p(3, 1, 0, 0, 1),
        ];
        let puzzle = Puzzle::new(2, 2, 2, pieces).unwrap();
        let hints = eternity2_core::Hints::default();
        let colors = compute_heuristic_sides(&puzzle, &hints);
        // Both interior colors (0, 1) appear on corners (every piece is
        // a corner on 2×2), so all colors are forbidden.
        assert!(colors.is_empty() || colors.iter().all(|&c| c != 0 && c != 1));
    }

    #[test]
    fn blackwood_solve_tiny_puzzle_still_finds_solution() {
        // The 2×2 trivial puzzle has 1 solution. With a vacuous
        // BlackwoodSchedule (target 0 everywhere, no breaks) the engine
        // must still find it.
        let pieces = vec![
            p(0, 0, 1, 1, 0), p(1, 0, 0, 1, 1),
            p(2, 1, 1, 0, 0), p(3, 1, 0, 0, 1),
        ];
        let puzzle = Puzzle::new(2, 2, 2, pieces).unwrap();
        let schedule = Arc::new(BlackwoodSchedule {
            heuristic_sides: vec![1],
            exhaustion_targets: vec![(0, 0), (4, 0)],
            heuristic_pool_size: 0,
            max_heuristic_index: 4,
            break_indexes_allowed: vec![],
        });
        let mut cfg = EngineConfig::BORDER_FIRST_LCV;
        cfg.value_order = ValueOrder::BlackwoodHeuristic;
        cfg.scan_order = Some(ScanOrder::RowMajorBottomUp);
        let mut s = EngineSolver::new(cfg, "engine", "blackwood_test")
            .with_blackwood_schedule(schedule);
        let mut sink = eternity2_events::BufferSink::new();
        let outcome = s.solve(&puzzle, &SolveOpts::default(), &mut sink);
        assert!(matches!(outcome, SolveOutcome::Solved(_)),
            "Blackwood-mode solve on 2x2 trivial returned {outcome:?}");
    }

    #[test]
    fn blackwood_x_hint_rectangle_composition_path_is_coherent() {
        // When both path_skeleton=HintRectangle AND scan_order are set,
        // the engine's path_order must be: [rectangle cells in their
        // build order] ++ [remaining cells in scan order]. Every cell
        // appears exactly once. Engine traversal depth therefore equals
        // path_order index, and break_indexes_allowed (as depths) map
        // unambiguously to "the i-th cell placed".
        use eternity2_generator::{generate, GeneratorConfig};
        let puzzle = generate(GeneratorConfig {
            size: 5, interior_colors: 5, seed: 13,
        }).unwrap();
        // Need ≥4 hints for HintRectangle to be non-trivial. Use the
        // 4 corners of the generated puzzle (any valid (piece, rot)
        // there counts; for this structural test we don't care if the
        // hint is satisfiable, just that the path-builder consumes it).
        let mk_hint = |pos: u32| -> eternity2_core::Hint {
            // Use any piece for the hint slot; build_hint_rectangle_path
            // only consults the .position field.
            eternity2_core::Hint {
                position: pos,
                piece_id: 0,
                rotation: eternity2_core::Rotation::from_u8(0).unwrap(),
            }
        };
        let w = puzzle.width;
        let h = puzzle.height;
        let hints = eternity2_core::Hints::new(vec![
            mk_hint(0),
            mk_hint(w - 1),
            mk_hint((h - 1) * w),
            mk_hint(h * w - 1),
        ]);
        let mut cfg = EngineConfig::BORDER_FIRST_LCV;
        cfg.path_skeleton = Some(PathSkeleton::HintRectangle);
        cfg.scan_order = Some(ScanOrder::RowMajorBottomUp);
        let solver = EngineSolver::new(cfg, "engine", "comp");
        let mut opts = SolveOpts::default();
        opts.hints = hints;
        let state = SearchState::new(&puzzle, &solver, &opts);
        let n = puzzle.cell_count() as usize;
        // Sanity: every cell appears exactly once.
        assert_eq!(state.path_order.len(), n);
        let unique: std::collections::HashSet<_> = state.path_order.iter().copied().collect();
        assert_eq!(unique.len(), n);
        // auto_skeleton_path_k must cover the entire path (so the
        // PrefixConstraint / auto-skeleton machinery applies for all
        // depths).
        assert_eq!(state.auto_skeleton_path_k as usize, n);
        // Prefix length: the rectangle skeleton length depends on the
        // generated puzzle size; just assert it's non-empty.
        let skel = build_hint_rectangle_path(&puzzle, &opts.hints);
        let k = skel.len();
        assert!(k > 0, "rectangle skeleton should be non-empty for 5x5 + 4 corner hints");
        assert!(state.path_order[..k] == skel,
            "first k cells of path_order should be the rectangle path verbatim");
    }

    #[test]
    fn blackwood_break_index_allows_one_mismatch_on_small_puzzle() {
        // Construct a tiny puzzle where the only completion requires
        // exactly 1 edge mismatch. Without the break index, the engine
        // returns Exhausted; with it, the engine returns a partial
        // depth equal to all 4 cells placed (since one is at a break
        // index and may mismatch).
        //
        // 2×2 puzzle with corners that DON'T tile cleanly: piece 0 has
        // an asymmetric N-S to force a 1-mismatch completion. We'll
        // just verify the engine's recurse can place a row at the
        // break-index cell whose placed-neighbour edge color differs
        // from one of its neighbours' edges — without crashing —
        // exercising the augmented-candidate path.
        //
        // Concretely: the 2×2 trivial puzzle solves cleanly with 0
        // mismatches anyway, so we verify the engine SOLVES with break
        // index present (no false rejections from the break path).
        let pieces = vec![
            p(0, 0, 1, 1, 0), p(1, 0, 0, 1, 1),
            p(2, 1, 1, 0, 0), p(3, 1, 0, 0, 1),
        ];
        let puzzle = Puzzle::new(2, 2, 2, pieces).unwrap();
        let schedule = Arc::new(BlackwoodSchedule {
            heuristic_sides: vec![],
            exhaustion_targets: vec![(0, 0), (4, 0)],
            heuristic_pool_size: 0,
            max_heuristic_index: 4,
            break_indexes_allowed: vec![3],  // last cell in bu scan
        });
        let mut cfg = EngineConfig::BORDER_FIRST_LCV;
        cfg.value_order = ValueOrder::BlackwoodHeuristic;
        cfg.scan_order = Some(ScanOrder::RowMajorBottomUp);
        let mut s = EngineSolver::new(cfg, "engine", "blackwood_break_test")
            .with_blackwood_schedule(schedule);
        let mut sink = eternity2_events::BufferSink::new();
        let outcome = s.solve(&puzzle, &SolveOpts::default(), &mut sink);
        // Must still solve cleanly: break index doesn't prevent exact
        // matches, it just *also* allows ≤1 mismatch.
        assert!(matches!(outcome, SolveOutcome::Solved(_)),
            "engine should still find clean solution with break index: got {outcome:?}");
    }
}
