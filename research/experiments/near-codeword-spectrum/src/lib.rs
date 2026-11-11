//! Near-codeword spectrum: shared board geometry, instance statistics, the
//! annealed first-moment baseline and the stage-3 samplers. The CP-SAT model
//! lives in `scripts/cpsat_spectrum.py`.

use eternity2_engine::generator::{generate_solved_framed, XorShift};
use eternity2_engine::types::{rotated, BORDER};

pub type Edges = [u8; 4];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Class {
    Corner,
    Edge,
    Interior,
}

pub fn cell_class(n: usize, c: usize) -> Class {
    let (x, y) = (c % n, c / n);
    let rx = x == 0 || x == n - 1;
    let ry = y == 0 || y == n - 1;
    match (rx, ry) {
        (true, true) => Class::Corner,
        (false, false) => Class::Interior,
        _ => Class::Edge,
    }
}

pub fn piece_class(e: Edges) -> Class {
    match e.iter().filter(|&&c| c == BORDER).count() {
        2 => Class::Corner,
        1 => Class::Edge,
        _ => Class::Interior,
    }
}

/// Outward (gray) sides of a cell, URDL.
pub fn outward(n: usize, c: usize) -> [bool; 4] {
    let (x, y) = (c % n, c / n);
    [y == 0, x == n - 1, y == n - 1, x == 0]
}

pub fn legal(n: usize, c: usize, e: Edges) -> bool {
    let o = outward(n, c);
    (0..4).all(|s| o[s] == (e[s] == BORDER))
}

/// An internal joint: (cell a, side of a, cell b, side of b, along-frame?).
#[derive(Clone, Copy, Debug)]
pub struct Joint {
    pub a: usize,
    pub sa: usize,
    pub b: usize,
    pub sb: usize,
    pub frame: bool,
}

pub fn joints(n: usize) -> Vec<Joint> {
    let mut v = Vec::new();
    for y in 0..n {
        for x in 0..n - 1 {
            let c = y * n + x;
            v.push(Joint { a: c, sa: 1, b: c + 1, sb: 3, frame: y == 0 || y == n - 1 });
        }
    }
    for y in 0..n - 1 {
        for x in 0..n {
            let c = y * n + x;
            v.push(Joint { a: c, sa: 2, b: c + n, sb: 0, frame: x == 0 || x == n - 1 });
        }
    }
    v
}

pub struct Instance {
    pub n: usize,
    pub colors: u8,
    pub seed: u32,
    /// Piece i at cell i in rotation 0 is the planted solution.
    pub pieces: Vec<Edges>,
}

impl Instance {
    pub fn generate(n: usize, colors: u8, seed: u32) -> Self {
        let p = generate_solved_framed(n as u8, colors, seed, true);
        Instance { n, colors, seed, pieces: p.pieces }
    }

    /// Rotation-duplicate pieces: pieces minus distinct canonical keys.
    pub fn rotation_duplicates(&self) -> usize {
        let mut keys: Vec<Edges> = self
            .pieces
            .iter()
            .map(|&e| (0..4).map(|r| rotated(e, r)).min().unwrap())
            .collect();
        keys.sort_unstable();
        keys.dedup();
        self.pieces.len() - keys.len()
    }

    /// (p_f, p_i): collision probabilities of along-frame half-edges and of
    /// every other non-gray half-edge, from the planted board.
    pub fn collision_probs(&self) -> (f64, f64) {
        let mut f = vec![0u64; 256];
        let mut i = vec![0u64; 256];
        for j in joints(self.n) {
            let ca = self.pieces[j.a][j.sa];
            let cb = self.pieces[j.b][j.sb];
            let t = if j.frame { &mut f } else { &mut i };
            t[ca as usize] += 1;
            t[cb as usize] += 1;
        }
        (collision(&f), collision(&i))
    }

