//! Entrant adapters. Each calls a repo engine unchanged, in its committed
//! configuration, charges its work to one `Work` counter at one place, and
//! returns its best complete board in the main engine's encoding. The board is
//! verified by the caller (`verify::verify`), never trusted.
//!
//! | name     | engine                                           | unit       |
//! |----------|--------------------------------------------------|------------|
//! | break-r1 | `eternity2_engine::BreakSolver`, snake, bottom 1 row licensed  | placement |
//! | break-r2 | same, bottom 2 rows licensed                     | placement  |
//! | sa       | `eternity2_localsearch::run_sa`, `SaConfig::default()` | move_eval |
//! | pt       | `run_pt_from` default path (8 replicas, 0.05..2.0, 100k-step bursts, greedy fill), sequential twin | move_eval |
//! | alns     | `run_alns`, `alns_only` defaults (winning5 ops, SA acceptance t = 1, SA repair) with a fixed repair step budget | move_eval |

use eternity2_core::{Board, Edges, Piece, Puzzle as LsPuzzle};
use eternity2_engine::{build_path, rotated, BreakSolver, Puzzle, Status};
use eternity2_localsearch::{
    run_alns, run_sa, run_sa_steps_fixed_temp, Acceptance, AlnsConfig, ConflictDriven, DestroyOp,
    MwpmDefectPair, RandomRegion, RepairKind, RngHandle, SaConfig, StateRef, WorstBand,
    WorstWindow,
};
use serde_json::{json, Value};

use crate::verify::verify;
use crate::work::{Unit, Work};

pub const ENTRANTS: [&str; 5] = ["break-r1", "break-r2", "sa", "pt", "alns"];

/// PT: the repo defaults (`PtConfig::default()`).
pub const PT_REPLICAS: usize = 8;
pub const PT_T_MIN: f64 = 0.05;
pub const PT_T_MAX: f64 = 2.0;
pub const PT_BURST: u64 = 100_000;

/// ALNS repair step budget (SA moves per repair). `alns_only` has a
/// deterministic `--repair-step-budget` mode with no committed value; this
/// is fixed to the PT burst length (100,000) before any run, not tuned.
pub const ALNS_REPAIR_STEPS: u64 = 100_000;

/// SA trajectory grid: budgets `W >> j` down to at least this many moves.
pub const SA_GRID_MIN: u64 = 1 << 20;

#[derive(Debug, Clone)]
pub struct Outcome {
    pub work: Work,
    /// Best complete board (engine encoding), if any.
    pub board: Option<Vec<i32>>,
    /// (work, score) at each improvement of the best complete board.
    pub trajectory: Vec<(u64, u32)>,
    /// Whether every trajectory point's board was verified (else only the
    /// final board is verified and earlier points are engine-reported).
    pub trajectory_verified: bool,
    pub extra: Value,
}

#[must_use]
pub fn unit_of(entrant: &str) -> Unit {
    if entrant.starts_with("break-") {
        Unit::Placement
    } else {
        Unit::MoveEval
    }
}

pub fn run(entrant: &str, puzzle: &Puzzle, budget: u64, run_seed: u64) -> Result<Outcome, String> {
    match entrant {
        "break-r1" => run_break(puzzle, 1, budget),
        "break-r2" => run_break(puzzle, 2, budget),
        "sa" => run_sa_grid(puzzle, budget, run_seed),
        "pt" => run_pt(puzzle, budget, run_seed),
        "alns" => run_alns_entrant(puzzle, budget, run_seed),
        e => Err(format!("unknown entrant {e}")),
    }
}

// ------------------------------------------------------------ constructive

fn run_break(puzzle: &Puzzle, rows: usize, budget: u64) -> Result<Outcome, String> {
    let (w, h) = (puzzle.width as usize, puzzle.height as usize);
    let path = build_path("snake", puzzle.width, puzzle.height, 0).ok_or("snake path")?;
    let licensed: Vec<u16> = (h.saturating_sub(rows) * w..h * w)
        .map(|c| c as u16)
        .collect();
    let mut s = BreakSolver::new(puzzle, &path, false, &licensed)?;
    let mut rep = s.report();
    // Each step iteration places at most one piece, so asking for at most the
    // remaining placements never overshoots the budget.
    while rep.status == Status::Running && (rep.nodes as u64) < budget {
        let left = budget - rep.nodes as u64;
        rep = s.step(left.min(1 << 24) as u32);
    }
    let mut work = Work::new(Unit::Placement, budget);
    work.charge(rep.nodes as u64);
    let solved = rep.status == Status::Solved;
    let best = s.best_board().to_vec();
    let partial = crate::verify::partial_score(puzzle, &best);
    let (board, trajectory) = if solved {
        let sc = verify(puzzle, &best)?;
        (Some(best), vec![(work.used, sc)])
    } else {
        (None, vec![])
    };
    Ok(Outcome {
        work,
        board,
        trajectory,
        trajectory_verified: true,
        extra: json!({
            "status": format!("{:?}", rep.status),
            "attempts": rep.attempts, "backtracks": rep.backtracks,
            "best_placed": rep.best_placed, "best_partial_score": partial,
            "breaks_used": s.breaks_used(), "licensed_rows": rows,
        }),
    })
}

