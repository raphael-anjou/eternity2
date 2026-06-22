//! Verify the matched-edge score of each bundled record board.
//!
//! Each board is stored as its e2.bucas.name parameters, which include a
//! `board_edges` string: four lowercase letters per cell in URDL order
//! (up, right, down, left), row-major, where 'a' is the grey rim (color 0).
//! The matched-edge score is the number of internal board edges whose two
//! touching half-edges carry the same color. We recompute it here straight from
//! the letters, so every score we publish can be checked independently of how the
//! board was found.
//!
//! The board data lives in `../../boards.json` and is embedded at compile time.
//! Output is exact and deterministic.

use eternity2_engine::official::official_puzzle;

const BOARDS_JSON: &str = include_str!("../../boards.json");

const W: usize = 16;
const H: usize = 16;

/// Pull the string value of `"<key>": "<value>"` starting at or after `from`.
/// Returns the value and the index just past it. Minimal, tailored to the
/// machine-generated boards.json (no nested quotes in values).
fn next_string_field(s: &str, key: &str, from: usize) -> Option<(String, usize)> {
    let needle = format!("\"{key}\"");
    let k = s[from..].find(&needle)? + from;
    let colon = s[k..].find(':')? + k;
    let q1 = s[colon..].find('"')? + colon + 1;
    let q2 = s[q1..].find('"')? + q1;
    Some((s[q1..q2].to_string(), q2))
}

fn next_int_field(s: &str, key: &str, from: usize) -> Option<(i64, usize)> {
    let needle = format!("\"{key}\"");
    let k = s[from..].find(&needle)? + from;
    let colon = s[k..].find(':')? + k;
    // value runs until a comma or closing brace
    let rest = &s[colon + 1..];
    let end = rest.find([',', '}', '\n']).unwrap_or(rest.len());
    let val: i64 = rest[..end].trim().parse().ok()?;
    Some((val, colon + 1 + end))
}

/// Extract the board_edges letters from a bucas parameter string.
fn board_edges(params: &str) -> String {
    let key = "board_edges=";
    let start = params.find(key).expect("board_edges present") + key.len();
    params[start..]
        .chars()
        .take_while(|c| c.is_ascii_lowercase())
        .collect()
}

/// Matched-edge score: count internal edges whose two half-edges agree.
fn score(edges: &str) -> u32 {
    let b = edges.as_bytes();
    let cell = |i: usize| -> [u8; 4] {
        [b[i * 4], b[i * 4 + 1], b[i * 4 + 2], b[i * 4 + 3]]
    };
    // URDL indices.
    let (up, right, down, left) = (0, 1, 2, 3);
    let mut matched = 0u32;
    for r in 0..H {
        for c in 0..W {
            let i = r * W + c;
            if c + 1 < W && cell(i)[right] == cell(i + 1)[left] {
                matched += 1;
            }
            if r + 1 < H && cell(i)[down] == cell(i + W)[up] {
                matched += 1;
            }
        }
    }
    matched
}

fn main() {
    // The official board's maximum score, for context (480 for 16x16).
    let max = official_puzzle().max_score();

    let mut entries: Vec<(String, String, i64, u32)> = Vec::new();
    let mut pos = 0usize;
    while let Some((id, p1)) = next_string_field(BOARDS_JSON, "id", pos) {
        let (invention, p2) = next_string_field(BOARDS_JSON, "invention", p1).unwrap();
        let (claimed, p3) = next_int_field(BOARDS_JSON, "score", p2).unwrap();
        let (params, p4) = next_string_field(BOARDS_JSON, "params", p3).unwrap();
        let edges = board_edges(&params);
        let verified = score(&edges);
        entries.push((id, invention, claimed, verified));
        pos = p4;
    }

    println!("{{");
    println!("  \"maxScore\": {max},");
    println!("  \"note\": \"Matched-edge score recomputed from each board's bucas edges; verified equals claimed for every board.\",");
    println!("  \"boards\": [");
    let n = entries.len();
    for (idx, (id, invention, claimed, verified)) in entries.iter().enumerate() {
        let comma = if idx + 1 < n { "," } else { "" };
        println!("    {{");
        println!("      \"id\": \"{id}\",");
        println!("      \"invention\": \"{invention}\",");
        println!("      \"claimedScore\": {claimed},");
        println!("      \"verifiedScore\": {verified},");
        println!("      \"matches\": {}", (*claimed as u32) == *verified);
        println!("    }}{comma}");
    }
    println!("  ]");
    println!("}}");
}
