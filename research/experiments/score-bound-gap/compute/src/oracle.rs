//! Exact best completion of a prefix, independent of the decision diagram.
//!
//! Minimises the number k of mismatched remaining edges by iterative
//! deepening: for k = k0, k0+1, ... a row-major DFS over the remaining cells,
//! using only unused pieces, looks for a completion with at most k
//! mismatches. The first k that succeeds is the optimum. Pruning uses two
//! lower bounds on future mismatches, both independent of `dd.rs`:
//! - colour counting (distinctness exact, geometry relaxed): a matched edge
//!   needs two half-edges of one colour among the unused pieces, or one
//!   half-edge equal to a frontier colour;
//! - top check: a column whose next unfilled cell has top colour c, where no
//!   unused piece can show c on top at that cell, costs one mismatch (these
//!   edges are distinct).
//!
//! Best total score = prefix score + remaining edges - k.

use crate::model::{Model, Prefix};

const NCLASS: usize = 5; // interior, left edge, right edge, bottom edge, bottom-left, bottom-right
const BL: usize = 5;

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OracleOut {
    /// Minimum mismatches among remaining edges, if proven.
    pub kstar: Option<u32>,
    /// Largest k proven infeasible (-1 if none).
    pub k_infeasible: i32,
    pub nodes: u64,
    pub wall_us: u64,
    /// Witness completion (piece*4+rot is not stored; (piece, rot-colours) as codes).
    #[serde(skip)]
    pub witness: Vec<(u8, [u8; 4])>,
}

struct St<'a> {
    m: &'a Model,
    n: usize,
    ncol: usize,
    used: u64,
    /// Placed URDL colours per cell (valid for filled cells).
    cells: Vec<[u8; 4]>,
    pieces_at: Vec<u8>,
    /// Remaining interior-facing half-edges per colour (unused pieces).
    h: [i32; 16],
    /// topmask[class][piece]: colours a piece can show on top at that class.
    topmask: Vec<Vec<u16>>,
    /// cnt[class][c]: unused pieces able to show c on top at that class.
    cnt: Vec<[i32; 16]>,
    nodes: u64,
    cap: u64,
    aborted: bool,
}

fn class_of(n: usize, x: usize, y: usize) -> usize {
    if y == n - 1 {
        if x == 0 {
            4
        } else if x == n - 1 {
            BL
        } else {
            3
        }
    } else if x == 0 {
        1
    } else if x == n - 1 {
        2
    } else {
        0
    }
}

