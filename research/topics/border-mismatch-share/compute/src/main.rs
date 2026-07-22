//! Border-balance recount: where do the unmatched edges of a near-solution sit?
//!
//! The border-balance invariant (NS-1) inspects only the seam between the outer
//! ring of border pieces and the first ring of interior pieces. A near-solution's
//! remaining errors may or may not land on that seam. This crate takes every
//! bundled high board, finds each of its unmatched internal edges, and sorts them
//! into three exhaustive classes by which two cells the edge separates:
//!
//!   * interior-interior — both cells strictly inside the 14x14 core (NS-1 blind)
//!   * interior-border   — the seam NS-1 actually inspects
//!   * border-border     — between two adjacent pieces on the outer rim
//!
//! Boards are read straight from the site's own committed data files
//! (web/src/data/known-boards.ts and record-boards.ts) as their board_edges
//! strings: four lowercase letters per cell in URDL order (up, right, down, left),
//! row-major, 'a' being the grey rim. Exact and deterministic; reproduces
//! byte-for-byte.

use eternity2_engine::official::official_puzzle;

const KNOWN: &str = include_str!("../../../../../web/src/data/known-boards.ts");
const RECORD: &str = include_str!("../../../../../web/src/data/record-boards.ts");

const W: usize = 16;
const H: usize = 16;

/// A cell (r,c) is on the border ring iff it is on the outer perimeter.
fn is_border(r: usize, c: usize) -> bool {
    r == 0 || r == H - 1 || c == 0 || c == W - 1
}

/// The three exhaustive classes an unmatched edge can fall into.
#[derive(Default, Clone, Copy)]
struct Split {
    interior_interior: u32,
    interior_border: u32,
    border_border: u32,
}

impl Split {
    fn total(&self) -> u32 {
        self.interior_interior + self.interior_border + self.border_border
    }
    fn add(&mut self, other: &Split) {
        self.interior_interior += other.interior_interior;
        self.interior_border += other.interior_border;
        self.border_border += other.border_border;
    }
}

/// Matched-edge score and the mismatch split, in one pass over the board.
/// `edges` is the flat URDL letter string; U=0, R=1, D=2, L=3.
fn analyse(edges: &str) -> (u32, Split) {
    let b = edges.as_bytes();
    let cell = |i: usize| -> [u8; 4] { [b[i * 4], b[i * 4 + 1], b[i * 4 + 2], b[i * 4 + 3]] };
    let (up, right, down, left) = (0usize, 1usize, 2usize, 3usize);
    let mut matched = 0u32;
    let mut split = Split::default();

    let bump = |ra: usize, ca: usize, rb: usize, cb: usize, s: &mut Split| {
        let a = is_border(ra, ca);
        let d = is_border(rb, cb);
        if a && d {
            s.border_border += 1;
        } else if a || d {
            s.interior_border += 1;
        } else {
            s.interior_interior += 1;
        }
    };

    for r in 0..H {
        for c in 0..W {
            let i = r * W + c;
            if c + 1 < W {
                if cell(i)[right] == cell(i + 1)[left] {
                    matched += 1;
                } else {
                    bump(r, c, r, c + 1, &mut split);
                }
            }
            if r + 1 < H {
                if cell(i)[down] == cell(i + W)[up] {
                    matched += 1;
                } else {
                    bump(r, c, r + 1, c, &mut split);
                }
            }
        }
    }
    (matched, split)
}

/// Pull the board_edges letter run out of a `board_edges=...` parameter string.
fn board_edges(after: &str) -> Option<String> {
    let key = "board_edges=";
    let start = after.find(key)? + key.len();
    let run: String = after[start..]
        .chars()
        .take_while(|ch| ch.is_ascii_lowercase())
        .collect();
    if run.len() == W * H * 4 {
        Some(run)
    } else {
        None
    }
}

/// Extract the string value of a `"<key>": "<value>"` or `<key>: "<value>"` field
/// found at or after `from`, tolerant of both JSON-ish and TS object literals.
fn string_field(s: &str, key: &str, from: usize) -> Option<(String, usize)> {
    // Try quoted key first, then bare key.
    for needle in [format!("\"{key}\""), format!("{key}:")] {
        if let Some(rel) = s[from..].find(&needle) {
            let k = from + rel;
            let colon = s[k..].find(':')? + k;
            let q1 = s[colon..].find('"')? + colon + 1;
            let q2 = s[q1..].find('"')? + q1;
            return Some((s[q1..q2].to_string(), q2));
        }
    }
    None
}

/// Extract an integer `score` field, or None if the value is `null`.
fn score_field(s: &str, from: usize) -> (Option<i64>, usize) {
    for needle in ["\"score\"", "score:"] {
        if let Some(rel) = s[from..].find(needle) {
            let k = from + rel;
            if let Some(cp) = s[k..].find(':') {
                let colon = k + cp;
                let rest = &s[colon + 1..];
                let end = rest.find([',', '}', '\n']).unwrap_or(rest.len());
                let tok = rest[..end].trim();
                let val = tok.parse::<i64>().ok();
                return (val, colon + 1 + end);
            }
        }
    }
    (None, from)
}

struct Board {
    id: String,
    score: Option<i64>,
    edges: String,
}