// ------------------------------------------------------------ local search

#[must_use]
pub fn ls_puzzle(p: &Puzzle) -> LsPuzzle {
    let pieces: Vec<Piece> = p
        .pieces
        .iter()
        .enumerate()
        .map(|(i, e)| Piece::new(i as u16, Edges::new(e[0], e[1], e[2], e[3])))
        .collect();
    LsPuzzle::new(
        u32::from(p.width),
        u32::from(p.height),
        u32::from(p.num_colors) + 1,
        pieces,
    )
    .expect("valid puzzle")
}

/// Local-search board to the main engine's encoding: the rotation is chosen by
/// matching the oriented edges, so a convention difference cannot leak in.
pub fn to_engine(p: &Puzzle, lp: &LsPuzzle, b: &Board) -> Result<Vec<i32>, String> {
    b.cells()
        .iter()
        .enumerate()
        .map(|(pos, c)| {
            let Some((pid, rot)) = c else { return Ok(-1) };
            let e = lp
                .piece(*pid)
                .ok_or("piece id")?
                .edges
                .rotated(*rot)
                .as_array();
            let base = p.pieces.get(*pid as usize).ok_or("piece id")?;
            (0..4u8)
                .find(|&r| rotated(*base, r) == e)
                .map(|r| i32::from(*pid) * 4 + i32::from(r))
                .ok_or_else(|| format!("no engine rotation for cell {pos}"))
        })
        .collect()
}

fn run_sa_grid(puzzle: &Puzzle, budget: u64, seed: u64) -> Result<Outcome, String> {
    let lp = ls_puzzle(puzzle);
    let mut grid = vec![budget];
    while *grid.last().unwrap() / 2 >= SA_GRID_MIN {
        grid.push(grid.last().unwrap() / 2);
    }
    grid.reverse();
    let mut trajectory: Vec<(u64, u32)> = Vec::new();
    let mut total = 0u64;
    let mut last = None;
    for &g in &grid {
        let cfg = SaConfig {
            max_iters: g,
            seed,
            ..SaConfig::default()
        };
        let out = run_sa(&lp, &cfg);
        total += out.iterations;
        let board = to_engine(puzzle, &lp, &out.best_board)?;
        let sc = verify(puzzle, &board)?;
        if sc != out.best_score {
            return Err(format!(
                "sa self-report {} != verified {sc}",
                out.best_score
            ));
        }
        // The run with max_iters = g is the prefix of every longer run (same
        // seed, no clock), so its best is the best-so-far at work g. A run that
        // reached the optimum stopped at out.iterations < g.
        if trajectory.last().is_none_or(|&(_, s)| sc > s) {
            trajectory.push((out.iterations.min(g), sc));
        }
        last = Some((out, board));
        if sc == puzzle_opt(puzzle) {
            break;
        }
    }
    let (out, board) = last.unwrap();
    let mut work = Work::new(Unit::MoveEval, budget);
    work.charge(out.iterations);
    Ok(Outcome {
        work,
        board: Some(board),
        trajectory,
        trajectory_verified: true,
        extra: json!({ "grid": grid, "compute_total_move_evals": total }),
    })
}

fn puzzle_opt(p: &Puzzle) -> u32 {
    let (w, h) = (u32::from(p.width), u32::from(p.height));
    (w - 1) * h + w * (h - 1)
}

#[must_use]
pub fn pt_ladder() -> Vec<f64> {
    let ratio = (PT_T_MAX / PT_T_MIN).powf(1.0 / (PT_REPLICAS as f64 - 1.0));
    (0..PT_REPLICAS)
        .map(|i| PT_T_MIN * ratio.powi(i as i32))
        .collect()
}

