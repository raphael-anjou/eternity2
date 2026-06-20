# Eternity II engine — C++ / WebAssembly port

A from-scratch **C++** re-implementation of the Eternity II engine, compiled to
**freestanding `wasm32`** with `clang++` (no Emscripten, no libc, no STL heap).
It is byte-for-byte identical to the canonical Rust engine and is a drop-in,
build-time-switchable backend for the community website: `glue.ts` exposes the
exact same surface as [`web/src/engine/index.ts`](../engine/index.ts).

The algorithm itself is **not** documented here — it lives once, authoritatively,
in [`engine-side-quests/ALGORITHM.md`](../../../engine-side-quests/ALGORITHM.md).
`engine.cpp` is a literal translation of that document and of the Rust source in
[`engine/src/`](../../../engine/src), cross-referenced section by section in the
comments. Read those two together.

## Files

| File | What it is |
| --- | --- |
| `engine.cpp` | The whole engine: RNG, generator, embedded official set, paths, step-able solver, scorer, and the C ABI exports. Also compiles natively (debug aid). |
| `engine.wasm` | The built artifact (committed, so CI needs no C++ toolchain). ~15 KB. |
| `build.sh` | Reproducible build: `clang++` → wasm object → `wasm-ld` link → optional `wasm-opt`. |
| `glue.ts` | TypeScript loader/wrapper. Same 8 functions + `createSolver → SolverHandle` as the Rust backend. Passes the project's strict typecheck and type-checked ESLint. |
| `parity.mjs` | Node-22 validation: runs `engine.wasm` and checks every line of `golden.txt`. |

## Rebuilding

```sh
bash web/src/engine-cpp/build.sh
```

Apple's `/usr/bin/clang++` has **no** wasm32 codegen target, so the script uses
Homebrew LLVM clang (`brew install llvm`) to compile, and links with whatever
wasm linker it can find — a standalone `wasm-ld`, or the `rust-lld` that ships
with any `rustup` toolchain (driven in wasm flavor). Override the compiler with
`CLANGXX=/path/to/clang++`.

Exact compile + link (what `build.sh` runs):

```sh
clang++ --target=wasm32 -nostdlib -ffreestanding -fno-exceptions -fno-rtti \
        -O3 -std=c++20 -Wall -Wextra -c engine.cpp -o engine.o
wasm-ld --no-entry --export-dynamic --allow-undefined \
        --initial-memory=2097152 -o engine.wasm engine.o
wasm-opt -O3 engine.wasm -o engine.wasm   # optional
```

Each ABI function is annotated `__attribute__((export_name(...)))` so the
exports survive both dead-code elimination and the `wasm-opt` pass.

## Validating

```sh
node web/src/engine-cpp/parity.mjs        # "99 passed, 0 failed"
```

Optional native cross-check (re-emits golden.txt and diffs it):

```sh
clang++ -O2 -std=c++20 -DE2_NATIVE_MAIN web/src/engine-cpp/engine.cpp -o /tmp/e2
/tmp/e2 | diff - <(grep -v '^$' engine-side-quests/engine-lua/golden.txt)
```

## The ABI

The module has **no imports** — it is a flat block of linear memory plus numeric
C exports. Data crosses the boundary through two shared `i32` regions whose base
pointers the engine reports:

- `e2_wire_ptr()` → a **puzzle** wire buffer:
  `[width, height, numColors, nPieces, (u,r,d,l)×nPieces, nHints, (pos,piece,rot)×nHints]`.
- `e2_scratch_ptr()` / `e2_scratch_len()` → a generic `i32` region for **paths**
  and **boards** (`cell → piece*4+rot | -1`).

Exports:

| Export | Effect |
| --- | --- |
| `e2_official()` | write the official set into the wire buffer; returns piece count |
| `e2_generate(size, colors, seed)` | write a scrambled solvable puzzle; returns piece count |
| `e2_generate_solved(size, colors, seed)` | same, in solution order |
| `e2_max_colors(size)` | largest usable colour count |
| `e2_build_path(kind, w, h, seed)` | write a path permutation into scratch; returns length (kind = index into the 9 path names; `-1` if unknown) |
| `e2_score_board(nCells)` | matched interior edges of the scratch board against the wire puzzle |
| `e2_solver_new(pathLen, useHints, shufflePieces, seed)` | create a solver from the wire puzzle + scratch path; returns a pool slot (`-1` on failure) |
| `e2_solver_step(slot, budget)` | run up to `budget` placements/backtracks |
| `e2_solver_status/placed/best_placed(slot)` | int report fields |
| `e2_solver_nodes/attempts/backtracks(slot)` | counters, returned as `f64` so they stay exact past 2³² |
| `e2_solver_board/best_board(slot)` | copy that board into scratch; returns nCells |
| `e2_solver_score/best_score(slot)` | score that board (uses the solver's own puzzle) |
| `e2_solver_reset(slot)` / `e2_solver_free(slot)` | restart / release a slot |

A small fixed pool (`E2_MAX_SOLVERS = 8`) backs `createSolver`/`free`. All state
lives in static buffers sized for the official 16×16 (256 cells) — the largest
puzzle the site runs — so there is no heap and no allocation at runtime.

## Parity guarantee

`parity.mjs` checks generated puzzles (RNG), the official set + hints, every
path permutation, and full solver runs **down to the exact node / attempt /
backtrack counts** — including the 50 000-step official probe (~21.3 M attempts).
Matching the counts proves this engine walks the identical search tree as the
Rust one, not merely that it also solves.
