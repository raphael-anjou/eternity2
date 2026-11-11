//! Strong-backdoor measurement for agenda A3 (see PREREG.md).
//!
//! A board state is one bitset of (piece, rotation) options per cell. The
//! propagation rule is the pre-registered one: rim unary, edge arc consistency,
//! piece singles and hidden singles, iterated to fixpoint.

use eternity2_engine::generator::XorShift;
use eternity2_engine::{rotated, Puzzle, BORDER};

pub const DX: [i32; 4] = [0, 1, 0, -1];
pub const DY: [i32; 4] = [-1, 0, 1, 0];

pub struct Model {
    pub n: usize,
    pub cells: usize,
    pub words: usize,
    pub num_colors: usize,
    pub opt_piece: Vec<u16>,
    pub opt_rot: Vec<u8>,
    pub opt_edges: Vec<[u8; 4]>,
    /// Option index range of each piece (options are grouped by piece).
    pub piece_range: Vec<(usize, usize)>,
    /// side_mask[d][c]: options with colour c on side d.
    side_mask: Vec<Vec<Vec<u64>>>,
    piece_mask: Vec<Vec<u64>>,
    /// neighbour[cell][d] = Some(cell') or None (rim).
    pub neighbour: Vec<[Option<usize>; 4]>,
    /// Planted option of each cell (piece = cell, rotation 0).
    pub planted: Vec<usize>,
    root: Vec<u64>,
    pub root_ok: bool,
}

pub struct Stats {
    pub propagations: u64,
}

fn set_bit(v: &mut [u64], i: usize) {
    v[i >> 6] |= 1u64 << (i & 63);
}

impl Model {
    pub fn new(p: &Puzzle) -> Self {
        let n = p.width as usize;
        assert_eq!(p.width, p.height);
        let cells = n * n;
        let mut opt_piece = Vec::new();
        let mut opt_rot = Vec::new();
        let mut opt_edges = Vec::new();
        let mut piece_range = Vec::new();
        let mut num_colors = 0usize;
        for (pi, &e) in p.pieces.iter().enumerate() {
            let start = opt_piece.len();
            let mut seen: Vec<[u8; 4]> = Vec::new();
            for r in 0..4u8 {
                let re = rotated(e, r);
                if seen.contains(&re) {
                    continue;
                }
                seen.push(re);
                for &c in &re {
                    num_colors = num_colors.max(c as usize);
                }
                opt_piece.push(pi as u16);
                opt_rot.push(r);
                opt_edges.push(re);
            }
            piece_range.push((start, opt_piece.len()));
        }
        let m = opt_piece.len();
        let words = m.div_ceil(64);
        let nc = num_colors + 1;
        let mut side_mask = vec![vec![vec![0u64; words]; nc]; 4];
        for (o, e) in opt_edges.iter().enumerate() {
            for d in 0..4 {
                set_bit(&mut side_mask[d][e[d] as usize], o);
            }
        }
        let mut piece_mask = vec![vec![0u64; words]; cells];
        for (pi, &(s, t)) in piece_range.iter().enumerate() {
            for o in s..t {
                set_bit(&mut piece_mask[pi], o);
            }
        }
        let mut neighbour = vec![[None; 4]; cells];
        for y in 0..n {
            for x in 0..n {
                for d in 0..4 {
                    let nx = x as i32 + DX[d];
                    let ny = y as i32 + DY[d];
                    if nx >= 0 && ny >= 0 && (nx as usize) < n && (ny as usize) < n {
                        neighbour[y * n + x][d] = Some(ny as usize * n + nx as usize);
                    }
                }
            }
        }
        let planted = (0..cells).map(|c| piece_range[c].0).collect();
        let mut model = Model {
            n,
            cells,
            words,
            num_colors: nc,
            opt_piece,
            opt_rot,
            opt_edges,
            piece_range,
            side_mask,
            piece_mask,
            neighbour,
            planted,
            root: Vec::new(),
            root_ok: true,
        };
        debug_assert!(model.opt_rot.iter().all(|&r| r < 4));
        // Unary rim constraint.
        let mut dom = vec![0u64; cells * words];
        for c in 0..cells {
            let mut allowed = vec![!0u64; words];
            for d in 0..4 {
                let grey = &model.side_mask[d][BORDER as usize];
                for w in 0..words {
                    if model.neighbour[c][d].is_none() {
                        allowed[w] &= grey[w];
                    } else {
                        allowed[w] &= !grey[w];
                    }
                }
            }
            for w in 0..words {
                let mut v = allowed[w];
                if w == words - 1 && m % 64 != 0 {
                    v &= (1u64 << (m % 64)) - 1;
                }
                dom[c * words + w] = v;
            }
        }
        let all: Vec<usize> = (0..cells).collect();
        let mut st = Stats { propagations: 0 };
        model.root_ok = model.propagate(&mut dom, &all, &mut st);
        model.root = dom;
        model
    }

