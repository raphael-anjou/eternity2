//! The repair step: how the hole left by the destroy operator is refilled from
//! the pool of lifted pieces. This is the second axis. The destroy chose *where*
//! to work; the repair decides *how well* the region is rebuilt, and the two
//! interact — a smarter repair pays off only where the destroy left it something
//! solvable.
//!
//! Every repair here works only on the destroyed cells and the pieces lifted
//! from them, using the [`State`]'s `O(1)` delta scoring, so an iteration costs
//! `O(k · |pool|)` rather than a board rescan.

use crate::state::{Rng, State};

/// How the destroyed cells are refilled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Repair {
    /// Most-constrained cell first (the empty cell with the most placed
    /// neighbours), best-matching piece placed, ties broken deterministically by
    /// id. The standard greedy refill.
    Greedy,
    /// The same greedy pass, but exact score ties are broken by a seeded coin —
    /// the controlled-exploration knob the vault's producer study found is
    /// mandatory, so a repair is not one deterministic rebuild.
    GreedyJitterTies,
    /// For a *small* hole (`<= exact_max` cells): try the pieces in the hole
    /// against the cells by a bounded exhaustive assignment, keeping the best
    /// total. Falls back to greedy above the size cap, where exact is too slow.
    /// This is the small-subproblem exact refill the ALNS theory page describes.
    ExactSmall { exact_max: usize },
}