    /// Interior-tile collision q_int and mu = 4 (n-2)^2 q_int^4.
    pub fn mu(&self) -> (f64, f64) {
        let mut cnt = vec![0u64; 256];
        for (c, e) in self.pieces.iter().enumerate() {
            if cell_class(self.n, c) == Class::Interior {
                for &k in e {
                    cnt[k as usize] += 1;
                }
            }
        }
        let q = collision(&cnt);
        let m = (self.n - 2) * (self.n - 2);
        (q, 4.0 * m as f64 * q.powi(4))
    }
}

fn collision(cnt: &[u64]) -> f64 {
    let t: u64 = cnt.iter().sum();
    if t == 0 {
        return 0.0;
    }
    cnt.iter().map(|&k| (k as f64 / t as f64).powi(2)).sum()
}

// ---------------------------------------------------------------- annealed

fn derangements(k: usize) -> f64 {
    let mut d = vec![1.0f64, 0.0];
    for m in 2..=k {
        let v = (m as f64 - 1.0) * (d[m - 1] + d[m - 2]);
        d.push(v);
    }
    d[k]
}

fn choose(n: usize, k: usize) -> f64 {
    (0..k).fold(1.0, |acc, j| acc * (n - j) as f64 / (j + 1) as f64)
}

fn factorial(k: usize) -> f64 {
    (1..=k).fold(1.0, |acc, j| acc * j as f64)
}

/// Arrangements of k interior pieces with rotations, no cell keeping its own
/// piece in rotation 0.
fn interior_changed(k: usize) -> f64 {
    (0..=k)
        .map(|j| {
            let s = if j % 2 == 0 { 1.0 } else { -1.0 };
            s * choose(k, j) * factorial(k - j) * 4f64.powi((k - j) as i32)
        })
        .sum()
}

/// Annealed first-moment E(a, b): returns a matrix e[a][b] (a = 0..N, b =
/// 0..joints). Exact sum over all subsets S of cells (cell 0 excluded) by a
/// row-mask transfer matrix.
pub fn annealed(n: usize, pf: f64, pi: f64) -> Vec<Vec<f64>> {
    let nc = 3usize; // free corners
    let ne = 4 * (n - 2);
    let ni = (n - 2) * (n - 2);
    let nj = 2 * n * (n - 1);
    let nm = 1usize << n;
    // factor polynomials (pf + (1-pf) y)^jf (pi + (1-pi) y)^ji
    let mut factor = vec![vec![Vec::new(); 2 * n + 1]; 2 * n + 1];
    for (jf, row) in factor.iter_mut().enumerate() {
        for (ji, slot) in row.iter_mut().enumerate() {
            let mut p = vec![1.0];
            for _ in 0..jf {
                p = mul(&p, &[pf, 1.0 - pf]);
            }
            for _ in 0..ji {
                p = mul(&p, &[pi, 1.0 - pi]);
            }
            *slot = p;
        }
    }
    let idx = |m: usize, sc: usize, se: usize, si: usize| (((m * (nc + 1) + sc) * (ne + 1) + se) * (ni + 1)) + si;
    let nstate = nm * (nc + 1) * (ne + 1) * (ni + 1);
    let mut cur: Vec<Option<Vec<f64>>> = vec![None; nstate];
    cur[idx(0, 0, 0, 0)] = Some(vec![1.0]);
    for y in 0..n {
        let mut next: Vec<Option<Vec<f64>>> = vec![None; nstate];
        for m in 0..nm {
            if y == 0 && m & 1 == 1 {
                continue;
            }
            let (mut dc, mut de, mut di) = (0, 0, 0);
            for x in 0..n {
                if m >> x & 1 == 1 {
                    match cell_class(n, y * n + x) {
                        Class::Corner => dc += 1,
                        Class::Edge => de += 1,
                        Class::Interior => di += 1,
                    }
                }
            }
            let rim_row = y == 0 || y == n - 1;
            let mut hf = 0;
            let mut hi = 0;
            for x in 0..n - 1 {
                if (m >> x) & 3 != 0 {
                    if rim_row {
                        hf += 1;
                    } else {
                        hi += 1;
                    }
                }
            }
            for pm in 0..nm {
                if y == 0 && pm != 0 {
                    continue;
                }
                let (mut vf, mut vi) = (0, 0);
                if y > 0 {
                    let t = pm | m;
                    for x in 0..n {
                        if t >> x & 1 == 1 {
                            if x == 0 || x == n - 1 {
                                vf += 1;
                            } else {
                                vi += 1;
                            }
                        }
                    }
                }
                let fac = &factor[hf + vf][hi + vi];
                for sc in 0..=nc - dc.min(nc) {
                    for se in 0..=ne - de.min(ne) {
                        for si in 0..=ni - di.min(ni) {
                            let Some(p) = &cur[idx(pm, sc, se, si)] else { continue };
                            let q = mul(p, fac);
                            let slot = &mut next[idx(m, sc + dc, se + de, si + di)];
                            match slot {
                                Some(acc) => add_into(acc, &q),
                                None => *slot = Some(q),
                            }
                        }
                    }
                }
            }
        }
        cur = next;
    }
    let nn = n * n;
    let mut e = vec![vec![0.0f64; nj + 1]; nn + 1];
    for m in 0..nm {
        for sc in 0..=nc {
            for se in 0..=ne {
                for si in 0..=ni {
                    let Some(p) = &cur[idx(m, sc, se, si)] else { continue };
                    let f = derangements(sc) * derangements(se) * interior_changed(si);
                    let a = sc + se + si;
                    for (b, &v) in p.iter().enumerate() {
                        e[a][b] += f * v;
                    }
                }
            }
        }
    }
    e
}

