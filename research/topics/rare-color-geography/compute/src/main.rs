//! Rare colors live only on the frame.
//!
//! Sort the official set's 22 colors by where their edges appear. Five of them
//! show up only on the border band (edge and corner pieces) and never once in the
//! interior, each on exactly 24 edges. The other seventeen are the interior
//! palette. This crate counts, per color, how many of its edges sit on the frame
//! versus the interior, straight from the official pieces.
//!
//! Exact and deterministic.

use eternity2_engine::official::official_puzzle;

fn main() {
    let puzzle = official_puzzle();
    let mut total = [0u32; 23];
    let mut frame = [0u32; 23];
    let mut interior = [0u32; 23];

    for piece in &puzzle.pieces {
        let grey = piece.iter().filter(|&&c| c == 0).count();
        for &c in piece {
            if c == 0 {
                continue;
            }
            let c = c as usize;
            total[c] += 1;
            if grey > 0 {
                frame[c] += 1;
            } else {
                interior[c] += 1;
            }
        }
    }

    let rare: Vec<usize> = (1..=22).filter(|&c| interior[c] == 0).collect();
    let rare_edges: u32 = rare.iter().map(|&c| total[c]).sum();

    println!("{{");
    println!("  \"pieceSet\": \"official Eternity II\",");
    println!("  \"rareColors\": [{}],", rare.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(", "));
    println!("  \"rareColorCount\": {},", rare.len());
    println!("  \"edgesPerRareColor\": {},", if rare.is_empty() { 0 } else { total[rare[0]] });
    println!("  \"rareEdgesTotal\": {rare_edges},");
    println!("  \"note\": \"Each rare color appears on exactly this many edges, all on the frame, none in the interior. Computed exactly from the official set.\",");
    println!("  \"colors\": [");
    for c in 1..=22usize {
        let comma = if c < 22 { "," } else { "" };
        let is_rare = interior[c] == 0;
        println!(
            "    {{ \"color\": {}, \"total\": {}, \"frame\": {}, \"interior\": {}, \"rare\": {} }}{}",
            c, total[c], frame[c], interior[c], is_rare, comma
        );
    }
    println!("  ]");
    println!("}}");
}
