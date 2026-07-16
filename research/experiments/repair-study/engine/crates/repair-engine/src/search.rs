//! The destroy-and-repair loop, and the one entry point [`run`] that executes a
//! variant against an instance under a time budget.
//!
//! One iteration: snapshot the working board, destroy a set of cells (chosen by
//! the variant's operator), refill them (by the variant's repair), then ask the
//! variant's acceptance rule whether to keep the result. On reject, restore the
//! snapshot. The global best is tracked separately and is the run's output. A
//! restart policy watches for a stall and perturbs when one is detected.
//!
//! The score is maintained incrementally by [`State`], so the loop never
//! rescans the board; the canonical scorer is applied once, at the end, to the
//! output board, so the published number is always the site's source of truth.

use std::time::Instant;

use e2_core::{score_cells, Board, N};
use e2_io::Instance;

use crate::spec::Spec;
use crate::state::{Rng, State};
use crate::stats::{RepairStats, CURVE_STRIDE};
use crate::strategy::restart::Restart;

/// A run's configuration: wall-clock budget and RNG seed.
#[derive(Debug, Clone, Copy)]
pub struct RunConfig {
    pub budget_ms: u64,
    pub seed: u64,
}

/// The result of one run: the best board and the statistics behind it.
pub struct RunResult {
    /// Best board as row-major cell codes (`piece*4 + rot`, `-1` empty).
    pub best_codes: Vec<i32>,
    pub best_score: u32,
    pub stats: RepairStats,
    pub elapsed_s: f64,
}

impl RunResult {
    /// Package the best board through the instance's canonical output contract,
    /// re-scoring it with the one true scorer.
    #[must_use]
    pub fn output(&self, inst: &Instance) -> e2_io::SolveOutput {
        inst.finish(&Board::from_cell_codes(&self.best_codes))
    }
}

/// How often (in iterations) the loop checks the wall clock. Checking every
/// iteration would let the clock dominate a cheap greedy iteration; 64 keeps the
/// overhead negligible while staying responsive to a 60 s budget.
const CLOCK_STRIDE: u64 = 64;

