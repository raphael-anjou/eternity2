# Eternity II community site

**Live at [eternity2.dev](https://eternity2.dev)**

An open-source hub for the [Eternity II](https://en.wikipedia.org/wiki/Eternity_II_puzzle)
puzzle: play it, watch real solvers run **in your browser**, learn the algorithms behind
it, view/share community boards, and dig into the research. Available in English and
French.

**Everything is static.** The solver is a Rust engine compiled to WebAssembly; there is
no server. Host it on any static host, or run the provided Docker image (one stateless
nginx container).

## What's inside

| Section | What it does |
| --- | --- |
| **The Puzzle** | History (Eternity I → II), piece-set anatomy, all 256 pieces, the 22 motifs and their rarity, the 5 official clues, record table, complexity numbers |
| **Playground / Solve** | Solve generated 3×3–5×5 puzzles against the clock (drag and drop, or tap on mobile); on finish, the engine solves the same puzzle and reports how many times it could have done so meanwhile |
| **Playground / Watch** | A live, speed-controllable DFS with real stats (placements, backtracks, nodes/s) on generated puzzles or the official 16×16 |
| **Playground / Paths** | Draw a custom cell-visit order (mouse or finger) and race it against the classic orders on the same puzzle, all lanes to the finish |
| **Algorithms** | DFS and backtracking explained for high-schoolers: a scrubbable slow-motion 3×3 demo, the exponential wall (with all 557 zeros written out), live binary demos, difficulty charts measured by the engine itself |
| **Board Viewer** | Imports any `e2.bucas.name` or `eternity2.dev` URL (board_edges/board_pieces/motifs_order) and exports the canonical `eternity2.dev` link; live scoring, conflict marks, verification card (official set, duplicates, clues n/5), famous boards (467/468/469/470), and a solvable-board generator |
| **Format converter** | Paste any board and read back every format — the canonical `eternity2.dev` URL, the one board JSON every solver emits, `board_edges`/`board_pieces`, `e2pieces.txt`, puzzle CSV, and the legacy bucas link — with a live preview; see the [format reference](https://eternity2.dev/research/build/formats) |
| **Research** | Three doors: *Why it's hard* (the design and the structural walls, e.g. forbidden patterns), *Build a solver* (validation data, papers, records, and how to run the code), and *The lab notebook* (original findings, all reproducible from source) |

## Quick start (local development)

Prereqs: Rust stable + `wasm32-unknown-unknown` target, `wasm-pack`, Node 24+, pnpm
(via `corepack enable`). Details in [CONTRIBUTING.md](CONTRIBUTING.md).

With [`just`](https://just.systems) installed, `just setup && just dev` does the whole
first build and starts the dev server; `just` on its own lists every task (test, wasm,
build, check, and the research reproduce recipes). The raw commands are below if you
prefer them.

```bash
# 1. Engine → WASM (output lands in web/src/engine/pkg)
cd engine
cargo test --release                 # cross-validates against real e2.bucas.name boards
wasm-pack build --target web --out-dir ../web/src/engine/pkg --release

# 2. Web app
cd ../web
pnpm install
pnpm dev                             # http://localhost:5173
pnpm build                           # production build in web/build/client
```

Optional engine extras:

```bash
cargo run --release --bin stats > ../web/src/data/difficulty.json   # regenerate charts data
cargo run --release --bin bench                                     # native nodes/s benchmark
```

## Deployment

Two supported paths, both documented in [DEPLOYMENT.md](DEPLOYMENT.md):

- **Static hosting (recommended)**: pushing to the default branch auto-deploys to
  GitHub Pages via `.github/workflows/deploy.yml` (which also uploads the built site
  as a downloadable `web-dist` artifact); or upload `web/build/client/` to any CDN.
- **Docker**: `docker compose up -d --build` → static site on port 8080, served by one
  stateless nginx container. To serve under a reverse-proxy path prefix, build with
  `BASE_PATH=/prefix` (the prefix is baked into the prerendered routes and absolute
  asset paths, so the proxy passes it straight through — no `StripPrefix`); ready-made
  Traefik labels are in `docker-compose.yaml`.

## Repository layout

```
engine/   Rust crate (eternity2-engine): puzzle types, official piece set,
          seeded n×n generator, path-driven step-able DFS, wasm-bindgen API,
          `stats` and `bench` native bins. Tests cross-validate the bucas
          interop (piece set, rotations, clues, scoring) on real boards.
web/      React 19 + TypeScript + Vite + Tailwind v4 + shadcn/ui frontend.
          web/src/engine/pkg is the wasm-pack output (committed for now).
          web/src/i18n holds the EN/FR language context; each page colocates
          its translations. Research wiki prose lives in web/content/research
          (MDX); prerendered to build/client by react-router.
research/  Reproducible research backing the wiki. topics/<id>/ holds shared
          theory (compute + committed results); experiments/<author>/ holds one
          researcher's own runs, some with a self-contained runnable engine
          (e.g. the single-core benchmark). See research/README.md.
deploy/   nginx config template used by the Docker image.
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for setup, the project's load-bearing conventions
(URDL edge order, rotation math, bucas interop invariants, i18n pattern), and testing
expectations.

## Interop & conventions (summary)

- Edge order is **URDL** (up, right, down, left) everywhere, matching the bucas URL encoding.
- Color 0 = grey border; interior colors 1..22 map to bucas letters `b`..`w` (`'a' + color`).
- Rotation r = clockwise quarter-turns; `rotated(e, r)[i] = e[(i + 4 − r) % 4]`.
- Board cells are encoded `pieceId*4 + rotation` (or −1 when empty).
- The engine test suite cross-validates all of this against the real McGavin/Blackwood 469
  board and the official Clues board bundled with the bucas viewer (including `motifs_order`
  letter translation).

## License & attribution

All original code in this repository is **MIT** (see `LICENSE`), the most permissive
mainstream license: anyone can reuse the engine, the viewer, the playground, anything,
for any purpose.

One file is third-party: the 22 motif SVG definitions in `web/src/lib/motifs.ts` come from
[jfbucas/eternityII-viewer](https://github.com/jfbucas/eternityII-viewer)
(© 2019 Jef Bucas, **GPL-3.0**, see `LICENSES/GPL-3.0.txt`). Because the built site bundles
that artwork, the deployed site as a whole is governed by GPL-3.0; every other file can be
taken under MIT on its own. (If the motifs are ever redrawn as original art, the whole
project becomes pure MIT.)

Eternity II is a trademark of its rights holders; this is a non-commercial fan/community
project.
