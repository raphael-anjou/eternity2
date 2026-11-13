//! Bag census for the parity-defect-floor claim.
//!
//! Recomputes, from the official 256-piece instance, every bag statistic the
//! vol-226 theorems rest on, and compares each against the claimed value:
//!
//! * half-edge counts h(c) per color, their evenness, and the exact-fit
//!   identity (colored total = 960 = 2 x 480), premises of the parity
//!   invariant and the 479 gap;
//! * no rotationally self-symmetric tile, and the 4/56/196 class split,
//!   premise of the defect floor 2;
//! * the single-move census at defect 2: 125 near-twin pairs (50 interior,
//!   73 edge, 2 corner) and 29 two-side-keeping turns on 23 interior tiles,
//!   154 moves that land on 478 from every solution (a floor on the 478
//!   neighbours, not a ceiling; an earlier version summed 50 + 23 + 3 = 76
//!   and called it an upper bound, which it is not);
//! * the frame decomposition: colors 1-5 absent from interior tiles, 120
//!   frame-color half-edges saturating the 60 ring joints at 12 per color,
//!   and the fixed 56-entry inward demand vector.
//!
//! Prints a JSON report to stdout; exits nonzero if any check fails.

use serde::Serialize;

const JOINTS: u32 = 480;
const EXPECTED_H: [(u8, u32); 23] = [
    (0, 64),
    (1, 24),
    (2, 24),
    (3, 24),
    (4, 24),
    (5, 24),
    (6, 48),
    (7, 48),
    (8, 48),
    (9, 48),
    (10, 48),
    (11, 50),
    (12, 50),
    (13, 50),
    (14, 50),
    (15, 50),
    (16, 50),
    (17, 50),
    (18, 50),
    (19, 50),
    (20, 50),
    (21, 50),
    (22, 50),
];
/// Inward demand for colors 6..=22, cited from the vol-226 ring decomposition.
const EXPECTED_INWARD: [u32; 17] = [4, 5, 3, 3, 1, 1, 2, 3, 4, 6, 4, 2, 3, 6, 4, 3, 2];

#[derive(Serialize)]
struct Check {
    name: &'static str,
    computed: serde_json::Value,
    expected: serde_json::Value,
    ok: bool,
}

#[derive(Serialize)]
struct Report {
    instance: String,
    checks: Vec<Check>,
    all_ok: bool,
}

fn check<T: Serialize + PartialEq>(name: &'static str, computed: T, expected: T) -> Check {
    let ok = computed == expected;
    Check {
        name,
        computed: serde_json::to_value(computed).unwrap(),
        expected: serde_json::to_value(expected).unwrap(),
        ok,
    }
}

