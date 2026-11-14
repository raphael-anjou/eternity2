//! Copy of `backdoor_branching::BbSolver`'s DFS for `Rule::Mrv` (A16, A22
//! binding arm), with hooks added. The branching code (cell choice, option
//! order, `try_assign` placement, `assign`, backtrack count) is unchanged;
//! the node identity gate in `main.rs` checks it against committed rows.

use backdoor_branching::{assign, options};
use hard_ladder::NodeCounter;
use sat_backdoors::{Model, Stats};

/// Exit outcome of a node's subtree.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Exit {
    Dead,
    Found,
    Cap,
}

pub trait Hook {
    /// Called once per counted node, after its propagation. `parent` is the
    /// state before the assignment, `nd` the state after (meaningless when
    /// `!ok`), `assigned` already marks `c`.
    #[allow(clippy::too_many_arguments)]
    fn node(
        &mut self,
        id: u64,
        depth: usize,
        ok: bool,
        parent: &[u64],
        c: usize,
        o: usize,
        nd: &[u64],
        assigned: &[bool],
    );
    /// Called when a node whose propagation succeeded leaves the stack.
    fn exit(&mut self, id: u64, size: u64, how: Exit);
}

pub struct NoHook;
impl Hook for NoHook {
    fn node(
        &mut self,
        _: u64,
        _: usize,
        _: bool,
        _: &[u64],
        _: usize,
        _: usize,
        _: &[u64],
        _: &[bool],
    ) {
    }
    fn exit(&mut self, _: u64, _: u64, _: Exit) {}
}

/// bb-mrv cell choice: forced (|D| = 1) lowest index first, then min (|D|, index).
#[must_use]
pub fn choose(m: &Model, d: &[u64], assigned: &[bool]) -> usize {
    let free: Vec<usize> = (0..m.cells).filter(|&c| !assigned[c]).collect();
    if let Some(&c) = free.iter().find(|&&c| m.count(d, c) == 1) {
        return c;
    }
    *free
        .iter()
        .min_by_key(|&&c| (m.count(d, c), c))
        .expect("free cell")
}

pub struct Run<'a, H: Hook> {
    pub m: &'a Model,
    pub st: Stats,
    pub hook: H,
    /// FNV-1a over (cell, option) of every counted node, in order.
    pub hash: u64,
    pub board: Vec<u64>,
}

impl<'a, H: Hook> Run<'a, H> {
    pub fn new(m: &'a Model, hook: H) -> Self {
        Self {
            m,
            st: Stats { propagations: 1 },
            hook,
            hash: 0xcbf2_9ce4_8422_2325,
            board: Vec::new(),
        }
    }

    fn mix(&mut self, x: u64) {
        for b in x.to_le_bytes() {
            self.hash ^= u64::from(b);
            self.hash = self.hash.wrapping_mul(0x100_0000_01b3);
        }
    }

    pub fn dfs(
        &mut self,
        d: &[u64],
        assigned: &mut Vec<bool>,
        left: usize,
        counter: &mut NodeCounter,
    ) -> Exit {
        if left == 0 {
            self.board = d.to_vec();
            return Exit::Found;
        }
        let c = choose(self.m, d, assigned);
        let depth = self.m.cells - left + 1;
        let mut nd = d.to_vec();
        for o in options(self.m, d, c) {
            if !counter.try_assign() {
                return Exit::Cap;
            }
            let id = counter.nodes();
            self.mix(((c as u64) << 32) | o as u64);
            nd.copy_from_slice(d);
            assigned[c] = true;
            let ok = assign(self.m, &mut nd, c, o, &mut self.st);
            self.hook.node(id, depth, ok, d, c, o, &nd, assigned);
            if ok {
                let r = self.dfs(&nd, assigned, left - 1, counter);
                let size = counter.nodes() - id + 1;
                match r {
                    Exit::Dead => self.hook.exit(id, size, Exit::Dead),
                    done => {
                        self.hook.exit(id, size, done);
                        return done;
                    }
                }
            }
            assigned[c] = false;
            counter.backtracks += 1;
        }
        Exit::Dead
    }
}

/// Result of one full replay.
pub struct Replay<H> {
    pub how: Exit,
    pub nodes: u64,
    pub propagation: u64,
    pub backtracks: u64,
    pub hash: u64,
    pub hook: H,
    pub board: Vec<u64>,
}

/// Run bb-mrv from the root of `m` with `hook` under `cap`.
pub fn replay<H: Hook>(m: &Model, hook: H, cap: u64) -> Replay<H> {
    let mut counter = NodeCounter::new(cap);
    let mut run = Run::new(m, hook);
    let how = if m.root_ok {
        let root = m.root().to_vec();
        let mut assigned = vec![false; m.cells];
        run.dfs(&root, &mut assigned, m.cells, &mut counter)
    } else {
        Exit::Dead
    };
    Replay {
        how,
        nodes: counter.nodes(),
        propagation: run.st.propagations,
        backtracks: counter.backtracks,
        hash: run.hash,
        board: run.board,
        hook: run.hook,
    }
}

/// Complete exact search from a state (no cap): does a completion exist?
#[must_use]
pub fn completes(m: &Model, d: &[u64], assigned: &[bool]) -> bool {
    let mut counter = NodeCounter::new(u64::MAX);
    let mut run = Run::new(m, NoHook);
    let mut a = assigned.to_vec();
    let left = a.iter().filter(|&&x| !x).count();
    run.dfs(d, &mut a, left, &mut counter) == Exit::Found
}
