//! Seeded generator for solvable n×n edge-matching puzzles, and the engine's
//! own RNG (ALGORITHM.md §3 for `XorShift`, §4 for generation).
//!
//! Paints every interior adjacency with a random color (each color used at
//! least once), derives the pieces from the painted board, then shuffles
//! piece order and rotations. The result is solvable by construction.
//! Deterministic for a given (size, colors, seed) on every platform: no
//! getrandom, just xorshift — which is exactly why every port can reproduce
//! these puzzles byte-for-byte.

use crate::types::{rotated, Puzzle, BORDER};

pub struct XorShift(u64);

impl XorShift {
    pub fn new(seed: u32) -> Self {
        // Avoid the all-zero state; splat the seed through splitmix once.
        let mut z = u64::from(seed).wrapping_add(0x9E37_79B9_7F4A_7C15);
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        XorShift(z ^ (z >> 31) | 1)
    }

    pub fn next_u32(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        (x >> 32) as u32
    }

    /// Uniform in [0, n).
    pub fn below(&mut self, n: u32) -> u32 {
        self.next_u32() % n
    }

    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        for i in (1..slice.len()).rev() {
            let j = self.below(i as u32 + 1) as usize;
            slice.swap(i, j);
        }
    }
}

/// Interior edge count of an n×n board: 2·n·(n−1).
pub fn interior_edge_count(size: u8) -> u32 {
    2 * size as u32 * (size as u32 - 1)
}

/// Largest usable color count for a size (every color must appear at least
/// once, and we only have 22 renderable motifs).
pub fn max_colors(size: u8) -> u8 {
    interior_edge_count(size).min(22) as u8
}

pub fn generate(size: u8, colors: u8, seed: u32) -> Puzzle {
    generate_framed(size, colors, seed, false)
}

/// Scrambled, solvable puzzle with frame-restricted colors confined to the
/// border band (see [`generate_solved_framed`]). `framed == false` is identical
/// to [`generate`].
pub fn generate_framed(size: u8, colors: u8, seed: u32, framed: bool) -> Puzzle {
    let mut puzzle = generate_solved_framed(size, colors, seed, framed);
    let mut rng = XorShift::new(seed ^ 0xA5A5_A5A5);
    rng.shuffle(&mut puzzle.pieces);
    for p in &mut puzzle.pieces {
        *p = rotated(*p, rng.below(4) as u8);
    }
    puzzle
}

/// Same construction, but pieces stay in solution order and orientation:
/// piece i belongs at cell i with rotation 0, so the identity board is the
/// solution (used by the viewer's board generator).
pub fn generate_solved(size: u8, colors: u8, seed: u32) -> Puzzle {
    generate_solved_framed(size, colors, seed, false)
}

/// Number of frame-restricted colors when [`generate_solved_framed`] runs in
/// framed mode: `min(5, colors - 1)`, mirroring real Eternity II where 5 of the
/// 22 colors appear only on edges adjacent to the frame. Colors `1..=frame_count`
/// are frame colors; the rest are interior colors.
pub fn frame_color_count(colors: u8) -> u8 {
    if colors < 2 {
        0
    } else {
        (colors - 1).min(5)
    }
}

/// True when the framed split is meaningful for this board: we need at least one
/// interior color (`colors >= 2`) AND at least one deep-interior adjacency to
/// confine interior colors to (which only exists once `size >= 4`, since a
/// deep-interior adjacency joins two off-rim cells). Below this threshold the
/// framed path is equivalent to — and falls back to — the unrestricted path, so
/// every requested color still appears.
fn framed_is_meaningful(size: u8, colors: u8) -> bool {
    size >= 4 && colors >= 2
}

/// Is palette slot `i` (default flat order: `i < s*(s-1)` is `vert[i]`, else
/// `horiz[i - s*(s-1)]`) a *frame-band* adjacency — i.e. at least one of its two
/// incident cells lies on the outer ring (row 0, row s-1, col 0, col s-1)?
fn slot_is_frame_band(i: usize, s: usize) -> bool {
    let vcount = s * (s - 1);
    if i < vcount {
        // vert(x,y): between cells (x,y) and (x+1,y).
        let x = i % (s - 1);
        let y = i / (s - 1);
        y == 0 || y == s - 1 || x == 0 || x + 1 == s - 1
    } else {
        // horiz(x,y): between cells (x,y) and (x,y+1).
        let j = i - vcount;
        let x = j % s;
        let y = j / s;
        x == 0 || x == s - 1 || y == 0 || y + 1 == s - 1
    }
}

