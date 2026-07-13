// Directed local search: identify "bad" cells (those touching any
// unmatched edge), free them, and SA only within the freed set. The
// "good" surround acts as a hard constraint via fixed colors. Swaps
// happen ONLY between freed cells (so a bad-piece can move to another
// bad slot but cannot displace a good piece).
//
// This is meant to address the 449-plateau on official Eternity II:
// pure PT cannot break past 449 because random-pair swap rarely
// proposes the specific multi-piece rearrangements that fix
// interlocked mismatches. By restricting attention to the bad set
// (~30-60 cells), the search space shrinks 4-8x and proposals are
// always relevant. Focused exploration.
//
// Algorithm:
//   1. Identify bad cells = union of cells touching mismatched edges.
//      Optionally expand by 1-cell padding so the boundary can move.
//   2. Run SA where swap proposals must pair (bad_cell, bad_cell).
//      Rotate proposals can be on any cell but bias toward bad ones.
//   3. Periodically recompute the bad-set as the board evolves.

use crate::{
    local_match_count, match_count_with, best_rotation,
    State, StateRef, Rng, SaOutcome,
};
use eternity2_core::{Board, Position, Puzzle, Rotation};

#[derive(Debug, Clone)]
pub struct DirectedConfig {
    pub temperature: f64,
    pub iterations: u64,
    pub time_budget_ms: u64,
    pub seed: u64,
    /// Refresh the bad set every N iterations (0 = never refresh).
    pub refresh_every: u64,
    /// Pad the bad set by N cells (0 = exact bad cells only).
    pub pad: u32,
}

impl Default for DirectedConfig {
    fn default() -> Self {
        Self {
            temperature: 0.05,
            iterations: 0,
            time_budget_ms: 30_000,
            seed: 0xE2_E2_E2_E2,
            refresh_every: 50_000,
            pad: 1,
        }
    }
}

/// Compute the set of "bad" cells on `board` = cells touching at least
/// one mismatched interior edge.
fn bad_cells(state: &State, board: &Board) -> Vec<Position> {
    let w = state.puzzle.width;
    let h = state.puzzle.height;
    let mut bad = vec![false; (w * h) as usize];
    for y in 0..h {
        for x in 0..w {
            let p = y * w + x;
            // Check right edge
            if x + 1 < w {
                let pr = y * w + (x + 1);
                if !edge_match(state, board, p, pr, /*dir_right=*/ true) {
                    bad[p as usize] = true;
                    bad[pr as usize] = true;
                }
            }
            // Check down edge
            if y + 1 < h {
                let pb = (y + 1) * w + x;
                if !edge_match(state, board, p, pb, /*dir_right=*/ false) {
                    bad[p as usize] = true;
                    bad[pb as usize] = true;
                }
            }
        }
    }
    let mut out = Vec::new();
    for (i, &b) in bad.iter().enumerate() {
        if b { out.push(i as Position); }
    }
    out
}

fn edge_match(state: &State, board: &Board, a: Position, b: Position, dir_right: bool) -> bool {
    let Some((pa, ra)) = board.get(a) else { return false; };
    let Some((pb, rb)) = board.get(b) else { return false; };
    let ea = state.piece_rot_edges[usize::from(pa) * 4 + usize::from(ra.as_u8())];
    let eb = state.piece_rot_edges[usize::from(pb) * 4 + usize::from(rb.as_u8())];
    if dir_right {
        ea[1] != 0 && ea[1] == eb[3]
    } else {
        ea[2] != 0 && ea[2] == eb[0]
    }
}

/// Pad a cell set by N cells in 4-connected neighborhood. Operates
/// on a width × height grid.
fn pad_cells(cells: &[Position], puzzle: &Puzzle, n: u32) -> Vec<Position> {
    let w = puzzle.width;
    let h = puzzle.height;
    let mut mark = vec![false; (w * h) as usize];
    for &c in cells { mark[c as usize] = true; }
    for _ in 0..n {
        let mut new_mark = mark.clone();
        for y in 0..h {
            for x in 0..w {
                let p = y * w + x;
                if !mark[p as usize] { continue; }
                if y > 0 { new_mark[((y - 1) * w + x) as usize] = true; }
                if x + 1 < w { new_mark[(y * w + (x + 1)) as usize] = true; }
                if y + 1 < h { new_mark[((y + 1) * w + x) as usize] = true; }
                if x > 0 { new_mark[(y * w + (x - 1)) as usize] = true; }
            }
        }
        mark = new_mark;
    }
    let mut out = Vec::new();
    for (i, &b) in mark.iter().enumerate() {
        if b { out.push(i as Position); }
    }
    out
}

