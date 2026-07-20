//! Single-core throughput of the kit's hot primitives, native (not WASM).
//!
//!   cargo run --release --example bench
//!
//! The kit does not ship a search engine — that is your part — so this measures
//! the plumbing you build *on*: how fast boards generate, and how fast the
//! canonical scorer runs. Knowing these tells you the overhead budget your
//! solver's inner loop has to beat. Run with `--release`; a debug build is
//! ~10–30× slower and the numbers are meaningless.
//!
//! For a full solver's nodes/sec figure, benchmark your own `Solver` — the sweep
//! runner already records per-board wall time in `results.jsonl`.

use std::time::Instant;

use e2_kit::{generator, score_cells, Board, Instance};

fn main() {
    if cfg!(debug_assertions) {
        eprintln!("warning: debug build — run with --release for meaningful numbers.\n");
    }

    // ---- generation throughput: official 16×16/22 boards back to back --------
    // Warm up.
    let _ = generator::generate_framed(16, 22, 0, true);
    let mut gen_count = 0u64;
    let mut seed = 1u32;
    let t0 = Instant::now();
    while t0.elapsed().as_secs_f64() < 2.0 {
        let _ = generator::generate_framed(16, 22, seed, true);
        seed = seed.wrapping_add(1);
        gen_count += 1;
    }
    let gen_secs = t0.elapsed().as_secs_f64();
    let gen_per_s = gen_count as f64 / gen_secs;

    // ---- scoring throughput: re-score one solved 16×16 board repeatedly ------
    let solved = generator::generate_solved_framed(16, 22, 1, true);
    let instance = Instance {
        name: "bench".into(),
        width: solved.width,
        height: solved.height,
        num_colors: solved.num_colors,
        pieces: e2_kit::Pieces::new(solved.pieces.clone()),
        hints: Vec::new(),
    };
    let mut board = Board::new();
    for id in 0..solved.pieces.len() {
        board.place(id, id as u16, 0);
    }
    let cells = board.to_edge_cells(&instance.pieces);
    // Warm up + correctness anchor: a solved board scores the maximum.
    assert_eq!(score_cells(&cells), instance.max_score());

    let mut score_count = 0u64;
    let s0 = Instant::now();
    while s0.elapsed().as_secs_f64() < 2.0 {
        // Score a batch between clock reads so the timer isn't the bottleneck.
        for _ in 0..10_000 {
            std::hint::black_box(score_cells(std::hint::black_box(&cells)));
        }
        score_count += 10_000;
    }
    let score_secs = s0.elapsed().as_secs_f64();
    let scores_per_s = score_count as f64 / score_secs;

    println!("native single-core (release):");
    println!(
        "  {gen_per_s:.0} boards/s generated   (official 16×16/22 framed, {gen_count} boards in {gen_secs:.1}s)",
    );
    println!(
        "  {:.1}M scorings/s          (canonical rim-excluding scorer on a full 16×16 board)",
        scores_per_s / 1e6,
    );
}
