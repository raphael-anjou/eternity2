//! Optimization track for bb-mrv (see `../PREREG.md`).
//!
//! - [`RefSolver`]: a verbatim copy of A16's `BbSolver { rule: Rule::Mrv }`
//!   search (`backdoor-branching/compute/src/lib.rs`, `Search::choose` on the
//!   `Rule::Mrv` path and `Search::dfs`), calling the unmodified
//!   `backdoor_branching::{options, assign}` and `sat_backdoors::Model`, with
//!   one added line: a [`NodeHook`] call right after each successful
//!   `NodeCounter::try_assign`. It is checked node for node against the
//!   unmodified `BbSolver` on every gate seed.
//! - [`FastSolver`]: the same search with a cheaper node: piece-aligned
//!   option bitsets (4 bits per piece, so piece presence is a nibble fold),
//!   const-generic fixed-size domains, a preallocated state stack, no heap
//!   allocation per node, an all-different pass over piece-presence masks
//!   instead of a cells x pieces scan, and a colour-offer test that walks the
//!   set bits of small domains.

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use backdoor_branching::{assign, options};
use eternity2_engine::Puzzle;
use hard_ladder::{LadderSolver, NodeCounter, Outcome};
use sat_backdoors::{Model, Stats};

/// Per-run wall-clock guard in seconds (A22 default: 600 at cap 1e6).
pub static GUARD_SECS: AtomicU64 = AtomicU64::new(600);

#[must_use]
pub fn guard() -> Duration {
    Duration::from_secs(GUARD_SECS.load(Ordering::Relaxed))
}

/// Observer of the node sequence: called once per counted node with
/// (cell, piece, rotation), in search order.
pub trait NodeHook: Default {
    fn node(&mut self, cell: usize, piece: u16, rot: u8);
}

/// No-op hook (timed runs).
#[derive(Default)]
pub struct NoHook;

impl NodeHook for NoHook {
    #[inline(always)]
    fn node(&mut self, _: usize, _: u16, _: u8) {}
}

/// 64-bit FNV-1a over (cell u16 LE, piece u16 LE, rot u8) per node, plus the
/// number of nodes hashed.
pub struct HashHook {
    pub h: u64,
    pub n: u64,
}

impl Default for HashHook {
    fn default() -> Self {
        Self {
            h: 0xcbf2_9ce4_8422_2325,
            n: 0,
        }
    }
}

impl NodeHook for HashHook {
    #[inline]
    fn node(&mut self, cell: usize, piece: u16, rot: u8) {
        let c = cell as u16;
        for b in [
            c as u8,
            (c >> 8) as u8,
            piece as u8,
            (piece >> 8) as u8,
            rot,
        ] {
            self.h ^= u64::from(b);
            self.h = self.h.wrapping_mul(0x0100_0000_01b3);
        }
        self.n += 1;
    }
}

// ---------------------------------------------------------------------------
// Reference: verbatim copy of A16's Rule::Mrv search plus the hook call.
// ---------------------------------------------------------------------------

pub struct RefSolver<H: NodeHook> {
    pub guard: Duration,
    pub hook: H,
}

enum Step {
    Found,
    Failed,
    Cap,
}

struct RefSearch<'a, H: NodeHook> {
    m: &'a Model,
    st: Stats,
    start: Instant,
    guard: Duration,
    board: Vec<u64>,
    hook: &'a mut H,
}

impl<H: NodeHook> RefSearch<'_, H> {
    fn choose(&mut self, d: &[u64], assigned: &[bool]) -> usize {
        let m = self.m;
        let free: Vec<usize> = (0..m.cells).filter(|&c| !assigned[c]).collect();
        if let Some(&c) = free.iter().find(|&&c| m.count(d, c) == 1) {
            return c;
        }
        *free
            .iter()
            .min_by_key(|&&c| (m.count(d, c), c))
            .expect("free cell")
    }

    fn dfs(
        &mut self,
        d: &[u64],
        assigned: &mut Vec<bool>,
        left: usize,
        counter: &mut NodeCounter,
    ) -> Step {
        if left == 0 {
            self.board = d.to_vec();
            return Step::Found;
        }
        let c = self.choose(d, assigned);
        let mut nd = d.to_vec();
        for o in options(self.m, d, c) {
            if !counter.try_assign() || self.start.elapsed() > self.guard {
                return Step::Cap;
            }
            self.hook.node(c, self.m.opt_piece[o], self.m.opt_rot[o]);
            nd.copy_from_slice(d);
            assigned[c] = true;
            if assign(self.m, &mut nd, c, o, &mut self.st) {
                match self.dfs(&nd, assigned, left - 1, counter) {
                    Step::Failed => {}
                    done => return done,
                }
            }
            assigned[c] = false;
            counter.backtracks += 1;
        }
        Step::Failed
    }
}