fn mul(a: &[f64], b: &[f64]) -> Vec<f64> {
    let mut r = vec![0.0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        if x == 0.0 {
            continue;
        }
        for (j, &y) in b.iter().enumerate() {
            r[i + j] += x * y;
        }
    }
    r
}

fn add_into(acc: &mut Vec<f64>, q: &[f64]) {
    if acc.len() < q.len() {
        acc.resize(q.len(), 0.0);
    }
    for (a, &v) in acc.iter_mut().zip(q) {
        *a += v;
    }
}

/// b_ann(a) = smallest b with E(a, b) >= 1 (None if no b reaches 1).
pub fn b_ann(e: &[Vec<f64>]) -> Vec<Option<usize>> {
    e.iter().map(|row| row.iter().position(|&v| v >= 1.0)).collect()
}

// ---------------------------------------------------------------- boards

/// A full board: cell -> (piece, rotation), with an incremental break count
/// and incremental distance counters for the 4 whole-board rotations.
pub struct Board<'a> {
    pub inst: &'a Instance,
    pub cell: Vec<(usize, u8)>,
    pub breaks: u32,
    joints_of: Vec<Vec<usize>>,
    joints: Vec<Joint>,
    /// same[k] = cells c where the board rotated by k quarter turns matches the
    /// planted tuple at the image of c.
    same: [usize; 4],
    pos_of: Vec<usize>,
}

/// Image of cell c under a clockwise quarter turn of the whole board.
fn rot_cell(n: usize, c: usize) -> usize {
    let (x, y) = (c % n, c / n);
    x * n + (n - 1 - y)
}

impl<'a> Board<'a> {
    pub fn new(inst: &'a Instance, cell: Vec<(usize, u8)>) -> Self {
        let n = inst.n;
        let js = joints(n);
        let mut joints_of = vec![Vec::new(); n * n];
        for (k, j) in js.iter().enumerate() {
            joints_of[j.a].push(k);
            joints_of[j.b].push(k);
        }
        let mut pos_of = vec![0; n * n];
        for (c, &(p, _)) in cell.iter().enumerate() {
            pos_of[p] = c;
        }
        let mut b = Board { inst, cell, breaks: 0, joints_of, joints: js, same: [0; 4], pos_of };
        b.breaks = (0..b.joints.len()).filter(|&k| b.joint_broken(k)).count() as u32;
        for c in 0..n * n {
            for k in 0..4 {
                b.same[k] += usize::from(b.matches(c, k));
            }
        }
        b
    }

