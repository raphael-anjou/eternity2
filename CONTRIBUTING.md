# Contributing

Thanks for helping build the Eternity II community site! This page gets you productive
quickly and lists the conventions that keep the codebase coherent.

## Development setup

Prerequisites:

| Tool | Version | Notes |
| --- | --- | --- |
| Rust | stable (1.80+) | with the `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown` |
| wasm-pack | 0.13+ | `curl -sSfL https://rustwasm.github.io/wasm-pack/installer/init.sh \| sh` |
| Node.js | 22+ | |
| pnpm | 9+ | `corepack enable` |

First build:

```bash
# 1. Engine: test, then compile to WASM (output goes straight into the web app)
cd engine
cargo test --release
wasm-pack build --target web --out-dir ../web/src/engine/pkg --release

# 2. Web app
cd ../web
pnpm install
pnpm dev          # http://localhost:5173
```

You only need to rebuild the WASM when you touch `engine/`; pure frontend work is the
usual Vite hot-reload loop.

## Repository map

```
engine/                 Rust crate (also compiles natively)
  src/types.rs          Piece/Puzzle types, rotation math
  src/official.rs       official 256-piece set (embedded CSV) + the 5 clues
  src/generator.rs      seeded solvable-puzzle generator
  src/solver.rs         step-able backtracking DFS (the heart)
  src/paths.rs          built-in cell-visit orders
  src/bin/stats.rs      generates web/src/data/difficulty.json
  src/bin/bench.rs      native single-core throughput benchmark
  tests/bucas_interop.rs  cross-validation against real community boards
web/
  src/engine/           WASM pkg + typed wrapper (initEngine, createSolver, ...)
  src/lib/              bucas URL codec, board audit, motif table, shared types
  src/components/board/ BoardSvg, PieceSvg, motif <defs>, bucas actions
  src/components/learn/ self-running educational demos
  src/pages/            one file per route
  src/i18n/             the language context (see i18n below)
deploy/                 nginx config for the Docker image
```

## Conventions (load-bearing; please keep them)

These invariants are cross-validated by `engine/tests/bucas_interop.rs` against real
community boards. If you change one, that suite will tell you.

- **Edge order is URDL** (up, right, down, left) everywhere, matching the e2.bucas.name
  URL encoding.
- **Color 0 is the grey border**; interior colors 1..22 map to bucas letters `'b'..'w'`
  (`'a' + color`).
- **Rotation r = clockwise quarter-turns**: `rotated(e, r)[i] = e[(i + 4 - r) % 4]`.
- **Board cells encode `pieceId*4 + rotation`**, or `-1` when empty.
- **Piece numbers shown to users are 1-based** (piece ids in code are 0-based).
- **No em dashes** in any user-visible text or code comment, in any language. Use
  commas, colons, semicolons or new sentences.
- **Search metrics are called "nodes"** in the UI (one node = one candidate
  piece-rotation tested on a square), and rates are "nodes/s".
- The engine has **no clocks and no u64 across the JS boundary** (`std::time::Instant`
  panics on wasm32; JS numbers can't hold u64). Counters cross as f64; the browser does
  all timing.

## Internationalization (English + French)

Every page colocates its strings:

```tsx
const T = {
  en: { title: "Hello", solved: (n: number) => `Solved in ${n}!` },
  fr: { title: "Bonjour", solved: (n: number) => `Résolu en ${n} !` },
};
// inside the component
const t = useT(T);
```

Rules: both languages must have an identical dictionary shape (TypeScript enforces
usage); JSX fragments and functions are fine as values; French is written for the
general public including young students ("vous" form, science-museum register, never a
literal translation); path-kind names (`row-major`, `snake`, ...) stay in English in
both languages.

## Testing expectations

- `cargo test --release` in `engine/` must pass. If you touch piece data, rotation
  logic, the generator, or scoring, the bucas interop suite is your safety net.
- `pnpm build` in `web/` must pass (it runs `tsc` first).
- For UI changes, exercise the affected page in the browser, including the French
  toggle and a narrow viewport. Touch devices have no right-click and no HTML5 drag
  and drop; the playground supports tap + long-press flows, so keep those working.

## Licensing note for contributions

All original code is MIT. The single GPL-3.0 file is `web/src/lib/motifs.ts` (motif
artwork data © Jef Bucas). Don't copy code from the bucas viewer into other files;
reimplement from behavior instead, so the MIT/GPL boundary stays clean.
