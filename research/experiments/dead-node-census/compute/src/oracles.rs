//! The census oracles (PREREG.md "Oracles"). Each returns whether it proves
//! the node dead plus its own work count. None of them mutates the state.

use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

use sat_backdoors::{Model, Stats};

use crate::{bits, Ctx, Rng, State};

/// A state with the per-node data every oracle uses.
pub struct View<'a> {
    pub cx: &'a Ctx<'a>,
    pub s: &'a State,
    /// Option at each assigned cell, -1 elsewhere.
    pub opt: Vec<i32>,
    pub used: u128,
}

impl<'a> View<'a> {
    #[must_use]
    pub fn new(cx: &'a Ctx<'a>, s: &'a State) -> Self {
        let m = cx.m;
        let opt = (0..m.cells)
            .map(|c| {
                if s.assigned[c] {
                    s.opt_at(m, c) as i32
                } else {
                    -1
                }
            })
            .collect();
        Self {
            cx,
            s,
            opt,
            used: s.used(m),
        }
    }
    fn m(&self) -> &Model {
        self.cx.m
    }
    /// Colour the assigned neighbour of `c` in direction `dir` shows towards `c`.
    fn facing(&self, c: usize, dir: usize) -> Option<u8> {
        let nb = self.m().neighbour[c][dir]?;
        if self.s.assigned[nb] {
            Some(self.m().opt_edges[self.opt[nb] as usize][(dir + 2) & 3])
        } else {
            None
        }
    }
    /// B-pool / C candidates: options of unused pieces that fit the rim and
    /// every assigned neighbour.
    #[must_use]
    pub fn pool_cands(&self, c: usize) -> Vec<u32> {
        let m = self.m();
        let w = m.words;
        let face: [Option<u8>; 4] = [0, 1, 2, 3].map(|d| self.facing(c, d));
        bits(&self.cx.unary[c * w..(c + 1) * w])
            .filter(|&o| {
                self.used >> m.opt_piece[o] & 1 == 0
                    && (0..4).all(|d| face[d].map_or(true, |col| m.opt_edges[o][d] == col))
            })
            .map(|o| o as u32)
            .collect()
    }
    #[must_use]
    pub fn dom_cands(&self, c: usize) -> Vec<u32> {
        bits(self.m().dom(&self.s.d, c)).map(|o| o as u32).collect()
    }
}

// ---------------------------------------------------------------- exact

/// Exact feasibility of filling `cells` from `cands` (same order), all
/// internal edges matching, pieces distinct. None when the budget ran out.
pub fn exact(
    m: &Model,
    cells: &[usize],
    cands: &[Vec<u32>],
    budget: u64,
    tests: &mut u64,
) -> Option<bool> {
    let mut pos = vec![-1i32; m.cells];
    for (i, &c) in cells.iter().enumerate() {
        pos[c] = i as i32;
    }
    let mut chosen = vec![-1i32; cells.len()];
    let mut used = 0u128;
    exact_rec(
        m,
        cells,
        cands,
        &pos,
        &mut chosen,
        &mut used,
        0,
        budget,
        tests,
    )
}

