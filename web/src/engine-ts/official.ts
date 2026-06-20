// The official Eternity II piece set, parsed from the embedded CSV
// (ALGORITHM.md §5).
//
// CSV format: line 1 = board size; then one piece per line as
// `top,right,bottom,left,x,y,rotation` where each colour is a 16-bit binary
// word (65535 = grey border = colour 0). A piece whose (x,y,rotation) columns
// are not all zero is one of the five clue/hint pieces, pinned at position
// y*size+x with the given clockwise rotation.
//
// Canonical source: engine/src/official.rs.

import { OFFICIAL_CSV } from "./official_data.ts";
import { BORDER, type Edges, type Hint, type Puzzle } from "./types.ts";

/**
 * Parse one 16-bit binary colour word. `65535` (all ones) is the grey border,
 * i.e. colour 0; otherwise the value is the colour id directly.
 * Mirrors `parse_color_word` in engine/src/official.rs.
 */
function parseColorWord(s: string): number {
  const v = Number.parseInt(s.trim(), 2);
  if (Number.isNaN(v)) {
    throw new Error("invalid binary color word in official CSV");
  }
  if (v === 65535) {
    return BORDER;
  }
  if (v < 0 || v > 255) {
    throw new Error("color out of u8 range in official CSV");
  }
  return v;
}

/** Parse a base-10 column, defaulting to 0 (Rust's `parse().unwrap_or(0)`). */
function parseIntOrZero(s: string): number {
  const v = Number.parseInt(s.trim(), 10);
  return Number.isNaN(v) ? 0 : v;
}

/**
 * Return the real 16×16 puzzle (256 pieces, 22 colours, 5 hints).
 * Mirrors `official_puzzle` in engine/src/official.rs.
 */
export function officialPuzzle(): Puzzle {
  // Filter blank lines, matching Rust's `.filter(|l| !l.trim().is_empty())`.
  const lines = OFFICIAL_CSV.split("\n").filter((l) => l.trim().length > 0);
  const sizeLine = lines[0];
  if (sizeLine === undefined) {
    throw new Error("official CSV is empty");
  }
  const size = Number.parseInt(sizeLine.trim(), 10);

  const pieces: Edges[] = [];
  const hints: Hint[] = [];
  let maxColor = 0;

  // Remaining lines are pieces; `id` is the 0-based piece index (line order).
  for (let id = 0; id < lines.length - 1; id++) {
    const line = lines[id + 1];
    if (line === undefined) {
      continue;
    }
    const cols = line.split(",");
    const edges: Edges = [
      parseColorWord(cols[0] ?? ""),
      parseColorWord(cols[1] ?? ""),
      parseColorWord(cols[2] ?? ""),
      parseColorWord(cols[3] ?? ""),
    ];
    for (const c of edges) {
      if (c > maxColor) {
        maxColor = c;
      }
    }
    pieces.push(edges);

    if (cols.length >= 7) {
      const x = parseIntOrZero(cols[4] ?? "");
      const y = parseIntOrZero(cols[5] ?? "");
      const rot = parseIntOrZero(cols[6] ?? "");
      if (x !== 0 || y !== 0 || rot !== 0) {
        hints.push({ pos: y * size + x, piece: id, rot });
      }
    }
  }

  return {
    name: "official_eternity2",
    width: size,
    height: size,
    numColors: maxColor,
    pieces,
    hints,
  };
}