    pub fn root(&self) -> &[u64] {
        &self.root
    }

    #[inline]
    pub fn dom<'a>(&self, d: &'a [u64], c: usize) -> &'a [u64] {
        &d[c * self.words..(c + 1) * self.words]
    }

    pub fn count(&self, d: &[u64], c: usize) -> u32 {
        self.dom(d, c).iter().map(|w| w.count_ones()).sum()
    }

    pub fn singletons(&self, d: &[u64]) -> usize {
        (0..self.cells).filter(|&c| self.count(d, c) == 1).count()
    }

    pub fn total_options(&self, d: &[u64]) -> u64 {
        d.iter().map(|w| w.count_ones() as u64).sum()
    }

    pub fn is_complete(&self, d: &[u64]) -> bool {
        (0..self.cells).all(|c| self.count(d, c) == 1)
    }

    fn offered(&self, d: &[u64], c: usize, side: usize) -> u32 {
        let dc = self.dom(d, c);
        let mut mask = 0u32;
        for col in 0..self.num_colors {
            let sm = &self.side_mask[side][col];
            if dc.iter().zip(sm).any(|(a, b)| a & b != 0) {
                mask |= 1 << col;
            }
        }
        mask
    }

    /// Propagate to fixpoint starting from the cells in `changed`. Returns
    /// false on contradiction.
    pub fn propagate(&self, d: &mut [u64], changed: &[usize], st: &mut Stats) -> bool {
        st.propagations += 1;
        let w = self.words;
        let mut queue: Vec<usize> = changed.to_vec();
        let mut inq = vec![false; self.cells];
        for &c in changed {
            inq[c] = true;
        }
        let mut allowed = vec![0u64; w];
        loop {
            while let Some(b) = queue.pop() {
                inq[b] = false;
                for db in 0..4 {
                    let Some(a) = self.neighbour[b][db] else { continue };
                    // a sees b on side da = opposite of db.
                    let da = (db + 2) & 3;
                    let mask = self.offered(d, b, db);
                    allowed.iter_mut().for_each(|x| *x = 0);
                    for col in 0..self.num_colors {
                        if mask & (1 << col) != 0 {
                            for (x, y) in allowed.iter_mut().zip(&self.side_mask[da][col]) {
                                *x |= *y;
                            }
                        }
                    }
                    let mut changed_a = false;
                    let mut empty = true;
                    for k in 0..w {
                        let old = d[a * w + k];
                        let new = old & allowed[k];
                        if new != old {
                            changed_a = true;
                            d[a * w + k] = new;
                        }
                        if new != 0 {
                            empty = false;
                        }
                    }
                    if empty {
                        return false;
                    }
                    if changed_a && !inq[a] {
                        inq[a] = true;
                        queue.push(a);
                    }
                }
            }
            // All-different pass.
            let pieces = self.piece_range.len();
            let mut cnt = vec![0u8; pieces];
            let mut where_ = vec![0usize; pieces];
            let mut single: Vec<Option<usize>> = vec![None; self.cells];
            for c in 0..self.cells {
                let mut first: Option<usize> = None;
                let mut multi = false;
                for (pi, &(s, t)) in self.piece_range.iter().enumerate() {
                    let pm = &self.piece_mask[pi];
                    // quick skip using word range
                    let (ws, we) = (s >> 6, (t - 1) >> 6);
                    let mut has = false;
                    for k in ws..=we {
                        if d[c * w + k] & pm[k] != 0 {
                            has = true;
                            break;
                        }
                    }
                    if has {
                        if cnt[pi] < 2 {
                            cnt[pi] += 1;
                        }
                        where_[pi] = c;
                        match first {
                            None => first = Some(pi),
                            Some(_) => multi = true,
                        }
                    }
                }
                if !multi {
                    single[c] = first;
                }
            }
            let mut any = false;
            if cnt.contains(&0) {
                return false;
            }
            for (c, sc) in single.iter().enumerate() {
                if let Some(pi) = *sc {
                    let (s, t) = self.piece_range[pi];
                    let pm = &self.piece_mask[pi];
                    for c2 in 0..self.cells {
                        if c2 == c {
                            continue;
                        }
                        let mut ch = false;
                        for k in (s >> 6)..=((t - 1) >> 6) {
                            if d[c2 * w + k] & pm[k] != 0 {
                                d[c2 * w + k] &= !pm[k];
                                ch = true;
                            }
                        }
                        if ch {
                            if self.dom(d, c2).iter().all(|&x| x == 0) {
                                return false;
                            }
                            any = true;
                            if !inq[c2] {
                                inq[c2] = true;
                                queue.push(c2);
                            }
                        }
                    }
                }
            }
            for pi in 0..pieces {
                if cnt[pi] == 1 {
                    let c = where_[pi];
                    let pm = &self.piece_mask[pi];
                    let mut ch = false;
                    for k in 0..w {
                        let old = d[c * w + k];
                        let new = old & pm[k];
                        if new != old {
                            d[c * w + k] = new;
                            ch = true;
                        }
                    }
                    if ch {
                        if self.dom(d, c).iter().all(|&x| x == 0) {
                            return false;
                        }
                        any = true;
                        if !inq[c] {
                            inq[c] = true;
                            queue.push(c);
                        }
                    }
                }
            }
            if !any {
                return true;
            }
        }
    }

    /// Fix `cell` to its planted option in `d` and propagate.
    pub fn fix(&self, d: &mut [u64], cell: usize, st: &mut Stats) -> bool {
        let w = self.words;
        let o = self.planted[cell];
        for k in 0..w {
            d[cell * w + k] = 0;
        }
        d[cell * w + (o >> 6)] = 1u64 << (o & 63);
        self.propagate(d, &[cell], st)
    }

    /// State after fixing every cell of `set` to its planted option.
    pub fn state_for(&self, set: &[usize], st: &mut Stats) -> (Vec<u64>, bool) {
        let mut d = self.root.clone();
        let w = self.words;
        for &cell in set {
            let o = self.planted[cell];
            for k in 0..w {
                d[cell * w + k] = 0;
            }
            d[cell * w + (o >> 6)] = 1u64 << (o & 63);
        }
        let ok = self.propagate(&mut d, set, st);
        (d, ok)
    }

    pub fn completes(&self, set: &[usize], st: &mut Stats) -> bool {
        let (d, ok) = self.state_for(set, st);
        assert!(ok, "contradiction under planted fixes: propagation bug");
        self.is_complete(&d)
    }

    /// For a complete state: (valid board, equals planted).
    pub fn check_board(&self, d: &[u64]) -> (bool, bool) {
        let mut opt = vec![0usize; self.cells];
        for (c, slot) in opt.iter_mut().enumerate() {
            let dc = self.dom(d, c);
            let k = dc.iter().position(|&x| x != 0).unwrap();
            *slot = k * 64 + dc[k].trailing_zeros() as usize;
        }
        let mut used = vec![false; self.piece_range.len()];
        let mut valid = true;
        for c in 0..self.cells {
            let p = self.opt_piece[opt[c]] as usize;
            if used[p] {
                valid = false;
            }
            used[p] = true;
            let e = self.opt_edges[opt[c]];
            for dd in 0..4 {
                match self.neighbour[c][dd] {
                    None => valid &= e[dd] == BORDER,
                    Some(c2) => {
                        let e2 = self.opt_edges[opt[c2]];
                        valid &= e[dd] == e2[(dd + 2) & 3] && e[dd] != BORDER;
                    }
                }
            }
        }
        let planted = (0..self.cells).all(|c| opt[c] == self.planted[c]);
        (valid, planted)
    }
}

