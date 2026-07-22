//! Piece-set census helpers: the structural measurements that recur across
//! research topics (orbit structure, colour budgets, piece classes), so a
//! measurement is one call instead of a re-derivation.
//!
//! Everything here is a pure function of a [`Pieces`] set (or an
//! [`Instance`]); nothing solves anything. Sides are URDL at rotation 0 and
//! [`crate::BORDER`] (grey, `0`) is the rim colour throughout.

use e2_core::{rotated, Pieces};
use e2_io::Instance;

/// The size of a piece's rotation orbit: `4` for a fully asymmetric piece,
/// `2` for a 180-degree-symmetric one, `1` for a fully rotation-symmetric one.
#[must_use]
pub fn orbit_size(edges: [u8; 4]) -> u8 {
    if rotated(edges, 1) == edges {
        1
    } else if rotated(edges, 2) == edges {
        2
    } else {
        4
    }
}

/// The lexicographically smallest rotation of `edges`: a canonical key that is
/// equal for two pieces iff one is a rotation of the other.
#[must_use]
pub fn canonical_key(edges: [u8; 4]) -> [u8; 4] {
    (0..4).map(|r| rotated(edges, r)).min().unwrap_or(edges)
}

/// Census of rotation-orbit sizes over a piece set: how many pieces have a
/// full 4-orbit, a 2-orbit, or are rotation-invariant (1-orbit).
///
/// On the official set this is `(256, 0, 0)`: zero rotation-symmetric pieces,
/// one of the puzzle's designed-in asymmetries.
#[must_use]
pub fn orbit_census(pieces: &Pieces) -> (usize, usize, usize) {
    let (mut full, mut half, mut fixed) = (0, 0, 0);
    for (_, p) in pieces.iter() {
        match orbit_size(p.rotated(0)) {
            4 => full += 1,
            2 => half += 1,
            _ => fixed += 1,
        }
    }
    (full, half, fixed)
}

/// Per-colour half-edge counts over the whole set: `census[c]` is the number
/// of piece sides showing colour `c` (all four sides of every piece counted,
/// rim colour [`crate::BORDER`] included at index 0).
///
/// The vector is sized to the largest colour present plus one. On the official
/// set the grey entry is 64 (the rim supply) and the interior entries sum to
/// 960 = 2 x 480.
#[must_use]
pub fn color_half_edge_census(pieces: &Pieces) -> Vec<u32> {
    let mut census = vec![0u32; usize::from(pieces.max_color()) + 1];
    for (_, p) in pieces.iter() {
        for &c in &p.rotated(0) {
            census[usize::from(c)] += 1;
        }
    }
    census
}

/// Piece ids partitioned by rim-side count: corners (2 grey sides), edge
/// pieces (1), interior pieces (0). On the official set: 4 / 56 / 196.
#[derive(Debug, Clone, Default)]
pub struct PieceClasses {
    pub corners: Vec<u16>,
    pub edges: Vec<u16>,
    pub interior: Vec<u16>,
}

/// Partition a piece set into corner / edge / interior classes by counting
/// [`crate::BORDER`] sides.
#[must_use]
pub fn piece_classes(pieces: &Pieces) -> PieceClasses {
    let mut out = PieceClasses::default();
    for (id, p) in pieces.iter() {
        match p.border_edge_count() {
            2 => out.corners.push(id),
            1 => out.edges.push(id),
            _ => out.interior.push(id),
        }
    }
    out
}

/// The clue cells of an instance as board positions (row-major), in hint
/// order. Geometry only; pair with `instance.hints` when the pinned piece and
/// rotation matter too.
#[must_use]
pub fn clue_cells(instance: &Instance) -> Vec<usize> {
    instance.hints.iter().map(|h| usize::from(h.pos)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::official_instance;
    use e2_core::BORDER;

    #[test]
    fn official_set_census_matches_the_known_structure() {
        let inst = official_instance(true);
        // Zero rotation-symmetric pieces: every orbit is full.
        assert_eq!(orbit_census(&inst.pieces), (256, 0, 0));
        // 4 corners, 56 edges, 196 interior.
        let classes = piece_classes(&inst.pieces);
        assert_eq!(
            (classes.corners.len(), classes.edges.len(), classes.interior.len()),
            (4, 56, 196)
        );
        // Grey supply 64; interior half-edges 960 = 2 x 480.
        let census = color_half_edge_census(&inst.pieces);
        assert_eq!(census[usize::from(BORDER)], 64);
        let interior: u32 = census.iter().skip(1).sum();
        assert_eq!(interior, 960);
        // Every interior colour has an even half-edge count (the pairing
        // argument behind the colour-multiset bound).
        for (c, &n) in census.iter().enumerate().skip(1) {
            assert_eq!(n % 2, 0, "colour {c} has odd supply {n}");
        }
        // Five official clues.
        assert_eq!(clue_cells(&inst).len(), 5);
    }

    #[test]
    fn canonical_key_identifies_rotations() {
        let e = [3, 1, 4, 1];
        for r in 0..4 {
            assert_eq!(canonical_key(rotated(e, r)), canonical_key(e));
        }
        assert_eq!(orbit_size([7, 7, 7, 7]), 1);
        assert_eq!(orbit_size([1, 2, 1, 2]), 2);
        assert_eq!(orbit_size([1, 2, 3, 4]), 4);
    }
}