    pub fn tuple(&self, c: usize) -> Edges {
        let (p, r) = self.cell[c];
        rotated(self.inst.pieces[p], r)
    }

    fn joint_broken(&self, k: usize) -> bool {
        let j = self.joints[k];
        self.tuple(j.a)[j.sa] != self.tuple(j.b)[j.sb]
    }

    fn matches(&self, c: usize, k: usize) -> bool {
        let mut d = c;
        for _ in 0..k {
            d = rot_cell(self.inst.n, d);
        }
        rotated(self.tuple(c), k as u8) == self.inst.pieces[d]
    }

    /// Distance after rotating the board so the planted cell-0 corner piece
    /// (piece 0) sits at cell 0.
    pub fn distance(&self) -> usize {
        let n = self.inst.n;
        let mut c = self.pos_of[0];
        let mut k = 0;
        while c != 0 {
            c = rot_cell(n, c);
            k += 1;
        }
        n * n - self.same[k]
    }

    fn local(&self, cells: &[usize]) -> u32 {
        let mut ks: Vec<usize> = cells.iter().flat_map(|&c| self.joints_of[c].iter().copied()).collect();
        ks.sort_unstable();
        ks.dedup();
        ks.into_iter().filter(|&k| self.joint_broken(k)).count() as u32
    }

    /// Replace the contents of `cells` with `new` and keep counters exact.
    pub fn set(&mut self, cells: &[usize], new: &[(usize, u8)]) {
        let before = self.local(cells);
        for &c in cells {
            for k in 0..4 {
                self.same[k] -= usize::from(self.matches(c, k));
            }
        }
        for (&c, &v) in cells.iter().zip(new) {
            self.cell[c] = v;
            self.pos_of[v.0] = c;
        }
        for &c in cells {
            for k in 0..4 {
                self.same[k] += usize::from(self.matches(c, k));
            }
        }
        let after = self.local(cells);
        self.breaks = self.breaks + after - before;
    }

    /// Break change if `cells` took `new`, without committing.
    pub fn delta(&mut self, cells: &[usize], new: &[(usize, u8)]) -> i64 {
        let old: Vec<(usize, u8)> = cells.iter().map(|&c| self.cell[c]).collect();
        let before = self.local(cells);
        for (&c, &v) in cells.iter().zip(new) {
            self.cell[c] = v;
        }
        let after = self.local(cells);
        for (&c, &v) in cells.iter().zip(&old) {
            self.cell[c] = v;
        }
        i64::from(after) - i64::from(before)
    }

    pub fn full_recount(&self) -> (u32, usize) {
        let br = (0..self.joints.len()).filter(|&k| self.joint_broken(k)).count() as u32;
        let n = self.inst.n;
        let mut c = self.pos_of[0];
        let mut k = 0;
        while c != 0 {
            c = rot_cell(n, c);
            k += 1;
        }
        let same = (0..n * n).filter(|&c| self.matches(c, k)).count();
        (br, n * n - same)
    }
}

/// Legal rotation of piece p at cell c (border pieces have exactly one).
pub fn border_rot(inst: &Instance, p: usize, c: usize) -> u8 {
    (0..4u8)
        .find(|&r| legal(inst.n, c, rotated(inst.pieces[p], r)))
        .expect("border piece has a legal rotation")
}

pub fn class_cells(n: usize) -> [Vec<usize>; 3] {
    let mut v = [Vec::new(), Vec::new(), Vec::new()];
    for c in 0..n * n {
        v[cell_class(n, c) as usize].push(c);
    }
    v
}

