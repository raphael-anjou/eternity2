---
id: scaling-ladder
title: The scaling ladder
summary: A ladder of planted, solvable N by N instances with a known full-solve target of 2N(N-1) separates solver families by the largest N they still fully solve; propagating CSP-style methods hold a perfect score through N=12, border-first orderings break one rung earlier, and every method collapses at N=14.
status: draft
created: 2026-07-22
updated: 2026-07-22
contributors:
  - name: Raphael Anjou
    role: author
tags:
  - infrastructure
  - scaling
  - benchmarks
sources:
  - label: "Ansotegui et al., phase transition in edge-matching puzzles"
    url: https://www.iiia.csic.es/~jordi/
reproduce:
  - cd research/topics/scaling-ladder/compute && for s in 8 10 12 14; do cargo run --release --features size-$s -- --seeds 1,2,3 --budget-s 12; done > ../results/ladder.jsonl
results:
  - label: Ladder rows (JSONL, one row per solver x instance x seed)
    path: results/ladder.jsonl
site:
  render: false
  dataFile: null
---

# The scaling ladder

> **Status:** draft · **Created:** 2026-07-22 · **Updated:** 2026-07-22
> **Authors:** Raphael Anjou

## Summary

Every Eternity II style instance built backwards from a planted solution is
solvable by design, and its full-solve target is known for free: an N by N grid
has exactly 2N(N-1) internal adjacencies, the planted board matches all of
them, and no board can match more. That gives a ladder of smaller puzzles
(N = 8, 10, 12, 14) on which any solver can be run unchanged, each rung
carrying a proven ceiling that needs no external solver to certify. The useful
readout is not any single pass or fail but the shape of each method's
score-over-ceiling curve as N grows: the method whose curve stays highest is
the one most worth scaling toward the real 16 by 16.

Measured over thirteen methods on instances matched to the real puzzle's
colour profile (22 interior colours, 5 rare colours confined to the
frame-adjacent ring), the ladder cleanly separates two families. The
propagating, constraint-driven family holds a perfect 1.000 ratio all the way
through N=12 (264 of 264 matched edges). The border-first ordering family
breaks a full rung earlier, collapsing to roughly 0.18 to 0.22 at N=12. At
N=14 every method drops to a ratio of 0.05 to 0.16, independently reproducing
a hardness threshold at N=14 that had been documented before on entirely
different instances.

Two cautions travel with the result. First, an independent re-score of every
returned board is part of the harness, not an afterthought: the score used is
always a recomputation from the board's edges, never the solver's self-report.
Second, small-N success is not a predictor of 16 by 16 performance: the very
methods that are perfect at N=12 are near worthless at N=14, so the
deliverable is the degradation curve, not a ranking at any one size.

## Reproduction

The reproduction builds the ladder from the kit's seeded generator: for each
(N, seed) a solvable framed instance is generated, five solution cells are
pinned as clues in the manner of the official puzzle, and the planted board is
re-scored to assert it reaches the 2N(N-1) ceiling before any solver runs.
Because the shared engine fixes the board size at compile time, one binary is
built per rung (`--features size-8` through `size-14`, exactly the four rungs
the original study used) and the four outputs concatenate into one result
file. Each solver runs on each instance under a fixed single-core wall budget,
and every returned board is re-scored through the canonical rim-excluding
scorer. One JSON row per (solver, N, seed) records the verified score, the
ceiling, their ratio, whether the solve was full, the node count, and the
viewer URL of the board.

The compute crate ships two baseline solvers spanning the two families the
finding separates: a backtracking depth-first search that only places fully
matching pieces (the propagating family's simplest member) and a greedy
row-major filler with no backtracking (the naive family). The original
thirteen-method registry is not yet ported to the kit; the plan in
`compute/PLAN.md` states which numbers the two baselines can and cannot be
expected to reproduce, and how the original committed instance set can be
substituted for the kit-generated ladder for a fully faithful re-run.
