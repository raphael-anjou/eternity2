// R4 — BLACKWOOD-STYLE DEPTH-GATED BREAK BACKTRACKER (from scratch).
//
// Our from-scratch record engine is a SOFT BEAM that tolerates breaks
// uniformly/greedily. The community record engines (Blackwood 468-470) are
// DEPTH-FIRST BACKTRACKERS that FORBID edge mismatches ("breaks") until deep
// in the board, then release them one at a time on a tuned depth schedule
// with a non-adjacency rule. This binary implements that algorithm faithfully
// from the decoded specification (NO community source imported).
//
// Algorithm components (see the project write-ups):
//   1. classic DFS place/check/backtrack over a fixed fill order
//   2. piece value-ordering by 3 privileged colors (tunable)
//   3. a 256-entry quota schedule (hard prune on privileged-color count)
//   4. a depth-gated cumulative break budget with a non-adjacency rule
//   5. restart portfolio: shuffled opening + per-attempt node cap
//
// Score convention: SCORE = 480 - (#breaks) on a full 256-placement board.
// A break = a mismatched interior edge between two placed pieces, OR a placed
// piece whose border-facing edge is not the gray border color. We only ever
// count internal adjacencies once (right + down from each cell).
//
// Stages are selectable via --stage {1,2,3,4} so each can be measured in
// isolation. Stage 4 is the full engine.

#![forbid(unsafe_code)]

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Instant;

use rayon::prelude::*;

use eternity2_core::{Board, Rotation};
use eternity2_export::bucas_url;
use eternity2_puzzle_io::load_puzzle_with_hints;

const N: usize = 256; // cells
const W: usize = 16;
const BORDER: u8 = 0;

// ---------------------------------------------------------------------------
// PRNG — splitmix64 seeded xoshiro256**; strictly better than an LCG.
// ---------------------------------------------------------------------------
#[derive(Clone)]
struct Rng {
    s: [u64; 4],
}
impl Rng {
    fn new(seed: u64) -> Self {
        // splitmix64 to fill the state
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
    #[inline]
    fn next_u64(&mut self) -> u64 {
        let result = self.s[1].wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        let t = self.s[1] << 17;
        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];
        self.s[2] ^= t;
        self.s[3] = self.s[3].rotate_left(45);
        result
    }
    #[inline]
    fn below(&mut self, n: usize) -> usize {
        (self.next_u64() % (n as u64)) as usize
    }
    fn shuffle<T>(&mut self, v: &mut [T]) {
        let len = v.len();
        for i in (1..len).rev() {
            let j = self.below(i + 1);
            v.swap(i, j);
        }
    }
}

// ---------------------------------------------------------------------------
// Rotated-piece table: for each piece id we precompute the 4 rotations'
// edges [top,right,bottom,left] and metadata. A "candidate" is a
// (piece_id, rotation) pair. We build per-cell candidate lists lazily during
// search (the compact struct is a RotatedPiece).
// ---------------------------------------------------------------------------
#[derive(Clone, Copy)]
struct RotatedPiece {
    top: u8,
    right: u8,
    bottom: u8,
    left: u8,
    heuristic_count: u8, // # of privileged colors on this oriented piece
}

fn rotate(edges: [u8; 4], r: u8) -> [u8; 4] {
    // edges = [t,r,b,l]; clockwise rotation by r quarter-turns.
    // matches eternity2_core::Edges::rotated
    let [t, ri, b, l] = edges;
    match r {
        0 => [t, ri, b, l],
        1 => [l, t, ri, b],
        2 => [b, l, t, ri],
        _ => [ri, b, l, t],
    }
}

// ---------------------------------------------------------------------------
// Quota schedule (Blackwood piecewise-linear ramp). heuristic_array[depth] =
// minimum # privileged placements that must ALREADY be on the board.
// ---------------------------------------------------------------------------
fn build_quota(scale: f32) -> [u16; 256] {
    let mut a = [0u16; 256];
    for i in 0..256 {
        let f = i as f32;
        let v: f32 = if i <= 16 {
            0.0
        } else if i <= 26 {
            (f - 16.0) * 2.8
        } else if i <= 56 {
            (f - 26.0) * 1.43333 + 28.0
        } else if i <= 76 {
            (f - 56.0) * 0.9 + 71.0
        } else if i <= 102 {
            (f - 76.0) * 0.6538 + 89.0
        } else if i <= 160 {
            (f - 102.0) / 4.4615 + 106.0
        } else {
            // hold last value
            (160.0 - 102.0) / 4.4615 + 106.0
        };
        a[i] = (v * scale) as u16;
    }
    a
}

// Depths at which a cumulative break unlocks (10 breaks total).
const BREAK_UNLOCK: [usize; 10] = [201, 206, 211, 216, 221, 225, 229, 233, 237, 239];

#[inline]
fn breaks_allowed_at(depth: usize) -> u8 {
    BREAK_UNLOCK.iter().filter(|&&d| d <= depth).count() as u8
}

// ---------------------------------------------------------------------------
// Fill order. Blackwood: start bottom-left corner (nearest center hint), row
// scan to depth 180, then intersperse remaining BORDER pieces afterward.
// We build a permutation of 0..256 (row-major positions). y=0 is TOP row in
// this CSV; the center hint sits at y=8. "Bottom-left" => start at the bottom
// row and scan upward, left to right.
//
// interior_first_border_last=true reproduces the refinement: interior cells in
// bottom-up row-scan order, with border cells (frame ring) interspersed in the
// tail (roughly after 180 interior placements).
// ---------------------------------------------------------------------------
fn build_fill_order(border_interspersed: bool) -> Vec<usize> {
    // base: standard row scan, top row (y=0) first, left-to-right within a row.
    // This guarantees a cell's top and left neighbors are ALWAYS already placed
    // when it is filled, so matching constraints are exact. (Blackwood's
    // "bottom-left start" is a physical-corner choice; the canonical puzzle has
    // no symmetry, so any single corner-anchored row scan is equivalent for the
    // depth-indexed schedule. What matters is that top+left are the placed
    // neighbors, which a top-left row scan gives.)
    let base: Vec<usize> = (0..W)
        .flat_map(|y| (0..W).map(move |x| y * W + x))
        .collect();
    if !border_interspersed {
        return base;
    }
    // Refinement: pull interior cells first; then intersperse the frame-ring
    // (border) cells starting around position 180.
    let is_border = |pos: usize| {
        let x = pos % W;
        let y = pos / W;
        x == 0 || x == W - 1 || y == 0 || y == W - 1
    };
    let interior: Vec<usize> = base.iter().copied().filter(|&p| !is_border(p)).collect();
    let border: Vec<usize> = base.iter().copied().filter(|&p| is_border(p)).collect();
    // interior has 196 cells, border has 60. Place first ~180 interior, then
    // interleave the remaining interior + border every few steps.
    let mut order = Vec::with_capacity(N);
    let split = 180usize.min(interior.len());
    order.extend_from_slice(&interior[..split]);
    // interleave remaining interior with border: 1 border every ~2 interior
    let mut bi = 0usize;
    let mut ii = split;
    let mut step = 0usize;
    while ii < interior.len() || bi < border.len() {
        if bi < border.len() && (step % 2 == 0 || ii >= interior.len()) {
            order.push(border[bi]);
            bi += 1;
        } else if ii < interior.len() {
            order.push(interior[ii]);
            ii += 1;
        }
        step += 1;
    }
    debug_assert_eq!(order.len(), N);
    order
}

