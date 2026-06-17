// Full interop with the e2.bucas.name viewer URL format.
//
// board_edges: 4 lowercase letters per cell (URDL), row-major, 'a' = 0 = grey
// border, "aaaa" = empty cell. Rotation is implicit: the letters are the
// already-rotated edges. board_pieces: 3 decimal digits per cell, 1-based
// official piece number, 000 = empty. motifs_order permutes which glyph each
// letter draws; "marie" and "jblackwood" are identical.

import { BORDER, rotateEdges, maxScore } from "./types";
import type { Puzzle, BoardCells } from "./types";

export type Edges = [number, number, number, number];

export interface BucasBoard {
  width: number;
  height: number;
  /** Edge colors per cell in DEFAULT bucas letters (null = empty cell). */
  cells: (Edges | null)[];
  /** 1-based piece numbers when board_pieces was present (0 = empty). */
  pieceNumbers: number[] | null;
  puzzleName: string | null;
}

// Letter translation tables between motif orders (index = letter - 'a').
const TO_BUCAS: Record<string, string> = {
  jef: "atojeqlgbupkfrmhcwvsnid",
  marie: "abglqejotchmrfkpudinsvw",
  jblackwood: "abglqejotchmrfkpudinsvw",
};

/** Accepts a full URL, a location hash, or a bare params string. */
export function parseParams(input: string): Record<string, string> {
  let s = input.trim();
  const hashIdx = s.indexOf("#");
  if (hashIdx >= 0) s = s.slice(hashIdx + 1);
  else {
    const qIdx = s.indexOf("?");
    if (qIdx >= 0) s = s.slice(qIdx + 1);
  }
  const out: Record<string, string> = {};
  for (const part of s.split("&")) {
    const eq = part.indexOf("=");
    if (eq > 0) out[part.slice(0, eq)] = part.slice(eq + 1);
  }
  return out;
}

/** Normalize any bucas-style param map into our clean URL params: collapse
 *  board_w/board_h into a single square `puzzle_size`, keep only the keys we
 *  round-trip, and sanitize the name. All resulting values are URL-safe. */
export function toOurParams(raw: Record<string, string>): Record<string, string> {
  const out: Record<string, string> = {};
  const size = raw["puzzle_size"] ?? raw["board_w"] ?? raw["board_h"];
  if (size) out["puzzle_size"] = size;
  if (raw["puzzle"]) out["puzzle"] = raw["puzzle"].replace(/[^A-Za-z0-9_]+/g, "_");
  for (const k of ["board_edges", "board_pieces", "motifs_order"]) {
    if (raw[k]) out[k] = raw[k];
  }
  return out;
}

export function decodeBucas(input: string | Record<string, string>): BucasBoard {
  const params = typeof input === "string" ? parseParams(input) : input;
  const edges = params["board_edges"];
  if (!edges) throw new Error("no board_edges parameter found");
  // Our URLs use a single square `puzzle_size`; bucas links use board_w/board_h.
  const size = params["puzzle_size"];
  const width = parseInt(size ?? params["board_w"] ?? "16", 10);
  const height = parseInt(size ?? params["board_h"] ?? "16", 10);
  if (edges.length !== width * height * 4) {
    throw new Error(
      `board_edges has ${edges.length} letters; expected ${width * height * 4} for ${width}×${height}`,
    );
  }

  const order = params["motifs_order"];
  const translate = order && TO_BUCAS[order] ? TO_BUCAS[order] : null;

  const cells: (Edges | null)[] = [];
  for (let i = 0; i < width * height; i++) {
    const four = edges.slice(i * 4, i * 4 + 4);
    const e = [...four].map((ch) => {
      let c = ch.charCodeAt(0) - 97;
      if (c < 0 || c > 25) throw new Error(`bad edge letter "${ch}"`);
      if (translate && c < translate.length) c = translate.charCodeAt(c) - 97;
      return c;
    }) as Edges;
    cells.push(e.every((c) => c === 0) ? null : e);
  }

  let pieceNumbers: number[] | null = null;
  const bp = params["board_pieces"];
  if (bp && bp.length === width * height * 3) {
    pieceNumbers = [];
    for (let i = 0; i < width * height; i++) {
      pieceNumbers.push(parseInt(bp.slice(i * 3, i * 3 + 3), 10));
    }
  }

  return {
    width,
    height,
    cells,
    pieceNumbers,
    puzzleName: params["puzzle"] ?? null,
  };
}

/** Match decoded cells to a piece set: cell -> pieceId*4+rot, or -1. */
export function matchToPieces(puzzle: Puzzle, board: BucasBoard): Int32Array {
  const out = new Int32Array(board.width * board.height).fill(-1);
  const used = new Array(puzzle.pieces.length).fill(false);

  // Honor explicit piece numbers first (1-based), deriving rotation.
  board.cells.forEach((cell, pos) => {
    if (!cell) return;
    const hinted = board.pieceNumbers?.[pos];
    if (hinted && hinted >= 1 && hinted <= puzzle.pieces.length) {
      const pid = hinted - 1;
      const piece = puzzle.pieces[pid];
      if (!piece) return;
      for (let r = 0; r < 4; r++) {
        const e = rotateEdges(piece, r);
        if (e[0] === cell[0] && e[1] === cell[1] && e[2] === cell[2] && e[3] === cell[3]) {
          out[pos] = pid * 4 + r;
          used[pid] = true;
          return;
        }
      }
    }
  });

  board.cells.forEach((cell, pos) => {
    if (!cell || (out[pos] ?? -1) >= 0) return;
    for (let pid = 0; pid < puzzle.pieces.length; pid++) {
      if (used[pid]) continue;
      const piece = puzzle.pieces[pid];
      if (!piece) continue;
      for (let r = 0; r < 4; r++) {
        const e = rotateEdges(piece, r);
        if (e[0] === cell[0] && e[1] === cell[1] && e[2] === cell[2] && e[3] === cell[3]) {
          out[pos] = pid * 4 + r;
          used[pid] = true;
          return;
        }
      }
    }
  });
  return out;
}