impl<H: NodeHook> LadderSolver for RefSolver<H> {
    fn solve(&mut self, puzzle: &Puzzle, counter: &mut NodeCounter) -> Outcome {
        let start = Instant::now();
        let m = Model::new(puzzle);
        if !m.root_ok {
            return Outcome::Exhausted;
        }
        let mut search = RefSearch {
            m: &m,
            st: Stats { propagations: 1 },
            start,
            guard: self.guard,
            board: Vec::new(),
            hook: &mut self.hook,
        };
        let mut assigned = vec![false; m.cells];
        let root = m.root().to_vec();
        let out = search.dfs(&root, &mut assigned, m.cells, counter);
        counter.propagation = search.st.propagations;
        match out {
            Step::Found => {
                let board = (0..m.cells)
                    .map(|c| {
                        let o = options(&m, &search.board, c)[0];
                        i32::from(m.opt_piece[o]) * 4 + i32::from(m.opt_rot[o])
                    })
                    .collect();
                Outcome::Solved(board)
            }
            Step::Failed => Outcome::Exhausted,
            Step::Cap => Outcome::CapReached,
        }
    }
}

// ---------------------------------------------------------------------------
// Fast solver.
// ---------------------------------------------------------------------------

pub struct FastSolver<H: NodeHook> {
    pub guard: Duration,
    pub hook: H,
}

const NONE: u32 = u32::MAX;
/// Domains with at most this many options compute offered colours by walking
/// their set bits; larger ones test each colour mask.
const SMALL: u32 = 12;

/// Gather the low bit of each nibble of `x` (already folded) into 16 bits.
#[inline(always)]
fn nib(x: u64) -> u64 {
    let mut y = x | (x >> 1);
    y |= y >> 2;
    y &= 0x1111_1111_1111_1111;
    y = (y | (y >> 3)) & 0x0303_0303_0303_0303;
    y = (y | (y >> 6)) & 0x000F_000F_000F_000F;
    y = (y | (y >> 12)) & 0x0000_00FF_0000_00FF;
    (y | (y >> 24)) & 0xFFFF
}

struct Fast<'a, const W: usize, H: NodeHook> {
    cells: usize,
    nc: usize,
    /// side_mask[side * nc + col]: bits with colour col on side.
    side_mask: Vec<[u64; W]>,
    /// colour of each bit on each side.
    bit_col: Vec<[u8; 4]>,
    bit_piece: Vec<u16>,
    bit_rot: Vec<u8>,
    nb: Vec<[u32; 4]>,
    all_pieces: u128,
    /// (cells + 1) levels of `cells` domains.
    states: Vec<[u64; W]>,
    assigned: u128,
    queue: Vec<u32>,
    pres: Vec<u128>,
    propagations: u64,
    start: Instant,
    guard: Duration,
    board_level: usize,
    hook: &'a mut H,
}

