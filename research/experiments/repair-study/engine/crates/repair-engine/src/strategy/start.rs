//! The starting board a repair loop begins from. A destroy-and-repair loop
//! polishes a board it is *given*, so where it starts is one of the study's
//! axes: the same loop reaches a very different place from random noise than
//! from a greedy construction, and the vault's record boards all start from a
//! strong construction, not from noise.
//!
//! Every start honours the instance's pinned hints (the five official clues and
//! any corner pins): those cells are placed first and never handed to the
//! destroy operator, so a repair run on a corner-pinned variant keeps the pins.

use e2_io::Instance;

use crate::state::Rng;

/// How the initial full board is built before the first destroy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartBoard {
    /// Every non-hint cell gets a random unused piece at a random rotation. The
    /// worst possible start; the control that prices what construction is worth.
    Random,
    /// A greedy constructive pass in row-major order: at each empty cell, place
    /// the unused (piece, rotation) that matches the most already-placed
    /// neighbours. This is the cheap constructor the records refine.
    GreedyConstruct,
    /// The greedy construction, but candidate pieces are considered rarest-color
    /// first — the Selby/Riordan heuristic that a scarce color should be spent
    /// where it is forced. A one-change delta over [`Self::GreedyConstruct`].
    GreedyRareFirst,
    /// Start from a *strong backtracked board*: run the DFS study's break-DFS
    /// (`break-1`) for `dfs_budget_ms`, then hand its best board to the repair
    /// loop. This is the construct-then-refine pipeline the records use, and it
    /// answers the question the greedy starts cannot: does repair add anything on
    /// top of a board that is already near the backtracking wall? The DFS time is
    /// spent from the *same* run budget, so a 60 s run with a 20 s DFS seed leaves
    /// the loop 40 s to repair. A break-DFS board is only partially filled (it
    /// stops at the depth wall), so the empty tail is greedily completed before
    /// the loop begins, keeping the start a full board like every other.
    FromDfsBoard { dfs_budget_ms: u64 },
}

