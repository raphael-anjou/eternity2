//! The color split of the official Eternity II set.
//!
//! The published difficulty argument says the puzzle's color counts were chosen
//! to sit at the hardness peak of edge-matching puzzles: around 17 interior
//! colors. This crate shows that split is really there in the pieces, by reading
//! the official set straight from the shared engine and sorting its colors by
//! where they appear.
//!
//! A piece's edges are colored 0..=22, where 0 is the grey rim. A piece is a
//! corner (two grey edges), an edge piece (one grey edge), or an interior piece
//! (no grey edge). We record, for every non-grey color, whether it ever shows up
//! on an interior piece and whether it ever shows up on the frame (an edge or
//! corner piece). The output is exact and deterministic.

use eternity2_engine::official::official_puzzle;
use std::collections::BTreeSet;

fn main() {
    let puzzle = official_puzzle();

    let mut corners = 0u32;
    let mut edges = 0u32;
    let mut interiors = 0u32;
    let mut interior_colors: BTreeSet<u8> = BTreeSet::new();
    let mut frame_colors: BTreeSet<u8> = BTreeSet::new();

    for piece in &puzzle.pieces {
        let grey = piece.iter().filter(|&&c| c == 0).count();
        match grey {
            2 => corners += 1,
            1 => edges += 1,
            _ => interiors += 1,
        }
        for &c in piece {
            if c == 0 {
                continue;
            }
            if grey == 0 {
                interior_colors.insert(c);
            } else {
                frame_colors.insert(c);
            }
        }
    }

    let frame_only: Vec<u8> = frame_colors.difference(&interior_colors).copied().collect();
    let interior_only: Vec<u8> = interior_colors.difference(&frame_colors).copied().collect();
    let shared: Vec<u8> = interior_colors.intersection(&frame_colors).copied().collect();

    let list = |v: &[u8]| {
        v.iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    };

    println!("{{");
    println!("  \"pieceSet\": \"official Eternity II\",");
    println!("  \"width\": {},", puzzle.width);
    println!("  \"height\": {},", puzzle.height);
    println!("  \"totalColors\": {},", puzzle.num_colors);
    println!("  \"pieces\": {{");
    println!("    \"corner\": {},", corners);
    println!("    \"edge\": {},", edges);
    println!("    \"interior\": {}", interiors);
    println!("  }},");
    println!("  \"interiorColorCount\": {},", interior_colors.len());
    println!("  \"frameOnlyColorCount\": {},", frame_only.len());
    println!("  \"frameOnlyColors\": [{}],", list(&frame_only));
    println!("  \"interiorColors\": [{}],", list(&interior_colors.iter().copied().collect::<Vec<_>>()));
    println!("  \"sharedColorCount\": {},", shared.len());
    println!("  \"interiorOnlyColorCount\": {},", interior_only.len());
    println!("  \"note\": \"Colors that appear only on the frame are the rare border colors; the interior palette is the colors that appear on interior pieces. Computed exactly from the official set.\"");
    println!("}}");
}
