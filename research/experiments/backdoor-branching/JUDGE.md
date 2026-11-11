# Judge report: backdoor-guided branching (round 2)

Branch `devin/r2-backdoor-branching`, commit `3c85866`, directory `research/experiments/backdoor-branching`.

## Verdict: kill

The test is valid and its preregistered kill criterion fired. Stage 1 passes its own gate (K1 does not fire), but Stage 2 fails: both variants are NO-GAIN against the strongest dynamic-ordering baseline at R6, R7 and R8. That fires K2 (no PASS at 8x8) and K3 (NO-GAIN at all three rungs). The kill does not depend on the 240 s wall-clock guard. R6 has no censoring and the variants lose there by 8x to 17x in nodes. At R7 and R8 the variants still lose to `bb-mrv` on node counts alone, even if every time-censored run is assumed to have finished one node after it stopped.

## Reasons

1. **Preregistration.** `PREREG.md` was committed alone in `fd42c55` (13:49 UTC). The first code and results came in `92d3485` (14:11 UTC). `git diff fd42c55 HEAD -- PREREG.md` is empty and there is no `PREREG_AMENDMENTS.md`. The kill criterion matches the binding statement. K1 is "Stage 1 AUC does not exceed fail-first with the 95% CI of the difference above 0". K2 is "Stage 2 fails G1 at 8x8 against the strongest dynamic-ordering baseline". PASS requires Holm rejection plus ratio <= 0.5, which is G1 made stricter by Holm. K3 is an extra kill route that only makes the test stricter. The novelty check is in PREREG.md: it lists the fixed and dynamic orders the repo has measured, adds MRV with the same propagation (`bb-mrv`) and Refalo-style impact (`bb-impact`, not measured in the repo before), and defines "strongest baseline" as the worst label across `bb-mrv`, `bb-impact` and `scb-gacolor_ac3_random`.
2. **Sample-size rule.** The rule was applied as written: the projection was 85.8 min, above the 50 min threshold, so every solver ran on seeds 1..20 (`results/calibration/projection.json`). The committed R6/R7 baseline rows are byte-identical to `origin/devin/r2-hard-ladder` and are restricted to the same seeds.
3. **Fairness.** All four `bb-*` solvers share one code path (`compute/src/lib.rs`): A3's `Model::propagate`, the lowest-index forced-move rule, option-index value order, and one `NodeCounter::try_assign` per assignment. They differ only in `Search::choose`. Probes never prune, so pruning power is identical and a node means the same thing in each. The variants lose to `bb-mrv` on nodes and on wall clock, so no node win hides a wall-clock loss.
4. **Leakage.** Stage 1 labels come from A3's solution-in-hand backdoors, as designed. The features (`features()` in `lib.rs`) use only position, domains, the colour census and propagation probes, and none of them depends on piece order. Stage 2 runs on scrambled ladder puzzles with no planted information. `equals_planted` is rare and is not concentrated in the variants (0 to 1 per variant file).
5. **Reproduction.** I reran parts of the experiment into `/tmp`, without touching the branch. `bb_features` reproduces `results/stage1/features.jsonl` exactly. `bb-mrv`, `bb-impact`, `bb-single` and `bb-learned` at R6, plus `bb-mrv` at R7 and R8 (seeds 1..20), reproduce every committed (status, nodes_counted) pair. `cargo test --release` passes (6 tests), and `hard_ladder check-instances` passes.

## Recomputed numbers (own scripts, from committed raw rows)

Stage 1, from `results/stage1/features.jsonl` and `model.json`. I checked the labels against `sat-backdoors/results/results.jsonl`: 0 mismatches out of 300 instances. 19,800 candidate cells, with no root singletons; 8.3% of cells are positive.

| quantity | committed | recomputed |
|---|---|---|
| learned test AUC | 0.637 [0.617, 0.657] | 0.6368 [0.6171, 0.6568] |
| fail-first (-\|D\|) AUC | 0.373 | 0.3731 [0.359, 0.387] |
| Delta (K1 statistic) | +0.264 [0.233, 0.295] | +0.2637 [0.2329, 0.2953] |
| learned minus signed +\|D\| | +0.010 [-0.005, 0.025] | +0.0100 [-0.005, 0.025] |
| learned minus `impact` alone | not reported | -0.011 [-0.029, 0.007] |
| learned, interior cells only | not reported | 0.518 (signed +\|D\| 0.488) |

