//! Hall matching on top of bb-mrv (see `../PREREG.md`).
//!
//! `HallMrv` is A16's `BbSolver { rule: Rule::Mrv }` search (A3 propagation
//! via `sat_backdoors::Model::propagate`, forced cells first, then min
//! (|D|, index), option-index value order) with one hook after a successful
//! propagation: the Hall check of A17. The check builds, for every unassigned
//! cell in scope, the set of pieces with at least one (piece, rotation) still
//! in the cell's propagated domain, and asks whether the cells can be given
//! pairwise distinct pieces (Kuhn augmenting paths). It never changes a
//! domain; on failure (and only when `prune`) the assignment is treated as a
//! propagation failure.

use std::time::{Duration, Instant};

use backdoor_branching::{assign, options};
use hard_ladder::{LadderSolver, NodeCounter, Outcome};
use sat_backdoors::{Model, Stats};

/// Which cells the check covers and how often it runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hall {
    /// 0 = off, 1 = frontier (unassigned with an assigned neighbour), 2 = every unassigned cell.
    pub scope: u8,
    /// Run on every `period`-th successful propagation (1 = every node).
    pub period: u32,
    /// false = compute the check but never act on it (cost control).
    pub prune: bool,
}

pub const OFF: Hall = Hall {
    scope: 0,
    period: 1,
    prune: false,
};

/// One `try_assign` that succeeded: (depth, cell, option, pruned by Hall).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TraceEntry {
    pub depth: u32,
    pub cell: u32,
    pub opt: u32,
    pub hall_pruned: bool,
}

/// Counters filled by a run.
#[derive(Debug, Clone, Default)]
pub struct HallStats {
    /// Hall checks computed.
    pub checks: u64,
    /// Checks that found no matching (pruned if `prune`, else only counted).
    pub fails: u64,
    /// FNV-1a over (cell, option) of every counted node, in order.
    pub seq_hash: u64,
    /// The run stopped on the wall-clock guard.
    pub guard_stop: bool,
}

pub struct HallMrv {
    pub hall: Hall,
    pub guard: Duration,
    pub stats: HallStats,
    pub trace: Option<Vec<TraceEntry>>,
}

impl HallMrv {
    #[must_use]
    pub fn new(hall: Hall, guard: Duration) -> Self {
        Self {
            hall,
            guard,
            stats: HallStats::default(),
            trace: None,
        }
    }
}

/// True iff every mask (a cell's admissible pieces) can be given a distinct
/// piece. Same routine as A17's `hall_match` (cells sorted by popcount).
#[must_use]
pub fn hall_match(masks: &mut [u128]) -> bool {
    fn augment(c: usize, masks: &[u128], owner: &mut [usize; 128], seen: &mut u128) -> bool {
        let mut cand = masks[c] & !*seen;
        while cand != 0 {
            let p = cand.trailing_zeros() as usize;
            cand &= cand - 1;
            *seen |= 1u128 << p;
            if owner[p] == usize::MAX || augment(owner[p], masks, owner, seen) {
                owner[p] = c;
                return true;
            }
        }
        false
    }
    masks.sort_unstable_by_key(|m| m.count_ones());
    let mut owner = [usize::MAX; 128];
    for c in 0..masks.len() {
        let mut seen = 0u128;
        if !augment(c, masks, &mut owner, &mut seen) {
            return false;
        }
    }
    true
}

/// Hall feasibility of the unassigned cells in `scope` in state `d`.
/// Pieces of assigned cells are already absent from every other domain
/// (A3's single-piece rule), so no used-piece mask is needed.
#[must_use]
pub fn hall_feasible(
    m: &Model,
    d: &[u64],
    assigned: &[bool],
    scope: u8,
    masks: &mut Vec<u128>,
) -> bool {
    assert!(m.piece_range.len() <= 128, "Hall masks are u128");
    masks.clear();
    for c in 0..m.cells {
        if assigned[c] {
            continue;
        }
        if scope == 1
            && !m.neighbour[c]
                .iter()
                .any(|nb| nb.is_some_and(|a| assigned[a]))
        {
            continue;
        }
        let mut mask = 0u128;
        for (k, &w) in m.dom(d, c).iter().enumerate() {
            let mut x = w;
            while x != 0 {
                let o = k * 64 + x.trailing_zeros() as usize;
                x &= x - 1;
                mask |= 1u128 << m.opt_piece[o];
            }
        }
        if mask == 0 {
            return false;
        }
        masks.push(mask);
    }
    hall_match(masks)
}

