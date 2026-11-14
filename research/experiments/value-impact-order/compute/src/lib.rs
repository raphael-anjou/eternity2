//! A21 arms (see `../PREREG.md`): bb-mrv with every option of the branching
//! cell probed by real propagation, then tried in an order chosen from the
//! probes. The cell rule, the propagation and the node rule are bb-mrv's.

use std::cell::Cell;
use std::time::{Duration, Instant};

use backdoor_branching::{assign, options};
use eternity2_engine::Puzzle;
use hard_ladder::{LadderSolver, NodeCounter, Outcome};
use sat_backdoors::{Model, Stats};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueOrder {
    /// bb-mrv's option-index order (implementation check arm).
    Index,
    /// Ascending options removed by propagation, failed last, ties by index.
    Impact,
    /// Failed last in index order, the non-failed ones shuffled (control).
    ProbeRand(u64),
}

thread_local! {
    /// FNV-1a hash of the last run's node sequence on this thread.
    pub static SEQ_HASH: Cell<u64> = const { Cell::new(0) };
    /// Probes made by the last run on this thread.
    pub static PROBES: Cell<u64> = const { Cell::new(0) };
}

pub struct ProbeSolver {
    pub order: ValueOrder,
    pub guard: Duration,
}

enum Step {
    Found,
    Failed,
    Cap,
}

const FNV_OFF: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0100_0000_01b3;

fn fnv(h: u64, x: u32) -> u64 {
    x.to_le_bytes()
        .iter()
        .fold(h, |h, &b| (h ^ u64::from(b)).wrapping_mul(FNV_PRIME))
}

struct Search<'a> {
    m: &'a Model,
    order: ValueOrder,
    rng: u64,
    st: Stats,
    probes: u64,
    hash: u64,
    start: Instant,
    guard: Duration,
    board: Vec<u64>,
}

impl Search<'_> {
    /// scb's generator (`SearchState::next_random`, splitmix64), as A22's `mrv-rand`.
    fn next_random(&mut self) -> u64 {
        self.rng = self.rng.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.rng;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// bb-mrv's cell rule.
    fn choose(&self, d: &[u64], assigned: &[bool]) -> usize {
        let m = self.m;
        let free = (0..m.cells).filter(|&c| !assigned[c]);
        if let Some(c) = free.clone().find(|&c| m.count(d, c) == 1) {
            return c;
        }
        free.min_by_key(|&c| (m.count(d, c), c)).expect("free cell")
    }

    fn dfs(
        &mut self,
        d: &[u64],
        assigned: &mut Vec<bool>,
        left: usize,
        counter: &mut NodeCounter,
    ) -> Step {
        if left == 0 {
            self.board = d.to_vec();
            return Step::Found;
        }
        let m = self.m;
        let c = self.choose(d, assigned);
        let opts = options(m, d, c);
        let k = opts.len();
        let len = d.len();
        // Probe every option: one propagate call each, no node.
        let mut buf = vec![0u64; k * len];
        let mut ok = vec![false; k];
        let mut impact = vec![u64::MAX; k];
        let before = m.total_options(d);
        for j in 0..k {
            let nd = &mut buf[j * len..(j + 1) * len];
            nd.copy_from_slice(d);
            self.probes += 1;
            ok[j] = assign(m, nd, c, opts[j], &mut self.st);
            if ok[j] {
                impact[j] = before - m.total_options(nd);
            }
        }
        let mut idx: Vec<usize> = (0..k).collect();
        match self.order {
            ValueOrder::Index => {}
            ValueOrder::Impact => idx.sort_by_key(|&j| (impact[j], j)),
            ValueOrder::ProbeRand(_) => {
                let mut live: Vec<usize> = (0..k).filter(|&j| ok[j]).collect();
                if live.len() > 1 {
                    for i in (1..live.len()).rev() {
                        let r = (self.next_random() as usize) % (i + 1);
                        live.swap(i, r);
                    }
                }
                live.extend((0..k).filter(|&j| !ok[j]));
                idx = live;
            }
        }
        for j in idx {
            if !counter.try_assign() || self.start.elapsed() > self.guard {
                return Step::Cap;
            }
            self.hash = fnv(fnv(self.hash, c as u32), opts[j] as u32);
            assigned[c] = true;
            if ok[j] {
                let nd = buf[j * len..(j + 1) * len].to_vec();
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

impl LadderSolver for ProbeSolver {
    fn solve(&mut self, puzzle: &Puzzle, counter: &mut NodeCounter) -> Outcome {
        let start = Instant::now();
        let m = Model::new(puzzle);
        SEQ_HASH.with(|h| h.set(FNV_OFF));
        PROBES.with(|p| p.set(0));
        if !m.root_ok {
            return Outcome::Exhausted;
        }
        let seed = match self.order {
            ValueOrder::ProbeRand(s) => s,
            _ => 0,
        };
        let mut search = Search {
            m: &m,
            order: self.order,
            rng: seed.wrapping_add(0x9E37_79B9_7F4A_7C15),
            st: Stats { propagations: 1 },
            probes: 0,
            hash: FNV_OFF,
            start,
            guard: self.guard,
            board: Vec::new(),
        };
        let mut assigned = vec![false; m.cells];
        let root = m.root().to_vec();
        let out = search.dfs(&root, &mut assigned, m.cells, counter);
        counter.propagation = search.st.propagations;
        SEQ_HASH.with(|h| h.set(search.hash));
        PROBES.with(|p| p.set(search.probes));
        match out {
            Step::Found => Outcome::Solved(
                (0..m.cells)
                    .map(|c| {
                        let o = options(&m, &search.board, c)[0];
                        i32::from(m.opt_piece[o]) * 4 + i32::from(m.opt_rot[o])
                    })
                    .collect(),
            ),
            Step::Failed => Outcome::Exhausted,
            Step::Cap => Outcome::CapReached,
        }
    }
}
