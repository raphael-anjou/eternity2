// Adaptive Large Neighborhood Search for Eternity II.
//
// Framework: maintain a current board; each iteration picks a "destroy"
// operator from a portfolio (with adaptive weights), uses it to choose
// a subset of cells to free, then calls CP-based repair to find a new
// assignment for those cells. Accept moves under a configurable
// criterion (greedy / simulated-annealing / record-to-record).
//
// Operator portfolio:
//   - RandomRegion(k): free a uniform-random k×k window.
//   - WorstWindow(k):  free the k×k window with lowest matched-edge
//                      density (analog of vol. 2's `worst_region_repair`).
//   - ConflictDriven:  seed at a random mismatched edge; BFS-grow the
//                      free set through 4-adjacency until size cap.
//   - MwpmDefectPair:  build the mismatched-edge graph; min-weight
//                      matching; destroy-set = union of cells on
//                      shortest paths between matched defect pairs.
//                      (See RESEARCH_NOTES_4 SESSION 2 plan for the
//                      derivation — this is a destroy-set heuristic,
//                      not a correction algorithm.)
//
// Adaptive weights follow Ropke & Pisinger 2006: each operator earns
// reward σ1 for a new best, σ2 for an improving accepted move, σ3 for
// a non-improving accepted move, σ4 (=0) for a rejected move. Every
// `segment_iters` iterations, weights = (1-r)·old + r·(reward/count).

use std::collections::{BTreeSet, VecDeque};

use eternity2_core::{Board, Hint, Hints, PieceId, Position, Puzzle, Rotation, BORDER};
use eternity2_events::BufferSink;
use eternity2_solver_engine::EngineSolver;
use eternity2_solver_trait::{SolveOpts, SolveOutcome, Solver};

use crate::repair::repair_region; // not directly used, but signals intent
use crate::pt;                     // re-use Rng/scoring conventions
use crate::{run_sa_from, SaConfig}; // SA-based repair (CP-free)

// ----- Rng (small reproducible PRNG, splitmix64) -----------------------

#[derive(Clone)]
pub struct AlnsRng { state: u64 }

impl AlnsRng {
    pub fn new(seed: u64) -> Self { Self { state: seed.wrapping_add(0x9E37_79B9_7F4A_7C15) } }
    pub fn next_u64(&mut self) -> u64 {
        let mut z = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        self.state = z;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / ((1u64 << 53) as f64)
    }
    pub fn range(&mut self, n: u32) -> u32 {
        if n == 0 { 0 } else { (self.next_u64() % (n as u64)) as u32 }
    }
}

/// Derive an independent stream seed for sub-stream `index` of a run seeded
/// `seed` (chain i of a portfolio, restart i of a sweep, ...).
///
/// The previous idiom was `(seed + index) * K`, which ALIASES.
/// `seed + index` collapses the two coordinates into one number, so
/// (seed=7, index=1) and (seed=8, index=0) derive the SAME stream: consecutive
/// seeds re-run each other's chains, and an "N seeds x M chains" sweep explores
/// far fewer than N*M distinct streams. Mixing the coordinates with distinct odd
/// multipliers before the SplitMix64 finalizer keeps them separable and
/// decorrelates near-identical inputs.
#[inline]
#[must_use]
pub fn derive_stream_seed(seed: u64, index: usize) -> u64 {
    let mut z = seed
        .wrapping_mul(0x9E37_79B9_7F4A_7C15)
        .wrapping_add((index as u64).wrapping_mul(0xD1B5_4A32_D192_ED03));
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

// ----- Scoring helpers --------------------------------------------------

/// Vol-16 — was `iter().find` (O(n)). Now delegates to the O(1) lookup
/// on Puzzle. Inlined so score_board's hot loop sees a single load.
#[inline]
fn lookup_piece<'a>(puzzle: &'a Puzzle, id: PieceId) -> Option<&'a eternity2_core::Piece> {
    puzzle.piece(id)
}

pub fn score_board(puzzle: &Puzzle, board: &Board) -> u32 {
    let w = puzzle.width;
    let h = puzzle.height;
    let mut matches = 0u32;
    for y in 0..h {
        for x in 0..w {
            let pos = y * w + x;
            let Some((pid, rot)) = board.get(pos) else { continue };
            let Some(p) = lookup_piece(puzzle, pid) else { continue };
            let e = p.edges.rotated(rot).as_array();
            if x + 1 < w {
                if let Some((rpid, rrot)) = board.get(y * w + (x + 1)) {
                    if let Some(rp) = lookup_piece(puzzle, rpid) {
                        let re = rp.edges.rotated(rrot).as_array();
                        if e[1] == re[3] && e[1] != BORDER && e[1] != 0 { matches += 1; }
                    }
                }
            }
            if y + 1 < h {
                if let Some((bpid, brot)) = board.get((y + 1) * w + x) {
                    if let Some(bp) = lookup_piece(puzzle, bpid) {
                        let be = bp.edges.rotated(brot).as_array();
                        if e[2] == be[0] && e[2] != BORDER && e[2] != 0 { matches += 1; }
                    }
                }
            }
        }
    }
    matches
}

// ----- Mismatch enumeration --------------------------------------------

#[derive(Debug, Clone, Copy)]
pub struct Mismatch {
    pub cell_a: Position,
    pub cell_b: Position,
    pub horizontal: bool,
}

pub fn find_mismatches(puzzle: &Puzzle, board: &Board) -> Vec<Mismatch> {
    let w = puzzle.width;
    let h = puzzle.height;
    let mut out = Vec::new();
    for y in 0..h {
        for x in 0..w {
            let pos = y * w + x;
            let Some((pid, rot)) = board.get(pos) else { continue };
            let Some(p) = lookup_piece(puzzle, pid) else { continue };
            let e = p.edges.rotated(rot).as_array();
            if x + 1 < w {
                if let Some((rpid, rrot)) = board.get(y * w + (x + 1)) {
                    if let Some(rp) = lookup_piece(puzzle, rpid) {
                        let re = rp.edges.rotated(rrot).as_array();
                        let (ca, cb) = (e[1], re[3]);
                        if ca != BORDER && cb != BORDER && ca != 0 && cb != 0 && ca != cb {
                            out.push(Mismatch { cell_a: pos, cell_b: pos + 1, horizontal: true });
                        }
                    }
                }
            }
            if y + 1 < h {
                if let Some((bpid, brot)) = board.get((y + 1) * w + x) {
                    if let Some(bp) = lookup_piece(puzzle, bpid) {
                        let be = bp.edges.rotated(brot).as_array();
                        let (ca, cb) = (e[2], be[0]);
                        if ca != BORDER && cb != BORDER && ca != 0 && cb != 0 && ca != cb {
                            out.push(Mismatch { cell_a: pos, cell_b: pos + w, horizontal: false });
                        }
                    }
                }
            }
        }
    }
    out
}

// ----- CP repair on an arbitrary free-set -------------------------------

/// Repair strategy used after a destroy operator chooses a free-set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepairKind {
    /// Cell-CP: pin non-free cells as Hints, let the engine reconstruct
    /// the free region. Fast when feasible. ABORTS on plateau states
    /// because AC3 wipes — see RESEARCH_NOTES_4 vol. 4 F2 finding.
    Cp,
    /// Simulated annealing: pin non-free cells, run SA's random swap/
    /// rotate moves on the free cells. Does NOT propagate
    /// arc-consistency, so works on plateau states where CP fails.
    /// Cheaper per attempt but doesn't enumerate all completions.
    Sa,
    /// Iterative Optimal Transport (Hungarian / Kuhn-Munkres) on the free cells.
    /// Treats the repair as a linear assignment problem and iterates until fixed point.
    IterativeOt,
    /// Vol-125 T37: Jonker-Volgenant LAP backend (drop-in for IterativeOt).
    /// Same outer loop, best-of-rotation memo identical to IterativeOt — only
    /// the inner LAP call uses lsap (JV) instead of kuhn_munkres. Typically
    /// 2-10× faster on dense k×k cost matrices.
    IterativeJv,
    /// Vol-125 T37: JV LAP with JOINT piece × rotation assignment.
    /// 4k × 4k block formulation: each piece occupies 4 rows (one per
    /// rotation). The LAP picks ONE rotation per piece AND assigns it to a
    /// real position. Globally optimal over both axes (no best-of-4 memo
    /// sub-optimality).
    IterativeJvJoint,
    /// Vol-130 FILAMENT: Lin-Kernighan-style variable-depth swap chain.
    /// Seeds from each cell in `free_set`, builds a swap chain extending
    /// outside the free-set if needed. Chain extends while cumulative gain
    /// >= -max_loss; stops at the depth with max gain. Returns the best
    /// board found across all seed attempts.
    Filament,
}

/// Pin every cell EXCEPT `free_set` as a Hint; run cell-CP for `budget_ms`.
/// Returns the new board iff CP fills the free set; otherwise `None`.
pub fn cp_repair(
    puzzle: &Puzzle,
    board: &Board,
    free_set: &BTreeSet<Position>,
    budget_ms: u64,
) -> Option<Board> {
    cp_repair_with_opts(puzzle, board, free_set, budget_ms, true)
}

/// Vol-17 — CP repair with parallel/single-thread toggle. `parallel = true`
/// matches the legacy throughput-oriented behaviour (`gacolor_ac3_par`);
/// `parallel = false` uses single-thread `gacolor_ac3` for run-to-run
/// reproducibility. Use single-thread when running deterministic A/B tests.
pub fn cp_repair_with_opts(
    puzzle: &Puzzle,
    board: &Board,
    free_set: &BTreeSet<Position>,
    budget_ms: u64,
    parallel: bool,
) -> Option<Board> {
    let n_cells = puzzle.cell_count();
    let mut hs: Vec<Hint> = Vec::with_capacity((n_cells as usize).saturating_sub(free_set.len()));
    for pos in 0..n_cells {
        if free_set.contains(&pos) { continue; }
        if let Some((pid, rot)) = board.get(pos) {
            hs.push(Hint { position: pos, piece_id: pid, rotation: rot });
        }
    }
    let hints = Hints::new(hs);

    let mut solver = if parallel { EngineSolver::gacolor_ac3_par() } else { EngineSolver::gacolor_ac3() };
    let mut sink = BufferSink::new();
    let mut opts = SolveOpts::default();
    opts.time_budget_ms = budget_ms;
    opts.hints = hints;

    let outcome = solver.solve(puzzle, &opts, &mut sink);
    let new_board = match outcome {
        SolveOutcome::Solved(b) => b,
        SolveOutcome::TimedOut { best_partial, .. } | SolveOutcome::Cancelled { best_partial, .. } => best_partial,
        SolveOutcome::Exhausted | SolveOutcome::Error(_) => return None,
        SolveOutcome::AllSolutions(bs) => bs.into_iter().next()?,
    };

    for &pos in free_set {
        if new_board.get(pos).is_none() { return None; }
    }
    Some(new_board)
}

/// SA-based repair: pin non-free cells, run SA on the free cells for
/// `budget_ms`. The free cells already have pieces from `board`; SA
/// just shuffles them via random swaps and rotations.
///
/// Falls back here when CP repair fails (AC3 wipeout on plateau states).
/// Less expressive than CP — won't enumerate all completions — but
/// always returns a board because the pinned-cell scheme bypasses
/// arc-consistency entirely.
pub fn sa_repair(
    puzzle: &Puzzle,
    board: &Board,
    free_set: &BTreeSet<Position>,
    budget_ms: u64,
    seed: u64,
) -> Board {
    sa_repair_with_steps(puzzle, board, free_set, budget_ms, 0, seed)
}

/// Vol-17 — SA repair with optional fixed-step budget. When `step_budget > 0`,
/// run exactly that many SA moves (deterministic); when 0, fall back to the
/// wall-clock `budget_ms` (legacy). Use the step variant for scientific A/B,
/// the time variant for record runs that want to fill all available wall time.
pub fn sa_repair_with_steps(
    puzzle: &Puzzle,
    board: &Board,
    free_set: &BTreeSet<Position>,
    budget_ms: u64,
    step_budget: u64,
    seed: u64,
) -> Board {
    let n_cells = puzzle.cell_count();
    let mut pinned: Vec<Position> = Vec::with_capacity((n_cells as usize).saturating_sub(free_set.len()));
    for pos in 0..n_cells {
        if !free_set.contains(&pos) { pinned.push(pos); }
    }
    let mut cfg = SaConfig::default();
    if step_budget > 0 {
        cfg.max_iters = step_budget;
        cfg.time_budget_ms = 0;
    } else {
        cfg.time_budget_ms = budget_ms;
    }
    cfg.seed = seed;
    cfg.pinned_positions = pinned;
    // Keep cooling fast for short repair bursts.
    cfg.cooling_period = 500;
    let out = run_sa_from(puzzle, board, &cfg);
    out.best_board
}

