// The default engine backend: the canonical Rust engine compiled to
// WebAssembly via wasm-bindgen. This is the fast, production backend; the
// alternative backends (../../engine-ts, ../../engine-c, ../../engine-cpp)
// implement the identical surface and are selected at build time by the
// VITE_ENGINE switch (see web/README.md and vite.config.ts).
//
// All four backends export the same eight functions + createSolver, so the
// rest of the app imports from "@/engine" and never knows which one it got.

import init, {
  WasmSolver,
  buildPath,
  generatePuzzle,
  generateSolvedPuzzle,
  maxColors,
  officialPuzzle,
  pathKinds,
  scoreBoard,
} from "../pkg/eternity2_engine";
import wasmUrl from "../pkg/eternity2_engine_bg.wasm?url";
import type { Puzzle, SolverReport, SolverHandle } from "@/lib/types";

let ready: Promise<void> | null = null;

export function initEngine(): Promise<void> {
  if (!ready) {
    ready = init({ module_or_path: wasmUrl }).then(() => undefined);
  }
  return ready;
}

export function getOfficialPuzzle(): Puzzle {
  return officialPuzzle() as Puzzle;
}

export function getGeneratedPuzzle(size: number, colors: number, seed: number): Puzzle {
  return generatePuzzle(size, colors, seed) as Puzzle;
}

/** Pieces in solution order/orientation: piece i belongs at cell i, rot 0. */
export function getGeneratedSolvedPuzzle(size: number, colors: number, seed: number): Puzzle {
  return generateSolvedPuzzle(size, colors, seed) as Puzzle;
}

export function getMaxColors(size: number): number {
  return maxColors(size);
}

export function getPathKinds(): string[] {
  return pathKinds();
}

export function getPath(kind: string, width: number, height: number, seed = 0): Uint16Array {
  return buildPath(kind, width, height, seed);
}

export function getBoardScore(puzzle: Puzzle, board: Int32Array): number {
  return scoreBoard(puzzle, board);
}

export function createSolver(
  puzzle: Puzzle,
  path: Uint16Array,
  opts: { useHints?: boolean; shufflePieces?: boolean; seed?: number } = {},
): SolverHandle {
  const solver = new WasmSolver(
    puzzle,
    path,
    opts.useHints ?? true,
    opts.shufflePieces ?? false,
    opts.seed ?? 0,
  );
  return {
    step: (budget) => solver.step(budget) as SolverReport,
    report: () => solver.report() as SolverReport,
    board: () => solver.board(),
    bestBoard: () => solver.bestBoard(),
    score: () => solver.score(),
    bestScore: () => solver.bestScore(),
    reset: () => solver.reset(),
    free: () => solver.free(),
  };
}
