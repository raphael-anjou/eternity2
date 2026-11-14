//! The preregistered statistics of a partial board (PREREG.md, section 4).
//! Every function reads only the state (placed and fixed cells, used tiles),
//! the tile set and the scan order; none reads the planted solution.
//!
//! Lookahead bounds: S1 one cell, S8 four scan cells, S10 one row (n scan
//! cells, node cap), S7 propagation to a fixed point (polynomial, no search),
//! the others none.

use crate::puzzle::{rotate, Edges};
use crate::tree::{State, Tree};

pub const NONE: u8 = 255;
pub const NSTAT: usize = 11;
pub const NAMES: [&str; NSTAT] = [
    "s1_next_fits",
    "s2_frontier_fits",
    "s3_hall_surplus",
    "s4_balance_chi2",
    "s5_goodness_used",
    "s6_first_moment",
    "s7_ac_domains",
    "s8_block4_fills",
    "s9_colour_entropy",
    "s10_row_lookahead",
    "h0_hash",
];

pub struct Ctx {
    pub n: usize,
    pub ncol: usize,
    pub pieces: Vec<Edges>,
    /// distinct orientations per tile
    pub orients: Vec<Vec<Edges>>,
    pub frame_col: Vec<bool>,
    /// share of the colour among all joints of its band (frame or interior)
    pub pi: Vec<f64>,
    /// standardised goodness per tile (S5)
    pub good_z: Vec<f64>,
    pub is_clue: Vec<bool>,
    lnfact: Vec<f64>,
    pub s10_cap: u64,
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

fn kind(e: &Edges) -> usize {
    e.iter().filter(|&&c| c == 0).count() // 0 interior, 1 edge, 2 corner
}

impl Ctx {
    pub fn new(n: usize, pieces: &[Edges], clue_pos: &[usize]) -> Ctx {
        let ncol = pieces.iter().flat_map(|e| e.iter().copied()).max().unwrap() as usize + 1;
        let orients: Vec<Vec<Edges>> = pieces
            .iter()
            .map(|&e| {
                let mut v: Vec<Edges> = Vec::new();
                for r in 0..4 {
                    let re = rotate(e, r);
                    if !v.contains(&re) {
                        v.push(re);
                    }
                }
                v
            })
            .collect();
        // frame colours: on a side adjacent to a grey side
        let mut frame_col = vec![false; ncol];
        for e in pieces {
            for s in 0..4 {
                if e[s] == 0 {
                    for t in [(s + 1) % 4, (s + 3) % 4] {
                        if e[t] != 0 {
                            frame_col[e[t] as usize] = true;
                        }
                    }
                }
            }
        }
        let mut cnt = vec![0f64; ncol];
        for e in pieces {
            for &c in e {
                if c != 0 {
                    cnt[c as usize] += 1.0;
                }
            }
        }
        let tot_f: f64 = (1..ncol).filter(|&c| frame_col[c]).map(|c| cnt[c]).sum();
        let tot_i: f64 = (1..ncol).filter(|&c| !frame_col[c]).map(|c| cnt[c]).sum();
        let pi: Vec<f64> = (0..ncol)
            .map(|c| {
                if c == 0 {
                    0.0
                } else if frame_col[c] {
                    cnt[c] / tot_f
                } else {
                    cnt[c] / tot_i
                }
            })
            .collect();
        let mut lnfact = vec![0f64; 4 * n * n + 8];
        for i in 1..lnfact.len() {
            lnfact[i] = lnfact[i - 1] + (i as f64).ln();
        }
        let mut is_clue = vec![false; n * n];
        for &p in clue_pos {
            is_clue[p] = true;
        }
        let mut ctx = Ctx {
            n,
            ncol,
            pieces: pieces.to_vec(),
            orients,
            frame_col,
            pi,
            good_z: vec![0.0; pieces.len()],
            is_clue,
            lnfact,
            s10_cap: 100_000,
        };
        ctx.good_z = ctx.goodness();
        ctx
    }

