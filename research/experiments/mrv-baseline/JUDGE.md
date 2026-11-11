# JUDGE: A22, bb-mrv as a ladder baseline (branch devin/r3-mrv-baseline, commit 59f1b22)

## Verdict: keep

The experiment is valid and the binding kill criterion does not fire. At 8x8 c=10 on fresh seeds 21..50, bb-mrv passes PROTOCOL.md G1 against scb-gacolor_ac3_random (paired node ratio 0.147, 95% CI [0.076, 0.313]) and is faster in wall clock (median paired ratio 0.194, not above 1). The binding Q2 claim is correctly "no evidence of a growing gain". Adoption is supported at the R8 rung only; at R6 and R7 bb-mrv fails G1 and is slower in wall clock, and Q2 is uninformative above n=8 because every pair is censored.

## Reasons

1. Preregistration order holds. `ac13493` (PREREG.md) precedes `b6daa50` (hard-ladder A4 amendment and new instance files), `424b4c8` (arms and runner) and `59f1b22` (all raw results). The kill rule in PREREG.md line 144 matches the binding statement word for word in substance (G1 PASS at 8x8 c=10 seeds 21..50, and median paired wall ratio not above 1). A4 in hard-ladder PREREG_AMENDMENTS.md only adds `8-c10-fresh` (seeds 31..50) and `plus2` (c = primary + 2), states the kill criterion unchanged, and was committed before any run. No A22 amendment exists. No post-hoc change found.
2. Numbers reproduce. My own script over committed raw JSONL reproduces every headline number. I rebuilt the compute crate and reran the full binding R8 subset (bb-mrv and scb, 60 runs, 167 s): 60/60 node counts and statuses identical to the committed rows; the rerun wall ratio is 0.173 (CI [0.087, 0.325]), also below 1. `cargo test` passes (5 tests). `hard_ladder check-instances` reports the instances match the generator; the R8 seeds 21..30 file is identical to the earlier hard-ladder file and overlapping scb rows match the prior committed hard-ladder results.
3. Node accounting is common. Every arm calls `NodeCounter::try_assign` once per (cell, piece, rotation) before propagation; forced cells and hidden singles go through the same counted path in lib.rs; scb is wrapped (scb.rs) so its native `ValueTried` events are replayed through the same counter with the same cap and guard. scb's own pruning work is not counted as nodes, but that is a per-node cost which the wall-clock criterion captures, and bb-mrv also wins wall clock at R8.
4. No planted-solution leakage. The `LadderSolver` trait (hard-ladder solver.rs:18-23) receives only the scrambled puzzle; `inst.planted` is read only in runner.rs:66 after a board is returned and verified, to set `equals_planted`. The planted board is a generator property, not a solver input.
5. Censoring is handled as preregistered. At R8 no row is time-censored (all censored rows hit the 1e6 node cap; max wall 53 s against a 600 s guard), so the "time-censored baseline scores 0" rule does not touch the binding comparison. scb's 10 censored rows are scored ln(c/C) against the cap, which is conservative for bb-mrv.
6. The baseline is the one the statement names and is not a weak draw. The binding scb uses run seed 1 as specified. As a judge-side sensitivity check (not preregistered, does not ride on the verdict) I reran scb at R8 with run seeds 2..5: G1 still passes at every seed (ratios 0.255, 0.273, 0.177, 0.146; CI upper ends 0.381, 0.401, 0.313, 0.203). Seed 1 is in the middle of the range, not a lucky low.
7. No node win hides a wall-clock loss at the binding rung. It does at the small rungs (R6 wall 1.64, R7 wall 1.85), which the README reports; the adoption is scoped to R8.

## Recomputed numbers (own script, committed raw results)

| set | bb-mrv solved | scb solved | KM median bb / scb | paired node ratio [95% CI] | G1 | median paired wall ratio |
|---|---|---|---|---|---|---|
| R6 6x6 c=5 s21-30 | 10/10 | 10/10 | | 0.799 | fail | 1.636 |
| R7 7x7 c=6 s21-30 | 8/10 | 7/10 | | 0.700 | fail | 1.854 |
| R8 8x8 c=10 s21-50 (binding) | 30/30 | 20/30 | 69,036 / 555,342 | 0.147 [0.076, 0.313] | PASS | 0.194 (committed CI [0.106, 0.353]; rerun 0.173 [0.087, 0.325]) |

KM medians use the lower order statistic (15th of 30); the paired ratio uses the upper median, as PROTOCOL.md specifies.

Q2 (+2 track, cap 1e6): per-size median paired log10 ratio n=6 -0.255, n=7 -0.379, n=8 -0.832, n=9 0 (30/30 both-censored), n=10 0 (30/30 both-censored). Binding slope +0.0888 per n, my bootstrap CI [+0.063, +0.126] (committed [+0.065, +0.124]; difference is resampling noise). Not below zero: "no evidence of a growing gain". Extended runs at n=9 and 10 (1e7 nominal, 180 s guard): every row is time-censored before 1e7 (bb-mrv 2.69M to 3.48M nodes at n=9 and 1.26M to 2.64M at n=10; scb 3.50M to 4.60M and 1.66M to 4.19M), so both caps give 30/30 both-censored.

