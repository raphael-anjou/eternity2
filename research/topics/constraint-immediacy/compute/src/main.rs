//! Reproduction code for the constraint-immediacy principle.
//!
//!   cargo run --release -- korder            # Part 1: exact path-invariance check
//!   cargo run --release -- solve [S]         # Part 2: fixed-order solvers, S sec/order
//!   cargo run --release -- repro [S] [DIR]   # final run: both parts -> DIR/*.json
//!
//! Part 1 verifies, for five visit orders of the official 16x16 board, that the
//! sum of per-cell already-decided constraint counts k_i equals the interior
//! edge count (480) regardless of order, and prints each order's k-histogram.
//!
//! Part 2 runs two fixed-order solvers per visit order at an equal budget:
//! a greedy best-fit pass (full board, breaks allowed) and a perfect-fit DFS
//! with chronological backtracking (deepest consistent prefix). The claim under
//! test is the ranking border-first > row-major > spiral > hint-link. The repro
//! mode also runs a generated-board variance supplement on the top-2 orders.

use e2_kit::analysis::clue_cells;
use e2_kit::fit::{edge_constraints, fit_counts, fit_score};
use e2_kit::{
    generator, instance_from_generated, official_instance, Board, Budget, Instance, SolveOutcome,
    Solver,
};
use std::fmt::Write as _;

const N: usize = 16;
const CELLS: usize = N * N;

// ---------------------------------------------------------------------------
// Visit orders
// ---------------------------------------------------------------------------

fn row_major() -> Vec<usize> {
    (0..CELLS).collect()
}

fn boustrophedon() -> Vec<usize> {
    let mut order = Vec::with_capacity(CELLS);
    for y in 0..N {
        if y % 2 == 0 {
            order.extend((0..N).map(|x| y * N + x));
        } else {
            order.extend((0..N).rev().map(|x| y * N + x));
        }
    }
    order
}

/// Clockwise outer spiral from (0,0) inward.
fn outer_spiral() -> Vec<usize> {
    let mut order = Vec::with_capacity(CELLS);
    let (mut top, mut bottom, mut left, mut right) = (0isize, N as isize - 1, 0isize, N as isize - 1);
    while top <= bottom && left <= right {
        for x in left..=right {
            order.push((top * N as isize + x) as usize);
        }
        top += 1;
        for y in top..=bottom {
            order.push((y * N as isize + right) as usize);
        }
        right -= 1;
        if top <= bottom {
            for x in (left..=right).rev() {
                order.push((bottom * N as isize + x) as usize);
            }
            bottom -= 1;
        }
        if left <= right {
            for y in (top..=bottom).rev() {
                order.push((y * N as isize + left) as usize);
            }
            left += 1;
        }
    }
    debug_assert_eq!(order.len(), CELLS);
    order
}

/// All rim cells first (clockwise walk from the top-left corner), then the
/// interior in row-major order. The border sub-pool is the most constrained
/// (grey edges are hard), so its restrictions bind immediately.
fn border_first() -> Vec<usize> {
    let mut order = Vec::with_capacity(CELLS);
    for x in 0..N {
        order.push(x); // top row
    }
    for y in 1..N {
        order.push(y * N + (N - 1)); // right column
    }
    for x in (0..N - 1).rev() {
        order.push((N - 1) * N + x); // bottom row
    }
    for y in (1..N - 1).rev() {
        order.push(y * N); // left column
    }
    for y in 1..N - 1 {
        for x in 1..N - 1 {
            order.push(y * N + x);
        }
    }
    debug_assert_eq!(order.len(), CELLS);
    order
}

