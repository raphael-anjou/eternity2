//! Backdoor-guided branching (see `../PREREG.md`).
//!
//! Every `bb-*` solver shares A3's propagation (`sat_backdoors::Model`), one
//! DFS skeleton, the option-index value order and the forced-move rule; they
//! differ only in the branching-cell rule. Lookahead probes score cells and
//! never prune.

use std::time::{Duration, Instant};

use eternity2_engine::{Puzzle, BORDER};
use hard_ladder::{LadderSolver, NodeCounter, Outcome};
use sat_backdoors::{Model, Stats};

pub const FEATURES: [&str; 8] =
    ["ln_dom", "corner", "edge", "frame_adj", "degree", "rarity", "impact", "fail_frac"];
pub const N_FEATURES: usize = FEATURES.len();
pub const IMPACT: usize = 6;
pub const FAIL_FRAC: usize = 7;

/// Wall-clock guard per run (PREREG.md).
pub const TIME_GUARD: Duration = Duration::from_secs(240);

/// Options (bit indices) of cell `c` in state `d`.
#[must_use]
pub fn options(m: &Model, d: &[u64], c: usize) -> Vec<usize> {
    let mut v = Vec::new();
    for (k, &w) in m.dom(d, c).iter().enumerate() {
        let mut x = w;
        while x != 0 {
            v.push(k * 64 + x.trailing_zeros() as usize);
            x &= x - 1;
        }
    }
    v
}

/// Sum over cells of ln |D|.
#[must_use]
pub fn log_size(m: &Model, d: &[u64]) -> f64 {
    (0..m.cells).map(|c| f64::from(m.count(d, c).max(1)).ln()).sum()
}

/// Restrict cell `c` to option `o` and propagate. Returns false on contradiction.
pub fn assign(m: &Model, d: &mut [u64], c: usize, o: usize, st: &mut Stats) -> bool {
    let w = m.words;
    d[c * w..(c + 1) * w].iter_mut().for_each(|x| *x = 0);
    d[c * w + (o >> 6)] = 1u64 << (o & 63);
    m.propagate(d, &[c], st)
}

/// Probe results for one cell: (sum of exp(L_o - L), mean (L - L_o) over
/// non-failing probes, failing fraction).
#[must_use]
pub fn probe(m: &Model, d: &[u64], c: usize, l: f64, st: &mut Stats) -> (f64, f64, f64) {
    let opts = options(m, d, c);
    let (mut s, mut red, mut ok, mut fail) = (0.0, 0.0, 0usize, 0usize);
    let mut nd = d.to_vec();
    for &o in &opts {
        nd.copy_from_slice(d);
        if assign(m, &mut nd, c, o, st) {
            let lo = log_size(m, &nd);
            s += (lo - l).exp();
            red += l - lo;
            ok += 1;
        } else {
            fail += 1;
        }
    }
    let mean_red = if ok > 0 { red / ok as f64 } else { 0.0 };
    (s, mean_red, fail as f64 / opts.len().max(1) as f64)
}

/// Static per-puzzle data for the features.
pub struct Static {
    pub n: usize,
    /// rarity term of each option: mean over non-grey sides of 1/m(colour).
    pub opt_rarity: Vec<f64>,
}

impl Static {
    #[must_use]
    pub fn new(m: &Model, p: &Puzzle) -> Self {
        let mut census = vec![0u32; 256];
        for e in &p.pieces {
            for &c in e {
                if c != BORDER {
                    census[c as usize] += 1;
                }
            }
        }
        let opt_rarity = m
            .opt_edges
            .iter()
            .map(|e| {
                let sides: Vec<f64> =
                    e.iter().filter(|&&c| c != BORDER).map(|&c| 1.0 / f64::from(census[c as usize])).collect();
                sides.iter().sum::<f64>() / sides.len().max(1) as f64
            })
            .collect();
        Self { n: m.n, opt_rarity }
    }
}

/// Features of candidate cell `c` (PREREG.md). Probes only when `with_probe`.
#[must_use]
#[allow(clippy::too_many_arguments)]
pub fn features(
    m: &Model,
    s: &Static,
    d: &[u64],
    assigned: &[bool],
    c: usize,
    l: f64,
    with_probe: bool,
    st: &mut Stats,
) -> ([f64; N_FEATURES], f64) {
    let n = s.n;
    let (x, y) = (c % n, c / n);
    let on_x = x == 0 || x == n - 1;
    let on_y = y == 0 || y == n - 1;
    let corner = on_x && on_y;
    let edge = (on_x || on_y) && !corner;
    let frame_adj = !on_x && !on_y && (x == 1 || y == 1 || x == n - 2 || y == n - 2);
    let degree = m.neighbour[c].iter().flatten().filter(|&&nb| !assigned[nb]).count();
    let opts = options(m, d, c);
    let rarity = opts.iter().map(|&o| s.opt_rarity[o]).sum::<f64>() / opts.len().max(1) as f64;
    let (sum_exp, impact, fail_frac) = if with_probe { probe(m, d, c, l, st) } else { (0.0, 0.0, 0.0) };
    let b = |v: bool| if v { 1.0 } else { 0.0 };
    (
        [
            f64::from(m.count(d, c)).ln(),
            b(corner),
            b(edge),
            b(frame_adj),
            degree as f64,
            rarity,
            impact,
            fail_frac,
        ],
        sum_exp,
    )
}

