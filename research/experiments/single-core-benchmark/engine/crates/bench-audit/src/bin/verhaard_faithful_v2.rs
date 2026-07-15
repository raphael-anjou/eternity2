// vol-234 R8 — FAITHFUL VERHAARD "eii" SOLVER v2 (real good-groups + adaptive abort).
//
// Louis Verhaard's win32 "eii" solver reached 467/480 (the pre-Blackwood
// community ceiling). Its algorithm SHAPE is published on shortestpath.se
// (formerly fingerboys.se/eii); its TUNED CONSTANTS shipped only as a binary.
// This binary re-implements Verhaard's search engine using constants recovered
// from that binary (eii.exe, PE32 i386) plus the published algorithm shape.
//
// v2 vs v1 (R7): closes the two RE gaps identified in R7:
//   GAP 1 (good groups): R7 approximated the quota's "good piece" membership
//     via a privileged-COLOR heuristic. R8 uses the REAL 60 good groups
//     recovered byte-exact from eii.exe .rdata (group table @ 0x42b330, piece-id
//     blob @ 0x427420). Each group = a 253-piece subset (tier-A ~150 privileged
//     + tier-B ~103) of the 256 pieces. Per restart, a group is chosen (matching
//     the binary's `rand % 60` group selection @ .text 0x4145a6), and a piece
//     counts toward the depth-quota iff it is a tier-A member of the active group.
//   GAP 2 (adaptive abort): R7 used a fixed node cap. R8 adds the adaptive
//     restart-abort — a restart is killed early if it fails to reach an expected
//     depth-vs-nodes trajectory (checkpoints derived from the depth knots the
//     binary tracks; see VERHAARD_ABORT_KNOTS).
//
// See vault/papers/vol-234/R7-VERHAARD-RE.md + R8-VERHAARD-GOODGROUPS.md for the
// RE evidence: recovered .data/.rdata arrays with addresses + disassembly xrefs.
//
// Distinguishing components vs Blackwood (blackwood_bt.rs, same family):
//   1. COMB FILL ORDER — border ring first (corners then edges), then interior
//      filled "most rows horizontal, remaining rows vertical" (comb/teeth).
//      [recovered shape; per-cell top+left kept placed-before for exactness]
//   2. DEPTH-GATED EDGE-SLIP schedule {193:1, 202:2, ... 240:12} — the max
//      cumulative INTERIOR-edge mismatches allowed at each depth. Border edges
//      NEVER slip. [published tuned-for-468 constant; NOT a static array in the
//      v1.0 binary — see report. Blackwood's non-adjacency rule is NOT applied;
//      Verhaard gates on a pure cumulative budget.]
//   3. DEPTH-GATED PIECE QUOTA {(10,4),(20,7),(30,9),(40,12),(50,14),(60,16),
//      (79,18),(95,19),(100,20)} — recovered from binary .data @ 0x42f000; the
//      per-depth minimum count of "privileged"/good-group pieces on the board.
//      [recovered, high confidence — schedule-expansion loop @ .text 0x4144a1]
//   4. RESTART PORTFOLIO with shuffled opening + per-attempt node cap.
//
// Score convention: SCORE = 480 - (#breaks) on a full 256-placement board.
// A break = a mismatched interior edge between two placed pieces, OR a placed
// piece whose border-facing edge is not the border color. Internal adjacencies
// counted once (right + down from each cell). Matches blackwood_bt.rs & the
// vol-232 verifier.

#![forbid(unsafe_code)]

// Real recovered good groups (60 × (tier_a, tier_b) piece-id slices).
include!("verhaard_good_groups.in");

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
const EMPTYP: u16 = u16::MAX;

// ---------------------------------------------------------------------------
// PRNG — splitmix64-seeded xoshiro256** (same as blackwood_bt).
// ---------------------------------------------------------------------------
#[derive(Clone)]
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
// RECOVERED CONSTANTS (see vault/papers/vol-234/R7-VERHAARD-RE.md).
// ---------------------------------------------------------------------------

// [RECOVERED, high confidence] Depth-gated piece quota, binary .data @ 0x42f000,
// expanded into a per-depth table by the schedule-expansion loop at .text
// 0x4144a1 (strides of 2 u32, terminated by 0/0xFFFFFFFF). Pairs are
// (depth, min-good-count): at each depth the board must already carry at least
// this many "good group" pieces or the branch is pruned.
const VERHAARD_QUOTA_PAIRS: [(usize, u16); 9] = [
    (10, 4),
    (20, 7),
    (30, 9),
    (40, 12),
    (50, 14),
    (60, 16),
    (79, 18),
    (95, 19),
    (100, 20),
];

fn build_quota_verhaard(scale: f32) -> [u16; 256] {
    // piecewise-constant step from the recovered pairs, held to the tail.
    let mut a = [0u16; 256];
    let mut cur = 0u16;
    let mut pi = 0usize;
    for (d, slot) in a.iter_mut().enumerate() {
        while pi < VERHAARD_QUOTA_PAIRS.len() && VERHAARD_QUOTA_PAIRS[pi].0 <= d {
            cur = VERHAARD_QUOTA_PAIRS[pi].1;
            pi += 1;
        }
        *slot = ((cur as f32) * scale) as u16;
    }
    a
}

