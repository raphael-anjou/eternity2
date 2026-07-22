//! A worked *backtracking* solver with a configurable visit order — the
//! second reference point after `my_solver.rs`'s greedy pass.
//!
//!   cargo run --release --example backtracking                    # row-major, seed 1, 10s
//!   cargo run --release --example backtracking -- 42 border 30    # seed, order, seconds
//!
//! Orders: `row` (row-major), `border` (rim first, then row-major interior),
//! `spiral` (outside-in). The solver is a chronological perfect-fit DFS: place
//! only candidates that violate no constraint ([`fit::fit_score`]), backtrack
//! when a cell has none, restart with a reshuffled candidate order when the
//! budget's fraction says a restart is affordable. It keeps the best partial
//! board seen. Path order changes results dramatically on this puzzle — that
//! observation is itself a published finding; measure it with `sweep`.

use e2_kit::{
    fit, generator, instance_from_generated, Board, Budget, Instance, SolveOutcome, Solver,
    XorShift,
};

/// The order in which cells are visited: the search's single most underrated
/// hyperparameter.
#[derive(Clone, Copy, Debug)]
enum Order {
    Row,
    Border,
    Spiral,
}

fn visit_order(order: Order, w: usize, h: usize) -> Vec<usize> {
    match order {
        Order::Row => (0..w * h).collect(),
        Order::Border => {
            let mut rim: Vec<usize> = Vec::new();
            let mut interior: Vec<usize> = Vec::new();
            for pos in 0..w * h {
                let (x, y) = (pos % w, pos / w);
                if x == 0 || y == 0 || x == w - 1 || y == h - 1 {
                    rim.push(pos);
                } else {
                    interior.push(pos);
                }
            }
            rim.extend(interior);
            rim
        }
        Order::Spiral => {
            let (mut x0, mut y0, mut x1, mut y1) = (0usize, 0usize, w - 1, h - 1);
            let mut out = Vec::with_capacity(w * h);
            while x0 <= x1 && y0 <= y1 {
                for x in x0..=x1 {
                    out.push(y0 * w + x);
                }
                for y in (y0 + 1)..=y1 {
                    out.push(y * w + x1);
                }
                if y1 > y0 {
                    for x in (x0..x1).rev() {
                        out.push(y1 * w + x);
                    }
                }
                if x1 > x0 {
                    for y in ((y0 + 1)..y1).rev() {
                        out.push(y * w + x0);
                    }
                }
                x0 += 1;
                y0 += 1;
                if x1 == 0 || y1 == 0 {
                    break;
                }
                x1 -= 1;
                y1 -= 1;
            }
            out
        }
    }
}

struct BacktrackingDfs {
    order: Order,
    seed: u32,
}

impl Solver for BacktrackingDfs {
    fn name(&self) -> String {
        format!("backtracking-{:?}", self.order).to_lowercase()
    }

    fn solve(&mut self, instance: &Instance, start: &Board, budget: Budget) -> SolveOutcome {
        let w = usize::from(instance.width);
        let h = usize::from(instance.height);
        let pieces = &instance.pieces;
        let mut rng = XorShift::new(self.seed);

        // Cells still to fill, in the configured order (pins are skipped).
        let path: Vec<usize> = visit_order(self.order, w, h)
            .into_iter()
            .filter(|&pos| start.piece_at(pos).is_none())
            .collect();

        let mut best = start.clone();
        let mut best_placed = 0usize;
        let mut nodes = 0u64;

        // Restart loop: each restart reshuffles the per-cell candidate order.
        while !budget.expired() {
            let mut board = start.clone();
            let mut used = vec![false; pieces.len()];
            for pos in 0..w * h {
                if let Some((pid, _)) = board.piece_at(pos) {
                    used[usize::from(pid)] = true;
                }
            }

            // A candidate order per restart: piece ids shuffled once, shared
            // by every cell. Cheap and enough to decorrelate restarts.
            let mut cand: Vec<u16> = (0..pieces.len() as u16).collect();
            for i in (1..cand.len()).rev() {
                let j = rng.next_below(i as u32 + 1) as usize;
                cand.swap(i, j);
            }

            // Chronological DFS over `path` with per-cell resume indices.
            let mut choice: Vec<usize> = vec![0; path.len()];
            let mut depth = 0usize;
            loop {
                if budget.expired() {
                    break;
                }
                if depth == path.len() {
                    // Full board: perfect fit everywhere by construction.
                    return SolveOutcome::complete(board).with_nodes(nodes);
                }
                let pos = path[depth];
                let (x, y) = (pos % w, pos / w);
                let want = fit::edge_constraints(&board, pieces, x, y, w, h);
                let mut placed = false;
                while choice[depth] < cand.len() * 4 {
                    let k = choice[depth];
                    choice[depth] += 1;
                    let pid = cand[k / 4];
                    if used[usize::from(pid)] {
                        continue;
                    }
                    let r = (k % 4) as u8;
                    let e = pieces.get(pid).expect("piece").rotated(r);
                    if fit::fit_score(&e, &want).is_some() {
                        board.place(pos, pid, r);
                        used[usize::from(pid)] = true;
                        nodes += 1;
                        placed = true;
                        break;
                    }
                }
                if placed {
                    depth += 1;
                    if depth > best_placed {
                        best_placed = depth;
                        best = board.clone();
                    }
                } else {
                    // Exhausted this cell: reset its counter, undo the cell
                    // below, and resume its enumeration.
                    choice[depth] = 0;
                    if depth == 0 {
                        break; // restart
                    }
                    depth -= 1;
                    let back = path[depth];
                    if let Some((pid, _)) = board.piece_at(back) {
                        used[usize::from(pid)] = false;
                    }
                    board.clear(back);
                }
            }
        }
        SolveOutcome::improved(best).with_nodes(nodes)
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let seed: u32 = args.next().and_then(|s| s.parse().ok()).unwrap_or(1);
    let order = match args.next().as_deref() {
        Some("border") => Order::Border,
        Some("spiral") => Order::Spiral,
        _ => Order::Row,
    };
    let secs: f64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(10.0);

    let puzzle = generator::generate_framed(16, 22, seed, true);
    let instance = instance_from_generated(&format!("gen-{seed}"), &puzzle);
    let mut solver = BacktrackingDfs { order, seed };
    let out = solver.solve(&instance, &instance.seed_board(), Budget::seconds(secs));
    let solved = instance.finish(&out.board);
    println!(
        "{} seed={} score={} nodes={} url={}",
        solver.name(),
        seed,
        solved.score,
        out.nodes,
        solved.url
    );
}
