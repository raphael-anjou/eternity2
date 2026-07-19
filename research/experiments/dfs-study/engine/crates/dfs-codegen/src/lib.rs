//! `NAIVE-CODEGEN` — the same algorithm as `NAIVE-CLEAN`, engineered for one
//! fixed shape: a strict, row-major, 16×16 depth-first backtracker with no
//! heuristics and no breaks. It exists to answer one question the study poses —
//! *what does low-level specialisation buy over a clean general engine?* — so it
//! is deliberately narrow and is kept out of the general `dfs-engine`.
//!
//! The specialisations, all only valid for this fixed shape:
//!
//! * **Row-major means only two neighbours ever constrain a cell**: the one
//!   above (its down edge) and the one to the left (its right edge). The engine
//!   never looks at the down or right neighbours, because in a row-major fill
//!   they are always still empty. That halves the per-node constraint work.
//! * **Candidates are indexed by `(up, left)`**: a flat table maps the required
//!   up-colour and left-colour straight to a contiguous, sentinel-terminated
//!   list of oriented pieces that fit both — no per-candidate edge compare
//!   beyond the piece-used check.
//! * **Explicit stack, fixed arrays, no per-node allocation**: the search is an
//!   iterative loop over a 256-deep cursor stack; the board, the used-set
//!   (a `u64` bitset over ≤256 pieces), and the running score are flat arrays.
//!
//! It reads the same [`Instance`] and emits the same [`e2_io::SolveOutput`] as
//! every other variant, so it sits on the same leaderboard.

#![forbid(unsafe_code)]

use std::time::Instant;

use e2_core::{Board, SearchStats, N, W};
use e2_io::Instance;

/// The 22 real colours plus border fit comfortably; we size the index table to
/// the instance's colour count at build time.
/// A candidate packed into one 64-bit word so the hot loop reads it with a
/// single contiguous load (no `pool → oriented` pointer-chase) and can test
/// `is_used(pid)` before touching the edges. Layout:
///   bits 48..64 = piece id (u16)
///   bits 40..48 = rotation (u8)
///   bits  8..40 = URDL edges as a u32 (e0<<24 | e1<<16 | e2<<8 | e3)
///   bits  0.. 8 = unused
/// The all-ones value `u64::MAX` is the bucket terminator.
type Packed = u64;
const PACK_END: Packed = u64::MAX;

#[inline]
fn pack(pid: u16, rot: u8, e: [u8; 4]) -> Packed {
    let edges = (u32::from(e[0]) << 24)
        | (u32::from(e[1]) << 16)
        | (u32::from(e[2]) << 8)
        | u32::from(e[3]);
    (u64::from(pid) << 48) | (u64::from(rot) << 40) | (u64::from(edges) << 8)
}

// The unpack helpers deliberately truncate — they extract fixed bit fields from
// the packed word, so the narrowing casts are exact by construction.
#[inline]
#[allow(clippy::cast_possible_truncation)]
const fn unpack_pid(w: Packed) -> u16 {
    (w >> 48) as u16
}
#[inline]
#[allow(clippy::cast_possible_truncation)]
const fn unpack_rot(w: Packed) -> u8 {
    (w >> 40) as u8
}
#[inline]
#[allow(clippy::cast_possible_truncation)]
const fn unpack_edges(w: Packed) -> [u8; 4] {
    let edges = (w >> 8) as u32;
    [
        (edges >> 24) as u8,
        (edges >> 16) as u8,
        (edges >> 8) as u8,
        edges as u8,
    ]
}

struct Index {
    colors: usize,
    /// `by_ul_start[up * colors + left]` → offset into `pool` of that bucket's
    /// first packed candidate; the bucket is `PACK_END`-terminated.
    by_ul_start: Vec<u32>,
    pool: Vec<Packed>,
    /// Same, keyed only by up-colour (left unconstrained: the first column).
    by_u_start: Vec<u32>,
    pool_u: Vec<Packed>,
    /// Candidates with neither constraint (cell 0 only): all oriented pieces.
    all: Vec<Packed>,
}

impl Index {
    // `by_u_start` / `by_ul_start` are the up-only and up+left bucket offsets —
    // the one-letter difference is meaningful, not a typo.
    #[allow(clippy::similar_names)]
    fn build(inst: &Instance) -> Self {
        let colors = inst.pieces.max_color() as usize + 1;
        // Every distinct oriented piece, packed. Buckets store the packed word
        // directly so the hot loop needs no `oriented` array at all.
        let mut oriented: Vec<Packed> = Vec::with_capacity(inst.pieces.len() * 4);
        for (pid, p) in inst.pieces.iter() {
            let mut seen: Vec<[u8; 4]> = Vec::with_capacity(4);
            for rot in 0..4u8 {
                let e = p.rotated(rot);
                if seen.contains(&e) {
                    continue;
                }
                seen.push(e);
                oriented.push(pack(pid, rot, e));
            }
        }

        // Build the (up,left) buckets of packed candidates, then flatten.
        let mut buckets_ul: Vec<Vec<Packed>> = vec![Vec::new(); colors * colors];
        let mut buckets_u: Vec<Vec<Packed>> = vec![Vec::new(); colors];
        let mut all: Vec<Packed> = Vec::with_capacity(oriented.len() + 1);
        for &w in &oriented {
            let e = unpack_edges(w);
            buckets_ul[e[0] as usize * colors + e[3] as usize].push(w);
            buckets_u[e[0] as usize].push(w);
            all.push(w);
        }
        // `all` is iterated by the same terminator-based loop as the pooled
        // buckets (the neither-constrained case, e.g. a free cell 0).
        all.push(PACK_END);

        let (by_ul_start, pool) = flatten(&buckets_ul);
        let (by_u_start, pool_u) = flatten(&buckets_u);
        Self {
            colors,
            by_ul_start,
            pool,
            by_u_start,
            pool_u,
            all,
        }
    }
}