    /// Selby-style goodness: ln(1 + number of exact fills of a 2x2 window of
    /// the tile's own type that contain the tile), the window filled from the
    /// whole tile set; standardised (mean 0, sd 1) within each tile type
    /// using this board's own tiles. No solution information, no fitted
    /// parameter.
    fn goodness(&self) -> Vec<f64> {
        let n = self.n;
        let mut count = vec![0f64; self.pieces.len()];
        // windows: (cells as (r,c)), and which window cells the counted type sits in
        let mid = n / 2 - 1;
        let windows: [([(usize, usize); 4], usize); 3] = [
            ([(mid, mid), (mid, mid + 1), (mid + 1, mid), (mid + 1, mid + 1)], 0),
            ([(0, mid), (0, mid + 1), (1, mid), (1, mid + 1)], 1),
            ([(0, 0), (0, 1), (1, 0), (1, 1)], 2),
        ];
        for (cells, want_kind) in windows.iter() {
            let mut chosen: Vec<(usize, Edges)> = Vec::with_capacity(4);
            self.win_rec(cells, 0, &mut chosen, &mut count, *want_kind);
        }
        let g: Vec<f64> = count.iter().map(|&c| (1.0 + c).ln()).collect();
        let mut z = vec![0f64; g.len()];
        for k in 0..3 {
            let idx: Vec<usize> = (0..g.len()).filter(|&i| kind(&self.pieces[i]) == k).collect();
            if idx.is_empty() {
                continue;
            }
            let m = idx.iter().map(|&i| g[i]).sum::<f64>() / idx.len() as f64;
            let v = idx.iter().map(|&i| (g[i] - m).powi(2)).sum::<f64>() / idx.len() as f64;
            let sd = v.sqrt().max(1e-12);
            for &i in &idx {
                z[i] = (g[i] - m) / sd;
            }
        }
        z
    }

    fn win_rec(
        &self,
        cells: &[(usize, usize); 4],
        j: usize,
        chosen: &mut Vec<(usize, Edges)>,
        count: &mut [f64],
        want_kind: usize,
    ) {
        let n = self.n;
        if j == 4 {
            for &(t, _) in chosen.iter() {
                if kind(&self.pieces[t]) == want_kind {
                    count[t] += 1.0;
                }
            }
            return;
        }
        let (r, c) = cells[j];
        for t in 0..self.pieces.len() {
            if chosen.iter().any(|&(u, _)| u == t) {
                continue;
            }
            'o: for &e in &self.orients[t] {
                // rim / non-rim
                let rim = [r == 0, c == n - 1, r == n - 1, c == 0];
                for s in 0..4 {
                    if rim[s] != (e[s] == 0) {
                        continue 'o;
                    }
                }
                // match earlier window cells
                for (jj, &(u_r, u_c)) in cells[..j].iter().enumerate() {
                    let f = chosen[jj].1;
                    if u_r == r && u_c + 1 == c && f[1] != e[3] {
                        continue 'o;
                    }
                    if u_c == c && u_r + 1 == r && f[2] != e[0] {
                        continue 'o;
                    }
                }
                chosen.push((t, e));
                self.win_rec(cells, j + 1, chosen, count, want_kind);
                chosen.pop();
            }
        }
    }

    #[inline]
    fn used(st: &State, t: usize) -> bool {
        st.used[t / 64] >> (t % 64) & 1 == 1
    }

    /// Unary requirement per side of empty cell p: 0 = grey (rim),
    /// NONE = open (any non-grey), else the facing colour.
    fn want(&self, st: &State, p: usize) -> [u8; 4] {
        let mut w = [NONE; 4];
        for s in 0..4 {
            w[s] = match nb(self.n, p, s) {
                None => 0,
                Some(q) => {
                    let c = st.board[q * 4 + (s + 2) % 4];
                    c // NONE if q empty
                }
            };
        }
        w
    }

    #[inline]
    fn fits(e: &Edges, w: &[u8; 4]) -> bool {
        for s in 0..4 {
            let ok = if w[s] == NONE { e[s] != 0 } else { e[s] == w[s] };
            if !ok {
                return false;
            }
        }
        true
    }

    fn empty(st: &State, p: usize) -> bool {
        st.board[p * 4] == NONE
    }