pub fn random_board(inst: &Instance, rng: &mut XorShift) -> Vec<(usize, u8)> {
    let n = inst.n;
    let cc = class_cells(n);
    let mut cell = vec![(0usize, 0u8); n * n];
    for cells in &cc {
        let mut ps: Vec<usize> = cells.clone();
        rng.shuffle(&mut ps);
        for (&c, &p) in cells.iter().zip(&ps) {
            let r = if cell_class(n, c) == Class::Interior { rng.below(4) as u8 } else { border_rot(inst, p, c) };
            cell[c] = (p, r);
        }
    }
    cell
}

/// Per-run record of visited states.
pub struct Tally {
    pub n2: usize,
    pub min_b_by_a: Vec<i64>,
    /// visits[b][a]
    pub visits: Vec<Vec<u64>>,
    pub best: (u32, usize),
}

impl Tally {
    pub fn new(n: usize) -> Self {
        let nj = 2 * n * (n - 1);
        Tally {
            n2: n * n,
            min_b_by_a: vec![-1; n * n + 1],
            visits: vec![vec![0; n * n + 1]; nj + 1],
            best: (u32::MAX, 0),
        }
    }
    pub fn see(&mut self, b: u32, a: usize) {
        let m = &mut self.min_b_by_a[a];
        if *m < 0 || i64::from(b) < *m {
            *m = i64::from(b);
        }
        self.visits[b as usize][a] += 1;
        if b < self.best.0 {
            self.best = (b, a);
        }
    }
    /// Visits with breaks <= best + 2, per distance a.
    pub fn low_by_a(&self) -> Vec<u64> {
        let mut v = vec![0u64; self.n2 + 1];
        if self.best.0 == u32::MAX {
            return v;
        }
        for b in 0..=(self.best.0 as usize + 2).min(self.visits.len() - 1) {
            for (a, &k) in self.visits[b].iter().enumerate() {
                v[a] += k;
            }
        }
        v
    }

    /// Overlap histogram (10 bins of 1 - a/N, last bin includes overlap 1) of
    /// visits with breaks <= best + 2.
    pub fn low_hist(&self) -> Vec<u64> {
        let mut h = vec![0u64; 10];
        if self.best.0 == u32::MAX {
            return h;
        }
        for b in 0..=(self.best.0 as usize + 2).min(self.visits.len() - 1) {
            for (a, &v) in self.visits[b].iter().enumerate() {
                h[overlap_bin(a, self.n2)] += v;
            }
        }
        h
    }
}

pub fn overlap_bin(a: usize, n2: usize) -> usize {
    let ov = 1.0 - a as f64 / n2 as f64;
    ((ov * 10.0).floor() as usize).min(9)
}

fn uniform(rng: &mut XorShift) -> f64 {
    f64::from(rng.next_u32()) / 4_294_967_296.0
}

/// Plain simulated annealing on swaps and interior rotations.
pub fn anneal(inst: &Instance, seed: u32, attempts: u64, t0: f64, t1: f64) -> Tally {
    let n = inst.n;
    let mut rng = XorShift::new(seed);
    let start = random_board(inst, &mut rng);
    let mut b = Board::new(inst, start);
    let cc = class_cells(n);
    let mut tally = Tally::new(n);
    tally.see(b.breaks, b.distance());
    let ratio = t1 / t0;
    for t in 0..attempts {
        let temp = t0 * ratio.powf(t as f64 / attempts as f64);
        let (cells, new) = propose(inst, &b, &cc, &mut rng);
        if cells.is_empty() {
            continue;
        }
        let d = b.delta(&cells, &new);
        if d <= 0 || uniform(&mut rng) < (-(d as f64) / temp).exp() {
            b.set(&cells, &new);
            tally.see(b.breaks, b.distance());
        }
    }
    tally
}

