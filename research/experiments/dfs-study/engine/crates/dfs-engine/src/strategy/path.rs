//! Path order — the sequence in which cells are filled. The study's primary
//! axis. Static orders precompute a fixed fill sequence; the dynamic order
//! (MRV) chooses the next cell from the search state at each step.
//!
//! Attribution and the measured verdicts for each order live on the study's
//! findings page; the orders known to be poor (spiral-in's closure tax,
//! hint-centric) are kept here as controls, not silently dropped.

use e2_core::{H, N, W};

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
    /// Row-major, but the rows that contain the official Eternity II clues (rows
    /// 2, 8, 13) are swept FIRST, then the remaining rows top-down. The intent
    /// was a hybrid of "reach the pinned clues ASAP" and "keep the frontier
    /// compact". MEASURED VERDICT: poor, kept as a control. Sweeping non-adjacent
    /// rows first fragments the fill into disconnected horizontal strips — three
    /// open frontiers instead of one — and a backtracker's branching is
    /// exponential in the open frontier, so it plateaus far below plain RowMajor
    /// (≈60/480 vs RowMajor's 370+ on 5-clue boards). It demonstrates that
    /// reaching the hints early cannot beat a single compact sweep: frontier
    /// compactness dominates hint-proximity.
    ClueRowsFirst,
    /// **Hint-seeking / "connect the hints first"** — a deliberately hint-centric
    /// order for the Hint Study. Multi-source BFS outward from every pinned cell
    /// simultaneously, so the fill grows blobs around each hint and tries to
    /// *connect* them before completing the board. The intuition ("get to the
    /// clues, link them up") is seductive and WRONG: seeding from k scattered
    /// anchors opens k separate frontiers at once, and a backtracker's branching
    /// is exponential in the open frontier, so this fragments far worse than a
    /// single compact sweep. Kept as the study's headline negative control. Falls
    /// back to row-major when there are no hints. Hint-aware: needs the pin
    /// positions, so it is filled via `sequence_with_hints`.
    ConnectHintsFirst,
    /// **Trace the hints** — a shorter, more deliberate hint-seeking order for the
    /// Hint Study. Rather than flooding outward from every hint at once
    /// ([`ConnectHintsFirst`]), it first draws a thin *skeleton* between the pins:
    /// straight lines joining the four outer hints into a square, then diagonals
    /// from the square's corners in to the central hint. Only then does it fill the
    /// rest, growing outward from that skeleton so each new cell abuts an
    /// already-placed one (constraint-greedy). The idea is that a sparse connecting
    /// skeleton "locks in" the hint relationships early; the study measures whether
    /// that helps a backtracker (it does not — the skeleton is a scattered set of
    /// commitments with no way to check consistency until much later). Falls back to
    /// row-major with no hints. Hint-aware: derives the pins from `pinned`.
    TraceHints,
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
            PathOrder::ClueRowsFirst => clue_rows_first(),
            // Hint-aware: BFS outward from the pinned cells. Derives the anchor
            // positions from `pinned` itself, so no extra argument is needed.
            PathOrder::ConnectHintsFirst => connect_hints_first(pinned),
            PathOrder::TraceHints => trace_hints(pinned),
            PathOrder::Mrv => panic!("MRV has no precomputed sequence"),
        };
        raw.into_iter().filter(|&pos| !pinned[pos]).collect()
    }
}

/// Multi-source BFS outward from every pinned cell at once: the fill grows a
/// blob around each hint and links them up, instead of sweeping the board in one
/// pass. With no hints this is just row-major. The returned order is a full
/// permutation of all cells (pinned cells are filtered out by the caller). This
/// is the study's headline *negative* control — see the `ConnectHintsFirst`
/// doc-comment for why it fragments the frontier.
fn connect_hints_first(pinned: &[bool]) -> Vec<usize> {
    let anchors: Vec<usize> = (0..N).filter(|&p| pinned[p]).collect();
    if anchors.is_empty() {
        return row_major();
    }
    let mut order = Vec::with_capacity(N);
    let mut seen = vec![false; N];
    // Seed the BFS queue with all anchors (they are emitted then filtered out by
    // the caller, but seed the frontier so their neighbours come next).
    let mut frontier: std::collections::VecDeque<usize> = std::collections::VecDeque::new();
    for &a in &anchors {
        seen[a] = true;
        order.push(a);
        frontier.push_back(a);
    }
    while let Some(cell) = frontier.pop_front() {
        let (r, c) = (cell / W, cell % W);
        // 4-neighbourhood, deterministic order (up, left, right, down).
        let mut nbrs: Vec<usize> = Vec::with_capacity(4);
        if r > 0 { nbrs.push(pos(r - 1, c)); }
        if c > 0 { nbrs.push(pos(r, c - 1)); }
        if c + 1 < W { nbrs.push(pos(r, c + 1)); }
        if r + 1 < H { nbrs.push(pos(r + 1, c)); }
        for nb in nbrs {
            if !seen[nb] {
                seen[nb] = true;
                order.push(nb);
                frontier.push_back(nb);
            }
        }
    }
    // Any cell not reached (disconnected — cannot happen on a grid, but be safe).
    for p in 0..N {
        if !seen[p] {
            order.push(p);
        }
    }
    order
}