/// Run directed SA: free only the bad cells, swap only among them.
pub fn run_directed(
    puzzle: &Puzzle,
    initial: &Board,
    cfg: &DirectedConfig,
) -> SaOutcome {
    let started = std::time::Instant::now();
    let state_ref = StateRef::new(puzzle);
    let state = state_ref.inner();
    let total_edges = state_ref.total_interior_edges();

    let mut board = initial.clone();
    let mut score = state_ref.score(&board);
    let mut best_board = board.clone();
    let mut best_score = score;
    let mut rng = Rng::new(cfg.seed);

    let mut bad = bad_cells(state, &board);
    bad = pad_cells(&bad, puzzle, cfg.pad);
    eprintln!("[directed] initial: score={} bad_cells={}",
        score, bad.len());

    // Partition bad cells by class (corner/edge/interior). Swap can only
    // happen within a class.
    let mut bad_corner: Vec<Position> = Vec::new();
    let mut bad_edge: Vec<Position> = Vec::new();
    let mut bad_interior: Vec<Position> = Vec::new();
    let partition = |bad: &[Position], state: &State,
                     bc: &mut Vec<Position>, be: &mut Vec<Position>, bi: &mut Vec<Position>| {
        bc.clear(); be.clear(); bi.clear();
        for &p in bad {
            use crate::CellClass::*;
            match state.cell_class[p as usize] {
                Corner => bc.push(p),
                Edge => be.push(p),
                Interior => bi.push(p),
            }
        }
    };
    partition(&bad, state, &mut bad_corner, &mut bad_edge, &mut bad_interior);

    let mut iters: u64 = 0;
    let timed_out = |start: &std::time::Instant| -> bool {
        cfg.time_budget_ms != 0
            && start.elapsed().as_millis() >= u128::from(cfg.time_budget_ms)
    };

    loop {
        if cfg.iterations != 0 && iters >= cfg.iterations { break; }
        if timed_out(&started) { break; }
        if best_score == total_edges { break; }

        // Refresh bad set occasionally.
        if cfg.refresh_every > 0 && iters > 0 && iters % cfg.refresh_every == 0 {
            bad = bad_cells(state, &board);
            bad = pad_cells(&bad, puzzle, cfg.pad);
            partition(&bad, state, &mut bad_corner, &mut bad_edge, &mut bad_interior);
            eprintln!("[directed] iter={}: refreshed bad_cells={} score={}",
                iters, bad.len(), score);
            if bad.is_empty() {
                eprintln!("[directed] all edges matched! score={}", score);
                break;
            }
        }

        // Move mix:
        //   0..3 rotate single (30%)
        //   3..10 swap two bad cells of same class (70%)
        let mk = rng.gen_range(10);
        if mk < 3 {
            // Rotate a random bad interior cell.
            if bad_interior.is_empty() {
                iters += 1; continue;
            }
            let pos = bad_interior[rng.gen_range(bad_interior.len() as u32) as usize];
            let Some((pid, old_rot)) = board.get(pos) else { iters += 1; continue; };
            let mut new_r = rng.gen_range(4) as u8;
            if new_r == old_rot.as_u8() { new_r = (new_r + 1) & 0b11; }
            let new_rot = Rotation::from_u8(new_r).unwrap();
            let old_local = local_match_count(state, &board, pos);
            let e_new = state.piece_rot_edges[usize::from(pid) * 4 + new_r as usize];
            let new_local = match_count_with(state, &board, pos, e_new);
            let delta = (new_local as i64) - (old_local as i64);
            let accept = if delta >= 0 { true }
                else { let p = (delta as f64 / cfg.temperature).exp(); rng.next_f64() < p };
            if accept {
                board.place(pos, pid, new_rot);
                score = (score as i64 + delta) as u32;
                if score > best_score { best_score = score; best_board = board.clone(); }
            }
        } else {
            // Swap two bad cells of same class.
            let class_pick = rng.gen_range(3);
            let cells = match class_pick {
                0 => &bad_corner,
                1 => &bad_edge,
                _ => &bad_interior,
            };
            if cells.len() < 2 { iters += 1; continue; }
            let i = rng.gen_range(cells.len() as u32) as usize;
            let mut j = rng.gen_range(cells.len() as u32) as usize;
            if i == j { j = (j + 1) % cells.len(); }
            let p_i = cells[i];
            let p_j = cells[j];
            let Some((pid_i, rot_i)) = board.get(p_i) else { iters += 1; continue; };
            let Some((pid_j, rot_j)) = board.get(p_j) else { iters += 1; continue; };
            let old_local_i = local_match_count(state, &board, p_i);
            let old_local_j = local_match_count(state, &board, p_j);
            let adj_old = crate::adjacent_match(state, &board, p_i, p_j);
            let old_local = old_local_i + old_local_j - adj_old;
            board.place(p_i, pid_j, rot_j);
            board.place(p_j, pid_i, rot_i);
            let best_rot_i = best_rotation(state, &board, p_i, pid_j);
            board.place(p_i, pid_j, best_rot_i);
            let best_rot_j = best_rotation(state, &board, p_j, pid_i);
            board.place(p_j, pid_i, best_rot_j);
            let new_local_i = local_match_count(state, &board, p_i);
            let new_local_j = local_match_count(state, &board, p_j);
            let new_adj = crate::adjacent_match(state, &board, p_i, p_j);
            let new_local = new_local_i + new_local_j - new_adj;
            let delta = (new_local as i64) - (old_local as i64);
            let accept = if delta >= 0 { true }
                else { let p = (delta as f64 / cfg.temperature).exp(); rng.next_f64() < p };
            if accept {
                score = (score as i64 + delta) as u32;
                if score > best_score { best_score = score; best_board = board.clone(); }
            } else {
                board.place(p_i, pid_i, rot_i);
                board.place(p_j, pid_j, rot_j);
            }
        }
        iters += 1;
    }

    SaOutcome {
        best_board,
        best_score,
        total_edges,
        iterations: iters,
        elapsed_us: started.elapsed().as_micros(),
    }
}
