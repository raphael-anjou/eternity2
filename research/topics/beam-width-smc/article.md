---
id: beam-width-smc
title: Beam width dominates smart signals; SMC resampling is the one survivor rule that helps
summary: On planted N x N ladder instances, raw beam width is the dominant quality lever (mean score over optimum 0.21 to 0.74 across widths 32 to 2048), per-node oracles lose at equal wall clock, and SMC particle-filter resampling of survivors is the one measured improvement (+6% mean over top-k at width 1024, 96 paired seeds).
status: draft
created: 2026-07-22
updated: 2026-07-22
contributors:
  - name: Raphael Anjou
    role: author
credits:
  - Vol-246 beam producer study on the vol-240 scaling ladder; verified 2026 beam-search literature survey.
tags:
  - beam-search
  - smc
  - scaling-ladder
sources:
  - label: Zhang, Complete Anytime Beam Search (AAAI-98)
    url: https://cdn.aaai.org/AAAI/1998/AAAI98-060.pdf
  - label: Particle-filter inference scaling (2025)
    url: https://arxiv.org/html/2502.01618v3
  - label: SMC steering
    url: https://arxiv.org/pdf/2306.03081
reproduce:
  - cd research/topics/beam-width-smc/compute && cargo run --release --features size-10 -- --seeds 1..48 --budget-s 3 --width 1024
  - cd research/topics/beam-width-smc/compute && cargo run --release --features size-12 -- --seeds 1..48 --budget-s 3 --width 1024
results:
  - label: Results JSON (not yet generated)
    path: results/beam-width-smc.json
site:
  render: false
  dataFile: null
---

# Beam width dominates smart signals; SMC resampling is the one survivor rule that helps

> **Status:** draft · **Created:** 2026-07-22 · **Updated:** 2026-07-22
> **Authors:** Raphael Anjou

## Summary

We asked what makes a wide-beam Eternity II style constructor better per unit
of compute. On a ladder of planted, solvable N x N instances (each with a
proven optimum of 2N(N-1) matched interior edges), the answer has three parts.
First, raw beam width is the dominant lever: at a fixed 3 second budget the
mean score over optimum climbs monotonically from 0.210 at width 32 to 0.346
(width 128), 0.527 (width 512), and 0.743 (width 2048). Second, expensive
per-node "smart" signals lose to raw width at equal wall clock: a Hall-matching
completability oracle and a decision-diagram style dominance merge both score
worse than simply spending the same time on a wider plain beam, because their
per-node cost grows with board size exactly where depth is scarcest. Third, one
cheap change to the survivor rule does help: replacing deterministic top-k
truncation with SMC particle-filter resampling (survivors drawn proportionally
to a softmax of the score) gains +8.6 mean score, about +6% relative, over 96
paired seeds at N in {10, 12}, width 1024 (paired t = 2.08). Randomising among
exactly tied truncation keys is a second, free additive win (width 128 mean
ratio 0.346 to 0.376).

The mechanism behind the SMC gain is not extra compute (node counts are
comparable, 86k vs 93k) but which partials survive: resampling keeps a more
diverse frontier that survives deeper before dying, where a deterministic top-k
frontier can choke early on near-duplicate survivors.

A methodological finding travels with the claim. At N = 14 the score
distribution is strongly bimodal and seed variance dwarfs mechanism variance,
so single-seed or few-seed comparisons at that size are noise. Survivor-rule
A/B tests must be run on the less bimodal rungs (N at most 12) with at least 16
paired seeds. This is why the reproduction below targets N in {10, 12}.

## Reproduction

The `compute/` crate builds planted framed N x N instances with the kit
generator (5 solution cells pinned as hints, mirroring the official clue
setup), runs a parametric layer beam in row-major fill order, and A/B tests
three survivor rules at fixed width and wall clock: deterministic top-k
(plain), top-k with randomised tie-breaking, and SMC resampling. Every board is
re-scored through the canonical rim-excluding scorer; no solver self-report is
trusted. Scores are reported as matched interior edges over the planted optimum
2N(N-1), which is exactly the kit's scoring convention. See `compute/PLAN.md`
for the full design, the expected numbers, and the scale-faithfulness argument.

## Notes / caveats

This reproduces the survivor-rule claim on ladder instances, not on the
canonical 16x16 producer: the original study bed used its own planted-instance
generator, so numbers are expected to reproduce in sign and rough magnitude
across a paired-seed distribution, not digit for digit. Whether the +6% SMC
gain transfers to the real 16x16 pipeline (and to the quality of the pool it
feeds downstream refinement) is explicitly open in the source study and is not
claimed here.
