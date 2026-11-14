//! frame-cores engine: puzzle files, valid-frame sampler and counter (SMC over
//! the border ring), and the plain-search refuters R0 (depth-first interior
//! fill, rows or inward spiral) and R1 (R0 plus a matching check on the
//! constrained cells).
//!
//! Subcommands (all write JSON lines or plain text, see README):
//!   real   --csv FILE --out PUZ
//!   gen    --n N --colors C [--bc B] --seed S [--clues none|five] --out PUZ
//!   sample --puz PUZ --stratum U|H --count K --np N --seed S --out FRAMES
//!   count  --puz PUZ --stratum U|H --np N --reps R --seed S [--subsets FILE]
//!   dfs    --puz PUZ --frames FRAMES --order rows|spiral [--hall] --cap NODES --tcap SECS
//!
//! Puzzle file: `n N`, then one `p t r b l` line per piece (id = line order,
//! edges URDL, 0 = grey), `f pos piece rot` per clue, `s pos piece rot` per
//! planted cell (verification only, never read by a refuter).
//! Frames file: one line per frame or subset: `id pos piece rot pos piece rot ...`.

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;
use std::time::Instant;

type Edges = [u8; 4];

fn rotate(e: Edges, r: u8) -> Edges {
    let [t, ri, b, l] = e;
    match r & 3 {
        0 => [t, ri, b, l],
        1 => [l, t, ri, b],
        2 => [b, l, t, ri],
        _ => [ri, b, l, t],
    }
}

// splitmix-seeded xoshiro256** (same generator as the Track 10 engine).
struct Rng {
    s: [u64; 4],
}
impl Rng {
    fn new(seed: u64) -> Self {
        let mut z = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut next = || {
            z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut x = z;
            x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            x ^ (x >> 31)
        };
        Self {
            s: [next(), next(), next(), next()],
        }
    }
    fn next_u64(&mut self) -> u64 {
        let result = self.s[1].wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        let t = self.s[1] << 17;
        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];
        self.s[2] ^= t;
        self.s[3] = self.s[3].rotate_left(45);
        result
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next_u64() % (n as u64)) as usize
    }
    fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
}

struct Puz {
    n: usize,
    pieces: Vec<Edges>,
    fixed: Vec<(usize, u16, u8)>,
    planted: Vec<(usize, u16, u8)>,
}

fn load_puz(path: &str) -> Puz {
    let raw = fs::read_to_string(path).expect("read puzzle");
    let mut n = 0;
    let mut pieces = Vec::new();
    let mut fixed = Vec::new();
    let mut planted = Vec::new();
    for l in raw.lines() {
        let t: Vec<&str> = l.split_whitespace().collect();
        if t.is_empty() {
            continue;
        }
        let v: Vec<usize> = t[1..].iter().map(|x| x.parse().unwrap()).collect();
        match t[0] {
            "n" => n = v[0],
            "p" => pieces.push([v[0] as u8, v[1] as u8, v[2] as u8, v[3] as u8]),
            "f" => fixed.push((v[0], v[1] as u16, v[2] as u8)),
            "s" => planted.push((v[0], v[1] as u16, v[2] as u8)),
            _ => panic!("bad puzzle line {l}"),
        }
    }
    assert_eq!(pieces.len(), n * n);
    Puz {
        n,
        pieces,
        fixed,
        planted,
    }
}

fn save_puz(p: &Puz, path: &str) {
    let mut s = format!("n {}\n", p.n);
    for e in &p.pieces {
        s += &format!("p {} {} {} {}\n", e[0], e[1], e[2], e[3]);
    }
    for &(a, b, c) in &p.fixed {
        s += &format!("f {a} {b} {c}\n");
    }
    for &(a, b, c) in &p.planted {
        s += &format!("s {a} {b} {c}\n");
    }
    fs::write(path, s).expect("write puzzle");
}

type Placement = (usize, u16, u8);

fn load_frames(path: &str) -> Vec<(String, Vec<Placement>)> {
    let raw = fs::read_to_string(path).expect("read frames");
    raw.lines()
        .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
        .map(|l| {
            let t: Vec<&str> = l.split_whitespace().collect();
            let v: Vec<usize> = t[1..].iter().map(|x| x.parse().unwrap()).collect();
            let pl = v
                .chunks(3)
                .map(|c| (c[0], c[1] as u16, c[2] as u8))
                .collect();
            (t[0].to_string(), pl)
        })
        .collect()
}

// ---------- geometry ----------

