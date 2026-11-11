# Judge report: ProbCut on the hard ladder (devin/r2-probcut @ 7a3aef7)

## Verdict: kill

The experiment is valid, and its own kill criterion fired together with the round's G1 rule at 8x8. The cutoffs never reduced nodes to first solution on any rung. On R8 (8x8 c=10) they removed solutions that the same DFS finds: v1 solved 13/30 and v2 17/30, against 19/30 for fc-dfs. Every comparison is NO-GAIN under G1 (Holm). No post-hoc analysis rescues the method. The one post-hoc signal found (below) points at a different configuration and is a next-round hypothesis, not a positive result.

## Reasons

1. Preregistration chronology holds. `PREREG.md` was committed in 7ae11f4 (13:48:28 UTC). The first results commit is 6d7c361 (13:54:16), followed by test runs in ccada34 (13:57:39) and the README in 7a3aef7 (14:01:49). `PREREG.md` is unchanged after 7ae11f4. Amendment B1 in `PREREG_AMENDMENTS.md` is in 8626045, before any calibration row. It resolves the v_0 / "k cells deeper" indexing conflict in the original write-up, where v_0 is the immediate-extension count, by having v_j count nodes j+1 cells deeper. The idea's own text matches that reading, and B1 changes no threshold or statistic. It is not a post-hoc change. The Luby-unit choice, pooled across rungs, was recorded in `CALIBRATION.md` (6d7c361) before test runs, so it is a calibration decision and not a test-informed one.
2. The kill criterion matches the binding statement. Stage 1 (AUC gain, R^2, held-out false-cut 2p) and stage 2 (0.7x the better baseline on both rungs, or first pass failing on more than half) are copied faithfully. Two adaptations are declared before any run: R7/R8 instead of 8x8/9x9 to fit the ladder, and "at most 15/30" for "more than half fail". The round rule adds G1 failure at 8x8, counting every node spent on failed passes and re-searches. The code meets that rule: one `NodeCounter` is shared across all passes and probe placements go through `try_assign()` (search.rs:210, 339).
3. Stage 2 fires on direct measurements. On R8, v1's KM median is censored (> 1e6). It fails 0.7x against fc-dfs (508,613), scb (505,364) and every other baseline. v1's first pass solved 13/30, which is at most 15. v2 is no better in substance: median 550,557 vs 508,613 for fc-dfs, and 2 lost solves with no gains. On R6 the cuts cost extra nodes: v1 2.55x fc-dfs, v2 1.06x.
4. G1 at 8x8 fails for both variants against the ladder baseline scb-gacolor_ac3_random and against the experiment's own base fc-dfs. The median ratio is above 0.5, and the CI upper end is not below 0 or the candidate median is censored.
5. Stage 1 fires only through its AUC-gain clause, and that clause is close to vacuous here (finding F1). Because stage 2 fires on its own and the round rule needs the own kill AND G1, the verdict does not depend on stage 1.
6. Leakage: none found. Solvers receive only the scrambled `Puzzle` through the ladder interface. The planted solution is read only by the stage-1 labelling and by the test-seed false-cut diagnostic, which runs as a separate pass and has no effect on the search. Calibration (seeds 1001..1010, `params.json` committed in 6d7c361 and unchanged since) and test seeds (1..30) are disjoint.
7. Effort accounting: nodes mean the same thing for every solver, one per rim/neighbour-consistent (cell, piece, rotation) assignment, with probe placements charged the same way. No node win hides a wall-clock loss, because there is no node win. ProbCut wall time on R8 (1.6 s total) equals fc-dfs.
8. Censoring is handled correctly. KM treats censored runs as infinite, a censored candidate against a solved baseline counts as a loss in G1, and both censored is a tie. All rows have cap 1,000,000, with no error rows.

## Recomputed numbers

My own script (`recompute.py`, not committed) recomputed these from the committed JSONL and matched every table entry in `results/RESULTS.md` and `README.md`.

| rung | solver | solved | KM q25 / median / q75 |
|---|---|---|---|
| R6 6x6 c=5 | scb-gacolor_ac3_random | 30/30 | 620 / 951 / 1,940 |
| R6 | hp-dfs | 30/30 | 27,904 / 41,823 / 62,556 |
| R6 | fc-dfs | 30/30 | 25,961 / 38,866 / 56,883 |
| R6 | fc-luby | 30/30 | 32,290 / 74,830 / 144,530 |
| R6 | probcut-v1 | 30/30 | 26,858 / 171,321 / 248,763 |
| R6 | probcut-v2 | 30/30 | 26,946 / 41,315 / 77,397 |
| R7 7x7 c=6 | scb-gacolor_ac3_random | 16/30 | 272,281 / 921,773 / inf |
| R7 | hp-dfs, fc-dfs, fc-luby, v1, v2 | 0/30 each | censored |
| R8 8x8 c=10 | scb-gacolor_ac3_random | 19/30 | 363,902 / 505,364 / inf |
| R8 | hp-dfs | 19/30 | 360,947 / 509,862 / inf |
| R8 | fc-dfs | 19/30 | 360,829 / 508,613 / inf |
| R8 | fc-luby | 16/30 | 306,878 / 820,909 / inf |
| R8 | probcut-v1 | 13/30 | 395,888 / inf / inf |
| R8 | probcut-v2 | 17/30 | 408,107 / 550,557 / inf |