/// Flatten a vector of packed-candidate buckets into (start-offsets, pool) with
/// a `PACK_END` terminator after each bucket.
fn flatten(buckets: &[Vec<Packed>]) -> (Vec<u32>, Vec<Packed>) {
    let mut start = Vec::with_capacity(buckets.len());
    let mut pool = Vec::new();
    for b in buckets {
        start.push(pool.len() as u32);
        pool.extend_from_slice(b);
        pool.push(PACK_END);
    }
    (start, pool)
}

/// The run result, mirroring the general engine's.
pub struct RunResult {
    pub best: Board,
    pub best_score: u32,
    pub stats: SearchStats,
    pub elapsed_s: f64,
}

impl RunResult {
    #[must_use]
    pub fn output(&self, inst: &Instance) -> e2_io::SolveOutput {
        inst.finish(&self.best)
    }
}

/// Run the specialised row-major strict DFS. `budget_ms` is the wall-clock
/// budget; there is no seed (the order is deterministic).
#[must_use]
pub fn run(inst: &Instance, budget_ms: u64) -> RunResult {
    assert_eq!(inst.width as usize, W, "codegen engine is 16-wide only");
    assert_eq!(inst.height as usize, W, "codegen engine is 16-tall only");
    let idx = Index::build(inst);

    // Fill order: row-major over the non-pinned cells. Pinned cells are set and
    // never revisited; the cursor stack only covers the free cells.
    let mut pinned = [false; N];
    for h in &inst.hints {
        pinned[h.pos as usize] = true;
    }
    let free: Vec<usize> = (0..N).filter(|&p| !pinned[p]).collect();
    let depth = free.len();

    // The resolved edges per cell, and the (piece,rot) that produced them so a
    // best board reconstructs exactly (two pieces can share an edge tuple, so
    // edge-matching alone would be ambiguous). Pinned cells from the seed board.
    let seed = inst.seed_board();
    let mut cell = [[NC; 4]; N];
    let mut cell_pr = [(u16::MAX, 0u8); N];
    for pos in 0..N {
        if let Some((pid, rot)) = seed.piece_at(pos) {
            cell[pos] = inst.pieces.get(pid).unwrap().rotated(rot);
            cell_pr[pos] = (pid, rot);
        }
    }

    // used[piece] bitset (≤256 pieces → 4 u64 words).
    let words = (inst.pieces.len() + 63) / 64;
    let mut used = vec![0u64; words.max(4)];
    for h in &inst.hints {
        set_used(&mut used, h.piece);
    }

    // Per-level cursor into the current cell's candidate pool, and the piece
    // placed at that level (so we can free it on pop).
    let mut cursor = vec![0usize; depth + 1];
    let mut placed_piece = vec![u16::MAX; depth + 1];
    // Running matched-edge score at each level.
    let mut score_at = vec![0u32; depth + 1];

    let mut best_score = e2_core::score_board(&seed, &inst.pieces);
    // Snapshot the best board as a (piece,rot) grid — a cheap 512-byte memcpy
    // per improvement, instead of rebuilding a Board on the hot path.
    let mut best_cell_pr = cell_pr;
    score_at[0] = best_score;

    let mut stats = SearchStats::new();
    let pins = inst.hints.len() as u32;

    let start = Instant::now();
    let deadline = start + std::time::Duration::from_millis(budget_ms);

    let mut level: usize = 0; // index into `free`
    // Set the candidate cursor for level 0.
    reset_cursor(&mut cursor, level);

    loop {
        // Periodic time check.
        if stats.nodes & 0xFFF == 0 && Instant::now() >= deadline {
            stats.depth_at_timeout = pins + level as u32;
            break;
        }

        if level == depth {
            // Complete board (all N cells filled): record max depth here too, so
            // a solved board reads depth N, matching the general engine (which
            // observes depth at the top of its terminal call).
            let full = pins + level as u32;
            if full > stats.max_depth {
                stats.max_depth = full;
            }
            let sc = score_at[level];
            if sc > best_score {
                best_score = sc;
                best_cell_pr = cell_pr;
            }
            // Force a pop.
            if level == 0 {
                break;
            }
            level -= 1;
            pop(&mut used, &mut cell, &mut cell_pr, &free, &mut placed_piece, level);
            continue;
        }

        let pos = free[level];
        let up = if pos < W { 0 } else { cell[pos - W][2] };
        let left = if pos % W == 0 { 0 } else { cell[pos - 1][1] };

        // Candidate pool for this (up,left).
        let (pool, base): (&[Packed], usize) = candidate_pool(&idx, up, left);

        let observed = pins + level as u32;
        if observed > stats.max_depth {
            stats.max_depth = observed;
        }

        // Advance the cursor from where we left off, trying pieces until one
        // fits and is unused.
        let mut advanced = false;
        let mut c = cursor[level];
        loop {
            // One contiguous load of the packed candidate — no pool→oriented
            // pointer-chase. Extract the piece id and test `is_used` BEFORE
            // touching the edges, so already-placed pieces are skipped with a
            // single load + bitset probe (McGavin's `tileFree[t]`-first pattern).
            let w = pool[base + c];
            if w == PACK_END {
                break; // exhausted this cell
            }
            c += 1;
            let pid = unpack_pid(w);
            if is_used(&used, pid) {
                continue;
            }
            let e = unpack_edges(w);
            // Strict fit: up and left already guaranteed by the (up,left) index;
            // border-facing edges are keyed too (border colour 0 is a normal
            // key). Down/right are empty → no check.
            stats.nodes += 1;
            // Matched-edge gain: up and left, when non-border and equal.
            let mut gain = 0;
            if up != NC && up != 0 && e[0] == up {
                gain += 1;
            }
            if left != NC && left != 0 && e[3] == left {
                gain += 1;
            }
            cell[pos] = e;
            cell_pr[pos] = (pid, unpack_rot(w));
            set_used(&mut used, pid);
            placed_piece[level] = pid;
            score_at[level + 1] = score_at[level] + gain;
            cursor[level] = c; // resume here on the next sibling
            advanced = true;
            break;
        }

        if advanced {
            // Record best partial cheaply (only when it can beat the incumbent).
            if score_at[level + 1] > best_score {
                best_score = score_at[level + 1];
                best_cell_pr = cell_pr;
            }
            level += 1;
            reset_cursor(&mut cursor, level);
        } else {
            stats.backtracks += 1;
            if level == 0 {
                break;
            }
            level -= 1;
            pop(&mut used, &mut cell, &mut cell_pr, &free, &mut placed_piece, level);
        }
    }

    let elapsed_s = start.elapsed().as_secs_f64();
    // This engine is strict — it never breaks an edge — so its break count is
    // always zero. (A partial board's `max_score - score` counts still-empty
    // edges, which are not breaks; reporting it here would be a mislabel.)
    stats.breaks = 0;
    RunResult {
        best: rebuild_board(&best_cell_pr),
        best_score,
        stats,
        elapsed_s,
    }
}