struct Geo {
    n: usize,
}
impl Geo {
    fn nbr(&self, pos: usize, side: usize) -> Option<usize> {
        let (r, c) = (pos / self.n, pos % self.n);
        match side {
            0 if r > 0 => Some(pos - self.n),
            1 if c + 1 < self.n => Some(pos + 1),
            2 if r + 1 < self.n => Some(pos + self.n),
            3 if c > 0 => Some(pos - 1),
            _ => None,
        }
    }
    fn is_border(&self, pos: usize) -> bool {
        let (r, c) = (pos / self.n, pos % self.n);
        r == 0 || c == 0 || r == self.n - 1 || c == self.n - 1
    }
    fn side_to(&self, a: usize, b: usize) -> usize {
        (0..4).find(|&s| self.nbr(a, s) == Some(b)).expect("adjacent")
    }
    /// Clockwise ring from (0,0).
    fn ring(&self) -> Vec<usize> {
        let n = self.n;
        let mut v = Vec::new();
        for c in 0..n {
            v.push(c);
        }
        for r in 1..n {
            v.push(r * n + n - 1);
        }
        for c in (0..n - 1).rev() {
            v.push((n - 1) * n + c);
        }
        for r in (1..n - 1).rev() {
            v.push(r * n);
        }
        v
    }
    /// Rotation that puts the grey sides of piece e outward at border cell pos.
    fn orient(&self, pos: usize, e: Edges) -> Option<u8> {
        (0..4u8).find(|&r| {
            let f = rotate(e, r);
            (0..4).all(|s| (f[s] == 0) == self.nbr(pos, s).is_none())
        })
    }
    fn interior_order(&self, kind: &str) -> Vec<usize> {
        let n = self.n;
        match kind {
            "rows" => (1..n - 1)
                .flat_map(|r| (1..n - 1).map(move |c| r * n + c))
                .collect(),
            "spiral" => {
                let mut v = Vec::new();
                let (mut lo, mut hi) = (1usize, n - 2);
                while lo <= hi {
                    if lo == hi {
                        v.push(lo * n + lo);
                        break;
                    }
                    for c in lo..hi {
                        v.push(lo * n + c);
                    }
                    for r in lo..hi {
                        v.push(r * n + hi);
                    }
                    for c in (lo + 1..=hi).rev() {
                        v.push(hi * n + c);
                    }
                    for r in (lo + 1..=hi).rev() {
                        v.push(r * n + lo);
                    }
                    lo += 1;
                    hi -= 1;
                }
                v
            }
            _ => panic!("order rows|spiral"),
        }
    }
}

fn zeros(e: &Edges) -> usize {
    e.iter().filter(|&&x| x == 0).count()
}

/// Index of free interior (piece, rot) by the colour on one side.
fn side_index(p: &Puz) -> Vec<Vec<Vec<(u16, u8)>>> {
    let clue: Vec<u16> = p.fixed.iter().map(|f| f.1).collect();
    let maxc = p.pieces.iter().flatten().copied().max().unwrap() as usize + 1;
    let mut idx = vec![vec![Vec::new(); maxc]; 4];
    for (id, e) in p.pieces.iter().enumerate() {
        if zeros(e) > 0 || clue.contains(&(id as u16)) {
            continue;
        }
        for r in 0..4u8 {
            let f = rotate(*e, r);
            if r > 0 && (0..r).any(|q| rotate(*e, q) == f) {
                continue; // symmetric piece: same placement twice
            }
            for s in 0..4 {
                idx[s][f[s] as usize].push((id as u16, r));
            }
        }
    }
    idx
}

// ---------- clue-block test ----------

struct Blocks {
    /// per corner: corner pos, three free cells, chain of five border cells
    list: Vec<(usize, [usize; 3], [usize; 5])>,
}

fn blocks(p: &Puz, g: &Geo) -> Blocks {
    let n = p.n;
    let mut list = Vec::new();
    if n < 6 {
        return Blocks { list };
    }
    for &(cr, cc) in &[(0usize, 0usize), (0, n - 1), (n - 1, n - 1), (n - 1, 0)] {
        let dr: isize = if cr == 0 { 1 } else { -1 };
        let dc: isize = if cc == 0 { 1 } else { -1 };
        let at = |a: isize, b: isize| ((cr as isize + a * dr) as usize) * n + (cc as isize + b * dc) as usize;
        let clue = at(2, 2);
        if !p.fixed.iter().any(|f| f.0 == clue) {
            continue;
        }
        let cells = [at(1, 1), at(1, 2), at(2, 1)];
        let chain = [at(0, 0), at(0, 1), at(0, 2), at(1, 0), at(2, 0)];
        for &c in &chain {
            assert!(g.is_border(c));
        }
        list.push((at(0, 0), cells, chain));
    }
    Blocks { list }
}

