---
id: frame-is-not-the-basin
title: The frame is not the basin selector
summary: A different strong border was supposed to open a different high interior, but generating many distinct fully-matched borders and completing each with one fixed interior producer gives tops that are near-maximally distinct and uniformly LOW, none within reach of the record band; the border diversifies the basin without predicting its ceiling.
status: draft
created: 2026-07-23
updated: 2026-07-23
contributors:
  - name: Raphael Anjou
    role: author
tags:
  - border
  - structure
  - search
sources: []
reproduce:
  - cd research/topics/frame-is-not-the-basin/compute && cargo run --release -- --frames 16 --beam 128 --seeds 3 --seed0 1 > ../results/frame_first.json
  - cd research/topics/frame-is-not-the-basin/compute && cargo run --release -- --frames 12 --beam 128 --seeds 2 --seed0 7001 > ../results/frame_first_seed7001.json
results:
  - label: Main run (16 distinct frames, beam 128, 3 seeds, seed0 1)
    path: results/frame_first.json
  - label: Independent-seed robustness run (12 frames, seed0 7001)
    path: results/frame_first_seed7001.json
site:
  render: false
  dataFile: null
---

# The frame is not the basin selector

> **Status:** draft · **Created:** 2026-07-23 · **Updated:** 2026-07-23
> **Authors:** Raphael Anjou

## Summary

The border ring is the most constrained part of the puzzle: 60 cells over only
five ring colours, filled by the four corner pieces and the 56 edge pieces. A
natural hope follows from that: if a fully matched border is what pins the top
of the board, then a *different* fully matched border should pin a *different*
high interior, and building several distinct strong borders would open several
distinct high basins. Border-first diversification would then be a lever for
scores.

It is not. Generating many structurally distinct fully-matched borders and
completing each one with the same fixed interior producer gives tops that are
near-maximally different from each other and yet all land in the same low band,
nowhere near the record region. The border is a strong *diversifier* of the
resulting board and a useless *selector* of its quality. Frame identity does not
predict the interior ceiling.

This is a negative result, and it matters because it closes a plausible door.
Where the high-scoring information actually lives is the interior of the top
rows, not the ring: the ring couples to the interior only through a
low-information boundary word and under-determines a 196-cell, 22-colour
interior almost completely. Diversity that helps scores has to be manufactured
in the interior skeleton, not in the border.

## What was measured

On the official 256-piece set with the five official clues pinned (all five are
interior cells, none on the ring, so every completion is automatically
strict-5/5 and the border is hint-unconstrained):

1. generate N structurally distinct fully-matched borders from scratch (a
   randomised depth-first placement over the 60 border cells, grey outward, all
   ring adjacencies matched, deduplicated by exact ring equality);
2. freeze each border and fill its 196 interior cells with **one** fixed
   producer at a fixed budget — a break-tolerant beam that fills row-major,
   scoring each candidate by matched-minus-mismatched edges against its placed
   neighbours (the frozen border included) and pruning to a fixed width with a
   seeded tie-break;
3. report the resulting full-board score band, the pairwise interior tile-
   Hamming distance between the best completions, and whether any border reaches
   the record band.

The two questions are simply: **are the tops distinct?** and **are the tops
high?**

### Measured against expected

Main run: 16 structurally distinct fully-matched borders (this crate's own
randomised DFS, seeds 1 upward), interior beam width 128, three seeds per
border, best kept.

| Quantity | Source (vol-238 D) | Measured here |
| --- | --- | --- |
| Distinct strong borders generated | 8 | 16 (17 DFS attempts) |
| Interior producer | hard-fit greedy beam (weak) | break-tolerant beam, width 128 (stronger) |
| Top score band (min / median / max) | 252 / — / 258 | 434 / 437 / 440 |
| Any border reaches the record band (>= 455) | no | no |
| Gap from the best border-first top to 455 | ~197 | 15 |
| Pairwise interior tile-Hamming (of 196 cells) | ~200 of 208 rows-0..12 tiles | 188 to 191, mean 190.4 (120 pairs) |
| Best board strict-5/5 and border still perfect | (unhinted in source) | yes and yes (score 440, 40 breaks) |

The two load-bearing rows are the last-but-two and the Hamming row: **no border
reaches the record band** (best 440, still 15 short of 455 and about 20 short of
our record), and the completions are **near-maximally distinct** (188 to 191 of
196 interior cells differ between any two). The borders diversify the board
almost completely and select its quality not at all: every one of the 16 lands
in a six-point window, 434 to 440.

An independent-seed robustness pass (12 borders, seed base 7001) agrees: band
434 to 441 (median 438), best 441 still 14 short of 455, interior Hamming 187 to
191 (mean 190.3), and the best board again a strict-5/5 completion of a perfect
border. Two independent seed bases, 28 distinct borders in all, and not one of
them crosses 445, let alone the record band.

The source study (vol-238 invention D) used a weaker, hard-fit greedy interior
that stalled at 252 to 258 and never approached the record band, and froze the
record board's own border only to complete it to the same band as random
borders. This reproduction uses a deliberately *stronger* interior producer, so
its absolute band is higher than the source's 250s. That does not soften the
finding; it sharpens it. Even with a much better interior solver, no distinct
border reaches the record band, and the borders remain near-maximally distinct.
A stronger interior cannot make the border informative, because the border is a
downstream, low-entropy shell hanging off the interior, not the thing that pins
it.

## What this does not establish

The record-board freeze control and the freeze-rows curve from the source are
out of scope here: they need the record board, which lives in the research
repo's untracked output under a foreign colour labelling and is not part of the
public starter kit. The negative reproduced here does not depend on it. It is
established purely from borders generated from scratch, which is exactly the
robustness form the source itself asked for. No claim is made about the exact
250s band of the source's weak greedy; the claim is the record-band gap, and
that reproduces with room to spare.

## Reproduction

The compute crate builds fully-matched borders of the official set from scratch,
freezes each, and completes its interior with the one fixed break-tolerant beam,
then reports the band and the distinctness. See `compute/PLAN.md` for the exact
claim numbers, the interior-producer definition and why it is stronger than the
source's, the scoring-convention mapping, and the scale-faithfulness argument,
and `compute/src/main.rs` for the implementation. Runs on one core of an Apple
Silicon machine; one border completion at beam 128 takes about nine seconds, so
the main run of 16 borders over three seeds is roughly seven to eight minutes.

The best border-first board of the main run is viewable at the
`best_frame_first_board.url` field of `results/frame_first.json` (an
eternity2.dev viewer link); it is a legal strict-5/5 completion of a perfect
border, and its score is far below the record band.

## Notes / caveats

The absolute band this reproduction reports (mid-430s) is higher than the
source's 250s because the interior producer here is stronger, not because the
finding shifted. The comparison that carries the claim is producer-independent:
under the same producer, no border reaches the record band and the completions
are near-maximally distinct. The band is a property of the interior solver; the
negative is a property of the frame-to-interior coupling.

Two deliberate scope choices. The frames are generated by this crate's own
randomised DFS rather than drawn from the original run, which is the robustness
check the source itself asked for. And the run scale is reduced from a
hypothetical exhaustive sweep (16 and 12 borders over a few seeds each) so the
whole reproduction stays under twenty minutes on one core; the finding is a band
result and does not need many borders to land, but two seed bases and 28 borders
give an honest spread. What is not attempted here is the record-board freeze
control from the source, which needs a board that is not part of the public
starter kit; the negative stands without it.
