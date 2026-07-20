//! The recursive depth-first backtracker that every `Engine`-kind variant runs.
//!
//! One function, parameterised by a [`Spec`]. It fills cells in the variant's
//! path order, trying candidate pieces in the variant's value order, pruning
//! with the variant's propagator, and tolerating mismatches under the variant's
//! break policy. It keeps the best board seen (by matched-edge score) so a
//! timed-out run still reports its best partial — the anytime behaviour the
//! record engines rely on.
//!
//! Statistics ([`e2_core::SearchStats`]) are updated inline: nodes per
//! attempted placement, backtracks per retreat, max depth, depth at timeout,
//! and the break count of the best board.

use std::time::Instant;

use e2_core::{Board, Pieces, SearchStats, N, W};
use e2_io::Instance;

use crate::geometry::Geometry;
use crate::strategy::breaks::BreakPolicy;
use crate::strategy::propagate::Propagator;
use crate::strategy::value::ValueOrder;
use crate::Spec;

/// How to run: the time budget and the seed (for value orders that use it).
#[derive(Debug, Clone, Copy)]
pub struct RunConfig {
    pub budget_ms: u64,
    pub seed: u64,
    /// Optional hard cap on search nodes, independent of the time budget. `None`
    /// means "time only". Used for deterministic, wall-clock-independent testing:
    /// a fixed node count yields a bit-identical board for a given seed, so an
    /// optimization to the node loop can be proved to preserve behaviour by
    /// comparing the board hash at a fixed `max_nodes` before and after.
    pub max_nodes: Option<u64>,
}

impl RunConfig {
    /// A time-only config (no node cap), the normal grid mode.
    #[must_use]
    pub const fn timed(budget_ms: u64, seed: u64) -> Self {
        Self { budget_ms, seed, max_nodes: None }
    }
}

/// The result of one run: the best board and the statistics behind it.
pub struct RunResult {
    pub best: Board,
    pub best_score: u32,
    pub stats: SearchStats,
    pub elapsed_s: f64,
}

impl RunResult {
    /// Package the best board through the instance's canonical output contract.
    #[must_use]
    pub fn output(&self, inst: &Instance) -> e2_io::SolveOutput {
        inst.finish(&self.best)
    }
}

/// splitmix64 → xoshiro256** — a fast, decent PRNG for value-order shuffles and
/// opening jitter. Deterministic given the seed.
struct Rng {
    s: [u64; 4],
}
impl Rng {
    fn new(seed: u64) -> Self {
        let mut z = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut next = || {
            z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut x = z;
            x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            x ^ (x >> 31)
        };
        Self { s: [next(), next(), next(), next()] }
    }
    fn next_u64(&mut self) -> u64 {
        let r = self.s[0].wrapping_add(self.s[3]).rotate_left(23).wrapping_add(self.s[0]);
        let t = self.s[1] << 17;
        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];
        self.s[2] ^= t;
        self.s[3] = self.s[3].rotate_left(45);
        r
    }
    fn shuffle(&mut self, v: &mut [u32]) {
        for i in (1..v.len()).rev() {
            let j = (self.next_u64() % (i as u64 + 1)) as usize;
            v.swap(i, j);
        }
    }
}

/// The mutable state carried through the recursion.
struct SearchState<'a> {
    geom: &'a Geometry,
    spec: &'a Spec,
    board: Board,
    /// Resolved URDL edges per cell, kept in lockstep with `board` so the hot
    /// loop reads neighbour colors as plain array lookups instead of resolving
    /// a rotation each time. `[255;4]` marks an empty cell.
    cell_edges: [[u8; 4]; N],
    /// Count of incident broken (mismatched, non-border) interior edges per
    /// cell. Maintained only by the break search, so the non-adjacency rule can
    /// be enforced on BOTH endpoints of a broken edge, not just the cell being
    /// placed.
    cell_breaks: [u8; N],
    /// `used[piece_id]` — is this piece already placed?
    used: Vec<bool>,
    /// The static fill sequence (empty for a dynamic order).
    seq: Vec<usize>,
    /// Rare-color rank per color, for `ValueOrder::RareColorFirst` (lower =
    /// rarer). Empty unless that value order is used.
    color_rank: Vec<u32>,
    stats: SearchStats,
    best: Board,
    best_score: u32,
    /// Cells filled at the current search frontier. Kept live so that when the
    /// clock expires we record where the search actually WAS (the frontier),
    /// not its high-water mark — the same "depth at timeout" the codegen engine
    /// reports, so the two are comparable.
    cur_depth: u32,
    /// Interior breaks on the best board (break variants only). Tracked from the
    /// break search's own budget accounting, not inferred from the score, so a
    /// timed-out partial reports its true break count rather than the whole
    /// unmatched-edge deficit.
    best_breaks: u32,
    /// Wall-clock deadline; checked periodically on the hot path.
    deadline: Instant,
    start: Instant,
    timed_out: bool,
    /// Optional node cap for deterministic testing (see [`RunConfig::max_nodes`]).
    max_nodes: Option<u64>,
    rng: Rng,
}

