//! Reference table of sub-grid placement counts for the official Eternity II
//! set — the canonical generator behind the site's "Reference numbers" page and
//! the `research/subgrid-placement-counts/` article.
//!
//! For a w×h block placed at a given board position we count the ways to fill it
//! with DISTINCT official pieces such that:
//!   * each cell is filled only from the piece class matching its number of
//!     border-facing sides (corner piece = 2 grey edges, edge piece = 1, interior
//!     piece = 0);
//!   * every edge that faces the board's outer rim is grey (color 0), and every
//!     edge that faces the board interior (a block-boundary edge that is not on
//!     the rim) is a real interior color (non-grey);
//!   * all internal shared edges between adjacent cells match.
//!
//! Distinctness IS enforced (verified: the distinct DFS and the non-distinct
//! transfer matrix disagree, e.g. 2×2 middle = 4 059 952 distinct vs 4 550 669
//! reusable).
//!
//! ## Method
//!
//! The distinct count is computed by an **exact, parallel distinct-counting
//! depth-first search** ([`count_distinct_opt`]): fill the block cell by cell in
//! row-major order, at each cell trying only the candidates whose up/left colours
//! match the already-placed neighbours (looked up through a colour index, so each
//! step touches ~1 candidate) and rejecting any piece id already used in the block
//! (a `used` marker — this enforces global distinctness directly). A prefix of the
//! cells is fanned out across up to 8 worker threads. The result is exact and,
//! because thread-private counters are summed at the end, deterministic.
//!
//! The DFS visits ~one node per valid partial filling, so its cost is bounded by
//! the *non-distinct* count, which the broken-profile colour transfer matrix
//! ([`color_dp`]) gives instantly as an exact upper bound. Blocks whose
//! non-distinct count exceeds [`DFS_BUDGET`] — the 3×3 middle (~1.5e11) and the
//! 4×4 side/middle and 4×4-corner *empty*/*fixed* columns (up to ~2.5e13), which
//! would take minutes-to-hours to enumerate exactly — are reported as `null` with
//! a `note` (their exact values are sylvogel's published reference table). Every
//! other column is computed exactly, and the whole table runs in well under a
//! minute. The per-corner cN split is bucketed in a single DFS, and a corner
//! block's four-hint count is derived as the sum of its split (no extra sweep).

use eternity2_engine::official::official_puzzle;
use eternity2_engine::types::{rotated, Color, Puzzle};
use std::collections::HashMap as StdHashMap;
use std::hash::{BuildHasherDefault, Hasher};

/// Minimal FxHash-style hasher (rustc's algorithm) — far faster than the default
/// SipHash for the small integer keys used by the DPs, and deterministic.
#[derive(Default)]
struct FxHasher {
    hash: u64,
}
impl Hasher for FxHasher {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        const SEED: u64 = 0x51_7c_c1_b7_27_22_0a_95;
        for &b in bytes {
            self.hash = (self.hash.rotate_left(5) ^ b as u64).wrapping_mul(SEED);
        }
    }
    #[inline]
    fn write_u64(&mut self, i: u64) {
        const SEED: u64 = 0x51_7c_c1_b7_27_22_0a_95;
        self.hash = (self.hash.rotate_left(5) ^ i).wrapping_mul(SEED);
    }
    #[inline]
    fn write_u128(&mut self, i: u128) {
        self.write_u64(i as u64);
        self.write_u64((i >> 64) as u64);
    }
    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }
}
type HashMap<K, V> = StdHashMap<K, V, BuildHasherDefault<FxHasher>>;

const SIZE: usize = 16;

/// Number of grey (border) edges → 2 corner, 1 edge, 0 interior piece.
fn piece_class(e: [Color; 4]) -> u8 {
    e.iter().filter(|&&c| c == 0).count() as u8
}

#[derive(Clone, Copy)]
struct Block {
    x0: usize,
    y0: usize,
    w: usize,
    h: usize,
}

impl Block {
    /// (up, down, left, right) sides on the board rim.
    fn touches(&self) -> (bool, bool, bool, bool) {
        (
            self.y0 == 0,
            self.y0 + self.h == SIZE,
            self.x0 == 0,
            self.x0 + self.w == SIZE,
        )
    }
}

