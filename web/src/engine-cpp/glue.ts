// Typed wrapper around the freestanding C++/wasm engine (engine.wasm). Exposes
// the EXACT same surface as web/src/engine/index.ts (the Rust/wasm-bindgen
// backend): the 8 standalone functions plus createSolver -> SolverHandle. The
// website can switch backends at build time without any call-site change.
//
// The wasm module has no imports and no JS runtime: it is a flat block of
// linear memory plus a handful of C exports (see engine-cpp/README.md for the
// ABI). All data crosses the boundary through two shared int regions — a
// "wire" buffer for puzzles and a "scratch" buffer for paths and boards.

import wasmUrl from "./engine.wasm?url";
import type { Puzzle, SolverReport } from "@/lib/types";

const PATH_KINDS = [
  "row-major",
  "snake",
  "column-major",
  "spiral-in",
  "spiral-out",
  "diagonal",
  "border-first",
  "double-snake",
  "random",
] as const;

// The set of exports the C engine provides, typed. Every export is a plain
// numeric function (wasm has only number params/returns), so no `any` anywhere.
interface EngineExports {
  readonly memory: WebAssembly.Memory;
  readonly e2_wire_ptr: () => number;
  readonly e2_scratch_ptr: () => number;
  readonly e2_scratch_len: () => number;
  readonly e2_official: () => number;
  readonly e2_generate: (size: number, colors: number, seed: number) => number;
  readonly e2_generate_solved: (size: number, colors: number, seed: number) => number;
  readonly e2_generate_framed: (
    size: number,
    colors: number,
    seed: number,
    framed: number,
  ) => number;
  readonly e2_generate_solved_framed: (
    size: number,
    colors: number,
    seed: number,
    framed: number,
  ) => number;
  readonly e2_max_colors: (size: number) => number;
  readonly e2_path_count: () => number;
  readonly e2_build_path: (kind: number, w: number, h: number, seed: number) => number;
  readonly e2_score_board: (nCells: number) => number;
  readonly e2_solver_new: (
    pathLen: number,
    useHints: number,
    shufflePieces: number,
    seed: number,
  ) => number;
  readonly e2_solver_free: (slot: number) => void;
  readonly e2_solver_reset: (slot: number) => void;
  readonly e2_solver_step: (slot: number, budget: number) => void;
  readonly e2_solver_status: (slot: number) => number;
  readonly e2_solver_nodes: (slot: number) => number;
  readonly e2_solver_attempts: (slot: number) => number;
  readonly e2_solver_backtracks: (slot: number) => number;
  readonly e2_solver_placed: (slot: number) => number;
  readonly e2_solver_best_placed: (slot: number) => number;
  readonly e2_solver_board: (slot: number) => number;
  readonly e2_solver_best_board: (slot: number) => number;
  readonly e2_solver_score: (slot: number) => number;
  readonly e2_solver_best_score: (slot: number) => number;
}

// Narrow an unknown export to a function (avoids `any` from WebAssembly types).
function asFn(v: WebAssembly.ExportValue, name: string): (...args: number[]) => number {
  if (typeof v !== "function") throw new Error(`wasm export ${name} is not a function`);
  return v as (...args: number[]) => number;
}

class Engine {
  readonly mem: WebAssembly.Memory;
  readonly wire: number; // i32 index of the wire buffer
  readonly scratch: number; // i32 index of the scratch buffer
  private readonly ex: EngineExports;

