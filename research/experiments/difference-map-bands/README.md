# Difference map with exact band projections (track 11, night 2026-11-12)

Preregistration: [`PREREG.md`](PREREG.md) (commit `9a0c4a3`, SHA-256
`8f784d3f...cdbc3`, committed alone before any test seed; every one of the 52
test jobs in `results/run-log.jsonl` carries that hash, first start
2026-11-12T22:59:06Z, after the commit at 22:58:52Z). No amendments.
Track label: method. Generated boards only; nothing here is official
Eternity II progress, and nothing extends past 8x8.

Source of the hypothesis: `research/program/night/2026-11-12/memos/solve-path-6-free.md`
section 6 (unrefereed) and its referee `memos/literature-and-path-6.REVIEW.md`.

## Self-assessed outcome (a separate judge decides)

**Dead by censoring within the preregistered cap**, the fallback reading
written in PREREG.md before the run. At the primary rungs n = 7 (c = 6) and
n = 8 (c = 8) the map solved 0 of 30 instances at every k (one run of 90 at
n = 7, k = 3), so every rung median there is censored at 20,000 iterations and
s_0 = s_1 = s_2 = s_3 = +infinity. Kill (a) fires (s_3 = +inf > 0.5 x s' =
0.058) and kill (b) fires (no finite fall; s_3 - s_0 scored 0, CI [0, 0]).
This is a statement about this cap and budget, not a measured exponent.

## Rows (`results/summary.json`, from `results/raw/*.jsonl`)

Per-instance value: median over run seeds 1..3 (censored = +inf). KM median:
15th order statistic of 30. Wall: single thread, this Mac, 4 jobs in parallel.
DM cap 20,000 iterations, beta 0.25. Baselines cap 1,000,000 nodes.

| ladder | n | arm | instances solved (median of 3 runs) | Wilson 95% | runs solved /90 | KM median | Q1 | Q3 | KM median wall ms | total wall s | us per iteration |
|---|---|---|---|---|---|---|---|---|---|---|---|
| primary | 6 | dm-k0 | 21/30 | 0.52-0.83 | 57 | 12,156 it | 6,532 | > cap | 376 | 35.0 | 31 |
| primary | 6 | dm-k1 | 23/30 | 0.59-0.88 | 69 | 9,077 it | 4,912 | 19,221 | 811 | 83.4 | 90 |
| primary | 6 | dm-k2 | 22/30 | 0.56-0.86 | 64 | 7,936 it | 5,908 | > cap | 1,385 | 162.3 | 173 |
| primary | 6 | dm-k3 | 15/30 | 0.33-0.67 | 50 | 14,049 it | 5,672 | > cap | 14,678 | 1,198.4 | 1,040 |
| primary | 6 | bb-mrv | 30/30 | 0.89-1.00 | 30/30 | 460 nodes | 317 | 1,092 | 3.3 | 0.2 | |
| primary | 6 | scb-gacolor_ac3_random | 30/30 | 0.89-1.00 | 30/30 | 951 nodes | 620 | 1,940 | 4.5 | 0.2 | |
| primary | 7 | dm-k0 | 0/30 | 0.00-0.11 | 0 | > cap | | | > cap | 110.9 | 62 |
| primary | 7 | dm-k1 | 0/30 | 0.00-0.11 | 0 | > cap | | | > cap | 327.4 | 186 |
| primary | 7 | dm-k2 | 0/30 | 0.00-0.11 | 0 | > cap | | | > cap | 530.9 | 283 |
| primary | 7 | dm-k3 | 0/30 | 0.00-0.11 | 1 | > cap | | | > cap | 3,939.8 | 2,285 |
| primary | 7 | bb-mrv | 23/30 | 0.59-0.88 | 23/30 | 588,305 nodes | 253,491 | 976,206 | 11,155 | 326.8 | |
| primary | 7 | scb-gacolor_ac3_random | 16/30 | 0.36-0.70 | 16/30 | 921,773 nodes | 272,281 | > cap | 6,372 | 149.4 | |
| primary | 8 | dm-k0 | 0/30 | 0.00-0.11 | 0 | > cap | | | > cap | 228.1 | 125 |
| primary | 8 | dm-k1 | 0/30 | 0.00-0.11 | 0 | > cap | | | > cap | 657.0 | 373 |
| primary | 8 | dm-k2 | 0/30 | 0.00-0.11 | 0 | > cap | | | > cap | 1,412.6 | 769 |
| primary | 8 | dm-k3 | 0/30 | 0.00-0.11 | 0 | > cap | | | > cap | 10,864.9 | 6,133 |
| primary | 8 | bb-mrv | 0/30 | 0.00-0.11 | 0/30 | > cap | | | > cap | 1,066.4 | |
| primary | 8 | scb-gacolor_ac3_random | 0/30 | 0.00-0.11 | 0/30 | > cap | | | > cap | 438.4 | |
| 8-c10 (easy, not binding) | 8 | dm-k0 | 0/30 | 0.00-0.11 | 0 | > cap | | | > cap | 238.9 | 131 |
| 8-c10 | 8 | dm-k1 | 0/30 | 0.00-0.11 | 0 | > cap | | | > cap | 677.7 | 386 |
| 8-c10 | 8 | dm-k2 | 0/30 | 0.00-0.11 | 1 | > cap | | | > cap | 1,532.9 | 840 |
| 8-c10 | 8 | bb-mrv | 30/30 | 0.89-1.00 | 30/30 | 95,128 nodes | 47,320 | 174,107 | 3,715 | 146.6 | |
| 8-c10 | 8 | scb-gacolor_ac3_random | 19/30 | 0.46-0.78 | 19/30 | 505,364 nodes | 363,902 | > cap | 12,338 | 397.0 | |