- G1 median ratios on R6: v1/scb 109.03, v2/scb 34.16, v1/fc-dfs 2.55, v2/fc-dfs 1.06, v1/hp-dfs 2.27, v2/hp-dfs 0.98. Bootstrap CIs (seed 20261111) match the committed ones, for example v1/scb log CI [+4.10, +5.17].
- R8 paired against fc-dfs: v1 loses seeds 3, 5, 7, 19, 21, 23, 27 and gains seed 2. v2 loses seeds 7 and 27 and gains none. On the 17 instances both v2 and fc-dfs solve, per-instance node ratios run from 0.70 to 1.10.
- Passes: R6 v1 used 1 pass on 27 instances and 2 passes on 3; R6 v2 used 1 pass on all 30. On R7 and R8 every candidate run ended in pass 1, so the relaxation schedule never ran there.
- Test-seed false-cut rates (planted path cut at any test depth): R6 0.700 (p=0.2), 0.067 (p=0.05); R7 0.433, 0.133; R8 0.300, 0.133. The calibration held-out R8 rates, 0.067 (p=0.05) and 0.200 (p=0.2), come from a different split and also match.
- Stage 1 on R8: 6 sibling sets in total. The v0..v4 values are identical between the planted and sibling child in 5 of the 6. AUC gain is 0.000 at every depth, and best-k R^2 is 0.12 / 0.17 / 0.61.
- R8 total wall seconds: scb 597.8, hp-dfs 20.2, fc-dfs 1.6, fc-luby 1.8, v1 1.6, v2 1.6.
- Rerun: `run.sh` on a clean worktree at 7a3aef7 took 2 min 24 s wall (13.2 CPU-min on 8 vCPU). It reproduced every committed JSONL file exactly, ignoring `wall_ms`, plus `params.json` and `CALIBRATION.md`. The only differences are wall-time fields in `RESULTS.md`, `summary.json` and the two G2 files. `cargo test --release` passes 6 of 6, and `cargo clippy --release --all-targets -- -D warnings` is clean.

## Findings

The four adversarial lenses (design validity, scope, prose, code and reproducibility) were run by this judge directly, without separate reviewer sessions. Every finding below was checked against code or committed rows.

file:line | severity | claim | problem | fix
---|---|---|---|---
research/experiments/probcut/README.md:119 | should-fix | "a subtree-size probe carried no planted-path information beyond the forward-checking count" | At a row-boundary node (d = n, 2n, 3n), the planted child and its forward-checked siblings sit in the last cell of a row. They share top, left and right (border) colours, so they differ only in the bottom colour and the piece used. A probe of at most 5 cells into the next row never reaches the cell below that last cell at n >= 6, so only piece-usage effects can show. That is why v0..v4 are identical in 5 of 6 R8 sets and 8 of 17 R6/R7 sets. The null is mostly structural, not a measurement of quiet planting. The committed L8 column (8 cells deeper) does separate the classes: the planted child has the higher L8 in 6 of 6 R8 sets (post-hoc, n = 6). | Scope the sentence: "at row-boundary tests a probe shorter than one row cannot see the sibling difference; this run does not measure whether deeper probes carry signal". Do not record the stage-1 kill as the statistical counterpart of the "fades two cells out" wall.
research/experiments/probcut/PREREG.md:133 | should-fix | Stage-1 AUC-gain clause | With 6 R8 sibling sets, most of them identical in v0..v4 by construction, the clause fires whatever the signal. It carries almost no evidential weight. | Report stage 1 as underpowered in the ledger line. Make stage 2, which fires independently, the basis of the kill.
research/experiments/probcut/results/RESULTS.md:7 | minor | Gaussian thresholds "act as random cuts at the stated rate" (README.md:123) | With 3 tests per path, the nominal cumulative rate at p = 0.2 is at most 1 - 0.8^3 = 0.49. R6 shows 21/30 = 0.70, so calibration does not transfer from calibration to test seeds at R6. R8 (0.30) is within the nominal bound. | Say "at roughly the stated rate on R8; over-cutting on R6 (0.70 vs at most 0.49)".
research/experiments/probcut/PREREG.md:139 | minor | 9x9 is replaced by R7 (7x7 c=6) | Every forward-checked solver is 0/30 at the R7 cap, so the R7 half of the stage-2 clause is vacuous and the kill rests on R8 alone. This is declared before the run, so it is not post-hoc. | State in the README that R7 cannot discriminate the FC-based solvers at this cap.
research/experiments/probcut/PREREG.md:74 | minor | The method is "the same DFS plus cutoffs" | The cuts sit on fc-dfs, not on the ladder's strongest complete DFS (scb-gacolor_ac3_random), as the binding statement asked. The stage-2 clause takes the best of fc-dfs, fc-luby and scb, and v1/v2 lose to fc-dfs itself, so the verdict is unaffected. | Keep it as is. A future variant should put the cuts on scb to test the binding statement literally.
research/experiments/probcut/README.md:88 | minor | Wall-clock columns | scb wall time is not comparable to the FC solvers (full AC-3 per node), as stated. This does not affect the verdict. | None needed beyond the existing caveat.

No blocking issues.

## Next-round test

The idea that survives is narrower: do deep (at least one row) subtree-size probes predict barren subtrees well enough to pay for themselves?

Preregister the following before any run, on fresh calibration seeds. On R8 (8x8 c=10), compute v_k at k + 1 in {n, n+2, 2n} cells deeper at row-boundary nodes, plus mid-row nodes at d = n + n/2. Use at least 60 sibling sets, raising the sibling count by sampling more calibration instances until 60 sets exist. Run the planted-vs-sibling AUC against v_0 with a paired bootstrap.

- Kill if the AUC gain is < 0.05 or its CI includes 0.
- If not killed, put the cut on scb-gacolor_ac3_random and on fc-dfs with probes charged through `try_assign()`. Kill if G1 at R8 is not PASS against the uncut solver with all probe and re-search nodes counted, or if the solve rate drops below the uncut solver's.

The committed L8 column (6/6 on R8) motivates this test. That column is post-hoc and must not be reused as evidence.

