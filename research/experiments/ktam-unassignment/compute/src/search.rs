//! The shared placement machinery (consistent partial board, MRV attachment,
//! random value) and the three unassignment rules: uniform random removal
//! (B3), kTAM-style bond-weighted detachment (V1) and IMS-style eviction of
//! the blocking pieces (V2). See `../PREREG.md` section 5.
//!
//! Events: every placement goes through `NodeCounter::try_assign` (a ladder
//! node); every removal is counted in `Stats::removals`.

use eternity2_engine::{rotated, Color, Puzzle, BORDER};
use hard_ladder::{LadderSolver, NodeCounter, Outcome};

/// Key value for "any non-grey colour" (an interior side with no placed neighbour).
const ANY: u32 = 31;

/// Unassignment rule.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rule {
    /// B3: remove `k` uniformly random placed pieces within distance `r` of the dead cell.
    Random { r: Option<u32>, k: u32 },
    /// V1: remove `k` placed pieces within distance `r`, weight exp(-beta * bonds).
    Ktam { beta: f64, r: Option<u32>, k: u32 },
    /// V2: place a min-cost piece at the dead cell, evicting its blockers; tabu `tabu` placements.
    Ims { tabu: u64, noise: f64 },
}

impl Rule {
    #[must_use]
    pub fn name(&self) -> String {
        let rs = |r: &Option<u32>| r.map_or_else(|| "inf".to_string(), |v| v.to_string());
        match self {
            Self::Random { r, k } => format!("b3-r{}-k{k}", rs(r)),
            Self::Ktam { beta, r, k } => format!("v1-beta{beta}-r{}-k{k}", rs(r)),
            Self::Ims { tabu, noise } => format!("v2-t{tabu}-n{noise}"),
        }
    }

    /// Inverse of [`Rule::name`].
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('-').collect();
        let r = |p: &str| -> Option<Option<u32>> {
            let v = p.strip_prefix('r')?;
            Some(if v == "inf" { None } else { Some(v.parse().ok()?) })
        };
        let k = |p: &str| -> Option<u32> { p.strip_prefix('k')?.parse().ok() };
        match parts.as_slice() {
            ["b3", a, b] => Some(Self::Random { r: r(a)?, k: k(b)? }),
            ["v1", a, b, c] => Some(Self::Ktam { beta: a.strip_prefix("beta")?.parse().ok()?, r: r(b)?, k: k(c)? }),
            ["v2", a, b] => Some(Self::Ims {
                tabu: a.strip_prefix('t')?.parse().ok()?,
                noise: b.strip_prefix('n')?.parse().ok()?,
            }),
            _ => None,
        }
    }
}

/// Preregistered tuning grids (PREREG.md section 5), in grid order.
#[must_use]
pub fn grid(arm: &str) -> Vec<Rule> {
    let rs = [Some(1), Some(2), None];
    let mut v = Vec::new();
    match arm {
        "B3" => {
            for r in rs {
                for k in 1..=3 {
                    v.push(Rule::Random { r, k });
                }
            }
        }
        "V1" => {
            for beta in [0.5, 1.0, 2.0, 4.0] {
                for r in rs {
                    for k in 1..=3 {
                        v.push(Rule::Ktam { beta, r, k });
                    }
                }
            }
        }
        "V2" => {
            for tabu in [0, 16, 128] {
                for noise in [0.0, 0.1, 0.3] {
                    v.push(Rule::Ims { tabu, noise });
                }
            }
        }
        _ => {}
    }
    v
}

/// Per-run counters beyond the ladder's.
#[derive(Debug, Clone, Default)]
pub struct Stats {
    pub removals: u64,
    pub dead_ends: u64,
    pub filled_at_end: u64,
    /// Removal and placement counts per (cell, value), value = piece*4+rot,
    /// index cell * 4 * cells + value. Read by the driver after the run only.
    pub removed_at: Vec<u32>,
    pub placed_at: Vec<u32>,
}

struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        let mut z = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        Self(z ^ (z >> 31) | 1)
    }
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn below(&mut self, n: usize) -> usize {
        ((self.next() >> 11) % n as u64) as usize
    }
    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
}