// ---------------------------------------------------------------------------
// Solver state.
// ---------------------------------------------------------------------------
// Clone: each parallel restart worker takes its own Solver. The bulk
// (rotpieces / cand_by_tl / cand_by_tl_pristine) is a few hundred KB of tables
// that are read-only during a search, so a per-worker copy is cheap.
#[derive(Clone)]
struct Solver {
    npieces: usize,
    // quota schedule
    quota: [u16; 256],
    // fill order & inverse (pos -> depth)
    order: Vec<usize>,
    // border mask per position [t,r,b,l] = true where edge faces gray border
    border_mask: [[bool; 4]; N],
    // hint pinning: hint_at[pos] = Some((pid, rot))
    hint_at: [Option<(u16, u8)>; N],
    // stage flags
    use_quota: bool,
    use_breaks: bool,
    max_breaks: u8,

    // per-cell candidate lists, indexed by depth. For a given cell we need the
    // candidates (piece,rot) whose top/left match already-placed neighbors and
    // whose border-facing edges are border. But because breaks are allowed
    // late, we keep ALL orientations of ALL pieces available and filter/score
    // at placement time. For throughput we precompute, per (piece), its 4
    // RotatedPiece structs once.
    rotpieces: Vec<[RotatedPiece; 4]>,
    distinct_rots: Vec<u8>, // number of distinct rotations per piece (dedupe)
    // exact-match candidate table for the no-break hot path: for a cell whose
    // required top color is `t` and left color is `l`, cand_by_tl[t*NC+l] lists
    // every (piece_id, rot) with exactly that top & left. NC = ncolors.
    ncolors: usize,
    cand_by_tl: Vec<Vec<(u16, u8)>>,
    // Construction-order copy of cand_by_tl, never mutated after Solver::new.
    // reshuffle_candidates restores from this so each restart's opening depends
    // only on its own seed (see reshuffle_candidates).
    cand_by_tl_pristine: Vec<Vec<(u16, u8)>>,

    // working state
    board_piece: [u16; N], // u16::MAX = empty
    board_rot: [u8; N],
    used: Vec<bool>,
    priv_on_board: u16, // running count of privileged-color placements
    breaks_used: u8,

    // counters
    nodes: u64,
    node_cap: u64,
    verbose: bool,
    best_score: u16,
    best_board: Option<([u16; N], [u8; N])>,
    deepest: usize, // deepest depth reached (partial-progress metric for stage 1)
    best_partial: Option<([u16; N], [u8; N], usize)>, // deepest partial snapshot

    // score histogram of completed boards (index by score)
    completed: Vec<u32>, // scores of finished boards
}

const EMPTYP: u16 = u16::MAX;

impl Solver {
    #[allow(clippy::too_many_arguments)]
    fn new(
        edges: Vec<[u8; 4]>,
        privileged_colors: &[u8],
        order: Vec<usize>,
        hints: &[(usize, u16, u8)],
        use_quota: bool,
        use_breaks: bool,
        max_breaks: u8,
        node_cap: u64,
        quota_scale: f32,
    ) -> Self {
        let npieces = edges.len();
        let mut privileged = [false; 256];
        for &c in privileged_colors {
            privileged[c as usize] = true;
        }
        let quota = build_quota(quota_scale);

        // border mask
        let mut border_mask = [[false; 4]; N];
        for pos in 0..N {
            let x = pos % W;
            let y = pos / W;
            border_mask[pos] = [y == 0, x == W - 1, y == W - 1, x == 0];
        }

        // hints
        let mut hint_at: [Option<(u16, u8)>; N] = [None; N];
        for &(pos, pid, rot) in hints {
            hint_at[pos] = Some((pid, rot));
        }

        // rotated pieces + heuristic counts
        let mut rotpieces = Vec::with_capacity(npieces);
        let mut distinct_rots = Vec::with_capacity(npieces);
        for pid in 0..npieces {
            let e = edges[pid];
            let mut arr = [RotatedPiece {
                top: 0,
                right: 0,
                bottom: 0,
                left: 0,
                heuristic_count: 0,
            }; 4];
            let mut seen: Vec<[u8; 4]> = Vec::new();
            for r in 0..4u8 {
                let re = rotate(e, r);
                let hc = re.iter().filter(|&&c| privileged[c as usize]).count() as u8;
                arr[r as usize] = RotatedPiece {
                    top: re[0],
                    right: re[1],
                    bottom: re[2],
                    left: re[3],
                    heuristic_count: hc,
                };
                if !seen.contains(&re) {
                    seen.push(re);
                }
            }
            distinct_rots.push(seen.len() as u8);
            rotpieces.push(arr);
        }

        // ncolors = max color + 1 (colors are 0..=maxcolor)
        let ncolors = edges
            .iter()
            .flat_map(|e| e.iter().copied())
            .max()
            .unwrap_or(0) as usize
            + 1;
        // exact (top,left) candidate table
        let mut cand_by_tl: Vec<Vec<(u16, u8)>> = vec![Vec::new(); ncolors * ncolors];
        for pid in 0..npieces {
            let ndr = distinct_rots[pid] as usize;
            for r in 0..ndr {
                let rp = rotpieces[pid][r];
                let key = (rp.top as usize) * ncolors + (rp.left as usize);
                cand_by_tl[key].push((pid as u16, r as u8));
            }
        }
        // value-order each list: privileged (heuristic_count) to the front so
        // the no-break hot path needs no per-node sort.
        for list in cand_by_tl.iter_mut() {
            list.sort_by(|a, b| {
                let ha = rotpieces[a.0 as usize][a.1 as usize].heuristic_count;
                let hb = rotpieces[b.0 as usize][b.1 as usize].heuristic_count;
                hb.cmp(&ha)
            });
        }

        Self {
            npieces,
            quota,
            order,
            border_mask,
            hint_at,
            use_quota,
            use_breaks,
            max_breaks,
            rotpieces,
            distinct_rots,
            ncolors,
            cand_by_tl_pristine: cand_by_tl.clone(),
            cand_by_tl,
            board_piece: [EMPTYP; N],
            board_rot: [0u8; N],
            used: vec![false; npieces],
            priv_on_board: 0,
            breaks_used: 0,
            nodes: 0,
            node_cap,
            verbose: false,
            best_score: 0,
            best_board: None,
            deepest: 0,
            best_partial: None,
            completed: vec![0; 481],
        }
    }