Stage 2 (seeds 1..20, cap 1e6). My independent implementation of the PROTOCOL.md log-ratio table, the time-guard tie rule, the upper median and the paired bootstrap (seed 20261111) matches `results/summary.json` everywhere.

| rung | solver | solved/20 | KM median nodes | median wall s |
|---|---|---|---|---|
| R6 | bb-learned / bb-single | 20 / 20 | 5,453 / 2,886 | 1.3 / 0.8 |
| R6 | bb-mrv / bb-impact / scb | 20 / 20 / 20 | 400 / 965 / 1,350 | 0.005 / 0.5 / 0.006 |
| R7 | bb-learned / bb-single | 1 / 1 | > cap | 240 (guard) |
| R7 | bb-mrv / bb-impact / scb | 15 / 5 / 9 | 588,305 / > cap / > cap | 14.6 / 240 / 7.7 |
| R8 | bb-learned / bb-single | 2 / 1 | > cap | 240 (guard) |
| R8 | bb-mrv / bb-impact / scb / hp-dfs | 20 / 14 / 12 / 12 | 95,128 / 33,987 / 468,270 / 585,007 | 5.1 / 164.7 / 24.9 / 0.9 |

Paired median ratio, variant over `bb-mrv` (learned, single). R6: 17.2 and 8.1, CI of the median log-ratio [1.51, 3.97] and [1.17, 3.12]. R7: +inf and +inf. R8: +inf and +inf. All p = 1.0, NO-GAIN.

Censoring sensitivity (my addition). Here a time-censored run is credited as if it finished at nodes_counted + 1, which is the best case for the candidate that the node data allow. The best-case paired median ratio against `bb-mrv` is then 1.36 (learned) and 1.13 (single) at R7, and 4.51 and 3.56 at R8. Variant runs stopped by the guard had already counted 590k to 999k nodes at R7 and 298k to 612k at R8, against a `bb-mrv` R8 median of 95k. So against `bb-mrv` the NO-GAIN label at every rung holds without the guard. Against `scb-gacolor_ac3_random` the guard leaves 2 to 7 pairs undecided at R7/R8. That cannot change the "worst over the three baselines" label, because `bb-mrv` already settles it.

Throughput from the raw rows (nodes_counted / wall): at R7, `bb-learned` 3.8k nodes/s vs `bb-mrv` 46k. At R8, `bb-learned` 1.7k vs `bb-mrv` 25k. `bb-impact` runs at 240 nodes/s at R8.

## Findings

Format: `file:line | severity | claim | problem | fix`. I applied the four adversarial lenses (design validity, scope, prose, code and reproducibility) and the rigor checklist directly, in one pass, and checked every finding below against the code or the raw rows.

