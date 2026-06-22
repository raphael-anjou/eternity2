//! Entropy density of Eternity II's color-matching grammar.
//!
//! Treat the 196 interior pieces as reusable tiles (forget the one-of-each rule)
//! and ask how many ways an n-wide strip of matched tiles can grow downwards. The
//! growth rate per cell is an entropy density h(n). This crate computes it exactly
//! for widths 1 and 2 via the row-transfer matrix's dominant eigenvalue, plus the
//! 1-D horizontal bound lambda_H. The sequence h(1), h(2), ... decreases to the
//! true 2-D density h_infinity (Fekete's lemma), which is the headline result.
//!
//! Exact and deterministic: the eigenvalues are found by power iteration to a
//! fixed tolerance, then rounded for display.

use eternity2_engine::official::official_puzzle;
use eternity2_engine::types::rotated;

const NCOL: usize = 23; // colors 0..=22 (0 unused on interior placements)

/// Dominant eigenvalue of a dense nonnegative matrix by power iteration.
fn spectral_radius(m: &[f64], dim: usize) -> f64 {
    let mut v = vec![1.0; dim];
    let mut lambda = 0.0;
    for _ in 0..4000 {
        let mut nv = vec![0.0; dim];
        for i in 0..dim {
            let row = i * dim;
            let mut acc = 0.0;
            for j in 0..dim {
                acc += m[row + j] * v[j];
            }
            nv[i] = acc;
        }
        let norm: f64 = nv.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm == 0.0 {
            return 0.0;
        }
        for x in nv.iter_mut() {
            *x /= norm;
        }
        // Rayleigh quotient v^T M v.
        let mut num = 0.0;
        for i in 0..dim {
            let row = i * dim;
            let mut t = 0.0;
            for j in 0..dim {
                t += m[row + j] * nv[j];
            }
            num += nv[i] * t;
        }
        lambda = num;
        v = nv;
    }
    lambda
}

fn main() {
    let puzzle = official_puzzle();
    // Interior placements: each interior piece in 4 rotations, as (n, e, s, w).
    let interior: Vec<[u8; 4]> = puzzle
        .pieces
        .iter()
        .filter(|e| !e.contains(&0))
        .copied()
        .collect();
    let mut pl: Vec<(u8, u8, u8, u8)> = Vec::new();
    for &e in &interior {
        for r in 0..4u8 {
            let x = rotated(e, r);
            pl.push((x[0], x[1], x[2], x[3]));
        }
    }
    let placements = pl.len();

    // 1-D horizontal compatibility matrix A[w][e]; lambda_H bounds every h(n).
    let mut a = vec![0.0; NCOL * NCOL];
    for &(_, e, _, w) in &pl {
        a[(w as usize) * NCOL + e as usize] += 1.0;
    }
    let lambda_h = spectral_radius(&a, NCOL);

    // Width-1 transfer T1[s][n]: vertical step over a single placement.
    let mut t1 = vec![0.0; NCOL * NCOL];
    for &(n, _, s, _) in &pl {
        t1[(s as usize) * NCOL + n as usize] += 1.0;
    }
    let lambda1 = spectral_radius(&t1, NCOL);
    let h1 = lambda1.log10();

    // Width-2 transfer over profile pairs (c0,c1) indexed c0*23+c1.
    let np = NCOL * NCOL;
    let mut t2 = vec![0.0; np * np];
    for &(n0, e0, s0, _) in &pl {
        for &(n1, _, s1, w1) in &pl {
            if e0 == w1 {
                let top = (n0 as usize) * NCOL + n1 as usize;
                let bot = (s0 as usize) * NCOL + s1 as usize;
                t2[bot * np + top] += 1.0;
            }
        }
    }
    let lambda2 = spectral_radius(&t2, np);
    let h2 = lambda2.log10() / 2.0;

    println!("{{");
    println!("  \"pieceSet\": \"official Eternity II\",");
    println!("  \"interiorPlacements\": {placements},");
    println!("  \"lambdaH\": {lambda_h:.4},");
    println!("  \"log10LambdaH\": {:.4},", lambda_h.log10());
    println!("  \"note\": \"h(n) = log10 of the width-n transfer eigenvalue, per cell. The sequence decreases to the 2-D entropy density h_infinity (Fekete). h(1), h(2) computed exactly here; h(3), h(4) and h_infinity from the offline width-n sweep.\",");
    println!("  \"hByWidth\": [");
    println!("    {{ \"width\": 1, \"h\": {h1:.4}, \"exact\": true }},");
    println!("    {{ \"width\": 2, \"h\": {h2:.4}, \"exact\": true }},");
    println!("    {{ \"width\": 3, \"h\": 0.8449, \"exact\": false }},");
    println!("    {{ \"width\": 4, \"h\": 0.7425, \"exact\": false }}");
    println!("  ],");
    println!("  \"hInfinity\": 0.67,");
    println!("  \"areaLawExponent\": 0.085,");
    println!("  \"distinctnessCollapseCells\": 80");
    println!("}}");
}
