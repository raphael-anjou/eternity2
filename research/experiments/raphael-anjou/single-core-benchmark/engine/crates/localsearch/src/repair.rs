// Region-repair: take a fully-placed board, find the worst k×k region
// (by internal match density), pin everything outside the region as
// hints, run CP on the inside cells with a small time budget.
//
// This is the "mini-CP-repair" SA move operator: it makes the kinds
// of cooperative multi-piece changes that single-piece SA cannot.
// Closely related to RTCP (Exp N) but used as a *local* operator
// inside SA, not a top-level algorithm.

use eternity2_core::{Board, Hint, Hints, Position, Puzzle, Rotation, PieceId};
use eternity2_events::BufferSink;
use eternity2_solver_engine::EngineSolver;
use eternity2_solver_trait::{SolveOpts, SolveOutcome, Solver};

/// Find the (x, y) of the k×k region with the lowest internal+boundary
/// match score on `board`. (x, y) is the top-left of the region.
/// Returns None if k > min(w, h).
pub fn worst_region(puzzle: &Puzzle, board: &Board, k: u32) -> Option<(u32, u32)> {
    let w = puzzle.width;
    let h = puzzle.height;
    if k == 0 || k > w || k > h { return None; }
    let mut best: Option<((u32, u32), i32)> = None;
    for y0 in 0..=h - k {
        for x0 in 0..=w - k {
            // Count matched edges inside or touching the region.
            // We use the "internal-only" score for simplicity: pairs
            // of cells both inside the region whose shared edge matches.
            let mut matched = 0i32;
            let mut edges = 0i32;
            // Horizontal edges inside the region:  (k-1)*k of them.
            for dy in 0..k {
                for dx in 0..k - 1 {
                    let p = (y0 + dy) * w + (x0 + dx);
                    let pr = p + 1;
                    edges += 1;
                    if edges_match_right(puzzle, board, p, pr) { matched += 1; }
                }
            }
            // Vertical edges inside the region: k*(k-1) of them.
            for dy in 0..k - 1 {
                for dx in 0..k {
                    let p = (y0 + dy) * w + (x0 + dx);
                    let pb = p + w;
                    edges += 1;
                    if edges_match_down(puzzle, board, p, pb) { matched += 1; }
                }
            }
            // Worst = lowest matched count (or highest mismatched count).
            let score = matched;
            match best {
                None => best = Some(((x0, y0), score)),
                Some((_, bs)) if score < bs => best = Some(((x0, y0), score)),
                _ => {}
            }
            let _ = edges;
        }
    }
    best.map(|(p, _)| p)
}

fn edges_match_right(puzzle: &Puzzle, board: &Board, a: Position, b: Position) -> bool {
    let Some((pid_a, rot_a)) = board.get(a) else { return false; };
    let Some((pid_b, rot_b)) = board.get(b) else { return false; };
    let pa = puzzle.piece(pid_a);
    let pb = puzzle.piece(pid_b);
    if let (Some(pa), Some(pb)) = (pa, pb) {
        let ea = pa.edges.rotated(rot_a).as_array();
        let eb = pb.edges.rotated(rot_b).as_array();
        ea[1] != 0 && ea[1] == eb[3]
    } else { false }
}

fn edges_match_down(puzzle: &Puzzle, board: &Board, a: Position, b: Position) -> bool {
    let Some((pid_a, rot_a)) = board.get(a) else { return false; };
    let Some((pid_b, rot_b)) = board.get(b) else { return false; };
    let pa = puzzle.piece(pid_a);
    let pb = puzzle.piece(pid_b);
    if let (Some(pa), Some(pb)) = (pa, pb) {
        let ea = pa.edges.rotated(rot_a).as_array();
        let eb = pb.edges.rotated(rot_b).as_array();
        ea[2] != 0 && ea[2] == eb[0]
    } else { false }
}

