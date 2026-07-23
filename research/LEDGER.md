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

Bundled + verified the from-scratch→ALNS 460 board into record-boards
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

New section /research/build/techniques — the techniques every solver author
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

First synthesis from the archive digests: /research/community/hunt, a
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

/research/community/hunt-part-2: the story's second half from digests 0007-0015 —
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
- /research/build/analysis/parity-arguments — the 479 story in three acts,
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

/research/community/boards: every board that matters, whoever found it — the
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

### 2026-07-02 — Phase 6 complete: all 9 concepts are now educational

Every concept page carries a purpose-built interactive lab, a step-by-step
walkthrough on the lab's exact instance, and a verified complexity section:
AC-3 worklist (with a real AC-1 baseline), Régin's filter (augmenting paths
+ SCCs on a Hall trap), dancing links (Knuth's own instance, trace verified
move-by-move), the MITM square-root trade + hash join, beam diversity
collapse, parity flips on a real solved board (measured deltas), an ALNS
destroy-repair loop with adaptive weights on an engine board, a tempering
ladder with a ghost single-chain control (the cold replica demonstrably
escapes via swaps), and a SAT encoding-size explorer with a unit-propagation
cascade. All labs obey useRunWhileVisible + frame budgets. Two errors in
published prose fell to the work: a sign error in the tempering swap
formula, and MDX brace-escaping crashes caught in preview. All browser-
verified EN+FR; typecheck/lint/link-check green.

### 2026-07-02 — Verhaard's eii + edge slipping: the prize-winning technique, decoded

Two linked P1 pages close the catalogue's biggest gap. /research/build/
solvers/verhaard-eii: the solver behind the only prize ever paid — the
misread-rules origin, the "I am stuck" release, forward pruning and comb
orders from his own messages, the Markov-chain slip array, JSA's 82-day
public reproduction (and the 82h typo in msg 6687 diagnosed from the log
series). /research/build/reduce/edge-slipping: the technique itself, with
Max's target-multiplication formula RECOMPUTED and verified digit-by-digit
(S(478)=4.699e9 ✓, the 467→468 ratio telescoping to ~63 ✓), Brendan's
complex-theory extension and the honest 7-orders gap explained, and the
EdgeSlipLab whose slider notches land on real history (N=13 = Verhaard's
467; N=10 = Blackwood's break budget; N=1 = the exact parity zero, linked
to parity-arguments). The lineage slipping→breaks is now told end to end.

- Verified: typecheck ✓ lint ✓ build ✓ (163 paths) ✓ links (120 files).

### 2026-07-02 — Phase 7 wave 2: fill orders, solver engineering, restarts, tooling

Four more archive-sourced pages at the educational bar:
- concepts/fill-order — Owen's 121-185 decisive band, the magic 10x16
 square, the 8-orientations race, comb search folded in; FillOrderLab
 paints visit sequences against a live constraint-count curve (join counts
 hand-verified); /playground/paths framed as the hands-on companion.
- concepts/solver-engineering — the craft census (perfect hashing, cache
 structs, bipieces incl. the honest 2x2 slowdown, codegen, BMI2,
 node-counting discipline), closing on 4x-per-core vs 3 edges in 20 years.
- concepts/restarts — Txibilis's 2007 heavy-tail measurement, Gomes/Luby
 literature, Blackwood's "arbitrary" 50e9 cap, McGavin's row-farming as
 restart practice; RestartTailLab shows the cutoff U-curve (~217x at the
 historical cutoff).
- build/tooling — the 28-tool census 2007-2026 with the conventions layer;
 two brief-corrections from primary sources (group Files survived the
 migration; eii-pgen = msg 11752).

### 2026-07-02 — P2 wave: GPU solving, distributed solving, clue puzzles (+ another self-correction)

Three more archive pages: gpu-solving (eleven attempts 2007-2025, Field's
bandwidth anchor, Miles's enumeration win — first page of the hardware
topic hub), distributed-solving (swarms vs fleets, the DFS-partitioning
problem, what 10^19 ops bought), clue-puzzles (the four products per-puzzle
with the checker fiasco and the not-publicly-documented gaps stated). The
clue-puzzles sourcing exposed a site-wide error: known-facts and the Puzzle
page said "two clue puzzles + two later placements" — the archive documents
FOUR puzzles, each revealing one placement (rules msg 182, mechanism 4187).
Corrected in both languages on both pages.

### 2026-07-02 — P2 wave: LP relaxations, iterated maps, solution counting

- concepts/lp-relaxations — the optimizer's road: the compact ILP
 formulation, why the LP is happy while the puzzle isn't (Birkhoff broken
 by seam constraints, the verbatim fractional corner), the measured plateau
 table (12 campaigns 2007-2025), what LP/MIP is still for.
- concepts/iterated-maps — the Elser lineage with scope honesty: a road
 lightly traveled (one empirical test, a promised write-up that never
 appeared), the exact PNAS difference map, Elser's own 5x5 code in the
 group files, and a concrete attempt-today protocol.
- concepts/solution-counting — the three regimes (exact, expectation,
 culled) with the 9x9 replication lesson and Owen's 4.05e37 border count
 as the methodology gem; the naive toy expectation shown honestly failing
 by six orders.
The concepts shelf now covers 17 techniques.

### 2026-07-02 — Day recap: Phases 6+7 — the wiki went deep

