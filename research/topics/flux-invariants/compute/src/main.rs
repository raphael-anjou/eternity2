//! Flux and Character Invariants checker.
//!
//! Recomputes, from the official 256-piece instance, the instance facts and the
//! endgame certificate of the vol-226 "Flux and Character Invariants" paper
//! (ALG.md): piece-type histogram, half-edge colour multiplicities (all even,
//! frame colours border-only), the five clue placements, the complex/real/F2
//! ranks of the flux law, and the sound mod-2 endgame certificate that flags a
//! single rotation error with a detection rate rising toward full fill.
//!
//! Edge convention (kit): a piece's `edges` are URDL = (up, right, down, left).
//! The paper reads a tile clockwise from the top as (N, E, S, W); URDL is exactly
//! (N, E, S, W). A CW quarter turn is `e2_kit::rotated(e, 1)`, which sends
//! (N,E,S,W) -> (W,N,E,S), the paper's rho. So the kit's r=k is the paper's k CW
//! turns, and the flux vector rotates by i^k with no reindexing.
//!
//! Deterministic instance facts hard-fail (nonzero exit) on any mismatch; the
//! certificate detection rates are seeded and reported for comparison against
//! the source's 13% / 50% / 100% figures. Emits one JSON object on stdout.

use e2_kit::{official_instance, rotated, XorShift};
use serde_json::json;
use std::process::ExitCode;

const NCOL: usize = 23; // colours 0..=22 (0 = gray border)

fn fail(msg: &str) -> ExitCode {
    eprintln!("FLUX-INVARIANTS CHECK FAILED: {msg}");
    ExitCode::FAILURE
}

/// Base (rotation-0) flux Gaussian coefficient d^(0)_{t,c} = (r, s) where
/// r = [E=c]-[W=c], s = [S=c]-[N=c], on a URDL=(N,E,S,W) quad.
#[inline]
fn dvec(edges: [u8; 4], c: u8) -> (i32, i32) {
    let [n, e, s, w] = edges;
    let r = i32::from(e == c) - i32::from(w == c);
    let s = i32::from(s == c) - i32::from(n == c);
    (r, s)
}

/// Rank of an f64 matrix (rows x cols) by Gaussian elimination with partial
/// pivoting. Entries here are small integers, so a 1e-6 pivot threshold is safe.
fn rank_f64(mut m: Vec<Vec<f64>>) -> usize {
    if m.is_empty() {
        return 0;
    }
    let rows = m.len();
    let cols = m[0].len();
    let mut rank = 0;
    let mut col = 0;
    while rank < rows && col < cols {
        // find pivot
        let mut piv = rank;
        let mut best = m[rank][col].abs();
        for r in (rank + 1)..rows {
            if m[r][col].abs() > best {
                best = m[r][col].abs();
                piv = r;
            }
        }
        if best < 1e-6 {
            col += 1;
            continue;
        }
        m.swap(rank, piv);
        let inv = 1.0 / m[rank][col];
        for r in 0..rows {
            if r != rank && m[r][col].abs() > 1e-12 {
                let f = m[r][col] * inv;
                for c in col..cols {
                    m[r][c] -= f * m[rank][c];
                }
            }
        }
        rank += 1;
        col += 1;
    }
    rank
}

/// Rank over F2 of a matrix given as rows of bitsets (Vec<u64> words, cols bits).
fn rank_f2(mut rows: Vec<Vec<u64>>, cols: usize) -> usize {
    let words = cols.div_ceil(64);
    let mut rank = 0;
    for bit in 0..cols {
        let w = bit / 64;
        let mask = 1u64 << (bit % 64);
        // find a row at/after rank with this bit set
        let mut piv = None;
        for r in rank..rows.len() {
            if rows[r][w] & mask != 0 {
                piv = Some(r);
                break;
            }
        }
        let Some(p) = piv else { continue };
        rows.swap(rank, p);
        // clear this bit in all other rows
        for r in 0..rows.len() {
            if r != rank && rows[r][w] & mask != 0 {
                for k in 0..words {
                    rows[r][k] ^= rows[rank][k];
                }
            }
        }
        rank += 1;
        if rank == rows.len() {
            break;
        }
    }
    rank
}

