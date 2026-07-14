// Vol-18 — OracleCycleSwap: cross the 447 → 456 first-order barrier in
// one move by applying coordinated σ-cycle swaps from a known higher-
// score oracle board.
//
// Motivation (RESEARCH_NOTES_18 R5e/R5f):
// - 447 and 456 boards differ by 76 misplaced pieces.
// - σ = oracle_pos ∘ current_pos⁻¹ decomposes into ~10 cycles.
// - Every cycle has Δ ∈ {-4 .. -34} when applied alone.
// - The full 10-cycle move gives Δ = +9 (the score gap).
// - No subset < 76 cells crosses Δ ≥ 0.
// - Standard MCMC at T=1 cannot accept the move (p ≈ 10^-15).
//
// This module bypasses MCMC barrier-crossing by applying oracle's
// piece+rotation assignment to a chosen σ-cycle in ONE step. Combined
// with a normal accept/reject test (SA at any T accepts Δ ≥ 0), this
// gives ALNS access to the cooperative basin.

use std::collections::BTreeMap;
use eternity2_core::{Board, PieceId, Position, Puzzle, Rotation};

/// One σ-cycle: positions in the order such that piece at positions[i]
/// belongs at positions[(i+1) % k].
#[derive(Debug, Clone)]
pub struct SigmaCycle {
    pub positions: Vec<Position>,
    /// Pre-computed pieces currently at each position (the cycle moves these around).
    pub pieces_in_order: Vec<PieceId>,
    /// Oracle's rotation for each piece (lookup by index into pieces_in_order).
    pub oracle_rotations: Vec<Rotation>,
}

/// Compute σ-cycles between current board and oracle board.
///
/// σ on positions: σ(p) = ora_pos[piece at p].
/// A cycle is positions [p, σ(p), σ²(p), ...] that closes.
/// Cycles of length 1 (fixed points) are filtered out — for rotation
/// correction at fixed points use [`rotation_fixups`].
pub fn compute_sigma_cycles(current: &Board, oracle: &Board, cell_count: usize) -> Vec<SigmaCycle> {
    // Build piece -> oracle_pos and piece -> oracle_rot maps.
    let mut ora_pos: BTreeMap<PieceId, (Position, Rotation)> = BTreeMap::new();
    for pos in 0..cell_count as Position {
        if let Some((pid, rot)) = oracle.get(pos) {
            ora_pos.insert(pid, (pos, rot));
        }
    }
    // σ(p) for each placed p.
    let mut sigma: BTreeMap<Position, Position> = BTreeMap::new();
    for pos in 0..cell_count as Position {
        if let Some((pid, _)) = current.get(pos) {
            if let Some(&(target, _)) = ora_pos.get(&pid) {
                sigma.insert(pos, target);
            }
        }
    }
    // Cycle decomposition.
    let mut seen: std::collections::BTreeSet<Position> = std::collections::BTreeSet::new();
    let mut out: Vec<SigmaCycle> = Vec::new();
    for &start in sigma.keys() {
        if seen.contains(&start) {
            continue;
        }
        let mut positions: Vec<Position> = Vec::new();
        let mut q = start;
        // Walk only while q is in sigma's domain (placed in current
        // AND its piece has an oracle target). Otherwise we'd push a
        // non-placed position into the cycle and crash at piece lookup.
        // Vol-108 T1 fix: was pushing q before sigma-membership check.
        while !seen.contains(&q) && sigma.contains_key(&q) {
            seen.insert(q);
            positions.push(q);
            match sigma.get(&q) {
                Some(&next) => q = next,
                None => break,
            }
        }
        if positions.len() < 2 {
            continue;
        }
        // Capture pieces and oracle rotations.
        let mut pieces_in_order: Vec<PieceId> = Vec::with_capacity(positions.len());
        let mut oracle_rotations: Vec<Rotation> = Vec::with_capacity(positions.len());
        for &p in &positions {
            let (pid, _) = current.get(p).expect("position has piece");
            pieces_in_order.push(pid);
            let (_, rot) = ora_pos.get(&pid).expect("piece has oracle pos");
            oracle_rotations.push(*rot);
        }
        out.push(SigmaCycle { positions, pieces_in_order, oracle_rotations });
    }
    // Sort by length descending — biggest cycles are the hard moves.
    out.sort_by_key(|c| std::cmp::Reverse(c.positions.len()));
    out
}

/// Apply one σ-cycle: for each i, place pieces_in_order[i] at
/// positions[(i+1) % k] with oracle_rotations[i].
pub fn apply_cycle(board: &Board, cycle: &SigmaCycle) -> Board {
    let mut new = board.clone();
    let k = cycle.positions.len();
    for i in 0..k {
        let pid = cycle.pieces_in_order[i];
        let rot = cycle.oracle_rotations[i];
        let target = cycle.positions[(i + 1) % k];
        new.place(target, pid, rot);
    }
    new
}

/// Apply ALL cycles atomically (the full 76-cell move in our reference case).
pub fn apply_all_cycles(board: &Board, cycles: &[SigmaCycle]) -> Board {
    let mut new = board.clone();
    for c in cycles {
        new = apply_cycle(&new, c);
    }
    new
}

/// Find positions where current and oracle agree on piece_id but disagree
/// on rotation. These are σ fixed points; compute_sigma_cycles filters
/// them out because their cycle length is 1, but they still contribute
/// score gains when corrected.
pub fn rotation_fixups(current: &Board, oracle: &Board, cell_count: usize) -> Vec<(Position, Rotation)> {
    let mut out = Vec::new();
    for pos in 0..cell_count as Position {
        if let (Some((c_pid, c_rot)), Some((o_pid, o_rot))) = (current.get(pos), oracle.get(pos)) {
            if c_pid == o_pid && c_rot != o_rot {
                out.push((pos, o_rot));
            }
        }
    }
    out
}

/// Apply rotation fixups to a board.
pub fn apply_rotation_fixups(board: &Board, fixups: &[(Position, Rotation)]) -> Board {
    let mut new = board.clone();
    for &(pos, rot) in fixups {
        if let Some((pid, _)) = new.get(pos) {
            new.place(pos, pid, rot);
        }
    }
    new
}

/// Apply a subset of cycles atomically.
pub fn apply_cycle_subset(board: &Board, cycles: &[SigmaCycle], indices: &[usize]) -> Board {
    let mut new = board.clone();
    for &i in indices {
        new = apply_cycle(&new, &cycles[i]);
    }
    new
}

#[cfg(test)]
mod tests {
    use super::*;
    // Smoke test in lib::test_oracle_swap (which has puzzle fixtures available).
}