    fn frontier(&self, st: &State) -> Vec<usize> {
        let n = self.n;
        (0..n * n)
            .filter(|&p| {
                Self::empty(st, p)
                    && (0..4).any(|s| matches!(nb(n, p, s), Some(q) if !Self::empty(st, q)))
            })
            .collect()
    }

    fn cand_tiles(&self, st: &State, p: usize) -> (u32, Vec<u16>) {
        let w = self.want(st, p);
        let mut c = 0u32;
        let mut tiles = Vec::new();
        for t in 0..self.pieces.len() {
            if Self::used(st, t) {
                continue;
            }
            let mut any = false;
            for e in &self.orients[t] {
                if Self::fits(e, &w) {
                    c += 1;
                    any = true;
                }
            }
            if any {
                tiles.push(t as u16);
            }
        }
        (c, tiles)
    }

    /// All statistics; `k` = free depth of `st` in tree `t`.
    /// Returns values and nanoseconds per statistic.
    pub fn all(&self, t: &Tree, st: &State, k: usize) -> ([f64; NSTAT], [u64; NSTAT]) {
        let mut v = [0f64; NSTAT];
        let mut ns = [0u64; NSTAT];
        let mut tm = std::time::Instant::now();
        let mut lap = |i: usize, tm: &mut std::time::Instant| {
            ns[i] = tm.elapsed().as_nanos() as u64;
            *tm = std::time::Instant::now();
        };
        v[0] = (1.0 + t.child_count(st, k) as f64).log2();
        lap(0, &mut tm);
        let fr = self.frontier(st);
        let mut adj: Vec<Vec<u16>> = Vec::with_capacity(fr.len());
        let mut s2 = 0f64;
        for &p in &fr {
            let (c, tiles) = self.cand_tiles(st, p);
            s2 += (1.0 + c as f64).log2();
            adj.push(tiles);
        }
        v[1] = s2;
        lap(1, &mut tm);
        v[2] = hall_surplus(&adj, self.pieces.len(), 16) as f64;
        lap(2, &mut tm);
        let (s4, s6) = self.ledger(st);
        v[3] = s4;
        lap(3, &mut tm);
        v[4] = self.goodness_used(st);
        lap(4, &mut tm);
        v[5] = s6;
        lap(5, &mut tm);
        v[6] = self.ac(st);
        lap(6, &mut tm);
        let mut s = st.clone();
        v[7] = (1.0 + t.descendants(&mut s, k, 4, u64::MAX).0 as f64).log2();
        lap(7, &mut tm);
        v[8] = self.entropy(st);
        lap(8, &mut tm);
        v[9] = (1.0 + t.descendants(&mut s, k, self.n, self.s10_cap).0 as f64).log2();
        lap(9, &mut tm);
        v[10] = self.hash(st);
        lap(10, &mut tm);
        (v, ns)
    }

    /// Remaining colour supply r_c (sides of unused tiles) and frontier
    /// demand f_c (open sides facing a known cell).
    fn supply_demand(&self, st: &State) -> (Vec<i64>, Vec<i64>, i64, i64) {
        let n = self.n;
        let mut r = vec![0i64; self.ncol];
        for t in 0..self.pieces.len() {
            if !Self::used(st, t) {
                for &c in &self.pieces[t] {
                    r[c as usize] += 1;
                }
            }
        }
        let mut f = vec![0i64; self.ncol];
        let (mut jf, mut ji) = (0i64, 0i64); // remaining empty-empty joints by band
        for p in 0..n * n {
            if !Self::empty(st, p) {
                continue;
            }
            for s in 0..4 {
                if let Some(q) = nb(n, p, s) {
                    if Self::empty(st, q) {
                        if s == 1 || s == 2 {
                            let band_frame = is_frame_joint(n, p, q);
                            if band_frame {
                                jf += 1;
                            } else {
                                ji += 1;
                            }
                        }
                    } else {
                        f[st.board[q * 4 + (s + 2) % 4] as usize] += 1;
                    }
                }
            }
        }
        (r, f, jf, ji)
    }

    /// A4: census-dead = some colour's frontier demand exceeds its remaining
    /// supply (r_c < f_c), a sound certificate that the prefix has no
    /// completion.
    pub fn census_dead(&self, st: &State) -> bool {
        let (r, f, _, _) = self.supply_demand(st);
        (1..self.ncol).any(|c| r[c] < f[c])
    }

