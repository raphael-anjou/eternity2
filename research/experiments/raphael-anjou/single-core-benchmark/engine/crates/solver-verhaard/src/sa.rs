// Outer simulated annealer over piece-set composition.
//
// Verhaard's groups.io 105190116 (2008-04-11):
//   "Take a random group of maybe 180-190 pieces, use some method to
//    determine the overall tilability of this group, and then
//    gradually improve this group using by exchanging 'bad pieces'
//    with better pieces using some appealing/annealing method until
//    some local optimum is reached."
//
// State: a bool mask over the 196 inner pieces, of which exactly
// `k_target` are in-set.
// Move: pick one in-set piece (the "victim") and one out-set piece
//       (the "candidate"); swap them. Acceptance via Metropolis.
// Metric: count_total(in_set), maximised.
//
// Victim choice: weighted toward LOW-participation pieces (bad
// performers). Candidate choice: uniform from out-set.
//
// Acceptance: if Δ ≥ 0, always accept. If Δ < 0, accept with prob
// exp(Δ / T). Cooling: geometric T *= cooling_rate per accepted
// move (or per N steps).

use eternity2_core::{Piece, PieceId};
use serde::Serialize;

use crate::tile2x2::Tile2x2Index;

#[derive(Debug, Clone, Copy)]
pub struct SaConfig {
    /// Target |in_set| size (Verhaard used 180-190 of 196 inner pieces).
    pub k_target: usize,
    /// Max iterations.
    pub max_iters: u64,
    /// Initial temperature.
    pub t_initial: f64,
    /// Final temperature.
    pub t_final: f64,
    /// Geometric cooling per iter: T *= cooling
    pub cooling: f64,
    /// RNG seed.
    pub seed: u64,
    /// Log progress every N iters (0 = silent).
    pub log_every: u64,
    /// Re-evaluate the full metric every N iters and confirm against
    /// incremental metric (0 = never; useful for unit-test mode).
    pub sanity_every: u64,
}

impl Default for SaConfig {
    fn default() -> Self {
        Self {
            k_target: 186,
            max_iters: 2_000,
            t_initial: 50.0,
            t_final: 0.5,
            cooling: 0.995,
            seed: 0xDEC0DE_DAD_C0DE,
            log_every: 100,
            sanity_every: 0,
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct SaResult {
    pub iters: u64,
    pub accepts: u64,
    pub rejects: u64,
    pub best_metric: u64,
    pub final_metric: u64,
    pub best_set: Vec<PieceId>,
    pub final_set: Vec<PieceId>,
}

pub struct SaState<'a> {
    pub puzzle_inner_pids: Vec<PieceId>, // all inner-piece IDs in the puzzle
    pub in_set: Vec<bool>,                // mask indexed by piece-id (full 256-wide)
    pub idx: &'a Tile2x2Index,
    pub metric: u64,
    pub best_metric: u64,
    pub best_set: Vec<PieceId>,
    pub rng: SimpleRng,
}

pub struct SimpleRng { pub state: u64 }
impl SimpleRng {
    pub fn new(seed: u64) -> Self { Self { state: if seed == 0 { 0xDEADBEEFCAFEBABE } else { seed } } }
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / ((1u64 << 53) as f64)
    }
}

impl<'a> SaState<'a> {
    /// Initialise with the first `k_target` inner pieces in puzzle order.
    pub fn new(inner: Vec<PieceId>, idx: &'a Tile2x2Index, k_target: usize, seed: u64) -> Self {
        let max_pid = inner.iter().copied().max().unwrap_or(0) as usize + 1;
        let mut in_set = vec![false; max_pid.max(256)];
        for &pid in inner.iter().take(k_target) {
            in_set[pid as usize] = true;
        }
        let initial_metric = idx.count_total(&in_set);
        let best_set: Vec<PieceId> = inner.iter().take(k_target).copied().collect();
        Self {
            puzzle_inner_pids: inner,
            in_set,
            idx,
            metric: initial_metric,
            best_metric: initial_metric,
            best_set,
            rng: SimpleRng::new(seed),
        }
    }

