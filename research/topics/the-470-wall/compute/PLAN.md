# Reproduction plan: the-470-wall

> **Repro tier: exact (step 1); steps 2-3 seeded-statistical.**
> exact = re-run reproduces committed results byte-for-byte. seeded-statistical =
> committed results are the record of our run (hardware noted in the article);
> reproduction = fresh run agreeing within the stated tolerance. qualitative =
> the effect (sign/ordering/band) reappears; exact digits are lineage-bound.

Source of the finding: research repo, `vault/papers/vol-226/STAT.md` (vol-226,
family STAT). This plan maps its labeled claims C1-C9 onto reproducible
computations in the blog repo's starter-kit substrate.

## (a) The exact claim, with expected numbers (cited from the vault source)

All numbers below are quoted from STAT.md and are what the reproduction must
reproduce (exact claims to machine tolerance, conjectures stated as such).

Instance facts (COMPUTATION, exact):

- 256 tiles: 4 corners, 56 edge pieces, 196 interior pieces.
- Non-gray half-edges: 256*4 - 64 = 960 = 2*480 (bookkeeping-critical, no slack).
- Two decoupled subsystems:
  - frame ring: colors 1-5 only, 60 adjacencies, 24 half-edges per color,
    collision probability p_f = 5*(24/120)^2 = 0.200;
  - interior subsystem: colors 6-22 (5 colors at 48 half-edges, 12 at 50),
    420 adjacencies, collision probability p_i = 0.0588 (about 1/17.0).

C1 (COMPUTATION, exact): constraint density
mu = 196 * 4 * (interior collision)^4, with interior collision 0.0589,
giving mu ~ 0.0094 << 1 (rigid/condensed regime).