/// A hint-linking path: from cell 0, walk L-shaped Manhattan corridors to each
/// official clue cell in greedy nearest order, then fill the rest row-major.
///
/// Faithfulness: this reproduces the *shape* of the vol-36 hint-link path
/// (k=1 corridors punctuated by high-k pockets), not the exact historical cell
/// list, which was not preserved. The claim is about the shape; see PLAN.md.
fn hint_link(hint_cells: &[usize]) -> Vec<usize> {
    let mut order = Vec::with_capacity(CELLS);
    let mut seen = [false; CELLS];
    let push = |order: &mut Vec<usize>, seen: &mut [bool; CELLS], pos: usize| {
        if !seen[pos] {
            seen[pos] = true;
            order.push(pos);
        }
    };
    let mut cur = 0usize;
    push(&mut order, &mut seen, cur);
    let mut remaining: Vec<usize> = hint_cells.to_vec();
    while !remaining.is_empty() {
        let (cx, cy) = (cur % N, cur / N);
        let (idx, _) = remaining
            .iter()
            .enumerate()
            .min_by_key(|(_, &h)| {
                let (hx, hy) = (h % N, h / N);
                cx.abs_diff(hx) + cy.abs_diff(hy)
            })
            .expect("non-empty");
        let target = remaining.swap_remove(idx);
        let (tx, ty) = (target % N, target / N);
        let (mut x, mut y) = (cur % N, cur / N);
        while x != tx {
            x = if tx > x { x + 1 } else { x - 1 };
            push(&mut order, &mut seen, y * N + x);
        }
        while y != ty {
            y = if ty > y { y + 1 } else { y - 1 };
            push(&mut order, &mut seen, y * N + x);
        }
        cur = target;
    }
    for pos in 0..CELLS {
        push(&mut order, &mut seen, pos);
    }
    debug_assert_eq!(order.len(), CELLS);
    order
}

fn orders(instance: &Instance) -> Vec<(&'static str, Vec<usize>)> {
    let hint_cells = clue_cells(instance);
    vec![
        ("hint-link", hint_link(&hint_cells)),
        ("outer-spiral", outer_spiral()),
        ("row-major", row_major()),
        ("boustrophedon", boustrophedon()),
        ("border-first", border_first()),
    ]
}

// ---------------------------------------------------------------------------
// Part 1: the invariance check
// ---------------------------------------------------------------------------

/// k_i for each cell of `order`: how many orthogonal neighbours are already
/// placed when the cell is filled. Returns (histogram over k=0..4, sum).
fn k_profile(order: &[usize]) -> ([u32; 5], u32) {
    let mut placed = [false; CELLS];
    let mut hist = [0u32; 5];
    let mut sum = 0u32;
    for &pos in order {
        let (x, y) = (pos % N, pos / N);
        let mut k = 0usize;
        if y > 0 && placed[pos - N] {
            k += 1;
        }
        if y < N - 1 && placed[pos + N] {
            k += 1;
        }
        if x > 0 && placed[pos - 1] {
            k += 1;
        }
        if x < N - 1 && placed[pos + 1] {
            k += 1;
        }
        hist[k] += 1;
        sum += k as u32;
        placed[pos] = true;
    }
    (hist, sum)
}

fn run_korder(instance: &Instance) -> Vec<(&'static str, [u32; 5], u32)> {
    let expected = 2 * (N as u32) * (N as u32 - 1); // interior edges: 480 for N=16
    println!("Part 1: sum(k) path-invariance on the official 16x16 board");
    println!("expected constant: {expected} interior edges\n");
    println!("{:<14} {:>5} {:>5} {:>5} {:>5} {:>5}   sum(k)", "order", "k=0", "k=1", "k=2", "k=3", "k=4");
    let mut all_ok = true;
    let mut out = Vec::new();
    for (label, order) in orders(instance) {
        let (hist, sum) = k_profile(&order);
        let ok = sum == expected;
        all_ok &= ok;
        println!(
            "{label:<14} {:>5} {:>5} {:>5} {:>5} {:>5}   {sum} {}",
            hist[0],
            hist[1],
            hist[2],
            hist[3],
            hist[4],
            if ok { "OK" } else { "MISMATCH" }
        );
        out.push((label, hist, sum));
    }
    assert!(all_ok, "invariance violated: some order's sum(k) != {expected}");
    println!("\nsum(k) is identical for every order: a path cannot add restriction,");
    println!("it only schedules when each restriction binds (see PLAN.md).");
    out
}

// ---------------------------------------------------------------------------
// Part 2: fixed-order solvers
// ---------------------------------------------------------------------------

/// Greedy best-fit in a fixed visit order: fills the whole board, breaks
/// allowed when nothing fits perfectly (it then places the least-breaking
/// unused piece), no backtracking. This is the "full board with breaks"
/// measurement arm.
struct FixedOrderGreedy {
    label: String,
    order: Vec<usize>,
}

impl Solver for FixedOrderGreedy {
    fn name(&self) -> String {
        format!("greedy-{}", self.label)
    }

