//! CAS (Concentric Annular Solving) reproduction.
//!
//!   cargo run --release                     # 20 frames CAS + 8-frame baseline arm
//!   cargo run --release -- 20 8 1024 60     # frames, baseline frames, beam width, baseline secs
//!
//! CAS fills the board from the outside in: ring 0 (the border) is placed as a
//! perfect 60/60 frame, then each inner ring is solved in turn as ONE
//! assignment over the remaining pieces, maximising matched edges against the
//! already-placed outer ring (plus ring-internal edges). The vault (vol-76)
//! solved each shell as a MIP; here each shell is solved by a wide
//! deterministic beam search along the ring walk, which plays the same role
//! (one near-exact assignment per shell, committed before moving inward).
//!
//! The baseline arm is an ALNS-lite proxy (random destroy + greedy repair,
//! first-improvement acceptance) started from the same pinned frame with an
//! empty interior, 60 s per frame, standing in for the vault's alns_only
//! continuation (claim 2 direction test; the kit has no full ALNS).
//!
//! Faithfulness notes (see ../PLAN.md):
//!   - ring 0: faithful. Restart-DFS over the border pieces finds perfect
//!     60/60 frames; distinct seeds yield distinct frames (checked).
//!   - rings 1-7: per-shell beam (width configurable) instead of per-shell
//!     MIP. Deterministic given the frame.
//!   - clues: pinned (official_instance(true)); the conservative choice.

use std::collections::HashSet;
use std::fmt::Write as _;

use e2_kit::fit::{edge_constraints, fit_counts, fit_score};
use e2_kit::{official_instance, score_board, Board, Budget, Instance, XorShift, BORDER};

const S: usize = 16;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n_frames: usize = args.get(1).and_then(|a| a.parse().ok()).unwrap_or(20);
    let n_baseline: usize = args.get(2).and_then(|a| a.parse().ok()).unwrap_or(8);
    let beam_w: usize = args.get(3).and_then(|a| a.parse().ok()).unwrap_or(1024);
    let base_secs: f64 = args.get(4).and_then(|a| a.parse().ok()).unwrap_or(60.0);

    // Canonical instance, five official clues pinned (conservative; PLAN b).
    let instance = official_instance(true);
    let start = instance.seed_board();
    let rot_tab = rotation_table(&instance);

    // ---- Frame enumeration: distinct perfect 60/60 frames, one per seed. ----
    let mut frames: Vec<(u32, Board)> = Vec::new();
    let mut seen: HashSet<Vec<(u16, u8)>> = HashSet::new();
    let mut seed: u32 = 1;
    while frames.len() < n_frames && seed <= 200 {
        if let Some(board) = find_frame(&instance, &start, seed) {
            let sig: Vec<(u16, u8)> = ring_cells(0, S)
                .iter()
                .map(|&p| board.piece_at(p).expect("frame cell"))
                .collect();
            if seen.insert(sig) {
                eprintln!("frame seed {seed}: distinct perfect 60/60 frame ({}/{})", frames.len() + 1, n_frames);
                frames.push((seed, board));
            } else {
                eprintln!("frame seed {seed}: duplicate frame, skipped");
            }
        } else {
            eprintln!("frame seed {seed}: no frame within node cap, skipped");
        }
        seed += 1;
    }

    // ---- CAS arm: per-shell beam on every frame. ----
    let mut cas_rows: Vec<(u32, u32, u32, String, u64)> = Vec::new(); // seed, score, breaks, url, nodes
    for (fseed, frame_board) in &frames {
        let mut board = frame_board.clone();
        let mut used = used_from(&instance, &board);
        let nodes = cas_fill(&mut board, &mut used, &instance, &rot_tab, beam_w);
        let out = instance.finish(&board);
        eprintln!("CAS  seed {fseed}: score {} breaks {} nodes {nodes}", out.score, out.breaks);
        cas_rows.push((*fseed, out.score, out.breaks, out.url, nodes));
    }

    // ---- Baseline arm: ALNS-lite from the same frames (first n_baseline). ----
    let mut base_rows: Vec<(u32, u32, u64)> = Vec::new(); // seed, score, iters
    for (fseed, frame_board) in frames.iter().take(n_baseline) {
        let (score, iters) = alns_lite(&instance, frame_board, &rot_tab, *fseed, base_secs);
        eprintln!("BASE seed {fseed}: score {score} iters {iters}");
        base_rows.push((*fseed, score, iters));
    }

    // ---- Emit JSON (stdout). ----
    println!("{}", render_json(beam_w, base_secs, &cas_rows, &base_rows));
}