impl<const W: usize, H: NodeHook> Fast<'_, W, H> {
    #[inline(always)]
    fn count(x: &[u64; W]) -> u32 {
        x.iter().map(|w| w.count_ones()).sum()
    }

    #[inline(always)]
    fn offered(&self, dom: &[u64; W], side: usize) -> u32 {
        let mut mask = 0u32;
        if Self::count(dom) <= SMALL {
            for (k, &w) in dom.iter().enumerate() {
                let mut x = w;
                while x != 0 {
                    let b = k * 64 + x.trailing_zeros() as usize;
                    mask |= 1 << self.bit_col[b][side];
                    x &= x - 1;
                }
            }
        } else {
            let base = side * self.nc;
            for col in 0..self.nc {
                let sm = &self.side_mask[base + col];
                let mut hit = 0u64;
                for k in 0..W {
                    hit |= dom[k] & sm[k];
                }
                if hit != 0 {
                    mask |= 1 << col;
                }
            }
        }
        mask
    }

    #[inline(always)]
    fn presence(dom: &[u64; W]) -> u128 {
        let mut p = 0u128;
        for (k, &w) in dom.iter().enumerate() {
            if w != 0 {
                p |= u128::from(nib(w)) << (16 * k);
            }
        }
        p
    }

    #[inline(always)]
    fn push(&mut self, inq: &mut u128, c: usize) {
        if *inq >> c & 1 == 0 {
            *inq |= 1u128 << c;
            self.queue.push(c as u32);
        }
    }

    /// Propagate level `lv` to the common fixpoint of A3's rules starting
    /// from cell `start`. Returns false on contradiction.
    fn propagate(&mut self, lv: usize, start: usize) -> bool {
        self.propagations += 1;
        let cells = self.cells;
        let base = lv * cells;
        self.queue.clear();
        let mut inq = 0u128;
        self.push(&mut inq, start);
        loop {
            while let Some(b) = self.queue.pop() {
                let b = b as usize;
                inq &= !(1u128 << b);
                let db_dom = self.states[base + b];
                for db in 0..4 {
                    let a = self.nb[b][db];
                    if a == NONE {
                        continue;
                    }
                    let a = a as usize;
                    let da = (db + 2) & 3;
                    let mut mask = self.offered(&db_dom, db);
                    let mut allowed = [0u64; W];
                    let sbase = da * self.nc;
                    while mask != 0 {
                        let col = mask.trailing_zeros() as usize;
                        mask &= mask - 1;
                        let sm = &self.side_mask[sbase + col];
                        for k in 0..W {
                            allowed[k] |= sm[k];
                        }
                    }
                    let da_dom = &mut self.states[base + a];
                    let mut changed = false;
                    let mut nz = 0u64;
                    for k in 0..W {
                        let old = da_dom[k];
                        let new = old & allowed[k];
                        changed |= new != old;
                        da_dom[k] = new;
                        nz |= new;
                    }
                    if nz == 0 {
                        return false;
                    }
                    if changed {
                        self.push(&mut inq, a);
                    }
                }
            }
            // All-different pass (piece singles, hidden singles, piece with no cell).
            let (mut once, mut twice) = (0u128, 0u128);
            for c in 0..cells {
                let p = Self::presence(&self.states[base + c]);
                self.pres[c] = p;
                twice |= once & p;
                once |= p;
            }
            if once != self.all_pieces {
                return false;
            }
            let mut any = false;
            // Piece singles whose piece still appears in another cell.
            for c in 0..cells {
                let p = self.pres[c];
                if p.count_ones() != 1 || p & twice == 0 {
                    continue;
                }
                let pi = p.trailing_zeros() as usize;
                let (k, m) = (pi >> 4, 0xFu64 << ((pi & 15) * 4));
                for c2 in 0..cells {
                    if c2 == c {
                        continue;
                    }
                    let dom = &mut self.states[base + c2];
                    if dom[k] & m != 0 {
                        dom[k] &= !m;
                        if dom.iter().all(|&x| x == 0) {
                            return false;
                        }
                        any = true;
                        self.push(&mut inq, c2);
                    }
                }
            }
            // Hidden singles: pieces present in exactly one cell.
            let hs = once & !twice;
            if hs != 0 {
                for c in 0..cells {
                    let h = self.pres[c] & hs;
                    if h == 0 {
                        continue;
                    }
                    if h.count_ones() > 1 {
                        return false;
                    }
                    let pi = h.trailing_zeros() as usize;
                    let (k, m) = (pi >> 4, 0xFu64 << ((pi & 15) * 4));
                    let dom = &mut self.states[base + c];
                    let mut changed = false;
                    for (j, x) in dom.iter_mut().enumerate() {
                        let new = if j == k { *x & m } else { 0 };
                        changed |= new != *x;
                        *x = new;
                    }
                    if changed {
                        if dom.iter().all(|&x| x == 0) {
                            return false;
                        }
                        any = true;
                        self.push(&mut inq, c);
                    }
                }
            }
            if !any {
                return true;
            }
        }
    }

    /// Free cell minimising (|D|, index).
    #[inline]
    fn choose(&self, lv: usize) -> usize {
        let base = lv * self.cells;
        let mut best = usize::MAX;
        let mut best_cnt = u32::MAX;
        for c in 0..self.cells {
            if self.assigned >> c & 1 != 0 {
                continue;
            }
            let k = Self::count(&self.states[base + c]);
            if k < best_cnt {
                best_cnt = k;
                best = c;
                if k == 1 {
                    break;
                }
            }
        }
        best
    }

    fn dfs(&mut self, lv: usize, left: usize, counter: &mut NodeCounter) -> Step {
        if left == 0 {
            self.board_level = lv;
            return Step::Found;
        }
        let cells = self.cells;
        let c = self.choose(lv);
        let dom = self.states[lv * cells + c];
        for (k, &w) in dom.iter().enumerate() {
            let mut x = w;
            while x != 0 {
                let b = k * 64 + x.trailing_zeros() as usize;
                x &= x - 1;
                if !counter.try_assign() || self.start.elapsed() > self.guard {
                    return Step::Cap;
                }
                self.hook.node(c, self.bit_piece[b], self.bit_rot[b]);
                let (lo, hi) = self.states.split_at_mut((lv + 1) * cells);
                hi[..cells].copy_from_slice(&lo[lv * cells..]);
                self.assigned |= 1u128 << c;
                let mut single = [0u64; W];
                single[k] = 1u64 << (b & 63);
                hi[c] = single;
                if self.propagate(lv + 1, c) {
                    match self.dfs(lv + 1, left - 1, counter) {
                        Step::Failed => {}
                        done => return done,
                    }
                }
                self.assigned &= !(1u128 << c);
                counter.backtracks += 1;
            }
        }
        Step::Failed
    }
}

