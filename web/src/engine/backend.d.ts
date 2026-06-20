// Type surface of the build-time-selected engine backend.
//
// `virtual:engine-backend` is resolved by Vite (see vite.config.ts) to one of
// the four interchangeable backends (rust/ts/c/cpp) based on VITE_ENGINE. They
// all implement exactly this surface; this declaration lets `tsc` type-check
// `src/engine/index.ts` (which re-exports the virtual module) without knowing
// which concrete backend is wired in.
declare module "virtual:engine-backend" {
  import type { Puzzle, SolverHandle } from "@/lib/types";

  export function initEngine(): Promise<void>;
  export function getOfficialPuzzle(): Puzzle;
  export function getGeneratedPuzzle(size: number, colors: number, seed: number): Puzzle;
  export function getGeneratedSolvedPuzzle(size: number, colors: number, seed: number): Puzzle;
  export function getMaxColors(size: number): number;
  export function getPathKinds(): string[];
  export function getPath(kind: string, width: number, height: number, seed?: number): Uint16Array;
  export function getBoardScore(puzzle: Puzzle, board: Int32Array): number;
  export function createSolver(
    puzzle: Puzzle,
    path: Uint16Array,
    opts?: { useHints?: boolean; shufflePieces?: boolean; seed?: number },
  ): SolverHandle;
}
