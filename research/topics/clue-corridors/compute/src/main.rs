//! Stage-1 checker for the clue-corridors topic.
//!
//! Recomputes, from the kit's bundled official instance and nothing else:
//!
//! 1. the clue geometry (cells, pairwise Manhattan distances, minimum gap);
//! 2. the uniform-model branching table b_k = 4n / C^k;
//! 3. the real per-side colour supply of the inner (frame-free) piece set;
//! 4. the exact transfer-matrix corridor counts: walks of length L between
//!    interior colours, total and to one fixed target colour, plus the
//!    mean-field distinct-piece correction for L = 10.
//!
//! Prints a single JSON object to stdout. Expected headline numbers (from the
//! source study): 196 inner pieces, 17 inner colours, supply mean 46.1
//! (min 43, max 49), minimum clue distance 10, L=10 fixed start-and-target
//! walk count ~2.4e15 (within the reported min..max range), depletion factor
//! 0.792, corrected corridor count ~2e15 (source cites ~1.9e15 for the
//! specific clue colour pair).

use e2_kit::analysis::{clue_cells, piece_classes};
use e2_kit::{official_instance, rotated};
use serde_json::json;

/// Colour ids are u8; the official set uses 0 (border) plus interior colours.
const NC: usize = 32;

type Mat = Vec<Vec<u128>>;

fn matmul(a: &Mat, b: &Mat) -> Mat {
    let mut out = vec![vec![0u128; NC]; NC];
    for i in 0..NC {
        for k in 0..NC {
            let v = a[i][k];
            if v == 0 {
                continue;
            }
            for j in 0..NC {
                out[i][j] += v * b[k][j];
            }
        }
    }
    out
}

fn mat_pow(t: &Mat, l: u32) -> Mat {
    let mut out = t.clone();
    for _ in 1..l {
        out = matmul(&out, t);
    }
    out
}