pub fn permutation(len: usize, seed: u32) -> Vec<usize> {
    let mut v: Vec<usize> = (0..len).collect();
    let mut rng = XorShift::new(seed);
    rng.shuffle(&mut v);
    v
}

/// Removal pass: visit `order`, drop each cell of `set` whose removal keeps
/// completion. `set` must complete.
pub fn removal_pass(m: &Model, set: &[usize], order: &[usize], st: &mut Stats) -> Vec<usize> {
    let mut inset = vec![false; m.cells];
    for &c in set {
        inset[c] = true;
    }
    for &c in order {
        if !inset[c] {
            continue;
        }
        inset[c] = false;
        let s: Vec<usize> = (0..m.cells).filter(|&x| inset[x]).collect();
        if !m.completes(&s, st) {
            inset[c] = true;
        }
    }
    (0..m.cells).filter(|&x| inset[x]).collect()
}

/// Forward greedy then reverse prune in insertion order.
pub fn forward_greedy(m: &Model, st: &mut Stats) -> Vec<usize> {
    let mut cur = m.root().to_vec();
    let mut chosen = Vec::new();
    while !m.is_complete(&cur) {
        let mut best: Option<(usize, u64, usize, Vec<u64>)> = None;
        for c in 0..m.cells {
            if m.count(&cur, c) == 1 {
                continue;
            }
            let mut d = cur.clone();
            assert!(m.fix(&mut d, c, st), "contradiction: propagation bug");
            let s = m.singletons(&d);
            let t = m.total_options(&d);
            let better = match &best {
                None => true,
                Some((bs, bt, _, _)) => s > *bs || (s == *bs && t < *bt),
            };
            if better {
                best = Some((s, t, c, d));
            }
        }
        let (_, _, c, d) = best.expect("incomplete state with no free cell");
        chosen.push(c);
        cur = d;
    }
    let order = chosen.clone();
    removal_pass(m, &chosen, &order, st)
}

