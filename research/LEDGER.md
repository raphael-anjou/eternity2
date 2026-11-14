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

## 2026-07-23 — Second wave: six new reproduction-backed pages

Six standalone pages shipped, each gated on a kit reproduction topic that was
built and run (four REPRODUCED, two WITH-CAVEAT, zero FAILED):
why/flux-invariants (the complete lattice of linear tile invariants: a color
census, a Gaussian-integer flux law, a checkerboard parity law; instance facts
and rank 22 reproduced exact, digit-for-digit), why/permutation-code-wall (the
codeword/syndrome lens; the score = 480 minus breaks identity reproduced
exactly, the reframing labeled as a reframing), why/no-height-function (the
defects reformulation and its three-part negative law, qualitative tier: the
law reappears on our own boards while the paper's board-specific counts do
not, and are labeled as not-reproduced), why/frame-is-not-the-basin (the
frame is not the basin selector, extending the frame-manifold reproduction,
seeded-statistical), build/exact/exact-tail-endgame (the bottom-band endgame
as its own exact problem; mechanism and delta shape reproduced, absolute
digits scoped), and why/irreducible-hard-region (the localization half of a
three-part conjecture reproduced, the rest labeled conjecture).

Adversarial review returned 18 findings, all verified and fixed before
landing. The load-bearing ones enforced the records-framing rule: a 463
matched-edges basin-attack board and a 451 board had been mislabeled as
best from-scratch and were corrected (the from-scratch ceiling is 460, the
notebook record 461, and the community marks 464/470 are higher). Also fixed:
a rigidity link that pointed at the mismatch page instead of the rigidity
wall, two medians miscomputed against the committed run logs (12.5 not 14,
4.5 not 5), a repro-tier field added to match the topic, and several French
mistranslations and metaDescription parity gaps. Full build, style, i18n,
parity, and 43 tests all green.

## 2026-09-17 — A call to action: the five-clue fleet

The site gained its first page about something still running, plus a homepage
call to action pointing at it. Benjamin Riotte, who took the strict five-clue
record from 460 to 464 in July 2026, opened a shared runner infrastructure to
the community in August 2026: a control plane that allocates search roots,
records completed work units, deduplicates them and re-scores every submitted
board, with a Windows/Linux worker anyone can run on the threads they choose.
The announcement reports 7,645 boards at 463, 327 at 464 and 11 distinct boards
at 465 on the strict five-clue board, at a peak of 28 machines and over
12 billion nodes per second.

New page `/research/community/five-clue-fleet` (EN/FR/ES) documents it: what
the orchestrator actually shares, what the fleet has found, what the solver is,
and how to join. The counts are attributed to the project's own announcement
and explicitly not recounted here, and the page links `/research/records` as
the canonical record table rather than restating it. The strict-record pages
still carry 464 as the wiki's verified line: the 465s are the project's report,
not a wiki verification, and updating the record ledger is a separate step.

Also: a highlighted CTA section on the homepage (trilingual, links the new page
and the worker download, with the dashboard in the fine print), a door on the
community hub, and a closing pointer from
`/research/build/faster/distributed-solving`, whose fifteen-year verdict (the
scarce resource was never compute, it was the bookkeeping) this effort is now
testing live. Style, links, anchors, citations, hardware, repro and parity gates
green across 423 files; build, typecheck, lint and 90 unit tests clean.

## 2026-09-17 — Five-clue fleet page corrected against the full thread

The announcement thread was read in full and the page was corrected on two
counts. First, a factual error: the mechanism described as "automatic
deduplication of symmetric roots" is not what the system does. Work tickets are
deduplicated through a completed-ticket ledger every worker downloads before
starting (an audit of 3,458,636 completed tickets found 3,458,636 distinct ids,
so no unit of work was computed twice), while boards are deduplicated after the
fact by canonical SHA-256, with the first discoverer keeping public credit. The
page now describes both, plus the ticket shape (root, deterministic offset,
budget capped at two billion DFS nodes) that the thread spells out.

Second, staleness: the announcement's counts are a snapshot from 30 August, and
the 465 count moved within a week. The numbers are now presented as a dated
snapshot with the dashboard and board library named as the live source, and the
homepage call to action carries no figure that can go stale (it says the
five-clue line is past the 464 this wiki records, and points at the dashboard).

Added a section on why a contributor's local counter outruns their public
credits: re-found boards are convergence of distinct tickets, detectable only
once the board exists, about 0.15% of completed tickets in the measured window.
It closes on Carlos Fernandez's exchange analysis of the first two 465s, which
generates 32 boards at 465 from them by border-piece exchanges and rotations:
32 boards, not 32 discoveries, and the rigidity picture from the other side.
All gates green; build, typecheck, lint and 90 tests clean.

## 2026-09-17 — The strict five-clue record moves to 465

The record ledger was updated across the site: the standing strict-five-clue
mark is now **465/480** (15 broken edges), found by the community's distributed
fleet in August 2026 and credited across several contributors' machines rather
than to one person. It supersedes Benjamin Riotte's own 464 of July 2026, which
held two months, which in turn ended Bruno Gauthier's 460 of 2023.

One board is not one discovery on this line, and the pages say so. The fleet
reported eleven distinct 465 boards at its announcement and many more since, but
Carlos Fernandez's exchange analysis turns the first two into 32 boards at the
same score by border-piece exchanges and rotations. The count of boards runs
well ahead of the count of independent ways to reach 465, and the records,
boards and fleet pages all carry that caveat.

No 465 board is bundled here. The boards live in the fleet's own public library
and this project has not decoded them, so `known-boards.ts` and
`community-boards-2026.ts` still carry the July 464 census, relabelled as such,
and the record row in `records-timeline.ts` is the first with no `board`.

Touched, in all three languages: the records view and its reference list, the
record and history timelines, boards, people, known-facts, open-problems
(retitled "the strict five-clue 465-to-466 target"), clue-puzzles, variants,
hunt-part-2, theorem-sweep, no-height-function, parity-defect-floor,
permutation-code-wall, rigidity-wall, the single-core benchmark, the DFS study
and REPLAY, plus the Status, Puzzle and Home pages and the AGENTS.md SOTA note.
The benchmark reference constants (`single-core-benchmark.json`,
`dfs-study.json`) moved to 465 as well; the measured scores are untouched.

One latent bug found on the way: `RecordTimeline` decided a row's track by
matching its author string against a hardcoded set, so the new record, whose
author reads "Benjamin Riotte & the five-clue fleet", plotted on the open track
and dragged the 470 line down to 465. The track is now declared on the data row
(`regime: "strict"`) and the author heuristic is gone. All gates green; build,
typecheck, lint and 90 tests clean.

## 2026-11-11 — GPU page rewritten after an external rigor review

The GPU page argued that GPUs cannot help, resting on Mike Field's 2011
memory-bandwidth argument (msg 9003). An external review found the thesis
stale and contradicted by the site's own benchmarks page (2026 GPU solvers
above 10 billion placements/s, msg 11826; Barr's port ~100x his CPU core,
msg 11835) and by GPU annealing with 4096 replicas on the rigidity page
(msg 11902). The rewrite separates the 2011 argument (read as history: it
conflates dependent-load latency with bandwidth, ignores that read-only
tables are shared, and uses 2011 on-chip memory figures), the measured
results, a fair-comparison section (same puzzle, same counting convention,
per socket rather than per core) and the open problems (a tuned full-board
GPU backtracker with matched node counts; per-watt numbers; GPU population
methods). Divergence and load imbalance are framed as solvable with standard
techniques, cited to the GPU backtracking and branch-and-bound literature
(Jenkins 2011, Gmys 2016, Carneiro Pessoa 2018, Cederman and Tsigas 2012,
Weigel 2012). The "GPUs verify, they don't search" split is dropped: the 9x9
re-verification is the same DFS on a smaller tree.

Learning: an older page anchored on one historical argument drifted out of
date while newer pages recorded the counter-evidence; nothing linked them.
No first-party GPU measurement exists; the page says so.

Reproducibility: prose (literature and archive citations). Message numbers
not re-checked against the local archive export (absent from the checkout).
Files: build/hardware/gpu-solving{,.fr,.es}.mdx, the GPU paragraphs of
build/hardware/fpga-solving{,.fr,.es}.mdx, the GPU bullet of
build/approaches-map{,.fr,.es}.mdx; then the hardware hub description
(build/hardware/index{,.fr,.es}.mdx) and the FPGA page's economics, ReConFig
and rack-year paragraphs. Commits: cd55b20, 5748d05.

## 2026-11-11 — Rigor pass: top-pages-and-tsx (external review follow-up)

Rigor pass on the top-level research pages and site TSX (commit c1907b7,
branch devin/1794390000-rigor-top-pages-and-tsx). All 17 audit findings were
checked against the repo, all held, and all were fixed in EN, FR and ES. Every
'about one / single solution' claim is reconciled with solution-counting.mdx:
about 14,702 expected solutions with the centre clue and effectively one with
all five, as an ensemble first-moment estimate. This covers Scam, Status,
RecordsView, PapersView and open-problems. Owen's msg 1947 is now an estimate,
not a proof of 'hardest possible'. Literature notes (Mateu 2012, Ansotegui
2008, Bejar 2009) now describe ensemble phase transitions, and the claim that
they explain E2's design is gone. Record lineage is credited to Verhaard's
engine, Blackwood's scheduled backtracker and Riotte's DFS rather than CP plus
VLNS or propagation and repair. McGavin's 295M/s figure is separated from his
2020 469, which was found with Blackwood's solver on about 200 cores. Set_2 is
now priced at the set_1 cost, about 2x10^17 nodes or 180 core-years. One solve
is a geometric draw spanning about 1/20 to 3x the expected cost (5th to 95th
percentile), and newcomers are sent to the 7x7-9x9 ladder. On Algorithms, the
22-colour extrapolation was wrong: the arithmetic gave about 14, the 8x8 peak
is censored, and the data came from the unframed painter. It is replaced by
the measured peaks (4, 6 and 6 to 7) plus Owen's criterion, recomputed to 5.2,
6.4, 7.6 and 17.14, and the committed 16x16 sweep is reported as having found
no peak (0 solves for interior colours 8 to 17 at a 20M-node budget). Puzzle
now contrasts the 10^560 teaching number with the about 10^45 pruned tree and
frames NP-completeness as a property of the family. A null model shows E2's
'no duplicates' is non-random (99.4% of random paintings have at least one
duplicate) but 'no symmetric piece' is not (48% of random paintings have one).
That Monte Carlo was done ad hoc and the script is not committed. Rigidity
claims are limited to the 3-4 basins tested, and the score gap is no longer
presented as distance to a solution. Learned: overclaims about design and the
solution count recur on non-owned pages (why-e2-is-hard 'scarcest', known-
facts 'single', phase-transition 'maximally hard'), and the Verhaard 467
method is described inconsistently across pages. Files:
web/content/research/{open-problems,contribute,people}{,.fr,.es}.mdx;
web/src/pages/{Algorithms,Puzzle,Scam,Status,Home}.tsx;
web/src/components/research/views/{PapersView,RecordsView}.tsx.

Verified 18 audit findings and fixed them, rejected 0 as not holding, deferred 4 (new experiments or files outside the group). Source commit c1907b7 on devin/1794390000-rigor-top-pages-and-tsx, integrated via devin/1794392000-rigor-top.

## 2026-11-11 — Rigor pass: lab-raphael-a (external review follow-up)

2026-11-11. Rigor pass on the raphael-anjou lab pages (hint study, DFS study,
going-fast, JIT backtracker, frostline, learning study, notebook hub, engines
hub), EN/FR/ES, following an external read-only audit; each finding was
checked against committed results before editing. Hint study: the five-clue
result is now scoped to the 8 s best-partial score of strict DFS. Paired data
from research/experiments/hint-study/results/results.jsonl show 7 of 8 orders
lose, clue-rows-first gains slightly (median +1), and the non-chronological
beam also loses ~3 on 13/15 boards, so the harm is not specific to
chronological search and needs a null control (not run). The 8x8 solve section
now carries the no-hint baseline from results/solve8 (24/30 solved, median
~1.8M nodes, 6 timeouts): 4 spread hints are worse than none (2/30),
contiguous blocks match none, and only 16+ spread hints help (~3.9k nodes).
The method page documents the faithful_colors rule as implemented (n=16: 5+18
capped to 5+17; n=8: 2+6; the code comment saying 2+7 is stale). The b^{sum f}
cost law is replaced by a descriptive frontier correlate and a fail-first
account. DFS study (results/report.md, results.jsonl, run_meta.json): one
deterministic seed, so comparisons are paired per instance. MRV runs at 6-8k
nodes/s, nearly four orders below row-major, so heuristic conclusions belong
to this implementation. Forward-checking sits 2-3 below plain MRV. Verhaard-
slip is bimodal (4/10 match BREAK-1, 6/10 stall near 380 with 2-3 breaks).
Border-first closes the frame, then stalls at depth 66-77. The cross-
validation claim was dropped. Codegen scores below row-major on 9/10. Hardware
is 6 concurrent single-threaded runs on 4P+4E cores. Speed pages: McGavin's is
the fastest published single-core CPU engine, not the fastest overall. The JIT
comparison is within ~5% on one hard board with different scan paths (not the
same tree). The 2.8x easy-board lift (different boards) and the cache-miss
explanation were removed. 4.9e32 years is exhaustion of the five-hint tree (~1
expected solution; halving gives ~2.5e32), and exact search is separated from
stochastic record hunting. Records came from break-tolerant DFS with large
compute. Frostline: BP is exact only for the relaxed chain model, and the
exact label is cheap exactly where the predictor works. Learning: the 451-463
plateau sits below the corpus's own high-460s boards, so it belongs to the
learners. Missing equal-budget baselines, PALIMPSEST's perturbation control,
LODESTONE's null model and PRIOR's nearest-board distance are stated as open.
Learned: committed result files disagree in places (community.json vs
community.md), single-seed grids were described as replicated, and
metaDescription is capped at 300 characters (caught only by pnpm typecheck).
Reproducibility: verification by recomputation from committed results; no new
runs. Commit 1508a14. Files: web/content/research/lab/experiments/raphael-
anjou/{hint-study/{index,findings,method},dfs-
study/{index,findings,method},going-fast,jit-backtracker,frostline,index,engin
es/index,learning/{index,palimpsest,lodestone,prior}}{,.fr,.es}.mdx.

Verified 21 audit findings and fixed them, rejected 2 as not holding, deferred 10 (new experiments or files outside the group). Source commit 1508a14 on devin/1794390000-rigor-lab-raphael-a, integrated via devin/1794392000-rigor-lab.

## 2026-11-11 — Rigor pass: lab-raphael-b (external review follow-up)

Rigor pass on Raphael's lab notebook (replay, ledger, bandsaw and MITM hub,
beam-width, cas, cloister, fluid-frame, gauntlet, ladder, midden, mosaic,
staged, repair-study findings/index, scaling-ladder; EN/FR/ES, updated
2026-11-11; branch devin/1794390000-rigor-lab-raphael-b, final commit
0a7ebf3). Every claim now matches committed evidence. REPLAY now shows witness
representability, not a plateau cause (the dead-ends equal-wall-clock A/B
ties). FLUID-FRAME's 45 free swaps are derived from the piece multiset. CAS's
empty-interior ALNS arm is a weak baseline, and piece starvation is a
hypothesis. LEDGER is framed as a Max-CSP global counting bound (Freuder &
Wallace 1992, Larrosa & Schiex 2004), with the 1.7x-45.3x ratio limited to one
uncensored board where depth and budget covary. The scaling ladder confounds
size with constrainedness (fixed 22 colours) and the 12 s budget; its N=14
collapse does not reproduce on the kit (DFS medians 0.512/0.477). Beam-width
was retitled; width 32 to 2048 gives about +5%; stochastic/diverse beam and
Beam-ACO precedents cited. Repair-study targeting is confounded with hole
size, and START-DFS has no DFS-only control. BANDSAW was downgraded proven to
measured: corrected first-feasible-budget optimality argument, no generic sqrt
saving, 437 decoupled from the 10x10 certificate. MIDDEN generalises depth-
gated (cell-set) schedules, at +14 to +21 perfect-prefix cells. The GAUNTLET
458 is unhinted, and its scan-order result is evidence at unequal run counts.
MOSAIC's 30 s blocks measure MaxSAT speed, not tractability. CLOISTER's rim
effect and STAGED's frame valuation and mismatch anatomy are unmeasured. rigor
was removed from unbacked pages (REPLAY, MIDDEN, LADDER, MOSAIC, CLOISTER).
Learned: several headline effects were artifacts of unequal samples, covarying
factors, or properties fixed by construction. Open: per-ring CAS stats, LEDGER
fixed-budget sweep and PFC baseline, ladder colour/budget sweep, size-matched
destroy controls, equal-count scan-order control, BANDSAW table sizes.
Reproducibility: text-only corrections against existing results. Hub index.mdx
labels (not owned) still need updating.