#[allow(clippy::too_many_arguments)]
fn exact_rec(
    m: &Model,
    cells: &[usize],
    cands: &[Vec<u32>],
    pos: &[i32],
    chosen: &mut [i32],
    used: &mut u128,
    filled: usize,
    budget: u64,
    tests: &mut u64,
) -> Option<bool> {
    if filled == cells.len() {
        return Some(true);
    }
    let ok = |i: usize, o: u32, chosen: &[i32], used: u128| -> bool {
        let o = o as usize;
        if used >> m.opt_piece[o] & 1 != 0 {
            return false;
        }
        let e = m.opt_edges[o];
        for d in 0..4 {
            if let Some(nb) = m.neighbour[cells[i]][d] {
                let p = pos[nb];
                if p >= 0
                    && chosen[p as usize] >= 0
                    && m.opt_edges[chosen[p as usize] as usize][(d + 2) & 3] != e[d]
                {
                    return false;
                }
            }
        }
        true
    };
    let mut best = usize::MAX;
    let mut best_cnt = usize::MAX;
    for i in 0..cells.len() {
        if chosen[i] >= 0 {
            continue;
        }
        let mut cnt = 0;
        for &o in &cands[i] {
            *tests += 1;
            if ok(i, o, chosen, *used) {
                cnt += 1;
            }
        }
        if cnt == 0 {
            return Some(false);
        }
        if cnt < best_cnt {
            best_cnt = cnt;
            best = i;
        }
    }
    if *tests > budget {
        return None;
    }
    for &o in &cands[best] {
        *tests += 1;
        if ok(best, o, chosen, *used) {
            chosen[best] = o as i32;
            *used |= 1u128 << m.opt_piece[o as usize];
            let r = exact_rec(
                m,
                cells,
                cands,
                pos,
                chosen,
                used,
                filled + 1,
                budget,
                tests,
            );
            *used &= !(1u128 << m.opt_piece[o as usize]);
            chosen[best] = -1;
            match r {
                Some(false) => {}
                other => return other,
            }
        }
    }
    Some(false)
}

/// Naive enumeration (row-major order, no heuristics) for the fuzz tests.
pub fn naive(m: &Model, cells: &[usize], cands: &[Vec<u32>], distinct: bool) -> bool {
    fn rec(
        m: &Model,
        cells: &[usize],
        cands: &[Vec<u32>],
        pos: &[i32],
        ch: &mut Vec<i32>,
        distinct: bool,
    ) -> bool {
        let i = ch.len();
        if i == cells.len() {
            return true;
        }
        'o: for &o in &cands[i] {
            let o = o as usize;
            if distinct
                && ch
                    .iter()
                    .any(|&p| m.opt_piece[p as usize] == m.opt_piece[o])
            {
                continue;
            }
            for d in 0..4 {
                if let Some(nb) = m.neighbour[cells[i]][d] {
                    let p = pos[nb];
                    if p >= 0
                        && (p as usize) < i
                        && m.opt_edges[ch[p as usize] as usize][(d + 2) & 3] != m.opt_edges[o][d]
                    {
                        continue 'o;
                    }
                }
            }
            ch.push(o as i32);
            if rec(m, cells, cands, pos, ch, distinct) {
                return true;
            }
            ch.pop();
        }
        false
    }
    let mut pos = vec![-1i32; m.cells];
    for (i, &c) in cells.iter().enumerate() {
        pos[c] = i as i32;
    }
    rec(m, cells, cands, &pos, &mut Vec::new(), distinct)
}

// ---------------------------------------------------------------- keys

fn hash_bytes(b: &[u8]) -> u64 {
    let mut h = DefaultHasher::new();
    h.write(b);
    h.finish()
}

/// View (vx, vy) of a k x k window rotated `rot` quarter turns clockwise ->
/// window coordinates.
fn v2o(mut vx: usize, mut vy: usize, rot: usize, k: usize) -> (usize, usize) {
    for _ in 0..rot {
        let (x, y) = (vy, k - 1 - vx);
        vx = x;
        vy = y;
    }
    (vx, vy)
}

/// Encoded boundary key of window (x0, y0, k) seen under rotation `rot`.
#[must_use]
pub fn window_key(v: &View, x0: usize, y0: usize, k: usize, rot: usize, with_pos: bool) -> Vec<u8> {
    let m = v.m();
    let n = v.cx.n;
    let mut out = vec![k as u8];
    if with_pos {
        out.push(x0 as u8);
        out.push(y0 as u8);
    }
    let cell = |vx: usize, vy: usize| {
        let (x, y) = v2o(vx, vy, rot, k);
        (y0 + y) * n + x0 + x
    };
    for vy in 0..k {
        for vx in 0..k {
            let c = cell(vx, vy);
            if v.s.assigned[c] {
                let e = m.opt_edges[v.opt[c] as usize];
                for s in 0..4 {
                    out.push(e[(s + 4 - rot) % 4]);
                }
            } else {
                out.push(255);
            }
        }
    }
    let mut slots: Vec<(usize, usize, usize)> = Vec::with_capacity(4 * k);
    for vx in 0..k {
        slots.push((vx, 0, 0));
    }
    for vy in 0..k {
        slots.push((k - 1, vy, 1));
    }
    for vx in (0..k).rev() {
        slots.push((vx, k - 1, 2));
    }
    for vy in (0..k).rev() {
        slots.push((0, vy, 3));
    }
    for (vx, vy, s) in slots {
        let c = cell(vx, vy);
        let od = (s + 4 - rot) % 4;
        out.push(match m.neighbour[c][od] {
            None => 254,
            Some(nb) if v.s.assigned[nb] => m.opt_edges[v.opt[nb] as usize][(od + 2) & 3],
            Some(_) => 253,
        });
    }
    out
}

