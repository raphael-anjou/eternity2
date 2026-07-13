// Edge-as-variable topology for a W×H board.
//
// Interior edges only: the W·(H-1) horizontal edges between vertically
// adjacent cells, plus the (W-1)·H vertical edges between horizontally
// adjacent cells. Boundary sides (cells touching the outer perimeter)
// are not variables — they're forced to BORDER.
//
// Indexing convention:
//   h_edge(x, y) for y ∈ [0, H-1):  edge between (x, y) and (x, y+1)
//     linear index: y * W + x,   range [0, W·(H-1))
//   v_edge(x, y) for x ∈ [0, W-1):  edge between (x, y) and (x+1, y)
//     linear index: W·(H-1) + y · (W-1) + x,   range [W·(H-1), N_EDGES)
//
// Edge-to-cell adjacency:
//   h_edge(x, y): "above" cell (x, y), "below" cell (x, y+1).
//     For the upper cell, this is its BOTTOM side. For the lower cell, its TOP.
//   v_edge(x, y): "left" cell (x, y), "right" cell (x+1, y).
//     For the left cell, this is its RIGHT side. For the right cell, its LEFT.

use eternity2_core::Puzzle;

pub type EdgeId = u32;
pub type CellId = u32;

/// Side of a cell. Order matches Edges' [top, right, bottom, left].
pub const SIDE_TOP: usize = 0;
pub const SIDE_RIGHT: usize = 1;
pub const SIDE_BOTTOM: usize = 2;
pub const SIDE_LEFT: usize = 3;

#[derive(Debug, Clone)]
pub struct Topology {
    pub width: u32,
    pub height: u32,
    pub n_horiz: u32,
    pub n_vert: u32,
    pub n_edges: u32,
    pub n_cells: u32,
    /// For each cell c (row-major), the 4 sides' edge references.
    /// `None` means that side is on the puzzle boundary (BORDER-fixed).
    /// Side order matches Edges: [top, right, bottom, left].
    pub cell_sides: Vec<[Option<EdgeId>; 4]>,
    /// For each edge, the two cells (in any order) sharing it, with
    /// the side index pointing back from each cell.
    pub edge_cells: Vec<[(CellId, usize); 2]>,
}

impl Topology {
    #[must_use]
    pub fn new(puzzle: &Puzzle) -> Self {
        let w = puzzle.width;
        let h = puzzle.height;
        let n_horiz = w * (h.saturating_sub(1));
        let n_vert = (w.saturating_sub(1)) * h;
        let n_edges = n_horiz + n_vert;
        let n_cells = w * h;

        let h_id = |x: u32, y: u32| -> EdgeId { y * w + x };
        let v_id = |x: u32, y: u32| -> EdgeId { n_horiz + y * w.saturating_sub(1) + x };

        let mut cell_sides = vec![[None; 4]; n_cells as usize];
        let mut edge_cells = vec![[(0u32, 0usize); 2]; n_edges as usize];

        for y in 0..h {
            for x in 0..w {
                let c = (y * w + x) as usize;
                // top side: h_edge(x, y-1), this cell is the lower (its TOP).
                if y > 0 {
                    cell_sides[c][SIDE_TOP] = Some(h_id(x, y - 1));
                }
                // bottom side: h_edge(x, y), this cell is upper (its BOTTOM).
                if y + 1 < h {
                    cell_sides[c][SIDE_BOTTOM] = Some(h_id(x, y));
                }
                // left side: v_edge(x-1, y), this cell is the right one (its LEFT).
                if x > 0 {
                    cell_sides[c][SIDE_LEFT] = Some(v_id(x - 1, y));
                }
                // right side: v_edge(x, y), this cell is the left one (its RIGHT).
                if x + 1 < w {
                    cell_sides[c][SIDE_RIGHT] = Some(v_id(x, y));
                }
            }
        }

        for y in 0..h.saturating_sub(1) {
            for x in 0..w {
                let e = h_id(x, y) as usize;
                let upper = y * w + x;
                let lower = (y + 1) * w + x;
                edge_cells[e] = [(upper, SIDE_BOTTOM), (lower, SIDE_TOP)];
            }
        }
        for y in 0..h {
            for x in 0..w.saturating_sub(1) {
                let e = v_id(x, y) as usize;
                let left_cell = y * w + x;
                let right_cell = y * w + x + 1;
                edge_cells[e] = [(left_cell, SIDE_RIGHT), (right_cell, SIDE_LEFT)];
            }
        }

        Self {
            width: w,
            height: h,
            n_horiz,
            n_vert,
            n_edges,
            n_cells,
            cell_sides,
            edge_cells,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eternity2_core::{Edges, Piece, Puzzle};

    fn dummy_puzzle(w: u32, h: u32) -> Puzzle {
        let pieces: Vec<Piece> = (0..(w * h) as u16)
            .map(|i| Piece::new(i, Edges::new(1, 1, 1, 1)))
            .collect();
        Puzzle::new(w, h, 2, pieces).unwrap()
    }

    #[test]
    fn counts_3x3() {
        let p = dummy_puzzle(3, 3);
        let t = Topology::new(&p);
        assert_eq!(t.n_horiz, 6);
        assert_eq!(t.n_vert, 6);
        assert_eq!(t.n_edges, 12);
        assert_eq!(t.n_cells, 9);
    }

    #[test]
    fn counts_16x16() {
        let p = dummy_puzzle(16, 16);
        let t = Topology::new(&p);
        assert_eq!(t.n_horiz, 16 * 15);
        assert_eq!(t.n_vert, 15 * 16);
        assert_eq!(t.n_edges, 480);
    }

    #[test]
    fn corner_cell_only_has_2_interior_sides() {
        let p = dummy_puzzle(3, 3);
        let t = Topology::new(&p);
        // top-left corner: cell (0,0) -> sides top=None, left=None, right=Some, bottom=Some
        let s = t.cell_sides[0];
        assert!(s[SIDE_TOP].is_none());
        assert!(s[SIDE_LEFT].is_none());
        assert!(s[SIDE_RIGHT].is_some());
        assert!(s[SIDE_BOTTOM].is_some());
    }

    #[test]
    fn edge_adjacency_reciprocal() {
        let p = dummy_puzzle(4, 4);
        let t = Topology::new(&p);
        for e in 0..t.n_edges as usize {
            let [(c0, side0), (c1, side1)] = t.edge_cells[e];
            assert_eq!(t.cell_sides[c0 as usize][side0], Some(e as EdgeId));
            assert_eq!(t.cell_sides[c1 as usize][side1], Some(e as EdgeId));
        }
    }
}