/// Run the DFS described by `spec` on `inst` within the budget.
#[must_use]
pub fn run(inst: &Instance, spec: &Spec, cfg: RunConfig) -> RunResult {
    let pieces = &inst.pieces;
    let geom = Geometry::build(pieces);
    let seed_board = inst.seed_board();

    let mut used = vec![false; pieces.len()];
    for h in &inst.hints {
        used[h.piece as usize] = true;
    }

    let mut pinned = vec![false; N];
    for h in &inst.hints {
        pinned[h.pos as usize] = true;
    }
    let seq = if spec.path.is_dynamic() {
        Vec::new()
    } else {
        spec.path.sequence(&pinned)
    };

    let color_rank = if spec.value == ValueOrder::RareColorFirst {
        rare_color_ranks(pieces, geom.colors)
    } else {
        Vec::new()
    };

    let start = Instant::now();
    let deadline = start + std::time::Duration::from_millis(cfg.budget_ms);
    let initial_score = e2_core::score_board(&seed_board, pieces);

    // Seed the resolved-edge cache from the pinned board.
    let mut cell_edges = [[255u8; 4]; N];
    for pos in 0..N {
        if let Some((pid, rot)) = seed_board.piece_at(pos) {
            cell_edges[pos] = pieces.get(pid).unwrap().rotated(rot);
        }
    }

    let mut st = SearchState {
        geom: &geom,
        spec,
        board: seed_board.clone(),
        cell_edges,
        cell_breaks: [0; N],
        used,
        seq,
        color_rank,
        stats: SearchStats::new(),
        best: seed_board,
        best_score: initial_score,
        cur_depth: inst.hints.len() as u32,
        best_breaks: 0,
        deadline,
        start,
        timed_out: false,
        max_nodes: cfg.max_nodes,
        rng: Rng::new(cfg.seed),
    };

    // Depth 0 = number of pre-pinned cells; the search fills the rest. The
    // running matched-edge count starts at the pinned board's score.
    let placed = inst.hints.len() as u32;
    if spec.breaks.allows_breaks() {
        // Break variants use a fixed path and no propagator (a local propagator
        // is unsound under a global break budget). The break count is threaded
        // like the score.
        st.search_break(0, placed, initial_score, 0);
    } else if spec.path.is_dynamic() {
        st.search_dynamic(placed, initial_score);
    } else {
        st.search_static(0, placed, initial_score);
    }

    let elapsed_s = st.start.elapsed().as_secs_f64();
    // The TRUE interior-break count on the best board (0 for strict variants),
    // taken from the break search's own accounting — NOT `max_score - score`,
    // which on a timed-out partial would also count every still-empty edge.
    st.stats.breaks = st.best_breaks;
    RunResult {
        best: st.best,
        best_score: st.best_score,
        stats: st.stats,
        elapsed_s,
    }
}