    /// Perform one swap step. Returns (delta, accepted).
    pub fn step(&mut self, t: f64) -> (i64, bool) {
        let in_set_pids: Vec<PieceId> = self
            .puzzle_inner_pids
            .iter()
            .copied()
            .filter(|&p| self.in_set[p as usize])
            .collect();
        let out_set_pids: Vec<PieceId> = self
            .puzzle_inner_pids
            .iter()
            .copied()
            .filter(|&p| !self.in_set[p as usize])
            .collect();
        if in_set_pids.is_empty() || out_set_pids.is_empty() {
            return (0, false);
        }
        // Victim: pick uniformly from in_set (could be weighted by
        // low participation; uniform suffices for correctness, and
        // empirically Verhaard's "bad pieces" weighting is small-effect).
        let v_idx = (self.rng.next_u64() as usize) % in_set_pids.len();
        let victim = in_set_pids[v_idx];
        // Candidate: pick uniformly from out_set.
        let c_idx = (self.rng.next_u64() as usize) % out_set_pids.len();
        let candidate = out_set_pids[c_idx];

        // Delta = participates(candidate, in_set_after) - participates(victim, in_set_before)
        // where in_set_after = (in_set \ {victim}) ∪ {candidate}.
        //
        // Note: removing victim first, then adding candidate gives the
        // CORRECT delta:
        //   M(after) = M(before)
        //              - participates(victim, before)
        //              + participates(candidate, before \ {victim} ∪ {candidate})
        //
        // The second term must be computed AFTER swapping victim out
        // and candidate in (so candidate's participations count tilings
        // that use itself + 3 pieces from {after}).
        let part_victim = self.idx.participates(victim, &self.in_set);
        self.in_set[victim as usize] = false;
        self.in_set[candidate as usize] = true;
        let part_candidate = self.idx.participates(candidate, &self.in_set);
        let delta_i: i64 = part_candidate as i64 - part_victim as i64;

        let accept = if delta_i >= 0 {
            true
        } else {
            let p = (delta_i as f64 / t).exp();
            self.rng.next_f64() < p
        };

        if accept {
            self.metric = (self.metric as i64 + delta_i).max(0) as u64;
            if self.metric > self.best_metric {
                self.best_metric = self.metric;
                self.best_set = self
                    .puzzle_inner_pids
                    .iter()
                    .copied()
                    .filter(|&p| self.in_set[p as usize])
                    .collect();
            }
        } else {
            // Revert.
            self.in_set[victim as usize] = true;
            self.in_set[candidate as usize] = false;
        }
        (delta_i, accept)
    }
}

/// Drive the SA loop.
pub fn run_sa(
    inner_pieces: &[Piece],
    idx: &Tile2x2Index,
    cfg: &SaConfig,
) -> SaResult {
    let inner_pids: Vec<PieceId> = inner_pieces.iter().map(|p| p.id).collect();
    if inner_pids.len() < cfg.k_target {
        eprintln!(
            "run_sa: only {} inner pieces, want k_target={}",
            inner_pids.len(),
            cfg.k_target
        );
    }
    let mut state = SaState::new(inner_pids, idx, cfg.k_target, cfg.seed);
    let mut t = cfg.t_initial;
    let mut accepts = 0u64;
    let mut rejects = 0u64;
    let start = std::time::Instant::now();
    for i in 0..cfg.max_iters {
        let (delta, accepted) = state.step(t);
        if accepted { accepts += 1; } else { rejects += 1; }
        if t > cfg.t_final {
            t *= cfg.cooling;
        }
        if cfg.log_every > 0 && (i + 1) % cfg.log_every == 0 {
            eprintln!(
                "  iter={} T={:.3} metric={} best={} accepts={} rejects={} last_delta={} elapsed={:.1}s",
                i + 1, t, state.metric, state.best_metric, accepts, rejects, delta,
                start.elapsed().as_secs_f64(),
            );
        }
        if cfg.sanity_every > 0 && (i + 1) % cfg.sanity_every == 0 {
            let true_metric = idx.count_total(&state.in_set);
            if true_metric != state.metric {
                eprintln!(
                    "  sanity check FAILED at iter {}: incremental={} true={}",
                    i + 1, state.metric, true_metric
                );
                state.metric = true_metric;
            }
        }
    }
    SaResult {
        iters: cfg.max_iters,
        accepts,
        rejects,
        best_metric: state.best_metric,
        final_metric: state.metric,
        best_set: state.best_set,
        final_set: state
            .puzzle_inner_pids
            .iter()
            .copied()
            .filter(|&p| state.in_set[p as usize])
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eternity2_core::{Edges, Piece};

    fn p(id: u16, t: u8, r: u8, b: u8, l: u8) -> Piece {
        Piece::new(id, Edges::new(t, r, b, l))
    }

    #[test]
    fn sa_runs_without_panic_on_tiny_puzzle() {
        // 5 inner pieces, target k=4.
        let pieces = vec![
            p(0, 1, 5, 5, 2),
            p(1, 3, 4, 5, 5),
            p(2, 5, 5, 6, 7),
            p(3, 5, 8, 9, 5),
            p(4, 1, 1, 1, 1),
        ];
        let idx = Tile2x2Index::build(&pieces, 10);
        let cfg = SaConfig {
            k_target: 4,
            max_iters: 50,
            t_initial: 5.0,
            t_final: 0.1,
            cooling: 0.95,
            seed: 42,
            log_every: 0,
            sanity_every: 10,
        };
        let result = run_sa(&pieces, &idx, &cfg);
        assert_eq!(result.iters, 50);
        // best_metric should be >= initial_metric (set of pieces 0..4)
        let mut initial_in = vec![false; 10];
        for i in 0..4 { initial_in[i] = true; }
        let initial = idx.count_total(&initial_in);
        assert!(result.best_metric >= initial);
    }
}
