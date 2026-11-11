//! Null model for the demand counts: the same (north, west) server statistic on
//! randomly painted 16x16 boards with Eternity II's palette (5 frame colours +
//! 17 interior), built by the engine's framed generator. Deterministic.

use eternity2_engine::generator::generate_framed;
use eternity2_engine::official::official_puzzle;
use eternity2_engine::types::rotated;
use std::collections::{BTreeMap, BTreeSet};

const UP: usize = 0;
const LEFT: usize = 3;

fn stats(pieces: &[[u8; 4]]) -> (usize, usize, f64, usize) {
    let mut servers: BTreeMap<(u8, u8), BTreeSet<usize>> = BTreeMap::new();
    for (pi, &e) in pieces.iter().filter(|e| !e.contains(&0)).enumerate() {
        for r in 0..4u8 {
            let x = rotated(e, r);
            servers.entry((x[UP], x[LEFT])).or_default().insert(pi);
        }
    }
    let n = servers.len();
    let unique = servers.values().filter(|s| s.len() == 1).count();
    let sum: usize = servers.values().map(|s| s.len()).sum();
    let max = servers.values().map(|s| s.len()).max().unwrap_or(0);
    (n, unique, sum as f64 / n as f64, max)
}

fn main() {
    let seeds: u32 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(100);
    let (n, u, mean, max) = stats(&official_puzzle().pieces);
    println!("{{");
    println!("  \"null\": \"engine generate_framed(16, 22, seed, true): 5 frame colours, 17 interior colours, each joint painted uniformly at random\",");
    println!("  \"official\": {{ \"occurringDemands\": {n}, \"uniqueServerDemands\": {u}, \"meanServers\": {mean:.2}, \"maxServers\": {max} }},");
    println!("  \"random\": [");
    for s in 1..=seeds {
        let p = generate_framed(16, 22, s, true);
        let (n, u, mean, max) = stats(&p.pieces);
        let comma = if s < seeds { "," } else { "" };
        println!("    {{ \"seed\": {s}, \"occurringDemands\": {n}, \"uniqueServerDemands\": {u}, \"meanServers\": {mean:.2}, \"maxServers\": {max} }}{comma}");
    }
    println!("  ]");
    println!("}}");
}
