// Pure-TypeScript engine backend (ALGORITHM.md §9).
//
// Exposes the *same surface* as web/src/engine/index.ts (the WASM wrapper): the
// eight exported functions plus `createSolver` returning a `SolverHandle`. The
// website can swap between this and the WASM engine at build time. Since there
// is no WebAssembly module to load, `initEngine()` resolves immediately and the
// handle's `free()` is a no-op.
//
// Cross-references the language-agnostic spec in engine-side-quests/ALGORITHM.md
// and the Rust originals under engine/src/.

import type { Puzzle, SolverReport } from "@/lib/types";
import {
  generate,
  generateFramed,
  generateSolved,
  generateSolvedFramed,
  maxColors as maxColorsImpl,
} from "./generator.ts";
import { officialPuzzle } from "./official.ts";
import { buildPath, PATH_KINDS } from "./paths.ts";
import { Solver, scoreBoard } from "./solver.ts";

/**
 * Initialise the engine. The pure-TS backend has nothing to load, so this
 * resolves immediately; it exists only to match the WASM wrapper's surface.
 */
export function initEngine(): Promise<void> {
  return Promise.resolve();
}

export function getOfficialPuzzle(): Puzzle {
  return officialPuzzle();
}

export function getGeneratedPuzzle(size: number, colors: number, seed: number): Puzzle {
  return generate(size, colors, seed);
}

/** Pieces in solution order/orientation: piece i belongs at cell i, rot 0. */
export function getGeneratedSolvedPuzzle(size: number, colors: number, seed: number): Puzzle {
  return generateSolved(size, colors, seed);
}

/**
 * Framed variant: when `framed` is true, frame-restricted colours are confined
 * to the border band (mirrors real Eternity II). `framed=false` is identical to
 * `getGeneratedPuzzle`.
 */
export function getGeneratedPuzzleFramed(
  size: number,
  colors: number,
  seed: number,
  framed: boolean,
): Puzzle {
  return generateFramed(size, colors, seed, framed);
}

/** Solved-order counterpart of `getGeneratedPuzzleFramed`. */
export function getGeneratedSolvedPuzzleFramed(
  size: number,
  colors: number,
  seed: number,
  framed: boolean,
): Puzzle {
  return generateSolvedFramed(size, colors, seed, framed);
}

export function getMaxColors(size: number): number {
  return maxColorsImpl(size);
}

export function getPathKinds(): string[] {
  return [...PATH_KINDS];
}

export function getPath(kind: string, width: number, height: number, seed = 0): Uint16Array {
  const path = buildPath(kind, width, height, seed);
  if (path === null) {
    throw new Error(`unknown path kind: ${kind}`);
  }
  return path;
}

export function getBoardScore(puzzle: Puzzle, board: Int32Array): number {
  return scoreBoard(puzzle, board);
}

/** Same handle shape as web/src/engine/index.ts so the two are interchangeable. */
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
  const useHints = opts.useHints ?? true;
  const shufflePieces = opts.shufflePieces ?? false;
  const seed = opts.seed ?? 0;
  let solver = new Solver(puzzle, path, useHints, shufflePieces, seed);
  return {
    step: (budget) => solver.step(budget),
    report: () => solver.report(),
    board: () => solver.board(),
    bestBoard: () => solver.bestBoard(),
    score: () => scoreBoard(puzzle, solver.board()),
    bestScore: () => scoreBoard(puzzle, solver.bestBoard()),
    reset: () => {
      solver = new Solver(puzzle, path, useHints, shufflePieces, seed);
    },
    free: () => {
      // No WASM resource to release in the pure-TS backend.
    },
  };
}