enum Step {
    Found,
    Failed,
    Cap,
}

struct Search<'a> {
    m: &'a Model,
    hall: Hall,
    tick: u64,
    masks: Vec<u128>,
    st: Stats,
    hs: HallStats,
    trace: Option<Vec<TraceEntry>>,
    start: Instant,
    guard: Duration,
    board: Vec<u64>,
}

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

impl Search<'_> {
    /// Same as backdoor-branching `Search::choose` for `Rule::Mrv`.
    fn choose(&self, d: &[u64], assigned: &[bool]) -> usize {
        let m = self.m;
        let free: Vec<usize> = (0..m.cells).filter(|&c| !assigned[c]).collect();
        if let Some(&c) = free.iter().find(|&&c| m.count(d, c) == 1) {
            return c;
        }
        *free
            .iter()
            .min_by_key(|&&c| (m.count(d, c), c))
            .expect("free cell")
    }

    fn hash(&mut self, c: usize, o: usize) {
        let mut h = self.hs.seq_hash;
        for b in (c as u32)
            .to_le_bytes()
            .into_iter()
            .chain((o as u32).to_le_bytes())
        {
            h ^= u64::from(b);
            h = h.wrapping_mul(FNV_PRIME);
        }
        self.hs.seq_hash = h;
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
        let c = self.choose(d, assigned);
        let mut nd = d.to_vec();
        for o in options(self.m, d, c) {
            if !counter.try_assign() {
                return Step::Cap;
            }
            if self.start.elapsed() > self.guard {
                self.hs.guard_stop = true;
                return Step::Cap;
            }
            self.hash(c, o);
            nd.copy_from_slice(d);
            assigned[c] = true;
            let mut ok = assign(self.m, &mut nd, c, o, &mut self.st);
            let mut pruned = false;
            if ok && self.hall.scope != 0 {
                self.tick += 1;
                if self.tick.is_multiple_of(u64::from(self.hall.period.max(1))) {
                    self.hs.checks += 1;
                    if !hall_feasible(self.m, &nd, assigned, self.hall.scope, &mut self.masks) {
                        self.hs.fails += 1;
                        if self.hall.prune {
                            ok = false;
                            pruned = true;
                        }
                    }
                }
            }
            if let Some(t) = self.trace.as_mut() {
                t.push(TraceEntry {
                    depth: (self.m.cells - left) as u32,
                    cell: c as u32,
                    opt: o as u32,
                    hall_pruned: pruned,
                });
            }
            if ok {
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

impl LadderSolver for HallMrv {
    fn solve(&mut self, puzzle: &eternity2_engine::Puzzle, counter: &mut NodeCounter) -> Outcome {
        let start = Instant::now();
        let m = Model::new(puzzle);
        if !m.root_ok {
            return Outcome::Exhausted;
        }
        let mut search = Search {
            m: &m,
            hall: self.hall,
            tick: 0,
            masks: Vec::with_capacity(m.cells),
            st: Stats { propagations: 1 },
            hs: HallStats {
                seq_hash: FNV_OFFSET,
                ..HallStats::default()
            },
            trace: self.trace.take(),
            start,
            guard: self.guard,
            board: Vec::new(),
        };
        let mut assigned = vec![false; m.cells];
        let root = m.root().to_vec();
        let out = search.dfs(&root, &mut assigned, m.cells, counter);
        counter.propagation = search.st.propagations;
        self.stats = search.hs.clone();
        self.trace = search.trace.take();
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

/// Checks that `cand` is `base` with whole subtrees removed below nodes that
/// `cand` pruned by Hall. Returns the number of base nodes
/// skipped, or an error naming the first mismatch.
pub fn subtree_removal(base: &[TraceEntry], cand: &[TraceEntry]) -> Result<u64, String> {
    let (mut i, mut skipped) = (0usize, 0u64);
    for (j, e) in cand.iter().enumerate() {
        let Some(b) = base.get(i) else {
            // base hit its cap earlier; cand may legitimately go further
            return Ok(skipped);
        };
        if (b.depth, b.cell, b.opt) != (e.depth, e.cell, e.opt) {
            return Err(format!("mismatch at cand {j} / base {i}: {b:?} vs {e:?}"));
        }
        i += 1;
        if e.hall_pruned {
            while base.get(i).is_some_and(|x| x.depth > e.depth) {
                i += 1;
                skipped += 1;
            }
        }
    }
    Ok(skipped)
}
