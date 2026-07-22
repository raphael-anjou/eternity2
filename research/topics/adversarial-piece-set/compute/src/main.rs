//! Piece-set invariants behind the "adversarial by design" claim.
//!
//! Recomputes, from the kit's official instance alone:
//!   A1  rotation-orbit census (expect: all 256 pieces have orbit size 4)
//!   A1b unordered edge-multiset census (expect: 251 distinct, 5 twin pairs)
//!   A1c near-twin census, 3-of-4 same-position edges in stored orientation
//!       (expect: 114 pairs in 79 groups; orientation-dependent, see PLAN.md)
//!   A2  fixed-orientation matching cap sum_c min(E,W)+min(N,S)
//!       (expect: 307, vs geometric max 480; orientation-dependent)
//!   A3  colour budget: every non-border colour has an even side count and
//!       the pairing capacities sum_c floor(n_c/2) total exactly 480
//!
//! Prints one JSON document to stdout; exits 1 if any expected value fails.

#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet, HashMap};

use e2_kit::analysis::{color_half_edge_census, orbit_census};
use e2_kit::official_instance;

fn main() {
    let instance = official_instance(false);
    let n = instance.pieces.len();

    // A1: rotation orbits (kit census: full 4-orbit / 2-orbit / fixed).
    let (full_orbit, half_orbit, fixed_orbit) = orbit_census(&instance.pieces);
    let distinct_piece_rotations = 4 * full_orbit + 2 * half_orbit + fixed_orbit;

    // A1b: unordered edge-colour multisets.
    let mut by_multiset: HashMap<[u8; 4], Vec<u16>> = HashMap::new();
    for (pid, p) in instance.pieces.iter() {
        let mut m = p.edges;
        m.sort_unstable();
        by_multiset.entry(m).or_default().push(pid);
    }
    let distinct_multisets = by_multiset.len();
    let mut twin_pairs: Vec<(u16, u16)> = Vec::new();
    for ids in by_multiset.values() {
        if ids.len() >= 2 {
            for i in 0..ids.len() {
                for j in (i + 1)..ids.len() {
                    twin_pairs.push((ids[i].min(ids[j]), ids[i].max(ids[j])));
                }
            }
        }
    }
    twin_pairs.sort_unstable();

    // A1c: near-twins, 3-of-4 same-position edges in the stored orientation.
    // For each omitted side, group pieces by the remaining three edges in
    // position; every group of k >= 2 contributes C(k,2) pairs. A pair can in
    // principle recur across omitted sides, so distinct pairs are deduped.
    let mut near_groups = 0usize;
    let mut near_pairs: BTreeSet<(u16, u16)> = BTreeSet::new();
    for omit in 0..4 {
        let mut groups: HashMap<[u8; 3], Vec<u16>> = HashMap::new();
        for (pid, p) in instance.pieces.iter() {
            let mut key = [0u8; 3];
            let mut k = 0;
            for side in 0..4 {
                if side != omit {
                    key[k] = p.edges[side];
                    k += 1;
                }
            }
            groups.entry(key).or_default().push(pid);
        }
        for ids in groups.values() {
            if ids.len() >= 2 {
                near_groups += 1;
                for i in 0..ids.len() {
                    for j in (i + 1)..ids.len() {
                        near_pairs.insert((ids[i].min(ids[j]), ids[i].max(ids[j])));
                    }
                }
            }
        }
    }

    // A2: per-colour side counts in the stored orientation. Edge order is
    // URDL, i.e. N, E, S, W. Border colour 0 is excluded throughout. The kit
    // has no per-side census, so this stays local.
    let mut per_side: HashMap<u8, [u32; 4]> = HashMap::new();
    for (_, p) in instance.pieces.iter() {
        for side in 0..4 {
            let c = p.edges[side];
            if c != 0 {
                per_side.entry(c).or_insert([0; 4])[side] += 1;
            }
        }
    }
    let mut colors: Vec<u8> = per_side.keys().copied().collect();
    colors.sort_unstable();

    let mut matching_cap = 0u32; // A2
    for &c in &colors {
        let [north, east, south, west] = per_side[&c];
        matching_cap += east.min(west) + north.min(south);
    }

    // A3: colour budget from the kit's half-edge census (index 0 = rim grey,
    // excluded; the census is rotation-invariant so orientation is moot here).
    let census = color_half_edge_census(&instance.pieces);
    let mut pairing_capacity = 0u32;
    let mut odd_colors: Vec<usize> = Vec::new();
    for (c, &total) in census.iter().enumerate().skip(1) {
        pairing_capacity += total / 2;
        if total % 2 != 0 {
            odd_colors.push(c);
        }
    }

    // Expected values from the vault sources (see PLAN.md for provenance and
    // the orientation caveats on A1c and A2).
    let checks = [
        ("A1_all_pieces_full_orbit", full_orbit == 256 && n == 256),
        ("A1_1024_piece_rotations", distinct_piece_rotations == 1024),
        ("A1b_251_distinct_multisets", distinct_multisets == 251),
        ("A1b_5_twin_pairs", twin_pairs.len() == 5),
        ("A1c_114_near_twin_pairs", near_pairs.len() == 114),
        ("A1c_79_near_twin_groups", near_groups == 79),
        ("A2_matching_cap_307", matching_cap == 307),
        ("A3_all_colors_even", odd_colors.is_empty()),
        ("A3_pairing_capacity_480", pairing_capacity == 480),
    ];
    let all_pass = checks.iter().all(|(_, ok)| *ok);

    let doc = serde_json::json!({
        "topic": "adversarial-piece-set",
        "instance": { "name": instance.name, "pieces": n },
        "a1_rotation_orbits": {
            "full_orbit_pieces": full_orbit,
            "half_orbit_pieces": half_orbit,
            "fixed_pieces": fixed_orbit,
            "distinct_piece_rotations": distinct_piece_rotations,
        },
        "a1b_edge_multisets": {
            "distinct_multisets": distinct_multisets,
            "twin_pairs": twin_pairs,
        },
        "a1c_near_twins": {
            "pairs": near_pairs.len(),
            "groups": near_groups,
        },
        "a2_fixed_orientation_matching_cap": {
            "cap": matching_cap,
            "geometric_max": e2_kit::MAX_SCORE_16,
        },
        "a3_color_budget": {
            "non_border_colors": colors.len(),
            "odd_total_colors": odd_colors,
            "pairing_capacity": pairing_capacity,
        },
        "checks": checks
            .iter()
            .map(|(name, ok)| (name.to_string(), *ok))
            .collect::<BTreeMap<_, _>>(),
        "all_pass": all_pass,
    });
    println!("{}", serde_json::to_string_pretty(&doc).expect("serializable"));

    if !all_pass {
        eprintln!("FAIL: one or more expected invariants did not reproduce (see checks)");
        std::process::exit(1);
    }
}
