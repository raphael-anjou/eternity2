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

### 2026-06-22 — Set up this ledger; refined the writing voice

Created `research/LEDGER.md` (this file) as the cross-session memory for the
multi-day build, and backfilled it oldest-first. Refined the writing voice: the
first-person researcher journal voice ("we tried this and it failed") is welcome
and professional; what we avoid is exposing the automated process. Warmed the
dead-ends copy accordingly.

- Commit `<this commit>`.
- Files: `research/LEDGER.md`, `AGENTS.md`, `web/src/pages/research/build/dead-ends.tsx`.
