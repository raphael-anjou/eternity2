//! frame-manifold reproduction bin.
//!
//! Builds BB=60 perfect frames of the OFFICIAL 256-piece set from scratch
//! (randomised DFS over the 60 border cells, grey exactly outward, all ring
//! adjacencies matched), then measures the vol-244 claims directly:
//!
//!   1. the pair-exchange bb_delta distribution (expected: 45 free, 0 gains);
//!   2. free-swap composition (BB stays 60, free count stays 45 along a walk);
//!   3. non-cosmetic-ness (0 free swaps preserve inward colour; distinct
//!      rim-target vectors visited along the walk);
//!   4. the dead-frame census (share of perfect frames with an unfillable
//!      frame-adjacent interior cell; share dead at the interior scan start);
//!   5. greedy revival of dead frames by free swaps only.
//!
//! Usage:
//!   cargo run --release -- [--frames N] [--walk L] [--seed0 S]
//!
//! Prints one JSON report to stdout. Runs on `official_instance(false)`
//! (unhinted, as in the source measurement). BB is obtained by scoring the
//! frame-only board with the kit's canonical scorer: empty cells score
//! nothing, so `score_board(frame) == BB` exactly (see PLAN.md section b).

use std::collections::HashSet;

use e2_kit::{generator::XorShift, official_instance, score_board, Board, Pieces};

const S: usize = 16;
const N_RING: usize = 60;
const PERFECT_BB: u32 = 60;
/// Node cap per frame-DFS seed; an unlucky seed is skipped, not fought.
const DFS_NODE_CAP: u64 = 5_000_000;

fn main() {
    let args = Args::parse();
    let instance = official_instance(false);
    let pieces = &instance.pieces;
    let ring = ring_cells();
    let classes = PieceClasses::of(pieces);

    // ---- generate frames -------------------------------------------------
    let mut frames: Vec<Board> = Vec::new();
    let mut seed = args.seed0;
    while frames.len() < args.frames {
        if let Some(frame) = build_frame(pieces, &classes, &ring, seed) {
            debug_assert_eq!(score_board(&frame, pieces), PERFECT_BB);
            frames.push(frame);
        }
        seed = seed.wrapping_add(1);
    }

    // ---- claim 1: exchange cost distribution -----------------------------
    let mut delta_hist: std::collections::BTreeMap<i64, u64> = std::collections::BTreeMap::new();
    let mut free_counts: Vec<usize> = Vec::with_capacity(frames.len());
    for frame in &frames {
        let ex = enumerate_exchanges(frame, pieces, &ring, &classes);
        let mut free = 0usize;
        for &(_, _, d) in &ex {
            *delta_hist.entry(i64::from(d)).or_insert(0) += 1;
            if d == 0 {
                free += 1;
            }
        }
        free_counts.push(free);
    }
    let free_min = free_counts.iter().min().copied().unwrap_or(0);
    let free_max = free_counts.iter().max().copied().unwrap_or(0);
    let any_gain = delta_hist.keys().any(|&d| d > 0);

    // ---- claims 2+3: walk on the first frame -----------------------------
    let walk = walk_report(&frames[0], pieces, &ring, &classes, args.walk, args.seed0);

    // ---- claim 3 (static part): inward-colour preservation ---------------
    let ex0 = enumerate_exchanges(&frames[0], pieces, &ring, &classes);
    let rim0 = rim_targets(&frames[0], pieces, &ring);
    let mut preserving = 0usize;
    for &(a, b, d) in &ex0 {
        if d != 0 {
            continue;
        }
        let mut f = frames[0].clone();
        apply_exchange(&mut f, pieces, a, b);
        if rim_targets(&f, pieces, &ring) == rim0 {
            preserving += 1;
        }
    }

    // ---- claim 4: dead-frame census --------------------------------------
    let interior_ids = classes.interior.clone();
    let mut dead_cell_hist: std::collections::BTreeMap<usize, u64> =
        std::collections::BTreeMap::new();
    let mut dead_frames: Vec<usize> = Vec::new(); // indices into `frames`
    let mut dead_at_scan_start = 0usize;
    for (i, frame) in frames.iter().enumerate() {
        let dead = dead_cells(frame, pieces, &interior_ids);
        if !dead.is_empty() {
            *dead_cell_hist.entry(dead.len()).or_insert(0) += 1;
            dead_frames.push(i);
            // Interior scan start = cell (1,1) = pos 17, the first cell a
            // row-major interior DFS visits.
            if dead.contains(&(S + 1)) {
                dead_at_scan_start += 1;
            }
        }
    }

    // ---- claim 5: greedy free-swap revival -------------------------------
    let sample: Vec<usize> = dead_frames.iter().copied().take(args.revive_cap).collect();
    let mut revived = 0usize;
    for &i in &sample {
        let mut f = frames[i].clone();
        if revive(&mut f, pieces, &ring, &classes, &interior_ids) {
            debug_assert_eq!(score_board(&f, pieces), PERFECT_BB);
            revived += 1;
        }
    }

    // A board URL for the first frame, so the run leaves a viewable artifact.
    let frame0_url = instance.finish(&frames[0]).url;

    let report = serde_json::json!({
        "instance": "official 16x16, unhinted",
        "frames_generated": frames.len(),
        "frame0_url": frame0_url,
        "claim1_exchanges": {
            "bb_delta_histogram_pooled": delta_hist.iter()
                .map(|(d, c)| (d.to_string(), c)).collect::<std::collections::BTreeMap<_,_>>(),
            "free_per_frame_min": free_min,
            "free_per_frame_max": free_max,
            "expected_free": 45,
            "all_frames_free_eq_45": free_min == 45 && free_max == 45,
            "any_positive_delta": any_gain,
        },
        "claims2_3_walk": walk,
        "claim3_static": {
            "free_swaps_preserving_inward_colour": preserving,
            "expected": 0,
        },
        "claim4_dead_frames": {
            "dead_frames": dead_frames.len(),
            "total_frames": frames.len(),
            "dead_share": dead_frames.len() as f64 / frames.len() as f64,
            "dead_at_scan_start": dead_at_scan_start,
            "dead_cell_count_histogram": dead_cell_hist.iter()
                .map(|(k, v)| (k.to_string(), v)).collect::<std::collections::BTreeMap<_,_>>(),
            "source_reference": "180/500 dead (36.0%), 55/500 dead at scan start (11.0%)",
        },
        "claim5_revival": {
            "attempted": sample.len(),
            "revived_to_zero_dead_bb_still_60": revived,
            "expected_share": 1.0,
        },
    });
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
}

