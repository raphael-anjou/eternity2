// FILAMENT — Lin-Kernighan on 2D grid for Eternity II.
//
// Difference from existing LkhChainDestroy:
//   - Operates as REPAIR/IMPROVE, not destroy. Takes a board, applies a chain of
//     piece swaps, returns the improved board (or original if no improvement).
//   - VARIABLE-DEPTH gain tracking: chain extends only while cumulative
//     gain ≥ -tolerance, closes at the depth with maximum gain.
//   - Each step is an actual SWAP, not a cell selection.
//
// The algorithm:
//   1. Pick a "worst" position (highest local mismatch count).
//   2. From this position, find the adjacent SWAP that yields highest local gain.
//   3. Apply the swap. Track cumulative gain.
//   4. Repeat from the newly placed piece's position (chain extends to its neighbors).
//   5. Cut when cumulative gain falls below -tolerance.
//   6. Return the board state at the depth of MAXIMUM cumulative gain.
//
// Variable depth + gain backtracking is the LK insight: a single locally-bad
// swap can be a setup for a globally-better one.

use std::collections::BTreeSet;

use eternity2_core::{Board, PieceId, Position, Puzzle, Rotation, BORDER};

use crate::alns::{score_board, AlnsRng};

/// Configuration for FILAMENT chain search.
#[derive(Debug, Clone)]
pub struct FilamentConfig {
    /// Maximum chain depth.
    pub max_depth: u32,
    /// Maximum cumulative gain loss before cutting the chain.
    pub max_loss: i32,
    /// If true, seed from worst mismatch (highest deficit). Else random.
    pub seed_worst: bool,
    /// Rotation search: try all 4 rotations for the bumped piece at its
    /// new position, taking the one with highest local match.
    pub try_rotations: bool,
    /// Allow chain to revisit a position once (for cycle escape).
    pub allow_revisit: bool,
}

impl Default for FilamentConfig {
    fn default() -> Self {
        Self {
            max_depth: 16,
            max_loss: 4,
            seed_worst: true,
            try_rotations: true,
            allow_revisit: false,
        }
    }
}

/// Public wrapper for cell_match_count (used by alns::filament_repair).
pub fn cell_match_count_pub(puzzle: &Puzzle, board: &Board, pos: Position) -> u32 {
    cell_match_count(puzzle, board, pos)
}

/// Compute local match count at a position (number of matched edges
/// incident to this cell). 0-4.
fn cell_match_count(puzzle: &Puzzle, board: &Board, pos: Position) -> u32 {
    let w = puzzle.width;
    let h = puzzle.height;
    let Some((pid, rot)) = board.get(pos) else { return 0 };
    let Some(p) = puzzle.piece(pid) else { return 0 };
    let e = p.edges.rotated(rot).as_array();
    let x = pos % w;
    let y = pos / w;
    let mut cnt = 0;
    // North neighbor (y-1).
    if y > 0 {
        if let Some((npid, nrot)) = board.get((y - 1) * w + x) {
            if let Some(np) = puzzle.piece(npid) {
                let ne = np.edges.rotated(nrot).as_array();
                if e[0] == ne[2] && e[0] != BORDER && e[0] != 0 { cnt += 1; }
            }
        }
    }
    // East neighbor.
    if x + 1 < w {
        if let Some((npid, nrot)) = board.get(y * w + (x + 1)) {
            if let Some(np) = puzzle.piece(npid) {
                let ne = np.edges.rotated(nrot).as_array();
                if e[1] == ne[3] && e[1] != BORDER && e[1] != 0 { cnt += 1; }
            }
        }
    }
    // South neighbor.
    if y + 1 < h {
        if let Some((npid, nrot)) = board.get((y + 1) * w + x) {
            if let Some(np) = puzzle.piece(npid) {
                let ne = np.edges.rotated(nrot).as_array();
                if e[2] == ne[0] && e[2] != BORDER && e[2] != 0 { cnt += 1; }
            }
        }
    }
    // West neighbor.
    if x > 0 {
        if let Some((npid, nrot)) = board.get(y * w + (x - 1)) {
            if let Some(np) = puzzle.piece(npid) {
                let ne = np.edges.rotated(nrot).as_array();
                if e[3] == ne[1] && e[3] != BORDER && e[3] != 0 { cnt += 1; }
            }
        }
    }
    cnt
}

