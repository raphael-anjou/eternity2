# web — the Eternity II community site

React Router 7 (framework mode, `ssr:false`, static prerender) + Vite + Tailwind,
written in **strict TypeScript** (type-checked ESLint; `any`, non-null `!`,
`@ts-*` and floating promises are all banned, enforced in CI).

```sh
pnpm install
pnpm dev          # dev server
pnpm build        # static build into build/client
pnpm typecheck    # react-router typegen && tsc -b
pnpm lint         # eslint .
```

## The engine

The puzzle engine is **not** JavaScript. The canonical implementation is the
Rust crate in [`../engine`](../engine), compiled to WebAssembly; the only
JavaScript involved is the thin wasm-bindgen glue. The whole app imports it from
`@/engine` ([`src/engine/index.ts`](src/engine/index.ts)), which re-exports the
Rust/WASM backend in [`src/engine/backends/rust.ts`](src/engine/backends/rust.ts).
The shared `SolverHandle` type (the step-able solver surface) lives in
[`src/lib/types.ts`](src/lib/types.ts).

To rebuild the WASM from source: `just wasm` (or
`cd ../engine && wasm-pack build --target web --out-dir ../web/src/engine/pkg --release`).
The committed `pkg/` means `pnpm build` works on CI without a Rust toolchain.

### TS-only engine features

A couple of features that the Rust engine doesn't provide are implemented in
TypeScript and used at runtime regardless of the engine, in
[`src/engine-extras/`](src/engine-extras): the **block solver** (macro-piece
search, used by the paths playground) and the **complexity** estimator (used by
the complexity labs).

### The engine in other languages

The same engine has been reimplemented, faithfully, in several other languages —
TypeScript (zero WASM), C, C++, Python, Lua, COBOL, even Brainfuck. Those live in
the repo's [`../engine-ports/`](../engine-ports) collection, each validated
byte-for-byte against the Rust engine's golden output. They are a study exhibit,
**not** build backends: the site always runs the Rust/WASM engine.
