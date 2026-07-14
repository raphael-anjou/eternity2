// Pure geometric path-skeleton builders for the engine.
//
// All functions here are size-of-board pure: they take a `Puzzle` (for
// width/height/cell_count + xy mapping) and optionally a `Hints`
// reference, and return a `Vec<Position>` ordering. No domains, no
// propagator state, no `SearchState`.
//
// Moved here in vol-33 T3 (split solver-engine/src/lib.rs into modules).

use eternity2_core::{Hints, Position, Puzzle};

/// Build a "chess-class" rank ordering: corners → border → interior-black
/// (parity-0, spiraling from centre) → interior-white (parity-1, spiraling
/// from centre). Used as a deterministic tiebreaker when MRV ties happen.
pub(crate) fn compute_chess_rank(puzzle: &Puzzle) -> Vec<u32> {
    let n = puzzle.cell_count() as usize;
    let w = puzzle.width;
    let h = puzzle.height;
    let mut rank = vec![u32::MAX; n];
    let mut next_rank: u32 = 0;

    // Class 1: corners.
    let corners: [(u32, u32); 4] = [(0, 0), (w - 1, 0), (0, h - 1), (w - 1, h - 1)];
    let mut seen = vec![false; n];
    for (x, y) in corners {
        let p = y * w + x;
        if !seen[p as usize] {
            rank[p as usize] = next_rank;
            seen[p as usize] = true;
            next_rank += 1;
        }
    }

    // Class 2: border (non-corner) cells in row-major order. Keeps the
    // ordering deterministic and follows the perimeter naturally.
    for pos in 0..puzzle.cell_count() {
        if seen[pos as usize] {
            continue;
        }
        let mask = puzzle.border_mask(pos);
        if mask.iter().any(|m| *m) {
            rank[pos as usize] = next_rank;
            seen[pos as usize] = true;
            next_rank += 1;
        }
    }

    // Center coordinates for Chebyshev-distance spiral.
    let cx = (w - 1) as i32;
    let cy = (h - 1) as i32;
    let center = (cx as f32 / 2.0, cy as f32 / 2.0);

    let mut interior: Vec<(u32, u32, u32)> = Vec::new(); // (pos, parity, dist_metric)
    for pos in 0..puzzle.cell_count() {
        if seen[pos as usize] {
            continue;
        }
        let (x, y) = puzzle.xy(pos);
        let dx = (x as f32 - center.0).abs();
        let dy = (y as f32 - center.1).abs();
        // Use 1000 * Chebyshev + row-major as deterministic tiebreaker.
        let cheb = (dx.max(dy) * 1000.0) as u32;
        let parity = (x + y) & 1;
        interior.push((pos, parity, cheb * 10_000 + pos));
    }

    // Class 3: parity-0 ("black") spiraling from center.
    let mut blacks: Vec<_> = interior.iter().filter(|(_, par, _)| *par == 0).collect();
    blacks.sort_by_key(|(_, _, m)| *m);
    for (pos, _, _) in blacks {
        rank[*pos as usize] = next_rank;
        next_rank += 1;
    }
    // Class 4: parity-1 ("white") spiraling from center.
    let mut whites: Vec<_> = interior.iter().filter(|(_, par, _)| *par == 1).collect();
    whites.sort_by_key(|(_, _, m)| *m);
    for (pos, _, _) in whites {
        rank[*pos as usize] = next_rank;
        next_rank += 1;
    }
    rank
}

