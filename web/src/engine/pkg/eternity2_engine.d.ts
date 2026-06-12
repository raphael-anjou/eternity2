/* tslint:disable */
/* eslint-disable */

export class WasmSolver {
    free(): void;
    [Symbol.dispose](): void;
    bestBoard(): Int32Array;
    bestScore(): number;
    /**
     * Cell -> piece_id*4+rotation, or -1 while empty.
     */
    board(): Int32Array;
    constructor(puzzle: any, path: Uint16Array, use_hints: boolean, shuffle_pieces: boolean, seed: number);
    report(): any;
    reset(): void;
    score(): number;
    /**
     * Run up to `budget` placements/backtracks; returns a Report object.
     */
    step(budget: number): any;
}

export function buildPath(kind: string, width: number, height: number, seed: number): Uint16Array;

export function generatePuzzle(size: number, colors: number, seed: number): any;

/**
 * Pieces in solution order/orientation: piece i belongs at cell i, rot 0.
 */
export function generateSolvedPuzzle(size: number, colors: number, seed: number): any;

export function maxColors(size: number): number;

export function officialPuzzle(): any;

export function pathKinds(): string[];

export function scoreBoard(puzzle: any, board: Int32Array): number;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_wasmsolver_free: (a: number, b: number) => void;
    readonly buildPath: (a: number, b: number, c: number, d: number, e: number) => [number, number, number, number];
    readonly generatePuzzle: (a: number, b: number, c: number) => any;
    readonly generateSolvedPuzzle: (a: number, b: number, c: number) => any;
    readonly maxColors: (a: number) => number;
    readonly officialPuzzle: () => any;
    readonly pathKinds: () => [number, number];
    readonly scoreBoard: (a: any, b: number, c: number) => [number, number, number];
    readonly wasmsolver_bestBoard: (a: number) => [number, number];
    readonly wasmsolver_bestScore: (a: number) => number;
    readonly wasmsolver_board: (a: number) => [number, number];
    readonly wasmsolver_new: (a: any, b: number, c: number, d: number, e: number, f: number) => [number, number, number];
    readonly wasmsolver_report: (a: number) => any;
    readonly wasmsolver_reset: (a: number) => void;
    readonly wasmsolver_score: (a: number) => number;
    readonly wasmsolver_step: (a: number, b: number) => any;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __externref_drop_slice: (a: number, b: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
