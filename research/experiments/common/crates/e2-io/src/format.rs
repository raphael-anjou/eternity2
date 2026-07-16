//! The one canonical board format, defined over **plain data** so it has no
//! dependency on any engine's `Board`/`Puzzle` type. Both the study workspace
//! (`e2-core`) and the separate single-core-benchmark workspace
//! (`eternity2-core`) depend on this crate by path and feed it per-cell edge
//! quads extracted from their own boards — so the URL every algorithm on the
//! site emits, and the JSON every one writes, come from this single file.
//!
//! The canonical artifact is one [`BoardDoc`] serialized to JSON. Every other
//! shape the community speaks — the `board_edges`/`board_pieces` letter blobs,
//! the `e2pieces.txt` numbers, the eternity2.dev viewer URL, the legacy bucas
//! URL — is *derived* from the same cells, so they can never drift.
//!
//! Cells are row-major URDL edge colors, `[u8; 4]` per cell, color `0` = the
//! grey border. An empty cell is `[0; 4]` (which encodes as `"aaaa"`), matching
//! the viewer's convention. `codes` is the parallel row-major `piece*4 + rot`
//! vector (`-1` empty) that the site's `Puzzle` round-trips through unchanged.

use serde::{Deserialize, Serialize};

/// The public site the canonical URL points at. This is the single place the
/// domain is written; every URL the project emits flows through
/// [`viewer_url`]. The legacy `e2.bucas.name` form stays reachable via
/// [`bucas_url`] for interop, but it is no longer any algorithm's default.
pub const VIEWER_ORIGIN: &str = "https://eternity2.dev";

/// Map an edge color to its viewer letter. The viewer clamps out-of-range
/// colors to `'a'`; mirror that so our strings match byte-for-byte.
#[must_use]
pub const fn color_to_letter(c: u8) -> char {
    if c as usize > 22 {
        'a'
    } else {
        (b'a' + c) as char
    }
}

/// The `board_edges` blob: four viewer letters per cell, row-major URDL.
/// `256` cells ⇒ `1024` letters for the official puzzle.
#[must_use]
pub fn cells_to_edges(cells: &[[u8; 4]]) -> String {
    let mut s = String::with_capacity(cells.len() * 4);
    for c in cells {
        for &color in c {
            s.push(color_to_letter(color));
        }
    }
    s
}

/// The `board_pieces` blob: three decimal digits per cell, the 1-based piece
/// number, `000` for an empty cell. Needs the `piece*4 + rot` codes since the
/// edge cells alone do not carry a piece identity.
#[must_use]
pub fn codes_to_pieces(codes: &[i32]) -> String {
    let mut s = String::with_capacity(codes.len() * 3);
    for &v in codes {
        if v < 0 {
            s.push_str("000");
        } else {
            s.push_str(&format!("{:03}", (v >> 2) + 1));
        }
    }
    s
}

/// The canonical eternity2.dev viewer URL. Boards are square on the site, so we
/// emit a single `puzzle_size` (the viewer also accepts `board_w`/`board_h`).
/// Every value stays in the URL-safe `[A-Za-z0-9_]` range, so no
/// percent-encoding is needed and the link is copy-pasteable as-is.
#[must_use]
pub fn viewer_url(name: &str, size: u8, cells: &[[u8; 4]], codes: &[i32]) -> String {
    let name = sanitize_name(name);
    let edges = cells_to_edges(cells);
    let pieces = codes_to_pieces(codes);
    format!(
        "{VIEWER_ORIGIN}/viewer?puzzle={name}&puzzle_size={size}&board_edges={edges}&board_pieces={pieces}"
    )
}

/// The legacy `e2.bucas.name` viewer URL, kept as a derived converter for
/// interop with community tooling and Jef Bucas's original viewer. Not the
/// default output of any algorithm — [`viewer_url`] is.
#[must_use]
pub fn bucas_url(name: &str, width: u8, height: u8, cells: &[[u8; 4]]) -> String {
    let name = sanitize_name(name);
    let edges = cells_to_edges(cells);
    format!(
        "https://e2.bucas.name/#puzzle={name}&board_w={width}&board_h={height}&board_edges={edges}"
    )
}

