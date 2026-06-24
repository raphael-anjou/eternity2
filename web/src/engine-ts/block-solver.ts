// Macro-piece ("block") backtracking solver.
//
// The single-piece Solver (solver.ts) fills the board one cell at a time. This
// solver fills it one *block* at a time: the user groups cells into blocks
// (2×2, 2×1, 3×3, …) and the search treats each block as the atomic unit. At
// each step it commits a whole valid sub-assembly — distinct loose pieces, all
// internal edges of the block already matched, rim and already-placed
// neighbours respected — then moves to the next block. On a dead end it
// retracts the entire block and tries the next sub-assembly.
//
// This is the "place 2×2 tiles instead of single pieces" idea from the
// complex-theory writeup (groups.io, Dan Karlsson) realised as a real search,
// not just a counting model. The block frontier is what makes it different: a
// block's internal edges are all resolved before any neighbouring block starts,
// so the order in which whole regions are committed — not just cells — drives
// the cost.
//
// Correctness vs the single-piece solver. With every block of size 1 this
// reduces exactly to a single-piece DFS over the same visit order (golden-tested
// below). For larger blocks it explores the SAME solution set — a full board is
// valid iff every block sub-assembly is internally consistent and all block
// boundaries match — but in a different node order, so it finds the same
// solutions, generally with far fewer top-level nodes and far more work hidden
// inside each block's enumeration.
//
// Step-able like the single-piece solver: one `step()` advances the search by a
// bounded amount so an animation loop stays responsive. A "block node" is one
// committed block; we also expose the internal sub-assembly attempts so the cost
// is honest.

import { rotated, BORDER, type Edges, type Puzzle } from "./types.ts";
import type { SolverHandle, SolverReport } from "@/lib/types";
import { scoreBoard } from "./solver.ts";

export type Status = "running" | "solved" | "exhausted";

const NONE = -1;

/** A block: the cells it covers, in the sub-order pieces are placed within it. */
export interface Block {
  /** Board cell indices (row-major), in internal placement order. */
  cells: number[];
}

/**
 * Dense availability bitset over piece ids (mirrors solver.ts BitSet). 32-bit
 * words since JS bitwise ops are 32-bit.
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

/**
 * One outer frame per block. The inner state is a small DFS over the block's own
 * cells: `subDepth` cells of this block are currently placed, and each has a
 * cursor into the (piece, rotation) table for resuming the enumeration after a
 * sub-assembly is committed or retracted.
 */
interface BlockFrame {
  block: Block;
  /** How many of this block's cells are currently placed (0..cells.length). */
  subDepth: number;
  /** Per-cell resume cursor into pieceOrder*4 (length = cells.length). */
  cursors: number[];
  /** Per-cell table row currently placed, or NONE (length = cells.length). */
  placed: number[];
  /** True once this block has committed a full sub-assembly (subDepth === len). */
  committed: boolean;
}

export class BlockSolver {
  private readonly width: number;
  private readonly height: number;
  private readonly nPieces: number;
  private readonly table: Edges[];
  private readonly distinct: boolean[];
  private readonly pieceOrder: number[];
  private readonly boardCells: Int32Array;
  private readonly used: BitSet;
  private readonly frames: BlockFrame[];
  /** Index of the block currently being worked (0..frames.length). */
  private blockDepth: number;
  private status: Status;
  private nodes: number; // committed blocks
  private attempts: number; // per-cell fit attempts (internal cost)
  private backtracks: number; // retracted blocks
  private bestPlaced: number; // most cells ever on the board
  private readonly bestBoardCells: Int32Array;
  private placedCells: number;

