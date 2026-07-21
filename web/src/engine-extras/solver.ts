// Step-able backtracking DFS over an arbitrary cell-visit order
// (ALGORITHM.md §7) plus board scoring (ALGORITHM.md §8).
//
// The solver is an explicit state machine, not a recursive function: callers
// run `step(budget)` repeatedly and read off the board between calls, which is
// exactly what an animated browser UI needs. One "step" is one placement or one
// backtrack; the candidate scan inside a step is bounded by pieces × rotations.
//
// Canonical source: engine/src/solver.rs. The counters (nodes/attempts/
// backtracks) and the exact candidate-scan order are reproduced precisely:
// golden parity asserts them field-for-field, which proves the ports walk the
// identical search tree, not merely that they both solve.

import { XorShift } from "./rng.ts";
import { rotated, BORDER, type Edges, type Puzzle } from "./types.ts";
import type { SolverReport } from "@/lib/types";

/** Search status, matching the Rust `Status` enum (serde "lowercase"). */
export type Status = "running" | "solved" | "exhausted";

/**
 * Sentinel for "no candidate". Rust uses `u32::MAX`; we use -1, which never
 * collides with a real cursor/row value (those are always >= 0).
 */
const NONE = -1;

/**
 * Dense availability bitset over piece ids: bit `p` set ⇒ piece `p` is placed.
 * Mirrors the `BitSet` in engine/src/solver.rs (a `Vec<u64>` of words, tested
 * and flipped with single bit operations). We use 32-bit words since JS bitwise
 * operators are 32-bit; the behaviour is identical.
 */
class BitSet {
  private words: Int32Array;

  constructor(n: number) {
    this.words = new Int32Array(Math.ceil(n / 32));
  }

  contains(i: number): boolean {
    return ((this.words[i >> 5] ?? 0) >>> (i & 31)) & 1 ? true : false;
  }

  insert(i: number): void {
    this.words[i >> 5] = (this.words[i >> 5] ?? 0) | (1 << (i & 31));
  }

  remove(i: number): void {
    this.words[i >> 5] = (this.words[i >> 5] ?? 0) & ~(1 << (i & 31));
  }
}

/** One frame per non-hint cell, in visit order (ALGORITHM.md §7.1). */
interface Frame {
  /** Which board cell this frame fills. */
  pos: number;
  /** Next candidate row (orderIndex*4 + rotation) to try here. */
  cursor: number;
  /** Table row currently placed here, or NONE while empty. */
  placed: number;
}

/**
 * The step-able backtracking solver. Construct with a puzzle + path, then call
 * `step(budget)` in a loop. Mirrors `Solver` in engine/src/solver.rs.
 */
export class Solver {
  private readonly width: number;
  private readonly height: number;
  private readonly nPieces: number;
  /** Rotated edges for every (piece, rotation), piece-id major. */
  private readonly table: Edges[];
  /** False for rotations identical to a lower rotation of the same piece. */
  private readonly distinct: boolean[];
  /** Iteration order over pieces (optionally seed-shuffled). */
  private readonly pieceOrder: number[];
  /** Cell -> table row (pieceId*4+rot) or -1. */
  private readonly boardCells: Int32Array;
  /** Availability bitset: bit p set ⇒ piece p is already placed. */
  private readonly used: BitSet;
  /** One frame per non-hint cell, in visit order. */
  private readonly frames: Frame[];
  private depth: number;
  private readonly hintCount: number;
  private status: Status;
  private nodes: number;
  private attempts: number;
  private backtracks: number;
  private bestPlaced: number;
  private readonly bestBoardCells: Int32Array;