/// Walk a TS/JSON source, pulling every (id, score, edges) triple in order.
/// The three fields appear id -> score -> board_edges within each entry.
fn boards_from(src: &str) -> Vec<Board> {
    let mut out = Vec::new();
    let mut pos = 0usize;
    while let Some((id, p1)) = string_field(src, "id", pos) {
        let (score, p2) = score_field(src, p1);
        // board_edges lives inside the params string that follows.
        let edges = match board_edges(&src[p2..]) {
            Some(e) => e,
            None => {
                pos = p1;
                continue;
            }
        };
        // advance past this entry's edge run
        let key = "board_edges=";
        let estart = src[p2..].find(key).map(|x| p2 + x + key.len()).unwrap_or(p2);
        out.push(Board { id, score, edges });
        pos = estart + W * H * 4;
    }
    out
}

fn main() {
    let max = official_puzzle().max_score();

    // Board id -> score-class label. A board is only counted in an aggregate if
    // its verified score matches a real full-board score. The `Clues` board has
    // no score, and the linear-run partial board is excluded from aggregates.
    let mut boards = boards_from(KNOWN);
    boards.extend(boards_from(RECORD));

    // Per-board rows, sorted by verified score descending then id.
    struct Row {
        id: String,
        claimed: Option<i64>,
        verified: u32,
        split: Split,
    }
    let mut rows: Vec<Row> = boards
        .iter()
        .map(|b| {
            let (verified, split) = analyse(&b.edges);
            Row {
                id: b.id.clone(),
                claimed: b.score,
                verified,
                split,
            }
        })
        .collect();
    rows.sort_by(|a, b| b.verified.cmp(&a.verified).then(a.id.cmp(&b.id)));

    // The 469-class: every board whose claimed AND verified full-board score is 469.
    let is_469 = |r: &Row| r.claimed == Some(469) && r.verified == 469;
    let mut agg_469 = Split::default();
    let mut n_469 = 0u32;
    for r in rows.iter().filter(|r| is_469(r)) {
        agg_469.add(&r.split);
        n_469 += 1;
    }

    // The high community set: every distinct board whose claimed score equals its
    // verified score and is >= 464 (the community five-clue record and above).
    // This excludes the null-score Clues board and any board whose `score` field
    // measures something other than the full-board matched-edge count.
    let in_high = |r: &Row| matches!(r.claimed, Some(s) if s >= 464) && r.verified as i64 == r.claimed.unwrap();
    let mut agg_high = Split::default();
    let mut n_high = 0u32;
    for r in rows.iter().filter(|r| in_high(r)) {
        agg_high.add(&r.split);
        n_high += 1;
    }

    let pct = |x: u32, tot: u32| if tot == 0 { 0.0 } else { 100.0 * x as f64 / tot as f64 };

    println!("{{");
    println!("  \"maxScore\": {max},");
    println!("  \"boardSize\": \"16x16\",");
    println!("  \"note\": \"Unmatched internal edges of each bundled high board, split by which two cells the edge separates. URDL edges parsed from web/src/data/known-boards.ts and record-boards.ts. interior_border is the seam the border-balance invariant (NS-1) inspects; interior_interior and border_border are invisible to it.\",");

    // ---- 469-class aggregate ----
    let t469 = agg_469.total();
    println!("  \"class469\": {{");
    println!("    \"boardCount\": {n_469},");
    println!("    \"unmatchedEdges\": {t469},");
    println!("    \"interiorInterior\": {},", agg_469.interior_interior);
    println!("    \"interiorBorder\": {},", agg_469.interior_border);
    println!("    \"borderBorder\": {},", agg_469.border_border);
    println!(
        "    \"interiorInteriorSharePct\": {:.1},",
        pct(agg_469.interior_interior, t469)
    );
    println!(
        "    \"interiorBorderSharePct\": {:.1},",
        pct(agg_469.interior_border, t469)
    );
    println!(
        "    \"borderBorderSharePct\": {:.1}",
        pct(agg_469.border_border, t469)
    );
    println!("  }},");

    // ---- broader high-set aggregate ----
    let thigh = agg_high.total();
    println!("  \"highSet\": {{");
    println!("    \"scoreFloor\": 464,");
    println!("    \"boardCount\": {n_high},");
    println!("    \"unmatchedEdges\": {thigh},");
    println!("    \"interiorInterior\": {},", agg_high.interior_interior);
    println!("    \"interiorBorder\": {},", agg_high.interior_border);
    println!("    \"borderBorder\": {},", agg_high.border_border);
    println!(
        "    \"interiorInteriorSharePct\": {:.1},",
        pct(agg_high.interior_interior, thigh)
    );
    println!(
        "    \"interiorBorderSharePct\": {:.1},",
        pct(agg_high.interior_border, thigh)
    );
    println!(
        "    \"borderBorderSharePct\": {:.1}",
        pct(agg_high.border_border, thigh)
    );
    println!("  }},");

    // ---- per-board rows ----
    println!("  \"boards\": [");
    let n = rows.len();
    for (idx, r) in rows.iter().enumerate() {
        let comma = if idx + 1 < n { "," } else { "" };
        let claimed = match r.claimed {
            Some(s) => s.to_string(),
            None => "null".to_string(),
        };
        println!("    {{");
        println!("      \"id\": \"{}\",", r.id);
        println!("      \"claimedScore\": {claimed},");
        println!("      \"verifiedScore\": {},", r.verified);
        println!("      \"unmatchedEdges\": {},", r.split.total());
        println!("      \"interiorInterior\": {},", r.split.interior_interior);
        println!("      \"interiorBorder\": {},", r.split.interior_border);
        println!("      \"borderBorder\": {}", r.split.border_border);
        println!("    }}{comma}");
    }
    println!("  ]");
    println!("}}");
}
