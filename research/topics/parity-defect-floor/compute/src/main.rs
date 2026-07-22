//! Bag census for the parity-defect-floor claim.
//!
//! Recomputes, from the official 256-piece instance, every bag statistic the
//! vol-226 theorems rest on, and compares each against the claimed value:
//!
//! * half-edge counts h(c) per color, their evenness, and the exact-fit
//!   identity (colored total = 960 = 2 x 480) — premises of the parity
//!   invariant and the 479 gap;
//! * no rotationally self-symmetric tile, and the 4/56/196 class split —
//!   premise of the defect floor 2;
//! * the defect-2 generator census: 50 interior near-twin pairs, 23 interior
//!   half-turn tiles, 3 interior quarter-turn tiles, total bound 76;
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
    let mut h = [0u32; 23];
    for e in &pieces {
        for &c in e {
            h[c as usize] += 1;
        }
    }
    let h_vec: Vec<u32> = h.to_vec();
    let expected_h_vec: Vec<u32> = EXPECTED_H.iter().map(|&(_, v)| v).collect();
    checks.push(check("half_edge_counts_h0_to_h22", h_vec, expected_h_vec));

    let colored_total: u32 = h[1..].iter().sum();
    checks.push(check("exact_fit_identity_colored_total", colored_total, 2 * JOINTS));

    let all_even = h[1..].iter().all(|&x| x % 2 == 0);
    checks.push(check("all_colored_h_even_forces_479_gap", all_even, true));

    // --- No self-symmetric tile (premise of Theorem 2, floor = 2) ---
    let self_symmetric = pieces
        .iter()
        .filter(|&&e| (1..4).any(|r| e2_kit::Piece::new(e).rotated(r) == e))
        .count();
    checks.push(check("rotationally_self_symmetric_tiles", self_symmetric, 0));

    // --- Class split 4 / 56 / 196 ---
    let gray = |e: &[u8; 4]| e.iter().filter(|&&c| c == 0).count();
    let corners = pieces.iter().filter(|e| gray(e) == 2).count();
    let edge_tiles = pieces.iter().filter(|e| gray(e) == 1).count();
    let interior: Vec<[u8; 4]> = pieces.iter().copied().filter(|e| gray(e) == 0).collect();
    checks.push(check("corner_tiles", corners, 4));
    checks.push(check("edge_tiles", edge_tiles, 56));
    checks.push(check("interior_tiles", interior.len(), 196));

    // --- Theorem 3 census: defect-2 generators ---
    // Near-twin pairs: interior tiles that in some orientations agree on
    // exactly 3 of the 4 edge positions. Rotating both tiles together
    // preserves the agreement count, so fixing A at rotation 0 and trying all
    // four rotations of B covers every orientation pair.
    let mut twin_pairs = 0usize;
    for i in 0..interior.len() {
        for j in (i + 1)..interior.len() {
            let a = interior[i];
            let is_twin = (0..4).any(|r| {
                let b = e2_kit::Piece::new(interior[j]).rotated(r);
                a.iter().zip(b.iter()).filter(|(x, y)| x != y).count() == 1
            });
            if is_twin {
                twin_pairs += 1;
            }
        }
    }
    checks.push(check("interior_near_twin_pairs", twin_pairs, 50));

    // Half-turn tiles: a 180-degree turn in place breaks 4 - 2(1[N=S] + 1[E=W])
    // joints, so defect 2 means exactly one opposite pair equal.
    let half_turn = interior
        .iter()
        .filter(|e| usize::from(e[0] == e[2]) + usize::from(e[1] == e[3]) == 1)
        .count();
    checks.push(check("interior_half_turn_defect2_tiles", half_turn, 23));

    // Quarter-turn tiles: defect 2 means exactly two edge positions fixed by
    // the turn. The fixed-position count is the same for the 90- and
    // 270-degree turns (both count cyclic adjacent-equal pairs), so this is a
    // per-tile property.
    let quarter_turn = interior
        .iter()
        .filter(|&&e| {
            let r = e2_kit::Piece::new(e).rotated(1);
            e.iter().zip(r.iter()).filter(|(x, y)| x == y).count() == 2
        })
        .count();
    checks.push(check("interior_quarter_turn_defect2_tiles", quarter_turn, 3));

    checks.push(check(
        "defect2_generator_bound",
        twin_pairs + half_turn + quarter_turn,
        76,
    ));

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
