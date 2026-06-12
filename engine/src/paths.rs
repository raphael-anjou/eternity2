//! Built-in cell-visit orders ("paths"). A path is the order in which the
//! DFS fills cells: a permutation of 0..w*h (row-major cell indices).
//! Path order is a first-class search hyperparameter — on the same puzzle,
//! different paths can differ by orders of magnitude in nodes-to-solve.

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
            // Two rows at a time, zig-zagging — keeps a compact frontier.
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
