//! CAS (Concentric Annular Solving) reproduction skeleton.
//!
//!   cargo run --release           # official instance, 30 s budget
//!   cargo run --release -- 60 7   # budget seconds, frame seed
//!
//! CAS fills the board from the outside in: ring 0 (the border) is placed as a
//! perfect 60/60 frame, then each inner ring is solved in turn over the
//! remaining pieces. The vault result (vol-76): from any perfect frame, CAS
//! plateaus at 430-436 matched edges out of 480 on the official puzzle.
//!
//! Faithfulness status of this skeleton (see ../PLAN.md):
//!   - ring 0: faithful. A DFS over the border pieces finds a perfect 60/60
//!     frame (all rim edges grey, all within-ring adjacencies matched).
//!   - rings 1-7: STAND-IN. The vault solved each shell as one exact assignment
//!     (MIP maximising matched edges over the remaining pieces, 25-45 s/frame).
//!     Greedy best-fit per cell is used here and is expected to score BELOW the
//!     430-436 band. Do not read this binary's number as a refutation.
//!   - frame distribution: NOT YET. The vault measured 20 enumerated frames;
//!     this takes the first frame the DFS finds.

use e2_kit::{official_instance, Board, Budget, Instance, Pieces, SolveOutcome, Solver};

/// Concentric annular solver: perfect frame by restart-DFS, then greedy ring
/// fill. `frame_seed` picks which frame the restarts converge to; distinct
/// seeds give distinct frames (the hook for the 20-frame distribution).
struct CasAnnular {
    frame_seed: u64,
}

/// Minimal xorshift64 PRNG for the frame-restart shuffles (deterministic per
/// seed, no external dependency).
struct XorShift64(u64);

impl XorShift64 {
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    fn shuffle(&mut self, v: &mut [u16]) {
        for i in (1..v.len()).rev() {
            let j = (self.next_u64() % (i as u64 + 1)) as usize;
            v.swap(i, j);
        }
    }
}

impl Solver for CasAnnular {
    fn name(&self) -> String {
        "cas-annular".into()
    }

    fn solve(&mut self, instance: &Instance, start: &Board, budget: Budget) -> SolveOutcome {
        let s = usize::from(instance.width);
        let n = s * s;
        let mut board = start.clone();
        let mut used = vec![false; instance.pieces.len()];
        for pos in 0..n {
            if let Some((pid, _)) = board.piece_at(pos) {
                used[pid as usize] = true;
            }
        }
        let mut nodes = 0u64;

        // Ring 0: a perfect 60/60 frame. Every constraint is hard (rim edges
        // grey, within-ring adjacencies matched), so success means 60/60.
        // A single lexicographic DFS thrashes (it fails only near the walk's
        // closure, billions of nodes deep), so we run randomized restarts:
        // shuffle the piece order per attempt, cap each attempt's nodes.
        //
        // TODO(vol-76 port): enumerate 20 DISTINCT frames (different restart
        // seeds already yield different frames) and run the shell solver on
        // each, so the 20-frame distribution (430-436, mean 432.4) can be
        // reproduced.
        let frame = ring_cells(0, s);
        let mut frame_ok = false;
        let mut order: Vec<u16> = (0..instance.pieces.len() as u16).collect();
        let mut rng = XorShift64(0x9e37_79b9_7f4a_7c15 ^ self.frame_seed);
        while !frame_ok && !budget.expired() {
            rng.shuffle(&mut order);
            let cap = nodes.saturating_add(2_000_000);
            frame_ok = fill_frame_dfs(
                &mut board, &mut used, instance, &frame, 0, &order, cap, &budget, &mut nodes,
            );
            if !frame_ok {
                // Undo any partial frame before the next attempt.
                for &pos in &frame {
                    if let Some((pid, _)) = board.piece_at(pos) {
                        if start.is_empty_at(pos) {
                            used[pid as usize] = false;
                            board.clear(pos);
                        }
                    }
                }
            }
        }
        if !frame_ok {
            // Budget spent inside the frame search: return what we have.
            return SolveOutcome::improved(board).with_nodes(nodes);
        }

        // Rings 1..: fill each shell before moving inward.
        //
        // TODO(vol-76 port): replace greedy best-fit per cell with ONE exact
        // assignment per shell (maximise matched edges, ring-internal plus
        // edges into the already-placed outer ring, over remaining pieces),
        // as the vault did via MIP. This is the port that closes the gap to
        // the 430-436 plateau. Rings 5-7 are small (20, 12, 4 cells).
        let mut ring = 1;
        loop {
            let cells = ring_cells(ring, s);
            if cells.is_empty() {
                break;
            }
            for &pos in &cells {
                if budget.expired() || !board.is_empty_at(pos) {
                    continue;
                }
                let want = edge_constraints(&board, &instance.pieces, pos, s);
                // Best soft fit: most matched edges against placed neighbours,
                // ties by piece id. Always place (CAS shells are total
                // assignments; unmatched edges just score 0).
                let mut best: Option<(u16, u8, u32)> = None;
                for (pid, piece) in instance.pieces.iter() {
                    if used[pid as usize] {
                        continue;
                    }
                    for r in 0..4 {
                        nodes += 1;
                        let e = piece.rotated(r);
                        let score = soft_fit(&e, &want);
                        if best.is_none_or(|(_, _, b)| score > b) {
                            best = Some((pid, r, score));
                        }
                    }
                }
                if let Some((pid, r, _)) = best {
                    board.place(pos, pid, r);
                    used[pid as usize] = true;
                }
            }
            ring += 1;
        }

        let full = (0..n).all(|pos| !board.is_empty_at(pos));
        let outcome = if full {
            SolveOutcome::complete(board)
        } else {
            SolveOutcome::improved(board)
        };
        outcome.with_nodes(nodes)
    }
}