/// F2 solvability (consistency) of A p = b by Gaussian elimination on the
/// augmented system: rank(A|b) == rank(A). Rows are (coeff-bitset over |vars|
/// bits, rhs-bit). Returns true if consistent (a parity assignment exists).
fn f2_consistent(mut rows: Vec<(Vec<u64>, u8)>, nvars: usize) -> bool {
    let words = nvars.div_ceil(64);
    let mut rank = 0;
    for bit in 0..nvars {
        let w = bit / 64;
        let mask = 1u64 << (bit % 64);
        let mut piv = None;
        for r in rank..rows.len() {
            if rows[r].0[w] & mask != 0 {
                piv = Some(r);
                break;
            }
        }
        let Some(p) = piv else { continue };
        rows.swap(rank, p);
        for r in 0..rows.len() {
            if r != rank && rows[r].0[w] & mask != 0 {
                for k in 0..words {
                    rows[r].0[k] ^= rows[rank].0[k];
                }
                rows[r].1 ^= rows[rank].1;
            }
        }
        rank += 1;
        if rank == rows.len() {
            break;
        }
    }
    // Inconsistent iff some row has all-zero coeffs but rhs = 1.
    for r in &rows {
        if r.0.iter().all(|&x| x == 0) && r.1 == 1 {
            return false;
        }
    }
    true
}