// ---------------------------------------------------------------------------
// Geometry
// ---------------------------------------------------------------------------

/// Cells of concentric ring `k` on an SxS board, clockwise cyclic walk.
/// Consecutive walk cells (and the closing pair) are exactly the ring-internal
/// adjacencies. Ring sizes at 16x16: 60, 52, 44, 36, 28, 20, 12, 4.
fn ring_cells(k: usize, s: usize) -> Vec<usize> {
    let lo = k;
    if lo * 2 >= s + 1 {
        return Vec::new();
    }
    let hi = s - 1 - k;
    if lo > hi {
        return Vec::new();
    }
    if lo == hi {
        return vec![lo * s + lo];
    }
    let mut v = Vec::with_capacity(4 * (hi - lo));
    for x in lo..=hi {
        v.push(lo * s + x);
    }
    for y in lo + 1..=hi {
        v.push(y * s + hi);
    }
    for x in (lo..hi).rev() {
        v.push(hi * s + x);
    }
    for y in (lo + 1..hi).rev() {
        v.push(y * s + lo);
    }
    v
}

/// URDL side of `from` that faces the (adjacent) cell `to`.
fn side_facing(from: usize, to: usize) -> usize {
    let (fx, fy) = (from % S, from / S);
    let (tx, ty) = (to % S, to / S);
    if ty + 1 == fy && tx == fx {
        0
    } else if tx == fx + 1 && ty == fy {
        1
    } else if ty == fy + 1 && tx == fx {
        2
    } else {
        3
    }
}

fn want(board: &Board, instance: &Instance, pos: usize) -> [Option<u8>; 4] {
    edge_constraints(board, &instance.pieces, pos % S, pos / S, S, S)
}

fn rotation_table(instance: &Instance) -> Vec<[[u8; 4]; 4]> {
    (0..instance.pieces.len() as u16)
        .map(|pid| {
            let p = instance.pieces.get(pid).expect("piece");
            [p.rotated(0), p.rotated(1), p.rotated(2), p.rotated(3)]
        })
        .collect()
}

fn used_from(instance: &Instance, board: &Board) -> Vec<bool> {
    let mut used = vec![false; instance.pieces.len()];
    for pos in 0..S * S {
        if let Some((pid, _)) = board.piece_at(pos) {
            used[pid as usize] = true;
        }
    }
    used
}

// ---------------------------------------------------------------------------
// Ring 0: perfect 60/60 frame by restart-DFS (node-capped, deterministic)
// ---------------------------------------------------------------------------

/// A perfect 60/60 frame for `seed`, on top of the clue-seeded start board.
/// Deterministic: node-capped attempts with seeded piece-order shuffles.
fn find_frame(instance: &Instance, start: &Board, seed: u32) -> Option<Board> {
    let frame = ring_cells(0, S);
    let mut board = start.clone();
    let mut used = used_from(instance, &board);
    let mut order: Vec<u16> = (0..instance.pieces.len() as u16).collect();
    let mut rng = XorShift::new(seed);
    let mut nodes = 0u64;
    for _attempt in 0..64 {
        rng.shuffle(&mut order);
        let cap = nodes.saturating_add(2_000_000);
        if fill_frame_dfs(&mut board, &mut used, instance, &frame, 0, &order, cap, &mut nodes) {
            return Some(board);
        }
        for &pos in &frame {
            if let Some((pid, _)) = board.piece_at(pos) {
                if start.is_empty_at(pos) {
                    used[pid as usize] = false;
                    board.clear(pos);
                }
            }
        }
    }
    None
}