impl Repair {
    #[must_use]
    pub const fn tag(self) -> &'static str {
        match self {
            Self::Greedy => "greedy",
            Self::GreedyJitterTies => "greedy-jitter",
            Self::ExactSmall { .. } => "exact-small",
        }
    }

    /// Refill `cells` (currently empty in `st`) using the pieces in `pool`
    /// (currently lifted, i.e. not on the board). On return every cell in `cells`
    /// is filled and every pool piece is placed. Assumes `cells.len() ==
    /// pool.len()`. `remaining`/`open` are reused scratch buffers (cleared here),
    /// so the refill allocates nothing on the hot path.
    pub fn refill(
        self,
        st: &mut State,
        cells: &[usize],
        pool: &[u16],
        rng: &mut Rng,
        remaining: &mut Vec<u16>,
        open: &mut Vec<usize>,
    ) {
        match self {
            Self::Greedy => self.greedy(st, cells, pool, false, rng, remaining, open),
            Self::GreedyJitterTies => self.greedy(st, cells, pool, true, rng, remaining, open),
            Self::ExactSmall { exact_max } => {
                if cells.len() <= exact_max {
                    self.exact(st, cells, pool, rng);
                } else {
                    self.greedy(st, cells, pool, true, rng, remaining, open);
                }
            }
        }
    }

    /// Most-constrained-cell-first greedy refill. Places one piece per step: pick
    /// the empty target cell with the most placed neighbours, then the (piece,
    /// rotation) from the remaining pool that gains the most matched edges.
    #[allow(clippy::too_many_arguments)]
    fn greedy(
        self,
        st: &mut State,
        cells: &[usize],
        pool: &[u16],
        jitter: bool,
        rng: &mut Rng,
        remaining: &mut Vec<u16>,
        open: &mut Vec<usize>,
    ) {
        remaining.clear();
        remaining.extend_from_slice(pool);
        open.clear();
        open.extend_from_slice(cells);

        while !open.is_empty() {
            // Most-constrained cell: the empty target with the fewest still-empty
            // neighbours (equivalently, the most already-placed ones).
            let (ti, _) = open
                .iter()
                .enumerate()
                .max_by_key(|&(_, &pos)| placed_neighbours(st, pos))
                .expect("open non-empty");
            let target = open.swap_remove(ti);

            // The target's neighbour context is the same for every candidate piece
            // and rotation, so compute it once here instead of re-walking the
            // neighbourhood inside delta_if_placed 4x per piece.
            let ctx = st.target_context(target);

            let mut best: Option<(i32, usize, u8)> = None; // (score, pool index, rot)
            for (pi, &pid) in remaining.iter().enumerate() {
                let base = st.pieces.get(pid).map_or([e2_core::BORDER; 4], |p| p.edges);
                for r in 0..4u8 {
                    let e = e2_core::rotated(base, r);
                    let gain = crate::state::delta_against(&e, &ctx);
                    let key = if jitter { gain * 2 + (rng.next_u64() & 1) as i32 } else { gain * 2 };
                    if best.is_none_or(|(bk, _, _)| key > bk) {
                        best = Some((key, pi, r));
                    }
                }
            }
            let (_, pi, r) = best.expect("pool non-empty while cells remain");
            let pid = remaining.swap_remove(pi);
            st.place(target, pid, r);
        }
    }

    /// Bounded exact refill for a small hole. Enumerates assignments of pool
    /// pieces to cells (in a fixed cell order), each in its best rotation for the
    /// partial context, and keeps the assignment with the best total gain. Bounded
    /// by `exact_max` at the call site; here we run a depth-first best-first over
    /// the (cells × pieces) matching with simple pruning.
    fn exact(self, st: &mut State, cells: &[usize], pool: &[u16], rng: &mut Rng) {
        // For the small sizes this runs at (<= exact_max, single digits), a
        // straightforward branch-and-place over a fixed cell order, trying every
        // remaining piece in its locally-best rotation, is both correct and fast
        // enough. We greedily seed a best, then improve.
        let order = most_constrained_order(st, cells);
        let mut used = vec![false; pool.len()];
        let mut placed: Vec<(usize, u16, u8)> = Vec::with_capacity(cells.len());
        let mut best_plan: Vec<(usize, u16, u8)> = Vec::new();
        let mut best_gain = i32::MIN;

        // Recursion via an explicit helper working on `st` with place/clear so
        // delta scoring stays incremental.
        fn rec(
            st: &mut State,
            order: &[usize],
            pool: &[u16],
            used: &mut [bool],
            placed: &mut Vec<(usize, u16, u8)>,
            gain_so_far: i32,
            best_gain: &mut i32,
            best_plan: &mut Vec<(usize, u16, u8)>,
        ) {
            if placed.len() == order.len() {
                if gain_so_far > *best_gain {
                    *best_gain = gain_so_far;
                    *best_plan = placed.clone();
                }
                return;
            }
            let target = order[placed.len()];
            // The neighbour context at `target` is fixed for this recursion level
            // (it only changes as cells are placed, which happens deeper), so
            // compute it once here rather than inside delta_if_placed per rotation.
            let ctx = st.target_context(target);
            for (pi, &pid) in pool.iter().enumerate() {
                if used[pi] {
                    continue;
                }
                // Best rotation for this piece at this cell in the current context.
                let base = st.pieces.get(pid).map_or([e2_core::BORDER; 4], |p| p.edges);
                let mut br = 0u8;
                let mut bg = i32::MIN;
                for r in 0..4u8 {
                    let g = crate::state::delta_against(&e2_core::rotated(base, r), &ctx);
                    if g > bg {
                        bg = g;
                        br = r;
                    }
                }
                used[pi] = true;
                st.place(target, pid, br);
                placed.push((target, pid, br));
                rec(st, order, pool, used, placed, gain_so_far + bg, best_gain, best_plan);
                st.clear(target);
                placed.pop();
                used[pi] = false;
            }
        }

        rec(st, &order, pool, &mut used, &mut placed, 0, &mut best_gain, &mut best_plan);

        if best_plan.is_empty() {
            // Degenerate (no cells); nothing to do.
            let _ = rng;
            return;
        }
        for (pos, pid, r) in best_plan {
            st.place(pos, pid, r);
        }
    }
}

/// Order the destroyed cells most-constrained first (most placed neighbours),
/// the order both the greedy and the exact refill fill them in.
fn most_constrained_order(st: &State, cells: &[usize]) -> Vec<usize> {
    let mut v = cells.to_vec();
    v.sort_by_key(|&pos| std::cmp::Reverse(placed_neighbours(st, pos)));
    v
}

/// How many of a cell's on-board neighbours are currently filled.
fn placed_neighbours(st: &State, pos: usize) -> u8 {
    use e2_core::{H, W};
    let (y, x) = (pos / W, pos % W);
    let mut n = 0u8;
    if y > 0 && !st.is_empty_at(pos - W) {
        n += 1;
    }
    if x + 1 < W && !st.is_empty_at(pos + 1) {
        n += 1;
    }
    if y + 1 < H && !st.is_empty_at(pos + W) {
        n += 1;
    }
    if x > 0 && !st.is_empty_at(pos - 1) {
        n += 1;
    }
    n
}