/// Build a "rectangle skeleton" path from hint positions. Vol-14 user-
/// proposed heuristic for canonical E2 (works when there are ≥4
/// outermost hints + (optionally) a centre hint):
///
///   1. Take the 4 hints with the most-extreme coordinates.
///   2. Trace the rectangle through them: top-row TL→TR; right-col
///      TR→BR; bottom-row BR→BL; left-col BL up to one short of TL.
///   3. From the rectangle, lay a horizontal spoke at the centre hint's
///      y-row toward the centre hint.
///   4. Dedupe positions while preserving order; return.
///
/// Returns an empty Vec if `hints.hints.len() < 4` — caller should fall
/// back to default variable-order.
pub fn build_hint_rectangle_path(puzzle: &Puzzle, hints: &Hints) -> Vec<Position> {
    if hints.hints.len() < 4 {
        return Vec::new();
    }
    let w = puzzle.width;
    let xy = |p: Position| (p % w, p / w);
    let pos_of = |x: u32, y: u32| -> Position { y * w + x };
    // Compute Chebyshev distance from board centre for each hint.
    let cx = (w as i32 - 1) / 2;
    let cy = (puzzle.height as i32 - 1) / 2;
    let mut h: Vec<(Position, u32, u32, i32)> = hints
        .hints
        .iter()
        .map(|h| {
            let (x, y) = xy(h.position);
            let d = std::cmp::max((x as i32 - cx).abs(), (y as i32 - cy).abs());
            (h.position, x, y, d)
        })
        .collect();
    // Sort by Chebyshev distance descending — outermost first.
    h.sort_by(|a, b| b.3.cmp(&a.3));

    let mut tl: Option<(u32, u32)> = None;
    let mut tr: Option<(u32, u32)> = None;
    let mut br: Option<(u32, u32)> = None;
    let mut bl: Option<(u32, u32)> = None;
    for &(_, x, y, _) in &h {
        let is_left = (x as i32) <= cx;
        let is_top = (y as i32) <= cy;
        let slot = match (is_top, is_left) {
            (true, true) => &mut tl,
            (true, false) => &mut tr,
            (false, false) => &mut br,
            (false, true) => &mut bl,
        };
        if slot.is_none() {
            *slot = Some((x, y));
        }
    }
    let (Some((tlx, tly)), Some((trx, tryy)), Some((brx, bry)), Some((blx, bly))) = (tl, tr, br, bl)
    else {
        return Vec::new();
    };
    let _ = (tryy, blx, bly);

    let mut path = Vec::new();
    let mut seen = std::collections::HashSet::new();
    let push = |p: Position,
                path: &mut Vec<Position>,
                seen: &mut std::collections::HashSet<Position>| {
        if seen.insert(p) {
            path.push(p);
        }
    };
    let ymin = std::cmp::min(tly, tryy);
    for x in tlx..=trx {
        push(pos_of(x, ymin), &mut path, &mut seen);
    }
    let ymax = std::cmp::max(bry, bly);
    let xright = std::cmp::max(trx, brx);
    for y in (ymin + 1)..=ymax {
        push(pos_of(xright, y), &mut path, &mut seen);
    }
    for x in (tlx..xright).rev() {
        push(pos_of(x, ymax), &mut path, &mut seen);
    }
    for y in ((ymin + 1)..ymax).rev() {
        push(pos_of(tlx, y), &mut path, &mut seen);
    }
    let mut inner: Vec<&(Position, u32, u32, i32)> = h.iter().skip(4).collect();
    inner.sort_by(|a, b| a.3.cmp(&b.3));
    for entry in &inner {
        let hx: u32 = entry.1;
        let hy: u32 = entry.2;
        if hx >= tlx {
            for x in tlx..=hx {
                push(pos_of(x, hy), &mut path, &mut seen);
            }
        } else {
            for x in (hx..=tlx).rev() {
                push(pos_of(x, hy), &mut path, &mut seen);
            }
        }
    }
    path
}