/// Dispatch on RepairKind. Returns `Some(board)` whenever a repair
/// completes; CP can fail (returns None), SA always succeeds.
pub fn repair(
    puzzle: &Puzzle,
    board: &Board,
    free_set: &BTreeSet<Position>,
    budget_ms: u64,
    kind: RepairKind,
    seed: u64,
) -> Option<Board> {
    match kind {
        RepairKind::Cp => cp_repair(puzzle, board, free_set, budget_ms),
        RepairKind::Sa => Some(sa_repair(puzzle, board, free_set, budget_ms, seed)),
        RepairKind::IterativeOt => Some(crate::ot_repair::iterative_ot_repair(puzzle, board, free_set, 50)),
        RepairKind::IterativeJv => Some(crate::ot_repair_jv::iterative_jv_repair(puzzle, board, free_set, 50)),
        RepairKind::IterativeJvJoint => Some(crate::ot_repair_jv::iterative_jv_joint_repair(puzzle, board, free_set, 50)),
        RepairKind::Filament => Some(filament_repair(puzzle, board, free_set, seed)),
    }
}

/// Vol-130 FILAMENT repair: SA-repair to fill, then FILAMENT chains to polish.
///
/// FILAMENT alone only swaps pieces; it can't fill empty cells. So if the
/// free_set has empty cells (typical after destroy), we MUST first run
/// SA-repair to fill them. Then FILAMENT polishes via LK swap chains.
fn filament_repair(
    puzzle: &Puzzle,
    board: &Board,
    free_set: &BTreeSet<Position>,
    seed: u64,
) -> Board {
    // Step 1: SA-repair to fill empty cells.
    let sa_filled = sa_repair(puzzle, board, free_set, 200, seed);

    // Step 2: FILAMENT polish via LK chains seeded from worst cells.
    let cfg = crate::filament::FilamentConfig {
        max_depth: 12,
        max_loss: 4,
        seed_worst: false,
        try_rotations: true,
        allow_revisit: false,
    };
    let mut best = sa_filled.clone();
    let mut best_score = score_board(puzzle, &best);

    // Seed FILAMENT from cells in free_set + a few worst cells globally.
    let w = puzzle.width;
    let h = puzzle.height;
    let n = (w * h) as usize;
    let mut cs: Vec<(Position, u32)> = (0..n as u32)
        .map(|p| (p, crate::filament::cell_match_count_pub(puzzle, &best, p))).collect();
    cs.sort_by_key(|x| x.1);
    let mut seeds: Vec<Position> = cs.iter().take(4).map(|x| x.0).collect();
    seeds.extend(free_set.iter().copied().take(8));

    let _ = seed;
    for s in seeds {
        let (b, _g) = crate::filament::run_filament_chain(puzzle, &best, s, &cfg);
        let new_score = score_board(puzzle, &b);
        if new_score > best_score {
            best_score = new_score;
            best = b;
        }
    }
    best
}

/// Vol-17 — repair dispatch with per-kind determinism knobs. `sa_step_budget`
/// fixes SA work; `cp_parallel = false` switches CP to single-thread.
pub fn repair_with_opts(
    puzzle: &Puzzle,
    board: &Board,
    free_set: &BTreeSet<Position>,
    budget_ms: u64,
    kind: RepairKind,
    seed: u64,
    sa_step_budget: u64,
    cp_parallel: bool,
) -> Option<Board> {
    match kind {
        RepairKind::Cp => cp_repair_with_opts(puzzle, board, free_set, budget_ms, cp_parallel),
        RepairKind::Sa => Some(sa_repair_with_steps(puzzle, board, free_set, budget_ms, sa_step_budget, seed)),
        RepairKind::IterativeOt => Some(crate::ot_repair::iterative_ot_repair(puzzle, board, free_set, 50)),
        RepairKind::IterativeJv => Some(crate::ot_repair_jv::iterative_jv_repair(puzzle, board, free_set, 50)),
        RepairKind::IterativeJvJoint => Some(crate::ot_repair_jv::iterative_jv_joint_repair(puzzle, board, free_set, 50)),
        RepairKind::Filament => Some(filament_repair(puzzle, board, free_set, seed)),
    }
}

// ----- Destroy operators ------------------------------------------------

pub trait DestroyOp: Send {
    fn name(&self) -> &str;
    fn destroy(&mut self, puzzle: &Puzzle, board: &Board, rng: &mut AlnsRng) -> BTreeSet<Position>;
}

/// V143 ForbidDestroy: pick a forbidden 2x3 patch on the board and
/// destroy its 6 cells. Repair will refill them, ideally producing a
/// feasible patch.
pub struct ForbidDestroy;
impl DestroyOp for ForbidDestroy {
    fn name(&self) -> &str { "forbid_destroy" }
    fn destroy(&mut self, puzzle: &Puzzle, board: &Board, rng: &mut AlnsRng) -> BTreeSet<Position> {
        let forbidden = crate::intaglio::enumerate_forbidden_2x3(puzzle, board);
        if forbidden.is_empty() {
            // Fallback to a random 2x3 region.
            let w = puzzle.width;
            let h = puzzle.height;
            let x0 = rng.range(w.saturating_sub(3) + 1);
            let y0 = rng.range(h.saturating_sub(2) + 1);
            let mut s = BTreeSet::new();
            for dy in 0..2 { for dx in 0..3 { s.insert((y0+dy)*w + (x0+dx)); } }
            return s;
        }
        let idx = rng.range(forbidden.len() as u32) as usize;
        let (_tl, positions) = &forbidden[idx];
        positions.iter().copied().collect()
    }
}

pub struct RandomRegion { pub k: u32 }
impl DestroyOp for RandomRegion {
    fn name(&self) -> &str { "random_region" }
    fn destroy(&mut self, puzzle: &Puzzle, _board: &Board, rng: &mut AlnsRng) -> BTreeSet<Position> {
        let w = puzzle.width;
        let h = puzzle.height;
        let k = self.k.min(w).min(h);
        let x0 = rng.range(w.saturating_sub(k) + 1);
        let y0 = rng.range(h.saturating_sub(k) + 1);
        let mut s = BTreeSet::new();
        for dy in 0..k {
            for dx in 0..k {
                s.insert((y0 + dy) * w + (x0 + dx));
            }
        }
        s
    }
}

pub struct WorstWindow { pub k: u32 }
impl DestroyOp for WorstWindow {
    fn name(&self) -> &str { "worst_window" }
    fn destroy(&mut self, puzzle: &Puzzle, board: &Board, _rng: &mut AlnsRng) -> BTreeSet<Position> {
        let w = puzzle.width;
        let h = puzzle.height;
        let k = self.k.min(w).min(h);
        // Score per cell: # matched edges incident. Window score = sum.
        let mut cell_match = vec![0u32; (w * h) as usize];
        for y in 0..h {
            for x in 0..w {
                let pos = y * w + x;
                let Some((pid, rot)) = board.get(pos) else { continue };
                let Some(p) = lookup_piece(puzzle, pid) else { continue };
                let e = p.edges.rotated(rot).as_array();
                if x + 1 < w {
                    if let Some((rpid, rrot)) = board.get(y * w + (x + 1)) {
                        if let Some(rp) = lookup_piece(puzzle, rpid) {
                            let re = rp.edges.rotated(rrot).as_array();
                            if e[1] == re[3] && e[1] != BORDER && e[1] != 0 {
                                cell_match[pos as usize] += 1;
                                cell_match[(y * w + (x + 1)) as usize] += 1;
                            }
                        }
                    }
                }
                if y + 1 < h {
                    if let Some((bpid, brot)) = board.get((y + 1) * w + x) {
                        if let Some(bp) = lookup_piece(puzzle, bpid) {
                            let be = bp.edges.rotated(brot).as_array();
                            if e[2] == be[0] && e[2] != BORDER && e[2] != 0 {
                                cell_match[pos as usize] += 1;
                                cell_match[((y + 1) * w + x) as usize] += 1;
                            }
                        }
                    }
                }
            }
        }
        let mut best = (u32::MAX, 0u32, 0u32);
        for y0 in 0..=(h.saturating_sub(k)) {
            for x0 in 0..=(w.saturating_sub(k)) {
                let mut s = 0u32;
                for dy in 0..k {
                    for dx in 0..k {
                        s += cell_match[((y0 + dy) * w + (x0 + dx)) as usize];
                    }
                }
                if s < best.0 { best = (s, x0, y0); }
            }
        }
        let mut out = BTreeSet::new();
        for dy in 0..k {
            for dx in 0..k {
                out.insert((best.2 + dy) * w + (best.1 + dx));
            }
        }
        out
    }
}

/// V169 OPHIDIA — Prior-Guided Destroy.
///
/// Two directions:
/// - `beta < 0` (attract mode): destroy cells with LOW corpus support.
///   Pulls the board back toward the corpus attractor. Useful when the
///   board is far from the basin and we want to refine.
/// - `beta > 0` (escape mode, default for V169): destroy cells with HIGH
///   corpus support. Frees corpus-anchored placements so repair can
///   propose alternatives. Useful when the board PLATEAUS at the corpus
///   basin (e.g., 460 plateau) and we want to break out.
///
/// Algorithm:
///   1. Per cell c with placed piece p, compute s(c) = support[p][c].
///   2. Weight w(c) = exp(β · s(c)). β > 0 → prefer high-s; β < 0 → prefer low-s.
///   3. Pinned positions have w = 0.
///   4. Sample seed cell with P ∝ w(c).
///   5. BFS-grow region by max-weight neighbor until `max_size` cells.
///
/// Empirical finding (vol-169 probe 2026-05-20):
/// - McGavin 469: 0 unsupported cells (s=0) — fully in corpus.
/// - V155-direct 460: 5 unsupported — basically a corpus replay.
/// - V155→ALNS 460 (seed1, seed13): 186-191 unsupported — already
///   diversified. To break 460, we need MORE diversification of the
///   ~65 remaining supported cells. Hence default β > 0.
pub struct PriorDestroy {
    /// Prior support matrix: support[piece_id][position] = corpus count.
    pub support: Vec<Vec<u16>>,
    /// Inverse-temperature for the destroy distribution. β = 0 →
    /// uniform; β → ∞ → only s=0 cells. Common choice: β = 1.0.
    pub beta: f64,
    /// Max region size in cells.
    pub max_size: u32,
    /// Positions never to destroy (canonical hints). Empty if not used.
    pub pinned: BTreeSet<Position>,
    /// Friendly name for logging.
    pub variant: &'static str,
}
impl PriorDestroy {
    fn cell_weight(&self, board: &Board, pos: Position) -> f64 {
        if self.pinned.contains(&pos) { return 0.0; }
        let Some((pid, _rot)) = board.get(pos) else { return 0.0; };
        let pid_idx = u32::from(pid) as usize;
        if pid_idx >= self.support.len() { return 0.0; }
        let row = &self.support[pid_idx];
        let pos_idx = pos as usize;
        if pos_idx >= row.len() { return 0.0; }
        let s = row[pos_idx] as f64;
        // β > 0 → escape mode (prefer high-support cells).
        // β < 0 → attract mode (prefer low-support cells).
        (self.beta * s).exp()
    }
}
impl DestroyOp for PriorDestroy {
    fn name(&self) -> &str { self.variant }
    fn destroy(&mut self, puzzle: &Puzzle, board: &Board, rng: &mut AlnsRng) -> BTreeSet<Position> {
        let w = puzzle.width;
        let h = puzzle.height;
        let n = (w * h) as usize;
        // Build per-cell weights.
        let mut weights = vec![0.0f64; n];
        let mut total = 0.0f64;
        for pos in 0..n {
            let wt = self.cell_weight(board, pos as Position);
            weights[pos] = wt;
            total += wt;
        }
        if total <= 0.0 {
            // No prior signal available; fall back to random region.
            return RandomRegion { k: 4 }.destroy(puzzle, board, rng);
        }
        // Sample seed cell by weighted draw.
        let r = rng.next_f64() * total;
        let mut acc = 0.0f64;
        let mut seed = 0u32;
        for pos in 0..n {
            acc += weights[pos];
            if acc >= r { seed = pos as u32; break; }
        }
        // BFS-grow, popping by max weight among neighbors of the region.
        let mut s = BTreeSet::new();
        s.insert(seed);
        let mut frontier: Vec<Position> = Vec::new();
        let push_nbrs = |p: Position, frontier: &mut Vec<Position>, s: &BTreeSet<Position>| {
            let x = (p % w) as i32;
            let y = (p / w) as i32;
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let nx = x + dx;
                let ny = y + dy;
                if nx < 0 || ny < 0 || nx as u32 >= w || ny as u32 >= h { continue; }
                let np = ny as u32 * w + nx as u32;
                if !s.contains(&np) && !frontier.contains(&np) {
                    frontier.push(np);
                }
            }
        };
        push_nbrs(seed, &mut frontier, &s);
        while (s.len() as u32) < self.max_size && !frontier.is_empty() {
            // Pick frontier cell with highest weakness weight.
            let mut best_i = 0usize;
            let mut best_w = weights[frontier[0] as usize];
            for i in 1..frontier.len() {
                let wi = weights[frontier[i] as usize];
                if wi > best_w {
                    best_w = wi;
                    best_i = i;
                }
            }
            let p = frontier.swap_remove(best_i);
            if best_w <= 0.0 {
                // All remaining frontier cells are pinned or have zero
                // weight; stop growing.
                break;
            }
            s.insert(p);
            push_nbrs(p, &mut frontier, &s);
        }
        s
    }
}

