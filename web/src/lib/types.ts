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

export const BORDER = 0;

/** Clockwise rotation of URDL edges: new[i] = old[(i + 4 - r) % 4]. */
export function rotateEdges(
  e: readonly [number, number, number, number],
  r: number,
): [number, number, number, number] {
  const k = r & 3;
  return [e[(4 - k) & 3], e[(5 - k) & 3], e[(6 - k) & 3], e[(7 - k) & 3]];
}

/** Edges of a board cell given the engine encoding pieceId*4+rot. */
export function cellEdges(
  puzzle: Puzzle,
  cell: number,
): [number, number, number, number] | null {
  if (cell < 0) return null;
  const piece = puzzle.pieces[cell >> 2];
  if (!piece) return null;
  return rotateEdges(piece, cell & 3);
}

export function maxScore(width: number, height: number): number {
  return 2 * width * height - width - height;
}