/// Vol-14 user-proposed: extend `build_hint_rectangle_path` to cover
/// all cells in three subsequent phases (interior, annulus, outer border).
pub fn build_hint_rectangle_layered_path(puzzle: &Puzzle, hints: &Hints) -> Vec<Position> {
    let phase1 = build_hint_rectangle_path(puzzle, hints);
    if phase1.is_empty() {
        return Vec::new();
    }
    let w = puzzle.width;
    let h = puzzle.height;
    let n = puzzle.cell_count() as usize;
    let xy = |p: Position| (p % w, p / w);
    let pos_of = |x: u32, y: u32| -> Position { y * w + x };

    let (tlx, tly) = xy(phase1[0]);
    let mut max_x = tlx;
    let mut max_y = tly;
    let mut min_x = tlx;
    let mut min_y = tly;
    for &p in &phase1 {
        let (x, y) = xy(p);
        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }
        if x < min_x {
            min_x = x;
        }
        if y < min_y {
            min_y = y;
        }
    }
    let (trx, bry) = (max_x, max_y);

    let mut seen: std::collections::HashSet<Position> = phase1.iter().copied().collect();
    let mut path = phase1;

    let cx = (w as i32 - 1) / 2;
    let cy = (h as i32 - 1) / 2;
    let mut interior_rect: Vec<(Position, i32, u32)> = Vec::new();
    for y in (min_y + 1)..bry {
        for x in (min_x + 1)..trx {
            let p = pos_of(x, y);
            if seen.contains(&p) {
                continue;
            }
            let d = std::cmp::max((x as i32 - cx).abs(), (y as i32 - cy).abs());
            interior_rect.push((p, d, p));
        }
    }
    interior_rect.sort_by(|a, b| a.1.cmp(&b.1).then(a.2.cmp(&b.2)));
    for (p, _, _) in &interior_rect {
        if seen.insert(*p) {
            path.push(*p);
        }
    }

    for y in 0..h {
        for x in 0..w {
            let p = pos_of(x, y);
            if seen.contains(&p) {
                continue;
            }
            if x == 0 || x == w - 1 || y == 0 || y == h - 1 {
                continue;
            }
            path.push(p);
            seen.insert(p);
        }
    }

    let corners = [
        pos_of(0, 0),
        pos_of(w - 1, 0),
        pos_of(0, h - 1),
        pos_of(w - 1, h - 1),
    ];
    for &c in &corners {
        if seen.insert(c) {
            path.push(c);
        }
    }
    for x in 0..w {
        let p = pos_of(x, 0);
        if seen.insert(p) {
            path.push(p);
        }
    }
    for y in 0..h {
        let p = pos_of(w - 1, y);
        if seen.insert(p) {
            path.push(p);
        }
    }
    for x in 0..w {
        let p = pos_of(x, h - 1);
        if seen.insert(p) {
            path.push(p);
        }
    }
    for y in 0..h {
        let p = pos_of(0, y);
        if seen.insert(p) {
            path.push(p);
        }
    }

    debug_assert_eq!(
        path.len(),
        n,
        "layered path should cover all cells, got {}/{}",
        path.len(),
        n
    );
    path
}

