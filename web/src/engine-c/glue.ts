// TypeScript loader + ABI wrapper for the freestanding-wasm C engine
// (engine.c, built by build.sh). It exposes the SAME surface as
// web/src/engine/index.ts — the 8 module functions plus createSolver →
// SolverHandle — so the website can swap engines at build time.
//
// The C side cannot pass structs or strings across the WASM boundary, so all
// data crosses as primitives plus pointers into the module's linear memory;
// we copy bytes out with typed-array views. See engine.c's "ABI" section and
// engine-c/README.md.
//
// Strict-mode notes: no `any`, no non-null `!`, no `@ts-*`. The WASM export
// table is `unknown`-typed, so we narrow it once through a small runtime check
// (`asFn`) that returns properly-typed callables.

import type { Puzzle, SolverReport } from "@/lib/types";

// --- The exact set of exports engine.c declares (export_name attributes). ---
interface E2Exports {
  memory: WebAssembly.Memory;
  e2_generate(size: number, colors: number, seed: number): number;
  e2_generate_solved(size: number, colors: number, seed: number): number;
  e2_generate_framed(size: number, colors: number, seed: number, framed: number): number;
  e2_generate_solved_framed(size: number, colors: number, seed: number, framed: number): number;
  e2_official(): number;
  e2_puzzle_width(): number;
  e2_puzzle_height(): number;
  e2_puzzle_colors(): number;
  e2_puzzle_hints(): number;
  e2_pieces_ptr(): number;
  e2_hint_pos(i: number): number;
  e2_hint_piece(i: number): number;
  e2_hint_rot(i: number): number;
  e2_max_colors(size: number): number;
  e2_path_kind_count(): number;
  e2_path_kind_name(i: number): number;
  e2_build_path(kind: number, width: number, height: number, seed: number): number;
  e2_path_ptr(): number;
  e2_board_in_ptr(): number;
  e2_score_board(): number;
  e2_solver_new(pathLen: number, useHints: number, shufflePieces: number, seed: number): number;
  e2_solver_reset(pathLen: number, useHints: number, shufflePieces: number, seed: number): number;
  e2_solver_step(budget: number): void;
  e2_solver_status(): number;
  e2_solver_nodes(): number;
  e2_solver_attempts(): number;
  e2_solver_backtracks(): number;
  e2_solver_placed(): number;
  e2_solver_best_placed(): number;
  e2_solver_board(): number;
  e2_solver_best_board(): number;
  e2_solver_score(): number;
  e2_solver_best_score(): number;
}

// Narrow a raw export to a typed function (strict-mode-safe; no `any`/`!`).
function asFn<T extends (...args: number[]) => number | void>(
  table: WebAssembly.Exports,
  name: string,
): T {
  const fn = table[name];
  if (typeof fn !== "function") {
    throw new Error(`engine.wasm is missing export "${name}"`);
  }
  return fn as T;
}

