//! Width-W relaxed decision diagram: an upper bound on the best total score
//! (matched-edges) of any completion of a row-major prefix.
//!
//! Layer t is the cell d+t. A node holds
//! - `fr`: for each column x, the set of colours the next unfilled cell of that
//!   column may see on its top side (slot x), and the set of colours the next
//!   cell may see on its left side (slot n); an exact node has singleton sets;
//! - `all`: pieces used on every path to the node (prefix included);
//! - `some`: pieces used on some path to the node (prefix included);
//! - `val`: the largest number of remaining edges matched on a path to it.
//!
//! A piece is unavailable at a node if it is in `all`, or, when |some| equals
//! the number of filled cells, if it is in `some` (every path then used
//! exactly `some`). A transition places an available piece in a valid
//! orientation and gains one per scored side whose colour lies in the slot's
//! set. Merge rule (preregistered): identical full states are combined first
//! (exact, max of val). If a layer then has more than W nodes, nodes with an
//! identical frontier are merged (step 1, relaxes pieces only). If still more
//! than W, nodes are sorted by val descending, then frontier ascending; the
//! first W-1 are kept and the rest are merged into one node (step 2, relaxes
//! frontier colours). A merge takes the union of every frontier set, the
//! intersection of `all`, the union of `some`, and the max of `val`.
//! Validity: see PREREG.md (every completion of the prefix is a path whose
//! gains are its matched edges; a merged node dominates each node it absorbs).

use crate::model::{Model, Prefix};
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};

/// Deterministic FxHash-style hasher (no random state, reproducible order).
#[derive(Default)]
pub struct Fx(u64);
impl Hasher for Fx {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.write_u64(b as u64);
        }
    }
    fn write_u64(&mut self, i: u64) {
        self.0 = (self.0.rotate_left(5) ^ i).wrapping_mul(0x51_7c_c1_b7_27_22_0a_95);
    }
    fn write_u16(&mut self, i: u16) {
        self.write_u64(i as u64);
    }
    fn write_usize(&mut self, i: usize) {
        self.write_u64(i as u64);
    }
}
pub type FxMap<K, V> = HashMap<K, V, BuildHasherDefault<Fx>>;

pub type Frontier = [u16; 9];

#[derive(Clone, Copy, Debug)]
struct Node {
    fr: Frontier,
    all: u64,
    some: u64,
    val: i32,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DdOut {
    pub w: usize,
    /// Upper bound on the best total score of a completion.
    pub bound: u32,
    /// Sum of layer sizes after merging (nodes expanded).
    pub nodes: u64,
    /// Transitions generated (node, available option) pairs.
    pub transitions: u64,
    /// Nodes absorbed by step-1 merges (identical frontier, pieces relaxed).
    pub absorbed_pieces_only: u64,
    /// Nodes absorbed by step-2 merges (frontier colours relaxed).
    pub absorbed_frontier: u64,
    /// Layers in which a step-2 merge happened.
    pub layers_frontier_relaxed: u32,
    pub layers: u32,
    pub wall_us: u64,
}

pub fn initial_frontier(m: &Model, pre: &Prefix) -> Frontier {
    let n = m.n;
    let d = pre.d();
    let mut fr = [0u16; 9];
    for x in 0..n {
        // Last filled cell of column x.
        let mut last = None;
        let mut pos = x;
        while pos < d {
            last = Some(pos);
            pos += n;
        }
        if let Some(p) = last {
            if p / n < n - 1 {
                fr[x] = 1 << pre.cells[p].e[2];
            }
        }
    }
    if d % n != 0 {
        fr[n] = 1 << pre.cells[d - 1].e[1];
    }
    fr
}

pub fn dd_bound(m: &Model, pre: &Prefix, w: usize) -> DdOut {
    assert!(w >= 1);
    let t0 = std::time::Instant::now();
    let n = m.n;
    let d = pre.d();
    let n_cells = m.n_cells();
    let used = pre.used();
    let mut out = DdOut {
        w,
        ..Default::default()
    };
    let mut layer: Vec<Node> = vec![Node {
        fr: initial_frontier(m, pre),
        all: used,
        some: used,
        val: 0,
    }];
    for pos in d..n_cells {
        out.layers += 1;
        out.nodes += layer.len() as u64;
        let (x, y) = (pos % n, pos / n);
        let mut next: FxMap<(Frontier, u64, u64), i32> = FxMap::default();
        for nd in &layer {
            let unavail = if nd.some.count_ones() as usize == pos {
                nd.some
            } else {
                nd.all
            };
            for o in &m.opts[pos] {
                if unavail >> o.p & 1 == 1 {
                    continue;
                }
                out.transitions += 1;
                let mut gain = 0i32;
                if y > 0 && nd.fr[x] >> o.e[0] & 1 == 1 {
                    gain += 1;
                }
                if x > 0 && nd.fr[n] >> o.e[3] & 1 == 1 {
                    gain += 1;
                }
                let mut fr = nd.fr;
                fr[x] = if y < n - 1 { 1 << o.e[2] } else { 0 };
                fr[n] = if x < n - 1 { 1 << o.e[1] } else { 0 };
                let bit = 1u64 << o.p;
                let key = (fr, nd.all | bit, nd.some | bit);
                let v = nd.val + gain;
                let e = next.entry(key).or_insert(i32::MIN);
                if v > *e {
                    *e = v;
                }
            }
        }
        let mut nodes: Vec<Node> = next
            .into_iter()
            .map(|((fr, all, some), val)| Node { fr, all, some, val })
            .collect();
        if nodes.len() > w {
            // Step 1: merge identical frontiers (pieces relaxed only).
            let mut by_fr: FxMap<Frontier, Node> = FxMap::default();
            for nd in nodes.drain(..) {
                match by_fr.get_mut(&nd.fr) {
                    None => {
                        by_fr.insert(nd.fr, nd);
                    }
                    Some(g) => {
                        g.all &= nd.all;
                        g.some |= nd.some;
                        g.val = g.val.max(nd.val);
                        out.absorbed_pieces_only += 1;
                    }
                }
            }
            nodes = by_fr.into_values().collect();
            if nodes.len() > w {
                // Step 2: keep the W-1 best, merge the rest into one node.
                nodes.sort_unstable_by(|a, b| b.val.cmp(&a.val).then(a.fr.cmp(&b.fr)));
                let rest = nodes.split_off(w - 1);
                out.absorbed_frontier += (rest.len() - 1) as u64;
                out.layers_frontier_relaxed += 1;
                let mut g = rest[0];
                for nd in &rest[1..] {
                    for i in 0..9 {
                        g.fr[i] |= nd.fr[i];
                    }
                    g.all &= nd.all;
                    g.some |= nd.some;
                    g.val = g.val.max(nd.val);
                }
                nodes.push(g);
            }
        }
        layer = nodes;
    }
    let best = layer
        .iter()
        .map(|nd| nd.val)
        .max()
        .expect("a completion path exists");
    out.bound = pre.score(m) + best as u32;
    out.wall_us = t0.elapsed().as_micros() as u64;
    out
}

/// The trivial bound: prefix score plus every remaining edge.
pub fn trivial_bound(m: &Model, pre: &Prefix) -> u32 {
    pre.score(m) + m.remaining_edges(pre.d())
}