    fn solve(&mut self, instance: &Instance, start: &Board, budget: Budget) -> SolveOutcome {
        let pieces = &instance.pieces;
        let w = usize::from(instance.width);
        let h = usize::from(instance.height);
        let mut board = start.clone();
        let mut used = vec![false; pieces.len()];
        for pos in 0..w * h {
            if let Some((pid, _)) = board.piece_at(pos) {
                used[pid as usize] = true;
            }
        }
        let mut nodes = 0u64;
        let mut ran_out = false;
        for &pos in &self.order {
            if budget.expired() {
                ran_out = true;
                break;
            }
            if !board.is_empty_at(pos) {
                continue;
            }
            let want = edge_constraints(&board, pieces, pos % w, pos / w, w, h);
            // Best perfect fit first; failing that, the placement breaking the
            // fewest already-decided constraints (rim included as decided).
            let mut best: Option<(u16, u8, i32)> = None;
            for (pid, piece) in pieces.iter() {
                if used[pid as usize] {
                    continue;
                }
                for r in 0..4 {
                    nodes += 1;
                    let (ok, bad) = fit_counts(&piece.rotated(r), &want);
                    let score = ok as i32 - bad as i32;
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
        let full = (0..w * h).all(|pos| !board.is_empty_at(pos));
        let outcome = if full && !ran_out {
            SolveOutcome::complete(board)
        } else {
            SolveOutcome::improved(board)
        };
        outcome.with_nodes(nodes)
    }
}

/// Perfect-fit DFS in a fixed visit order with chronological backtracking.
/// Returns the deepest consistent prefix reached within the budget. The
/// decision-to-refutation distance of the order shows up directly as the
/// wasted subtree between a wrong corridor placement and its late refutation.
struct FixedOrderDfs {
    label: String,
    order: Vec<usize>,
}

impl Solver for FixedOrderDfs {
    fn name(&self) -> String {
        format!("dfs-{}", self.label)
    }

    fn solve(&mut self, instance: &Instance, start: &Board, budget: Budget) -> SolveOutcome {
        let pieces = &instance.pieces;
        let w = usize::from(instance.width);
        let h = usize::from(instance.height);
        let mut board = start.clone();
        let mut used = vec![false; pieces.len()];
        for pos in 0..w * h {
            if let Some((pid, _)) = board.piece_at(pos) {
                used[pid as usize] = true;
            }
        }
        let cells: Vec<usize> = self.order.iter().copied().filter(|&p| board.is_empty_at(p)).collect();

        let mut stack: Vec<Vec<(u16, u8)>> = Vec::with_capacity(cells.len());
        let mut depth = 0usize;
        let mut best_depth = 0usize;
        let mut best_board = board.clone();
        let mut nodes = 0u64;

        loop {
            if nodes.trailing_zeros() >= 12 && budget.expired() {
                break;
            }
            if depth == cells.len() {
                return SolveOutcome::complete(board).with_nodes(nodes);
            }
            if stack.len() == depth {
                let pos = cells[depth];
                let want = edge_constraints(&board, pieces, pos % w, pos / w, w, h);
                let mut cands: Vec<(u16, u8)> = Vec::new();
                for (pid, piece) in pieces.iter() {
                    if used[pid as usize] {
                        continue;
                    }
                    for r in 0..4 {
                        nodes += 1;
                        if fit_score(&piece.rotated(r), &want).is_some() {
                            cands.push((pid, r));
                        }
                    }
                }
                // Faithfulness note: the research engine ordered candidates by
                // a value heuristic and used restarts; plain id order here.
                cands.reverse(); // pop() explores low piece ids first
                stack.push(cands);
            }
            if let Some((pid, r)) = stack[depth].pop() {
                board.place(cells[depth], pid, r);
                used[pid as usize] = true;
                depth += 1;
                if depth > best_depth {
                    best_depth = depth;
                    best_board = board.clone();
                }
            } else {
                stack.pop();
                if depth == 0 {
                    break; // exhausted the whole order's perfect-fit space
                }
                depth -= 1;
                let (pid, _) = board.piece_at(cells[depth]).expect("placed at this depth");
                used[pid as usize] = false;
                board.clear(cells[depth]);
            }
        }
        SolveOutcome::improved(best_board).with_nodes(nodes)
    }
}

struct ArmResult {
    label: String,
    greedy_score: u32,
    greedy_url: String,
    dfs_depth: usize,
    dfs_score: u32,
    dfs_url: String,
    greedy_nodes: u64,
    dfs_nodes: u64,
}

fn run_orders(
    instance: &Instance,
    order_list: &[(&'static str, Vec<usize>)],
    secs: f64,
) -> Vec<ArmResult> {
    let mut out = Vec::new();
    for (label, order) in order_list {
        let start = instance.seed_board();

        let mut greedy = FixedOrderGreedy { label: (*label).into(), order: order.clone() };
        let g = greedy.solve(instance, &start, Budget::seconds(secs));
        let g_out = instance.finish(&g.board);

        let mut dfs = FixedOrderDfs { label: (*label).into(), order: order.clone() };
        let d = dfs.solve(instance, &start, Budget::seconds(secs));
        let d_out = instance.finish(&d.board);
        let cell_count = usize::from(instance.width) * usize::from(instance.height);
        let d_depth = (0..cell_count).filter(|&p| !d.board.is_empty_at(p)).count();

        out.push(ArmResult {
            label: (*label).into(),
            greedy_score: g_out.score,
            greedy_url: g_out.url,
            dfs_depth: d_depth,
            dfs_score: d_out.score,
            dfs_url: d_out.url,
            greedy_nodes: g.nodes,
            dfs_nodes: d.nodes,
        });
    }
    out
}

fn print_results(results: &[ArmResult], with_urls: bool) {
    println!(
        "{:<14} {:>14} {:>16} {:>12}",
        "order", "greedy score", "dfs best depth", "dfs score"
    );
    for r in results {
        println!(
            "{:<14} {:>10}/480 {:>14}/256 {:>8}/480",
            r.label, r.greedy_score, r.dfs_depth, r.dfs_score
        );
        if with_urls {
            println!("  greedy board: {}", r.greedy_url);
            println!("  dfs board:    {}", r.dfs_url);
        }
    }
}

fn run_solve(instance: &Instance, secs: f64) -> Vec<ArmResult> {
    println!("Part 2: fixed-order solvers on the official 16x16 board ({secs} s/order/arm)");
    println!("claim under test: border-first > row-major ~ boustrophedon > spiral > hint-link\n");
    let results = run_orders(instance, &orders(instance), secs);
    print_results(&results, true);
    println!("\nBoards above are the always-output artifacts; write results/ only from a final run.");
    results
}

// ---------------------------------------------------------------------------
// repro mode: final run -> results/*.json (deterministic field order)
// ---------------------------------------------------------------------------

fn json_korder(rows: &[(&'static str, [u32; 5], u32)]) -> String {
    let mut s = String::new();
    s.push_str("{\n  \"tier\": \"exact\",\n  \"board\": \"official-16x16\",\n");
    s.push_str("  \"expected_sum_k\": 480,\n  \"orders\": [\n");
    for (i, (label, hist, sum)) in rows.iter().enumerate() {
        let _ = write!(
            s,
            "    {{\"order\": \"{label}\", \"k_hist\": [{},{},{},{},{}], \"sum_k\": {sum}}}",
            hist[0], hist[1], hist[2], hist[3], hist[4]
        );
        s.push_str(if i + 1 < rows.len() { ",\n" } else { "\n" });
    }
    s.push_str("  ]\n}\n");
    s
}

fn json_arm(r: &ArmResult, indent: &str, url: bool) -> String {
    let mut s = String::new();
    let _ = write!(
        s,
        "{indent}{{\"order\": \"{}\", \"greedy_score\": {}, \"dfs_depth\": {}, \"dfs_score\": {}, \"greedy_nodes\": {}, \"dfs_nodes\": {}",
        r.label, r.greedy_score, r.dfs_depth, r.dfs_score, r.greedy_nodes, r.dfs_nodes
    );
    if url {
        let _ = write!(s, ",\n{indent} \"greedy_url\": \"{}\",\n{indent} \"dfs_url\": \"{}\"", r.greedy_url, r.dfs_url);
    }
    s.push('}');
    s
}

fn repro_korder(instance: &Instance, dir: &str) {
    std::fs::create_dir_all(dir).expect("create results dir");
    let korder_rows = run_korder(instance);
    let korder_path = format!("{dir}/korder-invariance.json");
    std::fs::write(&korder_path, json_korder(&korder_rows)).expect("write korder json");
    println!("\nwrote {korder_path}");
}

/// Part 2 headline: official instance, all five orders -> JSON.
fn repro_official(instance: &Instance, secs: f64, dir: &str) {
    std::fs::create_dir_all(dir).expect("create results dir");
    let official = run_solve(instance, secs);
    let mut s = String::new();
    s.push_str("{\n  \"tier\": \"seeded-statistical\",\n");
    s.push_str("  \"board\": \"official-16x16\",\n");
    let _ = write!(s, "  \"budget_secs_per_order_per_arm\": {secs},\n");
    s.push_str("  \"score_convention\": \"matched interior edges / 480 (rim-excluding canonical scorer)\",\n");
    s.push_str("  \"orders\": [\n");
    for (i, r) in official.iter().enumerate() {
        s.push_str(&json_arm(r, "    ", true));
        s.push_str(if i + 1 < official.len() { ",\n" } else { "\n" });
    }
    s.push_str("  ]\n}\n");
    let path = format!("{dir}/solver-ranking-official.json");
    std::fs::write(&path, s).expect("write official ranking json");
    println!("\nwrote {path}");
}

/// Variance supplement: generated framed 16x16/22 boards, top-2 orders
/// (border-first, row-major), same per-order budget.
fn repro_gen(secs: f64, seeds: &[u32], dir: &str) {
    std::fs::create_dir_all(dir).expect("create results dir");
    println!("Variance supplement: generated framed 16x16/22 boards, seeds {seeds:?},");
    println!("top-2 orders at the same budget ({secs} s/order/arm)\n");
    let mut gen_results: Vec<(u32, Vec<ArmResult>)> = Vec::new();
    for &seed in seeds {
        let p = generator::generate_framed(16, 22, seed, true);
        let gi = instance_from_generated(&format!("gen-16x16-22-s{seed}"), &p);
        let top2 = vec![("row-major", row_major()), ("border-first", border_first())];
        let rs = run_orders(&gi, &top2, secs);
        println!("seed {seed}:");
        print_results(&rs, false);
        gen_results.push((seed, rs));
    }
    let mut s = String::new();
    s.push_str("{\n  \"tier\": \"seeded-statistical\",\n");
    s.push_str("  \"board\": \"generated framed 16x16, 22 colours (kit generator), no hints\",\n");
    let _ = write!(s, "  \"budget_secs_per_order_per_arm\": {secs},\n");
    s.push_str("  \"score_convention\": \"matched interior edges / 480 (rim-excluding canonical scorer)\",\n");
    s.push_str("  \"seeds\": [\n");
    for (i, (seed, rs)) in gen_results.iter().enumerate() {
        let _ = write!(s, "    {{\"seed\": {seed}, \"arms\": [\n");
        for (j, r) in rs.iter().enumerate() {
            s.push_str(&json_arm(r, "      ", false));
            s.push_str(if j + 1 < rs.len() { ",\n" } else { "\n" });
        }
        s.push_str("    ]}");
        s.push_str(if i + 1 < gen_results.len() { ",\n" } else { "\n" });
    }
    s.push_str("  ]\n}\n");
    let path = format!("{dir}/solver-ranking-generated.json");
    std::fs::write(&path, s).expect("write generated ranking json");
    println!("\nwrote {path}");
}

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_else(|| "korder".into());
    let instance = official_instance(true);
    match mode.as_str() {
        "korder" => {
            run_korder(&instance);
        }
        "solve" => {
            let secs: f64 = std::env::args().nth(2).and_then(|s| s.parse().ok()).unwrap_or(5.0);
            run_solve(&instance, secs);
        }
        "repro-korder" => {
            let dir = std::env::args().nth(2).unwrap_or_else(|| "../results".into());
            repro_korder(&instance, &dir);
        }
        "repro-official" => {
            let secs: f64 = std::env::args().nth(2).and_then(|s| s.parse().ok()).unwrap_or(60.0);
            let dir = std::env::args().nth(3).unwrap_or_else(|| "../results".into());
            repro_official(&instance, secs, &dir);
        }
        "repro-gen" => {
            let secs: f64 = std::env::args().nth(2).and_then(|s| s.parse().ok()).unwrap_or(60.0);
            let seeds: Vec<u32> = std::env::args()
                .nth(3)
                .map(|s| s.split(',').filter_map(|t| t.parse().ok()).collect())
                .unwrap_or_else(|| (1..=4).collect());
            let dir = std::env::args().nth(4).unwrap_or_else(|| "../results".into());
            repro_gen(secs, &seeds, &dir);
        }
        other => {
            eprintln!(
                "unknown mode '{other}'; use: korder | solve [secs] | repro-korder [dir] | repro-official [secs] [dir] | repro-gen [secs] [seeds] [dir]"
            );
            std::process::exit(2);
        }
    }
}
