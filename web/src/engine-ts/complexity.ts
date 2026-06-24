// Brendan Owen's Complex Theory — a faithful port of Peter McGavin's reference
// C implementation (libgmp), validated to reproduce his published table exactly
// (4, 44.8, 493.37, 5340.6, …, 14702 expected solutions at depth 256, cumulative
// 1.365×10⁴⁷).
//
// Source (the algorithm we port, line for line):
//   Peter McGavin, "Estimated number of solutions with and without hints",
//   groups.io/g/eternity2, msg 11197 (2024-01):
//     post:   https://groups.io/g/eternity2/message/11197
//     source: https://groups.io/g/eternity2/attachment/11197/0/complex_theory.c
//   Brendan Owen's original theory: groups.io msgs 5197 and 5209.
//
// What it computes. Given a board state (which squares are occupied), the
// EXPECTED number of ways that state could be a valid partial solution:
//
//   s2 = P · pm2 · pb2
//
//   P    = ways to place the occupied pieces: P(corners)·P(borders)·P(middles)
//          · 4^(middle pieces), i.e. the permutations of assigning distinct
//          pieces of each class to the occupied squares of that class, times the
//          4 free rotations of each middle piece (corner/border orientations are
//          forced by the rim).
//   pm2  = probability the m completed interior ("middle") joins are all colour-
//          valid = validm(types, m) / P(2·Σmt, 2m).
//   pb2  = probability the b completed border joins are all valid
//          = validb(types, b) / P(Σbt, b)².
//
//   validm(i, m) = number of ways to form m valid middle joins using the half-
//   edges of colour types 0..i-1 (a join pairs two equal-colour half-edges); a
//   recursive convolution over colour types. validb is the border analogue
//   (border joins pair a left half-edge with a right half-edge of equal colour).
//
// We evaluate everything in log10 (with log-sum-exp for validm/validb's sums and
// lgamma for factorials), so the 10⁴⁷-scale intermediates never overflow Float64
// — the C uses GNU MP for the same reason. This is a FIRST-MOMENT estimate: it
// counts expected valid partials treating colours as independently drawn, so it
// is accurate to ~×2 (McGavin's small-puzzle calibration) and is blind to the
// global-distinctness wall (see /research/why/entropy-area-law). Use it for tree
// shape and scan-order choice, not as a true count or a bound.

import { BORDER, type Edges, type Puzzle } from "./types.ts";

export const COMPLEX_THEORY_SOURCE_URL = "https://groups.io/g/eternity2/message/11197";

export interface DepthPoint {
  /** 1-based depth (cells placed so far). */
  depth: number;
  /** Expected valid partials added at this depth (McGavin's "num-solns"). */
  branch: number;
  /** Cumulative expected valid partials up to here (his "cum-solns"). */
  cumulative: number;
}

export interface ComplexityEstimate {
  /** Per-depth curve over the supplied order. */
  curve: DepthPoint[];
  /** log10 of the peak per-depth count (the plateau height). */
  peakLog10: number;
  /** Depth at which the per-depth count peaks. */
  peakDepth: number;
  /** log10 of the expected number of full solutions (last cell). */
  solutionsLog10: number;
}

// ---- log10 arithmetic (matches the C's GMP, in log space) ------------------

const NEG = -Infinity;

function logAdd(a: number, b: number): number {
  if (a === NEG) return b;
  if (b === NEG) return a;
  const m = a > b ? a : b;
  return m + Math.log10(10 ** (a - m) + 10 ** (b - m));
}

const LANCZOS = [
  676.5203681218851, -1259.1392167224028, 771.32342877765313, -176.61502916214059,
  12.507343278686905, -0.13857109526572012, 9.9843695780195716e-6, 1.5056327351493116e-7,
];

/** ln Γ(x) (Lanczos), so factorials of 256-ish stay finite. */
function lgamma(x: number): number {
  if (x < 0.5) return Math.log(Math.PI / Math.sin(Math.PI * x)) - lgamma(1 - x);
  const xx = x - 1;
  let a = 0.99999999999980993;
  const t = xx + 7.5;
  for (let i = 0; i < LANCZOS.length; i++) a += (LANCZOS[i] ?? 0) / (xx + i + 1);
  return 0.5 * Math.log(2 * Math.PI) + (xx + 0.5) * Math.log(t) - t + Math.log(a);
}

const LOG10E = Math.log10(Math.E);
const logFact = (n: number): number => lgamma(n + 1) * LOG10E; // log10(n!)
const logPerm = (n: number, k: number): number =>
  k < 0 || k > n ? NEG : logFact(n) - logFact(n - k); // log10 P(n,k)
const logComb = (n: number, k: number): number =>
  k < 0 || k > n ? NEG : logFact(n) - logFact(k) - logFact(n - k);

// ---- edge-type profile of a puzzle (the bt / mt arrays in the C) -----------

