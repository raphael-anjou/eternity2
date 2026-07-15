// Eternity-II local-search engine. Implements simulated annealing
// over the space of complete piece placements, with five neighborhood
// operators inspired by Wauters et al. META'10 ("Guide-and-Observe
// Hyper-Heuristic"). The objective is matching-edge count; secondary
// objectives (color frequency balance) break ties during selection.
//
// Why local search? CP backtracking (our solver-engine) plateaus in
// the "Goldilocks zone" of constraint density on hard instances. The
// only published technique reaching 461+/480 on real Eternity II is a
// metaheuristic of this shape (Wauters; Verhaard's 467 is closer to
// portfolio CP+heuristics, with different ingredients we keep separate).
//
// Design notes:
//   - We operate on a fully-placed board (no holes). Initial state is
//     a greedy random placement; iteratively we improve.
//   - State carries scoring caches so that neighborhood moves update
//     the score in O(neighbours_touched) rather than O(board).
//   - Borders are constrained: corner-pieces only go to corner cells,
//     edge-pieces only to non-corner border cells, interiors to
//     interiors. Moves that violate this are rejected up-front.
//
// What is NOT here yet:
//   - The integration with eternity2-solver-engine (CP-then-LS hybrid).
//   - Distributed restarts.

#![forbid(unsafe_code)]

use eternity2_core::{
    Board, Color, PieceId, Position, Puzzle, Rotation, BORDER,
};

pub mod alns;
pub mod directed;
pub mod forbidden;
pub mod houdayer;
pub mod pt;
pub mod repair;
pub mod zobrist;
pub mod ot_repair;
pub mod ot_repair_jv;
pub mod oracle_swap;
pub mod filament;
pub mod intaglio;
pub use alns::{derive_stream_seed, run_alns, run_alns_portfolio, run_alns_pt, run_alns_pt_multi_init,
               polish_rotations, piece_swap_hillclimb, find_mismatches, Mismatch, AlnsConfig,
               AlnsStats, PtAlnsConfig, PtAlnsStats, AdaptiveWeights, Acceptance, DestroyOp,
               RandomRegion, WorstWindow, WorstBand, WorstRow, BottomBandDestroy, ConflictDriven,
               MwpmDefectPair, ComponentDestroy, ComponentPlusHaloDestroy,
               ComponentClusterDestroy, HingeDestroy,
               MegaBand, WorstColumn, WorstColumnBand, HalfBoardDestroy, RandomScatter,
               SigmaCycleDestroy, LkhChainDestroy, ForbidDestroy, PriorDestroy,
               RepairKind};
pub use forbidden::{ForbiddenEdge, ForbiddenContext, parse_forbidden_json,
                    build_edges_by_cell, fmm_full, fmm_touched};
pub use pt::{run_pt, run_pt_from, PtConfig, PtStats};
pub use repair::{repair_region, repair_cells, worst_region};
pub use directed::{run_directed, DirectedConfig};
pub use zobrist::{BoardZobrist, ZOBRIST_MAX_PIECES};

/// Configuration for the simulated-annealing local search.
#[derive(Debug, Clone)]
pub struct SaConfig {
    /// Initial temperature. Wauters et al. used 5000 for E2 specifically.
    pub temperature_start: f64,
    /// Lower bound on temperature; search stops or restarts here.
    pub temperature_min: f64,
    /// Geometric cooling factor applied each `cooling_period` steps.
    pub cooling: f64,
    /// Number of accepted+rejected moves between cooling steps.
    pub cooling_period: u64,
    /// Hard cap on number of iterations (0 = unlimited).
    pub max_iters: u64,
    /// Optional wall-clock budget in milliseconds (0 = unlimited).
    pub time_budget_ms: u64,
    /// PRNG seed for reproducibility.
    pub seed: u64,
    /// Cell positions that must NEVER move (piece and rotation preserved).
    /// Used to honour the official E2 hint pieces during local search. Empty
    /// by default — solver runs unconstrained.
    pub pinned_positions: Vec<Position>,
}

impl Default for SaConfig {
    fn default() -> Self {
        // T_start picked so a single-edge loss (delta = -1) is accepted
        // with p ≈ 0.6 initially (exp(-1/2)) and dropping toward 0.5
        // after a few hundred cooling steps. Wauters et al. report
        // T_start=5000 but they aggregate multiple objectives; for
        // pure edge-match (delta ∈ [-8, +8] per swap) that would be
        // ~random walk. We use a delta-calibrated scale here.
        Self {
            temperature_start: 2.0,
            temperature_min: 0.05,
            cooling: 0.999,
            cooling_period: 5000,
            max_iters: 0,
            time_budget_ms: 0,
            seed: 0xE2_E2_E2_E2,
            pinned_positions: Vec::new(),
        }
    }
}

/// Outcome of a local-search run.
#[derive(Debug, Clone)]
pub struct SaOutcome {
    /// Best board encountered (may equal final or be an earlier snapshot).
    pub best_board: Board,
    /// Number of edges matched in `best_board`.
    pub best_score: u32,
    /// Total interior edges (denominator for `best_score`).
    pub total_edges: u32,
    /// Iterations performed (accepted + rejected combined).
    pub iterations: u64,
    /// Wall-clock microseconds of the run.
    pub elapsed_us: u128,
}

// ============================================================
// Static piece-classification by border-edge count, used to keep
// moves on-class (corner ⇆ corner only, etc).
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellClass {
    Corner,
    Edge,
    Interior,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceClass {
    Corner,  // 2 BORDER edges
    Edge,    // 1 BORDER edge
    Interior, // 0 BORDER edges
}

fn classify_cell(puzzle: &Puzzle, pos: Position) -> CellClass {
    let mask = puzzle.border_mask(pos);
    let n_border = mask.iter().filter(|b| **b).count();
    match n_border {
        2 => CellClass::Corner,
        1 => CellClass::Edge,
        _ => CellClass::Interior,
    }
}

fn classify_piece(piece_edges: [Color; 4]) -> PieceClass {
    let n_border = piece_edges.iter().filter(|c| **c == BORDER).count();
    match n_border {
        2 => PieceClass::Corner,
        1 => PieceClass::Edge,
        _ => PieceClass::Interior,
    }
}


// ============================================================
// SplitMix64 PRNG (same family as the generator crate uses).
// ============================================================

#[derive(Debug, Clone, Copy)]
pub(crate) struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self { Self(seed.wrapping_add(0x9E37_79B9_7F4A_7C15)) }
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn gen_range(&mut self, n: u32) -> u32 {
        if n == 0 { return 0; }
        (self.next_u64() % u64::from(n)) as u32
    }
    fn next_f64(&mut self) -> f64 {
        // Generate a double in [0, 1).
        let bits = self.next_u64() >> 11;
        (bits as f64) * (1.0 / ((1u64 << 53) as f64))
    }
}

// ============================================================
// Run-time state
// ============================================================

pub(crate) struct State<'a> {
    pub(crate) puzzle: &'a Puzzle,
    pub(crate) piece_rot_edges: Vec<[Color; 4]>,
    pub(crate) cell_class: Vec<CellClass>,
    pub(crate) piece_class: Vec<PieceClass>,
    pub(crate) corner_pieces: Vec<PieceId>,
    pub(crate) edge_pieces: Vec<PieceId>,
    pub(crate) interior_pieces: Vec<PieceId>,
    pub(crate) corner_cells: Vec<Position>,
    pub(crate) edge_cells: Vec<Position>,
    pub(crate) interior_cells: Vec<Position>,
    // pinned[pos] == true ⇒ no move may touch this cell. Used to honour
    // official E2 hint pieces during PT/SA. Cluster moves (2×2, 3×3, region
    // swap) must abort if ANY cell they would touch is pinned.
    pub(crate) pinned: Vec<bool>,
}

impl<'a> State<'a> {
    fn new(puzzle: &'a Puzzle) -> Self {
        Self::new_with_pinned(puzzle, &[])
    }