/// `run_pt_from` default path (greedy fill, no Houdayer, repair, kick or
/// forbidden-edge penalty), run sequentially with the same RNG streams, so it
/// returns the same best board (test `pt_twin_matches_run_pt_from`). Stops at
/// the budget or at a perfect board, as `run_pt_from` does.
pub fn pt_core(
    lp: &LsPuzzle,
    seed: u64,
    burst: u64,
    budget: u64,
) -> (Board, u32, u64, Vec<(u64, u32, Board)>) {
    let state = StateRef::new(lp);
    let total = state.total_interior_edges();
    let temps = pt_ladder();
    let n = PT_REPLICAS;
    let empty = Board::empty(lp);
    let mut boards = Vec::with_capacity(n);
    let mut scores = Vec::with_capacity(n);
    let mut rngs = Vec::with_capacity(n);
    for r in 0..n {
        let mut rng =
            RngHandle::new(seed.wrapping_add((r as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)));
        let b = state.greedy_fill_initial(&empty, &mut rng);
        scores.push(state.score(&b));
        boards.push(b);
        rngs.push(rng);
    }
    let mut chain_best_scores = scores.clone();
    let mut chain_best_boards = boards.clone();
    let mut gbest = *chain_best_scores.iter().max().unwrap();
    let gi = chain_best_scores.iter().position(|s| *s == gbest).unwrap();
    let mut gboard = chain_best_boards[gi].clone();
    let mut improvements = vec![(0u64, gbest, gboard.clone())];
    let mut swap_rng = RngHandle::new(seed ^ 0xDEAD_BEEF_CAFE_BABE);
    let (mut done, mut rounds) = (0u64, 0u64);
    'outer: while done + burst * n as u64 <= budget {
        for i in 0..n {
            let mut bb = boards[i].clone();
            let mut bs = scores[i];
            run_sa_steps_fixed_temp(
                &state,
                &mut boards[i],
                &mut scores[i],
                &mut bb,
                &mut bs,
                &mut rngs[i],
                temps[i],
                burst,
            );
            done += burst;
            if bs > improvements.last().unwrap().1 {
                improvements.push((done, bs, bb.clone()));
            }
            if bs > chain_best_scores[i] {
                chain_best_scores[i] = bs;
                chain_best_boards[i] = bb;
            }
        }
        for i in 0..n {
            if chain_best_scores[i] > gbest {
                gbest = chain_best_scores[i];
                gboard = chain_best_boards[i].clone();
            }
        }
        if gbest == total {
            break 'outer;
        }
        let mut i = (rounds & 1) as usize;
        while i + 1 < n {
            let (blo, bhi) = (1.0 / temps[i], 1.0 / temps[i + 1]);
            let log_p = (f64::from(scores[i]) - f64::from(scores[i + 1])) * (bhi - blo);
            if log_p >= 0.0 || swap_rng.next_f64() < log_p.exp() {
                boards.swap(i, i + 1);
                scores.swap(i, i + 1);
                chain_best_boards.swap(i, i + 1);
                chain_best_scores.swap(i, i + 1);
            }
            i += 2;
        }
        rounds += 1;
    }
    (gboard, gbest, done, improvements)
}

fn run_pt(puzzle: &Puzzle, budget: u64, seed: u64) -> Result<Outcome, String> {
    let lp = ls_puzzle(puzzle);
    let (board, best, done, imps) = pt_core(&lp, seed, PT_BURST, budget);
    let mut trajectory = Vec::with_capacity(imps.len());
    for (w, s, b) in &imps {
        let eb = to_engine(puzzle, &lp, b)?;
        let v = verify(puzzle, &eb)?;
        if v != *s {
            return Err(format!("pt self-report {s} != verified {v} at work {w}"));
        }
        trajectory.push((*w, v));
    }
    let eb = to_engine(puzzle, &lp, &board)?;
    let v = verify(puzzle, &eb)?;
    if v != best || trajectory.last().map(|t| t.1) != Some(best) {
        return Err(format!("pt final {best} != verified {v} or trajectory end"));
    }
    let mut work = Work::new(Unit::MoveEval, budget);
    work.charge(done);
    Ok(Outcome {
        work,
        board: Some(eb),
        trajectory,
        trajectory_verified: true,
        extra: json!({ "burst": PT_BURST }),
    })
}

#[must_use]
pub fn winning5() -> Vec<Box<dyn DestroyOp>> {
    vec![
        Box::new(RandomRegion { k: 4 }),
        Box::new(WorstWindow { k: 5 }),
        Box::new(ConflictDriven { max_size: 30 }),
        Box::new(ConflictDriven { max_size: 80 }),
        Box::new(MwpmDefectPair { max_pairs: 12 }),
        Box::new(WorstBand { k_rows: 4 }),
    ]
}