The day's sweep (user away, autonomous): Phase 6 completed all 9 concept
educational upgrades (9 new interactive labs, walkthroughs, verified
complexity sections — plus a sign-error fix in published prose). Phase 7
then worked through the PAGE_CANDIDATES roadmap: all 9 P1s and 10 P2s are
live — the solver catalogue grew to four entries (Blackwood, Verhaard's eii,
McGavin's engine deep-dive, this site's engine as a modest peer), the
concepts shelf to 19 techniques (edge slipping, fill orders, solver
engineering, restarts, GPU, FPGA, distributed, LP relaxations, iterated
maps, solution counting, no-good learning...), and the reference row gained
who's-who, tooling, clue-puzzles, variants, and design-recipe pages. The
sourcing discipline kept correcting the site itself: the four-not-two clue
puzzles error (site-wide fix), the 295M-nps attribution (Joe's measurement,
not McGavin's claim), the "heavily optimized" 469 myth (ran as released),
and the FPGA record's did-reach-silicon nuance. Verified at close:
just check ✓, link check ✓ (146 content files, 0 dead links), 183+
prerendered pages, every commit local. The wiki now tells the community's
whole technical story with primary sources end to end.

### 2026-07-02 — Discord archive mined (digest 0016); a records row falls

The 1,198-message Discord #general export is digested. Headline: the
"2025-07 · 470 · onesmallstep" records row is REFUTED — the CSV shows a
July-2025 conversation where onesmallstep RELAYED the two existing 470
boards; he never claimed one (his self-reported best: ~466-467). Row
deleted; "tied twice" phrasing corrected to "tied once (Bucas, Dec 2024)"
across records and known-facts. Gained in exchange: Blackwood's first-person
470 account (a month on a home Threadripper 3970X, ~150 total hours, no
hints — Discord 2024-11), now enriching the 2021 row; people-page entries
for onesmallstep (server founder — with the honest story of the mis-carried
row) and Reinout Annaert (469, the linear-run subculture, hint-authenticity
settled). The lesson the wiki keeps teaching itself: unsourced rows die.

### 2026-07-02 — Pass 2 (data quality): every citation on the wiki verified

The full-sweep citation audit is done. Section verifiers (why/, concepts
A-L, concepts M-Z + solvers, flat + lab + history + people, with per-page
children on the dense files) checked every one of the wiki's 700 distinct
groups.io citations against the local archive — author, date, and verbatim
support for every quoted number and phrase — plus all 57 external URLs.
Roughly 140 fixes landed, all mirrored EN + FR: repointed message numbers
(quote lived in a sibling message), restored-verbatim quotes (including a
"[sic]" for an original's typo), inverted chronologies straightened
(parity-argument estimate/retraction order, solution-counting peer-review
sequence), attribution corrections (e2walker is Philippe Coustaux's, revised
by Ole Knudsen; the depth-161 baseline is Brendan Owen's; Bob Jenkins's
pointer was Nathan's own find), a renamed tool (eii-puzzles, not "eii-pgen"),
and scope-narrowing on overclaims ("verified for the 470", not both;
"~32,000 pairs must survive invalidation", not pin). Highlights by report:
history-2 21 fixes / 101 citations; tooling 13 / 82; concepts M-Z + solvers
~25 (Blackwood code blocks byte-identical to Jef's Notes.md and the 470
repo); why-section 12; history 12; known-facts + benchmarks 11; people 9;
clue-puzzles 7; distributed 6; boards + contribute 4; lp-relaxations 0.
One standing flag: msg 11905 (wrapper_blackwood permission) post-dates the
archive export (ends #11823) — documented as provided firsthand in
research/community/README.md, whitelisted in the checker. Verified at
close: citations check ✓ (700 msgs, 0 missing; 0 dead URLs; 0 no-source
pages), link check ✓ (150 files), typecheck ✓, lint ✓, build ✓ (193
prerendered pages). The audit's lesson: quotes drift one message inside
their thread; always cite the message that *contains the words*.

### 2026-07-02 — Pass 3 (article quality + style): the wiki loses its em dashes

The whole research section has been rewritten to drop the LLM-writing
artifacts the user flagged. Every em dash outside verbatim quoted material
is gone, roughly 3,190 of them across ~103 MDX files in both languages,
replaced not by a mechanical comma swap but by varied punctuation chosen per
sentence (commas, colons, periods, parentheses, sentence splits); the French
copy was reworked to idiomatic rhythm rather than a literal mirror of the
English fix. Every banned tic went too: the "honest/honestly" crutch (72
hits) rewritten to say what it actually meant (headings like "The honest
record" became "What was measured"/"Why nothing shipped"; "honest verdict"
became the verdict itself), plus a stray "crucially" and one "Spoiler:". The
French "honnête" mirrors were softened for EN/FR parity even though the
English-only checker never flagged them. A second sweep cleared the em
dashes hiding in frontmatter source labels, figure captions, and the
topics.json registry (the checker masks quoted spans, so ~130 label/caption
dashes were invisible to it and had to be hunted by grep), always preserving
the verbatim community quotes those labels wrap. The only two em dashes left
on the site are inside quoted source material ("didn't pan out — I had
memory bandwidth issues…") and are correct to keep.

Along the way the style checker gained a blockquote exemption (`> …` lines
hold verbatim quotes) and the pipeline caught one real bug: a rewriter had
put an unquoted colon into a restarts.mdx source label, breaking the YAML
frontmatter and the whole config load; fixed by double-quoting the two
labels. Commits: e1cd9be (checkpoint, 3190→1723), d65e313 (body prose to
0/0), plus this frontmatter+registry sweep. Verified at close: style 0
dashes / 0 tics, links 150 files, citations 700/0 missing + 0 dead URLs + 0
no-source pages, typecheck, lint, build (193 prerendered pages) all green.
The three-pass arc (Discord mining, citation audit, style) is complete: the
wiki now reads like one researcher wrote it carefully, with every claim
sourced and no machine tells.

---

2026-07-16 — The DFS study. A new, dedicated experiment
(`research/experiments/dfs-study/`, peer of single-core-benchmark) that takes
the depth-first backtracker apart: what does each fill order, each heuristic,
and the break mechanism actually buy? Reason for a fresh build rather than
reusing bench-grid: the goal was clean software engineering and an explicit
"what stacks on what" story, where every variant is one declared change over a
parent. A self-contained five-crate Rust workspace (dfs-core: board + the one
canonical scorer, parity-tested against a known 469 board; dfs-io: a shared IO
contract with lossless converters to site-JSON/CSV/bucas/hints, exposed via a
dfs-convert binary; dfs-engine: the composable DFS + a registry where the site
matrix is generated from the code, not hand-kept; dfs-codegen: NAIVE-CODEGEN, a
16x16-specialised row-major hot loop; dfs-run). 17 runnable variants across four
families plus two cited community engines, run on the same ten corner-pinned
variants the benchmark uses, single core, 60 s, seed 1. Every stat the study
raises is committed: score (canonical re-score), node/sec (labelled, never
cross-family), max depth, depth-at-timeout, true break count, backtracks.

Findings (committed grid, results.jsonl + report.md): path order is the largest
free lever (row-major mean 377 vs strict border-first/spiral ~67, a 300-point
swing); MRV rescues border-first to mean 324 at a 1000x throughput cost; more
propagation did not help at 60 s (fc/ac3/gacolor all within one point, inside an
11-point spread, so statistically indistinguishable); breaks clear the strict
~208 depth wall to mean 431 / depth 245, with the schedule the decisive lever
(Verhaard's early ladder 399 vs Blackwood's 431) and a second break per cell a
measured null.

Two adversarial audit passes before publishing caught real issues and fixed
them: depth_at_timeout meant max-vs-frontier across the two engines (now both
frontier); the published "break count" on a timed-out partial was actually the
whole unmatched-edge deficit (now the true committed-break count, strict = 0);
the fairness timer checked every 4096 nodes, giving the slow MRV family ~0.5 %
extra budget (now every 256); "soundness verified by tests" was unsupported
(downgraded to sound-by-construction, which a reviewer confirmed holds); and the
"double-breaks beat single" claim was contradicted by the corrected numbers
(rewritten as the null it is). A performance audit found the numbers fair; the
one research-relevant note (MRV recomputes domains from scratch, not
incrementally) is footnoted so the throughput is not misread as MRV's best
possible.

Both community record engines were built and run on the M1, and the result is a
finding: neither can take the corner-pinned grid. McGavin's C reaches depth 205
at ~109 M tiles/s on its native 5-clue puzzle but collapses to depth 21 with
three corner pins (its fixed scan path cannot absorb a pin); Blackwood's C#
(.NET 8, one thread) reaches only ~34 of 256 cells on the pinned 5-clue, since
its heuristic is tuned for the 1-clue instance. They are reported in a separate
community panel on their native instances, with the corner-pin collapse as the
finding that motivates the whole from-scratch study. Illustrated with a
PathOrderDiagram (the six fill orders shaded by fill sequence) and the
DfsStudyLeaderboard (score bars by family + the generated matrix + community
panel). Reproducibility: `just experiments dfs-study`. Verified at close: cargo
test 21/21, research style 0/0, links + citations green, typecheck, lint, build
all green. Working-state (NOTES.md) kept out of the committed record.

### 2026-07-16 — The repair study, on a shared central lib, and the audit that changed its findings

Built the **repair study**, the sibling of the DFS study, for the other way E2 is
attacked: destroy-and-repair local search taken apart one decision at a time, on
the same ten corner-pinned variants, single core, 60 s, seed 1. First, a
central-lib refactor: `dfs-core` and `dfs-io` were extracted out of the DFS study
into `research/experiments/common/` (crates renamed `e2-core`, `e2-io`), so both
studies now sit on the *same* board, canonical scorer and IO contract. A repaired
board and a backtracked board are scored by identical code. The DFS study kept only
its own crates and still builds + passes 15/15; the shared lib carries its own
`fixtures/variant_00.*` and passes 6/6.

The repair engine (`repair-study/engine/crates/repair-engine`) is one composable
destroy-repair loop with the same variant-as-delta registry idiom as dfs-engine.
Five axes: starting board (random / greedy / greedy-rare), destroy (random /
mismatched / worst-band / component-halo), repair (greedy / jitter / exact-small),
acceptance (greedy-equal / strict / annealing / late-acceptance), restart (none /
kick / revert-best). An incrementally-maintained mismatch map makes conflict-driven
destroy O(k). `run_repair` emits a RESULT line with the raised stats; the site JSON
carries a 60-point convergence curve per variant. Site: three MDX pages
(index/method/findings), a `RepairStudyLeaderboard` (leaderboard + lift + a
convergence-curve stall figure + the generated matrix), a four-phase animated
`RepairLoopDiagram` stepper, and a `DestroyOperatorDiagram`. Palette = the DFS
study's validated five-hue set. Cross-linked from the ALNS theory page, the
raphael-anjou hub, and `/algorithms`.

An **adversarial audit** (a subagent tasked to find research-poisoning) earned its
keep. It found two real defects the first "green" run had hidden by testing only in
`--release`: (1) the incremental conflict-map counter was not seam-symmetric (it
gated on one cell's own edge, not the seam), so `place` and `clear` disagreed and
the u8 counter leaked; the study's own tests in fact *panicked on overflow in dev
profile*, and in release the counter silently wrapped, leaving the map wrong on up
to 248/256 cells. The published scores were untouched (the score gate is symmetric
and every board is canonically re-scored), but the conflict-driven variants were
picking cells from a corrupt map. (2) REPAIR-EXACT was a confounded two-axis change
whose "exact" refill rarely even ran (12-cell holes, 8-cell cap). Fixed both: a
single symmetric `seam_kind` classifier shared by place / clear / recompute; release
profile now keeps `debug-assertions` + `overflow-checks` on (these are 60 s research
runs, not throughput-critical); a new test churns multi-cell destroy/refill and
asserts the whole incremental map equals a from-scratch recompute; and REPAIR-EXACT
was rebuilt as a true one-axis change over a new REPAIR-SMALL baseline (same 6-cell
destroy, only greedy→exact, so exact fires every iteration). Also verified, not
assumed: every output board is a valid 256-piece permutation (dedicated test, run in
both profiles).

The fix **changed real conclusions**, which is the whole point of running the audit
before publishing. On the corrected 60 s grid (14 variants, 140 runs, 0 failures):
RANDOM-DESTROY still wins outright at 400.5 (best 409) and is untouched by the fix
(it never reads the map). But every conflict-targeting operator now finishes clearly
below it and the more it fixates the worse it does (GREEDY-MISMATCH 366,
COMPONENT-DESTROY 350 stalling at iteration 76, BAND-DESTROY inert). ACCEPT-ANNEAL is
now the strong second at 374.9 (best 388), a real 14-point acceptance lever, not the
near-null the buggy run implied. And REPAIR-EXACT (360) does *not* beat REPAIR-SMALL's
greedy refill (363): a clean negative result, the reverse of the confounded first
run. The through-line: on a mediocre board, the moves that keep the loop *exploring*
(random destroy, annealing) beat the ones that *exploit* a stuck region (fixated
destroy, exact refill, restarts), which inverts on a near-record board and is exactly
why the records divide labour into a strong producer plus repair-as-last-mile.

Scout-mindset side-fix (user asked to solve problems even if not ours): cleared all
13 pre-existing hardware-disclosure failures honestly. Added `measured: false,
cores: 8` M1 blocks to the raphael-anjou runs/analyses (exact wall-clock was never
logged, so not fabricated), and relabelled Verhaard's `eii.mdx` from
`kind: experiment` to `kind: page` (it is an exposition on why his binary will not
run here, nothing was measured). Hardware check went 13 problems → 0.

Reproducibility: `just experiments repair-study`. Verified at close: cargo test
6 (common) + 15 (dfs) + 7 (repair, both dev and release) all green; research style
0/0, links + citations + hardware green; typecheck, lint, build green. Committed
results (results.jsonl, report.md, matrix.json, run_meta.json, 140 urls + 140
curves) promoted to `results/`; working-state (rerun/, *_grid.log) kept out of the
record by .gitignore. Deferred to a later deliberate pass, per the user: tag the
named runs with their pipeline stages, and an editorial audit of the published
engines/runs for what each brings (retiring dead-ends honestly).

### 2026-07-16 — Repair study: DFS-seeded pipeline, engine 2.5-4x faster, final 15-variant grid

Added a START-DFS starting board to the repair study: the loop spends its first
20 s running the DFS study's break-DFS, then repairs that board for the rest.
repair-engine now depends on dfs-engine by path (both share e2-core/e2-io, so the
board hands over with no conversion). On the final grid it is the study's top
score by a wide margin — mean 446 / best 449 — settling the construct-then-refine
question: repair adds only a handful of edges on top of a strong backtracked
board (+6.6), and the board it starts from decides almost everything. This
reproduces the records' division of labour end to end on one core in one minute.

Optimised both engines (behaviour-preserving, verified by tests + unchanged
scores). Repair loop 2.5-4x faster: hoist the target cell's neighbour context out
of the greedy/exact refill inner loop (the big win), reusable Scratch buffers
(zero per-iteration heap allocation), allocation-free destroy internals, and a
tuned profile (overflow-checks on, debug-assertions off). DFS MRV family 6-7x
faster: early-out on the candidate count (a minimum search needs no exact count
above the best) and skipping the edge checks the candidate list is already indexed
on; the strict engines (44M nodes/s) were already optimal and left untouched. A
profiler subagent + fixed-iteration/node A/B gates confirmed the wins. The faster
engine buys more compute in the same 60 s wall-clock, not a shorter grid.

Final 15-variant grid (150 runs, 0 failures): START-DFS 446, RANDOM-DESTROY 402,
ACCEPT-ANNEAL 377 (best 394), START-RARE 373, greedy-mismatch 366; every
conflict-targeting destroy loses to random, exact refill (360) does not beat greedy
small (363), restarts null, band inert. The findings/index prose was finalised to
these numbers.

Two adjacent fixes shipped alongside. (1) The per-page raw-markdown exports
(…/research/<slug>.md) were lossy — only the intro prose, JSX tags left as noise.
Now they carry the hub's child-page list and the related rail (derived from the
manifest), unwrap <Callout>/<Door>/<ProseLink> into markdown keeping their prose,
and reduce <Figure>/charts/labs to explicit "[Interactive: Name] rendered on the
canonical page" notes so a crawling agent knows to look. (2) Navigation cleanup
per review: removed the /research/topics index (duplicated Overview, now redirects
there; the per-topic hubs stay, reachable from the left rail), removed the
`records` topic entirely (the hub was just links; several pages were mis-tagged),
and promoted the glossary into the research subnav.

Verified at close: cargo 6+15+7 (both profiles), research style/links/citations/
hardware, typecheck, lint, build all green. Reproducibility: `just experiments
repair-study`.

---

## 2026-07-16 — The "learning from strong boards" section (theory hub + lab hub)

Built the section the `learning` topic had promised since it was defined (topics.json
order 80) but never had a home for: a whole family of methods that, unlike every
other approach on the site, does not reason only from the puzzle's rules but mines
the corpus of strong boards already found for structure and feeds it back into a
search as a bias. Closer to imitation learning than to search design.

Two hubs, mirroring the DFS/repair studies' theory-vs-experiment split. Theory hub
`build/learning/` (new build subsection, group "Learned guidance", slotted after
"Local search" in BUILD_GROUP_ORDER): an index plus four technique pages and a
failure-mode capstone. `corpus-priors` (PRIOR's position count + LODESTONE's
scarce-demand weight, safe only as a tiebreak), `learned-value-ordering` (KEYRING's
three-signal vote), `anti-pattern-mining` (PALIMPSEST separating shared structure
from the shared trap via basin-split frequencies, steering not banning),
`decoding-records` (REPLAY rebuilding a witness exactly to recover the double-break
move), and `when-learning-collapses` (the capstone: over-trust collapses the search,
LODESTONE 451→380 as the weight climbs, and even used perfectly none of these raise
the ceiling because the corpus is made of boards stuck at the same rigidity wall).
Lab hub `lab/experiments/raphael-anjou/learning/`: a curated index framing the five
runs, a table of what each learns / how the search uses it / what it reached, and the
common wall.

The section is a genuine scientific arc, not a bag of tricks: the load-bearing claim
is the failure mode. A learned signal reaches the top of a search's own range fast
and reliably, and no learned signal raises the ceiling, because the ceiling is the
[rigidity wall] and the corpus cannot contain a way past a wall all its boards hit.

Consistency fix folded in (the plan's honesty item): REPLAY and LODESTONE genuinely
learn from the corpus but were not tagged `learning` — re-tagged both, so the topic
page and hubs now list all five experiments plus the four technique pages. Cross-linked
each experiment ↔ its technique page ↔ the lab hub; added the family to the
approaches-map (new "Learning from strong boards" section) and a door on the
raphael-anjou hub. No new components — reused PriorDiagram, KeyringDiagram,
PalimpsestDiagram, DoubleBreakDiagram/Lab where each technique's experiment already
uses them; the lab hub's five-way comparison is a plain markdown table (short
enumerable facts). No routes.ts/seo.ts edits — the MDX scanner registers the pages.

Verified: research style (0 em dashes, 0 banned phrases — the capstone's prose was
scrubbed of the "honest" tic the checker hard-fails on), links (150 routes),
citations --no-fetch (the capstone cites the rigidity-wall page so no page is
source-less), hardware (0 problems), typecheck, lint, build all green; all 7 pages
prerender and their .md exports carry the child lists + related rails.

---

## 2026-07-16 — Content review of the learning section: reproducibility + a false-claim fix

Reviewed whether the five learning experiments can actually be run. Answer: no,
and by design. Unlike the DFS and repair studies (which ship full engines +
`just experiments <study>`), none of the five corpus-mining runs
(PRIOR/KEYRING/PALIMPSEST/LODESTONE/REPLAY) have runnable search code in the repo.
The only backing code for this family is `research/topics/record-boards/compute/`,
a board *verifier* (recomputes matched-edge score from a board's bucas edges), not
a solver. The pages already treat these as exploratory `measured: false` runs whose
artifact of record is the board, not a re-run; making one runnable would be a
from-scratch engine on the scale of a whole study, plus the private strong-board
corpus. So "runnable" is a project, not a review fixup — left as-is intentionally.

The review did catch one real defect (pre-existing, not from the section build):
two pages claimed a board "checkable in the viewer" that is not in the committed
viewer data. record-boards.ts / boards.json hold only four boards (prior-460,
keyring-460, gauntlet-458, palimpsest-463); LODESTONE's 451 and REPLAY's targets
are not among them, and neither page even renders a <RecordBoard>. Fixed the prose:
LODESTONE now says its reproducible artifact is the tiebreak *effect* across five
seeds (true), not a viewer board; REPLAY now says the boards it rebuilds are the
community's own strict-460 records (on the record timeline), and what the experiment
adds is the replay. Style + links green.

---

## 2026-07-16 — Published dataset: benchmark instances + strong-board corpus (CC0)

Aggregated a public dataset under research/datasets/, two parts, CC0. First did
the data analysis the user asked for (are the corpus boards derivatives of one
another?). Answer: NO. Of 1,089 source boards only 11 are exact copies; the median
board differs from its nearest sibling by 60 of 256 placements; 28 distinct corner
families. Genuinely diverse, so publishing ~1,078 is honest — and the diversity
check itself is published as content.

Part A — benchmark instances (loose JSON, research/datasets/instances/): the 10
corner-pinned 16x16 variants (all three engine studies use the identical 10,
md5-confirmed) + 4 clue sub-puzzles (6x6/12x6), normalized to one schema
(id/family/width/height/numColors/pieces URDL/hints/maxScore). All 4 clue instances
validate against their known solutions (piece count, dims, internal-edge max).

Part B — strong-board corpus (research/datasets/e2-strong-boards.zip, ~620 KB):
1,078 distinct boards scoring 400-469, each {id, score, family, edges}. Ships as
CSV + JSONL inside the zip with README + LICENSE. Loose corpus/boards.{csv,jsonl}
(2 MB each) are gitignored — regenerable, fully contained in the zip. stats.json
tracked.

Two correctness properties enforced by the builder (research/datasets/build/
build_dataset.py): (1) every score RECOMPUTED from the 1024-char URDL edge string,
never trusted — caught 1 stale source score (453 claimed, edges score 456). (2)
edge extraction bug found + fixed mid-build: some bucas_url variants concatenate
&board_types/&board_pieces with no separator, so a naive lowercase filter pulled
"boardtypes...boardpieces" into the edges; fixed by taking exactly the first 1024
lowercase chars. After the fix all 1078 edges are exactly 1024 chars and all scores
recompute to the published value (0 mismatches). The fix also exposed 1 more exact
dup (1079->1078).

Provenance scrub (user: strip ALL vault identifiers): dropped source_path,
ops_preset, seeds, filenames, bucas puzzle names, any vol-NNN; kept only board
content + verified score + derived corner-family. grep-verified 0 leaks in the
committed outputs.

Site: new page build/dataset.mdx (kind: reference, "Check your code" group),
topics [structure, learning]. Describes both parts, the recompute-verify property,
and the diversity analysis. Cross-linked from the learning lab hub (the corpus
those 5 experiments mine). Links to the GitHub tree for downloads (repo is the
delivery mechanism, matching how other backing dirs are linked). Verified: research
style/links/citations/hardware, typecheck, lint, build all green; page prerenders
with .md export. `python3 research/datasets/build/build_dataset.py` rebuilds it.

---

## 2026-07-16 — Clue puzzle pieces as machine-readable downloads (+ a consistency fix)

Follow-up to the dataset: the clue-puzzles page already offered each clue's pieces
as a per-piece SVG zip (visual), but not as solver-ready DATA. Added a second
"Pieces (.json)" download button per clue in CluePuzzlePieces.tsx, next to the
existing SVG one, generating the exact dataset instance schema
(id/family/width/height/numColors/pieces URDL/hints/maxScore) client-side from the
component's own piece data. Exported downloadBlob from pieceSvg.ts to reuse it.

Found and fixed a real consistency bug in the process. The dataset builder's clue
parser used raw colour numbers from the source .txt files (jwortmann's clue3/4
leave gaps and run up to 22), while the site's clue-puzzle-pieces.ts compacts
colours to a dense 0..k. So the published dataset and the site DISAGREED on the
clue piece definitions (same puzzles, different colour numbering). Verified against
the authoritative topic solver (compute/src/main.rs), which does the same first-seen
compaction, that compacting makes the raw parse EXACTLY equal the site data for all
four clues. Updated the builder to compact identically; now dataset == site data,
and the in-browser JSON download byte-matches the committed instance file for all
four clues (verified). Also gave instance files a trailing newline (via a write_json
helper) so file and download agree byte for byte.

Net: the 16x16 instances changed only by a trailing newline; the 4 clue instances
got the corrected (compacted) colour numbering; the corpus is unchanged (still 1,078
boards, 28 families) and its zip re-verified provenance-clean. Prose on the
clue-puzzles page updated to mention the data download. typecheck/lint/style/links/
build all green; both buttons render for all four clues in the prerendered page.

---

## 2026-07-16 — Engines audit, run pipeline tags, and the "Combination pipelines" rename

Closed the two long-deferred lab tasks (28: tag runs with stages; 29: editorial
audit of engines/runs), plus two follow-ons the user asked for mid-session.

**Engine audit (task 29), via two parallel Explore agents (audit + stage-extract).**
The "engines section published the easy way" concern was justified for 2 of 3 pages.
Fixes: removed the duplicate engines/csp-presets.mdx concept stub (the measured
version under single-core-benchmark has the leaderboard + numbers + repro), redirect
added, index door repointed; stopped the engines index leaning on the unpublished
beam/ALNS engines as if visible (they are not on the leaderboard, stated plainly);
Verhaard page reconciled the cross-metric slip (438 matched edges vs McGavin's 204
pieces-deep are different scales) and pointed at the committed rerunnable 438 board.
Caught + corrected a false claim I nearly introduced ("producer is on the
leaderboard" — it is not).

**KEYRING honesty:** read the actual v181_keyring.rs source — a pure beam that saves
its board "for ALNS feed", so the committed 460, like PRIOR's, takes beam + a
refinement tail. Said so, matching PRIOR's disclosure, instead of presenting 460 as
the raw beam result.

**vXXX id strip (user request):** v155-prior-460 -> prior-460 etc. across data files,
components and pages; joins verified consistent. Never leak internal volume numbers.

**Run stage tags (task 28):** added a `stages` frontmatter field (closed engine
vocabulary + `does` + optional `learns` + a `published` flag), wired through the
zod schema, the manifest, and the ResearchDoc type. All ten runs tagged. The
`published: false` flag records honestly, as data, when a stage runs on the still-
unpublished beam producer or ALNS. Decision (with user): keep the beam-dependent
runs, make the dependency honest rather than pull them.

**Rename runs -> Combination pipelines (user request):** the tagging proved these
are multi-engine compositions, not single algorithms. URL folder
raphael-anjou/runs/ -> raphael-anjou/pipelines/, title "Combination pipelines",
~112 inbound links rewritten, /runs/ redirects added for every moved page.

Scope discipline: a parallel IO-standardization effort was live this whole session
(e2-io format.rs/bucas.rs, Convert.tsx, Print.tsx, build/formats.mdx, dfs/repair
study scripts). Kept every commit strictly to my files; explicitly un-staged
formats.mdx when a blanket add swept it in. Verified per commit: research style/
links/citations/hardware, typecheck, lint, build all green.

### 2026-07-17 — Full-site content quality audit + fixes (multi-agent)

A scientific-publisher content quality audit across the whole site, then the
fixes. Method: a 70-agent workflow reviewed all 113 research MDX pages + 14 TSX
product pages against a 7-dimension rubric (correctness/evidence, clarity,
metadata, consistency, language, tone, accessibility), with corpus-wide checks
(terminology, numbers, links) and adversarial re-verification of every
critical/major finding. 182 raw findings -> 39 verified serious (7 rejected as
false positives) + 136 minor. Report + machine-readable findings saved under
`research/audits/`. Overall grade: A-.

Fixes applied across 87 files (81 MDX + 6 TSX):
- **One critical fact resolved.** Status/RecordsView/boards/hunt-part-2/people all
  say the 470 record was tied *once* (Bucas, Dec 2024); Puzzle.tsx alone said
  "twice / 2024 and 2025" (EN+FR). The 2025 tie is unsupported anywhere — Puzzle.tsx
  was wrong, corrected to match. (See memory [[e2-470-tie-count]].)
- **Self-contradicting numbers** fixed against each page's own data: repair-study
  swing 14->16 and "low 400s"->"360s-370s" (recomputed from results.jsonl),
  single-core "24 points short" (best 451 is 13 short; only the mean is ~24),
  solver-engineering "4x"->"~1x to 4x", sat-csp 262,144 reconciled by pruning,
  how-hard mantissa 1.15->1.115, known-facts date-range overlap.
- **Convention/voice:** three "invention"->"experiment" (AGENTS.md rule); sigma-cycles
  rigor proven->conjectured with title/470 claim scoped; we-voice process-narration
  recast on several pages; superlative/puffery trimmed.
- **8 truncated frontmatter descriptions** completed; `/tree/`->`/blob/` on 14
  file-pointing source URLs; engines hub gained a Door to the unlinked
  verhaard-reimpl (438) engine; garbled DLX/mismatch-geometry sentences fixed.
- **Terminology:** `16x16`->`16×16` in prose (~130, careful script skipping
  slugs/domains/code); metric name unified to the "edges" family on TSX
  (EN+FR); `frame-first`->`border-first` for the *fill-order name* (3 spots),
  but the `frame-first anchor`/`frame-free` compound deliberately kept.
- **Source archaeology** (17-agent workflow over the local groups.io archive at
  `../../v2/community-exports/messages.jsonl`, 11,511 msgs, every proposed
  citation adversarially re-verified against the exact message): caught a real
  misattribution (the "modified Blackwood's DFS" phrase is Igor Pejic's words,
  wrongly credited to Riotte — boards.mdx fixed, memory updated); clarified the
  kubzpa 479-parity entry (interior parity holds; loophole is the unscored rim,
  +msg 6319); added msg 10117 (Blackwood "470 Found") to approaches-map and an
  inline PALIMPSEST link for the 463; softened 5 genuinely-unsourced claims
  (producer-455->451, "brand-new 458", palimpsest 461/462 threshold,
  Annaert/onesmallstep Discord anchors marked unlinkable). Left dead-ends and
  the raphael-anjou scores alone (verified as non-defects: the "Reported" tier
  honestly promises no repro; Millilaw has zero archive posts).

Gotcha caught on myself: `sources[].url` is zod `.url()` (absolute only) — a
relative internal path fails the build loudly. Internal experiment evidence goes
in `related[]` + an inline prose link, never `sources[]`.

Citations 723 -> 725 (both new msgs verified in the archive). Verified: research
style (0 em dashes, 0 banned phrases), links, citations, typecheck all green.

### 2026-07-21 — Structural rework: SOTA surface, open-problems board, newcomer path, article chrome (multi-agent)

A structure-focused pass (the 2026-07-17 audit covered copy quality; this one
covered organization, article anatomy, and how experiments are showcased).
Method: an 81-agent gap analysis (6 web best-practice studies with named
exemplars: Chess Programming Wiki, Complexity Zoo, cube20, PapersWithCode,
Diataxis, distill/gwern, TASVideos, Erdos problems; 6 site-state readers;
8 judges; per-finding adversarial verification). 61 raw findings -> 52
survived (6 refuted against the repo, 3 verifier failures). Then a 6-agent
implementation workflow on disjoint file sets.

Shipped (commits 8ecf6df, 33edbf7, 1a6adf9, 57b1be0):
- **Chrome:** contribution/outcome badges, labeled tier chips, scoring-
  convention pill, header repro strip, verify-vs-reproduce split (record-
  boards commands now say "Verify the stored board"), stages[] renderer with
  unpublished markers, sidebar kind labels, Callout kind="open". New optional
  frontmatter: outcome, repro.produces, repro.scope, scoringConvention.
- **Records = canonical SOTA:** community-vs-project comparison table,
  timeline zoom, re-score labeling, sibling-boards accordion, shared census
  constant, cross-links from the pages that hand-restated the headline.
- **New /research/open-problems** (EN/FR/ES): status-tagged open angles,
  unattended vs attempted-and-hard, Owen's set_2 as the newcomer target.
- **Newcomer path:** numbered start-here on the root, where-to-start on all
  doors, ordered build list (toolkit step 1), techniques shrunk to a hub,
  solvers anatomy table, dead-ends "Instead:" pointers, contribute loop-back.
- **Lab comparability:** outcome/scoringConvention/wallClock backfill,
  experiment-scores.ts dedup, cross-researcher scored table on the hub,
  chart legend + strict-464 line, generated /research/build/reproduce and
  by-contribution hubs (negative results browsable).
- **Infra:** citation-checker archive path fixed after the v2->research
  rename (had silently broken the local msg_num validation).

Verified: typecheck, full build (all new routes prerender EN/FR/ES, sitemap
picks them up), check:research (0 em dashes, 725 citations 0 missing, parity
100%), check:i18n 100%, 43/43 tests.

Deferred backlog (verified-real but not this pass): glossary inline term
tooltips (needs localized glossary.json first), auto-backlinks ("Referenced
by" from the link map), per-variant spread plots (needs make_site_json.py
rerun for per-instance scores), in-house ladder progress chart, dfs-study
live path-order widget, hub board thumbnails, repro-presence CI for concept
pages. The "one overloaded kind field" finding returned a corrupt verifier
verdict and needs re-judging before any action.

### 2026-07-21 — UX backlog: in-context definitions, backlinks, thumbnails, spread plots, live lab (multi-agent)

The verified-but-deferred UX backlog from the morning's structural pass,
shipped by a 5-agent workflow with disjoint file ownership (commits 798dcdd,
fe9c6f0, and the studies commit following). Glossary localized to EN/FR/ES
and surfaced in-context via a build-time first-mention auto-linker with
tooltip popups; prose-link backlinks rendered as "Referenced by"; optional
board frontmatter puts best boards on person/topic hubs; dfs+repair studies
emit per-instance scores and render spread strip plots (the "~11-point
spread" prose claim now has its chart); dfs-study gains a live scan-order
lab on the real WASM engine and the Gauntlet race lanes become free-choice.
Palimpsest deliberately got no widget (corpus-level claim; a control would
be decorative). Verified: typecheck, full build, all research gates, i18n
100%, 43/43 tests. Remaining backlog: in-house ladder chart, repro-presence
CI for concepts, the kind-field re-judge; next planned track is publishing
the vault's 461 strict record + recent negative results.

### 2026-07-21 — Backlog close-out: repro gate, kind vocabulary, ladder chart declined

Final backlog items (3-agent workflow). New repro-presence CI gate
(check-research-repro.mjs, 62 claim pages audited green) + repro: prose
backfill on 32 expository pages + the status-tier sentence in contribute.
The crashed-verifier "overloaded kind" finding was re-judged: WEAKENED;
kind drives no layout (the shell is deliberately uniform) and the defect
was vocabulary, so hunt/hunt-part-2/history moved reference -> page; no
per-kind layout branch. The in-house ladder chart was implemented and then
DISCARDED before commit (user call, confirmed by analysis): the published
scores are parallel named experiments, not a dated progression; charting
them as a ladder would assert an ordering no page claims, and the honest
version needs the strict ladder published through the pipeline first.
Backlog from the 2026-07-21 gap analysis is now closed; the open next
track (on hold, needs vault publication) is the 461 strict record + recent
negative results.

### 2026-07-21 — /research/why deep review: corrections shipped, proposal + experiment slate pending

49-agent review of the why section (claim inventories on a five-tier
evidence scale, confidence map, wall-interdependence analysis, adversarial
complexity read, structure/consolidation judgment, first FR/ES native
audit; every major finding adversarially verified: 22 survived, 2 refuted).
Verdict: no rebuild (no page clears the fold bar; walls-and-methods IS the
synthesis); the real debt is five headline claims not backed by their own
repro commands (entropy alpha/80-cell offline-carried; sigma-cycle numbers
page-only; rigidity MIP table uncommitted; phase-transition repro covers
only the color split; prune-vs-speed results do not regenerate).
Corrections committed (db105ad): inverted complex-theory depth claim fixed
against msg 11725; design-recipe record sentence + retracted aabb (msg
8034); hint-geometry 88+ and border-balance 85% softened pending
measurement; phase-transition title hedged to the band; entropy/sigma/
rigidity presentation recalibrated to committed evidence (values kept);
index walls heading fixed; how-hard de-orphaned; tiers completed; FR
plateau unification; ES tu harmonization + separators. Notably refuted by
verification: 2 findings (benj39100 framing, Zamofing attribution), so
they were NOT "fixed". Pending user approval (audits/why-review-2026-07-21/
PROPOSAL.md): confidence-map rendering, connective-tissue prose + 3
open-problems questions, order alignment, and the 8-experiment slate
(E1 sigma-cycles all-pairs, E2 SAT halo extension, E3 in-repo area law,
E4 hardness-peak sweep, E5 prune-vs-speed repro fix, E6 contiguous-hint
measurement, E7 border recount, E8 archive refresh).

## 2026-07-22 — Fourteen reproduction drafts run through the starter kit + kit additions

Fourteen findings from the private research program landed as draft topics
(`research/topics/<id>/`, status draft, not yet surfaced), each a
self-contained reproduction package: the claim with its expected numbers, a
declared reproducibility tier (exact / seeded-statistical / qualitative), a
compute crate on the starter-kit substrate, and committed results. All
fourteen were executed; none failed. Six reproduced fully
(adversarial-piece-set, ring-purity, parity-defect-floor, design-recipe-464,
frame-manifold, cas-annular), two reproduced with a stated caveat
(beam-width-smc, tail-finishability-frostline), six are partial with the
reproduced half committed and the blocker named in the article. Reproduction
caught three source errors or over-claims that the articles now record:
a rounding artifact in the 470-wall first-moment count, an engine-bound
middle in the constraint-immediacy order ranking, and a colour-count
dependence of the N=13 solvability cliff. Exact-tier results were re-verified
byte-identical in four independent audits.

The starter kit gained the shared plumbing those topics kept re-deriving:
`e2_kit::analysis` (piece-set census, tested against the official set's
structure), `e2_kit::fit` (the example's placement arithmetic, promoted),
a worked backtracking example with configurable visit order, a seeded-RNG
re-export with a bounded draw, paired t and Wilcoxon statistics in
`scripts/compare.py`, a lossless `Instance::to_site_json` export for
non-Rust consumers, board-size feature passthrough, and a documented
piece-for-piece guarantee that `data/official.json` matches the community
CSV (verified 256/256, same order, same sides).

## 2026-07-22 — Six lab articles: Raphaël's explorations, reproduction-backed

Six write-ups landed in the lab section, each in native EN/FR/ES and each
backed by a published reproduction topic with committed results and a
`just research-<topic>` recipe: CAS (concentric annular solving; the 430-437
plateau from any perfect frame and the every-frame win over a
destroy-and-repair continuation), the fluid frame (perfect borders form a
connected manifold; 45 free exchanges, dead frames revive through one free
swap; mechanism only, the producer is future work), LEDGER (the sound
supply/demand prune; zero unsound fires, certificate rows on the community
464 boards with the compounding node-ratio law), beam width (width dominates
every per-node improvement; tie-break randomisation and SMC resampling are
the two real additives), FROSTLINE (a belief-propagation residual predicts
one-row tail finishability across producer regimes), and the scaling ladder
(planted fully-solvable rungs with proven ceilings; collapse size measured
before real-board compute is spent). Six backing topics flipped from draft
to published in the registry. Review pass before landing: internal volume
numbering scrubbed from public text, a wrong remembered DOI caught and
corrected against the actual CP 2008 chapter, source-study figures
attributed and given article-level traces, wall-clock strings aligned with
the hardware block's measured flag.

### 2026-07-22 — Why-section upgrade: seven experiments close the reproducibility debt + confidence map

The approved why-review follow-through (7 experiment agents + 1 structure
agent, then orchestrator verification). Every previously-unbacked headline
now has a committed artifact or an honest label:
- sigma-cycles: NEW topic; 246 ordered pairs / 1,154 large loops / 54,238
  partial applications, zero >= start. Three-pair conjecture -> exact
  population result (page + Lab updated).
- border-mismatch-share: NEW topic; 86.9% interior-interior, 0 border-border
  over 99 mismatches on the nine 469-class boards.
- rigidity-sat-halo: radii 1-4 on five boards (incl. Riotte 464); 18/20
  UNSAT, two radius-4 TIMEOUTs committed open. NOTE: the implementing agent
  had edited the page before its sweep finished (its session was captured
  mid-wait); the orchestrator caught the unbacked-claim window, reran the
  sweep to completion, and reconciled page vs artifact before commit.
- entropy-area-law: in-repo A(n)/B(n) exact through n=3 (B(2) cross-checks
  subgrid table); fitted alpha 0.044 supersedes off-site 0.085 ON THE PAGE
  with the change stated; five satellite pages de-numbered to links
  (the volatile number lives on one page; everyone else links).
- prune-vs-speed: stale results regenerated (engine generator had been
  rewritten twice since commit); qualitative claims unchanged.
- hint-study: contiguous-row family added; crossover = 5 rows/40 hints vs
  the 16-hint scattered lattice (medians 107 vs 3,913 nodes).
- archive: extended to msg 11931; citations 11848/11856/11901/11902
  de-allowlisted, now validating from the export.
- Structure: "What each wall rests on" confidence map on walls-and-methods
  (written against the post-experiment tree), connective tissue on the
  index (two supportable relationships stated; three causal questions
  routed to open-problems as entries), order aligned to the argument.
All gates green (0 em dashes, citations 0 missing, anchors, hardware,
repro, parity, i18n, 43/43 tests, full build). One expected transient:
the new topic's GitHub tree URL 404s until push. E4 (hardness-peak sweep)
remains queued as the last experiment.

### 2026-07-22 — E4 hardness-peak: Owen's criterion passes a scale-transfer test

The slate's last experiment. Calibration overturned the proposal's design:
a plain DFS never solves a full 16x16 under any feasible budget, and the
format's 22-motif alphabet caps a framed 16x16 at exactly 17 interior
colors, so the peak is only traversable on a smaller board. Dual-regime
result: (8x8) solve-rate collapses to 0/90 at exactly c=6-8 under a 30M
budget; the criterion's size-8 prediction 7.5579 (same formula that gives
Owen's 17.1377 at size 16, computed by the crate's --criterion mode) lands
inside the band. (16x16) 0/570 solved, median depth declines 238->199.5
to the ceiling; c=18..26 clamp bit-identical to 17. The full board sits at
the predicted peak and the format cannot express its far side.
phase-transition.mdx measured-here paragraph + repro scope (EN/FR/ES).
Ops note: one sweep was killed by editing run.sh while it executed
(orchestrator error, sequencing); rerun clean on the refactored resumable
runner. All gates green; full build; 30 topics.

## 2026-07-22 — Lab section reorganization: routing fixed, nothing moved

A structure review of the Raphael Anjou lab section (exemplar study against
Complexity Zoo, gwern.net, Diataxis, TASVideos and Chess Programming Wiki
hub patterns, then an adversarial verification pass over every proposed
change) concluded that no page needed deleting or moving: the content was
sound and the routing was broken. Applied: the section homepage now surfaces
the whole corpus (a hint-study door and the four-studies count corrected in
all three languages, a Findings-and-instruments strip for frostline, ledger
and scaling-ladder, a Where-the-notebook-stands narrative with both scoring
conventions stated side by side and the community standings linked, a
Being-written-up-next slot naming queued work, and a manifest-driven
Recently-added strip that keeps future publications from starting
orphaned). The pipelines, engines and meet-in-the-middle hubs were rewritten
to describe their children truthfully; the engines hub became the apparatus
hub its door promises. The hint study gained native French and Spanish
versions of all three pages and meet-in-the-middle its missing French index.
One frontmatter pass unified sidebar, door and prev/next order (four order
ties broken), retitled the colliding study sub-pages, and added reciprocal
related links so no page is an island. Two code fixes: the CAS row was
missing from the shared score table, and translated hardware blocks were
being ignored by the manifest build (English fallback preserved). Verified
findings that were refuted or deferred are recorded in the analysis run;
the one deferred item is a uniform reproduction-status sentence on the
older command-less experiment pages.

## 2026-07-22 — Records lineage, the theorem-sweep cluster, and a hygiene sweep

The records page gained a clearly separated notebook-lineage section: the
strict five-clue line (comb-order producer 457 raw, destroy-and-repair lift
through 459 and 460 to 461) and the best matched-edges board (463,
PALIMPSEST), every row convention-tagged, with an explicit statement that
nothing in the section is a community record. Four new theory pages landed
in why/: the theorem-sweep synthesis (a map of thirteen proved structural
families), ring purity (the zero-slack Eulerian border), the parity defect
floor (why 479 is impossible), and the 470 wall (the plateau as an entropic
phase boundary, exact layer separated from labeled conjecture). Each is
backed by its published reproduction topic. The piece-side-matching
480-tight negative was distilled into the LP-relaxations page, and the six
command-less lab pages now state their reproduction status uniformly.
Review catches fixed before landing: a wrong final character in the
Conway-Lagarias DOI, a 17-versus-16 check-count mismatch against the
committed census, an unscoped LP-ceiling comparison, and a seed-2 figure
quoted beyond what the committed JSON records. The review also surfaced
older debt invisible to the body-only style gate: em dashes in
metaDescription frontmatter and French/Spanish equivalents of banned words
across seven pre-existing pages, all cleaned.

## 2026-07-22 — First wave complete: the last four theory pages

The publish program's first wave closed with four pages in why/:
why-e2-is-hard (the flagship piece-set thesis: four committed measurements
from the adversarial-piece-set reproduction plus the palette split sourced
to the community record, twin pairs listed by id), constraint-immediacy
(the 480 path-invariance theorem with the engine-bound-middle caveat as a
first-class part of the claim), an enrichment of how-hard-is-this-instance
(the DFS collapse at twelve on 22-colour boards, the encoding gap, the
colour-count dependence of the cliff, and the correlated-agreement trap),
and clue-corridors (the 2-quadrillion corridor count reproduced exactly;
the ladder A/B attributed to the source study pending its rerun). Review
catches fixed before landing: a 3-of-4 that the committed JSON says is
4-of-4, a five-that-was-four measurement count, a corridor length
convention made explicit on page and checker alike, a Spanish
mistranslation of checked-in as confirmed, and scoped wording on the
CP-SAT slowdown and DFS sizes.

## 2026-07-23 — Merge wave: ten existing pages absorb their vault batches

The second publishing arc's first pass enriched ten already-live pages with
the vault material routed to them, rather than adding new pages. New sections
and entries landed on build/dead-ends (pattern databases, shared transposition
tables, Eulerian border-closure, forbidden-pattern pruning, and more failed
approaches with their mechanisms), why/rigidity-wall (worked MIP and SAT-halo
rigidity proofs, the donor-cell window trades scoped to the two larger windows,
the halo metric labeled Chebyshev vs Manhattan), why/sigma-cycles (level-set
clusters, twin swaps, the boundary-growth law), why/mismatch-geometry (skeleton
variability, the universal heatmap, the interior edge-gap), build/construct/
beam-search (width as the real dial, the Gumbel-temperature collapse, diversity),
build/backtracking/fill-order (scan order, prefix determinism, the depth-160
node-count peak), build/exact/lp-relaxations (the LP-integer gap anatomy, the
480-tight side-pairing negative), build/exact/meet-in-the-middle (the memory
crossover at n around 60, the band-split enumeration cost), and
build/faster/solver-engineering (candidate lookup tables, minimal perfect
hashing, bit-packing, the measured node-cost anatomy). One page created new:
build/construct/band-column-dp (the two-row band dynamic-programming family:
one band is always perfectly solvable, chaining reaches 444 of 480 in 35 s
with the entire shortfall provably in the vertical seams, the direction-
symmetric greedy-horizon failure), registered in the construct hub.

Reproduction gate held: every added number was verified against its vault
source in an adversarial review pass. Twenty-one findings were confirmed and
fixed before landing: a minimal-perfect-hash pointer re-credited to the
community member who actually shared it, the meet-in-the-middle memory
crossover corrected from n around 56 to n around 60 (checked against the
interactive cost component), two throughput figures flagged as being on
different puzzle sizes, a Spanish mistranslation of top boards, a depth-161
that should read 160, decimal-comma typography in French and Spanish LP tables,
and a from-scratch 460 given its records-page context so it is not read as an
overall record. Style and translation-parity gates green across 402 files;
full build clean.