/// "No constraint / empty" sentinel (matches the general engine).
const NC: u8 = 255;

#[inline]
fn candidate_pool<'a>(idx: &'a Index, up: u8, left: u8) -> (&'a [Packed], usize) {
    match (up == NC, left == NC) {
        (false, false) => (&idx.pool, idx.by_ul_start[up as usize * idx.colors + left as usize] as usize),
        (false, true) => (&idx.pool_u, idx.by_u_start[up as usize] as usize),
        _ => (&idx.all, 0),
    }
}

#[inline]
fn reset_cursor(cursor: &mut [usize], level: usize) {
    cursor[level] = 0;
}

/// Pop: free the piece placed at `level` and clear its cell (edges and the
/// `(piece,rot)` record both, so a shallower partial never reads a stale piece).
#[inline]
fn pop(
    used: &mut [u64],
    cell: &mut [[u8; 4]; N],
    cell_pr: &mut [(u16, u8); N],
    free: &[usize],
    placed: &mut [u16],
    level: usize,
) {
    let pid = placed[level];
    if pid != u16::MAX {
        clear_used(used, pid);
        cell[free[level]] = [NC; 4];
        cell_pr[free[level]] = (u16::MAX, 0);
        placed[level] = u16::MAX;
    }
}

#[inline]
fn set_used(used: &mut [u64], pid: u16) {
    used[pid as usize >> 6] |= 1 << (pid as usize & 63);
}
#[inline]
fn clear_used(used: &mut [u64], pid: u16) {
    used[pid as usize >> 6] &= !(1 << (pid as usize & 63));
}
#[inline]
fn is_used(used: &[u64], pid: u16) -> bool {
    used[pid as usize >> 6] & (1 << (pid as usize & 63)) != 0
}

/// Reconstruct a [`Board`] from the per-cell `(piece,rot)` record. Exact (no
/// edge ambiguity) and only called when a new best is found, so it is off the
/// hot path.
fn rebuild_board(cell_pr: &[(u16, u8); N]) -> Board {
    let mut board = Board::new();
    for (pos, &(pid, rot)) in cell_pr.iter().enumerate() {
        if pid != u16::MAX {
            board.place(pos, pid, rot);
        }
    }
    board
}
