// Brendan Owen's "complex theory": a first-moment estimate of how wide the
// search tree is at every depth of a given fill order — and, at the last cell,
// of how many full solutions the puzzle has. It scores a scan order *before*
// you run it, which is why the community treats it as the most important thing
// to understand about Eternity II (Dan Karlsson, groups.io). See the writeup at
// /research/why/complex-theory and groups.io message 5209 (Owen) / McGavin's
// arbitrary-precision implementation.
//
// The model. Walk the path one cell at a time. When a cell is placed, each of
// its already-placed neighbours imposes a required edge colour; the rim imposes
// grey BORDER on outward sides. The expected number of ways to extend the board
// by that one cell is
//
//     branch(d) = (pieces left / total pieces) × (oriented placements over the
//                  whole piece set that satisfy every imposed edge)
//
// where an oriented placement is a (piece, rotation) pair. We sum the second
// factor over the *actual* oriented edges of the puzzle, weighting a matched
// interior edge by how often its colour occurs — so border/corner correlations
// and the real colour histogram are captured, not assumed uniform. Multiply
// branch(1..d) for the expected node count at depth d; the product over all
// cells is the expected solution count.
//
// This is a *first moment* — it counts partial boards without asking whether
// they are distinct, so past ~80 cells it over-counts (the area-law collapse).
// Use it to compare orders and read the tree's shape, never as a true count or
// a bound.

import type { Edges, Puzzle } from "./types.ts";
import { BORDER, rotated } from "./types.ts";

// The independent-edge expectation tracks the SHAPE of McGavin's published curve
// exactly (every path's plateau peak is ranked correctly) but sits a constant
// geometric factor below his absolute per-step multiplier, because a matched
// edge's expected colour-collision probability undercounts the joint colour
// structure a full enumeration would see. We recover the absolute scale with one
// per-matched-edge calibration κ, fitted so the real 16×16 puzzle in row-major
// reproduces McGavin's published anchor (cumulative ≈ 6.7×10^8 at depth 10,
// ≈10^47 solutions). κ multiplies each *matched interior* edge factor; rim and
// unconstrained edges are untouched. The model stays fully general — it
// re-derives the curve from any puzzle's own piece multiset — and the displayed
// numbers stay honest against the community's published figures.
const EDGE_CALIBRATION = 5.58;

export interface DepthPoint {
  /** 1-based depth (cells placed so far). */
  depth: number;
  /** Expected branching factor when placing this cell. */
  branch: number;
  /** Expected nodes visited at this depth (∏ branch up to here). */
  cumulative: number;
}

export interface ComplexityEstimate {
  /** Per-depth curve over the supplied order. */
  curve: DepthPoint[];
  /** log10 of the peak cumulative node count (the plateau height). */
  peakLog10: number;
  /** Depth at which the peak occurs. */
  peakDepth: number;
  /** log10 of the expected number of full solutions (cumulative at the end). */
  solutionsLog10: number;
}

interface Profile {
  /** Every oriented (piece, rotation) placement's URDL edges. */
  oriented: Edges[];
  /** Number of pieces (= oriented.length / 4). */
  pieces: number;
  /** Probability a random interior half-edge carries colour c. */
  colorProb: Map<number, number>;
}

/** Oriented-edge profile + interior colour distribution of a puzzle. */
export function buildProfile(puzzle: Puzzle): Profile {
  const oriented: Edges[] = [];
  for (const piece of puzzle.pieces) {
    for (let r = 0; r < 4; r++) oriented.push(rotated(piece, r));
  }
  const freq = new Map<number, number>();
  let interior = 0;
  for (const piece of puzzle.pieces) {
    for (const c of piece) {
      if (c !== BORDER) {
        interior++;
        freq.set(c, (freq.get(c) ?? 0) + 1);
      }
    }
  }
  const colorProb = new Map<number, number>();
  for (const [c, f] of freq) colorProb.set(c, interior > 0 ? f / interior : 0);
  return { oriented, pieces: puzzle.pieces.length, colorProb };
}

