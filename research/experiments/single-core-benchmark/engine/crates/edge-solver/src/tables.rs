// Precomputed lookup tables for cell-consistency in edge-CP.
//
// A "piece row" is a (piece_id, rotation) pair, expanded to its 4 edge
// colors in cell-frame [top, right, bottom, left]. Duplicate rotations
// (when a piece has internal symmetry) are pruned: each piece has 1-4
// canonical rows. Row ids are dense within [0, n_rows).
//
// Hot-path data: for each (side, color), a bitmask over piece rows
// telling us which rows have that color on that side. Used in
// `cell_feasible`: AND the 4 bitmasks (one per side, masked to the
// pinned color or to "any of the colors still possible on that
// side"), AND with "rows belonging to unused pieces", check non-zero.

use eternity2_core::{Color, PieceId, Puzzle, Rotation, BORDER};
#[cfg(test)]
use eternity2_core::Piece;

use crate::topology::Topology;

pub type RowId = u32;

#[derive(Debug, Clone, Copy)]
pub struct Row {
    pub piece_id: PieceId,
    pub rotation: Rotation,
    pub edges: [Color; 4],
}

#[derive(Debug, Clone)]
pub struct Tables {
    pub rows: Vec<Row>,
    pub n_rows: u32,
    pub words_per_mask: usize,
    pub n_colors: u32, // includes BORDER (color 0); user colors are 1..n_colors-1
    pub max_piece_id: u32,
    /// `piece_to_rows[pid]` is the bitmask of rows belonging to piece `pid`.
    /// Length: max_piece_id+1 entries, each `words_per_mask` u64 wide.
    pub piece_to_rows: Vec<u64>,
    /// `side_color_rows[side][color]` is a bitmask over rows of those
    /// with `edges[side] == color`. Index: `side * n_colors + color`.
    pub side_color_rows: Vec<u64>,
    /// For each cell c, a "shape mask" — the bitmask over rows whose
    /// piece-class fits this cell's boundary signature (i.e., row
    /// edges that face the puzzle boundary are BORDER, and the
    /// non-boundary sides are non-BORDER). This is a hard prerequisite
    /// for ever being placed at this cell.
    pub cell_shape_rows: Vec<u64>,
    pub n_cells: u32,
}

impl Tables {
    #[must_use]
    pub fn new(puzzle: &Puzzle, topology: &Topology) -> Self {
        let pieces = puzzle.pieces();
        let max_piece_id = pieces.iter().map(|p| u32::from(p.id)).max().unwrap_or(0);
        let n_colors = puzzle.color_count;

        // Build canonical row list, deduplicating rotations that produce
        // identical 4-tuples (a piece with rotational symmetry).
        let mut rows: Vec<Row> = Vec::new();
        for piece in pieces {
            let mut seen: Vec<[Color; 4]> = Vec::with_capacity(4);
            for r in 0..4u8 {
                let rot = Rotation::from_u8(r).unwrap();
                let edges = piece.edges.rotated(rot).as_array();
                if seen.iter().any(|s| *s == edges) {
                    continue;
                }
                seen.push(edges);
                rows.push(Row { piece_id: piece.id, rotation: rot, edges });
            }
        }
        let n_rows = rows.len() as u32;
        let words_per_mask = (n_rows as usize).div_ceil(64).max(1);

        // piece_to_rows
        let mut piece_to_rows = vec![0u64; (max_piece_id as usize + 1) * words_per_mask];
        for (rid, row) in rows.iter().enumerate() {
            let pid = u32::from(row.piece_id) as usize;
            piece_to_rows[pid * words_per_mask + rid / 64] |= 1u64 << (rid % 64);
        }

        // side_color_rows. Side ∈ {0..4}, color ∈ {0..n_colors}.
        let stride = n_colors as usize;
        let mut side_color_rows = vec![0u64; 4 * stride * words_per_mask];
        for (rid, row) in rows.iter().enumerate() {
            for side in 0..4 {
                let c = row.edges[side] as usize;
                let base = (side * stride + c) * words_per_mask;
                side_color_rows[base + rid / 64] |= 1u64 << (rid % 64);
            }
        }

        // cell_shape_rows: rows whose BORDER sides exactly match the cell's
        // boundary signature.
        let mut cell_shape_rows = vec![0u64; topology.n_cells as usize * words_per_mask];
        for c in 0..topology.n_cells as usize {
            let sides = topology.cell_sides[c];
            let need_border = [
                sides[0].is_none(),
                sides[1].is_none(),
                sides[2].is_none(),
                sides[3].is_none(),
            ];
            for (rid, row) in rows.iter().enumerate() {
                let edges = row.edges;
                let row_border = [
                    edges[0] == BORDER,
                    edges[1] == BORDER,
                    edges[2] == BORDER,
                    edges[3] == BORDER,
                ];
                if row_border == need_border {
                    cell_shape_rows[c * words_per_mask + rid / 64] |= 1u64 << (rid % 64);
                }
            }
        }

        Self {
            rows,
            n_rows,
            words_per_mask,
            n_colors,
            max_piece_id,
            piece_to_rows,
            side_color_rows,
            cell_shape_rows,
            n_cells: topology.n_cells,
        }
    }

