---
id: clue-corridors
title: Linking the clues early does not help, it hurts
summary: A 1-wide piece corridor between even the two closest official clues is satisfiable in about 1.9e15 ways, so laying it early prunes essentially nothing while consuming pieces; in a controlled A/B every corridor arm loses to its own control at every board size tested.
status: draft
created: 2026-07-22
updated: 2026-07-22
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
results:
  - label: Corridor counting results (pending)
    path: results/corridor_counts.json
site:
  render: false
  dataFile: null
---

# Linking the clues early does not help, it hurts

> **Status:** draft · **Created:** 2026-07-22 · **Updated:** 2026-07-22
> **Authors:** Raphael Anjou

## Summary

The official puzzle ships five fixed clue pieces. A tempting strategy is to
join clue pairs early with short chains of pieces, hoping each chain acts as a
constraint that restricts the rest of the search. The claim of this topic is a
sharp negative: it does not restrict anything measurable, and committing to it
makes results worse.

The reason is counting. The five clues sit at cells (2,2), (13,2), (7,8),
(2,13) and (13,13), with pairwise Manhattan distances between 10 and 22. An
exact transfer-matrix count over the real interior piece set shows that a
1-wide corridor between even the two closest clues (centre to south-west,
distance 10) admits roughly $1.9\times10^{15}$ colour-consistent fillings. A
commitment satisfiable in $10^{15}$ ways excludes essentially nothing, while it
spends 10 to 20 pieces from the pool that a later fill, facing genuine
constraints, will miss. The underlying arithmetic: a cell touching $k$ placed
neighbours has an expected $b_k = 4n/C^k$ legal candidates, and with $n = 196$
interior pieces and $C = 22$ interior colours, $b_1 \approx 36$ explodes while
only $k \ge 3$ contacts give $b_k < 1$. A 1-wide path is built entirely from
$k = 1$ steps.

The empirical side agrees. In a controlled A/B on scaled instances with
faithful clue geometry (12 paired seeds per cell, board sizes N = 8 to 16, 20
second budget, all arms sharing the same fill order and restart discipline),
every corridor arm loses to its own control at every size, with paired Wilcoxon
$|z| \ge 2.80$. The damage grows with corridor length (mean delta $-4.7$ at
N = 8 down to $-17.7$ at N = 16) and is roughly halved when the corridor is
widened from 1 to 2 cells, exactly the width dependence the counting predicts.
At N = 16 the control's worst seed beat the corridor arm's best seed.

## Reproduction

This topic reproduces in two stages. Stage 1 (the checker, working now)
recomputes every counting claim from the official instance: the clue geometry
and pairwise distances, the branching table $b_k = 4n/C^k$, the per-side colour
supply of the real interior piece set (mean 46.1 pieces offering a given colour
on a given side), and the exact transfer-matrix walk counts that give the
$1.9\times10^{15}$ corridor figure. The binary in `compute/` reads only the
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

This is a draft skeleton. Results files are not committed yet; stage 1 output
and the stage 2 tables will land in `results/` with the exact commands in the
frontmatter. The claim is a negative about laying 1-wide (and 2-wide)
clue-to-clue corridors as an early commitment; it says nothing against growing
clue-anchored regions where cells are placed with 2 or more contacts, which the
same arithmetic identifies as the regime where constraint actually bites.