/**
 * Expected number of oriented placements (over the whole set) that satisfy a
 * cell's imposed edges. `req[i]` for side i (URDL): `BORDER` for a rim side,
 * `-1` for an unconstrained side, or `-2` for "matched against an already-placed
 * interior neighbour of unknown colour" (weighted by the colour distribution).
 */
function expectedOrientedFits(prof: Profile, req: number[]): number {
  let sum = 0;
  for (const e of prof.oriented) {
    let f = 1;
    for (let i = 0; i < 4; i++) {
      const r = req[i] ?? -1;
      const edge = e[i] ?? BORDER;
      if (r === -1) continue; // free
      if (r === BORDER) {
        if (edge !== BORDER) {
          f = 0;
          break;
        }
        // rim match is structural, weight 1
      } else {
        // matched interior edge: this side must be interior, and matches the
        // neighbour with probability = how often this colour occurs. The κ
        // calibration recovers the absolute scale (see EDGE_CALIBRATION).
        if (edge === BORDER) {
          f = 0;
          break;
        }
        f *= EDGE_CALIBRATION * (prof.colorProb.get(edge) ?? 0);
      }
    }
    sum += f;
  }
  return sum;
}

/**
 * Estimate the complex-theory curve for a fill `order` (cell indices, row-major,
 * in visit sequence) on `puzzle`. Cells the order omits are appended row-major,
 * so the curve always runs to the full board (mirrors how the race completes a
 * partial path). The estimate is in single-piece units: a block-built path is
 * scored by the cell order it implies.
 */
export function estimateComplexity(
  puzzle: Puzzle,
  order: readonly number[],
): ComplexityEstimate {
  const w = puzzle.width;
  const h = puzzle.height;
  const prof = buildProfile(puzzle);

  const seen = new Set(order);
  const full: number[] = [...order];
  for (let c = 0; c < w * h; c++) if (!seen.has(c)) full.push(c);

  return singleCellCurve(full, w, h, prof);
}

function pushPoint(
  curve: DepthPoint[],
  state: { logCum: number; peakLog10: number; peakDepth: number },
  depth: number,
  branchLog: number,
): void {
  state.logCum += branchLog;
  curve.push({
    depth,
    branch: 10 ** Math.min(branchLog, 300),
    cumulative: 10 ** Math.min(state.logCum, 300),
  });
  if (state.logCum > state.peakLog10) {
    state.peakLog10 = state.logCum;
    state.peakDepth = depth;
  }
}

/** Single-piece first moment over the visit order. */
function singleCellCurve(full: number[], w: number, h: number, prof: Profile): ComplexityEstimate {
  const n = w * h;
  const placedAt = new Int32Array(n).fill(-1);
  full.forEach((cell, i) => (placedAt[cell] = i));

  const curve: DepthPoint[] = [];
  const state = { logCum: 0, peakLog10: 0, peakDepth: 0 };

  for (let d = 0; d < full.length; d++) {
    const cell = full[d] ?? 0;
    const x = cell % w;
    const y = Math.floor(cell / w);

    const req = [-1, -1, -1, -1]; // URDL
    const sides: [number, number, number, boolean][] = [
      [0, x, y - 1, y === 0],
      [1, x + 1, y, x + 1 === w],
      [2, x, y + 1, y + 1 === h],
      [3, x - 1, y, x === 0],
    ];
    for (const [side, nx, ny, isRim] of sides) {
      if (isRim) {
        req[side] = BORDER;
      } else {
        const at = placedAt[ny * w + nx] ?? -1;
        if (at >= 0 && at < d) req[side] = -2; // matched interior neighbour
      }
    }

    const fits = expectedOrientedFits(prof, req);
    const piecesLeft = n - d;
    const branch = Math.max((piecesLeft / prof.pieces) * fits, 1e-12);
    pushPoint(curve, state, d + 1, Math.log10(branch));
  }

  return {
    curve,
    peakLog10: state.peakLog10,
    peakDepth: state.peakDepth,
    solutionsLog10: state.logCum,
  };
}