impl SearchState<'_> {
    /// Cheap wall-clock guard. The clock is sampled every 256 nodes: at the
    /// slowest variant's ~6k nodes/s that is ~40 ms of granularity, so no
    /// variant can overrun the shared budget by more than a small fraction of a
    /// second — the fast and slow families are held to the same deadline. On
    /// expiry we record the CURRENT frontier depth (not the high-water mark).
    #[inline]
    fn out_of_time(&mut self) -> bool {
        if self.timed_out {
            return true;
        }
        // Optional node cap (deterministic, wall-clock-independent testing).
        if let Some(cap) = self.max_nodes {
            if self.stats.nodes >= cap {
                self.timed_out = true;
                self.stats.depth_at_timeout = self.cur_depth;
                return true;
            }
        }
        if self.stats.nodes & 0xFF == 0 && Instant::now() >= self.deadline {
            self.timed_out = true;
            self.stats.depth_at_timeout = self.cur_depth;
        }
        self.timed_out
    }

    /// Place an oriented piece and keep every derived structure in lockstep:
    /// the board, the used-set, and the resolved-edge cache.
    #[inline]
    fn place(&mut self, pos: usize, piece: u16, rot: u8, edges: [u8; 4]) {
        self.board.place(pos, piece, rot);
        self.used[piece as usize] = true;
        self.cell_edges[pos] = edges;
    }

    /// Undo a [`Self::place`].
    #[inline]
    fn unplace(&mut self, pos: usize, piece: u16) {
        self.board.clear(pos);
        self.used[piece as usize] = false;
        self.cell_edges[pos] = [NO_CONSTRAINT; 4];
    }

    /// Update the best board if `score` (matched edges of the current board)
    /// beats the incumbent, remembering how many interior breaks that board
    /// carries so the reported break count is the true count on the best board,
    /// not the whole unmatched-edge deficit of a partial.
    #[inline]
    fn consider_best(&mut self, score: u32, breaks: u32) {
        if score > self.best_score {
            self.best_score = score;
            self.best_breaks = breaks;
            self.best = self.board.clone();
            // Stamp the first full solution: the node count and wall time at the
            // moment we first reach the maximum matched-edge score. The search
            // continues (it does not early-stop), but this records the size of
            // the tree explored to solve — the hint-geometry study's metric.
            if score == e2_core::MAX_SCORE_16 && self.stats.nodes_to_solution == 0 {
                self.stats.nodes_to_solution = self.stats.nodes;
                self.stats.secs_to_solution = self.start.elapsed().as_secs_f64();
            }
        }
    }

    // ----- static-order search (row-major, border-first, spiral, comb) -----

    /// `idx` is the position in `self.seq`; `placed` is the total filled count
    /// (pins + progress), used for depth stats and break gating. `partial` is
    /// the running matched-edge score of the current board, threaded
    /// incrementally so the hot loop never re-scores the whole board.
    fn search_static(&mut self, idx: usize, placed: u32, partial: u32) {
        self.cur_depth = placed;
        if self.out_of_time() {
            return;
        }
        self.stats.observe_depth(placed);
        self.consider_best(partial, 0);

        if idx >= self.seq.len() {
            return; // board complete; `partial` is exact and already recorded
        }

        let pos = self.seq[idx];
        let (u, l, d, r) = self.neighbor_colors(pos);
        let mut any = false;

        // `geom` is borrowed from the Geometry (not from `self`), so the
        // candidate slice can be held while other `self` fields are mutated —
        // this keeps the Insertion hot path allocation-free.
        let geom = self.geom;
        if self.spec.value == ValueOrder::Insertion {
            for &oi in geom.by_up_left_or(u, l) {
                if self.out_of_time() {
                    return;
                }
                let o = geom.oriented(oi);
                if self.used[o.piece as usize] || !self.fits(pos, o.edges, u, l, d, r) {
                    continue;
                }
                self.stats.nodes += 1;
                any = true;
                let gain = match_gain(o.edges, u, l, d, r);
                self.place(pos, o.piece, o.rot, o.edges);
                if self.propagate_ok(pos) {
                    self.search_static(idx + 1, placed + 1, partial + gain);
                }
                self.unplace(pos, o.piece);
                if self.timed_out {
                    return;
                }
            }
        } else {
            // Reordered value orders need an owned snapshot to sort/shuffle.
            let mut ordered: Vec<u32> = self.candidates(u, l).to_vec();
            self.reorder(&mut ordered);
            for oi in ordered {
                if self.out_of_time() {
                    return;
                }
                let o = self.geom.oriented(oi);
                if self.used[o.piece as usize] || !self.fits(pos, o.edges, u, l, d, r) {
                    continue;
                }
                self.stats.nodes += 1;
                any = true;
                let gain = match_gain(o.edges, u, l, d, r);
                self.place(pos, o.piece, o.rot, o.edges);
                if self.propagate_ok(pos) {
                    self.search_static(idx + 1, placed + 1, partial + gain);
                }
                self.unplace(pos, o.piece);
                if self.timed_out {
                    return;
                }
            }
        }

        if !any {
            self.stats.backtracks += 1;
        }
    }

    // ----- dynamic-order search (MRV) -----

    fn search_dynamic(&mut self, placed: u32, partial: u32) {
        self.cur_depth = placed;
        if self.out_of_time() {
            return;
        }
        self.stats.observe_depth(placed);
        self.consider_best(partial, 0);

        if placed as usize >= N {
            return;
        }

        // Pick the empty cell with the fewest candidates (MRV), border-first on
        // ties. `None` means some empty cell has zero candidates (a dead end).
        let Some(pos) = self.mrv_cell() else {
            self.stats.backtracks += 1;
            return;
        };

        let (u, l, d, r) = self.neighbor_colors(pos);
        let mut ordered: Vec<u32> = self.candidates(u, l).to_vec();
        self.reorder(&mut ordered);

        let mut any = false;
        for oi in ordered {
            if self.out_of_time() {
                return;
            }
            let o = self.geom.oriented(oi);
            if self.used[o.piece as usize] {
                continue;
            }
            if !self.fits(pos, o.edges, u, l, d, r) {
                continue;
            }
            self.stats.nodes += 1;
            any = true;
            let gain = match_gain(o.edges, u, l, d, r);
            self.place(pos, o.piece, o.rot, o.edges);
            if self.propagate_ok(pos) {
                self.search_dynamic(placed + 1, partial + gain);
            }
            self.unplace(pos, o.piece);
            if self.timed_out {
                return;
            }
        }
        if !any {
            self.stats.backtracks += 1;
        }
    }

    // ----- break-tolerant search (the elite axis) -----

    /// Depth-gated break search over a fixed fill order. Until the break budget
    /// opens (deep in the board) it behaves strictly and uses the tight
    /// candidate index; once breaks are available it widens the candidate set
    /// and lets a placement mismatch interior edges, spending the global budget
    /// under the non-adjacency rule. `partial` is matched edges; `breaks_used`
    /// is the cumulative interior mismatches so far. Score = 480 − breaks.
    fn search_break(&mut self, idx: usize, placed: u32, partial: u32, breaks_used: u32) {
        self.cur_depth = placed;
        if self.out_of_time() {
            return;
        }
        self.stats.observe_depth(placed);
        self.consider_best(partial, breaks_used);
        if idx >= self.seq.len() {
            return;
        }

        let pos = self.seq[idx];
        let (u, l, d, r) = self.neighbor_colors(pos);
        let budget = self.spec.breaks.budget_at(placed);
        let max_adjacent = match self.spec.breaks {
            BreakPolicy::DepthGated { max_adjacent, .. } => u32::from(max_adjacent),
            BreakPolicy::Strict => 0,
        };

        // In the strict zone (no budget left) use the tight two-sided index; in
        // the break zone widen to all unused pieces so a mismatch is reachable.
        // `geom` is borrowed from the Geometry, not `self`, so the candidate
        // slice can be iterated while `self` fields are mutated — no per-node
        // allocation even in the (hot) break zone.
        let geom = self.geom;
        let cand: &[u32] = if budget > breaks_used {
            geom.all()
        } else {
            geom.by_up_left_or(u, l)
        };

        let mut any = false;
        for &oi in cand {
            if self.out_of_time() {
                return;
            }
            let o = self.geom.oriented(oi);
            if self.used[o.piece as usize] {
                continue;
            }
            // Border-facing edges must always match the border (never break).
            if (u == 0 && o.edges[0] != 0)
                || (l == 0 && o.edges[3] != 0)
                || (d == 0 && o.edges[2] != 0)
                || (r == 0 && o.edges[1] != 0)
            {
                continue;
            }
            // Which ALREADY-PLACED neighbours (in any direction — a pin can sit
            // to the right or below) this placement breaks against. Each broken
            // edge is incident to two cells, so the non-adjacency rule is
            // enforced on both endpoints.
            let (row, col) = Board::rc(pos);
            // For each of the four sides: if the neighbour is placed and its
            // facing colour differs from ours (both non-border), that edge
            // breaks. `broken_neighbours[k]` = (neighbour pos, did-break).
            let broken_neighbours: [(usize, bool); 4] = [
                (pos.wrapping_sub(W), row > 0 && edge_breaks(o.edges[0], u)),
                (pos + 1, col < 15 && edge_breaks(o.edges[1], r)),
                (pos + W, row < 15 && edge_breaks(o.edges[2], d)),
                (pos.wrapping_sub(1), col > 0 && edge_breaks(o.edges[3], l)),
            ];
            let new_breaks: u32 = broken_neighbours.iter().filter(|&&(_, b)| b).count() as u32;

            // This cell's incident breaks, and each broken neighbour's, must all
            // stay within max_adjacent, and the running total within budget.
            if new_breaks > max_adjacent {
                continue;
            }
            let mut neighbour_ok = true;
            for &(npos, broke) in &broken_neighbours {
                if broke && u32::from(self.cell_breaks[npos]) + 1 > max_adjacent {
                    neighbour_ok = false;
                    break;
                }
            }
            if !neighbour_ok || breaks_used + new_breaks > budget {
                continue;
            }

            self.stats.nodes += 1;
            any = true;
            let gain = match_gain(o.edges, u, l, d, r);
            self.place(pos, o.piece, o.rot, o.edges);
            self.cell_breaks[pos] = new_breaks as u8;
            for &(npos, broke) in &broken_neighbours {
                if broke {
                    self.cell_breaks[npos] += 1;
                }
            }

            self.search_break(idx + 1, placed + 1, partial + gain, breaks_used + new_breaks);

            for &(npos, broke) in &broken_neighbours {
                if broke {
                    self.cell_breaks[npos] -= 1;
                }
            }
            self.cell_breaks[pos] = 0;
            self.unplace(pos, o.piece);
            if self.timed_out {
                return;
            }
        }
        if !any {
            self.stats.backtracks += 1;
        }
    }

    /// The empty cell with the fewest fitting candidates; ties broken toward the
    /// border. `None` if any empty cell has zero candidates (dead end).
    fn mrv_cell(&self) -> Option<usize> {
        let mut best: Option<(usize, usize, bool)> = None; // (count, pos, is_border)
        for pos in 0..N {
            if !self.board.is_empty_at(pos) {
                continue;
            }
            let (u, l, d, r) = self.neighbor_colors(pos);
            // Count placeable candidates, but stop as soon as the count exceeds the
            // current best: a most-constrained search only needs to know whether a
            // cell has *fewer* candidates than the best so far (or ties it and is a
            // border cell). Any count strictly greater than `best` can never win, so
            // there is no need to count past `limit`. This early-out is what turns
            // the per-node cost from "count every candidate of every empty cell"
            // into "count at most a handful," while picking the identical cell.
            let limit = best.map_or(usize::MAX, |(bc, _, _)| bc);
            // The candidate list is already filtered by whichever of (up, left) it
            // indexes on (see `candidates`), so re-checking those edges is wasted
            // work. `by_up_left` filters both up and left; `by_up` filters up (and
            // left is unconstrained here anyway); `all` filters neither, but is only
            // used when up is unconstrained, so up never needs a check. The only
            // side the list might leave unchecked besides down/right is left, when
            // the list is `all` and left is constrained. The common MRV frontier
            // cell (up+left constrained, down+right empty) reduces the inner test to
            // just the used-set.
            let check_l = u == NO_CONSTRAINT && l != NO_CONSTRAINT; // `all` list, left constrained
            let mut count = 0usize;
            for &oi in self.candidates(u, l) {
                let o = self.geom.oriented(oi);
                if self.used[o.piece as usize] {
                    continue;
                }
                let e = o.edges;
                let ok = (d == NO_CONSTRAINT || e[2] == d)
                    && (r == NO_CONSTRAINT || e[1] == r)
                    && (!check_l || e[3] == l);
                if ok {
                    count += 1;
                    if count > limit {
                        break; // cannot beat or tie the best; stop counting this cell
                    }
                }
            }
            if count == 0 {
                return None;
            }
            let (rr, cc) = Board::rc(pos);
            let is_border = rr == 0 || rr == 15 || cc == 0 || cc == 15;
            let better = match best {
                None => true,
                Some((bc, _, bb)) => count < bc || (count == bc && is_border && !bb),
            };
            if better {
                best = Some((count, pos, is_border));
            }
        }
        best.map(|(_, pos, _)| pos)
    }

    // ----- propagation (the heuristic axis) -----

    /// Run the variant's propagator after a placement at `pos`. Returns `false`
    /// if the placement is provably doomed (some empty cell can no longer be
    /// filled), so the caller prunes the branch. `None` is a no-op.
    #[inline]
    fn propagate_ok(&mut self, pos: usize) -> bool {
        match self.spec.propagate {
            Propagator::None => true,
            Propagator::ForwardCheck => self.forward_check(pos),
            // AC-3 extends forward-checking with a second-ply frontier scan;
            // gacolor adds a remaining-colour-supply check on top.
            Propagator::Ac3 => self.forward_check(pos) && self.arc_consistent(pos),
            Propagator::GacolorAc3 => {
                self.forward_check(pos) && self.arc_consistent(pos) && self.color_supply_ok(pos)
            }
        }
    }

    /// Forward-check: every empty cell orthogonally adjacent to `pos` must still
    /// have at least one placeable candidate. Cheap, sound under strict
    /// placement (never under breaks — the registry forbids that pairing).
    fn forward_check(&mut self, pos: usize) -> bool {
        let (row, col) = Board::rc(pos);
        let mut neigh: [Option<usize>; 4] = [None; 4];
        if row > 0 {
            neigh[0] = Some(pos - W);
        }
        if row < 15 {
            neigh[1] = Some(pos + W);
        }
        if col > 0 {
            neigh[2] = Some(pos - 1);
        }
        if col < 15 {
            neigh[3] = Some(pos + 1);
        }
        for cell in neigh.into_iter().flatten() {
            if self.board.is_empty_at(cell) && !self.has_any_candidate(cell) {
                self.stats.backtracks += 1; // pruned a doomed branch
                return false;
            }
        }
        true
    }

    /// Does `cell` (assumed empty) have at least one unused, fitting piece?
    fn has_any_candidate(&self, cell: usize) -> bool {
        let (u, l, d, r) = self.neighbor_colors(cell);
        for &oi in self.candidates(u, l) {
            let o = self.geom.oriented(oi);
            if !self.used[o.piece as usize] && self.fits(cell, o.edges, u, l, d, r) {
                return true;
            }
        }
        false
    }

    /// Arc-consistency (AC-3, one round): every empty frontier cell around
    /// `pos` must, for *each* of its own candidate placements, still leave every
    /// one of ITS empty neighbours with a candidate. A stronger, second-ply
    /// version of forward-checking. Sound only under strict placement.
    fn arc_consistent(&self, pos: usize) -> bool {
        let (row, col) = Board::rc(pos);
        let mut around = [None; 4];
        if row > 0 {
            around[0] = Some(pos - W);
        }
        if row < 15 {
            around[1] = Some(pos + W);
        }
        if col > 0 {
            around[2] = Some(pos - 1);
        }
        if col < 15 {
            around[3] = Some(pos + 1);
        }
        for cell in around.into_iter().flatten() {
            if !self.board.is_empty_at(cell) {
                continue;
            }
            // The cell must have at least one candidate that keeps its own
            // frontier alive (a revise step over the arc).
            if !self.cell_has_supported_candidate(cell) {
                return false;
            }
        }
        true
    }

    /// True if `cell` has at least one unused fitting piece whose placement does
    /// not immediately empty a neighbouring cell's domain (one AC revise).
    fn cell_has_supported_candidate(&self, cell: usize) -> bool {
        let (u, l, d, r) = self.neighbor_colors(cell);
        for &oi in self.candidates(u, l) {
            let o = self.geom.oriented(oi);
            if self.used[o.piece as usize] || !self.fits(cell, o.edges, u, l, d, r) {
                continue;
            }
            // Provisionally reason about this candidate's effect on the cell's
            // empty neighbours by the colours it would impose. If every empty
            // neighbour would still admit some unused piece, the candidate is
            // supported and the arc is consistent.
            if self.candidate_keeps_neighbours_alive(cell, o.edges) {
                return true;
            }
        }
        false
    }

    /// Would placing `edges` at `cell` leave each of its empty neighbours with a
    /// candidate? Uses the imposed colour on each shared edge.
    fn candidate_keeps_neighbours_alive(&self, cell: usize, edges: [u8; 4]) -> bool {
        let (row, col) = Board::rc(cell);
        // For each empty neighbour, does some unused piece carry the colour this
        // candidate imposes on the shared edge?
        if row < 15 && self.board.is_empty_at(cell + W) && !self.color_placeable_below(cell + W, edges[2]) {
            return false;
        }
        if col < 15 && self.board.is_empty_at(cell + 1) && !self.color_placeable_right(cell + 1, edges[1]) {
            return false;
        }
        true
    }

    /// Is there an unused piece able to sit at `cell` with up-edge `up`? (The
    /// imposed colour from a placement above.)
    fn color_placeable_below(&self, cell: usize, up: u8) -> bool {
        let (_, l, d, r) = self.neighbor_colors(cell);
        for &oi in self.geom.by_up_left_or(up, l) {
            let o = self.geom.oriented(oi);
            if !self.used[o.piece as usize] && self.fits(cell, o.edges, up, l, d, r) {
                return true;
            }
        }
        false
    }

    /// Is there an unused piece able to sit at `cell` with left-edge `left`?
    fn color_placeable_right(&self, cell: usize, left: u8) -> bool {
        let (u, _, d, r) = self.neighbor_colors(cell);
        for &oi in self.geom.by_up_left_or(u, left) {
            let o = self.geom.oriented(oi);
            if !self.used[o.piece as usize] && self.fits(cell, o.edges, u, left, d, r) {
                return true;
            }
        }
        false
    }

    /// Gacolor supply check (sound): the second ring of empty cells around
    /// `pos` — the empty neighbours of `pos`'s empty neighbours — must each
    /// still have a candidate. This extends AC-3's one-ring revise to two rings,
    /// a strictly stronger but still sound prune (it only rejects a state where
    /// some reachable cell is already unfillable). Never a false prune, so it can
    /// never kill the true solution.
    fn color_supply_ok(&self, pos: usize) -> bool {
        let (row, col) = Board::rc(pos);
        let mut ring1 = [None; 4];
        if row > 0 {
            ring1[0] = Some(pos - W);
        }
        if row < 15 {
            ring1[1] = Some(pos + W);
        }
        if col > 0 {
            ring1[2] = Some(pos - 1);
        }
        if col < 15 {
            ring1[3] = Some(pos + 1);
        }
        for c1 in ring1.into_iter().flatten() {
            if !self.board.is_empty_at(c1) {
                continue;
            }
            let (r2, c2) = Board::rc(c1);
            let ring2 = [
                (r2 > 0).then(|| c1 - W),
                (r2 < 15).then(|| c1 + W),
                (c2 > 0).then(|| c1 - 1),
                (c2 < 15).then(|| c1 + 1),
            ];
            for cell in ring2.into_iter().flatten() {
                if self.board.is_empty_at(cell) && !self.has_any_candidate(cell) {
                    return false;
                }
            }
        }
        true
    }

    // ----- shared helpers -----

    /// Colors required on the (up, left, down, right) edges of `pos`, from
    /// placed neighbours. `EMPTY_COLOR` (255) means "no constraint" (neighbour
    /// empty or off-board). Border cells impose the border color (0) on their
    /// off-board sides.
    #[inline]
    fn neighbor_colors(&self, pos: usize) -> (u8, u8, u8, u8) {
        let (row, col) = Board::rc(pos);
        let up = if row == 0 {
            0 // off-board top → border
        } else {
            self.edge_of(pos - W, 2) // neighbour's down edge faces our up
        };
        let left = if col == 0 {
            0
        } else {
            self.edge_of(pos - 1, 1) // neighbour's right faces our left
        };
        let down = if row == 15 {
            0
        } else {
            self.edge_of(pos + W, 0)
        };
        let right = if col == 15 {
            0
        } else {
            self.edge_of(pos + 1, 3)
        };
        (up, left, down, right)
    }

    /// The color on side `side` (URDL index) of the piece at `pos`, or
    /// `NO_CONSTRAINT` if that cell is empty. Reads the resolved-edge cache, so
    /// no rotation is recomputed on the hot path. An empty cell caches
    /// `[255;4]`, which already equals `NO_CONSTRAINT` on every side.
    #[inline]
    fn edge_of(&self, pos: usize, side: usize) -> u8 {
        self.cell_edges[pos][side]
    }

    /// Candidate oriented-piece indices given the up/left constraints. When both
    /// are constrained we use the two-sided index; when only up is, the up
    /// index; when neither, all. (Down/right, if constrained, are filtered by
    /// `fits`.)
    #[inline]
    fn candidates(&self, u: u8, l: u8) -> &[u32] {
        match (u == NO_CONSTRAINT, l == NO_CONSTRAINT) {
            (false, false) => self.geom.by_up_left(u, l),
            (false, true) => self.geom.by_up(u),
            _ => self.geom.all(),
        }
    }

    /// Strict fit: every constrained side must match. Break variants do not use
    /// this — they run [`Self::search_break`], which counts mismatches against
    /// the budget instead of rejecting them.
    #[inline]
    fn fits(&self, _pos: usize, edges: [u8; 4], u: u8, l: u8, d: u8, r: u8) -> bool {
        (u == NO_CONSTRAINT || edges[0] == u)
            && (l == NO_CONSTRAINT || edges[3] == l)
            && (d == NO_CONSTRAINT || edges[2] == d)
            && (r == NO_CONSTRAINT || edges[1] == r)
    }

    /// Apply the variant's value order to a snapshot of candidate indices, in
    /// place. Insertion order is a no-op (the candidates are already in
    /// piece-id order), so `NAIVE-CLEAN` pays nothing here beyond the snapshot.
    fn reorder(&mut self, cands: &mut [u32]) {
        match self.spec.value {
            ValueOrder::Insertion => {}
            ValueOrder::RareColorFirst => {
                let rank = &self.color_rank;
                let geom = self.geom;
                cands.sort_by_key(|&oi| {
                    let e = geom.oriented(oi).edges;
                    e.iter().map(|&c| rank.get(c as usize).copied().unwrap_or(0)).min().unwrap_or(0)
                });
            }
            ValueOrder::RandomShuffle => self.rng.shuffle(cands),
        }
    }
}

