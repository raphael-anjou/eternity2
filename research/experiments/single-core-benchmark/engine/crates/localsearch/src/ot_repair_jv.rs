// T37 — Jonker-Volgenant (JV) repair backend.
//
// Drop-in alternative to ot_repair's Kuhn-Munkres call. Same outer loop and
// best-of-rotation logic. JV (via lsap crate) is typically 2-10× faster on
// dense k×k cost matrices and JOINTLY optimizes piece × rotation when the
// "joint" mode is enabled.
//
// References:
// - Jonker & Volgenant 1987, "A shortest augmenting path algorithm for dense
//   and sparse linear assignment problems"
// - SciPy's `linear_sum_assignment` uses this same algorithm.
//
// API design:
// - iterative_jv_repair: signature-identical to iterative_ot_repair.
//   Uses lsap with the same best-of-rotation memo as KM version.
// - iterative_jv_joint_repair: 4k×k block formulation — each piece has 4
//   "virtual rows" (one per rotation). Solve assignment that picks one of
//   the 4 rows per piece AND one of the k columns. Guaranteed jointly
//   optimal over piece-permutation and rotation.

use std::collections::BTreeSet;
use eternity2_core::{Board, Position, Puzzle, Rotation, BORDER};

/// Compute the best-rotation match count for piece `pid` at position `pos`
/// against the current board's static neighbors. Returns (best_matches, best_rotation).
fn best_rotation_at(
    puzzle: &Puzzle,
    board: &Board,
    pid: eternity2_core::PieceId,
    pos: Position,
) -> (i64, Rotation) {
    let w = puzzle.width;
    let h = puzzle.height;
    let rotations = [Rotation::R0, Rotation::R90, Rotation::R180, Rotation::R270];
    let p = puzzle.piece(pid).expect("piece exists");
    let x = pos % w;
    let y = pos / w;
    let mut best_matches: i64 = -1;
    let mut best_rot = Rotation::R0;
    for &rot in &rotations {
        let e = p.edges.rotated(rot).as_array();
        let mut matches: i64 = 0;
        if y == 0 {
            if e[0] == BORDER { matches += 1; }
        } else if let Some((npid, nrot)) = board.get(pos - w) {
            let ne = puzzle.piece(npid).unwrap().edges.rotated(nrot).as_array();
            if e[0] == ne[2] { matches += 1; }
        }
        if x + 1 == w {
            if e[1] == BORDER { matches += 1; }
        } else if let Some((npid, nrot)) = board.get(pos + 1) {
            let ne = puzzle.piece(npid).unwrap().edges.rotated(nrot).as_array();
            if e[1] == ne[3] { matches += 1; }
        }
        if y + 1 == h {
            if e[2] == BORDER { matches += 1; }
        } else if let Some((npid, nrot)) = board.get(pos + w) {
            let ne = puzzle.piece(npid).unwrap().edges.rotated(nrot).as_array();
            if e[2] == ne[0] { matches += 1; }
        }
        if x == 0 {
            if e[3] == BORDER { matches += 1; }
        } else if let Some((npid, nrot)) = board.get(pos - 1) {
            let ne = puzzle.piece(npid).unwrap().edges.rotated(nrot).as_array();
            if e[3] == ne[1] { matches += 1; }
        }
        if matches > best_matches {
            best_matches = matches;
            best_rot = rot;
        }
    }
    (best_matches, best_rot)
}

/// Compute matches for piece `pid` with specific rotation `rot` at position `pos`.
fn matches_with_rotation(
    puzzle: &Puzzle,
    board: &Board,
    pid: eternity2_core::PieceId,
    rot: Rotation,
    pos: Position,
) -> i64 {
    let w = puzzle.width;
    let h = puzzle.height;
    let p = puzzle.piece(pid).expect("piece exists");
    let x = pos % w;
    let y = pos / w;
    let e = p.edges.rotated(rot).as_array();
    let mut matches: i64 = 0;
    if y == 0 {
        if e[0] == BORDER { matches += 1; }
    } else if let Some((npid, nrot)) = board.get(pos - w) {
        let ne = puzzle.piece(npid).unwrap().edges.rotated(nrot).as_array();
        if e[0] == ne[2] { matches += 1; }
    }
    if x + 1 == w {
        if e[1] == BORDER { matches += 1; }
    } else if let Some((npid, nrot)) = board.get(pos + 1) {
        let ne = puzzle.piece(npid).unwrap().edges.rotated(nrot).as_array();
        if e[1] == ne[3] { matches += 1; }
    }
    if y + 1 == h {
        if e[2] == BORDER { matches += 1; }
    } else if let Some((npid, nrot)) = board.get(pos + w) {
        let ne = puzzle.piece(npid).unwrap().edges.rotated(nrot).as_array();
        if e[2] == ne[0] { matches += 1; }
    }
    if x == 0 {
        if e[3] == BORDER { matches += 1; }
    } else if let Some((npid, nrot)) = board.get(pos - 1) {
        let ne = puzzle.piece(npid).unwrap().edges.rotated(nrot).as_array();
        if e[3] == ne[1] { matches += 1; }
    }
    matches
}