/// Does a fill of `cells` with distinct free interior pieces exist, matching
/// every neighbour already coloured on `board`?
fn exists_fill(
    g: &Geo,
    idx: &[Vec<Vec<(u16, u8)>>],
    pieces: &[Edges],
    board: &mut Vec<Option<Edges>>,
    cells: &[usize],
    used: &mut Vec<u16>,
) -> bool {
    let Some((&cell, rest)) = cells.split_first() else {
        return true;
    };
    let mut known = Vec::new();
    for s in 0..4 {
        if let Some(nb) = g.nbr(cell, s) {
            if let Some(e) = board[nb] {
                known.push((s, e[(s + 2) % 4]));
            }
        }
    }
    let (s0, c0) = known[0];
    for &(pid, r) in &idx[s0][c0 as usize] {
        if used.contains(&pid) {
            continue;
        }
        let f = rotate(pieces[pid as usize], r);
        if known.iter().all(|&(s, c)| f[s] == c) {
            used.push(pid);
            board[cell] = Some(f);
            let ok = exists_fill(g, idx, pieces, board, rest, used);
            board[cell] = None;
            used.pop();
            if ok {
                return true;
            }
        }
    }
    false
}

// ---------- SMC over the ring ----------

struct Smc<'a> {
    p: &'a Puz,
    g: Geo,
    order: Vec<usize>,
    step_of: HashMap<usize, usize>,
    fixed_at: Vec<Option<(u16, u8)>>,
    used0: u64,
    bit: Vec<u32>,
    cands: Vec<Vec<(u16, u8)>>,
    tests_at: Vec<Vec<usize>>,
    blocks: Blocks,
    idx: Vec<Vec<Vec<(u16, u8)>>>,
    board0: Vec<Option<Edges>>,
    stratum_h: bool,
}

impl<'a> Smc<'a> {
    fn new(p: &'a Puz, stratum_h: bool, subset: &[Placement]) -> Self {
        let g = Geo { n: p.n };
        let ring = g.ring();
        let l = ring.len();
        let start = if p.n >= 6 { 3 } else { 0 };
        let order: Vec<usize> = (0..l).map(|t| ring[(start + t) % l]).collect();
        let step_of: HashMap<usize, usize> = order.iter().enumerate().map(|(t, &c)| (c, t)).collect();
        let mut bit = vec![u32::MAX; p.pieces.len()];
        let mut k = 0;
        for (id, e) in p.pieces.iter().enumerate() {
            if zeros(e) > 0 {
                bit[id] = k;
                k += 1;
            }
        }
        assert!(k <= 64);
        let mut fixed_at = vec![None; p.n * p.n];
        let mut used0 = 0u64;
        for &(pos, pid, r) in subset {
            fixed_at[pos] = Some((pid, r));
            used0 |= 1 << bit[pid as usize];
        }
        let cands: Vec<Vec<(u16, u8)>> = order
            .iter()
            .map(|&pos| {
                p.pieces
                    .iter()
                    .enumerate()
                    .filter(|(_, e)| zeros(e) > 0)
                    .filter_map(|(id, e)| g.orient(pos, *e).map(|r| (id as u16, r)))
                    .collect()
            })
            .collect();
        let blocks = blocks(p, &g);
        let mut tests_at = vec![Vec::new(); l];
        if stratum_h {
            for (k, b) in blocks.list.iter().enumerate() {
                let last = b.2.iter().map(|c| step_of[c]).max().unwrap();
                tests_at[last].push(k);
            }
        }
        let idx = side_index(p);
        let mut board0 = vec![None; p.n * p.n];
        for &(pos, pid, r) in &p.fixed {
            board0[pos] = Some(rotate(p.pieces[pid as usize], r));
        }
        Smc {
            p,
            g,
            order,
            step_of,
            fixed_at,
            used0,
            bit,
            cands,
            tests_at,
            blocks,
            idx,
            board0,
            stratum_h,
        }
    }

    fn edges_of(&self, pl: u32) -> Edges {
        rotate(self.p.pieces[(pl >> 2) as usize], (pl & 3) as u8)
    }

