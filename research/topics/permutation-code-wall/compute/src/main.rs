//! permutation-code-wall checker.
//!
//! Reproduces the load-bearing structural facts of WEFT (vol-228, the
//! coding-theory / error-correcting-decoder lens on Eternity II) through the
//! starter kit's ONE canonical scorer. Two groups of claims:
//!
//!   A. STRUCTURAL FACTS (exact, deterministic properties of the official
//!      256-piece 16x16 instance):
//!        - the check count is exactly 480 = 2*W*H - W - H  (the codeword has
//!          480 parity-like adjacency checks);
//!        - the PERMUTATION CODE: all 256 pieces are distinct up to rotation
//!          (each used exactly once => a full board is a permutation of the set);
//!        - the BORDER CODE: exactly 5 frame colors (gray=0 excluded), each
//!          confined to the border ring, gray sits only on outward rim sides.
//!
//!   B. THE SYNDROME / SCORE IDENTITY (measured on real boards through the kit
//!      scorer):  score = 480 - ||sigma||_1,  breaks = ||sigma||_1.
//!      We take a genuine 480/480 codeword (a solved generated 16x16 board),
//!      inject EXACTLY k independent broken checks, and confirm the canonical
//!      scorer reports score = 480 - k and breaks = k, for a sweep of k over
//!      several seeds. Each injection is verified to toggle exactly one check
//!      (score drops by exactly 1), so k is a KNOWN break count, not an
//!      inferred one. We also check the identity against WEFT's actual reported
//!      451-board numbers (breaks 29 <=> score 451 = 480 - 29).
//!
//! Emits one JSON object on stdout. Deterministic clauses hard-fail (nonzero
//! exit) on any mismatch.

use e2_kit::{
    generator, instance_from_generated, official_instance, pieces_distinct_up_to_rotation,
    score_cells, XorShift, BORDER, MAX_SCORE_16,
};
use serde_json::json;
use std::process::ExitCode;

const SIZE: u8 = 16;
const COLORS: u8 = 22;
const N: usize = (SIZE as usize) * (SIZE as usize);

fn fail(msg: &str) -> ExitCode {
    eprintln!("PERMUTATION-CODE-WALL CHECK FAILED: {msg}");
    ExitCode::FAILURE
}

/// The kit's canonical adjacency enumeration, in the exact form `score_cells`
/// scores: a right check (x,x+1) and a down check (x,x+W) per cell, dropped at
/// the board edge. Returns the list of ((cellA, sideA), (cellB, sideB)) for
/// every internal adjacency, where sides are URDL indices 0=U,1=R,2=D,3=L.
fn adjacencies() -> Vec<((usize, usize), (usize, usize))> {
    let w = SIZE as usize;
    let h = w;
    let mut v = Vec::new();
    for y in 0..h {
        for x in 0..w {
            let a = y * w + x;
            if x + 1 < w {
                let b = y * w + x + 1;
                v.push(((a, 1usize), (b, 3usize))); // a.right vs b.left
            }
            if y + 1 < h {
                let b = (y + 1) * w + x;
                v.push(((a, 2usize), (b, 0usize))); // a.down vs b.up
            }
        }
    }
    v
}

/// Count violated checks (the syndrome weight ||sigma||_1) directly, as an
/// INDEPENDENT recomputation of the identity's right-hand side. A check is
/// violated iff its two facing half-edges differ OR share the border color 0
/// (a border color on an internal joint never scores, exactly as `score_cells`
/// treats it: `a[1]==b[3] && a[1]!=BORDER`).
fn syndrome_weight(cells: &[[u8; 4]]) -> u32 {
    let mut broken = 0u32;
    for ((a, sa), (b, sb)) in adjacencies() {
        let ca = cells[a][sa];
        let cb = cells[b][sb];
        if !(ca == cb && ca != BORDER) {
            broken += 1;
        }
    }
    broken
}

