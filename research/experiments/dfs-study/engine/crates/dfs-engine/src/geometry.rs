//! Precomputed piece geometry: the fast lookup tables the hot loop reads.
//!
//! The dominant query in a border-first / row-major DFS is "which oriented
//! pieces have up-edge `u` and left-edge `l`?" — because those two neighbours
//! are already placed. We answer it in O(1) with a two-sided index. For path
//! orders where other neighbours are constrained, the search falls back to a
//! direct four-edge match over a single-sided candidate list.

use dfs_core::{Pieces, Rot};

/// An oriented piece: a piece id at a specific rotation, with its resolved URDL
/// edges cached so the hot loop never recomputes a rotation.
#[derive(Debug, Clone, Copy)]
pub struct Oriented {
    pub piece: u16,
    pub rot: Rot,
    /// URDL edges at this rotation.
    pub edges: [u8; 4],
}

/// Precomputed lookup tables over the piece set. Built once per run.
pub struct Geometry {
    /// Number of distinct colors including border (0), i.e. `max_color + 1`.
    pub colors: usize,
    /// Every oriented piece (piece × 4 rotations), deduplicated for symmetric
    /// pieces so a piece with rotational symmetry is not tried four times.
    pub oriented: Vec<Oriented>,
    /// `by_up_left[u * colors + l]` = indices into `oriented` whose up-edge is
    /// `u` and left-edge is `l`. The row-major / border-first hot path.
    by_up_left: Vec<Vec<u32>>,
    /// `by_up[u]` = indices whose up-edge is `u` (used when only the up
    /// neighbour is fixed, e.g. the left column under row-major).
    by_up: Vec<Vec<u32>>,
    /// All oriented indices (used when no neighbour is fixed, e.g. cell 0).
    all: Vec<u32>,
}

impl Geometry {
    #[must_use]
    pub fn build(pieces: &Pieces) -> Self {
        let colors = pieces.max_color() as usize + 1;
        let mut oriented: Vec<Oriented> = Vec::with_capacity(pieces.len() * 4);
        for (piece, p) in pieces.iter() {
            // Deduplicate rotations that map a piece onto itself, so symmetric
            // pieces do not inflate the branching factor.
            let mut seen: Vec<[u8; 4]> = Vec::with_capacity(4);
            for rot in 0..4u8 {
                let edges = p.rotated(rot);
                if seen.contains(&edges) {
                    continue;
                }
                seen.push(edges);
                oriented.push(Oriented { piece, rot, edges });
            }
        }

        let mut by_up_left = vec![Vec::new(); colors * colors];
        let mut by_up = vec![Vec::new(); colors];
        let mut all = Vec::with_capacity(oriented.len());
        for (i, o) in oriented.iter().enumerate() {
            let i = i as u32;
            let u = o.edges[0] as usize;
            let l = o.edges[3] as usize;
            by_up_left[u * colors + l].push(i);
            by_up[u].push(i);
            all.push(i);
        }

        Self {
            colors,
            oriented,
            by_up_left,
            by_up,
            all,
        }
    }

    #[inline]
    #[must_use]
    pub fn oriented(&self, idx: u32) -> Oriented {
        self.oriented[idx as usize]
    }

    /// Candidates with up-edge `u` and left-edge `l`.
    #[inline]
    #[must_use]
    pub fn by_up_left(&self, u: u8, l: u8) -> &[u32] {
        &self.by_up_left[u as usize * self.colors + l as usize]
    }

    /// Candidates with up-edge `u` (any left).
    #[inline]
    #[must_use]
    pub fn by_up(&self, u: u8) -> &[u32] {
        &self.by_up[u as usize]
    }

    /// All oriented pieces.
    #[inline]
    #[must_use]
    pub fn all(&self) -> &[u32] {
        &self.all
    }

    /// Dispatch to the tightest candidate list given which of the up/left
    /// neighbours are constrained (`NO_CONSTRAINT` = 255 means unconstrained).
    /// Both constrained → two-sided index; only up → up index; neither → all.
    #[inline]
    #[must_use]
    pub fn by_up_left_or(&self, u: u8, l: u8) -> &[u32] {
        match (u == 255, l == 255) {
            (false, false) => self.by_up_left(u, l),
            (false, true) => self.by_up(u),
            _ => &self.all,
        }
    }
}