k = 3 on `8-c10` was not run (preregistered: worst case 5.1 core-hours).

## Slopes and kill fields (primary, n = 6, 7, 8)

| quantity | point | bootstrap 95% (10,000, seed 20261111) |
|---|---|---|
| s_0, s_1, s_2, s_3 (log10 iterations per cell) | +inf (censored rung medians at n = 7, 8) | [+inf, +inf] |
| s' (`bb-mrv`, censored n = 8 median set to the cap 1e6, a lower bound) | 0.1165 | [0.1080, 0.1195] |
| s_3 - s_0 | 0 (both +inf) | [0, 0] |
| relative fall s_0 to s_3 | undefined | |
| kill (a) s_3 > 0.5 s' | fires | |
| kill (b) | fires | |

## Censored runs (dynamics)

No censored run ended in a cycle (smallest period <= 50 over the last 200
iterations): 0 of 33, 21, 26, 40 (n = 6, k = 0..3), 0 of 359 (n = 7), 0 of 360
(n = 8), 0 of 269 (`8-c10`). The map wanders: median 1,775 to 2,000 distinct
layouts in the last 2,000 iterations. Median best / terminal mismatched seams
of censored runs (true medians; half-integers are kept), k = 0, 1, 2, 3:

| rung | best | terminal |
|---|---|---|
| primary n = 6 | 2, 2, 2, 3 | 11, 12, 15.5, 19 |
| primary n = 7 | 4, 4, 4, 5 | 16.5, 18, 19.5, 22 |
| primary n = 8 | 9, 10, 11, 13 | 27, 29, 31, 34 |
| 8-c10 n = 8 | 15, 15, 17 (k = 0..2) | 36, 34.5, 39 |

`results/summary.json` stores the upper median (`sorted(...)[len // 2]`), so it
shows 17, 16, 20 and 35 where the true medians are 16.5, 15.5, 19.5 and 34.5; it
is a committed result and is left as it is. The values above were recomputed
from `results/raw/*.jsonl`.

Wider bands leave censored runs farther from a solution, not closer.

## Cost axis