/// Does an oriented piece `t` (URDL) satisfy the rim / interior-boundary color
/// rules at cell (r,c) of a w×h block whose rim-touch flags are (bu,bd,bl,br)?
#[allow(clippy::too_many_arguments)]
fn cell_ok(
    t: [Color; 4],
    r: usize,
    c: usize,
    w: usize,
    h: usize,
    bu: bool,
    bd: bool,
    bl: bool,
    br: bool,
) -> bool {
    // up
    if r == 0 {
        if bu {
            if t[0] != 0 {
                return false;
            }
        } else if t[0] == 0 {
            return false;
        }
    }
    // left
    if c == 0 {
        if bl {
            if t[3] != 0 {
                return false;
            }
        } else if t[3] == 0 {
            return false;
        }
    }
    // right
    if c == w - 1 {
        if br {
            if t[1] != 0 {
                return false;
            }
        } else if t[1] == 0 {
            return false;
        }
    }
    // down
    if r == h - 1 {
        if bd {
            if t[2] != 0 {
                return false;
            }
        } else if t[2] == 0 {
            return false;
        }
    }
    true
}

/// Per-cell oriented-piece candidates.
///
/// Each cell's candidate list already encodes the cell's piece class (corner /
/// edge / interior) and all rim / block-boundary colour rules — `build_cands`
/// only admits pieces of the matching class with admissible boundary edges — so
/// the distinct counter need not track classes separately: distinctness is a pure
/// piece-id constraint and class agreement is implied by candidacy.
struct Cands {
    /// for each cell: list of (piece_id, oriented edges URDL)
    cands: Vec<Vec<(u16, [Color; 4])>>,
}

/// Build the candidate lists for a block under a pool/pin configuration.
fn build_cands(
    pieces: &[[Color; 4]],
    cls: &[u8],
    banned: &[bool],
    pins: &[(usize, usize, u8)],
    block: Block,
) -> Cands {
    let (bu, bd, bl, br) = block.touches();
    let (w, h) = (block.w, block.h);
    let cells = w * h;
    let mut cands: Vec<Vec<(u16, [Color; 4])>> = vec![Vec::new(); cells];
    for idx in 0..cells {
        let (r, c) = (idx / w, idx % w);
        let border_sides = (r == 0 && bu) as u8
            + (r == h - 1 && bd) as u8
            + (c == 0 && bl) as u8
            + (c == w - 1 && br) as u8;
        if let Some(&(_, pp, pr)) = pins.iter().find(|&&(ci, _, _)| ci == idx) {
            let t = rotated(pieces[pp], pr);
            if cell_ok(t, r, c, w, h, bu, bd, bl, br) {
                cands[idx].push((pp as u16, t));
            }
            continue;
        }
        for pi in 0..pieces.len() {
            if banned[pi] || cls[pi] != border_sides {
                continue;
            }
            for rr in 0..4u8 {
                let t = rotated(pieces[pi], rr);
                if cell_ok(t, r, c, w, h, bu, bd, bl, br) {
                    cands[idx].push((pi as u16, t));
                }
            }
        }
    }
    Cands { cands }
}

/// Pack a north-color profile (w colors) and a west color into one u64.
/// Colors are ≤ 22, plus sentinel 31 (5 bits each; w ≤ 4 ⇒ fits in u64).
#[inline]
fn pack(north: &[Color], west: Color) -> u64 {
    let mut k = 0u64;
    for &x in north {
        k = k * 32 + x as u64;
    }
    k * 32 + west as u64
}

#[inline]
fn unpack(key: u64, w: usize, north: &mut [Color]) -> Color {
    let mut k = key;
    let west = (k % 32) as u8;
    k /= 32;
    for i in (0..w).rev() {
        north[i] = (k % 32) as u8;
        k /= 32;
    }
    west
}

/// Per-cell candidate transitions indexed by their two incoming edge colours.
///
/// For each cell, `by_in[cell][need_up*32 + need_left]` lists the candidates whose
/// up-colour and left-colour match those incoming edges (sentinel 31 when the cell
/// is on the block's top row / left column, whose facing edge is already filtered
/// in `build_cands`). Each entry is `(pid, down, right)`. The DP then reads a flat
/// array slot instead of scanning the (up to ~800-long) candidate list, so every
/// transition touches ~1 candidate. `pid_by_in[cell]` mirrors `by_in` but keyed
/// also by pid (sorted entry list), used when a tied cell forces a specific pid.
struct CellIndex {
    by_in: Vec<Vec<Vec<(u16, Color, Color)>>>,
    has_up: Vec<bool>,
    has_left: Vec<bool>,
}