    // Per-attempt randomization: reshuffle each exact-match candidate list so
    // that ties in heuristic_count are broken differently each restart. This is
    // the "randomize the opening" lever expressed through value-order: a
    // different first legal piece at the early cells sends the DFS into a
    // different basin. Privileged-first grouping is preserved (stable within a
    // heuristic_count block; only intra-block order is shuffled).
    /// Re-derive this attempt's candidate order from `cand_by_tl_pristine` (the
    /// construction-order table), NOT from whatever the previous attempt left
    /// behind.
    ///
    /// This used to shuffle `cand_by_tl` in place, cumulatively -- the
    /// table was never restored between attempts (`reset` deliberately keeps the
    /// catalog), so attempt K's opening was a shuffle-of-a-shuffle-of-... over
    /// attempts 0..K on that Solver. That made an attempt's result depend on how
    /// many attempts had run before it on the same Solver, which (a) is not what
    /// "seeded restart portfolio" is supposed to mean -- the seed alone should
    /// determine the opening -- and (b) is precisely the hidden coupling that
    /// would make a per-thread Solver silently disagree with the serial run.
    /// Restoring first makes an attempt a pure function of its own seed, which
    /// is both the honest portfolio semantics and what lets attempts run
    /// concurrently. Attempt 0 is unaffected (nothing had shuffled yet).
    fn reshuffle_candidates(&mut self, rng: &mut Rng) {
        self.cand_by_tl.clone_from(&self.cand_by_tl_pristine);
        for list in self.cand_by_tl.iter_mut() {
            // Fisher-Yates within the whole list, then stable-sort by
            // heuristic_count desc restores the privileged grouping while
            // keeping the shuffled intra-group order.
            rng.shuffle(list);
        }
        let rotpieces = &self.rotpieces;
        for list in self.cand_by_tl.iter_mut() {
            list.sort_by(|a, b| {
                let ha = rotpieces[a.0 as usize][a.1 as usize].heuristic_count;
                let hb = rotpieces[b.0 as usize][b.1 as usize].heuristic_count;
                hb.cmp(&ha)
            });
        }
    }

    // Reset working state (keeps catalog/order/quota). Re-pins hints.
    fn reset(&mut self) {
        self.board_piece = [EMPTYP; N];
        self.board_rot = [0u8; N];
        for u in self.used.iter_mut() {
            *u = false;
        }
        self.priv_on_board = 0;
        self.breaks_used = 0;
        self.nodes = 0;
        // pin hints
        for pos in 0..N {
            if let Some((pid, rot)) = self.hint_at[pos] {
                self.board_piece[pos] = pid;
                self.board_rot[pos] = rot;
                self.used[pid as usize] = true;
                // count privileged for hint
                let rp = self.rotpieces[pid as usize][rot as usize];
                self.priv_on_board += u16::from(rp.heuristic_count);
            }
        }
    }

    // Neighbor lookups: for a cell, the required top color (from cell above,
    // its bottom edge) and left color (from cell to the left, its right edge).
    // Returns (top_req, left_req, right_known, bottom_known) where each is
    // Some(color) if the neighbor is placed, else None.
    #[inline]
    fn neighbor_colors(&self, pos: usize) -> (Option<u8>, Option<u8>, Option<u8>, Option<u8>) {
        let x = pos % W;
        let y = pos / W;
        let above = if y > 0 {
            let p = pos - W;
            if self.board_piece[p] != EMPTYP {
                Some(self.oriented(p).bottom)
            } else {
                None
            }
        } else {
            None
        };
        let left = if x > 0 {
            let p = pos - 1;
            if self.board_piece[p] != EMPTYP {
                Some(self.oriented(p).right)
            } else {
                None
            }
        } else {
            None
        };
        let right = if x < W - 1 {
            let p = pos + 1;
            if self.board_piece[p] != EMPTYP {
                Some(self.oriented(p).left)
            } else {
                None
            }
        } else {
            None
        };
        let below = if y < W - 1 {
            let p = pos + W;
            if self.board_piece[p] != EMPTYP {
                Some(self.oriented(p).top)
            } else {
                None
            }
        } else {
            None
        };
        (above, left, right, below)
    }

    #[inline]
    fn oriented(&self, pos: usize) -> RotatedPiece {
        let pid = self.board_piece[pos];
        let rot = self.board_rot[pos];
        self.rotpieces[pid as usize][rot as usize]
    }