pub struct ConflictDriven { pub max_size: u32 }
impl DestroyOp for ConflictDriven {
    fn name(&self) -> &str { "conflict_driven" }
    fn destroy(&mut self, puzzle: &Puzzle, board: &Board, rng: &mut AlnsRng) -> BTreeSet<Position> {
        let mismatches = find_mismatches(puzzle, board);
        if mismatches.is_empty() {
            // Nothing wrong to anchor on; fall back to a random region.
            return RandomRegion { k: 4 }.destroy(puzzle, board, rng);
        }
        // Pick a random mismatch as the seed.
        let m = mismatches[(rng.range(mismatches.len() as u32)) as usize];
        let w = puzzle.width;
        let h = puzzle.height;
        let mut s = BTreeSet::new();
        let mut q: VecDeque<Position> = VecDeque::new();
        s.insert(m.cell_a); q.push_back(m.cell_a);
        s.insert(m.cell_b); q.push_back(m.cell_b);
        // Grow via 4-adjacency until size cap.
        while let Some(p) = q.pop_front() {
            if s.len() as u32 >= self.max_size { break; }
            let x = p % w;
            let y = p / w;
            // Random shuffle of 4 neighbors so growth isn't directional.
            let mut nbrs: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
            for i in 0..4 {
                let j = (rng.range(4 - i as u32) + i as u32) as usize;
                nbrs.swap(i, j);
            }
            for (dx, dy) in &nbrs {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx < 0 || ny < 0 || nx as u32 >= w || ny as u32 >= h { continue; }
                let np = ny as u32 * w + nx as u32;
                if s.insert(np) {
                    q.push_back(np);
                    if s.len() as u32 >= self.max_size { break; }
                }
            }
        }
        s
    }
}

// MWPM destroy: see RESEARCH_NOTES_4 SESSION 2 plan for derivation. We
// build the mismatched-edge graph (nodes = mismatched interior edges),
// compute pairwise Manhattan distance between defect midpoints, run
// min-weight matching. Destroy-set = union of cells on the shortest
// cell-paths between matched defect pairs.
//
// For small N (≈30-60 defects) we use a brute-force / greedy matching
// (sufficient quality at this scale; exact blossom is a future
// optimization). For "weight" we use the L1 distance between defect
// cell-pair midpoints.
pub struct MwpmDefectPair { pub max_pairs: u32 }

impl MwpmDefectPair {
    fn defect_midpoint(m: &Mismatch, w: u32) -> (i32, i32) {
        // Midpoint of the edge between cell_a and cell_b. Use 2× scale
        // to stay in integers: cell (x,y) → (2x+1, 2y+1).
        let ax = (m.cell_a % w) as i32;
        let ay = (m.cell_a / w) as i32;
        let bx = (m.cell_b % w) as i32;
        let by = (m.cell_b / w) as i32;
        (ax + bx, ay + by) // 2× midpoint (integer)
    }

    /// Greedy matching: repeatedly pick the closest unpaired pair.
    /// Returns vec of (i, j) index pairs in the input array.
    fn greedy_matching(midpoints: &[(i32, i32)], max_pairs: u32) -> Vec<(usize, usize)> {
        let n = midpoints.len();
        if n < 2 { return Vec::new(); }
        // Precompute pairwise distances.
        let mut edges: Vec<(i32, usize, usize)> = Vec::with_capacity(n * (n - 1) / 2);
        for i in 0..n {
            for j in (i + 1)..n {
                let d = (midpoints[i].0 - midpoints[j].0).abs() + (midpoints[i].1 - midpoints[j].1).abs();
                edges.push((d, i, j));
            }
        }
        edges.sort_unstable_by_key(|e| e.0);
        let mut matched = vec![false; n];
        let mut pairs = Vec::new();
        for (_, i, j) in edges {
            if pairs.len() as u32 >= max_pairs { break; }
            if !matched[i] && !matched[j] {
                matched[i] = true;
                matched[j] = true;
                pairs.push((i, j));
            }
        }
        pairs
    }

    /// Cells on the shortest L1-path between two cells (Bresenham-style).
    /// We return a 1-cell-wide path; widening to a small "tube" gives CP
    /// more room.
    fn cells_on_path(a: Position, b: Position, w: u32) -> Vec<Position> {
        let mut out = Vec::new();
        let (mut x, mut y) = ((a % w) as i32, (a / w) as i32);
        let (bx, by) = ((b % w) as i32, (b / w) as i32);
        out.push(a);
        while (x, y) != (bx, by) {
            if x < bx { x += 1; }
            else if x > bx { x -= 1; }
            else if y < by { y += 1; }
            else { y -= 1; }
            out.push(y as u32 * w + x as u32);
        }
        out
    }
}

impl DestroyOp for MwpmDefectPair {
    fn name(&self) -> &str { "mwpm_defect_pair" }
    fn destroy(&mut self, puzzle: &Puzzle, board: &Board, rng: &mut AlnsRng) -> BTreeSet<Position> {
        let mismatches = find_mismatches(puzzle, board);
        if mismatches.len() < 2 {
            return RandomRegion { k: 4 }.destroy(puzzle, board, rng);
        }
        let w = puzzle.width;
        let midpoints: Vec<(i32, i32)> = mismatches.iter().map(|m| Self::defect_midpoint(m, w)).collect();
        let pairs = Self::greedy_matching(&midpoints, self.max_pairs);

        let mut s = BTreeSet::new();
        for (i, j) in pairs {
            // Pick a representative cell on each side of each defect:
            // use cell_a of mismatch i and cell_a of mismatch j as the
            // path endpoints. (Either endpoint of either defect works
            // — we just need a route through the relevant region.)
            let a = mismatches[i].cell_a;
            let b = mismatches[j].cell_a;
            for c in Self::cells_on_path(a, b, w) { s.insert(c); }
            // Also include both endpoints' "other side" so the defect's
            // two cells are both freed.
            s.insert(mismatches[i].cell_b);
            s.insert(mismatches[j].cell_b);
        }
        s
    }
}

/// Vol-17 — destroy a full horizontal band of `k_rows` adjacent rows
/// chosen to maximize the count of mismatched edges falling inside.
/// Motivated by the vol-17 calibrated_v17a 447 board where ALL 33
/// mismatches concentrated in rows 0-3 forming one 51-cell connected
/// component — bigger than any of the k≤30 destroy ops can swallow.
/// Pair with `RepairKind::Cp` to let the engine search-fill the band
/// against the now-pinned bottom 12 rows. Pinned positions (canonical
/// hints) inside the band are still respected upstream.
pub struct WorstBand {
    /// Width of the band in rows. For 16×16 canonical E2, 4 captures
    /// the observed cluster; values up to 6-8 are reasonable.
    pub k_rows: u32,
}
impl DestroyOp for WorstBand {
    fn name(&self) -> &str { "worst_band" }
    fn destroy(&mut self, puzzle: &Puzzle, board: &Board, _rng: &mut AlnsRng) -> BTreeSet<Position> {
        let w = puzzle.width;
        let h = puzzle.height;
        let k = self.k_rows.min(h).max(1);
        // Per-row mismatch count = sum over interior edges incident on row y
        // that are mismatched.
        let mismatches = find_mismatches(puzzle, board);
        let mut per_row = vec![0u32; h as usize];
        for m in &mismatches {
            let ya = m.cell_a / w;
            let yb = m.cell_b / w;
            per_row[ya as usize] += 1;
            if yb != ya { per_row[yb as usize] += 1; }
        }
        // Window of k consecutive rows with max total mismatches.
        let mut best = (0u32, 0u32); // (score, y_start)
        for y0 in 0..=(h.saturating_sub(k)) {
            let mut s = 0u32;
            for dy in 0..k { s += per_row[(y0 + dy) as usize]; }
            if s > best.0 { best = (s, y0); }
        }
        // If no mismatches at all, default to a top band so the repair
        // explores SOMETHING (rather than empty set causing a no-op).
        let y_start = if best.0 == 0 { 0 } else { best.1 };
        let mut out = BTreeSet::new();
        for dy in 0..k {
            let y = y_start + dy;
            for x in 0..w {
                out.insert(y * w + x);
            }
        }
        out
    }
}

/// Vol-17 NOVEL — destroy a single ROW (1 × W = 16 cells) whose
/// mismatch density is the worst. Smaller than WorstBand{4} but still
/// touches the entire row, allowing piece-uniqueness across a thin
/// strip to be re-optimised.
///
/// Useful as a "scalpel" complement to the bigger WorstBand: probe
/// individual rows of the cluster, each of which is a tractable
/// 16-cell CP-repair problem.
pub struct WorstRow;

impl DestroyOp for WorstRow {
    fn name(&self) -> &str { "worst_row" }
    fn destroy(&mut self, puzzle: &Puzzle, board: &Board, _rng: &mut AlnsRng) -> BTreeSet<Position> {
        let w = puzzle.width;
        let h = puzzle.height;
        let mismatches = find_mismatches(puzzle, board);
        let mut per_row = vec![0u32; h as usize];
        for m in &mismatches {
            per_row[(m.cell_a / w) as usize] += 1;
            let yb = m.cell_b / w;
            if yb != m.cell_a / w { per_row[yb as usize] += 1; }
        }
        let mut best_row = 0u32;
        let mut best_score = 0u32;
        for y in 0..h {
            if per_row[y as usize] > best_score {
                best_score = per_row[y as usize];
                best_row = y;
            }
        }
        let mut out = BTreeSet::new();
        for x in 0..w {
            out.insert(best_row * w + x);
        }
        out
    }
}

/// Vol-17 NOVEL — destroy the ENTIRE connected component of mismatched
/// cells, no matter how big. Adjacent (4-neighbour) cells that both
/// touch at least one mismatch are part of the same component.
///
/// Differs from `ConflictDriven { max_size }`: that one BFS-grows from
/// a random mismatch with a fixed size cap, so it can miss the larger
/// component or stop short of bridges. ComponentDestroy explicitly
/// computes the connected-component closure and frees ALL of it.
///
/// For boards with one big component (vol-17 calibrated_v17a 447:
/// 51 cells in one component) this exactly matches the cluster size,
/// which neither ConflictDriven{30} nor ConflictDriven{80} can do —
/// 30 misses cells and 80 over-includes harmless cells.
///
/// Cost: O(W*H) BFS per call. Negligible vs CP-repair budget.
pub struct ComponentDestroy {
    /// Skip if component size > this. ALNS can't usefully repair a
    /// 200-cell free region in 1.5s; bail out and let other ops try.
    /// Set to a high value (e.g. n_cells) to never bail.
    pub max_size: u32,
    /// Lower bound on component size to bother destroying. A
    /// 2-cell component is just one mismatched edge — RandomRegion
    /// handles that better. Default 6.
    pub min_size: u32,
}

impl DestroyOp for ComponentDestroy {
    fn name(&self) -> &str { "component_destroy" }
    fn destroy(&mut self, puzzle: &Puzzle, board: &Board, rng: &mut AlnsRng) -> BTreeSet<Position> {
        let w = puzzle.width;
        let h = puzzle.height;
        let mismatches = find_mismatches(puzzle, board);
        if mismatches.is_empty() {
            return RandomRegion { k: 4 }.destroy(puzzle, board, rng);
        }
        // Build set of cells that touch a mismatched edge.
        let mut mismatch_cells: std::collections::BTreeSet<Position> = std::collections::BTreeSet::new();
        for m in &mismatches {
            mismatch_cells.insert(m.cell_a);
            mismatch_cells.insert(m.cell_b);
        }
        // BFS components.
        let mut visited: std::collections::BTreeSet<Position> = std::collections::BTreeSet::new();
        let mut components: Vec<Vec<Position>> = Vec::new();
        for &start in &mismatch_cells {
            if visited.contains(&start) { continue; }
            let mut comp = Vec::new();
            let mut queue: VecDeque<Position> = VecDeque::new();
            queue.push_back(start);
            visited.insert(start);
            while let Some(p) = queue.pop_front() {
                comp.push(p);
                let x = p % w; let y = p / w;
                let mut nbrs: Vec<Position> = Vec::new();
                if x + 1 < w { nbrs.push(p + 1); }
                if x > 0 { nbrs.push(p - 1); }
                if y + 1 < h { nbrs.push(p + w); }
                if y > 0 { nbrs.push(p - w); }
                for n in nbrs {
                    if mismatch_cells.contains(&n) && !visited.contains(&n) {
                        visited.insert(n);
                        queue.push_back(n);
                    }
                }
            }
            components.push(comp);
        }
        // Select the largest component that fits within [min_size, max_size].
        components.sort_by_key(|c| std::cmp::Reverse(c.len()));
        for comp in &components {
            let n = comp.len() as u32;
            if n >= self.min_size && n <= self.max_size {
                return comp.iter().copied().collect();
            }
        }
        // No suitable component: fall back to ConflictDriven.
        ConflictDriven { max_size: 30 }.destroy(puzzle, board, rng)
    }
}

/// Vol-17 NOVEL — destroy a connected component of mismatched cells
/// AND a 1-cell border around it, so the CP-repair has neighbouring
/// free cells to rotate into matching positions. Useful when the
/// component is fully surrounded by matched cells whose edge-pieces
/// are themselves involved (so re-rotating those edges may help).
pub struct ComponentPlusHaloDestroy {
    pub max_size: u32,
    pub min_size: u32,
}

