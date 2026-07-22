//! Break-geometry checker for the design-recipe-464 topic.
//!
//! Reads `boards.txt` (label<TAB>viewer-URL per line), rescores every board
//! with the canonical rim-excluding scorer, recovers each placement against the
//! official piece set, verifies the five official clues, and computes the
//! numbers the claim rests on:
//!
//! * per-cell break attribution (each mismatched interior adjacency charged to
//!   the row-major-later cell, i.e. every cell counts its UP and LEFT
//!   mismatches) — the same rule as the research repo's `verify_bucas`;
//! * per-row break totals and the break band;
//! * double-break cells and exact break cell indices;
//! * the pairwise tile-Hamming matrix (piece id differs) — the basin map.
//!
//! Output: one JSON document on stdout. Runtime: well under a second.

use e2_kit::{official_instance, parse_board_edges, score_cells, Instance, MAX_SCORE_16};

const W: usize = 16;
const H: usize = 16;
/// Border colour: `'a'` decodes to 0 in the URL edge blob.
const BORDER: u8 = 0;

#[derive(serde::Serialize)]
struct BoardReport {
    label: String,
    url: String,
    score: u32,
    max_score: u32,
    breaks: u32,
    break_identity_holds: bool,
    placed: usize,
    hints_matched: usize,
    hints_total: usize,
    per_row_breaks: Vec<u32>,
    break_rows: Vec<usize>,
    double_break_cells: Vec<usize>,
    break_cells: Vec<usize>,
    break_grid: Vec<String>,
}

#[derive(serde::Serialize)]
struct Output {
    boards: Vec<BoardReport>,
    tile_hamming: Vec<Vec<u32>>,
}

/// Per-cell break counts under the row-major-later attribution rule.
fn break_counts(cells: &[[u8; 4]]) -> Vec<u32> {
    let mut counts = vec![0u32; W * H];
    for y in 0..H {
        for x in 0..W {
            let p = y * W + x;
            let e = cells[p];
            if y > 0 {
                let up = cells[p - W];
                let matched = e[0] == up[2] && e[0] != BORDER;
                let both_border = e[0] == BORDER && up[2] == BORDER;
                if !matched && !both_border {
                    counts[p] += 1;
                }
            }
            if x > 0 {
                let left = cells[p - 1];
                let matched = e[3] == left[1] && e[3] != BORDER;
                let both_border = e[3] == BORDER && left[1] == BORDER;
                if !matched && !both_border {
                    counts[p] += 1;
                }
            }
        }
    }
    counts
}

fn report(instance: &Instance, label: &str, url: &str) -> (BoardReport, Vec<i32>) {
    let cells = parse_board_edges(url)
        .unwrap_or_else(|| panic!("{label}: no parseable board_edges in URL"));
    assert_eq!(cells.len(), W * H, "{label}: expected {} cells", W * H);

    let score = score_cells(&cells);
    // Placement recovery: the official set is distinct up to rotation, so the
    // edge grid determines piece ids and rotations. `match_board` re-scores
    // through the same canonical scorer; the two scores must agree.
    let out = instance.match_board(&cells);
    assert_eq!(out.score, score, "{label}: scorer disagreement");
    let placed = out.board.iter().filter(|&&c| c >= 0).count();

    let hints_matched = instance
        .hints
        .iter()
        .filter(|h| {
            let code = i32::from(h.piece) * 4 + i32::from(h.rot);
            out.board.get(usize::from(h.pos)) == Some(&code)
        })
        .count();

    let counts = break_counts(&cells);
    let breaks: u32 = counts.iter().sum();
    let per_row_breaks: Vec<u32> = (0..H)
        .map(|y| counts[y * W..(y + 1) * W].iter().sum())
        .collect();
    let break_rows = (0..H).filter(|&y| per_row_breaks[y] > 0).collect();
    let double_break_cells = (0..W * H).filter(|&p| counts[p] >= 2).collect();
    let break_cells = (0..W * H).filter(|&p| counts[p] > 0).collect();
    let break_grid = (0..H)
        .map(|y| {
            (0..W)
                .map(|x| match counts[y * W + x] {
                    0 => '.',
                    1 => '1',
                    _ => '2',
                })
                .collect()
        })
        .collect();

    let r = BoardReport {
        label: label.to_string(),
        url: url.to_string(),
        score,
        max_score: MAX_SCORE_16,
        breaks,
        break_identity_holds: placed == W * H && breaks == MAX_SCORE_16 - score,
        placed,
        hints_matched,
        hints_total: instance.hints.len(),
        per_row_breaks,
        break_rows,
        double_break_cells,
        break_cells,
        break_grid,
    };
    (r, out.board)
}

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| "boards.txt".to_string());
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("cannot read {path}: {e}"));

    let instance = official_instance(true);

    let mut boards = Vec::new();
    let mut placements: Vec<Vec<i32>> = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (label, url) = line
            .split_once(['\t', ' '])
            .unwrap_or_else(|| panic!("bad line (want 'label<TAB>url'): {line}"));
        let (r, placement) = report(&instance, label, url.trim());
        boards.push(r);
        placements.push(placement);
    }

    let tile_hamming: Vec<Vec<u32>> = placements
        .iter()
        .map(|a| {
            placements
                .iter()
                .map(|b| {
                    a.iter()
                        .zip(b)
                        .filter(|(x, y)| (**x >> 2) != (**y >> 2))
                        .count() as u32
                })
                .collect()
        })
        .collect();

    let out = Output { boards, tile_hamming };
    println!("{}", serde_json::to_string_pretty(&out).expect("serialize"));
}