fn solve_w<const W: usize, H: NodeHook>(
    m: &Model,
    guard: Duration,
    start: Instant,
    hook: &mut H,
    counter: &mut NodeCounter,
) -> Outcome {
    let cells = m.cells;
    let pieces = m.piece_range.len();
    let nc = m.num_colors;
    let nbits = W * 64;
    // Bit of model option o: 4 * piece + rank of o within its piece.
    let bit_of: Vec<usize> = (0..m.opt_piece.len())
        .map(|o| {
            let p = m.opt_piece[o] as usize;
            4 * p + (o - m.piece_range[p].0)
        })
        .collect();
    let mut side_mask = vec![[0u64; W]; 4 * nc];
    let mut bit_col = vec![[0u8; 4]; nbits];
    let mut bit_piece = vec![0u16; nbits];
    let mut bit_rot = vec![0u8; nbits];
    for (o, &b) in bit_of.iter().enumerate() {
        let e = m.opt_edges[o];
        for s in 0..4 {
            side_mask[s * nc + e[s] as usize][b >> 6] |= 1u64 << (b & 63);
        }
        bit_col[b] = e;
        bit_piece[b] = m.opt_piece[o];
        bit_rot[b] = m.opt_rot[o];
    }
    let nb = m
        .neighbour
        .iter()
        .map(|r| r.map(|x| x.map_or(NONE, |v| v as u32)))
        .collect();
    let mut states = vec![[0u64; W]; (cells + 1) * cells];
    let root = m.root();
    for c in 0..cells {
        for o in options(m, root, c) {
            let b = bit_of[o];
            states[c][b >> 6] |= 1u64 << (b & 63);
        }
    }
    let all_pieces = if pieces == 128 {
        u128::MAX
    } else {
        (1u128 << pieces) - 1
    };
    let mut f = Fast::<W, H> {
        cells,
        nc,
        side_mask,
        bit_col,
        bit_piece,
        bit_rot,
        nb,
        all_pieces,
        states,
        assigned: 0,
        queue: Vec::with_capacity(4 * cells + 8),
        pres: vec![0; cells],
        propagations: 1,
        start,
        guard,
        board_level: 0,
        hook,
    };
    let out = f.dfs(0, cells, counter);
    counter.propagation = f.propagations;
    match out {
        Step::Found => {
            let base = f.board_level * cells;
            let board = (0..cells)
                .map(|c| {
                    let d = &f.states[base + c];
                    let k = d.iter().position(|&x| x != 0).expect("assigned cell");
                    let b = k * 64 + d[k].trailing_zeros() as usize;
                    i32::from(f.bit_piece[b]) * 4 + i32::from(f.bit_rot[b])
                })
                .collect();
            Outcome::Solved(board)
        }
        Step::Failed => Outcome::Exhausted,
        Step::Cap => Outcome::CapReached,
    }
}

impl<H: NodeHook> LadderSolver for FastSolver<H> {
    fn solve(&mut self, puzzle: &Puzzle, counter: &mut NodeCounter) -> Outcome {
        let start = Instant::now();
        let m = Model::new(puzzle);
        if !m.root_ok {
            return Outcome::Exhausted;
        }
        assert!(
            m.cells <= 128,
            "FastSolver supports at most 128 cells (n <= 11)"
        );
        let words = (4 * m.cells).div_ceil(64);
        let (g, h) = (self.guard, &mut self.hook);
        match words {
            1 => solve_w::<1, H>(&m, g, start, h, counter),
            2 => solve_w::<2, H>(&m, g, start, h, counter),
            3 => solve_w::<3, H>(&m, g, start, h, counter),
            4 => solve_w::<4, H>(&m, g, start, h, counter),
            5 => solve_w::<5, H>(&m, g, start, h, counter),
            6 => solve_w::<6, H>(&m, g, start, h, counter),
            7 => solve_w::<7, H>(&m, g, start, h, counter),
            8 => solve_w::<8, H>(&m, g, start, h, counter),
            _ => unreachable!(),
        }
    }
}
