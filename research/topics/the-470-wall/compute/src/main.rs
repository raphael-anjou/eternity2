//! Instance-side exact computations for the-470-wall topic.
//!
//! Everything here is a function of the official 256-piece bag, read through
//! the starter kit's `official_instance`. No search, no sampling: the output
//! is exact and deterministic.
//!
//! Computed (see ../PLAN.md for the claim list and expected values):
//! * the color economy: piece classes, per-color half-edge counts, the
//!   frame-ring / interior split and its collision probabilities p_f, p_i;
//! * the constraint-density parameter mu = 196 * 4 * (interior collision)^4;
//! * the annealed first moment log10 E[#480] for placements uncorrelated with
//!   the planted solution;
//! * the exact annealed score landscape: log10 Pr[S = s] for S the sum of
//!   60 Bernoulli(p_f) + 420 Bernoulli(p_i), by log-space convolution, and
//!   the corresponding log10 count of uncorrelated configurations;
//! * the rotation-duplicate count of the real bag (expected: exactly 0).
//!
//! The piece-set censuses (classes, per-color half-edges, rotation orbits)
//! come from `e2_kit::analysis`; only the frame/interior split and the
//! probability arithmetic are topic-specific.

use e2_kit::analysis::{canonical_key, color_half_edge_census, piece_classes};
use e2_kit::official_instance;

/// ln(n!) by direct summation (n <= 256 here, exactness over speed).
fn ln_factorial(n: u64) -> f64 {
    (2..=n).map(|k| (k as f64).ln()).sum()
}

/// ln(exp(a) + exp(b)), stable for widely separated inputs.
fn ln_add(a: f64, b: f64) -> f64 {
    if a == f64::NEG_INFINITY {
        return b;
    }
    if b == f64::NEG_INFINITY {
        return a;
    }
    let (hi, lo) = if a >= b { (a, b) } else { (b, a) };
    hi + (lo - hi).exp().ln_1p()
}

/// Convolve `n` iid Bernoulli(p) indicators into a running log-space pmf.
fn convolve_bernoulli(ln_pmf: &mut Vec<f64>, n: usize, p: f64) {
    let ln_p = p.ln();
    let ln_q = (1.0 - p).ln();
    for _ in 0..n {
        let mut next = vec![f64::NEG_INFINITY; ln_pmf.len() + 1];
        for (s, &lp) in ln_pmf.iter().enumerate() {
            if lp == f64::NEG_INFINITY {
                continue;
            }
            next[s] = ln_add(next[s], lp + ln_q);
            next[s + 1] = ln_add(next[s + 1], lp + ln_p);
        }
        *ln_pmf = next;
    }
}