fn build_index(cc: &Cands, w: usize) -> CellIndex {
    let cells = cc.cands.len();
    let mut by_in: Vec<Vec<Vec<(u16, Color, Color)>>> =
        (0..cells).map(|_| vec![Vec::new(); 32 * 32]).collect();
    let mut has_up = vec![false; cells];
    let mut has_left = vec![false; cells];
    for idx in 0..cells {
        let (r, c) = (idx / w, idx % w);
        let up = r > 0;
        let left = c > 0;
        has_up[idx] = up;
        has_left[idx] = left;
        for &(pid, t) in &cc.cands[idx] {
            let uk = if up { t[0] } else { 31 } as usize;
            let lk = if left { t[3] } else { 31 } as usize;
            by_in[idx][uk * 32 + lk].push((pid, t[2], t[1]));
        }
    }
    CellIndex { by_in, has_up, has_left }
}

/// Cache-friendly flattened candidate index for the hot DFS loop. For each cell
/// the candidates of every (up,left) colour slot are concatenated into one
/// contiguous `cands` array; `slot_off[cell*1024 + slot ..= +1]` gives the
/// half-open range for slot `up*32+left`. Avoids the triple `Vec` indirection of
/// `CellIndex` (one contiguous read per node instead of three pointer chases).
struct FlatIndex {
    cands: Vec<Vec<(u16, u8, u8)>>, // per-cell contiguous candidate array
    slot_off: Vec<[u32; 1025]>,     // per-cell slot offsets (1024 slots + sentinel)
    col: Vec<usize>,
    up_mask: Vec<bool>,
    left_mask: Vec<bool>,
}

fn build_flat(idx: &CellIndex, w: usize) -> FlatIndex {
    let cells = idx.by_in.len();
    let mut cands = Vec::with_capacity(cells);
    let mut slot_off = Vec::with_capacity(cells);
    for i in 0..cells {
        let mut flat: Vec<(u16, u8, u8)> = Vec::new();
        let mut off = [0u32; 1025];
        for slot in 0..1024usize {
            off[slot] = flat.len() as u32;
            for &(pid, down, right) in &idx.by_in[i][slot] {
                flat.push((pid, down, right));
            }
        }
        off[1024] = flat.len() as u32;
        cands.push(flat);
        slot_off.push(off);
    }
    FlatIndex {
        cands,
        slot_off,
        col: (0..cells).map(|i| i % w).collect(),
        up_mask: (0..cells).map(|i| idx.has_up[i]).collect(),
        left_mask: (0..cells).map(|i| idx.has_left[i]).collect(),
    }
}

/// Pure colour broken-profile DP (no distinctness, no id bookkeeping) — the
/// non-distinct count and the all-singleton I-E base term. u64-only state key.
fn color_dp(idx: &CellIndex, w: usize, cells: usize) -> u128 {
    let mut cur: HashMap<u64, u128> = HashMap::default();
    cur.insert(pack(&[31u8; 4][..w], 31), 1);
    let mut north = [31u8; 4];
    for i in 0..cells {
        let c = i % w;
        let has_up = idx.has_up[i];
        let has_left = idx.has_left[i];
        let by_in = &idx.by_in[i];
        let mut next: HashMap<u64, u128> =
            HashMap::with_capacity_and_hasher(cur.len() * 2, Default::default());
        for (&ckey, &cnt) in &cur {
            let west = unpack(ckey, w, &mut north[..w]);
            let up_key = if has_up { north[c] } else { 31 } as usize;
            let left_key = if has_left { west } else { 31 } as usize;
            let cands = &by_in[up_key * 32 + left_key];
            for &(_pid, down, right) in cands {
                let old = north[c];
                north[c] = down;
                let nkey = pack(&north[..w], right);
                north[c] = old;
                *next.entry(nkey).or_insert(0) += cnt;
            }
        }
        cur = next;
    }
    cur.values().sum()
}

/// Upper limit on the number of edge-valid (non-distinct) fillings for which we
/// run the exact distinct-counting DFS within the table's time budget.
///
/// The DFS visits ~one node per valid partial filling, so its running time scales
/// with the non-distinct count (the `color_dp` value, an exact upper bound on the
/// distinct count). At ~2x10^8 nodes/s/core × 8 cores a block of this size takes a
/// few seconds, keeping the whole table well under a minute. Blocks whose
/// non-distinct count exceeds this — the 3x3 middle (~1.5x10^11) and the 4x4 side
/// / middle / corner *empty* and *fixed* columns (up to ~2.5x10^13), each of which
/// would take minutes-to-hours to enumerate exactly — are reported as `null` with
/// a `note`; their published exact values are sylvogel's reference table. Every
/// other column (incl. all 4x4 fixed+4-hints and per-corner cN splits, which pin
/// the near-corner clue and so are small) is computed exactly here.
const DFS_BUDGET: u128 = 5_000_000_000; // 5e9

