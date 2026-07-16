// End-of-run reporting: writes ONE canonical board `.json` (an
// `e2_io::BoardDoc`, carrying an eternity2.dev viewer URL) for the final board
// into an output directory. Used by all benchmark binaries so every run leaves
// a verifiable board on disk — no more `.url.txt` sidecar, no competing shape.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use eternity2_core::{Board, Puzzle};
use serde_json::Value;

use crate::bucas::board_to_doc;
use crate::score::score_board;

#[must_use]
pub fn puzzle_name_from_path(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("puzzle")
        .to_string()
}

pub struct RunReport {
    pub json_path: PathBuf,
    /// The canonical eternity2.dev viewer URL for the board just written.
    pub url: String,
}

/// Write the final board as a single canonical `<stem>.json` (`e2_io::BoardDoc`)
/// in `output_dir`. `extra` is accepted for source compatibility but is no
/// longer written as a separate stats blob — the one artifact is the board
/// document itself. Returns the path written and its viewer URL.
pub fn write_report(
    output_dir: &Path,
    run_name: &str,
    puzzle: &Puzzle,
    puzzle_name: &str,
    board: &Board,
    _extra: Value,
) -> std::io::Result<RunReport> {
    fs::create_dir_all(output_dir)?;

    let (matched, total) = score_board(puzzle, board);

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let stem = format!("{run_name}_{timestamp}_{matched}of{total}");
    let json_path = output_dir.join(format!("{stem}.json"));

    let doc = board_to_doc(puzzle, board, puzzle_name, matched);
    doc.write_json(&json_path)?;

    Ok(RunReport {
        json_path,
        url: doc.url,
    })
}
