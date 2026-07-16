//! The bucas viewer URL: four URDL letters per cell, `'a' + color`. Identical
//! rendering to the benchmark's `cells_to_bucas_url`, so a study board opens in
//! `/viewer` exactly as a benchmark board does.

fn color_to_letter(c: u8) -> char {
    // The site clamps out-of-range colors to 'a'; mirror that so URLs match.
    if c as usize > 22 {
        'a'
    } else {
        (b'a' + c) as char
    }
}

/// Render a grid of per-cell URDL edge colors as a bucas viewer URL.
#[must_use]
pub fn board_to_bucas_url(name: &str, width: u8, height: u8, cells: &[[u8; 4]]) -> String {
    let mut edges = String::with_capacity(cells.len() * 4);
    for c in cells {
        for &color in c {
            edges.push(color_to_letter(color));
        }
    }
    format!(
        "https://e2.bucas.name/#puzzle={name}&board_w={width}&board_h={height}&board_edges={edges}"
    )
}
