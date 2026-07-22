//! Step 2 of ../PLAN.md: small-N overlap-gap enumeration (claim C4).
//!
//! For each (n, C) point we generate seeded planted n x n instances (uniform
//! random internal adjacency colors in 1..=C, gray border), exhaustively
//! enumerate every perfect gray-legal placement of the tile bag by DFS with a
//! distinct-tile constraint, deduplicate solutions modulo whole-board rotation,
//! and histogram each solution's overlap with the planted board (max cell
//! agreement over the four board rotations).
//!
//! The claim under test is the TREND: at loose constraint density (mu ~ 0.2)
//! the overlap histogram is continuous; as mu drops toward E2's 0.009 the
//! middle bins empty out, leaving the planted solution (overlap 1) plus a few
//! globally orthogonal solutions (overlap ~0). Tier: seeded-statistical.
//!
//! Determinism: seeded RNG (kit XorShift), a fixed node budget per seed
//! (no wall-clock cutoffs), no timestamps in the output.

use e2_kit::{bucas_url, rotated, XorShift};
use std::collections::HashSet;

/// Hard node budget per (n, C, seed) enumeration: a "node" is a successful
/// tile placement. Deterministic truncation guard; truncated seeds are
/// reported per point.
const NODE_CAP: u64 = 30_000_000;
/// Seeds per (n, C) point.
const SEEDS: u32 = 20;

/// The planted instance: tiles in cell order (tile i solves cell i).
fn generate_planted(n: usize, colors: u8, seed: u32) -> Vec<[u8; 4]> {
    let mut rng = XorShift::new(seed.wrapping_mul(0x9E37_79B9).wrapping_add(colors as u32));
    // h[y][x]: color between (x,y)-(x+1,y); v[y][x]: between (x,y)-(x,y+1).
    let mut h = vec![vec![0u8; n - 1]; n];
    let mut v = vec![vec![0u8; n]; n - 1];
    for row in h.iter_mut().flatten() {
        *row = 1 + rng.next_below(u32::from(colors)) as u8;
    }
    for cell in v.iter_mut().flatten() {
        *cell = 1 + rng.next_below(u32::from(colors)) as u8;
    }
    let mut tiles = Vec::with_capacity(n * n);
    for y in 0..n {
        for x in 0..n {
            let up = if y == 0 { 0 } else { v[y - 1][x] };
            let right = if x == n - 1 { 0 } else { h[y][x] };
            let down = if y == n - 1 { 0 } else { v[y][x] };
            let left = if x == 0 { 0 } else { h[y][x - 1] };
            tiles.push([up, right, down, left]);
        }
    }
    tiles
}

/// Rotate a whole placement (cell -> quad) a quarter turn clockwise.
fn rotate_board(cells: &[[u8; 4]], n: usize) -> Vec<[u8; 4]> {
    let mut out = vec![[0u8; 4]; n * n];
    for y in 0..n {
        for x in 0..n {
            let (nx, ny) = (n - 1 - y, x);
            out[ny * n + nx] = rotated(cells[y * n + x], 1);
        }
    }
    out
}

/// Canonical signature of a placement modulo the 4 board rotations.
fn canonical_signature(cells: &[[u8; 4]], n: usize) -> Vec<u8> {
    let mut best: Option<Vec<u8>> = None;
    let mut cur = cells.to_vec();
    for _ in 0..4 {
        let flat: Vec<u8> = cur.iter().flatten().copied().collect();
        if best.as_ref().is_none_or(|b| flat < *b) {
            best = Some(flat);
        }
        cur = rotate_board(&cur, n);
    }
    best.expect("four rotations examined")
}

/// Overlap with the planted board: max over the 4 board rotations of the
/// fraction of cells whose oriented quad equals the planted quad.
fn overlap(cells: &[[u8; 4]], planted: &[[u8; 4]], n: usize) -> f64 {
    let mut best = 0usize;
    let mut cur = cells.to_vec();
    for _ in 0..4 {
        let agree = cur.iter().zip(planted).filter(|(a, b)| a == b).count();
        best = best.max(agree);
        cur = rotate_board(&cur, n);
    }
    best as f64 / (n * n) as f64
}

/// Step-1-analog constraint density of a small planted bag: interior-tile
/// half-edge collision to the fourth power, times 4 fits per interior tile.
fn empirical_mu(tiles: &[[u8; 4]], colors: u8) -> f64 {
    let mut census = vec![0u64; usize::from(colors) + 1];
    let mut interior_tiles = 0u64;
    for t in tiles {
        if t.iter().all(|&c| c != 0) {
            interior_tiles += 1;
            for &c in t {
                census[usize::from(c)] += 1;
            }
        }
    }
    let total: u64 = census.iter().sum();
    if total == 0 {
        return f64::NAN;
    }
    let collision: f64 = census.iter().map(|&h| (h as f64 / total as f64).powi(2)).sum();
    interior_tiles as f64 * 4.0 * collision.powi(4)
}

struct Enumeration {
    solutions: Vec<Vec<[u8; 4]>>,
    nodes: u64,
    truncated: bool,
}

