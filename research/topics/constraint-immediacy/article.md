---
id: constraint-immediacy
title: The constraint-immediacy principle
summary: Total constraint volume is path-invariant across visit orders; search efficiency comes from minimizing the distance between a decision and its refutation, which explains why border-first ordering beats row-major and why hint-link and spiral orders collapse.
status: draft
created: 2026-07-22
updated: 2026-07-22
contributors:
  - name: Raphael Anjou
    role: author
credits:
  - Question posed during project Q&A about whether hint-linking paths "restrict the puzzle earlier".
tags:
  - search
  - visit-order
  - theory
sources: []
reproduce:
  - cd research/topics/constraint-immediacy/compute && cargo run --release -- repro-korder ../results
  - cd research/topics/constraint-immediacy/compute && cargo run --release -- repro-official 60 ../results
  - cd research/topics/constraint-immediacy/compute && cargo run --release -- repro-gen 60 1,2,3,4 ../results
results:
  - label: k-order invariance (exact)
    path: results/korder-invariance.json
  - label: solver ranking, official board
    path: results/solver-ranking-official.json
  - label: solver ranking, generated-board variance
    path: results/solver-ranking-generated.json
site:
  render: false
  dataFile: null
---

# The constraint-immediacy principle

> **Status:** draft · **Created:** 2026-07-22 · **Updated:** 2026-07-22
> **Authors:** Raphael Anjou

## Summary

For any complete visit order of the board, let $k_i$ be the number of already
placed neighbours cell $i$ faces at the moment it is filled. Every interior edge
is checked exactly once, by whichever of its two endpoints is placed second, so
$\sum_i k_i$ equals the number of interior edges for every visit order. On the
16x16 board this sum is 480, no matter the path. A visit order therefore cannot
add restriction; it can only schedule when each restriction binds.

What distinguishes orders in practice is immediacy: the cost of a wrong
placement is the size of the subtree explored before its refutation surfaces.
Orders with under-constrained stretches (long runs of cells that see only one
placed neighbour) followed by over-constrained closures (cells that see three or
four) are fail-last and maximally expensive. Orders that keep the
decision-to-refutation distance near zero are cheap. Measured on this project's
fixed-order search: a hint-linking path reached 51 of 480 matched edges, an
outer spiral 204, a two-front seam order scored 3 to 5 edges below row-major,
row-major reached 433, and border-first, which commits the most constrained
sub-pool first so its restrictions bind immediately, reached 445.

The two extremes carry the lesson. Border-first shows that restricting early is
right exactly when the restriction tests each decision at once. The hint-link
path shows that geometric earliness without immediacy is worse than doing
nothing special. Since a uniform two-constraint profile is the best a geometry
can do (high-k cells require low-k cells laid first, by the conservation law
above), any further early binding has to be informational rather than
geometric: requirement propagation, priors, pool restrictions, or computed
gates.

## Reproduction

Two parts, both in `compute/`. Runs were made on Apple Silicon, single core.
Scores are matched interior edges out of 480 under the canonical rim-excluding
scorer.

**Part 1, the invariance (exact tier): reproduced.** For each of five visit
orders the program computes the per-cell constraint counts $k_i$ and verifies
$\sum_i k_i = 480$ on the official 16x16 instance. All five orders sum to
exactly 480 with the predicted histogram shapes
(`results/korder-invariance.json`, byte-stable on rerun):

| order | k=0 | k=1 | k=2 | k=3 | k=4 | sum |
|---|---|---|---|---|---|---|
| hint-link | 1 | 75 | 137 | 41 | 2 | 480 |
| outer-spiral | 1 | 58 | 170 | 26 | 1 | 480 |
| row-major | 1 | 30 | 225 | 0 | 0 | 480 |
| boustrophedon | 1 | 30 | 225 | 0 | 0 | 480 |
| border-first | 1 | 58 | 170 | 26 | 1 | 480 |

**Part 2, the behavioural ranking (seeded-statistical tier): only partially
reproduced.** Each visit order ran two fixed-order arms at 60 s per order per
arm on the official instance: a greedy best-fit pass (full board, breaks
allowed) and a perfect-fit depth-first search with chronological backtracking
(deepest consistent prefix; 16 to 47 billion nodes per order, so the budget was
genuinely spent). Historical numbers came from the research engine with its own
candidate ordering and restart policy; this kit solver is deliberately plain.

| order | historical (engine) | greedy here | DFS score here | DFS depth |
|---|---|---|---|---|
| hint-link | 51 | 316 | 44 | 60/256 |
| outer-spiral | 204 | 366 | 28 | 35/256 |
| row-major | 433 | 343 | 344 | 194/256 |
| boustrophedon | about row-major | 359 | 342 | 193/256 |
| border-first | 445 | 358 | 28 | 35/256 |

What reproduces: hint-link is by far the worst perfect-fit order, and its DFS
score of 44 lands close to the historical 51; row-major and boustrophedon are
the solid mid-field in both arms; and on the official board the greedy arm
keeps border-first ahead of row-major (358 vs 343), matching the direction of
the historical 445 vs 433.

What does not reproduce: the full ordering. Under this solver the outer spiral
does not collapse to the 204 class (it ties border-first, consistent with the
fact that the two orders share an identical k-profile and an identical first 60
cells here), and under perfect-fit DFS border-first hits a rim-closure depth
wall at 35 of 256 instead of leading. A generated-board supplement (4 framed
16x16, 22-colour seeds, top-2 orders, same budgets) reverses the greedy
border-first advantage: row-major wins the greedy arm on 4 of 4 seeds and the
DFS arm on 3 of 4, with border-first DFS varying wildly across seeds (28 to
366). The immediacy extremes are robust; the fine-grained middle of the
historical table is a property of that engine's search policy, not of the
visit orders alone. Files: `results/solver-ranking-official.json` (includes
board URLs), `results/solver-ranking-generated.json`.

See `compute/PLAN.md` for the design, the scoring-convention mapping, and the
scale-faithfulness argument.