Exploratory n=6..8 slope: -0.289 per n, CI [-0.447, -0.101]. Not preregistered; it carries no claim.

Raw sanity: 0 error rows, 0 inconsistent rows, no solved row with fewer nodes than board cells.

scb run-seed sensitivity at R8 (judge-side, exploratory): seed 1 20/30 solved, ratio 0.147; seed 2 19/30, 0.255; seed 3 24/30, 0.273; seed 4 18/30, 0.177; seed 5 18/30, 0.146. G1 PASS at all five.

## Findings

`file:line | severity | claim | problem | fix`

- `research/experiments/mrv-baseline/README.md:270` and `results/tables.md:113` (generated by `analysis/analyze.py:180`) | moderate (prose) | exploratory n=6..8 slope line ends "claim: the gain grows with n" | the post-hoc subset reuses the binding claim string, so a reader skimming the table sees the binding claim attached to a non-preregistered analysis; the README prose elsewhere labels it a lead, but the table line contradicts that | emit "exploratory, no claim" for the subset slope in analyze.py and regenerate the tables
- `research/experiments/mrv-baseline/README.md:130` | minor (claims vs data) | "Free MRV order (versus border-first) is the largest single bb-side component at R8" | mrv-bf 1.83 [1.65, 2.08] against mrv-rand 1.62 [0.89, 2.12]: the CIs overlap and single-change ablations do not decompose additively, so a ranking is not supported | write "largest point estimate; the mrv-bf and mrv-rand intervals overlap"
- `research/experiments/mrv-baseline/PREREG.md:109` | moderate (design) | extended cap 1e7 answers Q2 where 1e6 is censored | the 180 s guard binds long before 1e7 at both sizes, so the "extended cap" is a wall-clock cap and gives the two arms unequal node budgets (bb-mrv reached fewer nodes per run than scb because it is slower per node at n=9,10). Both arms stayed fully censored, so it does not bias any reported number, but the rule could not have produced information | in the follow-up, cap on nodes only (guard far above expected time) or state the cap as equal wall time per arm
- `research/experiments/mrv-baseline/PREREG.md:134` | moderate (design) | binding Q2 slope over n=6..10 | both-censored pairs score 0, so any size where nothing solves pulls its median to 0 and forces a positive slope when smaller sizes show a gain. The binding slope +0.089 is a censoring artifact, not evidence of a shrinking gain. The binding wording ("no evidence of a growing gain") is the correct reading, and the README says so | next round: preregister the slope only over sizes with at least 10 both-solved pairs, with caps chosen on calibration seeds to reach that
- `research/experiments/mrv-baseline/README.md:16` | minor (scope) | "bb-mrv survives as a ladder baseline candidate at that rung" | correct, but R6 (wall 1.64) and R7 (wall 1.85, node ratio 0.70) show bb-mrv is not a better baseline on the primary ladder, so "adopted" must stay scoped to 8x8 c=10 | add one sentence that the primary ladder R6/R7 keeps scb as baseline, or report both
- `research/experiments/mrv-baseline/compute/src/main.rs` (scb arm, run_seed 1) | minor (baseline strength) | single scb run seed represents the baseline | value-order randomness is part of scb; one seed is a single draw. Judge-side seeds 2..5 all pass G1 (ratios 0.146 to 0.273), so the verdict is robust, but the committed evidence alone does not show this | for a ladder baseline decision, preregister scb over at least 5 run seeds and report the spread
- `research/experiments/mrv-baseline/results/tables.md` (propagate_calls_per_node column) | minor (reporting) | "propagation calls per node" reported for every arm | the value is 1.00 by construction for all arms, so it carries no information about per-node cost; the real cost signal is nodes/s and scb pruned rows per node | report propagation work in a common unit (domain values removed per node) or drop the column and point to nodes/s
- `research/experiments/hard-ladder/compute/src/runner.rs:66` | info | planted-solution leakage check | planted board is used only for the post-hoc `equals_planted` flag; solvers never see it | none
- `research/experiments/mrv-baseline/compute/src/scb.rs` | info | scb nodes counted through the ladder NodeCounter | ValueTried replay through `try_assign` with the same cap and guard; verified 20/20 rows identical to prior hard-ladder scb results | none

No blocking issue. No em dash or banned phrase found in README.md, PREREG.md or analyze.py.

## Next-round test

Q2 is unanswered. Preregister: the +2 track at n = 6..10 (c = 7, 8, 10, 11, 12), fresh seeds (n=8: new seeds 51..80; others 31..60), bb-mrv against scb at 5 scb run seeds (median over seeds per instance as the baseline). Per size, choose the node cap on calibration seeds 1001..1010 as the smallest power of 10 at which both arms solve at least 8/10, up to 1e9; nodes-only cap with a wall guard at least 10x the calibration median time. Binding Q2: OLS slope of the per-size median paired log10 node ratio over sizes with at least 10 both-solved pairs (at least 4 sizes required, else "uninformative"); "the gain grows with n" only if the 95% bootstrap CI over instances is entirely below 0. Also report the slope of the median paired log10 wall ratio, so a growing node gain cannot hide a flat wall-clock ratio. Kill for the scaling claim: CI not entirely below 0, or fewer than 4 informative sizes.