Verified 16 audit findings and fixed them, rejected 0 as not holding, deferred 8 (new experiments or files outside the group). Source commit 0a7ebf3 on devin/1794390000-rigor-lab-raphael-b, integrated via devin/1794392000-rigor-lab.

## 2026-11-11 — Rigor pass: lab-others-community (external review follow-up)

2026-11-11. Rigor pass on lab experiments and community pages (group lab-
others-community), commit c5697a3, branch devin/1794390000-rigor-lab-others-
community. Reproducibility: deterministic (new research/experiments/single-
core-benchmark/scripts/contender_stats.py writes results/contender_stats.json
from the committed grid, fixed RNG seed). What changed: the single-core
benchmark's headline 'node count is not score' rested on producer/ALNS rows
that are withheld (eligible=false). On the published rows score follows
throughput, so the thesis is dropped and the question left open. Contender
order: Verhaard-style leads on 8/10 variants, paired mean gap 4.4 (bootstrap
95% CI 0.8-8.5), median gap CI -0.5 to 9, so ranks 1-2 are not separated at
n=10 with one seed. The grid ran 6 jobs at once on a 4P+4E M1, which confounds
throughput with core type. CSP presets never beat naive DFS (best 349 vs naive
worst 359). Three preset groups emitted identical boards on all 10 variants:
rare_color_first is a placeholder equal to border-first MRV;
verhaard_preferred and joe_depth150_bp need inputs the grid never passes; NS-1
changed nothing. Despite its name, border_first_lcv uses insertion value
order. REPORT.md's 'not reproducible from this repo' line was stale; the
backtrackers live in bench-audit, but the just recipe builds only run_algo.
The Verhaard reimplementation was misdescribed as swap-annealing; it is a
comb/slip/quota DFS whose quota tables are byte-exact from eii.exe and whose
slip schedule comes from posts. Its 120 s 438 was never committed, so the page
now uses the committed variant boards (451 best, 440.8 mean). Blackwood's
depth 45 is a pin artifact (the clue-aware reimpl averages 436.4), the partial
248/256 board lost its leaderboard score, and the tuning sweep is 'suggests',
not 'near-optimal'. McGavin: single-core speed rose 2.5-4x in 15 years (not
'barely'); memory-wall and 'prunes harder' claims removed; the 10x10 is one
geometric draw (95% range about 1.8K-258K rows); the msg 11201 pair 3.1e40
nodes / 4.9e32 years is internally inconsistent; 'fastest' is scoped to
single-core CPU. Community: banding is built into Blackwood's depth-201 break
schedule, rigidity means flat plateaus rather than valley bottoms, fleet re-
finds imply overlapping regions, Owen's 17.14 covers interior colours only
(first-moment heuristic, ~14,702 expected), and the hunt-part-2 complexity
link now points to /research/why/complex-theory. Methodology: review requires
a reader other than the author, and non-re-runnable numbers are labelled and
unscored. Learned: metaDescription has a 300-char zod cap that
check:research:ci does not catch. Files: research/experiments/single-core-benc
hmark/{README.md,results/REPORT.md,results/contender_stats.json,scripts/make_r
eport.py,scripts/contender_stats.py}; web/content/research/lab/{index,experime
nts/index,experiments/methodology,experiments/single-core-
benchmark,experiments/single-core-benchmark/csp-presets,experiments/joshua-
blackwood/solver,experiments/louis-verhaard/{eii,verhaard-
reimpl},experiments/peter-mcgavin/{backtracker,index}}.{mdx,fr.mdx,es.mdx};
web/content/research/community/{boards,five-clue-fleet,hunt,hunt-
part-2}.{mdx,fr.mdx,es.mdx}.

Verified 22 audit findings and fixed them, rejected 2 as not holding, deferred 9 (new experiments or files outside the group). Source commit c5697a3 on devin/1794390000-rigor-lab-others-community, integrated via devin/1794392000-rigor-lab.

## 2026-11-11 — Rigor pass: why-a (external review follow-up)

2026-11-11. Rigor pass on ten why/ pages (border-balance, clue-corridors,
complex-theory, constraint-immediacy, design-recipe, entropy-area-law, flux-
invariants, forbidden-patterns, frame-is-not-the-basin, hint-geometry),
EN/FR/ES, after a read-only audit; all 21 findings verified and fixed. New
measurement (seeded, about 1 min on 8 cores): frame-is-not-the-basin --control
completes 16 fully-matched borders x 8 beam seeds with every completion kept.
One-way ANOVA F(15,112)=1.24, p=0.25, ICC about 0.03. Interior Hamming is
184.1 for two seeds on the same border against 190.4 between borders. So the
border effect is undetectable for this producer, and most 'border-driven
diversity' was seed sensitivity; the negative is now scoped to producers
topping out near 440, and 455 is relabelled a threshold rather than the
record. Recomputed exactly: the clue-corridors closest-pair corridor is L=9
(5.6e13 walks, 4.7e13 distinct, a 5e-13 fraction of 9.3e25 unconstrained
placements), and with C=17 the branching table is b1=46.1, b2=2.71, b3=0.16.
The corridor argument now rests on fail-last scheduling, not volume.
Corrections: the border balance is a trivial seam identity (Delta lower-bounds
seam mismatches and is identically 0 under a matching backtracker). The flux
law gives 44 real equations on the phasors (40 was the real rank on real
unknowns); the nonabelian van Kampen condition exists but is impractical; the
endgame certificate's benchmark is artificial and can only fire in about the
last 20 cells. Complex theory already includes use-once via P(n,k); its blind
spots are correlations, variance and planted clustering. 'Five clues give
about 1' is now 'the designed solution plus about 4e-8 others'. Null models
added: 99.72% forbidden 2x2 matches the 4^4/17^4 first-moment estimate; 20
void corner pairs vs 19.2 expected; no symmetric or duplicate pieces happens
about 1 time in 5 under random painting; flat frequencies are clearly non-
random (sd about 9). Owen's 2.08/4.14 ratios are orbit counting (4:2:1).
Constraint-immediacy uses the prefix induced-edge criterion (edge-
isoperimetric; row-major 108 vs 8x8 block 112 at 64 cells), cites fail-first,
and claims only that hint-link was worst under both engines; a FR '3 sur 4'
typo is now 4 of 4. Titles scoped: clue-corridors ('...hurt a greedy filler'),
hint-geometry ('Scattered hints beat contiguous top rows of hints', contact-
count confound and missing controls), flux ('...every solution must satisfy'),
border-balance (identity, rigor proven), entropy (no 'all the hardness'; NP-
completeness cited). Literature added: Haralick-Elliott 1980, Bollobas-Leader
1991, Williams-Hogg 1994, Knuth 1975, Demaine-Demaine 2007, van Emde Boas
1997, Thurston 1990. Learned: metaDescription is capped at 300 characters and
only pnpm typecheck catches it. Deferred: justfile recipe for --control,
ForbiddenCountLab note, hint bottom/centre arms, the flux re-benchmark, the
corridor A/B rerun, the record-border freeze, and PLAN.md sweeps. Files: the
30 MDX files above; research/topics/frame-is-not-the-
basin/{compute/src/main.rs, article.md, results/frame_variance_control.json};
research/topics/clue-corridors/{compute/src/main.rs, article.md,
results/corridor_counts.json}; research/topics/flux-invariants/article.md;
research/index.json. Commit 7dc143f.

Verified 21 audit findings and fixed them, rejected 0 as not holding, deferred 7 (new experiments or files outside the group). Source commit 7dc143f on devin/1794390000-rigor-why-a, integrated via devin/1794392000-rigor-why.

## 2026-11-11 — Rigor pass: why-b (external review follow-up)

Rigor pass on the why/ group (no-forced-moves, piece-theft, phase-transition,
how-hard-is-this-instance, irreducible-hard-region, mismatch-geometry, no-
height-function, index, parity-defect-floor, permutation-code-wall; EN/FR/ES),
commit 85b7de4 on devin/1794390000-rigor-why-b. New reproducible null models:
research/topics/no-forced-moves/compute/src/bin/null_model.rs and
research/topics/piece-theft/compute/src/bin/null_model.rs run the engine's
generate_framed(16, 22, seed, true) for seeds 1-100. Results are in
results/null-model.json; reproduce with `cargo run --release --bin null_model`
(exact; default-run added so the existing just recipes are unchanged).
Learned: E2's single-edge partner counts (median 125; random 120-128) and
(north, west) demand scarcity (269/47/2.90; random 261-282 / 38-63 /
2.77-3.00) are typical of any 5+17 16x16 puzzle, not design signatures. The
'no forced moves' claim holds only for single edges: with two neighbours
fixed, about 3 candidates remain and 17.5% of demands have one server. Piece
theft is a standard forward-checking domain wipe-out. The 8x8 hardness sweep
shows a noisy band at 6-8 interior colours, and the DFS also fails at 1-3 and
5. The 16x16 sweep locates no peak. Owen's 2xL strip count tracks the first-
moment threshold (crossing at L 19-20) and measures satisfiability, not
effort. The n13 cliff boards are 17+5 (22 total), not 22 interior; the
original round was 26 total (21 interior). The CP-SAT 11-to-12 slowdown is
about 20x at the median, not two orders of magnitude. The crude first-moment
crossing (N 12-13 at 17 interior colours) matches CP-SAT's wall, so paradigm
and instance position cannot be separated. Row-major localisation and the
0.503 random control are true by construction. Mismatch forests are expected
by chance (under 0.2% loop probability at 11-25 defects). The no-height parity
is automatic. Pages were retitled to match what they measure, record numbers
were replaced by /research/records links, and the solution-count wording rule
was applied. The phase-transition page now cites the random-CSP threshold
literature. Deferred: committing the 1,964-board mismatch corpus, the
ForcedMovesLab string, just recipes for the null models, the n13 article
frontmatter title, a multi-order hard-region test, and an invariant
classification. check:research:ci and typecheck pass. Files:
web/content/research/why/{no-forced-moves,piece-theft,phase-transition,how-
hard-is-this-instance,irreducible-hard-region,mismatch-geometry,no-height-
function,index,parity-defect-floor,permutation-code-wall}{,.fr,.es}.mdx;
research/topics/no-forced-
moves/compute/{Cargo.toml,README.md,src/bin/null_model.rs}, results/null-
model.json; research/topics/piece-
theft/compute/{Cargo.toml,README.md,src/bin/null_model.rs}, results/null-
model.json; research/topics/n13-encoding-cliff/article.md (body only).

Verified 18 audit findings and fixed them, rejected 2 as not holding, deferred 6 (new experiments or files outside the group). Source commit 85b7de4 on devin/1794390000-rigor-why-b, integrated via devin/1794392000-rigor-why.

## 2026-11-11 — Rigor pass: why-c (external review follow-up)

2026-11-11, rigor pass on why/ group C (branch devin/1794390000-rigor-why-c,
commit c8b2d22). Nine pages edited in EN/FR/ES: the-470-wall, prune-vs-speed,
theorem-sweep, ring-purity, rare-color-geography, rigidity-wall, why-e2-is-
hard, walls-and-methods, sigma-cycles. Also PruneVsSpeedLab.tsx (caption only)
and topic files for the-470-wall, prune-vs-speed, ring-purity and adversarial-
piece-set.

Why: an audit found these pages claimed more than their evidence measured.

- The 470 'phase boundary' was not in the landscape.

- A naive solution count of ~19 contradicted solution-counting.

- A prune-beats-speed headline contradicted its own committed data.

- A 'proven' theorem sweep was mostly unpublished claims.

- Design 'extremes' had no null model.

What changed:

- the-470-wall: probe scores extended to 465/475/478 and landscape.json rerun.
The curve falls smoothly (2.5 to 3 decades per point), crosses 1 at 480, has
no feature at 470, and predicts ~10^18 boards at 475. The page is retitled as
a conjecture, and the small-instance enumeration is marked unpublished.

- prune-vs-speed: now reports its measured comparison (at depth 36, b_eff
1.416, a 1000x speedup beats a 5% cut, worth only 6.3x) and separates
exhaustive search from record hunting.

- theorem-sweep: rigor set to conjectured, and each family tagged
committed/measured/conjectured/claimed. The PLS and proof-complexity
statements are corrected (CDCL p-simulates general resolution; BIW 2004;
Pipatsrisawat-Darwiche 2011).

- ring-purity: the ring is a directed multigraph (in=out=12). New
compute/ring_count.py gives the exact BEST count, 3432 x (11!)^5 ~ 3.5x10^41
circuits. Sampling 16M uniform circuits gives a corner-spacing fraction of
2.78e-5, so ~10^37.6 valid frame placements (the naive estimate is 10^34.3).
Kind: exact plus seeded sampling, ~90 s on 8 cores.

- adversarial-piece-set: new compute/null_model.py paints 20,000 seeded solved
boards with the official joint-colour counts.

  - Zero symmetric pieces is ordinary (54%).

  - 5 twins is ordinary (mean 6.6).

  - Zero rotation-duplicate pairs is rare (1.4%, mean 4.1); this is the only
design fingerprint that survives.

  - Even colour counts and zero slack are automatic.

- rigidity-wall: the null for border UNSAT is that a random border hosts a 480
with p < 10^-30, so 9/9 UNSAT is expected. Row uniqueness is at the null
expectation. A selection-bias caveat was added, and the 0.18 per-cell figure
corrected to 0.24 and the 22-colour figure to 17 colours.

- rare-color-geography: reasoning reversed (the 5-colour frame is the loose
subsystem), and 'edges' corrected to half-edges.

- walls-and-methods: speed has moved community records; the hardness peak is
retiered.

- sigma-cycles: the hot chains are at effectively infinite temperature.

Learned:

- The design story needs nulls. Most 'adversarial' piece-set properties are
what a random painting gives; zero rotation duplicates is the exception.

- The landscape gives no reason to expect a wall at 470.

- The research schema caps metaDescription at 300 chars, and only `pnpm
typecheck` catches it.

Open: rerun research/build-index.mjs (index.json not committed), and reconcile
the five-clue count (4e-8 vs ~1) between solution-counting and known-facts.

Verified 20 audit findings and fixed them, rejected 2 as not holding, deferred 7 (new experiments or files outside the group). Source commit c8b2d22 on devin/1794390000-rigor-why-c, integrated via devin/1794392000-rigor-why.

## 2026-11-11 — Rigor pass: build-exact (external review follow-up)