  constructor(instance: WebAssembly.Instance) {
    const raw = instance.exports;
    const memory = raw["memory"];
    if (!(memory instanceof WebAssembly.Memory)) throw new Error("wasm has no memory export");
    // Build a typed view of the exports. The bind below keeps each as a number
    // function; the EngineExports interface documents the real arities.
    const get = (name: string): WebAssembly.ExportValue => {
      const v = raw[name];
      if (v === undefined) throw new Error(`missing wasm export: ${name}`);
      return v;
    };
    const ex: EngineExports = {
      memory,
      e2_wire_ptr: asFn(get("e2_wire_ptr"), "e2_wire_ptr"),
      e2_scratch_ptr: asFn(get("e2_scratch_ptr"), "e2_scratch_ptr"),
      e2_scratch_len: asFn(get("e2_scratch_len"), "e2_scratch_len"),
      e2_official: asFn(get("e2_official"), "e2_official"),
      e2_generate: asFn(get("e2_generate"), "e2_generate"),
      e2_generate_solved: asFn(get("e2_generate_solved"), "e2_generate_solved"),
      e2_generate_framed: asFn(get("e2_generate_framed"), "e2_generate_framed"),
      e2_generate_solved_framed: asFn(
        get("e2_generate_solved_framed"),
        "e2_generate_solved_framed",
      ),
      e2_max_colors: asFn(get("e2_max_colors"), "e2_max_colors"),
      e2_path_count: asFn(get("e2_path_count"), "e2_path_count"),
      e2_build_path: asFn(get("e2_build_path"), "e2_build_path"),
      e2_score_board: asFn(get("e2_score_board"), "e2_score_board"),
      e2_solver_new: asFn(get("e2_solver_new"), "e2_solver_new"),
      e2_solver_free: asFn(get("e2_solver_free"), "e2_solver_free"),
      e2_solver_reset: asFn(get("e2_solver_reset"), "e2_solver_reset"),
      e2_solver_step: asFn(get("e2_solver_step"), "e2_solver_step"),
      e2_solver_status: asFn(get("e2_solver_status"), "e2_solver_status"),
      e2_solver_nodes: asFn(get("e2_solver_nodes"), "e2_solver_nodes"),
      e2_solver_attempts: asFn(get("e2_solver_attempts"), "e2_solver_attempts"),
      e2_solver_backtracks: asFn(get("e2_solver_backtracks"), "e2_solver_backtracks"),
      e2_solver_placed: asFn(get("e2_solver_placed"), "e2_solver_placed"),
      e2_solver_best_placed: asFn(get("e2_solver_best_placed"), "e2_solver_best_placed"),
      e2_solver_board: asFn(get("e2_solver_board"), "e2_solver_board"),
      e2_solver_best_board: asFn(get("e2_solver_best_board"), "e2_solver_best_board"),
      e2_solver_score: asFn(get("e2_solver_score"), "e2_solver_score"),
      e2_solver_best_score: asFn(get("e2_solver_best_score"), "e2_solver_best_score"),
    };
    this.ex = ex;
    this.mem = memory;
    this.wire = ex.e2_wire_ptr() >> 2;
    this.scratch = ex.e2_scratch_ptr() >> 2;
  }

  get exports(): EngineExports {
    return this.ex;
  }

  // A fresh Int32Array view each time: wasm memory can grow and detach old
  // buffers, so never cache the typed array across a call that may allocate.
  private i32(): Int32Array {
    return new Int32Array(this.mem.buffer);
  }

  readWirePuzzle(): Puzzle {
    const m = this.i32();
    const w = this.wire;
    const width = m[w + 0] ?? 0;
    const height = m[w + 1] ?? 0;
    const numColors = m[w + 2] ?? 0;
    const nPieces = m[w + 3] ?? 0;
    let o = w + 4;
    const pieces: [number, number, number, number][] = [];
    for (let i = 0; i < nPieces; i++) {
      pieces.push([m[o] ?? 0, m[o + 1] ?? 0, m[o + 2] ?? 0, m[o + 3] ?? 0]);
      o += 4;
    }
    const nHints = m[o++] ?? 0;
    const hints: Puzzle["hints"] = [];
    for (let i = 0; i < nHints; i++) {
      hints.push({ pos: m[o] ?? 0, piece: m[o + 1] ?? 0, rot: m[o + 2] ?? 0 });
      o += 3;
    }
    return { name: "", width, height, numColors, pieces, hints };
  }

  writeWirePuzzle(p: Puzzle): void {
    const m = this.i32();
    const w = this.wire;
    m[w + 0] = p.width;
    m[w + 1] = p.height;
    m[w + 2] = p.numColors;
    m[w + 3] = p.pieces.length;
    let o = w + 4;
    for (const e of p.pieces) {
      m[o++] = e[0];
      m[o++] = e[1];
      m[o++] = e[2];
      m[o++] = e[3];
    }
    m[o++] = p.hints.length;
    for (const h of p.hints) {
      m[o++] = h.pos;
      m[o++] = h.piece;
      m[o++] = h.rot;
    }
  }

  readScratch(n: number): Int32Array {
    const m = this.i32();
    return m.slice(this.scratch, this.scratch + n);
  }

  writeScratch(arr: ArrayLike<number>): void {
    const m = this.i32();
    for (let i = 0; i < arr.length; i++) m[this.scratch + i] = arr[i] ?? 0;
  }
}

let engine: Engine | null = null;
let ready: Promise<void> | null = null;

export function initEngine(): Promise<void> {
  if (!ready) {
    ready = (async () => {
      const resp = await fetch(wasmUrl);
      const bytes = await resp.arrayBuffer();
      const { instance } = await WebAssembly.instantiate(bytes, {});
      engine = new Engine(instance);
    })();
  }
  return ready;
}

function eng(): Engine {
  if (engine === null) {
    throw new Error("engine not initialized — await initEngine() first");
  }
  return engine;
}

export function getOfficialPuzzle(): Puzzle {
  const en = eng();
  en.exports.e2_official();
  const p = en.readWirePuzzle();
  return { ...p, name: "official_eternity2" };
}

export function getGeneratedPuzzle(size: number, colors: number, seed: number): Puzzle {
  const en = eng();
  en.exports.e2_generate(size, colors, seed >>> 0);
  const p = en.readWirePuzzle();
  return { ...p, name: `generated_${size}x${size}_c${colors}_s${seed}` };
}

