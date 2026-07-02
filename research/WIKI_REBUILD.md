# Research wiki rebuild — working plan + TODO

The research section becomes the **official Eternity II research wiki**: a
docs-grade reading experience (sidebar tree, search, TOC, breadcrumbs,
prev/next) with markdown-first authoring, while keeping every live
animation/WASM demo. This file is the working plan and checklist; LEDGER.md
records what actually happened; SITE_PLAN.md maps vault content → pages.

## The problem being solved

- 35 hand-written TSX pages (~5,100 lines): prose is trapped in EN/FR JS
  dictionaries inside JSX. Editing an article means editing React. Diffs are
  unreadable, translators are blocked, and vault content (704 md files,
  ~470k words) can't flow in.
- Adding one page touches 5 files (routes.ts, seo.ts, page, research-links.ts,
  Hub card) and nothing validates the cross-references.
- A reader has hub cards and a related-rail — no sidebar, no search, no TOC,
  no way to see the whole territory.

## Decision (ADR): custom MDX pipeline, headless, no docs framework

Considered:

1. **Mintlify** — hosted/proprietary, needs their platform. Violates
   static-only/self-contained. Rejected.
2. **Fumadocs (full: fumadocs-ui + core + mdx)** — officially supports React
   Router, but its example targets RR 8 + `ssr:true` with server loaders and
   its UI owns the shell (own look, own root provider). Our site is RR 7.17,
   `ssr:false` + full prerender, custom EN-root//fr i18n, custom sitemap
   single-source, strong existing design. Wholesale adoption fights all of
   that. Rejected for now — revisit if the site ever moves to RR8/ssr:true.
3. **Headless custom pipeline (chosen)** — `@mdx-js/rollup` + remark/rehype
   (gfm, math+KaTeX, slug, shiki) + a small Vite plugin that scans
   `web/content/research/**` frontmatter (zod-validated, fails the build
   loudly) into a virtual manifest. The manifest drives EVERYTHING: sidebar
   tree, hubs, breadcrumbs, prev/next, related rail, SEO meta, sitemap/
   prerender paths, search index. One MDX file per language
   (`page.mdx` + `page.fr.mdx`), FR falls back to EN with a notice, and
   interactive TSX components are imported directly inside MDX (per-page code
   splitting preserved).

Why: registration burden drops from 5 files to **1 content file**; prose
becomes markdown (readable diffs, translator-friendly, vault-portable);
zero framework lock-in; the docs shell is built from the site's own
shadcn/Tailwind language so research still feels like eternity2.dev.

## Target reading experience

- Left: full research tree (grouped: Why / Build / Lab / Reference), status +
  kind badges, active-page highlight, collapsible, mobile drawer.
- Right: on-page TOC with scrollspy.
- Top: breadcrumbs. Bottom: prev/next + related rail.
- ⌘K search over all research content (build-time index, client-side
  minisearch, per-language).
- Frontmatter-driven page chrome: kind/tier/repro badges, "reproduce this"
  block with command + GitHub link, updated date.
- Every page keeps/get its animation (the "alive KB" directive).

## Phases

### Phase 1 — pipeline + docs shell + pilot page ✅ DONE 2026-07-01

- [x] Deps: @mdx-js/rollup, @mdx-js/react, remark-gfm, remark-math,
      remark-frontmatter, remark-mdx-frontmatter, rehype-katex, rehype-slug,
      @shikijs/rehype, gray-matter, zod, github-slugger, @tailwindcss/typography
- [x] `web/content.config.ts`: fs scan of `web/content/research/**/*.mdx`,
      zod frontmatter schema (incl. `sources[]` + `topics[]`), slug/lang
      derivation, TOC extraction. Shared by sitemap + vite plugin.
- [x] Vite plugin `virtual:research-manifest-{en,fr}` (+ HMR on content edits)
- [x] `sitemap.config.ts`: merge scanned MDX paths; throw on overlap with
      static PAGE_PATHS (guards double-registration during migration)
- [x] Catch-all routes `research/*` + `fr/research/*` → doc page (explicit
      routes win, so migration is page-by-page)
