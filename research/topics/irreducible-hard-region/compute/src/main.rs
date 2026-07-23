//! hard-region localization measurement.
//!
//!   cargo run --release -- --n 16 --gen-seeds 1..40 --budget-secs 12 > ../results/hard_region.json
//!
//! Reproduces the concrete, measurable half of the irreducible-hard-region
//! conjecture (vault concept `irreducible-hard-region-conjecture`, vol-208):
//! that a *sequential spatial decomposition* of a solvable Eternity-II board
//! fills the top freely and concentrates the residual difficulty into a hard
//! region at the LAST part of the sweep. The conjecture's "exceeds the exact
//! window" and "too coupled for heuristics" halves are algorithm-specific
//! claims we do NOT prove here; this binary measures only the localization
//! (evidence-chain points 1 and 2 of the concept page).
//!
//! Instrument: a restarting row-major exact-match DFS run FROM SCRATCH (no
//! pinned hints) on framed, colour-balanced, planted-solvable boards from the
//! kit generator. Each board is guaranteed to admit a perfect solution, so any
//! stall is the search's, not the instance's. We record the deepest complete
//! row the search ever reaches (its frontier). Row-major sweeps top-to-bottom;
//! a control run uses a uniformly RANDOM cell order over the same boards, which
//! has no "last region" and so should NOT localize.
//!
//! Everything canonical (generator, scorer, fit arithmetic) comes from the kit;
//! nothing here re-implements scoring. One JSON object on stdout with the full
//! per-board record and the aggregate localization statistics.

use e2_kit::{
    fit, generator, instance_from_generated, score_board, viewer_url, Board, Instance, Pieces,
    XorShift,
};
use serde_json::json;

/// One board's outcome: how deep the frontier reached and the final score.
struct BoardResult {
    gen_seed: u32,
    /// Deepest fully-filled row index reached across restarts (0-based); a
    /// board of side n that stalls with its last complete row = r has a hard
    /// region in rows r+1..n. n means the whole board was solved.
    frontier_row: usize,
    /// Frontier as a fraction of n: 1.0 means fully solved, 0.5 means the
    /// search stalled at the middle.
    frontier_frac: f64,
    /// Best (deepest) cell count reached, and the target cell count.
    best_cells: usize,
    target_cells: usize,
    /// Of the unfilled (residual) cells, the fraction that lie in the bottom
    /// half of the board (rows n/2..n). Computed directly from the final board,
    /// so it is meaningful for any slot order (row-major or the random control).
    /// NaN when the board is fully solved (no residual).
    bottom_residual_frac: f64,
    /// Canonical edge score of the deepest partial board.
    score: u32,
    full_solve: bool,
    nodes: u64,
    /// eternity2.dev viewer URL of the deepest partial board (row-major arm
    /// only; empty for the control to keep the record compact).
    url: String,
}

/// Restarting randomized exact-match DFS over a fixed slot order. Places only
/// perfect fits (rim colour 0 excluded by the scorer, placed neighbours equal).
/// The best (deepest) partial across restarts is what we report, so the
/// frontier is honest even when the search never completes.
struct Search<'a> {
    pieces: &'a Pieces,
    n: usize,
    slots: &'a [usize],
    board: Board,
    used: Vec<bool>,
    rng: XorShift,
    nodes: u64,
    node_cap: u64,
    restart_start: u64,
    restart_cap: u64,
    best_depth: usize,
    best_board: Board,
}

impl Search<'_> {
    /// Required colour per URDL side of a cell: rim wants BORDER, a placed
    /// neighbour constrains exactly, empty neighbour is unconstrained.
    fn wants(&self, pos: usize) -> [Option<u8>; 4] {
        fit::edge_constraints(&self.board, self.pieces, pos % self.n, pos / self.n, self.n, self.n)
    }

    fn out_of_gas(&self) -> bool {
        self.nodes - self.restart_start >= self.restart_cap || self.nodes >= self.node_cap
    }

    fn dfs(&mut self, depth: usize) -> bool {
        if depth > self.best_depth {
            self.best_depth = depth;
            self.best_board = self.board.clone();
        }
        if depth == self.slots.len() {
            return true;
        }
        if self.out_of_gas() {
            return false;
        }
        let pos = self.slots[depth];
        let want = self.wants(pos);

        let mut cands: Vec<(u16, u8)> = Vec::new();
        for (pid, piece) in self.pieces.iter() {
            if self.used[pid as usize] {
                continue;
            }
            for r in 0..4 {
                if fit::fit_score(&piece.rotated(r), &want).is_some() {
                    cands.push((pid, r));
                }
            }
        }
        self.rng.shuffle(&mut cands);

        for (pid, r) in cands {
            self.nodes += 1;
            self.board.place(pos, pid, r);
            self.used[pid as usize] = true;
            if self.dfs(depth + 1) {
                return true;
            }
            self.board.clear(pos);
            self.used[pid as usize] = false;
            if self.out_of_gas() {
                return false;
            }
        }
        false
    }
}

