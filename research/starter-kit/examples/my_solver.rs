//! A worked baseline solver — copy this file, rename it, and make it your idea.
//!
//!   cargo run --release --example my_solver          # solve one official board
//!   cargo run --release --example my_solver -- 42    # ...for seed 42
//!
//! `GreedyRowMajor` is deliberately simple: it walks the board in reading order
//! and drops the best-fitting unused piece into each empty cell (most matched
//! edges against already-placed neighbours, ties broken by piece id). It honours
//! pinned hints (it starts from `instance.seed_board()`) and stops early if the
//! budget runs out. It is NOT good — a greedy pass with no backtracking plateaus
//! low — but it is a complete, correct [`Solver`] you can run, sweep, and then
//! improve. That gap is the point: swap in real search and watch `compare` move.
//!
//! What to change: replace the body of `solve`. Everything else in the kit — the
//! sweep runner, run directories, the compare tool — keeps working unchanged,
//! because they only depend on the [`Solver`] trait.

use e2_kit::{
    generator, instance_from_generated, Board, Budget, Instance, Pieces, SolveOutcome, Solver,
};

/// Greedy row-major placement. Your starting point.
struct GreedyRowMajor;

impl Solver for GreedyRowMajor {
    fn name(&self) -> String {
        "greedy-row-major".into()
    }

    fn solve(&mut self, instance: &Instance, start: &Board, budget: Budget) -> SolveOutcome {
        let n = usize::from(instance.width) * usize::from(instance.height);
        let w = usize::from(instance.width);
        let pieces = &instance.pieces;

        // Continue from the starting board (it already carries any pinned clues,
        // and could be a partial board handed in for seed-and-grow work).
        let mut board = start.clone();
        let mut used = vec![false; pieces.len()];
        for pos in 0..n {
            if let Some((pid, _)) = board.piece_at(pos) {
                used[pid as usize] = true;
            }
        }

        let mut nodes = 0u64; // piece+rotation fits tried — the search work done
        let mut ran_out = false;
        for pos in 0..n {
            if budget.expired() {
                ran_out = true;
                break; // return the best-so-far partial board
            }
            if !board.is_empty_at(pos) {
                continue; // a hint or an already-placed cell
            }
            let (x, y) = (pos % w, pos / w);

            // Required border colour on each side: 0 (grey) on a rim edge,
            // otherwise "must match the neighbour that is already placed", or
            // "don't care" (None) when the neighbour is empty/off-board interior.
            let want = edge_constraints(&board, pieces, x, y, instance.width);

            // Pick the unused piece+rotation matching the most hard constraints.
            let mut best: Option<(u16, u8, u32)> = None;
            for (pid, piece) in pieces.iter() {
                if used[pid as usize] {
                    continue;
                }
                for r in 0..4 {
                    nodes += 1;
                    let e = piece.rotated(r);
                    if let Some(score) = fit_score(&e, &want) {
                        if best.is_none_or(|(_, _, b)| score > b) {
                            best = Some((pid, r, score));
                        }
                    }
                }
            }

            if let Some((pid, r, _)) = best {
                board.place(pos, pid, r);
                used[pid as usize] = true;
            }
            // If nothing fits every hard (border) constraint, leave the cell
            // empty; a real solver would backtrack here instead.
        }

        // Be honest about the outcome. `Complete` means a *full* board — every
        // cell placed. Greedy has no backtracking, so it routinely leaves cells
        // empty; a board with holes is NOT complete, and calling a budget cut-off
        // "complete" would be worse. Either way it's `Improved`: a best-effort
        // board, neither full nor proven. (Greedy never exhausts a space or
        // computes a bound — `examples/bound.rs` shows those outcomes.)
        let full = (0..n).all(|pos| !board.is_empty_at(pos));
        let outcome = if full && !ran_out {
            SolveOutcome::complete(board)
        } else {
            SolveOutcome::improved(board)
        };
        outcome.with_nodes(nodes)
    }
}

/// Per-side requirements for the piece at `(x, y)`, URDL. `Some(c)` = this edge
/// must be colour `c` (a rim edge must be 0, or an interior edge must match an
/// already-placed neighbour); `None` = free.
fn edge_constraints(board: &Board, pieces: &Pieces, x: usize, y: usize, size: u8) -> [Option<u8>; 4] {
    let s = usize::from(size);
    let at = |cx: usize, cy: usize| -> Option<[u8; 4]> {
        let pos = cy * s + cx;
        board.piece_at(pos).map(|(pid, r)| pieces.get(pid).unwrap().rotated(r))
    };
    // Up (U): rim if y==0, else the down-edge of the piece above.
    let up = if y == 0 {
        Some(0)
    } else {
        at(x, y - 1).map(|e| e[2])
    };
    // Right (R): rim if x==s-1, else the left-edge of the piece to the right.
    let right = if x == s - 1 {
        Some(0)
    } else {
        at(x + 1, y).map(|e| e[3])
    };
    // Down (D): rim if y==s-1, else the up-edge of the piece below.
    let down = if y == s - 1 {
        Some(0)
    } else {
        at(x, y + 1).map(|e| e[0])
    };
    // Left (L): rim if x==0, else the right-edge of the piece to the left.
    let left = if x == 0 {
        Some(0)
    } else {
        at(x - 1, y).map(|e| e[1])
    };
    [up, right, down, left]
}

/// If piece edges `e` satisfy every *hard* constraint (rim colours and placed
/// neighbours), return how many constraints it matched (higher = tighter fit);
/// otherwise `None`. Rim mismatches are disqualifying, so pieces never hang off
/// the border.
fn fit_score(e: &[u8; 4], want: &[Option<u8>; 4]) -> Option<u32> {
    let mut matched = 0;
    for (side, req) in want.iter().enumerate() {
        if let Some(c) = req {
            if e[side] == *c {
                matched += 1;
            } else {
                return None; // violates a hard constraint
            }
        }
    }
    Some(matched)
}

fn main() {
    let seed: u32 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(1);

    // One official-shaped board for this seed.
    let puzzle = generator::generate_framed(16, 22, seed, true);
    let instance = instance_from_generated(&format!("gen-16x16-c22-s{seed}"), &puzzle);

    let mut solver = GreedyRowMajor;
    let budget = Budget::seconds(5.0);
    let start = instance.seed_board();
    let outcome = solver.solve(&instance, &start, budget);
    let out = instance.finish(&outcome.board);

    println!("solver:  {}", solver.name());
    println!("seed:    {seed}");
    println!("score:   {} / {}   (achieved)", out.score, out.max_score);
    println!("outcome: {:?}", outcome.kind);
    println!("nodes:   {}", outcome.nodes);
    println!("breaks:  {}", out.breaks);
    println!("board:   {}", out.url);

    // Continuation demo: `solve` takes a starting board, so you are not limited
    // to the fresh seeded board the sweep hands you. Here we take the board we
    // just produced and run the solver AGAIN from it — a stand-in for
    // seed-and-grow / repair, where each round continues the previous board. A
    // real repair solver would perturb `out.board` first; greedy just re-fills
    // any cells it left empty.
    let outcome2 = solver.solve(&instance, &outcome.board, Budget::seconds(5.0));
    let out2 = instance.finish(&outcome2.board);
    println!(
        "\ncontinued from that board: {} / {}  ({:?})",
        out2.score, out2.max_score, outcome2.kind
    );

    println!("\nnow open src/solver.rs, then copy this file and replace `solve` with your idea.");
}
