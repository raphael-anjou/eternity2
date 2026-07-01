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

### Phase 1 — pipeline + docs shell + pilot page ✅ when checked

- [ ] Deps: @mdx-js/rollup, @mdx-js/react, remark-gfm, remark-math,
      remark-frontmatter, remark-mdx-frontmatter, rehype-katex, rehype-slug,
      @shikijs/rehype, gray-matter, zod, github-slugger, @tailwindcss/typography
- [ ] `web/content.config.ts`: fs scan of `web/content/research/**/*.mdx`,
      zod frontmatter schema, slug/lang derivation, TOC extraction. Shared by
      sitemap + vite plugin.
- [ ] Vite plugin `virtual:research-manifest/{en,fr}` (+ HMR on content edits)
- [ ] `sitemap.config.ts`: merge scanned MDX paths; throw on overlap with
      static PAGE_PATHS (guards double-registration during migration)
- [ ] Catch-all routes `research/*` + `fr/research/*` → doc page (explicit
      routes win, so migration is page-by-page)
- [ ] Docs shell: sidebar tree, TOC (scrollspy), breadcrumbs, prev/next,
      badges, FR-fallback notice, related rail from frontmatter
- [ ] MDX component map: headings, language-aware links, code, tables,
      callouts; KaTeX css
- [ ] Pilot: migrate `why/border-balance` (EN+FR) end-to-end
- [ ] Verify: typecheck, lint, build; prerendered HTML contains full prose
      (EN + FR); sitemap has the path exactly once; visual pass in dev

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