/// Exact count of edge-valid fillings of the block in which **all chosen piece
/// ids are distinct**, or `None` when the block is too large for the exact DFS.
///
/// ## Method — parallel distinct-counting depth-first search
///
/// The block is filled cell by cell in row-major order. At each cell only the
/// candidates whose up/left colours match the already-placed neighbours are tried
/// (read through the colour index `CellIndex`, so each step touches ~1 candidate),
/// and a `used` marker rejects any piece id already placed elsewhere in the block
/// — this enforces global distinctness directly and exactly. A completed grid adds
/// one to the count.
///
/// Because the colour constraints prune extremely hard, the search tree has
/// essentially one node per valid partial filling, so the running time scales with
/// the *non-distinct* count (the transfer-matrix `color_dp` value, an exact upper
/// bound). When that exceeds `DFS_BUDGET` the block is reported as `None` (the few
/// largest interior columns — see `DFS_BUDGET`). Otherwise a prefix of the cells is
/// fanned out into independent subtrees across up to 8 worker threads via a shared
/// atomic work-queue, each accumulating a private counter; the partials are summed
/// at the end, so the result is exact and deterministic regardless of scheduling.
fn count_distinct_opt(cc: &Cands, w: usize, h: usize) -> Option<u128> {
    let cells = w * h;
    let idx = build_index(cc, w);

    // Non-distinct count = exact upper bound on the DFS work; bail if over budget.
    let nondistinct = color_dp(&idx, w, cells);
    if nondistinct > DFS_BUDGET {
        return None;
    }
    if cells == 0 {
        return Some(0);
    }

    // Row-major DFS. `north[c]` = colour on the bottom edge of the piece in column
    // c of the row above (31 for the top row); `west` = right-edge colour of the
    // piece just placed to the left (31 at column 0); `used[pid]` marks a piece
    // already placed in this block (enforces distinctness). Counts fit in u64 for
    // every block we actually run (≤ DFS_BUDGET ≈ 4e11).
    let flat = build_flat(&idx, w);

    #[inline]
    fn go(f: &FlatIndex, i: usize, north: &mut [u8; 4], west: u8, used: &mut [bool; 256], count: &mut u64) {
        if i == f.col.len() {
            *count += 1;
            return;
        }
        let c = f.col[i];
        let up_key = if f.up_mask[i] { north[c] as usize } else { 31 };
        let left_key = if f.left_mask[i] { west as usize } else { 31 };
        let slot = up_key * 32 + left_key;
        let off = &f.slot_off[i];
        let (lo, hi) = (off[slot] as usize, off[slot + 1] as usize);
        let cands = &f.cands[i][lo..hi];
        for &(pid, down, right) in cands {
            let u = &mut used[pid as usize];
            if *u {
                continue;
            }
            *u = true;
            let old = north[c];
            north[c] = down;
            go(f, i + 1, north, right, used, count);
            north[c] = old;
            used[pid as usize] = false;
        }
    }

    // Parallel fan-out: enumerate the placements of a PREFIX of cells (0..seed_n)
    // into independent tasks; each task DFS-completes cells seed_n..cells. Seeding
    // deep enough (≈ the first row plus a couple of cells) yields many small,
    // well-balanced subtrees so all cores stay busy even for the giant blocks; a
    // shared atomic work-queue hands them out. Each task carries its full
    // continuation state: the north profile, the west colour entering cell seed_n,
    // and the set of piece ids already placed in the prefix.
    type Seed = ([u8; 4], u8, Vec<u16>); // (north, west_at_seed_n, used pids)
    let seed_n = (w + 2).min(cells); // first row + up to 2 cells of the next row
    let mut seeds: Vec<Seed> = Vec::new();
    {
        #[allow(clippy::too_many_arguments)]
        fn prefix(
            f: &FlatIndex,
            cells: usize,
            seed_n: usize,
            i: usize,
            north: &mut [u8; 4],
            west: u8,
            used: &mut [bool; 256],
            used_ids: &mut Vec<u16>,
            seeds: &mut Vec<Seed>,
        ) {
            if i == seed_n {
                seeds.push((*north, west, used_ids.clone()));
                return;
            }
            let c = f.col[i];
            let up_key = if f.up_mask[i] { north[c] as usize } else { 31 };
            let left_key = if f.left_mask[i] { west as usize } else { 31 };
            let slot = up_key * 32 + left_key;
            let off = &f.slot_off[i];
            let (lo, hi) = (off[slot] as usize, off[slot + 1] as usize);
            for &(pid, down, right) in &f.cands[i][lo..hi] {
                if used[pid as usize] {
                    continue;
                }
                used[pid as usize] = true;
                used_ids.push(pid);
                let old = north[c];
                north[c] = down;
                // west entering the next cell is `right`, but reset to 31 at the
                // start of a new row (handled by left_mask at that cell).
                prefix(f, cells, seed_n, i + 1, north, right, used, used_ids, seeds);
                north[c] = old;
                used_ids.pop();
                used[pid as usize] = false;
            }
            let _ = cells;
        }
        let mut north = [31u8; 4];
        let mut used = [false; 256];
        let mut used_ids: Vec<u16> = Vec::new();
        prefix(&flat, cells, seed_n, 0, &mut north, 31, &mut used, &mut used_ids, &mut seeds);
    }

    if seed_n == cells {
        // The prefix already covers the whole block ⇒ every seed is a full filling.
        return Some(seeds.len() as u128);
    }

    let nthreads = std::thread::available_parallelism().map(|n| n.get().min(8)).unwrap_or(4);
    let flat = &flat;
    let seeds = std::sync::Arc::new(seeds);
    let total: u128 = std::thread::scope(|scope| {
        let next = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let mut handles = Vec::new();
        for _ in 0..nthreads {
            let next = next.clone();
            let seeds = seeds.clone();
            handles.push(scope.spawn(move || {
                let mut local: u64 = 0;
                let mut used = [false; 256];
                loop {
                    let k = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if k >= seeds.len() {
                        break;
                    }
                    let (seed_north, seed_west, seed_used) = &seeds[k];
                    let mut north = *seed_north;
                    for &pid in seed_used {
                        used[pid as usize] = true;
                    }
                    go(flat, seed_n, &mut north, *seed_west, &mut used, &mut local);
                    for &pid in seed_used {
                        used[pid as usize] = false;
                    }
                }
                local as u128
            }));
        }
        handles.into_iter().map(|h| h.join().unwrap()).sum()
    });

    Some(total)
}