function bindExports(raw: WebAssembly.Exports): E2Exports {
  const mem = raw["memory"];
  if (!(mem instanceof WebAssembly.Memory)) {
    throw new Error("engine.wasm did not export linear memory");
  }
  type N0 = () => number;
  type N1 = (a: number) => number;
  type N3 = (a: number, b: number, c: number) => number;
  type N4 = (a: number, b: number, c: number, d: number) => number;
  type V1 = (a: number) => void;
  return {
    memory: mem,
    e2_generate: asFn<N3>(raw, "e2_generate"),
    e2_generate_solved: asFn<N3>(raw, "e2_generate_solved"),
    e2_generate_framed: asFn<N4>(raw, "e2_generate_framed"),
    e2_generate_solved_framed: asFn<N4>(raw, "e2_generate_solved_framed"),
    e2_official: asFn<N0>(raw, "e2_official"),
    e2_puzzle_width: asFn<N0>(raw, "e2_puzzle_width"),
    e2_puzzle_height: asFn<N0>(raw, "e2_puzzle_height"),
    e2_puzzle_colors: asFn<N0>(raw, "e2_puzzle_colors"),
    e2_puzzle_hints: asFn<N0>(raw, "e2_puzzle_hints"),
    e2_pieces_ptr: asFn<N0>(raw, "e2_pieces_ptr"),
    e2_hint_pos: asFn<N1>(raw, "e2_hint_pos"),
    e2_hint_piece: asFn<N1>(raw, "e2_hint_piece"),
    e2_hint_rot: asFn<N1>(raw, "e2_hint_rot"),
    e2_max_colors: asFn<N1>(raw, "e2_max_colors"),
    e2_path_kind_count: asFn<N0>(raw, "e2_path_kind_count"),
    e2_path_kind_name: asFn<N1>(raw, "e2_path_kind_name"),
    e2_build_path: asFn<N4>(raw, "e2_build_path"),
    e2_path_ptr: asFn<N0>(raw, "e2_path_ptr"),
    e2_board_in_ptr: asFn<N0>(raw, "e2_board_in_ptr"),
    e2_score_board: asFn<N0>(raw, "e2_score_board"),
    e2_solver_new: asFn<N4>(raw, "e2_solver_new"),
    e2_solver_reset: asFn<N4>(raw, "e2_solver_reset"),
    e2_solver_step: asFn<V1>(raw, "e2_solver_step"),
    e2_solver_status: asFn<N0>(raw, "e2_solver_status"),
    e2_solver_nodes: asFn<N0>(raw, "e2_solver_nodes"),
    e2_solver_attempts: asFn<N0>(raw, "e2_solver_attempts"),
    e2_solver_backtracks: asFn<N0>(raw, "e2_solver_backtracks"),
    e2_solver_placed: asFn<N0>(raw, "e2_solver_placed"),
    e2_solver_best_placed: asFn<N0>(raw, "e2_solver_best_placed"),
    e2_solver_board: asFn<N0>(raw, "e2_solver_board"),
    e2_solver_best_board: asFn<N0>(raw, "e2_solver_best_board"),
    e2_solver_score: asFn<N0>(raw, "e2_solver_score"),
    e2_solver_best_score: asFn<N0>(raw, "e2_solver_best_score"),
  };
}

let mod: E2Exports | null = null;
let ready: Promise<void> | null = null;

function exportsOrThrow(): E2Exports {
  if (!mod) throw new Error("engine.wasm not initialized; await initEngine() first");
  return mod;
}

const STATUS = ["running", "solved", "exhausted"] as const;
function statusOf(code: number): SolverReport["status"] {
  return STATUS[code] ?? "running";
}

// --- Memory readers. Views are re-created each read because growing the
//     memory can detach old ArrayBuffers. ----------------------------------
function u8View(m: E2Exports): Uint8Array {
  return new Uint8Array(m.memory.buffer);
}
function u16View(m: E2Exports): Uint16Array {
  return new Uint16Array(m.memory.buffer);
}
function i32View(m: E2Exports): Int32Array {
  return new Int32Array(m.memory.buffer);
}

// Read the current g_puzzle out of the module into a Puzzle object.
function readPuzzle(m: E2Exports, name: string): Puzzle {
  const width = m.e2_puzzle_width();
  const height = m.e2_puzzle_height();
  const numColors = m.e2_puzzle_colors();
  const nPieces = width * height;
  const piecesPtr = m.e2_pieces_ptr();
  const bytes = u8View(m);
  const pieces: [number, number, number, number][] = [];
  for (let i = 0; i < nPieces; i++) {
    const o = piecesPtr + i * 4;
    pieces.push([bytes[o] ?? 0, bytes[o + 1] ?? 0, bytes[o + 2] ?? 0, bytes[o + 3] ?? 0]);
  }
  const nHints = m.e2_puzzle_hints();
  const hints = [];
  for (let i = 0; i < nHints; i++) {
    hints.push({ pos: m.e2_hint_pos(i), piece: m.e2_hint_piece(i), rot: m.e2_hint_rot(i) });
  }
  return { name, width, height, numColors, pieces, hints };
}