    // Count breaks a candidate placement at `pos` would create against its
    // already-placed neighbors and against the border. Returns None if the
    // placement would create a break that VIOLATES the non-adjacency rule with
    // an existing break, or if it would exceed budget (checked by caller via
    // count). We return the list of edges that break so the caller can enforce
    // non-adjacency. Each break is identified by the shared edge between pos
    // and a neighbor (or a border edge of pos).
    //
    // For simplicity & correctness we track broken edges as an undirected set
    // keyed by (min(pos,npos)*4 + side). Non-adjacency = no two broken edges
    // share a common cell that has ALL-matched requirement... Blackwood's rule:
    // a broken edge's neighboring edges must all match, i.e. no two broken
    // edges are incident to a common cell. We enforce: a new break at cell
    // `pos` on side s is illegal if the neighbor cell across a DIFFERENT side
    // already carries a break, or if `pos`'s other sides already broke.
    //
    // We implement non-adjacency as: no cell may be incident to 2+ broken
    // edges (each broken edge touches 2 cells; forbidding a cell from having
    // two broken edges makes every break isolated). We keep break_count_at[cell].
    #[inline]
    fn eval_candidate(
        &self,
        pos: usize,
        rp: &RotatedPiece,
        break_count_at: &[u8; N],
    ) -> Option<(u8, [Option<usize>; 4])> {
        // returns (num_new_breaks, [broken neighbor cell per side or None])
        // side order: 0=top,1=right,2=bottom,3=left
        let x = pos % W;
        let y = pos / W;
        let mut new_breaks = 0u8;
        let mut broke_side_cell: [Option<usize>; 4] = [None; 4];
        // this cell would gain broken edges; enforce pos itself gets <=1 break
        // AND each neighbor cell that breaks must not already have a break.
        let bm = self.border_mask[pos];

        // helper closure result accumulation is inline below.
        // TOP
        if y > 0 {
            let np = pos - W;
            if self.board_piece[np] != EMPTYP {
                if self.oriented(np).bottom != rp.top {
                    new_breaks += 1;
                    broke_side_cell[0] = Some(np);
                }
            }
        }
        // border-facing top
        if bm[0] && rp.top != BORDER {
            new_breaks += 1;
            broke_side_cell[0] = Some(usize::MAX); // border break, no neighbor cell
        }
        // RIGHT
        if x < W - 1 {
            let np = pos + 1;
            if self.board_piece[np] != EMPTYP {
                if self.oriented(np).left != rp.right {
                    new_breaks += 1;
                    broke_side_cell[1] = Some(np);
                }
            }
        }
        if bm[1] && rp.right != BORDER {
            new_breaks += 1;
            broke_side_cell[1] = Some(usize::MAX);
        }
        // BOTTOM
        if y < W - 1 {
            let np = pos + W;
            if self.board_piece[np] != EMPTYP {
                if self.oriented(np).top != rp.bottom {
                    new_breaks += 1;
                    broke_side_cell[2] = Some(np);
                }
            }
        }
        if bm[2] && rp.bottom != BORDER {
            new_breaks += 1;
            broke_side_cell[2] = Some(usize::MAX);
        }
        // LEFT
        if x > 0 {
            let np = pos - 1;
            if self.board_piece[np] != EMPTYP {
                if self.oriented(np).right != rp.left {
                    new_breaks += 1;
                    broke_side_cell[3] = Some(np);
                }
            }
        }
        if bm[3] && rp.left != BORDER {
            new_breaks += 1;
            broke_side_cell[3] = Some(usize::MAX);
        }

        // Non-adjacency: pos may hold at most 1 break total, and any neighbor
        // cell that we break against must not already carry a break.
        if new_breaks > 1 {
            return None;
        }
        for s in 0..4 {
            if let Some(nc) = broke_side_cell[s] {
                if nc != usize::MAX && break_count_at[nc] > 0 {
                    return None; // neighbor already has a break -> adjacency
                }
            }
        }
        Some((new_breaks, broke_side_cell))
    }

    // Recursive DFS over depths of the fill order. Returns true if a full
    // 256-placement board reached (we don't early-stop; we record best).
    fn search(&mut self, depth: usize, break_count_at: &mut [u8; N]) {
        if self.nodes >= self.node_cap {
            return;
        }
        if depth == N {
            // completed board — score = 480 - breaks_used
            let score = 480u16.saturating_sub(u16::from(self.breaks_used));
            if (score as usize) < self.completed.len() {
                self.completed[score as usize] += 1;
            }
            if score > self.best_score {
                self.best_score = score;
                self.best_board = Some((self.board_piece, self.board_rot));
            }
            return;
        }
        if depth > self.deepest {
            self.deepest = depth;
            // snapshot this deepest partial so a marathon that never fully
            // completes still yields a bankable board (greedy-completed later).
            self.best_partial = Some((self.board_piece, self.board_rot, depth));
            if self.verbose && depth >= 195 {
                eprintln!(
                    "#   new deepest depth={depth} breaks={} nodes={}",
                    self.breaks_used, self.nodes
                );
            }
        }
        let pos = self.order[depth];

        // if hint-pinned, it's already placed in reset(); just descend.
        if self.hint_at[pos].is_some() {
            self.search(depth + 1, break_count_at);
            return;
        }

        self.nodes += 1;

        // quota prune (privileged count must already meet quota[depth])
        if self.use_quota && self.priv_on_board < self.quota[depth] {
            return;
        }

        // current break budget at this depth
        let budget = if self.use_breaks {
            breaks_allowed_at(depth).min(self.max_breaks)
        } else {
            0
        };

        let (top_req, left_req, right_req, below_req) = self.neighbor_colors(pos);
        let bm = self.border_mask[pos];

        if budget == 0 {
            // FAST PATH: no breaks allowed. In row-scan the top and left are
            // always determined (a placed neighbor's color, or BORDER at the
            // frame). We look up the exact-match table cand_by_tl[top][left],
            // pre-sorted privileged-first, and only enforce the right/bottom
            // border constraints (interior right/bottom neighbors are unplaced
            // in a pure row-scan; with border-intersperse they may be placed,
            // so we check right_req/below_req too).
            let treq = match top_req {
                Some(c) => c,
                None => {
                    if bm[0] {
                        BORDER
                    } else {
                        // no top neighbor and not a border edge: impossible in a
                        // top-anchored row scan, but guard anyway (skip cell).
                        self.search(depth + 1, break_count_at);
                        return;
                    }
                }
            };
            let lreq = match left_req {
                Some(c) => c,
                None => {
                    if bm[3] {
                        BORDER
                    } else {
                        self.search(depth + 1, break_count_at);
                        return;
                    }
                }
            };
            let key = (treq as usize) * self.ncolors + (lreq as usize);
            // clone the slice pointer bounds to iterate without borrowing self
            // mutably during recursion: we index by position each iteration.
            let list_len = self.cand_by_tl[key].len();
            for idx in 0..list_len {
                if self.nodes >= self.node_cap {
                    return;
                }
                let (pid16, r) = self.cand_by_tl[key][idx];
                let pid = pid16 as usize;
                if self.used[pid] {
                    continue;
                }
                let rp = self.rotpieces[pid][r as usize];
                // right / bottom border + placed-neighbor constraints
                if let Some(rc) = right_req {
                    if rp.right != rc {
                        continue;
                    }
                } else if bm[1] && rp.right != BORDER {
                    continue;
                }
                if let Some(bc) = below_req {
                    if rp.bottom != bc {
                        continue;
                    }
                } else if bm[2] && rp.bottom != BORDER {
                    continue;
                }
                self.board_piece[pos] = pid16;
                self.board_rot[pos] = r;
                self.used[pid] = true;
                self.priv_on_board += u16::from(rp.heuristic_count);
                self.search(depth + 1, break_count_at);
                self.priv_on_board -= u16::from(rp.heuristic_count);
                self.used[pid] = false;
                self.board_piece[pos] = EMPTYP;
            }
            return;
        }

        // BREAK PATH (depth >= 201): full scan with break accounting. Collect
        // feasible candidates into a HEAP buffer (NOT a stack array — a large
        // per-frame array here is multiplied across all 256 recursion frames
        // and overflows the stack). The break path only fires at depth >= 201,
        // so the allocation is rare and cheap. value-ordered privileged first
        // then fewer breaks.
        let mut cands: Vec<(u16, u8, u8, [Option<usize>; 4])> = Vec::with_capacity(64);
        for pid in 0..self.npieces {
            if self.used[pid] {
                continue;
            }
            let ndr = self.distinct_rots[pid] as usize;
            for r in 0..ndr {
                let rp = self.rotpieces[pid][r];
                match self.eval_candidate(pos, &rp, break_count_at) {
                    None => continue,
                    Some((nb, cells)) => {
                        if self.breaks_used + nb > budget {
                            continue;
                        }
                        cands.push((pid as u16, r as u8, nb, cells));
                    }
                }
            }
        }
        if cands.is_empty() {
            return;
        }
        // value-order: privileged desc, then fewer breaks.
        cands.sort_by(|a, b| {
            let ha = self.rotpieces[a.0 as usize][a.1 as usize].heuristic_count;
            let hb = self.rotpieces[b.0 as usize][b.1 as usize].heuristic_count;
            hb.cmp(&ha).then(a.2.cmp(&b.2))
        });

        for i in 0..cands.len() {
            if self.nodes >= self.node_cap {
                return;
            }
            let (pid16, r, nb, cells) = cands[i];
            let pid = pid16 as usize;
            let rp = self.rotpieces[pid][r as usize];
            self.board_piece[pos] = pid16;
            self.board_rot[pos] = r;
            self.used[pid] = true;
            self.priv_on_board += u16::from(rp.heuristic_count);
            self.breaks_used += nb;
            break_count_at[pos] += nb;
            let mut touched: [Option<usize>; 4] = [None; 4];
            for s in 0..4 {
                if let Some(nc) = cells[s] {
                    if nc != usize::MAX {
                        break_count_at[nc] += 1;
                        touched[s] = Some(nc);
                    }
                }
            }

            self.search(depth + 1, break_count_at);

            for s in 0..4 {
                if let Some(nc) = touched[s] {
                    break_count_at[nc] -= 1;
                }
            }
            break_count_at[pos] -= nb;
            self.breaks_used -= nb;
            self.priv_on_board -= u16::from(rp.heuristic_count);
            self.used[pid] = false;
            self.board_piece[pos] = EMPTYP;
        }
    }

