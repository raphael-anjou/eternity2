---
id: tail-finishability-frostline
title: "FROSTLINE: a Bethe free energy finishability oracle for the last rows"
summary: The Bethe free energy of the leftover-piece residual factor graph rank-predicts the exact achievable tail score of a near-final partial board (Spearman rho -0.78, AUC 0.99 at a 16-cell residual), carries signal beyond the raw edge count, and dies once the residual exceeds about 32 cells.
status: draft
created: 2026-07-22
updated: 2026-07-22
contributors:
  - name: Raphael Anjou
    role: author
credits:
  - Vol-233 Round 2 research notes (R2-FROSTLINE), which posed and measured the question.
tags:
  - belief-propagation
  - statistical-physics
  - beam-search
  - finishability
sources:
  - label: Yedidia, Freeman, Weiss, "Understanding Belief Propagation and its Generalizations"
    url: https://www.merl.com/publications/docs/TR2001-22.pdf
reproduce:
  - cd research/topics/tail-finishability-frostline/compute && python3 frostline.py --seeds 120 --betas 1,2,3,4,6 --bootstrap 200 --out ../results/frostline_r15.json
results:
  - label: Correlation table (order parameter vs exact tail optimum)
    path: results/frostline_r15.json
site:
  render: false
  dataFile: null
---

# FROSTLINE: a Bethe free energy finishability oracle for the last rows

> **Status:** draft · **Created:** 2026-07-22 · **Updated:** 2026-07-22
> **Authors:** Raphael Anjou

## Summary

A row-by-row constructive solver ranks its partial boards by matched edges so
far. That score is pure `g`: it says nothing about whether the leftover piece
pool can still finish the remaining rows well. FROSTLINE is the missing `h`. It
builds a weighted factor graph over the empty tail cells, with states drawn from
the leftover pool and edge factors that reward colour agreement at inverse
temperature beta, runs sum-product belief propagation, and reads off the Bethe
free energy. The claim: at a small residual (the last row, 16 cells), lower
Bethe free energy predicts a higher exact achievable tail score, with Spearman
rho of about -0.78 and AUC of about 0.99 against ground-truth labels from an
exact tail solver.

Two qualifications make the claim honest. First, the signal is not a proxy for
the partial board's own edge count: the partial correlation with the exact tail
optimum, controlling for the raw score, remains about -0.70. Second, the signal
is sharply local. Holding the same label and cutting the board higher, the
correlation degrades from -0.74 at 16 residual cells to -0.19 at 32 and -0.12
at 48, as the mean per-cell domain explodes and the factor graph becomes
under-critical. FROSTLINE is a last-two-rows finishability oracle, not a
mid-construction steering signal.

The tail model matters. The residual is a max-matching problem, not a hard
constraint problem: a colour mismatch is a break worth one point, not a
forbidden state. The factor graph is therefore a finite-beta Potts-like model,
and the seam against the frozen rows must be a soft weighted factor, since a
hard seam filter contradicts itself on real near-optimal boards. Trivial
statistics of the same graph (mean domain size, a min-sum frozen-cell census)
carry no signal at all; the free energy is what discriminates.

A follow-up measurement is part of the record: FROSTLINE-based selection of
which partials to spend exact-solve compute on does not beat selection by raw
score, because the realized full-board score is dominated by the raw score
spread while FROSTLINE predicts only a small tail delta. The discriminator is
real; the selection value, at least in this form, is not.

## Reproduction

The reproduction is self-contained and CPU-cheap. Generate a pool of full
16x16 boards on the official piece set with a seeded randomized greedy filler.
For each board, freeze rows 0 to 14 as the top and take the 16 pieces the board
itself placed in row 15 as the residual pool, which guarantees the residual
instance is satisfiable. Compute the ground-truth label with an exact solver
over the last row (a bitmask dynamic program over piece subsets and seam
colours, exact and fast at 16 cells). Compute the FROSTLINE order parameters by
sum-product belief propagation on the residual graph with a soft seam factor,
at beta in {1, 2, 3, 4}. Report Spearman rho, tercile AUC, and a bootstrap SD
of rho for each order parameter, plus the partial correlation controlling for
the board's own raw score.

See `compute/PLAN.md` for the exact claim numbers cited from the source study,
the scoring-convention mapping, the scale-faithfulness argument, and the exact
commands. `compute/frostline.py` implements the full pipeline.

## Notes / caveats

This draft reproduces the discriminator claim with a greedy producer rather
than the original wide-beam producer, so the top-quality distribution differs
(greedy tops score lower than beam tops). The claim is about rank correlation
on a spread of tops, not about absolute scores, and the reproduction reports
its own spread. The R=14 and R=13 degradation points and the selection-value
negative are described in the plan but not yet scripted.