fn main() -> ExitCode {
    let inst = official_instance(true);
    let pieces = &inst.pieces;
    let n = pieces.len();
    if n != 256 {
        return fail(&format!("expected 256 pieces, got {n}"));
    }

    // ---- Instance fact 1: piece-type histogram --------------------------------
    let mut by_type = [0usize; 3]; // interior, edge, corner
    for (_, p) in pieces.iter() {
        let z = p.border_edge_count() as usize;
        if z > 2 {
            return fail("piece with more than 2 gray slots");
        }
        by_type[z] += 1;
    }
    if by_type != [196, 56, 4] {
        return fail(&format!("piece-type histogram {by_type:?} != [196, 56, 4]"));
    }

    // ---- Instance fact 2: half-edge colour multiplicities ---------------------
    let mut cnt_total = [0u32; NCOL];
    let mut cnt_interior = [0u32; NCOL];
    for (_, p) in pieces.iter() {
        let interior = p.border_edge_count() == 0;
        for &c in &p.edges {
            cnt_total[c as usize] += 1;
            if interior {
                cnt_interior[c as usize] += 1;
            }
        }
    }
    // Gray perimeter supply.
    if cnt_total[0] != 64 {
        return fail(&format!("gray (colour 0) has {} half-edges, expected 64", cnt_total[0]));
    }
    // Frame colours: non-gray colours never on an interior piece.
    let frame: Vec<u8> = (1u8..=22)
        .filter(|&c| cnt_total[c as usize] > 0 && cnt_interior[c as usize] == 0)
        .collect();
    if frame.len() != 5 {
        return fail(&format!("expected 5 frame colours, found {}: {frame:?}", frame.len()));
    }
    for &c in &frame {
        if cnt_total[c as usize] != 24 {
            return fail(&format!(
                "frame colour {c} has {} half-edges, expected 24",
                cnt_total[c as usize]
            ));
        }
    }
    // Every colour count even (necessary for a perfect half-edge matching).
    for c in 0..=22u8 {
        if cnt_total[c as usize] % 2 != 0 {
            return fail(&format!("colour {c} count {} is odd", cnt_total[c as usize]));
        }
    }
    // Non-frame interior colour multiplicities: paper says 6..10 at 48, 11..22 at 50.
    let mut mult_48 = 0;
    let mut mult_50 = 0;
    let mut nonframe_mults: Vec<u32> = Vec::new();
    for c in 1u8..=22 {
        if frame.contains(&c) {
            continue;
        }
        let m = cnt_total[c as usize];
        nonframe_mults.push(m);
        match m {
            48 => mult_48 += 1,
            50 => mult_50 += 1,
            _ => {}
        }
    }
    nonframe_mults.sort_unstable();
    // Paper: 5 colours at 48, 12 at 50 (colours 6-10 and 11-22).
    if mult_48 != 5 || mult_50 != 12 {
        return fail(&format!(
            "non-frame multiplicities: {mult_48} at 48, {mult_50} at 50 (expected 5 / 12); sorted {nonframe_mults:?}"
        ));
    }

    // ---- Instance fact 3: the five clue placements ----------------------------
    let width = inst.width as usize;
    let mut clues: Vec<serde_json::Value> = Vec::new();
    for h in &inst.hints {
        let cell = h.pos as usize;
        clues.push(json!({
            "piece": h.piece,
            "rot": h.rot,
            "cell": cell,
            "row": cell / width,
            "col": cell % width,
        }));
    }
    let all_clues_interior = inst
        .hints
        .iter()
        .all(|h| pieces.get(h.piece).map(|p| p.border_edge_count() == 0).unwrap_or(false));

    // ---- Flux law rank: complex, real, F2 -------------------------------------
    // Build the 22 x 256 base coefficient matrix d^(0)_{t,c} = r + i s.
    // rank_C M: represent complex row as a 512-wide real matrix over the pair
    // (Re, Im) is NOT how complex rank works; instead we compute complex rank by
    // Gaussian elimination over the reals on the 44 x 256 matrix whose rows are,
    // per colour, the real part row and the imaginary part row of d^(0). The
    // complex column rank equals the real rank of that stacked real matrix / … —
    // to avoid subtlety we report the real rank directly (the paper's "40 real
    // constraints") and the complex row rank separately via a complex GE.

    // Real stacked matrix: 2 rows per colour (Re and Im of d^(0)_{.,c}), 256 cols.
    let colours: Vec<u8> = (1u8..=22).collect();
    let mut real_rows: Vec<Vec<f64>> = Vec::new();
    for &c in &colours {
        let mut rre = vec![0.0f64; n];
        let mut rim = vec![0.0f64; n];
        for (t, (_, p)) in pieces.iter().enumerate() {
            let (r, s) = dvec(p.edges, c);
            rre[t] = f64::from(r);
            rim[t] = f64::from(s);
        }
        real_rows.push(rre);
        real_rows.push(rim);
    }
    let real_rank = rank_f64(real_rows.clone());

    // Complex row rank of the 22 x 256 matrix: emulate complex GE by stacking
    // each complex row as [Re | Im-shifted]? The cleanest exact route: the
    // complex row rank equals the real rank of the 22 x 512 matrix [Re(M) | Im(M)]
    // only for column rank over C of the augmented — instead we compute the
    // number of C-linearly-independent rows by a direct complex Gaussian
    // elimination.
    let complex_rank = {
        // rows of complex numbers (re, im)
        let mut rows: Vec<Vec<(f64, f64)>> = Vec::new();
        for &c in &colours {
            let mut row = vec![(0.0f64, 0.0f64); n];
            for (t, (_, p)) in pieces.iter().enumerate() {
                let (r, s) = dvec(p.edges, c);
                row[t] = (f64::from(r), f64::from(s));
            }
            rows.push(row);
        }
        complex_row_rank(rows, n)
    };

    // F2 parity constraints: per colour two equations on the 256 parity bits p_t
    //   real eq:  sum_t p_t * ((r+s) mod 2) = (sum_t r) mod 2
    //   imag eq:  sum_t p_t * ((r+s) mod 2) = (sum_t s) mod 2
    // (coefficient (r+s) mod 2 identical; constants differ). Rank of the coeff
    // matrix over F2.
    let words = n.div_ceil(64);
    let mut f2_rows: Vec<Vec<u64>> = Vec::new();
    let mut f2_aug: Vec<(Vec<u64>, u8)> = Vec::new();
    for &c in &colours {
        let mut coeff = vec![0u64; words];
        let mut sum_r = 0i32;
        let mut sum_s = 0i32;
        for (t, (_, p)) in pieces.iter().enumerate() {
            let (r, s) = dvec(p.edges, c);
            if (r + s).rem_euclid(2) == 1 {
                coeff[t / 64] |= 1u64 << (t % 64);
            }
            sum_r += r;
            sum_s += s;
        }
        // Two rows (real, imag) share the coeff bitset, differ in rhs.
        f2_rows.push(coeff.clone());
        f2_rows.push(coeff.clone());
        f2_aug.push((coeff.clone(), (sum_r.rem_euclid(2)) as u8));
        f2_aug.push((coeff, (sum_s.rem_euclid(2)) as u8));
    }
    let f2_rank = rank_f2(f2_rows, n);
    // Augmented rank (consistency check on the true solved parities below is a
    // separate soundness test); here confirm rank(A|b) == rank(A) as the paper does.
    let f2_aug_rank = {
        // build augmented as rows of (cols+1) bits
        let mut rows: Vec<Vec<u64>> = Vec::new();
        let augwords = (n + 1).div_ceil(64);
        for (coeff, rhs) in &f2_aug {
            let mut row = vec![0u64; augwords];
            for k in 0..words {
                row[k] = coeff[k];
            }
            if *rhs == 1 {
                row[n / 64] |= 1u64 << (n % 64);
            }
            rows.push(row);
        }
        rank_f2(rows, n + 1)
    };

    // Orthogonality to the census: the i->1 shadow (rotation-blind) is
    // sum_t (r+s) over each colour; if all even, the parity census is identically
    // zero and every flux constraint is invisible to colour counting.
    let census_shadow_all_zero = colours.iter().all(|&c| {
        let mut acc = 0i32;
        for (_, p) in pieces.iter() {
            let (r, s) = dvec(p.edges, c);
            acc += r + s;
        }
        acc.rem_euclid(2) == 0
    });

    // ---- Whole-board flux law holds on the official solution (soundness) ------
    // The official instance's identity placement (piece i @ cell i, rot from the
    // solved board) is not shipped, but the flux law is placement-agnostic: with
    // every tile at rotation 0 the base sum sum_t d^(0)_{t,c} is generally nonzero
    // (Nontriviality, paper 6.1); we record how many colours vanish at k=0.
    let mut base_nonzero_j1 = 0;
    for &c in &colours {
        let mut acc = (0i32, 0i32);
        for (_, p) in pieces.iter() {
            let (r, s) = dvec(p.edges, c);
            acc.0 += r;
            acc.1 += s;
        }
        if acc != (0, 0) {
            base_nonzero_j1 += 1;
        }
    }

    // =====================================================================
    // Endgame certificate: plant a solved board, inject one rotation error,
    // measure the mod-2 feasibility catch rate at fill 0.50 / 0.75 / 0.90.
    // =====================================================================
    // We plant on the OFFICIAL set itself (the endgame the paper targets): the
    // official solution's rotations are unknown, so we synthesise a self-planted
    // solution whose per-colour flux law is exactly zero by construction, then
    // test the certificate's soundness (never rejects a valid partial) and its
    // catch rate (rejects a partial with one illegal rotation), sweeping fill.
    //
    // A planted "solution" here is any rotation assignment k in {0,1,2,3}^256
    // satisfying the whole-board law sum_t i^{k_t} d^(0)_{t,c} = 0 for all c. The
    // official identity orientation need not satisfy it, so we instead run the
    // certificate on generated FRAMED solved boards, whose identity placement IS
    // a genuine valid tiling (piece i @ cell i, rot 0), giving a real k-vector to
    // corrupt. This matches the paper's "planted boards … same structure".

    let seed_base: u32 = std::env::args()
        .skip_while(|a| a != "--seed")
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);

    // Certificate over a generated framed solved 8x8 board (paper's endgame test
    // size), colours 22-capped -> use 8x8/7 like the paper's frame/interior split
    // is small; we use size 8, colours = frame_color_count-compatible max.
    let cert = run_certificate(seed_base);

    // ---- Emit -----------------------------------------------------------------
    let out = json!({
        "instance": "official 16x16, 256 pieces, 5 clues",
        "edge_convention": "kit URDL = paper (N,E,S,W); kit rot k = paper k CW turns; flux rotates by i^k",
        "instance_facts": {
            "piece_type_histogram": {"interior": by_type[0], "edge": by_type[1], "corner": by_type[2]},
            "gray_half_edges": cnt_total[0],
            "frame_colors": frame,
            "frame_half_edges_each": 24,
            "frame_supply": frame.iter().map(|&c| cnt_total[c as usize]).sum::<u32>(),
            "nonframe_at_48": mult_48,
            "nonframe_at_50": mult_50,
            "all_color_counts_even": true,
            "clues": clues,
            "all_clues_interior": all_clues_interior,
        },
        "flux_law_rank": {
            "colors": 22,
            "complex_rank": complex_rank,
            "real_constraint_rank": real_rank,
            "f2_parity_rank": f2_rank,
            "f2_parity_augmented_rank": f2_aug_rank,
            "f2_consistent": f2_rank == f2_aug_rank,
            "census_shadow_all_zero": census_shadow_all_zero,
            "base_orientation_nonzero_colors_j1": base_nonzero_j1,
        },
        "endgame_certificate": cert,
        "expected_source": {
            "piece_type_histogram": [196, 56, 4],
            "gray_half_edges": 64,
            "frame_colors_count": 5,
            "frame_half_edges_each": 24,
            "nonframe_at_48": 5,
            "nonframe_at_50": 12,
            "complex_rank": 22,
            "real_constraint_rank": 40,
            "f2_parity_rank": 21,
            "certificate_catch_rate_at_050_075_090": [0.13, 0.50, 1.00],
        },
    });
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
    ExitCode::SUCCESS
}

