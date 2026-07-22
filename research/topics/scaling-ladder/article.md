---
id: scaling-ladder
title: The scaling ladder
summary: A ladder of planted, solvable N by N instances with a known full-solve target of 2N(N-1) separates solver families by the largest N they still fully solve; propagating CSP-style methods hold a perfect score through N=12, border-first orderings break one rung earlier, and every method collapses at N=14.
status: published
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
  - cd research/topics/scaling-ladder/compute && for s in 8 10 12 14; do cargo run --release --features size-$s -- --seeds 1,2,3,4,5,6,7,8 --budget-s 12; done > ../results/ladder.jsonl
  - cd research/topics/scaling-ladder/compute && python3 summarize.py ../results/ladder.jsonl ../results/summary.json
results:
  - label: Ladder rows (JSONL, one row per solver x instance x seed)
    path: results/ladder.jsonl
  - label: Per (solver, N) summary (gap spread over 8 seeds, best board URLs)
    path: results/summary.json
  - label: Rung census (ceiling, duplicate pieces, planted board URL per rung)
    path: results/rungs.txt
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
breaks a full rung earlier, collapsing to roughly 0.15 to 0.22 at N=12. At
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

The run below is 4 rungs x 8 seeds x 2 solvers at a 12 second single-core
budget per cell (Apple Silicon, one core). All 32 planted boards re-scored to
exactly their 2N(N-1) ceiling before any solver ran, and all 64 returned
boards passed the independent re-score with the reported maximum equal to the
ceiling on every row, reproducing the verification-discipline claim at this
scale. Measured against the expected numbers:

| Quantity | Expected (vol-240) | Measured (this run) |
| --- | --- | --- |
| Ceilings N=8/10/12/14 | 112 / 180 / 264 / 364 | 112 / 180 / 264 / 364, planted board certified 32/32 |
| Independent re-score | 429/429 verified | 64/64 verified, 0 mismatches |
| DFS at N=8 | full solve well under 1 s | full solve 8/8 seeds (7 k to 8.7 M nodes) |
| DFS at N=10 | (not in source grid) | full solve 7/8 seeds; the eighth stalls at 0.283 |
| DFS at N=12 | 0.489 in a 5 s smoke run | median 0.512, one lucky full solve, min 0.216 |
| DFS at N=14 | 0.217 in a 5 s smoke run; registry collapse 0.055 to 0.157 | median 0.477, best 0.613, two seeds stuck at 0.047 to 0.049 |
| Greedy at N=8 | 0.313 | median 0.321 (0.295 to 0.366) |
| Duplicate pieces | 0 at N=8, up to 4 at N=12, 10 to 11 at N=14 | 0 on every rung (kit generator) |

The ladder's core mechanics reproduce: the free proven ceiling, the
certification of the planted board through the same scorer every solver faces,
the independent re-score of every returned board, and the monotone
degradation of the propagating baseline from perfect at N=8 to no full solve
at N=14 while the greedy baseline never full-solves anywhere. Two expected
numbers do not transfer, and both were flagged in advance as
instance-profile-bound: the depth of the N=14 collapse (the original grid
bottomed out at 0.157; kit-generated rungs allow partial boards near 0.6
because the kit generator produces zero duplicate pieces and does not
implement the rare-colour ring construction) and any per-cell equality with
the original 13-method table, which needs the committed vol-240 instance set
and the ported registry. The seed-to-seed spread at N=12 and N=14 is wide
(0.216 to 1.0 at N=12, 0.047 to 0.613 at N=14 for the DFS) and is the reason
the grid keeps 8 seeds per rung.