Rigor pass on the Exact-methods pages (build/exact: exact-tail-endgame, lp-
relaxations, meet-in-the-middle, sat-csp-encodings, exact-cover-dlx, iterated-
maps; EN/FR/ES), commit 706f82d on devin/1794390000-rigor-build-exact. Outcome
and learning: (1) exact-tail-endgame: the audit held. The greedy producer in
tailforge.py maximises incoming matches and accepts double breaks: every one
of its 24 one-row tails holds 1 to 6 double-break cells. The old 'cap=2'
constraint was a no-op, so the 'load-bearing double break' story was misread.
tailforge.py now runs three regimes (cap=1; at most one double-break cell in
the tail, a real global budget; uncapped), hints the producer tail, floors the
objective at it whenever it is admissible, and uses statistics.median. Rerun
(seeded, about 10 min on 8 cores): at one row, uncapped beats the producer on
24/24 seeds by 1 to 9 edges (median 4.5, mean 4.75); one double break gives
the uncapped optimum on all 24, while the producer used up to 6; cap=1 is
infeasible on 5 and gives 0 to 8 (median 4) on 19. At two rows the 45 s
incumbents (lower bounds) give medians 12.5 / 12 / 11.5, and uncapped still
reads 1 edge below capped on 3 seeds, which shows the incumbents are unproven.
The page no longer claims to separate tail from top: there is no local-search
control and no top-tail exchange. article.md is filled. (2) lp-relaxations:
480 is attained, so no sound unconditional bound below 480 can exist. The old
'conjecture' is the LP>=IP necessary condition, and it certifies that every
measured 477 to 479 border admits no perfect completion. The 420 to 440
'plateau' is one 2009 penalty-solver stall, not an integrality band. The seam-
type split is one LP vertex. 'Integer-locked' became the actual local-search
evidence. Conditional-LP numbers are labelled uncommitted. (3) meet-in-the-
middle: recomputed with the page's own T(a,k): log10 T = 3.0, 16.3, 41.4, 61.5
at a = 1, 16, 64, 128, peaking near 66 at a≈160. MITM saves about 4 orders on
the dominant term, not 'never' and not a square root, because partial counts
are bulge-shaped. Interior parameters (196 pieces, p≈0.0588) are computed from
the piece file. The model numbers are labelled uncommitted. (4) sat-csp: the
seam trick gives a ~400x clause reduction. The 'size ceiling near 10x10' is
replaced by the colour/formulation dependence measured on planted boards.
Cube-and-conquer is noted as untried at scale (checked 2026-11). The CDCL-
stall section is now a hypothesis. (5) exact-cover-dlx: the 10^100 figure is
replaced by the fill-order 10^40 to 10^43 tree. TAOCP 4B XCC and min-cost
covers are cited. (6) iterated-maps: corrected the one-hot P_2 vs replica
embedding and added nonconvex DR local-convergence literature (Hesse & Luke
2013, Phan 2016). Milne and JSA are now presented as opinion. Reproducibility:
tail sweep seeded (`just research-exact-tail-endgame`); other changes are
prose. Files: web/content/research/build/exact/{exact-tail-endgame,lp-
relaxations,meet-in-the-middle,sat-csp-encodings,exact-cover-dlx,iterated-
maps}{,.fr,.es}.mdx, research/topics/exact-tail-endgame/{article.md,compute/ta
ilforge.py,results/tailforge.json,results/run.log}, justfile (research-exact-
tail-endgame recipe caps). research/index.json still needs a regenerate to
pick up the filled summary.

Verified 22 audit findings and fixed them, rejected 0 as not holding, deferred 7 (new experiments or files outside the group). Source commit 706f82d on devin/1794390000-rigor-build-exact, integrated via devin/1794392000-rigor-build.

## 2026-11-11 — Rigor pass: build-reduce-backtracking (external review follow-up)

2026-11-11. Rigor pass on the build/reduce and build/backtracking pages
(alldiff-regin, arc-consistency, edge-slipping, nogood-learning, both index
pages, fill-order, restarts; EN/FR/ES), plus a new reproducible check at
research/experiments/fill-order-mean-field (repro kind: seeded; `python3
research/experiments/fill-order-mean-field/mean_field.py`, about 3 min on one
core). What changed and why. Restarts is no longer claimed for every record
solver: only Blackwood's engine is a cutoff-and-reshuffle portfolio, McGavin's
10x10 was a ranked exhaustive first-row partition, and eii documents no
restart policy. Blackwood's 1B to 50B cutoff insensitivity no longer counts as
heavy-tail evidence, since a memoryless runtime is just as flat. The 2007 race
covered 500 different 7x7 instances, so it shows inter-instance spread, not
seed-to-seed tails. The heavy-tail literature is now cited as CP-97 and JAR
2000 (with Crato). Edge slipping: the 470 came from dropping the eleventh
break (11 to 10) with the 469-era unlock depths unchanged, not from retuned
depths. Max's 63x per edge is a target-count ratio; JSA's observed per-edge
rarity was about 17 to 22x. The 467-to-470 target-class ratio is about 3x10^5,
with its effect on difficulty unmeasured. Arc consistency: d is exactly 784,
e*d = 752,640, and colour-equality constraints admit O(e*d) AC via per-colour
support counters. The committed DFS-study ablation shows forward checking,
AC-3 and AC-3 plus the colour check are indistinguishable at 60 s
(322.2/320.6/320.6), so the 'mandatory, thrashing vs 440s' framing is gone.
Hard AC and the exact colour filter are unsound under breaks, and soft AC /
soft global constraints are named as the sound alternative. No-good learning:
tables are probed once per state at a test line, so the saving is capped near
the ~1% exact-duplicate rate. Entries are ~42 B (about 1.6e9 in 64 GiB), the
probe cost is latency, and the remaining-piece set rather than frontier width
dominates the key space. Named literature added: dynamic backtracking, CBJ,
nogood recording, LCG, MUS extraction, component caching. Fill order: the
mean-field law recomputed with the interior-colour p = 0.0589 predicts
46/2.7/0.16/0.0094 against exact 45.0/2.58/0.148/0.0082. On a bordered 14x14,
row scan and the inward spiral have identical k-histograms (169/26/1), so the
'closure tax' explanation was withdrawn. A conditions table now separates the
tree-size figures (1.07e50 no starter, ~1.36e47 with the starter, 1e45
plateau, 1e43 peak, Max's 2.3e40 sampled tree). Border-first is scoped to
exhaustive search. The comb's 15-vs-9 of 24 is not significant (p~0.31). What
was learned: several headline claims came from mixing conditions (starter vs
none, total vs per-depth, target count vs difficulty) or from one engine's
anecdote. Open items: E2 run-length distribution, comb per-seed statistics,
reconciling the 1e43 and 1e45 peaks, a soft-AC bound, and a Régin-vs-counting
prune log. Also: metaDescription is capped at 300 characters and only `pnpm
typecheck` catches it. Files: web/content/research/build/reduce/{alldiff-
regin,arc-consistency,edge-slipping,index,nogood-learning}{,.fr,.es}.mdx;
web/content/research/build/backtracking/{fill-
order,index,restarts}{,.fr,.es}.mdx; research/experiments/fill-order-mean-
field/{mean_field.py,README.md,results/mean-field.json}. Commit 9b944a9.

Verified 26 audit findings and fixed them, rejected 2 as not holding, deferred 6 (new experiments or files outside the group). Source commit 9b944a9 on devin/1794390000-rigor-build-reduce-backtracking, integrated via devin/1794392000-rigor-build.

## 2026-11-11 — Rigor pass: build-localsearch-construct (external review follow-up)

2026-11-11. Rigour pass on build/local-search (index, local-search-alns,
parallel-tempering, evolutionary) and build/construct (beam-search, band-
column-dp), EN/FR/ES, following an audit of overclaims. ALNS: the rigidity
wall is now described as a halo-local (radius 1 to 4) MIP/SAT certificate on
the tested boards, not a global optimality proof. Budget and temperature
claims follow the committed repair study (millions of iterations per minute,
last improvement after thousands to hundreds of thousands, annealing
acceptance about 16 points over a strict hill-climb), and the Hungarian-refill
citation is split between Schaus (msg 5601) and antminder (msg 5589). Parallel
tempering: polynomial rung diffusion is stated as conditional, with torpid
mixing on entropic or first-order bottlenecks (Woodard, Schmidler & Huber
2009). The top-board plateau is explained as an entropic barrier (neutral
moves are T-independent, exits are not) and corroborated by benj39100's
4096-replica GPU run. Verhaard's 467 is attributed to eii's edge-slipping
backtracker rather than annealing. Local-search index: basin trapping is now
per method instead of a shared height. Evolutionary: pure GAs (~400 to 410)
are separated from the 462 to 463 hybrid, and the crossover verdict is limited
to position-wise crossover with the measured 56-hybrid test. Munoz, Gutierrez
& Sanchis 2009 (IWANN, CEC) and memetic references (Galinier & Hao 1999,
Benlic & Hao 2015) are added. Beam: the DFS/LDS comparison is scoped to an
untuned baseline at ~2e7 nodes, 'ceiling' becomes best observed (455/456/457),
committed SMC resampling results are cited with both beds kept separate, and
the tail finisher is identified as restricted. Band DP: loss described as
consistent with greedy-induced piece-supply failure (exact bottom solve not
run), cost O(n*K*c) with a timing-provenance caveat, seam estimate
(1/17)^14(1/5)^2 ~ 2e-19 before disjoint-piece constraints, and beam named as
dominant. Learned: several headline claims (shared rigidity height, annealing
record, 'always') rested on n-of-1 notebook runs or misattribution; committed
results (repair-study, beam-width-smc) contradicted or narrowed them.
Deferred: approaches-map Verhaard line, beam-width-smc summary inconsistency,
committed PT and band-DP reruns. Repro: prose (no new computation). Files:
web/content/research/build/local-search/{index,local-search-alns,parallel-
tempering,evolutionary}{,.fr,.es}.mdx,
web/content/research/build/construct/{beam-search,band-column-
dp}{,.fr,.es}.mdx. Commit f2af1db.

Verified 18 audit findings and fixed them, rejected 2 as not holding, deferred 5 (new experiments or files outside the group). Source commit f2af1db on devin/1794390000-rigor-build-localsearch-construct, integrated via devin/1794392000-rigor-build.

## 2026-11-11 — Rigor pass: build-learning-analysis (external review follow-up)

2026-11-11. Rigor pass on build/learning (6 pages) and build/analysis (3
pages), EN/FR/ES, commit 6751d70 on devin/1794390000-rigor-build-learning-
analysis. Solution counting: 14,702 is now presented as a first-moment
estimate over random puzzles with E2's statistics, not a count. New, verified
result (prose repro, Python on engine/data/official_eternity2.csv):
4!·56!·195!·4^195 × (12!)^5/60! × ∏(n_c−1)!!/839!! = 14,702.3. Here the frame
ring is directed with 12/12 half-edges per frame colour, and the interior
counts are 48×5 and 50×12. This exactly reproduces the canonical figure.
Freeing the ring direction gives about 22, the naive i.i.d. product gives
0.0209, and naive×2^20 parity gives about 21,940. The estimate is therefore
model-dependent by orders of magnitude. Also: 9x9 calibration (2 and 3 against
3.2) has negligible power; the planted bias is E[Z^2]/E[Z] (Krzakala &
Zdeborová 2009), not yet computed; Owen's four culled runs bound variance, not
bias (Knuth 1975, Purdom 1978, Chen 1992). Parity: balanced-set stalls are
explained by counting (about 3e147 sets against 1e4 to 1e7 solutions), and the
unsourced '10-28%' NS-1 figure was removed. Learning: the 460-463 plateau of
corpus-learned signals is reframed as a limit of these exploratory runs
(community boards exceed it), not a puzzle wall. The 'tiebreak only' rule is
narrowed to LODESTONE's 3-point fixed-weight sweep; KEYRING's weights are
unrecorded and its new family is not ablated against injected randomness. The
PALIMPSEST trap map has no permutation baseline and its 463 no random-wedge
control. REPLAY is a representability check, not a free search. Added the
elite-solution literature (AMP, path relinking, backbone-guided LS, CEM/EDA)
and noted that no experiment re-learned from its own outputs. Learned:
metaDescription is zod-capped at 300 characters and only pnpm typecheck
catches it. Deferred: rigor labels and matching wording on the lab experiment
pages, known-facts 'single expected solution by design', phase-transition
'about one solution'. Files: web/content/research/build/learning/{index,anti-
pattern-mining,corpus-priors,decoding-records,learned-value-ordering,when-
learning-collapses}{,.fr,.es}.mdx;
web/content/research/build/analysis/{index,parity-arguments,solution-
counting}{,.fr,.es}.mdx.

Verified 16 audit findings and fixed them, rejected 0 as not holding, deferred 7 (new experiments or files outside the group). Source commit 6751d70 on devin/1794390000-rigor-build-learning-analysis, integrated via devin/1794392000-rigor-build.

## 2026-11-11 — Rigor pass: build-faster-solvers-hardware (external review follow-up)

Rigor pass on build/faster (distributed-solving, index, solver-engineering),
build/solvers/index and build/hardware/quantum, in EN, FR and ES (15 files,
updated: 2026-11-11, commit c3a7c1c). The pages now separate exact exhaustive
search from stochastic record hunting. A constant factor or a fleet cannot
make the full tree feasible. More throughput does buy record attempts:
eternity2.net's 463 ceiling, eii's dozens of 467s, and the 469 on about 200
cores, where the one-core counterfactual was never tested. The 10×10 ranked-
row arrival (92,907 vs one per 70,000) is now treated as a single draw, with a
27% chance of a wait that long under a geometric model; the node-histogram
agreement is the real validation. Arithmetic fixes: 1e19 ops is about
1e17–1e18 nodes, or 1e-26 to 1e-25 of 2.2e43 nodes per solution, which is
about 1e28 CPU-years at 1e8 nodes/s. The M1 4→8 thread totals are 208M vs 176M
per second, 15% lower, cause not isolated. The +25% speed result is one fixed
budget point and needs a budget sweep. The memory-wall line in solver-
engineering now matches the page's own anatomy: loads are cache-served, and
pinning the residual cost on branch misprediction is an inference because PMU
branch-miss counters were not read. The solvers hub now follows fill-order
(row scan best on the 5/17 split; border-first about 1e57 vs scan-row about
1e43, winning only on tipped designs). Its reach table separates unit, clue
regime and provenance, with eii's 467 marked as a community fleet result, not
rerun. The quantum page prices backtracking on T' (Ambainis–Kokainis 2017,
Jarret–Wan 2018, Montanaro's n^{3/2} overhead): sqrt(2.2e43) × 256^1.5 ≈ 1e25
steps. The 'one needle, no gradient' line is replaced: the landscape descends
easily into the mid-460s, and the last ten or so edges are open. Exponential-
gap claims are cited for specific random ensembles only
(Altshuler–Krovi–Roland), and E2's QUBO is untested. Lessons: metaDescription
has a zod limit of 300 characters, and only typecheck catches it
(check:research:ci does not). Several metaDescriptions on main still contain
em dashes because the style gate skips frontmatter. Deferred: an owned-vs-
rented $/placement comparison (no owned cost data), and cross-links from the
GPU, FPGA and hardware-hub pages (another PR). Files:
web/content/research/build/faster/{distributed-solving,index,solver-
engineering}{,.fr,.es}.mdx,
web/content/research/build/solvers/index{,.fr,.es}.mdx,
web/content/research/build/hardware/quantum{,.fr,.es}.mdx.

Verified 12 audit findings and fixed them, rejected 0 as not holding, deferred 4 (new experiments or files outside the group). Source commit c3a7c1c on devin/1794390000-rigor-build-faster-solvers-hardware, integrated via devin/1794392000-rigor-build.

## 2026-11-11 — Rigor pass: build-top-a (external review follow-up)

Rigor pass on six build/ page families (approaches-map, benchmarks, clue-
puzzles, dataset, dead-ends, formats; EN/FR/ES, updated 2026-11-11), commit
659a6bd on devin/1794390000-rigor-build-top-a. All 16 audit findings were
checked against the repo data and held. approaches-map no longer says every
family stops at the same height: only depth-first backtracking with scheduled
late mismatches has reached 467-470, while ILP (420-440), the 461 hyper-
heuristic (moved from the LP bullet to local search) and the project's 460-463
engines plateau lower. The rigidity wall is now a hypothesis measured on
specific record boards. The Verhaard 'annealing record' claim is replaced with
backtracker plus a 438 subset-annealing reimplementation, and 14,702 is framed
as a first-moment ensemble estimate. benchmarks: the unsourced 400x became
about 4x per core and about 150x for a GPU against a 2008 core, with the
record +3 edges over the same period. The full-board/small-board ratio is a
third, not half. The 10x10 set_1 solve is a consistency check (p~0.27, rate
bounded only within ~150x), not strong validation, and the 69,284,103-node
figure is tied to the Txibilis row-scan convention. dataset: the ten v00-v09
instances are the five-clue puzzle plus three pinned corners. Equal hardness
is unmeasured, at most one arrangement is expected to admit a 480, and
maxScore is an upper bound. A new deterministic script
research/datasets/build/corpus_diversity.py (result
research/datasets/corpus/diversity.json, ~45 s) shows the 7,658-board corpus
is 86% one 451-455 beam harvest. 90.6% place all five clues but none at
460-463 or 466-470 do, so the strict best in the corpus is 464. Placement
distance has a near-256 random null. Shared adjacency (null ~7/480) gives NN
median 41, with 12.6% of boards having a sibling that shares >=240. dead-ends:
pattern databases are 'Measured (root only)' (the root bound of 480 is
trivial; interior-node patch DBs are untested; colours are 5 frame + 17
interior). Survey propagation is retagged measured plus heuristic. The tensor
entry is compared to 14,702 / ~1, with the unbacked 10^90 removed (the
archive's 10^80-10^90 is a tree-width product, a different quantity). The LP
478 gap now links /research/records. Neural, cube-and-conquer and Sinkhorn
verdicts are scoped to the runs made, with Mena et al. 2018 cited. The
inconsistent ant-colony 0.9% / ~600 numbers are dropped. clue-puzzles: Clues
3-4 are probable, not certain (single source, and solvability only catches
parity-breaking errors); matching apal1969's 2,195,647,488 is the open
fingerprint test. formats: hint and JSON examples regenerated from
e2-16x16-v00.json (34.1-45.1-135.0-210.1-221.2; pos 34 = piece 207 rot 1).
Learned: a root-node admissible bound on a solvable instance is always maximal
and proves nothing. Placement Hamming distance is uninformative without a
shift-insensitive null. Deferred: parallel-tempering.mdx Verhaard wording,
analysis/index.mdx expected-count wording, the approaches-map GPU bullet
(other PR), a maxScore schema rename, the ant-colony chance-rate recompute and
the Clue 3 exhaustive count. Reproducibility: page edits are deterministic
text; the corpus statistics script is deterministic (seed 1). Files:
web/content/research/build/{approaches-map,benchmarks,clue-
puzzles,dataset,dead-ends,formats}{,.fr,.es}.mdx,
research/datasets/build/corpus_diversity.py,
research/datasets/corpus/diversity.json.

Verified 18 audit findings and fixed them, rejected 0 as not holding, deferred 6 (new experiments or files outside the group). Source commit 659a6bd on devin/1794390000-rigor-build-top-a, integrated via devin/1794392000-rigor-build.

## 2026-11-11 — Rigor pass: build-top-b (external review follow-up)

Rigor pass on the build hub pages (build/index, known-facts, toolkit,
variants, run-it-yourself; EN/FR/ES), commit 456c6a8 on
devin/1794390000-rigor-build-top-b. Kind: prose fixes plus one cheap
deterministic measurement. known-facts: the five-clue expected-solution figure
is now ≈ 4×10⁻⁸ (McGavin msg 11193), consistent with solution-counting and
clue-puzzles, instead of '≈ 1, by design'. Both estimates are labelled random-
ensemble averages. The 17 interior colours are described as Owen's 17.14 one-
expected-solution heuristic (msg 1947), near the hardness band rather than
'exactly the peak'. The 469 row now matches community/boards (Bucas 469a–g
plus Fernandez, Oct–Nov 2020). The symmetry argument was corrected: I8 is
fixed by the anti-diagonal reflection, but reflections are not puzzle
symmetries because 213/256 mirrored pieces are absent from the official set
(computed). Colour parity and zero slack are flagged as automatic for any
painted puzzle. toolkit: the old 'sd 12–20' claim had no committed source. The
kit's default sweep (greedy, 40 generated framed 16×16 boards, seeds 1–40,
deterministic, seconds) measures mean 292.7, sd 14.8, so the figure holds for
a weak baseline across instances. Committed beam runs show 2–3 (beam-width-
smc, cas_plateau). The page now says the spread depends on the solver,
describes compare's paired-SE verdict, and warns that short-budget means are a
proxy for long-run record behaviour. variants: the unframed variant is easier
because it is far less constrained, not because difficulty lives in the frame
(consistent with frame-is-not-the-basin). The 'proven' radius-6 dead end had
no source and was removed. The top-band holes on two related boards are now an
untested observation, with a reversed-fill-order run named as the test. index
and run-it-yourself: 'provably' dead ends became tiered verdicts, and 'every
result is reproducible' is scoped to computed results (32/86 EN pages with a
repro field are prose). Learned: metaDescription ≤ 300 chars is enforced only
by typecheck, not check:research:ci. mismatch-geometry, complex-theory's table
and the starter-kit README/PROMPT need matching follow-ups. Files:
web/content/research/build/{index,known-facts,toolkit,variants,run-it-
yourself}{,.fr,.es}.mdx.