// ---------------------------------------------------------------------------
// geometry

/// The 60 border cells, clockwise from the top-left corner.
fn ring_cells() -> Vec<usize> {
    let mut v = Vec::with_capacity(N_RING);
    for x in 0..S {
        v.push(x); // top row, left to right (corners 0 and 15)
    }
    for y in 1..S {
        v.push(y * S + (S - 1)); // right column, down (corner 255)
    }
    for x in (0..S - 1).rev() {
        v.push((S - 1) * S + x); // bottom row, right to left (corner 240)
    }
    for y in (1..S - 1).rev() {
        v.push(y * S); // left column, up
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
fn forced_rot(piece: &e2_kit::Piece, pos: usize) -> Option<u8> {
    let mask = rim_mask(pos);
    (0..4u8).find(|&r| {
        let e = piece.rotated(r);
        (0..4).all(|s| (e[s] == 0) == mask[s])
    })
}

/// Oriented edges of the placed piece at `pos` (must be non-empty).
fn edges_at(board: &Board, pieces: &Pieces, pos: usize) -> [u8; 4] {
    let (pid, rot) = board.piece_at(pos).expect("cell must be placed");
    pieces.get(pid).expect("valid piece id").rotated(rot)
}

struct PieceClasses {
    corners: Vec<u16>,
    edges: Vec<u16>,
    interior: Vec<u16>,
}

impl PieceClasses {
    fn of(pieces: &Pieces) -> Self {
        let (mut corners, mut edges, mut interior) = (Vec::new(), Vec::new(), Vec::new());
        for (pid, p) in pieces.iter() {
            match p.border_edge_count() {
                2 => corners.push(pid),
                1 => edges.push(pid),
                _ => interior.push(pid),
            }
        }
        assert_eq!(corners.len(), 4, "official set has 4 corner pieces");
        assert_eq!(edges.len(), 56, "official set has 56 edge pieces");
        assert_eq!(interior.len(), 196, "official set has 196 interior pieces");
        Self { corners, edges, interior }
    }
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
/// (Checks all four sides, so it also validates the closing wrap of the ring.)
fn placement_matches(board: &Board, pieces: &Pieces, pos: usize) -> bool {
    let e = edges_at(board, pieces, pos);
    let (y, x) = (pos / S, pos % S);
    let sides: [(bool, usize, i64, usize); 4] = [
        (y > 0, 0, -(S as i64), 2),    // up: my U vs their D
        (x < S - 1, 1, 1, 3),          // right: my R vs their L
        (y < S - 1, 2, S as i64, 0),   // down: my D vs their U
        (x > 0, 3, -1, 1),             // left: my L vs their R
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
// exchanges

/// All legal same-class pair exchanges `(cell_a, cell_b, bb_delta)`.
/// Rotations are forced by the grey-outward rule, so the pair determines the
/// exchange. BB before/after is the canonical frame-only score.
fn enumerate_exchanges(
    board: &Board,
    pieces: &Pieces,
    ring: &[usize],
    _classes: &PieceClasses,
) -> Vec<(usize, usize, i32)> {
    let base = score_board(board, pieces) as i32;
    let corner_cells: Vec<usize> = ring
        .iter()
        .copied()
        .filter(|&p| rim_mask(p).iter().filter(|&&b| b).count() == 2)
        .collect();
    let edge_cells: Vec<usize> = ring
        .iter()
        .copied()
        .filter(|&p| rim_mask(p).iter().filter(|&&b| b).count() == 1)
        .collect();

    let mut out = Vec::new();
    for cells in [&corner_cells, &edge_cells] {
        for i in 0..cells.len() {
            for j in i + 1..cells.len() {
                let (a, b) = (cells[i], cells[j]);
                let mut f = board.clone();
                apply_exchange(&mut f, pieces, a, b);
                let d = score_board(&f, pieces) as i32 - base;
                out.push((a, b, d));
            }
        }
    }
    out
}

/// Swap the pieces at cells `a` and `b`, re-rotating each to keep grey
/// exactly outward at its new cell.
fn apply_exchange(board: &mut Board, pieces: &Pieces, a: usize, b: usize) {
    let (pa, _) = board.piece_at(a).expect("cell a placed");
    let (pb, _) = board.piece_at(b).expect("cell b placed");
    let ra = forced_rot(pieces.get(pb).unwrap(), a).expect("same-class swap fits");
    let rb = forced_rot(pieces.get(pa).unwrap(), b).expect("same-class swap fits");
    board.place(a, pb, ra);
    board.place(b, pa, rb);
}

/// The 56 inward-facing colours, in ring order over the non-corner border
/// cells. This is the constraint vector the frame presents to the interior.
fn rim_targets(board: &Board, pieces: &Pieces, ring: &[usize]) -> Vec<u8> {
    let mut v = Vec::with_capacity(56);
    for &pos in ring {
        let mask = rim_mask(pos);
        if mask.iter().filter(|&&b| b).count() != 1 {
            continue; // corners present no inward face
        }
        let e = edges_at(board, pieces, pos);
        // The inward side is opposite the rim side.
        let rim_side = mask.iter().position(|&b| b).unwrap();
        v.push(e[(rim_side + 2) % 4]);
    }
    v
}

// ---------------------------------------------------------------------------
// claims 2+3: the walk

fn walk_report(
    frame: &Board,
    pieces: &Pieces,
    ring: &[usize],
    classes: &PieceClasses,
    walk_len: usize,
    seed: u32,
) -> serde_json::Value {
    let checkpoints: HashSet<usize> = [1, 5, 10, 25, 50, 100].into_iter().collect();
    let start = frame.clone();
    let start_targets = rim_targets(&start, pieces, ring);
    let mut f = frame.clone();
    let mut rng = XorShift::new(seed ^ 0x00FA_CADE);
    let mut visited: HashSet<Vec<u8>> = HashSet::new();
    visited.insert(start_targets.clone());
    let mut rows = Vec::new();

    for step in 1..=walk_len {
        let free: Vec<(usize, usize, i32)> = enumerate_exchanges(&f, pieces, ring, classes)
            .into_iter()
            .filter(|&(_, _, d)| d == 0)
            .collect();
        if free.is_empty() {
            rows.push(serde_json::json!({ "step": step, "error": "free set empty" }));
            break;
        }
        let &(a, b, _) = &free[rng.below(free.len() as u32) as usize];
        apply_exchange(&mut f, pieces, a, b);
        visited.insert(rim_targets(&f, pieces, ring));

        if checkpoints.contains(&step) || step == walk_len {
            let free_now = enumerate_exchanges(&f, pieces, ring, classes)
                .iter()
                .filter(|&&(_, _, d)| d == 0)
                .count();
            let slots_changed = ring
                .iter()
                .filter(|&&p| f.piece_at(p) != start.piece_at(p))
                .count();
            let targets_now = rim_targets(&f, pieces, ring);
            let targets_changed = targets_now
                .iter()
                .zip(&start_targets)
                .filter(|(x, y)| x != y)
                .count();
            rows.push(serde_json::json!({
                "step": step,
                "bb": score_board(&f, pieces),
                "free_available": free_now,
                "ring_slots_changed_of_60": slots_changed,
                "rim_targets_changed_of_56": targets_changed,
            }));
        }
    }
    serde_json::json!({
        "walk_len": walk_len,
        "checkpoints": rows,
        "distinct_rim_target_vectors_visited": visited.len(),
        "source_reference": "BB stays 60 and 45 free at every checkpoint; 292 distinct vectors in the source walk",
    })
}

// ---------------------------------------------------------------------------
// claims 4+5: dead cells and revival

/// Frame-adjacent interior cells that no interior piece can fill given the
/// rim-imposed colours (other neighbours empty, so only rim constraints bind).
fn dead_cells(board: &Board, pieces: &Pieces, interior_ids: &[u16]) -> Vec<usize> {
    let mut dead = Vec::new();
    for y in 1..S - 1 {
        for x in 1..S - 1 {
            if y != 1 && y != S - 2 && x != 1 && x != S - 2 {
                continue; // not frame-adjacent
            }
            let pos = y * S + x;
            let want = rim_constraints(board, pieces, pos);
            let fillable = interior_ids.iter().any(|&pid| {
                let p = pieces.get(pid).unwrap();
                (0..4u8).any(|r| {
                    let e = p.rotated(r);
                    want.iter().all(|&(side, c)| e[side] == c)
                })
            });
            if !fillable {
                dead.push(pos);
            }
        }
    }
    dead
}

/// The colours the frame imposes on interior cell `pos`: for each side whose
/// neighbour is a placed border cell, the neighbour's facing edge colour.
fn rim_constraints(board: &Board, pieces: &Pieces, pos: usize) -> Vec<(usize, u8)> {
    let (y, x) = (pos / S, pos % S);
    let mut want = Vec::new();
    let sides: [(bool, usize, i64, usize); 4] = [
        (y == 1, 0, -(S as i64), 2),
        (x == S - 2, 1, 1, 3),
        (y == S - 2, 2, S as i64, 0),
        (x == 1, 3, -1, 1),
    ];
    for (is_rim_neighbour, my_side, delta, their_side) in sides {
        if is_rim_neighbour {
            let npos = (pos as i64 + delta) as usize;
            want.push((my_side, edges_at(board, pieces, npos)[their_side]));
        }
    }
    want
}

/// Greedy revival: repeatedly apply the free swap that most reduces the
/// dead-cell count, free set recomputed each step. Returns true when the
/// frame reaches zero dead cells (BB provably still 60: only free swaps used).
fn revive(
    board: &mut Board,
    pieces: &Pieces,
    ring: &[usize],
    classes: &PieceClasses,
    interior_ids: &[u16],
) -> bool {
    for _ in 0..20 {
        let dead_now = dead_cells(board, pieces, interior_ids).len();
        if dead_now == 0 {
            return true;
        }
        let free: Vec<(usize, usize, i32)> = enumerate_exchanges(board, pieces, ring, classes)
            .into_iter()
            .filter(|&(_, _, d)| d == 0)
            .collect();
        let mut best: Option<(usize, usize, usize)> = None; // (a, b, dead_after)
        for &(a, b, _) in &free {
            let mut f = board.clone();
            apply_exchange(&mut f, pieces, a, b);
            let da = dead_cells(&f, pieces, interior_ids).len();
            if best.is_none_or(|(_, _, d)| da < d) {
                best = Some((a, b, da));
            }
        }
        match best {
            Some((a, b, da)) if da < dead_now => apply_exchange(board, pieces, a, b),
            _ => return false, // no strictly improving free swap
        }
    }
    dead_cells(board, pieces, interior_ids).is_empty()
}

// ---------------------------------------------------------------------------
// args

struct Args {
    frames: usize,
    walk: usize,
    seed0: u32,
    revive_cap: usize,
}

impl Args {
    fn parse() -> Self {
        let mut a = Self { frames: 500, walk: 100, seed0: 1, revive_cap: 60 };
        let mut it = std::env::args().skip(1);
        while let Some(flag) = it.next() {
            let val = it.next();
            fn parse<T: std::str::FromStr>(v: &Option<String>) -> Option<T> {
                v.as_deref().and_then(|s| s.parse().ok())
            }
            match flag.as_str() {
                "--frames" => a.frames = parse(&val).expect("--frames N"),
                "--walk" => a.walk = parse(&val).expect("--walk L"),
                "--seed0" => a.seed0 = parse(&val).expect("--seed0 S"),
                "--revive-cap" => a.revive_cap = parse(&val).expect("--revive-cap N"),
                other => panic!("unknown flag {other}"),
            }
        }
        a
    }
}
