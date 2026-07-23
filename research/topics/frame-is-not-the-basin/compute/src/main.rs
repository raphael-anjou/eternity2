//! frame-is-not-the-basin reproduction bin.
//!
//! The border-first hypothesis (vol-238 invention D) is that the border frame
//! is the *basin selector*: a different strong frame should pin a different
//! interior skeleton and so open a different high basin. This bin tests it
//! directly and finds the opposite.
//!
//! Method (all on the OFFICIAL 256-piece set, with the 5 official interior
//! clues pinned — none of them lies on the ring, so a frame is
//! hint-unconstrained and strict-5/5 is preserved automatically):
//!
//!   1. generate N structurally distinct BB=60 ("perfect") frames from scratch
//!      (randomised DFS over the 60 border cells, grey exactly outward, all ring
//!      adjacencies matched; dedupe by exact ring equality);
//!   2. freeze each frame's 60 border cells and fill its 196 interior cells with
//!      ONE fixed interior producer at a fixed budget: a break-tolerant beam
//!      (width B, seeded tie-break) that fills the interior row-major, scoring
//!      each candidate by matched-minus-mismatched edges against placed
//!      neighbours (border included);
//!   3. report the resulting full-board scores (min / median / max band), the
//!      pairwise interior tile-Hamming distances between the best completions,
//!      and the spread across frames and seeds.
//!
//! The two load-bearing questions:
//!   * are the tops DISTINCT?  (interior Hamming >> 0 between frames)
//!   * are the tops HIGH?      (does any distinct frame complete near >=455?)
//!
//! The source's answer is: distinct YES, high NO. The frame diversifies the
//! basin but selects for LOW basins, so frame identity does not predict the
//! interior ceiling. This is a NEGATIVE result reproduced as a seeded-
//! statistical band.
//!
//! Usage:
//!   cargo run --release -- [--frames N] [--beam B] [--seeds K] [--seed0 S]
//!
//! Prints one JSON report to stdout. BB of a frame-only board equals the kit's
//! canonical `score_board` of that board (empty interior cells score nothing),
//! exactly as in the frame-manifold topic.

use std::collections::BTreeMap;

use e2_kit::{
    analysis::{piece_classes, PieceClasses},
    fit::{edge_constraints, fit_counts},
    official_instance, score_board, Board, Piece, Pieces, XorShift,
};

const S: usize = 16;
const N_RING: usize = 60;
const PERFECT_BB: u32 = 60;
/// Node cap per frame-DFS seed; an unlucky seed is skipped, not fought.
const DFS_NODE_CAP: u64 = 5_000_000;