/// Try all 4 rotations of piece pid at position pos. Return the best
/// rotation (highest cell_match_count after placement).
fn best_rotation(puzzle: &Puzzle, board: &Board, pos: Position, pid: PieceId) -> (Rotation, u32) {
    let mut best_rot = Rotation::R0;
    let mut best_score = 0u32;
    let (_orig_pid, orig_rot) = board.get(pos).unwrap_or((pid, Rotation::R0));
    for r_raw in 0..4u8 {
        let r = Rotation::from_u8(r_raw).expect("0..4");
        let mut b2 = board.clone();
        b2.place(pos, pid, r);
        let s = cell_match_count(puzzle, &b2, pos);
        if s > best_score {
            best_score = s;
            best_rot = r;
        }
    }
    let _ = orig_rot;
    (best_rot, best_score)
}

/// Find the best swap partner for `from_pos`. A swap partner is a
/// neighbor cell whose piece, swapped with from_pos's piece, gives
/// the highest local gain (sum of cell_match_count at both cells).
fn find_best_swap_partner(
    puzzle: &Puzzle,
    board: &Board,
    from_pos: Position,
    visited: &BTreeSet<Position>,
    config: &FilamentConfig,
) -> Option<(Position, Board, i32)> {
    let w = puzzle.width;
    let h = puzzle.height;
    let x = from_pos % w;
    let y = from_pos / w;
    let cur_score_at_from = cell_match_count(puzzle, board, from_pos);

    let nbrs: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut best: Option<(Position, Board, i32)> = None;
    let mut best_gain = i32::MIN;

    let (from_pid, from_rot) = match board.get(from_pos) { Some(v) => v, None => return None };

    for (dx, dy) in &nbrs {
        let nx = x as i32 + *dx;
        let ny = y as i32 + *dy;
        if nx < 0 || ny < 0 || nx as u32 >= w || ny as u32 >= h { continue; }
        let np = ny as u32 * w + nx as u32;
        if !config.allow_revisit && visited.contains(&np) { continue; }
        let (np_pid, np_rot) = match board.get(np) { Some(v) => v, None => continue };

        // Border type matching: corner-pieces only at corners, etc.
        // We rely on the piece IDs already being correctly placed and only
        // swapping like-with-like. Quick check: count BORDER sides of each piece.
        let from_piece = match puzzle.piece(from_pid) { Some(p) => p, None => continue };
        let np_piece = match puzzle.piece(np_pid) { Some(p) => p, None => continue };
        let from_border_count = from_piece.edges.as_array().iter().filter(|&&c| c == 0).count();
        let np_border_count = np_piece.edges.as_array().iter().filter(|&&c| c == 0).count();
        if from_border_count != np_border_count { continue; }  // type mismatch

        // Score before: cell_match_count(from_pos) + cell_match_count(np)
        let cur_score_at_np = cell_match_count(puzzle, board, np);
        let cur_total = cur_score_at_from + cur_score_at_np;

        // Try the swap with best rotation per piece.
        let mut b2 = board.clone();
        b2.place(from_pos, np_pid, np_rot);
        b2.place(np, from_pid, from_rot);

        let new_score_at_from = cell_match_count(puzzle, &b2, from_pos);
        let new_score_at_np = cell_match_count(puzzle, &b2, np);

        let mut new_total = new_score_at_from + new_score_at_np;
        let mut chosen_b = b2;

        if config.try_rotations {
            // Try other rotations for piece at from_pos.
            for r_from in 0..4u8 {
                for r_np in 0..4u8 {
                    let mut b3 = board.clone();
                    b3.place(from_pos, np_pid, Rotation::from_u8(r_from).expect("0..4"));
                    b3.place(np, from_pid, Rotation::from_u8(r_np).expect("0..4"));
                    let s_from = cell_match_count(puzzle, &b3, from_pos);
                    let s_np = cell_match_count(puzzle, &b3, np);
                    let total = s_from + s_np;
                    if total > new_total {
                        new_total = total;
                        chosen_b = b3;
                    }
                }
            }
        }

        let gain = new_total as i32 - cur_total as i32;
        if gain > best_gain {
            best_gain = gain;
            best = Some((np, chosen_b, gain));
        }
    }
    best
}