fn run_alns_entrant(puzzle: &Puzzle, budget: u64, seed: u64) -> Result<Outcome, String> {
    let lp = ls_puzzle(puzzle);
    let state = StateRef::new(&lp);
    let mut rng = RngHandle::new(seed);
    let init = state.greedy_fill_initial(&Board::empty(&lp), &mut rng);
    let iters = u32::try_from(budget / ALNS_REPAIR_STEPS).map_err(|_| "budget too large")?;
    let cfg = AlnsConfig {
        time_budget_ms: u64::MAX,
        repair_budget_ms: 1500,
        acceptance: Acceptance::SimulatedAnnealing { t: 1.0 },
        segment_iters: 50,
        seed,
        verbose: false,
        repair: RepairKind::Sa,
        cp_fallback_to_sa: true,
        pinned_positions: Vec::new(),
        iter_budget: iters,
        lex_break_isoscore: false,
        lex_break_intaglio: false,
        checkpoint_path: None,
        checkpoint_every_ms: 60_000,
        repair_step_budget: ALNS_REPAIR_STEPS,
        cp_repair_parallel: false,
    };
    let mut ops = winning5();
    let (best, stats) = run_alns(&lp, &init, &mut ops, &cfg);
    let eb = to_engine(puzzle, &lp, &best)?;
    let v = verify(puzzle, &eb)?;
    let reported = stats.best_score_history.last().map(|t| t.1);
    if reported != Some(v) {
        return Err(format!("alns history end {reported:?} != verified {v}"));
    }
    let mut work = Work::new(Unit::MoveEval, budget);
    work.charge(u64::from(stats.iters) * ALNS_REPAIR_STEPS);
    let trajectory = stats
        .best_score_history
        .iter()
        .map(|&(it, s)| (u64::from(it) * ALNS_REPAIR_STEPS, s))
        .collect();
    Ok(Outcome {
        work,
        board: Some(eb),
        trajectory,
        trajectory_verified: false,
        extra: json!({
            "iters": stats.iters, "repair_steps": ALNS_REPAIR_STEPS,
            "accepted_improving": stats.accepted_improving, "accepted_worse": stats.accepted_worse,
            "rejected": stats.rejected, "repair_failures": stats.repair_failures,
        }),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use eternity2_localsearch::{run_pt_from, PtConfig};

    #[test]
    fn pt_twin_matches_run_pt_from() {
        for (n, c, seed) in [(8u8, 10u8, 7001u32), (10, 12, 7002)] {
            let inst = crate::instance::make("calibration", n, c, seed);
            let lp = ls_puzzle(&inst.puzzle);
            let rounds = 4u64;
            let burst = 5_000u64;
            let cfg = PtConfig {
                inner_iters: burst,
                max_rounds: rounds,
                seed: 42,
                ..PtConfig::default()
            };
            let (repo, _) = run_pt_from(&lp, &Board::empty(&lp), &cfg);
            let (b, s, done, _) = pt_core(&lp, 42, burst, rounds * burst * PT_REPLICAS as u64);
            assert!(repo.best_score < repo.total_edges);
            assert_eq!(s, repo.best_score);
            assert_eq!(b.cells(), repo.best_board.cells());
            assert_eq!(done, rounds * burst * PT_REPLICAS as u64);
        }
    }

    #[test]
    fn conversion_preserves_score() {
        let inst = crate::instance::make("calibration", 8, 10, 7003);
        let lp = ls_puzzle(&inst.puzzle);
        let state = StateRef::new(&lp);
        let mut rng = RngHandle::new(3);
        let b = state.greedy_fill_initial(&Board::empty(&lp), &mut rng);
        let eb = to_engine(&inst.puzzle, &lp, &b).unwrap();
        assert_eq!(verify(&inst.puzzle, &eb).unwrap(), state.score(&b));
    }

    #[test]
    fn verifier_rejects_reused_and_missing() {
        let inst = crate::instance::make("calibration", 8, 10, 7001);
        assert_eq!(verify(&inst.puzzle, &inst.planted).unwrap(), 112);
        let mut b = inst.planted.clone();
        b[9] = b[10];
        assert!(verify(&inst.puzzle, &b).unwrap_err().contains("reused"));
        let mut b = inst.planted.clone();
        b[9] = -1;
        assert!(verify(&inst.puzzle, &b).unwrap_err().contains("missing"));
        let mut b = inst.planted.clone();
        b[0] = (b[0] / 4) * 4 + (b[0] + 1) % 4;
        assert!(verify(&inst.puzzle, &b).unwrap_err().contains("rim"));
    }

    #[test]
    fn entrants_run_and_verify_small() {
        let inst = crate::instance::make("calibration", 6, 6, 7001);
        for e in ENTRANTS {
            let out = run(e, &inst.puzzle, 4_000_000, 1).unwrap();
            assert!(out.work.used <= out.work.budget, "{e}");
            if let Some(b) = &out.board {
                let s = verify(&inst.puzzle, b).unwrap();
                assert_eq!(Some(s), out.trajectory.last().map(|t| t.1), "{e}");
            }
        }
    }
}