fn main() {
    let instance = e2_kit::official_instance(false);
    let pieces: Vec<[u8; 4]> = instance.pieces.iter().map(|(_, p)| p.edges).collect();
    assert_eq!(pieces.len(), 256, "official instance must have 256 pieces");

    let mut checks = Vec::new();

    // --- Half-edge statistics (premise of Theorem 1 / 1a) ---
    let h = e2_kit::analysis::color_half_edge_census(&instance.pieces);
    assert_eq!(h.len(), 23, "official instance colors are 0..=22");
    let expected_h_vec: Vec<u32> = EXPECTED_H.iter().map(|&(_, v)| v).collect();
    checks.push(check("half_edge_counts_h0_to_h22", h.clone(), expected_h_vec));

    let colored_total: u32 = h[1..].iter().sum();
    checks.push(check("exact_fit_identity_colored_total", colored_total, 2 * JOINTS));

    let all_even = h[1..].iter().all(|&x| x % 2 == 0);
    checks.push(check("all_colored_h_even_forces_479_gap", all_even, true));

    // --- No self-symmetric tile (premise of Theorem 2, floor = 2) ---
    let (_, half_orbit, fixed_orbit) = e2_kit::analysis::orbit_census(&instance.pieces);
    checks.push(check("rotationally_self_symmetric_tiles", half_orbit + fixed_orbit, 0));

    // --- Class split 4 / 56 / 196 ---
    let gray = |e: &[u8; 4]| e.iter().filter(|&&c| c == 0).count();
    let classes = e2_kit::analysis::piece_classes(&instance.pieces);
    let interior: Vec<[u8; 4]> = pieces.iter().copied().filter(|e| gray(e) == 0).collect();
    checks.push(check("corner_tiles", classes.corners.len(), 4));
    checks.push(check("edge_tiles", classes.edges.len(), 56));
    checks.push(check("interior_tiles", classes.interior.len(), 196));
    assert_eq!(interior.len(), classes.interior.len());

    // --- Theorem 3 census: single moves that land on defect 2 ---
    // Swaps: two tiles of the same kind such that some turn of B that keeps
    // its gray sides where A has them agrees with A on exactly 3 of 4 sides.
    // Swapped into each other's cells, each breaks only the joint across the
    // differing side, so the move lands on 478 from any solution (unless the
    // two sit side by side across that side). Rotating both tiles together
    // preserves the agreement count, so fixing A and trying the four turns of
    // B covers every alignment; a pair may align in more than one way, and
    // each alignment is a separate board.
    let near_twins = |group: &[[u8; 4]]| -> (usize, usize) {
        let mut pairs = 0usize;
        let mut swaps = 0usize;
        for i in 0..group.len() {
            for j in (i + 1)..group.len() {
                let a = group[i];
                let hits = (0..4u8)
                    .map(|r| e2_kit::Piece::new(group[j]).rotated(r))
                    .filter(|b| a.iter().zip(b.iter()).all(|(x, y)| (*x == 0) == (*y == 0)))
                    .filter(|b| a.iter().zip(b.iter()).filter(|(x, y)| x != y).count() == 1)
                    .count();
                if hits > 0 {
                    pairs += 1;
                    swaps += hits;
                }
            }
        }
        (pairs, swaps)
    };
    let edge_tiles: Vec<[u8; 4]> = pieces.iter().copied().filter(|e| gray(e) == 1).collect();
    let corner_tiles: Vec<[u8; 4]> = pieces.iter().copied().filter(|e| gray(e) == 2).collect();
    let (twin_pairs, interior_swaps) = near_twins(&interior[..]);
    let (edge_pairs, edge_swaps) = near_twins(&edge_tiles[..]);
    let (corner_pairs, corner_swaps) = near_twins(&corner_tiles[..]);
    checks.push(check("interior_near_twin_pairs", twin_pairs, 50));
    checks.push(check("edge_near_twin_pairs", edge_pairs, 73));
    checks.push(check("corner_near_twin_pairs", corner_pairs, 2));
    let swaps = interior_swaps + edge_swaps + corner_swaps;
    checks.push(check("near_twin_swaps", swaps, 125));

    // Half-turn tiles: a 180-degree turn in place breaks 4 - 2(1[N=S] + 1[E=W])
    // joints, so defect 2 means exactly one opposite pair equal.
    let is_half = |e: &[u8; 4]| usize::from(e[0] == e[2]) + usize::from(e[1] == e[3]) == 1;
    let half_turn = interior.iter().filter(|e| is_half(*e)).count();
    checks.push(check("interior_half_turn_defect2_tiles", half_turn, 23));

    // Quarter-turn tiles: defect 2 means exactly two edge positions fixed by
    // the turn. The fixed-position count is the same for the 90- and
    // 270-degree turns (both count cyclic adjacent-equal pairs), so each such
    // tile gives two distinct boards.
    let is_quarter = |e: &[u8; 4]| {
        let r = e2_kit::Piece::new(*e).rotated(1);
        e.iter().zip(r.iter()).filter(|(x, y)| x == y).count() == 2
    };
    let quarter_turn = interior.iter().filter(|e| is_quarter(*e)).count();
    checks.push(check("interior_quarter_turn_defect2_tiles", quarter_turn, 3));
    let turn_tiles = interior.iter().filter(|e| is_half(*e) || is_quarter(*e)).count();
    checks.push(check("interior_turn_defect2_tiles_union", turn_tiles, 23));
    let turn_moves = half_turn + 2 * quarter_turn;
    checks.push(check("interior_turn_defect2_moves", turn_moves, 29));

    // Every one of these moves lands on 478 from every solution, so their sum
    // is a floor on the single-move neighbours at 478, not a ceiling: swaps of
    // adjacent tiles that change the shared color, among others, can add more.
    checks.push(check("defect2_single_move_floor", swaps + turn_moves, 154));

    // --- Theorem 5: frame decomposition ---
    let interior_with_frame_colors = interior
        .iter()
        .filter(|e| e.iter().any(|&c| (1..=5).contains(&c)))
        .count();
    checks.push(check(
        "interior_tiles_carrying_colors_1_to_5",
        interior_with_frame_colors,
        0,
    ));

    let frame_color_half_edges: u32 = h[1..=5].iter().sum();
    checks.push(check("frame_color_half_edges", frame_color_half_edges, 120));
    let ring_per_color: Vec<u32> = h[1..=5].iter().map(|&x| x / 2).collect();
    checks.push(check("ring_joints_per_frame_color", ring_per_color, vec![12; 5]));

    // Inward demand vector: for each edge tile, the color of the edge opposite
    // its single gray edge.
    let mut inward = [0u32; 23];
    for e in pieces.iter().filter(|e| gray(e) == 1) {
        let g = e.iter().position(|&c| c == 0).unwrap();
        inward[e[(g + 2) % 4] as usize] += 1;
    }
    let inward_vec: Vec<u32> = inward[6..=22].to_vec();
    checks.push(check(
        "inward_demand_vector_colors_6_to_22",
        inward_vec,
        EXPECTED_INWARD.to_vec(),
    ));
    checks.push(check("inward_demand_total", inward[6..=22].iter().sum::<u32>(), 56));

    let all_ok = checks.iter().all(|c| c.ok);
    let report = Report {
        instance: instance.name,
        checks,
        all_ok,
    };
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
    if !all_ok {
        std::process::exit(1);
    }
}
