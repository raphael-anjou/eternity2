// Iterative Optimal Transport (Hungarian / Kuhn-Munkres) repair.
//
// Reframe local repair as a Linear Assignment Problem. For the k pieces
// currently residing in `free_set`, find the permutation that MAXIMIZES the
// sum of best-rotation match counts against current (static) neighbors.
//
// kuhn_munkres MAXIMIZES the total weight. Reward[i][j] = #matches piece i
// would have at position j with its current neighbors (maximized over the 4
// rotations of piece i). Borders match when the edge color equals BORDER.
//
// Iterate until fixed point or max_iters.
//
// Bug history (do NOT undo): prior version (a) called
// `rotated(rot as u8)` (wrong: needs Rotation), and (b) passed mismatch
// counts as kuhn_munkres weight (wrong: that maximises mismatches because
// kuhn_munkres is a maximiser). Both bugs are fixed here.

use std::collections::BTreeSet;
use eternity2_core::{Board, Position, Puzzle, Rotation, BORDER};
use pathfinding::kuhn_munkres::kuhn_munkres;
use pathfinding::matrix::Matrix;

pub fn iterative_ot_repair(
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
    let w = puzzle.width;
    let h = puzzle.height;
    let rotations = [Rotation::R0, Rotation::R90, Rotation::R180, Rotation::R270];

    for _iter in 0..max_iters {
        // 1. Snapshot the k pieces currently at the free positions.
        let mut pieces = Vec::with_capacity(k);
        for &pos in &positions {
            if let Some((pid, _)) = current.get(pos) {
                pieces.push(pid);
            } else {
                // free cell empty — OT can't reason; punt.
                return current;
            }
        }

        // 2. Reward matrix and best-rotation memo.
        let mut weights: Vec<i64> = Vec::with_capacity(k * k);
        let mut best_rotations = vec![vec![Rotation::R0; k]; k];

        for (i, &pid) in pieces.iter().enumerate() {
            let p = puzzle.piece(pid).expect("piece exists");
            for (j, &pos) in positions.iter().enumerate() {
                let x = pos % w;
                let y = pos / w;
                let mut best_matches: i64 = -1;
                let mut best_rot = Rotation::R0;
                for &rot in &rotations {
                    let e = p.edges.rotated(rot).as_array();
                    let mut matches: i64 = 0;
                    // North (row y-1 or grid border).
                    if y == 0 {
                        if e[0] == BORDER { matches += 1; }
                    } else if let Some((npid, nrot)) = current.get(pos - w) {
                        let ne = puzzle.piece(npid).unwrap().edges.rotated(nrot).as_array();
                        if e[0] == ne[2] { matches += 1; }
                    }
                    // East (col x+1 or border).
                    if x + 1 == w {
                        if e[1] == BORDER { matches += 1; }
                    } else if let Some((npid, nrot)) = current.get(pos + 1) {
                        let ne = puzzle.piece(npid).unwrap().edges.rotated(nrot).as_array();
                        if e[1] == ne[3] { matches += 1; }
                    }
                    // South.
                    if y + 1 == h {
                        if e[2] == BORDER { matches += 1; }
                    } else if let Some((npid, nrot)) = current.get(pos + w) {
                        let ne = puzzle.piece(npid).unwrap().edges.rotated(nrot).as_array();
                        if e[2] == ne[0] { matches += 1; }
                    }
                    // West.
                    if x == 0 {
                        if e[3] == BORDER { matches += 1; }
                    } else if let Some((npid, nrot)) = current.get(pos - 1) {
                        let ne = puzzle.piece(npid).unwrap().edges.rotated(nrot).as_array();
                        if e[3] == ne[1] { matches += 1; }
                    }
                    if matches > best_matches {
                        best_matches = matches;
                        best_rot = rot;
                    }
                }
                weights.push(best_matches);
                best_rotations[i][j] = best_rot;
            }
        }

        // 3. Kuhn-Munkres maximum-weight perfect matching.
        let cost_matrix = Matrix::from_vec(k, k, weights).expect("k*k weights");
        let (_score, assignment) = kuhn_munkres(&cost_matrix);

        // 4. Apply assignment. Detect fixed point.
        let mut next = current.clone();
        let mut changed = false;
        for (i, &j) in assignment.iter().enumerate() {
            let pid = pieces[i];
            let pos = positions[j];
            let rot = best_rotations[i][j];
            if next.get(pos) != Some((pid, rot)) {
                changed = true;
            }
            next.place(pos, pid, rot);
        }
        current = next;
        if !changed {
            break;
        }
    }
    current
}