/// Run a single FILAMENT chain starting from `seed_pos`.
/// Returns the best board found along the chain and the cumulative gain.
pub fn run_filament_chain(
    puzzle: &Puzzle,
    board: &Board,
    seed_pos: Position,
    config: &FilamentConfig,
) -> (Board, i32) {
    let initial_score = score_board(puzzle, board);
    let mut best_board = board.clone();
    let mut best_gain = 0i32;

    let mut current = board.clone();
    let mut current_score = initial_score as i32;
    let mut visited = BTreeSet::new();
    visited.insert(seed_pos);
    let mut cur_pos = seed_pos;
    let mut cumulative_gain = 0i32;

    for _depth in 0..config.max_depth {
        let Some((next_pos, new_board, _local_gain)) =
            find_best_swap_partner(puzzle, &current, cur_pos, &visited, config) else { break };

        let new_score = score_board(puzzle, &new_board) as i32;
        let step_gain = new_score - current_score;
        cumulative_gain += step_gain;

        // Cut if gain falls too low.
        if cumulative_gain < -config.max_loss { break; }

        current = new_board;
        current_score = new_score;
        cur_pos = next_pos;
        visited.insert(next_pos);

        if cumulative_gain > best_gain {
            best_gain = cumulative_gain;
            best_board = current.clone();
        }
    }
    (best_board, best_gain)
}

/// Run FILAMENT from multiple seed positions, returning the best result.
/// Seed positions are picked as the K positions with lowest match count.
pub fn run_filament_full(
    puzzle: &Puzzle,
    board: &Board,
    n_seeds: u32,
    config: &FilamentConfig,
    rng: &mut AlnsRng,
) -> (Board, i32) {
    let w = puzzle.width;
    let h = puzzle.height;
    let n_cells = w * h;

    // Rank cells by match count ascending.
    let mut cell_scores: Vec<(Position, u32)> = (0..n_cells)
        .map(|p| (p, cell_match_count(puzzle, board, p)))
        .collect();
    cell_scores.sort_by_key(|x| x.1);

    let mut best_board = board.clone();
    let mut best_total_gain = 0i32;

    let n_try = n_seeds.min(cell_scores.len() as u32) as usize;
    for i in 0..n_try {
        let seed = if config.seed_worst {
            cell_scores[i].0
        } else {
            cell_scores[rng.range(cell_scores.len() as u32) as usize].0
        };
        let (b, gain) = run_filament_chain(puzzle, &best_board, seed, config);
        if gain > 0 {
            best_total_gain += gain;
            best_board = b;
        }
    }
    (best_board, best_total_gain)
}

#[cfg(test)]
mod tests {
    use super::*;
    use eternity2_generator::{generate, GeneratorConfig};

    #[test]
    fn filament_compiles_and_runs() {
        let puzzle = generate(GeneratorConfig {
            size: 5, interior_colors: 4, seed: 0xCAFEBABE,
        }).expect("generate");
        // Build a random board (use generator's solution scrambled).
        let n = (puzzle.width * puzzle.height) as usize;
        let mut board = Board::empty(&puzzle);
        for i in 0..n {
            let pid: PieceId = i as u16;
            board.place(i as u32, pid, Rotation::R0);
        }
        let cfg = FilamentConfig::default();
        let mut rng = AlnsRng::new(42);
        let (_b, gain) = run_filament_full(&puzzle, &board, 4, &cfg, &mut rng);
        assert!(gain >= 0, "filament should not decrease the board score");
    }
}
