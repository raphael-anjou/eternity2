//! A20 arms (see `../PREREG.md`).
//!
//! `BjSolver` keeps bb-mrv's propagation (`sat_backdoors::Model::propagate`),
//! its cell rule (lowest-index forced cell, else min (|D|, index)) and its
//! option-index value order, but replaces the chronological stack by an
//! unordered decision set. On a dead end at cell x (every non-tabu option of x
//! fails on assignment) it evicts some decisions and re-propagates from the
//! root with the rest:
//!
//! - `Mode::Blocker` (the mechanism, IMS rule): over the oriented pieces in
//!   x's root domain, the direct blocker set B(o) is the decided orthogonal
//!   neighbours of x whose facing edge mismatches o plus the decided cell that
//!   already holds o's piece. Among non-tabu o with |B(o)| >= 1, take a
//!   minimum |B(o)| (ties uniformly at random) and evict B(o).
//! - `Mode::Random` (the matched twin): computes the same B(o) with the same
//!   tie-break draw, then evicts |B(o)| decisions drawn uniformly at random
//!   from all decisions instead.
//!
//! Both: if no candidate has |B(o)| >= 1, evict the most recent decision
//! (fallback, counted). Every evicted (cell, option) is tabu at that cell for
//! the next `tabu` nodes. If a dead end occurs with no decision left, the tabu
//! list is cleared (counted); if it was already empty the run returns
//! `Exhausted`. The method is incomplete: no no-goods are stored.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use backdoor_branching::options;
use eternity2_engine::Puzzle;
use hard_ladder::{LadderSolver, NodeCounter, Outcome};
use sat_backdoors::{Model, Stats};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Blocker,
    Random,
}

/// Per-run side statistics, one JSON line per run, drained by the runner.
pub static SIDECAR: Mutex<Vec<String>> = Mutex::new(Vec::new());

pub struct BjSolver {
    pub mode: Mode,
    pub tabu: u64,
    pub run_seed: u64,
    pub guard: Duration,
    pub label: &'static str,
}

fn splitmix(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn fnv(s: &str) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for b in s.bytes() {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    h
}

#[derive(Default, Debug, Clone)]
pub struct BjStats {
    pub dead_ends: u64,
    pub fallbacks: u64,
    pub evicted: u64,
    pub tabu_clears: u64,
    pub revisits: u64,
    pub max_mult: u32,
    pub distinct_states: u64,
    pub node_hash: u64,
    pub guard_fired: bool,
}

struct State<'a> {
    m: &'a Model,
    mode: Mode,
    tabu_len: u64,
    dom: Vec<u64>,
    /// Option placed at each cell by a decision, or None.
    at: Vec<Option<usize>>,
    /// Cell holding each piece by a decision, or None.
    piece_at: Vec<Option<usize>>,
    /// Decisions in the order made (cells).
    dec: Vec<usize>,
    tabu: Vec<Vec<(usize, u64)>>,
    rng_tie: u64,
    rng_twin: u64,
    zob: Vec<u64>,
    hash: u64,
    seen: HashMap<u64, u32>,
    st: Stats,
    bs: BjStats,
}

