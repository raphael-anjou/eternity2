---
id: sigma-cycles
title: Sigma-cycles, all pairs
summary: Across all 246 ordered pairs of same-piece-set bundled boards, every one of 54,238 partial cycle applications scores strictly worse than the board it started from.
status: published
created: 2026-07-22
updated: 2026-07-22
contributors:
  - name: Raphaël Anjou
    role: author
tags:
  - structure
  - local-search
sources:
  - label: "Peter McGavin's 469-record announcement (groups.io msg 10045, September 2020)"
    url: https://groups.io/g/eternity2/message/10045
  - label: "Eternity II board viewer (e2.bucas.name)"
    url: https://e2.bucas.name
reproduce:
  - cd research/topics/sigma-cycles/compute && cargo run --release > ../results/sigma-cycles.json
results:
  - label: Cycle structure and partial-application score curves for every same-piece-set board pair (JSON, computed here)
    path: results/sigma-cycles.json
  - label: The exact bundled boards used, extracted from web/src/data (JSON)
    path: compute/boards.json
site:
  render: false
---

# Sigma-cycles, all pairs

> **Created:** 2026-07-22 · **Updated:** 2026-07-22

## Summary

Take two complete Eternity II boards that use the same pieces. Because they place
the same pieces in different cells, one can be turned into the other by picking up
a set of pieces and shifting each into the cell the next one was using, around a
loop. Those loops are the disjoint cycles of the permutation that carries one
board's arrangement to the other. The question that matters for search is whether
any *partial* loop, applying only a prefix of a cycle, can raise the matched-edge
score above where it started. If it could, a step-by-step search could climb from
one strong board toward a better one.

We computed this over every ordered pair of bundled boards that share a piece set:
**246 pairs**, **1,154 large cycles**, **54,238 proper prefixes** scored. Not one
prefix scored as high as its start board. Every partial application, in every
pair, lost ground.

## Method

The bundled boards live in `web/src/data/known-boards.ts` (community records) and
`web/src/data/record-boards.ts` (this project's boards). We extract every complete
board into `compute/boards.json` and read it from a self-contained Rust crate.

A piece is identified from the board itself, not from any external id list: its
signature is the rotation-invariant form of its four edge colours (the smallest of
its four up-right-down-left rotations). Two boards "share a piece set" exactly when
their multisets of piece signatures match. Under the viewer's letter convention the
complete boards split into two such sets: the 13 boards written in the plain
`board_edges` alphabet (McGavin's 469, the three Riotte 464s, Blackwood 468, four
Verhaard 467s, and this project's 463 / 460 / 460 / 458), and 10 boards written in
the relabelled `motifs_order=jblackwood` alphabet (Blackwood 470 and eight 469s).
Both are the same physical puzzle; they differ only in how the colours are named,
so the permutation is computed inside each set separately.

For each ordered pair `(A, B)` in a set we build the position permutation, split it
into disjoint cycles, and for every cycle of at least 10 cells we score every
proper prefix. Applying the first `k` edges sets those `k` cells to the pieces `B`
places there, leaving the rest of `A` untouched, so a fully applied cycle
reproduces `B` on those cells. This is the exact mechanism the site's interactive
Sigma-Cycle Lab runs live. Scores are matched interior edges, the convention
[record-boards](record-boards) verifies. Everything is exact and deterministic.

## Results

| | Set 1 (`board_edges`) | Set 2 (`jblackwood`) | Total |
|---|---:|---:|---:|
| Boards | 13 | 10 | 23 |
| Ordered pairs | 156 | 90 | 246 |
| Large cycles (≥10 cells) | 710 | 444 | 1,154 |
| Proper prefixes scored | 33,868 | 20,370 | 54,238 |
| Prefixes reaching the start score | 0 | 0 | **0** |
| Pairs with any such prefix | 0 | 0 | **0** |

The property the page had measured on three hand-picked pairs holds across the
whole bundled population: **every proper prefix of every large cycle scores
strictly worse than the board it started from.** The closest any prefix ever came
was a single point below its start; the deepest single prefix fell 224 points.

Some structural detail worth keeping:

- The largest cycle in a pair ranges from 6 to 195 cells, with a median of 119.
  Two pairs (the two orderings of Riotte 464b and 464c) reduce to a *single* loop.
- The pairs to McGavin's 469 span one giant loop of up to 189 cells alongside a
  handful of shorter ones. Moving between a 464/467/468 board and the 469 shifts
  roughly 250 of the 256 cells.
- Two boards can also sit unusually close: this project's palimpsest-463 and
  McGavin's 469 differ in only 28 cells (7 small cycles, largest 10), yet even
  there no partial step between them improves the score.

## Why it matters

This is the second of the two escape routes out of a strong board. The
[rigidity wall](../../../web/content/research/why/rigidity-wall.mdx) closes the
local route: you cannot polish a good board into a better one. The sigma-cycle
result closes the jump route: the nearest better board is a single indivisible loop
of many cells away, and no prefix of that loop is an improving step a search could
follow. Measured on three pairs it was suggestive; measured on 246 pairs with zero
exceptions it is a population fact about the bundled boards.

## Reproduce

```sh
cd research/topics/sigma-cycles/compute
cargo run --release > ../results/sigma-cycles.json
```

Exact and deterministic; reproduces the committed `results/sigma-cycles.json`
byte-for-byte. The board list it reads, `compute/boards.json`, is extracted from
`web/src/data/known-boards.ts` and `web/src/data/record-boards.ts`.

## Notes

- The 10-cell threshold only governs which cycles get a full prefix scan reported;
  every cycle length is still recorded in each pair's structure summary.
- "Strictly worse" is measured against the start board `A` of each ordered pair, so
  each unordered pair is tested from both directions.
- The two piece-set groups are an artifact of the viewer's two colour alphabets,
  not two different puzzles. Within each alphabet the permutation is well defined;
  across alphabets the same physical piece carries a different signature, so no
  cross-alphabet pair is formed.
