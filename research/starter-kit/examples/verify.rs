//! Verify a board, and diff two boards — the checks to run before you claim
//! anything.
//!
//!   cargo run --release --example verify -- "<board URL>"
//!   cargo run --release --example verify -- "<URL A>" "<URL B>"
//!
//! One board: re-score it through the one canonical scorer, and check it against
//! the real official piece set — every cell is a genuine official piece, each
//! used once, and the five official clues sit where they must. Two boards: also
//! diff them by piece id per cell and report how many differ, and where.
//!
//! Why this exists: a record claim must be re-measured, never trusted. "I got
//! 469" means nothing until the board's own edges score 469 here on the real set,
//! and "same board as the 470" means nothing until a diff shows zero differing
//! cells. This is that check, in the reproducible kit, against the pieces the
//! kit now ships (`data/official.json`).

use e2_kit::{official_instance, parse_board_edges, score_cells};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [a] => verify_one(a),
        [a, b] => diff_two(a, b),
        _ => {
            eprintln!("usage: verify -- \"<URL>\"            (re-score one board)");
            eprintln!("       verify -- \"<URL A>\" \"<URL B>\"  (diff two boards)");
            std::process::exit(2);
        }
    }
}

/// Decode a board URL to per-cell URDL edge quads, or exit with a message.
fn cells_or_die(label: &str, url: &str) -> Vec<[u8; 4]> {
    let Some(cells) = parse_board_edges(url) else {
        eprintln!("{label}: could not parse board_edges out of that input");
        std::process::exit(1);
    };
    cells
}

fn verify_one(url: &str) {
    let cells = cells_or_die("board", url);
    let score = score_cells(&cells);
    let placed = cells.iter().filter(|c| **c != [0u8; 4]).count();
    let n = cells.len();
    let size = (n as f64).sqrt() as usize;
    // Max matched-edge score for a square board: 2·s² − 2s.
    let max = if size * size == n { (2 * size * size - 2 * size) as u32 } else { 0 };
    println!("cells:   {n} ({size}×{size}), {placed} placed");
    println!("score:   {score} / {max}   (canonical, rim excluded)");
    println!("breaks:  {}", max.saturating_sub(score));

    // Check against the real official set (only meaningful for a full 16×16).
    if n == 256 {
        let official = official_instance(true);
        let out = official.match_board(&cells);
        let matched = out.board.iter().filter(|&&c| c >= 0).count();
        let distinct: std::collections::HashSet<i32> =
            out.board.iter().filter(|&&c| c >= 0).map(|&c| c >> 2).collect();

        if matched == placed && distinct.len() == matched {
            // Every placed cell is a distinct official piece: this is a real E2
            // board, so the clue check is meaningful.
            println!("\nofficial set: OK — all {placed} placed cells are distinct official pieces");
            let clues_ok = official.hints.iter().all(|h| {
                let want = official.pieces.get(h.piece).unwrap().rotated(h.rot);
                cells.get(h.pos as usize) == Some(&want)
            });
            println!(
                "clue check:  {} — the five official clues {}",
                if clues_ok { "OK" } else { "VIOLATED" },
                if clues_ok { "are all in place" } else { "do NOT all match" }
            );
        } else {
            // Only some cells match: this is not the official puzzle (e.g. a
            // generated board), so an official-clue check would be meaningless.
            println!(
                "\nofficial set: this is NOT the official puzzle ({}/{placed} cells match official pieces)",
                distinct.len()
            );
        }
    }
    println!("\nre-scored from the board's own edges — this is the number to cite.");
}

fn diff_two(a: &str, b: &str) {
    let ca = cells_or_die("A", a);
    let cb = cells_or_die("B", b);
    if ca.len() != cb.len() {
        eprintln!("boards differ in size: A has {} cells, B has {}", ca.len(), cb.len());
        std::process::exit(1);
    }
    let n = ca.len();
    let size = (n as f64).sqrt() as usize;

    let sa = score_cells(&ca);
    let sb = score_cells(&cb);

    // A cell differs when its edge quads differ. Official pieces are distinct up
    // to rotation, so identical edges mean the same piece in the same rotation —
    // an edge diff is a piece-id diff. (For a 16×16 board you could resolve the
    // ids via official_instance().match_board; the edge comparison already gives
    // the same answer, so it is the honest, catalogue-free check.)
    let mut diffs: Vec<usize> = Vec::new();
    for pos in 0..n {
        if ca[pos] != cb[pos] {
            diffs.push(pos);
        }
    }

    println!("A score: {sa}");
    println!("B score: {sb}   (Δ {:+})", i64::from(sb) - i64::from(sa));
    println!("cells:   {n} ({size}×{size})");
    println!("differ:  {} of {n} cells", diffs.len());

    if diffs.is_empty() {
        println!("\n→ identical boards (every cell holds the same piece, same rotation).");
    } else {
        // Show the first handful as (row, col) so a human can spot-check.
        let show: Vec<String> = diffs
            .iter()
            .take(12)
            .map(|&p| format!("({},{})", p / size, p % size))
            .collect();
        let more = diffs.len().saturating_sub(show.len());
        print!("\n→ differing cells (row,col): {}", show.join(" "));
        if more > 0 {
            print!(" … +{more} more");
        }
        println!("\n→ NOT the same board.");
    }
}
