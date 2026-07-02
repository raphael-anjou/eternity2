# Research section build ledger

A running, ordered record of the work on the site's research section: every
experiment, finding, page, and learning, oldest first. This is multi-day work
across many topics, so this file is the memory that survives between sessions.
Append here as you go; never rewrite history. Each entry is dated and points at
the commit and the files it produced.

Conventions for entries:

- **One entry per meaningful step** (a finding, a page, a refutation, a decision).
- Record the **outcome and the learning**, not just "did X". If something failed
  or was inert, say so plainly: a recorded dead end is as valuable as a result.
- Note **reproducibility**: deterministic (reproduces byte-for-byte), stochastic
  (script + committed output, labelled), or prose (no compute).
- Link the **commit hash** and the **paths** touched.

---

## Plan and conventions (the spine)

- The research section is three doors: **Why it's hard**, **Build a solver**,
  **The lab notebook**. Folders mirror routes at arbitrary depth; every level has
  a `Hub.tsx`. See `AGENTS.md`.
- Every published result is reproducible. Findings go through the
  `research/topics/<id>/` pipeline (compute crate + committed results +
  `reproduce` command + GitHub link). Deterministic results reproduce
  byte-for-byte; stochastic or long ones ship the script and the board (checkable
  in the viewer), labelled honestly.
- Prose is accessible and human; findings are stated plainly without describing
  how they were produced.
- Verify before every commit: `just check` (engine tests, typecheck, lint,
  build), and for a research topic, `node research/build-index.mjs`.

---

## Log

### 2026-06-22 — Restructure: three doors

The flat `/research` became a three-door chooser (`Why` / `Build` / `Lab`) with a
folder tree that mirrors routes at any depth, so the leaf-heavy sections can grow
without re-homing. Existing pages (reference/papers/records) re-homed under Build;
their URLs unchanged. Not-yet-written children render as "In preparation" cards,
never dead links.

- Reproducibility: prose/structure.
- Commit `e1c4d28`.
- Files: `web/src/pages/Research.tsx`, `web/src/pages/research/{why,build,lab}/Hub.tsx`,
  `routes.ts`, `sitemap.config.ts`, `seo.ts`, `AGENTS.md`.

### 2026-06-22 — Finding: Forbidden patterns (first lab-notebook content)

Among the 196 interior pieces, almost every small placement is impossible. Exact,
exhaustive counts over distinct-piece placements (no sampling): two side by side
38.96% forbidden, L of three 83.26%, 2x2 square **99.72%** (1,427,039,544 of
1,431,033,240 forbidden). Horizontal and vertical pair counts come out identical,
the expected sign of a set with no directional bias. A correct board has zero
forbidden patches, so the count is a distance-to-valid signal.

- Reproducibility: deterministic; ~20s; verified byte-for-byte identical across
  two runs.
- Commit `41e0280`.
- Topic: `research/topics/forbidden-patterns/`. Page:
  `web/src/pages/research/why/forbidden-patterns.tsx`. Data:
  `web/src/data/forbidden-patterns.json`.
- Learning: the right population is the 196 interior pieces with distinct-piece
  placements; the vault's earlier 0.282% figure sampled a different population.
  The exact count (~0.28% feasible) is cleaner and reproduces.

### 2026-06-22 — DX: justfile + "Run it yourself" page

Made clone-build-run a few named commands and surfaced the path on the site.
`just` wraps the real cargo/pnpm/node commands plus per-topic reproduce recipes.
New page `/research/build/run-it-yourself`. Validated `just --list`,
`just research-index`, and `just research-forbidden-patterns` (identical output).

- Reproducibility: tooling/prose.
- Commit `855250a`.
- Files: `justfile`, `web/src/pages/research/build/run-it-yourself.tsx`,
  `README.md`, `AGENTS.md`.

### 2026-06-22 — Finding: Tuned to the hardness peak (phase transition)

The official set's 22 colors split **17 interior** to **5 frame-only** (colors
1-5 are the rare frame colors; 6-22 are interior), read straight from the pieces:
4 corners + 56 edges + 196 interiors. That 17 is the published difficulty peak for
framed edge-matching puzzles (about one expected solution), so the parameters sit
on the worst spot for search by design. Citations: Mateu/Hamadi 2012 (Constraints),
Ansotegui et al.

- Reproducibility: deterministic; instant; verified identical across runs.
- Commit `8c39c65`.
- Topic: `research/topics/phase-transition/`. Page:
  `web/src/pages/research/why/phase-transition.tsx`. Data:
  `web/src/data/phase-transition.json`.

### 2026-06-22 — Page: Dead ends (Build door)

Eight general approaches that don't crack E2, each tagged "proven" or "measured":
symmetry breaking (void: no symmetric pieces, center clue pins orientation), more
compute, survey propagation (grid has short cycles, messages flatten), belief-
propagation move ordering (near-uniform, random does as well), tensor-network
counting (can't enforce piece-uniqueness; ~10^90 vs ~1), relaxation bounds (~478
ceiling vs ~458 real; gap too large), learned heuristics (collapse across the
size/color gap), enumerate-clusters-first (counts blow up: ~10^12 four-corner
tuples). Stated as plain facts about the approaches, no internal references.