/// DFS over `cells[idx..]` with every constraint hard: rim edges grey, edges
/// facing a placed piece matched. The border walk closes on itself, so success
/// means a perfect 60/60 frame.
#[allow(clippy::too_many_arguments)]
fn fill_frame_dfs(
    board: &mut Board,
    used: &mut [bool],
    instance: &Instance,
    cells: &[usize],
    idx: usize,
    order: &[u16],
    cap: u64,
    nodes: &mut u64,
) -> bool {
    if idx == cells.len() {
        return true;
    }
    if *nodes >= cap {
        return false;
    }
    let pos = cells[idx];
    if !board.is_empty_at(pos) {
        return fill_frame_dfs(board, used, instance, cells, idx + 1, order, cap, nodes);
    }
    let w = want(board, instance, pos);
    for &pid in order {
        if used[pid as usize] {
            continue;
        }
        let piece = instance.pieces.get(pid).expect("piece");
        for r in 0..4 {
            *nodes += 1;
            let e = piece.rotated(r);
            if fit_score(&e, &w).is_none() {
                continue;
            }
            board.place(pos, pid, r);
            used[pid as usize] = true;
            if fill_frame_dfs(board, used, instance, cells, idx + 1, order, cap, nodes) {
                return true;
            }
            board.clear(pos);
            used[pid as usize] = false;
        }
    }
    false
}

// ---------------------------------------------------------------------------
// Rings 1-7: one beam-solved assignment per shell (the vault's MIP role)
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct RingState {
    used: [u64; 4],
    score: u32,
    prev: [u8; 4],
    first: Option<[u8; 4]>,
    placements: Vec<(u16, u8)>,
}

fn bit_get(mask: &[u64; 4], i: usize) -> bool {
    mask[i / 64] >> (i % 64) & 1 == 1
}

fn bit_set(mask: &mut [u64; 4], i: usize) {
    mask[i / 64] |= 1 << (i % 64);
}

/// Fill rings 1..7, each as one beam-search assignment along the ring walk,
/// maximising matched edges (into the committed outer ring, ring-internal
/// including the walk closure, and against pinned clues). Commits the best
/// assignment per shell before moving inward. Returns candidate evaluations.
fn cas_fill(
    board: &mut Board,
    used: &mut [bool],
    instance: &Instance,
    rot_tab: &[[[u8; 4]; 4]],
    beam_w: usize,
) -> u64 {
    let n_pieces = instance.pieces.len();
    let mut nodes = 0u64;
    for k in 1.. {
        let walk = ring_cells(k, S);
        if walk.is_empty() {
            break;
        }
        let m = walk.len();
        let fixed: Vec<[Option<u8>; 4]> = walk.iter().map(|&p| want(board, instance, p)).collect();
        let empty: Vec<bool> = walk.iter().map(|&p| board.is_empty_at(p)).collect();

        let mut init_used = [0u64; 4];
        for (i, &u) in used.iter().enumerate() {
            if u {
                bit_set(&mut init_used, i);
            }
        }
        let mut beam = vec![RingState {
            used: init_used,
            score: 0,
            prev: [BORDER; 4],
            first: None,
            placements: Vec::new(),
        }];

        let last_state_placed = m >= 2 && empty[m - 1] && empty[0];
        for i in 0..m {
            if !empty[i] {
                continue;
            }
            // The edge to the previous walk cell is state-carried only when
            // that cell is itself state-placed (a pinned clue there already
            // appears in `fixed`). Same for the closing edge to walk[0].
            let prev_side = if i > 0 && empty[i - 1] { Some(side_facing(walk[i], walk[i - 1])) } else { None };
            let close_side = if i == m - 1 && last_state_placed { Some(side_facing(walk[m - 1], walk[0])) } else { None };

            let mut cands: Vec<(u32, u32, u16, u8)> = Vec::new();
            for (si, st) in beam.iter().enumerate() {
                for pid in 0..n_pieces {
                    if bit_get(&st.used, pid) {
                        continue;
                    }
                    for r in 0..4u8 {
                        nodes += 1;
                        let e = rot_tab[pid][r as usize];
                        let mut sc = st.score + fit_counts(&e, &fixed[i]).0;
                        if let Some(ps) = prev_side {
                            let facing = st.prev[(ps + 2) % 4];
                            if e[ps] == facing && facing != BORDER {
                                sc += 1;
                            }
                        }
                        if let Some(cs) = close_side {
                            let first = st.first.unwrap_or([BORDER; 4]);
                            let facing = first[(cs + 2) % 4];
                            if e[cs] == facing && facing != BORDER {
                                sc += 1;
                            }
                        }
                        cands.push((sc, si as u32, pid as u16, r));
                    }
                }
            }
            // Deterministic top-`beam_w`: total order (score desc, then ids).
            let cmp = |a: &(u32, u32, u16, u8), b: &(u32, u32, u16, u8)| {
                b.0.cmp(&a.0).then(a.1.cmp(&b.1)).then(a.2.cmp(&b.2)).then(a.3.cmp(&b.3))
            };
            if cands.len() > beam_w {
                cands.select_nth_unstable_by(beam_w - 1, cmp);
                cands.truncate(beam_w);
            }
            cands.sort_unstable_by(cmp);
            let mut next = Vec::with_capacity(cands.len());
            for (sc, si, pid, r) in cands {
                let parent = &beam[si as usize];
                let mut st = parent.clone();
                st.score = sc;
                bit_set(&mut st.used, usize::from(pid));
                let e = rot_tab[usize::from(pid)][usize::from(r)];
                if i == 0 {
                    st.first = Some(e);
                }
                st.prev = e;
                st.placements.push((pid, r));
                next.push(st);
            }
            beam = next;
        }

        // Commit the best assignment for this shell.
        let best = beam.first().expect("beam never empties (soft fit)");
        let mut it = best.placements.iter();
        for (i, &pos) in walk.iter().enumerate() {
            if empty[i] {
                let &(pid, r) = it.next().expect("one placement per empty walk cell");
                board.place(pos, pid, r);
                used[usize::from(pid)] = true;
            }
        }
    }
    nodes
}