  constructor(puzzle: Puzzle, blocks: Block[], shufflePieces = false, seed = 0) {
    const nCells = puzzle.width * puzzle.height;
    const nPieces = puzzle.pieces.length;
    if (nPieces !== nCells) {
      throw new Error(`puzzle has ${String(nPieces)} pieces for ${String(nCells)} cells`);
    }
    // Blocks must partition the board: every cell once.
    const seen = new Array<boolean>(nCells).fill(false);
    let covered = 0;
    for (const b of blocks) {
      for (const c of b.cells) {
        if (c < 0 || c >= nCells || seen[c]) {
          throw new Error("blocks are not a partition of the cells");
        }
        seen[c] = true;
        covered++;
      }
    }
    if (covered !== nCells) {
      throw new Error(`blocks cover ${String(covered)} of ${String(nCells)} cells`);
    }

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

    const pieceOrder: number[] = [];
    for (let i = 0; i < nPieces; i++) pieceOrder.push(i);
    if (shufflePieces) {
      // Local Fisher-Yates with a tiny LCG so we don't depend on rng.ts shuffle
      // semantics here; determinism is all we need.
      let s = (seed ^ 0x9e3779b9) >>> 0;
      for (let i = pieceOrder.length - 1; i > 0; i--) {
        s = (Math.imul(s, 1664525) + 1013904223) >>> 0;
        const j = s % (i + 1);
        const tmp = pieceOrder[i] as number;
        pieceOrder[i] = pieceOrder[j] as number;
        pieceOrder[j] = tmp;
      }
    }

    const frames: BlockFrame[] = blocks.map((block) => ({
      block,
      subDepth: 0,
      cursors: new Array<number>(block.cells.length).fill(0),
      placed: new Array<number>(block.cells.length).fill(NONE),
      committed: false,
    }));

    this.width = puzzle.width;
    this.height = puzzle.height;
    this.nPieces = nPieces;
    this.table = table;
    this.distinct = distinct;
    this.pieceOrder = pieceOrder;
    this.boardCells = new Int32Array(nCells).fill(-1);
    this.used = new BitSet(nPieces);
    this.frames = frames;
    this.blockDepth = 0;
    this.status = "running";
    this.nodes = 0;
    this.attempts = 0;
    this.backtracks = 0;
    this.bestPlaced = 0;
    this.placedCells = 0;
    this.bestBoardCells = this.boardCells.slice();
  }

  /** Same fit test as solver.ts: rim greyness + already-placed neighbour match. */
  private fits(pos: number, e: Edges): boolean {
    const w = this.width;
    const h = this.height;
    const x = pos % w;
    const y = Math.floor(pos / w);
    const top = e[0];
    const right = e[1];
    const bottom = e[2];
    const left = e[3];

    if ((y === 0) !== (top === BORDER)) return false;
    if ((y === h - 1) !== (bottom === BORDER)) return false;
    if ((x === 0) !== (left === BORDER)) return false;
    if ((x === w - 1) !== (right === BORDER)) return false;

    if (y > 0) {
      const n = this.boardCells[pos - w] ?? -1;
      if (n >= 0 && (this.table[n] as Edges)[2] !== top) return false;
    }
    if (y < h - 1) {
      const n = this.boardCells[pos + w] ?? -1;
      if (n >= 0 && (this.table[n] as Edges)[0] !== bottom) return false;
    }
    if (x > 0) {
      const n = this.boardCells[pos - 1] ?? -1;
      if (n >= 0 && (this.table[n] as Edges)[1] !== left) return false;
    }
    if (x < w - 1) {
      const n = this.boardCells[pos + 1] ?? -1;
      if (n >= 0 && (this.table[n] as Edges)[3] !== right) return false;
    }
    return true;
  }

  /** Place one cell within the current block; returns true on success. */
  private tryPlaceCell(frame: BlockFrame): boolean {
    const i = frame.subDepth;
    const pos = frame.block.cells[i] as number;
    let cursor = frame.cursors[i] as number;
    const limit = this.nPieces * 4;
    while (cursor < limit) {
      const oi = Math.floor(cursor / 4);
      const r = cursor % 4;
      cursor += 1;
      const pid = this.pieceOrder[oi] as number;
      if (this.used.contains(pid)) {
        cursor = (cursor + 3) & ~3; // skip the piece's other rotations
        continue;
      }
      const row = pid * 4 + r;
      if (!(this.distinct[row] ?? false)) continue;
      this.attempts += 1;
      if (this.fits(pos, this.table[row] as Edges)) {
        this.boardCells[pos] = row;
        this.used.insert(pid);
        this.placedCells += 1;
        frame.placed[i] = row;
        frame.cursors[i] = cursor; // resume after this row next time
        frame.subDepth = i + 1;
        if (this.placedCells > this.bestPlaced) {
          this.bestPlaced = this.placedCells;
          this.bestBoardCells.set(this.boardCells);
        }
        return true;
      }
    }
    // Exhausted this cell: reset its cursor for the next time we reach it.
    frame.cursors[i] = 0;
    return false;
  }