fn main() {
    let instance = official_instance(false);
    let ln10 = std::f64::consts::LN_10;

    // ---- Piece classes and per-color half-edge census (kit censuses) ----
    let classes = piece_classes(&instance.pieces);
    let (corners, edge_pieces, interior_pieces) =
        (classes.corners.len(), classes.edges.len(), classes.interior.len());
    assert_eq!((corners, edge_pieces, interior_pieces), (4, 56, 196));

    // Whole-bag census: index 0 is the gray rim supply, 1.. the real colors.
    let census = color_half_edge_census(&instance.pieces);
    let half_edges: Vec<u64> = census.iter().map(|&n| u64::from(n)).collect();
    let total_half_edges: u64 = half_edges.iter().skip(1).sum();
    assert_eq!(total_half_edges, 960, "non-gray half-edges must be 960");

    // Interior-tile-only census, and which colors ever appear on an interior
    // tile (those colors form the interior subsystem; the rest are the frame
    // ring, colors 1..=5 in the official numbering).
    let mut interior_tile_half_edges = vec![0u64; half_edges.len()];
    for &pid in &classes.interior {
        let piece = instance.pieces.get(pid).expect("interior piece id");
        for &c in &piece.edges {
            if c != 0 {
                interior_tile_half_edges[c as usize] += 1;
            }
        }
    }

    let frame_colors: Vec<usize> = (1..half_edges.len())
        .filter(|&c| half_edges[c] > 0 && interior_tile_half_edges[c] == 0)
        .collect();
    let interior_colors: Vec<usize> = (1..half_edges.len())
        .filter(|&c| half_edges[c] > 0 && interior_tile_half_edges[c] > 0)
        .collect();
    let frame_half_edges: u64 = frame_colors.iter().map(|&c| half_edges[c]).sum();
    let interior_half_edges: u64 = interior_colors.iter().map(|&c| half_edges[c]).sum();
    assert_eq!(frame_half_edges, 120, "frame ring must carry 60 adjacencies");
    assert_eq!(interior_half_edges, 840);

    // Per-adjacency collision probabilities for a configuration uncorrelated
    // with the planted board: two independent half-edge draws share a color.
    let p_f: f64 = frame_colors
        .iter()
        .map(|&c| (half_edges[c] as f64 / frame_half_edges as f64).powi(2))
        .sum();
    let p_i: f64 = interior_colors
        .iter()
        .map(|&c| (half_edges[c] as f64 / interior_half_edges as f64).powi(2))
        .sum();

    // ---- C1: constraint density mu --------------------------------------
    // Interior collision over the 196 interior tiles' 784 half-edges; mu is
    // the expected number of tile x rotation fits for a fully constrained
    // interior cell.
    let interior_tile_total: u64 = interior_tile_half_edges.iter().sum();
    let interior_collision: f64 = interior_tile_half_edges
        .iter()
        .map(|&h| (h as f64 / interior_tile_total as f64).powi(2))
        .sum();
    let mu = 196.0 * 4.0 * interior_collision.powi(4);

    // ---- C3: annealed first moment for uncorrelated placements ----------
    let log10_w_geom =
        (ln_factorial(4) + ln_factorial(56) + ln_factorial(196) + 196.0 * 4f64.ln()) / ln10;
    let log10_e480 = log10_w_geom + 60.0 * p_f.log10() + 420.0 * p_i.log10();

    // ---- C6: exact score landscape (log-space convolution) --------------
    let mut ln_pmf = vec![0.0f64]; // pmf of the empty sum: Pr[S=0] = 1
    convolve_bernoulli(&mut ln_pmf, 60, p_f);
    convolve_bernoulli(&mut ln_pmf, 420, p_i);
    assert_eq!(ln_pmf.len(), 481);

    let mean = 60.0 * p_f + 420.0 * p_i;
    let sd = (60.0 * p_f * (1.0 - p_f) + 420.0 * p_i * (1.0 - p_i)).sqrt();

    let probe_scores = [37usize, 200, 400, 460, 470, 480];
    let landscape: Vec<serde_json::Value> = probe_scores
        .iter()
        .map(|&s| {
            let log10_pr = ln_pmf[s] / ln10;
            serde_json::json!({
                "score": s,
                "log10_pr": log10_pr,
                "log10_uncorrelated_configs": log10_w_geom + log10_pr,
            })
        })
        .collect();

    // ---- C8 (real-set side): rotation-duplicate count -------------------
    // Kit canonical key: two pieces collide iff one is a rotation of the other.
    let mut seen = std::collections::HashSet::new();
    let mut rotation_duplicates = 0u32;
    for (_, piece) in instance.pieces.iter() {
        if !seen.insert(canonical_key(piece.edges)) {
            rotation_duplicates += 1;
        }
    }

    let out = serde_json::json!({
        "instance": "official-16x16",
        "piece_classes": { "corners": corners, "edges": edge_pieces, "interior": interior_pieces },
        "non_gray_half_edges": total_half_edges,
        "frame_colors": frame_colors,
        "interior_colors": interior_colors,
        "p_frame": p_f,
        "p_interior": p_i,
        "interior_tile_collision": interior_collision,
        "mu": mu,
        "log10_w_geom": log10_w_geom,
        "log10_expected_480s_uncorrelated": log10_e480,
        "score_mean_uniform": mean,
        "score_sd_uniform": sd,
        "score_landscape": landscape,
        "rotation_duplicate_tiles": rotation_duplicates,
    });
    println!("{}", serde_json::to_string_pretty(&out).expect("serializable"));
}