- Reproducibility: prose (drawn from prior refutations).
- Page: `web/src/pages/research/build/dead-ends.tsx`. Build hub card live.
- Learning: kept only general approaches a newcomer would try; dropped items that
  were specific to private 459-basin work (basin-mix MIP, high-T MCMC from 459).

### 2026-06-22 — Inventions section + verified record boards

Built /research/lab/inventions (hub + PALIMPSEST 463, KEYRING 460, GAUNTLET 458)
with a shared InventionLayout and in-viewer board previews. New topic
research/topics/record-boards recomputes each board's matched-edge score from its
edges (463/460/458 all verified == claimed). Boards link via native eternity2.dev
params, not bucas URLs (user directive). Stochastic searches → ship the board,
not the run. Commit `772cf1f`.

### 2026-06-22 — Community infra moved; phase-transition reuses measured data

Community infrastructure moved to the /research hub (off the Build door, user
directive). The phase-transition "wall" chart now reuses the existing measured
difficulty.json (the same data Algorithms charts) instead of re-solving live —
caught that a live sweep would just recompute what we already have. Page reframed
to explain HOW the 17/5 claim is established and what it impacts, not assert it.
Commit `5bfb711`.

### 2026-06-22 — Master content plan (SITE_PLAN.md)

Stepped back and mined the whole vault with parallel agents (219 vols, 384
concepts, 17 basins). Wrote research/SITE_PLAN.md: every experiment mapped to
PUBLISH / SUPPORT / INTERNAL with its viz and repro kind, plus a "not published"
section so nothing is lost. This is now the build map. Door 1 (why) has a much
bigger hardness-evidence stack than first built: rigidity wall, sigma-cycles,
entropy/area-law, no-forced-moves, rare-color geography, watershed, depth-40,
NS-1. Commit `2b87969`.
Learnings recorded: small-board live sweeps duplicate difficulty.json; the
"alive" budget is best spent on viz the site doesn't already have (rigidity halo
animation, sigma-cycle overlay, entropy collapse curve, forbidden-patch heatmap).

### 2026-06-22 — /why findings: rigidity wall, sigma-cycles, no forced moves (loop)

Started a 2h self-paced loop (cron 55d3e9eb). Built the flagship hardness findings:
- rigidity-wall: every record board locally frozen (MIP, halo-4, 3 boards). Animated
  halo growing over the verified 463 board with mismatches marked. repro=heavy(MIP),
  board verifiable.
- sigma-cycles: basin-hopping impossible — one 154-cell interlocking cycle, every
  subset scores worse. Interactive "apply part of the loop, seam breaks" diagram.
  Cross-linked with rigidity (the two escape routes, both closed).
- no-forced-moves: NEW exact topic. Every interior piece has 73-137 right-partners,
  ZERO forced. Histogram chart. The flip side of forbidden-patterns (local freedom +
  no global consistency). repro=exact, deterministic.