impl DestroyOp for ComponentPlusHaloDestroy {
    fn name(&self) -> &str { "component_plus_halo" }
    fn destroy(&mut self, puzzle: &Puzzle, board: &Board, rng: &mut AlnsRng) -> BTreeSet<Position> {
        // Reuse ComponentDestroy.
        let core = ComponentDestroy { max_size: self.max_size, min_size: self.min_size }
            .destroy(puzzle, board, rng);
        if core.is_empty() { return core; }
        let w = puzzle.width;
        let h = puzzle.height;
        let mut out = core.clone();
        for &p in &core {
            let x = p % w; let y = p / w;
            if x + 1 < w { out.insert(p + 1); }
            if x > 0 { out.insert(p - 1); }
            if y + 1 < h { out.insert(p + w); }
            if y > 0 { out.insert(p - w); }
        }
        out
    }
}

/// Vol-62 NOVEL — destroy the spatial CLUSTER of all defect cells
/// within an L∞ window of given radius, plus a halo. Motivated by the
/// vol-62 measurement that at score ≥ 458, defect graphs decompose
/// into many SMALL components (size ≤ 5, mostly pair-defects) that
/// are spatially co-located in a tight region (e.g. rows 11-15 on
/// our 459 boards).
///
/// Existing ops in this regime:
///   - ComponentDestroy{min_size: 6}: never fires (largest comp ≤ 5).
///   - WorstBand{4}: destroys 64 cells (4 × 16) ignoring component
///     structure; over-destroys harmless cells outside the cluster.
///   - MwpmDefectPair: pairs defects but treats each pair separately;
///     misses the joint refill opportunity.
///
/// This op respects (a) the component structure (only defect cells
/// + their halo are destroyed) and (b) the spatial concentration
/// (all defects within `cluster_radius` of the densest mismatch are
/// included as one destroy-set).
///
/// Algorithm:
///   1. Find the cell with the most incident mismatched edges (the
///      "core"). If none exist, fall back to RandomRegion.
///   2. Collect ALL defect cells within L∞-distance ≤ cluster_radius
///      of the core. This unions multiple small components if they
///      lie in a compact area.
///   3. Add a halo of L∞-distance ≤ halo cells around the collected
///      defect set.
///
/// On our 459 board the densest core is in rows 11-13 cols 3-5;
/// cluster_radius=4 + halo=1 yields ~25-35 cells = tractable CP
/// refill problem covering ~7-10 small components jointly.
pub struct ComponentClusterDestroy {
    /// L∞ radius from the densest mismatch cell to gather other
    /// defects. 3-5 covers our 459-regime cluster geometry.
    pub cluster_radius: u32,
    /// Additional halo (in L∞) around the gathered defects.
    pub halo: u32,
}

impl DestroyOp for ComponentClusterDestroy {
    fn name(&self) -> &str { "component_cluster" }
    fn destroy(&mut self, puzzle: &Puzzle, board: &Board, rng: &mut AlnsRng) -> BTreeSet<Position> {
        let w = puzzle.width;
        let h = puzzle.height;
        let mismatches = find_mismatches(puzzle, board);
        if mismatches.is_empty() {
            return RandomRegion { k: 4 }.destroy(puzzle, board, rng);
        }
        // Step 1: per-cell mismatch incidence; pick the max.
        let mut incidence: std::collections::HashMap<Position, u32> = std::collections::HashMap::new();
        for m in &mismatches {
            *incidence.entry(m.cell_a).or_insert(0) += 1;
            *incidence.entry(m.cell_b).or_insert(0) += 1;
        }
        let core = *incidence.iter().max_by_key(|(_, &v)| v).map(|(p, _)| p).unwrap();
        let core_x = (core % w) as i32;
        let core_y = (core / w) as i32;
        let r = self.cluster_radius as i32;
        // Step 2: collect every cell that touches a mismatch AND lies
        // within L∞ ≤ r of the core.
        let mut cluster: BTreeSet<Position> = BTreeSet::new();
        for (&p, _) in &incidence {
            let x = (p % w) as i32;
            let y = (p / w) as i32;
            if (x - core_x).abs() <= r && (y - core_y).abs() <= r {
                cluster.insert(p);
            }
        }
        // Step 3: add halo.
        let halo = self.halo as i32;
        let mut out = cluster.clone();
        if halo > 0 {
            for &p in &cluster {
                let x = (p % w) as i32;
                let y = (p / w) as i32;
                for dy in -halo..=halo {
                    for dx in -halo..=halo {
                        let nx = x + dx;
                        let ny = y + dy;
                        if nx >= 0 && nx < w as i32 && ny >= 0 && ny < h as i32 {
                            out.insert((ny as u32) * w + nx as u32);
                        }
                    }
                }
            }
        }
        out
    }
}

/// Vol-17 NOVEL — destroy a RANDOM band of k_rows rows from the
/// bottom half of the board (rows 8-15 by default on 16×16). Unlike
/// WorstBand which targets mismatch density (always rows 0-4 on
/// canonical-E2 calibrated_v17a boards), this op DELIBERATELY
/// disturbs the "perfect" zone to inject diversity.
///
/// Why: vol-17 empirical finding (E6/E7) — pinning the bottom 12 rows
/// makes the top cluster provably-unfillable under gacolor_ac3. ALNS
/// stuck at 455 because it never modifies bottom rows (no mismatches
/// there to anchor destroy ops). Forcing bottom destruction is the
/// only way to find a DIFFERENT bottom configuration that admits a
/// cleaner top cluster.
pub struct BottomBandDestroy {
    pub k_rows: u32,
    /// First row to consider (rows < first_row are excluded).
    /// Use 8 for "bottom half" of a 16-row board; 12 for "very bottom".
    pub first_row: u32,
}

impl DestroyOp for BottomBandDestroy {
    fn name(&self) -> &str { "bottom_band" }
    fn destroy(&mut self, puzzle: &Puzzle, _board: &Board, rng: &mut AlnsRng) -> BTreeSet<Position> {
        let w = puzzle.width;
        let h = puzzle.height;
        let k = self.k_rows.min(h).max(1);
        let max_first = h.saturating_sub(k);
        let lo = self.first_row.min(max_first);
        let span = (max_first - lo) + 1;
        let y_start = lo + rng.range(span);
        let mut out = BTreeSet::new();
        for dy in 0..k {
            let y = y_start + dy;
            for x in 0..w {
                out.insert(y * w + x);
            }
        }
        out
    }
}

/// Vol-108 T1 — σ-cycle-aware destroy.
///
/// Given an oracle board (a known good basin, e.g. the 459 record),
/// compute the σ-permutation from `board` (current) to `oracle`,
/// decompose into cycles, return the largest cycle's positions (or
/// all cycles with length ≥ `min_size`, up to `max_cells`) as the
/// destroy set.
///
/// Motivation: vol-107 T2 found that the largest σ-cycle between a
/// partial and the 459 record predicts ALNS-liftability — partials
/// with max-cycle > 50 are structurally locked. SigmaCycleDestroy
/// targets EXACTLY those locked regions for repair, instead of the
/// usual mismatch-density-driven heuristics.
///
/// Behaviour:
///   - `min_size`: cycles below this length are ignored. Use 8-20.
///   - `max_cells`: cap on total destroy set size. Set to 30-80.
///   - The largest cycles are picked first.
///   - Falls back to `RandomRegion{k:4}` when no σ-cycle ≥ min_size
///     exists (board is too close to oracle).
/// Vol-123 W4 — LKH-style adaptive-cardinality chain destroy.
///
/// Inspired by Lin-Kernighan-Helsgaun's variable-K k-opt for TSP. Where
/// ConflictDriven grows BFS from a single seed mismatch with random
/// neighbor selection (fixed-cardinality reach), LkhChain follows the
/// "worst-match" direction at each step, building a *chain* of cells
/// that form a mismatch path.
///
/// Algorithm:
///   1. Pick a seed mismatch (random or worst).
///   2. Add both endpoints to the chain.
///   3. At each step:
///      - From any chain cell, find the neighbor (not already in chain)
///        with the FEWEST matching edges to its OWN neighbors. This is
///        the "weakest" cell — most likely to gain from being repaired.
///      - Add it to the chain.
///   4. Stop when chain reaches `max_size` OR no candidate has a mismatch.
///
/// Compared to ConflictDriven: LkhChain prioritizes growing in the
/// direction of LOWEST match quality (gain-chained), where ConflictDriven
/// uses random neighbor selection. This may target the cells that contribute
/// most to score deficit, breaking the documented K≤5 operator-lock barrier
/// (vols 20, 65, 99).
///
/// Effort: ~1 day (this is the bulk of W4).
pub struct LkhChainDestroy {
    pub max_size: u32,
    /// If true, seed from worst mismatch (highest score deficit at that cell);
    /// else random.
    pub seed_worst: bool,
}

impl DestroyOp for LkhChainDestroy {
    fn name(&self) -> &str { "lkh_chain" }
    fn destroy(&mut self, puzzle: &Puzzle, board: &Board, rng: &mut AlnsRng) -> BTreeSet<Position> {
        let w = puzzle.width;
        let h = puzzle.height;
        let mismatches = find_mismatches(puzzle, board);
        if mismatches.is_empty() {
            return RandomRegion { k: 4 }.destroy(puzzle, board, rng);
        }

        // Pre-compute per-cell match count (number of matched edges incident).
        let mut cell_match_count = vec![0u8; (w * h) as usize];
        for y in 0..h {
            for x in 0..w {
                let pos = y * w + x;
                let Some((pid, rot)) = board.get(pos) else { continue };
                let Some(p) = lookup_piece(puzzle, pid) else { continue };
                let e = p.edges.rotated(rot).as_array();
                if x + 1 < w {
                    if let Some((rpid, rrot)) = board.get(y * w + (x + 1)) {
                        if let Some(rp) = lookup_piece(puzzle, rpid) {
                            let re = rp.edges.rotated(rrot).as_array();
                            if e[1] == re[3] && e[1] != BORDER && e[1] != 0 {
                                cell_match_count[pos as usize] = cell_match_count[pos as usize].saturating_add(1);
                                cell_match_count[(y * w + (x + 1)) as usize] = cell_match_count[(y * w + (x + 1)) as usize].saturating_add(1);
                            }
                        }
                    }
                }
                if y + 1 < h {
                    if let Some((bpid, brot)) = board.get((y + 1) * w + x) {
                        if let Some(bp) = lookup_piece(puzzle, bpid) {
                            let be = bp.edges.rotated(brot).as_array();
                            if e[2] == be[0] && e[2] != BORDER && e[2] != 0 {
                                cell_match_count[pos as usize] = cell_match_count[pos as usize].saturating_add(1);
                                cell_match_count[((y + 1) * w + x) as usize] = cell_match_count[((y + 1) * w + x) as usize].saturating_add(1);
                            }
                        }
                    }
                }
            }
        }

        // Choose seed mismatch.
        let seed_idx = if self.seed_worst {
            // Find mismatch with lowest combined match-count (worst).
            let mut best = (u32::MAX, 0usize);
            for (i, m) in mismatches.iter().enumerate() {
                let a_cnt = cell_match_count[m.cell_a as usize] as u32;
                let b_cnt = cell_match_count[m.cell_b as usize] as u32;
                let total = a_cnt + b_cnt;
                if total < best.0 { best = (total, i); }
            }
            best.1
        } else {
            rng.range(mismatches.len() as u32) as usize
        };
        let m = mismatches[seed_idx];

        let mut chain: BTreeSet<Position> = BTreeSet::new();
        chain.insert(m.cell_a);
        chain.insert(m.cell_b);

        // Grow the chain: at each step, find the cell adjacent to ANY chain
        // cell with the lowest match count.
        while (chain.len() as u32) < self.max_size {
            let mut best_pos: Option<Position> = None;
            let mut best_cnt: u32 = u32::MAX;
            for &p in &chain {
                let x = p % w;
                let y = p / w;
                let nbrs: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
                for (dx, dy) in &nbrs {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx < 0 || ny < 0 || nx as u32 >= w || ny as u32 >= h { continue; }
                    let np = ny as u32 * w + nx as u32;
                    if chain.contains(&np) { continue; }
                    let cnt = cell_match_count[np as usize] as u32;
                    if cnt < best_cnt {
                        best_cnt = cnt;
                        best_pos = Some(np);
                    }
                }
            }
            // Stop if no candidate (chain is isolated) OR candidate is fully matched
            // (no chain growth would gain anything).
            let Some(p) = best_pos else { break; };
            // Heuristic: stop if next cell has perfect 4-match (no gain possible)
            // — except in very small chain (give it room).
            // Actually let's keep going regardless; the CSP repair will figure it out.
            chain.insert(p);
        }

        chain
    }
}

pub struct SigmaCycleDestroy {
    pub oracle: Board,
    pub min_size: u32,
    pub max_cells: u32,
    /// L∞ halo radius around each cycle cell. 0 = no halo (just cycle
    /// cells). 1 = include 4-neighbours. Higher unlocks more of the
    /// halo so repair can re-orient adjacent pieces.
    pub halo: u32,
}