/// The `e2pieces.txt` rendering: one line per *filled* cell, in reading order,
/// four whitespace-separated edge integers in URDL order, `0` = grey border.
/// A board carries pieces already rotated into place, so these are the pieces
/// **as placed**, not a canonical rotation-0 catalogue.
#[must_use]
pub fn cells_to_e2pieces(cells: &[[u8; 4]]) -> String {
    let mut lines: Vec<String> = Vec::new();
    for c in cells {
        if *c == [0u8; 4] {
            continue;
        }
        lines.push(format!("{} {} {} {}", c[0], c[1], c[2], c[3]));
    }
    lines.join("\n")
}

/// FNV-1a hash over the row-major `piece*4 + rot` codes — a stable board
/// fingerprint that does not need an engine's `Board` type, so every workspace
/// computes the *same* hash for the same placement. `-1` (empty) hashes like any
/// other code value.
#[must_use]
pub fn board_hash(codes: &[i32]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &c in codes {
        for b in c.to_le_bytes() {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    h
}

/// Pull the `board_edges` letter blob out of any viewer input — a full
/// eternity2.dev or bucas URL, a bare params string, or the blob on its own —
/// and decode it into per-cell URDL edge quads. `'a' + colour`, `"aaaa"` ⇒
/// empty ⇒ `[0; 4]`. Returns `None` if no `board_edges` is present or its
/// length is not a multiple of four.
#[must_use]
pub fn parse_board_edges(input: &str) -> Option<Vec<[u8; 4]>> {
    let s = input.trim();
    // Locate the blob: after `board_edges=` if present, else the whole string
    // if it is a bare lowercase run.
    let blob: &str = if let Some(idx) = s.find("board_edges=") {
        let rest = &s[idx + "board_edges=".len()..];
        rest.split(['&', '#', ' ', '\n']).next().unwrap_or(rest)
    } else if !s.is_empty() && s.bytes().all(|b| b.is_ascii_lowercase()) {
        s
    } else {
        return None;
    };
    if blob.is_empty() || blob.len() % 4 != 0 || !blob.bytes().all(|b| b.is_ascii_lowercase()) {
        return None;
    }
    let cells = blob
        .as_bytes()
        .chunks_exact(4)
        .map(|c| {
            [
                c[0].saturating_sub(b'a'),
                c[1].saturating_sub(b'a'),
                c[2].saturating_sub(b'a'),
                c[3].saturating_sub(b'a'),
            ]
        })
        .collect();
    Some(cells)
}

/// Pull the `puzzle=` name out of a viewer input, if present.
#[must_use]
pub fn parse_puzzle_name(input: &str) -> Option<String> {
    let idx = input.find("puzzle=")?;
    let rest = &input[idx + "puzzle=".len()..];
    let name = rest.split(['&', '#', ' ', '\n']).next().unwrap_or(rest);
    (!name.is_empty()).then(|| name.to_string())
}

/// Keep a puzzle name to the URL-safe subset so it never needs encoding.
#[must_use]
pub fn sanitize_name(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    let mut prev_underscore = false;
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            out.push(ch);
            prev_underscore = ch == '_';
        } else if !prev_underscore {
            out.push('_');
            prev_underscore = true;
        }
    }
    if out.is_empty() {
        out.push_str("eternity2_board");
    }
    out
}

/// The one canonical board document. Every solver on the site emits exactly
/// this, serialized to a single `.json` file — score, breaks, the placement
/// vector, the content hash, both letter blobs, and a ready-to-open
/// eternity2.dev URL as a field (not a separate file).
///
/// It is deliberately self-describing: given only this JSON, a tool can render
/// the board (`board_edges`), identify the pieces (`board_pieces`), re-derive
/// any other format, or open it in the viewer (`url`) — no companion file, no
/// out-of-band size or piece set required beyond the standard official set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoardDoc {
    /// Human name, carried into the URL.
    pub name: String,
    /// Board side length; boards are square.
    pub size: u8,
    /// Canonical matched-edge score — the single source of truth.
    pub score: u32,
    /// `MAX - score`: unmatched interior edges. On a full board, the breaks.
    pub breaks: u32,
    /// Row-major `piece*4 + rot`, `-1` empty. The site's `Puzzle` reads this
    /// back unchanged.
    pub board: Vec<i32>,
    /// FNV-1a content hash of the placement grid — the A/B identity gate.
    pub board_hash: u64,
    /// Four viewer letters per cell (URDL), row-major. `"aaaa"` = empty.
    pub board_edges: String,
    /// Three digits per cell, 1-based piece number, `000` empty.
    pub board_pieces: String,
    /// The canonical eternity2.dev viewer URL for this board.
    pub url: String,
}

