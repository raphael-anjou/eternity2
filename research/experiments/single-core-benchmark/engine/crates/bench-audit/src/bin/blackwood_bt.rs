// R4: BLACKWOOD-STYLE DEPTH-GATED BREAK BACKTRACKER.
//
// The community record engines (Blackwood 468-470) are depth-first
// backtrackers that forbid edge mismatches ("breaks") until deep in the board,
// then release them one at a time on a fixed depth schedule. This binary is
// the repo's port of Blackwood's engine (github.com/jblackwood345/
// EternityII_Solver, read at commit f544e42; no source copied). FIDELITY.md
// next to this file lists every known difference from his engine, its status
// and how it was tested.
//
// Algorithm components:
//   1. DFS place/check/backtrack over his fixed scan order
//   2. piece value-ordering by 3 privileged colours (his set by default)
//   3. a 256-entry quota schedule (prune on privileged sides placed)
//   4. a depth-gated cumulative break budget, his break rules
//   5. restart portfolio: shuffled opening + per-attempt node cap (the port's)
//
// Defaults follow his engine. The port's earlier behaviour stays selectable for
// results computed with it: --keying legacy-port, --order row-major,
// --legacy-break-path, --isolation, --legacy-hints, --legacy-quota-test (all
// six together reproduce the earlier port, except that no piece is ever placed
// showing grey toward an interior cell; that defect is fixed unconditionally).
// With other colours, `--priv a,b,c` alone now reads the quota as a share of
// their sides; add --quota-count for the earlier count reading.
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
use eternity2_export::{board_to_doc, BoardDoc};
use eternity2_puzzle_io::load_puzzle_with_hints;

const N: usize = 256; // cells
const W: usize = 16;
const BORDER: u8 = 0;

// ---------------------------------------------------------------------------
// PRNG: splitmix64 seeded xoshiro256**; strictly better than an LCG.
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
// Quota schedule (Blackwood piecewise-linear ramp, `Program.cs`
// heuristic_array). quota[depth] = minimum number of privileged sides on the
// board. The curve is his count for his keying, whose three colours carry
// PUBLISHED_SIDES sides on the official set; build_quota returns that count,
// and QuotaBase says how another keying reads it.
// ---------------------------------------------------------------------------
/// Sides carried by Blackwood's privileged colours on the official set (ring
/// colour 5: 24, interior 20: 50, interior 7: 48, in this repo's numbering).
const PUBLISHED_SIDES: f64 = 122.0;

/// Blackwood's `heuristic_sides` {13, 16, 10} in this repo's colour numbering.
/// His `Get_Pieces()` list maps onto this repo's piece file under exactly one
/// chirality-preserving colour bijection (his piece n is this repo's piece
/// n - 1, same sides up to rotation): his -> repo 1->1, 2->6, 3->11, 4->16,
/// 5->4, 6->9, 7->14, 8->19, 9->2, 10->7, 11->12, 12->17, 13->5, 14->10,
/// 15->15, 16->20, 17->3, 18->8, 19->13, 20->18, 21->21, 22->22, 0 (grey)->0.
const PUBLISHED_KEYING: [u8; 3] = [5, 20, 7];

/// The port's original keying, chosen by an A/B on this numbering when the
/// published labels were (wrongly) used literally.
const LEGACY_PORT_KEYING: [u8; 3] = [22, 21, 20];

/// How the quota curve is read for a given keying.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum QuotaBase {
    /// As a share of the keying's privileged sides T: quota = curve * T / 122,
    /// so every threshold asks the same fraction of T that Blackwood's curve
    /// asks of his 122 sides (identical to his count for his keying).
    Share,
    /// As an absolute count, whatever T is (the port's original reading; it
    /// silently changes meaning when the keying changes).
    Count,
}

/// Resolve `--keying`, `--priv` and `--quota-count` to (colours, base).
/// Keyings: `published` (default, {5, 20, 7}, share) and `legacy-port`
/// ({22, 21, 20}, count). `--priv` replaces the colours and keeps the base;
/// `--quota-count` forces the count reading.
fn resolve_keying(keying: Option<&str>, privileged: Option<Vec<u8>>, quota_count: bool) -> Result<(Vec<u8>, QuotaBase), String> {
    let (colours, base) = match keying.unwrap_or("published") {
        "published" => (PUBLISHED_KEYING.to_vec(), QuotaBase::Share),
        "legacy-port" => (LEGACY_PORT_KEYING.to_vec(), QuotaBase::Count),
        other => return Err(format!("unknown keying {other} (published | legacy-port)")),
    };
    let base = if quota_count { QuotaBase::Count } else { base };
    Ok((privileged.unwrap_or(colours), base))
}

/// Blackwood's curve value at depth i, in f32 as his C# computes it; held at
/// its depth-160 value beyond (his engine stops checking after 160).
fn quota_curve(i: usize) -> f32 {
    let f = i as f32;
    if i <= 16 {
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
        (160.0 - 102.0) / 4.4615 + 106.0
    }
}

/// The curve read as an absolute count (QuotaBase::Count).
fn build_quota(scale: f32) -> [u16; 256] {
    let mut a = [0u16; 256];
    for (i, q) in a.iter_mut().enumerate() {
        *q = (quota_curve(i) * scale) as u16;
    }
    a
}

/// The curve read as a share of `total` privileged sides (see QuotaBase).
/// Exact for total = 122: f64 holds v * 122 without rounding.
fn build_quota_share(scale: f32, total: f64) -> [u16; 256] {
    let mut a = [0u16; 256];
    for (i, q) in a.iter_mut().enumerate() {
        let v = quota_curve(i) * scale;
        *q = (f64::from(v) * total / PUBLISHED_SIDES) as u16;
    }
    a
}

/// Blackwood checks the quota up to this depth (`max_heuristic_index`).
const MAX_HEURISTIC_INDEX: usize = 160;

// Depths at which a cumulative break unlocks (10 breaks total).
const BREAK_UNLOCK: [usize; 10] = [201, 206, 211, 216, 221, 225, 229, 233, 237, 239];


// ---------------------------------------------------------------------------
// Scan order. A scan fixes, for every cell, two sides whose neighbours are
// always placed (or on the frame) when the cell is searched; the no-break hot
// path looks candidates up by the colours on those two "key" sides.
// ---------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ScanOrder {
    /// Blackwood's `Get_Board_Order()` (`Util.cs`): rows from the bottom-left
    /// corner for the first 176 cells, then the top five rows as a band (their
    /// first five columns row by row, the next seven column by column, the last
    /// four in his staircase). Every cell is matched on its left and bottom.
    Published,
    /// The port's original order: rows from the top-left corner, matched on top
    /// and left.
    RowMajor,
}