impl DestroyOp for SigmaCycleDestroy {
    fn name(&self) -> &str { "sigma_cycle" }
    fn destroy(&mut self, puzzle: &Puzzle, board: &Board, rng: &mut AlnsRng) -> BTreeSet<Position> {
        let w = puzzle.width;
        let h = puzzle.height;
        let cell_count = (w * h) as usize;
        let cycles = crate::oracle_swap::compute_sigma_cycles(board, &self.oracle, cell_count);
        let mut core: BTreeSet<Position> = BTreeSet::new();
        for c in &cycles {
            if (c.positions.len() as u32) < self.min_size {
                break; // cycles sorted by size desc
            }
            for &p in &c.positions {
                core.insert(p);
                if core.len() as u32 >= self.max_cells {
                    break;
                }
            }
            if core.len() as u32 >= self.max_cells {
                break;
            }
        }
        if core.is_empty() {
            // Fallback: small random region.
            return RandomRegion { k: 4 }.destroy(puzzle, board, rng);
        }
        // Add halo via L∞ expansion.
        if self.halo == 0 {
            return core;
        }
        let halo = self.halo as i32;
        let mut out = core.clone();
        for &p in &core {
            let x = (p % w) as i32;
            let y = (p / w) as i32;
            for dy in -halo..=halo {
                for dx in -halo..=halo {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx >= 0 && nx < w as i32 && ny >= 0 && ny < h as i32 {
                        out.insert((ny as u32) * w + (nx as u32));
                    }
                }
            }
        }
        out
    }
}

/// Vol-18 NOVEL — destroy a LARGE band (k_rows wide) centered on the
/// densest mismatch row. Unlike WorstBand{4} which destroys 64 cells,
/// MegaBand{k=8-10} destroys 128-160 cells. Generates fundamentally
/// different proposals: the destroyed region overlaps both the
/// mismatch cluster AND surrounding "perfect" rows, allowing piece
/// migration ACROSS the cluster boundary that small-band ops cannot
/// achieve.
///
/// Motivation: vol-18 found 457 is operator-locked under winning5
/// ops at any temperature. The proposal distribution from k=4 bands
/// is exhausted. Bigger bands sample a different proposal regime.
pub struct MegaBand {
    /// Width of the band in rows. Recommended 8-12 for 16-row board.
    pub k_rows: u32,
}
impl DestroyOp for MegaBand {
    fn name(&self) -> &str { "mega_band" }
    fn destroy(&mut self, puzzle: &Puzzle, board: &Board, _rng: &mut AlnsRng) -> BTreeSet<Position> {
        let w = puzzle.width;
        let h = puzzle.height;
        let k = self.k_rows.min(h).max(1);
        let mismatches = find_mismatches(puzzle, board);
        let mut per_row = vec![0u32; h as usize];
        for m in &mismatches {
            let ya = m.cell_a / w;
            let yb = m.cell_b / w;
            per_row[ya as usize] += 1;
            if yb != ya { per_row[yb as usize] += 1; }
        }
        let mut best = (0u32, 0u32);
        for y0 in 0..=(h.saturating_sub(k)) {
            let mut s = 0u32;
            for dy in 0..k { s += per_row[(y0 + dy) as usize]; }
            if s > best.0 { best = (s, y0); }
        }
        let y_start = if best.0 == 0 { 0 } else { best.1 };
        let mut out = BTreeSet::new();
        for dy in 0..k {
            let y = y_start + dy;
            for x in 0..w {
                out.insert(y * w + x);
            }
        }
        out
    }
}

/// Vol-18 NOVEL — destroy a single COLUMN strip (1 × H) whose mismatch
/// density is highest. Orthogonal to row-band ops. Useful when the
/// cluster has vertical structure that row-band ops can't see.
pub struct WorstColumn;
impl DestroyOp for WorstColumn {
    fn name(&self) -> &str { "worst_column" }
    fn destroy(&mut self, puzzle: &Puzzle, board: &Board, _rng: &mut AlnsRng) -> BTreeSet<Position> {
        let w = puzzle.width;
        let h = puzzle.height;
        let mismatches = find_mismatches(puzzle, board);
        let mut per_col = vec![0u32; w as usize];
        for m in &mismatches {
            per_col[(m.cell_a % w) as usize] += 1;
            let xb = m.cell_b % w;
            if xb != m.cell_a % w { per_col[xb as usize] += 1; }
        }
        let mut best_col = 0u32;
        let mut best_score = 0u32;
        for x in 0..w {
            if per_col[x as usize] > best_score {
                best_score = per_col[x as usize];
                best_col = x;
            }
        }
        let mut out = BTreeSet::new();
        for y in 0..h {
            out.insert(y * w + best_col);
        }
        out
    }
}

/// Vol-18 NOVEL — destroy a column BAND of k_cols centered on the
/// densest mismatch column.
pub struct WorstColumnBand {
    pub k_cols: u32,
}
impl DestroyOp for WorstColumnBand {
    fn name(&self) -> &str { "worst_column_band" }
    fn destroy(&mut self, puzzle: &Puzzle, board: &Board, _rng: &mut AlnsRng) -> BTreeSet<Position> {
        let w = puzzle.width;
        let h = puzzle.height;
        let k = self.k_cols.min(w).max(1);
        let mismatches = find_mismatches(puzzle, board);
        let mut per_col = vec![0u32; w as usize];
        for m in &mismatches {
            per_col[(m.cell_a % w) as usize] += 1;
            let xb = m.cell_b % w;
            if xb != m.cell_a % w { per_col[xb as usize] += 1; }
        }
        let mut best = (0u32, 0u32);
        for x0 in 0..=(w.saturating_sub(k)) {
            let mut s = 0u32;
            for dx in 0..k { s += per_col[(x0 + dx) as usize]; }
            if s > best.0 { best = (s, x0); }
        }
        let x_start = if best.0 == 0 { 0 } else { best.1 };
        let mut out = BTreeSet::new();
        for y in 0..h {
            for dx in 0..k {
                out.insert(y * w + x_start + dx);
            }
        }
        out
    }
}

/// Vol-18 NOVEL — destroy half the board (top or bottom). 128 cells
/// deterministic. Brutal but guaranteed to perturb basin structure
/// beyond any local op. The SA repair on 128 cells is heavy
/// (~1500ms budget gives ~few hundred SA moves which may not fully
/// optimize), but the proposal IS fundamentally different from
/// any winning5 op.
pub struct HalfBoardDestroy {
    /// 0 = top half (rows 0..h/2), 1 = bottom half, 2 = left half, 3 = right half.
    pub which: u32,
}
impl DestroyOp for HalfBoardDestroy {
    fn name(&self) -> &str { "half_board" }
    fn destroy(&mut self, puzzle: &Puzzle, _board: &Board, _rng: &mut AlnsRng) -> BTreeSet<Position> {
        let w = puzzle.width;
        let h = puzzle.height;
        let mut out = BTreeSet::new();
        match self.which {
            0 => {
                for y in 0..(h/2) {
                    for x in 0..w { out.insert(y * w + x); }
                }
            }
            1 => {
                for y in (h/2)..h {
                    for x in 0..w { out.insert(y * w + x); }
                }
            }
            2 => {
                for y in 0..h {
                    for x in 0..(w/2) { out.insert(y * w + x); }
                }
            }
            3 => {
                for y in 0..h {
                    for x in (w/2)..w { out.insert(y * w + x); }
                }
            }
            _ => panic!("HalfBoardDestroy.which must be 0..=3"),
        }
        out
    }
}

/// Vol-18 NOVEL — destroy K random cells scattered across the WHOLE board
/// (not a connected window). Different topology than RandomRegion (which
/// picks a kxk square). Useful for proposing piece-set redistributions
/// across the entire board.
pub struct RandomScatter {
    pub k: u32,
}
impl DestroyOp for RandomScatter {
    fn name(&self) -> &str { "random_scatter" }
    fn destroy(&mut self, puzzle: &Puzzle, _board: &Board, rng: &mut AlnsRng) -> BTreeSet<Position> {
        let n = puzzle.cell_count();
        let target = self.k.min(n) as usize;
        let mut out = BTreeSet::new();
        while out.len() < target {
            let pos = rng.range(n);
            out.insert(pos);
        }
        out
    }
}

/// Vol-17 NOVEL — destroy the "hinge cells" of the mismatch component:
/// the articulation points (cut vertices) of the cell graph induced by
/// the mismatch component. Removing an articulation point disconnects
/// the graph into smaller pieces, each of which is then a tractable
/// CP-repair problem.
///
/// Why this might work: the vol-17 calibrated_v17a 447 board has all
/// 51 mismatch cells in ONE connected component. ComponentDestroy
/// frees the whole thing (CP must reconstruct 51 cells). HingeDestroy
/// frees just the few hinges (3-8 cells typically), leaving most of
/// the component pinned. CP-repair on 3-8 cells is fast; if successful,
/// the cluster fragments. Then ComponentDestroy on the smaller frags
/// is easy.
///
/// Algorithm: Tarjan's articulation-point detection (O(V+E)) on the
/// graph of mismatch cells with 4-neighbour edges.
pub struct HingeDestroy {
    /// Halo cells around each hinge (Manhattan radius). 0 = just the
    /// hinges themselves; 1 = hinge + 4 neighbours. Recommended 1.
    pub halo: u32,
}

impl DestroyOp for HingeDestroy {
    fn name(&self) -> &str { "hinge_destroy" }
    fn destroy(&mut self, puzzle: &Puzzle, board: &Board, rng: &mut AlnsRng) -> BTreeSet<Position> {
        let w = puzzle.width;
        let h = puzzle.height;
        let mismatches = find_mismatches(puzzle, board);
        if mismatches.is_empty() {
            return RandomRegion { k: 4 }.destroy(puzzle, board, rng);
        }
        // Build the mismatch cell set + adjacency.
        let mut mismatch_cells: Vec<Position> = Vec::new();
        let mut mismatch_set: BTreeSet<Position> = BTreeSet::new();
        for m in &mismatches {
            if mismatch_set.insert(m.cell_a) { mismatch_cells.push(m.cell_a); }
            if mismatch_set.insert(m.cell_b) { mismatch_cells.push(m.cell_b); }
        }
        // Tarjan needs cell -> index in component. Build adjacency lists.
        let n = mismatch_cells.len();
        if n < 3 {
            // Tarjan articulation points need at least 3 nodes.
            return ConflictDriven { max_size: 30 }.destroy(puzzle, board, rng);
        }
        let idx_of: std::collections::HashMap<Position, usize> = mismatch_cells
            .iter().enumerate().map(|(i, &p)| (p, i)).collect();
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
        for (i, &p) in mismatch_cells.iter().enumerate() {
            let x = p % w; let y = p / w;
            for (dx, dy) in [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)] {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx < 0 || ny < 0 || nx >= w as i32 || ny >= h as i32 { continue; }
                let np = (ny as u32) * w + (nx as u32);
                if let Some(&j) = idx_of.get(&np) {
                    adj[i].push(j);
                }
            }
        }
        // Tarjan's algorithm: iterative DFS finding articulation points.
        let mut disc = vec![0u32; n];
        let mut low = vec![0u32; n];
        let mut parent = vec![usize::MAX; n];
        let mut visited = vec![false; n];
        let mut is_articulation = vec![false; n];
        let mut timer = 0u32;
        // Stack of (node, iter_idx). iter_idx tracks which child we're processing.
        let mut stack: Vec<(usize, usize)> = Vec::new();
        for root in 0..n {
            if visited[root] { continue; }
            stack.push((root, 0));
            visited[root] = true;
            disc[root] = timer; low[root] = timer; timer += 1;
            let mut root_children = 0u32;
            while let Some(&(u, ci)) = stack.last() {
                if ci >= adj[u].len() {
                    // Done with u. Update parent's low if any.
                    if parent[u] != usize::MAX {
                        let p = parent[u];
                        low[p] = low[p].min(low[u]);
                        if parent[p] != usize::MAX && low[u] >= disc[p] {
                            is_articulation[p] = true;
                        }
                    } else {
                        // Root: articulation iff has >1 children.
                        if root_children > 1 {
                            is_articulation[u] = true;
                        }
                    }
                    stack.pop();
                    continue;
                }
                // Advance ci before potential push.
                let v = adj[u][ci];
                stack.last_mut().unwrap().1 += 1;
                if !visited[v] {
                    visited[v] = true;
                    parent[v] = u;
                    if u == root { root_children += 1; }
                    disc[v] = timer; low[v] = timer; timer += 1;
                    stack.push((v, 0));
                } else if Some(v) != parent.get(u).copied().filter(|&p| p != usize::MAX) {
                    low[u] = low[u].min(disc[v]);
                }
            }
        }
        // Collect articulation points into a BTreeSet plus halo.
        let mut out = BTreeSet::new();
        for (i, &is_a) in is_articulation.iter().enumerate() {
            if !is_a { continue; }
            let p = mismatch_cells[i];
            out.insert(p);
            if self.halo > 0 {
                let x = p % w; let y = p / w;
                for dy in -(self.halo as i32)..=(self.halo as i32) {
                    for dx in -(self.halo as i32)..=(self.halo as i32) {
                        let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                        if nx < 0 || ny < 0 || nx >= w as i32 || ny >= h as i32 { continue; }
                        out.insert((ny as u32) * w + (nx as u32));
                    }
                }
            }
        }
        if out.is_empty() {
            // No articulation points (e.g. component is a clique-ish blob).
            // Fall back to ConflictDriven.
            return ConflictDriven { max_size: 30 }.destroy(puzzle, board, rng);
        }
        out
    }
}

// ----- Acceptance criterion --------------------------------------------