One `bb-mrv` node costs 7.5, 18.6 and 35.5 us at primary n = 6, 7, 8 and 39.1
us on `8-c10` (total wall over total counted nodes in the raw rows; 8-c10:
3,748,547 nodes in 146.6 s). One iteration at 8x8 costs 125
us (k = 0) to 6,133 us (k = 3), about 3 to 160 `bb-mrv` nodes of wall time. At
n = 6, the only rung where both arms have finite medians, the KM median wall
time to a solution is 3.3 ms for `bb-mrv` against 376 ms (k = 0) to 14,678 ms
(k = 3); iterations over `bb-mrv` nodes there are 26 (k = 0), 20, 17 and 31
(k = 3), so the map's count is 17 to 31 times the baseline's node count
before any per-iteration cost.

## Port validation (calibration only)

Every solved board (242 test rows, plus all calibration rows) passed both the
Rust checker and the independent Python checker in `analysis/analyze.py`
(0 failures). On the memo's own nine instances (`calib/memo-instances.jsonl`,
regenerated with the memo's `gen`), medians of 5 starts at cap 30,000
(`calib/memo-compare.txt`) against the memo's dm.py medians (`results.txt`,
3 to 5 starts): k = 0 vs "A only": 181 vs 195, 10,441 vs 5,657, 789 vs 937,
6,034 vs 1,246, 492 vs 322, 3,573 vs 5,506; k = 1 vs "A + rows + columns":
338 vs 351, 1,437 vs 2,287, 943 vs 465, 3,221 vs 970, 398 vs 299, 1,945 vs
2,613, 4,238 vs 2,515 (7x7), and the 8x8 c = 12 instance solved 4 of 5 at
k = 1 (median 5,490; memo 14,948 and 20,736) but 1 of 5 at k = 0. Same order
of magnitude everywhere, within the start-to-start scatter (often 10x).

## Exact commands

```
just experiments difference-map-bands          # = difference-map-bands/run.sh all
# stages: build | test | merge | analyze
python3 -I calib/memo_instances.py > calib/memo-instances.jsonl   # memo comparison inputs
bash calib/cal1.sh; bash calib/cal2.sh; python3 -I calib/peek.py calib/cal1
```

Compute: test grid 6.81 core-hours (sum of `seconds` in the run log, equal to
the sum of row wall times), calibration about 0.5 core-hours, 4 cores,
1 h 45 min wall (22:59 to 00:44 UTC).

## After the judgment

JUDGE.md (commit `279118a`) rules **kill**: both parts of the preregistered
kill rule fire through the censoring fallback, at this cap (beta 0.25, 20,000
iterations). It is a kill of this method at this cap on the primary ladder, not
a measured exponent.

- No band effect where it is measurable. At n = 6, the paired bootstrap of the
  KM median ratio k / k0 (5,000 resamples) is [0.41, 1.36] for k = 1,
  [0.43, 1.30] for k = 2 and [0.77, +inf] for k = 3; all contain 1.
- Wider bands cost 3x to 33x more per iteration and end further from a
  solution: median best mismatches of censored runs at n = 8 are 9, 10, 11, 13
  for k = 0..3, and none of the 1,108 censored runs cycles.
- Caveat on the cap: for k of 2 or less the cap gives the map less wall time
  than `bb-mrv` gets at n = 7 (20,000 iterations is 1.2 s at k = 0 to 5.7 s at
  k = 2 per run, against a `bb-mrv` median of 11.2 s and a cap of about
  18.6 s). A larger cap could give finite n = 7 medians at low k. The judge
  reads the n = 6 trend and the censored dynamics as pointing the wrong way,
  so this does not change the verdict.
- `bb-mrv` solves 0 of 30 at the primary n = 8 rung, so that rung separates
  nothing between arms.
- Reopening would need a different mechanism, not a larger cap: a band
  projection whose n = 6 iteration count has a bootstrap interval below 1
  against k = 0, before any scaling claim.
