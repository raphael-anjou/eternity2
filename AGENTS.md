# AGENTS.md — notes for AI agents working on this repo

The Eternity II community site: a Rust→WASM solver engine (`engine/`) plus a
static React Router 7 site (`web/`). Static-only, no server, trilingual (English
at the root, French under `/fr`, Spanish under `/es`). See `README.md` for the
user-facing tour and
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

- **Site-verification files must be committed to `public/`.** `BingSiteAuth.xml`
  (Bing Webmaster Tools) and the IndexNow key `c7259bc712de2f2d86298cf28e768b82.txt`
  prove domain ownership by being served from the site root. They live in
  `web/public/` so the build copies them every deploy. A verification file
  uploaded straight to the deployed branch instead is silently deleted by the
  next deploy (the Pages artifact only contains what the build emits), which
  unverifies the property without any warning. If you add a Google Search
  Console HTML-file verification, commit it here too, or prefer the DNS TXT
  method which no deploy can clobber.

- **Crawler/SEO files: static vs generated.** Only `web/public/robots.txt` is
  static (copied verbatim to the build root), alongside the ownership-proof
  files above. `sitemap.xml`, `llms.txt` and `llms-full.txt` are all *generated*
  at build time — do not add static copies to `public/`. The sitemap reads
  `VITE_SITE_ORIGIN`/`BASE_PATH`; `robots.txt` hardcodes the `eternity2.dev`
  sitemap URL.

- **`llms.txt` + research `.md` siblings, one set PER LANGUAGE.**
  `plugins/research-content.ts` emits, for every registry language, three
  machine-readable artifacts at build time: `llms.txt` (the curated header — the
  `LLMS_HEADER` record in that plugin, NOT a file in `public/` — followed by a
  generated map of that language's research pages), `llms-full.txt` (that
  language's whole corpus in one file), and a raw-markdown sibling per research
  page at the same URL with `.md` appended. English sits at the root
  (`/llms.txt`, `/research/x.md`), every other language under its prefix
  (`/fr/llms.txt`, `/fr/research/x.md`). A page is exported in language L only
  when `doc.translated` says it genuinely renders in L, the same predicate
  `researchPagePathsFor` uses for the sitemap and hreflang, so a `.md` never
  publishes English prose under a translated URL. The body always comes from that
  language's own source file (`doc.file` points at the `<slug>.<lang>.mdx`
  sidecar), so the export carries the written translation. Prose strings around
  the body (the header field names, the two folded-in link headings, the section
  titles) come from `LABELS` / `SECTION_LABELS` in the same plugin — add a
  language there when you add one to the registry. Non-research pages remain
  TSX-only (no `.md` stubs for them; prerendered HTML is clean enough).

- **Every URL these emit must be canonical.** Internal links in `llms.txt`,
  in `llms-full.txt` and in the `.md` siblings go through the plugin's
  `canonical()` helper (trailing slash) so an agent following them never pays a
  301. Same rule as the app's `canonicalPath` in `web/src/site.ts`. Known gap:
  relative links written inside MDX prose (`](/research/x)`) are still emitted
  verbatim into the `.md` exports and so still redirect; fixing that needs a
  markdown-aware rewrite, not a regex over prose.

- **`ssr: false` build runs twice.** Vite emits a client bundle and a temporary
  server bundle (removed afterward). Build plugins that write files must gate on
  the client output dir (`options.dir.includes("client")`) or they fail trying
  to write into the deleted server dir.