/// Vol-23 — build the X-skeleton path. 3-cell-wide diagonals through 5
/// hints (TL→C→BR, TR→C→BL). See `PathSkeleton::XSkeleton`.
/// Returns an empty Vec if fewer than 5 hints are supplied.
pub fn build_x_skeleton_path(puzzle: &Puzzle, hints: &Hints) -> Vec<Position> {
    if hints.hints.len() < 5 {
        return Vec::new();
    }
    let w = puzzle.width;
    let h = puzzle.height;
    let n = puzzle.cell_count() as usize;
    let xy = |p: Position| (p % w, p / w);
    let pos_of = |x: u32, y: u32| -> Position { y * w + x };

    let cx_i = (w as i32 - 1) / 2;
    let cy_i = (h as i32 - 1) / 2;
    let mut entries: Vec<(u32, u32, i32)> = hints
        .hints
        .iter()
        .map(|hh| {
            let (x, y) = xy(hh.position);
            let d = std::cmp::max((x as i32 - cx_i).abs(), (y as i32 - cy_i).abs());
            (x, y, d)
        })
        .collect();
    entries.sort_by_key(|e| e.2);
    let (ccx, ccy, _) = entries[0];
    let outer: Vec<(u32, u32)> = entries.iter().skip(1).map(|e| (e.0, e.1)).collect();
    let mut tl: Option<(u32, u32)> = None;
    let mut tr: Option<(u32, u32)> = None;
    let mut bl: Option<(u32, u32)> = None;
    let mut br: Option<(u32, u32)> = None;
    for &(x, y) in &outer {
        let is_left = (x as i32) <= cx_i;
        let is_top = (y as i32) <= cy_i;
        let slot = match (is_top, is_left) {
            (true, true) => &mut tl,
            (true, false) => &mut tr,
            (false, false) => &mut br,
            (false, true) => &mut bl,
        };
        if slot.is_none() {
            *slot = Some((x, y));
        }
    }
    let (Some(tlp), Some(trp), Some(blp), Some(brp)) = (tl, tr, bl, br) else {
        return Vec::new();
    };

    let mut path: Vec<Position> = Vec::with_capacity(n);
    let mut seen: std::collections::HashSet<Position> = std::collections::HashSet::new();
    let push = |x: i32,
                y: i32,
                path: &mut Vec<Position>,
                seen: &mut std::collections::HashSet<Position>| {
        if x < 0 || y < 0 || (x as u32) >= w || (y as u32) >= h {
            return;
        }
        let p = pos_of(x as u32, y as u32);
        if seen.insert(p) {
            path.push(p);
        }
    };

    let walk_main_diag = |x0: i32,
                          y0: i32,
                          x1: i32,
                          y1: i32,
                          path: &mut Vec<Position>,
                          seen: &mut std::collections::HashSet<Position>| {
        let dx = (x1 - x0).signum();
        let dy = (y1 - y0).signum();
        let steps = std::cmp::max((x1 - x0).abs(), (y1 - y0).abs());
        for k in 0..=steps {
            let x = x0 + k * dx;
            let y = y0 + k * dy;
            push(x, y, path, seen);
            push(x, y - 1, path, seen);
            push(x, y + 1, path, seen);
        }
    };
    let walk_anti_diag = |x0: i32,
                          y0: i32,
                          x1: i32,
                          y1: i32,
                          path: &mut Vec<Position>,
                          seen: &mut std::collections::HashSet<Position>| {
        let dx = (x1 - x0).signum();
        let dy = (y1 - y0).signum();
        let steps = std::cmp::max((x1 - x0).abs(), (y1 - y0).abs());
        for k in 0..=steps {
            let x = x0 + k * dx;
            let y = y0 + k * dy;
            push(x, y, path, seen);
            push(x - 1, y, path, seen);
            push(x + 1, y, path, seen);
        }
    };

    walk_main_diag(
        tlp.0 as i32,
        tlp.1 as i32,
        ccx as i32,
        ccy as i32,
        &mut path,
        &mut seen,
    );
    walk_main_diag(
        ccx as i32,
        ccy as i32,
        brp.0 as i32,
        brp.1 as i32,
        &mut path,
        &mut seen,
    );
    walk_anti_diag(
        trp.0 as i32,
        trp.1 as i32,
        ccx as i32,
        ccy as i32,
        &mut path,
        &mut seen,
    );
    walk_anti_diag(
        ccx as i32,
        ccy as i32,
        blp.0 as i32,
        blp.1 as i32,
        &mut path,
        &mut seen,
    );

    let mut rest: Vec<(Position, i32, Position)> = Vec::new();
    for y in 0..h {
        for x in 0..w {
            let p = pos_of(x, y);
            if seen.contains(&p) {
                continue;
            }
            let d = std::cmp::max(
                (x as i32 - ccx as i32).abs(),
                (y as i32 - ccy as i32).abs(),
            );
            rest.push((p, d, p));
        }
    }
    rest.sort_by(|a, b| a.1.cmp(&b.1).then(a.2.cmp(&b.2)));
    for (p, _, _) in &rest {
        if seen.insert(*p) {
            path.push(*p);
        }
    }
    debug_assert_eq!(
        path.len(),
        n,
        "X-skeleton path should cover all cells, got {}/{}",
        path.len(),
        n
    );
    path
}
