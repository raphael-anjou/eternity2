// Loads the legacy CSV puzzle format used by data/benchmark/*.csv and
// the canonical 5-clue Eternity II puzzle file.
//
// Format (per file):
//   line 1: board size N
//   lines 2..N²+1: top,right,bottom,left, x, y, rotation
//
// Each color is a 16-bit zero-padded binary string. `1111111111111111` = 65535
// is the gray border; everything else is a small integer color id (1, 2, ...).
//
// Hint convention (used in the official Eternity II puzzle file):
//   - Most piece rows have x=y=rotation=0 (no hint).
//   - "Hint" pieces have specific (x, y) ≠ (0, 0) and rotation ∈ {0..3}.
//   - These pinned placements correspond to the canonical clue pieces
//     (138, 180, 207, 248, 254 in the official file).
//
// `load_puzzle` returns the bare puzzle; `load_puzzle_with_hints`
// returns both the puzzle and any hints embedded in the CSV.
//
// T4 — moved out of `eternity2-benchmark::loader` so non-benchmark
// crates (server, solver-engine examples) can consume it without
// pulling in the benchmark runner.

#![forbid(unsafe_code)]

use std::fs;
use std::path::Path;

use eternity2_core::{Edges, Hint, Hints, Piece, Puzzle, Rotation, BORDER};

#[derive(Debug)]
pub enum LoadError {
    Io(std::io::Error),
    BadFormat(String),
    PuzzleBuild(eternity2_core::PuzzleError),
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io: {e}"),
            Self::BadFormat(s) => write!(f, "format: {s}"),
            Self::PuzzleBuild(e) => write!(f, "puzzle: {e}"),
        }
    }
}

impl std::error::Error for LoadError {}

fn parse_color_word(s: &str) -> Result<u8, LoadError> {
    let value = u32::from_str_radix(s.trim(), 2)
        .map_err(|_| LoadError::BadFormat(format!("invalid 16-bit binary: {s:?}")))?;
    if value == 65535 {
        return Ok(BORDER);
    }
    u8::try_from(value).map_err(|_| LoadError::BadFormat(format!("color {value} doesn't fit u8")))
}

pub fn load_puzzle<P: AsRef<Path>>(path: P) -> Result<Puzzle, LoadError> {
    let (p, _h) = load_puzzle_with_hints(path)?;
    Ok(p)
}

/// Load puzzle and any hint pinnings embedded in the CSV.
///
/// Hint encoding: a piece line whose columns (x, y, rotation) are not
/// all zero is treated as a pinned placement at position
/// `(y * size + x)` with the given rotation. The piece is added to the
/// puzzle normally; the hint is reported separately so callers can
/// decide whether to honour it.
///
/// Exception: piece index 0 with all-zero hint columns is NOT a hint
/// (so the file's "absent hint" convention works).
pub fn load_puzzle_with_hints<P: AsRef<Path>>(path: P) -> Result<(Puzzle, Hints), LoadError> {
    let raw = fs::read_to_string(path.as_ref()).map_err(LoadError::Io)?;
    let mut lines = raw.lines().filter(|l| !l.trim().is_empty());

    let size_line = lines
        .next()
        .ok_or_else(|| LoadError::BadFormat("missing size header".into()))?;
    let size: u32 = size_line
        .trim()
        .parse()
        .map_err(|_| LoadError::BadFormat(format!("bad size {size_line:?}")))?;

    let mut pieces = Vec::with_capacity((size * size) as usize);
    let mut hints: Vec<Hint> = Vec::new();
    let mut max_interior_color: u8 = 0;
    let mut next_id: u16 = 0;
    for line in lines {
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() < 4 {
            return Err(LoadError::BadFormat(format!(
                "piece line has too few columns: {line:?}"
            )));
        }
        let top = parse_color_word(cols[0])?;
        let right = parse_color_word(cols[1])?;
        let bottom = parse_color_word(cols[2])?;
        let left = parse_color_word(cols[3])?;
        for &c in &[top, right, bottom, left] {
            if c != BORDER && c > max_interior_color {
                max_interior_color = c;
            }
        }
        pieces.push(Piece::new(next_id, Edges::new(top, right, bottom, left)));

        if cols.len() >= 7 {
            let x: i64 = cols[4].trim().parse().unwrap_or(0);
            let y: i64 = cols[5].trim().parse().unwrap_or(0);
            let rot_raw: i64 = cols[6].trim().parse().unwrap_or(0);
            let all_zero = x == 0 && y == 0 && rot_raw == 0;
            if !all_zero
                && x >= 0
                && x < i64::from(size)
                && y >= 0
                && y < i64::from(size)
                && (0..=3).contains(&rot_raw)
            {
                let pos = (y as u32) * size + (x as u32);
                let rotation = Rotation::from_u8(rot_raw as u8).ok_or_else(|| {
                    LoadError::BadFormat(format!("bad rotation in hint line {line:?}"))
                })?;
                hints.push(Hint {
                    position: pos,
                    piece_id: next_id,
                    rotation,
                });
            }
        }

        next_id += 1;
    }

    let color_count = u32::from(max_interior_color) + 1;
    let puzzle = Puzzle::new(size, size, color_count, pieces).map_err(LoadError::PuzzleBuild)?;
    Ok((puzzle, Hints::new(hints)))
}