C3 (COMPUTATION, heuristic model): annealed first moment for uncorrelated
perfect placements, with class-restricted base
W_geom = 4! * 56! * 196! * 4^196 (log10 = 559.94):
log10 E[#480] = 559.94 + 60*log10(0.2) + 420*log10(0.0588) = +1.14,
i.e. E[#480] ~ 13-20; working model 10-20 mutually near-orthogonal 480s.
Reproduction note: with the exact p_i = 0.0588435 (not the rounded 0.0588)
the same formula gives +1.28, identical to the score-480 landscape entry;
E[#480] ~ 19, inside the same 10-20 band. The +1.14 in STAT.md is a rounding
artifact of quoting p_i to three significant figures.

C6 (COMPUTATION, exact large deviation): score landscape for uncorrelated
configurations, S = sum of 60 Bernoulli(p_f) + 420 Bernoulli(p_i):

| score | log10 Pr[S=s] | log10 # uncorrelated configs |
|---:|---:|---:|
| 37 (mean) | -1.16 | 558.78 |
| 200 | -97.1 | 462.8 |
| 400 | -373.4 | 186.6 |
| 460 | -500.4 | 59.6 |
| 470 | -526.8 | 33.1 |
| 480 | -558.7 | +1.28 |

Mean uniform-config score 36.7, sd about 5.7. Entropy of uncorrelated configs
stays above 10^30 up to score ~470 and crosses 1 only at 480.

C8 (COMPUTATION): the real tile set is indistinguishable from a matched null
in every second-order color statistic; the single genuine deviation is zero
rotation-duplicate tiles versus a null mean of 4.06 +/- 2.05 (p ~ 0.013).
The real-set side of this (exactly 0 rotation duplicates among 256 tiles) is
exact and reproduced here; the null side needs a matched-null generator.

C4 (COMPUTATION, n <= 7) / C5, C7, C9 (CONJECTURE): exhaustive enumeration of
small planted instances tuned to E2's mu shows a bimodal overlap histogram
with an empty band, collapsing at mu ~ 0.003-0.009 to exactly one planted
solution (overlap 1.0) plus a few globally orthogonal solutions (overlap 0.0),
e.g. n=5, C=11, mu=0.009: histogram 95,1,0,0,0,0,0,0,4,13,15 over overlap
bins 0,.1,...,1. The 16x16 statements (10-20 near-orthogonal 480s, empty
overlap gap, 470 wall = phase boundary) are extrapolations along the monotone
mu trend and must be labeled CONJECTURE in any published output.

## (b) Scoring-convention mapping

- The kit's canonical `score_cells`/`score_board` counts rim-excluding matched
  internal edges: 480 max on 16x16. STAT.md's score S is the same object (the
  number of the 480 internal adjacencies that are color-matched, gray border
  legality assumed). So the conventions coincide: kit score = STAT score,
  and the 60/420 split is frame-ring joints vs all other joints.
- The finding is about matched-edges scores of full gray-legal placements. It
  is NOT a strict-5/5 claim: hint compliance plays no role in the ensemble
  analysis, and the community "470 wall" cited is the matched-edges/center-clue
  validity class (Blackwood 470). No strict-5/5 mapping is needed; if the
  article later compares against strict-5/5 records (460/461) it must say the
  conventions differ.
- The frame ring (the 60-cycle of joints between border pieces) is included in
  the kit score. Only the 64 gray rim half-edges are excluded, matching
  STAT.md's 960 = 2*480 accounting exactly.

## (c) Scale faithfulness

Two layers with opposite answers; do not mix them.

- Instance-side computations (C1, C3, C6, real-set side of C8) only make sense
  on the canonical 16x16 official set: they are functions of the real bag's
  color profile. They are exact and cheap (< 1 s). Running them on a generated
  small instance would produce different numbers by construction and would say
  nothing about the claim.
- The overlap-gap structure (C4) is small-N BY DESIGN: STAT.md's methodology is
  exhaustive enumeration at n = 4..7 with the color count tuned so that mu
  matches E2's 0.009 (n=5, C=11 is the closest analog). Small N does not
  falsely refute here PROVIDED mu is matched: STAT.md's own table shows the gap
  is absent at loose mu (n=4, C=4, mu=0.204 gives a continuous histogram) and
  opens as mu drops. A small-N reproduction must therefore sweep C at fixed n
  and verify the trend in mu, not test a single arbitrary small instance.
- The 16x16 versions (C5, C7, C9) cannot be tested directly at any feasible
  compute; they remain conjectures and the reproduction must present them as
  extrapolations, exactly as the source does.

## (d) Exact reproduction steps

Step 1 (implemented, `src/main.rs`): instance-side exact numbers.
`cargo run --release > ../results/landscape.json` (< 60 s, actually < 1 s).
Verifies against the cited values: class counts 4/56/196; 960 non-gray
half-edges; frame colors {1..5} at 24 half-edges each; p_f = 0.200;
p_i ~ 0.0588; interior-tile collision ~ 0.0589; mu ~ 0.0094;
log10 W_geom ~ 559.94; log10 E[#480] ~ +1.14; exact log-space convolution
reproducing the C6 table (mean 36.7, sd 5.7, log10 counts at
37/200/400/460/470/480); rotation-duplicate count 0 on the real set.

Step 2 (not yet implemented): small-N overlap-gap enumeration. Generate planted
n x n instances with random internal colors and gray border (n = 4..6, C swept
so mu spans 0.2 down to 0.009; seeds >= 20 per (n, C)), enumerate all perfect
placements by DFS with distinct-tile constraint, record overlap-with-planted
histograms, and check the empty-band trend of the C4 table. Estimated compute:
minutes per (n, C) point at n <= 5, up to ~ 1 h total at n = 6; keep n = 7 out
of the default run.

Step 3 (not yet implemented): matched-null side of C8. Requires a generator of
random planted 16x16 boards with E2's EXACT color profile (60 frame + 420
interior adjacencies consuming the exact per-color half-edge counts) WITHOUT
the distinctness repair pass, then count rotation-duplicate tiles over 300-500
draws and recover the null mean 4.06 +/- 2.05 and p ~ 0.013. The kit generator
cannot serve as this null (it enforces distinctness and its own balance).

Success criterion for the draft: step 1 output matches every cited exact number
above to displayed precision. Steps 2-3 upgrade the topic from draft toward
publishable by reproducing the C4 trend and the C8 p-value.