  constructor(
    puzzle: Puzzle,
    path: Uint16Array | readonly number[],
    useHints: boolean,
    shufflePieces: boolean,
    seed: number,
  ) {
    const nCells = puzzle.width * puzzle.height;
    const nPieces = puzzle.pieces.length;
    if (nPieces !== nCells) {
      throw new Error(`puzzle has ${String(nPieces)} pieces for ${String(nCells)} cells`);
    }
    if (path.length !== nCells) {
      throw new Error(`path covers ${String(path.length)} of ${String(nCells)} cells`);
    }
    const seen = new Array<boolean>(nCells).fill(false);
    for (const c of path) {
      if (c >= nCells || seen[c]) {
        throw new Error("path is not a permutation of the cells");
      }
      seen[c] = true;
    }

    // Setup: rotation table + distinct-rotation mask (ALGORITHM.md §7.1).
    const table: Edges[] = new Array<Edges>(nPieces * 4);
    const distinct = new Array<boolean>(nPieces * 4).fill(true);
    for (let id = 0; id < nPieces; id++) {
      const e = puzzle.pieces[id] as Edges;
      for (let r = 0; r < 4; r++) {
        const re = rotated(e, r);
        table[id * 4 + r] = re;
        for (let prev = 0; prev < r; prev++) {
          const pe = table[id * 4 + prev] as Edges;
          if (pe[0] === re[0] && pe[1] === re[1] && pe[2] === re[2] && pe[3] === re[3]) {
            distinct[id * 4 + r] = false;
            break;
          }
        }
      }
    }

    // Piece try-order: 0,1,2,… by default; optionally a seeded shuffle.
    const pieceOrder: number[] = [];
    for (let i = 0; i < nPieces; i++) {
      pieceOrder.push(i);
    }
    if (shufflePieces) {
      new XorShift(seed).shuffle(pieceOrder);
    }

    const boardCells = new Int32Array(nCells).fill(-1);
    const used = new BitSet(nPieces);
    let hintCount = 0;
    if (useHints) {
      for (const hint of puzzle.hints) {
        const pos = hint.pos;
        if (pos >= nCells || hint.piece >= nPieces || used.contains(hint.piece)) {
          throw new Error(`invalid hint at position ${String(pos)}`);
        }
        boardCells[pos] = hint.piece * 4 + (hint.rot & 3);
        used.insert(hint.piece);
        hintCount += 1;
      }
    }

    // One frame per non-hint cell, in visit order.
    const frames: Frame[] = [];
    for (const c of path) {
      if (boardCells[c] === -1) {
        frames.push({ pos: c, cursor: 0, placed: NONE });
      }
    }

    this.width = puzzle.width;
    this.height = puzzle.height;
    this.nPieces = nPieces;
    this.table = table;
    this.distinct = distinct;
    this.pieceOrder = pieceOrder;
    this.boardCells = boardCells;
    this.used = used;
    this.frames = frames;
    this.depth = 0;
    this.hintCount = hintCount;
    this.status = "running";
    this.nodes = 0;
    this.attempts = 0;
    this.backtracks = 0;
    this.bestPlaced = hintCount;
    this.bestBoardCells = boardCells.slice();
  }

  /**
   * Does a piece with edges `e` fit at cell `pos`? (ALGORITHM.md §7.3).
   * Rim rule: a side's edge is grey iff that side is on the outer rim.
   * Neighbour rule: for each already-placed neighbour, shared colours match.
   */
  private fits(pos: number, e: Edges): boolean {
    const w = this.width;
    const h = this.height;
    const x = pos % w;
    const y = Math.floor(pos / w);
    const top = e[0];
    const right = e[1];
    const bottom = e[2];
    const left = e[3];

    // Rim edges must be grey; interior-facing edges must not be.
    if ((y === 0) !== (top === BORDER)) {
      return false;
    }
    if ((y === h - 1) !== (bottom === BORDER)) {
      return false;
    }
    if ((x === 0) !== (left === BORDER)) {
      return false;
    }
    if ((x === w - 1) !== (right === BORDER)) {
      return false;
    }

    // Match whichever neighbours are already placed (visit order is arbitrary,
    // so all four directions matter). table[n][2]=down, [0]=up, [1]=right,
    // [3]=left.
    if (y > 0) {
      const n = this.boardCells[pos - w] ?? -1;
      if (n >= 0 && (this.table[n] as Edges)[2] !== top) {
        return false;
      }
    }
    if (y < h - 1) {
      const n = this.boardCells[pos + w] ?? -1;
      if (n >= 0 && (this.table[n] as Edges)[0] !== bottom) {
        return false;
      }
    }
    if (x > 0) {
      const n = this.boardCells[pos - 1] ?? -1;
      if (n >= 0 && (this.table[n] as Edges)[1] !== left) {
        return false;
      }
    }
    if (x < w - 1) {
      const n = this.boardCells[pos + 1] ?? -1;
      if (n >= 0 && (this.table[n] as Edges)[3] !== right) {
        return false;
      }
    }
    return true;
  }