/// Distinct count, panicking if the block is over budget (callers that know a
/// block is small use this; the table emitter uses `count_distinct_opt`).
fn count_distinct(cc: &Cands, w: usize, h: usize) -> u128 {
    count_distinct_opt(cc, w, h).expect("block within DFS budget")
}

/// Of the distinct fills, the four-way split by which of corner pieces 0..3
/// occupies the block's board-corner cell.
///
/// This is a single distinct-counting DFS (the same machinery as
/// `count_distinct_opt`) that, instead of one total, keeps four counters bucketed
/// by which corner piece sits in the block's board-corner cell — far cheaper than
/// four separate sweeps, since for corner blocks the corner cell is the last cell
/// filled and a per-piece restriction would prune almost nothing until the leaf.
fn corner_split(
    pieces: &[[Color; 4]],
    cls: &[u8],
    banned: &[bool],
    pins: &[(usize, usize, u8)],
    block: Block,
) -> [u128; 4] {
    let (bu, _bd, bl, _br) = block.touches();
    let (w, h) = (block.w, block.h);
    let cells = w * h;
    let r = if bu { 0 } else { h - 1 };
    let c = if bl { 0 } else { w - 1 };
    let corner_cell = r * w + c;

    let cc = build_cands(pieces, cls, banned, pins, block);
    let idx = build_index(&cc, w);
    let flat = build_flat(&idx, w);

    // Map every piece id to its corner-piece bucket (0..3) or 4 = "not a corner
    // piece" (the corner cell only ever holds a corner piece, but be safe).
    let mut bucket = [4u8; 256];
    for b in 0..4u16.min(pieces.len() as u16) {
        bucket[b as usize] = b as u8;
    }

    // DFS that records the bucket of the piece placed in `corner_cell`.
    #[allow(clippy::too_many_arguments)]
    fn go(
        f: &FlatIndex,
        i: usize,
        corner_cell: usize,
        bucket: &[u8; 256],
        cur_bucket: u8,
        north: &mut [u8; 4],
        west: u8,
        used: &mut [bool; 256],
        out: &mut [u128; 4],
    ) {
        if i == f.col.len() {
            if (cur_bucket as usize) < 4 {
                out[cur_bucket as usize] += 1;
            }
            return;
        }
        let cc = f.col[i];
        let up_key = if f.up_mask[i] { north[cc] as usize } else { 31 };
        let left_key = if f.left_mask[i] { west as usize } else { 31 };
        let slot = up_key * 32 + left_key;
        let off = &f.slot_off[i];
        let (lo, hi) = (off[slot] as usize, off[slot + 1] as usize);
        for &(pid, down, right) in &f.cands[i][lo..hi] {
            if used[pid as usize] {
                continue;
            }
            let nb = if i == corner_cell { bucket[pid as usize] } else { cur_bucket };
            used[pid as usize] = true;
            let old = north[cc];
            north[cc] = down;
            go(f, i + 1, corner_cell, bucket, nb, north, right, used, out);
            north[cc] = old;
            used[pid as usize] = false;
        }
    }

    // Parallel over the first cell's candidates (corner blocks are ≤16 cells and
    // fixe4-pinned, so this is plenty of tasks). Each task keeps its own buckets.
    let off0 = &flat.slot_off[0];
    let slot0 = 31 * 32 + 31; // cell 0: top-left of block, both edges free of in-block neighbours
    let (lo0, hi0) = (off0[slot0] as usize, off0[slot0 + 1] as usize);
    let first: Vec<(u16, u8, u8)> = flat.cands[0][lo0..hi0].to_vec();
    let nthreads = std::thread::available_parallelism().map(|n| n.get().min(8)).unwrap_or(4);
    let flat = &flat;
    let bucket = &bucket;
    let first = std::sync::Arc::new(first);
    let res: [u128; 4] = std::thread::scope(|scope| {
        let next = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let mut handles = Vec::new();
        for _ in 0..nthreads {
            let next = next.clone();
            let first = first.clone();
            handles.push(scope.spawn(move || {
                let mut out = [0u128; 4];
                let mut used = [false; 256];
                let mut north = [31u8; 4];
                loop {
                    let k = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if k >= first.len() {
                        break;
                    }
                    let (pid, down, right) = first[k];
                    let nb = if corner_cell == 0 { bucket[pid as usize] } else { 4 };
                    used[pid as usize] = true;
                    north[0] = down;
                    go(flat, 1, corner_cell, bucket, nb, &mut north, right, &mut used, &mut out);
                    north[0] = 31;
                    used[pid as usize] = false;
                }
                out
            }));
        }
        handles.into_iter().fold([0u128; 4], |mut acc, h| {
            let o = h.join().unwrap();
            for i in 0..4 {
                acc[i] += o[i];
            }
            acc
        })
    });
    let _ = cells;
    res
}