// [RECOVERED/PUBLISHED] Depth-gated edge-slip schedule. shortestpath.se lists
// the tuned-for-468 slip array {193,1, 202,2, 209,3, 214,4, 218,5, 222,6,
// 226,7, 229,8, 232,9, 235,10, 238,11, 240,12}: at depths >= the given depth,
// at most `budget` cumulative INTERIOR-edge mismatches are permitted. Border
// edges never slip. This array is NOT a static u32 table in the v1.0 binary
// (the depths 193-240 do not appear as immediates or .data), so it is tagged
// [estimated-from-published] rather than byte-recovered.
const VERHAARD_SLIP_PAIRS: [(usize, u8); 12] = [
    (193, 1),
    (202, 2),
    (209, 3),
    (214, 4),
    (218, 5),
    (222, 6),
    (226, 7),
    (229, 8),
    (232, 9),
    (235, 10),
    (238, 11),
    (240, 12),
];

#[inline]
fn slip_budget_at(depth: usize) -> u8 {
    let mut b = 0u8;
    for &(d, budget) in VERHAARD_SLIP_PAIRS.iter() {
        if depth >= d {
            b = budget;
        }
    }
    b
}

// [GAP 2 — adaptive restart-abort]. The binary tracks a most-visited-depth
// histogram (.text 0x4144e0, dividing by 0x11=17) and, per its published note,
// aborts a restart on "failure to reach depth 128 in N nodes" (also: zero deep
// completions in 4e9, most-visited depth < 165). The exact tuned thresholds are
// not a static array in v1.0. R8 implements the DISCIPLINE: at each node
// checkpoint (depth, node_budget), if the restart's deepest depth so far is
// below the checkpoint depth, the restart is abandoned so the portfolio spends
// its time on promising restarts. Knots interpolate the published trajectory
// (must clear depth 128 by ~the mid budget, depth ~165 by the full budget). The
// (fraction-of-nodecap, min-depth) pairs below are scaled by --nodecap.
const VERHAARD_ABORT_KNOTS: [(f64, usize); 4] = [
    (0.05, 90),  // by 5% of budget: must have reached depth 90
    (0.15, 128), // by 15%: depth 128 (the binary's explicit knot)
    (0.40, 151), // by 40%: depth 151
    (0.70, 165), // by 70%: depth 165 (most-visited-depth floor)
];

// ---------------------------------------------------------------------------
// Rotated-piece table.
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
    let [t, ri, b, l] = edges;
    match r {
        0 => [t, ri, b, l],
        1 => [l, t, ri, b],
        2 => [b, l, t, ri],
        _ => [ri, b, l, t],
    }
}

// ---------------------------------------------------------------------------
// COMB FILL ORDER (Verhaard). The published shape fills the border RING first
// (corners then edge band), then the interior "most rows horizontal, remaining
// rows vertical" (comb/teeth). We build a permutation of 0..256 that:
//   (a) places the frame ring (60 cells) first, in a top->bottom row scan of
//       only the ring cells (so every ring cell's top+left ring-neighbor is
//       placed before it), then
//   (b) fills the 14x14 interior in a comb: `horiz_rows` interior rows scanned
//       horizontally (top->bottom, left->right), then the remaining interior
//       rows scanned VERTICALLY (column-major, left->right) — the "teeth".
// Every interior cell's top and left neighbors are always already placed:
//   - horizontal rows: left+top placed by construction;
//   - vertical teeth: top placed (earlier in same column or a horiz row above),
//     left placed (a full earlier column, or ring). We enforce this by scanning
//     teeth columns left->right, top->bottom, AFTER all horizontal rows.
// This preserves the exact-match fast path while realizing the comb geometry.
// ---------------------------------------------------------------------------
fn build_comb_order(horiz_rows: usize) -> Vec<usize> {
    // `horiz_rows` = number of TOP board rows filled fully horizontally (incl.
    // their frame cells). The remaining bottom rows are filled as VERTICAL TEETH
    // (column-major). This realizes Verhaard's "most rows horizontal, remaining
    // rows vertical" comb while GUARANTEEING every cell's top+left neighbor is
    // placed before it (invariant required by the exact-match fast path — no
    // wildcard neighbors, keeping the hot path fully engaged).
    let mut order = Vec::with_capacity(N);
    // split_y rows (0..split_y) are pure row-major. We treat horiz_rows as the
    // count of INTERIOR horizontal rows; the top frame row is always horizontal.
    let split_y = (1 + horiz_rows).min(W - 1);

    // (a) top band: rows 0..split_y fully row-major (frame + interior together).
    for y in 0..split_y {
        for x in 0..W {
            order.push(y * W + x);
        }
    }
    // (b) bottom band rows split_y..W-1 as vertical teeth:
    let rem_ys: Vec<usize> = (split_y..W).collect();
    //   left frame column first (its left neighbor is the border),
    for &y in rem_ys.iter() {
        order.push(y * W); // x = 0
    }
    //   then interior columns left->right, each top->bottom (a "tooth"),
    for x in 1..W - 1 {
        for &y in rem_ys.iter() {
            order.push(y * W + x);
        }
    }
    //   then the right frame column.
    for &y in rem_ys.iter() {
        order.push(y * W + (W - 1));
    }

    debug_assert_eq!(order.len(), N, "comb order must cover all 256 cells");
    order
}

// ---------------------------------------------------------------------------
// Solver state (mirrors blackwood_bt.rs; break-gating swapped to Verhaard's).
// ---------------------------------------------------------------------------
// Clone: each parallel restart worker takes its own Solver. The bulk
// (rotpieces / cand_by_tl / cand_by_tl_pristine) is a few hundred KB of tables
// that are read-only during a search, so a per-worker copy is cheap.
#[derive(Clone)]
struct Solver {
    npieces: usize,
    quota: [u16; 256],
    order: Vec<usize>,
    border_mask: [[bool; 4]; N],
    hint_at: [Option<(u16, u8)>; N],
    use_quota: bool,
    use_slip: bool,
    max_breaks: u8,