/// Sentinel for "no constraint on this side" (neighbour empty). Distinct from
/// border (0) and any real color.
const NO_CONSTRAINT: u8 = 255;

/// Does a shared interior edge break? True iff the neighbour is placed
/// (`theirs != NO_CONSTRAINT`), both sides carry a real (non-border) colour, and
/// the colours differ.
#[inline]
fn edge_breaks(mine: u8, theirs: u8) -> bool {
    theirs != NO_CONSTRAINT && theirs != 0 && mine != 0 && mine != theirs
}

/// Matched-edge gain from placing `edges` against the given neighbour colors —
/// each matching, non-border, constrained shared edge counts once. URDL edges;
/// neighbour colors are (up, left, down, right) as returned by
/// [`SearchState::neighbor_colors`].
#[inline]
fn match_gain(edges: [u8; 4], u: u8, l: u8, d: u8, r: u8) -> u32 {
    let mut g = 0;
    if u != NO_CONSTRAINT && u != 0 && edges[0] == u {
        g += 1;
    }
    if l != NO_CONSTRAINT && l != 0 && edges[3] == l {
        g += 1;
    }
    if d != NO_CONSTRAINT && d != 0 && edges[2] == d {
        g += 1;
    }
    if r != NO_CONSTRAINT && r != 0 && edges[1] == r {
        g += 1;
    }
    g
}

/// Rank colors by global scarcity (count of edges of that color across all
/// pieces): rarer color → lower rank. Used by `RareColorFirst`.
fn rare_color_ranks(pieces: &Pieces, colors: usize) -> Vec<u32> {
    let mut freq = vec![0u32; colors];
    for (_, p) in pieces.iter() {
        for &c in &p.edges {
            if (c as usize) < colors {
                freq[c as usize] += 1;
            }
        }
    }
    // rank = frequency itself works as a sort key (min frequency = rarest).
    freq
}