pub struct Unassign {
    pub rule: Rule,
    pub seed: u64,
    pub stats: Stats,
}

impl Unassign {
    #[must_use]
    pub fn new(rule: Rule, seed: u64) -> Self {
        Self { rule, seed, stats: Stats::default() }
    }
}

struct St<'a> {
    w: usize,
    h: usize,
    cells: usize,
    /// Oriented pieces: value -> edges (None for duplicate orientations).
    oriented: Vec<Option<[Color; 4]>>,
    /// Index: key -> values, flat with offsets.
    offs: Vec<u32>,
    vals: Vec<u32>,
    board: Vec<i32>,
    edges: Vec<[Color; 4]>,
    piece_at: Vec<i32>,
    filled: usize,
    rng: Rng,
    counter: &'a mut NodeCounter,
    stats: &'a mut Stats,
    placements: u64,
    tabu_until: Vec<u64>,
    tabu: u64,
}

fn key4(s: [u32; 4]) -> usize {
    (s[0] | s[1] << 5 | s[2] << 10 | s[3] << 15) as usize
}

impl St<'_> {
    fn nb(&self, cell: usize, side: usize) -> Option<usize> {
        let (x, y) = (cell % self.w, cell / self.w);
        match side {
            0 if y > 0 => Some(cell - self.w),
            1 if x + 1 < self.w => Some(cell + 1),
            2 if y + 1 < self.h => Some(cell + self.w),
            3 if x > 0 => Some(cell - 1),
            _ => None,
        }
    }

    /// Required side colours at `cell`: grey on the rim, the neighbour's colour
    /// if placed, else ANY (non-grey). `use_neighbours = false` keeps only the rim.
    fn key(&self, cell: usize, use_neighbours: bool) -> usize {
        let mut s = [ANY; 4];
        for (side, slot) in s.iter_mut().enumerate() {
            match self.nb(cell, side) {
                None => *slot = u32::from(BORDER),
                Some(n) if use_neighbours && self.board[n] >= 0 => {
                    *slot = u32::from(self.edges[n][(side + 2) % 4]);
                }
                Some(_) => {}
            }
        }
        key4(s)
    }

    fn list(&self, key: usize) -> &[u32] {
        &self.vals[self.offs[key] as usize..self.offs[key + 1] as usize]
    }

    /// Attachment bonds at an empty cell: placed orthogonal neighbours plus rim edges.
    fn attach_bonds(&self, cell: usize) -> u32 {
        (0..4).map(|s| u32::from(self.nb(cell, s).is_none_or(|n| self.board[n] >= 0))).sum()
    }

    /// Bond count of a placed piece: matched seams with placed neighbours plus rim edges.
    fn bonds(&self, cell: usize) -> u32 {
        self.attach_bonds(cell)
    }

    fn place(&mut self, cell: usize, v: u32) -> bool {
        if !self.counter.try_assign() {
            return false;
        }
        let e = self.oriented[v as usize].expect("oriented");
        debug_assert!(self.board[cell] < 0);
        debug_assert!(self.piece_at[(v / 4) as usize] < 0);
        self.board[cell] = v as i32;
        self.edges[cell] = e;
        self.piece_at[(v / 4) as usize] = cell as i32;
        self.filled += 1;
        self.placements += 1;
        self.stats.placed_at[cell * 4 * self.cells + v as usize] += 1;
        true
    }

    fn remove(&mut self, cell: usize) {
        let v = self.board[cell];
        debug_assert!(v >= 0);
        self.board[cell] = -1;
        self.piece_at[(v / 4) as usize] = -1;
        self.filled -= 1;
        self.stats.removals += 1;
        self.counter.backtracks += 1;
        let idx = cell * 4 * self.cells + v as usize;
        self.stats.removed_at[idx] += 1;
        if self.tabu > 0 {
            self.tabu_until[idx] = self.placements + self.tabu;
        }
    }

    fn region(&self, x: usize, r: Option<u32>) -> Vec<usize> {
        let (cx, cy) = ((x % self.w) as i64, (x / self.w) as i64);
        (0..self.cells)
            .filter(|&c| {
                self.board[c] >= 0
                    && r.is_none_or(|r| {
                        let (px, py) = ((c % self.w) as i64, (c / self.w) as i64);
                        ((px - cx).abs() + (py - cy).abs()) as u32 <= r
                    })
            })
            .collect()
    }

    /// Returns false on cap.
    fn dead_end(&mut self, x: usize, rule: Rule) -> bool {
        self.stats.dead_ends += 1;
        match rule {
            Rule::Random { r, k } => {
                let mut reg = self.region(x, r);
                if reg.is_empty() {
                    reg = self.region(x, None);
                }
                for _ in 0..(k as usize).min(reg.len()) {
                    let i = self.rng.below(reg.len());
                    let c = reg.swap_remove(i);
                    self.remove(c);
                }
                true
            }
            Rule::Ktam { beta, r, k } => {
                let mut reg = self.region(x, r);
                if reg.is_empty() {
                    reg = self.region(x, None);
                }
                // Bonds are taken before any removal of this dead end (one detachment step).
                let mut wts: Vec<f64> = reg.iter().map(|&c| (-beta * f64::from(self.bonds(c))).exp()).collect();
                for _ in 0..(k as usize).min(reg.len()) {
                    let tot: f64 = wts.iter().sum();
                    let mut u = self.rng.unit() * tot;
                    let mut i = 0;
                    while i + 1 < wts.len() && u >= wts[i] {
                        u -= wts[i];
                        i += 1;
                    }
                    let c = reg.swap_remove(i);
                    wts.swap_remove(i);
                    self.remove(c);
                }
                true
            }
            Rule::Ims { noise, .. } => {
                let key = self.key(x, false);
                let cand: Vec<u32> = self.list(key).to_vec();
                self.counter.propagation += cand.len() as u64;
                let base = x * 4 * self.cells;
                let cost = |st: &Self, v: u32| -> u32 {
                    let e = st.oriented[v as usize].expect("oriented");
                    let mut c = 0;
                    let mut nbs = [usize::MAX; 4];
                    for (s, slot) in nbs.iter_mut().enumerate() {
                        if let Some(n) = st.nb(x, s) {
                            if st.board[n] >= 0 && st.edges[n][(s + 2) % 4] != e[s] {
                                c += 1;
                                *slot = n;
                            }
                        }
                    }
                    let at = st.piece_at[(v / 4) as usize];
                    if at >= 0 && !nbs.contains(&(at as usize)) {
                        c += 1;
                    }
                    c
                };
                let open: Vec<u32> =
                    cand.iter().copied().filter(|&v| self.tabu_until[base + v as usize] <= self.placements).collect();
                let pool = if open.is_empty() { &cand } else { &open };
                let pick = if noise > 0.0 && self.rng.unit() < noise {
                    pool[self.rng.below(pool.len())]
                } else {
                    let mut best = u32::MAX;
                    let mut chosen = pool[0];
                    let mut ties = 0usize;
                    for &v in pool {
                        let c = cost(self, v);
                        if c < best {
                            best = c;
                            chosen = v;
                            ties = 1;
                        } else if c == best {
                            ties += 1;
                            if self.rng.below(ties) == 0 {
                                chosen = v;
                            }
                        }
                    }
                    chosen
                };
                let e = self.oriented[pick as usize].expect("oriented");
                for (s, &es) in e.iter().enumerate() {
                    if let Some(n) = self.nb(x, s) {
                        if self.board[n] >= 0 && self.edges[n][(s + 2) % 4] != es {
                            self.remove(n);
                        }
                    }
                }
                let at = self.piece_at[(pick / 4) as usize];
                if at >= 0 {
                    self.remove(at as usize);
                }
                self.place(x, pick)
            }
        }
    }
}

