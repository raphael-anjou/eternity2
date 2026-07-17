//! `edges-to-placement`: recover a board's explicit placement (piece id +
//! rotation per cell) from its bucas `board_edges` string.
//!
//! Why this exists: some board sources (e.g. the beam producer's `--emit-best`)
//! write only the edge string, but the canonical dataset pipeline (`board_doc`)
//! needs the explicit placement, because piece identity recovered from edges
//! alone can be ambiguous for near-record boards. This binary does that recovery
//! *safely*: it matches each cell's URDL quad against the official piece set at
//! all four rotations, requires the assignment to use each piece exactly once
//! (a permutation of the 256 pieces), and **aborts loudly** on any cell that
//! matches no piece, any piece used twice, or any piece left unused. So its
//! output is either a provably-correct placement or a hard error — never a
//! silently-wrong board.
//!
//! Input : one JSON object per line on stdin: `{ "name": "...", "edges": "<1024
//!         URDL letters>" }` (the letters only; any bucas URL wrapper should be
//!         stripped by the caller, but a `board_edges=` prefix is tolerated).
//! Output: one JSON object per line on stdout in `board_doc`'s input schema:
//!         `{ "name": "...", "placement": [ {pos,piece_id,rotation}, ... ] }`
//!         so the two binaries pipe directly together.
//!
//! Usage: edges_to_placement --official <site-schema puzzle json>

use std::collections::HashMap;
use std::io::{self, BufRead, Write};

use e2_core::rotated;
use e2_io::Instance;
use serde::{Deserialize, Serialize};

const N: usize = 16 * 16;

#[derive(Deserialize)]
struct InputBoard {
    name: String,
    edges: String,
}

#[derive(Serialize)]
struct OutPlacement {
    pos: usize,
    piece_id: u16,
    rotation: u8,
}

#[derive(Serialize)]
struct OutBoard {
    name: String,
    placement: Vec<OutPlacement>,
}

/// Decode a bucas edge string to 256 URDL quads. Tolerates a leading
/// `board_edges=` and any `&...` tail. Letters map a->0, b->1, ...
fn decode_quads(raw: &str) -> Result<Vec<[u8; 4]>, String> {
    let s = raw
        .rsplit("board_edges=")
        .next()
        .unwrap_or(raw)
        .split('&')
        .next()
        .unwrap_or(raw);
    let letters: Vec<u8> = s
        .chars()
        .filter(|c| c.is_ascii_lowercase())
        .map(|c| c as u8 - b'a')
        .collect();
    if letters.len() < N * 4 {
        return Err(format!("need {} edge letters, got {}", N * 4, letters.len()));
    }
    Ok((0..N)
        .map(|i| [letters[i * 4], letters[i * 4 + 1], letters[i * 4 + 2], letters[i * 4 + 3]])
        .collect())
}

fn main() -> io::Result<()> {
    let mut official_path: Option<String> = None;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--official" => official_path = args.next(),
            other => eprintln!("edges-to-placement: ignoring unknown arg {other}"),
        }
    }
    let official_path =
        official_path.expect("--official <site-schema puzzle json> is required");
    let inst = Instance::from_site_json(&official_path).expect("load official piece set");
    let n_pieces = inst.pieces.len();
    assert_eq!(n_pieces, N, "expected {N} pieces, got {n_pieces}");

    // Lookup: quad (at any rotation) -> (piece_id, rotation). Built once. If two
    // distinct (piece,rot) produce the same quad the puzzle would be inherently
    // ambiguous; the official E2 set has no such collision, and we assert it.
    let mut lut: HashMap<[u8; 4], (u16, u8)> = HashMap::with_capacity(n_pieces * 4);
    for (pid, piece) in inst.pieces.iter() {
        for rot in 0u8..4 {
            let q = rotated(piece.edges, rot);
            if let Some((prev_pid, prev_rot)) = lut.insert(q, (pid, rot)) {
                // Two distinct (piece, rotation) yielding the same quad would make
                // edges->placement inherently ambiguous. The official E2 set has
                // no such collision; if a non-standard set does, refuse loudly
                // rather than assign arbitrarily.
                panic!(
                    "quad {q:?} produced by both piece {prev_pid} rot {prev_rot} and \
                     piece {pid} rot {rot}; edges->placement is not unique for this set"
                );
            }
        }
    }

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let mut n_ok = 0usize;
    let mut n_fail = 0usize;

    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let inb: InputBoard = serde_json::from_str(&line).expect("valid input JSON line");
        match recover(&inst, &lut, &inb.edges) {
            Ok(placement) => {
                let ob = OutBoard { name: inb.name, placement };
                writeln!(out, "{}", serde_json::to_string(&ob).expect("serialize"))?;
                n_ok += 1;
            }
            Err(e) => {
                eprintln!("SKIP {}: {}", inb.name, e);
                n_fail += 1;
            }
        }
    }
    eprintln!("edges-to-placement: {n_ok} ok, {n_fail} failed");
    Ok(())
}

/// Recover the placement for one board, or return an error describing exactly
/// why it is not a clean permutation of the official pieces.
fn recover(
    _inst: &Instance,
    lut: &HashMap<[u8; 4], (u16, u8)>,
    raw_edges: &str,
) -> Result<Vec<OutPlacement>, String> {
    let quads = decode_quads(raw_edges)?;
    let mut placement = Vec::with_capacity(N);
    let mut used = vec![false; N];
    for (pos, q) in quads.iter().enumerate() {
        let (pid, rot) = *lut
            .get(q)
            .ok_or_else(|| format!("cell {pos}: quad {q:?} matches no piece"))?;
        if used[pid as usize] {
            return Err(format!("piece {pid} used more than once (cell {pos})"));
        }
        used[pid as usize] = true;
        placement.push(OutPlacement { pos, piece_id: pid, rotation: rot });
    }
    if let Some(missing) = used.iter().position(|&u| !u) {
        return Err(format!("piece {missing} never placed (board is not a permutation)"));
    }
    Ok(placement)
}