- `PREREG.md:183 | should-fix (design, does not change verdict) | K1 compares the learned score against fail-first -\|D\| | Fail-first has a test AUC of 0.373, well below 0.5, so it is anti-correlated with backdoor membership, and any score that flips its sign clears K1 easily. Against signed +\|D\| the model adds +0.010 [-0.005, 0.025], and against impact alone -0.011. K1 not firing therefore says little about a backdoor-specific signal | Next time, use the stronger of fail-first and its sign-flip (chosen on training) as the Stage 1 comparator.`
- `README.md:176 | should-fix (claims vs data) | "Backdoor membership is predictable at the root above chance (AUC about 0.64)"; the runner's headline says "Backdoor cells are predictable at the root" | Restricted to interior cells, the learned AUC is 0.518 (signed \|D\| 0.488). Most of the 0.64 is position class (A3's backdoors are interior cells, and border cells score low) plus domain size. The README already says it is no better than signed domain size, but the headline is stronger than the data | Say "predictable mostly from position class and domain size; within interior cells AUC 0.52".`
- `README.md:68-69 | nitpick (prose) | "Backdoor cells are the cells with large root domains (interior cells with many neighbours)" | This holds on average, but the interior-only AUC of +\|D\| is 0.488, so within the interior a large domain does not mark backdoor cells | Qualify it to "interior cells, which have large domains".`
- `compute/src/bin/bb_features.rs:17 | should-fix (scope) | The Stage 1 model is applied to the Stage 2 ladder | The model was trained on A3 boards with 2 to 3 frame colours, then applied to ladder boards with 5 frame colours (`generate_solved_framed_bc(n, c+5, seed, true, 5)`), so Stage 2 tests transfer across colour regimes, not the Stage 1 distribution. This does not rescue the idea, since `bb-single` (impact) does not depend on the training regime and also loses | State the regime shift in Limits.`
- `README.md:183 | nitpick (numbers) | "about 4,000 nodes/s against about 40,000 for bb-mrv" | From the raw rows this holds at R7 (3.8k vs 46k), but at R8 it is 1.7k vs 25k | Give per-rung figures.`
- `compute/src/lib.rs:247 | nitpick (baseline strength) | `bb-impact` is the Refalo-style baseline | It does a full probe of every free cell at every node with no impact caching, which gives 240 nodes/s and 6 time-censored runs at R8. It is a weak-in-wall-clock version of impact search. This does not matter for the verdict, because `bb-mrv` is the binding baseline, and the guard rule turns a time-censored baseline into a tie, so it cannot favour the candidate | If impact is reused, cache impacts as Refalo does and probe only at the root or every k nodes.`
- `../hard-ladder/compute/src/instance.rs:91 | nitpick (protocol) | New ladder `8-c10` | PROTOCOL.md allows only configurations that the ladder README or a recorded amendment declares usable. The 8-c10 configuration follows the ladder README's own note (c >= 10 needed at 8x8) and the ladder JUDGE's recommendation, but no amendment was recorded in `hard-ladder`. Existing ladder files are byte-identical, and `check-instances` passes | Record 8-c10 as a ladder amendment when merging.`
- `README.md:218 | nitpick (budget) | Total about 2 h against the 90 min preregistered estimate | This is stated openly. The overrun comes from the probe-based solvers hitting the guard | None.`
- `PREREG.md:13-18 | nitpick (scope) | A3's labels are one backdoor per board (exact up to 7x7, greedy upper bounds above) | Backdoors are not unique, so membership labels are noisy, and greedy-UB labels make up 63 of the 150 test instances. The by-source split (exact Delta 0.283, greedy 0.237) shows the same pattern | Already in Limits.`
- Unplanned observation (correctly labelled as not preregistered): `bb-mrv` (A3 propagation plus MRV) beats the calibrated ladder baseline `scb-gacolor_ac3_random` at R6, R7 and R8 on these seeds (R8: 20/20 vs 12/20, KM median 95k vs 468k, and faster in wall clock). If this holds on fresh seeds, the ladder's "best baseline" is not the strongest available, and future candidates should be compared with `bb-mrv` too.

No blocking issues. I found no planted-solution leakage, no unequal effort accounting, no dropped censored rows and no error rows. Censoring follows PROTOCOL.md plus the preregistered guard rule. Clippy was not rerun.

## Next-round test

Promote `bb-mrv` (A3's propagation, piece all-different with hidden singles, MRV) to a ladder baseline candidate, under its own preregistration:

- Run it on fresh test seeds 21..30 at R6/R7 primary and 8-c10 at R8, plus calibration-free seeds 31..50 at 8-c10.
- Compare it with `scb-gacolor_ac3_random` under PROTOCOL.md G1, and add wall clock.
- Kill: G1 fails at 8-c10, or the median wall-clock ratio against scb is above 1.
- If it survives, re-baseline the round-2 candidates that were judged only against scb (A12, A17) on bb-mrv.

Separately, any further backdoor work should use backdoor structure for decomposition or restarts, not as a first-branch rule. The Stage 1 comparator should be the stronger-signed domain score, with an interior-only AUC reported.
