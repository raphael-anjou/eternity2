//! Seeded generator for solvable n×n edge-matching puzzles, and the engine's
//! own RNG (ALGORITHM.md §3 for `XorShift`, §4 for generation).
//!
//! Paints every interior adjacency with a random color (each color used at
//! least once), derives the pieces from the painted board, then shuffles
//! piece order and rotations. The result is solvable by construction.
//! Deterministic for a given (size, colors, seed) on every platform: no
//! getrandom, just xorshift — which is exactly why every port can reproduce
//! these puzzles byte-for-byte.
//!
//! MATCHED PAIR: the generation algorithm here is duplicated, byte-for-byte, in
//! `research/experiments/common/crates/e2-core/src/generator.rs` (which the
//! starter kit uses). The two differ ONLY in the return type — this one builds
//! the engine's `Puzzle`, the other a crate-local `GeneratedPuzzle`. If you
//! change the algorithm (painting, the uniqueness pass, the RNG, the frame
//! recipe), change BOTH or generated boards will silently diverge between the
//! site and the kit. The e2-core copy's census test guards its output.

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

/// Number of border colors when [`generate_solved_framed`] runs in framed mode:
/// `min(5, colors - 1)`, mirroring real Eternity II's five border colors. These
/// colors (`1..=frame_count`) appear ONLY on along-frame edges — the seams
/// joining two neighbouring border pieces — and never in the interior. The
/// remaining colors (`frame_count+1..=colors`) are interior colors and appear
/// both in the interior AND on the inward-facing edge of border pieces, exactly
/// as the official puzzle does.
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

/// Is palette slot `i` an *along-frame* adjacency — a seam that runs ALONG the
/// outer ring, joining two neighbouring border pieces? Only these carry the
/// border colours (`1..=frame_count`) in real Eternity II. Crucially this is
/// NOT "touches the ring": the seam between a border piece and the first
/// interior piece points INWARD and carries an interior colour, so every
/// interior colour also appears on edge pieces — exactly as the official puzzle
/// does (verified against the official piece set: edge pieces' along-frame edges
/// use only colours 1–5, their interior-facing edge uses 6–22).
///
/// Slot order: `i < s*(s-1)` indexes `vert` (a left–right seam, `v(x,y)` between
/// cells `(x,y)` and `(x+1,y)`); otherwise `horiz` (an up–down seam, `h(x,y)`
/// between `(x,y)` and `(x,y+1)`).
fn slot_is_frame_band(i: usize, s: usize) -> bool {
    let vcount = s * (s - 1);
    if i < vcount {
        // vert = left–right seam between (x,y) and (x+1,y). Along the frame when
        // BOTH cells sit on the top or bottom rim, i.e. y is 0 or s-1.
        let y = i / (s - 1);
        y == 0 || y == s - 1
    } else {
        // horiz = up–down seam between (x,y) and (x,y+1). Along the frame when
        // BOTH cells sit on the left or right rim, i.e. x is 0 or s-1.
        let j = i - vcount;
        let x = j % s;
        x == 0 || x == s - 1
    }
}

/// Paint each palette slot in framed mode. Returns colors in default flat slot
/// order. Along-frame slots (seams joining two border pieces, see
/// [`slot_is_frame_band`]) get border colors `1..=frame_count`; every other slot
/// — including the inward-facing seams of border pieces — gets an interior color
/// `frame_count+1..=colors`. This reproduces the official Eternity II structure:
/// border colors live only on the frame, interior colors appear on both edge
/// pieces (their inward edge) and the deep interior.
///
/// Each band is filled by **cycling** its colors — slot `k` gets color
/// `base + (k mod count) + 1` — so every color appears as equally often as the
/// slot count allows (each ⌈slots/count⌉ or ⌊slots/count⌋ times), then the band
/// is shuffled. This mirrors real Eternity II, whose ~equal number of each edge
/// motif is what makes its boards look the way they do; the shuffle keeps the
/// board fully random despite the even counts. (Thanks to VV for the nudge.)
///
/// The only RNG primitive is `shuffle` — no `below` draws inside a band — so the
/// output is a pure function of the two shuffles and every port matches.
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

    // Frame band: cycle colors 1..=frame_count, then shuffle.
    let mut frame_colors: Vec<u8> = (0..frame_slots.len())
        .map(|k| (k % frame_count as usize) as u8 + 1)
        .collect();
    rng.shuffle(&mut frame_colors);
    for (k, &slot) in frame_slots.iter().enumerate() {
        palette[slot] = frame_colors[k];
    }

    // Deep interior: cycle colors frame_count+1..=colors, then shuffle.
    let mut interior_colors: Vec<u8> = (0..interior_slots.len())
        .map(|k| frame_count + (k % interior_count as usize) as u8 + 1)
        .collect();
    rng.shuffle(&mut interior_colors);
    for (k, &slot) in interior_slots.iter().enumerate() {
        palette[slot] = interior_colors[k];
    }

    palette
}

