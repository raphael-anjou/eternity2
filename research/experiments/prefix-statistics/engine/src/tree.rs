//! The exact (no break) search tree for any fixed cell order, counted exactly
//! by depth-first enumeration or estimated by population SMC. Both use the one
//! child function `Tree::for_each_child`, so the validation gate tests the
//! code path the real runs use.
//!
//! A placement on cell p is valid iff: every side facing the rim is grey (0);
//! every other side is not grey; every side facing a cell that is already
//! placed or fixed matches that cell's facing side. Fixed cells (clues, a
//! stand-in frame) are on the board from the start and are not nodes.

use crate::puzzle::{rotate, Edges, Puzzle, Rng};

const NONE: u8 = 255;

#[derive(Clone, Copy)]
enum Side {
    Rim,
    /// Neighbour cell, known at this step (placed earlier or fixed).
    Known(u16),
    /// Neighbour cell not yet on the board: side must be non-grey.
    Open,
}

struct Step {
    pos: usize,
    sides: [Side; 4],
    /// Up to two sides used as the lookup key (Rim or Known), 255 = unused.
    k1: u8,
    k2: u8,
}

pub struct Tree {
    pub cells: usize,
    pub nfree: usize,
    ncol: usize,
    steps: Vec<Step>,
    /// Oriented pieces: (pid, edges).
    oriented: Vec<(u16, Edges)>,
    /// single[s][c] -> indices into oriented
    single: Vec<Vec<u32>>,
    /// pair[(s1*4+s2)][c1*ncol+c2] -> indices into oriented
    pair: Vec<Vec<Vec<u32>>>,
    /// Edges of fixed cells, colours of the initial board.
    init_board: Vec<u8>,
    init_used: [u64; 4],
}

fn nb(n: usize, pos: usize, s: usize) -> Option<usize> {
    let (r, c) = (pos / n, pos % n);
    match s {
        0 => (r > 0).then(|| pos - n),
        1 => (c + 1 < n).then(|| pos + 1),
        2 => (r + 1 < n).then(|| pos + n),
        _ => (c > 0).then(|| pos - 1),
    }
}

impl Tree {
    /// `order`: the free cells in scan order (must not contain fixed cells).
    /// `fixed`: (pos, piece, rot) on the board from the start.
    pub fn new(p: &Puzzle, order: &[usize], fixed: &[(usize, u16, u8)]) -> Tree {
        let n = p.n;
        let cells = n * n;
        let ncol = p
            .pieces
            .iter()
            .flat_map(|e| e.iter().copied())
            .max()
            .unwrap() as usize
            + 1;
        let mut init_board = vec![NONE; cells * 4];
        let mut init_used = [0u64; 4];
        let mut known = vec![false; cells];
        for &(pos, pid, r) in fixed {
            let e = rotate(p.pieces[pid as usize], r);
            init_board[pos * 4..pos * 4 + 4].copy_from_slice(&e);
            init_used[pid as usize / 64] |= 1 << (pid % 64);
            known[pos] = true;
        }
        let mut steps = Vec::with_capacity(order.len());
        for &pos in order {
            assert!(!known[pos], "order contains a fixed or repeated cell");
            let mut sides = [Side::Open; 4];
            for s in 0..4 {
                sides[s] = match nb(n, pos, s) {
                    None => Side::Rim,
                    Some(q) if known[q] => Side::Known(q as u16),
                    Some(_) => Side::Open,
                };
            }
            let keyable: Vec<u8> = (0..4u8)
                .filter(|&s| !matches!(sides[s as usize], Side::Open))
                .collect();
            let (k1, k2) = match keyable.len() {
                0 => (NONE, NONE),
                1 => (keyable[0], NONE),
                _ => (keyable[0], keyable[1]),
            };
            steps.push(Step { pos, sides, k1, k2 });
            known[pos] = true;
        }
        let mut oriented = Vec::new();
        for (pid, &e) in p.pieces.iter().enumerate() {
            if init_used[pid / 64] >> (pid % 64) & 1 == 1 {
                continue;
            }
            let mut seen: Vec<Edges> = Vec::new();
            for r in 0..4 {
                let re = rotate(e, r);
                if !seen.contains(&re) {
                    seen.push(re);
                    oriented.push((pid as u16, re));
                }
            }
        }
        let mut single = vec![Vec::new(); 4 * ncol];
        let mut pair = vec![vec![Vec::new(); ncol * ncol]; 16];
        for (i, &(_, e)) in oriented.iter().enumerate() {
            for s in 0..4 {
                single[s * ncol + e[s] as usize].push(i as u32);
                for s2 in s + 1..4 {
                    pair[s * 4 + s2][e[s] as usize * ncol + e[s2] as usize].push(i as u32);
                }
            }
        }
        Tree {
            cells,
            nfree: order.len(),
            ncol,
            steps,
            oriented,
            single,
            pair,
            init_board,
            init_used,
        }
    }