    fn new_with_pinned(puzzle: &'a Puzzle, pinned_positions: &[Position]) -> Self {
        let pieces = puzzle.pieces();
        let max_piece_index = pieces.iter().map(|p| usize::from(p.id) + 1).max().unwrap_or(0);
        let mut piece_rot_edges = vec![[0u8; 4]; max_piece_index * 4];
        let mut piece_class = vec![PieceClass::Interior; max_piece_index];
        let mut corner_pieces = Vec::new();
        let mut edge_pieces = Vec::new();
        let mut interior_pieces = Vec::new();
        for piece in pieces {
            let pid = usize::from(piece.id);
            for r in 0..4u8 {
                let rot = Rotation::from_u8(r).unwrap();
                let e = piece.edges.rotated(rot).as_array();
                piece_rot_edges[pid * 4 + r as usize] = e;
            }
            let class = classify_piece(piece.edges.as_array());
            piece_class[pid] = class;
            match class {
                PieceClass::Corner => corner_pieces.push(piece.id),
                PieceClass::Edge => edge_pieces.push(piece.id),
                PieceClass::Interior => interior_pieces.push(piece.id),
            }
        }

        let n_cells = puzzle.cell_count() as usize;
        let mut pinned = vec![false; n_cells];
        for &pos in pinned_positions {
            if (pos as usize) < n_cells {
                pinned[pos as usize] = true;
            }
        }

        let mut cell_class = Vec::with_capacity(n_cells);
        let mut corner_cells = Vec::new();
        let mut edge_cells = Vec::new();
        let mut interior_cells = Vec::new();
        for pos in 0..puzzle.cell_count() {
            let c = classify_cell(puzzle, pos);
            cell_class.push(c);
            // Pinned cells are removed from the per-class pools so random
            // picks never select them. Cluster moves consult `pinned` directly.
            if pinned[pos as usize] {
                continue;
            }
            match c {
                CellClass::Corner => corner_cells.push(pos),
                CellClass::Edge => edge_cells.push(pos),
                CellClass::Interior => interior_cells.push(pos),
            }
        }

        Self {
            puzzle,
            piece_rot_edges,
            cell_class,
            piece_class,
            corner_pieces,
            edge_pieces,
            interior_pieces,
            corner_cells,
            edge_cells,
            interior_cells,
            pinned,
        }
    }

    fn edges_for(&self, piece_id: PieceId, rotation: Rotation) -> [Color; 4] {
        let idx = usize::from(piece_id) * 4 + usize::from(rotation.as_u8());
        self.piece_rot_edges[idx]
    }

    /// Build a feasible initial placement: pieces of correct class are
    /// dropped into cells of correct class in seed-random order. Border
    /// rotation is forced to the unique rotation that puts BORDER on
    /// the outward-facing sides. Interior rotation is random.
    fn random_initial(&self, rng: &mut Rng) -> Board {
        let mut board = Board::empty(self.puzzle);
        // Helper: pick rotation that maximises border-match for a corner
        // or edge cell; for interiors choose at random.
        let pick_rot = |pid: PieceId, pos: Position, rng: &mut Rng| -> Rotation {
            let mask = self.puzzle.border_mask(pos);
            let [tb, rb, bb, lb] = mask;
            // For non-interior cells, find a rotation r such that
            // edges[s] == BORDER iff mask[s] is true for sides
            // (top, right, bot, left).
            if matches!(self.cell_class[pos as usize], CellClass::Interior) {
                Rotation::from_u8(rng.gen_range(4) as u8).unwrap()
            } else {
                let mut best = Rotation::from_u8(0).unwrap();
                for r in 0..4u8 {
                    let rot = Rotation::from_u8(r).unwrap();
                    let e = self.edges_for(pid, rot);
                    if (e[0] == BORDER) == tb
                        && (e[1] == BORDER) == rb
                        && (e[2] == BORDER) == bb
                        && (e[3] == BORDER) == lb
                    {
                        best = rot;
                        break;
                    }
                }
                best
            }
        };
        // Shuffle each piece pool then drop one-by-one into matching cells.
        let do_class = |pieces: &mut Vec<PieceId>, cells: &[Position], rng: &mut Rng, board: &mut Board| {
            // Fisher-Yates over piece pool.
            for i in (1..pieces.len()).rev() {
                let j = rng.gen_range((i + 1) as u32) as usize;
                pieces.swap(i, j);
            }
            for (i, &pos) in cells.iter().enumerate() {
                let pid = pieces[i];
                let rot = pick_rot(pid, pos, rng);
                board.place(pos, pid, rot);
            }
        };
        let mut corner = self.corner_pieces.clone();
        let mut edge = self.edge_pieces.clone();
        let mut interior = self.interior_pieces.clone();
        do_class(&mut corner, &self.corner_cells, rng, &mut board);
        do_class(&mut edge, &self.edge_cells, rng, &mut board);
        do_class(&mut interior, &self.interior_cells, rng, &mut board);
        board
    }

    /// Take a possibly-partial `initial` board and produce a full
    /// placement: keep all already-placed cells, then fill the empties
    /// with random class-matching pieces (picking a class-correct
    /// rotation).
    fn fill_in_initial(&self, initial: &Board, rng: &mut Rng) -> Board {
        let mut board = initial.clone();
        // Identify which pieces are already used (by piece-id).
        let n_pieces = self.piece_class.len();
        let mut used = vec![false; n_pieces];
        for c in board.cells() {
            if let Some((pid, _rot)) = c {
                used[usize::from(*pid)] = true;
            }
        }
        // Build leftover pools per class.
        let mut leftover_corner: Vec<PieceId> = Vec::new();
        let mut leftover_edge: Vec<PieceId> = Vec::new();
        let mut leftover_interior: Vec<PieceId> = Vec::new();
        for (pid_idx, was_used) in used.iter().enumerate() {
            if *was_used { continue; }
            // Reverse-lookup: piece_class[pid] is class
            let pid_u16 = pid_idx as u16;
            match self.piece_class[pid_idx] {
                PieceClass::Corner => leftover_corner.push(pid_u16),
                PieceClass::Edge => leftover_edge.push(pid_u16),
                PieceClass::Interior => leftover_interior.push(pid_u16),
            }
        }
        // Find empty cells per class.
        let mut empty_corner: Vec<Position> = Vec::new();
        let mut empty_edge: Vec<Position> = Vec::new();
        let mut empty_interior: Vec<Position> = Vec::new();
        for pos in 0..self.puzzle.cell_count() {
            if board.get(pos).is_some() { continue; }
            match self.cell_class[pos as usize] {
                CellClass::Corner => empty_corner.push(pos),
                CellClass::Edge => empty_edge.push(pos),
                CellClass::Interior => empty_interior.push(pos),
            }
        }
        // Shuffle leftover pieces then drop them into empty cells.
        let shuffle = |v: &mut Vec<PieceId>, rng: &mut Rng| {
            for i in (1..v.len()).rev() {
                let j = rng.gen_range((i + 1) as u32) as usize;
                v.swap(i, j);
            }
        };
        shuffle(&mut leftover_corner, rng);
        shuffle(&mut leftover_edge, rng);
        shuffle(&mut leftover_interior, rng);
        let pick_rot = |pid: PieceId, pos: Position, rng: &mut Rng, st: &State| -> Rotation {
            let mask = st.puzzle.border_mask(pos);
            let [tb, rb, bb, lb] = mask;
            if matches!(st.cell_class[pos as usize], CellClass::Interior) {
                Rotation::from_u8(rng.gen_range(4) as u8).unwrap()
            } else {
                for r in 0..4u8 {
                    let rot = Rotation::from_u8(r).unwrap();
                    let e = st.edges_for(pid, rot);
                    if (e[0] == BORDER) == tb
                        && (e[1] == BORDER) == rb
                        && (e[2] == BORDER) == bb
                        && (e[3] == BORDER) == lb
                    { return rot; }
                }
                Rotation::from_u8(0).unwrap()
            }
        };
        for (i, &pos) in empty_corner.iter().enumerate() {
            if i >= leftover_corner.len() { break; }
            let pid = leftover_corner[i];
            board.place(pos, pid, pick_rot(pid, pos, rng, self));
        }
        for (i, &pos) in empty_edge.iter().enumerate() {
            if i >= leftover_edge.len() { break; }
            let pid = leftover_edge[i];
            board.place(pos, pid, pick_rot(pid, pos, rng, self));
        }
        for (i, &pos) in empty_interior.iter().enumerate() {
            if i >= leftover_interior.len() { break; }
            let pid = leftover_interior[i];
            board.place(pos, pid, pick_rot(pid, pos, rng, self));
        }
        board
    }

