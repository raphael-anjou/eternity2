---
id: subgrid-placement-counts
title: Sub-grid placement counts (validation reference table)
summary: Exact counts of valid piece placements for small blocks of the official Eternity II board, for checking your own solver.
status: published
created: 2026-06-21
updated: 2026-06-21
contributors:
  - name: Raphaël Anjou
    role: author
credits:
  - Requested by sylvogel on the Eternity II groups.io forum (message 11879).
tags:
  - enumeration
  - reference
  - newcomer
sources:
  - label: sylvogel's request (Eternity II groups.io, message 11879)
    url: https://groups.io/g/eternity2/message/11879
reproduce:
  - cd research/topics/subgrid-placement-counts/compute && cargo run --release > ../results/reference-table.json
results:
  - label: Reference table (JSON, computed here)
    path: results/reference-table.json
  - label: Published fallbacks (sylvogel's figures for intractable cells)
    path: results/published-reference.json
site:
  render: true
  dataFile: results/reference-table.json
---

# Sub-grid placement counts (validation reference table)

> **Status:** published · **Created:** 2026-06-21
> **Author:** Raphaël Anjou · **Credit:** requested by *sylvogel* on the groups.io forum.

## Summary

When you start writing an Eternity II solver, the first thing you want is a set
of **known-good numbers** to check your edge-matching and constraint code
against. This article gives exact counts of how many valid ways a small block
(2×2, 3×3, 4×4) can be filled at a given board position, under increasingly
constrained rules. Every number here is produced by a small, reproducible
enumerator over the official piece set.

## Background

The official Eternity II puzzle is a 16×16 board with 256 pieces. Four pieces are
**corners** (two grey border edges), 56 are **edge** pieces (one grey edge), and
196 are **interior** pieces (no grey edge). Five pieces are fixed **clues**: one
in the centre and four near-corner pieces.

A common early sanity check is: *"in isolation, how many ways can I legally fill
this little block?"* The community member **sylvogel** proposed exactly such a
reference table on the groups.io forum. We reproduced his numbers from scratch
with our engine — both to validate the table and to publish a version anyone can
re-run.

## Method

For a $w\times h$ block at a fixed board position we count the ways to fill it
with **distinct** official pieces such that:

1. **Piece class matches the cell.** Each cell draws only from the piece class
   matching its number of border-facing sides — a cell with two board-rim sides
   takes a corner piece, one rim side an edge piece, no rim side an interior
   piece. This mirrors how a real solver pools pieces.
2. **Border edges are grey.** Every edge facing the board's outer rim is grey
   (colour $0$); every block-boundary edge facing the board *interior* is a real
   interior colour (non-grey).
3. **Internal edges match.** Shared edges between adjacent cells of the block are
   equal.

The columns differ only in which pieces are available:

| Column | Rule |
|---|---|
| **Empty** (*Vide*) | Full piece pool. |
| **Fixed** (*Fixe*) | The centre clue piece is removed from the pool. |
| **Fixed + 4 hints** | All five clue pieces removed; any clue whose official position falls inside the block is pinned there at its official rotation. |
| **c0 / c1 / c2 / c3** | *Corner blocks only.* Of the "Fixed + 4 hints" fillings, how many place corner piece $N$ in the block's board-corner cell. The four official corner pieces are labelled c0–c3. The four columns partition the "Fixed + 4 hints" count exactly. |

These definitions reproduce sylvogel's published numbers exactly (all anchor
values matched: 2×2 corner = 1312, 2×2 side = 73 003, 2×2 middle = 4 059 952,
3×3 corner = 2 633 221, and every per-corner cN split).

## Results

The machine-readable table is [`results/reference-table.json`](results/reference-table.json).
The site renders it as an interactive table. A few anchor values:

| Block | Empty | Fixed | Fixed + 4 hints |
|---|---:|---:|---:|
| 2×2 corner | 1 312 | 1 307 | 1 291 |
| 2×2 side | 73 003 | 72 365 | 69 531 |
| 2×2 middle | 4 059 952 | 3 980 912 | 3 649 320 |
| 3×3 corner (TL) | 2 633 221 | 2 582 369 | 3 084 |
| 4×4 corner (TL) | 29 775 113 571 | 28 535 440 805 | 33 845 721 |

(The "Fixed + 4 hints" count drops sharply for 3×3+ corner blocks because the
near-corner clue then sits *inside* the block and is pinned, fixing one cell.)

## Reproduce

```sh
cd research/topics/subgrid-placement-counts/compute
cargo run --release > ../results/reference-table.json
```

The output is deterministic and reproduces the committed JSON byte-for-byte, and
the whole table runs in well under a minute on 8 cores.

## How the counts are produced

Every count is **exact**, by one of two methods:

- A **broken-profile colour transfer matrix** gives the *non-distinct* count (one
  piece may be reused) instantly for any block — it is the exact upper bound on the
  work of the next step.
- A **parallel distinct-counting depth-first search** then enumerates the fillings
  with *distinct* pieces: fill the block cell by cell, trying only candidates whose
  edges match the placed neighbours and rejecting any piece already used in the
  block. It visits about one node per valid partial filling, so its cost tracks the
  non-distinct count above.

## Notes / caveats

- **Exactly-enumerable scope.** The distinct DFS is run only when the block's
  non-distinct count is at most ~5×10⁹ fillings, which keeps the whole table well
  under a minute. The few interior columns above that — the **3×3 middle**, the
  **empty/fixed** columns of the **4×4 corners**, and the whole **4×4 side**
  (whose counts run from ~10¹⁰ up into the tens of trillions) — are emitted as
  `null` with a `note` by the generator. Everything else, including all 4×4
  *fixed + 4 hints* columns and every per-corner cN split, is computed exactly
  here.
- **Published fallbacks (shown in italics on the site).** So the table reads
  complete, the cells the generator leaves `null` are filled with sylvogel's
  published figures, kept in a separate file
  (`results/published-reference.json`) and rendered *in italics* on the site to
  mark them as published-but-not-independently-recomputed. They are never mixed
  into the generated `results/reference-table.json`.
- "Distinct pieces" is enforced *within the block only* — this is a local sanity
  check, not a statement about full-board completability.
