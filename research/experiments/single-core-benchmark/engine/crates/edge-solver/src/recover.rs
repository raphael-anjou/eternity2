// Recover a Board from an edge-color assignment.
//
// For each cell, if all 4 surrounding edges are determined (boundary
// sides are fixed BORDER, interior sides are pinned by the assignment),
// look up the (piece, rotation) row whose 4-tuple matches.
//
// Piece-alldiff: solve the bipartite matching (cells × pieces) with
// edges = "this piece can satisfy this cell." Maximum matching gives
// the most pieces placeable. Augmenting-path algorithm — O(V·E) which
// for E2 is well under a millisecond.
//
// Cells with any unassigned interior edge are left empty in the
// recovered Board — they're the "broken" cells the partial doesn't cover.

use eternity2_core::{Board, Color, PieceId, Position, Rotation, BORDER};

use crate::tables::Tables;
use crate::topology::Topology;

/// Diagnostic counts about an edge-color assignment.
#[derive(Debug, Clone, Copy)]
pub struct RecoveryStats {
    pub cells_fully_determined: u32,
    pub cells_with_any_candidates: u32,
    pub cells_with_zero_candidates: u32,
    pub total_candidate_count: u64,
    pub max_candidate_count: u32,
    pub cells_placed: u32,
}

/// Convert an edge-color assignment into a Board with maximum piece
/// placements respecting the alldiff. Solves the bipartite matching
/// (fully-determined-cells × pieces) via augmenting paths, then
/// records one rotation per matched piece-cell pair.
#[must_use]
pub fn recover_board(
    puzzle: &eternity2_core::Puzzle,
    topology: &Topology,
    tables: &Tables,
    edge_color: &[Option<Color>],
) -> Board {
    recover_board_with_stats(puzzle, topology, tables, edge_color).0
}

#[must_use]
pub fn recover_board_with_stats(
    puzzle: &eternity2_core::Puzzle,
    topology: &Topology,
    tables: &Tables,
    edge_color: &[Option<Color>],
) -> (Board, RecoveryStats) {
    let mut board = Board::empty(puzzle);
    let mut stats = RecoveryStats {
        cells_fully_determined: 0,
        cells_with_any_candidates: 0,
        cells_with_zero_candidates: 0,
        total_candidate_count: 0,
        max_candidate_count: 0,
        cells_placed: 0,
    };

    // Per-cell candidate (piece, rotation) lists. Cells with any
    // unassigned interior edge are skipped (no entry).
    let mut candidates_per_cell: Vec<(u32, Vec<(PieceId, Rotation)>)> =
        Vec::with_capacity(topology.n_cells as usize);
    for c in 0..topology.n_cells {
        let sides = topology.cell_sides[c as usize];
        let mut edges_known: [Color; 4] = [BORDER; 4];
        let mut all_known = true;
        for s in 0..4 {
            edges_known[s] = match sides[s] {
                None => BORDER,
                Some(e) => match edge_color[e as usize] {
                    Some(k) => k,
                    None => { all_known = false; break; }
                },
            };
        }
        if !all_known {
            continue;
        }
        let mut matches: Vec<(PieceId, Rotation)> = Vec::new();
        for row in &tables.rows {
            if row.edges == edges_known {
                matches.push((row.piece_id, row.rotation));
            }
        }
        stats.cells_fully_determined += 1;
        let n_cand = matches.len() as u32;
        if n_cand == 0 {
            stats.cells_with_zero_candidates += 1;
        } else {
            stats.cells_with_any_candidates += 1;
            stats.total_candidate_count += u64::from(n_cand);
            if n_cand > stats.max_candidate_count {
                stats.max_candidate_count = n_cand;
            }
            candidates_per_cell.push((c, matches));
        }
    }

    // Bipartite max-matching via Hopcroft-Karp-style augmenting paths.
    // Left = candidate cells (indexed 0..n_left). Right = pieces.
    // Edges = "piece can satisfy cell." Track piece→cell mapping.
    let n_left = candidates_per_cell.len();
    let n_pieces = tables.max_piece_id as usize + 1;
    let mut piece_to_left: Vec<Option<usize>> = vec![None; n_pieces];

    // For each left node, augmenting-path DFS.
    fn try_augment(
        left: usize,
        candidates_per_cell: &[(u32, Vec<(PieceId, Rotation)>)],
        piece_to_left: &mut Vec<Option<usize>>,
        visited: &mut Vec<bool>,
    ) -> bool {
        for (pid, _rot) in &candidates_per_cell[left].1 {
            let p_idx = u32::from(*pid) as usize;
            if visited[p_idx] {
                continue;
            }
            visited[p_idx] = true;
            let free = match piece_to_left[p_idx] {
                None => true,
                Some(other_left) => try_augment(other_left, candidates_per_cell, piece_to_left, visited),
            };
            if free {
                piece_to_left[p_idx] = Some(left);
                return true;
            }
        }
        false
    }

    for left in 0..n_left {
        let mut visited = vec![false; n_pieces];
        let _ = try_augment(left, &candidates_per_cell, &mut piece_to_left, &mut visited);
    }

    // Build inverse: cell → (piece, rotation). For each matched piece,
    // pick the first rotation that fits (there may be multiple; they
    // produce the same 4-tuple since rotations were dedup'd at row build).
    for (p_idx, maybe_left) in piece_to_left.iter().enumerate() {
        let Some(left) = maybe_left else { continue; };
        let (cell, candidates) = &candidates_per_cell[*left];
        for (pid, rot) in candidates {
            if u32::from(*pid) as usize == p_idx {
                board.place(*cell as Position, *pid, *rot);
                stats.cells_placed += 1;
                break;
            }
        }
    }

    (board, stats)
}

#[cfg(test)]
mod tests {
    use super::*;
    use eternity2_core::{Edges, Piece, Puzzle};

    #[test]
    fn fully_determined_2x2_yields_complete_board() {
        let pieces = vec![
            Piece::new(0, Edges::new(BORDER, 1, 1, BORDER)),
            Piece::new(1, Edges::new(BORDER, BORDER, 1, 1)),
            Piece::new(2, Edges::new(1, 1, BORDER, BORDER)),
            Piece::new(3, Edges::new(1, BORDER, BORDER, 1)),
        ];
        let puzzle = Puzzle::new(2, 2, 2, pieces).unwrap();
        let topo = Topology::new(&puzzle);
        let tables = Tables::new(&puzzle, &topo);
        // 2×2: 4 interior edges (right of cell 0, down of cell 0, ...)
        // We set them all to color 1.
        let n_edges = topo.n_edges as usize;
        let edge_color = vec![Some(1u8); n_edges];
        let board = recover_board(&puzzle, &topo, &tables, &edge_color);
        // All 4 cells should be filled.
        for c in 0..4 {
            assert!(board.get(c).is_some(), "cell {c} not placed");
        }
    }
}
