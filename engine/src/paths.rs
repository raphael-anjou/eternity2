//! Built-in cell-visit orders ("paths") — ALGORITHM.md §6. A path is the order
//! in which the DFS fills cells: a permutation of 0..w*h (row-major cell
//! indices). Path order is a first-class search hyperparameter: on the same
//! puzzle, different paths can differ by orders of magnitude in nodes-to-solve.

use crate::generator::XorShift;

pub const PATH_KINDS: &[&str] = &[
    "row-major",
    "snake",
    "column-major",
    "spiral-in",
    "spiral-out",
    "diagonal",
    "border-first",
    "double-snake",
    "random",
];

pub fn build_path(kind: &str, width: u8, height: u8, seed: u32) -> Option<Vec<u16>> {
    let (w, h) = (width as usize, height as usize);
    let idx = |x: usize, y: usize| (y * w + x) as u16;
    let mut out: Vec<u16> = Vec::with_capacity(w * h);
    match kind {
        "row-major" => {
            for y in 0..h {
                for x in 0..w {
                    out.push(idx(x, y));
                }
            }
        }
        "snake" => {
            for y in 0..h {
                if y % 2 == 0 {
                    for x in 0..w {
                        out.push(idx(x, y));
                    }
                } else {
                    for x in (0..w).rev() {
                        out.push(idx(x, y));
                    }
                }
            }
        }
        "column-major" => {
            for x in 0..w {
                for y in 0..h {
                    out.push(idx(x, y));
                }
            }
        }
        "spiral-in" | "spiral-out" => {
            let (mut x0, mut y0, mut x1, mut y1) = (0usize, 0usize, w, h);
            while x0 < x1 && y0 < y1 {
                for x in x0..x1 {
                    out.push(idx(x, y0));
                }
                for y in (y0 + 1)..y1 {
                    out.push(idx(x1 - 1, y));
                }
                if y1 > y0 + 1 {
                    for x in (x0..(x1 - 1)).rev() {
                        out.push(idx(x, y1 - 1));
                    }
                }
                if x1 > x0 + 1 {
                    for y in ((y0 + 1)..(y1 - 1)).rev() {
                        out.push(idx(x0, y));
                    }
                }
                x0 += 1;
                y0 += 1;
                x1 -= 1;
                y1 -= 1;
            }
            if kind == "spiral-out" {
                out.reverse();
            }
        }
        "diagonal" => {
            for d in 0..(w + h - 1) {
                for y in 0..h {
                    if d >= y && d - y < w {
                        out.push(idx(d - y, y));
                    }
                }
            }
        }
        "border-first" => {
            // Rim clockwise from (0,0), then interior row-major.
            for x in 0..w {
                out.push(idx(x, 0));
            }
            for y in 1..h {
                out.push(idx(w - 1, y));
            }
            if h > 1 {
                for x in (0..w - 1).rev() {
                    out.push(idx(x, h - 1));
                }
            }
            if w > 1 {
                for y in (1..h - 1).rev() {
                    out.push(idx(0, y));
                }
            }
            for y in 1..h.saturating_sub(1) {
                for x in 1..w.saturating_sub(1) {
                    out.push(idx(x, y));
                }
            }
        }
        "double-snake" => {
            // Two rows at a time, zig-zagging keeps a compact frontier.
            let mut y = 0;
            while y < h {
                let rows: Vec<usize> = if y + 1 < h { vec![y, y + 1] } else { vec![y] };
                if (y / 2) % 2 == 0 {
                    for x in 0..w {
                        for &r in &rows {
                            out.push(idx(x, r));
                        }
                    }
                } else {
                    for x in (0..w).rev() {
                        for &r in &rows {
                            out.push(idx(x, r));
                        }
                    }
                }
                y += 2;
            }
        }
        "random" => {
            out = (0..(w * h) as u16).collect();
            XorShift::new(seed).shuffle(&mut out);
        }
        _ => return None,
    }
    debug_assert_eq!(out.len(), w * h);
    Some(out)
}