impl<'a> St<'a> {
    fn new(m: &'a Model, pre: &Prefix, cap: u64) -> St<'a> {
        let n = m.n;
        let mut topmask = vec![vec![0u16; m.pieces.len()]; NCLASS + 1];
        // Representative cell of each class (rows 1..n-1 only).
        let reps = [
            (1, 1),
            (0, 1),
            (n - 1, 1),
            (1, n - 1),
            (0, n - 1),
            (n - 1, n - 1),
        ];
        for (k, &(x, y)) in reps.iter().enumerate() {
            let pos = y * n + x;
            for o in &m.opts[pos] {
                topmask[k][o.p as usize] |= 1 << o.e[0];
            }
        }
        let used = pre.used();
        let mut h = [0i32; 16];
        let mut cnt = vec![[0i32; 16]; NCLASS + 1];
        for (p, pe) in m.pieces.iter().enumerate() {
            if used >> p & 1 == 1 {
                continue;
            }
            for &c in pe {
                if c != 0 {
                    h[c as usize] += 1;
                }
            }
            for k in 0..=NCLASS {
                for c in 0..16 {
                    if topmask[k][p] >> c & 1 == 1 {
                        cnt[k][c] += 1;
                    }
                }
            }
        }
        let mut cells = vec![[0u8; 4]; n * n];
        let mut pieces_at = vec![0u8; n * n];
        for (i, o) in pre.cells.iter().enumerate() {
            cells[i] = o.e;
            pieces_at[i] = o.p;
        }
        St {
            m,
            n,
            ncol: m.ncol,
            used,
            cells,
            pieces_at,
            h,
            topmask,
            cnt,
            nodes: 0,
            cap,
            aborted: false,
        }
    }

    fn take(&mut self, p: usize, sign: i32) {
        for &c in &self.m.pieces[p] {
            if c != 0 {
                self.h[c as usize] -= sign;
            }
        }
        for k in 0..=NCLASS {
            let mut mask = self.topmask[k][p];
            while mask != 0 {
                let c = mask.trailing_zeros() as usize;
                self.cnt[k][c] -= sign;
                mask &= mask - 1;
            }
        }
    }

    /// Lower bound on mismatches among edges not yet decided, when cells
    /// 0..next-1 are filled.
    fn lb(&self, next: usize) -> u32 {
        let n = self.n;
        let total = n * n;
        if next == total {
            return 0;
        }
        let mut f = [0i32; 16];
        let mut fcount = 0i32;
        let mut top_lb = 0u32;
        let (nx, ny) = (next % n, next / n);
        for x in 0..n {
            // Next unfilled cell of column x.
            let y = if x >= nx { ny } else { ny + 1 };
            if y >= n || y == 0 {
                continue;
            }
            let c = self.cells[(y - 1) * n + x][2] as usize;
            f[c] += 1;
            fcount += 1;
            if self.cnt[class_of(n, x, y)][c] == 0 {
                top_lb += 1;
            }
        }
        if nx > 0 {
            let c = self.cells[next - 1][1] as usize;
            f[c] += 1;
            fcount += 1;
        }
        let hsum: i32 = self.h.iter().sum();
        let internal = (hsum - fcount) / 2;
        let rem = fcount + internal;
        let mut a = 0i32;
        let mut b = 0i32;
        let mut half = 0i32;
        for c in 1..self.ncol {
            let mc = f[c].min(self.h[c]);
            a += mc + (self.h[c] - mc) / 2;
            b += mc;
            half += self.h[c] / 2;
        }
        let ub = rem.min(a).min(b.min(fcount) + half.min(internal));
        let colour_lb = (rem - ub) as u32;
        colour_lb.max(top_lb)
    }

    fn dfs(&mut self, pos: usize, mis: u32, k: u32) -> bool {
        let n = self.n;
        if pos == n * n {
            return true;
        }
        let (x, y) = (pos % n, pos / n);
        let top = if y > 0 {
            Some(self.cells[pos - n][2])
        } else {
            None
        };
        let left = if x > 0 {
            Some(self.cells[pos - 1][1])
        } else {
            None
        };
        let m = self.m;
        for want in 0..=2u32 {
            if mis + want > k {
                break;
            }
            for o in &m.opts[pos] {
                if self.used >> o.p & 1 == 1 {
                    continue;
                }
                let mut cost = 0;
                if let Some(t) = top {
                    if t != o.e[0] {
                        cost += 1;
                    }
                }
                if let Some(l) = left {
                    if l != o.e[3] {
                        cost += 1;
                    }
                }
                if cost != want {
                    continue;
                }
                self.nodes += 1;
                if self.nodes > self.cap {
                    self.aborted = true;
                    return false;
                }
                self.used |= 1 << o.p;
                self.cells[pos] = o.e;
                self.pieces_at[pos] = o.p;
                self.take(o.p as usize, 1);
                let ok = mis + cost + self.lb(pos + 1) <= k && self.dfs(pos + 1, mis + cost, k);
                self.take(o.p as usize, -1);
                self.used &= !(1 << o.p);
                if ok {
                    return true;
                }
                if self.aborted {
                    return false;
                }
            }
        }
        false
    }
}

pub fn best_completion(m: &Model, pre: &Prefix, cap: u64) -> OracleOut {
    let t0 = std::time::Instant::now();
    let mut st = St::new(m, pre, cap);
    let d = pre.d();
    let rem = m.remaining_edges(d);
    let k0 = st.lb(d);
    let mut out = OracleOut {
        kstar: None,
        k_infeasible: k0 as i32 - 1,
        ..Default::default()
    };
    for k in k0..=rem {
        if st.dfs(d, 0, k) {
            out.kstar = Some(k);
            out.witness = (d..m.n_cells())
                .map(|i| (st.pieces_at[i], st.cells[i]))
                .collect();
            break;
        }
        if st.aborted {
            break;
        }
        out.k_infeasible = k as i32;
    }
    out.nodes = st.nodes;
    out.wall_us = t0.elapsed().as_micros() as u64;
    out
}

/// Exhaustive search without any bound (tests only, tiny cases).
pub fn brute_best(m: &Model, pre: &Prefix) -> u32 {
    fn rec(m: &Model, pos: usize, used: u64, cells: &mut Vec<[u8; 4]>, score: u32, best: &mut u32) {
        let n = m.n;
        if pos == n * n {
            *best = (*best).max(score);
            return;
        }
        let (x, y) = (pos % n, pos / n);
        for o in &m.opts[pos] {
            if used >> o.p & 1 == 1 {
                continue;
            }
            let mut s = score;
            if y > 0 && cells[pos - n][2] == o.e[0] {
                s += 1;
            }
            if x > 0 && cells[pos - 1][1] == o.e[3] {
                s += 1;
            }
            cells[pos] = o.e;
            rec(m, pos + 1, used | 1 << o.p, cells, s, best);
        }
    }
    let mut cells = vec![[0u8; 4]; m.n_cells()];
    for (i, o) in pre.cells.iter().enumerate() {
        cells[i] = o.e;
    }
    let mut best = 0;
    rec(m, pre.d(), pre.used(), &mut cells, pre.score(m), &mut best);
    best
}