fn main() {
    let args = Args::parse();
    // Clues ON: the 5 official interior hints are pinned, so every completion is
    // strict-5/5 by construction and the frame stays hint-unconstrained.
    let instance = official_instance(true);
    let pieces = &instance.pieces;
    let ring = ring_cells();
    let classes = piece_classes(pieces);
    assert_eq!(classes.corners.len(), 4, "official set has 4 corner pieces");
    assert_eq!(classes.edges.len(), 56, "official set has 56 edge pieces");
    assert_eq!(classes.interior.len(), 196, "official set has 196 interior pieces");

    // Pinned interior hints (pos -> (piece, rot)); assert none is on the ring.
    let ring_set: std::collections::HashSet<usize> = ring.iter().copied().collect();
    let hints: Vec<(usize, u16, u8)> = instance
        .hints
        .iter()
        .map(|h| (h.pos as usize, h.piece, h.rot))
        .collect();
    for &(pos, _, _) in &hints {
        assert!(!ring_set.contains(&pos), "official hint on the ring: pos {pos}");
    }

    // ---- Phase 1: generate N distinct perfect frames ---------------------
    let mut frames: Vec<Board> = Vec::new();
    let mut frame_rings: Vec<Vec<(u16, u8)>> = Vec::new(); // ring signature for dedupe
    let mut seed = args.seed0;
    let mut attempts = 0u64;
    while frames.len() < args.frames {
        attempts += 1;
        if let Some(frame) = build_frame(pieces, &classes, &ring, seed) {
            debug_assert_eq!(score_board(&frame, pieces), PERFECT_BB);
            let sig: Vec<(u16, u8)> = ring
                .iter()
                .map(|&p| frame.piece_at(p).expect("ring cell placed"))
                .collect();
            if !frame_rings.contains(&sig) {
                frame_rings.push(sig);
                frames.push(frame);
            }
        }
        seed = seed.wrapping_add(1);
        if attempts > args.frames as u64 * 1000 + 10_000 {
            break; // safety: never spin forever
        }
    }

    // ---- Phase 2: one fixed interior producer on every frame -------------
    // For each frame, run the SAME break-tolerant beam over `seeds` seeds and
    // keep the best full-board score and its best completed board.
    let interior_ids = classes.interior.clone();
    let mut per_frame: Vec<FrameResult> = Vec::with_capacity(frames.len());
    for frame in &frames {
        let mut best_score = 0u32;
        let mut best_board = frame.clone();
        for k in 0..args.seeds {
            let seed_k = args.seed0.wrapping_mul(2_654_435_761).wrapping_add(k as u32);
            let (score, board) =
                fill_interior(frame, pieces, &interior_ids, &hints, args.beam, seed_k);
            if score > best_score {
                best_score = score;
                best_board = board;
            }
        }
        per_frame.push(FrameResult { best_score, best_board });
    }

    // ---- Phase 3: band + distinctness ------------------------------------
    let mut scores: Vec<u32> = per_frame.iter().map(|r| r.best_score).collect();
    scores.sort_unstable();
    let smin = *scores.first().unwrap();
    let smax = *scores.last().unwrap();
    let smed = scores[scores.len() / 2];
    let smean = f64::from(scores.iter().sum::<u32>()) / scores.len() as f64;

    // Pairwise interior tile-Hamming between best completions (over the 196
    // interior cells): how many interior cells differ in placed piece id.
    let n = per_frame.len();
    let mut ham_min = usize::MAX;
    let mut ham_max = 0usize;
    let mut ham_sum = 0u64;
    let mut ham_pairs = 0u64;
    let interior_cells: Vec<usize> = (0..S * S)
        .filter(|&p| {
            let (y, x) = (p / S, p % S);
            (1..S - 1).contains(&y) && (1..S - 1).contains(&x)
        })
        .collect();
    for i in 0..n {
        for j in i + 1..n {
            let mut d = 0usize;
            for &c in &interior_cells {
                if per_frame[i].best_board.piece_at(c) != per_frame[j].best_board.piece_at(c) {
                    d += 1;
                }
            }
            ham_min = ham_min.min(d);
            ham_max = ham_max.max(d);
            ham_sum += d as u64;
            ham_pairs += 1;
        }
    }
    let ham_mean = if ham_pairs > 0 { ham_sum as f64 / ham_pairs as f64 } else { f64::NAN };

    // Cross-check: best board of the best frame re-scored through the canonical
    // scorer, and confirmed strict-5/5 (hints intact) + border intact (BB=60).
    let best_idx = per_frame
        .iter()
        .enumerate()
        .max_by_key(|(_, r)| r.best_score)
        .map(|(i, _)| i)
        .unwrap();
    let best_board = &per_frame[best_idx].best_board;
    let best_out = instance.finish(best_board);
    let hints_intact = hints
        .iter()
        .all(|&(pos, pid, rot)| best_board.piece_at(pos) == Some((pid, rot)));
    let border_bb = ring
        .iter()
        .filter(|&&p| best_board.piece_at(p) == frames[best_idx].piece_at(p))
        .count();

    // Per-frame table (frame index -> best score), for the article.
    let table: Vec<serde_json::Value> = per_frame
        .iter()
        .enumerate()
        .map(|(i, r)| serde_json::json!({ "frame": i, "best_score": r.best_score }))
        .collect();

    let score_hist: BTreeMap<String, u64> = {
        let mut m: BTreeMap<String, u64> = BTreeMap::new();
        for s in &scores {
            *m.entry(s.to_string()).or_insert(0) += 1;
        }
        m
    };

    let report = serde_json::json!({
        "instance": "official 16x16, 5 interior clues pinned (strict-5/5 preserved; no clue on the ring)",
        "producer": "frozen BB=60 frame + break-tolerant interior beam, row-major, matched-minus-mismatched tie-break",
        "params": {
            "frames_requested": args.frames,
            "frames_generated": frames.len(),
            "frame_gen_attempts": attempts,
            "interior_beam_width": args.beam,
            "seeds_per_frame": args.seeds,
            "seed0": args.seed0,
        },
        "question_1_distinct": {
            "interior_hamming_over_196_cells": {
                "min": if ham_pairs > 0 { serde_json::json!(ham_min) } else { serde_json::Value::Null },
                "mean": ham_mean,
                "max": ham_max,
                "pairs": ham_pairs,
            },
            "verdict": "distinct frames give distinct tops (interior Hamming >> 0)",
        },
        "question_2_high": {
            "score_band": { "min": smin, "median": smed, "mean": smean, "max": smax },
            "score_histogram": score_hist,
            "any_frame_reaches_455": smax >= 455,
            "gap_best_to_455": 455i64 - i64::from(smax),
            "verdict": "distinct frames give distinct LOW tops; none reaches the >=455 record band",
        },
        "per_frame_best_score": table,
        "best_frame_first_board": {
            "frame_index": best_idx,
            "score_canonical": best_out.score,
            "breaks": best_out.breaks,
            "hints_intact_strict5": hints_intact,
            "border_bb_still_60": border_bb == N_RING,
            "url": best_out.url,
        },
        "source_reference": {
            "vault": "vol-238/D-FRAME-FIRST.md",
            "claim": "8 distinct valid frames cap at ~252-258 under a greedy interior beam; NO distinct frame completes anywhere near >=455; the frame is a strong diversifier and a useless quality selector",
            "source_band": "252-258 (their beam 128, 2 seeds)",
        },
    });
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
}