/// Local search: first-improvement swap scan, as in PREREG.md.
pub fn local_search(m: &Model, start: Vec<usize>, cap: u64, st: &mut Stats) -> Vec<usize> {
    let mut best = start;
    let base = st.propagations;
    'outer: loop {
        let inset: Vec<bool> = (0..m.cells).map(|x| best.contains(&x)).collect();
        for &c in &best.clone() {
            for (d, &d_in) in inset.iter().enumerate() {
                if st.propagations - base >= cap {
                    break 'outer;
                }
                if d_in {
                    continue;
                }
                let mut s: Vec<usize> = best.iter().copied().filter(|&x| x != c).collect();
                s.push(d);
                s.sort_unstable();
                if m.completes(&s, st) {
                    let order = s.clone();
                    let r = removal_pass(m, &s, &order, st);
                    if r.len() < best.len() {
                        best = r;
                        continue 'outer;
                    }
                }
            }
        }
        break;
    }
    best
}

pub struct Exact {
    pub certified: bool,
    /// Exact minimum when certified, else certified lower bound.
    pub value: usize,
    pub nodes: u64,
}

fn dfs(
    m: &Model,
    d: &[u64],
    start: usize,
    left: usize,
    nodes: &mut u64,
    cap: u64,
    st: &mut Stats,
) -> Option<bool> {
    if m.is_complete(d) {
        return Some(true);
    }
    if left == 0 {
        return Some(false);
    }
    for c in start..m.cells {
        if m.count(d, c) == 1 {
            continue;
        }
        if *nodes >= cap {
            return None;
        }
        *nodes += 1;
        let mut nd = d.to_vec();
        assert!(m.fix(&mut nd, c, st), "contradiction: propagation bug");
        if dfs(m, &nd, c + 1, left - 1, nodes, cap, st)? {
            return Some(true);
        }
    }
    Some(false)
}

pub fn exact_min(m: &Model, ub: usize, cap: u64, st: &mut Stats) -> Exact {
    let mut nodes = 0u64;
    for k in 0..ub {
        match dfs(m, m.root(), 0, k, &mut nodes, cap, st) {
            Some(true) => return Exact { certified: true, value: k, nodes },
            Some(false) => {}
            None => return Exact { certified: false, value: k, nodes },
        }
    }
    Exact { certified: true, value: ub, nodes }
}

/// Smallest prefix length of `perm` that completes (binary search, monotone).
pub fn threshold(m: &Model, perm: &[usize], st: &mut Stats) -> usize {
    let (mut lo, mut hi) = (0usize, m.cells);
    if m.completes(&[], st) {
        return 0;
    }
    // invariant: prefix lo fails, prefix hi completes
    while hi - lo > 1 {
        let mid = (lo + hi) / 2;
        if m.completes(&perm[..mid], st) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    hi
}

#[cfg(test)]
mod tests {
    use super::*;
    use eternity2_engine::generate_solved_framed_bc;

    #[test]
    fn full_set_completes_to_planted() {
        for seed in 900..905 {
            let p = generate_solved_framed_bc(6, 7, seed, true, 2);
            let m = Model::new(&p);
            assert!(m.root_ok);
            let mut st = Stats { propagations: 0 };
            let all: Vec<usize> = (0..m.cells).collect();
            let (d, ok) = m.state_for(&all, &mut st);
            assert!(ok && m.is_complete(&d));
            assert_eq!(m.check_board(&d), (true, true));
            // planted option survives at root
            for c in 0..m.cells {
                let o = m.planted[c];
                assert!(m.root()[c * m.words + (o >> 6)] >> (o & 63) & 1 == 1);
            }
        }
    }

    #[test]
    fn completion_is_monotone_on_samples() {
        for seed in 910..913 {
            let p = generate_solved_framed_bc(6, 10, seed, true, 2);
            let m = Model::new(&p);
            let mut st = Stats { propagations: 0 };
            for r in 0..20 {
                let perm = permutation(m.cells, 5000 + r);
                let mut prev = false;
                for k in 0..=m.cells {
                    let c = m.completes(&perm[..k], &mut st);
                    assert!(!(prev && !c), "monotonicity violated");
                    prev = c;
                }
            }
        }
    }

    #[test]
    fn exact_not_above_greedy_and_greedy_completes() {
        let p = generate_solved_framed_bc(6, 10, 920, true, 2);
        let m = Model::new(&p);
        let mut st = Stats { propagations: 0 };
        let g = forward_greedy(&m, &mut st);
        assert!(m.completes(&g, &mut st));
        let e = exact_min(&m, g.len(), 2_000_000, &mut st);
        if e.certified {
            assert!(e.value <= g.len());
        }
    }
}