Verified 11 audit findings and fixed them, rejected 1 as not holding, deferred 4 (new experiments or files outside the group). Source commit 456c6a8 on devin/1794390000-rigor-build-top-b, integrated via devin/1794392000-rigor-build.

## 2026-11-11 · Translated pages render their own reproduction and hardware prose

A rendered-browser sweep of the live site (36 URLs, EN/FR/ES) showed English
"Reproduce" scope paragraphs on the FR and ES fill-order, ring-purity and
flux-invariants pages, although every sidecar carries a translated
`repro.scope`. Cause: `buildManifest` in web/content.config.ts copied `repro`
and `stages` from the English frontmatter for every language. It now keeps the
English machine fields of `repro` (kind, cmd, topic, produces) and takes
`scope` from the translation, and takes `stages[]` from the translation (the
parity check already pins engine/published to EN). The parity check now treats
`hardware.machine`, `wallClock` and `seedPolicy` as prose, as the manifest
already assumed, and those strings are translated on FR/ES single-core-benchmark
and jit-backtracker. Learned: the parity check validated translated prose that
the manifest then discarded, so a check passing is not evidence that a field
renders. Reproducibility: prose (site rendering only, no research numbers
changed). Source commit 3d8ca74. Files: web/content.config.ts,
web/scripts/check-research-parity.mjs,
web/content/research/lab/experiments/{single-core-benchmark,raphael-anjou/jit-backtracker}.{fr,es}.mdx.

## 2026-11-11 · Research program: explored map, agenda, lead procedure

Added `research/program/EXPLORED.md`, a compressed map of what has been
searched heavily (families, status, where each stopped, page), built from the
approaches map, dead ends, walls-and-methods, open problems and the lab
experiments. Added `research/program/AGENDA.md` with seven proposed directions
outside that map (spectral planted-solution recovery, extreme-value score
curves, SAT backdoors, quality-diversity over basins, exact frame count with
corners, colour-chain repair neighbourhoods, set_2 pricing), each with a
baseline, a kill criterion and a budget. Added `.claude/commands/research-lead.md`,
the round procedure for a lead session, and an AGENTS.md pointer. No results;
reproducibility kind: prose. Files: research/program/{EXPLORED,AGENDA}.md,
.claude/commands/research-lead.md, AGENTS.md, research/LEDGER.md.

## 2026-11-11 · Research program: GPU agenda item

Added agenda item A8 (a measured CUDA port of the break-tolerant DFS against
the single-core benchmark engine at equal convention and wall clock, kill
below 10x one core) and moved the GPU backtracker out of the parked list.
The lead procedure now routes GPU work to AWS g4dn / g5 instances after
correctness checks on the VM, and checks the G and VT vCPU quotas at the start
of each round. No results; reproducibility kind: prose. Files:
research/program/AGENDA.md, .claude/commands/research-lead.md,
research/LEDGER.md.

## 2026-11-11 · Round 1, A1 spectral planted recovery: killed

