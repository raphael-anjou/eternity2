//! No-height-function checker.
//!
//! The vol-228 PHYS-C paper asks whether Eternity II admits a dimer-style
//! *height function*, in which color-matching would be a smoothness condition
//! and mismatched joints (breaks) would carry a conserved oriented charge
//! annihilable in pairs. It gives two structural obstructions, both of which
//! this checker re-measures directly on committed high-scoring boards:
//!
//!   Theorem 2 — the Z/2 dual break-chain is not closed: the break set has
//!     dual vertices of ODD break-degree (dislocation "cores", i.e. open-string
//!     endpoints). A height would need every dual vertex even. Paper measured
//!     28 cores on its 451 board, 22 on 463, 32 on 458.
//!
//!   Theorem 3 — the signed per-color current is not globally conserved: for a
//!     color c, orient each c-half joint (a break where exactly one side is c)
//!     from the c-carrying cell toward its non-c neighbor, and sum the per-cell
//!     vectors. A genuine conserved current sums to zero. Paper measured many
//!     nonzero sums on its 451 board, e.g. color 6 -> (-1,-3).
//!
//! Our boards differ from the paper's, so exact core counts and per-color
//! sums differ; what reproduces is the QUALITATIVE LAW — cores exist and are
//! numerous, and at least one per-color current has a nonzero global sum — and
//! that law is exactly what forbids a height / Burgers-vector structure.
//! This is a qualitative-tier reproduction; the specific integers below are
//! properties of OUR boards, never presented as the paper's numbers.
//!
//! Emits one JSON object on stdout. Boards are re-scored in-checker, so the
//! measurement is independent of how each board was found.

use e2_kit::{parse_board_edges, score_cells};
use serde_json::json;
use std::process::ExitCode;

const BOARDS_JSON: &str = include_str!("../../boards.json");

const W: usize = 16;
const H: usize = 16;

/// Pull the string value of the next `"<key>": "<value>"` at or after `from`.
/// Tailored to the machine-written boards.json (no nested quotes in values).
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
    let rest = &s[colon + 1..];
    let end = rest.find([',', '}', '\n']).unwrap_or(rest.len());
    let val: i64 = rest[..end].trim().parse().ok()?;
    Some((val, colon + 1 + end))
}

fn fail(msg: &str) -> ExitCode {
    eprintln!("NO-HEIGHT-FUNCTION CHECK FAILED: {msg}");
    ExitCode::FAILURE
}

/// A cell's edge colors, indexed N=0, E=1, S=2, W=3 (the kit's URDL order).
type Cell = [u8; 4];
const N: usize = 0;
const E: usize = 1;
const S: usize = 2;
const WW: usize = 3;

fn idx(r: usize, c: usize) -> usize {
    r * W + c
}

struct BoardMeasure {
    id: String,
    claimed: i64,
    verified: u32,
    breaks: u32,
    /// Dual vertices (grid corners, 17x17) of odd break-degree — the cores.
    odd_cores: u32,
    /// Colors whose signed current has a nonzero global sum.
    nonzero_current_colors: u32,
    /// A few example (color, (x, y)) nonzero sums, for the article.
    sample_nonzero: Vec<(u8, i64, i64)>,
}