#[must_use]
pub fn raw_key(v: &View, x0: usize, y0: usize, k: usize) -> u64 {
    hash_bytes(&window_key(v, x0, y0, k, 0, true))
}

#[must_use]
pub fn canonical_key(v: &View, x0: usize, y0: usize, k: usize) -> u64 {
    let best = (0..4)
        .map(|r| window_key(v, x0, y0, k, r, false))
        .min()
        .expect("4 rotations");
    hash_bytes(&best)
}

/// True when the window holds an empty cell and some empty cell of it is
/// edge-adjacent to an assigned cell.
#[must_use]
pub fn window_cells(v: &View, x0: usize, y0: usize, k: usize) -> Option<Vec<usize>> {
    let n = v.cx.n;
    let m = v.m();
    let mut empt = Vec::new();
    let mut touch = false;
    for y in y0..y0 + k {
        for x in x0..x0 + k {
            let c = y * n + x;
            if !v.s.assigned[c] {
                empt.push(c);
                if m.neighbour[c].iter().flatten().any(|&nb| v.s.assigned[nb]) {
                    touch = true;
                }
            }
        }
    }
    if empt.is_empty() || !touch {
        None
    } else {
        Some(empt)
    }
}

// ---------------------------------------------------------------- B

pub struct Cavity {
    /// Smallest refuting k, None = inf.
    pub r: Option<usize>,
    pub work: u64,
    pub windows: u64,
    pub budget_outs: u64,
    pub at: Option<(usize, usize, usize)>,
    pub key_raw: Option<u64>,
    pub key_can: Option<u64>,
}

pub const CAVITY_BUDGET: u64 = 200_000;

pub fn cavity(v: &View, pool: bool) -> Cavity {
    let n = v.cx.n;
    let mut out = Cavity {
        r: None,
        work: 0,
        windows: 0,
        budget_outs: 0,
        at: None,
        key_raw: None,
        key_can: None,
    };
    for k in 2..=4.min(n) {
        for y0 in 0..=n - k {
            for x0 in 0..=n - k {
                let Some(cells) = window_cells(v, x0, y0, k) else {
                    continue;
                };
                let cands: Vec<Vec<u32>> = cells
                    .iter()
                    .map(|&c| {
                        if pool {
                            v.pool_cands(c)
                        } else {
                            v.dom_cands(c)
                        }
                    })
                    .collect();
                out.windows += 1;
                let mut t = 0;
                let res = exact(v.m(), &cells, &cands, CAVITY_BUDGET, &mut t);
                out.work += t;
                match res {
                    None => out.budget_outs += 1,
                    Some(true) => {}
                    Some(false) => {
                        out.r = Some(k);
                        out.at = Some((k, x0, y0));
                        out.key_raw = Some(raw_key(v, x0, y0, k));
                        out.key_can = Some(canonical_key(v, x0, y0, k));
                        return out;
                    }
                }
            }
        }
    }
    out
}

// ---------------------------------------------------------------- A

pub struct Band {
    pub dead: bool,
    pub dead_rows: bool,
    pub work: u64,
    pub removed: u64,
    pub values: u64,
}

/// Band cells and sides. Orientation 0: k rows, walk along x. 1: k columns, walk along y.
fn band_cell(n: usize, orient: usize, b0: usize, t: usize, j: usize) -> usize {
    if orient == 0 {
        (b0 + j) * n + t
    } else {
        t * n + b0 + j
    }
}

