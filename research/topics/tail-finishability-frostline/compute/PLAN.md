# Reproduction plan: tail-finishability-frostline

> **Repro tier: qualitative (correlation sign and strength band on a regenerated top ensemble; original lineage not reproducible).**
> exact = re-run reproduces committed results byte-for-byte. seeded-statistical =
> committed results are the record of our run (hardware noted in the article);
> reproduction = fresh run agreeing within the stated tolerance. qualitative =
> the effect (sign/ordering/band) reappears; exact digits are lineage-bound.

Class: **statistical** (a rank-correlation / AUC claim, not a solver score or a
counting result).

## (a) The exact claim, with expected numbers

Source: research repo `vault/papers/vol-233/R2-FROSTLINE.md` and
`vault/concepts/frostline-residual-bethe-discriminator.md` (vol-233, 2026-07-13).

Setup in the source study: 39 producer seeds of a width-2048 row-major beam on
the official 16x16 instance, final scores 446-452; beam members sampled and
deduplicated by tail piece-multiset to **n = 103 distinct tops**; top = rows
0..14 frozen, residual = last row (R = 15, 16 cells); pool = the 16 pieces the
board itself placed in row 15 ("self-pool", guarantees satisfiability); label =
exact CP-SAT max-matching tail optimum `tail_opt` (all 103 labels OPTIMAL,
about 0.1 s each; label range 18/20/23 min/med/max). Soft-seam factor model.

Claimed numbers to reproduce (R = 15, soft seam):

| quantity | expected value |
|---|---|
| Spearman rho(Bethe free energy, tail_opt), beta = 4 | **-0.776** (bootstrap SD 0.040) |
| AUC top-vs-bottom tercile, Bethe FE beta = 4 | **0.013** (i.e. clean, inverted axis) |
| Spearman rho(Bethe entropy, tail_opt), beta = 3 | **+0.703**, AUC 0.953 |
| rho(mean_maxprob, tail_opt), beta = 1 | +0.680 |
| rho(frozen_frac, tail_opt), beta = 6 | +0.430 (weak) |
| rho(min-sum frozen fraction) | ~0.09, flat (null control) |
| rho(mean_domain) | flat / nan (null control) |
| trivial baseline rho(final_score, tail_opt) | +0.605 |
| **partial rho(FE, tail_opt given final_score)** | **-0.697** |
| reverse partial rho(final_score, tail_opt given FE) | +0.426 |
| disjoint seed-half cross-validation | rho = -0.795 (n=52) / -0.754 (n=51) |
| degradation, held R=15 label: rho(FE) at cut R = 15/14/13 | -0.737 / -0.193 / -0.117 |
| mean BP domain at cut R = 15/14/13 | 12.5 / 32.6 / 72.2 |

Pre-registered GO gate in the source: |rho| >= 0.4 or AUC > 0.75 or < 0.25.

Also part of the finding (honest negative, from the concept page, vol-233 R4):
FROSTLINE **selection** does not beat raw-score selection at matched compute;
rho(raw, realized full score) = +0.979 vs rho(FE, realized) = -0.308. The
reproduction target here is the discriminator claim; the R4 negative is
documented but out of scope for the first results run.

## (b) Scoring-convention mapping

The kit's canonical scorer (`score_cells` / `score_board`) counts **interior
matched edges, rim-excluding** (max 480 on 16x16). The source study's numbers
live in the same convention family:

- `tail_opt` = matched edges attributable to the tail: the 15 horizontal
  interior edges inside row 15 plus the 16 vertical seam edges between row 14
  and row 15. Both edge classes are interior edges, so they are exactly the
  edges the kit scorer counts; `tail_opt` of a completed board equals
  `score_board(full) - score_board(top with row 15 empty)`. Max 31, observed
  labels 18-23.
- `final_score` (the confound control) = the producer's full-board matched-edge
  count, the same rim-excluding convention as the kit.
- Strict-5/5 vs matched-edges does **not** bite here: the discriminator claim
  is convention-free rank correlation, and hint compliance only changes which
  tops exist, not how an edge is counted. The reproduction runs unhinted; see
  (c).