/// Complex row rank of `rows` (each a length-`cols` vector of (re, im)), by
/// Gaussian elimination over C with partial pivoting on |pivot|.
fn complex_row_rank(mut rows: Vec<Vec<(f64, f64)>>, cols: usize) -> usize {
    let nrows = rows.len();
    let mut rank = 0;
    let mut col = 0;
    let cabs = |z: (f64, f64)| (z.0 * z.0 + z.1 * z.1).sqrt();
    let cmul = |a: (f64, f64), b: (f64, f64)| (a.0 * b.0 - a.1 * b.1, a.0 * b.1 + a.1 * b.0);
    let cinv = |z: (f64, f64)| {
        let d = z.0 * z.0 + z.1 * z.1;
        (z.0 / d, -z.1 / d)
    };
    while rank < nrows && col < cols {
        let mut piv = rank;
        let mut best = cabs(rows[rank][col]);
        for r in (rank + 1)..nrows {
            let v = cabs(rows[r][col]);
            if v > best {
                best = v;
                piv = r;
            }
        }
        if best < 1e-6 {
            col += 1;
            continue;
        }
        rows.swap(rank, piv);
        let inv = cinv(rows[rank][col]);
        for r in 0..nrows {
            if r != rank && cabs(rows[r][col]) > 1e-12 {
                let f = cmul(rows[r][col], inv);
                for c in col..cols {
                    let prod = cmul(f, rows[rank][c]);
                    rows[r][c].0 -= prod.0;
                    rows[r][c].1 -= prod.1;
                }
            }
        }
        rank += 1;
        col += 1;
    }
    rank
}

