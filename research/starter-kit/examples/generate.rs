//! Generate one solvable board with real Eternity-II colour balance and print
//! its eternity2.dev viewer URL.
//!
//!   cargo run --release --example generate                 # official 16×16, seed 1
//!   cargo run --release --example generate -- 16 22 7 framed
//!   cargo run --release --example generate -- <size> <colors> <seed> [framed]
//!
//! The generator is `e2-core::generator` — the same one the site's board
//! generator uses. `framed` confines five border colours to the frame, exactly
//! as the real puzzle does. Deterministic: the same arguments always print the
//! same board.

use e2_kit::{generator, instance_from_generated};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let size: u8 = arg(&args, 0).unwrap_or(16);
    let colors: u8 = arg(&args, 1).unwrap_or(22);
    let seed: u32 = arg(&args, 2).unwrap_or(1);
    let framed = args.get(3).is_some_and(|a| a == "framed" || a == "true");

    let max_colors = generator::max_colors(size);
    if colors > max_colors {
        eprintln!("note: {colors} colours exceeds the max for size {size} ({max_colors}); clamping.");
    }

    let puzzle = generator::generate_framed(size, colors, seed, framed);
    let instance = instance_from_generated(
        &format!("gen-{size}x{size}-c{colors}-s{seed}"),
        &puzzle,
    );

    // A solved board (identity placement) so the printed URL shows the solution;
    // finish() re-scores it canonically, which for a solved board is the max.
    let solved = generator::generate_solved_framed(size, colors, seed, framed);
    let solved_instance =
        instance_from_generated(&instance.name, &solved);
    let mut board = e2_kit::Board::new();
    for i in 0..solved.pieces.len() {
        board.place(i, i as u16, 0);
    }
    let out = solved_instance.finish(&board);

    println!("generated {size}×{size}, {} colours, seed {seed}, framed={framed}", puzzle.num_colors);
    println!("pieces: {}", puzzle.pieces.len());
    println!("solution score: {} / {}", out.score, out.max_score);
    println!("solution URL:\n{}", out.url);
}

fn arg<T: std::str::FromStr>(args: &[String], i: usize) -> Option<T> {
    args.get(i).and_then(|s| s.parse().ok())
}