// ---------------------------------------------------------------------------
// Baseline arm: ALNS-lite from a pinned frame + empty interior
// ---------------------------------------------------------------------------

/// Destroy-and-repair proxy for the vault's alns_only continuation: greedy
/// best-fit initial fill (scan order), then random destroy of 6-12 interior
/// cells + greedy repair in random order, first-improvement acceptance.
/// Returns (best canonical score, iterations run).
fn alns_lite(
    instance: &Instance,
    frame_board: &Board,
    rot_tab: &[[[u8; 4]; 4]],
    seed: u32,
    secs: f64,
) -> (u32, u64) {
    let mut board = frame_board.clone();
    let mut used = used_from(instance, &board);
    let mut rng = XorShift::new(seed ^ 0xB45E_11CE);
    let n_pieces = instance.pieces.len();

    let cells: Vec<usize> = (0..S * S).filter(|&p| board.is_empty_at(p)).collect();

    // Initial fill: best soft fit per cell, scan order.
    for &pos in &cells {
        let w = want(&board, instance, pos);
        let mut best: Option<(u16, u8, u32)> = None;
        for pid in 0..n_pieces {
            if used[pid] {
                continue;
            }
            for r in 0..4u8 {
                let e = rot_tab[pid][usize::from(r)];
                let sc = fit_counts(&e, &w).0;
                if best.is_none_or(|(_, _, b)| sc > b) {
                    best = Some((pid as u16, r, sc));
                }
            }
        }
        let (pid, r, _) = best.expect("piece pool covers the board");
        board.place(pos, pid, r);
        used[usize::from(pid)] = true;
    }

    let mut cur = score_board(&board, &instance.pieces);
    let mut best_score = cur;
    let budget = Budget::seconds(secs);
    let mut iters = 0u64;
    let mut removed: Vec<(usize, u16, u8)> = Vec::new();
    let mut order: Vec<usize> = Vec::new();

    while !budget.expired() && iters < 20_000_000 {
        iters += 1;
        let k = 6 + rng.next_below(7) as usize;
        removed.clear();
        while removed.len() < k {
            let pos = cells[rng.next_below(cells.len() as u32) as usize];
            if let Some((pid, r)) = board.piece_at(pos) {
                board.clear(pos);
                used[usize::from(pid)] = false;
                removed.push((pos, pid, r));
            }
        }
        order.clear();
        order.extend(removed.iter().map(|&(pos, _, _)| pos));
        rng.shuffle(&mut order);
        for &pos in &order {
            let w = want(&board, instance, pos);
            let mut best: Option<(u16, u8, u32)> = None;
            for &(_, pid, _) in &removed {
                if used[usize::from(pid)] {
                    continue;
                }
                for r in 0..4u8 {
                    let e = rot_tab[usize::from(pid)][usize::from(r)];
                    let sc = fit_counts(&e, &w).0;
                    if best.is_none_or(|(_, _, b)| sc > b) {
                        best = Some((pid, r, sc));
                    }
                }
            }
            let (pid, r, _) = best.expect("removed pieces refill removed cells");
            board.place(pos, pid, r);
            used[usize::from(pid)] = true;
        }
        let ns = score_board(&board, &instance.pieces);
        if ns >= cur {
            cur = ns;
            if ns > best_score {
                best_score = ns;
            }
        } else {
            for &(pos, _, _) in &removed {
                if let Some((pid, _)) = board.piece_at(pos) {
                    used[usize::from(pid)] = false;
                    board.clear(pos);
                }
            }
            for &(pos, pid, r) in &removed {
                board.place(pos, pid, r);
                used[usize::from(pid)] = true;
            }
        }
    }
    (best_score, iters)
}