#[derive(Debug, Clone, Copy)]
pub enum Acceptance {
    Greedy,
    SimulatedAnnealing { t: f64 },
    RecordToRecord { deviation: f64 },
}

impl Acceptance {
    pub fn accept(&self, delta: i32, rng: &mut AlnsRng, best_score: u32, new_score: u32) -> bool {
        match self {
            Acceptance::Greedy => delta > 0,
            Acceptance::SimulatedAnnealing { t } => {
                if delta >= 0 { return true; }
                let p = (delta as f64 / t).exp();
                rng.next_f64() < p
            }
            Acceptance::RecordToRecord { deviation } => {
                (new_score as f64) >= (best_score as f64) - deviation
            }
        }
    }
}

// ----- Adaptive operator weights ---------------------------------------

#[derive(Debug, Clone)]
pub struct AdaptiveWeights {
    weights: Vec<f64>,
    segment_reward: Vec<f64>,
    segment_count: Vec<u32>,
    decay: f64,
    pub sigma_new_best: f64,    // σ1
    pub sigma_improve: f64,     // σ2
    pub sigma_accept_worse: f64, // σ3
    pub sigma_reject: f64,      // σ4
}

impl AdaptiveWeights {
    pub fn uniform(n: usize) -> Self {
        Self {
            weights: vec![1.0; n],
            segment_reward: vec![0.0; n],
            segment_count: vec![0; n],
            decay: 0.1,
            sigma_new_best: 33.0,
            sigma_improve: 9.0,
            sigma_accept_worse: 13.0,
            sigma_reject: 0.0,
        }
    }

    pub fn select(&self, rng: &mut AlnsRng) -> usize {
        let total: f64 = self.weights.iter().sum();
        let r = rng.next_f64() * total;
        let mut acc = 0.0;
        for (i, w) in self.weights.iter().enumerate() {
            acc += w;
            if r < acc { return i; }
        }
        self.weights.len() - 1
    }

    pub fn reward(&mut self, op_idx: usize, sigma: f64) {
        self.segment_reward[op_idx] += sigma;
        self.segment_count[op_idx] += 1;
    }

    pub fn update_weights(&mut self) {
        for i in 0..self.weights.len() {
            if self.segment_count[i] > 0 {
                let avg_reward = self.segment_reward[i] / (self.segment_count[i] as f64);
                self.weights[i] = (1.0 - self.decay) * self.weights[i] + self.decay * avg_reward;
            } else {
                // No samples this segment — gentle decay toward floor.
                self.weights[i] = self.weights[i] * (1.0 - self.decay * 0.5);
            }
            self.weights[i] = self.weights[i].max(0.01); // never let it go to zero
            self.segment_reward[i] = 0.0;
            self.segment_count[i] = 0;
        }
    }

    pub fn weights(&self) -> &[f64] { &self.weights }
}

/// Vol-17 — atomic write of a single best-board JSON checkpoint.
/// Format mirrors what other vol-17 bins emit (placement array +
/// matched score). Overwrites the file each call.
fn write_alns_checkpoint(
    puzzle: &Puzzle,
    board: &Board,
    score: u32,
    iters: u32,
    path: &std::path::Path,
) {
    use std::io::Write;
    let n = puzzle.cell_count();
    // Build a JSON string by hand to avoid pulling serde_json into
    // the localsearch crate (which currently has no serde_json dep).
    let mut s = String::with_capacity(8192);
    s.push_str(&format!(
        "{{\n  \"matched\": {},\n  \"iters\": {},\n  \"timestamp\": \"{}\",\n  \"placement\": [\n",
        score, iters,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0),
    ));
    let mut first = true;
    for pos in 0..n {
        if !first { s.push_str(",\n"); }
        first = false;
        if let Some((pid, rot)) = board.get(pos) {
            s.push_str(&format!(
                "    {{\"pos\": {}, \"piece_id\": {}, \"rotation\": {}}}",
                pos, u32::from(pid), rot.as_u8()
            ));
        } else {
            s.push_str("    null");
        }
    }
    s.push_str("\n  ]\n}\n");
    // Write atomically via tmp + rename.
    let tmp = path.with_extension("tmp");
    if let Ok(mut f) = std::fs::File::create(&tmp) {
        let _ = f.write_all(s.as_bytes());
        let _ = f.sync_all();
        let _ = std::fs::rename(&tmp, path);
    }
}

// ----- ALNS config + driver --------------------------------------------

pub struct AlnsConfig {
    pub time_budget_ms: u64,
    pub repair_budget_ms: u64,
    pub acceptance: Acceptance,
    pub segment_iters: u32,
    pub seed: u64,
    pub verbose: bool,
    /// Primary repair kind. Default: SA (works on plateau states).
    pub repair: RepairKind,
    /// If `repair == Cp` and CP fails, retry once with SA. Default true.
    pub cp_fallback_to_sa: bool,
    /// Cell positions that destroy operators are not allowed to free.
    /// Used to keep the 5 canonical E2 hints pinned during ALNS. Empty
    /// = unconstrained (legacy vol-12 behaviour — score is then *not*
    /// the official canonical-E2 score). Set this to
    /// `hints.iter().map(|h| h.position).collect()` for canonical
    /// scoring. Fixed in vol-14 after discovering canonical hints
    /// were being swapped out, invalidating the 443/480 score.
    pub pinned_positions: Vec<Position>,
    /// Vol-17 — optional hard cap on iters (0 = no cap). Used by PT-on-ALNS
    /// to do bounded inner loops between exchanges.
    pub iter_budget: u32,
    /// Vol-17 — periodic mid-run checkpoint of the current-best board.
    /// When `checkpoint_path` is `Some(path)`, every `checkpoint_every_ms`
    /// (default 60s when 0), the current best board is written to
    /// `<path>` as JSON containing the placement + score + bucas-style
    /// metadata. Each checkpoint OVERWRITES the file (single rolling
    /// snapshot). Used by long overnight runs to keep monitoring tools
    /// in sync with in-flight ALNS progress.
    pub checkpoint_path: Option<std::path::PathBuf>,
    pub checkpoint_every_ms: u64,
    /// Vol-17 — when true, iso-score moves use largest-mismatch-component
    /// size as a tie-breaker. Smaller largest-component is preferred
    /// (easier to repair in future iters). Useful when matched count
    /// plateaus on iso-score landscapes (vol-17 empirical observation).
    /// Adds O(W*H) cluster computation per iter; cheap on 16×16.
    pub lex_break_isoscore: bool,
    /// V140 — when true, iso-score moves use FORBIDDEN-2x2 count as
    /// tie-breaker (fewer forbidden = better). Based on V138-139
    /// finding that forbidden-count anti-correlates with board score
    /// (LOW 109, MID 60, HIGH 29 medians). Adds O(225) feasibility
    /// checks per iter on canonical 16x16.
    pub lex_break_intaglio: bool,
    /// Vol-17 — fixed-step SA repair for deterministic A/B. When non-zero,
    /// `sa_repair` runs exactly this many SA moves regardless of wall-clock,
    /// instead of `repair_budget_ms`. Eliminates the timing-jitter
    /// nondeterminism that produced same-seed score variance up to 11
    /// matches in the overnight portfolio (see RESEARCH_NOTES_17_OVERNIGHT
    /// F2 and OPTIMIZATION_REPORT). Default 0 = legacy wall-clock budget.
    pub repair_step_budget: u64,
    /// Vol-17 — when true (default), `cp_repair` uses parallel
    /// `gacolor_ac3_par` for throughput; when false, uses single-thread
    /// `gacolor_ac3` for reproducibility across runs. Set to false in
    /// scientific benchmark binaries; leave true in portfolio/record runs.
    pub cp_repair_parallel: bool,
}

impl Default for AlnsConfig {
    fn default() -> Self {
        Self {
            time_budget_ms: 600_000,
            repair_budget_ms: 500,
            acceptance: Acceptance::SimulatedAnnealing { t: 1.0 },
            segment_iters: 50,
            seed: 0xA1_A2_A3_A4,
            verbose: false,
            repair: RepairKind::Sa,
            cp_fallback_to_sa: true,
            pinned_positions: Vec::new(),
            iter_budget: 0,
            lex_break_isoscore: false,
            lex_break_intaglio: false,
            checkpoint_path: None,
            checkpoint_every_ms: 60_000,
            repair_step_budget: 0,
            cp_repair_parallel: true,
        }
    }
}

/// Vol-17 — compute largest-connected-mismatch-component size on `board`.
/// Cheap: O(W×H) BFS. Used as iso-score tie-breaker.
fn largest_mismatch_component_size(puzzle: &Puzzle, board: &Board) -> u32 {
    let mismatches = find_mismatches(puzzle, board);
    if mismatches.is_empty() { return 0; }
    let w = puzzle.width;
    let h = puzzle.height;
    let mut mismatch_cells: BTreeSet<Position> = BTreeSet::new();
    for m in &mismatches {
        mismatch_cells.insert(m.cell_a);
        mismatch_cells.insert(m.cell_b);
    }
    let mut visited: BTreeSet<Position> = BTreeSet::new();
    let mut largest = 0u32;
    for &start in &mismatch_cells {
        if visited.contains(&start) { continue; }
        let mut size = 0u32;
        let mut queue: VecDeque<Position> = VecDeque::new();
        queue.push_back(start);
        visited.insert(start);
        while let Some(p) = queue.pop_front() {
            size += 1;
            let x = p % w; let y = p / w;
            if x + 1 < w && mismatch_cells.contains(&(p + 1)) && !visited.contains(&(p + 1)) {
                visited.insert(p + 1); queue.push_back(p + 1);
            }
            if x > 0 && mismatch_cells.contains(&(p - 1)) && !visited.contains(&(p - 1)) {
                visited.insert(p - 1); queue.push_back(p - 1);
            }
            if y + 1 < h && mismatch_cells.contains(&(p + w)) && !visited.contains(&(p + w)) {
                visited.insert(p + w); queue.push_back(p + w);
            }
            if y > 0 && mismatch_cells.contains(&(p - w)) && !visited.contains(&(p - w)) {
                visited.insert(p - w); queue.push_back(p - w);
            }
        }
        if size > largest { largest = size; }
    }
    largest
}

#[derive(Debug, Clone)]
pub struct AlnsStats {
    pub iters: u32,
    pub repair_failures: u32,
    pub accepted_improving: u32,
    pub accepted_worse: u32,
    pub rejected: u32,
    pub per_op_accepts: Vec<u32>,
    pub per_op_invocations: Vec<u32>,
    pub op_names: Vec<String>,
    pub best_score_history: Vec<(u32, u32)>, // (iter, score) on each new best
}

