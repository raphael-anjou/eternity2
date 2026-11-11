//! Row-major DFS with forward checking, its Luby-restart variant and the
//! ProbCut variant, all counting through the hard-ladder `NodeCounter`.

use eternity2_engine::{rotated, Puzzle, BORDER};
use hard_ladder::NodeCounter;

/// Oriented-piece tables for one puzzle.
pub struct Tables {
    pub w: usize,
    pub h: usize,
    nc: usize,
    pub edges: Vec<[u8; 4]>,
    pub piece: Vec<usize>,
    pub rot: Vec<u8>,
    /// Oriented ids by (top, left) colour, in (piece, rotation) order.
    by_tl: Vec<Vec<u32>>,
    /// Oriented ids by top colour, in (piece, rotation) order.
    by_t: Vec<Vec<u32>>,
}

impl Tables {
    #[must_use]
    pub fn new(p: &Puzzle) -> Self {
        let nc = p
            .pieces
            .iter()
            .flat_map(|e| e.iter())
            .copied()
            .max()
            .unwrap_or(0) as usize
            + 1;
        let (mut edges, mut piece, mut rot) = (Vec::new(), Vec::new(), Vec::new());
        for (i, &e0) in p.pieces.iter().enumerate() {
            for r in 0..4u8 {
                let e = rotated(e0, r);
                if (0..r).any(|q| rotated(e0, q) == e) {
                    continue;
                }
                edges.push(e);
                piece.push(i);
                rot.push(r);
            }
        }
        let mut by_tl = vec![Vec::new(); nc * nc];
        let mut by_t = vec![Vec::new(); nc];
        for (o, e) in edges.iter().enumerate() {
            by_tl[e[0] as usize * nc + e[3] as usize].push(o as u32);
            by_t[e[0] as usize].push(o as u32);
        }
        Self {
            w: p.width as usize,
            h: p.height as usize,
            nc,
            edges,
            piece,
            rot,
            by_tl,
            by_t,
        }
    }

    /// Oriented id of `piece` with edges `e` (planted boards use piece*4+rot).
    #[must_use]
    pub fn oriented(&self, piece: usize, e: [u8; 4]) -> u32 {
        (0..self.edges.len())
            .find(|&o| self.piece[o] == piece && self.edges[o] == e)
            .expect("orientation") as u32
    }
}

pub struct Board<'a> {
    pub t: &'a Tables,
    pub cells: Vec<i32>,
    pub used: Vec<bool>,
    pub fc: bool,
    pub fit_tests: u64,
}

impl<'a> Board<'a> {
    #[must_use]
    pub fn new(t: &'a Tables, fc: bool) -> Self {
        Self {
            t,
            cells: vec![-1; t.w * t.h],
            used: vec![false; t.w * t.h],
            fc,
            fit_tests: 0,
        }
    }

    #[inline]
    fn rim_ok(&self, e: &[u8; 4], x: usize, y: usize, left_known: bool) -> bool {
        let t = self.t;
        (e[1] == BORDER) == (x == t.w - 1)
            && (e[2] == BORDER) == (y == t.h - 1)
            && (left_known || ((e[3] == BORDER) == (x == 0)))
    }

    /// Consistent unused oriented pieces for cell `pos` (row-major: left and
    /// top neighbours are the only assigned ones), in (piece, rotation) order.
    pub fn candidates(&mut self, pos: usize, out: &mut Vec<u32>) {
        out.clear();
        let t = self.t;
        let (x, y) = (pos % t.w, pos / t.w);
        let top = if y == 0 {
            BORDER
        } else {
            t.edges[self.cells[pos - t.w] as usize][2]
        };
        let left = if x == 0 {
            BORDER
        } else {
            t.edges[self.cells[pos - 1] as usize][1]
        };
        for &o in &t.by_tl[top as usize * t.nc + left as usize] {
            self.fit_tests += 1;
            if !self.used[t.piece[o as usize]] && self.rim_ok(&t.edges[o as usize], x, y, true) {
                out.push(o);
            }
        }
    }

    fn any_candidate(&mut self, pos: usize) -> bool {
        let t = self.t;
        let (x, y) = (pos % t.w, pos / t.w);
        let top = if y == 0 {
            BORDER
        } else {
            t.edges[self.cells[pos - t.w] as usize][2]
        };
        let left = if x == 0 {
            BORDER
        } else {
            t.edges[self.cells[pos - 1] as usize][1]
        };
        for &o in &t.by_tl[top as usize * t.nc + left as usize] {
            self.fit_tests += 1;
            if !self.used[t.piece[o as usize]] && self.rim_ok(&t.edges[o as usize], x, y, true) {
                return true;
            }
        }
        false
    }

    /// Next-row cell (x, y + 1) with only its top neighbour assigned.
    fn any_below(&mut self, x: usize, y: usize) -> bool {
        let t = self.t;
        let top = t.edges[self.cells[y * t.w + x] as usize][2];
        for &o in &t.by_t[top as usize] {
            self.fit_tests += 1;
            let e = &t.edges[o as usize];
            if !self.used[t.piece[o as usize]] && self.rim_ok(e, x, y + 1, false) {
                return true;
            }
        }
        false
    }

    /// Forward check after placing at `pos`: the next cell of the row and
    /// every pending next-row cell keep at least one candidate.
    pub fn forward_ok(&mut self, pos: usize) -> bool {
        if !self.fc {
            return true;
        }
        let t = self.t;
        let (x, y) = (pos % t.w, pos / t.w);
        if x + 1 < t.w && !self.any_candidate(pos + 1) {
            return false;
        }
        if y + 1 < t.h {
            for xx in 0..=x {
                if !self.any_below(xx, y) {
                    return false;
                }
            }
        }
        true
    }

