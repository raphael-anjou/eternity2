// Board verification, in the spirit of the checks e2.bucas.name runs:
// is this the official piece set, are pieces duplicated, which clues are
// respected. Works on any decoded/displayed board.

import type { BucasBoard, Edges } from "./bucas";
import { rotateEdges } from "./types";
import type { Puzzle } from "./types";

export interface HintCheck {
  pos: number;
  pieceNumber: number; // 1-based
  respected: boolean;
  cellLabel: string; // e.g. "I8"
}

export interface BoardAudit {
  /** Only meaningful for 16x16 boards. */
  applicable: boolean;
  /** Placed pieces all belong to the official set (as a multiset). */
  officialSet: boolean;
  /** Number of pieces used more than once. */
  duplicates: number;
  hints: HintCheck[];
  hintsRespected: number;
}

function canonical(e: Edges): string {
  let best = e.join(",");
  let cur = e;
  for (let r = 0; r < 3; r++) {
    cur = [cur[3], cur[0], cur[1], cur[2]];
    const s = cur.join(",");
    if (s < best) best = s;
  }
  return best;
}

export function auditBoard(board: BucasBoard, official: Puzzle): BoardAudit {
  const applicable = board.width === 16 && board.height === 16;
  if (!applicable) {
    return { applicable, officialSet: false, duplicates: 0, hints: [], hintsRespected: 0 };
  }

  // Multiset check: every placed cell must consume a distinct official piece.
  const pool = new Map<string, number>();
  for (const e of official.pieces) {
    const k = canonical(e as Edges);
    pool.set(k, (pool.get(k) ?? 0) + 1);
  }
  let duplicates = 0;
  let officialSet = true;
  for (const cell of board.cells) {
    if (!cell) continue;
    const k = canonical(cell);
    const left = pool.get(k) ?? 0;
    if (left <= 0) {
      // Either not an official piece at all, or used more often than the
      // set allows.
      if (pool.has(k)) duplicates++;
      else officialSet = false;
    } else {
      pool.set(k, left - 1);
    }
  }
  if (duplicates > 0) officialSet = false;

  const hints: HintCheck[] = official.hints.map((h) => {
    const cell = board.cells[h.pos];
    const expected = rotateEdges(official.pieces[h.piece], h.rot);
    const respected =
      !!cell &&
      cell[0] === expected[0] &&
      cell[1] === expected[1] &&
      cell[2] === expected[2] &&
      cell[3] === expected[3];
    const x = h.pos % 16;
    const y = Math.floor(h.pos / 16);
    return {
      pos: h.pos,
      pieceNumber: h.piece + 1,
      respected,
      cellLabel: `${String.fromCharCode(65 + y)}${x + 1}`,
    };
  });

  return {
    applicable,
    officialSet,
    duplicates,
    hints,
    hintsRespected: hints.filter((h) => h.respected).length,
  };
}