Commits `262dc5f`, `04fbf5f`. Also fixed a stale dev-server (port 5173 squat) that
threw a "file not found" for sigma-cycles; restarted clean on 5174.
Remaining /why: entropy/area-law (#18), rare-color-geography (#19 other half).
Then lab inventions/basins (#20), solver catalogue (#21).

### 2026-06-22 — Math rendering (KaTeX) + animations + entropy theorem

Added KaTeX (`<Math>`/`<MathBlock>`, SSR-rendered into prerendered HTML). Note:
importing `Math` shadows the global Math object — import as `MathInline` on pages
that use `Math.exp` etc.
- forbidden-patterns: NEW live "draw four pieces" animation (real pieces, real
  2x2 feasibility, converges to 99.72%, speed slider) + a "where 99.72% comes
  from" math section.
- entropy-area-law (#18): NEW finding + topic. Recomputes grammar entropy exactly
  for widths 1,2 (transfer-matrix eigenvalue) + lambda_H=46.18; h(n) decay chart;
  rho(n) area-law collapse (log axis); the Fekete-subadditivity theorem in KaTeX.
  Carries h(3)/h(4)/h_inf≈0.67 from offline sweep, labelled.
Commits `f25231c`, `d4802d0`. **User said STOP PUSHING to GitHub — committing
locally only now ([[feedback_e2_site_repo_main_only]]).** Also: `pnpm add katex`
broke the rolldown native binding (dangling optional-dep symlink); fixed with
`pnpm install --force`.

### 2026-06-22 — rare-color geography; Door 1 (Why) complete

NEW finding + topic rare-color-geography: 5 rare colors (1-5) only on the border
ring, each exactly 24 edges, 0 interior; board-ring diagram + swatches + stacked
frame/interior bar chart. Exact, deterministic. Commit `efcdec9`.
**Door 1 (Why) now COMPLETE — 7 findings:** phase-transition, rigidity-wall,
sigma-cycles, forbidden-patterns, no-forced-moves, rare-color-geography,
entropy-area-law. All reproducible, all with a viz; entropy has full KaTeX math.
Next: Lab (#20) — remaining inventions (LADDER/REPLAY/PRIOR/STAGED/BANDSAW/
CLOISTER/MIDDEN), basins gallery, findings hub; then solver catalogue (#21).

### 2026-06-22 — Lab door completed: basins gallery + findings index

- /research/lab/basins: notable-boards gallery — 3 verified project boards
  (463/460/458) + 4 community (467/468/469/470), sorted by score, each opens in
  viewer via native params (bucas board_w/h + motifs_order stripped). Commit
  `59eac42`.
- /research/lab/findings: grouped index of the 7 structural results (design
  signatures / structural walls / rigidity proofs), each linking to its full /why
  write-up. Commit `dbd06b9`.
Lab hub now has all three cards live (findings, inventions, basins) — no
"in preparation" left on the Lab door. Remaining: more invention pages with their
own viz (LADDER/REPLAY/PRIOR/STAGED/BANDSAW/CLOISTER/MIDDEN, #20), and the solver
catalogue with a live demo (#21). All commits LOCAL only (no push, per user).

### 2026-06-22 — Solver catalogue; research section fully populated

/research/build/solvers: backtracking → fill-order effect → Blackwood schedule +
break-index → McGavin throughput; links to live Watch/Paths to see a solver run.
Commit `3237a9c`. **The whole research section now has NO "in preparation" cards**
— every hub (Why 7 findings, Build 6 tools, Lab 3 areas + 3 inventions) and child
is live.
REMAINING (enhancements, not gaps): per-invention deep-dive pages
(LADDER/REPLAY/PRIOR/STAGED/BANDSAW/CLOISTER/MIDDEN) with their own animations
(#22); a bespoke embedded solver demo (currently links to playground).
All commits LOCAL only (8+ ahead of origin; user said don't push).

### 2026-06-22 — PRIOR invention (constructive from scratch, 460)

Bundled + verified the vol-155 from-scratch→ALNS 460 board into record-boards
(recompute 460 == claimed; 4 boards now). New /research/lab/inventions/prior via
InventionLayout. 4 inventions live (PALIMPSEST/KEYRING/GAUNTLET/PRIOR). Commit
`1828ec3`. Remaining invention pages (LADDER/REPLAY/STAGED/BANDSAW/CLOISTER/
MIDDEN) still queued (#22) — need their boards bundled+verified or written
board-less. Local commits only.

### 2026-06-23 — Invention catalogue complete (10 named algorithms)

Added STAGED(436), BANDSAW(437), LADDER(451), REPLAY(460), CLOISTER(453),
MIDDEN(452) — board-less InventionLayout pages, honest repro labels. Plus PRIOR
earlier. Inventions hub now lists all 10: PALIMPSEST, KEYRING, GAUNTLET, PRIOR,
STAGED, BANDSAW, LADDER, REPLAY, CLOISTER, MIDDEN. Commits `33c492d`, `1c2bfe4`,
`323a719`. Tasks #20, #22 done. The whole research section is now content-complete
(no placeholders). Next: animations for the board-less invention pages (STAGED
stage-by-stage, MIDDEN mask shapes, etc.) per the alive-KB directive. Local
commits only.

### 2026-06-23 — Invention animations (alive KB)

Added an optional `visual` slot to InventionLayout (renders after "How it works")
and built four bespoke animations: STAGED (stage-by-stage frame-free build),
MIDDEN (interactive mask shapes — rows/cols/dispersed lattice), BANDSAW
(meet-in-the-middle seam join), LADDER (successive-halving tournament bars).
Commits `b34b0f4`, `afbf562`, `48f5af0`, `765a261`. Now every invention page has
either a verified board preview or an animation. Lint reminder: no non-null
assertions — use a typed non-empty tuple + `?? fallback` instead of `arr[i]!`.
Local commits only.

### 2026-06-23 — CLOISTER + REPLAY animations; all inventions have a visual

CLOISTER (border-anchor fill diagram), REPLAY (double-break toggle diagram).
Commit `ff9b4d7`. Now all 10 invention pages have either a verified board preview
or a bespoke animation. Local commits only.

### 2026-06-23 — Piece-theft finding + Experiments log (publish ALL attempts)

- piece-theft (#why): exact (N,W)-demand server scarcity (269 demands, 47 with a
  single server, mean 2.9), animation of a cell dying from a stolen scarce piece,
  histogram. NOTE: my exact recompute (269/47) differs from the vault note
  (362/121) — different convention; published what's reproducible. Commit `f279cf9`.
- USER DIRECTIVE: publish ALL experiments tried + result + why
  ([[feedback_e2_site_publish_all_experiments]]). Built the **Experiments log**:
  web/src/data/experiments.json rolled from all 380 vault concept frontmatters
  (245 built / 52 refuted / 41 partial / 39 idea / 2 wont-do), regenerable via
  research/topics/experiments-log/extract.py. New /research/lab/experiments page:
  search + outcome filters. Commit `0de1d0a`. Lab door now has 4 cards.
  NEXT: enrich the "why" — esp. pull a why-it-failed line from each of the 52
  refuted concepts' bodies.
Why door now has 8 findings. Local commits only (no push, per user).

### 2026-06-23 — Session close

Added experiments-log/article.md so `build-index.mjs` validates (9 topics OK).
Session state at close:
- **Research section content-complete.** Why: 8 findings (phase-transition,
  rigidity, sigma-cycles, forbidden-patterns, no-forced-moves, piece-theft,
  rare-color, entropy/area-law). Build: reference, papers, records, dead-ends,
  run-it-yourself, solvers. Lab: findings, 10 inventions (each with a verified
  board preview or animation), experiments log (380 attempts, searchable), basins
  gallery (7 boards).
- 9 reproducible `research/topics/`; KaTeX math; 7 bespoke animations.
- Verified green at close: typecheck ✓ lint ✓ build ✓ index ✓.
- **28 commits LOCAL ONLY — NOT pushed** (user said stop pushing; resume only on
  explicit say-so). `git push origin main` when cleared.
- OPEN / next-session backlog: clean up terse internal-shorthand in the
  experiments log for outsiders; basin case-studies for all 17 (gallery has 7);
  full 219-vol record-climb timeline on Records; ★ findings depth-40 + NS-1
  still TODO; bespoke embedded solver demo (solvers page currently links to
  playground). Plan: research/SITE_PLAN.md.

### 2026-06-22 — Set up this ledger; refined the writing voice

Created `research/LEDGER.md` (this file) as the cross-session memory for the
multi-day build, and backfilled it oldest-first. Refined the writing voice: the
first-person researcher journal voice ("we tried this and it failed") is welcome
and professional; what we avoid is exposing the automated process. Warmed the
dead-ends copy accordingly.

- Commit `<this commit>`.
- Files: `research/LEDGER.md`, `AGENTS.md`, `web/src/pages/research/build/dead-ends.tsx`.

### 2026-07-01 — Research wiki rebuild: MDX pipeline + docs shell (Phase 1)

The research section starts its move from hand-written TSX pages to an
MDX-first wiki (plan + ADR: research/WIKI_REBUILD.md). Considered Fumadocs and
Mintlify; chose a headless custom pipeline (@mdx-js/rollup + remark/rehype +
a small Vite manifest plugin) because the site is RR7 ssr:false with custom
EN//fr i18n and its own design system. New: web/content/research/**.mdx with
zod-validated frontmatter (title/description/kind/tier/score/repro/sources/
topics/related), per-language virtual manifests, docs shell (sidebar tree +
scrollspy TOC + breadcrumbs + prev/next + sources + repro + related rail),
catch-all research/* routes so migration is page-by-page with URLs unchanged.
Pilot page migrated: why/border-balance (EN+FR), now with real KaTeX math.
Registration burden for a new research page: 1 MDX file (was 5 files).

- Verified: typecheck ✓ lint ✓ build ✓; prerendered HTML carries the full
  prose in both languages (lazy MDX resolves during prerender — the key
  architectural risk, confirmed safe); sitemap symmetric; live Ns1Lab demo
  works embedded in MDX.
- User directives recorded: cross-cutting topic categories + a top category
  bar (Phase 1.5); always link supporting sources (frontmatter sources[]);
  double-check numbers/facts during every migration.
- Reproducibility: tooling/prose.
- Files: web/content.config.ts, web/plugins/research-content.ts,
  web/content/research/why/border-balance{,.fr}.mdx, web/src/components/docs/*,
  web/src/lib/research/*, web/src/pages/research/doc.tsx, routes.ts,
  sitemap.config.ts, seo.ts, vite.config.ts, index.css, research/WIKI_REBUILD.md.

### 2026-07-01 — Topic categories + two-level research navigation (Phase 1.5)

User directives: big cross-cutting categories browsable from a research-level
navbar (not everything in one huge sidebar), natural non-literal French, and
every claim linked to its source. Built: a curated topic registry
(content/research/topics.json — structure, search-space, backtracking, speed,
construction, local-search, exact-methods, learning, records, hardware,
quantum), `topics:` frontmatter validated against it, auto-generated hub pages
/research/topics/<slug> + index (prerendered EN+FR, 24 new pages), a research
subnav (Overview / three doors / Topics — user cut the chip strip, tab is
enough), and the sidebar scoped to the ACTIVE section only. Legacy TSX pages
carry topic tags in nav.ts so hubs are populated pre-migration. Pilot FR page
rewritten in natural French. hardware/quantum hubs are honest placeholders
(community history to be written in Phase 4). Also recorded: Jef Bucas's
Blackwood notes + parameter study offered for the site with permission
(groups.io msg 11905) — queued for the solver catalogue.

- Verified: typecheck ✓ lint ✓ build ✓ (119 prerendered paths), visual pass
  EN+FR desktop.
- Reproducibility: tooling/prose.
- Files: web/content/research/topics.json, content.config.ts, plugins/,
  src/lib/research/*, src/components/docs/* (Subnav, TopicPages new),
  src/pages/research/doc.tsx, index.css, border-balance MDX,
  research/WIKI_REBUILD.md.

### 2026-07-01 — Research search (Phase 2)

Full-text search across the research section: the content pipeline emits a
per-language plain-text index as its own lazy chunk (nothing loads until the
dialog opens), minisearch runs it client-side (fuzzy + prefix, title/
description boosted), and a ⌘K dialog in the research subnav shows results
with kind dots and body snippets. MDX pages are indexed full-text; the
not-yet-migrated TSX pages join via title+description so nothing is
unfindable. Static-only as always — no server, works offline.

- Verified: typecheck ✓ lint ✓ build ✓ (index chunks code-split); manual:
  "forbidden squares" → Forbidden patterns; "multiset seam tallies" →
  border-balance ranked first via body text.
- Reproducibility: tooling.
- Files: web/content.config.ts (plainText/searchEntries), plugins/
  research-content.ts, src/components/docs/SearchDialog.tsx, ResearchSubnav,
  src/lib/research/{types,nav}.ts, seo.ts (pageDescription), minisearch dep.

### 2026-07-02 — Why door fully migrated to MDX; "experiments, not inventions" (Phase 3 batch 1)

All 13 "Why it's hard" pages now live in web/content/research/why/*.mdx
(EN + natural-French FR, 26 files). Every number re-verified against
research/topics computed results; two errors in the old TSX corrected in the
process: complex-theory's growth-regime endpoint is ~10^27 (not 10^28 —
verified by running the exact McGavin estimator port), and the four non-centre
clues sit at interior cells (34/45/210/221), not corners. Sources frontmatter
now carries the Mateu/Hamadi 2012 and Ansótegui et al. papers and McGavin's
groups.io posts (5209, 11197) with working URLs. All interactive labs
preserved; recharts/motif-swatch blocks became in-MDX components.
Deregistered the 12 pages from routes.ts/PAGE_PATHS/seo.ts/nav.ts and deleted
the TSX (net: prose now lives in markdown).

USER DIRECTIVE recorded and applied: the project's own algorithms are
"experiments", never "inventions", and must not read as the wiki's core —
the lab section is now "Raphaël's explorations" / "Les explorations de
Raphaël", one researcher's work structurally parallel to Bucas/Blackwood/
McGavin material. kind:"experiment" replaces "invention" in the schema.

- Verified: typecheck ✓ lint ✓ build ✓ (119 paths), EN+FR titles/prose in
  prerendered HTML, visual pass on walls-and-methods.
- Reproducibility: prose/content (numbers backed by the cited topics).
- Files: web/content/research/why/** (26 MDX), routes.ts, sitemap.config.ts,
  seo.ts, src/lib/research/{nav,types}.ts, content.config.ts,
  src/components/research/RecordBoard.tsx (new, for the experiment batch),
  DocsShell HubCards, 12 TSX pages deleted.

### 2026-07-02 — Build pages + all 12 experiments migrated; InventionLayout retired (Phase 3 batch 2)

run-it-yourself, dead-ends, the solver catalogue (now build/solvers/index.mdx,
first hub-as-MDX) and all 12 experiment pages live in web/content/research
(30 MDX files, EN + natural FR). The experiment template decomposes the old
InventionLayout: score/repro in frontmatter (shell renders the badge + honesty
box), sections as h2, board previews via the new <RecordBoard> component,
all live labs preserved (GauntletLiveRace, CloisterLiveLab, MosaicBlockLab,
LadderLiveLab…). Copy de-invention-ified per user directive (experiment
framing; MIDDEN's novelty claims softened). Fact-checks: every board id +
score verified against record-boards data (463/460/460/458 recomputed =
claimed); run-it-yourself commands verified against the real justfile;
MOSAIC 448 / MIDDEN 452 / LADDER 451 / LODESTONE 451 confirmed from ledger +
experiments.json. InventionLayout.tsx deleted (unused).

- Verified: typecheck ✓ lint ✓ build ✓ (119 paths), EN/FR titles on all six
  spot-checked pages, visual pass on PALIMPSEST (shell, badges, board, TOC).
- Files: web/content/research/{build,lab}/** (30 MDX), routes.ts,
  sitemap.config.ts, seo.ts, nav.ts, 15 TSX + InventionLayout deleted,
  RecordBoard.tsx new.

### 2026-07-02 — Migration COMPLETE: the research section is a pure MDX wiki (Phase 3 done)

The last legacy layer is gone. Hubs (overview, why, build, lab, experiments,
findings) are index.mdx pages with auto-rendered child cards; papers/records/
reference/experiments-log are MDX wrappers around view components (data-heavy
TSX stays TSX); research-links.ts, RelatedRail, STATIC_LEAVES and all research
entries in routes.ts/seo.ts are deleted. web/content/research is now the
single source of truth: ~80 MDX files drive routes, prerender, sitemap, SEO,
sidebar, topic hubs, search and the related rail. URLs unchanged throughout
(119 prerendered paths before and after).

Also fixed on user report: "Reproduce this result" was near-empty on many
pages. Added 8 missing `just research-<topic>` recipes (phase-transition rerun
verified byte-identical), wired repro.cmd+topic into every compute-backed page,
pointed board-backed experiments at the record-boards verification, and the
shell now renders a compact note instead of a box when there is nothing to
run. complex-theory finally got its research/topics article.md — build-index
had been failing since the reference port landed (11 topics OK now).

- Verified: typecheck ✓ lint ✓ build ✓ (119 paths) ✓ research index ✓; visual
  pass on overview + experiments hub.
- Files: web/content/research/** (hubs + 8 wrapper MDX), views/ (4),
  ExperimentScoreChart (renamed from InventionScoreChart), justfile,
  research/topics/complex-theory/article.md, AGENTS.md, deletions listed above.

### 2026-07-02 — Blackwood decoded (credit: Jef Bucas), Known facts page, experiments log retired (Phase 4 begins)

- /research/build/solvers/blackwood: Joshua Blackwood's record backtracker
  explained through Jef Bucas's notes and wrapper_blackwood parameter study,
  published with his explicit permission (groups.io msg 11905) and full
  credit. Carries his verbatim schedule/break-index code, his near-optimal
  conclusion, his own low-sample caveat, and an open replication invitation.
  Numbers cross-verified against wrapper_blackwood's jobs.py defaults.
- /research/build/known-facts: the reference numbers researchers keep
  re-deriving, every row with provenance. Serious verification pass:
  1.115×10^557 reproduced exactly (4!·56!·195!·4^195), Verhaard 467 corrected
  to 2008, twin-pair piece ids re-based to the site's 1–256 numbering, the
  vault's internal-only material (basins, bounds, engine perf) excluded.
- USER DIRECTIVE: the experiments log page (auto-extracted one-liners) is
  removed — not a proper research artifact. Data + topic stay in the repo for
  future curation into real write-ups.
- Fixed MDX nested-<p> hydration warnings (phase-transition swatch cards, the
  overview Door component): text inside a JSX <p> is re-parsed as a markdown
  paragraph — use <div> wrappers in JSX islands.
- Verified: typecheck ✓ lint ✓ build ✓ (121 paths).

### 2026-07-02 — Blackwood article illustrated with Jef Bucas's figures

Pulled the four figures from Jef's wrapper_blackwood notes into the Blackwood
page (user directive: illustrate articles with Bucas images, with mention):
the two 470-class pattern-set boards, the quota-schedule traces, the fill
order, and the study's headline image — hundreds of random schedule
variations coloured by depth with Blackwood's hand-tuned blue line in the
middle of the green band. Every caption credits "Jef Bucas
(wrapper_blackwood), reproduced with permission", and the credit callout now
says the figures are his. Images live in web/src/assets/research/blackwood
(downscaled/JPEG-compressed, ~790 KB total, lazy-loaded, hashed by Vite).

- Verified: typecheck ✓ build ✓ (assets emitted, captions in prerendered
  HTML EN+FR, all four figures load in browser).

### 2026-07-02 — Raw-markdown siblings for every research page (Phase 5 begins)

Every research page now ships a .md sibling at build time (same URL + ".md",
FR under /fr — 78 files): a provenance header (title, description, canonical
URL, updated, topics, reproduce command, sources) plus the MDX body with the
ESM plumbing stripped. "View as Markdown" link in the page footer; llms.txt
rewritten to describe the wiki and point agents at the .md URLs (also fixed
its stale repo link). This deliberately revisits the old "llms.txt is
index-only" trade-off from AGENTS.md: content is markdown now, so the old
objection (TSX→thin stubs) is gone.

- Verified: typecheck ✓ lint ✓ build ✓; emitted .md spot-checked (header +
  clean body, export blocks stripped).

### 2026-07-02 — Records timeline fully sourced from the community archive

Every record entry on /research/records now carries a verifiable source
(new localized Source column): Wikipedia for the launch/prize/close,
shortestpath.se for Verhaard's 467, and real groups.io message links for the
modern era. The archive export exposed real errors: the four existing deep
links used groups.io internal global IDs (dead ends) — remapped to true
msg_nums (10033 Bucas/Blackwood 468, 10045 McGavin 469 announcement, 10117
Blackwood 470, 11074 Gauthier strict-460); and the timeline's "2021 470,
different heuristic colors" row had no archive support — corrected to Jef
Bucas's 2024-12-02 470 (msg 11401). The 2025 Discord-only 470 is honestly
marked unsourced. groups.io deep links are login-walled; a note says so.

### 2026-07-02 — Concepts encyclopedia v1: the technique shelf (8 concepts + hub)

New section /research/build/concepts — the techniques every solver author
meets, each attributed to its inventors with resolving primary sources, the
honest measured verdict from this project's engine (always hedged "not
independently replicated"), and cross-links into the wiki:
arc-consistency (Mackworth 77, AC-2001), alldiff-regin (Régin 94; the
colour-filter-vs-break-allowance soundness caveat stated plainly),
exact-cover-dlx (Knuth), meet-in-the-middle (Horowitz-Sahni; BANDSAW),
beam-search (Zhou-Hansen; GAUNTLET/PRIOR/LODESTONE), local-search-alns
(Shaw 98, Ropke-Pisinger 06, Schaus-Deville E2 precedent),
parallel-tempering (Swendsen-Wang; Verhaard's 467 method per public
sources), sat-csp-encodings (Heule 08, Ansótegui et al.). Internal
vol/basin/codename material excluded; one vault figure flagged fabricated
was replaced with "never profiled". 18 MDX files, EN + native FR.

- Verified: typecheck ✓ lint ✓ build ✓ (139 paths, EN/FR titles).

### 2026-07-02 — Community archive digests 0001-0003 (2000 → 2008-02)

The full local groups.io export (11,511 messages) is being swept
chronologically per user directive (pipeline: research/community/README.md).
First three windows digested (~4,600 messages, 86 sourced events): the
group's 2000 pre-history, launch-week analysis, Monckton's disqualification
drama, the 17-colour design derivation (msg 1947), the parity proof that 479
is impossible (msg 1640), 2007 border-balance precursors (msgs 414/2073),
eternity2.net's rise and shutdown, the benchmark culture, early SAT/DLX/ILP
attempts, and the score climb to 463 by late 2007. Every event carries its
public msg_num link. Leads for existing wiki pages recorded in the README
(earlier sources for border-balance and phase-transition, a known-facts
parity fact).

### 2026-07-02 — Digests 0004-0006; the wiki corrects itself on 479

Windows 0004-0006 digested (2008-03 → 2009-02, ~2,000 messages, 90+ events):
Owen's complex-theory completion posts (5197/5209), Schaus's Hungarian-refill
VLNS arriving, the 467 run-up (Verhaard past the "don't-talk limit", solver
released at fingerboys.se), and the first scrutiny date (TOMY silence, the
$10,000 to "Anna Karlsson" = Verhaard's household entry, 467 hit 40+ times by
his tool's users). Two synthesis traps recorded (no public 467 announcement;
the Oct-2008 "468" is a test puzzle).

LIVING-WIKI CORRECTION: digest 0006 flagged that Verhaard (msg 6317, 2009)
refuted the msg-1640 parity proof I had just published in known-facts.
Adjudicated from the archive bodies + verified against the official piece
set (14 border pieces have equal ring-facing sides): 479 IS reachable from
any full solution via a border-piece flip — the parity leaks through the 60
unscored outward edges. known-facts rewritten to tell the full story, both
messages cited. Lesson: categorical impossibility claims get the same
adversarial treatment as everything else.

- Verified: build ✓ (139 paths).

### 2026-07-02 — The hunt, a history — part I (2000–2009)

First synthesis from the archive digests: /research/build/history, a
narrative of the community's first decade — the group that predates its
puzzle, launch summer, the Monckton disqualification chill, the theory race
(17+5, parity, complex theory), eternity2.net and the Syndicate, the road to
467 and the scrutiny-date farce ("Anna Karlsson" decoded). 85 message-level
citations in the EN page; French written as its own prose ("La farce du
scrutin"). Ends with a To-be-continued: parts extend as the digest sweep
advances (0010-0012 mining now; 0013-0015 queued to reach 2026).

- Verified: typecheck ✓ build ✓ (141 paths, EN+FR titles).

### 2026-07-02 — MAJOR CORRECTION from the completed archive sweep: the record is 470

All 15 archive windows are digested (2000 → 2026-03, 11,511 messages, ~400
sourced events). The sweep's biggest yield falsifies the site's own headline
framing, inherited from community habit: "469 is the canonical ceiling; the
470s are an easier one-clue variant." Board-level verification (digest 0014)
shows the 468/469/470 record boards are all the same starter-only regime
(starter at I8, none of the optional clues), and per the contest's own entry
rules only the starter was mandatory (msg 11046; Bucas msg 10082; Blackwood
msg 10185). RecordsView reframed: official-pieces ceiling = 470 (Blackwood
2021, tied 2024/2025); strict all-5-clue best = 460 (Gauthier 2023); flag
semantics now "official pieces" vs "other set". Row nuances fixed (468
relayed from Reddit #10032; the November-wave boards are independent finds).
Also applied: Hopfer's NS-1 condition pinned to its exact messages
(10754/10757) on border-balance, and complex-theory gained a fully sourced
"Provenance and validation" section (Owen 5197/5209 → peak-depth closed form
8125 → McGavin's pdf 9188 → the 2017 10×10 validation 9686/9688 → the 2024
C reference 11197 → this site's port). A consistency agent is sweeping the
remaining "469 ceiling" mentions; history part II is being written.

### 2026-07-02 — History part II (2009-2026) + the 470 correction applied site-wide

/research/build/history-2: the story's second half from digests 0007-0015 —
the diehard years, the contest's death, the quiet era and the 10x10 triumph,
the Yahoo→groups.io migration, the record wave, the modern era through the
archive's edge (2026-03). Part I links forward. ~2,300 words EN, native FR,
six anchor sources, dense msg citations.

The 469→470 correction swept the whole site (not just research): records/
known-facts tables rewritten (470 the ceiling since 2021, tied twice; 469
demoted to "previous record, long quoted as the ceiling"; strict-460
reframed), experiments hub + PALIMPSEST comparison lines, walls-and-methods
and sigma-cycles gap phrasing (ten edges, since 2021), solver catalogue,
ScoringPrimer, ExperimentScoreChart ceiling line, PapersView, Puzzle page
timeline (the "easier one-clue variant" parenthetical deleted). Board-family
references ("469-class", McGavin's 469 analyses) deliberately preserved.

- Verified: typecheck ✓ lint ✓ build ✓ (143 paths), EN/FR titles, corrected
  records page in prerendered HTML.

### 2026-07-02 — Overnight wave: benchmarks, parity, Blackwood enrichment, contribute guide

Four pages from the digest goldmine, all EN + native FR:
- /research/build/benchmarks — the community's benchmark culture: the census
  verification protocol (3x3 corner 2,633,221; 2x2 counts), the named suites
  and their duels (hints15_2 down to 85,729 nodes), hints.20.3 (2.9e13 nodes,
  one solution), the 9x9 pair, and Brendan's 10x10 — set_1 solved 2017
  (~180 core-years), set_2 framed as the standing open challenge.
- /research/build/concepts/parity-arguments — the 479 story in three acts,
  colour sums → NS-1 lineage, the 2011 balanced-sets negatives, and the
  certificate-asymmetry lesson.
- Blackwood page enriched with primary levers (no-adjacent-breaks 10051, the
  11→10 retune behind the 470 10076, his own negative results 10056, the
  open-source spread arc). Agent corrected my brief from the digests
  (10161 = his repo re-publication, libblackwood = 10078).
- /research/contribute — the warm invitation: three ways in, house rules,
  the Jef Bucas precedent; lands in the Build sidebar via SECTION_OF.
Also: Schaus msg 5589 wired into the ALNS concept sources. Phase 5 complete.

- Verified: typecheck ✓ lint ✓ build ✓ (149 paths), contribute in Build
  sidebar confirmed in browser.

### 2026-07-02 — The notable boards gallery (lab/boards)

/research/lab/boards: every board that matters, whoever found it — the
record line (Verhaard's 467 story, Blackwood's 468 from Reddit, McGavin's
469 with its top-rows mismatch banding, the November wave with Fernandez's
single-swap, both 470s), the strict five-clue line (Gauthier 460 ladder),
and this project's four bundled boards — all previews embedded (RecordBoard
for bundled boards; a new in-MDX CommunityBoard pattern renders community
boards from KNOWN_BOARDS params with viewer deep-links). Every claim carries
its announcement message; no internal vault basin analyses used. This closes
the SITE_PLAN's "basins gallery" item with a fully public-sourced version.

- Verified: typecheck ✓ lint ✓ build ✓ (151 paths) ✓ link check (108 files).

### 2026-07-02 — McGavin's engine deep dive + who's who; the wiki corrects itself again

/research/build/solvers/mcgavin: the throughput story mined from his own
posts — a 2007→2026 timeline of sourced figures (Field's 2007 recipe msg
3098 as the acknowledged base, 44-105M nps single-core across CPU eras, the
10,160M/s dual-Xeon table msg 11369), the three-machines lineage (his
auto-generated C engines; Blackwood's solver run AS-IS for the 469; his
complex-theory implementations incl. the 25x lookahead msg 9751), closing
on the prune-vs-speed lesson. TWO of our claims fell to the sourcing pass:
"~295M nps" is Joe's measurement of McGavin's code (msg 11750), not
McGavin's own figure, and the solvers hub's "heavily optimized version" for
the 469 was unsupported (msg 10045: ran as released) — both corrected.

/research/people: who's who of E2 research — 25 full entries + 9 roll-call,
era-grouped, every contribution message-linked, identities only as stated
on-list (Wolz via msg 2972; Verhaard/Karlsson via his own msgs), Raphaël one
line among the others. Closes inviting corrections via the list.