/// JV-based iterative repair. Signature-compatible with iterative_ot_repair.
/// Uses lsap (Jonker-Volgenant) instead of Kuhn-Munkres.
///
/// Behavior: same as KM version. Best-rotation chosen per (i, j) cell against
/// the FROZEN halo; then LAP assigns pieces to cells.
pub fn iterative_jv_repair(
    puzzle: &Puzzle,
    board: &Board,
    free_set: &BTreeSet<Position>,
    max_iters: u32,
) -> Board {
    let mut current = board.clone();
    let positions: Vec<Position> = free_set.iter().copied().collect();
    let k = positions.len();
    if k == 0 {
        return current;
    }

    for _iter in 0..max_iters {
        let mut pieces = Vec::with_capacity(k);
        for &pos in &positions {
            if let Some((pid, _)) = current.get(pos) {
                pieces.push(pid);
            } else {
                return current;
            }
        }

        // Build cost matrix in row-major flat layout (k rows × k cols).
        // lsap MINIMIZES cost; convert match-counts to costs via negation.
        // Maximum possible matches per cell is 4. Use cost = (4 - matches) to
        // get non-negative integer costs in [0..=4].
        let mut cost: Vec<f64> = vec![0.0; k * k];
        let mut best_rotations = vec![vec![Rotation::R0; k]; k];

        for (i, &pid) in pieces.iter().enumerate() {
            for (j, &pos) in positions.iter().enumerate() {
                let (matches, rot) = best_rotation_at(puzzle, &current, pid, pos);
                // Convert to cost (smaller is better).
                cost[i * k + j] = (4 - matches) as f64;
                best_rotations[i][j] = rot;
            }
        }

        // Solve LAP via lsap (Jonker-Volgenant). maximize=false → minimize cost.
        let (_row_ind, col_ind) = lsap::solve(k, k, &cost, false).expect("LAP solvable");

        // col_ind[i] is the column (position-index) assigned to row i (piece-index).
        let mut next = current.clone();
        let mut changed = false;
        for i in 0..k {
            let j = col_ind[i] as usize;
            let pid = pieces[i];
            let pos = positions[j];
            let rot = best_rotations[i][j];
            if next.get(pos) != Some((pid, rot)) {
                changed = true;
            }
            next.place(pos, pid, rot);
        }
        current = next;
        if !changed { break; }
    }
    current
}

