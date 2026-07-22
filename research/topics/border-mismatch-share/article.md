---
id: border-mismatch-share
title: Where the last mismatches sit
summary: Across the nine bundled 469-class boards, 86.9% of the 99 remaining unmatched edges sit interior-to-interior, 13.1% on the border seam, and not one is border-to-border, so the border-balance check is blind to the large majority of what is still wrong.
status: published
created: 2026-07-22
updated: 2026-07-22
contributors:
  - name: Raphaël Anjou
    role: author
tags:
  - structure
  - search-space
sources:
  - label: "Eternity II board viewer (e2.bucas.name)"
    url: https://e2.bucas.name
reproduce:
  - cd research/topics/border-mismatch-share/compute && cargo run --release > ../results/mismatch-split.json
results:
  - label: Mismatch split per bundled high board (JSON, computed here)
    path: results/mismatch-split.json
site:
  render: true
  dataFile: results/mismatch-split.json
---

# Where the last mismatches sit

> **Created:** 2026-07-22 · **Updated:** 2026-07-22

## Summary

The [border-balance](/research/why/border-balance) invariant inspects one thing:
the seam between the outer ring of border pieces and the first ring of interior
pieces. On a near-solution, only some of the remaining errors land on that seam.
This recount takes every bundled high board and sorts each of its unmatched
internal edges into three exhaustive classes by which two cells the edge
separates. Across the nine 469-class boards, of 99 remaining unmatched edges,
**86.9% sit interior-to-interior**, **13.1% on the border seam**, and **none is
border-to-border**. The seam check the border-balance invariant runs cannot see
the large majority of what is still wrong.

## Method

A cell is a **border** cell if it sits on the outer perimeter of the 16×16 grid
(row or column 0 or 15), and an **interior** cell otherwise. Every internal edge
of the board separates exactly two cells, so it falls into exactly one of:

- **interior-interior**: both cells strictly inside the 14×14 core;
- **interior-border**: the seam the border-balance invariant inspects;
- **border-border**: between two adjacent pieces on the outer rim.

For each bundled board we read its `board_edges` string straight from the site's
own committed data (`web/src/data/known-boards.ts` and `record-boards.ts`), four
letters per cell in URDL order, and count every unmatched edge into its class.
The pass also recomputes each board's matched-edge score, which agrees with the
published score for every board. The computation is exact and deterministic.

## The 469-class

The nine bundled boards scoring exactly 469 carry 11 unmatched edges each, 99 in
total:

| Class | Edges | Share |
|---|---:|---:|
| interior-interior | 86 | 86.9% |
| interior-border (the seam) | 13 | 13.1% |
| border-border | 0 | 0.0% |

Not a single one of the 99 mismatches is border-to-border. The border ring on
these boards is already internally consistent; the errors that remain are almost
all buried in the interior, exactly where the seam check never looks.

## The broader high set

Widening to all nineteen bundled boards whose verified score is 464 or above
(the community five-clue record and up) keeps the same shape: of 231 unmatched
edges, 81.8% are interior-to-interior, 17.3% on the seam, and 0.9% (two edges
total) border-to-border. The interior share is the dominant term at every score
level in the set.

## Why it matters

This is the exact measurement behind the border-balance invariant's known
weakness. The invariant is a real, cheap necessary condition on the seam, but
a near-solution's remaining errors overwhelmingly do not live on the seam. They
live in the interior, where no border-only check can reach them, which is why
closing the border ring buys so little of the remaining distance to 480.