// ---------------------------------------------------------------------------
// Output
// ---------------------------------------------------------------------------

fn stats(scores: &[u32]) -> (u32, u32, f64, f64) {
    let min = *scores.iter().min().expect("non-empty");
    let max = *scores.iter().max().expect("non-empty");
    let mean = scores.iter().map(|&s| f64::from(s)).sum::<f64>() / scores.len() as f64;
    let var = scores.iter().map(|&s| (f64::from(s) - mean).powi(2)).sum::<f64>()
        / scores.len() as f64;
    (min, max, mean, var.sqrt())
}

fn render_json(
    beam_w: usize,
    base_secs: f64,
    cas: &[(u32, u32, u32, String, u64)],
    base: &[(u32, u32, u64)],
) -> String {
    let mut s = String::new();
    s.push_str("{\n");
    s.push_str("  \"instance\": \"official 16x16, five clues pinned\",\n");
    s.push_str("  \"convention\": \"matched interior edges out of 480, rim-excluding (canonical score_board)\",\n");
    let _ = writeln!(s, "  \"cas\": {{");
    let _ = writeln!(s, "    \"shell_solver\": \"per-shell beam, width {beam_w}, deterministic\",");
    s.push_str("    \"frames\": [\n");
    for (i, (seed, score, breaks, url, nodes)) in cas.iter().enumerate() {
        let comma = if i + 1 < cas.len() { "," } else { "" };
        let _ = writeln!(
            s,
            "      {{\"frame_seed\": {seed}, \"score\": {score}, \"breaks\": {breaks}, \"nodes\": {nodes}, \"url\": \"{url}\"}}{comma}"
        );
    }
    s.push_str("    ],\n");
    let scores: Vec<u32> = cas.iter().map(|r| r.1).collect();
    let (min, max, mean, std) = stats(&scores);
    let _ = writeln!(
        s,
        "    \"summary\": {{\"n\": {}, \"min\": {min}, \"max\": {max}, \"mean\": {mean:.2}, \"std\": {std:.2}}},",
        scores.len()
    );
    s.push_str("    \"expected\": {\"band\": [430, 436], \"mean\": 432.4, \"source\": \"vault cas-frame-final (vol-76), 20 frames\"}\n");
    s.push_str("  },\n");
    let _ = writeln!(s, "  \"baseline\": {{");
    let _ = writeln!(
        s,
        "    \"method\": \"ALNS-lite proxy (random destroy 6-12 cells + greedy repair, first-improvement), {base_secs} s/frame, from the same pinned frame with empty interior\","
    );
    s.push_str("    \"frames\": [\n");
    for (i, (seed, score, iters)) in base.iter().enumerate() {
        let comma = if i + 1 < base.len() { "," } else { "" };
        let cas_score = cas.iter().find(|r| r.0 == *seed).map_or(0, |r| r.1);
        let delta = i64::from(cas_score) - i64::from(*score);
        let _ = writeln!(
            s,
            "      {{\"frame_seed\": {seed}, \"baseline_score\": {score}, \"cas_score\": {cas_score}, \"delta\": {delta}, \"iters\": {iters}}}{comma}"
        );
    }
    s.push_str("    ],\n");
    if !base.is_empty() {
        let bscores: Vec<u32> = base.iter().map(|r| r.1).collect();
        let (min, max, mean, std) = stats(&bscores);
        let _ = writeln!(
            s,
            "    \"summary\": {{\"n\": {}, \"min\": {min}, \"max\": {max}, \"mean\": {mean:.2}, \"std\": {std:.2}}},",
            bscores.len()
        );
    }
    s.push_str("    \"expected\": {\"band\": [385, 398], \"source\": \"vault cas-beats-alns-from-frame (vol-77), alns_only 60 s/frame\", \"note\": \"proxy baseline, direction test only\"}\n");
    s.push_str("  }\n");
    s.push_str("}");
    s
}