    // Count total breaks (mismatched internal edges + non-border border-facing
    // edges) on a fully-placed board. score = 480 - breaks.
    fn count_breaks(&self, bp: &[u16; N], br: &[u8; N]) -> u16 {
        let mut breaks = 0u16;
        for pos in 0..N {
            if bp[pos] == EMPTYP {
                continue;
            }
            let rp = self.rotpieces[bp[pos] as usize][br[pos] as usize];
            let x = pos % W;
            let y = pos / W;
            // border-facing edges
            if y == 0 && rp.top != BORDER {
                breaks += 1;
            }
            if x == W - 1 && rp.right != BORDER {
                breaks += 1;
            }
            if y == W - 1 && rp.bottom != BORDER {
                breaks += 1;
            }
            if x == 0 && rp.left != BORDER {
                breaks += 1;
            }
            // internal right edge (count once)
            if x < W - 1 && bp[pos + 1] != EMPTYP {
                let rn = self.rotpieces[bp[pos + 1] as usize][br[pos + 1] as usize];
                if rp.right != rn.left {
                    breaks += 1;
                }
            }
            // internal bottom edge (count once)
            if y < W - 1 && bp[pos + W] != EMPTYP {
                let dn = self.rotpieces[bp[pos + W] as usize][br[pos + W] as usize];
                if rp.bottom != dn.top {
                    breaks += 1;
                }
            }
        }
        480u16.saturating_sub(breaks)
    }

    // A board is physical iff every frame-facing edge is the border colour, no
    // interior edge is border, and every cell is filled. (verify_bucas's
    // `physical` flag.) The greedy tail can complete non-physically via its
    // cross-kind fallback, so we gate banked/best boards on this.
    fn is_physical(&self, bp: &[u16; N], br: &[u8; N]) -> bool {
        for pos in 0..N {
            if bp[pos] == EMPTYP {
                return false;
            }
            let rp = self.rotpieces[bp[pos] as usize][br[pos] as usize];
            let x = pos % W;
            let y = pos / W;
            let e = [rp.top, rp.right, rp.bottom, rp.left];
            let border_face = [y == 0, x == W - 1, y == W - 1, x == 0];
            for s in 0..4 {
                if border_face[s] != (e[s] == BORDER) {
                    return false;
                }
            }
        }
        true
    }