    #[inline]
    pub fn piece_mask(&self, pid: PieceId) -> &[u64] {
        let i = usize::from(pid) * self.words_per_mask;
        &self.piece_to_rows[i..i + self.words_per_mask]
    }

    #[inline]
    pub fn side_color_mask(&self, side: usize, color: Color) -> &[u64] {
        let base = ((side * self.n_colors as usize) + color as usize) * self.words_per_mask;
        &self.side_color_rows[base..base + self.words_per_mask]
    }

    #[inline]
    pub fn cell_shape_mask(&self, cell: u32) -> &[u64] {
        let base = cell as usize * self.words_per_mask;
        &self.cell_shape_rows[base..base + self.words_per_mask]
    }
}

/// Helpers for bitmask manipulation over `n_rows` rows.
#[inline]
pub fn mask_and_into(dst: &mut [u64], src: &[u64]) {
    debug_assert_eq!(dst.len(), src.len());
    for i in 0..dst.len() {
        dst[i] &= src[i];
    }
}

#[inline]
pub fn mask_or_into(dst: &mut [u64], src: &[u64]) {
    debug_assert_eq!(dst.len(), src.len());
    for i in 0..dst.len() {
        dst[i] |= src[i];
    }
}

#[inline]
#[must_use]
pub fn mask_any(m: &[u64]) -> bool {
    m.iter().any(|w| *w != 0)
}

#[inline]
#[must_use]
pub fn mask_popcount(m: &[u64]) -> u32 {
    m.iter().map(|w| w.count_ones()).sum()
}

#[inline]
pub fn mask_copy(dst: &mut [u64], src: &[u64]) {
    debug_assert_eq!(dst.len(), src.len());
    dst.copy_from_slice(src);
}

#[inline]
pub fn mask_fill_zero(dst: &mut [u64]) {
    for w in dst.iter_mut() {
        *w = 0;
    }
}

/// Iterate set bit indices in `mask` (row ids), calling `f` for each.
pub fn mask_for_each<F: FnMut(RowId)>(mask: &[u64], mut f: F) {
    for (w_idx, &word) in mask.iter().enumerate() {
        let mut w = word;
        while w != 0 {
            let bit = w.trailing_zeros();
            f((w_idx as u32) * 64 + bit);
            w &= w - 1;
        }
    }
}

/// Pick the lowest-id row in mask, or None if empty.
#[must_use]
pub fn mask_first(mask: &[u64]) -> Option<RowId> {
    for (w_idx, &word) in mask.iter().enumerate() {
        if word != 0 {
            return Some((w_idx as u32) * 64 + word.trailing_zeros());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use eternity2_core::Edges;

    fn small_puzzle() -> Puzzle {
        // 2×2 with simple pieces.
        let pieces = vec![
            Piece::new(0, Edges::new(BORDER, 1, 1, BORDER)),
            Piece::new(1, Edges::new(BORDER, BORDER, 2, 1)),
            Piece::new(2, Edges::new(1, 2, BORDER, BORDER)),
            Piece::new(3, Edges::new(2, BORDER, BORDER, 2)),
        ];
        Puzzle::new(2, 2, 3, pieces).unwrap()
    }

    #[test]
    fn rows_dedup_symmetries() {
        // A piece with edges (1,1,1,1) has only 1 canonical row.
        let pieces = vec![
            Piece::new(0, Edges::new(1, 1, 1, 1)),
            Piece::new(1, Edges::new(1, 1, 1, 1)),
            Piece::new(2, Edges::new(1, 1, 1, 1)),
            Piece::new(3, Edges::new(1, 1, 1, 1)),
        ];
        let puzzle = Puzzle::new(2, 2, 2, pieces).unwrap();
        let topo = Topology::new(&puzzle);
        let t = Tables::new(&puzzle, &topo);
        assert_eq!(t.n_rows, 4);
    }

    #[test]
    fn corner_pieces_only_match_corner_cells() {
        let puzzle = small_puzzle();
        let topo = Topology::new(&puzzle);
        let t = Tables::new(&puzzle, &topo);
        // Cell 0 is top-left corner (top+left are BORDER).
        let cell_mask = t.cell_shape_mask(0);
        // Some row must satisfy this shape (the puzzle is valid).
        assert!(mask_any(cell_mask));
    }
}
