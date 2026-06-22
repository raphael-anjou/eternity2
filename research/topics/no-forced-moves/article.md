---
id: no-forced-moves
title: No forced moves
summary: Every interior piece has 73 to 137 possible right-hand neighbours; not one is ever pinned to a single option.
status: published
created: 2026-06-22
updated: 2026-06-22
contributors:
  - name: Raphaël Anjou
    role: author
tags:
  - structure
sources:
  - label: "Eternity II board viewer (e2.bucas.name)"
    url: https://e2.bucas.name
reproduce:
  - cd research/topics/no-forced-moves/compute && cargo run --release > ../results/partner-counts.json
results:
  - label: Partner-count distribution (JSON, computed here)
    path: results/partner-counts.json
site:
  render: true
  dataFile: results/partner-counts.json
---

# No forced moves

> **Created:** 2026-06-22 · **Updated:** 2026-06-22

## Summary

A reliable way to crack a logic puzzle is to find a spot where only one piece can
go, place it for free, and repeat until the answer falls out. That lever does not
exist in Eternity II's interior. Every one of the 196 interior pieces has between
73 and 137 other interior pieces that could legally sit beside it. Not a single
piece is ever down to one option.

## Background

This is the flip side of [forbidden patterns](forbidden-patterns). Forbidden
patterns shows that most *combinations* of pieces are impossible. You might expect
that to make the puzzle easy: surely all those constraints pin pieces into place.
They don't. The constraints rule out combinations without ever cornering an
individual piece, so a solver never gets a free, forced move to build on.

## Method

We look at the 196 interior pieces (the ones with no grey border edge). For each
piece we count how many of the others could sit immediately to its right: its
right-edge color matching their left-edge color, allowing any rotation of either
piece. That count is the number of "right-hand partners" the piece has. We do this
exactly for every piece, with no sampling.

## Results

| | Partners |
|---|---:|
| Fewest (any piece) | 73 |
| Median | 125 |
| Mean | 119 |
| Most | 137 |
| **Pieces with only one option** | **0** |

Every interior piece has dozens of legal neighbours, and the typical piece has
well over a hundred. The least flexible piece in the whole set still has 73.
Nothing is forced.

## Why it matters

Put this next to forbidden patterns and you get the real shape of the difficulty.
Locally, the puzzle looks loose: any piece fits next to plenty of others, so
there's no constraint to propagate, no domino-chain of forced moves to ride.
Globally, almost every way of combining pieces is illegal. The hardness lives in
the gap between those two facts: lots of local freedom, almost no global
consistency. A solver has to make a long run of free-looking choices that only
reveal themselves as wrong much later. That's the worst kind of search problem to
be in.

## Reproduce

```sh
cd research/topics/no-forced-moves/compute
cargo run --release > ../results/partner-counts.json
```

Exact and deterministic; reproduces the committed `results/partner-counts.json`
byte-for-byte.

## Notes

- We count right-hand partners as a representative direction; the picture is the
  same for any direction, because the piece set has no directional bias (see
  [forbidden patterns](forbidden-patterns), where the horizontal and vertical
  pair counts come out identical).
