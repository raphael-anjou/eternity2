# Judge report: cut-multiset pruning (Hall matching) on the hard ladder

Branch `devin/r2-cut-multiset-pruning`, judged at commit dc662c8 (prereg d54dfce).
Judge session: https://app.devin.ai/sessions/a32dc746a1ff4e679b71fdcb5590d5e6.
The branch was not modified.

## Verdict: keep

The test is valid and its preregistered kill criterion (NO-GAIN on all three
rungs R6, R7, R8) does not fire: both variants are GAIN at R6 (6x6 c = 5),
Holm-significant. PASS (G1 at 8x8, median paired ratio <= 0.5) is not met;
R7 and R8 are NO-GAIN. The outcome is "promising, needs a variant" in the
round's wording, but the measured effect is small: about 10 to 13% fewer
nodes at 6x6, where the baseline needs about 1,000 nodes, with wall clock
neutral to slightly worse at every rung. Nothing in the evidence supports a
gain at 7x7 or 8x8.

Reasons:

1. Preregistration holds. `PREREG.md` was committed alone in d54dfce
   (2026-11-11 13:52:27 UTC); every result file, the analysis code, the R8
   instances and the plug-in arrive in dc662c8 (14:07:54 UTC). `PREREG.md`
   is not touched by dc662c8 and there is no `PREREG_AMENDMENTS.md` in the
   experiment directory. The kill rule (section 7) matches the binding
   statement word for word in substance: kill iff NO-GAIN on all three rungs;
   PASS requires G1 at 8x8; GAIN without PASS is promising. The V2 selection
   rule was fixed before calibration and `analyze.py` applies it as written
   (recomputed: cmp-hall-frontier-k4 28/30, cmp-hall-all 28/30, cmp-hall-all-k4
   28/30 with the smallest sum of log10 KM medians 13.997, so V2 =
   cmp-hall-all-k4).
2. Novelty check is sound and was done first. Per-colour supply minus demand
   (>= 0 and even) is exactly gacolor, already inside the baseline; the ring
   cut is NS-1, measured identical to the baseline on the ladder calibration.
   The bipartite Hall condition across non-adjacent frontier cells is not in
   any single-core-benchmark propagator and not measured in the repo. It is
   the feasibility half of Régin's all-different, which `EXPLORED.md:22`
   lists as a mapped family; the prereg says so. Not a duplicate by the
   statement's rule (not implied, not measured), but a thin extension of a
   mapped family.
3. The node metric means the same thing for both arms: same fork, same
   harness, same `NodeCounter` (one node per consistent `ValueTried`), the
   Hall prune is a propagation wipeout after the assignment is counted, as
   every other propagator in the baseline. With the hook off the fork
   reproduces the committed baseline 60/60 node for node (rerun here).
   No planted-solution path reaches the solver: `Scb` receives only the
   `puzzle` object and the harness verifies boards independently.
4. The R6 GAIN is not an artefact of random value order. Because the
   baseline's value order draws from one RNG stream per run, any prune shifts
   every later shuffle, so each paired comparison is also a run-seed change
   (6 of 30 R6 pairs have the candidate worse, impossible for a pure prune
   at fixed order). I measured that noise directly (below): 39 seed-only
   reruns of the baseline never reach the significance level, while V1 under
   40 run seeds is below 1 every time and lower on all 30 instances after
   averaging over seeds.

## Recomputed numbers

Own script (`recompute.py`, independent of `analyze.py`) on the committed raw
rows; G1 exactly as PROTOCOL.md (upper median of 30 log ratios, censoring
table, 10,000 paired bootstrap resamples, seed 20261111). All values agree
with `results/summary.json` and the README table.

| rung | arm | solved | KM median nodes | ratio vs base | 95% CI (log) | one-sided p | W/L/T | median wall ratio | nodes/s |
|---|---|---|---|---|---|---|---|---|---|
| R6 6x6 c=5 | base scb-gacolor_ac3_random | 30/30 | 951 | | | | | | 197,538 |
| | V1 cmp-hall-frontier | 30/30 | 919 | 0.900 | [-0.200, -0.061] | 0.0002 | 24/6/0 | 1.000 | 172,836 |
| | V2 cmp-hall-all-k4 | 30/30 | 837 | 0.876 | [-0.252, -0.028] | 0.0179 | 21/9/0 | 0.946 | 180,756 |
| R7 7x7 c=6 | base | 16/30 | 921,773 | | | | | | 120,701 |
| | V1 | 18/30 | 654,651 | 1.000 | [-0.079, 0.000] | 0.826 | 13/5/12 | 1.018 | 110,828 |
| | V2 | 17/30 | 725,243 | 1.000 | [-0.140, 0.000] | 0.958 | 11/6/13 | 1.094 | 110,536 |
| R8 8x8 c=10 | base | 19/30 | 505,364 | | | | | | 34,053 |
| | hp-dfs | 19/30 | 509,862 | | | | | | 1,031,968 |
| | V1 | 22/30 | 449,692 | 0.932 | [-0.168, 0.000] | 0.172 | 18/4/8 | 1.023 | 29,985 |
| | V2 | 22/30 | 445,508 | 0.952 | [-0.245, 0.000] | 0.417 | 16/6/8 | 1.057 | 28,368 |

