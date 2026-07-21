// Shared domain types, mirroring the Rust engine's serde output.
// Edge order is always URDL (up, right, down, left); color 0 is the grey
// border; interior colors 1..22 map to bucas letters 'b'..'w'.

export interface Hint {
  pos: number;
  piece: number;
  rot: number;
}

export interface Puzzle {
  name: string;
  width: number;
  height: number;
  numColors: number;
  /** Piece edges in rotation 0, URDL. */
  pieces: [number, number, number, number][];
  hints: Hint[];
}

/** cell -> pieceId*4+rotation, or -1 while empty (engine board encoding). */
export type BoardCells = Int32Array | number[];

export interface SolverReport {
  status: "running" | "solved" | "exhausted";
  nodes: number;
  attempts: number;
  backtracks: number;
  placed: number;
  bestPlaced: number;
}

/** The step-able solver surface the engine exposes. The Rust/WASM engine
 * returns one of these; the reimplementations in `engine-ports/` match it. */
export interface SolverHandle {
  /** Run up to `budget` placements/backtracks, then report. */
  step(budget: number): SolverReport;
  report(): SolverReport;
  /** cell -> pieceId*4+rotation, or -1 while empty. */
  board(): Int32Array;
  /** Deepest board ever reached (by placed count). */
  bestBoard(): Int32Array;
  score(): number;
  bestScore(): number;
  reset(): void;
  /** Release native/WASM resources (a no-op for the pure-TS backend). */
  free(): void;
}

/** Break-tolerant solver surface (Rust/WASM backend only). Same step-able shape
 * as SolverHandle plus a live `breaks` count; it completes a full board within a
 * mismatch budget instead of stopping at the first imperfection. */
export interface BreakSolverHandle {
  step(budget: number): SolverReport;
  report(): SolverReport;
  board(): Int32Array;
  bestBoard(): Int32Array;
  score(): number;
  breaks(): number;
  reset(): void;
  free(): void;
}

export const BORDER = 0;

/** Clockwise rotation of URDL edges: new[i] = old[(i + 4 - r) % 4].
 * Destructured rather than index-computed so the result is a provably total
 * 4-tuple under noUncheckedIndexedAccess (no `T | undefined` element reads). */
export function rotateEdges(
  e: readonly [number, number, number, number],
  r: number,
): [number, number, number, number] {
  const [u, ri, d, l] = e;
  switch (r & 3) {
    case 1:
      return [l, u, ri, d];
    case 2:
      return [d, l, u, ri];
    case 3:
      return [ri, d, l, u];
    default:
      return [u, ri, d, l];
  }
}

export function maxScore(width: number, height: number): number {
  return 2 * width * height - width - height;
}