    pub fn root(&self) -> State {
        State {
            board: self.init_board.clone(),
            used: self.init_used,
            cell: vec![u16::MAX; self.cells],
        }
    }

    #[inline]
    fn want(&self, st: &State, side: Side, s: usize) -> u8 {
        match side {
            Side::Rim => 0,
            Side::Known(q) => st.board[q as usize * 4 + (s + 2) % 4],
            Side::Open => NONE,
        }
    }

    /// Calls f(index into oriented) for every valid child of `st` at step k.
    #[inline]
    pub fn for_each_child<F: FnMut(u32)>(&self, st: &State, k: usize, mut f: F) {
        let step = &self.steps[k];
        let list: &[u32] = if step.k1 == NONE {
            unreachable!("first cell must touch the rim")
        } else if step.k2 == NONE {
            let s = step.k1 as usize;
            &self.single[s * self.ncol + self.want(st, step.sides[s], s) as usize]
        } else {
            let (a, b) = (step.k1 as usize, step.k2 as usize);
            let ca = self.want(st, step.sides[a], a) as usize;
            let cb = self.want(st, step.sides[b], b) as usize;
            &self.pair[a * 4 + b][ca * self.ncol + cb]
        };
        'cand: for &i in list {
            let (pid, e) = self.oriented[i as usize];
            if st.used[pid as usize / 64] >> (pid % 64) & 1 == 1 {
                continue;
            }
            for s in 0..4 {
                let ok = match step.sides[s] {
                    Side::Rim => e[s] == 0,
                    Side::Open => e[s] != 0,
                    Side::Known(_) => e[s] == self.want(st, step.sides[s], s),
                };
                if !ok {
                    continue 'cand;
                }
            }
            f(i);
        }
    }

    #[inline]
    pub fn apply(&self, st: &mut State, k: usize, i: u32) {
        let (pid, e) = self.oriented[i as usize];
        let pos = self.steps[k].pos;
        st.board[pos * 4..pos * 4 + 4].copy_from_slice(&e);
        st.used[pid as usize / 64] |= 1 << (pid % 64);
        st.cell[pos] = pid;
    }

    #[inline]
    pub fn undo(&self, st: &mut State, k: usize, i: u32) {
        let (pid, _) = self.oriented[i as usize];
        let pos = self.steps[k].pos;
        st.board[pos * 4..pos * 4 + 4].copy_from_slice(&[NONE; 4]);
        st.used[pid as usize / 64] &= !(1 << (pid % 64));
        st.cell[pos] = u16::MAX;
    }

    /// Exact widths W_1..W_nfree by depth-first enumeration. Stops when the
    /// node count reaches `cap`; returns (widths, complete).
    pub fn enumerate(&self, cap: u64) -> (Vec<u64>, bool) {
        let mut w = vec![0u64; self.nfree + 1];
        let mut st = self.root();
        let mut nodes = 0u64;
        let complete = self.dfs(&mut st, 0, &mut w, &mut nodes, cap);
        w[0] = 1;
        (w, complete)
    }

    fn dfs(&self, st: &mut State, k: usize, w: &mut [u64], nodes: &mut u64, cap: u64) -> bool {
        if k == self.nfree {
            return true;
        }
        let mut kids: Vec<u32> = Vec::new();
        self.for_each_child(st, k, |i| kids.push(i));
        for i in kids {
            if *nodes >= cap {
                return false;
            }
            *nodes += 1;
            w[k + 1] += 1;
            self.apply(st, k, i);
            let ok = self.dfs(st, k + 1, w, nodes, cap);
            self.undo(st, k, i);
            if !ok {
                return false;
            }
        }
        true
    }
}