impl BoardDoc {
    /// Assemble a document from the plain pieces every engine can produce: the
    /// per-cell edge quads, the `piece*4 + rot` codes, the score, and the hash.
    /// All derived strings are built here so they are guaranteed consistent.
    #[must_use]
    pub fn new(
        name: &str,
        size: u8,
        score: u32,
        max_score: u32,
        cells: &[[u8; 4]],
        codes: &[i32],
        board_hash: u64,
    ) -> Self {
        Self {
            name: sanitize_name(name),
            size,
            score,
            breaks: max_score.saturating_sub(score),
            board: codes.to_vec(),
            board_hash,
            board_edges: cells_to_edges(cells),
            board_pieces: codes_to_pieces(codes),
            url: viewer_url(name, size, cells, codes),
        }
    }

    /// Serialize to the canonical pretty JSON string — the one artifact every
    /// algorithm writes to disk.
    #[must_use]
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("BoardDoc serializes")
    }

    /// Write the canonical JSON to a path.
    pub fn write_json<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<()> {
        std::fs::write(path, self.to_json())
    }

    /// The legacy bucas URL for this board, derived on demand.
    #[must_use]
    pub fn bucas_url(&self) -> String {
        // Rebuild cells from the codes-free edge blob: the blob already holds
        // the placed edges, so re-emit it under the bucas domain/params.
        format!(
            "https://e2.bucas.name/#puzzle={}&board_w={}&board_h={}&board_edges={}",
            self.name, self.size, self.size, self.board_edges
        )
    }
}

#[cfg(test)]
mod format_tests {
    use super::*;

    #[test]
    fn edges_length_matches_cells() {
        let cells = vec![[1u8, 2, 3, 4]; 256];
        assert_eq!(cells_to_edges(&cells).len(), 256 * 4);
    }

    #[test]
    fn empty_cell_is_aaaa_and_000() {
        assert_eq!(cells_to_edges(&[[0, 0, 0, 0]]), "aaaa");
        assert_eq!(codes_to_pieces(&[-1]), "000");
    }

    #[test]
    fn piece_number_is_one_based() {
        // code 0 = piece 0, rot 0 -> piece number 001.
        assert_eq!(codes_to_pieces(&[0]), "001");
        // code 4 = piece 1 -> 002; low two bits are rotation, ignored here.
        assert_eq!(codes_to_pieces(&[4 + 3]), "002");
    }

    #[test]
    fn viewer_url_uses_the_dev_origin() {
        let url = viewer_url("t", 16, &[[1, 2, 3, 4]; 256], &[0i32; 256]);
        assert!(url.starts_with("https://eternity2.dev/viewer?"));
        assert!(url.contains("puzzle_size=16"));
        assert!(url.contains("board_edges="));
        assert!(url.contains("board_pieces="));
    }

    #[test]
    fn doc_round_trips_through_json() {
        let doc = BoardDoc::new("demo", 16, 469, 480, &[[1, 2, 3, 4]; 256], &[0i32; 256], 42);
        let back: BoardDoc = serde_json::from_str(&doc.to_json()).unwrap();
        assert_eq!(doc, back);
        assert_eq!(back.breaks, 480 - 469);
        assert!(back.url.starts_with("https://eternity2.dev/viewer?"));
    }

    #[test]
    fn name_is_sanitized_for_urls() {
        assert_eq!(sanitize_name("Joshua Blackwood 470"), "Joshua_Blackwood_470");
        assert_eq!(sanitize_name(""), "eternity2_board");
    }
}