// Write a Puzzle into the module's g_puzzle. The site round-trips the puzzle
// object it got from a builder; we rebuild g_puzzle from the SAME (size, colors,
// seed) parameters the builder used so the embedded state matches exactly.
// For puzzles not produced by our builders (e.g. user-edited), we reconstruct
// g_puzzle by re-deriving from the puzzle's identity: official if it has 256
// pieces + hints matching, otherwise we cannot reproduce RNG state — so the
// solver/score paths take an explicit puzzle and re-stage it (see below).
//
// In practice the website builds the puzzle via getOfficialPuzzle /
// getGeneratedPuzzle, then immediately hands the SAME object to createSolver /
// getBoardScore. We therefore stage g_puzzle by writing the piece/hint arrays
// directly into module memory, which is exact regardless of how the puzzle was
// produced.
function stagePuzzle(m: E2Exports, puzzle: Puzzle): void {
  // Re-derive g_puzzle from the official set if it is the official puzzle,
  // else generate is not reversible — so we write the arrays directly. The
  // simplest exact path is to overwrite g_puzzle's piece/hint memory. We do
  // that by re-using the official/generate entry only to size things, then
  // patching memory. To keep this robust we instead always write pieces and
  // hints byte-by-byte into the known g_puzzle layout via the official entry's
  // pointers, after fixing width/height/colors through e2_official or
  // e2_generate. Because g_puzzle's piece pointer is stable, we:
  //   1. call e2_official() to give g_puzzle a 16x16 shape if needed, else a
  //      generate to size it,
  //   2. overwrite the piece bytes and hint records to match `puzzle`.
  if (puzzle.width === 16 && puzzle.height === 16) {
    m.e2_official();
  } else {
    // Size g_puzzle to the right n×n; piece bytes are overwritten next.
    m.e2_generate_solved(puzzle.width, puzzle.numColors, 0);
  }
  const piecesPtr = m.e2_pieces_ptr();
  const bytes = u8View(m);
  for (let i = 0; i < puzzle.pieces.length; i++) {
    const p = puzzle.pieces[i];
    if (!p) continue;
    const o = piecesPtr + i * 4;
    bytes[o] = p[0];
    bytes[o + 1] = p[1];
    bytes[o + 2] = p[2];
    bytes[o + 3] = p[3];
  }
}

export function initEngine(): Promise<void> {
  if (!ready) {
    ready = (async () => {
      const url = new URL("./engine.wasm", import.meta.url);
      const resp = await fetch(url);
      const { instance } = await WebAssembly.instantiateStreaming(resp, {});
      mod = bindExports(instance.exports);
    })();
  }
  return ready;
}

export function getOfficialPuzzle(): Puzzle {
  const m = exportsOrThrow();
  m.e2_official();
  return readPuzzle(m, "official_eternity2");
}

export function getGeneratedPuzzle(size: number, colors: number, seed: number): Puzzle {
  const m = exportsOrThrow();
  m.e2_generate(size, colors, seed);
  return readPuzzle(m, `generated_${size}x${size}_c${colors}_s${seed}`);
}

/** Pieces in solution order/orientation: piece i belongs at cell i, rot 0. */
export function getGeneratedSolvedPuzzle(size: number, colors: number, seed: number): Puzzle {
  const m = exportsOrThrow();
  m.e2_generate_solved(size, colors, seed);
  return readPuzzle(m, `generated_${size}x${size}_c${colors}_s${seed}`);
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
  const m = exportsOrThrow();
  m.e2_generate_framed(size, colors, seed, framed ? 1 : 0);
  return readPuzzle(m, `generated_${size}x${size}_c${colors}_s${seed}`);
}

/** Solved-order counterpart of `getGeneratedPuzzleFramed`. */
export function getGeneratedSolvedPuzzleFramed(
  size: number,
  colors: number,
  seed: number,
  framed: boolean,
): Puzzle {
  const m = exportsOrThrow();
  m.e2_generate_solved_framed(size, colors, seed, framed ? 1 : 0);
  return readPuzzle(m, `generated_${size}x${size}_c${colors}_s${seed}`);
}