Border legality (grey edges on the rim, none inside) is enforced as a hard
domain filter in both the label solver and the BP domains, as in the source.
The seam between the frozen top and the tail is a **soft** unary factor, never
a hard filter: the source measured that a hard seam contradicts 5/16 cells on
real tops (R2-FROSTLINE section 5.1).

## (c) Scale-faithfulness

The claim only makes sense **on the canonical 16x16 residual regime**, and more
precisely at residual sizes near 16 cells:

- The source's own degradation curve (R2-FROSTLINE section 5.5) shows the
  mechanism is residual-size critical: rho collapses -0.74 -> -0.19 -> -0.12 as
  the residual grows 16 -> 32 -> 48 cells because the mean BP domain explodes
  (12.5 -> 32.6 -> 72.2) and the graph goes under-critical. A test at a large
  residual would falsely refute; a test at a tiny residual (a few cells) would
  be trivially saturated. The faithful test is R = 15 on 16x16, i.e. a 16-cell
  chain residual.
- Small-N boards (e.g. 8x8 with a 8-cell last row) change both the pool
  composition and the colour statistics; the source has no evidence the
  correlation transfers, so a small-N null would be uninformative. Do not use
  small N to accept or reject.
- What CAN legitimately differ from the source: the top producer. The source
  used a width-2048 beam (tops 446-452). The kit has no beam producer, so the
  reproduction uses a seeded randomized greedy filler; its tops score lower and
  spread wider. The claim under test is rank correlation between FE and
  tail_opt across a spread of self-pool tops, which is producer-agnostic in its
  statement; the reproduction must report its own top-score distribution so the
  regimes can be compared. If rho lands well below the gate on greedy tops,
  that is a scope finding (producer-dependence), not automatically a
  refutation; the decisive check would need a beam-quality producer (kit gap).
- One structural note: at R = 15 the residual graph is a chain, so sum-product
  BP is exact and the Bethe free energy equals the true residual log partition
  function. Loops (and the Bethe approximation proper) only appear at R <= 14.
  This makes the R = 15 reproduction sharper, not weaker.

## (d) Exact reproduction steps

Requirements: Python 3.11+, numpy. No other dependencies (the exact tail label
is a bitmask DP, not CP-SAT, exact at 16 cells in milliseconds).

```sh
cd research/topics/tail-finishability-frostline/compute

# Smoke run (about 30 s): 24 producer seeds, beta sweep, prints the table.
python3 frostline.py --seeds 24 --betas 1,2,3,4 --out /tmp/frostline_smoke.json

# Full run (a few minutes): 120 seeds -> around 100+ distinct tops, plus
# bootstrap SDs, partial correlations, and disjoint seed-half CV.
mkdir -p ../results
python3 frostline.py --seeds 120 --betas 1,2,3,4,6 \
    --bootstrap 200 --out ../results/frostline_r15.json
```

Acceptance:
1. rho(Bethe FE, tail_opt) at beta 3-4 clears the GO gate (|rho| >= 0.4) with
   the sign of the source (negative), and AUC is clean.
2. partial rho(FE, tail_opt given final_score) stays past the gate, sign
   negative (source: -0.697 vs baseline +0.605).
3. The null controls stay null: mean_domain flat, min-sum frozen ~0.
4. Disjoint seed-half rhos agree in sign and magnitude class.

Smoke-run observation (2026-07-22, 16 greedy tops, beta 4, about 6 s): the
headline reproduces qualitatively in the greedy regime: rho(FE, tail_opt) =
-0.94, AUC 0.05 (clean, inverted), partial rho given raw = -0.94, mean_domain
flat, seed-half CV -0.97 / -0.90. Caveat: some secondary parameters differ from
the source in this lower-quality regime (greedy raw scores 373-391 vs beam
446-452); notably exp_matches shows signal here (rho +0.75) where the source
found it null, and the FE magnitude is larger, plausibly because greedy tops
have a wider finishability spread. The full run must report its own regime
alongside every number.

Not yet scripted (follow-ups before status: published): the R = 14 / R = 13
held-label degradation points, and a beam-quality producer to match the
source's top distribution.
