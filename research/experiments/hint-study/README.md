# Hint study

A study of **hint placement** for Eternity II. The question, one axis deeper than
the usual "how many hints" and "where": *given the same hints, how much are they
worth under different fill orders?* The answer is that the fill order dominates —
the same five hints are worth several times more under a compact sweep than under
an order that chases the hints — and that the reason is a single computable
quantity, the open frontier.

Everything here is **from scratch**: our own parametric board generator, our own
family of fill-path backtrackers (the DFS study's engine), our own canonical
scorer. No community board, puzzle, or engine is used; only the *shape* of
Eternity II's five-clue arrangement is borrowed, as a geometry to test.

## The three axes

1. **Path × placement** — fix the board and the hints (the five-clue shape),
   change only the fill order. The dominant lever; the headline result.
2. **Count, floor-corrected** — spread vs clustered ladders. Raw score is
   confounded by the *pinned-seam floor* (seams both of whose endpoints are
   pinned score for free); the honest metrics are solved-rate and reached depth.
3. **The clue shape** — Eternity II's own five-clue geometry on our boards, and
   which fill orders exploit or waste it.

## Layout

```
../common/              shared central lib (e2-core / e2-io), also used by the
                        DFS and repair studies — one canonical scorer for all.
                        e2-core now selects the board size by a `size-N` cargo
                        feature (default 16) for the scaling axis, at zero
                        runtime cost.
../../../engine/        the site engine; `hint_variants` (the parametric board +
                        hint-geometry generator) lives here as a binary.
../dfs-study/engine/    the fill-path backtracker (run_dfs); one new path,
                        `connect-hints-first`, is the study's negative control.
../single-core-benchmark/engine/  the beam solver (producer_trie), the
                        non-backtracker contrast.
scripts/
  run_grid.py           generate variants x run every (path, layout, seed) ->
                        results/rerun/results.jsonl  (parallel, 1 thread per run)
  make_site_json.py     results.jsonl -> web/src/data/hint-study.json
results/                committed run outputs
```

## Reproduce

From the repo root:

```
just experiments hint-study                      # 15 seeds, 8 s/run, 6-wide
just experiments hint-study budget_s=8 seeds=15 parallel=6
```

This builds the generator and the two solvers, runs the grid single-core per run,
and regenerates the site JSON the study pages read. The generator is parametric in
board size (`--size`) and colour count (`--colors`, defaulting to a
density-preserving recipe faithful to Eternity II at every size); see the study's
method page for the colour-census and frontier arithmetic.

## What every number means

Every board is re-scored by the one canonical matched-edge scorer that never
counts a border-facing (grey) seam. The maximum is `2·n·(n−1)` (480 at 16×16).
Each seed is a distinct generated instance, so across-seed spread is genuine
instance variance and is reported with every claim. The peak open frontier on the
path axis is computed from the fill order's geometry alone — no solver — and
predicts the measured score almost perfectly (r ≈ −0.95): that is the mechanism
the whole study turns on.
