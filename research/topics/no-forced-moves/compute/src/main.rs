//! No forced moves: every interior piece has many possible neighbours.
//!
//! A common way to crack a constraint puzzle is to find a cell where only one
//! piece can possibly go, place it, and repeat. This crate shows that lever does
//! not exist for Eternity II's interior: for every one of the 196 interior
//! pieces we count how many other interior pieces could sit immediately to its
//! right (its right edge matching their left edge, under some rotation of each).
//! Not a single piece is ever down to one option.
//!
//! Exact and exhaustive over all ordered interior pairs, so it reproduces
//! byte-for-byte.

use eternity2_engine::official::official_puzzle;
use eternity2_engine::types::rotated;

type Edges = [u8; 4];
const RIGHT: usize = 1;
const LEFT: usize = 3;

/// Can piece b sit to the right of piece a, for some rotation of each?
fn can_follow(a: &[Edges; 4], b: &[Edges; 4]) -> bool {
    for ra in a {
        for rb in b {
            if ra[RIGHT] == rb[LEFT] {
                return true;
            }
        }
    }
    false
}

fn main() {
    let puzzle = official_puzzle();
    let interior: Vec<[Edges; 4]> = puzzle
        .pieces
        .iter()
        .filter(|e| !e.contains(&0))
        .map(|&e| [rotated(e, 0), rotated(e, 1), rotated(e, 2), rotated(e, 3)])
        .collect();
    let m = interior.len();

    // Partner count per piece, and a histogram for the chart.
    let mut counts: Vec<u32> = Vec::with_capacity(m);
    for i in 0..m {
        let mut c = 0u32;
        for j in 0..m {
            if i == j {
                continue;
            }
            if can_follow(&interior[i], &interior[j]) {
                c += 1;
            }
        }
        counts.push(c);
    }
    counts.sort_unstable();

    let min = counts[0];
    let max = counts[m - 1];
    let median = counts[m / 2];
    let mean = counts.iter().map(|&c| c as f64).sum::<f64>() / m as f64;
    let forced = counts.iter().filter(|&&c| c <= 1).count();

    // Histogram in buckets of 10.
    let bucket = |c: u32| (c / 10) * 10;
    let mut hist: std::collections::BTreeMap<u32, u32> = std::collections::BTreeMap::new();
    for &c in &counts {
        *hist.entry(bucket(c)).or_insert(0) += 1;
    }

    println!("{{");
    println!("  \"pieceSet\": \"official Eternity II\",");
    println!("  \"interiorPieceCount\": {m},");
    println!("  \"metric\": \"number of distinct interior pieces that can sit immediately to the right of each piece, under some rotation\",");
    println!("  \"minPartners\": {min},");
    println!("  \"maxPartners\": {max},");
    println!("  \"medianPartners\": {median},");
    println!("  \"meanPartners\": {mean:.1},");
    println!("  \"forcedPieces\": {forced},");
    println!("  \"histogram\": [");
    let entries: Vec<(u32, u32)> = hist.into_iter().collect();
    for (idx, (lo, n)) in entries.iter().enumerate() {
        let comma = if idx + 1 < entries.len() { "," } else { "" };
        println!("    {{ \"bucket\": \"{}-{}\", \"count\": {} }}{}", lo, lo + 9, n, comma);
    }
    println!("  ]");
    println!("}}");
}
