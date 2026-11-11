//! A22 arms (see `../PREREG.md`). The binding `bb-mrv` arm is A16's
//! `BbSolver { rule: Rule::Mrv }` unchanged; this crate adds the guarded scb
//! wrapper, the three bb-mrv ablations (`MrvSolver`) and the `scb-ad` fork.

pub mod scb;

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use eternity2_engine::{Puzzle, BORDER};
use hard_ladder::{LadderSolver, NodeCounter, Outcome};
use sat_backdoors::{Model, Stats};

/// Per-run wall-clock guard in seconds (PREREG.md: 600 at cap 1e6, 180 at 1e7).
pub static GUARD_SECS: AtomicU64 = AtomicU64::new(600);

#[must_use]
pub fn guard() -> Duration {
    Duration::from_secs(GUARD_SECS.load(Ordering::Relaxed))
}

fn set_bit(v: &mut [u64], i: usize) {
    v[i >> 6] |= 1u64 << (i & 63);
}

/// A3's `Model::propagate` rebuilt from the model's public fields, with the
/// piece-side all-different rules (hidden singles, piece with no cell)
/// switchable. With `hs = true` it matches `Model::propagate` exactly (tested).
pub struct Prop<'a> {
    m: &'a Model,
    side_mask: Vec<Vec<Vec<u64>>>,
    piece_mask: Vec<Vec<u64>>,
    pub hs: bool,
}

impl<'a> Prop<'a> {
    #[must_use]
    pub fn new(m: &'a Model, hs: bool) -> Self {
        let w = m.words;
        let mut side_mask = vec![vec![vec![0u64; w]; m.num_colors]; 4];
        for (o, e) in m.opt_edges.iter().enumerate() {
            for (d, sm) in side_mask.iter_mut().enumerate() {
                set_bit(&mut sm[e[d] as usize], o);
            }
        }
        let mut piece_mask = vec![vec![0u64; w]; m.piece_range.len()];
        for (pi, &(s, t)) in m.piece_range.iter().enumerate() {
            for o in s..t {
                set_bit(&mut piece_mask[pi], o);
            }
        }
        Self { m, side_mask, piece_mask, hs }
    }

    /// Domains after the unary rim constraint only (before any propagation).
    #[must_use]
    pub fn unary_root(&self) -> Vec<u64> {
        let (m, w) = (self.m, self.m.words);
        let n_opt = m.opt_edges.len();
        let mut dom = vec![0u64; m.cells * w];
        for c in 0..m.cells {
            for k in 0..w {
                let mut v = !0u64;
                for d in 0..4 {
                    let grey = self.side_mask[d][BORDER as usize][k];
                    v &= if m.neighbour[c][d].is_none() { grey } else { !grey };
                }
                if k == w - 1 && n_opt % 64 != 0 {
                    v &= (1u64 << (n_opt % 64)) - 1;
                }
                dom[c * w + k] = v;
            }
        }
        dom
    }

    fn offered(&self, d: &[u64], c: usize, side: usize) -> u32 {
        let dc = self.m.dom(d, c);
        let mut mask = 0u32;
        for col in 0..self.m.num_colors {
            if dc.iter().zip(&self.side_mask[side][col]).any(|(a, b)| a & b != 0) {
                mask |= 1 << col;
            }
        }
        mask
    }

