---
id: beam-width-smc
title: Beam width dominates smart signals; SMC resampling is the one survivor rule that helps
summary: Beam width dominates every per-node improvement (measured monotone 0.836 to 0.878 score-over-optimum across widths 32 to 2048 on this bed; the harder source bed spanned 0.21 to 0.74), tie-break randomisation is a free win (+2.06 edges, paired t 3.47), and SMC resampling gives a small real gain (+1.83 edges, about +1 percent, paired t 5.78 over 96 pairs here; +6 percent on the source bed).
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
  - cd research/topics/beam-width-smc/compute && cargo run --release --features size-12 -- --widths 32,128,512,2048 --rules plain --seeds 1..12 --budget-s 12
  - cd research/topics/beam-width-smc/compute && cargo run --release --features size-12 -- --widths 128 --rules plain,stoch --seeds 1..16 --budget-s 3
  - cd research/topics/beam-width-smc/compute && cargo run --release --features size-10 -- --widths 1024 --rules plain,smc --temp 0.15 --seeds 1..48 --budget-s 3
  - cd research/topics/beam-width-smc/compute && cargo run --release --features size-12 -- --widths 1024 --rules plain,smc --temp 0.15 --seeds 1..48 --budget-s 3
results:
  - label: Per-seed scores, paired stats, and temperature sweep (JSON)
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

Hardware: Apple Silicon, single core. Instances are kit-generated framed
boards (one instance per seed, 22 interior colours capped at the size's
maximum, 5 pinned solution cells), so absolute score levels are not comparable
to the source bed; ordering and paired-delta signs are what the tier tests.

Measured against expected:

| Sub-claim | Expected (vol-246 bed) | Measured (kit bed) |
| --- | --- | --- |
| Width law, plain beam, mean score/opt at w32/128/512/2048 | 0.210 / 0.346 / 0.527 / 0.743, monotone | 0.836 / 0.854 / 0.866 / 0.878 at N=12, 12 paired seeds, monotone in width |
| Tie-break randomisation at w128 | 0.346 to 0.376, helps every N | 0.853 to 0.861 at N=12; paired delta +2.06 edges, t = 3.47, Wilcoxon z = 2.69, win/tie/loss 10/4/2 over 16 pairs |
| SMC resampling at w1024, 96 paired seeds, N in {10, 12} | +8.6 mean edges (about +6%), t = 2.08, win/tie/loss 48/14/34 | +1.83 mean edges (about +1.0%), t = 5.78, Wilcoxon z = 5.45, win/tie/loss 65/14/17, at T = 0.15 |
| SMC node counts vs plain | comparable (86k vs 93k) | identical by construction at fixed width (for example 20,013,677 both arms at N=12) |

All three signs reproduce. The width ordering is clean at every width step;
the tie-break win and the SMC win are both individually significant (the SMC
effect at each rung separately: +1.75 edges, t = 3.30 at N=10; +1.92 edges,
t = 5.41 at N=12). The SMC temperature had to be swept, as the plan
anticipated: T = 1.0 is catastrophic (-17 to -29 edges, every seed loses),
the gain lives on a plateau T in [0.1, 0.2], and T = 0.15 was used for the
powered run. The per-seed scores, the full sweep, and paired statistics are in
`results/beam-width-smc.json`, with representative boards, for example the
best SMC board at N=12
([237/264](https://eternity2.dev/viewer?puzzle=smc_n12_c22_s34&puzzle_size=12&board_edges=adcaacpdafjcacofafqcadmfaeldabmeaetbacpeabmcaadbcmeaplimjunloswuquismgtulmngmmmmtqhmpwwqmjrwdabjejbaipwjnqopwjsqiiojtskinqusmsoqhmgswukmrtuubadtblbawkolomjksplmolipknulunwnoopngrgokhvrughhdaegbpdaorlpjtwrltntisutukjswstkpvisgltvvtklhwsteafwdlbaltqlwvjhnivouspijumstrnuiwigtwvikkwwsjvkfaejbsbaqjgsjrqjvltrpgqlmmignthmitvtvpntwrhqvkhreabkbgcagpogqhpptvshqwjviwvwhuiovgrunnnghhjnhrohbafrcheajrqhpqkrshiqupnhvkopirhkrklrnutijoouovrofaevekcaqvnkkniviggnnuopqjuuhgqjvlggtsrloqhsrvpmeafvcmfanujqklquglkloitlgvkinsnvgqjsrlpkhomsprqofabrfeaakdaewdadifadgdafweadscaehcacpfacmdafrcadbaac&hints=67.0-62.1-7.2-19.0-112.0))
scored through the canonical scorer.

## Notes / caveats

This reproduces the survivor-rule claim on ladder instances, not on the
canonical 16x16 producer: the original study bed used its own planted-instance
generator, so numbers are expected to reproduce in sign and rough magnitude
across a paired-seed distribution, not digit for digit. Whether the +6% SMC
gain transfers to the real 16x16 pipeline (and to the quality of the pool it
feeds downstream refinement) is explicitly open in the source study and is not
claimed here.

Two measured caveats. First, the kit instances are markedly easier than the
source bed (score ratios 0.82 to 0.88 versus 0.21 to 0.74), which compresses
the headroom above the plain beam; the SMC gain lands at about +1% relative
here versus +6% in the source, with the same sign and stronger statistical
significance. Second, this port's per-node cost is higher than the source
engine's, so the w2048 pass does not finish inside 3 seconds of wall clock on
this hardware; the width law is therefore reported at saturation (12 second
cap, every pass completes) with node counts as the compute axis, which tests
the width ordering rather than the source's equal-wall-clock framing. The
survivor-rule A/Bs are unaffected: both arms expand identical node counts at
fixed width by construction.
