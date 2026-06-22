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
 * Number of frame-restricted colours in framed mode: `min(5, colors - 1)`,
 * mirroring real Eternity II (5 of 22 colours appear only on frame-adjacent
 * edges). Colours `1..=frameCount` are frame colours; the rest are interior.
 * Mirrors `frame_color_count` in engine/src/generator.rs.
 */
export function frameColorCount(colors: number): number {
  return colors < 2 ? 0 : Math.min(colors - 1, 5);
}

/**
 * The framed split is only meaningful when there is at least one interior
 * colour (`colors >= 2`) AND at least one deep-interior adjacency to confine it
 * to (which needs `size >= 4`, since a deep-interior adjacency joins two off-rim
 * cells). Below this threshold framed mode falls back to the unrestricted path
 * so every requested colour still appears. Mirrors `framed_is_meaningful`.
 */
function framedIsMeaningful(size: number, colors: number): boolean {
  return size >= 4 && colors >= 2;
}

/**
 * Is palette slot `i` (default flat order: `i < s*(s-1)` is `vert[i]`, else
 * `horiz[i - s*(s-1)]`) a frame-band adjacency — at least one incident cell on
 * the outer ring? Mirrors `slot_is_frame_band` in engine/src/generator.rs.
 */
function slotIsFrameBand(i: number, s: number): boolean {
  const vcount = s * (s - 1);
  if (i < vcount) {
    const x = i % (s - 1);
    const y = Math.floor(i / (s - 1));
    return y === 0 || y === s - 1 || x === 0 || x + 1 === s - 1;
  }
  const j = i - vcount;
  const x = j % s;
  const y = Math.floor(j / s);
  return x === 0 || x === s - 1 || y === 0 || y + 1 === s - 1;
}

/**
 * Paint each palette slot in framed mode (frame band gets colours
 * `1..=frameCount`; deep interior gets `frameCount+1..=colors`, each group
 * guaranteeing every colour appears, then shuffled within the group). Only
 * `below`/`shuffle` are used so every port matches. Mirrors `paint_framed`.
 */
function paintFramed(rng: XorShift, nEdges: number, s: number, colors: number): number[] {
  const frameCount = frameColorCount(colors); // >= 1 here (colors >= 2)
  const interiorCount = colors - frameCount; // >= 1 here (size >= 4)

  const frameSlots: number[] = [];
  const interiorSlots: number[] = [];
  for (let i = 0; i < nEdges; i++) {
    if (slotIsFrameBand(i, s)) frameSlots.push(i);
    else interiorSlots.push(i);
  }

  const palette: number[] = new Array<number>(nEdges).fill(0);

  const frameColors: number[] = [];
  for (let k = 0; k < frameSlots.length; k++) {
    frameColors.push(k < frameCount ? k + 1 : rng.below(frameCount) + 1);
  }
  rng.shuffle(frameColors);
  for (let k = 0; k < frameSlots.length; k++) {
    palette[frameSlots[k] ?? 0] = frameColors[k] ?? 0;
  }

  const interiorColors: number[] = [];
  for (let k = 0; k < interiorSlots.length; k++) {
    interiorColors.push(
      k < interiorCount ? frameCount + k + 1 : frameCount + rng.below(interiorCount) + 1,
    );
  }
  rng.shuffle(interiorColors);
  for (let k = 0; k < interiorSlots.length; k++) {
    palette[interiorSlots[k] ?? 0] = interiorColors[k] ?? 0;
  }

  return palette;
}

/**
 * Same construction as `generate`, but the pieces stay in solution order and
 * orientation: piece `i` belongs at cell `i` with rotation 0, so the identity
 * board is the solution. Used by the viewer's board generator.
 *
 * Mirrors `generate_solved` in engine/src/generator.rs.
 */
export function generateSolved(size: number, colors: number, seed: number): Puzzle {
  return generateSolvedFramed(size, colors, seed, false);
}

/**
 * Framed-capable solved-board generator. When `framed` is true (and the split is
 * meaningful), frame colours `1..=frameCount` are confined to frame-band
 * adjacencies and interior colours to the deep interior. When false, this is
 * byte-for-byte identical to the original `generateSolved`.
 *
 * Mirrors `generate_solved_framed` in engine/src/generator.rs.
 */
export function generateSolvedFramed(
  size: number,
  colors: number,
  seed: number,
  framed: boolean,
): Puzzle {
  if (size < 2) {
    throw new Error("size must be >= 2");
  }
  const clampedColors = clamp(colors, 1, maxColors(size));
  const s = size;
  const nEdges = interiorEdgeCount(size);
  const rng = new XorShift(seed);

  // Step 1: paint one colour per interior adjacency. Framed mode confines each
  // colour to its band; otherwise every colour is present at least once (the
  // first `colors` entries are 1..colors) and the palette is shuffled.
  let palette: number[];
  if (framed && framedIsMeaningful(size, clampedColors)) {
    palette = paintFramed(rng, nEdges, s, clampedColors);
  } else {
    palette = [];
    for (let i = 0; i < nEdges; i++) {
      palette.push(i < clampedColors ? i + 1 : rng.below(clampedColors) + 1);
    }
    rng.shuffle(palette);
  }

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
  return generateFramed(size, colors, seed, false);
}

/**
 * Framed-capable scrambled generator. `framed=false` is byte-for-byte identical
 * to `generate`; `framed=true` confines frame-restricted colours to the border
 * band (see `generateSolvedFramed`). Mirrors `generate_framed` in
 * engine/src/generator.rs.
 */
export function generateFramed(
  size: number,
  colors: number,
  seed: number,
  framed: boolean,
): Puzzle {
  const puzzle = generateSolvedFramed(size, colors, seed, framed);
  // `^` is signed 32-bit in JS; `>>> 0` brings it back to an unsigned u32 so
  // the XorShift seed matches Rust's `seed ^ 0xA5A5_A5A5`.
  const rng = new XorShift((seed ^ 0xa5a5a5a5) >>> 0);
  rng.shuffle(puzzle.pieces);
  for (let i = 0; i < puzzle.pieces.length; i++) {
    puzzle.pieces[i] = rotated(puzzle.pieces[i] as Edges, rng.below(4));
  }
  return puzzle;
}