Holm (two looks, one-sided alpha 0.025): R6 V1 p 0.0002 <= 0.0125, V2 p
0.0179 <= 0.025, both significant, both ratios > 0.5, so both GAIN. R7, R8:
neither significant, NO-GAIN. No error rows, 0 inconsistent tries, all caps
1,000,000, `equals_planted` identical across arms (3, 0/1, 4).

Against plain DFS at R8 (reported, not gating): V1 and V2 match `hp-dfs` in
median nodes (paired ratio 1.0) and cost about 33 to 35 times its wall clock
per run (median paired wall ratio 35.0 and 32.7). The baseline itself costs
about 26 times `hp-dfs` there for the same node median.

### Reruns by the judge

- `run.sh check`: build, 2 tests (hand-built example, `hall_match` basics),
  `check-instances`, `check-c10` and fork-off equivalence pass (42 s); 60/60
  fork-off rows equal the committed baseline in status, nodes and backtracks.
- Seed-sensitivity study (not preregistered, used only to test whether the
  R6 effect is an RNG artefact; it does not change any label). A scratch copy
  of the plug-in exposing run seeds 1..40 for fork-off, V1 and V2.
  Run seed 1 reproduces the committed R6 rows of all three arms node for node.
  - R6, 40 run seeds. Null (fork-off seed k vs seed 1, 39 comparisons):
    median ratio ranges 0.68 to 1.27, 0 of 39 reach p <= 0.025. V1 seed k vs
    fork-off seed k: median ratio 0.897 (range 0.816 to 0.952), never >= 1,
    32 of 40 seeds Holm-level significant; seed-averaged per-instance log
    ratio median -0.140 (x0.870), 30/30 instances lower; total nodes x0.874,
    total wall x1.084. V2: median ratio 0.867 (0.810 to 0.960), seed-averaged
    x0.854, total wall x1.012.
  - R7, run seeds 1..9: V1 and V2 never significant (median ratio 1.000 in
    most seeds, as the median pair is a double-censored tie); pooled solves
    165/270 base, 178/270 V1, 175/270 V2.
- R8, run seed 1: V1 (`cmp-hall-frontier`) and the fork with the hook off
  rerun on `instances/8-c10-test.jsonl` reproduce the committed R8 rows of
  V1 and of `scb-gacolor_ac3_random` 30/30 in status, nodes and backtracks.

## Findings

`file:line | severity | claim | problem | fix`