    /// A4: S4, S6, S9 only (the painter control needs nothing else).
    pub fn census3(&self, st: &State) -> [f64; 3] {
        let (s4, s6) = self.ledger(st);
        [s4, s6, self.entropy(st)]
    }

    /// S4 (balance chi-square of remaining joint colours against the board's
    /// colour shares) and S6 (configuration-model log first moment).
    fn ledger(&self, st: &State) -> (f64, f64) {
        let (r, f, jf, ji) = self.supply_demand(st);
        let mut chi = 0f64;
        let mut lm = 0f64;
        for c in 1..self.ncol {
            let x2 = r[c] - f[c]; // twice the remaining internal joints of colour c
            let j = if self.frame_col[c] { jf } else { ji } as f64;
            let e = j * self.pi[c];
            if e > 0.0 {
                chi += (x2 as f64 / 2.0 - e).powi(2) / e;
            }
            // A2: an infeasible colour (x2 < 0) is clamped (the falling
            // factorial runs to r!, no internal pairs), not a -1e6 sentinel.
            let rem = x2.max(0);
            let x = (rem / 2) as usize;
            // ln[ r!/(r-f)! ] + ln[(2x-1)!!]
            lm += self.lnfact[r[c] as usize] - self.lnfact[rem as usize];
            lm += self.lnfact[2 * x] - (x as f64) * std::f64::consts::LN_2 - self.lnfact[x];
        }
        (chi, lm)
    }

    /// S5: mean standardised goodness of the placed (non-clue) tiles.
    fn goodness_used(&self, st: &State) -> f64 {
        let (mut s, mut m) = (0f64, 0usize);
        for p in 0..self.n * self.n {
            let t = st.cell[p];
            if t != u16::MAX && !self.is_clue[p] {
                s += self.good_z[t as usize];
                m += 1;
            }
        }
        if m == 0 {
            0.0
        } else {
            s / m as f64
        }
    }

    /// S9: Shannon entropy (bits) of the non-grey side colours of unused tiles.
    fn entropy(&self, st: &State) -> f64 {
        let mut h = vec![0f64; self.ncol];
        let mut tot = 0f64;
        for t in 0..self.pieces.len() {
            if !Self::used(st, t) {
                for &c in &self.pieces[t] {
                    if c != 0 {
                        h[c as usize] += 1.0;
                        tot += 1.0;
                    }
                }
            }
        }
        if tot == 0.0 {
            return 0.0;
        }
        -h.iter()
            .filter(|&&x| x > 0.0)
            .map(|&x| {
                let p = x / tot;
                p * p.log2()
            })
            .sum::<f64>()
    }

    /// Negative control: a fixed pseudo-random hash of the placed (pos, tile)
    /// set, uniform in [0, 1).
    fn hash(&self, st: &State) -> f64 {
        let mut h: u64 = 0;
        for p in 0..self.n * self.n {
            let t = st.cell[p];
            if t != u16::MAX && !self.is_clue[p] {
                h ^= mix((p as u64) << 16 | t as u64);
            }
        }
        (mix(h ^ 0xD4D4) >> 11) as f64 / (1u64 << 53) as f64
    }

    /// S7: arc consistency over all empty cells (values = unused tile
    /// orientations meeting the unary requirement; support = a value of the
    /// empty neighbour with the matching colour on the facing side), to a
    /// fixed point. Returns sum over empty cells of log2(1 + domain size).
    fn ac(&self, st: &State) -> f64 {
        self.ac_domains(st).iter().map(|&d| (1.0 + d as f64).log2()).sum()
    }

    /// Follow-up (unplanned): arc consistency refutes the prefix when some
    /// empty cell's domain is wiped out at the fixed point.
    pub fn ac_refuted(&self, st: &State) -> bool {
        self.ac_domains(st).iter().any(|&d| d == 0)
    }

