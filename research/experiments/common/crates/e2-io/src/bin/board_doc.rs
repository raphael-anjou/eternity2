//! `board-doc`: turn a board's explicit placement into the canonical
//! [`BoardDoc`] JSON, using the same `e2-io` code every solver on the site emits
//! through.
//!
//! It exists so a published *dataset* of boards can be written in the site's one
//! canonical format without reimplementing that format anywhere. The dataset
//! builder (`research/datasets/build/build_dataset.py`) does the corpus work
//! (dedup, rescore, provenance scrub, stable ids) and hands each board here as
//! one line of JSON on stdin; this binary emits one canonical BoardDoc per line.
//!
//! Input takes the board's **explicit placement** (piece id + rotation per
//! cell), not just its edges. Recovering piece identity from edges alone is
//! ambiguous when pieces share edge patterns (a handful of near-record boards
//! genuinely mis-match), so the unambiguous placement is the authority; the edge
//! colours, score, hash and viewer URL are all derived from it here.
//!
//! Input line schema:
//!   { "name": "e2b_00001",
//!     "placement": [ { "pos": 0, "piece_id": 3, "rotation": 3 }, ... ] }
//! `piece_id` is 0-based; `rotation` is 0..=3, clockwise quarter-turns.
//!
//! Usage:  board-doc --official <variant_00.json>  < boards.in  > docs.out

use std::io::{self, BufRead, Write};

use e2_core::{rotated, score_cells};
use e2_io::format::{board_hash, viewer_url, BoardDoc};
use e2_io::Instance;
use serde::Deserialize;

const SIZE: u8 = 16;
const N: usize = (SIZE as usize) * (SIZE as usize);

#[derive(Deserialize)]
struct Placement {
    pos: usize,
    piece_id: u16,
    rotation: u8,
}

#[derive(Deserialize)]
struct InputBoard {
    name: String,
    placement: Vec<Placement>,
}

fn main() -> io::Result<()> {
    let mut official_path: Option<String> = None;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--official" => official_path = args.next(),
            other => eprintln!("board-doc: ignoring unknown arg {other}"),
        }
    }
    let official_path = official_path.expect("--official <site-schema puzzle json> is required");
    let inst = Instance::from_site_json(&official_path).expect("load official piece set");
    let max_score = inst.max_score();

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();

    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let inb: InputBoard = serde_json::from_str(&line).expect("valid input board JSON");

        // Build per-cell URDL quads and piece*4+rot codes from the explicit
        // placement. Every board here is fully placed; empties stay all-border.
        let mut cells = vec![[0u8; 4]; N];
        let mut codes = vec![-1i32; N];
        for p in &inb.placement {
            assert!(p.pos < N, "pos {} out of range", p.pos);
            let rot = p.rotation % 4;
            // Reuse e2-core's rotation so the convention is identical everywhere.
            cells[p.pos] = rotated(inst.pieces.edges(p.piece_id), rot);
            codes[p.pos] = i32::from(p.piece_id) * 4 + i32::from(rot);
        }

        // Canonical matched-edge score from the placed cells, the one scorer.
        let score = score_cells(&cells);
        let hash = board_hash(&codes);
        let doc = BoardDoc::new(&inb.name, SIZE, score, max_score, &cells, &codes, hash);
        // Defensive: the URL must be built from the same cells/codes.
        debug_assert_eq!(doc.url, viewer_url(&inb.name, SIZE, &cells, &codes));
        writeln!(out, "{}", serde_json::to_string(&doc).expect("serialize BoardDoc"))?;
    }
    Ok(())
}
