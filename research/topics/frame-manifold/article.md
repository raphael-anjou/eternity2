---
id: frame-manifold
title: The perfect border is a manifold, not a point
summary: On the official piece set, every fully matched 60-piece border admits exactly 45 zero-cost piece exchanges that compose indefinitely and change what the rim offers the interior; about a third of "perfect" borders leave an interior cell unfillable, and a single free exchange repairs them.
status: draft
created: 2026-07-22
updated: 2026-07-22
contributors:
  - name: Raphael Anjou
    role: author
credits:
  - The rim-swap idea (exchange a border piece instead of backtracking) that led to this measurement.
tags:
  - border
  - structure
  - search
sources: []
reproduce:
  - cd research/topics/frame-manifold/compute && cargo run --release > ../results/manifold.json
results: []
site:
  render: false
  dataFile: null
---

# The perfect border is a manifold, not a point

> **Status:** draft · **Created:** 2026-07-22 · **Updated:** 2026-07-22
> **Authors:** Raphael Anjou

## Summary

Border-first strategies build a fully matched 60-piece border (all 60
border-to-border adjacencies matched, BB = 60), freeze it, and treat the 56
inward-facing colours as hard constraints on the interior search. The premise
under that design is that a perfect border is a single object you commit to.

It is not. Enumerating every legal exchange of two same-class border pieces
(corner with corner, edge piece with edge piece, grey kept exactly outward) on
real BB = 60 frames shows a fixed structure: no exchange ever raises BB above
60, and exactly 45 exchanges are free, meaning BB stays at 60. The 45 free
exchanges compose: after 100 chained free swaps BB is still 60 and 45 free
moves are still available, the count never depletes. The moves are not
cosmetic: none of the 45 preserves the inward colours, so every free move
changes the constraint vector the border presents to the interior. A perfect
border is therefore a connected family of equally perfect borders, each
showing the interior a different face.

This matters because BB = 60 says nothing about whether the interior can
attach. In a survey of 500 independently generated perfect frames, 36 percent
had at least one interior cell that zero of the 196 interior pieces could
fill, and 11 percent were dead at the very first interior cell a scan-order
solver visits, so the search dies at depth 0 despite a perfect border score.
Every sampled dead frame was repaired to zero dead cells by free exchanges
alone, at no border cost. A perfect border and a usable border are different
properties, and the fix for the gap between them is free.

What this does not establish: no claim is made here that walking the manifold
improves final board scores. The measured facts are the exchange cost
distribution, the 45 free moves, their composition, the dead-frame rate, and
the zero-cost revival. Whether revival or walking raises completed-board
scores is an open question and is out of scope for this topic.

## Reproduction

The compute crate builds BB = 60 frames of the official 256-piece set from
scratch (a randomised depth-first placement over the 60 border cells, grey
outward, all ring adjacencies matched), then measures each claim directly:

1. enumerate all legal same-class pair exchanges on each frame and report the
   BB-delta distribution, checking that no exchange gains and counting the
   free ones;
2. chain free exchanges (recomputing the free set on the current ring each
   step) and report BB, the free-move count, and how many border slots and
   inward colours differ from the start;
3. for each frame, test every interior cell that borders the frame for
   fillability by the 196 interior pieces, and report the dead-frame and
   dead-at-first-cell rates over a batch of frames;
4. for each dead frame, attempt a greedy repair using free exchanges only and
   report the revival rate.

See `compute/PLAN.md` for the exact claim numbers, the scoring-convention
mapping, and the scale-faithfulness argument, and `compute/src/main.rs` for
the implementation.

## Notes / caveats

The original frames came from one specific frame generator. This reproduction
generates its own frames, which doubles as the robustness check the original
finding called for: the free-move count of 45 is claimed as a property of the
piece set and should replicate, while the 36 percent dead-frame rate is a
property of the generator's sampling and may shift.
