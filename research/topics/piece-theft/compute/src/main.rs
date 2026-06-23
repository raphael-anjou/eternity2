//! Piece theft: why depth-first solvers die.
//!
//! When a solver fills the board top-left to bottom-right, each new cell already
//! knows the color it needs on its north side (from the piece above) and its west
//! side (from the piece to the left). So the cell needs an unused interior piece
//! that can show that exact (north, west) color pair in some rotation. This crate
//! counts, for every (north, west) demand that can occur, how many distinct
//! interior pieces could ever serve it. The demands are brutally scarce: the
//! typical one has only about three possible pieces, and many have just one. When
//! that one piece has already been used elsewhere, the cell is dead, even though
//! the board globally still has plenty of pieces left. That mismatch is "piece
//! theft", and it's the dominant way these searches fail.
//!
//! Exact and exhaustive over the official interior pieces, so it reproduces
//! byte-for-byte.

use eternity2_engine::official::official_puzzle;
use eternity2_engine::types::rotated;
use std::collections::{BTreeMap, BTreeSet};

const UP: usize = 0;
const LEFT: usize = 3;

fn main() {
    let puzzle = official_puzzle();
    let interior: Vec<[u8; 4]> = puzzle
        .pieces
        .iter()
        .filter(|e| !e.contains(&0))
        .copied()
        .collect();

    // (north, west) -> set of distinct interior pieces that can serve it.
    let mut servers: BTreeMap<(u8, u8), BTreeSet<usize>> = BTreeMap::new();
    for (pi, &e) in interior.iter().enumerate() {
        for r in 0..4u8 {
            let x = rotated(e, r);
            servers.entry((x[UP], x[LEFT])).or_default().insert(pi);
        }
    }

    let total_pairs = servers.len();
    let mut hist: BTreeMap<usize, usize> = BTreeMap::new();
    let mut sum = 0usize;
    let mut max_servers = 0usize;
    for set in servers.values() {
        let c = set.len();
        *hist.entry(c).or_default() += 1;
        sum += c;
        if c > max_servers {
            max_servers = c;
        }
    }
    let unique = *hist.get(&1).unwrap_or(&0);
    let mean = sum as f64 / total_pairs as f64;

    println!("{{");
    println!("  \"pieceSet\": \"official Eternity II\",");
    println!("  \"interiorPieceCount\": {},", interior.len());
    println!("  \"demand\": \"(north, west) color pair an interior cell needs once its top and left neighbours are placed\",");
    println!("  \"occurringDemands\": {total_pairs},");
    println!("  \"uniqueServerDemands\": {unique},");
    println!("  \"uniqueServerPct\": {:.1},", 100.0 * unique as f64 / total_pairs as f64);
    println!("  \"meanServers\": {mean:.2},");
    println!("  \"maxServers\": {max_servers},");
    println!("  \"histogram\": [");
    let entries: Vec<(usize, usize)> = hist.into_iter().collect();
    for (idx, (servers_n, pairs)) in entries.iter().enumerate() {
        let comma = if idx + 1 < entries.len() { "," } else { "" };
        println!("    {{ \"servers\": {servers_n}, \"demands\": {pairs} }}{comma}");
    }
    println!("  ]");
    println!("}}");
}