/// 3×3 encircle: the 8 interior cells around the centre clue at (7,8). The
/// centre cell is the pinned clue; the ring cells must edge-match each other and
/// the clue. We model this as a 3×3 interior block centred on (7,8) with the
/// centre pinned, then the I-E machinery handles the 8 ring cells exactly.
fn encircle_block() -> Block {
    // centre clue at (x=7,y=8) → 3×3 block top-left (6,7) covers (6,7)..(8,9).
    Block { x0: 6, y0: 7, w: 3, h: 3 }
}

#[derive(Clone, Copy)]
enum Variant {
    Vide,
    Fixe,
    Fixe4,
}

/// Pool ban-list + cell pins for a (block, variant). For non-corner middle/side
/// blocks the caller positions the block away from clues so fixe4 only bans the
/// five clue pieces (no pin), matching the reference table.
fn config(puzzle: &Puzzle, block: Block, variant: Variant) -> (Vec<bool>, Vec<(usize, usize, u8)>) {
    let n = puzzle.pieces.len();
    let clue_ids: Vec<usize> = puzzle.hints.iter().map(|h| h.piece as usize).collect();
    let centre = puzzle
        .hints
        .iter()
        .find(|h| {
            let (x, y) = (h.pos as usize % SIZE, h.pos as usize / SIZE);
            x == SIZE / 2 - 1 && y == SIZE / 2 // official centre clue at (7,8)
        })
        .map(|h| h.piece as usize);
    let mut banned = vec![false; n];
    let mut pins = Vec::new();
    match variant {
        Variant::Vide => {}
        Variant::Fixe => {
            if let Some(c) = centre {
                banned[c] = true;
            }
        }
        Variant::Fixe4 => {
            for &c in &clue_ids {
                banned[c] = true;
            }
            for h in &puzzle.hints {
                let (hx, hy) = (h.pos as usize % SIZE, h.pos as usize / SIZE);
                if hx >= block.x0
                    && hx < block.x0 + block.w
                    && hy >= block.y0
                    && hy < block.y0 + block.h
                {
                    let cell = (hy - block.y0) * block.w + (hx - block.x0);
                    pins.push((cell, h.piece as usize, h.rot));
                }
            }
        }
    }
    (banned, pins)
}