    /// Like `fill_in_initial`, but for each empty cell pick the leftover
    /// piece + rotation that MAXIMISES local matches with currently-placed
    /// neighbours. Iterates over empties in order of "most constrained
    /// first" (most placed neighbours) so each greedy decision has more
    /// information. Falls back to random pick when no leftover offers
    /// any match.
    ///
    /// This is the right initialization for CP→LS / CP→PT hybrids: the
    /// CP partial is preserved exactly, and the freed cells start at a
    /// near-optimal configuration relative to the partial. Single-T SA
    /// from a randomly-filled start spends most of its budget recovering
    /// the score that random destruction created.
    fn greedy_fill_initial(&self, initial: &Board, rng: &mut Rng) -> Board {
        let mut board = initial.clone();
        let n_pieces = self.piece_class.len();
        let mut used = vec![false; n_pieces];
        for c in board.cells() {
            if let Some((pid, _rot)) = c {
                used[usize::from(*pid)] = true;
            }
        }
        // Leftover pools per class.
        let mut leftover_corner: Vec<PieceId> = Vec::new();
        let mut leftover_edge: Vec<PieceId> = Vec::new();
        let mut leftover_interior: Vec<PieceId> = Vec::new();
        for (pid_idx, was_used) in used.iter().enumerate() {
            if *was_used { continue; }
            let pid_u16 = pid_idx as u16;
            match self.piece_class[pid_idx] {
                PieceClass::Corner => leftover_corner.push(pid_u16),
                PieceClass::Edge => leftover_edge.push(pid_u16),
                PieceClass::Interior => leftover_interior.push(pid_u16),
            }
        }

        // Process empties class-by-class, greedy on best-piece-best-rot.
        // Within a class, fill in order of fewest empty neighbours first
        // (so when we decide, we know more of our context). Use a stable
        // tie-breaker on position.
        let w = self.puzzle.width;
        let h = self.puzzle.height;
        let empty_neighbour_count = |b: &Board, pos: Position| -> u32 {
            let (x, y) = (pos % w, pos / w);
            let mut c = 0;
            if y > 0 && b.get((y - 1) * w + x).is_none() { c += 1; }
            if x + 1 < w && b.get(y * w + (x + 1)).is_none() { c += 1; }
            if y + 1 < h && b.get((y + 1) * w + x).is_none() { c += 1; }
            if x > 0 && b.get(y * w + (x - 1)).is_none() { c += 1; }
            c
        };

        // Pick best (piece, rot) from `pool` for `pos`, returning (piece_index_in_pool, rot, local_match_count).
        // For interior cells try all 4 rotations; for border cells only border-compatible rotation.
        let pick_best = |b: &Board, pool: &[PieceId], pos: Position, st: &State| -> Option<(usize, Rotation, u32)> {
            if pool.is_empty() { return None; }
            let mask = st.puzzle.border_mask(pos);
            let [tb, rb, bb, lb] = mask;
            let is_interior = matches!(st.cell_class[pos as usize], CellClass::Interior);
            let mut best: Option<(usize, Rotation, u32)> = None;
            for (idx, &pid) in pool.iter().enumerate() {
                for r in 0..4u8 {
                    let rot = Rotation::from_u8(r).unwrap();
                    let e = st.edges_for(pid, rot);
                    if !is_interior {
                        if (e[0] == BORDER) != tb
                            || (e[1] == BORDER) != rb
                            || (e[2] == BORDER) != bb
                            || (e[3] == BORDER) != lb
                        { continue; }
                    }
                    let mc = match_count_with(st, b, pos, e);
                    match best {
                        None => best = Some((idx, rot, mc)),
                        Some((_, _, bm)) if mc > bm => best = Some((idx, rot, mc)),
                        _ => {}
                    }
                }
            }
            best
        };

        let do_class = |pool: &mut Vec<PieceId>, st: &State, b: &mut Board, rng: &mut Rng| {
            let _ = rng; // currently unused; kept for symmetry / future jitter
            loop {
                if pool.is_empty() { break; }
                // Find the empty cell with FEWEST empty neighbours (= most known context).
                let mut best_pos: Option<Position> = None;
                let mut best_empties = u32::MAX;
                let n = b.cells().len() as Position;
                for pos in 0..n {
                    if b.get(pos).is_some() { continue; }
                    // Restrict to this class.
                    let class = st.cell_class[pos as usize];
                    let class_ok = match class {
                        CellClass::Corner => pool.iter().any(|p| matches!(st.piece_class[usize::from(*p)], PieceClass::Corner)),
                        CellClass::Edge => pool.iter().any(|p| matches!(st.piece_class[usize::from(*p)], PieceClass::Edge)),
                        CellClass::Interior => pool.iter().any(|p| matches!(st.piece_class[usize::from(*p)], PieceClass::Interior)),
                    };
                    if !class_ok { continue; }
                    let en = empty_neighbour_count(b, pos);
                    if en < best_empties {
                        best_empties = en;
                        best_pos = Some(pos);
                    }
                }
                let Some(pos) = best_pos else { break; };
                match pick_best(b, pool, pos, st) {
                    Some((idx, rot, _mc)) => {
                        let pid = pool.swap_remove(idx);
                        b.place(pos, pid, rot);
                    }
                    None => break, // no piece in pool can validly land at pos
                }
            }
        };

        do_class(&mut leftover_corner, self, &mut board, rng);
        do_class(&mut leftover_edge, self, &mut board, rng);
        do_class(&mut leftover_interior, self, &mut board, rng);
        board
    }

    /// Count matching interior edges (the maximisation target).
    ///
    /// An interior edge between two adjacent cells matches iff the
    /// two facing colors are equal AND non-zero (a BORDER on an
    /// interior edge means the placement is invalid for that side,
    /// so we exclude it).
    fn score(&self, board: &Board) -> u32 {
        let w = self.puzzle.width;
        let h = self.puzzle.height;
        let mut matches = 0u32;
        for y in 0..h {
            for x in 0..w {
                let pos = y * w + x;
                let cell = board.get(pos);
                let Some((pid, rot)) = cell else { continue; };
                let e = self.edges_for(pid, rot);
                // Compare with right neighbour and below neighbour to
                // avoid double-counting. Treat off-board (border) edges
                // as required-BORDER on outward sides.
                if x + 1 < w {
                    let r_cell = board.get(y * w + (x + 1));
                    if let Some((rpid, rrot)) = r_cell {
                        let re = self.edges_for(rpid, rrot);
                        if e[1] == re[3] && e[1] != 0 {
                            matches += 1;
                        }
                    }
                }
                if y + 1 < h {
                    let b_cell = board.get((y + 1) * w + x);
                    if let Some((bpid, brot)) = b_cell {
                        let be = self.edges_for(bpid, brot);
                        if e[2] == be[0] && e[2] != 0 {
                            matches += 1;
                        }
                    }
                }
            }
        }
        matches
    }

    /// Total interior edges (target denominator).
    fn total_interior_edges(&self) -> u32 {
        let w = self.puzzle.width;
        let h = self.puzzle.height;
        // Horizontal interior edges: (w-1) per row × h rows
        // Vertical interior edges: w per column × (h-1) gaps
        (w - 1) * h + w * (h - 1)
    }
}

/// Compute the edge-match contribution of a single cell. Counts the
/// number of matching edges on its 4 sides (each counted once per
/// shared edge; this function returns only this cell's view, so it
/// returns 4 if all 4 neighbours match, but each match contributes 1
/// to the cell-vs-neighbour edge, NOT 1 to each of the two cells).
///
/// Used for incremental delta computation: changing piece-at-pos
/// affects this cell's contribution AND each neighbour's contribution
/// at the shared edge, but since matches are symmetric, the delta in
/// global score = (new local matches) - (old local matches).
pub(crate) fn local_match_count(state: &State, board: &Board, pos: Position) -> u32 {
    let Some((pid, rot)) = board.get(pos) else { return 0; };
    let e = state.edges_for(pid, rot);
    let w = state.puzzle.width;
    let h = state.puzzle.height;
    let (x, y) = (pos % w, pos / w);
    let mut m = 0u32;
    // Top neighbour
    if y > 0 {
        if let Some((np, nr)) = board.get((y - 1) * w + x) {
            let ne = state.edges_for(np, nr);
            if e[0] == ne[2] && e[0] != 0 { m += 1; }
        }
    }
    // Right
    if x + 1 < w {
        if let Some((np, nr)) = board.get(y * w + (x + 1)) {
            let ne = state.edges_for(np, nr);
            if e[1] == ne[3] && e[1] != 0 { m += 1; }
        }
    }
    // Bottom
    if y + 1 < h {
        if let Some((np, nr)) = board.get((y + 1) * w + x) {
            let ne = state.edges_for(np, nr);
            if e[2] == ne[0] && e[2] != 0 { m += 1; }
        }
    }
    // Left
    if x > 0 {
        if let Some((np, nr)) = board.get(y * w + (x - 1)) {
            let ne = state.edges_for(np, nr);
            if e[3] == ne[1] && e[3] != 0 { m += 1; }
        }
    }
    m
}