/// "Trace the hints": draw a thin skeleton joining the pins (square between the
/// four outer hints, then diagonals in to the centre hint), then fill the rest by
/// growing outward from that skeleton so each new cell abuts a placed one. With
/// fewer than 5 hints, or none, falls back to `connect_hints_first` /
/// `row_major`. See the `TraceHints` doc-comment.
fn trace_hints(pinned: &[bool]) -> Vec<usize> {
    let anchors: Vec<usize> = (0..N).filter(|&p| pinned[p]).collect();
    if anchors.is_empty() {
        return row_major();
    }
    if anchors.len() < 5 {
        return connect_hints_first(pinned);
    }

    // Identify the four outer hints (corners of the bounding box) and the centre
    // hint (closest to the board centre). The clue shape is 4 near-corner + 1
    // central, so this recovers them robustly.
    let rc = |cell: usize| (cell / W, cell % W);
    let centre_cell = *anchors
        .iter()
        .min_by_key(|&&cell| {
            let (r, c) = rc(cell);
            let dr = r as isize - (H as isize) / 2;
            let dc = c as isize - (W as isize) / 2;
            dr * dr + dc * dc
        })
        .unwrap();
    let mut outer: Vec<usize> = anchors.iter().copied().filter(|&c| c != centre_cell).collect();
    // Order the outer hints clockwise (TL, TR, BR, BL) so the square edges connect
    // adjacent corners.
    outer.sort_by_key(|&cell| {
        let (r, c) = rc(cell);
        // angle bucket around the centre: TL, TR, BR, BL
        let top = r < H / 2;
        let left = c < W / 2;
        match (top, left) {
            (true, true) => 0,
            (true, false) => 1,
            (false, false) => 2,
            (false, true) => 3,
        }
    });

    let mut order = Vec::with_capacity(N);
    let mut seen = vec![false; N];
    let mut push = |cell: usize, order: &mut Vec<usize>, seen: &mut [bool]| {
        if !seen[cell] {
            seen[cell] = true;
            order.push(cell);
        }
    };

    // Straight line between two cells (Bresenham on the grid), pushed in order.
    let line = |a: usize, b: usize, order: &mut Vec<usize>, seen: &mut [bool]| {
        let (mut r0, mut c0) = (rc(a).0 as isize, rc(a).1 as isize);
        let (r1, c1) = (rc(b).0 as isize, rc(b).1 as isize);
        let dr = (r1 - r0).abs();
        let dc = (c1 - c0).abs();
        let sr = if r0 < r1 { 1 } else { -1 };
        let sc = if c0 < c1 { 1 } else { -1 };
        let mut err = dc - dr;
        loop {
            push((r0 as usize) * W + c0 as usize, order, seen);
            if r0 == r1 && c0 == c1 {
                break;
            }
            let e2 = 2 * err;
            if e2 > -dr {
                err -= dr;
                c0 += sc;
            }
            if e2 < dc {
                err += dc;
                r0 += sr;
            }
        }
    };

    // 1) The square: connect consecutive outer hints (and close the loop).
    for i in 0..outer.len() {
        line(outer[i], outer[(i + 1) % outer.len()], &mut order, &mut seen);
    }
    // 2) Diagonals: each outer corner in to the centre hint.
    for &o in &outer {
        line(o, centre_cell, &mut order, &mut seen);
    }

    // 3) Constraint-greedy fill: BFS outward from the skeleton, so every newly
    // filled cell touches an already-placed one.
    let mut frontier: std::collections::VecDeque<usize> =
        order.iter().copied().collect();
    while let Some(cell) = frontier.pop_front() {
        let (r, c) = rc(cell);
        let mut nbrs: Vec<usize> = Vec::with_capacity(4);
        if r > 0 { nbrs.push(pos(r - 1, c)); }
        if c > 0 { nbrs.push(pos(r, c - 1)); }
        if c + 1 < W { nbrs.push(pos(r, c + 1)); }
        if r + 1 < H { nbrs.push(pos(r + 1, c)); }
        for nb in nbrs {
            if !seen[nb] {
                seen[nb] = true;
                order.push(nb);
                frontier.push_back(nb);
            }
        }
    }
    for p in 0..N {
        if !seen[p] {
            order.push(p);
        }
    }
    order
}

#[inline]
fn pos(r: usize, c: usize) -> usize {
    r * W + c
}

fn row_major() -> Vec<usize> {
    (0..N).collect()
}

/// Row-major, but the official-E2 clue rows (2, 8, 13) are swept first, then the
/// rest top-down. Same tight one-row frontier as row-major; the clue rows come
/// first so any pinned clue constrains the search from the opening. Rows are
/// still filled left-to-right, so the frontier never fragments the way an
/// anchor-seeded flood would.
fn clue_rows_first() -> Vec<usize> {
    let clue_rows = [2usize, 8, 13];
    let mut v = Vec::with_capacity(N);
    for &r in &clue_rows {
        for c in 0..W {
            v.push(pos(r, c));
        }
    }
    for r in 0..H {
        if clue_rows.contains(&r) {
            continue;
        }
        for c in 0..W {
            v.push(pos(r, c));
        }
    }
    v
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