#[derive(Clone)]
pub struct State {
    pub board: Vec<u8>,
    pub used: [u64; 4],
    /// prefix-statistics addition: tile id per cell (u16::MAX = empty or
    /// fixed). Written by apply/undo only; never read by the search.
    pub cell: Vec<u16>,
}

pub const ANC_DEPTHS: [usize; 4] = [16, 32, 64, 96];
pub const DIAG_DEPTHS: [usize; 3] = [128, 158, 175];

pub struct Level {
    pub e: u64,
    pub m: u64,
    pub zero_frac: f64,
    pub mean_ext: f64,
    pub var_ext: f64,
}

pub struct SmcRun {
    /// est[k] = estimated width after k free placements, est[0] = 1.
    pub est: Vec<f64>,
    pub levels: Vec<Level>,
    /// (depth, anc_depth, distinct ancestors, sum of squared family shares)
    pub diversity: Vec<(usize, usize, usize, f64)>,
}

/// One SMC repetition with population cap `np`.
pub fn smc(t: &Tree, np: usize, seed: u64) -> SmcRun {
    smc_with(t, np, seed, |_, _, _, _| {})
}

/// prefix-statistics addition: the same repetition, calling
/// `cb(k, pop, parent, est_k)` with the population at free depth k before it
/// is extended (and at k = nfree if the population survives), the index of
/// each particle's parent in the previous population (empty at k = 0) and the
/// width estimate at k. The callback cannot touch the RNG or the population,
/// so the run is identical to `smc` for the same seed.
pub fn smc_with<F: FnMut(usize, &[(State, [u32; 4])], &[u32], f64)>(
    t: &Tree,
    np: usize,
    seed: u64,
    mut cb: F,
) -> SmcRun {
    let mut rng = Rng::new(seed);
    let mut pop: Vec<(State, [u32; 4])> = vec![(t.root(), [0; 4])];
    let mut est = vec![0.0f64; t.nfree + 1];
    est[0] = 1.0;
    let mut levels = Vec::with_capacity(t.nfree);
    let mut diversity = Vec::new();
    let mut counts: Vec<u32> = Vec::new();
    let mut survived = true;
    let mut parent: Vec<u32> = Vec::new();
    for k in 0..t.nfree {
        cb(k, &pop, &parent, est[k]);
        counts.clear();
        let mut e: u64 = 0;
        let (mut s1, mut s2, mut zeros) = (0f64, 0f64, 0u64);
        for (st, _) in &pop {
            let mut c = 0u32;
            t.for_each_child(st, k, |_| c += 1);
            counts.push(c);
            e += c as u64;
            s1 += c as f64;
            s2 += (c as f64) * (c as f64);
            if c == 0 {
                zeros += 1;
            }
        }
        let m = pop.len() as u64;
        let mean = s1 / m as f64;
        levels.push(Level {
            e,
            m,
            zero_frac: zeros as f64 / m as f64,
            mean_ext: mean,
            var_ext: s2 / m as f64 - mean * mean,
        });
        est[k + 1] = est[k] * e as f64 / m as f64;
        if e == 0 {
            survived = false;
            break;
        }
        // choose which children survive: all, or N distinct uniformly
        let mut chosen: Vec<u64> = if e as usize <= np {
            (0..e).collect()
        } else {
            // Floyd's algorithm for a uniform N-subset of 0..e
            let mut set = std::collections::HashSet::with_capacity(np * 2);
            let mut out = Vec::with_capacity(np);
            for j in (e - np as u64)..e {
                let r = (rng.next_u64() % (j + 1)) as u64;
                let v = if set.contains(&r) { j } else { r };
                set.insert(v);
                out.push(v);
            }
            out.sort_unstable();
            out
        };
        chosen.sort_unstable();
        let free_depth = k + 1;
        let anc_slot = ANC_DEPTHS.iter().position(|&d| d == free_depth);
        let mut next: Vec<(State, [u32; 4])> = Vec::with_capacity(chosen.len());
        let mut next_parent: Vec<u32> = Vec::with_capacity(chosen.len());
        let mut ci = 0usize;
        let mut base = 0u64;
        for (j, (st, anc)) in pop.iter().enumerate() {
            let c = counts[j] as u64;
            if ci < chosen.len() && chosen[ci] < base + c {
                let mut off = 0u64;
                let mut kids: Vec<u32> = Vec::with_capacity(c as usize);
                t.for_each_child(st, k, |i| kids.push(i));
                for &i in &kids {
                    while ci < chosen.len() && chosen[ci] == base + off {
                        let mut ns = st.clone();
                        t.apply(&mut ns, k, i);
                        let mut a = *anc;
                        if let Some(sl) = anc_slot {
                            a[sl] = next.len() as u32;
                        }
                        next.push((ns, a));
                        next_parent.push(j as u32);
                        ci += 1;
                    }
                    off += 1;
                }
            }
            base += c;
        }
        pop = next;
        parent = next_parent;
        if DIAG_DEPTHS.contains(&free_depth) {
            for (sl, &ad) in ANC_DEPTHS.iter().enumerate() {
                if ad >= free_depth {
                    continue;
                }
                let mut ids: Vec<u32> = pop.iter().map(|(_, a)| a[sl]).collect();
                ids.sort_unstable();
                let tot = ids.len() as f64;
                let (mut distinct, mut ss, mut run) = (0usize, 0f64, 0usize);
                for i in 0..ids.len() {
                    run += 1;
                    if i + 1 == ids.len() || ids[i + 1] != ids[i] {
                        distinct += 1;
                        ss += (run as f64 / tot).powi(2);
                        run = 0;
                    }
                }
                diversity.push((free_depth, ad, distinct, ss));
            }
        }
    }
    if survived {
        cb(t.nfree, &pop, &parent, est[t.nfree]);
    }
    SmcRun {
        est,
        levels,
        diversity,
    }
}