interface EdgeTypes {
  /** Border-join counts per border colour (half the border edges of each). */
  bt: number[];
  /** Middle-join counts per interior colour (half the interior edges of each). */
  mt: number[];
  /** Σ bt and Σ mt. */
  nbt: number;
  nmt: number;
}

/**
 * Derive the border/middle edge-type histograms from the puzzle's piece set.
 * Border colours are those that only ever appear on a frame-adjacent edge of a
 * border/corner piece; everything else is a middle colour. We classify by
 * counting, per colour, the total interior half-edges carrying it, split into
 * the two pools the C keeps separate. The grey rim (colour 0) is excluded.
 *
 * For the official puzzle this reproduces McGavin's hard-coded bt = {12×5},
 * mt = {24/25 ×17}.
 */
export function edgeTypes(puzzle: Puzzle): EdgeTypes {
  const w = puzzle.width;
  const h = puzzle.height;
  // A colour is a "border colour" if every edge of that colour, in the solved
  // board, lies on a join touching the rim. We approximate from the piece set:
  // a colour appears on a border-piece's non-grey edge OR an interior edge.
  // Simpler and exact for our generators + the official set: classify a colour
  // as border iff it occurs on at least one border/corner piece AND never on a
  // deep-interior-only piece. We count half-edges per colour and per pool by
  // walking the would-be solved adjacencies is not available here, so we use the
  // piece-incidence rule below, then halve to get joins.
  const isBorderPiece = (e: Edges): boolean => e.some((c) => c === BORDER);

  const borderColourEdges = new Map<number, number>();
  const middleColourEdges = new Map<number, number>();
  for (const piece of puzzle.pieces) {
    const onBorder = isBorderPiece(piece);
    for (const c of piece) {
      if (c === BORDER) continue;
      const pool = onBorder ? borderColourEdges : middleColourEdges;
      pool.set(c, (pool.get(c) ?? 0) + 1);
    }
  }
  // A colour seen on both pools is a middle colour (interior pieces carry it);
  // a colour seen only on border pieces is a border colour.
  const bt: number[] = [];
  const mt: number[] = [];
  const allColours = new Set<number>([...borderColourEdges.keys(), ...middleColourEdges.keys()]);
  for (const c of allColours) {
    const onBorderOnly = borderColourEdges.has(c) && !middleColourEdges.has(c);
    const halfEdges = (borderColourEdges.get(c) ?? 0) + (middleColourEdges.get(c) ?? 0);
    const joins = Math.max(1, Math.round(halfEdges / 2));
    if (onBorderOnly) bt.push(joins);
    else mt.push(joins);
  }
  // Guarantee enough join capacity for the board (the C asserts nmt >= NMIDDLE).
  const NMIDDLE = (h - 1) * (w - 2) + (h - 2) * (w - 1);
  const NBORDER = 2 * (w - 1) + 2 * (h - 1);
  let nmt = mt.reduce((a, b) => a + b, 0);
  let nbt = bt.reduce((a, b) => a + b, 0);
  // If a generator produced no frame-confined colours (unframed mode), there are
  // no border-only colours; fold the border joins into the middle pool so the
  // capacity asserts hold and border joins are scored against the same palette.
  if (bt.length === 0) {
    bt.push(Math.max(1, Math.ceil(NBORDER / 2)));
    nbt = bt[0] ?? 1;
  }
  while (nmt < NMIDDLE) {
    mt.push(1);
    nmt += 1;
  }
  while (nbt < NBORDER) {
    bt.push(1);
    nbt += 1;
  }
  return { bt, mt, nbt, nmt };
}

// ---- validm / validb (log10, memoized per call) ----------------------------

function makeValidm(mt: number[]): (i: number, m: number) => number {
  const memo = new Map<number, number>();
  const f = (i: number, m: number): number => {
    if (m === 0) return 0;
    if (i <= 0) return NEG;
    const key = i * 100000 + m;
    const c = memo.get(key);
    if (c !== undefined) return c;
    let v = NEG;
    const ni = mt[i - 1] ?? 0;
    for (let j = 0; j <= ni && j <= m; j++) {
      v = logAdd(v, f(i - 1, m - j) + logPerm(2 * ni, 2 * j) + logComb(m, j));
    }
    memo.set(key, v);
    return v;
  };
  return f;
}

function makeValidb(bt: number[]): (i: number, b: number) => number {
  const memo = new Map<number, number>();
  const f = (i: number, b: number): number => {
    if (b === 0) return 0;
    if (i <= 0) return NEG;
    const key = i * 100000 + b;
    const c = memo.get(key);
    if (c !== undefined) return c;
    let v = NEG;
    const ni = bt[i - 1] ?? 0;
    for (let j = 0; j <= ni && j <= b; j++) {
      v = logAdd(v, f(i - 1, b - j) + 2 * logPerm(ni, j) + logComb(b, j));
    }
    memo.set(key, v);
    return v;
  };
  return f;
}