/// The deepest fully-filled row: the largest r such that every cell in rows
/// 0..=r is placed. On a strictly row-major fill this is `best_depth / n - 1`,
/// but computing it from the board makes it correct for any slot order.
fn deepest_full_row(board: &Board, n: usize) -> usize {
    let mut last = 0usize; // rows filled before this one
    for row in 0..n {
        let full = (0..n).all(|x| !board.is_empty_at(row * n + x));
        if full {
            last = row + 1;
        } else {
            break;
        }
    }
    last // number of complete rows from the top; 0..=n
}

fn run_board(
    instance: &Instance,
    n: usize,
    slots: &[usize],
    solver_seed: u32,
    node_cap: u64,
    restart_cap: u64,
    want_url: bool,
) -> BoardResult {
    let target_cells = n * n;
    let start = Board::new();
    let mut s = Search {
        pieces: &instance.pieces,
        n,
        slots,
        board: start.clone(),
        used: vec![false; instance.pieces.len()],
        rng: XorShift::new(solver_seed),
        nodes: 0,
        node_cap,
        restart_start: 0,
        restart_cap,
        best_depth: 0,
        best_board: start.clone(),
    };
    let solved = loop {
        s.restart_start = s.nodes;
        if s.dfs(0) {
            break true;
        }
        s.board = start.clone();
        for u in &mut s.used {
            *u = false;
        }
        if s.nodes >= s.node_cap {
            break false;
        }
    };
    let final_board = if solved { s.board.clone() } else { s.best_board.clone() };
    let frontier_row = deepest_full_row(&final_board, n);
    let best_cells = (0..target_cells).filter(|&p| !final_board.is_empty_at(p)).count();
    // Residual (unfilled) cells and how many lie in the bottom half.
    let bottom_start = (n / 2) * n; // first cell of row n/2
    let residual = target_cells - best_cells;
    let residual_bottom = (bottom_start..target_cells).filter(|&p| final_board.is_empty_at(p)).count();
    let bottom_residual_frac =
        if residual == 0 { f64::NAN } else { residual_bottom as f64 / residual as f64 };
    let score = score_board(&final_board, &instance.pieces);
    let url = if want_url {
        let cells = final_board.to_edge_cells(&instance.pieces);
        viewer_url(&instance.name, n as u8, &cells, &[])
    } else {
        String::new()
    };
    BoardResult {
        gen_seed: 0, // filled by caller
        frontier_row,
        frontier_frac: frontier_row as f64 / n as f64,
        best_cells,
        target_cells,
        bottom_residual_frac,
        score,
        full_solve: solved,
        nodes: s.nodes,
        url,
    }
}