    /// Same fixpoint loop as `Model::propagate`; the piece-side rules run
    /// only when `hs`.
    pub fn propagate(&self, d: &mut [u64], changed: &[usize], st: &mut Stats) -> bool {
        st.propagations += 1;
        let m = self.m;
        let w = m.words;
        let mut queue: Vec<usize> = changed.to_vec();
        let mut inq = vec![false; m.cells];
        for &c in changed {
            inq[c] = true;
        }
        let mut allowed = vec![0u64; w];
        loop {
            while let Some(b) = queue.pop() {
                inq[b] = false;
                for db in 0..4 {
                    let Some(a) = m.neighbour[b][db] else { continue };
                    let da = (db + 2) & 3;
                    let mask = self.offered(d, b, db);
                    allowed.iter_mut().for_each(|x| *x = 0);
                    for col in 0..m.num_colors {
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
            let pieces = m.piece_range.len();
            let mut cnt = vec![0u8; pieces];
            let mut where_ = vec![0usize; pieces];
            let mut single: Vec<Option<usize>> = vec![None; m.cells];
            for c in 0..m.cells {
                let mut first: Option<usize> = None;
                let mut multi = false;
                for (pi, &(s, t)) in m.piece_range.iter().enumerate() {
                    let pm = &self.piece_mask[pi];
                    let has = ((s >> 6)..=((t - 1) >> 6)).any(|k| d[c * w + k] & pm[k] != 0);
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
            if self.hs && cnt.contains(&0) {
                return false;
            }
            for (c, sc) in single.iter().enumerate() {
                if let Some(pi) = *sc {
                    let (s, t) = m.piece_range[pi];
                    let pm = &self.piece_mask[pi];
                    for c2 in 0..m.cells {
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
                            if m.dom(d, c2).iter().all(|&x| x == 0) {
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
            if self.hs {
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
                            if m.dom(d, c).iter().all(|&x| x == 0) {
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
            }
            if !any {
                return true;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Order {
    /// bb-mrv: min (|D|, index).
    Free,
    /// scb's `BorderFirstMrv` key: min (class, |D|, index), corner < edge < interior.
    BorderFirst,
}

/// bb-mrv with one switch flipped: `hs` (piece-side all-different), `order`
/// (cell choice after forced cells) or `random` (value order seed). The
/// forced-move rule (lowest-index cell with |D| = 1 first) is kept in all.
pub struct MrvSolver {
    pub order: Order,
    pub random: Option<u64>,
    pub hs: bool,
    pub guard: Duration,
}

enum Step {
    Found,
    Failed,
    Cap,
}

struct Search<'a> {
    m: &'a Model,
    p: Prop<'a>,
    order: Order,
    class: Vec<u8>,
    rng: Option<u64>,
    st: Stats,
    start: Instant,
    guard: Duration,
    board: Vec<u64>,
}

impl Search<'_> {
    /// scb's generator (`SearchState::next_random`, splitmix64).
    fn next_random(&mut self) -> u64 {
        let s = self.rng.as_mut().expect("rng");
        *s = s.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = *s;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn assign(&mut self, d: &mut [u64], c: usize, o: usize) -> bool {
        let w = self.m.words;
        d[c * w..(c + 1) * w].iter_mut().for_each(|x| *x = 0);
        d[c * w + (o >> 6)] = 1u64 << (o & 63);
        if self.p.hs {
            self.m.propagate(d, &[c], &mut self.st)
        } else {
            self.p.propagate(d, &[c], &mut self.st)
        }
    }

    fn choose(&self, d: &[u64], assigned: &[bool]) -> usize {
        let m = self.m;
        let free = (0..m.cells).filter(|&c| !assigned[c]);
        if let Some(c) = free.clone().find(|&c| m.count(d, c) == 1) {
            return c;
        }
        match self.order {
            Order::Free => free.min_by_key(|&c| (m.count(d, c), c)),
            Order::BorderFirst => free.min_by_key(|&c| (self.class[c], m.count(d, c), c)),
        }
        .expect("free cell")
    }

    fn dfs(&mut self, d: &[u64], assigned: &mut Vec<bool>, left: usize, counter: &mut NodeCounter) -> Step {
        if left == 0 {
            self.board = d.to_vec();
            return Step::Found;
        }
        let c = self.choose(d, assigned);
        let mut opts = backdoor_branching::options(self.m, d, c);
        if self.rng.is_some() && opts.len() > 1 {
            for i in (1..opts.len()).rev() {
                let j = (self.next_random() as usize) % (i + 1);
                opts.swap(i, j);
            }
        }
        let mut nd = d.to_vec();
        for o in opts {
            if !counter.try_assign() || self.start.elapsed() > self.guard {
                return Step::Cap;
            }
            nd.copy_from_slice(d);
            assigned[c] = true;
            if self.assign(&mut nd, c, o) {
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

impl LadderSolver for MrvSolver {
    fn solve(&mut self, puzzle: &Puzzle, counter: &mut NodeCounter) -> Outcome {
        let start = Instant::now();
        let m = Model::new(puzzle);
        let p = Prop::new(&m, self.hs);
        let mut st = Stats { propagations: 1 };
        let root = if self.hs {
            if !m.root_ok {
                return Outcome::Exhausted;
            }
            m.root().to_vec()
        } else {
            let mut d = p.unary_root();
            let all: Vec<usize> = (0..m.cells).collect();
            st.propagations = 0;
            if !p.propagate(&mut d, &all, &mut st) {
                return Outcome::Exhausted;
            }
            d
        };
        let n = m.n;
        let class = (0..m.cells)
            .map(|c| {
                let (x, y) = (c % n, c / n);
                let k = u8::from(x == 0 || x == n - 1) + u8::from(y == 0 || y == n - 1);
                2 - k
            })
            .collect();
        let mut search = Search {
            m: &m,
            p,
            order: self.order,
            class,
            rng: self.random.map(|s| s.wrapping_add(0x9E37_79B9_7F4A_7C15)),
            st,
            start,
            guard: self.guard,
            board: Vec::new(),
        };
        let mut assigned = vec![false; m.cells];
        let out = search.dfs(&root, &mut assigned, m.cells, counter);
        counter.propagation = search.st.propagations;
        match out {
            Step::Found => Outcome::Solved(
                (0..m.cells)
                    .map(|c| {
                        let o = backdoor_branching::options(&m, &search.board, c)[0];
                        i32::from(m.opt_piece[o]) * 4 + i32::from(m.opt_rot[o])
                    })
                    .collect(),
            ),
            Step::Failed => Outcome::Exhausted,
            Step::Cap => Outcome::CapReached,
        }
    }
}