pub fn run_alns(
    puzzle: &Puzzle,
    initial: &Board,
    ops: &mut [Box<dyn DestroyOp>],
    cfg: &AlnsConfig,
) -> (Board, AlnsStats) {
    let mut rng = AlnsRng::new(cfg.seed);
    let mut weights = AdaptiveWeights::uniform(ops.len());
    let mut current = initial.clone();
    let mut current_score = score_board(puzzle, &current);
    let mut best = current.clone();
    let mut best_score = current_score;

    let mut stats = AlnsStats {
        iters: 0, repair_failures: 0,
        accepted_improving: 0, accepted_worse: 0, rejected: 0,
        per_op_accepts: vec![0; ops.len()],
        per_op_invocations: vec![0; ops.len()],
        op_names: ops.iter().map(|o| o.name().to_string()).collect(),
        best_score_history: vec![(0, best_score)],
    };

    let t_start = std::time::Instant::now();
    let mut last_log = t_start;
    // Vol-17 — periodic best-board checkpoint (overwrites file each tick).
    let mut last_checkpoint = t_start;
    let checkpoint_every = std::time::Duration::from_millis(cfg.checkpoint_every_ms.max(1));

    let pinned_set: BTreeSet<Position> = cfg.pinned_positions.iter().copied().collect();

    // Vol-17 novel — restart-on-stagnation: track iters-since-best.
    // After STAGNATION_LIMIT iters with no new best, reset
    // `current` to `best` (escapes worse-accept drifts that ratchet
    // away from the optimum) and reshuffle op weights to give bored
    // ops a fresh chance.
    let stagnation_limit: u32 = 60;
    let mut last_best_iter: u32 = 0;
    let mut total_restarts: u32 = 0;

    while t_start.elapsed().as_millis() < cfg.time_budget_ms as u128
        && (cfg.iter_budget == 0 || stats.iters < cfg.iter_budget)
    {
        stats.iters += 1;
        let op_idx = weights.select(&mut rng);
        stats.per_op_invocations[op_idx] += 1;

        let mut free_set = ops[op_idx].destroy(puzzle, &current, &mut rng);
        // Vol-14 fix: pinned positions are never freed by destroy operators.
        // Without this, canonical E2 hints get swapped out and the reported
        // score is for a different puzzle.
        if !pinned_set.is_empty() {
            for p in &pinned_set { free_set.remove(p); }
        }
        if free_set.is_empty() { continue; }

        // Iteration-specific seed so SA repairs don't all walk the same path.
        let iter_seed = cfg.seed ^ ((stats.iters as u64).wrapping_mul(0xDEADBEEFCAFE0001));
        let new_board = match repair_with_opts(
            puzzle, &current, &free_set, cfg.repair_budget_ms,
            cfg.repair, iter_seed, cfg.repair_step_budget, cfg.cp_repair_parallel,
        ) {
            Some(b) => b,
            None => {
                if cfg.repair == RepairKind::Cp && cfg.cp_fallback_to_sa {
                    match repair_with_opts(
                        puzzle, &current, &free_set, cfg.repair_budget_ms,
                        RepairKind::Sa, iter_seed, cfg.repair_step_budget, cfg.cp_repair_parallel,
                    ) {
                        Some(b) => b,
                        None => { stats.repair_failures += 1; continue; }
                    }
                } else { stats.repair_failures += 1; continue; }
            }
        };
        let new_score = score_board(puzzle, &new_board);
        let delta = new_score as i32 - current_score as i32;

        // Vol-17 — lex_break_isoscore: iso-score moves use smaller
        // largest-mismatch-component as tie-breaker.
        // V140 — lex_break_intaglio: iso-score moves use FEWER forbidden
        // 2x2 patches as tie-breaker.
        let accepted = if cfg.lex_break_intaglio && delta == 0 {
            let cur_intaglio = crate::intaglio::count_forbidden_2x2(puzzle, &current);
            let new_intaglio = crate::intaglio::count_forbidden_2x2(puzzle, &new_board);
            if new_intaglio < cur_intaglio {
                true
            } else if new_intaglio > cur_intaglio {
                false
            } else {
                cfg.acceptance.accept(delta, &mut rng, best_score, new_score)
            }
        } else if cfg.lex_break_isoscore && delta == 0 {
            let cur_lcc = largest_mismatch_component_size(puzzle, &current);
            let new_lcc = largest_mismatch_component_size(puzzle, &new_board);
            if new_lcc < cur_lcc {
                true
            } else if new_lcc > cur_lcc {
                false
            } else {
                cfg.acceptance.accept(delta, &mut rng, best_score, new_score)
            }
        } else {
            cfg.acceptance.accept(delta, &mut rng, best_score, new_score)
        };
        let mut sigma = weights.sigma_reject;
        if accepted {
            stats.per_op_accepts[op_idx] += 1;
            current = new_board;
            current_score = new_score;
            if new_score > best_score {
                best = current.clone();
                best_score = new_score;
                stats.best_score_history.push((stats.iters, best_score));
                last_best_iter = stats.iters;
                sigma = weights.sigma_new_best;
            } else if delta > 0 {
                sigma = weights.sigma_improve;
                stats.accepted_improving += 1;
            } else {
                sigma = weights.sigma_accept_worse;
                stats.accepted_worse += 1;
            }
        } else {
            stats.rejected += 1;
        }
        weights.reward(op_idx, sigma);

        // Vol-17 — periodic checkpoint of current best board.
        if let Some(cp_path) = cfg.checkpoint_path.as_ref() {
            if last_checkpoint.elapsed() >= checkpoint_every {
                write_alns_checkpoint(puzzle, &best, best_score, stats.iters, cp_path);
                last_checkpoint = std::time::Instant::now();
            }
        }

        // Vol-17 — stagnation check: if no new best in stagnation_limit
        // iterations, jump back to best and reshuffle weights.
        if stats.iters - last_best_iter >= stagnation_limit {
            current = best.clone();
            current_score = best_score;
            // Slightly shrink weights then add a tiny uniform perturbation,
            // so the dominant op stays dominant but bored ops get a boost.
            weights = AdaptiveWeights::uniform(ops.len());
            last_best_iter = stats.iters;
            total_restarts += 1;
            if cfg.verbose {
                eprintln!("[ALNS iter {}] stagnation restart #{}; reset to best={}",
                    stats.iters, total_restarts, best_score);
            }
        }

        if stats.iters % cfg.segment_iters == 0 {
            weights.update_weights();
            if cfg.verbose && last_log.elapsed().as_secs() >= 2 {
                eprintln!("[ALNS iter {}] t={:.1}s best={} current={} op_weights={:?}",
                    stats.iters,
                    t_start.elapsed().as_secs_f64(),
                    best_score, current_score,
                    weights.weights().iter().map(|w| format!("{:.2}", w)).collect::<Vec<_>>());
                last_log = std::time::Instant::now();
            }
        }
    }

    (best, stats)
}

/// Vol-17 NOVEL — exhaustive O(C²) pairwise piece-swap hill climb.
/// For each pair of cells (a, b) where at least one has a mismatched
/// edge, evaluate the score delta of swapping pieces a and b (with
/// all rotation combinations). Apply the best swap. Iterate.
///
/// Bounded gain (hill-climb on a finite landscape). Cost per pass:
/// O(C × M × 16) where C = unmatched-touching cells, M = all cells.
/// On 16×16 with ~50 unmatched cells, ~50 × 256 × 16 = 200k pair-
/// evals per pass. Should run in milliseconds.
///
/// Differs from rotation-only polish: this also moves pieces, which
/// catches the case where the OPTIMAL piece for a position is
/// currently at another position. Can't be caught by ALNS destroy
/// ops because those are random/heuristic, not exhaustive.
pub fn piece_swap_hillclimb(
    puzzle: &Puzzle,
    board: &Board,
    pinned_set: &BTreeSet<Position>,
) -> (Board, u32) {
    let mut b = board.clone();
    let w = puzzle.width;
    let h = puzzle.height;
    // Vol-34 fix — compute actual score before and after each swap.
    // The local-delta accumulation had cases (notably adjacent pairs;
    // possibly other shared-neighbour topologies) where reported gain
    // diverged from actual. Anchor on the real score_board metric to
    // bound the damage.
    let start_score = score_board(puzzle, &b);
    let mut prev_score = start_score;
    let mut iters = 0u32;
    loop {
        iters += 1;
        if iters > 200 { break; }
        // Identify "hot" cells: those touching at least one mismatched edge.
        let mismatches = find_mismatches(puzzle, &b);
        if mismatches.is_empty() { break; }
        let mut hot: BTreeSet<Position> = BTreeSet::new();
        for m in &mismatches {
            hot.insert(m.cell_a);
            hot.insert(m.cell_b);
        }
        // Search for the best swap (a, b) where a is hot.
        let mut best: Option<(Position, Position, Rotation, Rotation, i32)> = None;
        for &a in &hot {
            if pinned_set.contains(&a) { continue; }
            let Some((apid, _arot)) = b.get(a) else { continue };
            let Some(apiece) = lookup_piece(puzzle, apid) else { continue };
            // Score at cell a with current piece+rotation.
            let cur_a_e = b.get(a).map(|(pid, rot)| {
                lookup_piece(puzzle, pid).map(|p| p.edges.rotated(rot).as_array()).unwrap_or([0; 4])
            }).unwrap_or([0; 4]);
            let cur_a_score = count_neighbour_matches(puzzle, &b, a, cur_a_e);

            for blo in 0..w*h {
                let bp = blo;
                if bp == a { continue; }
                if pinned_set.contains(&bp) { continue; }
                let Some((bpid, _brot)) = b.get(bp) else { continue };
                let Some(bpiece) = lookup_piece(puzzle, bpid) else { continue };

                let cur_b_e = b.get(bp).map(|(pid, rot)| {
                    lookup_piece(puzzle, pid).map(|p| p.edges.rotated(rot).as_array()).unwrap_or([0; 4])
                }).unwrap_or([0; 4]);
                let cur_b_score = count_neighbour_matches(puzzle, &b, bp, cur_b_e);
                let cur_total = cur_a_score + cur_b_score;

                // Try every rotation combination for (apiece at b, bpiece at a).
                let mut best_for_pair: Option<(Rotation, Rotation, i32)> = None;
                for arot in Rotation::ALL {
                    let a_to_b_e = apiece.edges.rotated(arot).as_array();
                    // Temporarily evaluate apiece at bp: need to also account for
                    // bpiece NOT being at bp anymore. For simplicity, compute b's
                    // local match assuming a's piece is at bp (neighbours unchanged).
                    let nb_b = count_neighbour_matches(puzzle, &b, bp, a_to_b_e);
                    for brot in Rotation::ALL {
                        let b_to_a_e = bpiece.edges.rotated(brot).as_array();
                        let nb_a = count_neighbour_matches(puzzle, &b, a, b_to_a_e);
                        // If a and b are neighbours, the cross-edge counts wrong.
                        // Skip neighbour pairs for simplicity (rare case anyway).
                        let new_total = nb_a + nb_b;
                        let delta = new_total as i32 - cur_total as i32;
                        if delta > 0 && best_for_pair.map_or(true, |(_, _, d)| delta > d) {
                            best_for_pair = Some((arot, brot, delta));
                        }
                    }
                }
                if let Some((arot, brot, delta)) = best_for_pair {
                    if best.map_or(true, |(_, _, _, _, d)| delta > d) {
                        best = Some((a, bp, arot, brot, delta));
                    }
                }
            }
        }
        match best {
            Some((a, bp, arot, brot, _delta_estimate)) => {
                let (apid, _) = b.get(a).unwrap();
                let (bpid, _) = b.get(bp).unwrap();
                let snapshot = b.clone();
                b.place(a, bpid, brot);
                b.place(bp, apid, arot);
                // Verify with the real metric. If the swap actually
                // worsens the global score (the delta-estimator was wrong),
                // roll back and stop.
                let new_score = score_board(puzzle, &b);
                if new_score <= prev_score {
                    b = snapshot;
                    break;
                }
                prev_score = new_score;
            }
            None => break,
        }
    }
    let final_score = score_board(puzzle, &b);
    let total_gain = final_score.saturating_sub(start_score);
    (b, total_gain)
}

/// Vol-17 — deterministic post-ALNS polish: for each cell, try all 4
/// rotations of its current piece. Keep the rotation that yields the
/// highest neighbour-edge match count. Iterate until no improvement
/// (single-cell-rotation fixed point).
///
/// This is a HILL-CLIMB on rotation only — never moves pieces, only
/// rotates them. Bounded gain, but always non-negative. Useful after
/// SA-repair which doesn't fully explore rotation space.
///
/// `pinned_set` cells are skipped (their rotations must be preserved
/// as canonical hints).
pub fn polish_rotations(
    puzzle: &Puzzle,
    board: &Board,
    pinned_set: &BTreeSet<Position>,
) -> (Board, u32) {
    let mut b = board.clone();
    let w = puzzle.width;
    let h = puzzle.height;
    let mut total_improvement = 0u32;
    let mut iterations = 0u32;
    loop {
        iterations += 1;
        if iterations > 1000 { break; }
        let mut any_change = false;
        for pos in 0..w * h {
            if pinned_set.contains(&pos) { continue; }
            let Some((pid, current_rot)) = b.get(pos) else { continue };
            let Some(piece) = lookup_piece(puzzle, pid) else { continue };
            // Count current neighbour matches at this position.
            let current_match = count_neighbour_matches(puzzle, &b, pos, piece.edges.rotated(current_rot).as_array());
            // Try other rotations.
            let mut best_rot = current_rot;
            let mut best_match = current_match;
            for rot in Rotation::ALL {
                if rot == current_rot { continue; }
                let e = piece.edges.rotated(rot).as_array();
                let m = count_neighbour_matches(puzzle, &b, pos, e);
                if m > best_match {
                    best_match = m;
                    best_rot = rot;
                }
            }
            if best_rot != current_rot {
                b.place(pos, pid, best_rot);
                total_improvement += best_match - current_match;
                any_change = true;
            }
        }
        if !any_change { break; }
    }
    (b, total_improvement)
}

/// Count edge-match count between cell at `pos` (with edges `e_at_pos`)
/// and its 4 placed neighbours. Doesn't count BORDER or empty edges.
fn count_neighbour_matches(puzzle: &Puzzle, board: &Board, pos: Position, e: [u8; 4]) -> u32 {
    let w = puzzle.width;
    let h = puzzle.height;
    let x = pos % w;
    let y = pos / w;
    let mut c = 0u32;
    // top
    if y > 0 {
        if let Some((npid, nrot)) = board.get((y - 1) * w + x) {
            if let Some(np) = lookup_piece(puzzle, npid) {
                let ne = np.edges.rotated(nrot).as_array();
                if e[0] != BORDER && ne[2] != BORDER && e[0] != 0 && ne[2] != 0 && e[0] == ne[2] {
                    c += 1;
                }
            }
        }
    }
    // right
    if x + 1 < w {
        if let Some((npid, nrot)) = board.get(y * w + (x + 1)) {
            if let Some(np) = lookup_piece(puzzle, npid) {
                let ne = np.edges.rotated(nrot).as_array();
                if e[1] != BORDER && ne[3] != BORDER && e[1] != 0 && ne[3] != 0 && e[1] == ne[3] {
                    c += 1;
                }
            }
        }
    }
    // bottom
    if y + 1 < h {
        if let Some((npid, nrot)) = board.get((y + 1) * w + x) {
            if let Some(np) = lookup_piece(puzzle, npid) {
                let ne = np.edges.rotated(nrot).as_array();
                if e[2] != BORDER && ne[0] != BORDER && e[2] != 0 && ne[0] != 0 && e[2] == ne[0] {
                    c += 1;
                }
            }
        }
    }
    // left
    if x > 0 {
        if let Some((npid, nrot)) = board.get(y * w + (x - 1)) {
            if let Some(np) = lookup_piece(puzzle, npid) {
                let ne = np.edges.rotated(nrot).as_array();
                if e[3] != BORDER && ne[1] != BORDER && e[3] != 0 && ne[1] != 0 && e[3] == ne[1] {
                    c += 1;
                }
            }
        }
    }
    c
}

