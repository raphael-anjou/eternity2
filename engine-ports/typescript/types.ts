// Core types for the pure-TypeScript engine port (ALGORITHM.md §2).
//
// The domain types live in @/lib/types — they are the camelCase shapes that
// the Rust/WASM engine serde-serializes (numColors, bestPlaced, …). This port
// must produce the *identical* shapes must match the canonical engine — see the port note; here they align at build
// time, so we re-export them here rather than redefine them.
//
// Edge order is always URDL (up, right, down, left), clockwise from the top.
// Colour 0 is the grey border; interior colours are 1..=22. A board cell holds
// `pieceId*4 + rotation`, or -1 when empty (ALGORITHM.md §2.4).

import { rotateEdges, BORDER, type Hint, type Puzzle } from "@/lib/types";

export { rotateEdges, BORDER };
export type { Hint, Puzzle };

/** A piece's four edge colours in rotation 0, URDL. */
export type Edges = [number, number, number, number];

/**
 * Clockwise rotation of URDL edges by `r` quarter-turns (ALGORITHM.md §2.3):
 * `new[i] = old[(i + 4 - r) % 4]`. Thin alias over the shared `rotateEdges`
 * so generator/solver/scorer code reads like engine/src/types.rs (`rotated`).
 */
export function rotated(e: Edges, r: number): Edges {
  return rotateEdges(e, r);
}