fn main() {
    let inst = official_instance(true);
    let w = usize::from(inst.width);

    // 1. Clue geometry from the bundled hints (kit census helper).
    let clues: Vec<(usize, usize, usize)> = clue_cells(&inst)
        .into_iter()
        .map(|pos| (pos, pos % w, pos / w))
        .collect();
    let mut dists = Vec::new();
    for i in 0..clues.len() {
        for j in (i + 1)..clues.len() {
            let (_, xi, yi) = clues[i];
            let (_, xj, yj) = clues[j];
            let d = xi.abs_diff(xj) + yi.abs_diff(yj);
            dists.push(json!({
                "cells": [clues[i].0, clues[j].0],
                "manhattan": d,
            }));
        }
    }
    let min_gap = dists
        .iter()
        .map(|d| d["manhattan"].as_u64().unwrap())
        .min()
        .unwrap();

    // 2. Inner pieces (kit piece-class partition) and the colours on them.
    let inner: Vec<[u8; 4]> = piece_classes(&inst.pieces)
        .interior
        .iter()
        .map(|&pid| inst.pieces.edges(pid))
        .collect();
    let n_inner = inner.len();
    let mut on_inner = [false; NC];
    for e in &inner {
        for &c in e {
            on_inner[usize::from(c)] = true;
        }
    }
    let icols: Vec<usize> = (1..NC).filter(|&c| on_inner[c]).collect();

    // 3. Uniform-model branching table, C = declared interior colour count.
    let c_declared = f64::from(inst.num_colors);
    let four_n = 4.0 * n_inner as f64;
    let b_k: Vec<f64> = (1..=4).map(|k| four_n / c_declared.powi(k)).collect();

    // 4. Transfer matrix over inner pieces: T[west][east], all four rotations.
    let mut t: Mat = vec![vec![0u128; NC]; NC];
    for e in &inner {
        for r in 0..4 {
            let re = rotated(*e, r);
            t[usize::from(re[3])][usize::from(re[1])] += 1;
        }
    }

    // Per-side supply per inner colour = row sums of T (rotation closure makes
    // every side identical). Expected mean 46.1, min 43, max 49.
    let row_sums: Vec<u128> = icols.iter().map(|&a| t[a].iter().sum()).collect();
    let supply_min = *row_sums.iter().min().unwrap();
    let supply_max = *row_sums.iter().max().unwrap();
    let supply_mean = row_sums.iter().sum::<u128>() as f64 / row_sums.len() as f64;

    // Walk counts at the lengths the claim cites. Length L walks correspond to
    // a corridor of L cells; the centre-to-SW clue gap of 10 gives L = 10.
    //
    // Semantics match the source study: a corridor grows out of one clue, so
    // the START colour is fixed (the clue's facing colour). "from_fixed_start"
    // is the row sum of T^L (all endpoints); "fixed_start_and_target" is a
    // single entry of T^L (the far clue also fixes its facing colour).
    let mut walks = Vec::new();
    let mut l10_entry_mean = 0.0;
    for l in [5u32, 9, 10] {
        let p = mat_pow(&t, l);
        let row_totals: Vec<u128> = icols
            .iter()
            .map(|&a| icols.iter().map(|&b| p[a][b]).sum())
            .collect();
        let all_pairs: u128 = row_totals.iter().sum();
        let row_mean = all_pairs as f64 / row_totals.len() as f64;
        let entry_mean = all_pairs as f64 / (row_totals.len() * row_totals.len()) as f64;
        let entries: Vec<u128> = icols
            .iter()
            .flat_map(|&a| icols.iter().map(|&b| p[a][b]).collect::<Vec<_>>())
            .collect();
        if l == 10 {
            l10_entry_mean = entry_mean;
        }
        walks.push(json!({
            "length": l,
            "from_fixed_start_mean": row_mean,
            "from_fixed_start_min": row_totals.iter().min().unwrap().to_string(),
            "from_fixed_start_max": row_totals.iter().max().unwrap().to_string(),
            "fixed_start_and_target_mean": entry_mean,
            "fixed_start_and_target_min": entries.iter().min().unwrap().to_string(),
            "fixed_start_and_target_max": entries.iter().max().unwrap().to_string(),
            "all_pairs_total": all_pairs.to_string(),
        }));
    }

    // Mean-field distinct-piece correction for a 10-cell corridor:
    // prod_{j=0..9} (1 - j/n_inner). Expected 0.792.
    let depletion: f64 = (0..10).map(|j| 1.0 - j as f64 / n_inner as f64).product();
    let corrected_l10 = l10_entry_mean * depletion;

    let out = json!({
        "instance": inst.name,
        "clues": {
            "cells": clues.iter().map(|c| c.0).collect::<Vec<_>>(),
            "xy": clues.iter().map(|c| [c.1, c.2]).collect::<Vec<_>>(),
            "pairwise": dists,
            "min_manhattan_gap": min_gap,
        },
        "pieces": {
            "inner_pieces": n_inner,
            "declared_interior_colors": inst.num_colors,
            "colors_on_inner_pieces": icols.len(),
            "frame_only_colors": usize::from(inst.num_colors) - icols.len(),
        },
        "branching_uniform_model": {
            "formula": "b_k = 4n / C^k, n = inner pieces, C = declared interior colours",
            "b_1": b_k[0], "b_2": b_k[1], "b_3": b_k[2], "b_4": b_k[3],
            "pruning_threshold_k": (four_n.ln() / c_declared.ln()).ceil(),
        },
        "per_side_supply": {
            "mean": supply_mean,
            "min": supply_min.to_string(),
            "max": supply_max.to_string(),
        },
        "corridor_walks": walks,
        "length_10_corridor": {
            "fixed_start_and_target_walks_mean": l10_entry_mean,
            "depletion_factor": depletion,
            "distinct_piece_estimate": corrected_l10,
            "claim": "a 1-wide corridor between the closest clue pair (gap 10) admits ~1.9e15 fillings; the commitment is vacuous",
        },
    });
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}