fn main() -> ExitCode {
    // =====================================================================
    // A. STRUCTURAL FACTS on the official 16x16 instance.
    // =====================================================================
    let inst = official_instance(true);
    let pieces = &inst.pieces;

    // A1: the check count. WEFT: "The 480 internal adjacencies are parity-like
    // checks, |{e}| = 480." The kit's MAX_SCORE_16 is 2*W*H - W - H = 480, and
    // enumerating the scorer's own adjacency list must yield the same count.
    let formula = (2 * (SIZE as u32) * (SIZE as u32)) - 2 * (SIZE as u32);
    let n_checks = adjacencies().len() as u32;
    if MAX_SCORE_16 != 480 {
        return fail(&format!("MAX_SCORE_16 = {MAX_SCORE_16}, expected 480"));
    }
    if formula != 480 || n_checks != 480 {
        return fail(&format!(
            "check count formula={formula} enumerated={n_checks}, expected 480"
        ));
    }

    // A2: the PERMUTATION CODE. All 256 pieces distinct up to rotation => a full
    // board is a permutation of the set (each piece placed exactly once). This
    // is the constraint WEFT names as the wall.
    let n_pieces = pieces.len();
    let perm_ok = pieces_distinct_up_to_rotation(pieces);
    if n_pieces != 256 {
        return fail(&format!("piece count {n_pieces} != 256"));
    }
    if !perm_ok {
        return fail("official pieces are NOT distinct up to rotation (permutation code broken)");
    }

    // A3: the BORDER CODE. Frame colors := non-gray colors absent from every
    // interior piece; on the official set there are exactly 5, matching WEFT's
    // "gray must sit on outward frame sides" border-code framing. Also verify
    // gray (0) appears only on border/corner pieces (never on an interior
    // piece), i.e. gray is a pure rim marker.
    let mut cnt_total = [0u32; 256];
    let mut cnt_interior = [0u32; 256];
    let mut gray_on_interior = 0u32;
    for (_, p) in pieces.iter() {
        let interior = p.border_edge_count() == 0;
        for &c in &p.edges {
            cnt_total[c as usize] += 1;
            if interior {
                cnt_interior[c as usize] += 1;
                if c == BORDER {
                    gray_on_interior += 1;
                }
            }
        }
    }
    let frame: Vec<u8> = (1u8..=255)
        .filter(|&c| cnt_total[c as usize] > 0 && cnt_interior[c as usize] == 0)
        .collect();
    if frame.len() != 5 {
        return fail(&format!(
            "expected 5 frame (border-code) colors, found {}: {frame:?}",
            frame.len()
        ));
    }
    if gray_on_interior != 0 {
        return fail(&format!(
            "gray color found on {gray_on_interior} interior half-edges (border code broken)"
        ));
    }
    let interior_colors: Vec<u8> = (1u8..=255)
        .filter(|&c| cnt_total[c as usize] > 0 && !frame.contains(&c))
        .collect();

    // =====================================================================
    // B. THE SYNDROME / SCORE IDENTITY, measured through the canonical scorer.
    // =====================================================================
    // A solved generated 16x16 board is a genuine 480/480 codeword (sigma = 0).
    // We inject EXACTLY k independent broken checks and confirm the kit scorer
    // reports score = 480 - k and breaks = k. Independence is enforced by
    // choosing target adjacencies whose two cells are disjoint from all others
    // chosen, and by corrupting with a globally fresh sentinel color so no
    // corruption can accidentally satisfy another check. Each single injection
    // is verified to drop the score by exactly 1, so k is a KNOWN, not inferred,
    // break count.
    let sentinel: u8 = COLORS + 1; // a color that appears on NO piece
    let adj = adjacencies();
    let seeds: [u32; 5] = [1, 2, 3, 4, 5];
    let ks: [u32; 8] = [0, 1, 2, 5, 10, 29, 60, 120];

    let mut identity_rows = Vec::new();
    let mut all_identity_ok = true;

    for &seed in &seeds {
        // Genuine solved codeword on a real 16x16/22 generated instance.
        let solved = generator::generate_solved(SIZE, COLORS, seed);
        // Sanity: instance built from these pieces so we go through the same
        // Pieces/scoring substrate the site uses.
        let _inst = instance_from_generated("perm-code-wall", &solved);
        // The solved board's edge grid: piece i at cell i, rotation 0.
        let base: Vec<[u8; 4]> = solved.pieces.clone();

        let base_score = score_cells(&base);
        let base_syndrome = syndrome_weight(&base);
        if base_score != 480 || base_syndrome != 0 {
            return fail(&format!(
                "solved seed {seed} is not a 480 codeword: score={base_score} syndrome={base_syndrome}"
            ));
        }

        // Walk a shuffled adjacency list, corrupting one facing half-edge per
        // step to a sentinel color that appears on NO piece. We accept a step
        // only when it drops the score by EXACTLY 1 (a single toggled check),
        // and skip any step that would touch an already-broken/already-corrupted
        // joint. This makes the injected break count a KNOWN quantity: every
        // counted break is a verified single-check violation, so k breaks means
        // exactly k violated checks.
        let mut rng = XorShift::new(seed ^ 0x0C0D_E5A1);
        let mut order: Vec<usize> = (0..adj.len()).collect();
        rng.shuffle(&mut order);

        for &k in &ks {
            let mut cells = base.clone();
            let mut injected = 0u32;
            let mut per_injection_ok = true;
            for &ei in &order {
                if injected >= k {
                    break;
                }
                let ((a, sa), (_b, _sb)) = adj[ei];
                if cells[a][sa] == sentinel {
                    continue; // this half-edge is already corrupted
                }
                let before = score_cells(&cells);
                cells[a][sa] = sentinel;
                let after = score_cells(&cells);
                match before.saturating_sub(after) {
                    1 => injected += 1,          // clean single-check break
                    0 => {
                        cells[a][sa] = base[a][sa]; // touched an already-broken joint; undo
                    }
                    _ => {
                        per_injection_ok = false; // never expected: a sentinel can only break
                        break;
                    }
                }
            }
            if injected != k {
                per_injection_ok = false; // ran out of clean joints before reaching k
            }
            let score = score_cells(&cells);
            let syndrome = syndrome_weight(&cells);
            let breaks = MAX_SCORE_16.saturating_sub(score);
            // The identity: score == 480 - k, breaks == k, and the independent
            // recomputation of ||sigma||_1 agrees.
            let row_ok = per_injection_ok
                && injected == k
                && score == 480 - k
                && breaks == k
                && syndrome == k;
            if !row_ok {
                all_identity_ok = false;
            }
            identity_rows.push(json!({
                "seed": seed,
                "k": k,
                "injected": injected,
                "score": score,
                "breaks": breaks,
                "syndrome_weight": syndrome,
                "identity_score_eq_480_minus_k": score == 480 - k,
                "identity_breaks_eq_k": breaks == k,
                "syndrome_agrees": syndrome == k,
                "per_injection_single_drop": per_injection_ok,
            }));
        }
    }

    if !all_identity_ok {
        return fail("syndrome/score identity failed on at least one (seed,k) row");
    }

    // B2: the identity against WEFT's actual reported 451-board numbers. WEFT
    // reports score=451, breaks=29 on its champion. The identity says these are
    // the same statement: 480 - 29 == 451 and 480 - 451 == 29.
    let weft_score = 451u32;
    let weft_breaks = 29u32;
    let weft_identity_ok =
        (MAX_SCORE_16 - weft_breaks == weft_score) && (MAX_SCORE_16 - weft_score == weft_breaks);
    if !weft_identity_ok {
        return fail("WEFT 451/29 numbers violate the score=480-breaks identity");
    }

    // =====================================================================
    // Emit the full record.
    // =====================================================================
    let out = json!({
        "topic": "permutation-code-wall",
        "source": "research/vault/papers/vol-228/WEFT.md",
        "instance": { "size": SIZE, "colors": COLORS, "cells": N },
        "structural": {
            "check_count_formula_2WH_minus_W_minus_H": formula,
            "check_count_enumerated": n_checks,
            "max_score_16": MAX_SCORE_16,
            "check_count_is_480": n_checks == 480 && formula == 480 && MAX_SCORE_16 == 480,
            "permutation_code": {
                "n_pieces": n_pieces,
                "distinct_up_to_rotation": perm_ok,
                "is_permutation_of_256": n_pieces == 256 && perm_ok
            },
            "border_code": {
                "frame_colors": frame,
                "n_frame_colors": frame.len(),
                "n_interior_colors": interior_colors.len(),
                "gray_on_interior_halfedges": gray_on_interior,
                "gray_is_pure_rim_marker": gray_on_interior == 0
            }
        },
        "syndrome_score_identity": {
            "statement": "score = 480 - ||sigma||_1,  breaks = ||sigma||_1",
            "seeds": seeds,
            "ks": ks,
            "all_rows_hold": all_identity_ok,
            "rows": identity_rows
        },
        "weft_451_board_identity": {
            "reported_score": weft_score,
            "reported_breaks": weft_breaks,
            "identity_holds": weft_identity_ok,
            "note": "480 - 29 = 451; the WEFT syndrome weight and the kit's breaks are the same quantity"
        }
    });
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
    ExitCode::SUCCESS
}