pub fn band(v: &View, k: usize) -> Band {
    let m = v.m();
    let n = v.cx.n;
    assert!(m.num_colors <= 16);
    let mut out = Band {
        dead: false,
        dead_rows: false,
        work: 0,
        removed: 0,
        values: 0,
    };
    for orient in 0..2 {
        // (across lower side, across upper side of next), (along out side, along in side)
        let (ac_lo, ac_hi, al_out, al_in) = if orient == 0 {
            (2, 0, 1, 3)
        } else {
            (1, 3, 2, 0)
        };
        for b0 in 0..=n - k {
            if (0..n).all(|t| (0..k).all(|j| v.s.assigned[band_cell(n, orient, b0, t, j)])) {
                continue;
            }
            // stacks[t]: (options, left key, right key)
            let mut stacks: Vec<Vec<([u32; 3], u16, u16)>> = Vec::with_capacity(n);
            for t in 0..n {
                let doms: Vec<Vec<u32>> = (0..k)
                    .map(|j| v.dom_cands(band_cell(n, orient, b0, t, j)))
                    .collect();
                let mut col = Vec::new();
                let mut cur = [0u32; 3];
                fn rec(
                    m: &Model,
                    doms: &[Vec<u32>],
                    j: usize,
                    cur: &mut [u32; 3],
                    col: &mut Vec<([u32; 3], u16, u16)>,
                    sides: (usize, usize, usize, usize),
                    work: &mut u64,
                ) {
                    let k = doms.len();
                    if j == k {
                        let (mut l, mut r) = (0u16, 0u16);
                        for (jj, &o) in cur.iter().take(k).enumerate() {
                            let e = m.opt_edges[o as usize];
                            l |= u16::from(e[sides.3]) << (4 * jj);
                            r |= u16::from(e[sides.2]) << (4 * jj);
                        }
                        col.push((*cur, l, r));
                        return;
                    }
                    for &o in &doms[j] {
                        *work += 1;
                        if j > 0
                            && m.opt_edges[cur[j - 1] as usize][sides.0]
                                != m.opt_edges[o as usize][sides.1]
                        {
                            continue;
                        }
                        cur[j] = o;
                        rec(m, doms, j + 1, cur, col, sides, work);
                    }
                }
                rec(
                    m,
                    &doms,
                    0,
                    &mut cur,
                    &mut col,
                    (ac_lo, ac_hi, al_out, al_in),
                    &mut out.work,
                );
                stacks.push(col);
            }
            let size = 1usize << (4 * k);
            let mut fwd = vec![vec![false; size]; n + 1];
            for s in &stacks[0] {
                fwd[0][s.1 as usize] = true;
            }
            let mut dead = false;
            for t in 0..n {
                let mut any = false;
                for s in &stacks[t] {
                    out.work += 1;
                    if fwd[t][s.1 as usize] {
                        fwd[t + 1][s.2 as usize] = true;
                        any = true;
                    }
                }
                if !any {
                    dead = true;
                    break;
                }
            }
            if dead {
                out.dead = true;
                if orient == 0 {
                    out.dead_rows = true;
                }
                continue;
            }
            let mut bwd = vec![vec![false; size]; n + 1];
            for s in &stacks[n - 1] {
                bwd[n][s.2 as usize] = true;
            }
            for t in (0..n).rev() {
                for s in &stacks[t] {
                    out.work += 1;
                    if bwd[t + 1][s.2 as usize] {
                        bwd[t][s.1 as usize] = true;
                    }
                }
            }
            for t in 0..n {
                for j in 0..k {
                    let c = band_cell(n, orient, b0, t, j);
                    let dom = v.dom_cands(c);
                    out.values += dom.len() as u64;
                    let mut sup: Vec<u32> = stacks[t]
                        .iter()
                        .filter(|s| fwd[t][s.1 as usize] && bwd[t + 1][s.2 as usize])
                        .map(|s| s.0[j])
                        .collect();
                    sup.sort_unstable();
                    sup.dedup();
                    out.removed += (dom.len() - sup.len()) as u64;
                }
            }
        }
    }
    out
}