/// Hint-aware path builder. Adds orders that need the pinned cell positions on
/// top of the hint-blind [`build_path`] set. Used by the hint study's scaling
/// axis, which runs the (runtime-sized) solver across board sizes.
///
/// - `connect-hints-first`: multi-source BFS outward from every hint at once —
///   the study's "reach the clues and connect them" negative control. It opens a
///   frontier around each hint and never keeps a single compact one.
///
/// Any `kind` that is not hint-aware falls through to [`build_path`], so this is
/// a superset. Returns a permutation of `0..w*h` (pinned cells included; the
/// solver skips already-placed cells itself).
pub fn build_path_with_hints(
    kind: &str,
    width: u8,
    height: u8,
    seed: u32,
    hints: &[u16],
) -> Option<Vec<u16>> {
    let (w, h) = (width as usize, height as usize);
    let idx0 = |x: usize, y: usize| (y * w + x) as u16;

    // Accept the hint study's canonical path names (as used by run_dfs and the
    // grid) so the scaling axis can drive this solver with the SAME path set the
    // 16×16 study uses — not just the site engine's `build_path` vocabulary.
    match kind {
        // Aliases onto build_path's names.
        "rowmajor" => return build_path("row-major", width, height, seed),
        "spiral-in" | "spiral-out" | "border-first" => {
            return build_path(kind, width, height, seed)
        }
        // Orders the site engine's build_path does not have; built here.
        "rowmajor-bottomup" => {
            let mut out = Vec::with_capacity(w * h);
            for y in (0..h).rev() {
                for x in 0..w {
                    out.push(idx0(x, y));
                }
            }
            return Some(out);
        }
        "verhaard-comb" => {
            let horiz = 10.min(h);
            let mut out = Vec::with_capacity(w * h);
            for y in 0..horiz {
                for x in 0..w {
                    out.push(idx0(x, y));
                }
            }
            for x in 0..w {
                for y in horiz..h {
                    out.push(idx0(x, y));
                }
            }
            return Some(out);
        }
        "clue-rows-first" => {
            // The official-E2 clue rows swept first (only meaningful at 16×16;
            // for other sizes we scale the fractions), then the rest top-down.
            let clue_rows: Vec<usize> = if h == 16 {
                vec![2, 8, 13]
            } else {
                vec![h / 8, h / 2, (13 * h) / 16]
            };
            let mut out = Vec::with_capacity(w * h);
            for &r in &clue_rows {
                for x in 0..w {
                    out.push(idx0(x, r));
                }
            }
            for y in 0..h {
                if clue_rows.contains(&y) {
                    continue;
                }
                for x in 0..w {
                    out.push(idx0(x, y));
                }
            }
            return Some(out);
        }
        "trace-hints" => {
            return Some(trace_hints_seq(w, h, hints));
        }
        _ => {}
    }

    if kind != "connect-hints-first" {
        return build_path(kind, width, height, seed);
    }
    let n = w * h;
    let idx = |x: usize, y: usize| (y * w + x) as u16;
    let mut seen = vec![false; n];
    let mut out: Vec<u16> = Vec::with_capacity(n);
    let mut queue: std::collections::VecDeque<u16> = std::collections::VecDeque::new();
    // Seed from the hints (in the interior, but any cell works). If there are no
    // hints this degenerates to row-major from cell 0.
    let anchors: Vec<u16> = if hints.is_empty() { vec![0] } else { hints.to_vec() };
    for &a in &anchors {
        if (a as usize) < n && !seen[a as usize] {
            seen[a as usize] = true;
            out.push(a);
            queue.push_back(a);
        }
    }
    while let Some(cell) = queue.pop_front() {
        let (cx, cy) = (cell as usize % w, cell as usize / w);
        let mut push = |x: usize, y: usize, out: &mut Vec<u16>, q: &mut std::collections::VecDeque<u16>, seen: &mut [bool]| {
            let i = idx(x, y);
            if !seen[i as usize] {
                seen[i as usize] = true;
                out.push(i);
                q.push_back(i);
            }
        };
        if cy > 0 { push(cx, cy - 1, &mut out, &mut queue, &mut seen); }
        if cx > 0 { push(cx - 1, cy, &mut out, &mut queue, &mut seen); }
        if cx + 1 < w { push(cx + 1, cy, &mut out, &mut queue, &mut seen); }
        if cy + 1 < h { push(cx, cy + 1, &mut out, &mut queue, &mut seen); }
    }
    // Any unreached cell (cannot happen on a connected grid) appended for safety.
    for i in 0..n as u16 {
        if !seen[i as usize] {
            out.push(i);
        }
    }
    debug_assert_eq!(out.len(), n);
    Some(out)
}

