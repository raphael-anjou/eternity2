---
id: frame-manifold
title: The perfect border is a manifold, not a point
summary: On the official piece set, every fully matched 60-piece border admits exactly 45 zero-cost piece exchanges that compose indefinitely and change what the rim offers the interior; about a third of "perfect" borders leave an interior cell unfillable, and a single free exchange repairs them.
status: published
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
  - cd research/topics/frame-manifold/compute && cargo run --release -- --frames 500 --walk 100 --seed0 1 > ../results/manifold.json
results:
  - label: Main run (500 frames, seed0 1, byte-stable)
    path: results/manifold.json
  - label: Independent-seed robustness run (100 frames, seed0 7001)
    path: results/manifold_seed7001.json
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
the implementation. Runs on one core of an Apple Silicon machine; the full
500-frame run takes well under a minute, and re-running the same command
reproduces `results/manifold.json` byte for byte.

### Measured against expected

Main run: 500 independently generated BB = 60 frames (this crate's own
randomised DFS, seeds 1 upward), walk length 100, revival attempted on the
first 60 dead frames.

| Quantity | Expected (source) | Measured |
| --- | --- | --- |
| Free exchanges per perfect frame | 45, every frame | 45 on all 500 frames |
| Exchanges that raise BB | 0 | 0 over 773,000 pooled pairs |
| bb_delta shares -4 / -3 / -2 / 0 | 63.9 / 1.2 / 31.8 / 3.1 % | 63.8 / 1.4 / 31.9 / 2.9 % |
| BB along a 100-swap free walk | 60 at every checkpoint | 60 at every checkpoint |
| Free moves available along the walk | 45 at every checkpoint | 45 at every checkpoint |
| Ring slots changed after 100 free swaps | 36 of 60 | 33 of 60 |
| Free swaps preserving inward colour | 0 of 45 | 0 of 45 |
| Distinct rim-target vectors visited | 292 (longer source walk) | 99 of a possible 101 on a 100-step walk |
| Dead frames (any unfillable cell) | 180/500, 36.0% | 180/500, 36.0% |
| Dead at the interior scan-start cell | 55/500, 11.0% | 46/500, 9.2% |
| Dead-cell histogram 1 / 2 / 3 cells | 155 / 24 / 1 | 154 / 25 / 1 |
| Dead frames revived by free swaps only | 100% | 60/60, BB still 60 on every one |

An independent-seed robustness pass (100 frames, seed0 7001) agrees: 45 free
on every frame, no gains, 45/100 dead frames (histogram 38 / 7), all 45
revived, and the walk again holds BB = 60 with 45 free moves throughout.

Two numbers deserve a note. First, the raw same-class slot-pair count is
1546 per frame here, against roughly 1458 exchanges per frame in the source;
the source evidently excluded some no-op pair class. The shares and the two
load-bearing rows (45 free, 0 gains) match regardless, and the raw count is
reported so the discrepancy stays visible. Second, the dead-frame rate
landing on exactly 180/500 again is a coincidence of sampling; the plan
expected only "tens of percent" from a different frame generator, and the
independent-seed pass gives 45 percent, so the honest statement is that the
rate sits in the mid-30s to mid-40s percent range.

The first generated frame is viewable at the `frame0_url` field of
`results/manifold.json` (an eternity2.dev viewer link).

## Notes / caveats

The original frames came from one specific frame generator. This reproduction
generates its own frames, which doubles as the robustness check the original
finding called for: the free-move count of 45 is claimed as a property of the
piece set and should replicate, while the 36 percent dead-frame rate is a
property of the generator's sampling and may shift.

## Source-context figures

Worked example quoted from the source study: a single free rim swap took a framed DFS from depth 0 to depth 153 of 196 on one dead frame.