/// Naive band check for the fuzz test: relaxed distinctness, plain enumeration.
#[must_use]
pub fn band_naive(v: &View, k: usize) -> bool {
    let n = v.cx.n;
    for orient in 0..2 {
        for b0 in 0..=n - k {
            let mut cells = Vec::new();
            for t in 0..n {
                for j in 0..k {
                    cells.push(band_cell(n, orient, b0, t, j));
                }
            }
            if cells.iter().all(|&c| v.s.assigned[c]) {
                continue;
            }
            let cands: Vec<Vec<u32>> = cells.iter().map(|&c| v.dom_cands(c)).collect();
            if !naive(v.m(), &cells, &cands, false) {
                return true;
            }
        }
    }
    false
}

// ---------------------------------------------------------------- E

pub struct Colour {
    pub dead: bool,
    pub overflow: bool,
    pub work: u64,
    pub max_states: u64,
}

pub const COLOUR_STATE_CAP: usize = 4_000_000;

pub fn colour_only(v: &View) -> Colour {
    let m = v.m();
    let n = v.cx.n;
    assert!(n <= 7 && m.num_colors <= 16);
    let mut out = Colour {
        dead: false,
        overflow: false,
        work: 0,
        max_states: 1,
    };
    let mut cur: Vec<u32> = vec![0];
    for y in 0..n {
        for x in 0..n {
            let c = y * n + x;
            // (N, W) -> list of (S, E), distinct colour tuples only.
            let mut table: Vec<Vec<(u8, u8)>> = vec![Vec::new(); 256];
            for o in bits(m.dom(&v.s.d, c)) {
                let e = m.opt_edges[o];
                let slot = &mut table[(e[0] as usize) << 4 | e[3] as usize];
                if !slot.contains(&(e[2], e[1])) {
                    slot.push((e[2], e[1]));
                }
            }
            let mut next: Vec<u32> = Vec::with_capacity(cur.len());
            for &st in &cur {
                let nc = (st >> (4 * x)) & 15;
                let wc = st >> 28;
                for &(s, e) in &table[(nc as usize) << 4 | wc as usize] {
                    out.work += 1;
                    let mut ns = st & !(15 << (4 * x)) & 0x0FFF_FFFF;
                    ns |= u32::from(s) << (4 * x);
                    if x + 1 < n {
                        ns |= u32::from(e) << 28;
                    }
                    next.push(ns);
                }
            }
            next.sort_unstable();
            next.dedup();
            out.max_states = out.max_states.max(next.len() as u64);
            if next.is_empty() {
                out.dead = true;
                return out;
            }
            if next.len() > COLOUR_STATE_CAP {
                out.overflow = true;
                return out;
            }
            cur = next;
        }
    }
    out
}

/// Naive colour-only check for the fuzz test (whole board, pieces reusable).
#[must_use]
pub fn colour_naive(v: &View) -> bool {
    let cells: Vec<usize> = (0..v.m().cells).collect();
    let cands: Vec<Vec<u32>> = cells.iter().map(|&c| v.dom_cands(c)).collect();
    !naive(v.m(), &cells, &cands, false)
}

// ---------------------------------------------------------------- D

pub struct Part {
    pub key: bool,
    pub dead: bool,
    pub calls: u64,
    pub parts: u64,
    pub dead_parts: u64,
}

pub struct Partition {
    pub cut: Part,
    pub rand: Part,
    pub poac: Part,
    pub poac_mrv: Part,
}

/// Undecided edge: (cell a, side da toward b, cell b, colour mask).
type Edge = (usize, usize, usize, u32);

fn offered(cx: &Ctx, d: &[u64], c: usize, side: usize) -> u32 {
    let dc = cx.m.dom(d, c);
    let mut mask = 0u32;
    for col in 0..cx.m.num_colors {
        if dc
            .iter()
            .zip(&cx.side_mask[side][col])
            .any(|(a, b)| a & b != 0)
        {
            mask |= 1 << col;
        }
    }
    mask
}