/// "Trace the hints" order (see the dfs-engine `TraceHints` doc): a skeleton
/// joining the pins (square between the four outer hints + diagonals to the
/// centre), then a constraint-greedy BFS fill outward from it. Mirrors the
/// dfs-engine implementation so `hint_scale` (scaling axis) matches the measured
/// 16×16 study. Falls back to a hint-BFS / row-major with fewer than 5 hints.
fn trace_hints_seq(w: usize, h: usize, hints: &[u16]) -> Vec<u16> {
    let n = w * h;
    let idx = |x: usize, y: usize| (y * w + x) as u16;
    let rc = |cell: u16| ((cell as usize) % w, (cell as usize) / w); // (x, y)
    if hints.len() < 5 {
        // BFS from whatever hints exist (or cell 0), like connect-hints-first.
        let anchors: Vec<u16> = if hints.is_empty() { vec![0] } else { hints.to_vec() };
        return bfs_fill(w, h, &anchors);
    }
    // Centre hint = nearest to board centre; the rest are the four outer corners.
    let centre = *hints
        .iter()
        .min_by_key(|&&cell| {
            let (x, y) = rc(cell);
            let dx = x as isize - w as isize / 2;
            let dy = y as isize - h as isize / 2;
            dx * dx + dy * dy
        })
        .unwrap();
    let mut outer: Vec<u16> = hints.iter().copied().filter(|&c| c != centre).collect();
    outer.sort_by_key(|&cell| {
        let (x, y) = rc(cell);
        let (top, left) = (y < h / 2, x < w / 2);
        match (top, left) {
            (true, true) => 0,
            (true, false) => 1,
            (false, false) => 2,
            (false, true) => 3,
        }
    });

    let mut order: Vec<u16> = Vec::with_capacity(n);
    let mut seen = vec![false; n];
    let mut push = |cell: u16, order: &mut Vec<u16>, seen: &mut [bool]| {
        if !seen[cell as usize] {
            seen[cell as usize] = true;
            order.push(cell);
        }
    };
    let mut line = |a: u16, b: u16, order: &mut Vec<u16>, seen: &mut [bool]| {
        let (mut x0, mut y0) = (rc(a).0 as isize, rc(a).1 as isize);
        let (x1, y1) = (rc(b).0 as isize, rc(b).1 as isize);
        let (dx, dy) = ((x1 - x0).abs(), (y1 - y0).abs());
        let (sx, sy) = (if x0 < x1 { 1 } else { -1 }, if y0 < y1 { 1 } else { -1 });
        let mut err = dx - dy;
        loop {
            push(idx(x0 as usize, y0 as usize), order, seen);
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x0 += sx;
            }
            if e2 < dx {
                err += dx;
                y0 += sy;
            }
        }
    };
    for i in 0..outer.len() {
        line(outer[i], outer[(i + 1) % outer.len()], &mut order, &mut seen);
    }
    for &o in &outer {
        line(o, centre, &mut order, &mut seen);
    }
    // Constraint-greedy fill outward from the skeleton.
    let mut q: std::collections::VecDeque<u16> = order.iter().copied().collect();
    while let Some(cell) = q.pop_front() {
        let (x, y) = rc(cell);
        let mut nbrs: Vec<u16> = Vec::with_capacity(4);
        if y > 0 { nbrs.push(idx(x, y - 1)); }
        if x > 0 { nbrs.push(idx(x - 1, y)); }
        if x + 1 < w { nbrs.push(idx(x + 1, y)); }
        if y + 1 < h { nbrs.push(idx(x, y + 1)); }
        for nb in nbrs {
            if !seen[nb as usize] {
                seen[nb as usize] = true;
                order.push(nb);
                q.push_back(nb);
            }
        }
    }
    for p in 0..n as u16 {
        if !seen[p as usize] {
            order.push(p);
        }
    }
    order
}

/// Multi-source BFS from `anchors` over the grid (shared by connect-hints and the
/// small-hint fallback of trace-hints). Deterministic up-left-right-down order.
fn bfs_fill(w: usize, h: usize, anchors: &[u16]) -> Vec<u16> {
    let n = w * h;
    let idx = |x: usize, y: usize| (y * w + x) as u16;
    let mut seen = vec![false; n];
    let mut order: Vec<u16> = Vec::with_capacity(n);
    let mut q: std::collections::VecDeque<u16> = std::collections::VecDeque::new();
    for &a in anchors {
        if (a as usize) < n && !seen[a as usize] {
            seen[a as usize] = true;
            order.push(a);
            q.push_back(a);
        }
    }
    while let Some(cell) = q.pop_front() {
        let (x, y) = ((cell as usize) % w, (cell as usize) / w);
        let mut nbrs: Vec<u16> = Vec::with_capacity(4);
        if y > 0 { nbrs.push(idx(x, y - 1)); }
        if x > 0 { nbrs.push(idx(x - 1, y)); }
        if x + 1 < w { nbrs.push(idx(x + 1, y)); }
        if y + 1 < h { nbrs.push(idx(x, y + 1)); }
        for nb in nbrs {
            if !seen[nb as usize] {
                seen[nb as usize] = true;
                order.push(nb);
                q.push_back(nb);
            }
        }
    }
    for p in 0..n as u16 {
        if !seen[p as usize] {
            order.push(p);
        }
    }
    order
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_kinds_are_permutations() {
        for kind in PATH_KINDS {
            for (w, h) in [(3u8, 3u8), (4, 4), (5, 5), (16, 16), (2, 2)] {
                let p = build_path(kind, w, h, 1).unwrap();
                let mut seen = vec![false; w as usize * h as usize];
                for &c in &p {
                    assert!(!seen[c as usize], "{kind} repeats cell {c} at {w}x{h}");
                    seen[c as usize] = true;
                }
                assert!(seen.iter().all(|&b| b), "{kind} misses cells at {w}x{h}");
            }
        }
    }

    #[test]
    fn spiral_in_3x3_matches_reference() {
        assert_eq!(
            build_path("spiral-in", 3, 3, 0).unwrap(),
            vec![0, 1, 2, 5, 8, 7, 6, 3, 4]
        );
    }
}