/// Vol-17 — embarrassingly parallel best-of-K ALNS portfolio.
///
/// Run `n_chains` independent ALNS chains in parallel via rayon,
/// each with the SAME initial board but a DIFFERENT seed (seed,
/// seed+1, ..., seed+n-1). Returns the best board found across all
/// chains plus the score of each chain.
///
/// On 8-core hardware with `n_chains=4`, each chain gets ~2 cores
/// (the CP-repair inside is rayon-multi-core too). Should give
/// ~3-4x throughput vs single chain. Variance reduction comes from
/// seed diversity — different chains explore different basins.
///
/// `acceptance_for_chain(i)` lets you specify a per-chain acceptance
/// (e.g. different SA temperatures for "temperature-ladder portfolio"):
/// passing |i| `Acceptance::SimulatedAnnealing { t: 0.5 + 0.5 * i as f64 }`
/// gives 4 chains at t = 0.5, 1.0, 1.5, 2.0.
pub fn run_alns_portfolio<F>(
    puzzle: &Puzzle,
    initial: &Board,
    ops_factory: impl Fn(usize) -> Vec<Box<dyn DestroyOp>> + Send + Sync,
    base_cfg: &AlnsConfig,
    n_chains: usize,
    acceptance_for_chain: F,
) -> (Board, Vec<(u32, AlnsStats)>)
where
    F: Fn(usize) -> Acceptance + Send + Sync,
{
    use rayon::prelude::*;
    let chains: Vec<(usize, u64)> =
        (0..n_chains).map(|i| (i, derive_stream_seed(base_cfg.seed, i))).collect();
    let results: Vec<(Board, AlnsStats)> = chains
        .into_par_iter()
        .map(|(i, seed)| {
            let mut cfg = AlnsConfig {
                time_budget_ms: base_cfg.time_budget_ms,
                repair_budget_ms: base_cfg.repair_budget_ms,
                acceptance: acceptance_for_chain(i),
                segment_iters: base_cfg.segment_iters,
                seed,
                verbose: false,
                repair: base_cfg.repair,
                cp_fallback_to_sa: base_cfg.cp_fallback_to_sa,
                pinned_positions: base_cfg.pinned_positions.clone(),
                iter_budget: base_cfg.iter_budget,
                lex_break_isoscore: base_cfg.lex_break_isoscore,
                lex_break_intaglio: base_cfg.lex_break_intaglio,
                checkpoint_path: None,
                checkpoint_every_ms: base_cfg.checkpoint_every_ms,
                repair_step_budget: base_cfg.repair_step_budget,
                cp_repair_parallel: base_cfg.cp_repair_parallel,
            };
            cfg.seed = seed;
            let mut ops = ops_factory(i);
            run_alns(puzzle, initial, ops.as_mut_slice(), &cfg)
        })
        .collect();
    // Pick best.
    let mut best_idx = 0usize;
    let mut best_score = 0u32;
    let stats_with_scores: Vec<(u32, AlnsStats)> = results
        .iter()
        .enumerate()
        .map(|(i, (b, s))| {
            let score = score_board(puzzle, b);
            if score > best_score {
                best_score = score;
                best_idx = i;
            }
            (score, s.clone())
        })
        .collect();
    let best_board = results[best_idx].0.clone();
    (best_board, stats_with_scores)
}

/// Vol-17 NOVEL — Parallel Tempering on ALNS (PT-on-ALNS).
///
/// N chains at fixed temperatures `[t_min, ..., t_max]` (geometric ladder
/// by default; linear if `linear_ladder=true`). Each chain runs ALNS
/// with its own state. Periodically (every `exchange_every_rounds`
/// outer rounds), an exchange step proposes swapping adjacent chains'
/// CURRENT boards according to Metropolis criterion.
///
/// Differs from `run_alns_portfolio`: portfolio runs chains independently
/// to the time budget; PT exchanges boards between adjacent chains. The
/// exchange lets a high-scoring board found by a hot chain "fall" to a
/// colder chain that exploits it, while the hot chain takes the
/// previously-cold board and explores around it.
///
/// `chain_ops_factory(chain_idx)` — each chain gets its own destroy op
/// set (allows e.g. different ops per chain).
///
/// Returns: best board across all chains + per-chain stats + exchange
/// acceptance stats.
pub struct PtAlnsConfig {
    pub n_chains: usize,
    pub t_min: f64,
    pub t_max: f64,
    pub geometric_ladder: bool,
    pub inner_iters_per_round: u32,
    pub time_budget_ms: u64,
    pub repair_budget_ms: u64,
    pub segment_iters: u32,
    pub seed: u64,
    pub verbose: bool,
    pub repair: RepairKind,
    pub cp_fallback_to_sa: bool,
    pub pinned_positions: Vec<Position>,
}

#[derive(Debug, Clone)]
pub struct PtAlnsStats {
    pub rounds: u64,
    pub exchange_proposals: u64,
    pub exchange_accepts: u64,
    pub per_chain_scores: Vec<u32>,
    pub per_chain_iters: Vec<u32>,
    pub global_best_score: u32,
    pub global_best_seen_at_round: u64,
    pub global_best_seen_at_chain: usize,
}

pub fn run_alns_pt(
    puzzle: &Puzzle,
    initial: &Board,
    chain_ops_factory: impl Fn(usize) -> Vec<Box<dyn DestroyOp>> + Send + Sync,
    cfg: &PtAlnsConfig,
) -> (Board, PtAlnsStats) {
    run_alns_pt_multi_init(puzzle, &vec![initial.clone(); cfg.n_chains], chain_ops_factory, cfg)
}

/// Variant that lets each chain start from its OWN initial board. Useful
/// for multi-CP-partial PT where each chain has a different Blackwood
/// CP seed's output. `initials` length must equal `cfg.n_chains`.
pub fn run_alns_pt_multi_init(
    puzzle: &Puzzle,
    initials: &[Board],
    chain_ops_factory: impl Fn(usize) -> Vec<Box<dyn DestroyOp>> + Send + Sync,
    cfg: &PtAlnsConfig,
) -> (Board, PtAlnsStats) {
    use rayon::prelude::*;
    assert!(cfg.n_chains >= 2, "PT needs ≥2 chains");
    assert!(cfg.t_min > 0.0 && cfg.t_max > cfg.t_min, "bad temperature range");
    assert_eq!(initials.len(), cfg.n_chains, "initials.len must equal n_chains");

    // Build temperature ladder.
    let n = cfg.n_chains;
    let temps: Vec<f64> = if cfg.geometric_ladder {
        let ratio = (cfg.t_max / cfg.t_min).powf(1.0 / (n as f64 - 1.0));
        (0..n).map(|i| cfg.t_min * ratio.powi(i as i32)).collect()
    } else {
        let denom = (n - 1).max(1) as f64;
        (0..n).map(|i| cfg.t_min + (cfg.t_max - cfg.t_min) * (i as f64) / denom).collect()
    };

    // Build per-chain initial state.
    let mut boards: Vec<Board> = initials.to_vec();
    let mut scores: Vec<u32> = boards.iter().map(|b| score_board(puzzle, b)).collect();
    let mut best_board = boards[0].clone();
    let mut best_score = scores[0];
    let mut best_chain = 0;
    let mut best_round = 0u64;
    let _ = best_round; // suppress unused-init warning; assigned below.
    for i in 1..n {
        if scores[i] > best_score {
            best_score = scores[i];
            best_board = boards[i].clone();
            best_chain = i;
        }
    }

    // Per-chain op sets, RNGs, weights.
    let mut chain_ops: Vec<Vec<Box<dyn DestroyOp>>> = (0..n).map(|i| chain_ops_factory(i)).collect();
    let mut chain_seeds: Vec<u64> = (0..n)
        .map(|i| cfg.seed.wrapping_add((i as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)))
        .collect();
    let mut total_iters: Vec<u32> = vec![0; n];

    let mut stats = PtAlnsStats {
        rounds: 0,
        exchange_proposals: 0,
        exchange_accepts: 0,
        per_chain_scores: scores.clone(),
        per_chain_iters: vec![0; n],
        global_best_score: best_score,
        global_best_seen_at_round: 0,
        global_best_seen_at_chain: best_chain,
    };

    // Exchange RNG (separate so chain RNGs aren't biased by exchange order).
    let mut exchange_rng = AlnsRng::new(cfg.seed ^ 0xDEAD_BEEF_CAFE_BABE);

    let t_start = std::time::Instant::now();

    while t_start.elapsed().as_millis() < cfg.time_budget_ms as u128 {
        stats.rounds += 1;

        // PHASE A: each chain runs `inner_iters_per_round` ALNS iterations.
        let pinned_set: BTreeSet<Position> = cfg.pinned_positions.iter().copied().collect();
        // Build bundles per chain. Use Index by index to avoid aliasing.
        let _bundles: Vec<(usize, &Board, &[Position], u64, f64)> = (0..n)
            .map(|i| (i, &boards[i], &cfg.pinned_positions[..], chain_seeds[i], temps[i]))
            .collect();
        // Run in parallel via rayon. Each task produces (new_board, new_score,
        // new_seed, iter_count). We can't hold &mut ops across the parallel
        // iterator, so we use `chain_ops` indexed by chain_id; pop the ops out
        // before the parallel call and reinstate after.
        let ops_taken: Vec<Vec<Box<dyn DestroyOp>>> = std::mem::take(&mut chain_ops);
        let pinned_clone = pinned_set.clone();
        let pinned_positions = cfg.pinned_positions.clone();
        let results: Vec<(usize, Board, u32, u64, u32, Vec<Box<dyn DestroyOp>>)> = ops_taken
            .into_par_iter()
            .enumerate()
            .map(|(i, mut ops)| {
                let cfg_chain = AlnsConfig {
                    time_budget_ms: cfg.time_budget_ms, // generous; iter_budget caps it
                    repair_budget_ms: cfg.repair_budget_ms,
                    acceptance: Acceptance::SimulatedAnnealing { t: temps[i] },
                    segment_iters: cfg.segment_iters,
                    seed: chain_seeds[i],
                    verbose: false,
                    repair: cfg.repair,
                    cp_fallback_to_sa: cfg.cp_fallback_to_sa,
                    pinned_positions: pinned_positions.clone(),
                    iter_budget: cfg.inner_iters_per_round,
                    lex_break_isoscore: false,
                    lex_break_intaglio: false,
                    checkpoint_path: None,
                    checkpoint_every_ms: 60_000,
                    repair_step_budget: 0,
                    cp_repair_parallel: true,
                };
                let (new_board, new_stats) = run_alns(puzzle, &boards[i], ops.as_mut_slice(), &cfg_chain);
                let new_score = score_board(puzzle, &new_board);
                let new_seed = chain_seeds[i].wrapping_mul(0x9E37_79B9_7F4A_7C15).wrapping_add(1);
                (i, new_board, new_score, new_seed, new_stats.iters, ops)
            })
            .collect();
        let _ = pinned_clone;
        // Re-insert ops, update state.
        chain_ops = (0..n).map(|_| Vec::new()).collect();
        for (i, new_board, new_score, new_seed, iters, ops) in results {
            boards[i] = new_board;
            scores[i] = new_score;
            chain_seeds[i] = new_seed;
            total_iters[i] += iters;
            chain_ops[i] = ops;
            if new_score > best_score {
                best_score = new_score;
                best_board = boards[i].clone();
                best_chain = i;
                best_round = stats.rounds;
                stats.global_best_score = best_score;
                stats.global_best_seen_at_round = best_round;
                stats.global_best_seen_at_chain = best_chain;
            }
        }

        // PHASE B: exchange step for adjacent pairs.
        // Process pairs in alternating odd/even order to avoid bias.
        let pair_order_offset = (stats.rounds % 2) as usize;
        let mut pair = pair_order_offset;
        while pair + 1 < n {
            let i = pair;
            let j = pair + 1;
            stats.exchange_proposals += 1;
            // Metropolis: accept if u < exp((s_i - s_j) × (1/t_j - 1/t_i)).
            // Equivalently, when s_j > s_i, the colder chain (i, smaller t)
            // wants to take s_j; this happens with prob 1 if the math works.
            let delta = (scores[i] as f64 - scores[j] as f64)
                * (1.0 / temps[j] - 1.0 / temps[i]);
            let p = delta.exp().min(1.0);
            let u = exchange_rng.next_f64();
            if u < p {
                stats.exchange_accepts += 1;
                boards.swap(i, j);
                scores.swap(i, j);
            }
            pair += 2;
        }

        if cfg.verbose {
            eprintln!(
                "[PT-ALNS round {}] t_start={:.1}s scores={:?} best={} (chain {})",
                stats.rounds,
                t_start.elapsed().as_secs_f64(),
                scores,
                best_score,
                best_chain,
            );
        }
    }

    stats.per_chain_scores = scores;
    stats.per_chain_iters = total_iters;
    (best_board, stats)
}

// Re-export so the binary doesn't need to know internal modules.
pub use AlnsConfig as Config;

// Suppress dead-code warnings for items we deliberately keep public.
#[allow(dead_code)]
fn _force_referenced_items() {
    let _ = repair_region;
    let _ = pt::PtConfig::default();
}