fn undecided(v: &View, a: usize, da: usize) -> Option<Edge> {
    let b = v.m().neighbour[a][da]?;
    if v.s.assigned[a] || v.s.assigned[b] {
        return None;
    }
    let mask = offered(v.cx, &v.s.d, a, da) & offered(v.cx, &v.s.d, b, (da + 2) & 3);
    (mask.count_ones() >= 2).then_some((a, da, b, mask))
}

fn run_partition(v: &View, e1: Edge, e2: Edge, st: &mut Stats) -> Part {
    let m = v.m();
    let w = m.words;
    let mut p = Part {
        key: true,
        dead: true,
        calls: 0,
        parts: 0,
        dead_parts: 0,
    };
    for c1 in 0..m.num_colors {
        if e1.3 >> c1 & 1 == 0 {
            continue;
        }
        for c2 in 0..m.num_colors {
            if e2.3 >> c2 & 1 == 0 {
                continue;
            }
            p.parts += 1;
            let mut nd = v.s.d.clone();
            let mut empty = false;
            for &(cell, side, col) in &[
                (e1.0, e1.1, c1),
                (e1.2, (e1.1 + 2) & 3, c1),
                (e2.0, e2.1, c2),
                (e2.2, (e2.1 + 2) & 3, c2),
            ] {
                for k in 0..w {
                    nd[cell * w + k] &= v.cx.side_mask[side][col][k];
                }
                if m.dom(&nd, cell).iter().all(|&x| x == 0) {
                    empty = true;
                }
            }
            let alive = !empty && {
                p.calls += 1;
                m.propagate(&mut nd, &[e1.0, e1.2, e2.0, e2.2], st)
            };
            if alive {
                p.dead = false;
            } else {
                p.dead_parts += 1;
            }
        }
    }
    p
}

const NO_KEY: Part = Part {
    key: false,
    dead: false,
    calls: 0,
    parts: 0,
    dead_parts: 0,
};

fn poac(v: &View, cells: &[usize], budget: u64, st: &mut Stats) -> Part {
    let m = v.m();
    let mut p = Part {
        key: true,
        dead: false,
        calls: 0,
        parts: 0,
        dead_parts: 0,
    };
    for &c in cells {
        let opts = v.dom_cands(c);
        if opts.len() as u64 > budget - p.calls {
            break;
        }
        let mut all_fail = true;
        for o in opts {
            p.calls += 1;
            p.parts += 1;
            let mut nd = v.s.d.clone();
            if backdoor_branching::assign(m, &mut nd, c, o as usize, st) {
                all_fail = false;
            } else {
                p.dead_parts += 1;
            }
        }
        if all_fail {
            p.dead = true;
            break;
        }
    }
    p
}

pub fn partition(v: &View, rng_seed: u64) -> Partition {
    let m = v.m();
    let n = v.cx.n;
    let mut st = Stats { propagations: 0 };
    // D-cut: horizontal lines, starting below the lowest row with an assigned cell.
    let y_low = (0..m.cells)
        .filter(|&c| v.s.assigned[c])
        .map(|c| c / n)
        .max()
        .unwrap_or(0);
    let mut lines: Vec<usize> = Vec::new();
    for y in (0..=y_low.min(n - 2)).rev() {
        lines.push(y);
    }
    for y in y_low + 1..n - 1 {
        lines.push(y);
    }
    let mut key: Option<(Edge, Edge)> = None;
    for &y in &lines {
        let mut es: Vec<(u32, usize, Edge)> = (0..n)
            .filter_map(|x| undecided(v, y * n + x, 2).map(|e| (e.3.count_ones(), y * n + x, e)))
            .collect();
        if es.len() >= 2 {
            es.sort_by_key(|e| (e.0, e.1));
            key = Some((es[0].2, es[1].2));
            break;
        }
    }
    let cut = match key {
        Some((e1, e2)) => run_partition(v, e1, e2, &mut st),
        None => NO_KEY,
    };
    // D-rand: two undecided edges (any orientation) not used by D-cut.
    let mut all: Vec<Edge> = Vec::new();
    for c in 0..m.cells {
        for da in [1usize, 2] {
            if let Some(e) = undecided(v, c, da) {
                let used =
                    key.is_some_and(|(a, b)| (a.0, a.1) == (e.0, e.1) || (b.0, b.1) == (e.0, e.1));
                if !used {
                    all.push(e);
                }
            }
        }
    }
    let rand = if all.len() >= 2 {
        let mut rng = Rng(rng_seed);
        let i = rng.below(all.len());
        let mut j = rng.below(all.len() - 1);
        if j >= i {
            j += 1;
        }
        run_partition(v, all[i], all[j], &mut st)
    } else {
        NO_KEY
    };
    // D-poac: bb-mrv cell order, budget = D-cut's propagate calls.
    let free: Vec<usize> = (0..m.cells).filter(|&c| !v.s.assigned[c]).collect();
    let mut order = free.clone();
    order.sort_by_key(|&c| {
        let k = m.count(&v.s.d, c);
        (u32::from(k != 1), if k == 1 { 0 } else { k }, c)
    });
    let poac_b = poac(v, &order, cut.calls, &mut st);
    let mrv = crate::tree::choose(m, &v.s.d, &v.s.assigned);
    let poac_m = poac(v, &[mrv], u64::MAX, &mut st);
    Partition {
        cut,
        rand,
        poac: poac_b,
        poac_mrv: poac_m,
    }
}

