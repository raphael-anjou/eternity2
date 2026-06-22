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

### 2026-06-22 — Set up this ledger; refined the writing voice

Created `research/LEDGER.md` (this file) as the cross-session memory for the
multi-day build, and backfilled it oldest-first. Refined the writing voice: the
first-person researcher journal voice ("we tried this and it failed") is welcome
and professional; what we avoid is exposing the automated process. Warmed the
dead-ends copy accordingly.

- Commit `<this commit>`.
- Files: `research/LEDGER.md`, `AGENTS.md`, `web/src/pages/research/build/dead-ends.tsx`.