fn propose(inst: &Instance, b: &Board, cc: &[Vec<usize>; 3], rng: &mut XorShift) -> (Vec<usize>, Vec<(usize, u8)>) {
    let n = inst.n;
    if rng.below(2) == 0 {
        let c1 = rng.below((n * n) as u32) as usize;
        let cls = cell_class(n, c1) as usize;
        let pool = &cc[cls];
        let c2 = pool[rng.below(pool.len() as u32) as usize];
        if c1 == c2 {
            return (vec![], vec![]);
        }
        swap_move(inst, b, c1, c2)
    } else {
        let pool = &cc[Class::Interior as usize];
        let c = pool[rng.below(pool.len() as u32) as usize];
        let (p, r) = b.cell[c];
        let nr = (r + 1 + rng.below(3) as u8) % 4;
        (vec![c], vec![(p, nr)])
    }
}

fn swap_move(inst: &Instance, b: &Board, c1: usize, c2: usize) -> (Vec<usize>, Vec<(usize, u8)>) {
    let n = inst.n;
    let (p1, r1) = b.cell[c1];
    let (p2, r2) = b.cell[c2];
    if cell_class(n, c1) == Class::Interior {
        (vec![c1, c2], vec![(p2, r2), (p1, r1)])
    } else {
        (vec![c1, c2], vec![(p2, border_rot(inst, p2, c1)), (p1, border_rot(inst, p1, c2))])
    }
}

/// Random legal start polished by first-improvement greedy descent.
pub fn greedy_null(inst: &Instance, seed: u32) -> Tally {
    let n = inst.n;
    let mut rng = XorShift::new(seed);
    let start = random_board(inst, &mut rng);
    let mut b = Board::new(inst, start);
    let cc = class_cells(n);
    let mut moves: Vec<(usize, usize, u8)> = Vec::new(); // (c1, c2, rot) ; c1==c2 => rotate by rot
    for pool in &cc {
        for i in 0..pool.len() {
            for j in i + 1..pool.len() {
                moves.push((pool[i], pool[j], 0));
            }
        }
    }
    for &c in &cc[Class::Interior as usize] {
        for r in 1..4 {
            moves.push((c, c, r));
        }
    }
    let mut tally = Tally::new(n);
    tally.see(b.breaks, b.distance());
    loop {
        rng.shuffle(&mut moves);
        let mut improved = false;
        for &(c1, c2, r) in &moves {
            let (cells, new) = if c1 == c2 {
                let (p, r0) = b.cell[c1];
                (vec![c1], vec![(p, (r0 + r) % 4)])
            } else {
                swap_move(inst, &b, c1, c2)
            };
            if b.delta(&cells, &new) < 0 {
                b.set(&cells, &new);
                tally.see(b.breaks, b.distance());
                improved = true;
                break;
            }
        }
        if !improved {
            break;
        }
    }
    tally
}

