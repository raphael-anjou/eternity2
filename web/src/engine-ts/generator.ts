// Seeded generator for solvable n×n edge-matching puzzles (ALGORITHM.md §4).
//
// Strategy: build a *solved* board first (paint every interior adjacency with a
// random colour, derive each piece from its four surrounding adjacencies), then
// scramble piece order and rotations. The result is solvable by construction
// and fully deterministic for a given (size, colors, seed).
//
// Canonical source: engine/src/generator.rs.

import { XorShift } from "./rng.ts";
import { rotated, BORDER, type Edges, type Puzzle } from "./types.ts";

/** Interior edge count of an n×n board: 2·n·(n−1) (ALGORITHM.md §4 step 1). */
export function interiorEdgeCount(size: number): number {
  return 2 * size * (size - 1);
}

/**
 * Largest usable colour count for a size: `min(2·n·(n−1), 22)`. You cannot ask
 * for more colours than there are interior edges (every colour must appear at
 * least once), nor more than the 22 renderable motifs (ALGORITHM.md §4).
 */
export function maxColors(size: number): number {
  return Math.min(interiorEdgeCount(size), 22);
}

/** Clamp `v` into [lo, hi], matching Rust's `u8::clamp`. */
function clamp(v: number, lo: number, hi: number): number {
  return v < lo ? lo : v > hi ? hi : v;
}

/**
 * Same construction as `generate`, but the pieces stay in solution order and
 * orientation: piece `i` belongs at cell `i` with rotation 0, so the identity
 * board is the solution. Used by the viewer's board generator.
 *
 * Mirrors `generate_solved` in engine/src/generator.rs.
 */
export function generateSolved(size: number, colors: number, seed: number): Puzzle {
  if (size < 2) {
    throw new Error("size must be >= 2");
  }
  const clampedColors = clamp(colors, 1, maxColors(size));
  const s = size;
  const nEdges = interiorEdgeCount(size);
  const rng = new XorShift(seed);

  // Step 1: one colour per interior adjacency, every colour present at least
  // once (the first `colors` entries are 1..colors), then shuffle the palette.
  const palette: number[] = [];
  for (let i = 0; i < nEdges; i++) {
    palette.push(i < clampedColors ? i + 1 : rng.below(clampedColors) + 1);
  }
  rng.shuffle(palette);

  // Step 2: split into vertical and horizontal adjacency colours.
  // vert[y*(s-1)+x] = colour between (x,y) and (x+1,y);
  // horiz[y*s+x]    = colour between (x,y) and (x,y+1).
  const vert = palette.slice(0, s * (s - 1));
  const horiz = palette.slice(s * (s - 1));
  const v = (x: number, y: number): number => vert[y * (s - 1) + x] ?? 0;
  const h = (x: number, y: number): number => horiz[y * s + x] ?? 0;

  // Step 3: derive each piece from its four surrounding adjacencies. Rim sides
  // are BORDER (0).
  const pieces: Edges[] = [];
  for (let y = 0; y < s; y++) {
    for (let x = 0; x < s; x++) {
      const up = y === 0 ? BORDER : h(x, y - 1);
      const down = y === s - 1 ? BORDER : h(x, y);
      const left = x === 0 ? BORDER : v(x - 1, y);
      const right = x === s - 1 ? BORDER : v(x, y);
      pieces.push([up, right, down, left]);
    }
  }

  return {
    name: `generated_${String(size)}x${String(size)}_c${String(clampedColors)}_s${String(seed)}`,
    width: size,
    height: size,
    numColors: clampedColors,
    pieces,
    hints: [],
  };
}

/**
 * Generate a scrambled, solvable puzzle (ALGORITHM.md §4 step 4).
 *
 * Build the solved board, then make a *second* RNG seeded with
 * `seed XOR 0xA5A5A5A5`, shuffle the piece list, and give each piece a random
 * rotation. The two distinct seeds keep the puzzle's shape (`seed`) and its
 * shuffle (`seed XOR 0xA5A5A5A5`) independent.
 *
 * Mirrors `generate` in engine/src/generator.rs.
 */
export function generate(size: number, colors: number, seed: number): Puzzle {
  const puzzle = generateSolved(size, colors, seed);
  // `^` is signed 32-bit in JS; `>>> 0` brings it back to an unsigned u32 so
  // the XorShift seed matches Rust's `seed ^ 0xA5A5_A5A5`.
  const rng = new XorShift((seed ^ 0xa5a5a5a5) >>> 0);
  rng.shuffle(puzzle.pieces);
  for (let i = 0; i < puzzle.pieces.length; i++) {
    puzzle.pieces[i] = rotated(puzzle.pieces[i] as Edges, rng.below(4));
  }
  return puzzle;
}