/// Paint each palette slot in framed mode. Returns colors in default flat slot
/// order. Frame-band slots get colors `1..=frame_count`; deep-interior slots get
/// `frame_count+1..=colors`. Each group is filled so every color in the group
/// appears at least once (the first `k` slots of a group seed colors `1..=k`,
/// the rest are random within the group), then shuffled within the group. The
/// only RNG primitives used are `below` and `shuffle`, so all ports match.
fn paint_framed(rng: &mut XorShift, n_edges: usize, s: usize, colors: u8) -> Vec<u8> {
    let frame_count = frame_color_count(colors); // >= 1 here (colors >= 2)
    let interior_count = colors - frame_count; // >= 1 here (size >= 4)

    // Partition slot indices, ascending, into the two bands.
    let mut frame_slots: Vec<usize> = Vec::new();
    let mut interior_slots: Vec<usize> = Vec::new();
    for i in 0..n_edges {
        if slot_is_frame_band(i, s) {
            frame_slots.push(i);
        } else {
            interior_slots.push(i);
        }
    }

    let mut palette = vec![0u8; n_edges];

    // Frame band: colors 1..=frame_count.
    let mut frame_colors: Vec<u8> = (0..frame_slots.len())
        .map(|k| {
            if k < frame_count as usize {
                k as u8 + 1
            } else {
                rng.below(u32::from(frame_count)) as u8 + 1
            }
        })
        .collect();
    rng.shuffle(&mut frame_colors);
    for (k, &slot) in frame_slots.iter().enumerate() {
        palette[slot] = frame_colors[k];
    }

    // Deep interior: colors frame_count+1..=colors.
    let mut interior_colors: Vec<u8> = (0..interior_slots.len())
        .map(|k| {
            if k < interior_count as usize {
                frame_count + k as u8 + 1
            } else {
                frame_count + rng.below(u32::from(interior_count)) as u8 + 1
            }
        })
        .collect();
    rng.shuffle(&mut interior_colors);
    for (k, &slot) in interior_slots.iter().enumerate() {
        palette[slot] = interior_colors[k];
    }

    palette
}