    // Greedy tail-completion: given a partial board filled up to `depth` cells
    // in fill order, fill the remaining cells one at a time, each time choosing
    // the unused piece+rotation that creates the FEWEST breaks vs already-placed
    // neighbors (schedule ignored — this is the fallback that turns any deep
    // partial into a scored complete board, mirroring our soft-beam). Returns
    // (complete board, complete rot, score).
    fn greedy_complete(
        &self,
        bp0: &[u16; N],
        br0: &[u8; N],
        depth: usize,
    ) -> ([u16; N], [u8; N], u16) {
        let mut bp = *bp0;
        let mut br = *br0;
        let mut used = vec![false; self.npieces];
        for pos in 0..N {
            if bp[pos] != EMPTYP {
                used[bp[pos] as usize] = true;
            }
        }
        for d in depth..N {
            let pos = self.order[d];
            if bp[pos] != EMPTYP {
                continue; // hint
            }
            // neighbor colors
            let x = pos % W;
            let y = pos / W;
            let top = if y > 0 && bp[pos - W] != EMPTYP {
                Some(self.rotpieces[bp[pos - W] as usize][br[pos - W] as usize].bottom)
            } else if y == 0 {
                Some(BORDER)
            } else {
                None
            };
            let left = if x > 0 && bp[pos - 1] != EMPTYP {
                Some(self.rotpieces[bp[pos - 1] as usize][br[pos - 1] as usize].right)
            } else if x == 0 {
                Some(BORDER)
            } else {
                None
            };
            let right = if x < W - 1 && bp[pos + 1] != EMPTYP {
                Some(self.rotpieces[bp[pos + 1] as usize][br[pos + 1] as usize].left)
            } else if x == W - 1 {
                Some(BORDER)
            } else {
                None
            };
            let below = if y < W - 1 && bp[pos + W] != EMPTYP {
                Some(self.rotpieces[bp[pos + W] as usize][br[pos + W] as usize].top)
            } else if y == W - 1 {
                Some(BORDER)
            } else {
                None
            };
            // required piece KIND at this cell (physicality): corners get corner
            // pieces (2 border edges), frame edges get edge pieces (1), interior
            // gets interior pieces (0). Keeps the completion physical.
            let on_left = x == 0;
            let on_right = x == W - 1;
            let on_top = y == 0;
            let on_bottom = y == W - 1;
            let nb_border = on_left as u8 + on_right as u8 + on_top as u8 + on_bottom as u8;
            let want_kind = nb_border; // 2=corner,1=edge,0=interior
            // choose min-break unused piece of the right kind
            let mut best: Option<(u16, u8, u8)> = None; // (pid,rot,breaks)
            for pid in 0..self.npieces {
                if used[pid] {
                    continue;
                }
                // piece kind = # border edges (rotation-invariant), from r0
                let e0 = self.rotpieces[pid][0];
                let pk = (e0.top == BORDER) as u8
                    + (e0.right == BORDER) as u8
                    + (e0.bottom == BORDER) as u8
                    + (e0.left == BORDER) as u8;
                if pk != want_kind {
                    continue;
                }
                let ndr = self.distinct_rots[pid] as usize;
                for r in 0..ndr {
                    let rp = self.rotpieces[pid][r];
                    let mut b = 0u8;
                    if let Some(c) = top {
                        if rp.top != c {
                            b += 1;
                        }
                    }
                    if let Some(c) = left {
                        if rp.left != c {
                            b += 1;
                        }
                    }
                    if let Some(c) = right {
                        if rp.right != c {
                            b += 1;
                        }
                    }
                    if let Some(c) = below {
                        if rp.bottom != c {
                            b += 1;
                        }
                    }
                    match best {
                        Some((_, _, bb)) if bb <= b => {}
                        _ => best = Some((pid as u16, r as u8, b)),
                    }
                }
            }
            // Fallback: if the required kind's pool is exhausted, take any
            // unused piece min-break to keep the board COMPLETE. This can cost
            // physicality on that cell (an interior piece on a frame cell) — the
            // greedy tail is only a soft progress signal, NOT a record
            // candidate. Record banking requires a NATIVE completion (physical
            // by construction) AND an external verify_bucas physical=true check.
            if best.is_none() {
                for pid in 0..self.npieces {
                    if used[pid] {
                        continue;
                    }
                    let ndr = self.distinct_rots[pid] as usize;
                    for r in 0..ndr {
                        let rp = self.rotpieces[pid][r];
                        let mut b = 0u8;
                        if let Some(c) = top {
                            if rp.top != c {
                                b += 1;
                            }
                        }
                        if let Some(c) = left {
                            if rp.left != c {
                                b += 1;
                            }
                        }
                        if let Some(c) = right {
                            if rp.right != c {
                                b += 1;
                            }
                        }
                        if let Some(c) = below {
                            if rp.bottom != c {
                                b += 1;
                            }
                        }
                        match best {
                            Some((_, _, bb)) if bb <= b => {}
                            _ => best = Some((pid as u16, r as u8, b)),
                        }
                    }
                }
            }
            if let Some((pid, r, _)) = best {
                bp[pos] = pid;
                br[pos] = r;
                used[pid as usize] = true;
            }
        }
        let score = self.count_breaks(&bp, &br);
        (bp, br, score)
    }
}

// ---------------------------------------------------------------------------
// A single restart attempt: shuffle opening, run search, return best score.
// ---------------------------------------------------------------------------
fn run_attempt(solver: &mut Solver, rng: &mut Rng, shuffle_open: bool) {
    solver.reset();
    if shuffle_open {
        solver.reshuffle_candidates(rng);
    }
    let mut break_count_at = [0u8; N];
    solver.search(0, &mut break_count_at);
}