/// Whether `e` at `cell` agrees with the rim and every placed neighbour.
fn fits(st: &St, cell: usize, e: [Color; 4]) -> bool {
    (0..4).all(|s| match st.nb(cell, s) {
        None => e[s] == BORDER,
        Some(n) => e[s] != BORDER && (st.board[n] < 0 || st.edges[n][(s + 2) % 4] == e[s]),
    })
}

impl LadderSolver for Unassign {
    fn solve(&mut self, puzzle: &Puzzle, counter: &mut NodeCounter) -> Outcome {
        let (w, h) = (puzzle.width as usize, puzzle.height as usize);
        let cells = w * h;
        let mut oriented = vec![None; 4 * cells];
        for (p, &pe) in puzzle.pieces.iter().enumerate() {
            for r in 0..4u8 {
                let e = rotated(pe, r);
                if (0..r).any(|q| rotated(pe, q) == e) {
                    continue;
                }
                oriented[p * 4 + r as usize] = Some(e);
            }
        }
        // Flat index over every key an oriented piece can satisfy.
        let mut entries: Vec<(usize, u32)> = Vec::new();
        for (v, e) in oriented.iter().enumerate() {
            let Some(e) = e else { continue };
            for mask in 0..16u32 {
                let mut s = [0u32; 4];
                let mut ok = true;
                for side in 0..4 {
                    if mask >> side & 1 == 1 {
                        s[side] = u32::from(e[side]);
                    } else if e[side] == BORDER {
                        ok = false;
                    } else {
                        s[side] = ANY;
                    }
                }
                if ok {
                    entries.push((key4(s), v as u32));
                }
            }
        }
        entries.sort_unstable();
        entries.dedup();
        let mut offs = vec![0u32; (1 << 20) + 1];
        for &(k, _) in &entries {
            offs[k + 1] += 1;
        }
        for i in 0..(1 << 20) {
            offs[i + 1] += offs[i];
        }
        let vals: Vec<u32> = entries.iter().map(|&(_, v)| v).collect();

        self.stats = Stats {
            removed_at: vec![0; cells * 4 * cells],
            placed_at: vec![0; cells * 4 * cells],
            ..Stats::default()
        };
        let rule = self.rule;
        let mut st = St {
            w,
            h,
            cells,
            oriented,
            offs,
            vals,
            board: vec![-1; cells],
            edges: vec![[0; 4]; cells],
            piece_at: vec![-1; cells],
            filled: 0,
            rng: Rng::new(self.seed),
            counter,
            stats: &mut self.stats,
            placements: 0,
            tabu_until: vec![0; cells * 4 * cells],
            tabu: if let Rule::Ims { tabu, .. } = rule { tabu } else { 0 },
        };
        let out = loop {
            if st.filled == cells {
                break Outcome::Solved(st.board.clone());
            }
            // MRV over empty cells; ties by attachment bonds, then uniform.
            let mut best: Option<(usize, u32, usize)> = None; // (count, bonds, cell)
            let mut ties = 0usize;
            for cell in 0..cells {
                if st.board[cell] >= 0 {
                    continue;
                }
                let limit = best.map_or(usize::MAX, |b| b.0);
                let key = st.key(cell, true);
                let mut cnt = 0usize;
                let mut scanned = 0u64;
                for &v in st.list(key) {
                    scanned += 1;
                    if st.piece_at[(v / 4) as usize] < 0 {
                        cnt += 1;
                        if cnt > limit {
                            break;
                        }
                    }
                }
                st.counter.propagation += scanned;
                if cnt > limit {
                    continue;
                }
                let b = st.attach_bonds(cell);
                match best {
                    Some((bc, bb, _)) if cnt == bc && b < bb => {}
                    Some((bc, bb, _)) if cnt == bc && b == bb => {
                        ties += 1;
                        if st.rng.below(ties) == 0 {
                            best = Some((cnt, b, cell));
                        }
                    }
                    _ => {
                        best = Some((cnt, b, cell));
                        ties = 1;
                    }
                }
            }
            let (cnt, _, cell) = best.expect("an empty cell");
            if cnt == 0 {
                if !st.dead_end(cell, rule) {
                    break Outcome::CapReached;
                }
                continue;
            }
            let key = st.key(cell, true);
            let free: Vec<u32> = st.list(key).iter().copied().filter(|&v| st.piece_at[(v / 4) as usize] < 0).collect();
            let v = free[st.rng.below(free.len())];
            debug_assert!(fits(&st, cell, st.oriented[v as usize].expect("oriented")));
            if !st.place(cell, v) {
                break Outcome::CapReached;
            }
        };
        st.stats.filled_at_end = st.filled as u64;
        out
    }
}