- [x] Docs shell: sidebar tree, TOC (scrollspy), breadcrumbs, prev/next,
      badges, FR-fallback notice, sources block, related rail from frontmatter
- [x] MDX component map: headings, language-aware links, code, tables,
      callouts; KaTeX css
- [x] Pilot: migrated `why/border-balance` (EN+FR) end-to-end
- [x] Verify: typecheck, lint, build green; prerendered HTML contains full
      prose + KaTeX in EN and FR (lazy MDX resolves during prerender);
      sitemap symmetric (47 EN + 47 FR + SPA); visual pass in dev (3-column
      desktop, mobile disclosure, live Ns1Lab works inside MDX)

Learned: YAML bare dates parse as Date (schema accepts both); rolldown
binding breaks on pnpm add → `pnpm install --force`; react-refresh wants
component-only modules (mdx-map.ts split); heading anchors need opacity
utilities (prose link color beats transparent-color trick).

### Phase 1.5 — topic categories + research top nav ✅ DONE 2026-07-01

- [x] `topics: [slug]` frontmatter + curated registry (topics.json, 11 themes)
- [x] Auto topic hubs at /research/topics/<slug> + index (EN+FR prerendered)
- [x] Research subnav (Overview / doors / Topics tab); chip strip cut on user
      feedback — the Topics tab is enough; sidebar scoped to active section

### User directives (standing, from 2026-07-01)

- **Double-check numbers/facts** against research/topics data or the vault
  whenever they exist; don't propagate unverified figures.
- **Every claim links its supporting resource** (groups.io post, paper,
  research/topics compute) — `sources:` frontmatter list + inline links.
- **French is written, not translated**: natural idiomatic FR, never literal.
  Improve existing literal FR passages whenever a page is touched.
- **Two-level navigation**: the left sidebar must NOT carry the whole wiki.
  A research sub-navbar (under the site header) carries the doors + topic
  categories; the sidebar shows only the active section's tree.
- **Experiments, not inventions** (2026-07-01): the project's own algorithms
  are "experiments" (we can't be sure they're inventions) and must NOT be the
  wiki's core topic. Frame them as one researcher's work — "Raphaël's
  exploration" — structurally parallel to other researchers' work (Jef
  Bucas's Blackwood study, Blackwood, McGavin…). kind: experiment in the
  schema; prose in that voice.

### Community-contributed sources (to publish, with permission)