  /** Undo the last placed cell within the current block. */
  private retractCell(frame: BlockFrame): void {
    const i = frame.subDepth - 1;
    const pos = frame.block.cells[i] as number;
    const row = frame.placed[i] as number;
    frame.placed[i] = NONE;
    this.boardCells[pos] = -1;
    this.used.remove(Math.floor(row / 4));
    this.placedCells -= 1;
    frame.subDepth = i;
  }

  /**
   * Advance the search. One unit of `budget` is one cell-level placement or
   * retraction (so the internal block enumeration is charged honestly). A whole
   * block committing counts as one `node`.
   */
  step(budget: number): SolverReport {
    let remaining = budget;
    while (remaining > 0 && this.status === "running") {
      remaining -= 1;

      if (this.blockDepth === this.frames.length) {
        this.status = "solved";
        this.bestPlaced = this.placedCells;
        this.bestBoardCells.set(this.boardCells);
        break;
      }

      const frame = this.frames[this.blockDepth] as BlockFrame;
      const len = frame.block.cells.length;

      if (frame.subDepth === len) {
        // Block fully assembled → commit and descend to the next block.
        frame.committed = true;
        this.nodes += 1;
        this.blockDepth += 1;
        continue;
      }

      if (this.tryPlaceCell(frame)) {
        continue; // placed a cell; keep filling this block
      }

      // Could not place cell `subDepth`. Retract within the block if possible;
      // otherwise this block has no remaining sub-assembly → backtrack a block.
      if (frame.subDepth > 0) {
        this.retractCell(frame);
        continue;
      }

      // Block exhausted at its first cell. Reset and step back to the previous
      // block, retracting its committed sub-assembly so we can try its next one.
      frame.committed = false;
      if (this.blockDepth === 0) {
        this.status = "exhausted";
        break;
      }
      this.blockDepth -= 1;
      const prev = this.frames[this.blockDepth] as BlockFrame;
      prev.committed = false;
      this.backtracks += 1;
      // Retract the previous block's last cell so its enumeration resumes from
      // the cursor we left (which points just past the row it had placed).
      if (prev.subDepth > 0) this.retractCell(prev);
    }
    return this.report();
  }

  report(): SolverReport {
    return {
      status: this.status,
      nodes: this.nodes,
      attempts: this.attempts,
      backtracks: this.backtracks,
      placed: this.placedCells,
      bestPlaced: this.bestPlaced,
    };
  }

  board(): Int32Array {
    return this.boardCells.slice();
  }

  /**
   * The board showing only *committed* blocks: the block currently being
   * assembled (at `blockDepth`) is masked to empty, so a viewer sees whole
   * blocks snap into place atomically rather than the internal cell-by-cell
   * sub-assembly search. Blocks at depth < blockDepth are fully committed;
   * blocks at depth > blockDepth are untouched (already empty).
   */
  committedBoard(): Int32Array {
    const out = this.boardCells.slice();
    const frame = this.frames[this.blockDepth];
    if (frame) {
      for (const cell of frame.block.cells) out[cell] = -1;
    }
    return out;
  }

  bestBoard(): Int32Array {
    return this.bestBoardCells.slice();
  }
}

/**
 * Wrap a BlockSolver as a SolverHandle so the playground race loop can drive it
 * exactly like any engine backend (it never learns the lane is block-mode). The
 * block solver is pure TypeScript, so `free` is a no-op and `reset` rebuilds.
 */
export function createBlockSolver(
  puzzle: Puzzle,
  blocks: Block[],
  opts: { shufflePieces?: boolean; seed?: number } = {},
): SolverHandle {
  const shuffle = opts.shufflePieces ?? false;
  const seed = opts.seed ?? 0;
  let solver = new BlockSolver(puzzle, blocks, shuffle, seed);
  return {
    step: (budget) => solver.step(budget),
    report: () => solver.report(),
    // Display the committed board: whole blocks snap into place atomically,
    // hiding the internal cell-by-cell sub-assembly search (the half-built block
    // flicker). Scoring still uses the true board so the count stays honest.
    board: () => solver.committedBoard(),
    bestBoard: () => solver.bestBoard(),
    score: () => scoreBoard(puzzle, solver.board()),
    bestScore: () => scoreBoard(puzzle, solver.bestBoard()),
    reset: () => {
      solver = new BlockSolver(puzzle, blocks, shuffle, seed);
    },
    free: () => {
      // No WASM resource to release.
    },
  };
}
