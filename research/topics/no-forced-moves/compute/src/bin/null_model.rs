//! Null model for the partner counts: the same statistic on randomly painted
//! 16x16 boards with Eternity II's palette (5 frame colours + 17 interior),
//! built by the engine's framed generator. Deterministic for the seed range.

use eternity2_engine::generator::generate_framed;
use eternity2_engine::official::official_puzzle;
use eternity2_engine::types::rotated;

type Edges = [u8; 4];
const RIGHT: usize = 1;
const LEFT: usize = 3;

fn stats(pieces: &[[u8; 4]]) -> (u32, u32, u32, f64, usize) {
    let interior: Vec<[Edges; 4]> = pieces
        .iter()
        .filter(|e| !e.contains(&0))
        .map(|&e| [rotated(e, 0), rotated(e, 1), rotated(e, 2), rotated(e, 3)])
        .collect();
    let m = interior.len();
    let mut counts: Vec<u32> = (0..m)
        .map(|i| {
            (0..m)
                .filter(|&j| {
                    j != i
                        && interior[i]
                            .iter()
                            .any(|ra| interior[j].iter().any(|rb| ra[RIGHT] == rb[LEFT]))
                })
                .count() as u32
        })
        .collect();
    counts.sort_unstable();
    let mean = counts.iter().map(|&c| c as f64).sum::<f64>() / m as f64;
    let forced = counts.iter().filter(|&&c| c <= 1).count();
    (counts[0], counts[m / 2], counts[m - 1], mean, forced)
}

fn main() {
    let seeds: u32 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(100);
    let (mn, med, mx, mean, forced) = stats(&official_puzzle().pieces);
    println!("{{");
    println!("  \"null\": \"engine generate_framed(16, 22, seed, true): 5 frame colours, 17 interior colours, each joint painted uniformly at random\",");
    println!("  \"official\": {{ \"min\": {mn}, \"median\": {med}, \"max\": {mx}, \"mean\": {mean:.1}, \"forced\": {forced} }},");
    println!("  \"random\": [");
    for s in 1..=seeds {
        let p = generate_framed(16, 22, s, true);
        let (mn, med, mx, mean, forced) = stats(&p.pieces);
        let comma = if s < seeds { "," } else { "" };
        println!("    {{ \"seed\": {s}, \"min\": {mn}, \"median\": {med}, \"max\": {mx}, \"mean\": {mean:.1}, \"forced\": {forced} }}{comma}");
    }
    println!("  ]");
    println!("}}");
}