// ---------------------------------------------------------------- C

pub struct Pocket {
    pub dead: bool,
    pub size: usize,
    pub work: u64,
    pub overflow: bool,
    pub key_can: u64,
}

pub const POCKET_BUDGET: u64 = 1_000_000;

pub fn pocket(v: &View, wipes: &[u64]) -> Pocket {
    let m = v.m();
    let n = v.cx.n;
    let free: Vec<usize> = (0..m.cells).filter(|&c| !v.s.assigned[c]).collect();
    let start = *free
        .iter()
        .min_by_key(|&&c| (std::cmp::Reverse(wipes[c]), c))
        .expect("empty cell");
    let mut cells = vec![start];
    let mut inp = vec![false; m.cells];
    inp[start] = true;
    while cells.len() < 2 * n {
        let next = cells
            .iter()
            .flat_map(|&c| m.neighbour[c].iter().flatten().copied())
            .filter(|&c| !v.s.assigned[c] && !inp[c])
            .min_by_key(|&c| (std::cmp::Reverse(wipes[c]), c));
        let Some(c) = next else { break };
        inp[c] = true;
        cells.push(c);
    }
    let cands: Vec<Vec<u32>> = cells.iter().map(|&c| v.pool_cands(c)).collect();
    let mut t = 0;
    let res = exact(m, &cells, &cands, POCKET_BUDGET, &mut t);
    // Canonical key: shape and boundary word, minimum over 4 board rotations.
    let mut best: Option<Vec<u8>> = None;
    for rot in 0..4 {
        let mut pts: Vec<(usize, usize, usize)> = cells
            .iter()
            .map(|&c| {
                let (mut x, mut y) = (c % n, c / n);
                for _ in 0..rot {
                    let (nx, ny) = (n - 1 - y, x);
                    x = nx;
                    y = ny;
                }
                (y, x, c)
            })
            .collect();
        let (mx, my) = (
            pts.iter().map(|p| p.1).min().unwrap_or(0),
            pts.iter().map(|p| p.0).min().unwrap_or(0),
        );
        pts.sort_unstable();
        let mut enc = Vec::new();
        for &(y, x, c) in &pts {
            enc.push((x - mx) as u8);
            enc.push((y - my) as u8);
            for s in 0..4 {
                let od = (s + 4 - rot) % 4;
                enc.push(match m.neighbour[c][od] {
                    None => 254,
                    Some(nb) if inp[nb] => 252,
                    Some(nb) if v.s.assigned[nb] => m.opt_edges[v.opt[nb] as usize][(od + 2) & 3],
                    Some(_) => 253,
                });
            }
        }
        if best.as_ref().map_or(true, |b| enc < *b) {
            best = Some(enc);
        }
    }
    Pocket {
        dead: res == Some(false),
        size: cells.len(),
        work: t,
        overflow: res.is_none(),
        key_can: hash_bytes(&best.expect("key")),
    }
}
