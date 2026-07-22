//! Hardness-peak sweep: solver effort vs interior-color count on generated
//! framed 16x16 puzzles.
//!
//! The published difficulty argument (Ansotegui et al.; the community's own
//! observation, Owen msg 1947) says framed edge-matching puzzles are hardest at
//! a particular interior-color count, near Eternity II's own 17. This binary
//! measures that curve in-repo, using the same seeded generator and step-able
//! DFS solver the site ships.
//!
//! One invocation does ONE (color, seed) cell of the sweep so the shell can run
//! cells in parallel and resume: it generates a solvable framed 16x16 board with
//! `c` interior colors plus a real-E2-style 5-color border band, then runs the
//! engine's backtracking DFS until it either reaches a first full solution or
//! spends a fixed node budget, and prints a single JSON row with the outcome.
//!
//! Why interior colors, not total colors: real Eternity II fixes 5 border colors
//! and varies the interior palette (17 of them). We hold the border band at 5
//! and sweep the interior count `c`, so `total = c + 5`. That keeps the border
//! structure identical across the sweep and isolates the one knob the phase
//! transition is about.
//!
//! Why a NODE budget, not wall-clock: one DFS step (a placement or a backtrack)
//! is the same unit of work on any machine, so a node cap makes the "solved /
//! effort" verdict deterministic and hardware-independent. Wall-clock seconds
//! are still recorded, but only as a convenience; the peak is located by nodes
//! and solve rate, never by seconds.

use eternity2_engine::generator::{frame_color_count, generate_solved_framed_bc, max_colors, XorShift};
use eternity2_engine::{build_path, rotated, Solver, Status};
use std::time::Instant;

