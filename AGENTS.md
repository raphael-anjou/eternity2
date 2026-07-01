# AGENTS.md — notes for AI agents working on this repo

The Eternity II community site: a Rust→WASM solver engine (`engine/`) plus a
static React Router 7 site (`web/`). Static-only, no server, bilingual (English
at the root, French under `/fr`). See `README.md` for the user-facing tour and
`DEPLOYMENT.md` for build/deploy details.

## Conventions & gotchas

- **Single source of truth for the page list.** Crawlable page paths live in
  `web/sitemap.config.ts` (`PAGE_PATHS` + `allRoutePaths()`). Both
  `web/react-router.config.ts` (the prerender list) and the `emit-sitemap` Vite
  plugin in `web/vite.config.ts` import from it. **Add new pages there**, not by
  re-declaring an array — an inline page list will silently drift from the
  sitemap. Note React Router applies `basename` itself, so the prerender list
  uses *unprefixed* paths (`allRoutePaths()`), while the sitemap applies
  `BASE_PATH` (`allRoutePaths(base)`) because it emits absolute URLs.

- **Crawler/SEO files live in two places.** `web/public/robots.txt` and
  `web/public/llms.txt` are static (copied verbatim to the build root).
  `sitemap.xml` is *generated* at build time — do not add a static one to
  `public/`. The sitemap reads `VITE_SITE_ORIGIN`/`BASE_PATH`; `robots.txt`
  hardcodes the `eternity2.dev` sitemap URL.

- **`llms.txt` is index-only by design.** We deliberately do **not** generate
  per-page `.md` counterparts. Page content is authored in interactive React/TSX
  (playground, live solvers), so HTML→Markdown would produce thin or meaningless
  stubs. The prerendered HTML is clean enough for agents to parse. Don't add a
  per-page `.md` build step without revisiting that trade-off.

- **`ssr: false` build runs twice.** Vite emits a client bundle and a temporary
  server bundle (removed afterward). Build plugins that write files must gate on
  the client output dir (`options.dir.includes("client")`) or they fail trying
  to write into the deleted server dir.

- **`research/` holds reproducible research topics.** Each topic under
  `research/topics/<id>/` is self-contained: `article.md` (YAML frontmatter is the
  single source of truth for its metadata), a `compute/` crate/script, and
  committed `results/`. `node research/build-index.mjs` regenerates+validates
  `research/index.json` (the registry; fails if an `id` mismatches its dir or a
  declared result file is missing — usable as a CI gate). See `research/README.md`.

- **Site reference-table data is a copy of a research result.** `web/src/data/
  reference-table.json` is copied from `research/topics/subgrid-placement-counts/
  results/reference-table.json` (the site imports from `src/`). If you regenerate
  the research result, copy it across again — there is no automatic sync.

- **The generator has an optional frame-restricted-colors mode.** `generate`/
  `generate_solved` (Rust) and their TS/C/C++ mirrors keep their original
  signatures (flag off = byte-for-byte identical output, parity-tested). The
  framed variants — `generate_framed(size,colors,seed,framed)` /
  `generate_solved_framed(...)`, exposed to JS as `getGeneratedPuzzleFramed` /
  `getGeneratedSolvedPuzzleFramed` and as wasm exports `e2_generate_framed` /
  `e2_generate_solved_framed` — confine the first `min(5, colors-1)` colors to
  the border band and the rest to the deep interior (real-E2 behavior; only
  active for size ≥ 4 and colors ≥ 2, else falls back to the unrestricted
  painter). The Viewer's board generator surfaces this as a Switch. Any change
  to the generator must stay byte-for-byte across all four ports; run the
  `parity.mjs` harnesses + `cargo test` and rebuild the C/C++/Rust wasm.

- **The research section is an MDX wiki.** Content lives in
  `web/content/research/**` — one `.mdx` file per page (`page.mdx` = EN,
  `page.fr.mdx` = FR; `index.mdx` = a directory's hub). **Adding an MDX file IS
  the registration**: the scanner (`web/content.config.ts`) feeds the prerender
  list + sitemap, the Vite plugin (`web/plugins/research-content.ts`) builds
  per-language manifests, and the catch-all `research/*` routes render pages in
  the docs shell (`web/src/components/docs/`) with sidebar, TOC, breadcrumbs,
  prev/next, search (⌘K) and topic hubs. Frontmatter is zod-validated and
  fails the build loudly: title, description, kind (finding | experiment |
  tool | reference | concept | basin | paper | page), order, updated, topics
  (validated against `content/research/topics.json`), sources[] (every claim
  links its evidence), related[] (site paths), repro {kind, cmd, topic}, and
  score for experiments. Interactive components are imported directly inside
  MDX (`@/components/...`) — per-page code splitting is preserved, and prerender
  waits for them (full prose ends up in the static HTML). Do NOT add research
  pages to `routes.ts`/`seo.ts` — those are only for non-research pages now.

- **Research writing rules.** French pages are *written*, never translated
  literally. The project's own algorithms are **experiments** (never
  "inventions") under "Raphaël's explorations" — one researcher's notebook,
  structurally equal to other researchers' work (Bucas, Blackwood, McGavin).
  Every number is fact-checked against `research/topics/*/results` before
  publication; `repro.cmd` should be a real `just research-<topic>` recipe.

- **Keep the research ledger.** `research/LEDGER.md` is an append-only,
  oldest-first record of every research-section step: findings, pages,
  refutations, decisions, and what was learned (including failures and inert
  results). This is multi-day work, so the ledger is the cross-session memory.
  Add an entry for each meaningful step, with the outcome/learning, the
  reproducibility kind, the commit hash, and the files touched. Never rewrite past
  entries.

- **Every published result must be reproducible.** Findings/inventions/boards
  are published through the `research/topics/<id>/` pipeline (see the `research/`
  bullet above): a `compute/` crate (path-dep on `engine/`), committed
  `results/`, a copy-pasteable `reproduce:` command in the frontmatter, and a
  GitHub "computed from…" link on the page. Deterministic results reproduce
  bit-for-bit; stochastic or long-running searches still ship their script plus
  the board they produced (verifiable in `/viewer`), labelled honestly
  (e.g. "stochastic — won't reproduce exactly" / "~N h on 8 cores"). No result
  on the site is an unbacked claim.

- **Write research content attribution-neutral.** State findings and methods
  plainly as the project's research. Do not describe the process that produced
  them.

- **Papers and records are MDX + view components.** `content/research/
  papers.mdx` and `records.mdx` wrap `web/src/components/research/views/
  {Papers,Records}View.tsx` (data-heavy TSX stays TSX; the MDX wrapper carries
  the metadata). Both are sourced from the research vault at
  `../../v2/vault/reference/` (`academic-references.md`,
  `community-e2-history.md`). When adding a paper, verify the URL resolves
  (prefer institutional-repository / HAL / DIAL / arXiv links).

- **`justfile` wraps the common tasks.** `just` lists them; `just setup`,
  `just dev`, `just test`, `just wasm`, `just build`, `just check`, and the
  per-topic `just research-<topic>` reproduce recipes are thin wrappers over the
  underlying cargo/pnpm/node commands (no logic of their own). Keep it in sync
  when commands change; the raw commands still work without `just`.

- **Verify after web changes:** `cd web && pnpm build && pnpm typecheck`.