/// JV-based iterative repair with JOINT piece × rotation LAP.
///
/// Block formulation: 4k rows (piece × rotation) × k columns (positions).
/// Each piece has 4 candidate rows; LAP must pick exactly ONE row per piece
/// AND assign it to exactly ONE column. This is encoded by adding 3 "dummy"
/// columns per piece (large cost) so the LAP must pick a non-dummy column.
///
/// Equivalent formulation: build a (4k × 4k) cost matrix where the extra
/// columns are dummies that disallow more than one rotation per piece.
///
/// Joint LAP guarantees globally optimal assignment over piece-permutation
/// AND rotation, removing the sub-optimality of "best-of-4 in cost".
pub fn iterative_jv_joint_repair(
    puzzle: &Puzzle,
    board: &Board,
    free_set: &BTreeSet<Position>,
    max_iters: u32,
) -> Board {
    let mut current = board.clone();
    let positions: Vec<Position> = free_set.iter().copied().collect();
    let k = positions.len();
    if k == 0 {
        return current;
    }
    let rotations = [Rotation::R0, Rotation::R90, Rotation::R180, Rotation::R270];

    // Large cost for invalid combinations (so LAP avoids them).
    // Max valid cost per cell is 4 (cost = 4 - matches, matches in [0..=4]).
    // Using 1000 ensures LAP never picks a dummy when a valid choice exists.
    let big: f64 = 1000.0;

    for _iter in 0..max_iters {
        let mut pieces = Vec::with_capacity(k);
        for &pos in &positions {
            if let Some((pid, _)) = current.get(pos) {
                pieces.push(pid);
            } else {
                return current;
            }
        }

        // Build (4k × 4k) cost matrix.
        // Rows: 4k = k pieces × 4 rotations. Row index r = piece_i * 4 + rot_i.
        // Columns: 4k = k positions × 4 "slots".
        //   - Columns 0..k are the REAL positions.
        //   - Columns k..4k are DUMMY positions (one dummy per excess row).
        // Constraint: each piece must be assigned to exactly one column. Since
        // a piece occupies 4 rows in the LAP, 3 of its rows must go to dummies.
        // Make dummies CHEAP for the same piece's other rotations and EXPENSIVE
        // for other pieces' rotations, so the LAP picks consistent (piece, rot) tuples.
        //
        // Trick: assign 3 dummy columns per piece. Cost is BIG for any piece-row
        // OTHER than this piece's rows, and ZERO for this piece's other-rotation
        // rows. Then the LAP must pick 1 of the 4 rows to the real position and
        // the other 3 to dummies.
        //
        // Dummy column dummy_idx = k + piece_i * 3 + (0|1|2). The piece_i's
        // rows at rotations OTHER than the chosen one go to these dummies (cost 0);
        // any other piece's row at this dummy column gets BIG cost.

        let dim = 4 * k;
        let mut cost: Vec<f64> = vec![big; dim * dim];

        for (i, &pid) in pieces.iter().enumerate() {
            for (r_idx, &rot) in rotations.iter().enumerate() {
                let row = i * 4 + r_idx;
                // Real positions (columns 0..k).
                for (j, &pos) in positions.iter().enumerate() {
                    let m = matches_with_rotation(puzzle, &current, pid, rot, pos);
                    cost[row * dim + j] = (4 - m) as f64;
                }
                // Dummy positions for THIS piece (cost 0).
                // dummy_idx = k + piece_i * 3 + d for d in 0..3
                for d in 0..3 {
                    let dummy_col = k + i * 3 + d;
                    cost[row * dim + dummy_col] = 0.0;
                }
                // Other pieces' dummies remain BIG (already set).
            }
        }

        let (_row_ind, col_ind) = lsap::solve(dim, dim, &cost, false).expect("LAP solvable");

        // Parse col_ind: row_idx -> col_idx. For each piece, find which row
        // (rotation) got assigned to a REAL column.
        let mut next = current.clone();
        let mut changed = false;
        for i in 0..k {
            // Among rows i*4..(i+1)*4, find the one with col < k (a real position).
            let mut chosen_row = None;
            for r in 0..4 {
                let row = i * 4 + r;
                let col = col_ind[row] as usize;
                if col < k {
                    chosen_row = Some((r, col));
                    break;
                }
            }
            if let Some((r_idx, j)) = chosen_row {
                let pid = pieces[i];
                let pos = positions[j];
                let rot = rotations[r_idx];
                if next.get(pos) != Some((pid, rot)) {
                    changed = true;
                }
                next.place(pos, pid, rot);
            }
        }
        current = next;
        if !changed { break; }
    }
    current
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    /// Build a small symmetric LAP and verify lsap result matches a brute-force.
    #[test]
    fn lsap_basic_3x3() {
        // 3x3 LAP. Manually craft a cost matrix.
        // Expected min: row 0 → col 1 (cost 1), row 1 → col 0 (cost 2), row 2 → col 2 (cost 2). Total = 5.
        // Or: row 0 → col 1, row 1 → col 2, row 2 → col 0 → 1+5+3 = 9. So min is 5.
        let cost = vec![
            4.0, 1.0, 3.0,
            2.0, 0.0, 5.0,
            3.0, 2.0, 2.0,
        ];
        let (_, col_ind) = lsap::solve(3, 3, &cost, false).unwrap();
        let total: f64 = (0..3).map(|i| cost[i * 3 + col_ind[i] as usize]).sum();
        assert!(total <= 5.0, "lsap result not optimal: total={}, assignment={:?}", total, col_ind);
    }

    /// Sanity: iterative_jv_repair preserves board scoring on a trivial 0-free-set.
    #[test]
    fn jv_repair_empty_freeset() {
        // We can't easily build a real Puzzle here without big setup. Test will
        // be added via the integration crate; this stub serves as a placeholder.
    }
}