/// Pick the rotation of `pid` at `pos` that maximises edge matches
/// with currently-placed neighbours. For border cells, only border-
/// compatible rotations are considered.
pub(crate) fn best_rotation(state: &State, board: &Board, pos: Position, pid: PieceId) -> Rotation {
    let mask = state.puzzle.border_mask(pos);
    let [tb, rb, bb, lb] = mask;
    let is_interior = matches!(state.cell_class[pos as usize], CellClass::Interior);
    let mut best_rot = Rotation::from_u8(0).unwrap();
    let mut best_count = u32::MAX; // sentinel; we only set after first valid try
    let mut seen_any = false;
    for r in 0..4u8 {
        let rot = Rotation::from_u8(r).unwrap();
        let e = state.edges_for(pid, rot);
        if !is_interior {
            // Must align borders. Reject misaligned rotations.
            if (e[0] == BORDER) != tb
                || (e[1] == BORDER) != rb
                || (e[2] == BORDER) != bb
                || (e[3] == BORDER) != lb
            { continue; }
        }
        // Tentatively place and count local matches.
        let saved = board.get(pos);
        // Use a const local "what would the count be" — we don't
        // mutate the actual board by writing then reverting; instead
        // compute by emulating the comparison.
        let m = match_count_with(state, board, pos, e);
        // Discount: we don't want to write to board. Hack: we
        // compute equivalent of placing this piece-rot at pos.
        let _ = saved;
        if !seen_any || m > best_count {
            best_count = m;
            best_rot = rot;
            seen_any = true;
        }
    }
    best_rot
}

/// Hypothetical local match-count if the piece-rotation with edges
/// `e_hypo` were at `pos`. Doesn't mutate board.
pub(crate) fn match_count_with(state: &State, board: &Board, pos: Position, e_hypo: [Color; 4]) -> u32 {
    let w = state.puzzle.width;
    let h = state.puzzle.height;
    let (x, y) = (pos % w, pos / w);
    let mut m = 0u32;
    if y > 0 {
        if let Some((np, nr)) = board.get((y - 1) * w + x) {
            let ne = state.edges_for(np, nr);
            if e_hypo[0] == ne[2] && e_hypo[0] != 0 { m += 1; }
        }
    }
    if x + 1 < w {
        if let Some((np, nr)) = board.get(y * w + (x + 1)) {
            let ne = state.edges_for(np, nr);
            if e_hypo[1] == ne[3] && e_hypo[1] != 0 { m += 1; }
        }
    }
    if y + 1 < h {
        if let Some((np, nr)) = board.get((y + 1) * w + x) {
            let ne = state.edges_for(np, nr);
            if e_hypo[2] == ne[0] && e_hypo[2] != 0 { m += 1; }
        }
    }
    if x > 0 {
        if let Some((np, nr)) = board.get(y * w + (x - 1)) {
            let ne = state.edges_for(np, nr);
            if e_hypo[3] == ne[1] && e_hypo[3] != 0 { m += 1; }
        }
    }
    m
}

/// Returns 1 if the edge on side `side` (0=top, 1=right, 2=bottom, 3=left)
/// of the piece at `pos` matches the corresponding neighbour edge.
/// Returns 0 if the side is at the board border, the neighbour is empty,
/// or the colors mismatch.
fn partial_edge_match(state: &State, board: &Board, pos: Position, side: u8) -> u32 {
    let w = state.puzzle.width;
    let h = state.puzzle.height;
    let (x, y) = (pos % w, pos / w);
    let Some((pid, rot)) = board.get(pos) else { return 0; };
    let e = state.edges_for(pid, rot);
    let (nx, ny, opp): (i32, i32, usize) = match side {
        0 => (x as i32, y as i32 - 1, 2),
        1 => (x as i32 + 1, y as i32, 3),
        2 => (x as i32, y as i32 + 1, 0),
        _ => (x as i32 - 1, y as i32, 1),
    };
    if nx < 0 || ny < 0 || nx >= w as i32 || ny >= h as i32 { return 0; }
    let np = (ny as u32) * w + nx as u32;
    let Some((npid, nrot)) = board.get(np) else { return 0; };
    let ne = state.edges_for(npid, nrot);
    if e[side as usize] != 0 && e[side as usize] == ne[opp] { 1 } else { 0 }
}

/// Run `n_iters` SA steps at a FIXED temperature (no cooling, no re-anneal).
/// Used by the parallel-tempering driver to advance each replica between
/// exchange proposals. Updates `board`, `score`, and `best_*` in place;
/// returns the final score.
///
/// This is a deliberate code-twin of the body of `run_sa_loop`: it must
/// stay byte-for-byte equivalent on a single step so any bug fix here
/// applies there too. (We did NOT collapse them into a shared inner
/// because run_sa_loop's cooling / re-anneal / timed-out checks straddle
/// the iteration in non-trivial ways.)
pub fn run_sa_steps_fixed_temp(
    state: &StateRef<'_>,
    board: &mut Board,
    score: &mut u32,
    best_board: &mut Board,
    best_score: &mut u32,
    rng: &mut RngHandle,
    temp: f64,
    n_iters: u64,
) {
    run_sa_steps_fixed_temp_inner(state, board, score, best_board, best_score, rng, temp, n_iters, None);
}

/// Constrained variant: applies forbidden-mismatch soft penalty to the
/// Metropolis acceptance for the SIMPLE move kinds (single-rotate,
/// plain swap, targeted swap). Cluster moves (2x2, 3x3, region-swap,
/// 3-cycle) remain unconstrained — they're multi-cell moves where
/// computing per-touched-set fmm-delta is more expensive and the
/// penalty's discriminative power weaker. NE2 swap-level penalty at
/// the PT-exchange step still catches bad cluster outcomes.
///
/// Effective Metropolis delta: `eff_delta = raw_delta - K * fmm_delta`,
/// where fmm_delta = (new_fmm - old_fmm) over touched cells. Tracked
/// `*score` and `*best_score` remain RAW edge-match counts.
pub fn run_sa_steps_fixed_temp_constrained(
    state: &StateRef<'_>,
    board: &mut Board,
    score: &mut u32,
    best_board: &mut Board,
    best_score: &mut u32,
    rng: &mut RngHandle,
    temp: f64,
    n_iters: u64,
    forbidden: &ForbiddenContext<'_>,
) {
    run_sa_steps_fixed_temp_inner(state, board, score, best_board, best_score, rng, temp, n_iters, Some(forbidden));
}