    rotpieces: Vec<[RotatedPiece; 4]>,
    distinct_rots: Vec<u8>,
    ncolors: usize,
    cand_by_tl: Vec<Vec<(u16, u8)>>,
    // Construction-order copy of cand_by_tl, never mutated after Solver::new.
    // reshuffle_candidates restores from this so each restart's opening depends
    // only on its own seed (see reshuffle_candidates).
    cand_by_tl_pristine: Vec<Vec<(u16, u8)>>,

    // [GAP 1] real good-group membership for the ACTIVE group (set per restart).
    // good_a[pid] = pid in tier-A (privileged 150) of the active group;
    // good_b[pid] = pid in tier-B (103). The quota counts tier-A members placed.
    good_a: Vec<bool>,
    good_b: Vec<bool>,
    active_group: usize,
    use_real_groups: bool,
    // per-piece quota contribution under the ACTIVE group. Real-groups mode:
    // 1 if pid in tier-A else 0. Fallback (color proxy): max heuristic_count over
    // the piece's orientations (rotation-independent so place/unplace balances).
    quota_contrib: Vec<u16>,

    board_piece: [u16; N],
    board_rot: [u8; N],
    used: Vec<bool>,
    priv_on_board: u16,
    breaks_used: u8,

    nodes: u64,
    node_cap: u64,
    // adaptive abort bookkeeping
    abort_enabled: bool,
    aborted: bool,
    verbose: bool,
    best_score: u16,
    best_board: Option<([u16; N], [u8; N])>,
    deepest: usize,
    best_partial: Option<([u16; N], [u8; N], usize)>,
    completed: Vec<u32>,
}