/** Pieces in solution order/orientation: piece i belongs at cell i, rot 0. */
export function getGeneratedSolvedPuzzle(size: number, colors: number, seed: number): Puzzle {
  const en = eng();
  en.exports.e2_generate_solved(size, colors, seed >>> 0);
  const p = en.readWirePuzzle();
  return { ...p, name: `generated_${size}x${size}_c${colors}_s${seed}` };
}

/**
 * Framed variant: when `framed` is true, frame-restricted colors are confined to
 * the border band (mirrors real Eternity II). `framed=false` is identical to
 * `getGeneratedPuzzle`.
 */
export function getGeneratedPuzzleFramed(
  size: number,
  colors: number,
  seed: number,
  framed: boolean,
): Puzzle {
  const en = eng();
  en.exports.e2_generate_framed(size, colors, seed >>> 0, framed ? 1 : 0);
  const p = en.readWirePuzzle();
  return { ...p, name: `generated_${size}x${size}_c${colors}_s${seed}` };
}

/** Solved-order counterpart of `getGeneratedPuzzleFramed`. */
export function getGeneratedSolvedPuzzleFramed(
  size: number,
  colors: number,
  seed: number,
  framed: boolean,
): Puzzle {
  const en = eng();
  en.exports.e2_generate_solved_framed(size, colors, seed >>> 0, framed ? 1 : 0);
  const p = en.readWirePuzzle();
  return { ...p, name: `generated_${size}x${size}_c${colors}_s${seed}` };
}

export function getMaxColors(size: number): number {
  return eng().exports.e2_max_colors(size);
}

export function getPathKinds(): string[] {
  return [...PATH_KINDS];
}

export function getPath(kind: string, width: number, height: number, seed = 0): Uint16Array {
  const en = eng();
  const idx = PATH_KINDS.indexOf(kind as (typeof PATH_KINDS)[number]);
  if (idx < 0) throw new Error(`unknown path kind: ${kind}`);
  const n = en.exports.e2_build_path(idx, width, height, seed >>> 0);
  if (n < 0) throw new Error(`unknown path kind: ${kind}`);
  return Uint16Array.from(en.readScratch(n));
}

export function getBoardScore(puzzle: Puzzle, board: Int32Array): number {
  const en = eng();
  en.writeWirePuzzle(puzzle);
  en.writeScratch(board);
  return en.exports.e2_score_board(board.length);
}

export interface SolverHandle {
  step(budget: number): SolverReport;
  report(): SolverReport;
  board(): Int32Array;
  bestBoard(): Int32Array;
  score(): number;
  bestScore(): number;
  reset(): void;
  free(): void;
}

function statusOf(code: number): SolverReport["status"] {
  switch (code) {
    case 1:
      return "solved";
    case 2:
      return "exhausted";
    default:
      return "running";
  }
}

function reportFor(en: Engine, slot: number): SolverReport {
  const ex = en.exports;
  return {
    status: statusOf(ex.e2_solver_status(slot)),
    nodes: ex.e2_solver_nodes(slot),
    attempts: ex.e2_solver_attempts(slot),
    backtracks: ex.e2_solver_backtracks(slot),
    placed: ex.e2_solver_placed(slot),
    bestPlaced: ex.e2_solver_best_placed(slot),
  };
}

export function createSolver(
  puzzle: Puzzle,
  path: Uint16Array,
  opts: { useHints?: boolean; shufflePieces?: boolean; seed?: number } = {},
): SolverHandle {
  const en = eng();
  const ex = en.exports;
  const useHints = opts.useHints ?? true;
  const shufflePieces = opts.shufflePieces ?? false;
  const seed = opts.seed ?? 0;

  en.writeWirePuzzle(puzzle);
  en.writeScratch(path);
  const slot = ex.e2_solver_new(
    path.length,
    useHints ? 1 : 0,
    shufflePieces ? 1 : 0,
    seed >>> 0,
  );
  if (slot < 0) throw new Error("failed to create solver (bad config or pool full)");

  let freed = false;
  return {
    step: (budget) => {
      ex.e2_solver_step(slot, budget);
      return reportFor(en, slot);
    },
    report: () => reportFor(en, slot),
    board: () => {
      const n = ex.e2_solver_board(slot);
      return en.readScratch(n);
    },
    bestBoard: () => {
      const n = ex.e2_solver_best_board(slot);
      return en.readScratch(n);
    },
    score: () => ex.e2_solver_score(slot),
    bestScore: () => ex.e2_solver_best_score(slot),
    reset: () => {
      ex.e2_solver_reset(slot);
    },
    free: () => {
      if (!freed) {
        ex.e2_solver_free(slot);
        freed = true;
      }
    },
  };
}
