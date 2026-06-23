// Shared candidate-legality check for the step-through explainers. Given a piece
// (in rotation 0), a target cell, and the current board, decide whether the piece
// could legally sit there in *some* rotation — i.e. it would be one of the green
// "candidates" the solver is choosing between at that cell. This is the same test
// the real engine applies; the explainers use it only to *highlight* which pieces
// are in play, never to drive the search itself (the engine does that).

import { BORDER, rotateEdges } from "./types";
import type { Edges } from "./bucas";

export function pieceFitsAt(
  piece: Edges,
  pos: number,
  cells: (Edges | null)[],
  width: number,
  height: number,
): boolean {
  const x = pos % width;
  const y = Math.floor(pos / width);
  for (let r = 0; r < 4; r++) {
    const e = rotateEdges(piece, r);
    const [top, right, bottom, left] = e;
    if ((y === 0) !== (top === BORDER)) continue;
    if ((y === height - 1) !== (bottom === BORDER)) continue;
    if ((x === 0) !== (left === BORDER)) continue;
    if ((x === width - 1) !== (right === BORDER)) continue;
    const up = y > 0 ? cells[pos - width] : null;
    if (up && up[2] !== top) continue;
    const down = y < height - 1 ? cells[pos + width] : null;
    if (down && down[0] !== bottom) continue;
    const lf = x > 0 ? cells[pos - 1] : null;
    if (lf && lf[1] !== left) continue;
    const rt = x < width - 1 ? cells[pos + 1] : null;
    if (rt && rt[3] !== right) continue;
    return true;
  }
  return false;
}

/** Count how many unused pieces could legally sit at `pos` right now. */
export function countCandidates(
  pieces: Edges[],
  pos: number,
  cells: (Edges | null)[],
  used: Set<number>,
  width: number,
  height: number,
): Set<number> {
  const fitting = new Set<number>();
  pieces.forEach((piece, i) => {
    if (!used.has(i) && pieceFitsAt(piece, pos, cells, width, height)) fitting.add(i);
  });
  return fitting;
}