    /// Run once; returns (log Z estimate, final particles).
    fn run(&self, np: usize, rng: &mut Rng) -> (f64, Vec<Vec<u32>>) {
        let l = self.order.len();
        let mut parts: Vec<(Vec<u32>, u64)> = vec![(Vec::with_capacity(l), self.used0); np];
        let mut logz = 0.0;
        let mut board = self.board0.clone();
        let mut buf: Vec<(u16, u8)> = Vec::new();
        for t in 0..l {
            let pos = self.order[t];
            let prev = if t > 0 { Some(self.order[t - 1]) } else { None };
            let next = self.order[(t + 1) % l];
            let mut w = vec![0.0f64; np];
            for (pi, (pl, used)) in parts.iter_mut().enumerate() {
                buf.clear();
                let fits = |e: Edges, pl: &Vec<u32>| -> bool {
                    if let Some(pv) = prev {
                        let pe = self.edges_of(pl[t - 1]);
                        if e[self.g.side_to(pos, pv)] != pe[self.g.side_to(pv, pos)] {
                            return false;
                        }
                    }
                    if t + 1 == l {
                        let fe = self.edges_of(pl[0]);
                        if e[self.g.side_to(pos, next)] != fe[self.g.side_to(next, pos)] {
                            return false;
                        }
                    } else if let Some((fp, fr)) = self.fixed_at[next] {
                        let fe = rotate(self.p.pieces[fp as usize], fr);
                        if e[self.g.side_to(pos, next)] != fe[self.g.side_to(next, pos)] {
                            return false;
                        }
                    }
                    true
                };
                if let Some((fp, fr)) = self.fixed_at[pos] {
                    if fits(rotate(self.p.pieces[fp as usize], fr), pl) {
                        buf.push((fp, fr));
                    }
                } else {
                    for &(pid, r) in &self.cands[t] {
                        if *used >> self.bit[pid as usize] & 1 == 1 {
                            continue;
                        }
                        if fits(rotate(self.p.pieces[pid as usize], r), pl) {
                            buf.push((pid, r));
                        }
                    }
                }
                if buf.is_empty() {
                    continue;
                }
                let (pid, r) = buf[rng.below(buf.len())];
                pl.push(((pid as u32) << 2) | r as u32);
                *used |= 1 << self.bit[pid as usize];
                let mut alive = true;
                for &k in &self.tests_at[t] {
                    let (_, cells, chain) = &self.blocks.list[k];
                    for &c in chain {
                        board[c] = Some(self.edges_of(pl[self.step_of[&c]]));
                    }
                    let mut u = Vec::new();
                    let ok = exists_fill(&self.g, &self.idx, &self.p.pieces, &mut board, cells, &mut u);
                    for &c in chain {
                        board[c] = None;
                    }
                    if !ok {
                        alive = false;
                        break;
                    }
                }
                if alive {
                    w[pi] = buf.len() as f64;
                }
            }
            let total: f64 = w.iter().sum();
            if total == 0.0 {
                return (f64::NEG_INFINITY, Vec::new());
            }
            logz += (total / np as f64).ln();
            // systematic resampling
            // pick the smallest j whose cumulative weight reaches the target
            let u0 = rng.unit() / np as f64;
            let mut newp = Vec::with_capacity(np);
            let mut j = 0;
            let mut cum = w[0] / total;
            for i in 0..np {
                let target = u0 + i as f64 / np as f64;
                while cum < target && j + 1 < np {
                    j += 1;
                    cum += w[j] / total;
                }
                let mut k = j;
                while w[k] == 0.0 {
                    k -= 1; // only reachable through rounding at the tail
                }
                newp.push(parts[k].clone());
            }
            parts = newp;
        }
        let _ = self.stratum_h;
        (logz, parts.into_iter().map(|(pl, _)| pl).collect())
    }
}

// ---------- DFS refuter ----------

struct Dfs<'a> {
    p: &'a Puz,
    g: Geo,
    order: Vec<usize>,
    known: Vec<Vec<(usize, usize)>>,
    bcand: Vec<Vec<(u16, u8)>>,
    idx: Vec<Vec<Vec<(u16, u8)>>>,
    rot: Vec<[Edges; 4]>,
    edges: Vec<Edges>,
    filled: Vec<bool>,
    used: Vec<bool>,
    place: Vec<(u16, u8)>,
    hall: bool,
    nodes: u64,
    cap: u64,
    tcap: f64,
    start: Instant,
    stop: bool,
    maxdepth: usize,
    hall_calls: u64,
    hall_cuts: u64,
    // matching scratch
    mt: Vec<i32>,
    seen: Vec<u32>,
    stamp: u32,
}

