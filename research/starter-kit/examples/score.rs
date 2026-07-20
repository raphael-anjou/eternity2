//! Score any board through the one canonical (rim-excluding) scorer.
//!
//!   cargo run --release --example score -- "<eternity2.dev or bucas URL>"
//!   cargo run --release --example score           # scores a generated solved board (480)
//!
//! The score is `score_cells` from `e2-core` — the same number the site's
//! viewer and every other engine report, because it is literally the same code.
//! Rim (grey/0) seams never count.

use e2_kit::{generator, instance_from_generated, score_url, Board};

fn main() {
    if let Some(url) = std::env::args().nth(1) {
        report_url(&url);
    } else {
        eprintln!("no URL given — scoring a generated solved board.");
        eprintln!("usage: cargo run --release --example score -- \"<board URL>\"\n");
        report_generated_solution();
    }
}

fn report_url(url: &str) {
    if let Some(score) = score_url(url) {
        println!("score: {score}");
        println!("(matched interior edges, rim excluded — the canonical convention)");
    } else {
        eprintln!("could not parse a board out of that URL.");
        eprintln!("expected an eternity2.dev / e2.bucas.name link with board_edges=…");
        std::process::exit(1);
    }
}

/// With no URL, score a generated *solved* board: the generator builds it so the
/// identity placement (piece i at cell i) is the solution, so it scores the
/// maximum, 480. A quick proof that the generator and the canonical scorer agree.
fn report_generated_solution() {
    let solved = generator::generate_solved_framed(16, 22, 1, true);
    let instance = instance_from_generated("solved", &solved);
    let mut board = Board::new();
    for id in 0..solved.pieces.len() {
        board.place(id, id as u16, 0);
    }
    let out = instance.finish(&board);
    println!("score: {} / {}", out.score, out.max_score);
    println!("(a generated board placed at its solution — a full board scores the max)");
}