/// Engine break-tolerant DFS on a shuffled piece list; returns the final
/// board as (piece, rot) per cell if the run completed.
pub fn break_dfs(inst: &Instance, seed: u32, steps: u64) -> Option<Vec<(usize, u8)>> {
    use eternity2_engine::solver::Status;
    use eternity2_engine::types::Puzzle;
    let n = inst.n;
    let mut rng = XorShift::new(seed);
    let mut perm: Vec<usize> = (0..n * n).collect();
    rng.shuffle(&mut perm);
    let puzzle = Puzzle {
        name: "ncs".into(),
        width: n as u8,
        height: n as u8,
        num_colors: inst.colors,
        pieces: perm.iter().map(|&p| inst.pieces[p]).collect(),
        hints: vec![],
    };
    let path = eternity2_engine::build_path("snake", n as u8, n as u8, 0).expect("snake path");
    let breaks: Vec<u16> = path[n * n - 2 * n..].to_vec();
    let mut s = eternity2_engine::BreakSolver::new(&puzzle, &path, false, &breaks).expect("solver");
    let mut done = 0u64;
    let mut status = s.report().status;
    while status == Status::Running && done < steps {
        let chunk = (steps - done).min(1_000_000) as u32;
        status = s.step(chunk).status;
        done += u64::from(chunk);
    }
    if status != Status::Solved {
        return None;
    }
    Some(
        s.board()
            .iter()
            .map(|&code| (perm[(code / 4) as usize], (code % 4) as u8))
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn planted(inst: &Instance) -> Vec<(usize, u8)> {
        (0..inst.pieces.len()).map(|p| (p, 0)).collect()
    }

    #[test]
    fn planted_has_zero_breaks_and_distance() {
        let inst = Instance::generate(6, 14, 1001);
        let b = Board::new(&inst, planted(&inst));
        assert_eq!(b.breaks, 0);
        assert_eq!(b.distance(), 0);
    }

    #[test]
    fn rotated_planted_has_zero_distance() {
        let inst = Instance::generate(5, 13, 1002);
        let n = inst.n;
        let mut cell = planted(&inst);
        for _ in 0..3 {
            let mut next = vec![(0, 0); n * n];
            for (c, &(p, r)) in cell.iter().enumerate() {
                next[rot_cell(n, c)] = (p, (r + 1) % 4);
            }
            cell = next;
            let b = Board::new(&inst, cell.clone());
            assert_eq!(b.breaks, 0);
            assert_eq!(b.distance(), 0);
        }
    }

    #[test]
    fn incremental_counters_match_recount() {
        let inst = Instance::generate(6, 14, 1003);
        let t = {
            let mut rng = XorShift::new(7);
            let mut b = Board::new(&inst, random_board(&inst, &mut rng));
            let cc = class_cells(6);
            for _ in 0..5000 {
                let (cells, new) = propose(&inst, &b, &cc, &mut rng);
                if cells.is_empty() {
                    continue;
                }
                let d = b.delta(&cells, &new);
                let before = b.breaks;
                b.set(&cells, &new);
                assert_eq!(i64::from(b.breaks) - i64::from(before), d);
                assert_eq!(b.full_recount(), (b.breaks, b.distance()));
                for c in 0..36 {
                    assert!(legal(6, c, b.tuple(c)));
                }
            }
            b.breaks
        };
        assert!(t > 0);
    }

    #[test]
    fn annealed_matches_brute_force_on_4x4() {
        let n = 4;
        let (pf, pi) = (0.3, 0.15);
        let e = annealed(n, pf, pi);
        let js = joints(n);
        let mut brute = vec![vec![0.0f64; js.len() + 1]; n * n + 1];
        for s in 0u32..(1 << (n * n)) {
            if s & 1 == 1 {
                continue;
            }
            let inside = |c: usize| s >> c & 1 == 1;
            let (mut sc, mut se, mut si) = (0, 0, 0);
            for c in 0..n * n {
                if inside(c) {
                    match cell_class(n, c) {
                        Class::Corner => sc += 1,
                        Class::Edge => se += 1,
                        Class::Interior => si += 1,
                    }
                }
            }
            let mut poly = vec![1.0];
            for j in &js {
                if inside(j.a) || inside(j.b) {
                    let p = if j.frame { pf } else { pi };
                    poly = mul(&poly, &[p, 1.0 - p]);
                }
            }
            let f = derangements(sc) * derangements(se) * interior_changed(si);
            for (b, &v) in poly.iter().enumerate() {
                brute[sc + se + si][b] += f * v;
            }
        }
        for a in 0..=n * n {
            for b in 0..=js.len() {
                let (x, y) = (e[a][b], brute[a][b]);
                assert!((x - y).abs() <= 1e-9 * y.abs().max(1.0), "a={a} b={b} {x} {y}");
            }
        }
    }

    #[test]
    fn interior_changed_small_values() {
        // k=1: 3 non-identity rotations. k=2: 2!*16 - 2*1!*4 + 1 = 25.
        assert!((interior_changed(1) - 3.0).abs() < 1e-9);
        assert!((interior_changed(2) - 25.0).abs() < 1e-9);
        assert!((derangements(3) - 2.0).abs() < 1e-9);
    }
}