  /**
   * Run up to `budget` steps (placements + backtracks), then return a report.
   * Cheap to call in an animation loop. Mirrors `Solver::step`.
   */
  step(budget: number): SolverReport {
    let remaining = budget;
    while (remaining > 0 && this.status === "running") {
      remaining -= 1;

      // (1) All cells filled → Solved (ALGORITHM.md §7.2 step 1).
      if (this.depth === this.frames.length) {
        this.status = "solved";
        this.bestPlaced = this.placedCount();
        this.bestBoardCells.set(this.boardCells);
        break;
      }

      const frame = this.frames[this.depth] as Frame;
      const pos = frame.pos;
      let cursor = frame.cursor;
      const limit = this.nPieces * 4;
      let placedRow = NONE;

      // (2) Scan candidates starting at the frame's cursor.
      while (cursor < limit) {
        const oi = Math.floor(cursor / 4);
        const r = cursor % 4;
        cursor += 1;
        const pid = this.pieceOrder[oi] as number;
        if (this.used.contains(pid)) {
          // Skip all four rotations of an already-used piece at once.
          cursor = (cursor + 3) & ~3;
          continue;
        }
        const row = pid * 4 + r;
        if (!(this.distinct[row] ?? false)) {
          // Non-distinct rotation: skip just it.
          continue;
        }
        this.attempts += 1;
        if (this.fits(pos, this.table[row] as Edges)) {
          placedRow = row;
          break;
        }
      }

      if (placedRow !== NONE) {
        // (3) A candidate fit: place it, advance depth, count a node.
        const pid = Math.floor(placedRow / 4);
        this.boardCells[pos] = placedRow;
        this.used.insert(pid);
        frame.cursor = cursor;
        frame.placed = placedRow;
        this.depth += 1;
        this.nodes += 1;
        const placed = this.placedCount();
        if (placed > this.bestPlaced) {
          this.bestPlaced = placed;
          this.bestBoardCells.set(this.boardCells);
        }
      } else {
        // (4) Dead end: reset this frame's cursor; if at the top, Exhausted.
        frame.cursor = 0;
        if (this.depth === 0) {
          this.status = "exhausted";
          break;
        }
        // Step back one frame and undo its placement.
        this.depth -= 1;
        const prev = this.frames[this.depth] as Frame;
        const row = prev.placed;
        prev.placed = NONE;
        this.boardCells[prev.pos] = -1;
        this.used.remove(Math.floor(row / 4));
        this.backtracks += 1;
      }
    }
    return this.report();
  }

  /** Pieces currently on the board, hints included. */
  private placedCount(): number {
    return this.hintCount + this.depth;
  }

  report(): SolverReport {
    return {
      status: this.status,
      nodes: this.nodes,
      attempts: this.attempts,
      backtracks: this.backtracks,
      placed: this.placedCount(),
      bestPlaced: this.bestPlaced,
    };
  }

  /** Current board: cell -> pieceId*4+rotation, or -1 when empty. */
  board(): Int32Array {
    return this.boardCells.slice();
  }

  /** Deepest board ever reached (by placed count). */
  bestBoard(): Int32Array {
    return this.bestBoardCells.slice();
  }
}

/**
 * Matched interior edges of a board encoded as cell -> piece*4+rot | -1
 * (ALGORITHM.md §8). Bucas convention: a grey/grey interior contact does NOT
 * score. Max for n×n is 2·n·(n−1). Mirrors `score_board` in solver.rs.
 */
export function scoreBoard(puzzle: Puzzle, board: Int32Array | readonly number[]): number {
  const w = puzzle.width;
  const h = puzzle.height;
  const edgesOf = (row: number): Edges | null => {
    if (row < 0) {
      return null;
    }
    const pid = Math.floor(row / 4);
    const r = row % 4;
    const piece = puzzle.pieces[pid];
    if (piece === undefined) {
      return null;
    }
    return rotated(piece, r);
  };
  let score = 0;
  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      const here = edgesOf(board[y * w + x] ?? -1);
      if (x + 1 < w) {
        const b = edgesOf(board[y * w + x + 1] ?? -1);
        if (here !== null && b !== null && here[1] === b[3] && here[1] !== BORDER) {
          score += 1;
        }
      }
      if (y + 1 < h) {
        const b = edgesOf(board[(y + 1) * w + x] ?? -1);
        if (here !== null && b !== null && here[2] === b[0] && here[2] !== BORDER) {
          score += 1;
        }
      }
    }
  }
  return score;
}