impl State<'_> {
    fn is_tabu(&self, c: usize, o: usize, now: u64) -> bool {
        self.tabu[c].iter().any(|&(t, e)| t == o && e > now)
    }

    fn choose(&self) -> usize {
        let m = self.m;
        let free = (0..m.cells).filter(|&c| self.at[c].is_none());
        if let Some(c) = free.clone().find(|&c| m.count(&self.dom, c) == 1) {
            return c;
        }
        free.min_by_key(|&c| (m.count(&self.dom, c), c))
            .expect("free cell")
    }

    fn place(&mut self, c: usize, o: usize, nd: Vec<u64>) {
        self.dom = nd;
        self.at[c] = Some(o);
        self.piece_at[self.m.opt_piece[o] as usize] = Some(c);
        self.dec.push(c);
        self.hash ^= self.zob[c * self.m.opt_piece.len() + o];
    }

    fn unplace(&mut self, c: usize, now: u64) {
        let o = self.at[c].take().expect("decided cell");
        self.piece_at[self.m.opt_piece[o] as usize] = None;
        self.dec.retain(|&x| x != c);
        self.hash ^= self.zob[c * self.m.opt_piece.len() + o];
        self.tabu[c].retain(|&(_, e)| e > now);
        if self.tabu_len > 0 {
            self.tabu[c].push((o, now + self.tabu_len));
        }
        self.bs.evicted += 1;
    }

    /// Direct blocker set of option `o` at cell `x`.
    fn blockers(&self, x: usize, o: usize) -> Vec<usize> {
        let m = self.m;
        let mut b = Vec::with_capacity(5);
        for d in 0..4 {
            if let Some(nb) = m.neighbour[x][d] {
                if let Some(on) = self.at[nb] {
                    if m.opt_edges[o][d] != m.opt_edges[on][(d + 2) & 3] {
                        b.push(nb);
                    }
                }
            }
        }
        if let Some(pc) = self.piece_at[m.opt_piece[o] as usize] {
            if pc != x && !b.contains(&pc) {
                b.push(pc);
            }
        }
        b
    }

    /// Repair after a dead end at `x`. Returns false if nothing can be evicted
    /// and the tabu list is already empty.
    fn repair(&mut self, x: usize, now: u64) -> bool {
        self.bs.dead_ends += 1;
        if self.dec.is_empty() {
            let had = self.tabu.iter().any(|t| t.iter().any(|&(_, e)| e > now));
            self.tabu.iter_mut().for_each(Vec::clear);
            self.bs.tabu_clears += 1;
            return had;
        }
        // Candidates: non-tabu options in x's root domain with a non-empty
        // direct blocker set; keep those of minimum size.
        let root_opts = options(self.m, self.m.root(), x);
        let mut best: Vec<Vec<usize>> = Vec::new();
        let mut best_k = usize::MAX;
        for o in root_opts {
            if self.is_tabu(x, o, now) {
                continue;
            }
            let b = self.blockers(x, o);
            if b.is_empty() {
                continue;
            }
            if b.len() < best_k {
                best_k = b.len();
                best.clear();
            }
            if b.len() == best_k {
                best.push(b);
            }
        }
        let evict: Vec<usize> = if best.is_empty() {
            self.bs.fallbacks += 1;
            vec![*self.dec.last().expect("decision")]
        } else {
            let pick = (splitmix(&mut self.rng_tie) % best.len() as u64) as usize;
            let b = best.swap_remove(pick);
            match self.mode {
                Mode::Blocker => b,
                Mode::Random => {
                    let mut pool = self.dec.clone();
                    let k = b.len().min(pool.len());
                    let mut out = Vec::with_capacity(k);
                    for _ in 0..k {
                        let j = (splitmix(&mut self.rng_twin) % pool.len() as u64) as usize;
                        out.push(pool.swap_remove(j));
                    }
                    out
                }
            }
        };
        for c in evict {
            self.unplace(c, now);
        }
        // Re-propagate from the root with the remaining decisions. Removing
        // decisions only loosens a monotone fixpoint, so this cannot fail.
        let w = self.m.words;
        let mut d = self.m.root().to_vec();
        for &c in &self.dec {
            let o = self.at[c].expect("decided");
            d[c * w..(c + 1) * w].iter_mut().for_each(|v| *v = 0);
            d[c * w + (o >> 6)] = 1u64 << (o & 63);
        }
        let changed = self.dec.clone();
        assert!(
            self.m.propagate(&mut d, &changed, &mut self.st),
            "re-propagation failed after eviction"
        );
        self.dom = d;
        let e = self.seen.entry(self.hash).or_insert(0);
        *e += 1;
        if *e > 1 {
            self.bs.revisits += 1;
        }
        self.bs.max_mult = self.bs.max_mult.max(*e);
        true
    }
}