impl<'a> Dfs<'a> {
    /// `partial`: None = full frame given (interior cells only are searched);
    /// Some(cells) = only these border cells are given, every other cell
    /// (border included) is searched in bottom-up row order, left to right
    /// (Track 10's row scan).
    fn new(p: &'a Puz, order_kind: &str, hall: bool, cap: u64, tcap: f64, partial: Option<&[usize]>) -> Self {
        let g = Geo { n: p.n };
        let clue_pos: Vec<usize> = p.fixed.iter().map(|f| f.0).collect();
        let nn = p.n * p.n;
        let given = |c: usize| -> bool {
            clue_pos.contains(&c)
                || match partial {
                    None => g.is_border(c),
                    Some(cells) => cells.contains(&c),
                }
        };
        let order: Vec<usize> = match partial {
            None => g.interior_order(order_kind).into_iter().filter(|&c| !given(c)).collect(),
            Some(_) => (0..p.n)
                .rev()
                .flat_map(|r| (0..p.n).map(move |c| r * p.n + c))
                .filter(|&c| !given(c))
                .collect(),
        };
        let mut seen_before = vec![false; nn];
        for (pos, sb) in seen_before.iter_mut().enumerate() {
            *sb = given(pos);
        }
        let mut known = Vec::new();
        for &c in &order {
            let k: Vec<(usize, usize)> = (0..4)
                .filter_map(|s| g.nbr(c, s).filter(|&nb| seen_before[nb]).map(|nb| (s, nb)))
                .collect();
            assert!(!k.is_empty());
            known.push(k);
            seen_before[c] = true;
        }
        let rot = p
            .pieces
            .iter()
            .map(|&e| [rotate(e, 0), rotate(e, 1), rotate(e, 2), rotate(e, 3)])
            .collect();
        let np = p.pieces.len();
        let bcand: Vec<Vec<(u16, u8)>> = (0..nn)
            .map(|pos| {
                if !g.is_border(pos) {
                    return Vec::new();
                }
                p.pieces
                    .iter()
                    .enumerate()
                    .filter(|(_, e)| zeros(e) > 0)
                    .filter_map(|(id, e)| g.orient(pos, *e).map(|r| (id as u16, r)))
                    .collect()
            })
            .collect();
        Dfs {
            p,
            g,
            order,
            known,
            bcand,
            idx: side_index(p),
            rot,
            edges: vec![[0; 4]; p.n * p.n],
            filled: vec![false; p.n * p.n],
            used: vec![false; np],
            place: vec![(0, 0); p.n * p.n],
            hall,
            nodes: 0,
            cap,
            tcap,
            start: Instant::now(),
            stop: false,
            maxdepth: 0,
            hall_calls: 0,
            hall_cuts: 0,
            mt: vec![-1; np],
            seen: vec![0; np],
            stamp: 0,
        }
    }

    fn reset(&mut self, frame: &[Placement]) {
        let nn = self.p.n * self.p.n;
        self.filled = vec![false; nn];
        self.used = vec![false; self.p.pieces.len()];
        for &(pos, pid, r) in frame.iter().chain(self.p.fixed.iter()) {
            self.edges[pos] = self.rot[pid as usize][r as usize];
            self.filled[pos] = true;
            self.used[pid as usize] = true;
            self.place[pos] = (pid, r);
        }
        self.nodes = 0;
        self.stop = false;
        self.maxdepth = 0;
        self.hall_calls = 0;
        self.hall_cuts = 0;
        self.start = Instant::now();
    }

    fn cands_of(&self, cell: usize, out: &mut Vec<u16>) {
        out.clear();
        let mut known = [(0usize, 0u8); 4];
        let mut k = 0;
        for s in 0..4 {
            if let Some(nb) = self.g.nbr(cell, s) {
                if self.filled[nb] {
                    known[k] = (s, self.edges[nb][(s + 2) % 4]);
                    k += 1;
                }
            }
        }
        let (s0, c0) = known[0];
        for &(pid, r) in &self.idx[s0][c0 as usize] {
            if self.used[pid as usize] {
                continue;
            }
            let f = self.rot[pid as usize][r as usize];
            if known[..k].iter().all(|&(s, c)| f[s] == c) && !out.contains(&pid) {
                out.push(pid);
            }
        }
    }

    /// Matching (Hall) check: the unfilled cells that touch a filled cell
    /// must have a system of distinct representatives among unused pieces.
    /// Cells touching nothing filled accept every piece, so this is exactly
    /// the perfect-matching condition on remaining cells and pieces.
    fn hall_ok(&mut self, from: usize) -> bool {
        self.hall_calls += 1;
        let mut lists: Vec<Vec<u16>> = Vec::new();
        let mut buf = Vec::new();
        for i in from..self.order.len() {
            let c = self.order[i];
            let touches = (0..4).any(|s| self.g.nbr(c, s).is_some_and(|nb| self.filled[nb]));
            if !touches {
                continue;
            }
            self.cands_of(c, &mut buf);
            if buf.is_empty() {
                return false;
            }
            lists.push(buf.clone());
        }
        lists.sort_by_key(|l| l.len());
        for l in &lists {
            for &pid in l {
                self.mt[pid as usize] = -1;
            }
        }
        for ci in 0..lists.len() {
            self.stamp += 1;
            if !self.augment(ci, &lists) {
                return false;
            }
        }
        true
    }

