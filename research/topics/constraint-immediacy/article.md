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
  - cd research/topics/constraint-immediacy/compute && cargo run --release -- korder
  - cd research/topics/constraint-immediacy/compute && cargo run --release -- solve 5
results: []
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

Two parts, both in `compute/`.

First, the invariance itself is checked exactly: for each of five visit orders
(row-major, boustrophedon, outer spiral, border-first, hint-link) the program
computes the per-cell constraint counts $k_i$, prints their histogram, and
verifies that $\sum_i k_i = 480$ on the official 16x16 instance. This part is
deterministic and runs in milliseconds.

Second, the behavioural ranking is re-measured with one fixed-order solver run
under each visit order at an equal per-order time budget: a greedy best-fit
pass (full board, breaks allowed, matched edges scored by the canonical
rim-excluding scorer) and a perfect-fit depth-first search with chronological
backtracking (deepest consistent prefix). The claim under test is the ordering
border-first > row-major > spiral > hint-link, not the exact historical
numbers, which came from a differently configured engine. See
`compute/PLAN.md` for the design, the scoring-convention mapping, and the
scale-faithfulness argument.