impl LadderSolver for BjSolver {
    fn solve(&mut self, puzzle: &Puzzle, counter: &mut NodeCounter) -> Outcome {
        let start = Instant::now();
        let m = Model::new(puzzle);
        if !m.root_ok {
            return Outcome::Exhausted;
        }
        let n_opt = m.opt_piece.len();
        let mut zs = 0x5EED_2020_1112u64;
        let zob = (0..m.cells * n_opt).map(|_| splitmix(&mut zs)).collect();
        let base = self.run_seed ^ fnv(&puzzle.name);
        let mut s = State {
            m: &m,
            mode: self.mode,
            tabu_len: self.tabu,
            dom: m.root().to_vec(),
            at: vec![None; m.cells],
            piece_at: vec![None; m.piece_range.len()],
            dec: Vec::with_capacity(m.cells),
            tabu: vec![Vec::new(); m.cells],
            rng_tie: base ^ 0x7E1E_0001,
            rng_twin: base ^ 0x7E1E_0002,
            zob,
            hash: 0,
            seen: HashMap::new(),
            st: Stats { propagations: 1 },
            bs: BjStats {
                node_hash: 0xcbf2_9ce4_8422_2325,
                ..BjStats::default()
            },
        };
        let mut nd = vec![0u64; s.dom.len()];
        let out = 'search: loop {
            if s.dec.len() == m.cells {
                break 'search Some(true);
            }
            let c = s.choose();
            let mut placed = false;
            for o in options(&m, &s.dom, c) {
                if s.is_tabu(c, o, counter.nodes()) {
                    continue;
                }
                if !counter.try_assign() {
                    break 'search None;
                }
                if start.elapsed() > self.guard {
                    s.bs.guard_fired = true;
                    break 'search None;
                }
                for v in [c as u64, o as u64] {
                    s.bs.node_hash = (s.bs.node_hash ^ v).wrapping_mul(0x0100_0000_01b3);
                }
                nd.copy_from_slice(&s.dom);
                if backdoor_branching::assign(&m, &mut nd, c, o, &mut s.st) {
                    let keep = std::mem::replace(&mut nd, vec![0u64; s.dom.len()]);
                    s.place(c, o, keep);
                    placed = true;
                    break;
                }
                counter.backtracks += 1;
            }
            if placed {
                continue;
            }
            if !s.repair(c, counter.nodes()) {
                break 'search Some(false);
            }
        };
        counter.propagation = s.st.propagations;
        s.bs.distinct_states = s.seen.len() as u64;
        let bs = &s.bs;
        let line = serde_json::json!({
            "solver": self.label,
            "run_seed": self.run_seed,
            "tabu": self.tabu,
            "puzzle": puzzle.name,
            "nodes": counter.nodes(),
            "dead_ends": bs.dead_ends,
            "fallbacks": bs.fallbacks,
            "evicted": bs.evicted,
            "tabu_clears": bs.tabu_clears,
            "repairs_revisiting_state": bs.revisits,
            "max_state_multiplicity": bs.max_mult,
            "distinct_states": bs.distinct_states,
            "node_seq_hash": format!("{:016x}", bs.node_hash),
            "guard_fired": bs.guard_fired,
            "propagations": s.st.propagations,
        });
        SIDECAR.lock().expect("sidecar").push(line.to_string());
        match out {
            Some(true) => Outcome::Solved(
                (0..m.cells)
                    .map(|c| {
                        let o = s.at[c].expect("full board");
                        i32::from(m.opt_piece[o]) * 4 + i32::from(m.opt_rot[o])
                    })
                    .collect(),
            ),
            Some(false) => Outcome::Exhausted,
            None => Outcome::CapReached,
        }
    }
}