- `research/experiments/cut-multiset-pruning/PREREG.md:110-116` | should-fix | R8 is "8x8 c = 10, above the critical count" with instances generated here | `hard-ladder/PROTOCOL.md:5-7` allows only colour sets the ladder README or a recorded amendment declares usable; `hard-ladder/README.md:26-30` says c >= 10 at 8x8 "needs its own amendment and calibration before candidates use it". Amendment A3 (shoulder track) exists only on `devin/r2-hard-ladder-shoulder` (7963868) and had not chosen D. The baseline was not recalibrated at c = 10, and it is weak there: plain DFS matches it in nodes at 1/26 of the wall clock. The verdict does not depend on it (the kill is avoided at R6 on the declared critical track; PASS fails at R8 either way, and at the declared c = 8 the baseline is 0/30 so PASS would need >= 15/30 solves). | Label R8 as an off-protocol, uncalibrated rung in the README; re-evaluate on the A3 shoulder 8x8 rung once D and its calibrated baseline are committed.
- `research/experiments/cut-multiset-pruning/compute/engine-fork/src/lib.rs:2574-2586` | should-fix | paired node ratios measure the effect of the prune | The RandomShuffle value order consumes one RNG stream per run, so every prune also changes all later value orders; each pair is a prune plus a run-seed change (R6: 6 of 30 candidate losses, R7 5 of 16 and R8 4 of 19 among both-solved pairs). With one run seed, single-rung results carry seed noise whose median-ratio spread (0.68 to 1.27 at R6, judge rerun) is larger than the claimed effect. At R6 the effect survives a 40-seed check; at R7/R8 the single-seed solve-count moves (16 to 18, 19 to 22) are within that noise. | For the next variant, report several run seeds per arm, or derive the shuffle seed from (run seed, depth, cell) so a prune does not perturb the order of surviving subtrees.
- `research/experiments/cut-multiset-pruning/compute/tests/example.rs:120-131` | should-fix | the hand-built state passes "class_balance, parity, gacolor, island, NS-1, Eulerian ring check and pairwise AC" | The test computes its own pairwise-AC domains and calls the propagator functions directly; it never loads the state into the baseline engine (`EngineSolver::gacolor_ac3_random`, its `GaColorState` and AC-3), so "the baseline does not prune it" is inferred from the functions, not tested on the engine that is compared. | Add a test that places the ring through the fork with the hook off and asserts no wipeout, then hook on and asserts a wipeout.
- `research/experiments/cut-multiset-pruning/README.md:89` | nitpick | "8 to 17% lower node throughput at R7 and R8 that leaves wall clock roughly unchanged" | R6 throughput is also lower (V1 -12.5%, V2 -8.5%); summed over 40 run seeds V1 costs x1.08 total wall at R6 while saving 13% nodes. | State the throughput loss per rung, including R6.
- `research/experiments/cut-multiset-pruning/README.md:96-101` | nitpick | side observation: baseline equals plain DFS in nodes at R8 and costs 26x wall | Correct (recomputed 19/30 each, KM 505,364 vs 509,862; median wall 15,367 vs 593 ms), but the more decision-relevant number is that V1/V2 are about 33 to 35 times slower than `hp-dfs` at R8 for the same paired node median. | Add the candidate-vs-hp-dfs wall ratio to the R8 row.
- `research/experiments/cut-multiset-pruning` (git history) | info | baselines at R8 and the fork-off check ran before the method | All rows land in one commit (dc662c8), so the run order stated in PREREG section 3 and 4 cannot be verified from git; node counts are deterministic and the judge reproduced fork-off (60/60) and the R6 seed-1 rows of all arms, so this does not affect the result. | Commit baseline rows and the equivalence check before the method rows in future rounds.
- `research/experiments/cut-multiset-pruning/PREREG.md:28` | info | other straight cut lines give only a total-parity condition | Plausible (which unused pieces fall on each side of an interior cut is free), stated as algebra without a written argument or measurement. | One paragraph of proof or a measured zero on calibration seeds.
- `research/experiments/cut-multiset-pruning/compute/engine-fork/src/lib.rs:1587-1630` | checked, no issue | Hall check soundness | Masks are built from the cell's current domain rows (`row = piece*4 + rotation`, so `>> 2` is the piece id), minus used pieces; a dead state has no perfect matching only if no completion exists, so the prune is sound given sound domains. The `> 128` pieces early-return keeps it sound above 11x11. No error rows or exhausted planted instances anywhere.

Reviewer lenses (design, scope, prose, code and reproducibility) were run by
the judge in one pass; every finding above was checked against the code or
recomputed from raw rows. Misfire deliberately not raised: the double-censored
ties that pin the R7/R8 median at ratio 1.000 are the protocol's rule
(PROTOCOL.md:35-40), not an analysis error.

## Next-round test

Hall matching with Régin filtering (remove every (cell, piece) edge that lies
in no maximum matching, not just feasibility) on the frontier, with a
prune-invariant value order (shuffle seeded by run seed, depth and cell),
run on the A3 shoulder track 6..10 once D and its calibrated baseline are
committed, against that baseline and `hp-dfs`, with 5 run seeds per arm and
the paired statistic taken on seed-averaged log nodes. Kill: NO-GAIN at
shoulder 8x8 in nodes, or a median wall-clock ratio > 1 against the better of
the calibrated baseline and `hp-dfs` at 8x8. PASS: G1 at shoulder 8x8 and
wall ratio < 1 against `hp-dfs`. Given that pairwise AC already removes most
of what a one-cell-deep matching sees ("pruning fades about two cells out",
EXPLORED.md:22), a cheaper alternative test is to log, on R7 calibration
seeds, how often the Hall check fires per node and at what depth before
spending a full round on filtering.