/// `Get_Board_Order()`'s table as printed in his source: line t is the board
/// row t from the top (his row 15 - t, this repo's y = t), column x; the entry
/// is that cell's index in the scan. His board and this repo's share their
/// orientation (his centre piece 139 at rotation 2 is this repo's piece 138 at
/// rotation 0, at y = 8, x = 7).
const BLACKWOOD_BOARD_ORDER: [[u8; W]; W] = [
    [196, 197, 198, 199, 200, 205, 210, 215, 220, 225, 230, 235, 243, 249, 254, 255],
    [191, 192, 193, 194, 195, 204, 209, 214, 219, 224, 229, 234, 242, 248, 252, 253],
    [186, 187, 188, 189, 190, 203, 208, 213, 218, 223, 228, 233, 241, 247, 250, 251],
    [181, 182, 183, 184, 185, 202, 207, 212, 217, 222, 227, 232, 240, 244, 245, 246],
    [176, 177, 178, 179, 180, 201, 206, 211, 216, 221, 226, 231, 236, 237, 238, 239],
    [160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175],
    [144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159],
    [128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143],
    [112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127],
    [96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111],
    [80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95],
    [64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79],
    [48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63],
    [32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47],
    [16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
];

impl Default for ScanOrder {
    fn default() -> Self {
        Self::Published
    }
}

impl ScanOrder {
    fn parse(name: &str) -> Result<Self, String> {
        match name {
            "published" => Ok(Self::Published),
            "row-major" => Ok(Self::RowMajor),
            other => Err(format!("unknown order {other} (published | row-major)")),
        }
    }

    /// order[depth] = cell (y * W + x).
    fn cells(self) -> Vec<usize> {
        match self {
            Self::RowMajor => (0..N).collect(),
            Self::Published => {
                let mut order = vec![usize::MAX; N];
                for (y, line) in BLACKWOOD_BOARD_ORDER.iter().enumerate() {
                    for (x, &depth) in line.iter().enumerate() {
                        order[depth as usize] = y * W + x;
                    }
                }
                order
            }
        }
    }

    /// The two sides (0 top, 1 right, 2 bottom, 3 left) a cell is matched on.
    const fn key_sides(self) -> [usize; 2] {
        match self {
            Self::Published => [3, 2],
            Self::RowMajor => [0, 3],
        }
    }
}

/// The cell across side `s` of `pos`, if on the board.
#[inline]
const fn neighbour(pos: usize, s: usize) -> Option<usize> {
    let (x, y) = (pos % W, pos / W);
    match s {
        0 if y > 0 => Some(pos - W),
        1 if x < W - 1 => Some(pos + 1),
        2 if y < W - 1 => Some(pos + W),
        3 if x > 0 => Some(pos - 1),
        _ => None,
    }
}

/// A side colour against a `want_on` constraint.
#[inline(always)]
fn fits_want(w: Option<u8>, c: u8) -> bool {
    match w {
        Some(x) => c == x,
        None => c != BORDER,
    }
}

#[inline(always)]
const fn side(rp: &RotatedPiece, s: usize) -> u8 {
    match s {
        0 => rp.top,
        1 => rp.right,
        2 => rp.bottom,
        _ => rp.left,
    }
}

// ---------------------------------------------------------------------------
// Solver state.
// ---------------------------------------------------------------------------
// Clone: each parallel restart worker takes its own Solver. The bulk
// (rotpieces / cand_by_key / cand_by_key_pristine) is a few hundred KB of tables
// that are read-only during a search, so a per-worker copy is cheap.
#[derive(Clone)]
struct Solver {
    npieces: usize,
    // quota schedule
    quota: [u16; 256],
    // scan order (order[depth] = cell) and the two sides it matches cells on
    order: Vec<usize>,
    key_sides: [usize; 2],
    // the other two sides, checked per candidate on the hot path
    other_sides: [usize; 2],
    // border mask per position [t,r,b,l] = true where edge faces gray border
    border_mask: [[bool; 4]; N],
    // hint pinning: hint_at[pos] = Some((pid, rot))
    hint_at: [Option<(u16, u8)>; N],
    // stage flags
    use_quota: bool,
    use_breaks: bool,
    max_breaks: u8,
    // depths at which a cumulative break unlocks (BREAK_UNLOCK by default)
    unlock: Vec<usize>,
    // break budget per depth, filled from the three fields above by reset()
    budget_at: [u8; N],
    // break path: false = Blackwood's rules (class-correct pieces only, no
    // mismatch on a ring-coloured side of the piece, none at the cells his
    // keyed lists never break, exact fits tried before breaks); true = the
    // port's original path, kept for results computed with it.
    legacy_break_path: bool,
    // hints: false = a pinned piece's privileged sides count once the scan
    // reaches its cell, which is a node with the quota test (Blackwood places
    // his centre piece in the scan, at depth 119); true = they count from the
    // start and the scan steps over the cell (the port's original handling).
    legacy_hints: bool,
    // quota test: false = Blackwood's, per candidate up to depth 160 (the
    // sides placed so far plus the candidate's must reach quota[depth]);
    // true = the port's, per node at every depth (the sides placed so far
    // must reach quota[depth] before any candidate is tried).
    legacy_quota_test: bool,
    // isolation: refuse a break against a cell that already carries one.
    // The port's own rule; Blackwood's engine has none, so off by default.
    isolation: bool,
    // ring[c]: colour c sits next to a grey side on some frame piece
    // (Blackwood's `side_edges`, derived from the pieces).
    ring: [bool; 256],
    // no_break_cell[pos]: Blackwood's keyed lists offer no mismatch here
    // (corners, and frame cells whose grey side is one of the two key sides
    // of the scan: his bottom row and left column).
    no_break_cell: [bool; N],

    // per-cell candidate lists, indexed by depth. For a given cell we need the
    // candidates (piece,rot) whose top/left match already-placed neighbors and
    // whose border-facing edges are border. But because breaks are allowed
    // late, we keep ALL orientations of ALL pieces available and filter/score
    // at placement time. For throughput we precompute, per (piece), its 4
    // RotatedPiece structs once.
    rotpieces: Vec<[RotatedPiece; 4]>,
    distinct_rots: Vec<u8>, // number of distinct rotations per piece (dedupe)
    // exact-match candidate table for the no-break hot path: for a cell whose
    // key sides (a, b) require colours (ca, cb), cand_by_key[ca*NC+cb] lists
    // every (piece_id, rot) showing exactly those colours there. NC = ncolors.
    ncolors: usize,
    cand_by_key: Vec<Vec<(u16, u8)>>,
    // Construction-order copy of cand_by_key, never mutated after Solver::new.
    // reshuffle_candidates restores from this so each restart's opening depends
    // only on its own seed (see reshuffle_candidates).
    cand_by_key_pristine: Vec<Vec<(u16, u8)>>,

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

    // diagnostics (never read by the search): nodes entered per depth, and
    // placements of a piece whose grey sides do not face exactly the frame
    // sides of its cell (a piece that cannot sit there on a physical board).
    hist: Vec<u64>,
    illegal_placed: u64,
}

const EMPTYP: u16 = u16::MAX;

impl Solver {
    #[allow(clippy::too_many_arguments)]
    fn new(
        edges: Vec<[u8; 4]>,
        privileged_colors: &[u8],
        scan: ScanOrder,
        hints: &[(usize, u16, u8)],
        use_quota: bool,
        use_breaks: bool,
        max_breaks: u8,
        node_cap: u64,
        quota_scale: f32,
        quota_base: QuotaBase,
    ) -> Self {
        let npieces = edges.len();
        let mut privileged = [false; 256];
        for &c in privileged_colors {
            privileged[c as usize] = true;
        }
        let quota = match quota_base {
            QuotaBase::Count => build_quota(quota_scale),
            QuotaBase::Share => {
                let total: u32 = edges
                    .iter()
                    .flat_map(|e| e.iter())
                    .filter(|&&c| privileged[c as usize])
                    .count() as u32;
                build_quota_share(quota_scale, f64::from(total))
            }
        };

        // border mask
        let mut border_mask = [[false; 4]; N];
        for pos in 0..N {
            let x = pos % W;
            let y = pos / W;
            border_mask[pos] = [y == 0, x == W - 1, y == W - 1, x == 0];
        }

        // ring colours: the non-grey sides adjacent to a grey side
        let mut ring = [false; 256];
        for e in &edges {
            for k in 0..4 {
                if e[k] == BORDER {
                    for c in [e[(k + 1) % 4], e[(k + 3) % 4]] {
                        if c != BORDER {
                            ring[c as usize] = true;
                        }
                    }
                }
            }
        }
        // cells where his lists never break: corners, and frame cells whose
        // grey side is a key side (for his scan, his bottom row and left
        // column; on the official set the class and ring rules already exclude
        // every break there, his lists make it structural)
        let order = scan.cells();
        let key_sides = scan.key_sides();
        let mut no_break_cell = [false; N];
        for (pos, nb) in no_break_cell.iter_mut().enumerate() {
            let bm = border_mask[pos];
            let corner = bm.iter().filter(|&&b| b).count() == 2;
            *nb = corner || bm[key_sides[0]] || bm[key_sides[1]];
        }

        // hints
        let mut hint_at: [Option<(u16, u8)>; N] = [None; N];
        for &(pos, pid, rot) in hints {
            hint_at[pos] = Some((pid, rot));
        }

        // every searched cell must find both key sides placed (earlier in the
        // scan, or pinned) or on the frame
        let mut depth_of = [0usize; N];
        for (d, &pos) in order.iter().enumerate() {
            depth_of[pos] = d;
        }
        for (d, &pos) in order.iter().enumerate() {
            for &k in &key_sides {
                if let Some(nb) = neighbour(pos, k) {
                    assert!(depth_of[nb] < d || hint_at[nb].is_some(), "{scan:?}: cell {pos} is searched before its key neighbour {nb}");
                }
            }
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
        // exact candidate table on the scan's two key sides
        let mut cand_by_key: Vec<Vec<(u16, u8)>> = vec![Vec::new(); ncolors * ncolors];
        for pid in 0..npieces {
            let ndr = distinct_rots[pid] as usize;
            for r in 0..ndr {
                let rp = rotpieces[pid][r];
                let key = (side(&rp, key_sides[0]) as usize) * ncolors + (side(&rp, key_sides[1]) as usize);
                cand_by_key[key].push((pid as u16, r as u8));
            }
        }
        // value-order each list: privileged (heuristic_count) to the front so
        // the no-break hot path needs no per-node sort.
        for list in cand_by_key.iter_mut() {
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
            key_sides,
            other_sides: {
                let mut o = (0..4).filter(|k| !key_sides.contains(k));
                [o.next().expect("two other sides"), o.next().expect("two other sides")]
            },
            border_mask,
            hint_at,
            use_quota,
            use_breaks,
            max_breaks,
            unlock: BREAK_UNLOCK.to_vec(),
            budget_at: [0; N],
            legacy_break_path: false,
            legacy_hints: false,
            legacy_quota_test: false,
            isolation: false,
            ring,
            no_break_cell,
            rotpieces,
            distinct_rots,
            ncolors,
            cand_by_key_pristine: cand_by_key.clone(),
            cand_by_key,
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
            hist: vec![0; N + 1],
            illegal_placed: 0,
        }
    }

    #[inline]
    fn breaks_allowed_at(&self, depth: usize) -> u8 {
        self.unlock.iter().filter(|&&d| d <= depth).count() as u8
    }

    /// Fill `budget_at` (the break budget per depth) from `unlock`,
    /// `max_breaks` and `use_breaks`, so the hot path reads one byte.
    fn fill_budget(&mut self) {
        for d in 0..N {
            self.budget_at[d] = if self.use_breaks { self.breaks_allowed_at(d).min(self.max_breaks) } else { 0 };
        }
    }

    /// Blackwood's quota test (`Program.cs` 141 to 145) for a candidate with
    /// `hc` privileged sides at `depth`: true when it falls short and, the
    /// candidates being privileged-first, ends the loop.
    #[inline]
    fn short_of_quota(&self, depth: usize, hc: u8) -> bool {
        self.use_quota
            && !self.legacy_quota_test
            && depth <= MAX_HEURISTIC_INDEX
            && self.priv_on_board + u16::from(hc) < self.quota[depth]
    }

    /// The port's original quota test, per node: true when the sides already
    /// placed fall short of quota[depth].
    #[inline]
    fn short_of_quota_at_node(&self, depth: usize) -> bool {
        self.use_quota && self.legacy_quota_test && self.priv_on_board < self.quota[depth]
    }

    /// Blackwood's rule (`Util.cs` `Get_Rotated_Pieces`, `side_breaks`): a
    /// mismatch is refused when the piece's own side there is a ring colour.
    #[inline]
    fn breaks_on_ring(&self, rp: &RotatedPiece, broke: &[Option<usize>; 4]) -> bool {
        let sides = [rp.top, rp.right, rp.bottom, rp.left];
        (0..4).any(|k| broke[k].is_some() && self.ring[sides[k] as usize])
    }

    /// Candidates for `pos` on the break path, in the order they are tried:
    /// (piece, rotation, breaks it adds, broken side to neighbour cell).
    fn break_candidates(
        &self,
        pos: usize,
        budget: u8,
        break_count_at: &[u8; N],
    ) -> Vec<(u16, u8, u8, [Option<usize>; 4])> {
        let legal = !self.legacy_break_path;
        let mut cands: Vec<(u16, u8, u8, [Option<usize>; 4])> = Vec::with_capacity(64);
        for pid in 0..self.npieces {
            if self.used[pid] {
                continue;
            }
            let ndr = self.distinct_rots[pid] as usize;
            for r in 0..ndr {
                let rp = self.rotpieces[pid][r];
                if legal && self.wrong_class(pos, &rp) {
                    continue;
                }
                let Some((nb, cells)) = self.eval_candidate(pos, &rp, break_count_at) else {
                    continue;
                };
                if self.breaks_used + nb > budget {
                    continue;
                }
                if legal && nb > 0 && (self.no_break_cell[pos] || self.breaks_on_ring(&rp, &cells)) {
                    continue;
                }
                cands.push((pid as u16, r as u8, nb, cells));
            }
        }
        let hc = |c: &(u16, u8, u8, [Option<usize>; 4])| self.rotpieces[c.0 as usize][c.1 as usize].heuristic_count;
        if legal {
            // his Score = 100 * heuristic count - 100000 * breaks: every exact
            // fit first, privileged-first within each group
            cands.sort_by(|a, b| a.2.cmp(&b.2).then(hc(b).cmp(&hc(a))));
        } else {
            // the port's order: privileged desc, then fewer breaks
            cands.sort_by(|a, b| hc(b).cmp(&hc(a)).then(a.2.cmp(&b.2)));
        }
        cands
    }

    /// True when the oriented piece shows grey toward a cell side that is not
    /// on the frame, or a colour toward a side that is: it cannot sit at `pos`
    /// on a physical board, whatever its neighbours are.
    #[inline]
    fn wrong_class(&self, pos: usize, rp: &RotatedPiece) -> bool {
        let bm = self.border_mask[pos];
        (rp.top == BORDER) != bm[0]
            || (rp.right == BORDER) != bm[1]
            || (rp.bottom == BORDER) != bm[2]
            || (rp.left == BORDER) != bm[3]
    }

    // Per-attempt randomization: reshuffle each exact-match candidate list so
    // that ties in heuristic_count are broken differently each restart. This is
    // the "randomize the opening" lever expressed through value-order: a
    // different first legal piece at the early cells sends the DFS into a
    // different basin. Privileged-first grouping is preserved (stable within a
    // heuristic_count block; only intra-block order is shuffled).
    /// Re-derive this attempt's candidate order from `cand_by_key_pristine` (the
    /// construction-order table), NOT from whatever the previous attempt left
    /// behind.
    ///
    /// This used to shuffle `cand_by_key` in place, cumulatively -- the
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
        self.cand_by_key.clone_from(&self.cand_by_key_pristine);
        for list in self.cand_by_key.iter_mut() {
            // Fisher-Yates within the whole list, then stable-sort by
            // heuristic_count desc restores the privileged grouping while
            // keeping the shuffled intra-group order.
            rng.shuffle(list);
        }
        let rotpieces = &self.rotpieces;
        for list in self.cand_by_key.iter_mut() {
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
        self.fill_budget();
        // pin hints
        for pos in 0..N {
            if let Some((pid, rot)) = self.hint_at[pos] {
                self.board_piece[pos] = pid;
                self.board_rot[pos] = rot;
                self.used[pid as usize] = true;
                if self.legacy_hints {
                    let rp = self.rotpieces[pid as usize][rot as usize];
                    self.priv_on_board += u16::from(rp.heuristic_count);
                }
            }
        }
    }

    /// Colour a piece at `pos` must show on side `k`: the facing side of the
    /// placed neighbour, or grey on the frame. Only called for key sides,
    /// which Solver::new guarantees are placed or on the frame.
    #[inline]
    fn required(&self, pos: usize, k: usize) -> u8 {
        neighbour(pos, k).map_or(BORDER, |nb| side(&self.oriented(nb), (k + 2) % 4))
    }

    /// Hot-path constraint on side `k` of `pos`: Some(c) = must show exactly
    /// c (a placed neighbour's facing colour, or grey on the frame); None =
    /// must not show grey (an empty interior neighbour).
    #[inline(always)]
    fn want_on(&self, pos: usize, k: usize, bm: [bool; 4]) -> Option<u8> {
        if bm[k] {
            return Some(BORDER);
        }
        let nb = match k {
            0 => pos - W,
            1 => pos + 1,
            2 => pos + W,
            _ => pos - 1,
        };
        if self.board_piece[nb] == EMPTYP {
            None
        } else {
            Some(side(&self.oriented(nb), (k + 2) % 4))
        }
    }

    /// Every side of `rp` that faces a placed neighbour matches it.
    #[inline]
    fn fits_placed(&self, pos: usize, rp: &RotatedPiece) -> bool {
        (0..4).all(|k| match neighbour(pos, k) {
            Some(nb) if self.board_piece[nb] != EMPTYP => side(rp, k) == side(&self.oriented(nb), (k + 2) % 4),
            _ => true,
        })
    }

    #[inline]
    fn oriented(&self, pos: usize) -> RotatedPiece {
        let pid = self.board_piece[pos];
        let rot = self.board_rot[pos];
        self.rotpieces[pid as usize][rot as usize]
    }

    // Count the breaks a candidate placement at `pos` would create against
    // its placed neighbours and against the frame. Returns None when the piece
    // would carry two or more (Blackwood admits at most one per piece) or,
    // with `isolation` on, when it would break against a neighbour cell that
    // already carries a break (the port's own non-adjacency rule; his engine
    // has none). The budget is checked by the caller. Also returns, per side,
    // the broken neighbour cell (usize::MAX for a frame side) so the caller can
    // keep break_count_at[cell].
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
        if !self.isolation {
            return Some((new_breaks, broke_side_cell));
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
            // completed board: score = 480 - breaks_used
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

        // a pinned cell: placed in reset(); see `legacy_hints`
        if let Some((hpid, hrot)) = self.hint_at[pos] {
            if self.legacy_hints {
                self.search(depth + 1, break_count_at);
                return;
            }
            self.nodes += 1;
            self.hist[depth] += 1;
            let hc = self.rotpieces[hpid as usize][hrot as usize].heuristic_count;
            if self.short_of_quota_at_node(depth) || self.short_of_quota(depth, hc) {
                return;
            }
            self.priv_on_board += u16::from(hc);
            self.search(depth + 1, break_count_at);
            self.priv_on_board -= u16::from(hc);
            return;
        }

        self.nodes += 1;
        self.hist[depth] += 1;

        if self.short_of_quota_at_node(depth) {
            return;
        }

        // current break budget at this depth
        let budget = self.budget_at[depth];

        if budget == 0 {
            // FAST PATH: no breaks allowed. The scan's two key sides are always
            // determined (a placed neighbour's colour, or grey on the frame);
            // cand_by_key lists the exact matches, pre-sorted privileged-first.
            let [ka, kb] = self.key_sides;
            let (ca, cb) = (self.required(pos, ka), self.required(pos, kb));
            let bm = self.border_mask[pos];
            // A key side off the frame showing grey means the neighbour there
            // is itself wrong-class (only the legacy break path places one):
            // no class-correct piece fits.
            if (!bm[ka] && ca == BORDER) || (!bm[kb] && cb == BORDER) {
                return;
            }
            let key = (ca as usize) * self.ncolors + (cb as usize);
            // The lookup only tests the key sides, so a piece could show grey
            // toward an empty interior cell (or a corner piece could take an
            // edge cell). Blackwood's per-class lists never offer such a
            // piece. Per other side, computed once per node: Some(c) = must
            // show exactly c (a placed neighbour, or grey on the frame);
            // None = must not show grey. This is wrong_class plus fits_placed,
            // specialised for the hot path.
            // (plain code, no iterator adapters: they were not inlined here)
            let [oa, ob] = self.other_sides;
            let wa = self.want_on(pos, oa, bm);
            let wb = self.want_on(pos, ob, bm);
            if (!bm[oa] && wa == Some(BORDER)) || (!bm[ob] && wb == Some(BORDER)) {
                return;
            }
            // index by position each iteration: the list cannot stay borrowed
            // across the recursive call
            let list_len = self.cand_by_key[key].len();
            for idx in 0..list_len {
                if self.nodes >= self.node_cap {
                    return;
                }
                let (pid16, r) = self.cand_by_key[key][idx];
                let pid = pid16 as usize;
                if self.used[pid] {
                    continue;
                }
                let rp = self.rotpieces[pid][r as usize];
                if !fits_want(wa, side(&rp, oa)) || !fits_want(wb, side(&rp, ob)) {
                    continue;
                }
                debug_assert!(!self.wrong_class(pos, &rp) && self.fits_placed(pos, &rp));
                // the list is privileged-first, so no later candidate passes
                if self.short_of_quota(depth, rp.heuristic_count) {
                    break;
                }
                self.board_piece[pos] = pid16;
                self.board_rot[pos] = r;
                self.used[pid] = true;
                self.priv_on_board += u16::from(rp.heuristic_count);
                // illegal_placed: never on this path (the checks above)
                self.search(depth + 1, break_count_at);
                self.priv_on_board -= u16::from(rp.heuristic_count);
                self.used[pid] = false;
                self.board_piece[pos] = EMPTYP;
            }
            return;
        }

        self.search_breaks(depth, pos, budget, break_count_at);
    }

    /// BREAK PATH (depth >= first unlock): full scan with break accounting.
    /// Candidates go into a heap buffer (a large per-frame stack array is
    /// multiplied across all 256 recursion frames and overflows the stack);
    /// the path fires only late, so the allocation is rare and cheap. Kept out
    /// of `search` so the hot path's frame and code stay small.
    #[inline(never)]
    fn search_breaks(&mut self, depth: usize, pos: usize, budget: u8, break_count_at: &mut [u8; N]) {
        let cands = self.break_candidates(pos, budget, break_count_at);

        for i in 0..cands.len() {
            if self.nodes >= self.node_cap {
                return;
            }
            let (pid16, r, nb, cells) = cands[i];
            let pid = pid16 as usize;
            let rp = self.rotpieces[pid][r as usize];
            // as his loop does (a break era before depth 160 only)
            if self.short_of_quota(depth, rp.heuristic_count) {
                break;
            }
            self.board_piece[pos] = pid16;
            self.board_rot[pos] = r;
            self.used[pid] = true;
            self.priv_on_board += u16::from(rp.heuristic_count);
            self.illegal_placed += u64::from(self.wrong_class(pos, &rp));
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
    // neighbors (schedule ignored; this is the fallback that turns any deep
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
            // physicality on that cell (an interior piece on a frame cell); the
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
    hist: Vec<u64>,
    illegal_placed: u64,
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
    solver.hist.iter_mut().for_each(|h| *h = 0);
    solver.illegal_placed = 0;

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

    AttemptOutcome {
        score,
        board,
        physical,
        native,
        deepest: solver.deepest,
        nodes: solver.nodes,
        hist: solver.hist.clone(),
        illegal_placed: solver.illegal_placed,
    }
}

// Rebuild a Board from the packed piece/rotation arrays.
fn board_from_packed(puzzle: &eternity2_core::Puzzle, bp: &[u16; N], br: &[u8; N]) -> Board {
    let mut b = Board::empty(puzzle);
    for pos in 0..N {
        if bp[pos] != EMPTYP {
            b.place(pos as u32, bp[pos], Rotation::from_u8(br[pos]).unwrap());
        }
    }
    b
}

// Canonical board document (one .json with an eternity2.dev viewer URL).
fn board_doc(puzzle: &eternity2_core::Puzzle, bp: &[u16; N], br: &[u8; N], score: u32) -> BoardDoc {
    board_to_doc(puzzle, &board_from_packed(puzzle, bp, br), "size_16_official_eternity", score)
}

// ---------------------------------------------------------------------------
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!(
            "usage: blackwood_bt <puzzle.csv> [--stage 1|2|3|4] [--keying published|legacy-port] [--priv c1,c2,c3] [--quota-count] \
             [--maxbreaks K] [--nodecap N] [--restarts R] [--seed S] \
             [--threads T] [--order published|row-major] [--hist] [--legacy-break-path] [--isolation] [--legacy-hints] [--legacy-quota-test] [--emit-dir DIR] [--emit-prefix P]\n\
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
    // Privileged colours and quota reading: see resolve_keying.
    let mut keying: Option<String> = None;
    let mut priv_arg: Option<Vec<u8>> = None;
    let mut quota_count = false;
    let mut max_breaks = 10u8;
    let mut node_cap = 200_000_000u64;
    let mut restarts = 1usize;
    let mut seed = 0xE2E2u64;
    let mut scan = ScanOrder::default();
    let mut verbose = false;
    let mut quota_scale = 1.0f32;
    let mut emit_dir: Option<String> = None;
    let mut emit_prefix = "bw".to_string();
    let mut threads: usize = 1; // 1 = serial restart portfolio (the default)
    let mut print_hist = false;
    let mut legacy_break_path = false;
    let mut isolation = false;
    let mut legacy_hints = false;
    let mut legacy_quota_test = false;

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--stage" => {
                stage = args[i + 1].parse().unwrap();
                i += 2;
            }
            "--priv" => {
                priv_arg = Some(args[i + 1].split(',').map(|s| s.trim().parse().unwrap()).collect());
                i += 2;
            }
            "--keying" => {
                keying = Some(args[i + 1].clone());
                i += 2;
            }
            "--quota-count" => {
                quota_count = true;
                i += 1;
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
            "--order" => {
                scan = ScanOrder::parse(&args[i + 1]).unwrap_or_else(|e| {
                    eprintln!("{e}");
                    std::process::exit(2);
                });
                i += 2;
            }
            "--verbose" => {
                verbose = true;
                i += 1;
            }
            "--hist" => {
                print_hist = true;
                i += 1;
            }
            "--legacy-break-path" => {
                legacy_break_path = true;
                i += 1;
            }
            "--isolation" => {
                isolation = true;
                i += 1;
            }
            "--legacy-hints" => {
                legacy_hints = true;
                i += 1;
            }
            "--legacy-quota-test" => {
                legacy_quota_test = true;
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

    let (priv_colors, quota_base) = resolve_keying(keying.as_deref(), priv_arg, quota_count).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(2);
    });

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
    // plus the shuffled restart portfolio.


    let mut solver = Solver::new(
        edges,
        &priv_colors,
        scan,
        &hints,
        use_quota,
        use_breaks,
        max_breaks,
        node_cap,
        quota_scale,
        quota_base,
    );
    solver.verbose = verbose;
    solver.legacy_break_path = legacy_break_path;
    solver.isolation = isolation;
    solver.legacy_hints = legacy_hints;
    solver.legacy_quota_test = legacy_quota_test;

    if threads == 0 {
        threads = std::thread::available_parallelism().map_or(1, std::num::NonZero::get);
    }
    // Never spin up more workers than there are restarts to run.
    threads = threads.min(restarts.max(1));

    eprintln!(
        "# blackwood_bt stage={stage} priv={priv_colors:?} quota={quota_base:?} maxbreaks={max_breaks} \
         nodecap={node_cap} restarts={restarts} seed={seed} threads={threads} order={scan:?} \
         legacy_break_path={legacy_break_path} isolation={isolation} legacy_hints={legacy_hints} \
         legacy_quota_test={legacy_quota_test}"
    );
    if threads > 1 {
        eprintln!("# --threads {threads}: restarts run concurrently; per-restart results are unchanged (each depends only on its own seed), but \"new best\" log order reflects completion order.");
    }

    // global best is over COMPLETE boards (native completion OR greedy-tail
    // completion of the deepest partial). This makes every restart yield a
    // scorable full board, so the marathon always banks something.
    let t0 = Instant::now();

    let emit_dir_s = emit_dir.clone().unwrap_or_else(|| ".".into());
    std::fs::create_dir_all(&emit_dir_s).ok();
    let checkpoint_path = Path::new(&emit_dir_s).join("best_so_far.json");
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
        node_hist: Vec<u64>,
        illegal_placed: u64,
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
            for (a, b) in self.node_hist.iter_mut().zip(&o.hist) {
                *a += b;
            }
            self.illegal_placed += o.illegal_placed;
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
                    let doc = board_doc(self.puzzle, bp, br, u32::from(self.global_best));
                    let url = doc.url.clone();
                    doc.write_json(&self.checkpoint_path).ok();
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
                            "RECORD_{}_{}_{ts}.json",
                            self.global_best, self.emit_prefix
                        ));
                        doc.write_json(&rec).ok();
                        eprintln!("# ***** NEW FROM-SCRATCH RECORD CANDIDATE score={} native={} *****", self.global_best, o.native);
                        eprintln!("# ***** banked -> {}, VERIFY 3 WAYS *****", rec.display());
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
        node_hist: vec![0u64; N + 1],
        illegal_placed: 0,
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
    let (node_hist, illegal_placed) = (p.node_hist, p.illegal_placed);

    let elapsed = t0.elapsed().as_secs_f64();
    let mnps = (total_nodes as f64 / 1e6) / elapsed.max(1e-9);

    eprintln!("# ===== SUMMARY =====");
    eprintln!("# stage={stage} restarts={restarts} elapsed={elapsed:.2}s nodes={total_nodes} throughput={mnps:.1} Mnode/s restarts/s={:.2}", restarts as f64 / elapsed.max(1e-9));
    eprintln!("# best_complete_score={global_best} best_native_score={best_natural} deepest_depth={deepest_overall} illegal_placements={illegal_placed}");
    if print_hist {
        // nodes entered per depth, summed over restarts (one line, depth:count)
        let cells: Vec<String> = node_hist
            .iter()
            .enumerate()
            .filter(|(_, c)| **c > 0)
            .map(|(d, c)| format!("{d}:{c}"))
            .collect();
        eprintln!("# node-hist {}", cells.join(" "));
    }
    eprint!("# complete-score histogram (score:count): ");
    for (s, c) in hist.iter().enumerate() {
        if *c > 0 {
            eprint!("{s}:{c} ");
        }
    }
    eprintln!();

    // emit best board (timestamped, never overwrites)
    if let Some((bp, br)) = &global_best_board {
        let doc = board_doc(&puzzle, bp, br, u32::from(global_best));
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let path = Path::new(&emit_dir_s).join(format!(
            "{emit_prefix}_stage{stage}_score{global_best}_{ts}.json"
        ));
        doc.write_json(&path).expect("write board json");
        eprintln!("# emitted best board -> {}", path.display());
        println!("{}", path.display());
    }

    println!("BEST {global_best}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn official() -> (Vec<[u8; 4]>, Vec<(usize, u16, u8)>) {
        let csv = concat!(env!("CARGO_MANIFEST_DIR"), "/../../data/puzzles/size_16_official_eternity.csv");
        let (puzzle, hints) = load_puzzle_with_hints(Path::new(csv)).expect("official puzzle");
        let mut edges = vec![[0u8; 4]; puzzle.pieces().len()];
        for p in puzzle.pieces() {
            edges[p.id as usize] = p.edges.as_array();
        }
        let hints = hints
            .hints
            .iter()
            .map(|h| (h.position as usize, h.piece_id, h.rotation.as_u8()))
            .collect();
        (edges, hints)
    }

    fn solver(edges: &[[u8; 4]], privileged: &[u8], hints: &[(usize, u16, u8)], node_cap: u64) -> Solver {
        // the port's original scan, quota reading and quota test, as in the
        // Track 13 runs
        let mut s = Solver::new(edges.to_vec(), privileged, ScanOrder::RowMajor, hints, true, true, 10, node_cap, 1.0, QuotaBase::Count);
        s.legacy_hints = true;
        s.legacy_quota_test = true;
        s
    }

    /// A board state around one cell: `pos`'s top neighbour holds `top`, its
    /// left neighbour `left` (piece, rotation); both marked used.
    fn with_neighbours(s: &mut Solver, pos: usize, top: (u16, u8), left: (u16, u8)) {
        for (p, (pid, r)) in [(pos - W, top), (pos - 1, left)] {
            s.board_piece[p] = pid;
            s.board_rot[p] = r;
            s.used[pid as usize] = true;
        }
    }

    /// First (piece, rotation) whose oriented sides satisfy `want`.
    fn find(s: &Solver, want: impl Fn(&RotatedPiece) -> bool) -> (u16, u8) {
        for pid in 0..s.npieces {
            for r in 0..4u8 {
                if want(&s.rotpieces[pid][r as usize]) {
                    return (pid as u16, r);
                }
            }
        }
        panic!("no such piece")
    }

    fn interior(rp: &RotatedPiece) -> bool {
        [rp.top, rp.right, rp.bottom, rp.left].iter().all(|&c| c != BORDER)
    }

    /// His Score (`Util.cs` 94, sorted in `Program.cs` 203 to 227) puts every
    /// exact fit before any break, even a break candidate with more
    /// privileged sides. Built at a cell where the two orders disagree.
    #[test]
    fn exact_fits_come_before_more_privileged_breaks() {
        let (edges, _) = official();
        let pos = 5 * W + (W - 1);
        let setup = |privileged: &[u8], left: (u16, u8)| {
            let mut s = solver(&edges, privileged, &[], 1);
            let top = find(&s, |rp| rp.right == BORDER && rp.top != BORDER && rp.bottom != BORDER && rp.left != BORDER);
            with_neighbours(&mut s, pos, top, left);
            s
        };
        // find a left neighbour and a colour such that some break candidate
        // shows the colour and some exact fit does not
        let zero = [0u8; N];
        let probe0 = setup(&[], (0, 0));
        let lefts: Vec<(u16, u8)> = (0..probe0.npieces as u16)
            .flat_map(|p| (0..4u8).map(move |r| (p, r)))
            .filter(|&(p, r)| interior(&probe0.rotpieces[p as usize][r as usize]))
            .collect();
        let (left, colour) = lefts
            .iter()
            .find_map(|&left| {
                let probe = setup(&[], left);
                let cands = probe.break_candidates(pos, 1, &zero);
                let sides = |c: &(u16, u8, u8, [Option<usize>; 4])| {
                    let rp = probe.rotpieces[c.0 as usize][c.1 as usize];
                    [rp.top, rp.right, rp.bottom, rp.left]
                };
                let colour = cands
                    .iter()
                    .filter(|b| b.2 == 1)
                    .flat_map(|b| sides(b))
                    .find(|&c| c != BORDER && cands.iter().any(|e| e.2 == 0 && !sides(e).contains(&c)));
                colour.map(|c| (left, c))
            })
            .expect("a cell where the two orders disagree");
        let mut s = setup(&[colour], left);
        let hc = |s: &Solver, c: &(u16, u8, u8, [Option<usize>; 4])| s.rotpieces[c.0 as usize][c.1 as usize].heuristic_count;
        let cands = s.break_candidates(pos, 1, &zero);
        let best_break = cands.iter().filter(|c| c.2 == 1).map(|c| hc(&s, c)).max().expect("a break");
        let worst_exact = cands.iter().filter(|c| c.2 == 0).map(|c| hc(&s, c)).min().expect("an exact fit");
        assert!(best_break > worst_exact, "the orders disagree here");
        let first_break = cands.iter().position(|c| c.2 == 1).expect("a break");
        assert!(cands[first_break..].iter().all(|c| c.2 == 1), "every exact fit comes first");
        // the port's old order (privileged first) puts some break before
        // some exact fit here
        s.legacy_break_path = true;
        let legacy = s.break_candidates(pos, 1, &zero);
        let first_break = legacy.iter().position(|c| c.2 == 1).expect("a break");
        assert!(legacy[first_break..].iter().any(|c| c.2 == 0), "legacy interleaves");
    }

    #[test]
    fn legal_break_path_offers_only_physical_pieces_and_his_breaks() {
        let (edges, _) = official();
        let mut s = solver(&edges, &[22, 21, 20], &[], 1);
        // a right-column cell (grey on its right): above it an edge piece
        // with grey on the right, to its left an interior piece
        let pos = 5 * W + (W - 1);
        let top = find(&s, |rp| rp.right == BORDER && rp.top != BORDER && rp.bottom != BORDER && rp.left != BORDER);
        let left = find(&s, interior);
        with_neighbours(&mut s, pos, top, left);
        let zero = [0u8; N];
        let cands = s.break_candidates(pos, 1, &zero);
        assert!(cands.iter().any(|c| c.2 == 1), "some break is admitted here");
        for (i, &(pid, r, nb, cells)) in cands.iter().enumerate() {
            let rp = s.rotpieces[pid as usize][r as usize];
            assert!(!s.wrong_class(pos, &rp), "wrong-class candidate {pid}/{r}");
            assert!(nb == 0 || !s.breaks_on_ring(&rp, &cells), "break on a ring side {pid}/{r}");
            assert!(i == 0 || cands[i - 1].2 <= nb, "exact fits come before breaks");
        }
        // at an interior cell between interior pieces the port's original
        // path offers frame pieces showing grey toward an empty cell
        let mut s = solver(&edges, &[22, 21, 20], &[], 1);
        let pos = 5 * W + 5;
        let top = find(&s, interior);
        let left = find(&s, |rp| interior(rp) && rp.right != s.rotpieces[top.0 as usize][top.1 as usize].right);
        with_neighbours(&mut s, pos, top, left);
        let wrong = |s: &Solver, c: &(u16, u8, u8, [Option<usize>; 4])| s.wrong_class(pos, &s.rotpieces[c.0 as usize][c.1 as usize]);
        assert!(!s.break_candidates(pos, 1, &zero).iter().any(|c| wrong(&s, c)));
        s.legacy_break_path = true;
        assert!(s.break_candidates(pos, 1, &zero).iter().any(|c| wrong(&s, c)));
    }

    /// Blackwood's engine lets a break sit next to another; the port's
    /// isolation rule (now `--isolation`) refused it.
    #[test]
    fn a_break_may_touch_a_cell_that_already_carries_one() {
        let (edges, _) = official();
        let mut s = solver(&edges, &[22, 21, 20], &[], 1);
        let pos = 5 * W + (W - 1);
        let top = find(&s, |rp| rp.right == BORDER && rp.top != BORDER && rp.bottom != BORDER && rp.left != BORDER);
        let left = find(&s, interior);
        with_neighbours(&mut s, pos, top, left);
        let mut bca = [0u8; N];
        bca[pos - 1] = 1; // the left neighbour already carries a break
        let toward_left = |c: &(u16, u8, u8, [Option<usize>; 4])| c.3[3] == Some(pos - 1);
        assert!(s.break_candidates(pos, 2, &bca).iter().any(toward_left));
        s.isolation = true;
        assert!(!s.break_candidates(pos, 2, &bca).iter().any(toward_left));
    }

    #[test]
    fn legal_break_path_never_breaks_in_the_scan_first_column_or_at_corners() {
        let (edges, _) = official();
        for pos in [5 * W, (W - 1) * W, (W - 1) * W + (W - 1)] {
            let mut s = solver(&edges, &[22, 21, 20], &[], 1);
            let top = find(&s, |rp| rp.left == BORDER && rp.top != BORDER && rp.right != BORDER);
            let left_cell_piece = if pos % W == 0 {
                None
            } else {
                Some(find(&s, |rp| rp.bottom == BORDER && rp.left != BORDER && rp.right != BORDER))
            };
            // top neighbour always; left neighbour only off the first column
            s.board_piece[pos - W] = top.0;
            s.board_rot[pos - W] = top.1;
            s.used[top.0 as usize] = true;
            if let Some((pid, r)) = left_cell_piece {
                s.board_piece[pos - 1] = pid;
                s.board_rot[pos - 1] = r;
                s.used[pid as usize] = true;
            }
            assert!(s.no_break_cell[pos], "cell {pos}");
            let zero = [0u8; N];
            assert!(s.break_candidates(pos, 1, &zero).iter().all(|c| c.2 == 0), "cell {pos}");
            s.legacy_break_path = true;
            assert!(s.break_candidates(pos, 1, &zero).iter().any(|c| c.2 == 1), "legacy, cell {pos}");
        }
    }

    /// End to end with breaks from depth 30 on: the legal path places only
    /// class-correct pieces, the port's original path does not.
    #[test]
    fn early_breaks_place_only_class_correct_pieces() {
        let (edges, _) = official();
        let early: Vec<usize> = (0..10).map(|k| 30 + 5 * k).collect();
        for (legacy, expect_illegal) in [(false, false), (true, true)] {
            let mut s = solver(&edges, &[22, 21, 20], &[], 300_000);
            s.unlock.clone_from(&early);
            if legacy {
                s.legacy_break_path = true;
            }
            let o = run_attempt_scored(&mut s, 1, 0, true);
            assert!(o.hist[35] > 0, "the run reaches the break era");
            assert_eq!(o.illegal_placed > 0, expect_illegal, "legacy={legacy}: {} illegal", o.illegal_placed);
        }
    }

    #[test]
    fn default_keying_is_blackwoods_set_read_as_a_share() {
        assert_eq!(resolve_keying(None, None, false), Ok((vec![5, 20, 7], QuotaBase::Share)));
        assert_eq!(resolve_keying(Some("legacy-port"), None, false), Ok((vec![22, 21, 20], QuotaBase::Count)));
        assert_eq!(resolve_keying(None, Some(vec![3, 12, 13]), true), Ok((vec![3, 12, 13], QuotaBase::Count)));
        assert!(resolve_keying(Some("blackwood"), None, false).is_err());
    }

    /// His `heuristic_array`, depths 0 to 160 (`Program.cs` 310 to 325 at
    /// f544e42, evaluated outside this port; f32 and f64 give the same integers).
    const HIS_HEURISTIC_ARRAY: [u16; 161] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 5, 8, 11, 14, 16, 19, 22, 25, 28,
        29, 30, 32, 33, 35, 36, 38, 39, 40, 42, 43, 45, 46, 48, 49, 50, 52, 53, 55, 56, 58, 59,
        60, 62, 63, 65, 66, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 80, 81, 82, 83,
        84, 85, 86, 87, 88, 89, 89, 90, 90, 91, 92, 92, 93, 94, 94, 95, 96, 96, 97, 98, 98, 99,
        100, 100, 101, 102, 102, 103, 104, 104, 105, 105, 106, 106, 106, 106, 107, 107, 107,
        107, 108, 108, 108, 108, 108, 109, 109, 109, 109, 110, 110, 110, 110, 110, 111, 111,
        111, 111, 112, 112, 112, 112, 112, 113, 113, 113, 113, 114, 114, 114, 114, 114, 115,
        115, 115, 115, 116, 116, 116, 116, 116, 117, 117, 117, 117, 118, 118, 118, 118, 119,
    ];

    /// The share reading for 150 sides: the truncation of the unrounded curve
    /// times 150 / 122, with the curve taken before his integer cast, evaluated
    /// outside this port (the port's own design, not his). It is not
    /// floor(his integer array * 150 / 122), which differs at depth 17 (3
    /// against 2) and others.
    const SHARE_150: [u16; 161] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 6, 10, 13, 17, 20, 24, 27, 30, 34,
        36, 37, 39, 41, 43, 44, 46, 48, 50, 52, 53, 55, 57, 59, 60, 62, 64, 66, 67, 69, 71, 73,
        74, 76, 78, 80, 82, 83, 85, 87, 88, 89, 90, 91, 92, 93, 95, 96, 97, 98, 99, 100, 101,
        102, 103, 105, 106, 107, 108, 109, 110, 111, 111, 112, 113, 114, 115, 115, 116, 117,
        118, 119, 119, 120, 121, 122, 123, 123, 124, 125, 126, 127, 127, 128, 129, 130, 130,
        130, 131, 131, 131, 131, 132, 132, 132, 133, 133, 133, 133, 134, 134, 134, 135, 135,
        135, 135, 136, 136, 136, 136, 137, 137, 137, 138, 138, 138, 138, 139, 139, 139, 139,
        140, 140, 140, 141, 141, 141, 141, 142, 142, 142, 143, 143, 143, 143, 144, 144, 144,
        144, 145, 145, 145, 146, 146,
    ];

    #[test]
    fn share_quota_is_his_count_for_his_keying_and_scales_with_the_sides() {
        let (edges, _) = official();
        let mk = |p: &[u8], base| Solver::new(edges.clone(), p, ScanOrder::RowMajor, &[], true, true, 10, 1, 1.0, base);
        let his = HIS_HEURISTIC_ARRAY;
        let published = mk(&PUBLISHED_KEYING, QuotaBase::Share);
        assert_eq!(&published.quota[..=160], &his[..], "published keying, share");
        assert_eq!(published.quota[160], 119);
        let count = mk(&PUBLISHED_KEYING, QuotaBase::Count);
        assert_eq!(&count.quota[..=160], &his[..], "published keying, count");
        // (22, 21, 20) carries 150 sides: the share asks 150 / 122 of the count
        let legacy_share = mk(&LEGACY_PORT_KEYING, QuotaBase::Share);
        let legacy_count = mk(&LEGACY_PORT_KEYING, QuotaBase::Count);
        // the old port read the same curve as a count, whatever the keying
        assert_eq!(&legacy_count.quota[..=160], &his[..], "legacy-port, count");
        assert!(legacy_count.quota[161..].iter().all(|&q| q == 119), "held past 160");
        assert_eq!(&legacy_share.quota[..=160], &SHARE_150[..], "share of 150 sides");
    }

    #[test]
    fn published_order_is_his_board_order_in_this_frame() {
        let order = ScanOrder::Published.cells();
        let mut sorted = order.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, (0..N).collect::<Vec<_>>(), "a permutation");
        let at = |x: usize, y: usize| y * W + x;
        // his row r is y = 15 - r; spot checks against Get_Board_Order
        assert_eq!(order[0], at(0, 15), "starts at the bottom-left corner");
        assert_eq!(order[15], at(15, 15));
        assert_eq!(order[16], at(0, 14));
        assert_eq!(order[119], at(7, 8), "his start piece cell, this repo's centre clue");
        assert_eq!(order[175], at(15, 5));
        assert_eq!(order[176], at(0, 4), "the five-row band starts");
        assert_eq!(order[200], at(4, 0));
        assert_eq!(order[201], at(5, 4), "columns from depth 201");
        assert_eq!(order[205], at(5, 0));
        assert_eq!(order[236], at(12, 4));
        assert_eq!(order[243], at(12, 0));
        assert_eq!(order[255], at(15, 0), "ends at the top-right corner");
        assert_eq!(ScanOrder::Published.key_sides(), [3, 2], "matched on left and bottom");
        assert_eq!(ScanOrder::parse("published"), Ok(ScanOrder::Published));
        assert_eq!(ScanOrder::default(), ScanOrder::Published);
    }

    #[test]
    fn published_order_searches_class_correct_boards_with_and_without_clues() {
        let (edges, hints) = official();
        // his engine's setting (the centre clue only) and no clue
        let centre: Vec<_> = hints.iter().copied().filter(|h| h.0 == 8 * W + 7).collect();
        assert_eq!(centre.len(), 1);
        for clues in [&centre[..], &[][..]] {
            let mut s = Solver::new(edges.clone(), &PUBLISHED_KEYING, ScanOrder::Published, clues, true, true, 10, 300_000, 1.0, QuotaBase::Share);
            let o = run_attempt_scored(&mut s, 3, 0, true);
            assert_eq!(o.illegal_placed, 0);
            assert!(o.deepest > 60, "{} clues: deepest {}", clues.len(), o.deepest);
            // the search followed his order: cells are filled in scan order
            let (bp, _, d) = s.best_partial.expect("a partial board");
            for (k, &pos) in s.order.iter().enumerate() {
                assert_eq!(bp[pos] != EMPTYP, k < d || s.hint_at[pos].is_some(), "depth {k}");
            }
        }
    }

    /// A quota of 1 at depth 1 on a colour no corner carries: the sides placed
    /// before depth 1 (one corner) never reach it, but a candidate showing the
    /// colour does. His per-candidate test lets the search through; the
    /// port's per-node test killed every node at depth 1.
    #[test]
    fn quota_counts_the_candidate_like_his() {
        let (edges, _) = official();
        // the inner colour of some bottom-row edge piece (grey at the bottom)
        let s0 = Solver::new(edges.clone(), &[], ScanOrder::Published, &[], true, true, 10, 1, 1.0, QuotaBase::Count);
        let (pid, r) = find(&s0, |rp| rp.bottom == BORDER && rp.left != BORDER && rp.right != BORDER);
        let c = s0.rotpieces[pid as usize][r as usize].top;
        for legacy in [false, true] {
            let mut s = Solver::new(edges.clone(), &[c], ScanOrder::Published, &[], true, true, 10, 20_000, 1.0, QuotaBase::Count);
            s.quota = [0; 256];
            s.quota[1] = 1;
            // the faithful case runs on the default
            if legacy {
                s.legacy_quota_test = true;
            }
            let o = run_attempt_scored(&mut s, 1, 0, true);
            assert_eq!(o.hist[2] > 0, !legacy, "legacy={legacy}: nodes at depth 2 = {}", o.hist[2]);
        }
    }

    /// A pinned piece counts toward the quota once the scan reaches it, and
    /// its cell is a node (his centre piece is placed at depth 119).
    #[test]
    fn hints_count_when_the_scan_reaches_them() {
        let (edges, hints) = official();
        let centre: Vec<_> = hints.iter().copied().filter(|h| h.0 == 8 * W + 7).collect();
        // privileged colours chosen so that the centre piece carries some
        let (_, cp, cr) = centre[0];
        let probe = Solver::new(edges.clone(), &[], ScanOrder::Published, &[], true, true, 10, 1, 1.0, QuotaBase::Count);
        let crp = probe.rotpieces[cp as usize][cr as usize];
        let privileged = [crp.top, crp.left];
        for legacy in [false, true] {
            let mut s = Solver::new(edges.clone(), &privileged, ScanOrder::Published, &centre, true, true, 10, 1, 1.0, QuotaBase::Count);
            if legacy {
                s.legacy_hints = true;
            }
            s.reset();
            let hc = u16::from(s.rotpieces[cp as usize][cr as usize].heuristic_count);
            assert!(hc > 0);
            assert_eq!(s.priv_on_board, if legacy { hc } else { 0 }, "legacy={legacy}");
        }
        // a clue on the first scanned cell: its visit is a node
        let corner_cell = ScanOrder::Published.cells()[0];
        let s0 = Solver::new(edges.clone(), &[], ScanOrder::Published, &[], true, true, 10, 1, 1.0, QuotaBase::Count);
        let (pid, r) = find(&s0, |rp| rp.left == BORDER && rp.bottom == BORDER);
        for legacy in [false, true] {
            let mut s = Solver::new(edges.clone(), &PUBLISHED_KEYING, ScanOrder::Published, &[(corner_cell, pid, r)], true, true, 10, 1000, 1.0, QuotaBase::Share);
            if legacy {
                s.legacy_hints = true;
            }
            let o = run_attempt_scored(&mut s, 1, 0, true);
            assert_eq!(o.hist[0], u64::from(!legacy), "legacy={legacy}");
        }
    }

    #[test]
    fn wrong_class_matches_the_frame_sides_of_the_cell() {
        let (edges, _) = official();
        let s = solver(&edges, &[22, 21, 20], &[], 1);
        // piece 0 is a corner: grey on top and right at rotation 0
        let corner = s.rotpieces[0][0];
        assert_eq!((corner.top, corner.right), (BORDER, BORDER));
        assert!(!s.wrong_class(W - 1, &corner), "top-right corner cell");
        assert!(s.wrong_class(5, &corner), "edge cell of the top row");
        assert!(s.wrong_class(5 * W + 5, &corner), "interior cell");
    }

    /// Track 13 lost whole 1e9-node runs to this defect: seed 105 with the
    /// port's keying (22, 21, 20) and seed 108 with (3, 12, 13), no clue, went
    /// illegal within the first dozen cells and never came back (5 and 2 legal
    /// nodes). With the fix every placement is class-correct and the runs leave
    /// the opening.
    #[test]
    fn no_break_path_never_places_a_wrong_class_piece() {
        let (edges, _) = official();
        for (seed, privileged) in [(105u64, [22u8, 21, 20]), (108, [3, 12, 13])] {
            let mut s = solver(&edges, &privileged, &[], 1_000_000);
            let o = run_attempt_scored(&mut s, seed, 0, true);
            assert_eq!(o.illegal_placed, 0, "seed {seed}: wrong-class placements");
            assert!(o.deepest > 40, "seed {seed}: deepest {} (stranded runs stop at 22)", o.deepest);
        }
    }
}