    #[inline]
    pub fn place(&mut self, pos: usize, o: u32) {
        self.cells[pos] = o as i32;
        self.used[self.t.piece[o as usize]] = true;
    }

    #[inline]
    pub fn unplace(&mut self, pos: usize) {
        let o = self.cells[pos] as usize;
        self.used[self.t.piece[o]] = false;
        self.cells[pos] = -1;
    }

    #[must_use]
    pub fn ladder_board(&self) -> Vec<i32> {
        self.cells
            .iter()
            .map(|&o| (self.t.piece[o as usize] * 4) as i32 + i32::from(self.t.rot[o as usize]))
            .collect()
    }
}

/// Counter abstraction: the ladder counter in search, a plain limit in stage 1.
pub trait Count {
    fn take(&mut self) -> bool;
}

impl Count for NodeCounter {
    #[inline]
    fn take(&mut self) -> bool {
        self.try_assign()
    }
}

/// Plain budgeted counter for offline measurements.
pub struct Budget {
    pub left: u64,
}

impl Count for Budget {
    #[inline]
    fn take(&mut self) -> bool {
        if self.left == 0 {
            false
        } else {
            self.left -= 1;
            true
        }
    }
}

/// Number of forward-checked nodes exactly `k` cells below the current
/// partial board (whose next empty cell is `pos`), stopping once `stop` is
/// reached. Every placement is charged to `c`. `Err(partial)` = budget exhausted.
pub fn count_deeper<C: Count>(
    b: &mut Board,
    pos: usize,
    k: usize,
    stop: u64,
    c: &mut C,
) -> Result<u64, u64> {
    if k == 0 || pos == b.cells.len() {
        return Ok(u64::from(k == 0));
    }
    let mut cands = Vec::new();
    b.candidates(pos, &mut cands);
    let mut total = 0u64;
    for o in cands {
        if !c.take() {
            return Err(total);
        }
        b.place(pos, o);
        let sub = if b.forward_ok(pos) {
            count_deeper(b, pos + 1, k - 1, stop - total, c)
        } else {
            Ok(0)
        };
        b.unplace(pos);
        match sub {
            Ok(v) => total += v,
            Err(v) => return Err(total + v),
        }
        if total >= stop {
            return Ok(total);
        }
    }
    Ok(total)
}

/// One ProbCut test: at `filled` cells, cut when log2(1 + count at depth
/// `probe` below) < `t`.
#[derive(Debug, Clone, Copy)]
pub struct CutTest {
    pub filled: usize,
    pub probe: usize,
    pub t: f64,
}

pub enum Step {
    Found,
    Failed,
    Cap,
    /// Local (restart) budget exhausted.
    Limit,
}

pub struct Dfs<'a, 'b> {
    pub b: Board<'a>,
    pub c: &'b mut NodeCounter,
    pub cuts: Vec<CutTest>,
    pub cut_fired: u64,
    pub probes: u64,
    /// Luby: local node limit (u64::MAX = none) and order randomisation.
    pub local_left: u64,
    pub rng: Option<u64>,
}

impl Dfs<'_, '_> {
    fn next_rand(&mut self) -> u64 {
        let s = self.rng.as_mut().expect("rng");
        *s ^= *s << 13;
        *s ^= *s >> 7;
        *s ^= *s << 17;
        *s
    }

    pub fn run(&mut self, pos: usize) -> Step {
        if pos == self.b.cells.len() {
            return Step::Found;
        }
        if let Some(ct) = self.cuts.iter().find(|ct| ct.filled == pos).copied() {
            if ct.t > 0.0 {
                self.probes += 1;
                // v < t  <=>  count < 2^t - 1; stop the probe once count reaches it.
                let need = (ct.t.exp2() - 1.0).ceil().max(1.0) as u64;
                match count_deeper(&mut self.b, pos, ct.probe, need, self.c) {
                    Err(_) => return Step::Cap,
                    Ok(cnt) => {
                        if (1.0 + cnt as f64).log2() < ct.t {
                            self.cut_fired += 1;
                            return Step::Failed;
                        }
                    }
                }
            }
        }
        let mut cands = Vec::new();
        self.b.candidates(pos, &mut cands);
        let len = cands.len();
        let off = if self.rng.is_some() && len > 1 {
            (self.next_rand() % len as u64) as usize
        } else {
            0
        };
        for i in 0..len {
            let o = cands[(i + off) % len];
            if self.local_left == 0 {
                return Step::Limit;
            }
            if !self.c.try_assign() {
                return Step::Cap;
            }
            self.local_left = self.local_left.saturating_sub(1);
            self.b.place(pos, o);
            if self.b.forward_ok(pos) {
                match self.run(pos + 1) {
                    Step::Failed => {}
                    done => {
                        if !matches!(done, Step::Found) {
                            self.b.unplace(pos);
                        }
                        return done;
                    }
                }
            }
            self.b.unplace(pos);
            self.c.backtracks += 1;
        }
        Step::Failed
    }
}

/// Luby sequence 1,1,2,1,1,2,4,... (1-based).
#[must_use]
pub fn luby(mut i: u64) -> u64 {
    loop {
        let mut k = 1u32;
        while (1u64 << k) - 1 < i {
            k += 1;
        }
        if i == (1u64 << k) - 1 {
            return 1u64 << (k - 1);
        }
        i -= (1u64 << (k - 1)) - 1;
    }
}