/// Generalized board painter. When `framed` is true (and the split is
/// meaningful, see [`framed_is_meaningful`]), border colors `1..=frame_count` are
/// confined to along-frame adjacencies (seams joining two border pieces) and the
/// interior colors `frame_count+1..=colors` cover every other adjacency —
/// including a border piece's inward-facing edge — so interior colors appear on
/// both edge pieces and the deep interior, matching real Eternity II. When
/// false, behaves byte-for-byte like the original unrestricted painter (it does
/// not even create the second RNG path).
/// Rotation-canonical key of a piece's URDL edges: the lexicographically
/// smallest of its four rotations. Two pieces are the "same" to a hint-pinning
/// solver iff they share this key.
fn rot_key(e: [u8; 4]) -> [u8; 4] {
    let mut best = e;
    let mut r = e;
    for _ in 0..3 {
        r = [r[3], r[0], r[1], r[2]]; // rotate URDL clockwise
        if r < best {
            best = r;
        }
    }
    best
}

/// Make every piece unique up to rotation while **preserving each colour's exact
/// count** — real Eternity II is both all-distinct AND colour-balanced, and the
/// cycle-then-shuffle fill already achieves the balance. So instead of recolouring
/// a seam (which would unbalance the palette), we SWAP the colours of two seams
/// in the same band: a swap changes the two incident pieces but leaves every
/// colour's total unchanged. Swapping only within a band keeps the frame recipe
/// (border colours on along-frame seams, interior colours elsewhere) intact.
/// Bounded; returns having reduced collisions as far as it got if it cannot fully
/// converge.
fn make_pieces_rotation_unique(
    vert: &mut [u8],
    horiz: &mut [u8],
    s: usize,
    rng: &mut XorShift,
) {
    use std::collections::HashMap;

    let piece_at = |vert: &[u8], horiz: &[u8], x: usize, y: usize| -> [u8; 4] {
        let up = if y == 0 { BORDER } else { horiz[(y - 1) * s + x] };
        let down = if y == s - 1 { BORDER } else { horiz[y * s + x] };
        let left = if x == 0 { BORDER } else { vert[y * (s - 1) + x - 1] };
        let right = if x == s - 1 { BORDER } else { vert[y * (s - 1) + x] };
        [up, right, down, left]
    };

    let n_vert = s * (s - 1);
    // Read/write a seam by (is_vert, index).
    let get = |vert: &[u8], horiz: &[u8], is_vert: bool, idx: usize| -> u8 {
        if is_vert {
            vert[idx]
        } else {
            horiz[idx]
        }
    };

    // All swappable seams grouped by band, so a swap stays in-band (preserving
    // the frame recipe) and thus preserves each colour's count. Palette slot of a
    // vert index i is i; of a horiz index j is n_vert + j.
    let mut frame_seams: Vec<(bool, usize)> = Vec::new();
    let mut interior_seams: Vec<(bool, usize)> = Vec::new();
    for i in 0..n_vert {
        if slot_is_frame_band(i, s) {
            frame_seams.push((true, i));
        } else {
            interior_seams.push((true, i));
        }
    }
    for j in 0..s * (s - 1) {
        if slot_is_frame_band(n_vert + j, s) {
            frame_seams.push((false, j));
        } else {
            interior_seams.push((false, j));
        }
    }

    let max_iters = 200_000;
    for _ in 0..max_iters {
        // Count rotation-canonical keys; find a duplicated cell.
        let mut seen: HashMap<[u8; 4], usize> = HashMap::new();
        let mut dup_cell: Option<(usize, usize)> = None;
        for y in 0..s {
            for x in 0..s {
                let k = rot_key(piece_at(vert, horiz, x, y));
                let c = seen.entry(k).or_insert(0);
                *c += 1;
                if *c > 1 && dup_cell.is_none() {
                    dup_cell = Some((x, y));
                }
            }
        }
        let Some((dx, dy)) = dup_cell else { return }; // all unique

        // A seam incident to the duplicated cell, to be swapped with another seam
        // of a DIFFERENT colour in the same band.
        let incident: Vec<(bool, usize)> = {
            let mut v = Vec::new();
            if dx > 0 {
                v.push((true, dy * (s - 1) + dx - 1));
            }
            if dx < s - 1 {
                v.push((true, dy * (s - 1) + dx));
            }
            if dy > 0 {
                v.push((false, (dy - 1) * s + dx));
            }
            if dy < s - 1 {
                v.push((false, dy * s + dx));
            }
            v
        };
        if incident.is_empty() {
            return;
        }
        let (a_vert, a_idx) = incident[rng.below(incident.len() as u32) as usize];
        let a_slot = if a_vert { a_idx } else { n_vert + a_idx };
        let a_color = get(vert, horiz, a_vert, a_idx);
        let pool = if slot_is_frame_band(a_slot, s) {
            &frame_seams
        } else {
            &interior_seams
        };
        // Find a partner seam in the same band whose colour differs.
        let start = rng.below(pool.len() as u32) as usize;
        let mut swapped = false;
        for off in 0..pool.len() {
            let (b_vert, b_idx) = pool[(start + off) % pool.len()];
            if get(vert, horiz, b_vert, b_idx) != a_color {
                // Swap the two seam colours (count-preserving).
                let b_color = get(vert, horiz, b_vert, b_idx);
                if a_vert {
                    vert[a_idx] = b_color;
                } else {
                    horiz[a_idx] = b_color;
                }
                if b_vert {
                    vert[b_idx] = a_color;
                } else {
                    horiz[b_idx] = a_color;
                }
                swapped = true;
                break;
            }
        }
        if !swapped {
            return; // band is monochromatic; nothing to do
        }
    }
}

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
    let mut vert: Vec<u8> = palette[..s * (s - 1)].to_vec();
    let mut horiz: Vec<u8> = palette[s * (s - 1)..].to_vec();

    // Break rotation-canonical duplicate pieces so every piece is distinct up to
    // rotation. Real Eternity-II-style puzzles have all-distinct pieces, and
    // solvers that pin hints (e.g. McGavin's) require a hinted piece to be the
    // unique bearer of its edge pattern. We recolour one interior seam incident
    // to a duplicated cell until no rotation-canonical collisions remain. Only
    // interior seams are touched, so the frame band and border/corner structure
    // are preserved. Bounded so it always terminates.
    if colors >= 2 {
        make_pieces_rotation_unique(&mut vert, &mut horiz, s, &mut rng);
    }

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

    
    #[test]
    fn generated_pieces_are_rotation_unique() {
        // Every piece must be distinct up to rotation, so a hint-pinning solver
        // can place a hinted piece unambiguously (its edge pattern is unique).
        for &(size, colors, seed) in &[
            (16u8, 22u8, 1u32),
            (16, 22, 2),
            (16, 22, 7),
            (10, 22, 5),
            (8, 12, 99),
        ] {
            for framed in [true, false] {
                let p = generate_solved_framed(size, colors, seed, framed);
                let mut keys: Vec<[u8; 4]> = p.pieces.iter().map(|&e| rot_key(e)).collect();
                keys.sort_unstable();
                let n = keys.len();
                keys.dedup();
                assert_eq!(
                    keys.len(),
                    n,
                    "duplicate piece up to rotation at {size}/{colors}/{seed} framed={framed}"
                );
            }
        }
    }

    #[test]
    fn framed_color_census_is_balanced_and_even() {
        // Real Eternity II is colour-balanced: every colour's edge-instance count
        // is even (so a perfect solution's matches, sum(N_c)/2, land exactly with
        // zero slack), border colours are perfectly equal, and interior colours
        // sit within one "row" of each other. The swap-based uniqueness repair
        // must preserve this (a swap moves no colour's total). Verified for the
        // official-shaped 16×16/22-colour board against its census.
        for seed in [1u32, 2, 7, 42] {
            let p = generate_solved_framed(16, 22, seed, true);
            let mut count = [0usize; 23];
            for e in &p.pieces {
                for &c in e {
                    count[c as usize] += 1;
                }
            }
            // Grey border rim.
            assert_eq!(count[0], 64, "grey border count wrong at seed {seed}");
            // Every colour count is even.
            for (c, &n) in count.iter().enumerate() {
                assert!(n % 2 == 0, "colour {c} count {n} is odd at seed {seed}");
            }
            // The 5 border colours are exactly balanced.
            let border: Vec<usize> = (1..=5).map(|c| count[c]).collect();
            assert!(
                border.iter().all(|&n| n == border[0]),
                "border colours unbalanced at seed {seed}: {border:?}"
            );
            // The 17 interior colours differ by at most one fill-cycle (2 edges).
            let interior: Vec<usize> = (6..=22).map(|c| count[c]).collect();
            let (lo, hi) = (
                *interior.iter().min().unwrap(),
                *interior.iter().max().unwrap(),
            );
            assert!(
                hi - lo <= 2,
                "interior colours too unbalanced at seed {seed}: {lo}..{hi}"
            );
        }
    }

    #[test]
    fn framed_matches_real_e2_color_structure() {
        // The defining structural property of a real Eternity-II board, verified
        // against the official piece set: the `frame_count` border colours appear
        // ONLY on along-frame edges (edges joining two border pieces), and NEVER
        // in the deep interior; the remaining interior colours appear both on the
        // inward-facing edge of border pieces AND in the interior. So on an edge
        // piece, the two along-frame edges are border colours and the single
        // interior-facing edge is an interior colour.
        for &(size, colors, seed) in &[(10u8, 22u8, 5u32), (16, 22, 3), (16, 22, 7)] {
            let p = generate_solved_framed(size, colors, seed, true);
            let fc = frame_color_count(colors);
            let border_colors: std::collections::HashSet<u8> = (1..=fc).collect();

            let is_border = |e: u8| e == BORDER;
            let mut deep_colors = std::collections::HashSet::new();
            for e in &p.pieces {
                let nborder = e.iter().filter(|&&c| is_border(c)).count();
                if nborder == 1 {
                    // Edge piece: URDL, one border edge. The edge OPPOSITE the
                    // border faces the interior; the two adjacent ones run along
                    // the frame.
                    let bi = e.iter().position(|&c| is_border(c)).unwrap();
                    let facing = e[(bi + 2) % 4];
                    let along = [e[(bi + 1) % 4], e[(bi + 3) % 4]];
                    for a in along {
                        assert!(
                            border_colors.contains(&a),
                            "along-frame edge {a} is not a border colour at {size}/{colors}/{seed}"
                        );
                    }
                    assert!(
                        !border_colors.contains(&facing),
                        "interior-facing edge {facing} is a border colour at {size}/{colors}/{seed}"
                    );
                } else if nborder == 0 {
                    for &c in e {
                        deep_colors.insert(c);
                    }
                }
            }
            // No border colour ever appears deep in the interior.
            for b in &border_colors {
                assert!(
                    !deep_colors.contains(b),
                    "border colour {b} leaked into the interior at {size}/{colors}/{seed}"
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