fn measure(id: &str, claimed: i64, cells: &[Cell]) -> BoardMeasure {
    let verified = score_cells(cells);

    // A joint is broken iff its two facing half-edges differ. On these boards
    // internal joints never carry the gray rim color, so "differ" == "break",
    // matching the kit scorer's non-border-match convention.
    let mut breaks = 0u32;

    // Theorem 2: dual break-chain. Dual vertices sit at the (H+1)x(W+1) grid
    // corners. An H-joint between (r,c)|(r,c+1) pierces the dual edge joining
    // corners (r,c+1) and (r+1,c+1); a V-joint between (r,c)|(r+1,c) joins
    // corners (r+1,c) and (r+1,c+1). Track break-degree per dual vertex.
    let corner = |r: usize, c: usize| r * (W + 1) + c;
    let mut dual_deg = vec![0u32; (H + 1) * (W + 1)];

    // Theorem 3: signed per-color current. For a color k, orient each k-half
    // joint (a break with exactly one side k) from the k-carrying cell toward
    // its non-k neighbor. Accumulate a per-COLOR global sum (x, y): +x when a
    // cell shows k on its East half of a break, -x on its West half; +y for
    // South, -y for North.
    let ncolors = 256usize;
    let mut cur_x = vec![0i64; ncolors];
    let mut cur_y = vec![0i64; ncolors];

    for r in 0..H {
        for c in 0..W {
            let cell = cells[idx(r, c)];
            // East joint of this cell.
            if c + 1 < W {
                let right = cells[idx(r, c + 1)];
                let a = cell[E];
                let b = right[WW];
                if a != b {
                    breaks += 1;
                    dual_deg[corner(r, c + 1)] += 1;
                    dual_deg[corner(r + 1, c + 1)] += 1;
                    // a is on this cell's East; b is on the right cell's West.
                    if a != 0 {
                        cur_x[a as usize] += 1;
                    }
                    if b != 0 {
                        cur_x[b as usize] -= 1;
                    }
                }
            }
            // South joint of this cell.
            if r + 1 < H {
                let below = cells[idx(r + 1, c)];
                let a = cell[S];
                let b = below[N];
                if a != b {
                    breaks += 1;
                    dual_deg[corner(r + 1, c)] += 1;
                    dual_deg[corner(r + 1, c + 1)] += 1;
                    if a != 0 {
                        cur_y[a as usize] += 1;
                    }
                    if b != 0 {
                        cur_y[b as usize] -= 1;
                    }
                }
            }
        }
    }

    let odd_cores = dual_deg.iter().filter(|&&d| d % 2 == 1).count() as u32;

    let mut nonzero_current_colors = 0u32;
    let mut sample_nonzero = Vec::new();
    for k in 1..ncolors {
        if cur_x[k] != 0 || cur_y[k] != 0 {
            nonzero_current_colors += 1;
            if sample_nonzero.len() < 6 {
                sample_nonzero.push((k as u8, cur_x[k], cur_y[k]));
            }
        }
    }

    BoardMeasure {
        id: id.to_string(),
        claimed,
        verified,
        breaks,
        odd_cores,
        nonzero_current_colors,
        sample_nonzero,
    }
}

fn main() -> ExitCode {
    let mut boards = Vec::new();
    let mut pos = 0usize;
    while let Some((id, p1)) = next_string_field(BOARDS_JSON, "id", pos) {
        let (score, p2) = match next_int_field(BOARDS_JSON, "score", p1) {
            Some(v) => v,
            None => break,
        };
        let (params, p3) = next_string_field(BOARDS_JSON, "params", p2).unwrap();
        pos = p3;
        let cells = match parse_board_edges(&params) {
            Some(c) if c.len() == W * H => c,
            _ => return fail(&format!("board {id}: unparseable or wrong-size board_edges")),
        };
        let m = measure(&id, score, &cells);
        if m.verified as i64 != m.claimed {
            return fail(&format!(
                "board {id}: re-scored {} != claimed {}",
                m.verified, m.claimed
            ));
        }
        boards.push(m);
    }

    if boards.is_empty() {
        return fail("no boards parsed from boards.json");
    }

    // The qualitative law: on every board, cores exist and are numerous, and
    // at least one per-color current has a nonzero global sum. Either failing
    // would refute the obstruction and admit a height function.
    let law_holds = boards
        .iter()
        .all(|b| b.odd_cores > 0 && b.nonzero_current_colors > 0);
    if !law_holds {
        return fail("a board has zero cores or a fully conserved current — height function not obstructed");
    }

    let board_json: Vec<_> = boards
        .iter()
        .map(|b| {
            json!({
                "id": b.id,
                "claimed_score": b.claimed,
                "verified_score": b.verified,
                "breaks": b.breaks,
                "odd_dual_cores": b.odd_cores,
                "nonzero_current_colors": b.nonzero_current_colors,
                "sample_nonzero_currents": b.sample_nonzero
                    .iter()
                    .map(|&(c, x, y)| json!({"color": c, "sum": [x, y]}))
                    .collect::<Vec<_>>(),
            })
        })
        .collect();

    let min_cores = boards.iter().map(|b| b.odd_cores).min().unwrap();
    let max_cores = boards.iter().map(|b| b.odd_cores).max().unwrap();

    let out = json!({
        "topic": "no-height-function",
        "tier": "qualitative",
        "instance": "official 16x16, measured on committed high-scoring boards",
        "law": {
            "statement": "Every board has odd-degree dual cores (breaks are open strings, not dual cycles) AND at least one per-color signed current with nonzero global sum, so no height / Burgers-vector function exists.",
            "holds_on_all_boards": law_holds,
            "min_odd_cores_across_boards": min_cores,
            "max_odd_cores_across_boards": max_cores,
        },
        "boards": board_json,
        "paper_reference_numbers": {
            "note": "PHYS-C measured these on ITS boards (different from ours); listed only to show our numbers land in the same regime, NOT as our results.",
            "odd_cores_451_463_458": [28, 22, 32],
            "signed_current_example_451": {"color": 6, "sum": [-1, -3]},
        },
    });
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
    ExitCode::SUCCESS
}
