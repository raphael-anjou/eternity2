# PREREG: difference map with exact band projections (track 11, night 2026-11-12)

Track label: **method** (added by the lead at 22:45; not in NIGHT_BRIEF.md).
Generated boards only; nothing here is official Eternity II progress and no
claim extends past 8x8.

Commit history note: a work-in-progress commit (961d1aa: the Rust port, the
memo-instance exporter and the memo comparison rows, calibration seeds and memo
instances only) precedes this file, at the lead's instruction at 23:53. This
commit contains PREREG.md alone. No test seed has run before it (the run log
`results/run-log.jsonl` does not exist yet).

## Source and hypothesis

Source: `research/program/night/2026-11-12/memos/solve-path-6-free.md`
(unrefereed claim) and its referee
`memos/literature-and-path-6.REVIEW.md` rows P1 to P9.

Method: divide and concur with relaxed difference-map step beta over exact
projections: A (layouts, each piece once, one Hungarian assignment), B (seam
subspace, facing sides averaged, rim grey), and for k >= 1 the band sets R_k
(rows partitioned into bands of k consecutive rows from row 0; each band a
fully matched tiling with repeats allowed; exact max-plus broken-profile DP
whose state is the k lane colours plus one vertical colour) and K_k (same for
columns). k = 0 means A against B only. The last band is shorter when k does
not divide n (n = 7: k = 2 gives 2,2,2,1; k = 3 gives 3,3,1; n = 8: k = 3 gives
3,3,2). Update: pc = P_B(mean of replicas); X_r += beta (P_r(2 pc - X_r) - pc).
Stop at the first iteration whose A layout has zero mismatched seams. Start:
uniform [0,1) per entry from SplitMix64 seeded by (instance seed, run seed);
the A replica is drawn first, so it starts identically for every k.

**Hypothesis (memo section 6).** The growth rate with board size of the
iterations to an exact solution FALLS as the band width k grows.

## Kill criterion (copied unchanged from the lead's brief)

> KILL, written before the run and copied into PREREG.md unchanged: dead if
> (a) s_3 > 0.5 x s', or (b) s_k does not fall by at least 25% from k = 0 to
> k = 3 with a bootstrap 95% interval excluding zero change. Passing both
> proves nothing about 16x16; it buys a second stage.

Exact statistics:

- s_k: OLS slope of log10 m_{n,k} on cells n^2 over the primary rungs
  n = 6, 7, 8 (36, 49, 64 cells), where m_{n,k} is the Kaplan-Meier median over
  the 30 instances (15th order statistic, censored = +infinity) of the
  per-instance value, and the per-instance value is the median over the 3 run
  seeds of iterations to an exact solution (2nd order statistic of 3, censored
  = +infinity).
- s': the same OLS slope for `bb-mrv` nodes to first solution (one
  deterministic run per instance), cap 1,000,000 nodes.