/// Cells of concentric ring `k` on an `s`x`s` board, in a clockwise walk
/// (top row, right column, bottom row, left column). Empty when `k` is past
/// the centre. Ring sizes at 16x16: 60, 52, 44, 36, 28, 20, 12, 4.
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

/// DFS over `cells[idx..]` placing unused pieces (tried in `order`) so that
/// EVERY constraint is hard: rim edges must be grey (0) and edges facing an
/// already-placed piece must match. On the border walk this yields a perfect
/// 60/60 frame (the walk closes on itself, so the last cell is checked against
/// the first). Gives up when `*nodes` passes `cap` or the budget expires, so
/// the caller can restart with a different order.
#[allow(clippy::too_many_arguments)]
fn fill_frame_dfs(
    board: &mut Board,
    used: &mut [bool],
    instance: &Instance,
    cells: &[usize],
    idx: usize,
    order: &[u16],
    cap: u64,
    budget: &Budget,
    nodes: &mut u64,
) -> bool {
    if idx == cells.len() {
        return true;
    }
    if *nodes >= cap || budget.expired() {
        return false;
    }
    let s = usize::from(instance.width);
    let pos = cells[idx];
    if !board.is_empty_at(pos) {
        return fill_frame_dfs(board, used, instance, cells, idx + 1, order, cap, budget, nodes);
    }
    let want = edge_constraints(board, &instance.pieces, pos, s);
    for &pid in order {
        if used[pid as usize] {
            continue;
        }
        let piece = instance.pieces.get(pid).unwrap();
        for r in 0..4 {
            *nodes += 1;
            let e = piece.rotated(r);
            if !hard_fit(&e, &want) {
                continue;
            }
            board.place(pos, pid, r);
            used[pid as usize] = true;
            if fill_frame_dfs(board, used, instance, cells, idx + 1, order, cap, budget, nodes) {
                return true;
            }
            board.clear(pos);
            used[pid as usize] = false;
        }
    }
    false
}

/// Per-side requirements for the piece at `pos`, URDL. `Some(c)` = this edge is
/// a rim edge (c = 0) or faces an already-placed neighbour whose facing edge is
/// colour `c`; `None` = the neighbour is empty, no requirement.
fn edge_constraints(board: &Board, pieces: &Pieces, pos: usize, s: usize) -> [Option<u8>; 4] {
    let (x, y) = (pos % s, pos / s);
    let at = |cx: usize, cy: usize| -> Option<[u8; 4]> {
        board
            .piece_at(cy * s + cx)
            .map(|(pid, r)| pieces.get(pid).unwrap().rotated(r))
    };
    let up = if y == 0 { Some(0) } else { at(x, y - 1).map(|e| e[2]) };
    let right = if x == s - 1 { Some(0) } else { at(x + 1, y).map(|e| e[3]) };
    let down = if y == s - 1 { Some(0) } else { at(x, y + 1).map(|e| e[0]) };
    let left = if x == 0 { Some(0) } else { at(x - 1, y).map(|e| e[1]) };
    [up, right, down, left]
}

/// True iff `e` satisfies every present constraint exactly (frame building:
/// all constraints are hard).
fn hard_fit(e: &[u8; 4], want: &[Option<u8>; 4]) -> bool {
    want.iter()
        .enumerate()
        .all(|(side, req)| req.is_none_or(|c| e[side] == c))
}

/// Number of present constraints `e` matches (interior greedy: soft, mismatches
/// allowed and simply score 0 later).
fn soft_fit(e: &[u8; 4], want: &[Option<u8>; 4]) -> u32 {
    want.iter()
        .enumerate()
        .filter(|(side, req)| req.is_some_and(|c| e[*side] == c))
        .count() as u32
}

fn main() {
    let secs: f64 = std::env::args()
        .nth(1)
        .and_then(|a| a.parse().ok())
        .unwrap_or(30.0);

    // The canonical instance with the five official clues pinned. The vault
    // pages do not record clue status for vols 74-79; pinning is the
    // conservative choice (see ../PLAN.md, section b).
    let instance = official_instance(true);
    let frame_seed: u64 = std::env::args()
        .nth(2)
        .and_then(|a| a.parse().ok())
        .unwrap_or(1);
    let mut solver = CasAnnular { frame_seed };
    let start = instance.seed_board();
    let budget = Budget::seconds(secs);

    let outcome = solver.solve(&instance, &start, budget);
    let out = instance.finish(&outcome.board);

    println!("solver:  {}", solver.name());
    println!("budget:  {secs} s");
    println!("score:   {} / {}   (canonical, rim-excluding)", out.score, out.max_score);
    println!("outcome: {:?}", outcome.kind);
    println!("nodes:   {}", outcome.nodes);
    println!("breaks:  {}", out.breaks);
    println!("board:   {}", out.url);
    println!();
    println!("NOTE: greedy ring fill is a stand-in for the vault's per-shell MIP;");
    println!("expect a score below the 430-436 plateau until that port lands (PLAN.md).");
}
