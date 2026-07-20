//! Score any board through the one canonical (rim-excluding) scorer.
//!
//!   cargo run --release --example score -- "<eternity2.dev or bucas URL>"
//!   cargo run --release --example score           # scores a built-in 469 board
//!
//! The score is `score_cells` from `e2-core` — the same number the site's
//! viewer and every other engine report, because it is literally the same code.
//! Rim (grey/0) seams never count.

use e2_kit::score_url;

/// A verified 469/480 community board (McGavin/Blackwood), as the bare
/// `board_edges` blob, so `score` works with no arguments. This is the same
/// board `e2-io`'s cross-validation test scores at 469.
const SAMPLE_469: &str = include_str!("data/known-469.edges");

fn main() {
    let url = std::env::args().nth(1);
    if let Some(u) = url { report(&u) } else {
        eprintln!("no URL given — scoring the built-in sample.");
        eprintln!("usage: cargo run --release --example score -- \"<board URL>\"\n");
        report(SAMPLE_469);
    }
}

fn report(url: &str) {
    if let Some(score) = score_url(url) {
        println!("score: {score}");
        println!("(matched interior edges, rim excluded — the canonical convention)");
    } else {
        eprintln!("could not parse a board out of that URL.");
        eprintln!("expected an eternity2.dev / e2.bucas.name link with board_edges=…");
        std::process::exit(1);
    }
}
