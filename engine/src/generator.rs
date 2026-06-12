//! Seeded generator for solvable n×n edge-matching puzzles.
//!
//! Paints every interior adjacency with a random color (each color used at
//! least once), derives the pieces from the painted board, then shuffles
//! piece order and rotations. The result is solvable by construction.
//! Deterministic for a given (size, colors, seed) on every platform — no
//! getrandom, just xorshift.

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
    let mut puzzle = generate_solved(size, colors, seed);
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
    assert!(size >= 2, "size must be >= 2");
    let colors = colors.clamp(1, max_colors(size));
    let s = size as usize;
    let n_edges = interior_edge_count(size) as usize;
    let mut rng = XorShift::new(seed);

    // One color per interior adjacency, every color present at least once.
    let mut palette: Vec<u8> = (0..n_edges)
        .map(|i| {
            if i < colors as usize {
                i as u8 + 1
            } else {
                rng.below(u32::from(colors)) as u8 + 1
            }
        })
        .collect();
    rng.shuffle(&mut palette);

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
}