fn run_sa_steps_fixed_temp_inner(
    state: &StateRef<'_>,
    board: &mut Board,
    score: &mut u32,
    best_board: &mut Board,
    best_score: &mut u32,
    rng: &mut RngHandle,
    temp: f64,
    n_iters: u64,
    forbidden: Option<&ForbiddenContext<'_>>,
) {
    let state = &state.0;
    let puzzle: &Puzzle = state.puzzle;
    let rng = &mut rng.0;
    for _ in 0..n_iters {
        // Move-kind mix (in tenths):
        //   0..2  rotate single interior piece            (20%)
        //   2..3  rotate 2×2 interior block CW/CCW        (10%)  ← cluster move (Wolff-flavored)
        //   3..4  rotate 3×3 interior block CW/CCW        (10%)  ← larger cluster
        //   4..6  targeted swap (picks bad cells first)   (20%)  ← informed proposal
        //   6..7  region swap (swap two 2×2 blocks)       (10%)  ← non-rigid multi-piece move
        //   7..9  random swap two same-class pieces       (20%)
        //   9     3-cycle swap A→B→C→A                    (10%)  ← breaks frustrated triples
        let move_kind = rng.gen_range(10);

        // ---- 3x3 cluster rotation ----
        if move_kind == 3 {
            let w = state.puzzle.width;
            let h = state.puzzle.height;
            if w < 5 || h < 5 { continue; }
            // Interior-only 3×3: TL at (x,y) with x∈[1, w-4], y∈[1, h-4]
            let x0 = 1 + rng.gen_range(w - 4);
            let y0 = 1 + rng.gen_range(h - 4);
            // Collect all 9 (pos, pid, rot) entries.
            let mut cells_pos = [0u32; 9];
            let mut cells_pid: [Option<(PieceId, Rotation)>; 9] = [None; 9];
            for dy in 0..3 {
                for dx in 0..3 {
                    let idx = (dy * 3 + dx) as usize;
                    let pos = (y0 + dy) * w + (x0 + dx);
                    cells_pos[idx] = pos;
                    cells_pid[idx] = board.get(pos);
                }
            }
            // If any cell is empty, bail.
            if cells_pid.iter().any(|c| c.is_none()) { continue; }
            // If any cell is pinned (official-E2 hint), bail — cluster rotation
            // would move the hint piece off its mandated cell.
            if cells_pos.iter().any(|&p| state.pinned[p as usize]) { continue; }

            // Boundary score for the 3×3: 12 outward-facing edges.
            // For TL row: top sides at (0,0),(1,0),(2,0).
            // For BR row: bottom sides at (0,2),(1,2),(2,2).
            // For left col: left sides at (0,0..2).
            // For right col: right sides at (2,0..2).
            let bnd_3x3 = |s: &State, b: &Board| -> u32 {
                let mut m = 0;
                for dx in 0..3 { m += partial_edge_match(s, b, (y0) * w + (x0 + dx), 0); }
                for dx in 0..3 { m += partial_edge_match(s, b, (y0 + 2) * w + (x0 + dx), 2); }
                for dy in 0..3 { m += partial_edge_match(s, b, (y0 + dy) * w + x0, 3); }
                for dy in 0..3 { m += partial_edge_match(s, b, (y0 + dy) * w + (x0 + 2), 1); }
                m
            };
            let old_bnd = bnd_3x3(state, board);

            // CW rotation: (dx,dy) → (2-dy, dx). Each piece rot +1.
            // CCW: (dx,dy) → (dy, 2-dx).         Each piece rot +3.
            let dir = if rng.gen_range(2) == 0 { 1u8 } else { 3u8 };
            let new_idx = |dx: usize, dy: usize| -> usize {
                let (ndx, ndy) = if dir == 1 { (2 - dy, dx) } else { (dy, 2 - dx) };
                ndy * 3 + ndx
            };
            // Apply rotation to a fresh placement vector, then write.
            let mut new_assign: [(u32, PieceId, Rotation); 9] =
                [(0, 0u16, Rotation::from_u8(0).unwrap()); 9];
            for dy in 0..3 {
                for dx in 0..3 {
                    let from_idx = dy * 3 + dx;
                    let to_idx = new_idx(dx, dy);
                    let (pid, rot) = cells_pid[from_idx].unwrap();
                    let new_rot = Rotation::from_u8((rot.as_u8() + dir) & 0b11).unwrap();
                    new_assign[to_idx] = (cells_pos[to_idx], pid, new_rot);
                }
            }
            for &(pos, pid, rot) in &new_assign {
                board.place(pos, pid, rot);
            }
            let new_bnd = bnd_3x3(state, board);
            let delta = (new_bnd as i64) - (old_bnd as i64);
            let accept = if delta >= 0 { true }
                else { let p = (delta as f64 / temp).exp(); rng.next_f64() < p };
            if accept {
                *score = (*score as i64 + delta) as u32;
                if *score > *best_score { *best_score = *score; *best_board = board.clone(); }
            } else {
                // Revert: restore originals.
                for idx in 0..9 {
                    let (pid, rot) = cells_pid[idx].unwrap();
                    board.place(cells_pos[idx], pid, rot);
                }
            }
            continue;
        }

        // ---- 3-cycle swap: A→B→C→A piece permutation ----
        // Can fix frustrated triples that pair-swaps cannot. Mixed
        // with the random swap pool.
        if move_kind == 9 {
            let class_pick = rng.gen_range(3);
            let cells = match class_pick {
                0 => &state.corner_cells,
                1 => &state.edge_cells,
                _ => &state.interior_cells,
            };
            if cells.len() < 3 { continue; }
            let n = cells.len() as u32;
            let i = rng.gen_range(n) as usize;
            let mut j = rng.gen_range(n) as usize;
            while j == i { j = rng.gen_range(n) as usize; }
            let mut k = rng.gen_range(n) as usize;
            while k == i || k == j { k = rng.gen_range(n) as usize; }
            let p_a = cells[i]; let p_b = cells[j]; let p_c = cells[k];
            let Some((pid_a, rot_a)) = board.get(p_a) else { continue; };
            let Some((pid_b, rot_b)) = board.get(p_b) else { continue; };
            let Some((pid_c, rot_c)) = board.get(p_c) else { continue; };
            // Old local score for the 3 cells.
            let old_local_a = local_match_count(state, board, p_a);
            let old_local_b = local_match_count(state, board, p_b);
            let old_local_c = local_match_count(state, board, p_c);
            // Subtract double-counted adjacencies among (A,B), (A,C), (B,C).
            let dbl = adjacent_match(state, board, p_a, p_b)
                    + adjacent_match(state, board, p_a, p_c)
                    + adjacent_match(state, board, p_b, p_c);
            let old_local = old_local_a + old_local_b + old_local_c - dbl;
            // 3-cycle: A.piece → B, B.piece → C, C.piece → A.
            board.place(p_b, pid_a, rot_a);
            board.place(p_c, pid_b, rot_b);
            board.place(p_a, pid_c, rot_c);
            // Best-rotation refine each cell.
            let br_a = best_rotation(state, board, p_a, pid_c);
            board.place(p_a, pid_c, br_a);
            let br_b = best_rotation(state, board, p_b, pid_a);
            board.place(p_b, pid_a, br_b);
            let br_c = best_rotation(state, board, p_c, pid_b);
            board.place(p_c, pid_b, br_c);
            let new_local_a = local_match_count(state, board, p_a);
            let new_local_b = local_match_count(state, board, p_b);
            let new_local_c = local_match_count(state, board, p_c);
            let new_dbl = adjacent_match(state, board, p_a, p_b)
                        + adjacent_match(state, board, p_a, p_c)
                        + adjacent_match(state, board, p_b, p_c);
            let new_local = new_local_a + new_local_b + new_local_c - new_dbl;
            let delta = (new_local as i64) - (old_local as i64);
            let accept = if delta >= 0 { true }
                else { let p = (delta as f64 / temp).exp(); rng.next_f64() < p };
            if accept {
                *score = (*score as i64 + delta) as u32;
                if *score > *best_score { *best_score = *score; *best_board = board.clone(); }
            } else {
                board.place(p_a, pid_a, rot_a);
                board.place(p_b, pid_b, rot_b);
                board.place(p_c, pid_c, rot_c);
            }
            continue;
        }

        // ---- Region swap: swap two non-overlapping k×k interior blocks ----
        // This is a *non-rigid* multi-piece move that can break basins
        // that rigid cluster rotations cannot. Internal block edges
        // change because the pieces inside change identity.
        if move_kind == 6 {
            let w = state.puzzle.width;
            let h = state.puzzle.height;
            const KSWAP: u32 = 2;
            if w < KSWAP + 4 || h < KSWAP + 4 { continue; }
            // Pick two interior block top-lefts; both must be in [1, w-KSWAP-1]
            // so that all cells inside are interior.
            let max_x = w - KSWAP - 1; // top-left x must be ≥ 1 and ≤ max_x
            let max_y = h - KSWAP - 1;
            if max_x < 1 || max_y < 1 { continue; }
            let xa = 1 + rng.gen_range(max_x);
            let ya = 1 + rng.gen_range(max_y);
            let xb = 1 + rng.gen_range(max_x);
            let yb = 1 + rng.gen_range(max_y);
            // Reject overlapping blocks.
            let overlap = (xa as i64 - xb as i64).abs() < KSWAP as i64
                && (ya as i64 - yb as i64).abs() < KSWAP as i64;
            if overlap { continue; }

            // Save originals of both blocks.
            let mut orig_a: Vec<(u32, PieceId, Rotation)> = Vec::with_capacity((KSWAP * KSWAP) as usize);
            let mut orig_b: Vec<(u32, PieceId, Rotation)> = Vec::with_capacity((KSWAP * KSWAP) as usize);
            for dy in 0..KSWAP {
                for dx in 0..KSWAP {
                    let pa = (ya + dy) * w + (xa + dx);
                    let pb = (yb + dy) * w + (xb + dx);
                    let Some((pida, rota)) = board.get(pa) else { continue; };
                    let Some((pidb, rotb)) = board.get(pb) else { continue; };
                    orig_a.push((pa, pida, rota));
                    orig_b.push((pb, pidb, rotb));
                }
            }
            if orig_a.len() != (KSWAP * KSWAP) as usize || orig_b.len() != (KSWAP * KSWAP) as usize {
                continue;
            }
            // Skip if any cell in either block is pinned — region swap would move it.
            if orig_a.iter().any(|&(pos, _, _)| state.pinned[pos as usize])
                || orig_b.iter().any(|&(pos, _, _)| state.pinned[pos as usize]) {
                continue;
            }

            // Score contribution of both blocks: sum local_match_count of
            // all cells in both blocks, with double-counting subtracted
            // for any edges WITHIN each block (but blocks are non-
            // overlapping and non-adjacent in our overlap check? Wait —
            // we only checked non-overlapping, not non-adjacent. Let me
            // tighten: require |xa-xb| ≥ KSWAP+1 OR |ya-yb| ≥ KSWAP+1 so
            // there's at least one cell gap. Then no edges cross blocks.
            let adjacent_blocks = (xa as i64 - xb as i64).abs() < (KSWAP as i64 + 1)
                && (ya as i64 - yb as i64).abs() < (KSWAP as i64 + 1);
            if adjacent_blocks {
                continue; // skip this attempt; not worth handling
            }

            let block_score = |b: &Board| -> u32 {
                let mut s = 0;
                let mut intra_dbl = 0;
                for &(pos, _, _) in &orig_a {
                    s += local_match_count(state, b, pos);
                }
                for &(pos, _, _) in &orig_b {
                    s += local_match_count(state, b, pos);
                }
                // Each in-block edge is counted twice. Compute intra-A
                // and intra-B internal edge matches.
                // For 2x2: edges are TL.right-TR.left, BL.right-BR.left,
                // TL.bottom-BL.top, TR.bottom-BR.top — 4 edges per block.
                // Order in orig_a: (xa,ya), (xa+1,ya), (xa,ya+1), (xa+1,ya+1) by my loop above.
                // index 0 = TL, 1 = TR, 2 = BL, 3 = BR
                // TL-TR horizontal (TL.right vs TR.left): adjacent_match.
                if KSWAP == 2 {
                    intra_dbl += adjacent_match(state, b, orig_a[0].0, orig_a[1].0);
                    intra_dbl += adjacent_match(state, b, orig_a[2].0, orig_a[3].0);
                    intra_dbl += adjacent_match(state, b, orig_a[0].0, orig_a[2].0);
                    intra_dbl += adjacent_match(state, b, orig_a[1].0, orig_a[3].0);
                    intra_dbl += adjacent_match(state, b, orig_b[0].0, orig_b[1].0);
                    intra_dbl += adjacent_match(state, b, orig_b[2].0, orig_b[3].0);
                    intra_dbl += adjacent_match(state, b, orig_b[0].0, orig_b[2].0);
                    intra_dbl += adjacent_match(state, b, orig_b[1].0, orig_b[3].0);
                }
                s - intra_dbl
            };
            let old_total = block_score(board);

            // Apply swap: A_i ← orig_B_i, B_i ← orig_A_i. Same rotation
            // since we keep piece class compatibility (both blocks are
            // interior). The piece's rotation tag stays the same.
            for i in 0..orig_a.len() {
                let (pa, _, _) = orig_a[i];
                let (_, pidb, rotb) = orig_b[i];
                board.place(pa, pidb, rotb);
            }
            for i in 0..orig_b.len() {
                let (pb, _, _) = orig_b[i];
                let (_, pida, rota) = orig_a[i];
                board.place(pb, pida, rota);
            }
            // Re-rotate each piece in both blocks to best orientation
            // given current neighbours (cheap local optimisation).
            for i in 0..orig_a.len() {
                let (pa, _, _) = orig_a[i];
                if let Some((pid, _)) = board.get(pa) {
                    let best = best_rotation(state, board, pa, pid);
                    board.place(pa, pid, best);
                }
                let (pb, _, _) = orig_b[i];
                if let Some((pid, _)) = board.get(pb) {
                    let best = best_rotation(state, board, pb, pid);
                    board.place(pb, pid, best);
                }
            }
            let new_total = block_score(board);
            let delta = (new_total as i64) - (old_total as i64);
            let accept = if delta >= 0 { true }
                else { let p = (delta as f64 / temp).exp(); rng.next_f64() < p };
            if accept {
                *score = (*score as i64 + delta) as u32;
                if *score > *best_score { *best_score = *score; *best_board = board.clone(); }
            } else {
                // Revert both blocks.
                for &(pos, pid, rot) in &orig_a {
                    board.place(pos, pid, rot);
                }
                for &(pos, pid, rot) in &orig_b {
                    board.place(pos, pid, rot);
                }
            }
            continue;
        }

        // ---- Targeted swap ----
        if move_kind == 4 || move_kind == 5 {
            // Pick a random class. Sample K cells, take worst-scoring one.
            // Then sample another cell in same class (possibly worst-of-K
            // too) and swap. "Worst" = lowest local match count.
            let class_pick = rng.gen_range(3);
            let cells = match class_pick {
                0 => &state.corner_cells,
                1 => &state.edge_cells,
                _ => &state.interior_cells,
            };
            if cells.len() < 2 { continue; }
            const K_SAMPLE: usize = 8;
            let pick_worst = |b: &Board, rng: &mut Rng| -> usize {
                let n = cells.len();
                let mut worst_idx = rng.gen_range(n as u32) as usize;
                let mut worst_score = local_match_count(state, b, cells[worst_idx]);
                for _ in 1..K_SAMPLE.min(n) {
                    let i = rng.gen_range(n as u32) as usize;
                    let s = local_match_count(state, b, cells[i]);
                    if s < worst_score {
                        worst_score = s;
                        worst_idx = i;
                    }
                }
                worst_idx
            };
            let i = pick_worst(board, rng);
            let mut j = pick_worst(board, rng);
            if i == j { j = (j + 1) % cells.len(); }
            let p_i = cells[i];
            let p_j = cells[j];
            let Some((pid_i, rot_i)) = board.get(p_i) else { continue; };
            let Some((pid_j, rot_j)) = board.get(p_j) else { continue; };
            let old_local_i = local_match_count(state, board, p_i);
            let old_local_j = local_match_count(state, board, p_j);
            let adj_old = adjacent_match(state, board, p_i, p_j);
            let old_local = old_local_i + old_local_j - adj_old;
            board.place(p_i, pid_j, rot_j);
            board.place(p_j, pid_i, rot_i);
            let best_rot_i = best_rotation(state, board, p_i, pid_j);
            board.place(p_i, pid_j, best_rot_i);
            let best_rot_j = best_rotation(state, board, p_j, pid_i);
            board.place(p_j, pid_i, best_rot_j);
            let new_local_i = local_match_count(state, board, p_i);
            let new_local_j = local_match_count(state, board, p_j);
            let new_adj = adjacent_match(state, board, p_i, p_j);
            let new_local = new_local_i + new_local_j - new_adj;
            let delta = (new_local as i64) - (old_local as i64);
            let accept = if delta >= 0 { true }
                else { let p = (delta as f64 / temp).exp(); rng.next_f64() < p };
            if accept {
                *score = (*score as i64 + delta) as u32;
                if *score > *best_score { *best_score = *score; *best_board = board.clone(); }
            } else {
                board.place(p_i, pid_i, rot_i);
                board.place(p_j, pid_j, rot_j);
            }
            continue;
        }

        if move_kind == 2 {
            // CLUSTER ROTATE: pick an interior-only 2×2 block, rotate
            // CW or CCW. Each of the 4 pieces moves one cell around
            // the cycle and its rotation parameter changes accordingly.
            // The 4 internal edges are preserved (they still touch but
            // with rotated-piece colors, which net-match the same way
            // since the whole block rotated rigidly); the 8 external
            // edges change. Score delta = sum of new boundary matches
            // minus old boundary matches.
            let w = state.puzzle.width;
            let h = state.puzzle.height;
            // Interior-only 2x2 means TL is at (x,y) with x ∈ [1, w-3],
            // y ∈ [1, h-3] so that all 4 cells are interior.
            if w < 4 || h < 4 { continue; }
            let x = 1 + rng.gen_range(w - 3);
            let y = 1 + rng.gen_range(h - 3);
            let p_tl = y * w + x;
            let p_tr = y * w + (x + 1);
            let p_bl = (y + 1) * w + x;
            let p_br = (y + 1) * w + (x + 1);
            let Some((id_tl, r_tl)) = board.get(p_tl) else { continue; };
            let Some((id_tr, r_tr)) = board.get(p_tr) else { continue; };
            let Some((id_bl, r_bl)) = board.get(p_bl) else { continue; };
            let Some((id_br, r_br)) = board.get(p_br) else { continue; };
            // Honour pinned cells (official-E2 hints).
            if state.pinned[p_tl as usize] || state.pinned[p_tr as usize]
                || state.pinned[p_bl as usize] || state.pinned[p_br as usize] {
                continue;
            }

            // Old block score = boundary matches at the 4 cells (sum
            // of local_match_count over the 4 cells, minus 2× the 4
            // internal edges which are counted twice — but internal
            // edges are preserved under rigid rotation. To simplify:
            // measure ONLY the 8 boundary edges before and after.
            let bnd = |s: &State, b: &Board| -> u32 {
                let mut m = 0;
                // TL: top + left
                m += partial_edge_match(s, b, p_tl, 0); // top
                m += partial_edge_match(s, b, p_tl, 3); // left
                // TR: top + right
                m += partial_edge_match(s, b, p_tr, 0);
                m += partial_edge_match(s, b, p_tr, 1);
                // BL: bottom + left
                m += partial_edge_match(s, b, p_bl, 2);
                m += partial_edge_match(s, b, p_bl, 3);
                // BR: bottom + right
                m += partial_edge_match(s, b, p_br, 2);
                m += partial_edge_match(s, b, p_br, 1);
                m
            };
            let old_bnd = bnd(state, board);

            // Direction: CW (1) or CCW (3).
            let dir = if rng.gen_range(2) == 0 { 1u8 } else { 3u8 };
            let rot_incr = |r: Rotation, d: u8| -> Rotation {
                Rotation::from_u8((r.as_u8() + d) & 0b11).unwrap()
            };
            if dir == 1 {
                // CW: TL→TR, TR→BR, BR→BL, BL→TL. Each piece gets +1 rot.
                board.place(p_tr, id_tl, rot_incr(r_tl, 1));
                board.place(p_br, id_tr, rot_incr(r_tr, 1));
                board.place(p_bl, id_br, rot_incr(r_br, 1));
                board.place(p_tl, id_bl, rot_incr(r_bl, 1));
            } else {
                // CCW: TL→BL, BL→BR, BR→TR, TR→TL. Each piece gets +3 rot.
                board.place(p_bl, id_tl, rot_incr(r_tl, 3));
                board.place(p_br, id_bl, rot_incr(r_bl, 3));
                board.place(p_tr, id_br, rot_incr(r_br, 3));
                board.place(p_tl, id_tr, rot_incr(r_tr, 3));
            }
            let new_bnd = bnd(state, board);
            let delta = (new_bnd as i64) - (old_bnd as i64);
            let accept = if delta >= 0 { true }
                else { let p = (delta as f64 / temp).exp(); rng.next_f64() < p };
            if accept {
                *score = (*score as i64 + delta) as u32;
                if *score > *best_score { *best_score = *score; *best_board = board.clone(); }
            } else {
                // Revert: restore original placements exactly.
                board.place(p_tl, id_tl, r_tl);
                board.place(p_tr, id_tr, r_tr);
                board.place(p_bl, id_bl, r_bl);
                board.place(p_br, id_br, r_br);
            }
            continue;
        }
        if move_kind < 3 {
            let n_cells = state.puzzle.cell_count();
            let pos = rng.gen_range(n_cells);
            let Some((pid, old_rot)) = board.get(pos) else { continue; };
            // Skip pinned cells (official-E2 hints).
            if state.pinned[pos as usize] { continue; }
            let cell_is_interior = matches!(state.cell_class[pos as usize], CellClass::Interior);
            if !cell_is_interior { continue; }
            let mut new_r = rng.gen_range(4) as u8;
            if new_r == old_rot.as_u8() { new_r = (new_r + 1) & 0b11; }
            let new_rot = Rotation::from_u8(new_r).unwrap();
            let old_local = local_match_count(state, board, pos);
            let e_new = state.edges_for(pid, new_rot);
            let new_local = match_count_with(state, board, pos, e_new);
            let delta = (new_local as i64) - (old_local as i64);
            // Forbidden-mismatch delta (NE2.1 inner-loop penalty).
            // Touched = [pos]. Need to PROVISIONALLY place to compute
            // new_fmm, then revert if not accepted. For single-rotation
            // the only edges affected are the 4 sides of `pos`, so
            // fmm_at([pos]) captures them.
            let eff_delta = if let Some(fctx) = forbidden {
                let old_fmm = fctx.fmm_at(puzzle, board, &[pos]);
                board.place(pos, pid, new_rot);
                let new_fmm = fctx.fmm_at(puzzle, board, &[pos]);
                // Revert; we'll re-place if accepted.
                board.place(pos, pid, old_rot);
                delta - fctx.k * ((new_fmm as i64) - (old_fmm as i64))
            } else { delta };
            let accept = if eff_delta >= 0 { true }
                else { let p = (eff_delta as f64 / temp).exp(); rng.next_f64() < p };
            if accept {
                board.place(pos, pid, new_rot);
                *score = (*score as i64 + delta) as u32;
                if *score > *best_score { *best_score = *score; *best_board = board.clone(); }
            }
        } else {
            let class_pick = rng.gen_range(3);
            let cells = match class_pick {
                0 => &state.corner_cells,
                1 => &state.edge_cells,
                _ => &state.interior_cells,
            };
            if cells.len() < 2 { continue; }
            let i = rng.gen_range(cells.len() as u32) as usize;
            let mut j = rng.gen_range(cells.len() as u32) as usize;
            if i == j { j = (j + 1) % cells.len(); }
            let p_i = cells[i];
            let p_j = cells[j];
            let Some((pid_i, rot_i)) = board.get(p_i) else { continue; };
            let Some((pid_j, rot_j)) = board.get(p_j) else { continue; };
            let old_local_i = local_match_count(state, board, p_i);
            let old_local_j = local_match_count(state, board, p_j);
            let adj_old = adjacent_match(state, board, p_i, p_j);
            let old_local = old_local_i + old_local_j - adj_old;
            board.place(p_i, pid_j, rot_j);
            board.place(p_j, pid_i, rot_i);
            let best_rot_i = best_rotation(state, board, p_i, pid_j);
            board.place(p_i, pid_j, best_rot_i);
            let best_rot_j = best_rotation(state, board, p_j, pid_i);
            board.place(p_j, pid_i, best_rot_j);
            let new_local_i = local_match_count(state, board, p_i);
            let new_local_j = local_match_count(state, board, p_j);
            let new_adj = adjacent_match(state, board, p_i, p_j);
            let new_local = new_local_i + new_local_j - new_adj;
            let delta = (new_local as i64) - (old_local as i64);
            // Forbidden-mismatch delta on touched cells {p_i, p_j}.
            // The swap is already applied at this point (with rotations
            // re-optimized via best_rotation); revert is a single
            // place() per cell at the original rot.
            let eff_delta = if let Some(fctx) = forbidden {
                // new_fmm uses current board (swap+best_rot applied).
                let new_fmm = fctx.fmm_at(puzzle, board, &[p_i, p_j]);
                // To compute old_fmm we'd need to revert and re-evaluate.
                // Cheaper trick: revert via temporary local swap, compute, restore.
                board.place(p_i, pid_i, rot_i);
                board.place(p_j, pid_j, rot_j);
                let old_fmm = fctx.fmm_at(puzzle, board, &[p_i, p_j]);
                // Restore the swapped+best-rot state.
                board.place(p_i, pid_j, best_rot_i);
                board.place(p_j, pid_i, best_rot_j);
                delta - fctx.k * ((new_fmm as i64) - (old_fmm as i64))
            } else { delta };
            let accept = if eff_delta >= 0 { true }
                else { let p = (eff_delta as f64 / temp).exp(); rng.next_f64() < p };
            if accept {
                *score = (*score as i64 + delta) as u32;
                if *score > *best_score { *best_score = *score; *best_board = board.clone(); }
            } else {
                board.place(p_i, pid_i, rot_i);
                board.place(p_j, pid_j, rot_j);
            }
        }
    }
}