- **`research/` holds reproducible research.** Shared theory lives under
  `research/topics/<id>/`, each self-contained: `article.md` (YAML frontmatter is the
  single source of truth for its metadata), a `compute/` crate/script, and
  committed `results/`. `node research/build-index.mjs` regenerates+validates
  `research/index.json` (the registry; fails if an `id` mismatches its dir or a
  declared result file is missing — usable as a CI gate). Measured runs (shared
  facilities any researcher's engine can join) live under
  `research/experiments/<experiment>/` and may carry a self-contained runnable
  engine plus a `just` module (root: `mod experiments 'research/experiments/justfile'`,
  so `just experiments` lists and `just experiments <name>` runs). Who ran it and
  on what hardware is recorded on the write-up page's `author:`/`hardware:` fields.
  See `research/README.md`.

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
  `page.fr.mdx` = FR, `page.es.mdx` = ES; `index.mdx` = a directory's hub; the
  parity check gates that sidecars mirror the EN frontmatter). **Adding an MDX file IS
  the registration**: the scanner (`web/content.config.ts`) feeds the prerender
  list + sitemap, the Vite plugin (`web/plugins/research-content.ts`) builds
  per-language manifests, and the catch-all `research/*` routes render pages in
  the docs shell (`web/src/components/docs/`) with sidebar, TOC, breadcrumbs,
  prev/next, search (⌘K) and topic hubs. Frontmatter is zod-validated and
  fails the build loudly: title, description, kind (finding | experiment |
  tool | reference | concept | basin | paper | page), order, updated, topics
  (validated against `content/research/topics.json`), sources[] (every claim
  links its evidence), related[] (site paths), repro {kind, cmd, topic}, and
  score for experiments. Optional but render-bearing (2026-07-21): `outcome`
  (plateaued|refuted|parked|new-basin|superseded), `scoringConvention`
  (matched-edges|strict-5-clue — set it whenever `score` is set),
  `repro.produces` (search|artifact: artifact = the command only re-verifies a
  stored board; defaults to artifact when `repro.topic` is `record-boards`),
  `repro.scope` (one line: what the command does and does not reproduce), and
  `stages[]` (rendered as the pipeline strip; `published: false` marks engines
  without their own write-up). Interactive components are imported directly inside
  MDX (`@/components/...`) — per-page code splitting is preserved, and prerender
  waits for them (full prose ends up in the static HTML). Do NOT add research
  pages to `routes.ts`/`seo.ts` — those are only for non-research pages now.
  **`sources[].url` is zod `.url()` — absolute URLs only.** A relative internal
  path (`/research/...`) fails the build. To cite one of the site's own
  experiments as evidence for a number, put the page in `related[]` (which takes
  site paths) and add an inline prose link at the claim; never a relative
  `sources[]` url.

- **Research writing rules.** French pages are *written*, never translated
  literally. The project's own algorithms are **experiments** (never
  "inventions") under "Raphaël's explorations" — one researcher's notebook,
  structurally equal to other researchers' work (Bucas, Blackwood, McGavin).
  Every number is fact-checked against `research/topics/*/results` before
  publication; `repro.cmd` should be a real `just research-<topic>` recipe.
  **A CI style gate (`pnpm --dir web check:research`, script
  `check-research-style.mjs`) fails on ANY em dash (—) outside code/quotes and
  on a banned-phrase list (e.g. "honest"/"honestly").** Join clauses with a
  period, semicolon, colon, or parentheses instead of an em dash; verbatim
  archive quotes are exempt. `check:research` also validates that every
  groups.io `message/N` citation exists in the local archive
  (`../../research/community-exports/messages.jsonl`), so never invent a `msg_num` —
  grep the archive for the number before citing it.

- **`/research/records` is the canonical SOTA page.** The headline numbers
  (community 470 open / 465 strict, the project's own 463/460s) live there in
  the "where this project stands" table. New or edited pages LINK to
  `/research/records` (and `/research/open-problems` for the frontier) instead
  of hand-restating the numbers — the pre-rework site had them independently
  restated in ~9 files, which is exactly the drift this rule prevents.

- **Some research routes are generated, not MDX.** `/research/glossary`,
  `/research/build/reproduce`, and `/research/lab/experiments/by-contribution/*`
  are components registered in `content.config.ts` (`researchPagePaths()` /
  `researchPagePathsFor()`) plus the switch in `web/src/pages/research/doc.tsx`.
  Follow that pattern for new computed surfaces; a new MDX page costs ×3
  languages, a generated route costs one trilingual string table.

- **Parallel agents in this repo: partition by file, build once.** Concurrent
  full `pnpm build` runs collide on the output dir; agents should verify with
  `pnpm typecheck` + `pnpm check:research:ci` only, and the orchestrating
  session runs the single full build at the end. Give each agent an explicit
  disjoint file-ownership list (shared files like `content.config.ts`,
  `types.ts`, `DocsShell.tsx` go to one sequential agent first).

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
  `../../research/vault/reference/` (`academic-references.md`,
  `community-e2-history.md`). When adding a paper, verify the URL resolves
  (prefer institutional-repository / HAL / DIAL / arXiv links).

- **`justfile` wraps the common tasks.** `just` lists them; `just setup`,
  `just dev`, `just test`, `just wasm`, `just build`, `just check`, and the
  per-topic `just research-<topic>` reproduce recipes are thin wrappers over the
  underlying cargo/pnpm/node commands (no logic of their own). Keep it in sync
  when commands change; the raw commands still work without `just`.

- **Verify after web changes:** `cd web && pnpm build && pnpm typecheck`.
