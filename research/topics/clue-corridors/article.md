---
id: clue-corridors
title: Laying clue-to-clue corridors first hurt a greedy filler
summary: A 1-wide piece corridor between the two closest official clues (9 cells) keeps about 4.7e13 of 9.3e25 unconstrained placements, the same 1/17-per-check rate as any chain, so it adds no constraint over another order; its cost is fail-last scheduling. In one A/B with a max-contact greedy filler on generated boards, every corridor arm scored below its control at every size tested.
status: draft
created: 2026-07-22
updated: 2026-11-11
contributors:
  - name: Raphael Anjou
    role: author
credits:
  - The idea of linking the five clue pieces to each other early, so that the links restrict the rest of the search, is a natural first instinct when looking at the clue layout. This topic quantifies why it does not pay.
tags:
  - clues
  - branching
  - negative-result
sources:
  - label: Official clue placements (pieces 34, 45, 135, 210, 221 in cell order)
    url: https://e2.bucas.name/
reproduce:
  - cd research/topics/clue-corridors/compute && cargo run --release --bin corridors > ../results/corridor_counts.json
  - cd research/topics/clue-corridors/compute && for s in 8 10 12 14 16; do cargo run --release --bin ladder --features "size-$s" -- "$s" 12 20 8 > "/tmp/ladder_$s.json"; done && python3 stats.py /tmp/ladder_*.json > ../results/ladder_ab.json
results:
  - label: Corridor counting (exact transfer-matrix checker)
    path: results/corridor_counts.json
site:
  render: false
  dataFile: null
---

# Linking the clues early does not help, it hurts

> **Status:** draft · **Created:** 2026-07-22 · **Updated:** 2026-11-11
> **Authors:** Raphael Anjou

## Summary

The official puzzle ships five fixed clue pieces. A tempting strategy is to
join clue pairs early with short chains of pieces, hoping each chain acts as a
constraint that restricts the rest of the search. The claim of this topic is a
sharp negative: it does not restrict anything measurable, and committing to it
makes results worse.

The reason is scheduling, not volume. The five clues sit at cells (2,2),
(13,2), (7,8), (2,13) and (13,13), with pairwise Manhattan distances between 10
and 22. Two clues at distance 10 (centre to south-west) have 9 cells between
them. An exact transfer-matrix count over the real interior piece set gives
about $5.6\times10^{13}$ colour-consistent walks for that corridor, about
$4.7\times10^{13}$ after the distinct-piece correction. Against the
$P(196,9)\cdot4^9 \approx 9.3\times10^{25}$ unconstrained placements of nine
pieces, that keeps a fraction of about $5\times10^{-13}$: the same $1/17$ per
matched edge as any other chain, such as the first row of a row-major scan.
Every fill order pays the same 480 edge checks (see the constraint-immediacy
topic); what a corridor changes is when its closing checks arrive (fail-last)
and that its pieces leave the pool before any $k \ge 3$ cell exists. With
$n = 196$ interior pieces and $C = 17$ colours on them, $b_k = 4n/C^k$ gives
$b_1 \approx 46$, $b_2 \approx 2.7$, $b_3 \approx 0.16$: only $k \ge 3$
contacts give $b_k < 1$, and a 1-wide path is built from $k = 1$ steps.

The empirical side agrees. In a controlled A/B on scaled instances with
faithful clue geometry (12 paired seeds per cell, board sizes N = 8 to 16, 20
second budget, all arms sharing the same fill order and restart discipline),
every corridor arm loses to its own control at every size, with paired Wilcoxon
$|z| \ge 2.80$. The damage grows with corridor length (mean delta $-4.7$ at
N = 8 down to $-17.7$ at N = 16) and is roughly halved when the corridor is
widened from 1 to 2 cells, the direction the scheduling argument predicts. This A/B is one producer
(a max-contact greedy with restarts) at one budget on generated instances, and
its tables are the original run, not yet committed.
At N = 16 the control's worst seed beat the corridor arm's best seed.

## Reproduction

This topic reproduces in two stages. Stage 1 (the checker, working now)
recomputes every counting claim from the official instance: the clue geometry
and pairwise distances, the branching table $b_k = 4n/C^k$, the per-side colour
supply of the real interior piece set (mean 46.1 pieces offering a given colour
on a given side), and the exact transfer-matrix walk counts that give the
$4.7\times10^{13}$ corridor figure. The binary in `compute/` reads only the
kit's bundled official instance and prints a JSON of all numbers.

Stage 2 (the A/B, planned in `compute/PLAN.md`) reruns the three-arm
experiment: a control (clues pinned, max-contact greedy fill with restarts), a
1-wide corridor arm and a 2-wide ribbon arm that lay clue-to-clue routes before
the identical fill. The prediction to check is that both corridor arms lose to
the control at every board size, that the loss grows with N, and that the
2-wide arm loses about half as much as the 1-wide arm. See `compute/PLAN.md`
for the exact design, the scoring-convention mapping, and the scale
faithfulness discussion.

## Notes / caveats

Stage 1 output is committed in `results/corridor_counts.json`; the measured
transfer-matrix figures are about 4.7e13 distinct-piece corridor fillings
(5.6e13 raw walks) for the 9-cell closest-pair corridor. The source study used
L = 10 (2.05e15 here, 1.9e15 in the source); two clues at distance 10 have 9
cells between them, so L = 9 is the right length. The uniform model now uses
C = 17, the colours actually present on interior pieces (C = 22 counted the 5
frame-only colours). The stage 2 A/B tables will land in `results/` when that arm runs. The claim is a negative about laying 1-wide (and 2-wide)
clue-to-clue corridors as an early commitment; it says nothing against growing
clue-anchored regions where cells are placed with 2 or more contacts, which the
same arithmetic identifies as the regime where constraint actually bites.
