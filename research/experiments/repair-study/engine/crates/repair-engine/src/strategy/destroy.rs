//! The destroy operator: which cells the loop rips out each iteration. This is
//! the study's headline axis, because *where* you demolish decides what the
//! repair step even has a chance to fix. A partial Eternity II board tells you
//! exactly where it hurts — the mismatched edges — so most operators here are
//! conflict-aware, drawing from the live conflict map the [`State`] maintains.
//!
//! Pinned hint cells are never destroyed: they are filtered out of every
//! operator's output, so a corner-pinned variant keeps its pins across the whole
//! run.

use e2_core::{H, W};

use crate::state::{Rng, State};

/// Which cells to unplace each iteration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Destroy {
    /// `k` uniformly random non-hint cells. The geometry-blind control: it does
    /// not look at where the board is broken at all.
    RandomCells { k: usize },
    /// Cells incident to a broken edge, up to `k`, drawn from the conflict map.
    /// The cheapest conflict-aware operator, and the vault's `MismatchedCells`.
    MismatchedCells { k: usize },
    /// The single worst *band* of `rows` consecutive rows (the contiguous row
    /// window carrying the most conflicts), all of its non-hint cells destroyed.
    /// The vault's `WorstBand`: a large, spatially-coherent hole.
    WorstBand { rows: usize },
    /// One connected component of the mismatch graph (broken edges as graph
    /// edges), grown from the worst cell up to `max` cells, plus a one-cell halo.
    /// The vault's `ComponentPlusHaloDestroy`: destroy exactly the tangle, and a
    /// ring around it so the repair has room to re-seat the boundary.
    ComponentHalo { max: usize },
}

impl Destroy {
    #[must_use]
    pub const fn tag(self) -> &'static str {
        match self {
            Self::RandomCells { .. } => "random-cells",
            Self::MismatchedCells { .. } => "mismatched-cells",
            Self::WorstBand { .. } => "worst-band",
            Self::ComponentHalo { .. } => "component-halo",
        }
    }

    /// Select the cells to destroy. `is_hint[pos]` marks a pinned cell that must
    /// never be selected. Returns positions to unplace (possibly empty, e.g. a
    /// conflict-driven operator on a board with no conflicts left).
    #[must_use]
    pub fn select(self, st: &State, is_hint: &[bool], rng: &mut Rng) -> Vec<usize> {
        let out = match self {
            Self::RandomCells { k } => self.random(st, is_hint, k, rng),
            Self::MismatchedCells { k } => self.mismatched(st, k, rng),
            Self::WorstBand { rows } => self.worst_band(st, rows),
            Self::ComponentHalo { max } => self.component_halo(st, max, rng),
        };
        out.into_iter().filter(|&p| !is_hint[p]).collect()
    }

    fn random(self, st: &State, is_hint: &[bool], k: usize, rng: &mut Rng) -> Vec<usize> {
        let mut pool: Vec<usize> = (0..W * H).filter(|&p| !is_hint[p] && !st.is_empty_at(p)).collect();
        rng.shuffle(&mut pool);
        pool.truncate(k);
        pool
    }

    fn mismatched(self, st: &State, k: usize, rng: &mut Rng) -> Vec<usize> {
        let mut cells = st.conflicted_cells();
        // Most-conflicted first, then a shuffle within equal conflict counts so
        // the operator is not deterministic across seeds.
        rng.shuffle(&mut cells);
        cells.sort_by_key(|&p| std::cmp::Reverse(st.conflicts_at(p)));
        cells.truncate(k);
        cells
    }

    fn worst_band(self, st: &State, rows: usize) -> Vec<usize> {
        let rows = rows.clamp(1, H);
        // Conflicts per row, then the sliding window of `rows` rows with the most.
        let mut per_row = [0u32; H];
        for pos in 0..W * H {
            per_row[pos / W] += u32::from(st.conflicts_at(pos));
        }
        let (mut best_start, mut best_sum) = (0usize, 0u32);
        for start in 0..=(H - rows) {
            let sum: u32 = per_row[start..start + rows].iter().sum();
            if sum > best_sum {
                best_sum = sum;
                best_start = start;
            }
        }
        (best_start * W..(best_start + rows) * W).filter(|&p| !st.is_empty_at(p)).collect()
    }

    fn component_halo(self, st: &State, max: usize, rng: &mut Rng) -> Vec<usize> {
        let conflicted = st.conflicted_cells();
        if conflicted.is_empty() {
            return Vec::new();
        }
        // Seed from a random conflicted cell, grow the connected component along
        // broken edges (BFS over conflict-adjacency) up to `max` cells.
        let seed = conflicted[rng.below(conflicted.len())];
        let mut in_set = vec![false; W * H];
        let mut comp = Vec::new();
        let mut queue = vec![seed];
        in_set[seed] = true;
        while let Some(pos) = queue.pop() {
            comp.push(pos);
            if comp.len() >= max {
                break;
            }
            for np in neighbours(pos) {
                // Grow only into cells that are themselves conflicted, so the
                // component tracks the tangle rather than flooding the board.
                if !in_set[np] && st.conflicts_at(np) > 0 {
                    in_set[np] = true;
                    queue.push(np);
                }
            }
        }
        // One-cell halo: every placed neighbour of a component cell.
        let mut with_halo = comp.clone();
        for &pos in &comp {
            for np in neighbours(pos) {
                if !in_set[np] && !st.is_empty_at(np) {
                    in_set[np] = true;
                    with_halo.push(np);
                }
            }
        }
        with_halo
    }
}

/// The 4-neighbourhood of a cell, on-board only.
#[inline]
fn neighbours(pos: usize) -> Vec<usize> {
    let (y, x) = (pos / W, pos % W);
    let mut v = Vec::with_capacity(4);
    if y > 0 {
        v.push(pos - W);
    }
    if x + 1 < W {
        v.push(pos + 1);
    }
    if y + 1 < H {
        v.push(pos + W);
    }
    if x > 0 {
        v.push(pos - 1);
    }
    v
}
