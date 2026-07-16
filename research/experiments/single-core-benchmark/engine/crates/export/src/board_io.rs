// Board serialization — standardized onto the canonical `e2_io::BoardDoc`.
//
// One on-disk WRITE format: the canonical board document
// (`e2_io::BoardDoc`) — a single `.json` with `board: Vec<i32>` codes
// (`piece*4+rot`, `-1` empty), the derived edge/piece blobs, and an
// `https://eternity2.dev/viewer?…` URL. `save_board` writes exactly this.
//
// `load_board` still ACCEPTS every in-the-wild shape so nothing on disk
// stops loading:
//   - canonical BoardDoc: {"board": [i32, ...]} (dense, index = position)
//   - placement-sparse:   {"placement": [{"pos","piece_id","rotation"}]}
//   - placement-indexed:  {"placement": [{"piece_id","rotation"}]}
//   - DumpedBoard cells:  {"cells": [[pid, rot] | null, ...]}
//   - nested:             {"board": {"placement": [...]}}
//
// See docs/BOARD_FORMAT.md for the schema.

use std::fs;
use std::path::Path;

use eternity2_core::{Board, PieceId, Puzzle, Rotation};
use serde::{Deserialize, Serialize};

/// Metadata attached to a saved board. Kept as an input to `save_board` (its
/// `source` names the board in the canonical document); the fields no longer
/// have their own on-disk section.
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct BoardMetadata {
    /// Free-form identifier of the producing bin/algorithm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// RNG seed if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,
    /// Wall-clock time the producer spent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elapsed_ms: Option<u64>,
    /// Other relevant tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

#[derive(Debug)]
pub enum LoadError {
    Io(std::io::Error),
    Parse(String),
    BadEntry(String),
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::Io(e) => write!(f, "I/O error: {e}"),
            LoadError::Parse(s) => write!(f, "parse error: {s}"),
            LoadError::BadEntry(s) => write!(f, "bad placement entry: {s}"),
        }
    }
}

impl std::error::Error for LoadError {}

impl From<std::io::Error> for LoadError {
    fn from(e: std::io::Error) -> Self {
        LoadError::Io(e)
    }
}

/// Load a board from JSON. Accepts all in-the-wild formats:
/// - Canonical BoardDoc: `{"board": [i32, ...]}` (dense codes, index = position)
/// - Placement-sparse: `{"placement": [{"pos","piece_id","rotation"}]}`
/// - Placement-indexed: `{"placement": [{"piece_id","rotation"}]}` (array
///   index = position)
/// - DumpedBoard: `{"cells": [[pid, rot] | null, ...]}`
/// - Nested: `{"board": {"placement": [...]}}` (from run_e2_restart summaries)
pub fn load_board(path: &Path, puzzle: &Puzzle) -> Result<Board, LoadError> {
    let s = fs::read_to_string(path)?;
    let v: serde_json::Value =
        serde_json::from_str(&s).map_err(|e| LoadError::Parse(e.to_string()))?;

    // 1. Try the canonical BoardDoc format: "board" is a dense array of ints
    //    (`piece*4+rot`, -1 empty), index = position.
    if let Some(codes) = v
        .get("board")
        .and_then(|b| b.as_array())
        .filter(|a| a.iter().all(serde_json::Value::is_number))
    {
        let mut board = Board::empty(puzzle);
        for (pos, code) in codes.iter().enumerate() {
            let code = code
                .as_i64()
                .ok_or_else(|| LoadError::BadEntry(format!("board[{pos}] not an integer")))?;
            if code < 0 {
                continue;
            }
            let pid = (code / 4) as u32;
            let rot = (code % 4) as u8;
            let piece_id = PieceId::try_from(pid)
                .map_err(|e| LoadError::BadEntry(format!("piece_id {pid}: {e}")))?;
            let rotation = Rotation::from_u8(rot)
                .ok_or_else(|| LoadError::BadEntry(format!("rotation {rot} invalid")))?;
            board.place(pos as u32, piece_id, rotation);
        }
        return Ok(board);
    }

    // 2. Try DumpedBoard format (has "cells" array of pairs).
    if let Some(cells) = v.get("cells").and_then(|c| c.as_array()) {
        let mut board = Board::empty(puzzle);
        for (pos, slot) in cells.iter().enumerate() {
            if slot.is_null() {
                continue;
            }
            let arr = slot
                .as_array()
                .ok_or_else(|| LoadError::BadEntry(format!("cells[{pos}] not array/null")))?;
            let pid = arr
                .first()
                .and_then(|x| x.as_u64())
                .ok_or_else(|| LoadError::BadEntry(format!("cells[{pos}] missing piece_id")))?;
            let rot = arr
                .get(1)
                .and_then(|x| x.as_u64())
                .ok_or_else(|| LoadError::BadEntry(format!("cells[{pos}] missing rotation")))?;
            let piece_id = PieceId::try_from(pid as u32)
                .map_err(|e| LoadError::BadEntry(format!("piece_id {pid}: {e}")))?;
            let rotation = Rotation::from_u8(rot as u8)
                .ok_or_else(|| LoadError::BadEntry(format!("rotation {rot} invalid")))?;
            board.place(pos as u32, piece_id, rotation);
        }
        return Ok(board);
    }

    // 3. Try placement array (canonical OR indexed).
    let arr = v
        .get("placement")
        .and_then(|x| x.as_array())
        .or_else(|| {
            v.get("board")
                .and_then(|b| b.get("placement"))
                .and_then(|x| x.as_array())
        })
        .ok_or_else(|| LoadError::Parse("no 'placement' or 'cells' array found".to_string()))?;

    let mut board = Board::empty(puzzle);
    for (idx, item) in arr.iter().enumerate() {
        if item.is_null() {
            continue;
        }
        let pos = item
            .get("pos")
            .and_then(|x| x.as_u64())
            .unwrap_or(idx as u64) as u32;
        let pid = item
            .get("piece_id")
            .and_then(|x| x.as_u64())
            .ok_or_else(|| LoadError::BadEntry(format!("placement[{idx}] missing piece_id")))?;
        let rot = item
            .get("rotation")
            .and_then(|x| x.as_u64())
            .ok_or_else(|| LoadError::BadEntry(format!("placement[{idx}] missing rotation")))?;
        let piece_id = PieceId::try_from(pid as u32)
            .map_err(|e| LoadError::BadEntry(format!("piece_id {pid}: {e}")))?;
        let rotation = Rotation::from_u8(rot as u8)
            .ok_or_else(|| LoadError::BadEntry(format!("rotation {rot} invalid")))?;
        board.place(pos, piece_id, rotation);
    }
    Ok(board)
}

