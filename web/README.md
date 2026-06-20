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

## The engine, and the build-time engine switch

The puzzle engine is **not** JavaScript. The canonical implementation is the
Rust crate in [`../engine`](../engine), compiled to WebAssembly; the only
JavaScript involved is the thin wasm-bindgen glue. The site has **four
interchangeable engine backends**, all implementing the identical surface and
all validated byte-for-byte against the same Rust reference data
(see [`../engine-side-quests/ALGORITHM.md`](../engine-side-quests/ALGORITHM.md)):

| `VITE_ENGINE` | Backend | Notes |
| --- | --- | --- |
| `rust` *(default)* | [`src/engine/backends/rust.ts`](src/engine/backends/rust.ts) | the canonical Rust engine → WASM (~156 KB) |
| `ts` | [`src/engine-ts/`](src/engine-ts) | a **pure-TypeScript** port — **zero WASM** |
| `c` | [`src/engine-c/`](src/engine-c) | a C port → WASM via clang (~14 KB) |
| `cpp` | [`src/engine-cpp/`](src/engine-cpp) | a C++ port → WASM via clang++ (~16 KB) |

Pick the backend at build (or dev) time:

```sh
pnpm build                  # default: rust/WASM
VITE_ENGINE=ts  pnpm build  # pure TypeScript, no WASM at all
VITE_ENGINE=c   pnpm build  # C → WASM
VITE_ENGINE=cpp pnpm build  # C++ → WASM
VITE_ENGINE=ts  pnpm dev    # works in dev too
```

### How it works

The whole app imports the engine from `@/engine`. That module
([`src/engine/index.ts`](src/engine/index.ts)) re-exports a virtual module
`virtual:engine-backend`, which `vite.config.ts` resolves to the backend chosen
by `VITE_ENGINE`. So switching engines changes **one alias** and nothing else —
no app code is aware of which engine it's running. An unknown `VITE_ENGINE`
fails the build with a clear message.

The shared `SolverHandle` type (the step-able solver surface every backend
returns) lives in [`src/lib/types.ts`](src/lib/types.ts).

### Rebuilding the WASM backends

The committed `.wasm` files mean `pnpm build` works on CI **without** a C/C++ or
Rust toolchain. To rebuild them from source:

- Rust: build the wasm package for `../engine` (see that crate's README).
- C: [`src/engine-c/build.sh`](src/engine-c/build.sh) (needs a wasm-capable
  `clang` + `wasm-ld`, e.g. Homebrew LLVM + `lld`).
- C++: [`src/engine-cpp/build.sh`](src/engine-cpp/build.sh) (clang++ + `wasm-ld`).

### Why this exists

A maintainer worried the Rust+WASM+React stack was hard to maintain. The `ts`
backend is the direct answer: a fully-functional, **zero-WASM, pure-TypeScript**
build of the exact same engine. The C/C++ backends show the engine isn't tied to
Rust either. All of them stay honest because they're checked against the same
golden data on every run (`node src/engine-*/parity.mjs`).