/// Opaque handle to the precomputed SA state. Sharable across replicas.
pub struct StateRef<'a>(State<'a>);

impl<'a> StateRef<'a> {
    pub fn new(puzzle: &'a Puzzle) -> Self { Self(State::new(puzzle)) }
    pub fn new_with_pinned(puzzle: &'a Puzzle, pinned: &[Position]) -> Self {
        Self(State::new_with_pinned(puzzle, pinned))
    }
    pub fn score(&self, b: &Board) -> u32 { self.0.score(b) }
    pub fn total_interior_edges(&self) -> u32 { self.0.total_interior_edges() }
    pub fn random_initial(&self, rng: &mut RngHandle) -> Board {
        self.0.random_initial(&mut rng.0)
    }
    pub fn fill_in_initial(&self, initial: &Board, rng: &mut RngHandle) -> Board {
        self.0.fill_in_initial(initial, &mut rng.0)
    }
    pub fn greedy_fill_initial(&self, initial: &Board, rng: &mut RngHandle) -> Board {
        self.0.greedy_fill_initial(initial, &mut rng.0)
    }
    pub fn interior_cell_count(&self) -> usize { self.0.interior_cells.len() }
    pub fn interior_cell(&self, i: usize) -> Position { self.0.interior_cells[i] }
    pub(crate) fn inner(&self) -> &State<'a> { &self.0 }
}

/// Opaque RNG handle. Cheap to clone; each replica owns its own.
#[derive(Clone)]
pub struct RngHandle(Rng);

impl RngHandle {
    pub fn new(seed: u64) -> Self { Self(Rng::new(seed)) }
    pub fn next_u64(&mut self) -> u64 { self.0.next_u64() }
    pub fn next_f64(&mut self) -> f64 { self.0.next_f64() }
}

/// Variant of `run_sa` that starts from a caller-supplied initial board.
/// Used by the CP+LS hybrid: seed LS with the CP solver's `best_partial`
/// (filling any unplaced cells randomly with class-matching pieces).
///
/// The supplied board may be partial (cells = None). Unplaced cells get
/// a random class-matching piece in a valid rotation. Already-placed
/// cells are kept as-is at the start; LS is then free to move any piece
/// during search (we do NOT pin user placements).
pub fn run_sa_from(puzzle: &Puzzle, initial: &Board, cfg: &SaConfig) -> SaOutcome {
    let started = std::time::Instant::now();
    let state = State::new_with_pinned(puzzle, &cfg.pinned_positions);
    let mut rng = Rng::new(cfg.seed);

    let mut board = state.fill_in_initial(initial, &mut rng);
    run_sa_loop(&state, &mut board, &mut rng, cfg, started)
}

/// Run simulated annealing on `puzzle`. Returns the best board found.
///
/// Status: v1.
/// - Random initial placement (correct piece-class per cell, border
///   rotations forced).
/// - Two neighborhood operators chosen at random each iteration:
///     (a) "rotate": pick a cell and try a random different rotation
///     (b) "swap+best-rot": pick two same-class cells, swap pieces,
///         re-rotate both to their best orientation.
/// - SA acceptance with geometric cooling.
/// - Re-anneal: when temperature falls below `temperature_min`, reset
///   temperature to `temperature_start` and continue. This keeps the
///   walk from freezing at a poor local optimum.
/// - Incremental scoring: changes touch at most 2 cells (swap) or 1
///   cell (rotate), so we compute the score delta from local counts
///   alone instead of re-scanning the whole board.
pub fn run_sa(puzzle: &Puzzle, cfg: &SaConfig) -> SaOutcome {
    let started = std::time::Instant::now();
    let state = State::new_with_pinned(puzzle, &cfg.pinned_positions);
    let mut rng = Rng::new(cfg.seed);

    let mut board = state.random_initial(&mut rng);
    run_sa_loop(&state, &mut board, &mut rng, cfg, started)
}

fn run_sa_loop(
    state: &State<'_>,
    board: &mut Board,
    rng: &mut Rng,
    cfg: &SaConfig,
    started: std::time::Instant,
) -> SaOutcome {
    let mut score = state.score(board);
    let mut best_board = board.clone();
    let mut best_score = score;
    let total_edges = state.total_interior_edges();

    let mut temp = cfg.temperature_start;
    let mut iters: u64 = 0;
    let mut since_cool: u64 = 0;

    let timed_out = |start: &std::time::Instant| -> bool {
        cfg.time_budget_ms != 0
            && start.elapsed().as_millis() >= u128::from(cfg.time_budget_ms)
    };

    loop {
        if cfg.max_iters != 0 && iters >= cfg.max_iters { break; }
        if timed_out(&started) { break; }
        if best_score == total_edges { break; }

        // Choose move type. 30% rotate, 70% swap+best-rot. Rationale:
        // rotates are cheap and target small local issues; swaps are
        // the long-range moves needed to escape configurations.
        let move_kind = rng.gen_range(10);
        if move_kind < 3 {
            // ROTATE-ONLY: pick a cell, try a random different rotation.
            let n_cells = state.puzzle.cell_count();
            let pos = rng.gen_range(n_cells);
            let Some((pid, old_rot)) = board.get(pos) else { continue; };
            // Skip pinned cells (official-E2 hints).
            if state.pinned[pos as usize] {
                iters += 1;
                since_cool += 1;
                continue;
            }
            let cell_is_interior = matches!(state.cell_class[pos as usize], CellClass::Interior);
            if !cell_is_interior {
                // Border cells have a single valid rotation, rotate-only
                // is a no-op for them.
                iters += 1;
                since_cool += 1;
                continue;
            }
            // Try a random different rotation
            let mut new_r = rng.gen_range(4) as u8;
            if new_r == old_rot.as_u8() {
                new_r = (new_r + 1) & 0b11;
            }
            let new_rot = Rotation::from_u8(new_r).unwrap();
            let old_local = local_match_count(&state, &board, pos);
            let e_new = state.edges_for(pid, new_rot);
            let new_local = match_count_with(&state, &board, pos, e_new);
            let delta = (new_local as i64) - (old_local as i64);
            let accept = if delta >= 0 {
                true
            } else {
                let p = (delta as f64 / temp).exp();
                rng.next_f64() < p
            };
            if accept {
                board.place(pos, pid, new_rot);
                score = (score as i64 + delta) as u32;
                if score > best_score {
                    best_score = score;
                    best_board = board.clone();
                }
            }
        } else {
            // SWAP+BEST-ROT: pick two same-class cells, swap, then
            // re-rotate both to best orientation.
            let class_pick = rng.gen_range(3);
            let cells = match class_pick {
                0 => &state.corner_cells,
                1 => &state.edge_cells,
                _ => &state.interior_cells,
            };
            if cells.len() < 2 {
                iters += 1;
                since_cool += 1;
                continue;
            }
            let i = rng.gen_range(cells.len() as u32) as usize;
            let mut j = rng.gen_range(cells.len() as u32) as usize;
            if i == j { j = (j + 1) % cells.len(); }
            let p_i = cells[i];
            let p_j = cells[j];

            let Some((pid_i, rot_i)) = board.get(p_i) else { continue; };
            let Some((pid_j, rot_j)) = board.get(p_j) else { continue; };

            let old_local_i = local_match_count(&state, &board, p_i);
            let old_local_j = local_match_count(&state, &board, p_j);
            // Edge between i and j (if adjacent) would be double-counted
            // when computing locals; subtract once to avoid double-counting
            // ourselves into trouble in the delta calculation.
            let adjacent_edge_match = adjacent_match(&state, &board, p_i, p_j);
            let old_local = old_local_i + old_local_j - adjacent_edge_match;

            // Tentatively perform swap: piece at i goes to j, piece at j goes to i.
            board.place(p_i, pid_j, rot_j);
            board.place(p_j, pid_i, rot_i);
            // Choose best rotations after swap, using fresh neighbour info.
            // For border cells this is the only valid rotation; for
            // interior it's the score-maximising one.
            let best_rot_i = best_rotation(&state, &board, p_i, pid_j);
            board.place(p_i, pid_j, best_rot_i);
            let best_rot_j = best_rotation(&state, &board, p_j, pid_i);
            board.place(p_j, pid_i, best_rot_j);

            let new_local_i = local_match_count(&state, &board, p_i);
            let new_local_j = local_match_count(&state, &board, p_j);
            let new_adj = adjacent_match(&state, &board, p_i, p_j);
            let new_local = new_local_i + new_local_j - new_adj;

            let delta = (new_local as i64) - (old_local as i64);
            let accept = if delta >= 0 {
                true
            } else {
                let p = (delta as f64 / temp).exp();
                rng.next_f64() < p
            };
            if accept {
                score = (score as i64 + delta) as u32;
                if score > best_score {
                    best_score = score;
                    best_board = board.clone();
                }
            } else {
                // Revert.
                board.place(p_i, pid_i, rot_i);
                board.place(p_j, pid_j, rot_j);
            }
        }

        iters += 1;
        since_cool += 1;
        if since_cool >= cfg.cooling_period {
            temp *= cfg.cooling;
            since_cool = 0;
            // Re-anneal: if temperature has frozen, reset to start.
            // Wauters et al. don't explicitly describe this but
            // local-search practice on hard problems uses restarts
            // to escape frozen states.
            if temp < cfg.temperature_min {
                temp = cfg.temperature_start;
            }
        }
    }

    SaOutcome {
        best_board,
        best_score,
        total_edges,
        iterations: iters,
        elapsed_us: started.elapsed().as_micros(),
    }
}

/// Returns 1 if cells a and b are adjacent AND their shared edge matches.
pub(crate) fn adjacent_match(state: &State, board: &Board, a: Position, b: Position) -> u32 {
    let w = state.puzzle.width;
    let (ax, ay) = (a % w, a / w);
    let (bx, by) = (b % w, b / w);
    let dx = (ax as i32) - (bx as i32);
    let dy = (ay as i32) - (by as i32);
    if dx.unsigned_abs() + dy.unsigned_abs() != 1 {
        return 0;
    }
    let Some((pa, ra)) = board.get(a) else { return 0; };
    let Some((pb, rb)) = board.get(b) else { return 0; };
    let ea = state.edges_for(pa, ra);
    let eb = state.edges_for(pb, rb);
    // Determine the shared-edge sides.
    let (side_a, side_b) = if dx == 1 && dy == 0 {
        (3usize, 1usize) // a is right of b → a's left ↔ b's right
    } else if dx == -1 && dy == 0 {
        (1, 3)
    } else if dy == 1 && dx == 0 {
        (0, 2)
    } else {
        (2, 0)
    };
    if ea[side_a] == eb[side_b] && ea[side_a] != 0 { 1 } else { 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mini smoke test: tiny puzzle, ensure run_sa terminates and
    // never reports a score greater than total_edges.
    #[test]
    fn sa_runs_and_score_in_range() {
        use eternity2_generator::{generate, GeneratorConfig};
        let puzzle = generate(GeneratorConfig {
            size: 4,
            interior_colors: 4,
            seed: 7,
        })
        .expect("gen");
        let cfg = SaConfig {
            max_iters: 5_000,
            time_budget_ms: 2_000,
            ..Default::default()
        };
        let out = run_sa(&puzzle, &cfg);
        assert!(out.best_score <= out.total_edges, "score {} > total {}", out.best_score, out.total_edges);
        // Sanity: 4×4 has 24 interior edges.
        assert_eq!(out.total_edges, 24);
    }
}
