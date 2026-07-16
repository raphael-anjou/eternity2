//! Generate the 10 benchmark puzzle variants: the official E2 puzzle with 3
//! corners pinned (per the user's "pin 3 corners, 10 seeds" choice). Every
//! algorithm in the grid solves the SAME 10 variants.
//!
//! A variant is a corner *arrangement*: an assignment of the 4 corner pieces to
//! the 4 corner cells. There are 4! = 24 arrangements; we take 10 distinct ones
//! and pin the first 3 corners of each (leaving the 4th free so the instance is
//! still a real search, not a near-forced one). Each corner piece has exactly
//! one rotation that places its two border edges facing outward at a given
//! corner cell; we compute it rather than hardcode.
//!
//! Output: `<outdir>/variant_NN.json` (site-schema `SiteInstance`) + a
//! `manifest.json` recording the exact arrangement of each variant for
//! reproducibility.

use bench_grid::{rotated, SiteHint, SiteInstance, SiteInstanceExt, BORDER};
use serde::Serialize;

// The official 16x16 puzzle CSV, resolved at build time relative to this crate
// (engine/crates/bench-grid) so it works from any checkout. Override with the
// OFFICIAL_CSV env var if the data lives elsewhere.
fn official_csv() -> String {
    std::env::var("OFFICIAL_CSV").unwrap_or_else(|_| {
        format!(
            "{}/../../data/puzzles/size_16_official_eternity.csv",
            env!("CARGO_MANIFEST_DIR")
        )
    })
}

// The 4 corner cells of a 16x16 board (row-major pos) and, for each, which two
// URDL edge slots must be border (facing outward).
// slot indices: 0=top(up) 1=right 2=down 3=left.
const CORNER_CELLS: [(u16, [usize; 2]); 4] = [
    (0, [0, 3]),     // top-left: up + left border
    (15, [0, 1]),    // top-right: up + right border
    (240, [2, 3]),   // bottom-left: down + left border
    (255, [1, 2]),   // bottom-right: down + right border
];

#[derive(Serialize)]
struct Manifest {
    description: String,
    official_csv: String,
    corner_pieces: Vec<u16>,
    corner_cells: Vec<u16>,
    variants: Vec<VariantRecord>,
}

#[derive(Serialize)]
struct VariantRecord {
    id: usize,
    /// Full corner->piece arrangement (all 4); only the first 3 are pinned.
    arrangement: Vec<u16>,
    pinned_hints: Vec<PinRecord>,
}

#[derive(Serialize)]
struct PinRecord {
    pos: u16,
    piece: u16,
    rot: u8,
}

/// The rotation that places corner `piece`'s two borders in the required two
/// outward slots at a corner cell, or `None` if no rotation fits (shouldn't
/// happen for a genuine corner piece).
fn corner_rotation(piece_edges: [u8; 4], required_border_slots: [usize; 2]) -> Option<u8> {
    for r in 0..4u8 {
        let e = rotated(piece_edges, r);
        let borders_ok = required_border_slots.iter().all(|&s| e[s] == BORDER);
        let non_border_count = e.iter().filter(|&&c| c != BORDER).count();
        // Exactly the two required slots are border; the other two are interior.
        if borders_ok && non_border_count == 2 {
            return Some(r);
        }
    }
    None
}

/// 10 distinct permutations of [0,1,2,3] (corner-piece -> corner-cell). Fixed
/// list for reproducibility; first is identity, rest are lexicographic-ish
/// derangements to spread the pinned corners.
const ARRANGEMENTS: [[u16; 4]; 10] = [
    [0, 1, 2, 3],
    [1, 0, 3, 2],
    [2, 3, 0, 1],
    [3, 2, 1, 0],
    [0, 2, 3, 1],
    [1, 3, 2, 0],
    [2, 0, 1, 3],
    [3, 1, 0, 2],
    [0, 3, 1, 2],
    [2, 1, 3, 0],
];