// ---- cell orders (pos = row * n + col, row = CSV y) ----

/// Rows from the last (n-1) up to 0, columns left to right.
pub fn rows_bottom_up(n: usize) -> Vec<usize> {
    (0..n)
        .rev()
        .flat_map(|r| (0..n).map(move |c| r * n + c))
        .collect()
}

/// Clockwise inward spiral from (0,0): ring 0 (row 0 left to right, column
/// n-1 down, row n-1 right to left, column 0 up), then ring 1, ...
pub fn spiral(n: usize) -> Vec<usize> {
    let mut out = Vec::with_capacity(n * n);
    let (mut top, mut left, mut bot, mut right) = (0i64, 0i64, n as i64 - 1, n as i64 - 1);
    let n_ = n as i64;
    while top <= bot && left <= right {
        for c in left..=right {
            out.push((top * n_ + c) as usize);
        }
        for r in top + 1..=bot {
            out.push((r * n_ + right) as usize);
        }
        if top < bot {
            for c in (left..right).rev() {
                out.push((bot * n_ + c) as usize);
            }
        }
        if left < right {
            for r in (top + 1..bot).rev() {
                out.push((r * n_ + left) as usize);
            }
        }
        top += 1;
        left += 1;
        bot -= 1;
        right -= 1;
    }
    out
}

