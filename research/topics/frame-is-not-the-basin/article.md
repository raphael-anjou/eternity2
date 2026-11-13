---
id: frame-is-not-the-basin
title: The frame is not the basin selector
summary: With one fixed break-tolerant beam whose ceiling sits near 440, 28 random fully-matched borders all complete into a 434-441 band, and a same-border multi-seed control finds no detectable border effect on score (F(15,112) = 1.24, p = 0.25); two seeds on one border are almost as different as two borders. A negative for this producer, not yet for near-record producers.
status: draft
created: 2026-07-23
updated: 2026-11-11
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
  - cd research/topics/frame-is-not-the-basin/compute && cargo run --release -- --frames 16 --beam 128 --seeds 8 --seed0 1 --control > ../results/frame_variance_control.json
results:
  - label: Main run (16 distinct frames, beam 128, 3 seeds, seed0 1)
    path: results/frame_first.json
  - label: Independent-seed robustness run (12 frames, seed0 7001)
    path: results/frame_first_seed7001.json
  - label: Same-border multi-seed control (16 frames x 8 seeds, every completion kept, one-way ANOVA)
    path: results/frame_variance_control.json
site:
  render: false
  dataFile: null
---

# The frame is not the basin selector

> **Status:** draft · **Created:** 2026-07-23 · **Updated:** 2026-11-11
> **Authors:** Raphael Anjou

## Summary

The border ring is the most constrained part of the puzzle: 60 cells over only
five ring colours, filled by the four corner pieces and the 56 edge pieces. A
natural hope follows from that: if a fully matched border is what pins the top
of the board, then a *different* fully matched border should pin a *different*
high interior, and building several distinct strong borders would open several
distinct high basins. Border-first diversification would then be a lever for
scores.

With one fixed interior producer, it did not. Generating many structurally
distinct fully-matched borders and completing each one with the same
break-tolerant beam gives tops that all land in the same mid-430s band, and a
same-border multi-seed control shows that the spread between borders is no
larger than the spread between seeds on one border. With this producer the
border choice did not visibly change the score.

This is a negative result scoped to its producer. The beam tops out near 440
whatever border it gets, so it cannot reveal a border effect that would only
show up near the record. The structural argument (the ring couples to the
interior only through 56 inward-facing colours, against a 196-cell interior
painted with 17 colours) makes a weak border effect plausible, but it is an
argument, not a measurement.

## What was measured

On the official 256-piece set with the five official clues pinned (all five are
interior cells, none on the ring, so every completion is automatically
strict-5/5 and the border is hint-unconstrained):

1. generate N structurally distinct fully-matched borders from scratch (a
   randomised depth-first placement over the 60 border cells, grey outward, all
   ring adjacencies matched, deduplicated by exact ring equality);
2. freeze each border and fill its 196 interior cells with **one** fixed
   producer at a fixed budget, a break-tolerant beam that fills row-major,
   scoring each candidate by matched-minus-mismatched edges against its placed
   neighbours (the frozen border included) and pruning to a fixed width with a
   seeded tie-break;
3. report the resulting full-board score band, the pairwise interior tile-
   Hamming distance between the best completions, and whether any border reaches
   455, a fixed high-band threshold carried from the source study (the current
   strict-five-clue record is well above it; see the site's records page).

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
| Top score band (min / median / max) | 252 / – / 258 | 434 / 437 / 440 |
| Any border reaches the 455 threshold | no | no |
| Gap from the best border-first top to 455 | ~197 | 15 |
| Pairwise interior tile-Hamming (of 196 cells) | ~200 of 208 rows-0..12 tiles | 188 to 191, mean 190.4 (120 pairs) |
| Best board strict-5/5 and border still perfect | (unhinted in source) | yes and yes (score 440, 40 breaks) |

No border reaches 455 (best 440), and the completions differ almost everywhere
(188 to 191 of 196 interior cells; 191 is the maximum, since the five clues sit
on interior cells). Every one of the 16 lands in a six-point window, 434 to 440.

An independent-seed robustness pass (12 borders, seed base 7001) agrees: band
434 to 441 (median 438), best 441, interior Hamming 187 to
191 (mean 190.3), and the best board again a strict-5/5 completion of a perfect
border. Two independent seed bases, 28 distinct borders in all, and not one of
them crosses 445.

### Same-border multi-seed control

A band across borders means nothing without the spread on a single border. The
control (`--control`) completes each of 16 borders with 8 beam seeds and keeps
every completion (128, no best-of), then runs a one-way ANOVA of score on
border and compares interior Hamming within and between borders.

| Quantity | Measured |
| --- | --- |
| All 128 completions | 432 to 442 |
| Per-border mean | 434.5 to 437.4 (grand mean 436.0) |
| MS between / MS within | 5.88 / 4.75 |
| F(15, 112) | 1.24 (p = 0.25) |
| ICC(1), border share of score variance | 0.029 |
| Interior Hamming, same border, different seed | mean 184.1, min 166, max 191 (448 pairs) |
| Interior Hamming, different borders | mean 190.4, min 185, max 191 (7,680 pairs) |

The border effect on score is not detectable at this sample size, and two
seeds on the same border are already almost as different as two borders, so
most of the apparent border-driven diversity is the producer's seed
sensitivity. The run takes about one minute on 8 cores.

The source study (vol-238 invention D) used a weaker, hard-fit greedy interior
that stalled at 252 to 258. This reproduction uses a stronger producer, so its
absolute band is higher. Both producers end far below the record, so neither
can test whether a border effect appears near the top.

## What this does not establish

The record-board freeze control and the freeze-rows curve from the source are
out of scope here: they need the record board, which lives in the research
repo's untracked output under a foreign colour labelling and is not part of the
public starter kit. That control, with a producer able to reach the record band, is the informative
one and remains open; until it is run, the negative is scoped to producers that
top out near 440.

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
border, and its score is well below the record (see the records page).

## Notes / caveats

The absolute band this reproduction reports (mid-430s) is higher than the
source's 250s because the interior producer here is stronger. The negative is a
property of these producers and budgets, not yet of the frame-to-interior
coupling in general.

Two deliberate scope choices. The frames are generated by this crate's own
randomised DFS rather than drawn from the original run, which is the robustness
check the source itself asked for. And the run scale is reduced from a
hypothetical exhaustive sweep (16 and 12 borders over a few seeds each) so the
whole reproduction stays under twenty minutes on one core; the finding is a band
result and does not need many borders to land, but two seed bases and 28 borders
give an honest spread. What is not attempted here is the record-board freeze
control from the source, which needs a board that is not part of the public
starter kit and a producer able to reach that band.