    fn augment(&mut self, ci: usize, lists: &[Vec<u16>]) -> bool {
        for &pid in &lists[ci] {
            let p = pid as usize;
            if self.seen[p] == self.stamp {
                continue;
            }
            self.seen[p] = self.stamp;
            if self.mt[p] < 0 || self.augment(self.mt[p] as usize, lists) {
                self.mt[p] = ci as i32;
                return true;
            }
        }
        false
    }

    fn rec(&mut self, i: usize) -> bool {
        if i > self.maxdepth {
            self.maxdepth = i;
        }
        if i == self.order.len() {
            return true;
        }
        let cell = self.order[i];
        let (s0, nb0) = self.known[i][0];
        let c0 = self.edges[nb0][(s0 + 2) % 4] as usize;
        let border = !self.bcand[cell].is_empty();
        let len = if border { self.bcand[cell].len() } else { self.idx[s0][c0].len() };
        for k in 0..len {
            let (pid, r) = if border { self.bcand[cell][k] } else { self.idx[s0][c0][k] };
            if self.used[pid as usize] {
                continue;
            }
            let f = self.rot[pid as usize][r as usize];
            if !self.known[i].iter().all(|&(s, nb)| f[s] == self.edges[nb][(s + 2) % 4]) {
                continue;
            }
            self.nodes += 1;
            if self.nodes > self.cap || (self.nodes & 0xFFF == 0 && self.start.elapsed().as_secs_f64() > self.tcap) {
                self.stop = true;
                return false;
            }
            self.used[pid as usize] = true;
            self.filled[cell] = true;
            self.edges[cell] = f;
            self.place[cell] = (pid, r);
            let ok = !self.hall || {
                let h = self.hall_ok(i + 1);
                if !h {
                    self.hall_cuts += 1;
                }
                h
            };
            if ok && self.rec(i + 1) {
                return true;
            }
            self.used[pid as usize] = false;
            self.filled[cell] = false;
            if self.stop {
                return false;
            }
        }
        false
    }
}

// ---------- commands ----------

fn args() -> HashMap<String, String> {
    let a: Vec<String> = env::args().skip(2).collect();
    let mut m = HashMap::new();
    let mut i = 0;
    while i < a.len() {
        let k = a[i].trim_start_matches("--").to_string();
        if i + 1 < a.len() && !a[i + 1].starts_with("--") {
            m.insert(k, a[i + 1].clone());
            i += 2;
        } else {
            m.insert(k, "1".to_string());
            i += 1;
        }
    }
    m
}

fn num(m: &HashMap<String, String>, k: &str) -> f64 {
    m.get(k).unwrap_or_else(|| panic!("--{k} required")).parse::<f64>().unwrap()
}

/// Official clue cells as (row, col).
const CLUES16: [(usize, usize); 5] = [(8, 7), (2, 2), (2, 13), (13, 2), (13, 13)];

fn cmd_real(m: &HashMap<String, String>) {
    let raw = fs::read_to_string(&m["csv"]).expect("csv");
    let mut lines = raw.lines().filter(|l| !l.trim().is_empty());
    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut p = Puz {
        n,
        pieces: Vec::new(),
        fixed: Vec::new(),
        planted: Vec::new(),
    };
    for line in lines {
        let c: Vec<&str> = line.split(',').collect();
        let col = |s: &str| {
            let v = u32::from_str_radix(s.trim(), 2).unwrap();
            if v == 65535 {
                0u8
            } else {
                v as u8
            }
        };
        let id = p.pieces.len();
        p.pieces.push([col(c[0]), col(c[1]), col(c[2]), col(c[3])]);
        let x: usize = c[4].trim().parse().unwrap();
        let y: usize = c[5].trim().parse().unwrap();
        let r: u8 = c[6].trim().parse().unwrap();
        if x != 0 || y != 0 || r != 0 {
            p.fixed.push((y * n + x, id as u16, r));
        }
    }
    save_puz(&p, &m["out"]);
}