    /// Domain sizes of the empty cells at the arc-consistency fixed point.
    fn ac_domains(&self, st: &State) -> Vec<usize> {
        let n = self.n;
        let nc = self.ncol;
        let cells: Vec<usize> = (0..n * n).filter(|&p| Self::empty(st, p)).collect();
        let mut idx = vec![usize::MAX; n * n];
        for (i, &p) in cells.iter().enumerate() {
            idx[p] = i;
        }
        let mut dom: Vec<Vec<Edges>> = Vec::with_capacity(cells.len());
        let mut cnt: Vec<Vec<u32>> = Vec::with_capacity(cells.len()); // [side*nc + colour]
        for &p in &cells {
            let w = self.want(st, p);
            let mut d = Vec::new();
            let mut c = vec![0u32; 4 * nc];
            for t in 0..self.pieces.len() {
                if Self::used(st, t) {
                    continue;
                }
                for e in &self.orients[t] {
                    if Self::fits(e, &w) {
                        d.push(*e);
                        for s in 0..4 {
                            c[s * nc + e[s] as usize] += 1;
                        }
                    }
                }
            }
            dom.push(d);
            cnt.push(c);
        }
        let mut queue: Vec<usize> = (0..cells.len()).collect();
        let mut inq = vec![true; cells.len()];
        while let Some(i) = queue.pop() {
            inq[i] = false;
            let p = cells[i];
            let nbi: Vec<(usize, usize)> = (0..4)
                .filter_map(|s| nb(n, p, s).and_then(|q| (idx[q] != usize::MAX).then(|| (s, idx[q]))))
                .collect();
            let mut removed = false;
            let mut d = std::mem::take(&mut dom[i]);
            d.retain(|e| {
                let ok = nbi
                    .iter()
                    .all(|&(s, j)| cnt[j][((s + 2) % 4) * nc + e[s] as usize] > 0);
                if !ok {
                    for s in 0..4 {
                        cnt[i][s * nc + e[s] as usize] -= 1;
                    }
                    removed = true;
                }
                ok
            });
            dom[i] = d;
            if removed {
                for &(_, j) in &nbi {
                    if !inq[j] {
                        inq[j] = true;
                        queue.push(j);
                    }
                }
            }
        }
        dom.iter().map(|d| d.len()).collect()
    }
}

fn is_frame_joint(n: usize, p: usize, q: usize) -> bool {
    let rim = |x: usize| {
        let (r, c) = (x / n, x % n);
        (r == 0, r == n - 1, c == 0, c == n - 1)
    };
    let (a, b) = (rim(p), rim(q));
    (a.0 && b.0) || (a.1 && b.1) || (a.2 && b.2) || (a.3 && b.3)
}

fn mix(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^ (x >> 31)
}

/// S3: min over nonempty S of frontier cells of |N(S)| - |S|, N(S) the
/// unused tiles fitting some cell of S (tile level), truncated at `cap`.
/// Computed per cell v as (max flow with v uncapacitated) - |F|.
pub fn hall_surplus(adj: &[Vec<u16>], ntiles: usize, cap: i64) -> i64 {
    let f = adj.len();
    if f == 0 {
        return cap;
    }
    // base maximum matching
    let mut owner: Vec<i32> = vec![-1; ntiles]; // tile -> cell
    let mut base = 0i64;
    for v in 0..f {
        let mut seen = vec![false; ntiles];
        if augment(v, adj, &mut owner, &mut seen) {
            base += 1;
        }
    }
    let mut best = cap;
    for v in 0..f {
        let mut own = owner.clone();
        let mut extra = 0i64;
        // v may take further tiles: each success is one more unit of flow
        while extra < cap + f as i64 {
            let mut seen = vec![false; ntiles];
            if augment(v, adj, &mut own, &mut seen) {
                extra += 1;
            } else {
                break;
            }
            if base + extra - f as i64 >= best {
                break;
            }
        }
        best = best.min(base + extra - f as i64);
    }
    best
}

fn augment(v: usize, adj: &[Vec<u16>], owner: &mut [i32], seen: &mut [bool]) -> bool {
    for &t in &adj[v] {
        let t = t as usize;
        if seen[t] {
            continue;
        }
        seen[t] = true;
        if owner[t] < 0 || augment(owner[t] as usize, adj, owner, seen) {
            owner[t] = v as i32;
            return true;
        }
    }
    false
}
