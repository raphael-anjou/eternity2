# Reproduction plan: beam-width-smc

> **Repro tier: seeded-statistical (paired-seed deltas; width-dominance ordering + SMC sign).**
> exact = re-run reproduces committed results byte-for-byte. seeded-statistical =
> committed results are the record of our run (hardware noted in the article);
> reproduction = fresh run agreeing within the stated tolerance. qualitative =
> the effect (sign/ordering/band) reappears; exact digits are lineage-bound.

## (a) The exact claim and the expected numbers

Source: vol-246 paper "Making the beam producer 10x better" (research repo,
`vault/papers/vol-246/BEAM-10X.md`) and the verified survey
`vault/reference/beam-search-sota-2026.md`. Study bed: the vol-240 scaling
ladder, single core, planted-optimum instances N = 8/10/12/14, optimum
2N(N-1) matched interior edges, 12 planted instances
(`output/vol-246/ladder_20260717T150836/` in the research repo).

Three sub-claims, in decreasing strength:

1. **Width law.** Edges-ranked beam, 3 s budget, mean score/opt across the
   ladder: 0.210 (w32) -> 0.346 (w128) -> 0.527 (w512) -> 0.743 (w2048).
   Clean monotone. Width is the dominant quality-per-compute lever.
2. **Stochastic tie-break is a free win.** Randomising among exactly tied
   truncation keys lifts w128 from 0.346 to 0.376, helps every N. (Distinct
   from widening the score-tolerance window, which is catastrophic on the real
   16x16 producer: 450 -> 290 at tol 2. Rule: randomise ties, never widen the
   window.)
3. **SMC resampling is the one survivor-rule mechanism with a real positive
   effect.** At N in {10, 12}, width 1024, 3 s, 96 paired seeds: plain top-k
   mean 151.0 vs SMC 159.6 (+8.6 mean, ~+6% relative), paired t = 2.08
   (p ~ 0.04), win/tie/loss 48/14/34 (sign test p ~ 0.15). Node counts
   comparable (86k vs 93k): the gain is which partials survive, not more
   compute.

Negative controls the reproduction can optionally confirm (both lose to plain
width at equal wall clock in vol-246): Hall-deficit completability ranking
(edges-w8192 0.815 vs deficit-w64 0.441 at 10 s) and DD-style dominance merge
(0.679 -> 0.657). These are NOT built in the skeleton; the skeleton covers the
three positive sub-claims.

## (b) Scoring-convention mapping

The kit's canonical scorer (`score_cells` / `Instance::finish`) counts
**rim-excluding matched interior edges**. The vol-246 ladder scored matched
interior edges against the planted optimum 2N(N-1), which is exactly the count
of interior adjacencies of an N x N grid, i.e. the same rim-excluding
convention. **The mapping is the identity.** No strict-5/5 hint convention is
involved in the claim itself; the ladder instances pin 5 solution cells as
hints (mirroring the official clues) and the beam always honours them, so
every produced board is trivially hint-consistent. All reported ratios are
(kit score) / (2N(N-1)).

## (c) Scale-faithfulness

This claim is **natively a small-N claim**; small N is not merely acceptable,
it is mandated by the source. Vol-246 section 5: at N = 14 the score is
strongly bimodal (runs land on discrete plateaus 79/99/.../292) and seed
variance dwarfs mechanism variance, so mechanism A/Bs at N = 14 with few seeds
are statistically meaningless and could falsely refute (or falsely confirm)
the SMC effect. The banked rule is: run survivor-rule A/Bs on the less bimodal
rungs (N <= 12) with >= 16 paired seeds. The reproduction therefore targets
N in {10, 12} with 48 paired seeds each (96 total), matching the source's
powered design. The width law (sub-claim 1) is robust across the whole ladder
and can also be shown at a single rung.

What does NOT follow from small N: transfer to the canonical 16x16 producer.
The source explicitly leaves "does +6% transfer to the real comb producer and
its downstream pool" as its top open item. A 16x16 run of this skeleton is a
demonstration, not a test of that open question.

Caveat on exactness: the vol-246 bed used its own planted-instance generator;
this reproduction uses the kit generator (real colour balance, framed,
distinct up to rotation). Instances differ, so expect the same signs and
comparable magnitudes over the paired-seed distribution, not identical means.

## (d) Exact reproduction steps

Board size is a compile-time feature of e2-core, so one binary per rung
(the crate forwards the size features, same pattern as the scaling-ladder
topic).

1. Build check: `cargo check --features size-10` (and `size-12`).
2. Width law at one rung (about 4 x 8 x 3 s = 96 s per width set):
   `cargo run --release --features size-12 -- --widths 32,128,512,2048 --rules plain --seeds 1..8 --budget-s 3`
   Expect mean score/opt monotone increasing in width.
3. Tie-break win:
   `cargo run --release --features size-12 -- --widths 128 --rules plain,stoch --seeds 1..16 --budget-s 3`
   Expect stoch >= plain in paired means (source: 0.346 -> 0.376 at w128).
4. The SMC headline (the powered A/B, about 2 rungs x 48 seeds x 2 rules x
   3 s ~ 15 min single core):
   `cargo run --release --features size-10 -- --widths 1024 --rules plain,smc --seeds 1..48 --budget-s 3 > results/smc_n10.jsonl`
   `cargo run --release --features size-12 -- --widths 1024 --rules plain,smc --seeds 1..48 --budget-s 3 > results/smc_n12.jsonl`
   Then compute the paired differences (SMC minus plain per seed), the mean
   delta, and a paired t statistic over the 96 pairs. Expected: positive mean
   delta of roughly +5 to +10 edges, t around 2.
5. Report min/median/max per arm, never a single seed. Merge the JSONL rows
   into `results/beam-width-smc.json` and record every board URL emitted.

Skeleton status: the beam with all three survivor rules is implemented,
compiles, and passes a 0.5 s smoke A/B at N = 10 (stoch beats plain on 3 of 4
seeds already; the planted-optimum assertion holds; identical node counts
across arms confirm the rules differ only in which partials survive). It is a
faithful mechanism port (row-major layer beam, break-allowing placement,
rim-hard constraints, softmax resampling via Efraimidis-Spirakis keys), not a
port of the vol-232 comb-fill-order trie producer. The comb fill order, the
corpus prior, and the tol-window knob are intentionally out of scope here
(they belong to the 16x16 producer, not to the ladder claim).

Port uncertainties (must be swept, not assumed):

- **SMC temperature.** The source exposed it as a knob (`PBEAM_SMC_T`) and
  does not publish the winning value. The smoke run shows T = 1.0 over
  softmax of cumulative scores over-diversifies at w128 (SMC below plain);
  step 4 must sweep `--temp` over roughly {0.2, 0.3, 0.5, 1.0} at w1024 and
  report the best arm alongside all arms. Failing to find any T that beats
  plain at w1024 over 96 paired seeds would be an honest non-replication.
- **Resampling base.** "Resample survivors proportional to softmax(score)"
  underdetermines whether the draw is over all children or over a
  pre-truncated elite pool. The skeleton draws over all children; if the T
  sweep fails, test the elite-pool variant (top 4x width by score, then
  resample width from it) before concluding.