/// Exhaustive DFS over row-major cells: every unused tile, every rotation
/// satisfying rim gray-legality and the up/left color constraints.
fn enumerate_perfect(tiles: &[[u8; 4]], n: usize) -> Enumeration {
    // Unique rotated quads per tile (skip symmetric repeats).
    let rots: Vec<Vec<[u8; 4]>> = tiles
        .iter()
        .map(|&t| {
            let mut v: Vec<[u8; 4]> = (0..4).map(|r| rotated(t, r)).collect();
            v.sort_unstable();
            v.dedup();
            v
        })
        .collect();
    let mut st = Enumeration { solutions: Vec::new(), nodes: 0, truncated: false };
    let mut used = vec![false; tiles.len()];
    let mut placed: Vec<[u8; 4]> = Vec::with_capacity(n * n);
    dfs(&rots, n, &mut used, &mut placed, &mut st);
    st
}

fn dfs(
    rots: &[Vec<[u8; 4]>],
    n: usize,
    used: &mut [bool],
    placed: &mut Vec<[u8; 4]>,
    st: &mut Enumeration,
) {
    if st.truncated {
        return;
    }
    let idx = placed.len();
    if idx == n * n {
        st.solutions.push(placed.clone());
        return;
    }
    let (x, y) = (idx % n, idx / n);
    let want_up = if y == 0 { 0 } else { placed[idx - n][2] };
    let want_left = if x == 0 { 0 } else { placed[idx - 1][1] };
    let last_col = x == n - 1;
    let last_row = y == n - 1;
    for t in 0..rots.len() {
        if used[t] {
            continue;
        }
        for &q in &rots[t] {
            if q[0] != want_up || q[3] != want_left {
                continue;
            }
            // Gray-legality on the open sides: gray exactly on the rim.
            if (q[1] == 0) != last_col || (q[2] == 0) != last_row {
                continue;
            }
            st.nodes += 1;
            if st.nodes > NODE_CAP {
                st.truncated = true;
                return;
            }
            used[t] = true;
            placed.push(q);
            dfs(rots, n, used, placed, st);
            placed.pop();
            used[t] = false;
            if st.truncated {
                return;
            }
        }
    }
}

fn main() {
    // C swept per n so the step-1-analog mu spans ~0.2 down to ~0.003.
    let sweep: &[(usize, &[u8])] = &[
        (4, &[3, 4, 5, 6, 7]),
        (5, &[4, 5, 6, 7, 8, 10]),
        (6, &[5, 6, 8, 10, 12]),
    ];

    let mut points = Vec::new();
    let mut example_urls = Vec::new();

    for &(n, cs) in sweep {
        for &colors in cs {
            let mut histogram = [0u64; 11];
            let mut per_seed_solutions = Vec::with_capacity(SEEDS as usize);
            let mut truncated_seeds = 0u32;
            let mut mu_sum = 0.0f64;
            for seed in 0..SEEDS {
                let tiles = generate_planted(n, colors, seed);
                mu_sum += empirical_mu(&tiles, colors);
                let res = enumerate_perfect(&tiles, n);
                if res.truncated {
                    truncated_seeds += 1;
                    per_seed_solutions.push(serde_json::Value::Null);
                    continue;
                }
                let mut seen = HashSet::new();
                let mut count = 0u64;
                for sol in &res.solutions {
                    if !seen.insert(canonical_signature(sol, n)) {
                        continue;
                    }
                    count += 1;
                    let ov = overlap(sol, &tiles, n);
                    let bin = (ov * 10.0).round() as usize;
                    histogram[bin.min(10)] += 1;
                    // Keep one representative planted + one orthogonal board
                    // as viewable URLs (first point that yields both).
                    if example_urls.is_empty() && ov >= 1.0 {
                        example_urls.push(serde_json::json!({
                            "role": "planted solution",
                            "n": n, "colors": colors, "seed": seed,
                            "url": bucas_url("planted", n as u8, n as u8, sol),
                        }));
                    } else if example_urls.len() == 1 && ov <= 0.1 {
                        example_urls.push(serde_json::json!({
                            "role": "orthogonal solution (same bag)",
                            "n": n, "colors": colors, "seed": seed,
                            "url": bucas_url("orthogonal", n as u8, n as u8, sol),
                        }));
                    }
                }
                per_seed_solutions.push(serde_json::json!(count));
            }
            let solved_seeds = SEEDS - truncated_seeds;
            let mid_band: u64 = histogram[2..=8].iter().sum();
            let total: u64 = histogram.iter().sum();
            points.push(serde_json::json!({
                "n": n,
                "colors": colors,
                "mu_mean": mu_sum / f64::from(SEEDS),
                "seeds": SEEDS,
                "truncated_seeds": truncated_seeds,
                "solved_seeds": solved_seeds,
                "solutions_mod_rotation_total": total,
                "per_seed_solutions": per_seed_solutions,
                "overlap_histogram_bins_0_to_1": histogram.to_vec(),
                "mid_band_solutions_bins_02_08": mid_band,
                "mid_band_fraction": if total > 0 { mid_band as f64 / total as f64 } else { 0.0 },
            }));
        }
    }

    let out = serde_json::json!({
        "step": "small-N overlap-gap enumeration (PLAN.md step 2, claim C4)",
        "tier": "seeded-statistical",
        "node_cap_per_seed": NODE_CAP,
        "overlap_convention": "max over 4 board rotations of the fraction of cells whose oriented edge quad equals the planted quad; solutions deduplicated modulo board rotation",
        "mu_convention": "step-1 analog: (n-2)^2 interior tiles x 4 x (interior-tile half-edge collision)^4",
        "points": points,
        "example_boards": example_urls,
    });
    println!("{}", serde_json::to_string_pretty(&out).expect("serializable"));
}