/// The mod-2 endgame certificate over a generated framed solved board.
///
/// Plants a genuine valid tiling (a framed solved board: piece i @ cell i, rot
/// 0), records its per-tile rotation parities, then for a range of fill
/// fractions (0.50 / 0.75 / 0.90) runs `trials` per fill:
///   - a clean partial (first `fill*N` tiles placed at their true parities):
///     must always PASS (soundness);
///   - a corrupted partial (one placed tile's rotation flipped by an odd amount):
///     PASS/FAIL is the detection outcome.
/// The certificate solves the residual F2 parity system for consistency: the
/// remaining (unplaced) tiles' parities must satisfy the two-per-colour equations
/// with constants shifted by the placed prefix. Inconsistent => dead partial.
fn run_certificate(seed_base: u32) -> serde_json::Value {
    use e2_kit::generator;
    // Board: framed solved 8x8, the paper's endgame test size. Colours = 13
    // gives the same 5 frame / 8 interior split shape as the official set (5
    // frame colours, a richer interior palette than a bare 7), so the per-colour
    // parity equations have realistic density. The framed generator is the
    // expensive step, so we build a POOL of boards ONCE and reuse it across every
    // fill fraction and both the clean and corrupted probes.
    let size: u8 = 8;
    let colors: u8 = 13;
    let ncells = usize::from(size) * usize::from(size);

    let n_boards = 30usize;
    // Pre-build the board pool: for each board, the base (rotation-0) quads and
    // the planted solution rotations ktrue that restore the solved orientation.
    struct Planted {
        base: Vec<[u8; 4]>,
        ktrue: Vec<u8>,
    }
    let mut pool: Vec<Planted> = Vec::with_capacity(n_boards);
    for b in 0..n_boards {
        let seed = seed_base.wrapping_mul(1_000_003).wrapping_add(b as u32);
        let solved = generator::generate_solved_framed(size, colors, seed, true);
        let mut rng = XorShift::new(0xF10C_0000 ^ seed);
        let mut base = Vec::with_capacity(ncells);
        let mut ktrue = Vec::with_capacity(ncells);
        for q in &solved.pieces {
            // Scramble each tile by a random rotation rk; the solution rotation
            // that restores it is k = (4-rk)%4. This gives the certificate a
            // genuine nontrivial planted k-vector (the whole-board flux law then
            // holds for {ktrue} on the base coefficients — the paper's planted
            // form with real rotation freedom).
            let rk = rng.next_below(4) as u8;
            base.push(rotated(*q, rk));
            ktrue.push(((4 - rk) % 4) as u8);
        }
        pool.push(Planted { base, ktrue });
    }

    // Fill sweep, including near-full to exhibit the rise toward 100%.
    let fills = [0.50f64, 0.75, 0.90, 0.95];
    let orderings = 30u32; // random placement orders per board per fill
    let mut results = Vec::new();
    let mut soundness_ok = true;

    for &fill in &fills {
        let mut caught = 0u32;
        let mut total = 0u32;
        let mut clean_pass = 0u32;
        let mut clean_total = 0u32;
        for (b, pl) in pool.iter().enumerate() {
            let mut rng = XorShift::new(
                0xCE27_0000 ^ seed_base ^ ((fill * 1000.0) as u32) ^ (b as u32).wrapping_mul(7),
            );
            for _ in 0..orderings {
                let mut order: Vec<usize> = (0..ncells).collect();
                rng.shuffle(&mut order);
                let placed_n = ((fill * ncells as f64).round() as usize).min(ncells);
                let placed: Vec<usize> = order[..placed_n].to_vec();
                let remaining: Vec<usize> = order[placed_n..].to_vec();

                // Clean certificate: placed at true parity must always pass.
                let clean =
                    certificate_consistent(&pl.base, colors, &placed, &pl.ktrue, &remaining, None);
                clean_total += 1;
                if clean {
                    clean_pass += 1;
                } else {
                    soundness_ok = false;
                }

                // Corrupted: flip one placed tile's rotation parity (odd amount).
                if !placed.is_empty() {
                    let pick = placed[rng.next_below(placed.len() as u32) as usize];
                    let corrupted = certificate_consistent(
                        &pl.base,
                        colors,
                        &placed,
                        &pl.ktrue,
                        &remaining,
                        Some(pick),
                    );
                    total += 1;
                    if !corrupted {
                        caught += 1;
                    }
                }
            }
        }
        results.push(json!({
            "fill": fill,
            "trials": total,
            "clean_partials_passing": clean_pass,
            "clean_partials_total": clean_total,
            "single_error_caught": caught,
            "catch_rate": f64::from(caught) / f64::from(total.max(1)),
        }));
    }

    json!({
        "board": format!("generated framed solved {size}x{size}, {colors} colours, planted rotations"),
        "seed_base": seed_base,
        "board_pool": n_boards,
        "orderings_per_board_per_fill": orderings,
        "soundness_no_valid_partial_rejected": soundness_ok,
        "by_fill": results,
    })
}

