// Forbidden 2×2 patches (vault concept `intaglio-forbidden-patterns`, V138-139).
//
// A 2×2 window of four placed pieces is "forbidden" when NO assignment of
// rotations to those four pieces makes the square's four internal edges all
// match. ~99.72% of random interior 4-tuples are forbidden; a *valid* full board
// has zero. The count of forbidden windows in a real board is a secondary
// progress signal: it falls monotonically as the matched-edge score rises
// (verified live — 470 boards ≈18, 467 boards ≈23 forbidden of 225 windows),
// and a 480 solution reaches 0.
//
// This is the single source of the computation used by the live demo.

import { rotateEdges } from "@/lib/types";
import type { Edges } from "@/lib/bucas";

/**
 * Can the four pieces at a 2×2 window (top-left, top-right, bottom-left,
 * bottom-right, each in rotation 0) be rotated so the square's four internal
 * edges all match? Brute-force over 4⁴ rotation tuples, pruning early on each
 * internal edge. A window where this returns false is "forbidden".
 */
export function patchFeasible(tl: Edges, tr: Edges, bl: Edges, br: Edges): boolean {
  for (let a = 0; a < 4; a++) {
    const A = rotateEdges(tl, a);
    for (let b = 0; b < 4; b++) {
      const B = rotateEdges(tr, b);
      if (A[1] !== B[3]) continue; // TL.right == TR.left
      for (let c = 0; c < 4; c++) {
        const C = rotateEdges(bl, c);
        if (A[2] !== C[0]) continue; // TL.down == BL.up
        for (let d = 0; d < 4; d++) {
          const D = rotateEdges(br, d);
          if (C[1] !== D[3]) continue; // BL.right == BR.left
          if (B[2] !== D[0]) continue; // TR.down == BR.up
          return true;
        }
      }
    }
  }
  return false;
}

export interface ForbiddenScan {
  /** Number of forbidden 2×2 windows. */
  forbidden: number;
  /** Total fully-placed 2×2 windows examined. */
  total: number;
  /** Top-left cell index of each forbidden window. */
  windows: number[];
}

/**
 * Scan every fully-placed 2×2 window of an n-wide board (cells as Edges|null,
 * row-major) and report which are forbidden. Windows touching an empty cell are
 * skipped (a partial board can't be judged there).
 */
export function scanForbidden(cells: (Edges | null)[], width: number, height: number): ForbiddenScan {
  const at = (x: number, y: number): Edges | null => cells[y * width + x] ?? null;
  let forbidden = 0;
  let total = 0;
  const windows: number[] = [];
  for (let y = 0; y < height - 1; y++) {
    for (let x = 0; x < width - 1; x++) {
      const tl = at(x, y);
      const tr = at(x + 1, y);
      const bl = at(x, y + 1);
      const br = at(x + 1, y + 1);
      if (!tl || !tr || !bl || !br) continue;
      total++;
      if (!patchFeasible(tl, tr, bl, br)) {
        forbidden++;
        windows.push(y * width + x);
      }
    }
  }
  return { forbidden, total, windows };
}