fn median(v: &mut [f64]) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let m = v.len() / 2;
    if v.len() % 2 == 1 {
        v[m]
    } else {
        (v[m - 1] + v[m]) / 2.0
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let n: usize = flag(&args, "--n").unwrap_or(16);
    let seed_lo: u32 = flag(&args, "--seed-lo").unwrap_or(1);
    let seed_hi: u32 = flag(&args, "--seed-hi").unwrap_or(40);
    let colors: u8 = flag(&args, "--colors").unwrap_or(22);
    let solver_seed: u32 = flag(&args, "--solver-seed").unwrap_or(1);
    // Node budget per board and per-restart cap. Chosen so a full 16x16 sweep of
    // ~40 boards runs in a few minutes; the frontier is budget-insensitive
    // (more nodes do not push the stall row up), which the concept predicts.
    let node_cap: u64 = flag(&args, "--node-cap").unwrap_or(3_000_000);
    let restart_cap: u64 = flag(&args, "--restart-cap").unwrap_or(200_000);

    let effective = colors.min(generator::max_colors(n as u8));

    // Row-major (the sequential decomposition) and a random-order control.
    let row_major: Vec<usize> = (0..n * n).collect();

    eprintln!(
        "hard-region localization: n={n} colors={effective} seeds {seed_lo}..={seed_hi}, node_cap {node_cap}"
    );
    eprintln!("{:>5} {:>9} {:>9} {:>6} {:>7} {:>9}", "seed", "front/n", "cells", "full", "score", "nodes");

    let mut rows_main: Vec<BoardResult> = Vec::new();
    let mut rows_ctrl: Vec<BoardResult> = Vec::new();

    for gs in seed_lo..=seed_hi {
        let puzzle = generator::generate_framed(n as u8, effective, gs, true);
        let instance = instance_from_generated(&format!("hr-{n}x{n}-c{effective}-s{gs}"), &puzzle);

        // Row-major arm.
        let mut r = run_board(&instance, n, &row_major, solver_seed, node_cap, restart_cap, true);
        r.gen_seed = gs;
        eprintln!(
            "{:>5} {:>9.3} {:>4}/{:<4} {:>6} {:>7} {:>9}",
            gs, r.frontier_frac, r.best_cells, r.target_cells, r.full_solve, r.score, r.nodes
        );

        // Control arm: same board, uniformly shuffled slot order (no "last region").
        let mut ctrl_order = row_major.clone();
        let mut ord_rng = XorShift::new(0xC0FF_EE00 ^ gs);
        ord_rng.shuffle(&mut ctrl_order);
        let mut c = run_board(&instance, n, &ctrl_order, solver_seed, node_cap, restart_cap, false);
        c.gen_seed = gs;

        rows_main.push(r);
        rows_ctrl.push(c);
    }

    // --- Aggregate localization statistics (row-major arm) ------------------
    // Concept prediction: the top fills freely (high frontier fraction on
    // partial boards) and the stall concentrates in a bottom band, i.e. the
    // median frontier of the UNSOLVED boards sits well past the middle.
    let unsolved: Vec<&BoardResult> = rows_main.iter().filter(|r| !r.full_solve).collect();
    let mut unsolved_front: Vec<f64> = unsolved.iter().map(|r| r.frontier_frac).collect();
    let n_full = rows_main.iter().filter(|r| r.full_solve).count();
    // Fraction of the residual (unfilled) cells that lie in the bottom half of
    // the board, measured directly from each unsolved board and averaged.
    // Concept => most residual is bottom-concentrated (row-major); the random
    // control should sit near 0.5 (residual scattered, no "last region").
    let mut bottom_residual_main: Vec<f64> =
        unsolved.iter().map(|r| r.bottom_residual_frac).filter(|x| x.is_finite()).collect();

    let ctrl_unsolved: Vec<&BoardResult> = rows_ctrl.iter().filter(|r| !r.full_solve).collect();
    let mut ctrl_front: Vec<f64> = ctrl_unsolved.iter().map(|r| r.frontier_frac).collect();
    let mut bottom_residual_ctrl: Vec<f64> =
        ctrl_unsolved.iter().map(|r| r.bottom_residual_frac).filter(|x| x.is_finite()).collect();

    let median_unsolved_front = median(&mut unsolved_front);
    let mean_bottom_residual = if bottom_residual_main.is_empty() {
        f64::NAN
    } else {
        bottom_residual_main.iter().sum::<f64>() / bottom_residual_main.len() as f64
    };
    let median_bottom_residual = median(&mut bottom_residual_main);
    let median_ctrl_front = median(&mut ctrl_front);
    let mean_bottom_residual_ctrl = if bottom_residual_ctrl.is_empty() {
        f64::NAN
    } else {
        bottom_residual_ctrl.iter().sum::<f64>() / bottom_residual_ctrl.len() as f64
    };
    let _ = median(&mut bottom_residual_ctrl);

    let per_board: Vec<serde_json::Value> = rows_main
        .iter()
        .zip(rows_ctrl.iter())
        .map(|(r, c)| {
            json!({
                "gen_seed": r.gen_seed,
                "row_major": {
                    "frontier_row": r.frontier_row,
                    "frontier_frac": r.frontier_frac,
                    "best_cells": r.best_cells,
                    "target_cells": r.target_cells,
                    "bottom_residual_frac": nan_null(r.bottom_residual_frac),
                    "score": r.score,
                    "full_solve": r.full_solve,
                    "nodes": r.nodes,
                    "url": r.url,
                },
                "random_control": {
                    "frontier_row": c.frontier_row,
                    "frontier_frac": c.frontier_frac,
                    "best_cells": c.best_cells,
                    "bottom_residual_frac": nan_null(c.bottom_residual_frac),
                    "full_solve": c.full_solve,
                    "nodes": c.nodes,
                },
            })
        })
        .collect();

    let out = json!({
        "concept": "irreducible-hard-region-conjecture (vol-208)",
        "measures": "localization only (evidence-chain points 1-2); NOT the exact-window or too-coupled halves",
        "config": {
            "n": n,
            "colors": effective,
            "seed_lo": seed_lo,
            "seed_hi": seed_hi,
            "boards": (seed_hi - seed_lo + 1),
            "solver_seed": solver_seed,
            "node_cap": node_cap,
            "restart_cap": restart_cap,
            "from_scratch": true,
            "planted_solvable": true,
        },
        "aggregate": {
            "row_major": {
                "full_solves": n_full,
                "unsolved": unsolved.len(),
                "median_frontier_frac_unsolved": median_unsolved_front,
                "mean_bottom_half_residual_frac": mean_bottom_residual,
                "median_bottom_half_residual_frac": median_bottom_residual,
                "reading": "median frontier of unsolved boards >0.5 and bottom-residual >0.5 => hard region localizes to the last (bottom) band",
            },
            "random_control": {
                "full_solves": rows_ctrl.iter().filter(|r| r.full_solve).count(),
                "unsolved": ctrl_unsolved.len(),
                "median_frontier_frac_unsolved": median_ctrl_front,
                "mean_bottom_half_residual_frac": mean_bottom_residual_ctrl,
                "reading": "no 'last region' under a random order; residual scatters (bottom-residual ~0.5), so localization is a property of the sequential decomposition, not the board",
            },
        },
        "per_board": per_board,
    });
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}

/// JSON-encode a possibly-NaN float as `null` (JSON has no NaN).
fn nan_null(x: f64) -> serde_json::Value {
    if x.is_finite() {
        json!(x)
    } else {
        serde_json::Value::Null
    }
}

// -- tiny arg helper ---------------------------------------------------------

fn flag<T: std::str::FromStr>(args: &[String], name: &str) -> Option<T> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
}