/// Decide the residual mod-2 parity system for a partial fill.
///
/// `base[t]` are rotation-0 quads; `ktrue[t]` the planted solution rotation.
/// `placed` are the placed cells (parity known = ktrue % 2, unless corrupted),
/// `remaining` the unplaced cells (free parity bits). `corrupt` optionally names
/// a placed cell whose recorded parity is flipped (injected rotation error).
///
/// Whole-board law per colour c (mod 2): sum_t p_t*((r+s) mod2) = (sum_t r) mod2
/// and likewise with s. Splitting placed vs remaining moves the placed part to
/// the RHS constant. Returns true iff the remaining free bits admit a solution.
fn certificate_consistent(
    base: &[[u8; 4]],
    colors: u8,
    placed: &[usize],
    ktrue: &[u8],
    remaining: &[usize],
    corrupt: Option<usize>,
) -> bool {
    let nrem = remaining.len();
    let rem_index: std::collections::HashMap<usize, usize> =
        remaining.iter().enumerate().map(|(i, &t)| (t, i)).collect();
    let words = nrem.div_ceil(64).max(1);

    // parity of a placed tile (corrupted flips one bit).
    let placed_parity = |t: usize| -> u8 {
        let base_p = ktrue[t] & 1;
        if Some(t) == corrupt {
            base_p ^ 1
        } else {
            base_p
        }
    };

    let mut rows: Vec<(Vec<u64>, u8)> = Vec::new();
    for c in 1..=colors {
        // coefficient a_t = (r+s) mod 2 for tile t at colour c.
        // real-part constant = sum_t r ; imag-part constant = sum_t s.
        let mut coeff_rem = vec![0u64; words];
        let mut rhs_r = 0i32; // sum over ALL tiles of r
        let mut rhs_s = 0i32;
        let mut placed_contrib = 0u8; // sum over placed of a_t * p_t (mod 2)
        for (t, q) in base.iter().enumerate() {
            let [nn, ee, ss, ww] = *q;
            let rr = i32::from(ee == c) - i32::from(ww == c);
            let ssv = i32::from(ss == c) - i32::from(nn == c);
            rhs_r += rr;
            rhs_s += ssv;
            let a = ((rr + ssv).rem_euclid(2)) as u8;
            if a == 1 {
                if let Some(&ri) = rem_index.get(&t) {
                    coeff_rem[ri / 64] |= 1u64 << (ri % 64);
                }
            }
        }
        // placed contribution to the LHS (a_t * p_t) for placed tiles:
        for &t in placed {
            let [nn, ee, ss, ww] = base[t];
            let rr = i32::from(ee == c) - i32::from(ww == c);
            let ssv = i32::from(ss == c) - i32::from(nn == c);
            let a = ((rr + ssv).rem_euclid(2)) as u8;
            placed_contrib ^= a & placed_parity(t);
        }
        // Equation (real): sum_rem a_t p_t = (rhs_r mod2) XOR placed_contrib
        let rhs_real = ((rhs_r.rem_euclid(2)) as u8) ^ placed_contrib;
        let rhs_imag = ((rhs_s.rem_euclid(2)) as u8) ^ placed_contrib;
        rows.push((coeff_rem.clone(), rhs_real));
        rows.push((coeff_rem, rhs_imag));
    }
    f2_consistent(rows, nrem)
}