- Censoring, as the hard ladder's G2: a censored candidate rung median makes
  that s_k = +infinity; a censored baseline rung median is set to the cap
  (a lower bound, which lowers s').
- (a) fires iff the point estimate s_3 > 0.5 x s' (literal; if s' <= 0 then
  any s_3 > 0.5 s' fires it).
- (b) passes iff both: relative fall (s_0 - s_3) / s_0 >= 0.25 on point
  estimates (if s_0 = +infinity and s_3 finite, the fall counts as >= 25%; if
  s_3 = +infinity, or s_0 <= 0 finite, it fails), AND the upper end of the
  95% percentile bootstrap interval of s_3 - s_0 is < 0 (a resample where both
  are +infinity scores 0; +infinity minus finite scores +infinity; finite minus
  +infinity scores -infinity). (b) fires otherwise.
- Bootstrap: 10,000 resamples, seed 20261111, resampling the 30 instances
  with replacement within each rung, the same resampled instances for every k
  and for `bb-mrv` (paired).
- Dead iff (a) or (b) fires. The kill is read on the primary rungs only.

**Fallback reading, stated before the run.** Calibration (below) solved 0 of
10 calibration instances at n = 7 and n = 8 for every k within 10,000
iterations (n = 8, k = 3 only 2,000). If the test rung medians at n = 7 or 8
are censored, every affected s_k is +infinity: (a) fires because +infinity
> 0.5 s', and (b) fires (no finite fall). The verdict is then "dead by
censoring within the preregistered cap", which is a statement about this cap
and this budget, not a measured exponent. In that case the report adds, as
descriptive and non-binding: solved fractions per (rung, k) and the
distribution of best and terminal mismatches of censored runs per (rung, k).
No other reading is substituted for the kill rule.

## Instances and rungs

- Binding: hard-ladder `primary` test instances, n = 6 (c = 5), 7 (c = 6),
  8 (c = 8), test seeds 1..30, `research/experiments/hard-ladder/instances/primary-test.jsonl`
  (colour counts checked in the file: numColors 10, 11, 13 = 5 frame + c
  interior; Owen criterion 5.22, 6.38, 7.56; the reviewed critical counts the
  lead gives are 4.65, 5.99, 7.32).
- Easy-regime arm, labelled, not binding: `8-c10` test seeds 1..30
  (c = 10, about 2.7 colours above critical), k = 0, 1, 2, 3, same cap.
- Calibration (tuning only): primary calibration seeds 1001..1010 at
  n = 6, 7, 8; `8-c10` calibration seeds 1001..1010; and the memo's own nine
  instances (`calib/memo-instances.jsonl`).

## Arms and controls

- dm-k0 (control: everything except band projections; shares cap, step, run
  seeds and starts of the A replica), dm-k1, dm-k2, dm-k3.
- Run seeds: 3 per (instance, k): run seeds 1, 2, 3.
- Baselines, rerun by this track on this Mac, single thread, cap 1,000,000
  nodes, through hard-ladder's `NodeCounter` (`mrv_baseline` binary of
  `research/experiments/mrv-baseline`, unchanged code): `bb-mrv` (the kill
  rule's s') on primary n = 6, 7, 8 and on `8-c10`; `scb-gacolor_ac3_random`
  (the ladder's calibrated baseline at R6/R7) on the same instances, reported.
  Every ratio in the report names its baseline.
- A random-value-order control (rule 3) does not apply: the map has no cell
  or value order; its stochastic element is the start, covered by 3 run seeds
  and the k = 0 control arm.

## Frozen parameters (tuned on calibration seeds only)

- beta = 0.25 (memo value). Calibration at k = 0 and 1, n = 6 and 7, cap
  20,000, beta in {0.1, 0.25, 0.5, 0.8}: n = 6 solved 4, 8, 6, 9 of 10 (k = 0)
  and 5, 6, 8, 8 (k = 1); n = 7 solved 0, 0, 1, 0 (k = 0) and 0 of 10 for every
  beta (k = 1). No beta is clearly better; the memo value is kept
  (`calib/cal2/`).
- Iteration cap: 20,000 per run, identical for every k, rung and ladder.
- Wall-clock guard: none (iteration cap only).
- Weights: equal replica weights in the concur average.

## Measurements per run

Status (solved / censored), iterations to solution, wall ms, us per
iteration, DP transitions (a work count), best and terminal mismatched seams,
cycle detection on censored runs (smallest period p <= 50 with layout(i) =
layout(i - p) for all of the last 200 iterations; number of distinct layouts
in the last 2,000), equals_planted, and the board. Every solved board is
re-verified twice: in Rust (same rules as hard-ladder `verify_board`) and by
the analysis script's independent Python checker against the instance file.
Any verification failure makes that row invalid (counted as censored and
reported).

## Cost axis

Per rung and arm: KM median and total wall clock, single thread, same Mac,
for dm-k and `bb-mrv`. Calibration measured cost per iteration (us, median):
k = 0: 31 / 64 / 129 (n = 6 / 7 / 8); k = 1: 83 / 177 / 347; k = 2:
178 / 354 / 838; k = 3: 1,028 / 2,063 / 6,144; 8-c10 at k = 3: 9,107. One
`bb-mrv` node costs about 40 us at 8x8 (A22: 66,382 nodes in 2.56 s), so one
k = 3 iteration at 8x8 costs about 150 `bb-mrv` nodes of wall time and one
k = 0 iteration about 3.

## Budget and the largest k per rung

Worst case (every run censored): k = 3 at primary n = 8 is 90 x 20,000 x
6.1 ms = 3.1 core-hours; at n = 7 1.2; at n = 6 0.5; k = 2 all rungs 0.7;
k = 0 and 1 0.4; easy arm k = 0, 1, 2 0.8, k = 3 5.1 core-hours worst case.
k = 3 fits at every primary rung, so no primary rung is dropped and the kill
rule is evaluated as written. The easy arm at k = 3 (worst case 5.1
core-hours) does not fit the 10 core-hour budget next to the binding grid:
**the easy arm runs k = 0, 1, 2 only**; k = 3 on `8-c10` is not run. Baselines
about 1 core-hour. Total worst case about 8 core-hours on 4 cores (`JOBS=4`,
one thread per job, `nice`).

## Analysis

`analysis/analyze.py` (stdlib), on `results/raw/*.jsonl`. Outputs per rung and
arm: solved runs and instances out of 30 with Wilson 95% intervals, KM median
and quartiles, KM median wall and total wall, us per iteration, censored-run
dynamics; slopes s_0..s_3 and s' with bootstrap intervals; s_3 - s_0 with its
interval; the kill rule fields above. Reported but not binding: the same
slopes and per-rung tables on `scb-gacolor_ac3_random`.

## Commands

```
./run.sh build
./run.sh test      # every test job via ../common/prereg_guard.py (job.sh)
./run.sh merge
./run.sh analyze
```