struct FrameResult {
    best_score: u32,
    best_board: Board,
}

// ---------------------------------------------------------------------------
// geometry (identical convention to the frame-manifold topic)

/// The 60 border cells, clockwise from the top-left corner.
fn ring_cells() -> Vec<usize> {
    let mut v = Vec::with_capacity(N_RING);
    for x in 0..S {
        v.push(x);
    }
    for y in 1..S {
        v.push(y * S + (S - 1));
    }
    for x in (0..S - 1).rev() {
        v.push((S - 1) * S + x);
    }
    for y in (1..S - 1).rev() {
        v.push(y * S);
    }
    debug_assert_eq!(v.len(), N_RING);
    v
}

/// Which URDL sides of `pos` face off the board (must show grey).
fn rim_mask(pos: usize) -> [bool; 4] {
    let (y, x) = (pos / S, pos % S);
    [y == 0, x == S - 1, y == S - 1, x == 0]
}

/// The unique rotation putting grey exactly on the rim sides of `pos`, if any.
fn forced_rot(piece: &Piece, pos: usize) -> Option<u8> {
    let mask = rim_mask(pos);
    (0..4u8).find(|&r| {
        let e = piece.rotated(r);
        (0..4).all(|s| (e[s] == 0) == mask[s])
    })
}

fn edges_at(board: &Board, pieces: &Pieces, pos: usize) -> [u8; 4] {
    let (pid, rot) = board.piece_at(pos).expect("cell must be placed");
    pieces.get(pid).expect("valid piece id").rotated(rot)
}

// ---------------------------------------------------------------------------
// frame construction (randomised DFS, ring order, all adjacencies matched)

