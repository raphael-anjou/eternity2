# Partial-match ladder (n = 8..12, planted optimum, score at fixed work)

An instrument, like the hard ladder, for the regime the real puzzle is in: the
best score on a board where no engine reaches a perfect solution. Planted
boards give a known optimum (every interior seam, 2n(n-1)); the metric is the
best verified score at a fixed amount of work, and the work needed to reach the
optimum minus k for k = 1..6. Preregistration: `PREREG.md` (commit `a796609`);
amendments: `PREREG_AMENDMENTS.md`.

## Limits (read first)

- Generated boards are not the Eternity II piece set. The generator is the hard
  ladder's framed generator (5 frame colours, c interior colours) without E2's
  frame census, rare-colour ring or clue pieces.
- The planted optimum here is 100% of the seams. The 16x16 optimum is unknown;
  470 of 480 is a gap to an unknown target, not to a known one.
- Run log: none (`results/run-log.jsonl` is absent; see "Corrections after the
  judgment"). Ordering of preregistration, calibration and test rests on commit
  order and file birth times.
- Verdict (`JUDGE.md`): kill, for this entrant pool and budget pair only.
- Nothing here is progress on the real puzzle, and no result on these boards is
  an Eternity II result. No 16x16 run was made.
- The five entrants are the repo's score-maximising engines that run at any
  board size. The engines with committed 16x16 scores (the `blackwood_bt` and
  `verhaard_faithful_v2` reimplementations, the beam producer, repair-study and
  dfs-study engines) are compiled for 16x16 and are not entrants (see
  `PREREG.md`, "Excluded").

## Outcome against the preregistered usability criteria (self-assessment; a separate judge decides)

**Not usable at the calibrated rung. Criterion (1) fails; criterion (3) is
undetermined at implementation level and fails at family level.**

| criterion | statistic (rung A: 8x8, c = 6, 30 test instances) | threshold | verdict |
|---|---|---|---|
| (1) stall band | 2 entrants with median gap in [2%, 4%] (`sa` 3.57%, `pt` 3.57%); best entrant below the optimum on 30 of 30 | at least 3 entrants; at least 25 of 30 | **fail** (entrant count) |
| (2) separation, implementation level | no established pair | | **undetermined** |
| (2) separation, family level | 3 of 3 family pairs significant after Holm (sign-flip Wilcoxon p < 1e-5 each) | all significant | pass (family level only) |
| (3) order, implementation level | no entrant has a committed 16x16 result from its own implementation | at least 3 linked entrants | **undetermined** |
| (3) order, family level | all 3 family pairs reversed: `sa` above `break-r1` (30/30 instances), above `break-r2` (30/30) and above `alns` (30/30) | same direction as the 16x16 | **fail** (family level) |

No rung B: no cell at n = 10 or 12 met the calibration rule (`CALIBRATION.md`).
At the preregistered budget the best entrant stalls 5.0% to 11.9% below the
optimum there; the break-tolerant entrants return no complete board on 47 of
64 calibration runs at those sizes, and their completed boards sit 3.3% to 11.1%
below the optimum.

What the instrument shows, read plainly:
- The band is reached at the calibrated rung by the two annealing entrants
  (8x8 c = 6, below the whole-board critical point: first-moment expected
  solutions 1.1e7, x = 0.25; the real 16x16 with 17 + 5 colours has x = 0.37).
  It is not reached only below critical: at 8x8 c = 10 (c - c_crit = +2.68)
  `break-r1` is in band at 3.57% (4 calibration seeds; the cell qualified, but
  rung A was chosen as the cell with more entrants in band). At every cell at
  n = 10 and 12, all entrants stall above 4%. *(Corrected after the judgment,
  finding 2; the earlier text said annealing-only and below-critical-only.)*
- The repo's break-tolerant DFS (`BreakSolver`) stops at its first completed
  board: on rung A it used at most 3.8e6 of its 5.12e7 placements and its score
  does not depend on the budget. It is a different object from the record
  backtrackers (Blackwood's search keeps going after a completed board).
- The ALNS configuration (`alns_only` defaults from a greedy start) is the
  weakest entrant everywhere; its 16x16 rows start from a beam board.
- Because no entrant is the implementation behind a committed 16x16 number, the
  ladder cannot yet be checked against the 16x16 order. The family-level
  reversal is confounded: the data cannot separate implementation from board
  size. The 16x16 side is measured on other implementations (single-core
  benchmark, repair-study and dfs-study rows) plus reported records
  (`EXPLORED.md`: 470, 467), so it is graded reported / other-implementation,
  and no verdict leans on it. *(Softened after the judgment, finding 3; the
  earlier text read the reversal as these implementations not standing in for
  their families' record engines.)*

## Corrections after the judgment (made at the commits after `226baf4`)

The verdict is kill, from `JUDGE.md` (`226baf4`). These corrections follow its
findings; none changes a verdict, a threshold, a raw row or `PREREG.md`.

- **Finding 1, tie handling.** The signed-rank test ranked differences of float
  shares, so equal integer score differences could split their ties.
  `ranks_abs` now ranks integer score differences (`analysis/analyze.py`,
  `int_score`). One reported number changes: `break-r2` vs `alns` goes from
  W = -183.0, p = 0.04717 to W = -196.0, p = 0.0315 (the judge's own
  recomputation gives the same). Every other W, p, median difference and win
  count in the regenerated output is identical to the committed one. No verdict
  depends on it: this is a descriptive pair, outside the three family pairs
  (each 30 / 0, p < 1e-5). The regenerated files carry the suffix
  `_corrected_after_judge`; the committed originals are untouched, and
  `analyze` now writes the suffixed names.
- **Finding 2, band reachability.** Corrected in "What the instrument shows".
- **Finding 3, family-level reversal.** Softened in "What the instrument shows".
- **Finding 4, PREREG wording (not edited).** `PREREG.md` says three entrants
  are linked by the family pairs; the pairs link four (`break-r1`, `break-r2`,
  `alns`, `sa`), as `summary.json` records. Wording only; `PREREG.md` stays as
  committed.
- **Missing run log (a stated limit, not a correction).** There is no
  `results/run-log.jsonl`: the preregistration guard was not on this branch and
  the test runs did not go through it. The order PREREG, calibration, test rests
  on commit order and file birth times only. Not invalidating (one PREREG commit,
  no threshold touched after any data), but the gap is real.
- `cargo fmt` was applied to `compute/src` (formatting only). `cargo clippy`
  reports no warning in this crate; two warnings remain in the
  `single-core-benchmark` path dependency (`best_rotation` unused, an unused
  `Puzzle` import) and are left alone.

## Test results, rung A

Generated by `python3 -I analysis/analyze.py test` from
`results/test/*/*.jsonl`. The committed `results/test_tables.md` and
`results/summary.json` are the pre-judgment output and are left as committed;
the corrected output is `results/test_tables_corrected_after_judge.md` and
`results/summary_corrected_after_judge.json` (see "Corrections after the
judgment"). Shares are best verified score / 112. Work to
optimum minus k is comparable only within a unit; `sa` work-to-target is an
upper bound within a factor 2 (doubling grid), `alns` points are
engine-reported (final board verified).

### Rung n8-c6 (30 test instances)

First-moment expected solutions 1.11e+07 (ln 16.2), x = c^2/(4(n-2)^2) = 0.250, whole-board critical c = 7.32 (c - c_crit = -1.32).

| entrant | unit | budget | share Q1 / median / Q3 | mean share | median gap | at optimum | no complete board | errors | median wall s | machines |
|---|---|---|---|---|---|---|---|---|---|---|
| break-r1 | placement | 5.12e+07 | 92.86% / 93.75% / 94.64% | 93.99% | 6.25% | 0 | 0 | 0 | 0.2 | m1 |
| break-r2 | placement | 5.12e+07 | 86.61% / 86.61% / 87.50% | 87.20% | 13.39% | 0 | 0 | 0 | 0.0 | m1 |
| sa | move_eval | 2.56e+08 | 95.54% / 96.43% / 96.43% | 96.25% | 3.57% | 0 | 0 | 0 | 100.3 | x86 |
| pt | move_eval | 2.56e+08 | 96.43% / 96.43% / 97.32% | 96.82% | 3.57% | 0 | 0 | 0 | 100.5 | x86 |
| alns | move_eval | 2.56e+08 | 86.83% / 88.39% / 89.29% | 88.15% | 11.61% | 0 | 0 | 0 | 21.8 | m1 |

Work to optimum minus k (Kaplan-Meier median, censored at the budget; reached / 30):

| entrant | k=1 | k=2 | k=3 | k=4 | k=5 | k=6 |
|---|---|---|---|---|---|---|
| break-r1 | > budget (0) | > budget (0) | > budget (0) | > budget (0) | > budget (5) | > budget (13) |
| break-r2 | > budget (0) | > budget (0) | > budget (0) | > budget (0) | > budget (0) | > budget (0) |
| sa | > budget (0) | > budget (1) | > budget (3) | 2.56e+08 (20) | 3.2e+07 (30) | 1.6e+07 (30) |
| pt | > budget (0) | > budget (0) | > budget (13) | 4.36e+07 (30) | 6.9e+06 (30) | 3.4e+06 (30) |
| alns | > budget (0) | > budget (0) | > budget (0) | > budget (0) | > budget (0) | > budget (0) |

Criterion (1): entrants in band 2, best below optimum on 30/30, pass = False

| pair | median diff of share | a wins / b wins | Wilcoxon W | p (sign flips) |
|---|---|---|---|---|
| break-r1 vs break-r2 | 6.70 pt | 30 / 0 | 465.0 | 0.00000 |
| break-r1 vs sa | -1.79 pt | 0 / 30 | -465.0 | 0.00000 |
| break-r1 vs pt | -2.68 pt | 0 / 30 | -465.0 | 0.00000 |
| break-r1 vs alns | 5.80 pt | 30 / 0 | 465.0 | 0.00000 |
| break-r2 vs sa | -8.93 pt | 0 / 30 | -465.0 | 0.00000 |
| break-r2 vs pt | -9.82 pt | 0 / 30 | -465.0 | 0.00000 |
| break-r2 vs alns | -0.89 pt | 11 / 18 | -196.0 | 0.03150 (corrected after the judgment, was -183.0 and 0.04717) |
| sa vs pt | -0.45 pt | 2 / 15 | -129.0 | 0.00090 |
| sa vs alns | 8.48 pt | 30 / 0 | 465.0 | 0.00000 |
| pt vs alns | 8.93 pt | 30 / 0 | 465.0 | 0.00000 |

implementation_level: {"level": "implementation", "verdict": "undetermined", "pairs": []}

family_level: {"level": "family", "pairs": [{"pair": ["break-r1", "sa"], "p": 0.0, "holm_significant": true, "median_diff_expected_positive": -0.017857142857142905, "agrees": false}, {"pair": ["break-r2", "sa"], "p": 0.0, "holm_significant": true, "median_diff_expected_positive": -0.0892857142857143, "agrees": false}, {"pair": ["alns", "sa"], "p": 0.0, "holm_significant": true, "median_diff_expected_positive": -0.08482142857142855, "agrees": false}], "linked_entrants": 4, "criterion2_pass": true, "criterion3_pass": false}


## Calibration

From `results/calibration/*/*.jsonl` by `python3 -I analysis/analyze.py
calibrate`; the choice and the exchange rate are in `CALIBRATION.md`. Median
relative gap per entrant (four calibration instances, scores in brackets;
`none` = no complete board).

| n | c | break-r1 median gap (scores) | break-r2 median gap (scores) | sa median gap (scores) | pt median gap (scores) | alns median gap (scores) | best | best below opt | in band | qualifies | E[solutions] | x | c - c_crit |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 8 | 6 | 5.80% (106, 104, 105, 106) | 12.95% (98, 99, 96, 97) | 3.57% (109, 108, 108, 108) | 2.68% (108, 110, 109, 109) | 11.16% (99, 100, 102, 93) | pt | 4/4 | 2 | yes | 1.11e+07 | 0.250 | -1.32 |
| 8 | 8 | 100.00% (none, none, none, none) | 12.50% (98, 98, 97, 100) | 6.70% (105, 104, 104, 105) | 7.14% (104, 104, 104, 104) | 18.30% (91, 91, 92, 92) | sa | 4/4 | 0 | no | 0.000769 | 0.444 | +0.68 |
| 8 | 10 | 3.57% (108, 108, 105, 110) | 12.50% (97, 97, 100, 99) | 9.38% (102, 102, 99, 101) | 8.93% (102, 102, 101, 102) | 22.32% (87, 87, 88, 87) | break-r1 | 4/4 | 1 | yes | 1.14e-11 | 0.694 | +2.68 |
| 8 | 12 | 4.02% (107, 108, 108, 106) | 11.16% (97, 99, 100, 103) | 12.50% (98, 98, 98, 97) | 10.27% (99, 99, 112, 102) | 26.34% (85, 82, 82, 83) | break-r1 | 4/4 | 0 | no | 4.34e-18 | 1.000 | +4.68 |
| 10 | 8 | 100.00% (none, 170, none, none) | 9.72% (164, 161, 164, 161) | 5.83% (168, 170, 169, 170) | 5.00% (172, 171, 171, 171) | 13.06% (154, 156, 157, 161) | pt | 4/4 | 0 | no | 1.91e+13 | 0.250 | -1.94 |
| 10 | 10 | 100.00% (none, none, none, none) | 10.83% (161, 162, 160, 160) | 8.06% (163, 167, 165, 166) | 7.50% (166, 166, 167, 167) | 15.28% (155, 153, 152, 147) | pt | 4/4 | 0 | no | 0.456 | 0.391 | +0.06 |
| 10 | 12 | 100.00% (none, none, none, none) | 100.00% (none, none, none, none) | 11.11% (161, 160, 160, 160) | 9.72% (161, 163, 162, 163) | 18.33% (146, 148, 149, 137) | pt | 4/4 | 0 | no | 3.28e-12 | 0.562 | +2.06 |
| 10 | 14 | 100.00% (none, none, none, 174) | 100.00% (none, none, none, 160) | 13.33% (156, 156, 155, 156) | 11.94% (158, 159, 159, 158) | 23.61% (138, 137, 133, 139) | pt | 4/4 | 0 | no | 1.7e-21 | 0.766 | +4.06 |
| 12 | 10 | 100.00% (none, none, none, none) | 8.14% (243, 242, 244, 240) | 7.77% (243, 243, 244, 245) | 5.68% (251, 248, 249, 249) | 12.69% (231, 234, 228, 230) | pt | 4/4 | 0 | no | 1.72e+21 | 0.250 | -2.54 |
| 12 | 12 | 100.00% (none, none, none, none) | 54.55% (none, 240, none, 241) | 10.04% (238, 236, 238, 237) | 7.95% (244, 243, 243, 243) | 15.53% (227, 225, 220, 221) | pt | 4/4 | 0 | no | 1.39e+04 | 0.360 | -0.54 |
| 12 | 14 | 100.00% (none, none, none, none) | 100.00% (none, none, none, none) | 12.12% (232, 232, 232, 230) | 9.85% (238, 237, 239, 238) | 19.89% (213, 210, 215, 210) | pt | 4/4 | 0 | no | 5.25e-11 | 0.490 | +1.46 |
| 12 | 16 | 100.00% (none, none, none, none) | 100.00% (none, none, none, none) | 13.64% (228, 228, 226, 229) | 11.36% (233, 232, 236, 235) | 21.97% (205, 198, 208, 207) | pt | 4/4 | 0 | no | 1.85e-23 | 0.640 | +3.46 |

Choice: {'A': (8, 6, 'in band')}

The three last columns place each cell relative to the real puzzle's regime:
the balanced first-moment expected number of solutions of the generator's
ensemble (formula of the refereed review `growth-law.REVIEW.md`, scripts in
`growth-law-review/rvlib.py`, branch `night/2026-11-12-lead`; the memo's
earlier uniform-colour numbers were refuted and are not used), x = c^2 / (4
(n-2)^2), and c minus the whole-board critical colour count (4.65, 5.99, 7.32,
8.63, 9.94, 12.54 at n = 6..10, 12; 17.68 for 16x16 with 5 frame colours, where
c = 17 gives x = 0.37). These columns are descriptive; they never gated
anything. The generator call cannot vary the number of frame colours (the
hard ladder's `generate` fixes 5); it was not modified.

## Instrument

- Instances: `instances/cal-n<n>-c<c>.jsonl` (seeds 7001..7004),
  `instances/test-n8-c6.jsonl` (seeds 7101..7130). Generator:
  `hard_ladder::instance::generate(n, c, seed)` (framed, 5 frame colours, the
  hardness-peak scramble). `pml check <file>` regenerates every row and
  verifies the planted board at the optimum.
- Verifier: `compute/src/verify.rs`. Empty cell, out-of-range or reused piece,
  or rim violation is an error row. The harness re-verifies every returned
  board and checks it equals the last trajectory point. Error rows: 0 of 432.
- Work counter: `compute/src/work.rs`. `placement` for `break-*` (BreakSolver
  `nodes`), `move_eval` for `sa`, `pt`, `alns`. Budget pair per size:
  4e6 n^2 move evaluations and rho = 0.2 times that in placements.
- Entrants: `compute/src/entrants.rs` (configurations and exclusions in
  `PREREG.md`). `pt` is a sequential twin of `run_pt_from`, tested equal
  (`pt_twin_matches_run_pt_from`).
- Determinism: all five entrants give identical score, work, trajectory and
  board on `x86` (x86_64 Linux) and `m1` (arm64 macOS)
  (`results/determinism.json`). Every row records its machine.

## Reproduce

```
just experiments partial-match-ladder check             # build, unit tests, instance check
just experiments partial-match-ladder analyze           # every table above from the raw rows
just experiments partial-match-ladder rerun-test 8 6 4  # re-run rung A (about 2 thread-hours)
```

Compute used: probe 0.27, calibration 6.94, test 2.03 thread-hours (sum of
per-row wall clock), on `x86` (Linux x86_64, shared) and `m1` (Apple M1,
shared), 2026-11-12. Wall-clock columns are machine-dependent; scores and work
are not.