/// Save a board as the canonical `e2_io::BoardDoc` JSON: one `.json` with the
/// dense `board` codes, the derived edge/piece blobs, and an eternity2.dev
/// viewer URL. `metadata.source` (if set) names the board in the document.
/// The verifier always recomputes the score from the placement.
pub fn save_board(
    path: &Path,
    puzzle: &Puzzle,
    board: &Board,
    metadata: &BoardMetadata,
) -> Result<(), LoadError> {
    let (matched, _total_adjacencies) = crate::score::score_board(puzzle, board);
    let name = metadata.source.as_deref().unwrap_or("board");
    let doc = crate::bucas::board_to_doc(puzzle, board, name, matched);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    doc.write_json(path).map_err(LoadError::Io)
}

#[cfg(test)]
mod tests {
    use super::*;
    use eternity2_core::{Edges, Piece};

    fn tiny_puzzle() -> Puzzle {
        let pieces = vec![
            Piece::new(0, Edges::new(0, 1, 1, 0)),
            Piece::new(1, Edges::new(0, 0, 1, 1)),
            Piece::new(2, Edges::new(1, 1, 0, 0)),
            Piece::new(3, Edges::new(1, 0, 0, 1)),
        ];
        Puzzle::new(2, 2, 2, pieces).expect("tiny")
    }

    #[test]
    fn canonical_roundtrip_json() {
        let p = tiny_puzzle();
        let mut b = Board::empty(&p);
        b.place(0, 0u16, Rotation::R0);
        b.place(1, 1u16, Rotation::R0);
        b.place(2, 2u16, Rotation::R0);
        b.place(3, 3u16, Rotation::R0);

        let tmp = tempdir();
        let path = tmp.join("board.json");
        let meta = BoardMetadata {
            source: Some("test".to_string()),
            seed: Some(42),
            ..Default::default()
        };
        save_board(&path, &p, &b, &meta).expect("save");
        let loaded = load_board(&path, &p).expect("load");
        for pos in 0..4 {
            assert_eq!(b.get(pos), loaded.get(pos));
        }
    }

    #[test]
    fn save_board_emits_canonical_doc() {
        // save_board now writes an e2_io::BoardDoc: a dense `board` codes array
        // plus an eternity2.dev viewer URL. load_board reads it straight back.
        let p = tiny_puzzle();
        let mut b = Board::empty(&p);
        b.place(0, 0u16, Rotation::R0);
        b.place(3, 3u16, Rotation::R0);

        let tmp = tempdir();
        let path = tmp.join("board.json");
        save_board(&path, &p, &b, &BoardMetadata::default()).expect("save");
        let text = std::fs::read_to_string(&path).unwrap();
        assert!(text.contains("https://eternity2.dev/viewer?"));
        assert!(text.contains("\"board\""));

        let loaded = load_board(&path, &p).expect("load canonical doc");
        for pos in 0..4 {
            assert_eq!(b.get(pos), loaded.get(pos));
        }
    }

    #[test]
    fn loads_dumped_board_cells_format() {
        // load_board still accepts the legacy {"cells": [[pid,rot]|null,...]}.
        let p = tiny_puzzle();
        let json = r#"{"cells":[[0,0],[1,0],null,null]}"#;
        let tmp = tempdir();
        let path = tmp.join("dump.json");
        std::fs::write(&path, json).unwrap();

        let loaded = load_board(&path, &p).expect("load cells shape via load_board");
        assert_eq!(loaded.get(0), Some((0u16, Rotation::R0)));
        assert_eq!(loaded.get(1), Some((1u16, Rotation::R0)));
    }

    fn tempdir() -> std::path::PathBuf {
        // A process-unique counter, not just a timestamp: tests run in parallel
        // and two can read the same nanosecond, collide on one dir, and clobber
        // each other's board.json. The counter makes every call distinct.
        use std::sync::atomic::{AtomicU64, Ordering};
        static SEQ: AtomicU64 = AtomicU64::new(0);
        let n = SEQ.fetch_add(1, Ordering::Relaxed);
        let base = std::env::temp_dir().join(format!(
            "e2_export_test_{}_{n}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&base).unwrap();
        base
    }
}