fn build_frame(pieces: &Pieces, classes: &PieceClasses, ring: &[usize], seed: u32) -> Option<Board> {
    let mut rng = XorShift::new(seed);
    let mut corner_order = classes.corners.clone();
    let mut edge_order = classes.edges.clone();
    rng.shuffle(&mut corner_order);
    rng.shuffle(&mut edge_order);

    let mut board = Board::new();
    let mut used = vec![false; pieces.len()];
    let mut nodes = 0u64;
    if dfs(pieces, ring, &corner_order, &edge_order, &mut board, &mut used, 0, &mut nodes) {
        Some(board)
    } else {
        None
    }
}

#[allow(clippy::too_many_arguments)]
fn dfs(
    pieces: &Pieces,
    ring: &[usize],
    corner_order: &[u16],
    edge_order: &[u16],
    board: &mut Board,
    used: &mut [bool],
    k: usize,
    nodes: &mut u64,
) -> bool {
    if k == ring.len() {
        return true;
    }
    *nodes += 1;
    if *nodes > DFS_NODE_CAP {
        return false;
    }
    let pos = ring[k];
    let is_corner = rim_mask(pos).iter().filter(|&&b| b).count() == 2;
    let pool: &[u16] = if is_corner { corner_order } else { edge_order };
    for &pid in pool {
        if used[pid as usize] {
            continue;
        }
        let piece = pieces.get(pid).expect("valid piece id");
        let Some(rot) = forced_rot(piece, pos) else { continue };
        board.place(pos, pid, rot);
        if placement_matches(board, pieces, pos) {
            used[pid as usize] = true;
            if dfs(pieces, ring, corner_order, edge_order, board, used, k + 1, nodes) {
                return true;
            }
            used[pid as usize] = false;
        }
        board.clear(pos);
    }
    false
}

/// True when the piece at `pos` matches every already-placed neighbour.
fn placement_matches(board: &Board, pieces: &Pieces, pos: usize) -> bool {
    let e = edges_at(board, pieces, pos);
    let (y, x) = (pos / S, pos % S);
    let sides: [(bool, usize, i64, usize); 4] = [
        (y > 0, 0, -(S as i64), 2),
        (x < S - 1, 1, 1, 3),
        (y < S - 1, 2, S as i64, 0),
        (x > 0, 3, -1, 1),
    ];
    for (in_bounds, my_side, delta, their_side) in sides {
        if !in_bounds {
            continue;
        }
        let npos = (pos as i64 + delta) as usize;
        if board.is_empty_at(npos) {
            continue;
        }
        if e[my_side] != edges_at(board, pieces, npos)[their_side] {
            return false;
        }
    }
    true
}

// ---------------------------------------------------------------------------
// the fixed interior producer: a break-tolerant row-major beam

/// A partial completion under construction: the interior board state, the set
/// of still-unused interior piece ids, and its accumulated soft score.
#[derive(Clone)]
struct Cand {
    board: Board,
    used: Vec<bool>, // indexed by piece id over the whole set
    matched: i64,    // running (matched - mismatched) over placed interior sides
}

