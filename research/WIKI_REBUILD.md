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

### Phase 1.5 — topic categories + research top nav (user directive 2026-07-01)

Readers should be able to browse by THEME, not only by door: big cross-cutting
categories with many pages under each. Examples the user named: improving
solver speed, reducing the search space, everything-DFS, GPU work, quantum
attempts (both exist in groups.io history). Design:

- [ ] `topics: [slug]` in frontmatter; a curated topic registry (label EN/FR,
      description, icon/color) — pages can belong to several topics
- [ ] Auto-generated topic hub pages (/research/topic/<slug>) listing member
      pages grouped by kind
- [ ] A top category bar inside the research shell (above the article, under
      the site header) for the big categories; sidebar stays the full tree
- [ ] Seed categories from vault themes + groups.io history (quantum, GPU,
      SAT/MIP, backtracking, local search, structure/theory, records)

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

### Phase 2 — search

- [ ] Build-time text index per language (strip MDX → text), lazy-loaded chunk
- [ ] minisearch + ⌘K dialog (site-styled), result grouping by section
- [ ] Manifest entries for non-MDX research pages (reference, log, records…)
      so they're findable too

### Phase 3 — full migration (page by page, commit per batch)

- [ ] why/ remaining 12 pages
- [ ] build/ (run-it-yourself, dead-ends, solvers)
- [ ] lab/experiments 12 invention pages (frontmatter carries score/board/
      repro; InventionLayout becomes the kind=invention template)
- [ ] papers, records (long prose → MDX; keep data-driven parts as components)
- [ ] hubs become manifest-driven (kill hand-maintained hub card lists)
- [ ] delete research-links.ts (related lives in frontmatter, validated),
      prune migrated seo.ts entries + routes.ts entries
- [ ] AGENTS.md + research/README.md updated to the new authoring flow

### Phase 4 — wiki growth from the vault (the actual point)

Sources: ../v2/vault (704 files). SITE_PLAN.md maps fates. Priorities:
- [ ] Known-facts reference page (from E2_KNOWN_FACTS.md — numbers researchers
      keep re-deriving)
- [ ] Research timeline/history enrichment (TIMELINE.md, 219 vols compressed)
- [ ] Concepts encyclopedia: curated subset of the 386 concept files
      (propagators, search, local-search families…), attribution-neutral
- [ ] Basins: case studies for all 17 boards (gallery has 7)
- [ ] Papers/math notes (rigidity theorem, σ-cycles, entropy) as long-form
      technical write-ups under lab/findings
- [ ] groups.io community links/history verification

### Phase 5 — researcher affordances

- [ ] Per-page raw markdown export (`.md` next to each page) + llms.txt
      regeneration (revisits the AGENTS.md index-only tradeoff: content is
      now markdown, so the old objection is gone)
- [ ] "Edit this page" / "View source" GitHub links
- [ ] Contribution guide for community researchers

## Working rules

- Commit locally as I go; **never push** (standing user directive).
- LEDGER.md entry per meaningful step.
- `cd web && pnpm typecheck && pnpm build` before every commit; lint clean.
- Attribution-neutral research voice; every claim reproducible or labelled.
- The old TSX pages keep working until their MDX replacement is verified —
  no reader-visible regression mid-migration.