Preregistered cheap test of agenda A1 on generated planted boards, judged by a
separate session (rigor-check, adversarial review, recomputation from
`results/`, rerun of the primary configurations). The spectral pair ranking
places the planted neighbour lower than plain colour-rarity ranking: paired
difference -0.00472 (SE 0.00059, z -7.94) at 8x8 c=8 and -0.00349 (SE 0.00059,
z -5.91) at 10x10, 200 puzzles each; rescrambling leakage below 1.4e-15.
Learned: global pair statistics of the piece set carry no neighbour signal
beyond colour rarity on these boards, which weakens the "read the solution off
statistics" family. Reproducibility: deterministic, `just experiments
spectral-planted-recovery`. Source commit bc5353d (PR #14). Files:
research/experiments/spectral-planted-recovery/.

## 2026-11-11 · Round 1, A3 SAT backdoors: kept

Preregistered cheap test of agenda A3, judged by a separate session. On planted
generated boards, the number of correct placements after which propagation
finishes the board grows as N^b with b = 0.589 in regime A (bootstrap 95% CI
[0.549, 0.626]; kill line 0.9), difference statistic D = -0.039 (CI [-0.045,
-0.034]). Limits: at 8x8 and above k is a greedy upper bound, not a proven
minimum; boards are generated regimes, not the E2 piece set; the backdoors
were found with the planted solution in hand. Learned: on these boards the
hardness concentrates in a sublinear set of decisions; whether that set can be
recognised without the solution is agenda A16. Reproducibility: deterministic,
`just experiments sat-backdoors`. Source commit 1705965 (PR #15). Files:
research/experiments/sat-backdoors/.

## 2026-11-11 · Round 1, A4 quality-diversity over basins: killed

Preregistered cheap test of agenda A4 (no amendments), judged by a separate
session that recomputed the Wilcoxon tests and basin counts from `results/` and
reran a declared subset of cells byte for byte. At equal move budgets,
MAP-Elites over basin descriptors found fewer distinct near-best basins than
parallel tempering on 58 of 60 puzzles (2 ties, 0 wins) and reached worse best
scores. Mean distinct basins: 8x8 PT 22.33 vs ME 5.63; 10x10 PT 5.30 vs ME
1.00. Learned: an explicit diversity archive does not add basin diversity over
PT on these boards, so diversity is not shown to be the missing ingredient of
local search. Reproducibility: stochastic with fixed seeds, `just experiments
quality-diversity-basins` (full grid takes hours). Source commit 3f96c0f
(PR #16). Files: research/experiments/quality-diversity-basins/.

## 2026-11-11 · Round 1, A5 exact frame count with corners: kept

Preregistered cheap test of agenda A5, judged by a separate session that also
wrote its own brute-force checker. Transfer-matrix count of frame placements
with the corner constraint: N = 40519056449429419697492635191544381440
(log10 N = 37.6077). The fraction f = N_cyc / EC = 2.913e-5 (N_cyc = N/4,
EC the BEST-theorem Euler-circuit count) is the quantity the existing Monte
Carlo estimate measured, 2.775e-5 (SE 1.32e-6, z -1.04). Validation:
exact agreement with brute force on all 34 reduced rings where it finishes and
with an independent method on all 45. Limit: the official-ring number comes
from one implementation; an independent cross-check on the full ring is open.
Peak RSS about 17.5 GiB. Reproducibility: deterministic, `just experiments
frame-count-corners`. Source commit 8b8c8b5 (PR #17). Files:
research/experiments/frame-count-corners/.

## 2026-11-11 · Round 1 idea pass: eight agenda entries, two parked

Ten sessions, one outside field each (coding theory, compilers, operations
research, computational biology, tiling and words, spin glasses,
cryptanalysis, game-tree search, program synthesis, auctions and matching),
each returned one idea in agenda format; each was filtered against
EXPLORED.md by naming the mechanism its nearest explored cell lacks. Added as
proposed or running: A9 Murty k-best checkerboard assignments, A10
near-codeword spectrum, A11 coupled same-temperature replicas, A12 ProbCut,
A13 synteny decomposition of local optima, A14 local branching, A15
bond-weighted non-chronological unassignment (kTAM and iterative modulo
scheduling merged, same mechanism). Added from round-1 results: A16
backdoor-guided branching (from A3) and A17 cut-multiset feasibility pruning
(from A5's transfer-matrix view). Parked: distinguished-point collision
detection (premise not supported by A4) and abstraction refinement on piece
distinctness (closest to the fake-480 dead end). Reproducibility: prose.
Files: research/program/AGENDA.md, research/LEDGER.md.

## 2026-11-11 · Round 2, hard-instance ladder: infrastructure valid, own kill fired

Preregistered shared harness for round 2 (node-counted DFS on generated 6x6
to 10x10 boards, colours by Owen's one-expected-solution criterion: 5.22,
6.38, 7.56, 8.74, 9.93 interior colours), judged by a separate session. The
preregistered usability condition (best baseline solves at least 15/30 at
8x8) failed: 0/30 at cap 1e6 nodes, and 1/30 (c=8) / 0/30 (c=7) at the
original 1e7 cap in the judge's rerun. Usable rungs: 6x6 c=5 (best baseline
30/30, KM median 951 nodes, 44x fewer than plain DFS) and 7x7 c=6 (16/30).
Learning: on this generator the counts at or just below the critical point
are the hardest; an 8x8 rung that separates pruning methods needs a colour
count between 8 and 10. 360/360 committed rows reproduce node for node.
Reproducibility: deterministic, `just experiments hard-ladder`. Source commit
521c1f4 (PR #19). Files: research/experiments/hard-ladder/.

## 2026-11-11 · Round 2, A12 ProbCut statistical cutoffs: killed

Preregistered test on the ladder, judged by a separate session. Own stage-2
kill fired (R8 first pass 13/30, line 15). At 8x8 c=10 neither variant gains
on nodes against `scb` or `fc-dfs`; cuts removed planted-solution branches
(V1 13/30, V2 17/30 vs 19/30) and cost nodes at 6x6 c=5. The stage-1 AUC null
is structural (probes shorter than one row), so it is not read as further
evidence for the propagation-signal wall. Clean rerun reproduced every JSONL
row. Reproducibility: deterministic, `just experiments probcut`. Source
commit aa4216f (PR #20). Files: research/experiments/probcut/.

## 2026-11-11 · Round 2, A17 cut-multiset (Hall matching) pruning: kept, small gain

Preregistered test on the ladder, judged by a separate session. Novelty check
reduced the idea to Hall matching feasibility (count/parity already in
`gacolor`; ring cuts measured at zero). Paired G1, Holm over 2 variants:
6x6 c=5 GAIN (node ratio 0.900, p 0.0002; 0.876, p 0.018); 7x7 c=6 and 8x8
c=10 NO-GAIN. Wall clock neutral to slightly worse. Side result: at 8x8 c=10
plain row-major DFS matches the best propagating baseline in nodes (19/30
solved each) at about 26x lower wall time, so that rung cannot separate
pruning methods. Reproducibility: deterministic, `just experiments
cut-multiset-pruning`. Source commit e2641a5 (PR #21). Files:
research/experiments/cut-multiset-pruning/.

## 2026-11-11 · Agenda A18 proposed: exact frame marginals

Added after the A5 count: exact per-slot and per-adjacency frequencies over
all valid frames, tested for whether planted placements rank above colour
rarity on generated boards before any use as a border ordering. Nearest
dead ends: belief and survey propagation, A1 spectral statistics.
Reproducibility: none yet (proposal only, no computation). Source commit
c54478d. Files: research/program/AGENDA.md.

## 2026-11-11 · Round 3, A15 non-chronological bond-weighted unassignment: killed

Preregistered test on the hard ladder, judged by a separate session. kTAM
detachment (V1) and IMS blocker eviction (V2) against the ladder DFS
`scb-gacolor_ac3_random`, plain DFS and a tuned random-eviction arm, 1e6
events per instance, 6x6 c=5 and 7x7 c=6 first. R6 solved/30: 30, 30, 19, 23,
27; G1 against the ladder DFS NO-GAIN (ratios 186 and 100). R7: ladder DFS
10, every other arm 0. 8x8 c=9 not triggered. Learning: V2 beats random
eviction (ratio 0.43, p 0.004), so the blocker choice carries signal, but
non-chronological search without propagation loses to propagating complete
DFS by two orders of magnitude, also at equal CPU (exploratory). Follow-ups
A19 (ablation of the ladder DFS), A20 (blocker eviction inside propagating
DFS) and A21 (value order by propagation impact) added as proposed.
Reproducibility: deterministic, `just experiments ktam-unassignment`, full
rerun bit for bit except wall clock. Source commit 617c131 (PR #23). Files:
research/experiments/ktam-unassignment/, research/program/AGENDA.md.

## 2026-11-11 · Round 2, A16 backdoor-guided branching: killed

Preregistered test on the hard ladder, judged by a separate session. Stage 1
(root features on A3's 300 boards, labels from A3's backdoors): learned AUC
0.637 vs fail-first 0.373, but only +0.010 over signed domain size and 0.518
within interior cells, so the signal is position class plus domain size.
Stage 2 (seeds 1..20, cap 1e6 nodes): branching first on predicted backdoor
cells is NO-GAIN against MRV with the same propagation at 6x6 c=5 (node ratio
8.1 to 17.2), 7x7 c=6 (1/20 vs 15/20) and 8x8 c=10 (1 to 2/20 vs 20/20); K2
and K3 fire, and the loss holds on nodes without the time guard. Learning:
backdoor cells are not good first branching cells. Side observation, not
preregistered: `bb-mrv` (A3 propagation, piece all-different, MRV) beats the
ladder baseline `scb-gacolor_ac3_random` at all three rungs (8x8 c=10: 20/20
vs 12/20), so the ladder's calibrated baseline may not be the strongest
available; a fresh-seed baseline test is the next step. New ladder file
`8-c10` added. Reproducibility: node counts deterministic,
`just experiments backdoor-branching` (about 2 h on 8 vCPU; runs stopped by
the 240 s guard depend on machine speed). Source commit 3c85866 (PR #24).
Files: research/experiments/backdoor-branching/,
research/experiments/hard-ladder/instances/8-c10-*.jsonl,
research/program/AGENDA.md.

## 2026-11-11 · Round 2, A10 near-codeword spectrum: killed

Preregistered CP-SAT measurement of the fewest internal breaks of full boards
in near, mid and far overlap bands around a planted solution, judged twice.
Judge 1: invalid, because the corner pin leaves the 4-fold board rotation, so
the far band held the planted solution's own turned copy, and K1c was
undetermined. Fix run on rotation-quotiented distance d* (amendment A6,
committed before its solves). Judge 2: K1a fires on closed data; 5x5 C=13
median mid band 4 breaks against far band 7 (B* -3, 30/30 optimal), 6x6 C=14
B* in [-7, -1]; an independent judge-written CP-SAT model reproduces all 60
5x5 optima. Learning: on single-solution-class small boards the mid band is a
cheap saddle, not a barrier; no board far from the planted solution has 0
breaks. Untested: the barrier between two distinct solutions.
Reproducibility: CP-SAT with time limits, `just experiments
near-codeword-spectrum` (about 10 core-hours; unclosed solves reported as
intervals); witnesses rechecked by an independent numpy checker. Source commit
49bd769 (PR #25). Files: research/experiments/near-codeword-spectrum/,
research/program/AGENDA.md.

## 2026-11-11 · Round 3, A22 bb-mrv as a ladder baseline: kept at 8x8 c=10

Preregistered fresh-seed test of the A16 side observation, judged by a separate
session. At 8x8 c=10 seeds 21..50, `bb-mrv` against `scb-gacolor_ac3_random`:
30/30 vs 20/30 solved, KM median 69,036 vs 555,342 nodes, G1 ratio 0.147 (CI
[0.076, 0.313]) PASS, wall ratio 0.194; the judge's rerun is node-identical
60/60. At 6x6 c=5 and 7x7 c=6 G1 fails (0.80, 0.70) and `bb-mrv` is slower in
wall clock. Scaling on c = critical + 2: no evidence of a growing gain, since
9x9 c=11 and 10x10 c=12 are censored for both solvers on every pair at 1e7
nodes; exploratory n = 6..8 median log10 ratios -0.26, -0.38, -0.83.
Ablations (R8 node ratio against parent): no hidden singles 1.11, border-first
1.83, random value order 1.62, scb plus piece all-different 0.23. Learning:
the ladder baseline is beaten above the critical colour count by piece-side
propagation plus free MRV; the scaling question needs caps of 1e8 or more.
New ladder files `8-c10-fresh` and `plus2` (hard-ladder amendment A4).
Reproducibility: node counts deterministic, `just experiments mrv-baseline`
(about 90 min on 8 vCPU; wall clock depends on machine). Source commit
046324c (PR #26). Files: research/experiments/mrv-baseline/,
research/experiments/hard-ladder/, research/program/AGENDA.md.

## 2026-11-12 · Night of 2026-11-12: sixteen tracks, how to read the entries below

One unattended night (2026-11-12 18:30 to 2026-11-13 05:49), sixteen tracks,
each preregistered and judged by a separate session that did not run it:
eight keep, eight kill. Nothing was pushed, so no track has a pull request:
every branch named below is a local branch, not merged, not pushed, and its
head commit is given so the tree can be found again. Verdicts and numbers are
the judge's (`JUDGE.md` on each branch), checked on 2026-11-13 against the
drafts in `research/program/night/2026-11-12/DRAFTS.md`; where the two
differed the judge's file was followed. Tracks 1 to 8 have no preregistration
guard log (`prereg_guard.py` was written at 20:41, after they branched), so
their order rests on commit order and file times, which each judge checked;
Tracks 9 to 15 have the log. Night record: `research/program/night/2026-11-12/`
(REPORT.md, OBSERVATIONS.md, DRAFTS.md).

## 2026-11-12 · Night track 0, bb-mrv per-node cost: kept (optimization), 5x to 6x faster, node-identical

Optimization track from the night brief. `bb-mrv-fast` keeps the `bb-mrv`
search and changes only per-node cost (4-bit option bitsets, fixed-size
domains, a preallocated state stack, 128-bit piece masks). Outcome: keep. The
judge recomputed the node-sequence hash gate against the unmodified original
source: 140 of 140 instances identical on status, nodes, backtracks,
propagate calls and board (R6, R7, 8x8 c=10, plus 8x8 fresh seeds 31..50; 130
solved, 10 censored at 1e6, all at R7). Wall clock fast / original, median of
per-instance minima over 5 runs: R6 0.188 [0.178, 0.203], R7 0.163 [0.161,
0.166], 8x8 c=10 0.190 [0.188, 0.191]; no instance above 1. Limits from the
judge: identity at 9x9 c=11 is measured only on censored prefixes (7
instances, at most 300,000 nodes), so a track reporting 9x9 counts from the
fast solver runs `mrv_node_cost gate` on a few of its own instances; "supports
n <= 11" is argued from an assert, not measured; n >= 12 panics. Learning: the
ladder baseline's cost was per-node overhead, so node-count experiments can
use the fast solver where only counts, statuses and boards are read.
Reproducibility: node counts deterministic, wall clock machine-dependent,
`just experiments mrv-node-cost` (about 1 h on 1 core). Branch
`night/2026-11-12-t0-node-cost`, head 0860e322 (local branch, not merged, not
pushed); results b916767, judge 087b3013; d4334794 is formatting only after
the verdict, and the gate was rerun on it. Files:
research/experiments/mrv-node-cost/.

## 2026-11-12 · Night track 1, A22 follow-up: 9x9 c=11 pilot killed (cap too low)

Measurement of whether the `bb-mrv` / `scb-gacolor_ac3_random` node ratio keeps
shrinking at 9x9 c=11 (seeds 101..108, cap 1e8, threshold T = -0.83215 from
8x8 c=10). Outcome: kill by the preregistered rule, read as "no evidence the
ratio keeps shrinking", not "the ratio stopped". Upper median of the paired
log10 ratio M = 0.000 (K1 fires) and U = 6 uninformative pairs against a
limit of 4 (K2 fires, and decides); 95% CI of M [-0.106, 0.000]. 6 of 8 pairs
are censored at 1e8 on both solvers; seed 103 (`bb-mrv` 78,349,212 nodes, r <=
-0.106) and seed 106 (9,970,108 nodes, r <= -1.001) favour `bb-mrv`; scb was
censored 8 of 8. The judge reran `bb-mrv` seed 106 on another architecture:
node-identical. Learning: at 1e8 the cap, not the ratio, binds; an informative
follow-up needs an scb cap near 1e9 (14 to 23 h per run) or per-solver
Kaplan-Meier medians, and the judge would not spend a 30-seed run because
plain DFS is reported (a referee's runs, not measured here) to be about three
orders of magnitude cheaper per node at this size. Reproducibility: node
counts deterministic, `just experiments mrv-scaling-pilot analyze` (seconds)
or `test` (27.37 core-hours on three machines). Branch
`night/2026-11-12-t1-9x9-pilot`, head 0af81f7a (local branch, not merged, not
pushed); analysis 6762e141, judge ba7d7b11. Files:
research/experiments/mrv-scaling-pilot/, research/program/AGENDA.md (A22).

## 2026-11-12 · Night track 2, A20 blocker eviction as backjump rule: killed (this implementation)

Method test of blocker eviction (the IMS rule) as a repair rule inside
`bb-mrv` propagation, against the ladder DFS `scb-gacolor_ac3_random`, `bb-mrv`
and a random-eviction twin with identical propagation; 6x6 c=5 and 7x7 c=6,
30 seeds, 1e6 nodes, tabu tenure 16 chosen on calibration seeds by a rule
fixed in code first. Outcome: kill on the method clause (dropped at R6: node
ratio against scb 4.68 [2.40, 6.55], against `bb-mrv` 9.18 [5.39, 15.20], wall
32x and 25x) and K1 (R7 solved/30: scb 16, `bb-mrv` 23, blocker 5, run-seed
median 4); K2 also fires (twin median 3/30, paired ratio 1.000). The judge
reran 210 of 210 rows identically. Learning: an incomplete repair search on
propagation loses to complete chronological DFS with MRV at these sizes. K2
has low power at R7 (both arms nearly all censored), so it shows no
detectable difference between two failing arms, not that blocker choice
carries no signal. As run, 24 to 31% of blocker dead ends fall back to the
latest decision, and both arms are partly restart methods (tabu list cleared
4,169 and 10,922 times over 30 R7 runs). Scope: does not cover
conflict-directed backjumping proper (Prosser 1993), conflict sets from
propagation explanations, dynamic backtracking or stored no-goods, Rau's IMS
placement rule, other tenures, or 8x8. PREREG.md prose was finished after
calibration (disjoint seeds, same kill rules as the agenda). Reproducibility:
node counts deterministic, `just experiments blocker-backjump` (about 2.5 h
on 2 threads, summed from the README's stage times). Branch
`night/2026-11-12-t2-blocker-backjump`, head 388b4167 (local branch, not
merged, not pushed); judge 5dd7adca. Files:
research/experiments/blocker-backjump/, research/program/AGENDA.md (A20).

## 2026-11-12 · Night track 3, A21 value order by propagation impact: killed

Method test (PREREG a4507f5, no amendments). `vi-impact` is `bb-mrv` with each
option of the chosen cell probed and tried in ascending number of option bits
removed. Outcome: kill. G1 against `bb-mrv` fails at every rung: node ratio
1.075 [0.779, 1.931] at 6x6 c=5, 0.829 [0.544, 1.316] at 7x7 c=6, 1.096
[0.669, 1.871] at 8x8 c=10; wall 1.143, 0.814, 1.141. Against the
probe-then-random twin: 1.297 [1.058, 1.621] at R6 (worse than random among
the same probed options), 1.000 at R7, 1.128 at R8; against `mrv-rand` 1.325,
1.000, 1.127. The judge reran 210 rows identically (R7 not rerun). Learning:
ranking values by propagation impact adds nothing over random order among the
same options. Unplanned judge observation that matters beyond this track:
one run seed of random value order, `mrv-rand-s3`, passes G1 against `bb-mrv`
at 8x8 c=10 (0.495 [0.409, 0.768]) while `s1` sits at 1.623, so a G1 pass
against one deterministic baseline can come from a reshuffle; order-type
methods need run-seed controls. Scope: count of option bits, least first,
inside `bb-mrv`; not a log-product (Refalo) impact, not the most-constraining
direction, not 8x8 c=9. Reproducibility: node counts deterministic, `just
experiments value-impact-order` (about 1 h on 2 cores). Branch
`night/2026-11-12-t3-value-impact`, head a2656271 (local branch, not merged,
not pushed); results 596a3ad9, judge 5c65f77d. Files:
research/experiments/value-impact-order/, research/program/AGENDA.md (A21).

## 2026-11-12 · Night track 4, A17 Hall matching rebaselined on bb-mrv: killed

Method test: round 2's A17 (Hall matching on the frontier, V1 and V2)
rebaselined on `bb-mrv`, which already applies A3's size-1 Hall rules.
Outcome: kill by K2 for both variants. Paired G1 node ratios: 6x6 c=5 1.000
and 1.000 (NO-GAIN), 7x7 c=6 0.993 [0.992, 0.996] and 0.992 [0.989, 0.993]
(GAIN), 8x8 c=10 0.999 and 0.997 (GAIN): under 1% fewer nodes. Wall ratio at
every GAIN rung above 1 (R7 1.028 [1.013, 1.055] and 1.025 [1.009, 1.038]);
cost-control arms that compute the check but never act show it costs 2 to 5%
of wall clock. The judge's rerun reproduces 1,500 R6, 90 R7 and 90 R8 rows
node for node. Learning: round 2's 0.90 at R6 was measured against `scb`,
which has no piece all-different; on the stronger baseline the gain vanishes.
Round 2's result stands as measured against `scb` and is not adopted. The R6
preregistration order cannot be fully established from file times (R6 test
rows about 14 s after PREREG 860e368); R6 is NO-GAIN and cannot drive K2.
Reproducibility: node counts deterministic, `just experiments hall-on-mrv`
(about 75 min on 2 threads). Branch `night/2026-11-12-t4-hall-on-mrv`, head
d5134870 (local branch, not merged, not pushed); judge 67059eb3. Files:
research/experiments/hall-on-mrv/, research/program/AGENDA.md (A17).

## 2026-11-12 · Night track 5, baseline tournament of 47 exact-search entrants: kept (measurement)

Every solver-like component of the repo through one node counter at a 1e6 cap
on R6, R7 and 8x8 c=10, 30 instances, 204 cells. Outcome: keep; the kill
criterion (anchor identity) does not fire: `bb-mrv` 130/130 and
`scb-gacolor_ac3_random` seed 1 110/110 reproduce committed rows, 0
mismatches over 1,310 rows; the judge's rerun of 420 rows across both machines
matched. Results: R6 `bb-mrv` 30/30, KM median 460; R7 `scb-ad` 27/30, `mrv-rand`
26, `bb-mrv` 23, the adopted baseline 18; R8 `bb-mrv` 30/30, KM median 69,036;
row-major DFS solves 24/30 at R8 at about 1% of `bb-mrv`'s per-run wall clock,
but that is one cell order (column-major 20, double-snake 10, five other
orders 0). Learning (judge's corrections, all applied to the README): "random
value order helps at R7 and costs at R8" is not supported; the first two
places at R6 and R8 lie inside run-seed spread; the A16 solvers are ranked at
R7 and R8 on their 240 s guard, not on nodes; `bb-mrv`'s R6 screening flag
holds against baseline seed 1 only. Defect found in A12's code: `fc-luby`
seeding `(seed ^ hash) | 1` makes run seeds 2k and 2k+1 the same run.
Limits: 19.10 h of summed wall clock against 10 preregistered; some arms not
reached (listed in `not-reached.txt`). Reproducibility: node counts
deterministic, `just experiments baseline-tournament` (resumes; `analyze`
rebuilds the tables). Branch `night/2026-11-12-t5-baseline-tournament`, head
2fcbc132 (local branch, not merged, not pushed); results 240268a, PREREG
81860a4, judge 1f4e211. Files: research/experiments/baseline-tournament/.

## 2026-11-12 · Night track 6, dead-node refutation census: kept (measurement), three idea-slot proposals killed at stage 0

Measurement outside the brief (PREREG 41c069f). On `bb-mrv` trees (cap 1e6,
ladder test seeds 1..30) at R6, R7 and the 7x7 c=8 null, up to 200 dead nodes
per tree, stratified by depth, were tested with five bounded-region oracles.
Soundness: 0 violations on 7,524 live states, and 0 on the judge's own naive
search over 39,574 states; node identity 450/450. Weighted share of R7
dead-node weight proven dead: cavities r <= 3 0.008 (pool) and 0.040
(domains), 2-row bands 0.002, colour-only 0.017, pockets 0.035, partitioned
propagation 0.103 (cut) and 0.096 (random). Outcome: keep, with the judge's
rulings: T2 boundary-keyed cavity summaries killed at stage 0 on K0a only
(K0c compared two infinite medians and decides nothing); T5 band consistency
killed; ST boundary-word lemmas killed for the `bb-mrv` parent (0 of 1,447
sampled R7 nodes with subtree >= 200 proven); T3 cut-keyed partitioning NOT
killed here (the census used a one-shot share and a random control not
matched on partition count, median 16 against 6), so it went to Track 12; OC
piece-priced frontier bound not decided. Learning: local reasoning over small
regions proves the small dead subtrees and almost none of the weight, and
every share rises steeply with depth. Shares are upper bounds on savings with
unchanged branching; the oracles' own work is not subtracted. Not measured:
the `scb` parent. Reproducibility: deterministic counts, `just experiments
dead-node-census` (about 40 min on 4 threads). Branch
`night/2026-11-12-t6-dead-node-census`, head 966e9c0e (local branch, not
merged, not pushed); results 70df1dd, judge f5f9b9a3. Files:
research/experiments/dead-node-census/, research/experiments/justfile.

## 2026-11-12 · Night track 7, partial-match ladder: killed (this entrant pool and budget)

Measurement: an instrument for the 470 regime (best verified score at fixed
work on planted-optimum boards, n in {8, 10, 12}, five entrants). Outcome:
kill. Calibration qualified only n8 c6 and n8 c10; on rung A (n8 c6, 30
instances) median gaps were `break-r1` 6.25%, `break-r2` 13.39%, `sa` 3.57%,
`pt` 3.57%, `alns` 11.61%: 2 of 5 in the [2%, 4%] band against 3 required.
`sa` beats `break-r1`, `break-r2` and `alns` on 30 of 30 instances each, the
reverse of the 16x16 family order (confounded: implementation and board size
cannot be separated). Scope (judge): this pool and budget; not a ladder whose
constructive entrant keeps searching after its first complete board, nor one
built from the 16x16-native engines. `BreakSolver` stops at its first
complete board (at most 3.8e6 of 5.12e7 placements used). Learning: at n = 10
and 12 every entrant stalls 5% to 12% below the optimum at this budget, wider
than the 16x16 record gap (2.08%), so small boards at these budgets are not in
the regime of 470 against 480. The README's claim that the band is reachable
only by annealing entrants below critical was wrong (`break-r1` is in band at
n8 c10). After the verdict, signed-rank ties were fixed; one descriptive p
moved (0.047 to 0.032), no verdict depends on it. Reproducibility:
deterministic in score and work, `just experiments partial-match-ladder check`
and `analyze`; a third-machine rerun matched 98 of 98 rows. Branch
`night/2026-11-12-t7-partial-match-ladder`, head 65d05984 (local branch, not
merged, not pushed); judged at c8b129b, verdict 226baf4. Files:
research/experiments/partial-match-ladder/.

## 2026-11-12 · Night track 8, decision-diagram score-bound gap (stage 0): kept, on the threshold

Measurement of how tight a width-W relaxed decision-diagram upper bound is on
the best matched-edges completion of a break-tolerant row-major prefix,
against an exact oracle, on generated 6x6 to 8x8 boards; 8,063 prefixes, 0
soundness violations, decision stratum and 293 rows from the second machine reproduced bit for
bit by the judge. Decision stratum (7x7 c=6, 3 rows left): median gap 7 at
W=1, 5 at 64 and 512, 4 at 4096, 7 for the trivial bound. Outcome: keep. K1
(median gap above 4 at every W) does not fire, exactly on the threshold (166
of 240 prefixes at gap 4 or less); K2 does not fire (7 - 4 = 3). The W=1
control equals the trivial bound at the median on every stratum with 2 or more
rows left, so "3 edges better" means better than no bound. Size trend the
wrong way: at 2 rows left the W=4096 gap is 1, 2, 4 at n = 6, 7, 8. With many
rows left the diagram adds little at the critical colour counts (trivial minus
W=4096 median 1 at r=4, 0 at r=5 and 6) but more above them (3 at r=4 on 6x6
c=7 and 7x7 c=8). Scope: every decision prefix is deeply dead (median true
deficit 10 and 7), so the gap near the optimum is unmeasured; no search gain,
no wall-clock or 16x16 claim. Learning: stage 0 passes by an integer at its
threshold, on dead prefixes, and loosens with size; stage 1 needs prefixes
near the optimum and a wall-clock arm. Whether a record engine spends enough
of its search in the tail for a bound to matter is not settled (see the
record-cost entry below). Reproducibility: deterministic counts, `just
experiments score-bound-gap` (30.95 thread-hours against 10 to 15
preregistered). Branch `night/2026-11-12-t8-score-bound-gap`, head c96a6d73
(local branch, not merged, not pushed); results d5f2075, judge 11a2b912.
Files: research/experiments/score-bound-gap/.

## 2026-11-12 · Night track 9, resumable long runs for bb-mrv-fast: kept (optimization, after one fix round)

Harness change: progress records, checkpoints and node-identical resume for
`bb-mrv-fast`, so a 1e8-node 9x9 run survives a kill. Judged twice. Judge 1:
keep with defects (an edited checkpoint resumed silently as a different
search; errors exited 0; closed share 0 on a root refutation). Fix round under
preregistered amendment A1 (checkpoint v2 with a content digest); rejudge:
keep. Gates: progress on against the unmodified solver, 150 instances, 0
mismatches; three-process resume 0 mismatches; 9x9 c=11 at 3e6 0 mismatches;
fix-round rerun equal to version 1 on 368 rows; tamper gate 2,202 of 2,202
alterations refused. Overhead 1.008 [0.998, 1.011] (judge's spot check
0.996). Limits: the digest detects corruption and hand edits, not forgery; a
forged checkpoint dropping a last stack level of index 0 under a forced level
passes the digest-independent checks (structural cause, not float
resolution); version 2 cross-machine resume is argued, not measured; a guard
stop is not resumable. Learning: a state hash taken only at the deepest level
cannot see forced levels above it, common deep in a 9x9 stack. Follow-up not
done: put stack depth and last-level index into the state hash.
Reproducibility: node counts deterministic, `just experiments resumable-runs`
(fix-round gates about 11 min on 2 cores). Branch
`night/2026-11-12-t9-resumable-runs`, head 6e60130a (local branch, not merged,
not pushed); fix 299f8d3, results 0227330e, judge 1 86f302a, rejudge
1ecac8ba. Files: research/experiments/resumable-runs/.

## 2026-11-12 · Night track 10, real 16x16 profile: kept (measurement), the row-scan price measured on the real pieces

Two measurements on the official piece set, not a record attempt. M2: exact
row-scan tree size by population SMC (N = 1e6, 5 repetitions). Five clues
10^40.457 [10^40.440, 10^40.473] against the first-moment model's 10^40.49;
centre clue only 10^47.140 [47.061, 47.208] against 10^47.14; no clue
10^50.039 [50.017, 50.059] against 10^50.03 (same peak 10^48.76 at free depth
161). The model is an analytic computation (`memos/solve-path-5-price.md`), so
the agreement is not circular. Null: 20 instances painted from the official
colour counts, median 10^40.606 (sd 0.125); the official set ranks 6th of 21.
Gates passed before any M2 number was read (10 of 1,110 pairs beyond 3 s.e.;
frame count hit, 0.99994). M1: late-node share of a port of `blackwood_bt`
under the `record` unlock list, 4.5e-7 in the median of 6 seeds at 1e10
nodes; flagged budget-dependent by the preregistered rule (ratio 2.89 against
a limit of 2), so any statement at a record run's budget is argued. Outcome:
keep, with three wording fixes applied. Scope of M1: the `record` schedule and
this port only (its colour keying is not the published engine's, see track 13);
it says nothing about a record engine. Limit of M2: the population collapses
to about 300 families by free depth 175, so a heavy subtree it never visits
cannot be excluded. Learning: the quota prune is not why the late share is
small (`record-noquota` 1.4e-7), and a port equivalence check on the node
total at the cap is vacuous (compare deepest depth at several caps: 18 of 18
agree). Reproducibility: deterministic given seed and binary, `just
experiments real-puzzle-profile m1` and `m2` plus `scripts/analyze.py`, about
6.5 core-hours. Branch `night/2026-11-12-t10-real-puzzle-profile`, head
dc71ad56 (local branch, not merged, not pushed); results f48e1b5 and 06cde25b,
judge d6012509. Files: research/experiments/real-puzzle-profile/.

## 2026-11-12 · Night track 11, difference map with exact band projections: killed (at this cap)

Method test (PREREG 9a0c4a3) of a divide-and-concur difference map whose
projection solves k-row bands exactly, k = 0..3, beta 0.25, cap 20,000
iterations, 30 instances x 3 run seeds, on 6x6 c=5, 7x7 c=6, 8x8 c=8 and 8x8
c=10. Outcome: kill; both parts of the rule fire through the preregistered
censoring fallback: 0 of 30 instances solved at every k at 7x7 and 8x8 (one
run of 90 at 7x7 k=3), so every s_k is infinite against s' = 0.1165. The
band-width exponent was not measured. No band effect at 6x6, the only rung
with finite medians (KM ratio k/k0: k1 [0.41, 1.36], k2 [0.43, 1.30], k3
[0.77, inf]); bands cost 3x to 33x more per iteration; censored runs end
further from a solution as k grows (median best mismatches at 8x8 9, 10, 11,
13). Easy 8x8 c=10: the map solved 1 of 270 runs, `bb-mrv` 30 of 30. Caveat:
for k <= 2 the cap gives the map less wall time than `bb-mrv` at 7x7. Scope:
not larger caps, other step sizes or maps, or over-coloured boards (where the
map does solve: the memo's 8x8 c=12 instance 4 times in 5). Judge on
reopening: a different mechanism, not a larger cap, meaning a band projection
that lowers iterations at 6x6 with an interval below 1 against k = 0 before
any scaling claim. Learning: exact band projections do not buy iterations at
the critical colour count and their per-iteration cost grows faster than any
gain. Reproducibility: deterministic given seeds, `just experiments
difference-map-bands` (6.81 core-hours); judge rerun of 90 rows identical,
242 solved boards pass an independent checker. Branch
`night/2026-11-12-t11-difference-map-bands`, head d1ed89cc (local branch, not
merged, not pushed); judge 279118a9. Files:
research/experiments/difference-map-bands/.

## 2026-11-12 · Night track 12, T3 cut-keyed trace partitioning: killed at stage 0 on its matched control

Replay (method label) of idea-slot proposal T3 on the committed `bb-mrv`
trees, R6 and R7, branching held fixed so the node ratio is exact (PREREG
9375c26). Upper median node ratio, surviving over `bb-mrv`: cut key 0.483 (R6)
and 0.486 [0.469, 0.500] (R7); C1, a random key off the cut line with the
same partition count (mean of 5 seeds), 0.444 and 0.419; C2, POAC at matched
budget, 0.750 and 0.702. Outcome: kill. K2 fires (C1 beats the cut by 0.039
and 0.067, on all five seeds at both rungs and on 30 of 30 instances at R7);
K1-IDEAS fires (R7 work ratio 1.211, every arm costs more propagation work
than `bb-mrv`); K1 (0.486 against 0.5) and K3 do not fire. Gates: node
identity 60 of 60 trees, soundness 0 violations over 8 arms x 60 trees.
Judge notes: at R7 C1 re-keys more often and spends 9% more work, which does
not rescue T3 (C1 also wins on work at R6); under MRV branching the cut line
is close to an arbitrary line, so a cut key for a row-scan solver is
untested. Learning: any two-edge partition prunes about half of `bb-mrv`'s
nodes (0.42 to 0.49), generic to low-colour-set edges, not specific to the
cut. This replay is the proposal's own K2 and supersedes the census's
non-standing kill (track 6). Reproducibility: deterministic, `just experiments
cut-keyed-replay` (about 2 core-hours at R7). Branch
`night/2026-11-12-t12-cut-keyed-replay`, head 606e7e52 (local branch, not
merged, not pushed); results 2046e64, judge 18280596. Files:
research/experiments/cut-keyed-replay/.

## 2026-11-12 · Night track 13, colour keying of the record-style engine on the real pieces: kept (measurement)

Measurement on the official pieces, not a record attempt (PREREG 49a3ec2;
part 2 by amendment before its runs). Established from Blackwood's public
source and a bijection of his piece list onto ours (`scripts/map_colours.py`,
agreeing with a referee's map): his privileged colours are this repo's
{5, 20, 7}, one ring colour (24 sides) and two interior (50 and 48), 122
sides. Gain G = nodes per arrival at depth 201 for the port default {22, 21,
20} over the arm's, 10 openings, 1e9 nodes per run, one attempt: no clue,
published keying 1.31 [0.69, 2.43] (the judge's resample: [0.68, 2.43]); centre
clue 0.74 [0.42, 1.27]; with five clues the published keying never reached
depth 201 (deepest 52 to 191) against 135 arrivals for the default. Outcome:
keep. C1, a memo's keying gain of 450 to 4,600, is contradicted at 1e9 nodes,
one attempt, on this port. C2, the legal break path, is supported: legal
nodes at depth 201 or more per node, 7.63 [4.82, 13.9] for the published
keying (judge's resample [4.87, 14.3]); the full fidelity variant (legal
breaks, ring rule, class-specific candidates, no isolation rule) 10.8 with no
clue and 4.47 at the centre; dropping the isolation rule alone 1.86 and 1.53.
C3, staging more colours, is dead by the literal kill line for the memo's
schedule on this port only (top-left row scan); A2's zero is an infeasibility
of that schedule under this scan, not a yield. Wall clock: on one host the
published keying costs about 1,500 s per 1e9 nodes against 64 s for the
default (rough, partly the port's break path), so per-node gains are a
wall-clock loss. Port defect found: the no-break path can place a wrongly
oriented piece early and strand a whole run (seeds 105 and 108). Learning: the
port differs from the published engine in keying, non-physical pieces in the
break path, an isolation rule, the centre piece, scan order, and the
orientation defect; keying alone is not a lever on nodes and loses on time;
break-path fidelity is worth 4 to 11 times in legal deep nodes. Disputed
since: the 2026-11-13 port-fidelity track (D2, reported, not judged) says the
published scan has no border interleaving after depth 180 and the centre-piece
difference is in counting, not placement. Reproducibility: deterministic per
seed (16.1 thread-hours); judge reran 19 runs bit-identically. Branch
`night/2026-11-12-t13-quota-keying`, head ba921de3 (local branch, not merged,
not pushed); rows 55bd52e, judge 17c56889. Files:
research/experiments/quota-keying/.

## 2026-11-12 · Night track 14, five-clue colour keying on the real pieces: killed (this family under this quota)

Measurement (PREREG 6bcad1c) of whether a one-ring keying (one ring colour
plus two interior, quota scaled by (T - 3) / 119) helps five-clue search reach
legal depth 201. Screening: 680 keyings x 3 openings, 2,040 runs at 1e9
nodes: none placed a legal node at depth 201 or more (deepest legal maximum
136; median over keyings of each keying's maximum 47; median over runs 39);
the default keying reached 209, 204, 209. Confirmation on 20 held-out
openings: the screened top five, the published keying and five random keys
all at exactly 0; the default yields 4.88e-7 legal late nodes per node
(9,758 nodes, 195 arrivals). Outcome: kill, worded as the judge requires: not
a lever within the one-ring family under the scaled quota, a floor effect
(every arm 0, the ratio 0/0 set to 1 by convention), not "keying does not
matter". Confound: the default runs the unscaled quota (about 79% of its
sides by cell 160), every one-ring keying about 97.5%, so keying and slack are
not separated; the published keying at factor 1.0 is Blackwood's own quota,
so its failure points at port-versus-engine differences at least as much as
at keying. 503 of 2,040 screening runs leave legality in the top border row.
Judge reran 20 of 20 rows identically. Learning: the test that separates
keying from slack is track 15. Scope: 1e9 nodes per run, one attempt, this
port, top-left row scan. Reproducibility: deterministic rows, `just
experiments five-clue-keying` (20.6 thread-hours). Branch
`night/2026-11-12-t14-five-clue-keying`, head 4ce8a55e (local branch, not
merged, not pushed); rows 92690117, judge ee5391c8. Files:
research/experiments/five-clue-keying/.

## 2026-11-12 · Night track 15, five-clue quota curve and port defect: kept (measurement)

Measurement on the real pieces, five clues, 32 arms x 16 openings (seeds 501
to 516), 1e9 nodes per run, 5.06 thread-hours (PREREG 7bb763a5; amendment 2,
the deconfounding arms, committed 29 s before the first test run). Outcome:
keep. Reading R5, slack explains track 14: the default keying under a quota
scaled to the one-ring share (147 of 150 sides by cell 160) reaches a legal
node at depth 201 or more on 0 of 16 openings; the published keying under a
quota matched to the default's share on 14 of 16, under the default's absolute
curve on 2 of 16. At matched slack the default is ahead on point estimate
(published-style keyings 0.61 to 0.75 of its yield per second, intervals
including 1), so keying is not shown to matter. Loosening the whole curve
helps; delaying or halving its early part does not (the halving arm is a
design artifact, a step from 50 to 102 at cell 96). Which curve segment binds
is not shown (judge's wording fix W1). Orientation defect: it strands top-row
runs; the one-ring family is rarely stranded (3 of 128); the fix costs
nothing (0.983 [0.95, 1.04] of yield per second) and changes no non-stranded
run (233 of 233 pairs identical). Learning: on this port the share of
privileged sides the quota demands, not the colours, decides whether
five-clue search reaches depth 201; quotas should be stated as a share of the
keying's sides. Judge: order verified (584 paired guard lines), 8 rows rerun
exactly. Reproducibility: deterministic per row (seconds vary), recipe in the
README. Branch `night/2026-11-12-t15-five-clue-quota`, head b0a185c2 (local
branch, not merged, not pushed); judged at 7fb856e9, judge commit c9088558,
wording fixes 55e12880, b0a185c2 is cargo fmt only. Files:
research/experiments/five-clue-quota/.

## 2026-11-12 · Refereed: the price of a full solve by row scan, model and measurement agree

Night memos `memos/solve-path-5-price.md` and `solve-path-3-exponent.md`,
refereed in `memos/price-and-ledger.REVIEW.md` (all sixteen memo scripts
rerun, an independent implementation in `memos/price-and-ledger-review/`),
then checked on the real pieces by track 10. What the program now believes:
exhausting the row-scan tree with the five clues costs 10^40.49 nodes by the
first-moment model (a node is a valid partial board) and 10^40.457 [40.440,
40.473] measured on the official pieces; one clue 10^47.14 (model) and
10^47.140 (measured); no clue 10^50.03 and 10^50.039. The gap to 1,000
core-years is 17 to 20 orders. Struck by the referee: path 2's 10^45.3 and
path 6's 10^47.7 (one script error, spreading the full-board match
probability over joints; corrected 10^40.66), and path 3's 10^14.8 (a second
script error). Kept: with the true frame given, the interior costs about
10^8.3 nodes by inward spiral (model mean over frames, not measured); at the
row-scan peak 30 of the 39 digits are partial-frame multiplicity;
frame-first search costs 10^37.6 frames times about 10^8.3 per refutation,
10^45.9, 5.5 digits above row scan; no join or merging changes the exponent
(best gain 1.0 digit, at most 2.4 for a band join storing both lists). Not
established: "ranking is worth 25 to 150" and "no error beyond 0.3 orders for
a random 16x16". Expected solutions (ensemble estimate, refereed in
`partial-match-regime.REVIEW.md`): about 14,702 with the starter clue, about
4.1e-8 with five. Learning: the price is now a measured number on the
instance, and the model that predicted it is validated at three clue
settings; two memos that reproduced the solution count were still 4.6 to 4.8
digits off on the tree, so reproducing a count does not validate a tree
model. Reproducibility: model scripts deterministic (referee's rerun);
measurement as in the track 10 entry. Source commits d3d727b4 (path 5 memo),
627736f5 (review), lead branch history, not pushed. Files:
research/program/night/2026-11-12/memos/.

## 2026-11-12 · Refereed: no cheap information about the frame was found (local and corner tests)

Memo `memos/frame-information.md` (about 25 core-minutes of scripts),
refereed in `memos/theory-and-frame.REVIEW.md` (7 core-minutes, quantities
recomputed by other routes). What holds: the four corners carry about 20
cheap bits, 17.45 exactly on the official pieces (57,131 valid five-piece
border chains per corner; the memo's 17.17 corrected by taking the other clue
pieces out of the pool); 91 more bits would be needed to bring row scan to
1,000 core-years; on the real pieces arc consistency over all 196 interior
cells kills 0.4313 of shuffled border words against 0.4293 for the first ring
alone, so propagation sees nothing past the corner windows (measured by the
referee); about 2 ranking bits on generated boards (z = 1.40 against valid
frames), half of it in the corner counts. Struck: "the corner bits are already
inside the price" (true for two corner blocks only; the far two close at scan
cell 239; closing them first costs 0.64 digits, so the practical conclusion
stands for another reason) and the 6x6 refuter result (one weak refuter; arc
consistency plus all-different refutes 94.4% to 100% of wrong 6x6 frames with
zero search nodes; 6x6 says nothing about 16x16 either way). Learning: the
frame question is closed for counting, corner lists, first-ring counts and arc
consistency (graded 0.75 by the referee); open: all-different reasoning and
clause learning on partial frames of the real pieces, the question the
2026-11-13 track D1 is measuring. Reproducibility: deterministic scripts in
`memos/frame-information/` and `memos/theory-and-frame-review/`. Source
commits 902d69e6 (memo), bc5fef40 (review), lead branch history, not pushed.
Files: research/program/night/2026-11-12/memos/.

## 2026-11-12 · Refereed: claims struck from the night's memos, and where a record's cost is paid

Every memo that got a second reader lost at least one headline claim. Struck
or not established, by referee (grades are the referees'): growth law, "no
ladder rung is critical" refuted by enumeration (6x6 c=4 boards have 357
solutions on average; corrected critical colour counts 4.65, 5.99, 7.32,
8.63, 9.94 for n = 6..10), "relaxing distinctness cannot refute before the
peak" wrong as stated (37% and 57% refuted more than a row before the peak),
"propagation is only a surface factor" not established (`growth-law.REVIEW.md`,
774e9b53; kept: `bb-mrv`'s node lead over plain DFS does not grow with n, and
plain row-major DFS solves 9x9 c=11 in 2 to 6 s); partial-match regime, "471
costs 80 to 235 times 470" not established and "a score bound has little to
remove" not established (`partial-match-regime.REVIEW.md`, 3186a18c; kept:
10^34.4 boards at 470, 10^31.65 at 471, a step of 10^2.7 per point, as
ensemble expectations); literature, "the record moved this month" wrong (a
five-clue 466 has been public since at least 2026-10-08) and "row-major
scanning is optimal" wrong (`literature-and-path-6.REVIEW.md`, ebaad0ae);
solve paths 2, 3 and 6, the prices above (script errors); where a record is
paid, "keying alone is worth 450 to 4,600 times" wrong (unlike comparison; 3
to 20 like for like) and "anything after depth 201 is capped at 1.1 to 1.9"
wrong (`where-a-record-is-paid.REVIEW.md`, 3b103c2b); frame information, two
supporting statements (entry above); theory referee, all six rulings held,
but 3 of 5 sampled "sound" pages were one grade too kind. Established from
Blackwood's public source by the record-cost referee: the published engine
keys on this repo's colours 5, 20, 7 (quota 119 by cell 160), builds legal
boards only, pins piece 139 in the centre and has no isolation rule. Where a
record engine pays its nodes is not settled: track 10 measured 4.5e-7 late on
a port keyed differently; the partial-match referee inferred 92% to 98% from
one model row; the record-cost referee saw 0.4% to 53% of legal nodes late
across seeds and clue settings, and track 13 confirms the spread. Learning: a
memo is a claim until refereed (draft protocol rule P9); the score-bound line
was declared closed twice in one night and reopened both times. Files:
research/program/night/2026-11-12/memos/, REPORT.md ("What the referees
changed").

## 2026-11-13 · Refereed: the puzzle's numbering and stored orientation carry no recoverable trace; route closed for now

Memo `night/2026-11-13/memos/how-the-puzzle-was-made.md`, refereed in
`how-the-puzzle-was-made.REVIEW.md` (facts recomputed independently). Holds:
the official numbering is exactly a sort (class, then smallest rotation; 0
inversions in 20,656 pairs) under one key and one colour labelling, and the
stored orientation is that canonical form, so both are functions of the piece
set and say nothing about the arrangement; no statistic of about 25 distinct
ones passes the multiple-comparison line against a null matched on colour
counts and the frame and interior split. Reworded: "zero bits" becomes "at
most 42.7 bits (the untested order within colour-count tiers), with no
mechanism". Struck: the seed-enumeration odds of 1 in 1,000 (a guess) and a
sentence expecting one solution (the project's figure is about 14,702 with the
starter clue). This agrees with the 2026-11-12 path 2 memo (its
zero-information argument is Bayes' rule and rules out design traces, not
solution marginals). Learning: the construction route is closed for now; the
one loophole left (a coarse stratified painter) is invisible in the piece set
and testable only by running search under a guessed painter. Reproducibility:
deterministic scripts in `night/2026-11-13/memos/how-the-puzzle-was-made.scripts/`.
Source commit 5ed776d9, lead branch history, not pushed. Files:
research/program/night/2026-11-13/memos/.

## 2026-11-13 · The strict five-clue record is 466, not 465: a public board verifies

The site's strict five-clue record of 465 (ledger entry of 2026-09-17) is
stale. The board `FiveClue_466_2c037e70f7e9` from the public repository
`gearashi/EternityIISolver` (`data/record466.json`, fetched 2026-11-13,
attributed there to "Jef", origin `stats.eternityathome.org`) was checked with
independent scripts (`research/program/night/2026-11-12/checks/verify-466/`):
256 pieces each used once, the piece multiset equal to the official set up to
rotation under one colour relabelling, 5 of 5 official clues honoured in the
repo's own definition, 466 of 480 internal edges matched, no grey side off
the rim. Outcome: measured; the record moves to 466 strict once
`/research/records` is updated (not edited by this entry). Limits: the colour
relabelling was found by search (the score and the multiset do not depend on
it, the clue check does); the first-publication date (2026-10-08, from the
literature referee) was not rechecked; attribution is the source's metadata;
the downloaded data is not committed, so the check reruns only after fetching
it. Reported, not verified: a BOINC project page shows 466 with five clues
(four boards dated 7 to 12 November) whose boards cannot be downloaded. The
repo has no command that verifies a board given as pieces and rotations; one
should come with the records update. Reproducibility: deterministic checker,
`python3 -I check.py <data dir> <official csv>`. Source commit d31b82bb, lead
branch history, not pushed. Files:
research/program/night/2026-11-12/checks/verify-466/.

## 2026-11-13 · Follow-up to the 466 entry: data committed, verifier exists, library date 2026-09-19

Corrects the entry "The strict five-clue record is 466, not 465" above, which
was stale when committed (d75ad2e8, one minute after 7c4757eb). Three of its
statements no longer held: the board data is committed
(`research/topics/record-boards/community/fiveclue-466-2c037e70f7e9.json`);
a verifier for boards given as pieces and rotations exists
(`research/topics/record-boards/compute/verify_pieces.py`, recipe
`just research-record-boards-pieces`, output
`results/community-verified.json`); and the earliest public date is
2026-09-19T11:27:35Z, the timestamp of this board in the Eternity@Home library
listing `s/466/0.json`, not 2026-10-08 (that is the date of the GitHub mirror
commit). The listing held 14 boards at 466 on 2026-11-13, timestamps
2026-09-19 to 2026-11-12; `s/467/0.json` returned 404. An independent check
from the piece file (rotation convention unambiguous: the opposite one puts
grey sides off the frame) confirmed 256 distinct pieces, the five clues at
their cells and rotations, and 466 of 480 edges. Outcome: measured, valid
strict five-clue 466, stated on the site as the highest verified as of
2026-11-13 (only the library and one mirror were checked). Reproducibility:
artifact verification, deterministic. Commits 7c4757eb, 5b7c687c, 4d05811c.
Files: research/topics/record-boards/, web/src/data/records-timeline.ts,
web/src/components/research/views/RecordsView.tsx.

## 2026-11-13 · Documentation corrections: what was withdrawn and why

Theory and build pages were corrected against committed results and an
independent review (`research/corrections/2026-11-13-REVIEW.md`, 29 numbers
recomputed, 0 mismatches; 17 required fixes, all applied). Withdrawn: the
entropy limit 0.67 (it exceeds h(6) = 0.640, impossible for an infimum), the
area-law fit exp(-0.0445 n^2) and its 155-cell collapse (the sampled fraction
at 48 cells is about 300 times below it); the 76 ceiling on one-move
neighbours at 478 (it counted interior pairs only; the census gives a floor of
154 = 125 near-twin swaps + 29 turns); the flat 10^45 search-tree plateau (the
committed complex-theory reference peaks at 7.3e45 at depth 161, 1.6e27 at
depth 50, 5.0e40 at depth 200, so the fill-order "two-order gap" is close to
three); "proven" on the rigidity page and on open problems (the MIP runs are
uncommitted notebook results, graded measured); the sigma-cycles title "Why
basin-hopping looks impossible" (the body shows only that interpolating
between known boards fails and that the loops match a random null; retitled
"Interpolating between record boards fails"); the glossary's causal clause on
near-solution scarcity; the "standing" strict five-clue record wording (now
"highest verified as of 2026-11-13"). Regraded: 13 dead-end entries by
evidence, 22 moved to untested claims; the strict five-clue record moved from
465 to 466 site-wide; AGENDA and EXPLORED rewritten with evidence grades. The
entropy scripts moved into `research/topics/entropy-area-law/compute/scripts/`
and the single-move script into `research/topics/parity-defect-floor/compute/
scripts/`, so site pages cite topic results, not the correction log. The
distinct-piece sampler, the one input the review could not reproduce, was
rerun with seeds 101 to 106: 4x4 0.3261, 4x6 0.1071, 4x8 0.02443, 4x12
0.000389 against 0.3263, 0.1060, 0.02383, 0.000375; largest gap 1.8 combined
standard errors; 1e-3 crossing 44.4 cells against 44.2. The entropy page's
repro is now graded seeded. Learning: carried constants from offline sweeps
are where the withdrawn numbers came from; every carried value now names a
committed script and output. Reproducibility: exact for the census and the
counts, seeded for the sampled fractions. Commits e390a746 to the commit
adding this entry on night/2026-11-13-docs (not pushed). Files:
web/content/research/, web/src/data/, research/topics/{entropy-area-law,
parity-defect-floor,record-boards}/, research/corrections/, research/program/
{AGENDA,EXPLORED}.md, AGENTS.md, justfile.

## 2026-11-13 · Note on a miscited commit

The entry on the construction memo (2026-11-13, "Source commit 5ed776d9, lead
branch history") names the wrong branch by implication: 5ed776d9 is on
`night/2026-11-13-lead`, not on `night/2026-11-12-lead`. The entry stands
otherwise.

## 2026-11-13 · Frame cores, port fidelity, scan orders and pins, prefix statistics, pin map: five tracks independently assessed and closed

Five tracks, each independently assessed and closed on 2026-11-13. Frame cores (D1, real pieces): cheap refutation of full frames is dead (lower medians 0.89, 0.12 and 1.00 against a line of 100). The stage 2 pass (36.75 bits [32.90, 53.19], median core 5 cells) holds by the letter but is a corner artefact: 21 of 23 near frames die on a 3x3 clue block. The far stratum is undecided (37 frames, 16 censored, median 75.12 bits, upper end infinite). Nested arc cost 27.68, 31.37, 35.39 and 39.49 bits, each 2.2 to 2.5 bits above its break-even budget. About 59 core-hours. Kept as a measurement, not as evidence that frame search with nogoods beats row scan. Port fidelity (D2): seven differences from the published engine fixed, each with a test and a legacy switch; 23 of 23 node-sequence comparisons identical; with five clues the faithful defaults do worse (deepest depth 18 to 54 and no legal node past 200, against 33 to 215 for the old defaults, eight seeds). Scan orders and pins (D3): none of 18 static orders is shown to beat row scan (the kill line is reached for 15, three are undecided because the estimator's population dies); a 2.11-digit "win" of one searched order is withdrawn. On painted 16x16 boards the best single extra pin is worth 10.05 bits, the median 5.79; a 10^15 run is over budget at 10 spread pins and under at 16 to 24 (checked end to end only). Prefix statistics (D4): a census statistic tilts about 2 sd toward the planted prefix at depths 144 and 160 on 48 painted boards; worth 1.02 bits [0.72, 1.27] in one scan direction, -0.04 with two, -1.88 with four; no claim of transfer to the real puzzle. Pin map (D5, 8x8 and 10x10 toys): under row scan a known tile's worth is set by its place in the scan order; a touching pair loses about half of the second tile's worth; under an adaptive solver the placement rules still differ by 6.8 bits at k = 3 and converge only by k = 6 to 8. An unplanned follow-up measured how much of the tree arc consistency refutes: the share of sampled wrong prefixes not refuted is 0.427 at depth 96, 0.096 at 112, 0.179 at 128, 0.064 at 144, 0.007 at 152 and 0 at 160, and the true prefix is never refuted; applied to the official profile that is about 5.2 bits of nodes (derived), at a per-node cost of about 13 bits at the measured speed (not independently assessed; the engine changed after the D4 assessment). Learning: every check that works, works too late; the old claim that arc consistency refutes close to nothing at depths 96 to 128 was wrong (it refutes 57 to 90 percent there), with the conclusion unchanged. Generated and painted boards are toys. Reproducibility: deterministic per seed for D2 and D3, seeded for D4 and D5; `just experiments frame-cores`, `orders-and-pins`, `prefix-statistics`, `pin-map`. (commit: pending) Files: research/experiments/frame-cores/, research/experiments/orders-and-pins/, research/experiments/prefix-statistics/, research/experiments/pin-map/, research/experiments/single-core-benchmark/, research/experiments/approach-pricing/.

## 2026-11-13 · Seven gap studies, each independently re-checked: no route priced at 10 bits or more

Seven desk studies on the places the three known obstructions do not reach, each followed by an independent re-check that narrowed or struck claims. Search that learns: "empty" is unproven as written and holds only for reasons attached to row-scan nodes, used one at a time; about 2 bits (ceiling 6.6); an off-the-shelf clause-learning solver needs 2^5 to 2^6.5 fewer conflicts than tree nodes on 5x5 and 6x6, flat across the two sizes, which is not a time gain. Lossy representations: the two-list join gains 3.0 bits (2^131.6), not 5.9; a colour-blind sound sketch saves at most 2.0 bits (a derivation that needs a second reader). Shape of the tree: blind restarts and portfolios net 1 to 3 bits with oracle cutoffs (ceiling 6.5) and lose about 1 bit blind; the counting bound in the study is unproven. Adaptive orders: row-shaped rules within 3 bits of row scan under forward checking and within 1 under arc consistency; free smallest-domain is 13 to 49 bits worse; arc consistency alone 5.5 bits; finishing rows one or two at a time gains at most 5.3 or 6.7 bits (proved in the re-check). Tile-set structure: no colour automorphism, invariant group Z^2 + Z/2, 0 bits before depth 192; the study's "0 of 184 prefixes refuted" had no evidential weight (all 184 are dead). Algebra and counting: none found among fifteen reformulations, not a barrier; one lemma was wrong on real tiles, the merge factor is about 0.033 bit, not under 0.02. Search-space reformulation: row scan within 0.24 bit of the best static order (annealing, no certificate); a four-side colour pattern identifies one tile (1,024 oriented patterns, all distinct), so searching by edges is searching by tiles. Largest gain priced anywhere: 5.5 bits of nodes (arc consistency), whose per-node cost exceeds it at the measured speed. Learning: a gap study's "empty" verdict is only as wide as the family it priced; of 60 claims later struck or corrected in the arc, most came from desk studies and survey citations (seven survey citations corrected, 121 checked, 103 confirmed, none invented). Open from these: a second reader for the 2-bit sketch bound, a certificate that row scan is the best order at the bottleneck, weighted degree at 8x8 and 9x9, the injective-only hole (an object with one term per solution always exists; whether it can be built without the solutions has no model). Reproducibility: desk studies with small toy-board scripts (boards generated, never Eternity II progress); the numbers are re-derived by the second analyses, not rerun for the index. (commit: pending) Files: research/experiments/approach-pricing/notes/gap-*.md, research/experiments/approach-pricing/surveys/, research/experiments/approach-pricing/scripts/, research/experiments/approach-pricing/results/.

## 2026-11-13 · Linearisation degree on toy boards: degree 2 fails at 4x4 and 5x5, row not formally closed

Test fixed before any run on the algebraic row (linearisation, in the family of XL, Nullstellensatz and polynomial calculus): the price of 2^67 for degree 2 stays alive only if degree 2 pins the solution at both 4x4 and 5x5 on most boards. Result on generated boards with one solution, over GF(2) and a large prime: degree 2 determines 10 of 10 boards at 3x3, 0 of 10 at 4x4, 0 of 5 at 5x5. At 4x4 the needed degree is 3 on 6 or 7 of 10 boards and 4 on the rest; at 5x5 it is at least 3, and whether 3 suffices did not fit in memory (16.2 million monomials, stopped at rank 2.35 million with 1.07 billion nonzeros). Degree 2 only propagates (a median 18 of 128 variables forced at 4x4, 60 of 461 at 5x5); controls pass. Outcome: the degree-2 price is dead; the row is not formally closed (degree 3 at 5x5 was never measured). Learning: sparse elimination cost grew about linearly in monomials where runs completed, so the stated price per degree may be too high; memory, not time, was the wall. Not independently checked. Reproducibility: deterministic scripts, toy boards from the study's own painter. (commit: pending) Files: research/experiments/approach-pricing/notes/gap-6a-linearisation-degree.md, research/experiments/approach-pricing/scripts/gap-6a-linearisation/, research/experiments/approach-pricing/results/gap-6a-linearisation/.

## 2026-11-14 · Relaxation line closed as a negative: convex lift, low-rank form, annealing comparison, growth exponent

A bounded study of relaxation solvers on generated toy boards, closed on 2026-11-14. Convex lift (degree-2 semidefinite with entrywise nonnegativity): on five generated 5x5 boards, three are certified clean by a converged largest-leak solve (seeds 1, 2 and 4, with 2 to 8 solutions), one is undecided (seed 0) and one is not exact (seed 3, 403 solutions: 0.0047 of the mass per cell stays on 10 placements that are in no solution at violation 5e-9, and about 0.042 once polished to violation 1.5e-9; an independent enumeration gives the same 403 solutions). One solver family only; a second solver is still needed. The sentence "the lift is exact at 5x5" is withdrawn, and so is the 60, 20 and 2 percent odds for exactness at 6x6, 8x8 and 16x16 (a judgment, formed before a 5x5 board was found inexact). Cost: 400 to 1,100 s per 5x5 solve on boards a depth-first search clears in under a second; at 16x16 one dense copy is 183 GiB, the factor plus nonnegativity multipliers about 119 GiB. Three 6x6 largest-leak runs did not converge before they were stopped (primal residual about 3e-4 against a target of 1e-7; mass off solutions per cell 0.843, 0.865, 0.730, flat to 0.001 over the last 1,000 iterations; lower bounds of 24, 13.5 and 13 hours to reach the target): the stop line decides nothing, and exactness at 6x6 stays open with an unconverged reading that points to a large leak. A pseudo-inverse cutoff fault was found at 6x6 on one machine; no 5x5 figure and no re-checked claim rests on it (not independently checked). Low-rank form: a penalty-guided randomised search with an exact acceptance test, not a relaxation solver. At 5x5 it solved 200 of 200 runs on 15 boards (configuration chosen on 50 runs, held on 150 unselected), about 5,200 evaluations per solution; annealing with restarts is 8 to 35 times cheaper per solution at 5x5 and 3.6 to 63 times cheaper at 6x6, and exhaustive search takes 0.3 to 0.6 s. The nonnegative factor solved 0 of 50 starts at 5x5 in each of four variants (0 of 2,000 in the simplest), against 26 of 50 for annealing; the formulation aspect is nearly empty (six of eight smaller lifts leak or are vacuous at 5x5). Annealing at 7x7 found nothing in 210 runs; its rate is below 26 percent per 3e9-step run but not shown to be zero; the earlier "0 of 1,000 at 6x6" was a budget artefact (the same annealer solves 2 of 3 runs at 1,000 times the budget). Growth exponent: on a 10-core Apple M4 laptop with 30 CPU minutes per start, 22 of 50 starts solved at 6x6; charging capped starts in full, 460,416 evaluations and 3,084 CPU seconds per solution against 5,083 and 6 at 5x5, an exponent of 4.5 in P (bootstrap over boards 3.97 to 5.31, two sizes only; the censored median, 270,051 evaluations, gives 4.53); a replicate on a 14-core low-power x86 laptop solved 3 of 50 and agrees on the speed-independent measures (exponent 4.86). The broken-board control gave 0 of 10 on each machine. Two sizes cannot tell a power from an exponential; extrapolated to 16x16 it is about 1.3e15 evaluations per solution, labelled an extrapolation. Outcome: closed as a negative: the lift is exact on some 5x5 toys, leaks on at least one and cannot run at scale; the low-rank form loses to annealing at 5x5 and 6x6. Learning: a heuristic's success at a small size says nothing about the lift (the low-rank form also solves 46 of 50 with nonnegativity removed); "solves happen early, restart sooner" read from three rows was wrong (median solve at step 605 of 1,000), and "460,416 per solution with capped runs censored" was a full-charge figure. Toy boards only; none of this is Eternity II progress. Reproducibility: seeded, long runs; the committed notes carry the commands and the raw logs. (commit: pending) Files: research/experiments/relaxation-small-boards/ (notes/, scripts/, results/, lowrank-rust/).

## 2026-11-14 · Route audit and anomaly hunt: three weak closures, no surviving anomaly

Route audit of 78 lines of the routes index: 47 closures (32 sound, 13 of them with a stale or overstated index row; 10 weak and bearing on a solve; 5 weak but relevant to records only). The yardstick behind most sound kills: a filter on a row scan gains at most the ceiling at its depth plus the bits it removes there. The ceiling of a perfect free cut from free depth d onward, in bits: 57.1 at 52, 44.8 at 64, 29.8 at 80, 17.1 at 96, 10.7 at 112, 6.8 at 128, 2.4 at 144, 0.4 at 160; the widest level is at depth 158. The stratified-painter loophole is priced at 23.6 bits under row scan (derived) and stays closed. Three weak closures to reopen first, none above 1 percent odds of a solve: exactness of the convex lift at 6x6, growth of non-tree solvers against the tree on the same boards, learning solvers at 7x7 and on real-piece prefixes with a propagation-only control. Anomaly hunt on 51 candidates: 14 disagree with their model beyond noise at face value; none survives a painted-board null and a count of comparisons. The three-row staircase (+2.85 bits in a gap study) is not reproduced at ten times the population (135.2 and 135.7 bits, above row scan); the frame count's 0.106-digit deficit is explained by the 14 edge pieces with the same colour on both sides (0 bits); the official level profile shows no |z| above 3 across 185 widths, 179 branching ratios and 156 dead-end shares. Outcome: negative. Learning: the weak kills sit where the yardstick does not reach, in solvers that are not trees and in reformulations that would become polynomial if some property held. Both are not independently checked; the wording for the staircase is "no gain shown" (two runs). Reproducibility: desk analysis; the ceiling table is printed by `python3 -I scripts/route-audit/strata_gain.py` from the committed level profile. (commit: pending) Files: research/experiments/approach-pricing/notes/route-audit-weak-kills.md, research/experiments/approach-pricing/notes/anomaly-hunt.md, research/experiments/approach-pricing/scripts/route-audit/, research/experiments/approach-pricing/routes.md.

## 2026-11-14 · Learning against propagation on toy boards, 5x5 to 7x7: independently re-checked, stop line undecidable

Measurement of how many bits of search work clause learning saves over a solver that only propagates, on ten generated boards per size, with the stop line written before the run (the lead over row scan with arc consistency must rise by 2 bits from 6x6 to 7x7, and by 3 bits from depth 104 to 88 on real-piece prefixes). Same solver and order, learning on minus off: +1.31 bits at 5x5 (95 percent +1.06 to +1.49) and +2.03 at 6x6 (+1.33 to +2.26); no 7x7 value, because the learning-on arm finished 0 of 10 boards in the one-hour limit. A table entry of +6.06 at 7x7 measures the time cut, not learning, and is an upper bound. CaDiCaL over row scan with arc consistency: +2.19 at 6x6 and at most +4.28 at 7x7, a rise of at most +2.09 against a threshold of 2, with no lower bound (a cut solver may need any number of further conflicts). The re-check found a fault in the first note: 7x7 board 5 was dropped from the paired figure because the arc consistency run was killed with no output although CaDiCaL finished it in 430 s, which is the learning solver's best case, so the note's +3.92 bounds only the 8 boards it used. Outcome: the size clause is undecidable and the depth clause was never run (no real-piece part); the stop line is undecidable on this data. In seconds the learners lose at every size (log2 time ratios -2.00, -3.15, -2.98 in the first note). Estimated worth for the full puzzle: 0 to 2 bits in time. Size is confounded with tightness (inner colours 4, 5, 7). Not run: 8x8, the real-piece part, a cardinality solver, weighted degree with restarts; some 7x7 jobs were lost when one machine restarted. Independently re-checked; toy boards only. Reproducibility: seeded boards regenerate byte for byte; `just experiments learning-vs-propagation` reruns the analysis from the committed raw output (solver builds not verified). (commit: pending) Files: research/experiments/learning-vs-propagation/ (notes/measurement.md, notes/recheck.md, scripts/, results/).

## 2026-11-14 · Decision: stop the hunt for a method, publish the negative results, park with tripwires

Decision of 14 November: stop the research here and publish all the work. Basis: none of the seven re-checked gap studies prices a mechanism at 10 bits or more; the only candidate (linearisation at degree 2) failed its toy test; the relaxation line closed negative; the route audit and the anomaly hunt returned nothing that survives; the learning measurement left its stop line undecidable. The one-sentence reason: every check that works, works too late. The tests that refute wrong boards refute nearly all of them only from depth 144 to 192 (arc consistency already refutes 57 to 90 percent at depths 96 to 128, where the levels hold few nodes), the widest level of the measured tree is at depth 158 and a perfect cut at depth 144 saves at most 2.4 bits, so the largest gain priced is about 5.5 bits of nodes against a gap of 56 to 73 bits (about 84 bits against a modest run of about 10^15 nodes). A cardinality solver arm was not run: one more learning arm cannot change a comparison where the family's best case is about 2 bits against that gap. Tripwires for reopening: a published method with a class-level gain (price at 10 bits or more with a falsifying experiment under a day; the ceiling table is the first filter); tiles of the solution becoming available from outside the search (best extra pin about 10 bits, median about 6; a 10^15 run needs more than 10 and probably 16 to 24 spread pins); hardware gains of 15 orders of magnitude or more (every credible multiplier together is about 2.5 orders); or a re-check or run that overturns a closure (convex lift exact at 6x6; far stratum of frame cores below break-even; a non-tree solver's work growing by less than 0.75 of the tree's from 6x6 to 7x7; a learning solver's lead rising with size). Learning: a closure is worth publishing with its reopening conditions attached; 60 claims were struck or corrected during the arc, so every carried number now names a committed result. Reproducibility: not applicable (decision). (commit: pending) Files: research/program/night/2026-11-13/REPORT.md, research/program/night/2026-11-13/STATUS.md, research/program/AGENDA.md, research/program/EXPLORED.md.

## 2026-11-14 · Publication of the arc: evidence folders, twenty experiment folders, ten pages

The arc is published under the Raphaël Anjou experiments section. Evidence folders: `research/experiments/approach-pricing` (question, fixed numbers, routes index, desk studies with their re-checks, eight field surveys with citation and derivation re-checks, scripts, results), `research/experiments/relaxation-small-boards` (the relaxation line on toy boards), `research/experiments/learning-vs-propagation` (clause learning against propagation). Twenty experiment folders copied in, one commit each, with recipes under `just experiments`: mrv-node-cost, mrv-scaling-pilot, real-puzzle-profile, difference-map-bands, cut-keyed-replay, dead-node-census, quota-keying, five-clue-keying, five-clue-quota, blocker-backjump, value-impact-order, hall-on-mrv, baseline-tournament, partial-match-ladder, score-bound-gap, resumable-runs, frame-cores, orders-and-pins, prefix-statistics, pin-map (the existing single-core-benchmark folder gained the port-fidelity audit). Pages, in English, French and Spanish, under `web/content/research/lab/experiments/raphael-anjou/full-solve/`: index (the price of a full solution), price-of-a-full-search, approaches-priced-out, frame-and-corners, port-fidelity, late-filters, keying-experiments, search-engine-tracks, worth-of-a-known-tile, relaxations-on-small-boards; generic pages (approaches map, known facts, how hard is this instance, single-core benchmark, the experiments index) carry mentions and links. Withdrawn or corrected results keep their status on the pages; results on generated boards are labelled toy boards; unpublished boards and private statements are left out. Not published: the raw logs of the arc. Learning: raw run folders carry host-specific paths and a few large raw files, which were scrubbed or left out; two experiment ids were renamed to remove a banned word, so outside links to the old ids break. Reproducibility: exact for the ceiling table and the reprinted analyses; seeded or long-running for the runs, as each folder states. (commit: pending) Files: research/experiments/{approach-pricing,relaxation-small-boards,learning-vs-propagation}/, the twenty folders above, research/experiments/justfile, web/content/research/lab/experiments/raphael-anjou/full-solve/, web/content/research/lab/experiments/raphael-anjou/index.mdx, web/content/research/build/approaches-map.mdx, web/content/research/build/known-facts.mdx, web/content/research/why/how-hard-is-this-instance.mdx.