/// Within-set z-scores of each feature column (sd 0 gives 0).
#[must_use]
pub fn zscore(rows: &[[f64; N_FEATURES]]) -> Vec<[f64; N_FEATURES]> {
    let k = rows.len() as f64;
    let mut out = rows.to_vec();
    for j in 0..N_FEATURES {
        let mean = rows.iter().map(|r| r[j]).sum::<f64>() / k;
        let var = rows.iter().map(|r| (r[j] - mean).powi(2)).sum::<f64>() / k;
        let sd = var.sqrt();
        for r in &mut out {
            r[j] = if sd > 1e-12 { (r[j] - mean) / sd } else { 0.0 };
        }
    }
    out
}

#[derive(Clone, Debug)]
pub enum Rule {
    Mrv,
    Impact,
    /// Weights on z-scored features; maximise.
    Learned([f64; N_FEATURES]),
    /// Single feature index and sign; maximise sign * feature.
    Single(usize, f64),
}

/// Learned model written by `analysis/stage1.py`.
#[must_use]
pub fn load_model() -> serde_json::Value {
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../results/stage1/model.json");
    let txt = std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("{}: {e} (run stage 1 first)", p.display()));
    serde_json::from_str(&txt).expect("model json")
}

#[must_use]
pub fn learned_rule() -> Rule {
    let v = load_model();
    let mut w = [0.0; N_FEATURES];
    for (j, x) in v["weights"].as_array().expect("weights").iter().enumerate() {
        w[j] = x.as_f64().expect("weight");
    }
    Rule::Learned(w)
}

#[must_use]
pub fn single_rule() -> Rule {
    let v = load_model();
    let name = v["single"]["feature"].as_str().expect("single feature");
    let j = FEATURES.iter().position(|f| *f == name).expect("known feature");
    Rule::Single(j, v["single"]["sign"].as_f64().expect("sign"))
}

pub struct BbSolver {
    pub rule: Rule,
    pub guard: Duration,
}

enum Step {
    Found,
    Failed,
    Cap,
}

struct Search<'a> {
    m: &'a Model,
    s: &'a Static,
    rule: &'a Rule,
    st: Stats,
    start: Instant,
    guard: Duration,
    board: Vec<u64>,
}

impl Search<'_> {
    fn needs_probe(&self) -> bool {
        match self.rule {
            Rule::Mrv => false,
            Rule::Impact => true,
            Rule::Learned(w) => w[IMPACT] != 0.0 || w[FAIL_FRAC] != 0.0,
            Rule::Single(j, _) => *j == IMPACT || *j == FAIL_FRAC,
        }
    }

    fn choose(&mut self, d: &[u64], assigned: &[bool]) -> usize {
        let m = self.m;
        let free: Vec<usize> = (0..m.cells).filter(|&c| !assigned[c]).collect();
        if let Some(&c) = free.iter().find(|&&c| m.count(d, c) == 1) {
            return c;
        }
        if let Rule::Mrv = self.rule {
            return *free.iter().min_by_key(|&&c| (m.count(d, c), c)).expect("free cell");
        }
        let probe = self.needs_probe();
        let l = log_size(m, d);
        let mut feats = Vec::with_capacity(free.len());
        let mut sums = Vec::with_capacity(free.len());
        for &c in &free {
            let (f, s) = features(m, self.s, d, assigned, c, l, probe, &mut self.st);
            feats.push(f);
            sums.push(s);
        }
        let score: Vec<f64> = match self.rule {
            Rule::Mrv => unreachable!(),
            Rule::Impact => sums.iter().map(|s| -s).collect(),
            Rule::Learned(w) => zscore(&feats)
                .iter()
                .map(|z| z.iter().zip(w).map(|(a, b)| a * b).sum())
                .collect(),
            Rule::Single(j, sign) => feats.iter().map(|f| sign * f[*j]).collect(),
        };
        let mut best = 0usize;
        for i in 1..free.len() {
            let better = match self.rule {
                // ties by |D| then index
                Rule::Impact => {
                    score[i] > score[best]
                        || (score[i] == score[best] && m.count(d, free[i]) < m.count(d, free[best]))
                }
                _ => score[i] > score[best],
            };
            if better {
                best = i;
            }
        }
        free[best]
    }

    fn dfs(&mut self, d: &[u64], assigned: &mut Vec<bool>, left: usize, counter: &mut NodeCounter) -> Step {
        if left == 0 {
            self.board = d.to_vec();
            return Step::Found;
        }
        let c = self.choose(d, assigned);
        let mut nd = d.to_vec();
        for o in options(self.m, d, c) {
            if !counter.try_assign() || self.start.elapsed() > self.guard {
                return Step::Cap;
            }
            nd.copy_from_slice(d);
            assigned[c] = true;
            if assign(self.m, &mut nd, c, o, &mut self.st) {
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

impl LadderSolver for BbSolver {
    fn solve(&mut self, puzzle: &Puzzle, counter: &mut NodeCounter) -> Outcome {
        let start = Instant::now();
        let m = Model::new(puzzle);
        if !m.root_ok {
            return Outcome::Exhausted;
        }
        let s = Static::new(&m, puzzle);
        let mut search = Search {
            m: &m,
            s: &s,
            rule: &self.rule,
            st: Stats { propagations: 1 },
            start,
            guard: self.guard,
            board: Vec::new(),
        };
        let mut assigned = vec![false; m.cells];
        let root = m.root().to_vec();
        let out = search.dfs(&root, &mut assigned, m.cells, counter);
        counter.propagation = search.st.propagations;
        match out {
            Step::Found => {
                let board = (0..m.cells)
                    .map(|c| {
                        let o = options(&m, &search.board, c)[0];
                        i32::from(m.opt_piece[o]) * 4 + i32::from(m.opt_rot[o])
                    })
                    .collect();
                Outcome::Solved(board)
            }
            Step::Failed => Outcome::Exhausted,
            Step::Cap => Outcome::CapReached,
        }
    }
}