/// RNG seed for restart `attempt` of a run seeded `seed`.
///
/// This used to be `(seed + attempt) * K`, which ALIASES -- `seed+attempt` is
/// one number, so (seed=7, attempt=1) and (seed=8, attempt=0) are the same key
/// and produce the same opening. Consecutive seeds shared all but one of their
/// restarts: a "12-seed x 6-restart sweep" was really ~17 distinct openings, not
/// 72. (The old cumulative reshuffle partly hid this -- colliding attempts sat
/// at different shuffle depths -- so the aliasing only became visible once each
/// attempt derived from its own seed alone. It was always there.)
///
/// Fix: mix the two coordinates with distinct odd multipliers and run them
/// through a SplitMix64 finalizer, so (seed, attempt) pairs are distinct keys
/// and near-identical inputs decorrelate.
#[inline]
fn attempt_key(seed: u64, attempt: usize) -> u64 {
    let mut z = seed
        .wrapping_mul(0x9E37_79B9_7F4A_7C15)
        .wrapping_add((attempt as u64).wrapping_mul(0xD1B5_4A32_D192_ED03));
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// What one restart attempt produced, extracted so an attempt is a pure
/// `attempt_index -> AttemptOutcome` function with no shared mutable state.
/// That is what lets the portfolio run attempts on N threads: the
/// attempts were ALREADY independent -- each seeds its own Rng from
/// `seed + attempt` and resets the solver first -- they were merely sharing
/// one Solver buffer and being run one at a time.
struct AttemptOutcome {
    score: u16,
    board: Option<([u16; N], [u8; N])>,
    physical: bool,
    native: bool,
    deepest: usize,
    nodes: u64,
}

/// Run restart `attempt` to completion on its own 256MB-stack thread and report
/// what it found. `solver` is this worker's private Solver (reset internally),
/// so nothing is shared with concurrent attempts.
fn run_attempt_scored(solver: &mut Solver, seed: u64, attempt: usize, shuffle: bool) -> AttemptOutcome {
    let mut rng = Rng::new(attempt_key(seed, attempt));
    solver.best_score = 0;
    solver.best_board = None;
    solver.deepest = 0;
    solver.best_partial = None;
    solver.nodes = 0;

    // The recursive search needs a LARGE stack (256 MB): the 256-deep DFS with
    // per-frame candidate scratch overflows the default 8 MB. These stacks are
    // lazily committed by the OS -- 8 of them cost ~1.7MB RSS, not 2GB -- so one
    // per worker is affordable. std::thread::scope lets us borrow solver/rng.
    std::thread::scope(|s| {
        std::thread::Builder::new()
            .stack_size(256 * 1024 * 1024)
            .spawn_scoped(s, || {
                run_attempt(solver, &mut rng, shuffle);
            })
            .expect("spawn search thread")
            .join()
            .expect("search thread panicked");
    });

    let native = solver.best_score > 0;
    let (score, board): (u16, Option<([u16; N], [u8; N])>) = if native {
        (solver.best_score, solver.best_board.clone())
    } else if let Some((bp, br, d)) = solver.best_partial {
        let (cbp, cbr, sc) = solver.greedy_complete(&bp, &br, d);
        (sc, Some((cbp, cbr)))
    } else {
        (0, None)
    };
    // Only PHYSICAL complete boards are eligible to be the best/banked board
    // (the greedy tail can complete non-physically via its cross-kind fallback,
    // which verify_bucas would reject). Native completions are physical by
    // construction.
    let physical = board.as_ref().is_some_and(|(bp, br)| solver.is_physical(bp, br));

    AttemptOutcome { score, board, physical, native, deepest: solver.deepest, nodes: solver.nodes }
}

// Build a bucas URL string from a complete board.
fn board_url(puzzle: &eternity2_core::Puzzle, bp: &[u16; N], br: &[u8; N]) -> String {
    let mut b = Board::empty(puzzle);
    for pos in 0..N {
        if bp[pos] != EMPTYP {
            b.place(pos as u32, bp[pos], Rotation::from_u8(br[pos]).unwrap());
        }
    }
    bucas_url(puzzle, &b, "size_16_official_eternity")
}

// ---------------------------------------------------------------------------
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!(
            "usage: blackwood_bt <puzzle.csv> [--stage 1|2|3|4] [--priv c1,c2,c3] \
             [--maxbreaks K] [--nodecap N] [--restarts R] [--seed S] \
             [--threads T] [--border-intersperse] [--emit-dir DIR] [--emit-prefix P]\n\
             \n  --threads 1 (default): restarts run one at a time.\
             \n  --threads T (0 = all cores): restarts run concurrently. Each restart is a\
             \n              pure function of its own seed, so the SET of restart results is\
             \n              the same at any T; only their completion order (and thus which\
             \n              one is seen as \"new best\" first) varies."
        );
        std::process::exit(2);
    }
    let csv = PathBuf::from(&args[1]);

    let mut stage = 4usize;
    // Privileged colors for the quota-schedule pruning. Blackwood's published
    // spec names colors {13,16,10} — but those are in HIS piece-set labeling.
 // A/B on OUR labeling (, 3 seeds, nodecap 30M): {13,16,10}=387,
    // {22,21,20}=435, freq-top3 {12,14,15}=418. So the spec-literal colors
    // TRANSFER BADLY (mis-keyed quota over-prunes); {22,21,20} is empirically
    // near-optimal for our numbering and is kept as the default.
    let mut priv_colors: Vec<u8> = vec![22, 21, 20];
    let mut max_breaks = 10u8;
    let mut node_cap = 200_000_000u64;
    let mut restarts = 1usize;
    let mut seed = 0xE2E2u64;
    let mut border_intersperse = false;
    let mut verbose = false;
    let mut quota_scale = 1.0f32;
    let mut emit_dir: Option<String> = None;
    let mut emit_prefix = "bw".to_string();
    let mut threads: usize = 1; // 1 = serial restart portfolio (the default)

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--stage" => {
                stage = args[i + 1].parse().unwrap();
                i += 2;
            }
            "--priv" => {
                priv_colors = args[i + 1]
                    .split(',')
                    .map(|s| s.trim().parse().unwrap())
                    .collect();
                i += 2;
            }
            "--maxbreaks" => {
                max_breaks = args[i + 1].parse().unwrap();
                i += 2;
            }
            "--nodecap" => {
                node_cap = args[i + 1].parse().unwrap();
                i += 2;
            }
            "--restarts" => {
                restarts = args[i + 1].parse().unwrap();
                i += 2;
            }
            "--threads" => {
                threads = args[i + 1].parse().unwrap();
                i += 2;
            }
            "--seed" => {
                seed = args[i + 1].parse().unwrap();
                i += 2;
            }
            "--border-intersperse" => {
                border_intersperse = true;
                i += 1;
            }
            "--verbose" => {
                verbose = true;
                i += 1;
            }
            "--quota-scale" => {
                quota_scale = args[i + 1].parse().unwrap();
                i += 2;
            }
            "--emit-dir" => {
                emit_dir = Some(args[i + 1].clone());
                i += 2;
            }
            "--emit-prefix" => {
                emit_prefix = args[i + 1].clone();
                i += 2;
            }
            other => {
                eprintln!("unknown arg {other}");
                std::process::exit(2);
            }
        }
    }

    let (puzzle, hints_raw) = load_puzzle_with_hints(&csv).expect("load puzzle+hints");
    let npieces = puzzle.pieces().len();
    assert_eq!(npieces, N, "expected 256 pieces");

    // edges[pid] = [t,r,b,l]
    let mut edges = vec![[0u8; 4]; npieces];
    for p in puzzle.pieces() {
        let e = p.edges.as_array();
        edges[p.id as usize] = e;
    }

    let hints: Vec<(usize, u16, u8)> = hints_raw
        .hints
        .iter()
        .map(|h| (h.position as usize, h.piece_id, h.rotation.as_u8()))
        .collect();

    // stage config
    let (use_quota, use_breaks) = match stage {
        1 => (false, false),
        2 => (false, true),
        3 => (true, true),
        _ => (true, true),
    };
    // stage 3 measures quota effect at fixed breaks; stage 4 == stage 3 config
    // plus restart portfolio & border intersperse handled by caller flags.

    let order = build_fill_order(border_intersperse && stage >= 4);

    let mut solver = Solver::new(
        edges,
        &priv_colors,
        order,
        &hints,
        use_quota,
        use_breaks,
        max_breaks,
        node_cap,
        quota_scale,
    );
    solver.verbose = verbose;

    if threads == 0 {
        threads = std::thread::available_parallelism().map_or(1, std::num::NonZero::get);
    }
    // Never spin up more workers than there are restarts to run.
    threads = threads.min(restarts.max(1));

    eprintln!(
        "# blackwood_bt stage={stage} priv={priv_colors:?} maxbreaks={max_breaks} \
         nodecap={node_cap} restarts={restarts} seed={seed} threads={threads} border_intersperse={}",
        border_intersperse && stage >= 4
    );
    if threads > 1 {
        eprintln!("# --threads {threads}: restarts run concurrently; per-restart results are unchanged (each depends only on its own seed), but \"new best\" log order reflects completion order.");
    }

    // global best is over COMPLETE boards (native completion OR greedy-tail
    // completion of the deepest partial). This makes every restart yield a
    // scorable full board — the marathon always banks something.
    let t0 = Instant::now();

    let emit_dir_s = emit_dir.clone().unwrap_or_else(|| ".".into());
    std::fs::create_dir_all(&emit_dir_s).ok();
    let checkpoint_path = Path::new(&emit_dir_s).join("best_so_far.url");
    let log_path = Path::new(&emit_dir_s).join(format!("{emit_prefix}_portfolio.log"));

    let shuffle = stage >= 4 || restarts > 1;
    // Bookkeeping shared by every finished attempt. Under --threads 1 this is an
    // uncontended mutex around the same sequence of updates the serial loop did;
    // under N>1 it serializes only the (rare, cheap) post-attempt bookkeeping and
    // the record/checkpoint file writes -- never the search itself. Keeping the
    // banking inside the lock is what stops two threads that both beat the record
    // from interleaving their RECORD_*.url writes.
    struct Portfolio<'a> {
        global_best: u16,
        global_best_board: Option<([u16; N], [u8; N])>,
        best_natural: u16,
        deepest_overall: usize,
        total_nodes: u64,
        hist: Vec<u32>,
        done: usize,
        puzzle: &'a eternity2_core::Puzzle,
        checkpoint_path: PathBuf,
        log_path: PathBuf,
        emit_dir_s: String,
        emit_prefix: String,
        restarts: usize,
        t0: Instant,
    }
    impl Portfolio<'_> {
        /// Fold one finished attempt into the portfolio state. Called under the
        /// mutex, so the >best test and the banking that follows it are atomic.
        fn record(&mut self, attempt: usize, o: &AttemptOutcome) {
            self.done += 1;
            if o.deepest > self.deepest_overall {
                self.deepest_overall = o.deepest;
            }
            self.total_nodes += o.nodes;
            if o.native && o.score > self.best_natural {
                self.best_natural = o.score;
            }
            if (o.score as usize) < self.hist.len() {
                self.hist[o.score as usize] += 1;
            }

            if o.score > self.global_best && o.physical {
                self.global_best = o.score;
                self.global_best_board = o.board.clone();
                let el = self.t0.elapsed().as_secs_f64();
                let msg = format!(
                    "# attempt {attempt}: NEW BEST complete score={} (native={}) deepest={} nodes={} ({:.1}s)",
                    self.global_best, o.native, self.deepest_overall, o.nodes, el
                );
                eprintln!("{msg}");
                if let Some((bp, br)) = &self.global_best_board {
                    let url = board_url(self.puzzle, bp, br);
                    std::fs::write(&self.checkpoint_path, &url).ok();
                    use std::io::Write;
                    if let Ok(mut f) =
                        std::fs::OpenOptions::new().create(true).append(true).open(&self.log_path)
                    {
                        let _ = writeln!(f, "{msg}");
                        let _ = writeln!(f, "  url: {url}");
                    }
                    // BANK + FLAG only NATIVE completions beating 459 (greedy-tail
                    // boards are a soft lower bound, not strict-5 record candidates).
                    if self.global_best > 459 && o.native {
                        let ts = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs();
                        let rec = Path::new(&self.emit_dir_s).join(format!(
                            "RECORD_{}_{}_{ts}.url",
                            self.global_best, self.emit_prefix
                        ));
                        std::fs::write(&rec, &url).ok();
                        eprintln!("# ***** NEW FROM-SCRATCH RECORD CANDIDATE score={} native={} *****", self.global_best, o.native);
                        eprintln!("# ***** banked -> {} — VERIFY 3 WAYS *****", rec.display());
                    }
                }
            }
            // periodic progress / checkpoint refresh. Keyed on COMPLETED count
            // rather than attempt index so it still fires every 25 attempts when
            // attempts finish out of order.
            if self.restarts > 1 && self.done % 25 == 0 {
                let el = self.t0.elapsed().as_secs_f64();
                eprintln!(
                    "# progress {}/{} best_complete={} best_native={} deepest={} nodes={} {:.1} Mnode/s {:.1} restarts/s ({:.0}s)",
                    self.done,
                    self.restarts,
                    self.global_best,
                    self.best_natural,
                    self.deepest_overall,
                    self.total_nodes,
                    (self.total_nodes as f64 / 1e6) / el.max(1e-9),
                    (self.done as f64) / el.max(1e-9),
                    el,
                );
            }
        }
    }

    let portfolio = Mutex::new(Portfolio {
        global_best: 0,
        global_best_board: None,
        best_natural: 0,
        deepest_overall: 0,
        total_nodes: 0,
        hist: vec![0u32; 481],
        done: 0,
        puzzle: &puzzle,
        checkpoint_path: checkpoint_path.clone(),
        log_path: log_path.clone(),
        emit_dir_s: emit_dir_s.clone(),
        emit_prefix: emit_prefix.clone(),
        restarts,
        t0,
    });

    if threads > 1 {
        // Attempts run concurrently, each on a worker holding its OWN Solver
        // clone (the catalog tables are a few hundred KB; the 256MB search stacks
        // are lazily committed, so N workers cost ~N*few-hundred-KB in practice).
        // Cloning is what makes the attempts independent: with one shared Solver
        // they would race on board_piece/used/nodes.
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .expect("build rayon pool");
        pool.install(|| {
            (0..restarts).into_par_iter().for_each_init(
                || solver.clone(),
                |local, attempt| {
                    let o = run_attempt_scored(local, seed, attempt, shuffle);
                    portfolio.lock().unwrap_or_else(std::sync::PoisonError::into_inner).record(attempt, &o);
                },
            );
        });
    } else {
        for attempt in 0..restarts {
            let o = run_attempt_scored(&mut solver, seed, attempt, shuffle);
            portfolio.lock().unwrap_or_else(std::sync::PoisonError::into_inner).record(attempt, &o);
        }
    }

    let p = portfolio.into_inner().unwrap_or_else(std::sync::PoisonError::into_inner);
    let (global_best, global_best_board, best_natural, deepest_overall, total_nodes, hist) =
        (p.global_best, p.global_best_board, p.best_natural, p.deepest_overall, p.total_nodes, p.hist);

    let elapsed = t0.elapsed().as_secs_f64();
    let mnps = (total_nodes as f64 / 1e6) / elapsed.max(1e-9);

    eprintln!("# ===== SUMMARY =====");
    eprintln!("# stage={stage} restarts={restarts} elapsed={elapsed:.2}s nodes={total_nodes} throughput={mnps:.1} Mnode/s restarts/s={:.2}", restarts as f64 / elapsed.max(1e-9));
    eprintln!("# best_complete_score={global_best} best_native_score={best_natural} deepest_depth={deepest_overall}");
    eprint!("# complete-score histogram (score:count): ");
    for (s, c) in hist.iter().enumerate() {
        if *c > 0 {
            eprint!("{s}:{c} ");
        }
    }
    eprintln!();

    // emit best board (timestamped, never overwrites)
    if let Some((bp, br)) = &global_best_board {
        let url = board_url(&puzzle, bp, br);
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let path = Path::new(&emit_dir_s).join(format!(
            "{emit_prefix}_stage{stage}_score{global_best}_{ts}.url"
        ));
        std::fs::write(&path, &url).expect("write url");
        eprintln!("# emitted best board -> {}", path.display());
        println!("{}", path.display());
    }

    println!("BEST {global_best}");
}