fn cmd_gen(m: &HashMap<String, String>) {
    let n = num(m, "n") as u8;
    let colors = num(m, "colors") as u8;
    let seed = num(m, "seed") as u32;
    let bc = m
        .get("bc")
        .map(|v| v.parse::<u8>().unwrap())
        .unwrap_or_else(|| e2_core::generator::frame_color_count(colors));
    let gp = e2_core::generator::generate_solved_framed_bc(n, colors, seed, true, bc);
    let n = n as usize;
    // pieces are in solution order; scramble ids and rotations (seeded) so
    // that no refuter can read the solution from the piece order
    let mut rng = Rng::new(u64::from(seed) ^ 0xD1F0_0000);
    let mut perm: Vec<usize> = (0..n * n).collect();
    for i in (1..perm.len()).rev() {
        let j = rng.below(i + 1);
        perm.swap(i, j);
    }
    let mut pieces = vec![[0u8; 4]; n * n];
    let mut planted = Vec::new();
    for pos in 0..n * n {
        let e = gp.pieces[pos].map(|c| if c == e2_core::BORDER { 0 } else { c });
        let k = rng.below(4) as u8;
        pieces[perm[pos]] = rotate(e, k);
        planted.push((pos, perm[pos] as u16, (4 - k) % 4));
    }
    for &(pos, pid, r) in &planted {
        assert_eq!(rotate(pieces[pid as usize], r), gp.pieces[pos].map(|c| if c == e2_core::BORDER { 0 } else { c }));
    }
    let fixed = if m.get("clues").map(String::as_str) == Some("five") {
        assert_eq!(n, 16);
        CLUES16.iter().map(|&(r, c)| planted[r * n + c]).collect()
    } else {
        Vec::new()
    };
    save_puz(
        &Puz {
            n,
            pieces,
            fixed,
            planted,
        },
        &m["out"],
    );
}

fn cmd_sample(m: &HashMap<String, String>) {
    let p = load_puz(&m["puz"]);
    let h = m["stratum"] == "H";
    let count = num(m, "count") as usize;
    let np = num(m, "np") as usize;
    let mut rng = Rng::new(num(m, "seed") as u64);
    let smc = Smc::new(&p, h, &[]);
    let prefix = m.get("prefix").cloned().unwrap_or_else(|| "f".into());
    let mut out = fs::File::create(&m["out"]).unwrap();
    let mut got = 0;
    let mut runs = 0;
    while got < count {
        runs += 1;
        let (lz, parts) = smc.run(np, &mut rng);
        if parts.is_empty() {
            continue;
        }
        let pl = &parts[rng.below(parts.len())];
        let mut line = format!("{prefix}{got}");
        for (t, &x) in pl.iter().enumerate() {
            line += &format!(" {} {} {}", smc.order[t], x >> 2, x & 3);
        }
        writeln!(out, "{line}").unwrap();
        eprintln!("frame {got} run {runs} log10Z {:.3}", lz / std::f64::consts::LN_10);
        got += 1;
    }
}

fn cmd_count(m: &HashMap<String, String>) {
    let p = load_puz(&m["puz"]);
    let h = m["stratum"] == "H";
    let np = num(m, "np") as usize;
    let reps = num(m, "reps") as usize;
    let mut rng = Rng::new(num(m, "seed") as u64);
    let subsets = match m.get("subsets") {
        Some(f) => load_frames(f),
        None => vec![("empty".to_string(), Vec::new())],
    };
    for (id, s) in subsets {
        let t0 = Instant::now();
        let smc = Smc::new(&p, h, &s);
        let mut l10 = Vec::new();
        for _ in 0..reps {
            let (lz, _) = smc.run(np, &mut rng);
            l10.push(lz / std::f64::consts::LN_10);
        }
        // log10 of the mean of Z over reps (the unbiased estimator)
        let mx = l10.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let mean = if mx.is_finite() {
            mx + (l10.iter().map(|x| 10f64.powf(x - mx)).sum::<f64>() / reps as f64).log10()
        } else {
            f64::NEG_INFINITY
        };
        let reps_s: Vec<String> = l10.iter().map(|x| if x.is_finite() { format!("{x:.4}") } else { "null".into() }).collect();
        println!(
            "{{\"id\":\"{id}\",\"stratum\":\"{}\",\"size\":{},\"np\":{np},\"reps\":{reps},\"log10_mean\":{},\"log10_reps\":[{}],\"secs\":{:.2}}}",
            m["stratum"],
            s.len(),
            if mean.is_finite() { format!("{mean:.4}") } else { "null".into() },
            reps_s.join(","),
            t0.elapsed().as_secs_f64()
        );
    }
}