impl StartBoard {
    #[must_use]
    pub const fn tag(self) -> &'static str {
        match self {
            Self::Random => "random",
            Self::GreedyConstruct => "greedy",
            Self::GreedyRareFirst => "greedy-rare",
            Self::FromDfsBoard { .. } => "from-dfs",
        }
    }

    /// Build the starting board's cell codes for `inst`, using `rng` for random
    /// choices and greedy tie-breaks, `seed` for the deterministic DFS seed, and
    /// `run_budget_ms` to cap the DFS seed time to at most half the run (so a
    /// short run never starves the repair loop). Hints are always pre-placed.
    #[must_use]
    pub fn build(self, inst: &Instance, rng: &mut Rng, seed: u64, run_budget_ms: u64) -> Vec<i32> {
        if let Self::FromDfsBoard { dfs_budget_ms } = self {
            // Never let the seed eat more than half the run.
            let dfs_ms = dfs_budget_ms.min(run_budget_ms / 2).max(1);
            return self.build_from_dfs(inst, dfs_ms, seed, rng);
        }

        let n = (inst.width as usize) * (inst.height as usize);
        let mut codes = vec![-1i32; n];
        let mut used = vec![false; inst.pieces.len()];

        // Pinned hints first: placed, marked used, and (by staying filled) never
        // selected by construction below.
        for h in &inst.hints {
            codes[h.pos as usize] = i32::from(h.piece) * 4 + i32::from(h.rot);
            used[h.piece as usize] = true;
        }

        match self {
            Self::Random => self.fill_random(inst, &mut codes, &mut used, rng),
            Self::GreedyConstruct | Self::GreedyRareFirst => {
                self.fill_greedy(inst, &mut codes, &mut used, rng);
            }
            Self::FromDfsBoard { .. } => unreachable!("handled above"),
        }
        codes
    }

    /// Run break-DFS for `dfs_budget_ms`, take its best board, and greedily fill
    /// any cells it left empty at the depth wall so the result is a full board.
    fn build_from_dfs(self, inst: &Instance, dfs_budget_ms: u64, seed: u64, rng: &mut Rng) -> Vec<i32> {
        let spec = dfs_engine::find("break-1").expect("break-1 is a registered DFS variant");
        let result = dfs_engine::run(inst, &spec, dfs_engine::RunConfig::timed(dfs_budget_ms, seed));
        let mut codes = result.best.to_cell_codes();

        // The break-DFS board stops at the depth wall, so a tail of cells is
        // still empty. Complete it greedily (respecting used pieces) so the repair
        // loop begins from a full board, exactly like the other starts.
        let mut used = vec![false; inst.pieces.len()];
        for &c in &codes {
            if c >= 0 {
                used[(c / 4) as usize] = true;
            }
        }
        // Hints are already on the DFS board (it pins them), so no separate pass.
        self.fill_greedy(inst, &mut codes, &mut used, rng);
        codes
    }

    fn fill_random(self, inst: &Instance, codes: &mut [i32], used: &mut [bool], rng: &mut Rng) {
        let mut pool: Vec<u16> = (0..inst.pieces.len() as u16).filter(|&p| !used[p as usize]).collect();
        rng.shuffle(&mut pool);
        let mut next = 0usize;
        for code in codes.iter_mut() {
            if *code >= 0 {
                continue;
            }
            let pid = pool[next];
            next += 1;
            let rot = (rng.next_u64() % 4) as i32;
            *code = i32::from(pid) * 4 + rot;
            used[pid as usize] = true;
        }
    }

    fn fill_greedy(self, inst: &Instance, codes: &mut [i32], used: &mut [bool], rng: &mut Rng) {
        use e2_core::{rotated, BORDER, W};
        let n = codes.len();
        let rarity = color_rarity(inst);
        // Order pieces once by rarity if this variant asks for it; the greedy
        // scan then breaks score ties by this order rather than by id.
        let mut order: Vec<u16> = (0..inst.pieces.len() as u16).collect();
        if self == Self::GreedyRareFirst {
            order.sort_by(|&a, &b| {
                piece_rarity(inst, &rarity, b).cmp(&piece_rarity(inst, &rarity, a))
            });
        }

        for pos in 0..n {
            if codes[pos] >= 0 {
                continue;
            }
            // Resolve already-placed neighbour edges for scoring.
            let (y, x) = (pos / W, pos % W);
            let neigh = |np: Option<usize>, opp: usize| -> Option<u8> {
                np.filter(|&p| codes[p] >= 0).map(|p| {
                    let (pid, r) = ((codes[p] / 4) as u16, (codes[p] % 4) as u8);
                    rotated(inst.pieces.get(pid).map_or([BORDER; 4], |q| q.edges), r)[opp]
                })
            };
            let up = neigh(if y > 0 { Some(pos - W) } else { None }, 2);
            let left = neigh(if x > 0 { Some(pos - 1) } else { None }, 1);

            let mut best: Option<(i32, u16, u8)> = None;
            for &pid in &order {
                if used[pid as usize] {
                    continue;
                }
                let edges = inst.pieces.get(pid).map_or([BORDER; 4], |q| q.edges);
                for r in 0..4u8 {
                    let e = rotated(edges, r);
                    let mut gain = 0i32;
                    if let Some(u) = up {
                        if e[0] != BORDER && e[0] == u {
                            gain += 1;
                        }
                    }
                    if let Some(l) = left {
                        if e[3] != BORDER && e[3] == l {
                            gain += 1;
                        }
                    }
                    // A tiny random jitter breaks exact ties so a run is not a
                    // single deterministic construction, matching the vault's
                    // seeded tie-break discipline.
                    let jitter = (rng.next_u64() & 1) as i32;
                    let key = gain * 2 + jitter;
                    if best.is_none_or(|(bk, _, _)| key > bk) {
                        best = Some((key, pid, r));
                    }
                }
            }
            if let Some((_, pid, r)) = best {
                codes[pos] = i32::from(pid) * 4 + i32::from(r);
                used[pid as usize] = true;
            }
        }
    }
}

/// Per-color scarcity: how many piece-edges carry each color across the set. A
/// smaller count is rarer.
fn color_rarity(inst: &Instance) -> Vec<u32> {
    let mut count = vec![0u32; 256];
    for (_, p) in inst.pieces.iter() {
        for &c in &p.edges {
            count[c as usize] += 1;
        }
    }
    count
}

/// A piece's rarity weight: the rarer its colors, the higher. Border color is
/// ignored (it is not scarce in the interesting sense).
fn piece_rarity(inst: &Instance, rarity: &[u32], pid: u16) -> u32 {
    use e2_core::BORDER;
    let edges = inst.pieces.get(pid).map_or([BORDER; 4], |q| q.edges);
    edges
        .iter()
        .filter(|&&c| c != BORDER)
        .map(|&c| 1000u32.saturating_sub(rarity[c as usize].min(1000)))
        .sum()
}