- **Jef Bucas — Blackwood algorithm notes + parameter study**:
  https://github.com/jfbucas/wrapper_blackwood/blob/main/doc/Notes/Notes.md
  Explicit permission to copy/rephrase/improve for the site
  (https://groups.io/g/eternity2/message/11905). His wrapper_blackwood
  project varied each parameter of Joshua Blackwood's algorithm
  (server distributes jobs, clients compile+run variations, statistics back)
  and concluded Blackwood's manual tweaks were near-optimal. He offers his
  samples and NOTES he'd like independent validation — a perfect
  reproducible-research collaboration for /research/build/solvers/blackwood
  (cite both links; credit Jef).

### Phase 2 — search ✅ DONE 2026-07-01

- [x] Build-time text index per language (strip MDX → text), lazy-loaded chunk
      (virtual:research-search-{en,fr} — own chunk, loads on first ⌘K)
- [x] minisearch + ⌘K dialog (site-styled, keyboard nav, fuzzy+prefix,
      body snippets), search button in the research subnav
- [x] Legacy TSX pages searchable via title+description (full text arrives
      as they migrate)

### Repro blocks must never be empty ✅ DONE 2026-07-02 (user report)

- [x] justfile: `research-<topic>` recipes for all 8 missing topics
      (phase-transition byte-identical rerun verified)
- [x] every compute-backed MDX page carries `repro.cmd` + `repro.topic`
- [x] board-backed experiments (palimpsest/keyring/gauntlet/prior) link the
      record-boards verification recipe
- [x] shell: repro without cmd/topic renders a compact one-line note, not a box
- [x] complex-theory got a proper research/topics article.md (index was
      failing since the reference port landed)

### Phase 3 — full migration ✅ DONE 2026-07-02

- [x] why/ 13 pages (2 stale figures corrected during fact-check)
- [x] build/ run-it-yourself, dead-ends, solvers (first index.mdx hub)
- [x] lab/experiments 12 experiment pages (InventionLayout deleted; RecordBoard
      component; experiments-not-inventions voice)
- [x] papers, records, reference, log → MDX wrappers + view components
- [x] all hubs manifest-driven (index.mdx + auto child cards); overview too
- [x] research-links.ts + RelatedRail deleted; STATIC_LEAVES gone — the MDX
      manifest is the only source of truth; seo.ts/routes.ts hold only
      non-research pages
- [x] AGENTS.md rewritten for the new authoring flow

### New user directives (2026-07-02)

- **Animations must never block navigation**: clicking a link while a live
  lab runs must navigate immediately. Audit every research lab/animation
  component: time-boxed per-frame budgets (measure with performance.now(),
  ~10ms cap, never step-count-based), stop work when unmounted AND when
  scrolled out of view / tab hidden, no synchronous WASM bursts on the main
  thread longer than a frame.
- **Experiments log page removed** (was /research/lab/experiments/log): the
  auto-extracted one-line entries are not proper research artifacts. The
  extracted data + topic stay in the repo; future curation may turn selected
  entries into real write-ups (dead-ends candidates, per-experiment notes) —
  but nothing ships until it reads as a researched artifact.

### Community archive mining (user directive 2026-07-02, IN PROGRESS)

The FULL groups.io archive is local: `../v2/community-exports/messages.jsonl`
(11,511 messages, 2000→2026, with public msg_num → citable URLs). Directive:
sweep chronologically from the oldest messages toward today. Pipeline +
digest format + privacy rules: research/community/README.md; digests land in
research/community/digests/ (windows 0001–0003 launched: →2007-07,
2007-08→10, 2007-11→2008-02). Wiki pages (history, sourced records, concept
attributions) are synthesized FROM digests. Note: groups.io itself is
login-walled for tools (402) — the local archive is the source of truth;
the groups.io WIKI section still needs the user's browser session to mine.

### Phase 4 — wiki growth from the vault (IN PROGRESS)

Sources: ../v2/vault (704 files). SITE_PLAN.md maps fates. Status:
- [x] Known-facts reference page (/research/build/known-facts — every number
      with provenance; internal vault material excluded)
- [x] Blackwood solver page (/research/build/solvers/blackwood — Jef Bucas's
      notes + parameter study, with permission, full credit)
- [x] History: "The hunt" parts I+II (2000→2026) synthesized from the full
      community-archive sweep (better than the vault TIMELINE — primary
      sources throughout); records fully sourced and CORRECTED (470 ceiling)
- [x] Concepts encyclopedia v1 (9 techniques incl. parity-arguments) +
      community benchmarks page + Blackwood decoded & enriched
- [ ] Concepts encyclopedia: curated subset of the 386 concept files
      (propagators, search, local-search families…), attribution-neutral
- [x] Notable boards gallery (lab/boards) — public-sourced version: the
      record line + strict line + project boards, previews embedded, every
      claim carrying its announcement message. (Deep per-board case studies
      from vault analyses remain future work — vault-sensitive.)
- [ ] Papers/math notes (rigidity theorem, σ-cycles, entropy) as long-form
      technical write-ups under lab/findings
- [ ] groups.io community links/history verification

### Phase 5 — researcher affordances ✅ DONE 2026-07-02

- [x] Per-page raw markdown export (78 .md siblings) + llms.txt rewrite
- [x] "Page source" GitHub + "View as Markdown" links in the shell footer
- [x] Contribution guide (/research/contribute, in the Build section)

## Working rules

- Commit locally as I go; **never push** (standing user directive).
- LEDGER.md entry per meaningful step.
- `cd web && pnpm typecheck && pnpm build` before every commit; lint clean.
- Attribution-neutral research voice; every claim reproducible or labelled.
- The old TSX pages keep working until their MDX replacement is verified —
  no reader-visible regression mid-migration.