impl Solver {
    #[allow(clippy::too_many_arguments)]
    fn new(
        edges: Vec<[u8; 4]>,
        privileged_colors: &[u8],
        order: Vec<usize>,
        hints: &[(usize, u16, u8)],
        use_quota: bool,
        use_slip: bool,
        max_breaks: u8,
        node_cap: u64,
        quota_scale: f32,
        use_real_groups: bool,
        abort_enabled: bool,
    ) -> Self {
        let npieces = edges.len();
        let mut privileged = [false; 256];
        for &c in privileged_colors {
            privileged[c as usize] = true;
        }
        let quota = build_quota_verhaard(quota_scale);

        let mut border_mask = [[false; 4]; N];
        for (pos, bm) in border_mask.iter_mut().enumerate() {
            let x = pos % W;
            let y = pos / W;
            *bm = [y == 0, x == W - 1, y == W - 1, x == 0];
        }

        let mut hint_at: [Option<(u16, u8)>; N] = [None; N];
        for &(pos, pid, rot) in hints {
            hint_at[pos] = Some((pid, rot));
        }

        let mut rotpieces = Vec::with_capacity(npieces);
        let mut distinct_rots = Vec::with_capacity(npieces);
        for e in edges.iter().copied() {
            let mut arr = [RotatedPiece { top: 0, right: 0, bottom: 0, left: 0, heuristic_count: 0 }; 4];
            let mut seen: Vec<[u8; 4]> = Vec::new();
            for r in 0..4u8 {
                let re = rotate(e, r);
                let hc = re.iter().filter(|&&c| privileged[c as usize]).count() as u8;
                arr[r as usize] =
                    RotatedPiece { top: re[0], right: re[1], bottom: re[2], left: re[3], heuristic_count: hc };
                if !seen.contains(&re) {
                    seen.push(re);
                }
            }
            distinct_rots.push(seen.len() as u8);
            rotpieces.push(arr);
        }

        let ncolors = edges
            .iter()
            .flat_map(|e| e.iter().copied())
            .max()
            .unwrap_or(0) as usize
            + 1;
        let mut cand_by_tl: Vec<Vec<(u16, u8)>> = vec![Vec::new(); ncolors * ncolors];
        for pid in 0..npieces {
            let ndr = distinct_rots[pid] as usize;
            for r in 0..ndr {
                let rp = rotpieces[pid][r];
                let key = (rp.top as usize) * ncolors + (rp.left as usize);
                cand_by_tl[key].push((pid as u16, r as u8));
            }
        }
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
            use_slip,
            max_breaks,
            rotpieces,
            distinct_rots,
            ncolors,
            cand_by_tl_pristine: cand_by_tl.clone(),
            cand_by_tl,
            good_a: vec![false; npieces],
            good_b: vec![false; npieces],
            active_group: 0,
            use_real_groups,
            quota_contrib: vec![0u16; npieces],
            board_piece: [EMPTYP; N],
            board_rot: [0u8; N],
            used: vec![false; npieces],
            priv_on_board: 0,
            breaks_used: 0,
            nodes: 0,
            node_cap,
            abort_enabled,
            aborted: false,
            verbose: false,
            best_score: 0,
            best_board: None,
            deepest: 0,
            best_partial: None,
            completed: vec![0; 481],
        }
    }

    /// Re-derive this restart's candidate order from the pristine
    /// (construction-order) table rather than from the previous restart's
    /// leftovers.
    ///
    /// This used to shuffle `cand_by_tl` in place, and `reset` deliberately
    /// keeps the catalog, so restart K's opening was a shuffle-of-a-shuffle-of-
    /// ... over restarts 0..K on that Solver -- a restart's result depended on
    /// how many had run before it, which is not what a seeded restart portfolio
    /// means and is the hidden coupling that would make a per-thread Solver
    /// disagree with the serial run. Same defect as blackwood_bt's (same
    /// lineage); same fix. Restore first, so a restart is a pure function of its
    /// own (seed, group).
    ///
    /// Ordering note: the caller runs `set_active_group` BEFORE this, and the
    /// stable-sort below ranks by `value_rank`, which depends on the active
    /// group. Restoring the pristine order here (not in `reset`) keeps that
    /// dependency intact -- the group's ranking is applied fresh over pristine
    /// order every restart.
    fn reshuffle_candidates(&mut self, rng: &mut Rng) {
        self.cand_by_tl.clone_from(&self.cand_by_tl_pristine);
        for list in self.cand_by_tl.iter_mut() {
            rng.shuffle(list);
        }
        // precompute per-piece ordering value (depends on active group) then
        // stable-sort by it desc (shuffle survives as the tie-break => start
        // diversity across restarts, as in the binary's seeded restart config).
        let rank: Vec<u16> = (0..self.npieces).map(|p| self.value_rank(p)).collect();
        for list in self.cand_by_tl.iter_mut() {
            list.sort_by(|a, b| rank[b.0 as usize].cmp(&rank[a.0 as usize]));
        }
    }

    // [GAP 1] Select the active good group for this restart and rebuild the
    // per-piece quota contribution. Mirrors the binary's per-restart `rand % 60`
    // group pick (.text 0x4145a6) that loads the group's tier-A/tier-B piece sets
    // into the membership arrays consulted by the quota gate (.text 0x403c80).
    fn set_active_group(&mut self, group_idx: usize) {
        self.active_group = group_idx % VERHAARD_NUM_GROUPS;
        for v in self.good_a.iter_mut() {
            *v = false;
        }
        for v in self.good_b.iter_mut() {
            *v = false;
        }
        if self.use_real_groups {
            let (ta, tb) = VERHAARD_GOOD_GROUPS[self.active_group];
            for &pid in ta {
                if (pid as usize) < self.good_a.len() {
                    self.good_a[pid as usize] = true;
                }
            }
            for &pid in tb {
                if (pid as usize) < self.good_b.len() {
                    self.good_b[pid as usize] = true;
                }
            }
        }
        // build per-piece quota contribution.
        for pid in 0..self.npieces {
            self.quota_contrib[pid] = if self.use_real_groups {
                u16::from(self.good_a[pid])
            } else {
                // fallback color-proxy: rotation-independent max privileged count.
                let ndr = self.distinct_rots[pid] as usize;
                (0..ndr)
                    .map(|r| self.rotpieces[pid][r].heuristic_count)
                    .max()
                    .unwrap_or(0) as u16
            };
        }
    }

    fn reset(&mut self) {
        self.board_piece = [EMPTYP; N];
        self.board_rot = [0u8; N];
        for u in self.used.iter_mut() {
            *u = false;
        }
        self.priv_on_board = 0;
        self.breaks_used = 0;
        self.nodes = 0;
        self.aborted = false;
        for pos in 0..N {
            if let Some((pid, rot)) = self.hint_at[pos] {
                self.board_piece[pos] = pid;
                self.board_rot[pos] = rot;
                self.used[pid as usize] = true;
                self.priv_on_board += self.quota_contrib[pid as usize];
            }
        }
    }

    #[inline]
    fn neighbor_colors(&self, pos: usize) -> (Option<u8>, Option<u8>, Option<u8>, Option<u8>) {
        let x = pos % W;
        let y = pos / W;
        let above = if y > 0 && self.board_piece[pos - W] != EMPTYP {
            Some(self.oriented(pos - W).bottom)
        } else {
            None
        };
        let left = if x > 0 && self.board_piece[pos - 1] != EMPTYP {
            Some(self.oriented(pos - 1).right)
        } else {
            None
        };
        let right = if x < W - 1 && self.board_piece[pos + 1] != EMPTYP {
            Some(self.oriented(pos + 1).left)
        } else {
            None
        };
        let below = if y < W - 1 && self.board_piece[pos + W] != EMPTYP {
            Some(self.oriented(pos + W).top)
        } else {
            None
        };
        (above, left, right, below)
    }

    #[inline]
    fn oriented(&self, pos: usize) -> RotatedPiece {
        self.rotpieces[self.board_piece[pos] as usize][self.board_rot[pos] as usize]
    }

    // Count breaks a candidate at `pos` would create vs placed neighbors and
    // the border. Verhaard rule: BORDER-facing edges NEVER slip (a non-border
    // color on a frame edge is a HARD reject, returns None); only INTERIOR edge
    // mismatches count toward the slip budget. Returns Some(num_interior_breaks)
    // or None if border-illegal.
    #[inline]
    fn eval_candidate(&self, pos: usize, rp: &RotatedPiece) -> Option<u8> {
        let x = pos % W;
        let y = pos / W;
        let bm = self.border_mask[pos];
        let mut breaks = 0u8;

        // Border edges are hard constraints (never slip).
        if bm[0] && rp.top != BORDER {
            return None;
        }
        if bm[1] && rp.right != BORDER {
            return None;
        }
        if bm[2] && rp.bottom != BORDER {
            return None;
        }
        if bm[3] && rp.left != BORDER {
            return None;
        }
        // A non-border piece must not present BORDER color on an interior edge
        // (keeps completions physical). Interior edge = a side that is not a
        // frame edge.
        if !bm[0] && rp.top == BORDER {
            return None;
        }
        if !bm[1] && rp.right == BORDER {
            return None;
        }
        if !bm[2] && rp.bottom == BORDER {
            return None;
        }
        if !bm[3] && rp.left == BORDER {
            return None;
        }

        // interior mismatches vs placed neighbors (slippable).
        if y > 0 {
            let np = pos - W;
            if self.board_piece[np] != EMPTYP && self.oriented(np).bottom != rp.top {
                breaks += 1;
            }
        }
        if x < W - 1 {
            let np = pos + 1;
            if self.board_piece[np] != EMPTYP && self.oriented(np).left != rp.right {
                breaks += 1;
            }
        }
        if y < W - 1 {
            let np = pos + W;
            if self.board_piece[np] != EMPTYP && self.oriented(np).top != rp.bottom {
                breaks += 1;
            }
        }
        if x > 0 {
            let np = pos - 1;
            if self.board_piece[np] != EMPTYP && self.oriented(np).right != rp.left {
                breaks += 1;
            }
        }
        Some(breaks)
    }

    #[inline]
    fn should_stop(&mut self) -> bool {
        if self.aborted || self.nodes >= self.node_cap {
            return true;
        }
        // [GAP 2] adaptive restart-abort: kill the restart early if its deepest
        // depth is below the expected trajectory at this node checkpoint.
        if self.abort_enabled {
            let frac = self.nodes as f64 / self.node_cap as f64;
            for &(kf, kd) in VERHAARD_ABORT_KNOTS.iter() {
                if frac >= kf && self.deepest < kd {
                    self.aborted = true;
                    return true;
                }
            }
        }
        false
    }

    fn search(&mut self, depth: usize) {
        if self.should_stop() {
            return;
        }
        if depth == N {
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
            self.best_partial = Some((self.board_piece, self.board_rot, depth));
            if self.verbose && depth >= 195 {
                eprintln!("#   new deepest depth={depth} breaks={} nodes={}", self.breaks_used, self.nodes);
            }
        }
        let pos = self.order[depth];

        if self.hint_at[pos].is_some() {
            self.search(depth + 1);
            return;
        }

        self.nodes += 1;

        if self.use_quota && self.priv_on_board < self.quota[depth] {
            return;
        }

        let budget = if self.use_slip {
            slip_budget_at(depth).min(self.max_breaks)
        } else {
            0
        };

        let (top_req, left_req, right_req, below_req) = self.neighbor_colors(pos);
        let bm = self.border_mask[pos];

        // Determine required top/left colors. If a neighbor is an unplaced
        // interior cell (can happen in the comb order where a right/bottom
        // frame cell precedes its interior neighbor), the color is a free
        // wildcard and we must use the GENERAL constrained scan below rather
        // than the exact (top,left) table, otherwise the cell would be skipped.
        let treq_opt: Option<u8> = match top_req {
            Some(c) => Some(c),
            None if bm[0] => Some(BORDER),
            None => None, // interior top neighbor not yet placed -> wildcard
        };
        let lreq_opt: Option<u8> = match left_req {
            Some(c) => Some(c),
            None if bm[3] => Some(BORDER),
            None => None, // interior left neighbor not yet placed -> wildcard
        };

        if budget == 0 && treq_opt.is_some() && lreq_opt.is_some() {
            // FAST PATH: no slip allowed AND both top+left are determined.
            let treq = treq_opt.unwrap();
            let lreq = lreq_opt.unwrap();
            let key = (treq as usize) * self.ncolors + (lreq as usize);
            let list_len = self.cand_by_tl[key].len();
            for idx in 0..list_len {
                if self.should_stop() {
                    return;
                }
                let (pid16, r) = self.cand_by_tl[key][idx];
                let pid = pid16 as usize;
                if self.used[pid] {
                    continue;
                }
                let rp = self.rotpieces[pid][r as usize];
                if let Some(rc) = right_req {
                    if rp.right != rc {
                        continue;
                    }
                } else if bm[1] && rp.right != BORDER {
                    continue;
                } else if !bm[1] && rp.right == BORDER {
                    continue;
                }
                if let Some(bc) = below_req {
                    if rp.bottom != bc {
                        continue;
                    }
                } else if bm[2] && rp.bottom != BORDER {
                    continue;
                } else if !bm[2] && rp.bottom == BORDER {
                    continue;
                }
                let contrib = self.quota_contrib[pid];
                self.board_piece[pos] = pid16;
                self.board_rot[pos] = r;
                self.used[pid] = true;
                self.priv_on_board += contrib;
                self.search(depth + 1);
                self.priv_on_board -= contrib;
                self.used[pid] = false;
                self.board_piece[pos] = EMPTYP;
            }
            return;
        }

        // GENERAL / SLIP PATH. Reached when slip is active (budget>0) OR when the
        // fast path's exact (top,left) key is unavailable because a neighbor is
        // an unplaced interior cell (comb order). Full scan with cumulative
        // interior-break accounting against the Verhaard budget (budget may be
        // 0 here, in which case only perfect placements survive). Border faces
        // are hard-enforced in eval_candidate. No non-adjacency rule.
        let mut cands: Vec<(u16, u8, u8)> = Vec::with_capacity(64);
        for pid in 0..self.npieces {
            if self.used[pid] {
                continue;
            }
            let ndr = self.distinct_rots[pid] as usize;
            for r in 0..ndr {
                let rp = self.rotpieces[pid][r];
                if let Some(nb) = self.eval_candidate(pos, &rp) {
                    if self.breaks_used + nb <= budget {
                        cands.push((pid as u16, r as u8, nb));
                    }
                }
            }
        }
        if cands.is_empty() {
            return;
        }
        // value-order: good-group tier desc (real groups: tier-A>tier-B>none;
        // fallback: privileged-color count), then fewer breaks.
        cands.sort_by(|a, b| {
            let va = self.value_rank(a.0 as usize);
            let vb = self.value_rank(b.0 as usize);
            vb.cmp(&va).then(a.2.cmp(&b.2))
        });

        for i in 0..cands.len() {
            if self.should_stop() {
                return;
            }
            let (pid16, r, nb) = cands[i];
            let pid = pid16 as usize;
            let contrib = self.quota_contrib[pid];
            self.board_piece[pos] = pid16;
            self.board_rot[pos] = r;
            self.used[pid] = true;
            self.priv_on_board += contrib;
            self.breaks_used += nb;
            self.search(depth + 1);
            self.breaks_used -= nb;
            self.priv_on_board -= contrib;
            self.used[pid] = false;
            self.board_piece[pos] = EMPTYP;
        }
    }

    // Ordering value of a piece under the active group.
    #[inline]
    fn value_rank(&self, pid: usize) -> u16 {
        if self.use_real_groups {
            if self.good_a[pid] {
                2
            } else if self.good_b[pid] {
                1
            } else {
                0
            }
        } else {
            let ndr = self.distinct_rots[pid] as usize;
            (0..ndr)
                .map(|r| self.rotpieces[pid][r].heuristic_count)
                .max()
                .unwrap_or(0) as u16
        }
    }

    fn count_breaks(&self, bp: &[u16; N], br: &[u8; N]) -> u16 {
        let mut breaks = 0u16;
        for pos in 0..N {
            if bp[pos] == EMPTYP {
                continue;
            }
            let rp = self.rotpieces[bp[pos] as usize][br[pos] as usize];
            let x = pos % W;
            let y = pos / W;
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
            if x < W - 1 && bp[pos + 1] != EMPTYP {
                let rn = self.rotpieces[bp[pos + 1] as usize][br[pos + 1] as usize];
                if rp.right != rn.left {
                    breaks += 1;
                }
            }
            if y < W - 1 && bp[pos + W] != EMPTYP {
                let dn = self.rotpieces[bp[pos + W] as usize][br[pos + W] as usize];
                if rp.bottom != dn.top {
                    breaks += 1;
                }
            }
        }
        480u16.saturating_sub(breaks)
    }

    fn is_physical(&self, bp: &[u16; N], br: &[u8; N]) -> bool {
        for pos in 0..N {
            if bp[pos] == EMPTYP {
                if self.verbose {
                    eprintln!("#   is_physical: empty cell at pos={pos}");
                }
                return false;
            }
            let rp = self.rotpieces[bp[pos] as usize][br[pos] as usize];
            let x = pos % W;
            let y = pos / W;
            let e = [rp.top, rp.right, rp.bottom, rp.left];
            let border_face = [y == 0, x == W - 1, y == W - 1, x == 0];
            for s in 0..4 {
                if border_face[s] != (e[s] == BORDER) {
                    if self.verbose {
                        eprintln!("#   is_physical FAIL pos={pos} (x={x},y={y}) side={s} edge={} border_face={} pid={}", e[s], border_face[s], bp[pos]);
                    }
                    return false;
                }
            }
        }
        true
    }

    fn greedy_complete(&self, bp0: &[u16; N], br0: &[u8; N], depth: usize) -> ([u16; N], [u8; N], u16) {
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
                continue;
            }
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
            let nb_border =
                (x == 0) as u8 + (x == W - 1) as u8 + (y == 0) as u8 + (y == W - 1) as u8;
            let want_kind = nb_border;
            let mut best: Option<(u16, u8, u8)> = None;
            for pid in 0..self.npieces {
                if used[pid] {
                    continue;
                }
                let e0 = self.rotpieces[pid][0];
                let pk = (e0.top == BORDER) as u8
                    + (e0.right == BORDER) as u8
                    + (e0.bottom == BORDER) as u8
                    + (e0.left == BORDER) as u8;
                if pk != want_kind {
                    continue;
                }
                let ndr = self.distinct_rots[pid] as usize;
                let bm = self.border_mask[pos];
                for r in 0..ndr {
                    let rp = self.rotpieces[pid][r];
                    // HARD physicality: border-facing sides must be BORDER and
                    // interior-facing sides must NOT be BORDER (same rule as the
                    // search). This keeps every greedy completion physical.
                    let e = [rp.top, rp.right, rp.bottom, rp.left];
                    let mut phys_ok = true;
                    for s in 0..4 {
                        if bm[s] != (e[s] == BORDER) {
                            phys_ok = false;
                            break;
                        }
                    }
                    if !phys_ok {
                        continue;
                    }
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

fn run_attempt(solver: &mut Solver, rng: &mut Rng, shuffle_open: bool, group_idx: usize) {
    solver.set_active_group(group_idx);
    solver.reset();
    if shuffle_open {
        solver.reshuffle_candidates(rng);
    }
    solver.search(0);
}

/// RNG seed for restart `attempt` of a run seeded `seed`.
///
/// This used to be `(seed + attempt) * K`, which ALIASES -- `seed+attempt` is a
/// single number, so (seed=7, attempt=1) and (seed=8, attempt=0) are the same
/// key and give the same opening AND the same good-group pick. Consecutive seeds
/// shared all but one of their restarts. Identical defect (and identical
/// formula) to blackwood_bt's; identical fix -- mix both coordinates with
/// distinct odd multipliers, then a SplitMix64 finalizer to decorrelate
/// near-identical inputs.
#[inline]
fn attempt_key(seed: u64, attempt: usize) -> u64 {
    let mut z = seed
        .wrapping_mul(0x9E37_79B9_7F4A_7C15)
        .wrapping_add((attempt as u64).wrapping_mul(0xD1B5_4A32_D192_ED03));
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// What one restart produced. Extracted so a restart is a pure
/// `attempt -> AttemptOutcome` function over its own Solver, which is what lets
/// restarts run concurrently.
struct AttemptOutcome {
    score: u16,
    board: Option<([u16; N], [u8; N])>,
    physical: bool,
    native: bool,
    deepest: usize,
    nodes: u64,
}

/// Run restart `attempt` to completion on its own 256MB-stack thread (the
/// 256-deep DFS overflows the default 8MB; the big stacks are lazily committed,
/// so one per worker is cheap) and report what it found. `solver` is this
/// worker's private Solver, so nothing is shared with concurrent restarts.
fn run_attempt_scored(
    solver: &mut Solver,
    seed: u64,
    attempt: usize,
    shuffle: bool,
    fixed_group: Option<usize>,
) -> AttemptOutcome {
    let mut rng = Rng::new(attempt_key(seed, attempt));
    // [GAP 1] per-restart good-group pick (binary: rand % 60 @ .text 0x4145a6).
    // Drawn from this restart's own stream, before the search, exactly as the
    // serial loop did.
    let group_idx =
        fixed_group.unwrap_or_else(|| (rng.next_u64() % VERHAARD_NUM_GROUPS as u64) as usize);
    solver.best_score = 0;
    solver.best_board = None;
    solver.deepest = 0;
    solver.best_partial = None;
    solver.nodes = 0;

    std::thread::scope(|s| {
        std::thread::Builder::new()
            .stack_size(256 * 1024 * 1024)
            .spawn_scoped(s, || {
                run_attempt(solver, &mut rng, shuffle, group_idx);
            })
            .expect("spawn search thread")
            .join()
            .expect("search thread panicked");
    });

    let native = solver.best_score > 0;
    let (score, board): (u16, Option<([u16; N], [u8; N])>) = if native {
        (solver.best_score, solver.best_board)
    } else if let Some((bp, br, d)) = solver.best_partial {
        let (cbp, cbr, sc) = solver.greedy_complete(&bp, &br, d);
        (sc, Some((cbp, cbr)))
    } else {
        (0, None)
    };
    let physical = board.as_ref().is_some_and(|(bp, br)| solver.is_physical(bp, br));

    AttemptOutcome { score, board, physical, native, deepest: solver.deepest, nodes: solver.nodes }
}

fn board_url(puzzle: &eternity2_core::Puzzle, bp: &[u16; N], br: &[u8; N]) -> String {
    let mut b = Board::empty(puzzle);
    for pos in 0..N {
        if bp[pos] != EMPTYP {
            b.place(pos as u32, bp[pos], Rotation::from_u8(br[pos]).unwrap());
        }
    }
    bucas_url(puzzle, &b, "size_16_official_eternity")
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!(
            "usage: verhaard_faithful <puzzle.csv> [--stage 1|2|3] [--priv c1,c2,c3] \
             [--maxbreaks K] [--nodecap N] [--restarts R] [--seed S] [--threads T] \
             [--horiz-rows H] [--quota-scale F] [--emit-dir DIR] [--emit-prefix P] [--verbose]\n\
             stages: 1=comb DFS only, 2=+slip schedule, 3=+quota (full engine)"
        );
        std::process::exit(2);
    }
    let csv = PathBuf::from(&args[1]);

    let mut stage = 3usize;
    let mut priv_colors: Vec<u8> = vec![22, 21, 20];
    let mut max_breaks = 12u8;
    let mut node_cap = 200_000_000u64;
    let mut restarts = 1usize;
    let mut seed = 0xE2E2u64;
    let mut horiz_rows = 12usize; // "most rows horizontal": 12 of 14 interior rows
    let mut verbose = false;
    let mut quota_scale = 1.0f32;
    let mut emit_dir: Option<String> = None;
    let mut emit_prefix = "vh".to_string();
    let mut use_real_groups = true; // [GAP 1] default: real recovered good groups
    let mut abort_enabled = true; // [GAP 2] default: adaptive restart-abort on
    let mut fixed_group: Option<usize> = None; // pin a specific group (else rand)
    let mut threads: usize = 1; // 1 = serial restart portfolio (the default)

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--stage" => {
                stage = args[i + 1].parse().unwrap();
                i += 2;
            }
            "--priv" => {
                priv_colors = args[i + 1].split(',').map(|s| s.trim().parse().unwrap()).collect();
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
            "--threads" => {
                threads = args[i + 1].parse().unwrap();
                i += 2;
            }
            "--restarts" => {
                restarts = args[i + 1].parse().unwrap();
                i += 2;
            }
            "--seed" => {
                seed = args[i + 1].parse().unwrap();
                i += 2;
            }
            "--horiz-rows" => {
                horiz_rows = args[i + 1].parse().unwrap();
                i += 2;
            }
            "--quota-scale" => {
                quota_scale = args[i + 1].parse().unwrap();
                i += 2;
            }
            "--verbose" => {
                verbose = true;
                i += 1;
            }
            "--emit-dir" => {
                emit_dir = Some(args[i + 1].clone());
                i += 2;
            }
            "--emit-prefix" => {
                emit_prefix = args[i + 1].clone();
                i += 2;
            }
            "--groups" => {
                use_real_groups = matches!(args[i + 1].as_str(), "on" | "1" | "real" | "true");
                i += 2;
            }
            "--abort" => {
                abort_enabled = matches!(args[i + 1].as_str(), "on" | "1" | "true");
                i += 2;
            }
            "--group" => {
                fixed_group = Some(args[i + 1].parse().unwrap());
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

    let mut edges = vec![[0u8; 4]; npieces];
    for p in puzzle.pieces() {
        edges[p.id as usize] = p.edges.as_array();
    }

    let hints: Vec<(usize, u16, u8)> = hints_raw
        .hints
        .iter()
        .map(|h| (h.position as usize, h.piece_id, h.rotation.as_u8()))
        .collect();

    let (use_quota, use_slip) = match stage {
        1 => (false, false),
        2 => (false, true),
        _ => (true, true),
    };

    let order = build_comb_order(horiz_rows);

    let mut solver = Solver::new(
        edges,
        &priv_colors,
        order,
        &hints,
        use_quota,
        use_slip,
        max_breaks,
        node_cap,
        quota_scale,
        use_real_groups,
        abort_enabled,
    );
    solver.verbose = verbose;

    eprintln!(
        "# verhaard_faithful_v2 stage={stage} groups={} abort={abort_enabled} \
         fixed_group={fixed_group:?} priv={priv_colors:?} maxbreaks={max_breaks} \
         nodecap={node_cap} restarts={restarts} seed={seed} horiz_rows={horiz_rows} \
         quota_scale={quota_scale}",
        if use_real_groups { "REAL-60" } else { "color-proxy" }
    );

    if threads == 0 {
        threads = std::thread::available_parallelism().map_or(1, std::num::NonZero::get);
    }
    // Never spin up more workers than there are restarts to run.
    threads = threads.min(restarts.max(1));
    if threads > 1 {
        eprintln!("# --threads {threads}: restarts run concurrently; per-restart results are unchanged (each depends only on its own seed), but \"new best\" log order reflects completion order.");
    }

    let t0 = Instant::now();

    let emit_dir_s = emit_dir.clone().unwrap_or_else(|| ".".into());
    std::fs::create_dir_all(&emit_dir_s).ok();
    let checkpoint_path = Path::new(&emit_dir_s).join("best_so_far.url");
    let log_path = Path::new(&emit_dir_s).join(format!("{emit_prefix}_portfolio.log"));

    let shuffle = restarts > 1;
    // Shared bookkeeping. Under --threads 1 this is an uncontended mutex around
    // the same updates the serial loop did; under N>1 it serializes only the
    // cheap post-restart bookkeeping and the record/checkpoint writes -- never
    // the search. Keeping the banking inside the lock stops two threads that both
    // beat the record from interleaving their RECORD_*.url writes.
    struct Portfolio<'a> {
        global_best: u16,
        global_best_board: Option<([u16; N], [u8; N])>,
        best_natural: u16,
        best_physical: u16,
        n_physical: u32,
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
        /// Fold one finished restart into the portfolio. Called under the mutex,
        /// so the >best test and the banking that follows are atomic.
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
            if o.physical {
                self.n_physical += 1;
                if o.score > self.best_physical {
                    self.best_physical = o.score;
                }
            }

            if o.score > self.global_best && o.physical {
                self.global_best = o.score;
                self.global_best_board = o.board;
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
            // Keyed on COMPLETED count, not attempt index, so it still fires every
            // 25 restarts when they finish out of order.
            if self.restarts > 1 && self.done % 25 == 0 {
                let el = self.t0.elapsed().as_secs_f64();
                eprintln!(
                    "# progress {}/{} best_complete={} best_native={} deepest={} nodes={} {:.1} Mnode/s ({:.0}s)",
                    self.done,
                    self.restarts,
                    self.global_best,
                    self.best_natural,
                    self.deepest_overall,
                    self.total_nodes,
                    (self.total_nodes as f64 / 1e6) / el.max(1e-9),
                    el,
                );
            }
        }
    }

    let portfolio = Mutex::new(Portfolio {
        global_best: 0,
        global_best_board: None,
        best_natural: 0,
        best_physical: 0,
        n_physical: 0,
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
        // Each worker holds its OWN Solver clone -- with one shared Solver the
        // restarts would race on board_piece/used/nodes. The catalog tables are a
        // few hundred KB and the 256MB search stacks are lazily committed, so a
        // per-worker copy is cheap.
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .expect("build rayon pool");
        pool.install(|| {
            (0..restarts).into_par_iter().for_each_init(
                || solver.clone(),
                |local, attempt| {
                    let o = run_attempt_scored(local, seed, attempt, shuffle, fixed_group);
                    portfolio
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner)
                        .record(attempt, &o);
                },
            );
        });
    } else {
        for attempt in 0..restarts {
            let o = run_attempt_scored(&mut solver, seed, attempt, shuffle, fixed_group);
            portfolio
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .record(attempt, &o);
        }
    }

    let p = portfolio.into_inner().unwrap_or_else(std::sync::PoisonError::into_inner);
    let (global_best, global_best_board, best_natural, best_physical, n_physical, deepest_overall, total_nodes, hist) = (
        p.global_best,
        p.global_best_board,
        p.best_natural,
        p.best_physical,
        p.n_physical,
        p.deepest_overall,
        p.total_nodes,
        p.hist,
    );

    let elapsed = t0.elapsed().as_secs_f64();
    let mnps = (total_nodes as f64 / 1e6) / elapsed.max(1e-9);

    eprintln!("# ===== SUMMARY =====");
    eprintln!("# stage={stage} restarts={restarts} elapsed={elapsed:.2}s nodes={total_nodes} throughput={mnps:.1} Mnode/s");
    eprintln!("# best_complete_score={global_best} best_physical_score={best_physical} best_native_score={best_natural} n_physical={n_physical}/{restarts} deepest_depth={deepest_overall}");
    eprint!("# complete-score histogram (score:count): ");
    for (s, c) in hist.iter().enumerate() {
        if *c > 0 {
            eprint!("{s}:{c} ");
        }
    }
    eprintln!();

    if let Some((bp, br)) = &global_best_board {
        let url = board_url(&puzzle, bp, br);
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let path =
            Path::new(&emit_dir_s).join(format!("{emit_prefix}_stage{stage}_score{global_best}_{ts}.url"));
        std::fs::write(&path, &url).expect("write url");
        eprintln!("# emitted best board -> {}", path.display());
        println!("{}", path.display());
    }

    println!("BEST {global_best}");
}