fn count_for(puzzle: &Puzzle, pieces: &[[Color; 4]], cls: &[u8], block: Block, v: Variant) -> Option<u128> {
    let (banned, pins) = config(puzzle, block, v);
    let cc = build_cands(pieces, cls, &banned, &pins, block);
    count_distinct_opt(&cc, block.w, block.h)
}

/// Render an optional count as a JSON number, or `null` when the block exceeds the
/// exact-DFS budget (only the 4x4 side/middle).
fn jnum(v: Option<u128>) -> String {
    match v {
        Some(x) => x.to_string(),
        None => "null".to_string(),
    }
}

fn main() {
    let puzzle = official_puzzle();
    let pieces = &puzzle.pieces;
    let cls: Vec<u8> = pieces.iter().map(|&e| piece_class(e)).collect();

    if std::env::var("PROBE").is_ok() {
        // Quick timing/smoke path for a few representative blocks.
        let probes: Vec<(&str, Block, Variant)> = vec![
            ("2x2-mid vide", Block { x0: 4, y0: 4, w: 2, h: 2 }, Variant::Vide),
            ("3x3-side vide", Block { x0: 7, y0: 0, w: 3, h: 3 }, Variant::Vide),
            ("3x3-middle vide", Block { x0: 4, y0: 4, w: 3, h: 3 }, Variant::Vide),
            ("4x4-tl fixe4", Block { x0: 0, y0: 0, w: 4, h: 4 }, Variant::Fixe4),
            ("4x4-side vide", Block { x0: 6, y0: 0, w: 4, h: 4 }, Variant::Vide),
        ];
        for (name, b, v) in probes {
            let t = std::time::Instant::now();
            let val = count_for(&puzzle, pieces, &cls, b, v);
            eprintln!("PROBE {name} = {}  ({:?})", jnum(val), t.elapsed());
        }
        return;
    }

    let corner_positions = |s: usize| {
        [
            ("tl", Block { x0: 0, y0: 0, w: s, h: s }),
            ("tr", Block { x0: SIZE - s, y0: 0, w: s, h: s }),
            ("br", Block { x0: SIZE - s, y0: SIZE - s, w: s, h: s }),
            ("bl", Block { x0: 0, y0: SIZE - s, w: s, h: s }),
        ]
    };
    // Representative side/middle blocks positioned to contain NO clue cell, so
    // fixe4 bans the five clue pieces without pinning any cell (matching the
    // reference table). Clue cells: (7,8),(2,2),(13,2),(2,13),(13,13).
    let side = |s: usize| Block { x0: SIZE / 2 - s / 2, y0: 0, w: s, h: s };
    let middle = |s: usize| Block { x0: 4, y0: 4, w: s, h: s };

    let mut rows: Vec<String> = Vec::new();

    // A row where any of vide/fixe/fixe4 came back None (over the exact-DFS
    // budget) carries a "note" so consumers know the gap is a tractability limit,
    // not a missing measurement.
    const OVER_NOTE: &str =
        " \"note\": \"empty/fixed counts above the exact distinct-DFS budget (~5e9 fillings) are intractable to enumerate exactly in seconds; see sylvogel's published table\",";

    let emit_corner = |rows: &mut Vec<String>, key: &str, size: usize, b: Block| {
        let cv = count_for(&puzzle, pieces, &cls, b, Variant::Vide);
        let cf = count_for(&puzzle, pieces, &cls, b, Variant::Fixe);
        // The four-hint count is exactly the sum of the per-corner-piece split
        // (every fixe4 filling places exactly one corner piece in the corner cell),
        // so we compute the split once and derive fixe4 from it — avoiding a second
        // full DFS over the same block.
        let (banned, pins) = config(&puzzle, b, Variant::Fixe4);
        let split = corner_split(pieces, &cls, &banned, &pins, b);
        let cf4 = Some(split[0] + split[1] + split[2] + split[3]);
        let note = if cv.is_none() || cf.is_none() { OVER_NOTE } else { "" };
        rows.push(format!(
            "    {{ \"key\": \"{key}\", \"size\": {size}, \"position\": \"corner\",{note} \"vide\": {}, \"fixe\": {}, \"fixe4\": {}, \"cN\": [{},{},{},{}] }}",
            jnum(cv), jnum(cf), jnum(cf4), split[0], split[1], split[2], split[3]
        ));
        eprintln!("done {key}");
    };
    let emit_plain = |rows: &mut Vec<String>, key: &str, size: usize, b: Block, position: &str, do_vide: bool| {
        let cv = if do_vide { count_for(&puzzle, pieces, &cls, b, Variant::Vide) } else { None };
        let cf = count_for(&puzzle, pieces, &cls, b, Variant::Fixe);
        let cf4 = count_for(&puzzle, pieces, &cls, b, Variant::Fixe4);
        let note = if (do_vide && cv.is_none()) || cf.is_none() || cf4.is_none() { OVER_NOTE } else { "" };
        rows.push(format!(
            "    {{ \"key\": \"{key}\", \"size\": {size}, \"position\": \"{position}\",{note} \"vide\": {}, \"fixe\": {}, \"fixe4\": {}, \"cN\": null }}",
            jnum(cv), jnum(cf), jnum(cf4)
        ));
        eprintln!("done {key}");
    };

    for s in [2usize, 3] {
        for (name, b) in corner_positions(s) {
            emit_corner(&mut rows, &format!("{s}x{s}-{name}"), s, b);
        }
        emit_plain(&mut rows, &format!("{s}x{s}-side"), s, side(s), "side", true);
        emit_plain(&mut rows, &format!("{s}x{s}-middle"), s, middle(s), "middle", true);
    }
    // 4×4: corners + side (middle omitted — too large, as in the source table).
    for (name, b) in corner_positions(4) {
        emit_corner(&mut rows, &format!("4x4-{name}"), 4, b);
    }
    emit_plain(&mut rows, "4x4-side", 4, side(4), "side", true);

    // 3×3 encircle (around the centre clue). "vide" is N/A (the centre is the
    // fixed clue piece); fixe pins the centre and bans that piece; fixe4 pins the
    // centre and bans all five clues. The centre clue sits at the block's middle
    // cell (index 4) — we always pin it; the only difference between fixe and
    // fixe4 is whether we ban just the centre piece or all five clue pieces.
    let eb = encircle_block();
    let centre_hint = puzzle
        .hints
        .iter()
        .find(|h| (h.pos as usize % SIZE, h.pos as usize / SIZE) == (SIZE / 2 - 1, SIZE / 2))
        .expect("centre clue");
    let centre_cell = (centre_hint.pos as usize / SIZE - eb.y0) * eb.w
        + (centre_hint.pos as usize % SIZE - eb.x0);
    let enc_pins = vec![(centre_cell, centre_hint.piece as usize, centre_hint.rot)];
    let encircle = |ban_all: bool| -> u128 {
        let mut banned = vec![false; pieces.len()];
        if ban_all {
            for h in &puzzle.hints {
                banned[h.piece as usize] = true;
            }
        } else {
            banned[centre_hint.piece as usize] = true;
        }
        let cc = build_cands(pieces, &cls, &banned, &enc_pins, eb);
        count_distinct(&cc, eb.w, eb.h)
    };
    let enc_fixe = encircle(false);
    let enc_fixe4 = encircle(true);
    rows.push(format!(
        "    {{ \"key\": \"3x3-encircle\", \"size\": 3, \"position\": \"encircle\", \"vide\": null, \"fixe\": {enc_fixe}, \"fixe4\": {enc_fixe4}, \"cN\": null }}"
    ));
    eprintln!("done 3x3-encircle");

    println!("{{");
    println!("  \"puzzle\": \"official_eternity2\",");
    println!("  \"boardSize\": {SIZE},");
    println!("  \"rows\": [");
    println!("{}", rows.join(",\n"));
    println!("  ]");
    println!("}}");
}