/// Run `spec` on `inst` under `cfg`. Deterministic given the seed.
#[must_use]
pub fn run(inst: &Instance, spec: &Spec, cfg: RunConfig) -> RunResult {
    let start_time = Instant::now();
    let budget = f64::from(u32::try_from(cfg.budget_ms).unwrap_or(u32::MAX)) / 1000.0;
    let mut rng = Rng::new(cfg.seed);

    // Which cells are pinned hints — never destroyed, never kicked.
    let mut is_hint = vec![false; N];
    for h in &inst.hints {
        is_hint[h.pos as usize] = true;
    }

    // Build the starting board and the working state from it.
    let start_codes = spec.start.build(inst, &mut rng);
    let mut st = State::from_codes(&inst.pieces, &start_codes);

    let mut stats = RepairStats::new();
    stats.start_score = st.score();

    let mut best_codes = st.to_codes();
    let mut best_score = st.score();
    stats.curve.push(best_score);

    // Late-acceptance history: the working score `len` iterations ago.
    let hist_len = spec.accept.history_len();
    let mut history: Vec<u32> = if hist_len > 0 { vec![st.score(); hist_len] } else { Vec::new() };

    let patience = spec.restart.patience();
    let mut stalled: u64 = 0; // iterations since the last global-best improvement

    let mut elapsed;
    loop {
        // Clock check every CLOCK_STRIDE iterations.
        if stats.iterations % CLOCK_STRIDE == 0 {
            elapsed = start_time.elapsed().as_secs_f64();
            if elapsed >= budget {
                break;
            }
        }
        let progress = (start_time.elapsed().as_secs_f64() / budget).min(1.0);

        let before = st.score();

        // --- destroy: choose cells, snapshot them, lift their pieces ---
        let cells = spec.destroy.select(&st, &is_hint, &mut rng);
        if cells.is_empty() {
            // A conflict-driven operator on a board with no conflicts left: there
            // is nothing to repair. Perturb if we can, else we are done.
            if perturb_or_stop(spec, &mut st, &is_hint, &mut rng, &mut stats) {
                stalled = 0;
                continue;
            }
            break;
        }
        // Snapshot the (cell, piece, rot) we are about to change, to revert.
        let snapshot: Vec<(usize, i32)> = cells
            .iter()
            .map(|&p| (p, cell_code(&st, p)))
            .collect();
        let pool: Vec<u16> = cells.iter().filter_map(|&p| st.clear(p).map(|(pid, _)| pid)).collect();

        // --- repair: refill the hole ---
        spec.repair.refill(&mut st, &cells, &pool, &mut rng);

        let after = st.score();
        stats.iterations += 1;
        stats.destroyed_cells += cells.len() as u64;

        // --- accept or revert ---
        let hist_ref = if hist_len > 0 {
            history[(stats.iterations as usize) % hist_len]
        } else {
            0
        };
        let keep = spec.accept.keep(before, after, progress, hist_ref, &mut rng);
        if keep {
            stats.accepts += 1;
            if after > before {
                stats.improvements += 1;
            }
        } else {
            // Revert: clear the repaired cells, restore the snapshot placements.
            for &p in &cells {
                st.clear(p);
            }
            for &(p, code) in &snapshot {
                if code >= 0 {
                    st.place(p, (code / 4) as u16, (code % 4) as u8);
                }
            }
        }

        let working = st.score();
        if hist_len > 0 {
            history[(stats.iterations as usize) % hist_len] = working;
        }

        // --- track the global best ---
        if working > best_score {
            best_score = working;
            best_codes = st.to_codes();
            stats.best_improvements += 1;
            stats.last_best_iter = stats.iterations;
            stalled = 0;
        } else {
            stalled += 1;
        }

        // Convergence-curve sample.
        if stats.iterations % CURVE_STRIDE == 0 {
            stats.curve.push(best_score);
        }

        // --- restart / perturbation on a stall ---
        if patience > 0 && stalled >= patience {
            apply_restart(spec, &mut st, &best_codes, &is_hint, &mut rng, &mut stats);
            stalled = 0;
        }
    }

    // Final canonical re-score of the output board (never trust the incremental
    // score for the published number).
    let final_cells = Board::from_cell_codes(&best_codes).to_edge_cells(&inst.pieces);
    let canonical = score_cells(&final_cells);
    debug_assert_eq!(canonical, best_score, "incremental score must match canonical");

    RunResult {
        best_codes,
        best_score: canonical,
        stats,
        elapsed_s: start_time.elapsed().as_secs_f64(),
    }
}

/// The current cell code at `pos` (`piece*4 + rot`, `-1` empty).
#[inline]
fn cell_code(st: &State, pos: usize) -> i32 {
    match st.cell(pos) {
        Some((pid, r)) => i32::from(pid) * 4 + i32::from(r),
        None => -1,
    }
}

/// Fire the restart policy on a stall: a kick, or a revert to the incumbent best.
fn apply_restart(
    spec: &Spec,
    st: &mut State,
    best_codes: &[i32],
    is_hint: &[bool],
    rng: &mut Rng,
    stats: &mut RepairStats,
) {
    match spec.restart {
        Restart::None => {}
        Restart::Kick { kick, .. } => {
            let moved = Restart::apply_kick(kick, st, is_hint, rng);
            if moved > 0 {
                stats.restarts += 1;
            }
        }
        Restart::RevertToBest { .. } => {
            *st = State::from_codes(st.pieces, best_codes);
            stats.restarts += 1;
        }
    }
}

/// Used when a conflict-driven destroy has nothing to remove: perturb if the
/// variant has a restart policy, else signal stop. Returns `true` if it perturbed.
fn perturb_or_stop(
    spec: &Spec,
    st: &mut State,
    is_hint: &[bool],
    rng: &mut Rng,
    stats: &mut RepairStats,
) -> bool {
    match spec.restart {
        Restart::Kick { kick, .. } => {
            let moved = Restart::apply_kick(kick, st, is_hint, rng);
            if moved > 0 {
                stats.restarts += 1;
                return true;
            }
            false
        }
        // No perturbation available: a zero-conflict board is a solved board, so
        // stopping is correct.
        _ => false,
    }
}