/// Repair an arbitrary set of cells: pin everything except `free_cells`
/// from `board`, then run CP on the rest. Returns the new board if CP
/// found a complete fill, else None.
///
/// Cells in `free_cells` are emptied (and to be re-found by CP).
/// All other cells (and the hints, transitively) become hints to the CP.
/// `budget_ms` bounds CP time.
pub fn repair_cells(
    puzzle: &Puzzle,
    board: &Board,
    free_cells: &[Position],
    budget_ms: u64,
) -> Option<Board> {
    use std::collections::HashSet;
    let free_set: HashSet<Position> = free_cells.iter().copied().collect();
    let mut hs = Vec::with_capacity(puzzle.cell_count() as usize);
    for pos in 0..puzzle.cell_count() {
        if free_set.contains(&pos) { continue; }
        if let Some((pid, rot)) = board.get(pos) {
            hs.push(Hint { position: pos, piece_id: pid, rotation: rot });
        }
    }
    let hints = Hints::new(hs);

    let mut solver = EngineSolver::gacolor_ac3_par();
    let mut sink = BufferSink::new();
    let mut opts = SolveOpts::default();
    opts.time_budget_ms = budget_ms;
    opts.hints = hints;
    let outcome = solver.solve(puzzle, &opts, &mut sink);
    let new_board = match outcome {
        SolveOutcome::Solved(b) => b,
        SolveOutcome::TimedOut { best_partial, .. }
        | SolveOutcome::Cancelled { best_partial, .. } => best_partial,
        SolveOutcome::Exhausted | SolveOutcome::Error(_) => return None,
        SolveOutcome::AllSolutions(bs) => bs.into_iter().next()?,
    };

    // Only return if CP filled every freed cell.
    for &pos in free_cells {
        if new_board.get(pos).is_none() {
            return None;
        }
    }
    Some(new_board)
}

/// Attempt to repair a region by re-solving it with CP. Returns the new
/// board if CP found an improvement, else None.
///
/// `board` is fully placed. `(x0, y0)` is the top-left of the k×k region
/// to free. `budget_ms` is the CP time budget.
pub fn repair_region(
    puzzle: &Puzzle,
    board: &Board,
    x0: u32, y0: u32, k: u32,
    budget_ms: u64,
) -> Option<Board> {
    let w = puzzle.width;
    let h = puzzle.height;

    // Free a (k+2)×(k+2) window around the target region — gives CP
    // room to also fix the boundary, which is often where the bad
    // edges live. The interior k×k must be re-solved AND its
    // boundary cells get a chance to be re-chosen too.
    let pad = 1u32; // 1 cell on each side
    let fx0 = x0.saturating_sub(pad);
    let fy0 = y0.saturating_sub(pad);
    let fx1 = (x0 + k + pad).min(w);
    let fy1 = (y0 + k + pad).min(h);

    let mut hs = Vec::with_capacity((w * h) as usize);
    for y in 0..h {
        for x in 0..w {
            let in_free = x >= fx0 && x < fx1 && y >= fy0 && y < fy1;
            if in_free { continue; }
            let pos = y * w + x;
            if let Some((pid, rot)) = board.get(pos) {
                hs.push(Hint { position: pos, piece_id: pid, rotation: rot });
            }
        }
    }
    let hints = Hints::new(hs);

    // Run CP. The inside cells will be filled by CP search.
    let mut solver = EngineSolver::gacolor_ac3_par();
    let mut sink = BufferSink::new();
    let mut opts = SolveOpts::default();
    opts.time_budget_ms = budget_ms;
    opts.hints = hints;
    let outcome = solver.solve(puzzle, &opts, &mut sink);
    let new_board = match outcome {
        SolveOutcome::Solved(b) => b,
        SolveOutcome::TimedOut { best_partial, .. } | SolveOutcome::Cancelled { best_partial, .. } => best_partial,
        SolveOutcome::Exhausted | SolveOutcome::Error(_) => return None,
        SolveOutcome::AllSolutions(bs) => bs.into_iter().next()?,
    };

    // Only return if CP filled the entire freed window (padded region).
    // Otherwise the result would be a partial board and we can't score
    // it fairly against the original fully-placed board.
    for y in fy0..fy1 {
        for x in fx0..fx1 {
            if new_board.get(y * w + x).is_none() {
                return None; // CP didn't complete the freed window
            }
        }
    }
    let _ = (Rotation::from_u8(0), PieceId::default);
    Some(new_board)
}