fn cmd_dfs(m: &HashMap<String, String>) {
    let p = load_puz(&m["puz"]);
    let frames = load_frames(&m["frames"]);
    let hall = m.contains_key("hall");
    let order = m["order"].clone();
    let cap = num(m, "cap") as u64;
    let tcap = num(m, "tcap");
    let partial = m.contains_key("partial");
    assert!(!(partial && hall), "partial frames: R0 only");
    let name = if partial {
        "R0-rowsup-partial".to_string()
    } else {
        format!("{}-{}", if hall { "R1" } else { "R0" }, order)
    };
    let mut d = Dfs::new(&p, &order, hall, cap, tcap, None);
    for (id, fr) in frames {
        if partial {
            let cells: Vec<usize> = fr.iter().map(|f| f.0).collect();
            d = Dfs::new(&p, &order, false, cap, tcap, Some(&cells));
        }
        d.reset(&fr);
        let found = d.rec(0);
        let secs = d.start.elapsed().as_secs_f64();
        let verdict = if found {
            "sat"
        } else if d.stop {
            "capped"
        } else {
            "unsat"
        };
        // border cells adjacent to any interior cell the search reached
        let mut read: Vec<usize> = Vec::new();
        for i in 0..d.maxdepth.min(d.order.len() - 1) + 1 {
            for s in 0..4 {
                if let Some(nb) = d.g.nbr(d.order[i], s) {
                    if d.g.is_border(nb) && !read.contains(&nb) {
                        read.push(nb);
                    }
                }
            }
        }
        read.sort();
        let sol = if found {
            let mut v: Vec<String> = Vec::new();
            for pos in 0..p.n * p.n {
                let (pid, r) = d.place[pos];
                v.push(format!("[{pos},{pid},{r}]"));
            }
            format!("[{}]", v.join(","))
        } else {
            "null".into()
        };
        let read_s: Vec<String> = read.iter().map(|x| x.to_string()).collect();
        println!(
            "{{\"id\":\"{id}\",\"refuter\":\"{name}\",\"verdict\":\"{verdict}\",\"nodes\":{},\"secs\":{secs:.6},\"maxdepth\":{},\"cells\":{},\"hall_calls\":{},\"hall_cuts\":{},\"read_border\":[{}],\"solution\":{sol}}}",
            d.nodes,
            d.maxdepth,
            d.order.len(),
            d.hall_calls,
            d.hall_cuts,
            read_s.join(",")
        );
        std::io::stdout().flush().unwrap();
    }
}

/// Per frame: which clue corners have a dead 3-cell block and which a dead
/// 3x3 block (eight free cells around the clue against the six border cells
/// touching them), with distinct free interior pieces.
fn cmd_flags(m: &HashMap<String, String>) {
    let p = load_puz(&m["puz"]);
    let g = Geo { n: p.n };
    let idx = side_index(&p);
    let bl = blocks(&p, &g);
    let n = p.n as isize;
    for (id, fr) in load_frames(&m["frames"]) {
        let mut board: Vec<Option<Edges>> = vec![None; p.n * p.n];
        for &(pos, pid, r) in fr.iter().chain(p.fixed.iter()) {
            board[pos] = Some(rotate(p.pieces[pid as usize], r));
        }
        let mut dead3 = Vec::new();
        let mut dead9 = Vec::new();
        for (k, (corner, cells, _)) in bl.list.iter().enumerate() {
            let (cr, cc) = ((*corner / p.n) as isize, (*corner % p.n) as isize);
            let dr = if cr == 0 { 1 } else { -1 };
            let dc = if cc == 0 { 1 } else { -1 };
            let at = |a: isize, b: isize| ((cr + a * dr) * n + cc + b * dc) as usize;
            let mut u = Vec::new();
            if !exists_fill(&g, &idx, &p.pieces, &mut board, cells, &mut u) {
                dead3.push(k);
            }
            let nine = [at(1, 1), at(1, 2), at(1, 3), at(2, 1), at(3, 1), at(2, 3), at(3, 2), at(3, 3)];
            let mut u = Vec::new();
            if !exists_fill(&g, &idx, &p.pieces, &mut board, &nine, &mut u) {
                dead9.push(k);
            }
        }
        println!("{{\"id\":\"{id}\",\"dead_block3\":{:?},\"dead_block3x3\":{:?}}}", dead3, dead9);
    }
}

fn main() {
    let cmd = env::args().nth(1).unwrap_or_default();
    let m = args();
    match cmd.as_str() {
        "real" => cmd_real(&m),
        "gen" => cmd_gen(&m),
        "sample" => cmd_sample(&m),
        "count" => cmd_count(&m),
        "dfs" => cmd_dfs(&m),
        "flags" => cmd_flags(&m),
        _ => eprintln!("usage: fc real|gen|sample|count|dfs ..."),
    }
}
