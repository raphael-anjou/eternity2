// Board serialization — vol-118 T8 consolidation.
//
// Two on-disk formats are supported:
// 1. CANONICAL (placement-sparse): the format `save_board` writes.
//    {"placement": [{"pos": N, "piece_id": P, "rotation": R}, ...],
//     "metadata": {...}}
//    Empty cells are omitted from the array. The `pos` field is explicit
//    so position-ordering of the array doesn't matter.
//
// 2. LEGACY ACCEPTED on read (load_board reads any of):
//    - DumpedBoard: {"cells": [[pid, rot] | null, ...]} (this module's
//      original format; index-as-position).
//    - placement-sparse (canonical, with explicit pos).
//    - placement-indexed: {"placement": [{"piece_id": P, "rotation": R}]}
//      where array index = position; null entries = empty.
//
// See docs/BOARD_FORMAT.md for the schema.

use std::fs;
use std::io::Write;
use std::path::Path;

use eternity2_core::{Board, PieceId, Puzzle, Rotation};
use serde::{Deserialize, Serialize};

// =============================================================================
// Legacy DumpedBoard (kept for backwards compatibility).
// =============================================================================

#[derive(Serialize, Deserialize)]
pub struct DumpedBoard {
    pub width: u32,
    pub height: u32,
    pub seed: u64,
    pub score: u32,
    pub total_edges: u32,
    pub cells: Vec<Option<[u32; 2]>>,
}

impl DumpedBoard {
    #[must_use]
    pub fn from_board(board: &Board, seed: u64, score: u32, total_edges: u32) -> Self {
        let cells = board
            .cells()
            .iter()
            .map(|c| c.map(|(pid, rot)| [u32::from(pid), u32::from(rot.as_u8())]))
            .collect();
        Self {
            width: board.width(),
            height: board.height(),
            seed,
            score,
            total_edges,
            cells,
        }
    }

    #[must_use]
    pub fn to_board(&self, puzzle: &Puzzle) -> Board {
        let mut b = Board::empty(puzzle);
        for (pos, slot) in self.cells.iter().enumerate() {
            if let Some([pid, rot]) = slot {
                let rotation = Rotation::from_u8(*rot as u8).unwrap_or(Rotation::R0);
                b.place(pos as u32, *pid as u16, rotation);
            }
        }
        b
    }
}

pub fn write_dump(path: &Path, dump: &DumpedBoard) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut f = fs::File::create(path)?;
    f.write_all(serde_json::to_string(dump)?.as_bytes())?;
    f.write_all(b"\n")?;
    Ok(())
}