pub fn ring(n: usize) -> Vec<usize> {
    spiral(n)[..4 * (n - 1)].to_vec()
}

// ---- prefix-statistics additions (not in Track 10) ----

impl Tree {
    pub fn ncol(&self) -> usize {
        self.ncol
    }
    pub fn pos(&self, k: usize) -> usize {
        self.steps[k].pos
    }
    pub fn oriented(&self, i: u32) -> (u16, Edges) {
        self.oriented[i as usize]
    }
    /// Index into `oriented` of piece `pid` in orientation `e`.
    pub fn find(&self, pid: u16, e: Edges) -> Option<u32> {
        self.oriented
            .iter()
            .position(|&(p, f)| p == pid && f == e)
            .map(|i| i as u32)
    }
    pub fn child_count(&self, st: &State, k: usize) -> u32 {
        let mut c = 0;
        if k < self.nfree {
            self.for_each_child(st, k, |_| c += 1);
        }
        c
    }
    /// Number of descendants of `st` (at free depth k) at free depth k + l,
    /// by depth-first enumeration, stopping at `cap` nodes visited.
    /// Returns (count at depth k+l, nodes visited, capped).
    pub fn descendants(&self, st: &mut State, k: usize, l: usize, cap: u64) -> (u64, u64, bool) {
        let target = (k + l).min(self.nfree);
        let mut nodes = 0u64;
        let mut count = 0u64;
        let ok = self.desc_rec(st, k, target, &mut count, &mut nodes, cap);
        (count, nodes, !ok)
    }
    /// A4: does `st` (free depth k) have a descendant at depth k + l?
    /// Depth-first, stops at the first one. None if `cap` nodes were visited
    /// without an answer.
    pub fn exists_descendant(&self, st: &mut State, k: usize, l: usize, cap: u64) -> Option<bool> {
        let target = (k + l).min(self.nfree);
        let mut nodes = 0u64;
        self.exists_rec(st, k, target, &mut nodes, cap)
    }
    fn exists_rec(&self, st: &mut State, k: usize, target: usize, nodes: &mut u64, cap: u64) -> Option<bool> {
        if k == target {
            return Some(true);
        }
        let mut kids: Vec<u32> = Vec::new();
        self.for_each_child(st, k, |i| kids.push(i));
        for i in kids {
            if *nodes >= cap {
                return None;
            }
            *nodes += 1;
            self.apply(st, k, i);
            let r = self.exists_rec(st, k + 1, target, nodes, cap);
            self.undo(st, k, i);
            match r {
                Some(true) => return Some(true),
                None => return None,
                Some(false) => {}
            }
        }
        Some(false)
    }

    fn desc_rec(&self, st: &mut State, k: usize, target: usize, count: &mut u64, nodes: &mut u64, cap: u64) -> bool {
        if k == target {
            *count += 1;
            return true;
        }
        let mut kids: Vec<u32> = Vec::new();
        self.for_each_child(st, k, |i| kids.push(i));
        for i in kids {
            if *nodes >= cap {
                return false;
            }
            *nodes += 1;
            self.apply(st, k, i);
            let ok = self.desc_rec(st, k + 1, target, count, nodes, cap);
            self.undo(st, k, i);
            if !ok {
                return false;
            }
        }
        true
    }
}

/// The four row/column scans, as images of `rows_bottom_up` under the board's
/// rotations: dir 0 = rows from the last up, columns left to right (Track
/// 10's order); dir 1 = rotated by a quarter turn, and so on.
pub fn scan(n: usize, dir: usize) -> Vec<usize> {
    let rot = |pos: usize| -> usize {
        let (r, c) = (pos / n, pos % n);
        // clockwise quarter turn of the board: (r, c) -> (c, n-1-r)
        let (r2, c2) = (c, n - 1 - r);
        r2 * n + c2
    };
    rows_bottom_up(n)
        .into_iter()
        .map(|mut p| {
            for _ in 0..(dir & 3) {
                p = rot(p);
            }
            p
        })
        .collect()
}
