//! Path order — the sequence in which cells are filled. The study's primary
//! axis. Static orders precompute a fixed fill sequence; the dynamic order
//! (MRV) chooses the next cell from the search state at each step.
//!
//! Attribution and the measured verdicts for each order live on the study's
//! findings page; the orders known to be poor (spiral-in's closure tax,
//! hint-centric) are kept here as controls, not silently dropped.

use dfs_core::{H, N, W};

/// Which cell-visitation order a variant uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathOrder {
    /// Left-to-right, top-to-bottom. The classic baseline; constant k=2 damage
    /// zone (the "Goldilocks" order in the project's prior analysis).
    RowMajor,
    /// Left-to-right, bottom-to-top. Blackwood's scan direction.
    RowMajorBottomUp,
    /// Outer ring inward, clockwise. The "cloister" spiral; carries a closure
    /// tax at every ring's corners (known poor for the interior).
    SpiralIn,
    /// Center outward. The mirror control for SpiralIn.
    SpiralOut,
    /// All border cells first (ring), then the interior row-major. Isolates the
    /// frame, where the rare colors live.
    BorderFirst,
    /// Verhaard's COMB: a band of horizontal rows, then the remaining rows as
    /// vertical teeth. `horiz_rows` rows are filled row-major top-down; the rest
    /// column-major.
    VerhaardComb { horiz_rows: usize },
    /// Dynamic minimum-remaining-values: at each step, fill the empty cell with
    /// the fewest candidates given its placed neighbours. Border-first is used
    /// to break ties toward the frame. Chosen at search time, so this variant
    /// has no precomputed sequence.
    Mrv,
}

impl PathOrder {
    /// Is this order decided dynamically at search time (vs a fixed sequence)?
    #[must_use]
    pub const fn is_dynamic(self) -> bool {
        matches!(self, PathOrder::Mrv)
    }

    /// The fixed fill sequence for a static order: the cells to fill, in order,
    /// EXCLUDING pinned cells (`pinned[pos] == true`). Panics if called on a
    /// dynamic order.
    #[must_use]
    pub fn sequence(self, pinned: &[bool]) -> Vec<usize> {
        let raw = match self {
            PathOrder::RowMajor => row_major(),
            PathOrder::RowMajorBottomUp => row_major_bottom_up(),
            PathOrder::SpiralIn => spiral_in(),
            PathOrder::SpiralOut => spiral_out(),
            PathOrder::BorderFirst => border_first(),
            PathOrder::VerhaardComb { horiz_rows } => verhaard_comb(horiz_rows),
            PathOrder::Mrv => panic!("MRV has no precomputed sequence"),
        };
        raw.into_iter().filter(|&pos| !pinned[pos]).collect()
    }
}

#[inline]
fn pos(r: usize, c: usize) -> usize {
    r * W + c
}

fn row_major() -> Vec<usize> {
    (0..N).collect()
}

fn row_major_bottom_up() -> Vec<usize> {
    let mut v = Vec::with_capacity(N);
    for r in (0..H).rev() {
        for c in 0..W {
            v.push(pos(r, c));
        }
    }
    v
}

/// Outer ring to center, clockwise from the top-left corner.
fn spiral_in() -> Vec<usize> {
    let (mut top, mut bottom, mut left, mut right) = (0isize, H as isize - 1, 0isize, W as isize - 1);
    let mut v = Vec::with_capacity(N);
    while top <= bottom && left <= right {
        for c in left..=right {
            v.push(pos(top as usize, c as usize));
        }
        for r in (top + 1)..=bottom {
            v.push(pos(r as usize, right as usize));
        }
        if top < bottom {
            for c in (left..right).rev() {
                v.push(pos(bottom as usize, c as usize));
            }
        }
        if left < right {
            for r in ((top + 1)..bottom).rev() {
                v.push(pos(r as usize, left as usize));
            }
        }
        top += 1;
        bottom -= 1;
        left += 1;
        right -= 1;
    }
    v
}

/// Center outward: the reverse of the inward spiral.
fn spiral_out() -> Vec<usize> {
    let mut v = spiral_in();
    v.reverse();
    v
}

/// All border cells (the ring) first, in row-major order, then the interior in
/// row-major order.
fn border_first() -> Vec<usize> {
    let is_border = |r: usize, c: usize| r == 0 || r == H - 1 || c == 0 || c == W - 1;
    let mut border = Vec::new();
    let mut interior = Vec::new();
    for r in 0..H {
        for c in 0..W {
            if is_border(r, c) {
                border.push(pos(r, c));
            } else {
                interior.push(pos(r, c));
            }
        }
    }
    border.extend(interior);
    border
}

/// Verhaard's COMB fill: the top `horiz_rows` rows filled row-major, then the
/// remaining rows filled as vertical teeth (column-major over the bottom band).
fn verhaard_comb(horiz_rows: usize) -> Vec<usize> {
    let horiz_rows = horiz_rows.min(H);
    let mut v = Vec::with_capacity(N);
    for r in 0..horiz_rows {
        for c in 0..W {
            v.push(pos(r, c));
        }
    }
    for c in 0..W {
        for r in horiz_rows..H {
            v.push(pos(r, c));
        }
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_permutation(v: &[usize]) -> bool {
        let mut seen = vec![false; N];
        for &p in v {
            if p >= N || seen[p] {
                return false;
            }
            seen[p] = true;
        }
        v.len() == N
    }

    #[test]
    fn every_static_order_is_a_full_permutation() {
        let no_pins = vec![false; N];
        for order in [
            PathOrder::RowMajor,
            PathOrder::RowMajorBottomUp,
            PathOrder::SpiralIn,
            PathOrder::SpiralOut,
            PathOrder::BorderFirst,
            PathOrder::VerhaardComb { horiz_rows: 10 },
        ] {
            let seq = order.sequence(&no_pins);
            assert!(is_permutation(&seq), "{order:?} is not a permutation of all cells");
        }
    }

    #[test]
    fn pinned_cells_are_skipped() {
        let mut pinned = vec![false; N];
        pinned[0] = true;
        pinned[100] = true;
        let seq = PathOrder::RowMajor.sequence(&pinned);
        assert_eq!(seq.len(), N - 2);
        assert!(!seq.contains(&0) && !seq.contains(&100));
    }

    #[test]
    fn border_first_starts_on_the_ring_and_ends_in_the_interior() {
        let seq = PathOrder::BorderFirst.sequence(&vec![false; N]);
        // 16x16 ring = 4*16 - 4 = 60 border cells.
        assert_eq!(seq[59] < N, true);
        let interior_start = seq[60];
        let (r, c) = (interior_start / W, interior_start % W);
        assert!(r != 0 && r != H - 1 && c != 0 && c != W - 1);
    }
}