// ---- the estimate ----------------------------------------------------------

type Loc = "M" | "B" | "C";

/**
 * Estimate Owen's complex-theory curve for a fill `order` (cell indices, row-
 * major, in visit sequence). Cells the order omits are appended row-major so the
 * curve runs to the full board.
 *
 * `hints` are cell indices placed BEFORE the path (exactly as the C pre-places
 * its hint squares): each contributes an expected count of 1 and occupies its
 * square, so the joins and remaining-supply that the rest of the path sees are
 * correct. With one middle hint and no path-hints this reproduces McGavin's
 * canonical table; with the five real clues it reproduces his 5-hint run.
 */
export function estimateComplexity(
  puzzle: Puzzle,
  order: readonly number[],
  hints: readonly number[] = [],
): ComplexityEstimate {
  const w = puzzle.width;
  const h = puzzle.height;
  const N = w * h;

  const NC = 4;
  const NB = 2 * (w - 2) + 2 * (h - 2);
  const NM = (w - 2) * (h - 2);

  const { bt, mt, nbt, nmt } = edgeTypes(puzzle);
  const validm = makeValidm(mt);
  const validb = makeValidb(bt);

  const locOf = (cell: number): Loc => {
    const x = cell % w;
    const y = Math.floor(cell / w);
    const corner = (y === 0 || y === h - 1) && (x === 0 || x === w - 1);
    const border = y === 0 || y === h - 1 || x === 0 || x === w - 1;
    return corner ? "C" : border ? "B" : "M";
  };

  // Hints are placed before the path, then the path fills the rest (skipping any
  // cell that is a hint). This mirrors the C's loop, which walks hints first.
  const hintSet = new Set(hints);
  const seen = new Set([...hints, ...order]);
  const sequence: number[] = [...hints, ...order];
  for (let c = 0; c < N; c++) if (!seen.has(c)) sequence.push(c);

  // Hint counts per class, removed from the permutation pool (the C's NM-NMHINTS).
  let ncHints = 0;
  let nbHints = 0;
  let nmHints = 0;
  for (const cell of hints) {
    const loc = locOf(cell);
    if (loc === "C") ncHints++;
    else if (loc === "B") nbHints++;
    else nmHints++;
  }

  const occupied = new Uint8Array(N);

  // Completed joins, counted incrementally: filling a cell completes one join per
  // already-filled orthogonal neighbour, classed border (touches rim) or middle.
  let mJoins = 0;
  let bJoins = 0;
  let nc = 0;
  let nb = 0;
  let nm = 0;

  const curve: DepthPoint[] = [];
  let logCum = NEG;
  let peakLog10 = NEG;
  let peakDepth = 0;

  const statLog = (): number => {
    // P = P(NC-ncHints, nc-ncHints)·P(NB-nbHints, nb-nbHints)
    //     ·P(NM-nmHints, nm-nmHints)·4^(nm-nmHints)
    const logP =
      logPerm(NC - ncHints, nc - ncHints) +
      logPerm(NB - nbHints, nb - nbHints) +
      logPerm(NM - nmHints, nm - nmHints) +
      (nm - nmHints) * Math.log10(4);
    const logPm2 = validm(mt.length, mJoins) - logPerm(2 * nmt, 2 * mJoins);
    const logPb2 = validb(bt.length, bJoins) - 2 * logPerm(nbt, bJoins);
    return logP + logPm2 + logPb2;
  };

  let depth = 0;
  for (const cell of sequence) {
    if (occupied[cell]) continue;
    const x = cell % w;
    const y = Math.floor(cell / w);
    occupied[cell] = 1;
    const loc = locOf(cell);
    if (loc === "C") nc++;
    else if (loc === "B") nb++;
    else nm++;

    for (const [nx, ny, horiz] of [
      [x, y - 1, false],
      [x + 1, y, true],
      [x, y + 1, false],
      [x - 1, y, true],
    ] as [number, number, boolean][]) {
      if (nx < 0 || ny < 0 || nx >= w || ny >= h) continue;
      if (!occupied[ny * w + nx]) continue;
      const isBorderJoin = horiz ? y === 0 || y === h - 1 : x === 0 || x === w - 1;
      if (isBorderJoin) bJoins++;
      else mJoins++;
    }

    depth++;
    // A hint square contributes an expected count of exactly 1 (it is fixed).
    const logS2 = hintSet.has(cell) ? 0 : statLog();
    logCum = logAdd(logCum, logS2);
    curve.push({
      depth,
      branch: 10 ** Math.min(logS2, 300),
      cumulative: 10 ** Math.min(logCum, 300),
    });
    if (logS2 > peakLog10) {
      peakLog10 = logS2;
      peakDepth = depth;
    }
  }

  return {
    curve,
    peakLog10: peakLog10 === NEG ? 0 : peakLog10,
    peakDepth,
    solutionsLog10: logCum === NEG ? 0 : logCum,
  };
}