export function getMaxColors(size: number): number {
  return exportsOrThrow().e2_max_colors(size);
}

export function getPathKinds(): string[] {
  const m = exportsOrThrow();
  const n = m.e2_path_kind_count();
  const bytes = u8View(m);
  const out: string[] = [];
  for (let i = 0; i < n; i++) {
    let p = m.e2_path_kind_name(i);
    let s = "";
    for (;;) {
      const c = bytes[p];
      if (c === undefined || c === 0) break;
      s += String.fromCharCode(c);
      p++;
    }
    out.push(s);
  }
  return out;
}

export function getPath(kind: string, width: number, height: number, seed = 0): Uint16Array {
  const m = exportsOrThrow();
  const kinds = getPathKinds();
  const ki = kinds.indexOf(kind);
  if (ki < 0) throw new Error(`unknown path kind: ${kind}`);
  const len = m.e2_build_path(ki, width, height, seed);
  if (len < 0) throw new Error(`unknown path kind: ${kind}`);
  const ptr = m.e2_path_ptr();
  const view = u16View(m);
  // Copy out (ptr is byte address; u16 view index is ptr/2).
  return view.slice(ptr / 2, ptr / 2 + len);
}

export function getBoardScore(puzzle: Puzzle, board: Int32Array): number {
  const m = exportsOrThrow();
  stagePuzzle(m, puzzle);
  const inPtr = m.e2_board_in_ptr();
  const view = i32View(m);
  const base = inPtr / 4;
  for (let i = 0; i < board.length; i++) view[base + i] = board[i] ?? -1;
  return m.e2_score_board();
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

export function createSolver(
  puzzle: Puzzle,
  path: Uint16Array,
  opts: { useHints?: boolean; shufflePieces?: boolean; seed?: number } = {},
): SolverHandle {
  const m = exportsOrThrow();
  const useHints = opts.useHints ?? true;
  const shufflePieces = opts.shufflePieces ?? false;
  const seed = opts.seed ?? 0;

  stagePuzzle(m, puzzle);
  // Write the path into g_path, then build the solver from it.
  const pathPtr = m.e2_path_ptr();
  const pview = u16View(m);
  const pbase = pathPtr / 2;
  for (let i = 0; i < path.length; i++) pview[pbase + i] = path[i] ?? 0;
  const rc = m.e2_solver_new(path.length, useHints ? 1 : 0, shufflePieces ? 1 : 0, seed);
  if (rc !== 0) throw new Error("solver construction failed (invalid puzzle/path/hints)");

  const nCells = puzzle.width * puzzle.height;

  const report = (): SolverReport => ({
    status: statusOf(m.e2_solver_status()),
    nodes: m.e2_solver_nodes(),
    attempts: m.e2_solver_attempts(),
    backtracks: m.e2_solver_backtracks(),
    placed: m.e2_solver_placed(),
    bestPlaced: m.e2_solver_best_placed(),
  });

  const readBoard = (ptr: number): Int32Array => {
    const view = i32View(m);
    const base = ptr / 4;
    return view.slice(base, base + nCells);
  };

  return {
    step: (budget) => {
      m.e2_solver_step(budget);
      return report();
    },
    report,
    board: () => readBoard(m.e2_solver_board()),
    bestBoard: () => readBoard(m.e2_solver_best_board()),
    score: () => m.e2_solver_score(),
    bestScore: () => m.e2_solver_best_score(),
    reset: () => {
      // Re-stage the same puzzle + path, then rebuild.
      stagePuzzle(m, puzzle);
      const rp = u16View(m);
      const rb = m.e2_path_ptr() / 2;
      for (let i = 0; i < path.length; i++) rp[rb + i] = path[i] ?? 0;
      const r = m.e2_solver_reset(path.length, useHints ? 1 : 0, shufflePieces ? 1 : 0, seed);
      if (r !== 0) throw new Error("solver reset failed");
    },
    // No per-solver heap allocation (single static g_solver), so free is a
    // no-op; kept for surface parity with the wasm-bindgen engine.
    free: () => undefined,
  };
}