/// Generalized board painter. When `framed` is true (and the split is
/// meaningful, see [`framed_is_meaningful`]), frame colors `1..=frame_count` are
/// confined to frame-band adjacencies and interior colors `frame_count+1..=colors`
/// to deep-interior adjacencies. When false, behaves byte-for-byte like the
/// original unrestricted painter (it does not even create the second RNG path).
pub fn generate_solved_framed(size: u8, colors: u8, seed: u32, framed: bool) -> Puzzle {
    assert!(size >= 2, "size must be >= 2");
    let colors = colors.clamp(1, max_colors(size));
    let s = size as usize;
    let n_edges = interior_edge_count(size) as usize;
    let mut rng = XorShift::new(seed);

    // The painted color of each palette slot in default flat order.
    let palette: Vec<u8> = if framed && framed_is_meaningful(size, colors) {
        paint_framed(&mut rng, n_edges, s, colors)
    } else {
        // One color per interior adjacency, every color present at least once.
        let mut p: Vec<u8> = (0..n_edges)
            .map(|i| {
                if i < colors as usize {
                    i as u8 + 1
                } else {
                    rng.below(u32::from(colors)) as u8 + 1
                }
            })
            .collect();
        rng.shuffle(&mut p);
        p
    };

    // vertical[y][x] = color between (x,y) and (x+1,y); horizontal[y][x]
    // between (x,y) and (x,y+1).
    let vert: Vec<u8> = palette[..s * (s - 1)].to_vec();
    let horiz: Vec<u8> = palette[s * (s - 1)..].to_vec();
    let v = |x: usize, y: usize| vert[y * (s - 1) + x];
    let h = |x: usize, y: usize| horiz[y * s + x];

    let mut pieces = Vec::with_capacity(s * s);
    for y in 0..s {
        for x in 0..s {
            let up = if y == 0 { BORDER } else { h(x, y - 1) };
            let down = if y == s - 1 { BORDER } else { h(x, y) };
            let left = if x == 0 { BORDER } else { v(x - 1, y) };
            let right = if x == s - 1 { BORDER } else { v(x, y) };
            pieces.push([up, right, down, left]);
        }
    }

    Puzzle {
        name: format!("generated_{size}x{size}_c{colors}_s{seed}"),
        width: size,
        height: size,
        num_colors: colors,
        pieces,
        hints: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_pieces_form_valid_multiset() {
        let p = generate(5, 6, 42);
        assert_eq!(p.pieces.len(), 25);
        // 4 corners (two border edges), 12 border pieces (one), 9 interior.
        let count_border =
            |e: &[u8; 4]| e.iter().filter(|&&c| c == BORDER).count();
        let corners = p.pieces.iter().filter(|e| count_border(e) == 2).count();
        let edges = p.pieces.iter().filter(|e| count_border(e) == 1).count();
        let interior = p.pieces.iter().filter(|e| count_border(e) == 0).count();
        assert_eq!((corners, edges, interior), (4, 12, 9));
        // All requested colors actually appear.
        let mut seen = [false; 23];
        for e in &p.pieces {
            for &c in e {
                seen[c as usize] = true;
            }
        }
        assert!(seen[1..=6].iter().all(|&b| b));
    }

    #[test]
    fn deterministic_for_same_seed() {
        assert_eq!(generate(4, 4, 7).pieces, generate(4, 4, 7).pieces);
        assert_ne!(generate(4, 4, 7).pieces, generate(4, 4, 8).pieces);
    }

    #[test]
    fn framed_off_is_byte_identical_to_default() {
        // The framed flag OFF must reproduce the legacy output exactly, for
        // both the scrambled and solved constructions, across sizes/seeds.
        for &(size, colors, seed) in
            &[(4u8, 4u8, 7u32), (5, 6, 42), (3, 3, 1), (6, 5, 99), (8, 10, 123)]
        {
            assert_eq!(
                generate(size, colors, seed).pieces,
                generate_framed(size, colors, seed, false).pieces,
                "scrambled framed=false diverged at {size}/{colors}/{seed}"
            );
            assert_eq!(
                generate_solved(size, colors, seed).pieces,
                generate_solved_framed(size, colors, seed, false).pieces,
                "solved framed=false diverged at {size}/{colors}/{seed}"
            );
        }
    }

    /// Recompute the painted adjacency colors of a *solved* board (pieces are in
    /// solution order, piece i at cell i, rot 0): returns (frame-band colors,
    /// deep-interior colors) gathered from the right/down edges of each cell.
    fn band_colors(p: &Puzzle) -> (Vec<u8>, Vec<u8>) {
        let s = p.width as usize;
        let mut frame = Vec::new();
        let mut interior = Vec::new();
        let at = |x: usize, y: usize| -> [u8; 4] { p.pieces[y * s + x] };
        for y in 0..s {
            for x in 0..s {
                let e = at(x, y);
                // right edge = vertical adjacency between (x,y) and (x+1,y)
                if x + 1 < s {
                    let c = e[1];
                    // frame-band iff either cell on the rim
                    let fb = y == 0 || y == s - 1 || x == 0 || x + 1 == s - 1;
                    if fb {
                        frame.push(c);
                    } else {
                        interior.push(c);
                    }
                }
                // down edge = horizontal adjacency between (x,y) and (x,y+1)
                if y + 1 < s {
                    let c = e[2];
                    let fb = x == 0 || x == s - 1 || y == 0 || y + 1 == s - 1;
                    if fb {
                        frame.push(c);
                    } else {
                        interior.push(c);
                    }
                }
            }
        }
        (frame, interior)
    }

    #[test]
    fn framed_confines_colors_to_their_band() {
        // For a meaningful framed board: NO frame color (1..=frame_count) on any
        // deep-interior adjacency, NO interior color on any frame-band adjacency,
        // the two sets are disjoint, and every requested color still appears.
        for &(size, colors, seed) in
            &[(4u8, 4u8, 7u32), (6, 8, 42), (8, 12, 99), (10, 22, 5), (16, 22, 3)]
        {
            let p = generate_solved_framed(size, colors, seed, true);
            let frame_count = frame_color_count(colors);
            let (frame_band, deep) = band_colors(&p);

            // Deep interior contains only interior colors (> frame_count).
            for &c in &deep {
                assert!(
                    c > frame_count,
                    "frame color {c} leaked into deep interior at {size}/{colors}/{seed}"
                );
            }
            // Frame band contains only frame colors (<= frame_count).
            for &c in &frame_band {
                assert!(
                    c >= 1 && c <= frame_count,
                    "interior color {c} leaked into frame band at {size}/{colors}/{seed}"
                );
            }
            // Disjoint on the board (a color is never in both bands).
            let mut in_frame = [false; 23];
            let mut in_deep = [false; 23];
            for &c in &frame_band {
                in_frame[c as usize] = true;
            }
            for &c in &deep {
                in_deep[c as usize] = true;
            }
            for c in 1..=colors as usize {
                assert!(
                    !(in_frame[c] && in_deep[c]),
                    "color {c} present in both bands at {size}/{colors}/{seed}"
                );
            }
            // Every requested color appears somewhere.
            for c in 1..=colors as usize {
                assert!(
                    in_frame[c] || in_deep[c],
                    "color {c} missing at {size}/{colors}/{seed}"
                );
            }
        }
    }

    #[test]
    fn framed_falls_back_below_threshold() {
        // size < 4 or colors < 2: framed == unframed (so all colors still show).
        for &(size, colors, seed) in &[(3u8, 3u8, 1u32), (2, 2, 9), (5, 1, 4)] {
            assert_eq!(
                generate_solved(size, colors, seed).pieces,
                generate_solved_framed(size, colors, seed, true).pieces,
                "below-threshold framed should equal unframed at {size}/{colors}/{seed}"
            );
        }
    }

    #[test]
    fn framed_deterministic_for_same_seed() {
        assert_eq!(
            generate_framed(8, 12, 7, true).pieces,
            generate_framed(8, 12, 7, true).pieces
        );
        // Framed vs unframed differ (different painting) for a meaningful board.
        assert_ne!(
            generate_framed(8, 12, 7, true).pieces,
            generate_framed(8, 12, 7, false).pieces
        );
    }
}