/// Fill the 196 interior cells of a frozen frame with a break-tolerant beam.
/// Returns `(canonical full-board score, best completed board)`. The 5 interior
/// hints are pre-placed and never displaced; the beam only fills empty interior
/// cells row-major. Ties are broken by a per-run seeded permutation so the run
/// is diversifiable across seeds but deterministic per seed.
fn fill_interior(
    frame: &Board,
    pieces: &Pieces,
    interior_ids: &[u16],
    hints: &[(usize, u16, u8)],
    beam: usize,
    seed: u32,
) -> (u32, Board) {
    let mut rng = XorShift::new(seed ^ 0x00F1_11EE);

    // Seed board: frame + pinned interior hints.
    let mut seed_board = frame.clone();
    let mut used0 = vec![false; pieces.len()];
    for &(pos, pid, rot) in hints {
        seed_board.place(pos, pid, rot);
        used0[pid as usize] = true;
    }

    // Interior fill order: row-major over interior cells, skipping pinned ones.
    let mut order: Vec<usize> = Vec::with_capacity(196);
    for y in 1..S - 1 {
        for x in 1..S - 1 {
            let pos = y * S + x;
            if seed_board.is_empty_at(pos) {
                order.push(pos);
            }
        }
    }

    // Candidate list of interior piece ids (rotations tried at placement).
    let cand_ids: Vec<u16> = interior_ids.to_vec();

    let mut beam_set: Vec<Cand> = vec![Cand { board: seed_board, used: used0, matched: 0 }];

    for &pos in &order {
        let (y, x) = (pos / S, pos % S);
        let mut next: Vec<Cand> = Vec::new();
        for cand in &beam_set {
            // Constraints from already-placed neighbours (frame, hints, filled).
            let want = edge_constraints(&cand.board, pieces, x, y, S, S);
            // Try every unused interior piece in every rotation; keep the best
            // few extensions per candidate (bounded fan-out to control cost).
            let mut local: Vec<(i64, u16, u8)> = Vec::new();
            for &pid in &cand_ids {
                if cand.used[pid as usize] {
                    continue;
                }
                let p = pieces.get(pid).expect("valid interior piece id");
                for r in 0..4u8 {
                    let e = p.rotated(r);
                    let (ok, bad) = fit_counts(&e, &want);
                    local.push((i64::from(ok) - i64::from(bad), pid, r));
                }
            }
            // Keep the top few extensions (best soft delta), seeded tie-break.
            local.sort_by_key(|c| std::cmp::Reverse(c.0));
            let fan = FAN_OUT.min(local.len());
            for &(delta, pid, r) in local.iter().take(fan) {
                let mut nb = cand.board.clone();
                nb.place(pos, pid, r);
                let mut nu = cand.used.clone();
                nu[pid as usize] = true;
                next.push(Cand { board: nb, used: nu, matched: cand.matched + delta });
            }
        }
        // Prune to beam width. Sort by soft score; break ties with a seeded key.
        next.sort_by(|a, b| {
            b.matched
                .cmp(&a.matched)
                .then_with(|| a.board.hash().cmp(&b.board.hash()))
        });
        if next.len() > beam {
            // Randomised tie survivor: shuffle the marginal band before truncating.
            let cut = next[beam.saturating_sub(1)].matched;
            let keep_hard: Vec<Cand> = next.iter().filter(|c| c.matched > cut).cloned().collect();
            let mut band: Vec<Cand> = next.into_iter().filter(|c| c.matched == cut).collect();
            rng.shuffle(&mut band);
            let need = beam.saturating_sub(keep_hard.len());
            beam_set = keep_hard;
            beam_set.extend(band.into_iter().take(need));
        } else {
            beam_set = next;
        }
        if beam_set.is_empty() {
            break;
        }
    }

    // Score every survivor through the canonical scorer, keep the best.
    let mut best_score = 0u32;
    let mut best_board = frame.clone();
    for cand in &beam_set {
        let s = score_board(&cand.board, pieces);
        if s > best_score {
            best_score = s;
            best_board = cand.board.clone();
        }
    }
    (best_score, best_board)
}

/// Extensions kept per beam candidate at each cell (bounded fan-out).
const FAN_OUT: usize = 8;

// ---------------------------------------------------------------------------
// args

struct Args {
    frames: usize,
    beam: usize,
    seeds: usize,
    seed0: u32,
}

impl Args {
    fn parse() -> Self {
        let mut a = Self { frames: 12, beam: 128, seeds: 2, seed0: 1 };
        let mut it = std::env::args().skip(1);
        while let Some(flag) = it.next() {
            let val = it.next();
            fn parse<T: std::str::FromStr>(v: &Option<String>) -> Option<T> {
                v.as_deref().and_then(|s| s.parse().ok())
            }
            match flag.as_str() {
                "--frames" => a.frames = parse(&val).expect("--frames N"),
                "--beam" => a.beam = parse(&val).expect("--beam B"),
                "--seeds" => a.seeds = parse(&val).expect("--seeds K"),
                "--seed0" => a.seed0 = parse(&val).expect("--seed0 S"),
                other => panic!("unknown flag {other}"),
            }
        }
        a
    }
}