/** Matched interior edges; grey-grey contacts do not score (bucas rule). */
export function scoreCells(board: BucasBoard): number {
  const { width: w, height: h, cells } = board;
  let score = 0;
  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      const a = cells[y * w + x];
      if (!a) continue;
      if (x + 1 < w) {
        const b = cells[y * w + x + 1];
        if (b && a[1] === b[3] && a[1] !== BORDER) score++;
      }
      if (y + 1 < h) {
        const b = cells[(y + 1) * w + x];
        if (b && a[2] === b[0] && a[2] !== BORDER) score++;
      }
    }
  }
  return score;
}

/** Conflicted edge pairs, for rendering. Returns [pos, direction] pairs
 *  (direction in URDL indices); both half-edges of a conflict are listed.
 *  A grey-grey interior contact counts as a conflict, like on bucas. */
export function conflictEdges(board: BucasBoard): [number, number][] {
  const { width: w, height: h, cells } = board;
  const out: [number, number][] = [];
  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      const a = cells[y * w + x];
      if (!a) continue;
      if (x + 1 < w) {
        const b = cells[y * w + x + 1];
        if (b && (a[1] !== b[3] || a[1] === BORDER)) {
          out.push([y * w + x, 1], [y * w + x + 1, 3]);
        }
      }
      if (y + 1 < h) {
        const b = cells[(y + 1) * w + x];
        if (b && (a[2] !== b[0] || a[2] === BORDER)) {
          out.push([y * w + x, 2], [(y + 1) * w + x, 0]);
        }
      }
    }
  }
  return out;
}

/** Build a BucasBoard from an engine board (cell -> pieceId*4+rot | -1). */
export function boardFromEngine(puzzle: Puzzle, board: BoardCells): BucasBoard {
  const cells: (Edges | null)[] = [];
  for (let pos = 0; pos < puzzle.width * puzzle.height; pos++) {
    const v = board[pos] ?? -1;
    const piece = v < 0 ? undefined : puzzle.pieces[v >> 2];
    if (!piece) {
      cells.push(null);
    } else {
      cells.push(rotateEdges(piece, v & 3));
    }
  }
  return {
    width: puzzle.width,
    height: puzzle.height,
    cells,
    pieceNumbers: null,
    puzzleName: puzzle.name,
  };
}

/** Encode a board's cells into the `board_edges` / `board_pieces` strings
 *  (default motif letters). Shared by both our own URLs and bucas export. */
function encodeCells(puzzle: Puzzle, board: BoardCells): { edges: string; pieces: string } {
  const n = puzzle.width * puzzle.height;
  let edges = "";
  let pieces = "";
  for (let pos = 0; pos < n; pos++) {
    const v = board[pos] ?? -1;
    const piece = v < 0 ? undefined : puzzle.pieces[v >> 2];
    if (!piece) {
      edges += "aaaa";
      pieces += "000";
    } else {
      const e = rotateEdges(piece, v & 3);
      edges += e.map((c) => String.fromCharCode(97 + c)).join("");
      pieces += String((v >> 2) + 1).padStart(3, "0");
    }
  }
  return { edges, pieces };
}

/** Our own clean query params for a board. Boards are always square, so we
 *  emit a single `puzzle_size` instead of bucas's `board_w`/`board_h`, and we
 *  keep every value in the safe `[A-Za-z0-9_]` range so the URL needs no
 *  percent-encoding. `decodeBucas` reads these back transparently. */
export function ourParams(
  puzzle: Puzzle,
  board: BoardCells,
  name = "eternity2-community",
): Record<string, string> {
  const { edges, pieces } = encodeCells(puzzle, board);
  return {
    puzzle: name.replace(/[^A-Za-z0-9_]+/g, "_"),
    puzzle_size: String(puzzle.width),
    board_edges: edges,
    board_pieces: pieces,
  };
}

/** Bucas URL parameters for any board (default motif letters). Works for any
 *  size/piece set; bucas only *verifies* the official 16×16, but displays
 *  everything. Bucas needs the explicit `board_w`/`board_h` pair. */
export function bucasParams(
  puzzle: Puzzle,
  board: BoardCells,
  name = "eternity2-community",
): string {
  const { edges, pieces } = encodeCells(puzzle, board);
  return (
    `puzzle=${encodeURIComponent(name)}` +
    `&board_w=${puzzle.width}&board_h=${puzzle.height}` +
    `&board_edges=${edges}&board_pieces=${pieces}`
  );
}

export function encodeBucasUrl(
  puzzle: Puzzle,
  board: BoardCells,
  name = "eternity2-community",
): string {
  return `https://e2.bucas.name/#${bucasParams(puzzle, board, name)}`;
}

export function scoreSummary(board: BucasBoard): {
  score: number;
  max: number;
  placed: number;
} {
  return {
    score: scoreCells(board),
    max: maxScore(board.width, board.height),
    placed: board.cells.filter(Boolean).length,
  };
}
