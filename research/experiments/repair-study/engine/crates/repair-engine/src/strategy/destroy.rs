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

    /// Select the cells to destroy into `out` (cleared first). `is_hint[pos]` marks
    /// a pinned cell that must never be selected. `scratch` is a reused buffer for
    /// the conflicted-cell list. Leaves `out` possibly empty (e.g. a conflict-driven
    /// operator on a board with no conflicts left). No heap allocation on the hot
    /// path: both `out` and `scratch` keep their capacity between calls.
    pub fn select(self, st: &State, is_hint: &[bool], rng: &mut Rng, out: &mut Vec<usize>, scratch: &mut Vec<usize>) {
        out.clear();
        match self {
            Self::RandomCells { k } => self.random(st, is_hint, k, rng, out),
            Self::MismatchedCells { k } => self.mismatched(st, k, rng, out),
            Self::WorstBand { rows } => self.worst_band(st, rows, out),
            Self::ComponentHalo { max } => self.component_halo(st, max, rng, out, scratch),
        }
        // Drop any hint cell in place (retain keeps capacity).
        out.retain(|&p| !is_hint[p]);
    }

    fn random(self, st: &State, is_hint: &[bool], k: usize, rng: &mut Rng, out: &mut Vec<usize>) {
        out.extend((0..W * H).filter(|&p| !is_hint[p] && !st.is_empty_at(p)));
        rng.shuffle(out);
        out.truncate(k);
    }

    fn mismatched(self, st: &State, k: usize, rng: &mut Rng, out: &mut Vec<usize>) {
        st.conflicted_cells_into(out);
        // Most-conflicted first, then a shuffle within equal conflict counts so
        // the operator is not deterministic across seeds.
        rng.shuffle(out);
        out.sort_by_key(|&p| std::cmp::Reverse(st.conflicts_at(p)));
        out.truncate(k);
    }

    fn worst_band(self, st: &State, rows: usize, out: &mut Vec<usize>) {
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
        out.extend((best_start * W..(best_start + rows) * W).filter(|&p| !st.is_empty_at(p)));
    }

    fn component_halo(self, st: &State, max: usize, rng: &mut Rng, out: &mut Vec<usize>, scratch: &mut Vec<usize>) {
        st.conflicted_cells_into(scratch);
        if scratch.is_empty() {
            return;
        }
        // Seed from a random conflicted cell, grow the connected component along
        // broken edges (BFS over conflict-adjacency) up to `max` cells.
        let seed = scratch[rng.below(scratch.len())];
        let mut in_set = [false; W * H];
        // BFS the connected conflict component into `out`, capped at `max`. `out`
        // holds the component cells in discovery order; `comp_len` marks where the
        // component ends and (below) the halo begins, so no separate `comp` clone
        // is needed. `head` is the queue cursor (a plain index into `out`, so `out`
        // doubles as the BFS queue).
        out.push(seed);
        in_set[seed] = true;
        let mut head = 0usize;
        while head < out.len() && out.len() < max {
            let pos = out[head];
            head += 1;
            for np in neighbours(pos) {
                if out.len() >= max {
                    break;
                }
                // Grow only into cells that are themselves conflicted, so the
                // component tracks the tangle rather than flooding the board.
                if !in_set[np] && st.conflicts_at(np) > 0 {
                    in_set[np] = true;
                    out.push(np);
                }
            }
        }
        // One-cell halo: every placed neighbour of a component cell.
        let comp_len = out.len();
        for i in 0..comp_len {
            let pos = out[i];
            for np in neighbours(pos) {
                if !in_set[np] && !st.is_empty_at(np) {
                    in_set[np] = true;
                    out.push(np);
                }
            }
        }
    }
}

/// The 4-neighbourhood of a cell, on-board only, in URDL order. Allocation-free:
/// yields from a fixed-size array rather than a heap `Vec`, so the BFS/halo passes
/// that call this per expanded cell make no per-iteration allocations. The yield
/// order (up, right, down, left) is preserved, so BFS visit order is unchanged.
#[inline]
fn neighbours(pos: usize) -> impl Iterator<Item = usize> {
    let (y, x) = (pos / W, pos % W);
    [
        if y > 0 { Some(pos - W) } else { None },
        if x + 1 < W { Some(pos + 1) } else { None },
        if y + 1 < H { Some(pos + W) } else { None },
        if x > 0 { Some(pos - 1) } else { None },
    ]
    .into_iter()
    .flatten()
}