/// Board edge length. Defaults to 16 (real Eternity II). Overridable via the
/// E2_SIZE env var for calibration probes: a plain DFS cannot solve a full
/// 16x16 at any color count, so the peak in nodes-to-solution is only
/// TRAVERSABLE on a smaller board where instances actually get solved. See the
/// crate README and the topic article's Method section.
fn board_size() -> u8 {
    std::env::var("E2_SIZE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(16)
}
/// Real Eternity II's border-color count. Held fixed across the whole sweep so
/// only the interior palette varies.
const BORDER_COLORS: u8 = 5;
/// DFS cell-visit order. row-major is the plain, unguided order; the point of
/// the sweep is the instance family's intrinsic hardness, not a clever path.
const PATH_KIND: &str = "row-major";
/// Steps run per `step` call. Only affects how often we check the budget, not
/// the search itself. Large enough that the check overhead is negligible.
const STEP_CHUNK: u32 = 1_000_000;

fn usage() -> ! {
    eprintln!("usage: hardness_peak <interior_colors> <seed> <node_budget>");
    eprintln!("       hardness_peak --criterion <size>");
    eprintln!("  interior_colors: number of interior colors c (total = c + 5 border)");
    eprintln!("  seed:            u32 generator/scramble seed (deterministic)");
    eprintln!("  node_budget:     max DFS steps (placements + backtracks) before giving up");
    eprintln!("  --criterion <size>: print Owen's one-expected-solution interior-color");
    eprintln!("                      count for an n x n board and exit");
    std::process::exit(2)
}

/// Owen's one-expected-solution interior-color count for an n x n framed board.
///
/// First-moment (mean-field) estimate. The interior is a (n-2) x (n-2) grid of
/// `P = (n-2)^2` interior pieces, each with `4^P` orientations and `P!`
/// arrangements. Modelling each interior piece as contributing two independent
/// color-match constraints, each satisfied with probability `1/I`, the expected
/// number of matched interior arrangements is `P! * 4^P / I^(2P)`. Setting that
/// to one and solving for `I`:
///
///   I = (P! * 4^P)^(1/(2P)).
///
/// For n = 16 this is `(196! * 4^196)^(1/392) = 17.14`, exactly Owen's figure
/// (Design the hardest puzzle, groups.io msg 1947, August 2007). We keep his
/// `2P` exponent verbatim so the size-8 value is computed by the identical
/// formula, making the comparison to the measured 8x8 hard band a fair
/// scale-transfer test rather than a re-derivation. Computed in log space
/// (lgamma for `ln P!`) so it stays exact for any n.
fn owen_criterion(size: u32) -> f64 {
    let p = (size.saturating_sub(2)).pow(2) as f64;
    // ln(P!) via the log-gamma function: ln(Gamma(P+1)).
    let ln_p_fact = ln_gamma(p + 1.0);
    let ln_num = ln_p_fact + p * 4f64.ln();
    (ln_num / (2.0 * p)).exp()
}

/// Lanczos approximation of ln(Gamma(x)) for x > 0. Accurate to ~1e-13, far
/// tighter than the criterion needs, and dependency-free.
fn ln_gamma(x: f64) -> f64 {
    const G: f64 = 7.0;
    const C: [f64; 9] = [
        0.999_999_999_999_809_93,
        676.520_368_121_885_1,
        -1_259.139_216_722_402_8,
        771.323_428_777_653_1,
        -176.615_029_162_140_6,
        12.507_343_278_686_905,
        -0.138_571_095_265_720_12,
        9.984_369_578_019_572e-6,
        1.505_632_735_149_311_6e-7,
    ];
    if x < 0.5 {
        // Reflection formula (not needed for our x >= 1, but kept correct).
        std::f64::consts::PI.ln()
            - (std::f64::consts::PI * x).sin().ln()
            - ln_gamma(1.0 - x)
    } else {
        let x = x - 1.0;
        let mut a = C[0];
        let t = x + G + 0.5;
        for (i, &c) in C.iter().enumerate().skip(1) {
            a += c / (x + i as f64);
        }
        0.5 * (2.0 * std::f64::consts::PI).ln() + (x + 0.5) * t.ln() - t + a.ln()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Owen-criterion mode: `--criterion <size>` prints the predicted peak
    // interior-color count and exits, so the article's number is reproducible.
    if args.len() == 3 && args[1] == "--criterion" {
        let size: u32 = args[2].parse().unwrap_or_else(|_| usage());
        let i = owen_criterion(size);
        let p = (size.saturating_sub(2)).pow(2);
        println!(
            "{{\"criterion\": \"owen-one-expected-solution\", \"size\": {size}, \"interiorPieces\": {p}, \"exponent\": {}, \"predictedInteriorColors\": {i:.4}}}",
            2 * p
        );
        return;
    }

    if args.len() != 4 {
        usage();
    }
    let interior_colors: u8 = args[1].parse().unwrap_or_else(|_| usage());
    let seed: u32 = args[2].parse().unwrap_or_else(|_| usage());
    let node_budget: u64 = args[3].parse().unwrap_or_else(|_| usage());
    let size = board_size();

    // Total palette = interior colors + the fixed 5-color border band. Clamp to
    // what this board size can actually carry (max_colors caps at 22, exactly the
    // real puzzle's ceiling), so a request past the ceiling is reported honestly
    // rather than silently generating a different board than asked.
    let requested_total = u16::from(interior_colors) + u16::from(BORDER_COLORS);
    let ceiling = u16::from(max_colors(size));
    let clamped = requested_total > ceiling;
    let total_colors = requested_total.min(ceiling) as u8;

    // Build the solvable framed board with an explicit 5-color border band, so
    // the interior palette is exactly `total_colors - 5` regardless of the
    // default min(5, colors-1) rule. framed = true gives real-E2 structure
    // (border colors confined to the frame band). This returns pieces in
    // SOLUTION order, so we then apply the same scramble `generate_framed` uses
    // (shuffle piece order, random per-piece rotation, seeded by
    // `seed ^ 0xA5A5A5A5`) to hand the solver a genuinely unsolved board. We
    // scramble here rather than call `generate_framed` because only the `_bc`
    // entry point lets us pin the border-color count across the whole sweep.
    let mut puzzle = generate_solved_framed_bc(size, total_colors, seed, true, BORDER_COLORS);
    {
        let mut rng = XorShift::new(seed ^ 0xA5A5_A5A5);
        rng.shuffle(&mut puzzle.pieces);
        for p in &mut puzzle.pieces {
            *p = rotated(*p, rng.below(4) as u8);
        }
    }
    let actual_border = frame_color_count(total_colors).min(BORDER_COLORS);
    let actual_interior = puzzle.num_colors - actual_border;

    let path = build_path(PATH_KIND, size, size, 0).expect("row-major path exists for 16x16");
    // No hints, no piece-order shuffle: the instance's own seed already
    // randomizes piece order and rotations, and we want the raw DFS on the bare
    // board, which is what the phase-transition argument is about.
    let mut solver = Solver::new(&puzzle, &path, false, false, seed).expect("solver init");

    let start = Instant::now();
    let mut nodes_at_solution: Option<f64> = None;
    let mut final_status = Status::Running;
    loop {
        let report = solver.step(STEP_CHUNK);
        match report.status {
            Status::Solved => {
                nodes_at_solution = Some(report.nodes);
                final_status = Status::Solved;
                break;
            }
            Status::Exhausted => {
                final_status = Status::Exhausted;
                break;
            }
            Status::Running => {
                // Budget is measured in DFS steps = placements (nodes) +
                // backtracks. That total is the hardware-independent effort.
                let steps = report.nodes + report.backtracks;
                if steps as u64 >= node_budget {
                    break;
                }
            }
        }
    }
    let seconds = start.elapsed().as_secs_f64();
    let report = solver.report();
    let steps = (report.nodes + report.backtracks) as u64;
    let solved = final_status == Status::Solved;

    // A single JSON object on one line, so the shell can concatenate cells into a
    // JSON array without any parser. `nodes` is steps-to-first-solution when
    // solved, else the budget actually spent (== node_budget on a clean cap, or
    // the exhaustion point if the whole tree was searched first).
    let nodes_field = match nodes_at_solution {
        Some(n) => format!("{}", n as u64),
        None => format!("{steps}"),
    };
    let mut fields = vec![
        format!("\"size\": {size}"),
        format!("\"interiorColors\": {actual_interior}"),
        format!("\"borderColors\": {actual_border}"),
        format!("\"totalColors\": {}", puzzle.num_colors),
        format!("\"seed\": {seed}"),
        format!("\"nodeBudget\": {node_budget}"),
        format!("\"solved\": {solved}"),
        format!("\"exhausted\": {}", final_status == Status::Exhausted),
        format!("\"nodes\": {nodes_field}"),
        format!("\"placements\": {}", report.nodes as u64),
        format!("\"backtracks\": {}", report.backtracks as u64),
        format!("\"bestPlaced\": {}", report.best_placed),
        format!("\"seconds\": {seconds:.3}"),
    ];
    if clamped {
        fields.push(format!(
            "\"requestedInteriorColors\": {interior_colors}, \"clampedToCeiling\": true"
        ));
    }
    println!("{{{}}}", fields.join(", "));
}
