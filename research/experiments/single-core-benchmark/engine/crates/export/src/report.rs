// End-of-run reporting: writes a JSON stats file plus a clickable Bucas URL
// for the final board into an output directory. Used by all benchmark
// binaries so every run leaves a postmortem on disk.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use eternity2_core::{Board, Puzzle};
use serde_json::{json, Value};

use crate::bucas::bucas_url;
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
    pub url_path: PathBuf,
    pub url: String,
}

pub fn write_report(
    output_dir: &Path,
    run_name: &str,
    puzzle: &Puzzle,
    puzzle_name: &str,
    board: &Board,
    extra: Value,
) -> std::io::Result<RunReport> {
    fs::create_dir_all(output_dir)?;

    let (matched, total) = score_board(puzzle, board);
    let placed = board.cells().iter().filter(|c| c.is_some()).count() as u32;
    let url = bucas_url(puzzle, board, puzzle_name);

    let placement: Vec<Value> = board
        .cells()
        .iter()
        .map(|c| match c {
            Some((pid, rot)) => json!({"piece_id": *pid, "rotation": rot.as_u8()}),
            None => Value::Null,
        })
        .collect();

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let stem = format!("{run_name}_{timestamp}_{matched}of{total}");

    let json_path = output_dir.join(format!("{stem}.json"));
    let url_path = output_dir.join(format!("{stem}.url.txt"));

    let stats = json!({
        "run_name": run_name,
        "timestamp_unix": timestamp,
        "puzzle": {
            "name": puzzle_name,
            "width": puzzle.width,
            "height": puzzle.height,
            "color_count": puzzle.color_count,
            "piece_count": puzzle.pieces().len(),
        },
        "score": {
            "matched_edges": matched,
            "total_edges": total,
            "percent": 100.0 * (matched as f64) / (total as f64),
            "placed_cells": placed,
            "total_cells": puzzle.cell_count(),
        },
        "bucas_url": url,
        "placement": placement,
        "details": extra,
    });

    let mut f = fs::File::create(&json_path)?;
    f.write_all(serde_json::to_string_pretty(&stats)?.as_bytes())?;
    f.write_all(b"\n")?;

    let mut u = fs::File::create(&url_path)?;
    writeln!(
        u,
        "{matched}/{total} matched edges  ({:.1}%)",
        100.0 * (matched as f64) / (total as f64)
    )?;
    writeln!(u, "{}", url)?;

    Ok(RunReport {
        json_path,
        url_path,
        url,
    })
}