pub fn read_dump(path: &Path) -> std::io::Result<DumpedBoard> {
    let s = fs::read_to_string(path)?;
    serde_json::from_str(&s).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

// =============================================================================
// CANONICAL load_board / save_board — vol-118 T8.
// =============================================================================

/// Metadata attached to a saved board.
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

#[derive(Serialize)]
struct PlacementEntry {
    pos: u32,
    piece_id: u32,
    rotation: u8,
}

#[derive(Serialize)]
struct SavedBoard<'a> {
    placement: Vec<PlacementEntry>,
    score: u32,
    total_adjacencies: u32,
    metadata: &'a BoardMetadata,
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
/// - Canonical placement-sparse: `{"placement": [{"pos","piece_id","rotation"}]}`
/// - Placement-indexed: `{"placement": [{"piece_id","rotation"}]}` (array
///   index = position)
/// - DumpedBoard: `{"cells": [[pid, rot] | null, ...]}`
/// - Nested: `{"board": {"placement": [...]}}` (from run_e2_restart summaries)
pub fn load_board(path: &Path, puzzle: &Puzzle) -> Result<Board, LoadError> {
    let s = fs::read_to_string(path)?;
    let v: serde_json::Value =
        serde_json::from_str(&s).map_err(|e| LoadError::Parse(e.to_string()))?;

    // 1. Try DumpedBoard format (has "cells" array of pairs).
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

    // 2. Try placement array (canonical OR indexed).
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

/// Save a board in the canonical placement-sparse JSON format.
/// Includes the matched-edges score for informational purposes (the
/// verifier always recomputes from the placement).
pub fn save_board(
    path: &Path,
    puzzle: &Puzzle,
    board: &Board,
    metadata: &BoardMetadata,
) -> Result<(), LoadError> {
    let mut placement: Vec<PlacementEntry> = Vec::new();
    for pos in 0..puzzle.cell_count() {
        if let Some((pid, rot)) = board.get(pos) {
            placement.push(PlacementEntry {
                pos,
                piece_id: u32::from(pid),
                rotation: rot.as_u8(),
            });
        }
    }
    let (matched, total_adjacencies) = crate::score::score_board(puzzle, board);
    let saved = SavedBoard {
        placement,
        score: matched,
        total_adjacencies,
        metadata,
    };
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut f = fs::File::create(path)?;
    let json = serde_json::to_string_pretty(&saved)
        .map_err(|e| LoadError::Parse(format!("serialize: {e}")))?;
    f.write_all(json.as_bytes())?;
    f.write_all(b"\n")?;
    Ok(())
}

/// Save a board in CSV format. Columns: pos, piece_id, rotation.
pub fn save_board_csv(
    path: &Path,
    puzzle: &Puzzle,
    board: &Board,
) -> Result<(), LoadError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut f = fs::File::create(path)?;
    f.write_all(b"pos,piece_id,rotation\n")?;
    for pos in 0..puzzle.cell_count() {
        if let Some((pid, rot)) = board.get(pos) {
            writeln!(f, "{},{},{}", pos, u32::from(pid), rot.as_u8())?;
        }
    }
    Ok(())
}

/// Load a board from a CSV file produced by save_board_csv. Header row
/// optional; column order is fixed (pos, piece_id, rotation).
pub fn load_board_csv(path: &Path, puzzle: &Puzzle) -> Result<Board, LoadError> {
    let s = fs::read_to_string(path)?;
    let mut board = Board::empty(puzzle);
    for (i, line) in s.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // Skip header line if it starts with "pos".
        if i == 0 && line.starts_with("pos") {
            continue;
        }
        let mut cols = line.split(',');
        let pos: u32 = cols
            .next()
            .ok_or_else(|| LoadError::BadEntry(format!("line {i}: missing pos")))?
            .trim()
            .parse()
            .map_err(|e| LoadError::BadEntry(format!("line {i}: pos: {e}")))?;
        let pid: u32 = cols
            .next()
            .ok_or_else(|| LoadError::BadEntry(format!("line {i}: missing piece_id")))?
            .trim()
            .parse()
            .map_err(|e| LoadError::BadEntry(format!("line {i}: piece_id: {e}")))?;
        let rot: u8 = cols
            .next()
            .ok_or_else(|| LoadError::BadEntry(format!("line {i}: missing rotation")))?
            .trim()
            .parse()
            .map_err(|e| LoadError::BadEntry(format!("line {i}: rotation: {e}")))?;
        let piece_id = PieceId::try_from(pid)
            .map_err(|e| LoadError::BadEntry(format!("piece_id {pid}: {e}")))?;
        let rotation = Rotation::from_u8(rot)
            .ok_or_else(|| LoadError::BadEntry(format!("rotation {rot} invalid")))?;
        board.place(pos, piece_id, rotation);
    }
    Ok(board)
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
    fn canonical_roundtrip_csv() {
        let p = tiny_puzzle();
        let mut b = Board::empty(&p);
        b.place(0, 0u16, Rotation::R0);
        b.place(3, 3u16, Rotation::R0);

        let tmp = tempdir();
        let path = tmp.join("board.csv");
        save_board_csv(&path, &p, &b).expect("save");
        let loaded = load_board_csv(&path, &p).expect("load");
        for pos in 0..4 {
            assert_eq!(b.get(pos), loaded.get(pos));
        }
    }

    #[test]
    fn loads_dumped_board_format() {
        let p = tiny_puzzle();
        let mut b = Board::empty(&p);
        b.place(0, 0u16, Rotation::R0);
        b.place(1, 1u16, Rotation::R0);

        let dump = DumpedBoard::from_board(&b, 42, 0, 4);
        let tmp = tempdir();
        let path = tmp.join("dump.json");
        write_dump(&path, &dump).expect("write");

        let loaded = load_board(&path, &p).expect("load DumpedBoard via load_board");
        assert_eq!(loaded.get(0), Some((0u16, Rotation::R0)));
        assert_eq!(loaded.get(1), Some((1u16, Rotation::R0)));
    }

    fn tempdir() -> std::path::PathBuf {
        let base = std::env::temp_dir().join(format!(
            "e2_export_test_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&base).unwrap();
        base
    }
}