/// Write a CSV variant: copy the official CSV verbatim, but for each corner
/// pin set that piece's row (x, y, rot) columns to (pos%16, pos/16, rot).
/// Corner cell 0 always needs rot=3, so no pin collides with the (0,0,0)
/// "absent hint" sentinel.
fn write_csv_variant(official_csv: &str, out: &str, pins: &[PinRecord]) {
    let raw = std::fs::read_to_string(official_csv).expect("read official csv");
    let mut lines: Vec<String> = raw.lines().map(str::to_string).collect();
    // lines[0] = size header; piece `id` is at lines[id + 1].
    for pin in pins {
        let idx = pin.piece as usize + 1;
        let cols: Vec<&str> = lines[idx].split(',').collect();
        assert!(cols.len() >= 7, "piece row must have 7 columns");
        let x = pin.pos % 16;
        let y = pin.pos / 16;
        assert!(
            !(x == 0 && y == 0 && pin.rot == 0),
            "corner pin would collide with the absent-hint sentinel"
        );
        let new_row = format!(
            "{},{},{},{},{},{},{}",
            cols[0], cols[1], cols[2], cols[3], x, y, pin.rot
        );
        lines[idx] = new_row;
    }
    std::fs::write(out, lines.join("\n") + "\n").expect("write csv variant");
}

fn main() {
    let outdir = std::env::args()
        .nth(1)
        .expect("usage: gen_variants <outdir>");
    std::fs::create_dir_all(&outdir).expect("mkdir outdir");

    let official = official_csv();
    let base = SiteInstance::official(&official).expect("load official");

    // Identify the 4 corner pieces (exactly 2 border edges) in id order.
    let corner_pieces: Vec<u16> = base
        .pieces
        .iter()
        .enumerate()
        .filter(|(_, e)| e.iter().filter(|&&c| c == BORDER).count() == 2)
        .map(|(id, _)| id as u16)
        .collect();
    assert_eq!(corner_pieces.len(), 4, "expected exactly 4 corner pieces");

    let mut manifest = Manifest {
        description: "Official E2 + 3 pinned corners per variant (pin-3-corners, 10 arrangements). \
                      All engines solve these identical 10 instances."
            .into(),
        official_csv: official.clone(),
        corner_pieces: corner_pieces.clone(),
        corner_cells: CORNER_CELLS.iter().map(|c| c.0).collect(),
        variants: Vec::new(),
    };

    for (vid, arr) in ARRANGEMENTS.iter().enumerate() {
        let mut inst = base.clone();
        let arrangement: Vec<u16> = arr.iter().map(|&slot| corner_pieces[slot as usize]).collect();
        let mut pins = Vec::new();
        // Pin the FIRST 3 corner cells only.
        for ci in 0..3usize {
            let (cell, req) = CORNER_CELLS[ci];
            let piece = arrangement[ci];
            let rot = corner_rotation(base.pieces[piece as usize], req)
                .unwrap_or_else(|| panic!("no valid corner rotation for piece {piece} at cell {cell}"));
            inst.hints.push(SiteHint { pos: cell, piece, rot });
            pins.push(PinRecord { pos: cell, piece, rot });
        }
        inst.name = format!("e2_official_variant{vid:02}_pin3corners");

        let path = format!("{outdir}/variant_{vid:02}.json");
        std::fs::write(&path, serde_json::to_string_pretty(&inst).unwrap()).expect("write variant");

        // Also emit a CSV variant: the official CSV with the 3 corner pins
        // written into the piece rows' (x,y,rot) hint columns. Every
        // CSV-loading engine (producer, blackwood_bt, verhaard, alns) then
        // reads the identical 8 hints through the shared loader — no
        // per-engine hint plumbing needed.
        let csv_path = format!("{outdir}/variant_{vid:02}.csv");
        write_csv_variant(&official, &csv_path, &pins);

        manifest.variants.push(VariantRecord {
            id: vid,
            arrangement,
            pinned_hints: pins,
        });
        println!("variant {vid:02}: pinned 3 corners -> {path}");
    }

    let mpath = format!("{outdir}/manifest.json");
    std::fs::write(&mpath, serde_json::to_string_pretty(&manifest).unwrap()).expect("write manifest");
    println!("manifest -> {mpath}");
    println!(
        "corner pieces {:?} at cells {:?}; 10 variants, each pins 3.",
        corner_pieces,
        CORNER_CELLS.iter().map(|c| c.0).collect::<Vec<_>>()
    );
}
