# A21: value order by propagation impact inside bb-mrv

Preregistration: [`PREREG.md`](PREREG.md) (commit `a4507f5`, made before any
test seed ran). No amendments. Track label: method. Generated boards only;
nothing here is official Eternity II progress, and nothing is extrapolated
beyond 8x8.

Every number below is copied from [`results/tables.md`](results/tables.md),
which `analysis/analyze.py` generates from the raw rows in `results/test/*.jsonl`
and `results/check/*.jsonl` (full statistics in `results/summary.json`).

## Self-assessed outcome (a separate judge decides)

**KILL** by the preregistered A21 rule: G1 against `bb-mrv` fails at every rung,
so no rung has a node GAIN to gate on wall clock. The PROTOCOL.md dropping rule
drops the method at R6, the first rung evaluated.

| rung | arm vs bb-mrv | node ratio (G1) | 95% CI | G1 | wall ratio | wall 95% CI |
|---|---|---|---|---|---|---|
| R6 6x6 c=5 | vi-impact | 1.075 | [0.779, 1.931] | fail | 1.143 | [0.811, 2.041] |
| R7 7x7 c=6 | vi-impact | 0.829 | [0.544, 1.316] | fail | 0.814 | [0.495, 1.294] |
| R8 8x8 c=10 | vi-impact | 1.096 | [0.669, 1.871] | fail | 1.141 | [0.801, 1.904] |

Mechanism attribution (against the probe-then-random twin `vi-probe-rand`,
per-instance median of run seeds 1..3): node ratio 1.297 [1.058, 1.621] at R6
(the impact order is significantly worse than random among the same probed
options), 1.000 [0.817, 1.618] at R7, 1.128 [0.770, 2.036] at R8. Against the
`mrv-rand` composite: 1.325, 1.000, 1.127, none significant.

## What was run

- `vi-impact`: bb-mrv's cell rule and propagation; at each node every option of
  the chosen cell is probed with `Model::propagate`, then tried in ascending
  number of option bits removed (failed probes last, ties by index). The
  search reuses the probe's domains, so probes cost about one propagate call
  per node (probes/node 1.00 to 1.01 at every rung).
- Controls: `vi-probe-rand-s1..s3` (same probes, failed last, the rest in random
  order) and `mrv-rand-s1..s3` (A22's arm, unchanged). Composite = per-instance
  median of the three seeds.
- `scb-gacolor_ac3_random` (run seed 1) at R6 and R7.
- `vi-index` (probing code, index order): equal to `bb-mrv` on nodes,
  backtracks, status and `equals_planted` on 90 of 90 test instances.
- Instances: `primary-test` n = 6 and 7 seeds 1..30; `8-c10-test` 21..30 plus
  `8-c10-fresh-test` 31..50. Cap 1e6 nodes. No guard-stopped run, no error row.
- Wall clock: Apple M4, 2 worker threads, no pinning; every timed arm run back
  to back on the same instance in a rotated order; 5 repetitions at R6 (median
  per arm and instance), 1 at R7 and R8. Node counts are identical across the
  five R6 repetitions.
- Reproduction cross-check: `bb-mrv`, `mrv-rand-s1` and scb rows equal A22's
  committed rows (`../mrv-baseline/results/raw/`) on every overlapping instance
  (50, 50 and 20 instances).

## Per rung and arm (solved/30, KM median nodes)

| arm | R6 | R7 | R8 |
|---|---|---|---|
| bb-mrv | 30, 460 | 23, 588,305 | 30, 69,036 |
| vi-impact | 30, 817 | 23, 441,033 | 30, 130,339 |
| vi-probe-rand (composite) | 30, 521 | 26, 351,612 | 30, 93,781 |
| mrv-rand (composite) | 30, 554 | 27, 341,283 | 30, 80,270 |
| scb-gacolor_ac3_random | 30, 951 | 16, 921,773 | not run |

## Unplanned observations

- Random value order is as good as or better than every fixed order here, and
  run-seed spread is large: at R8 `mrv-rand-s3` passes G1 against `bb-mrv`
  (0.495 [0.409, 0.768], wall 0.486) while `mrv-rand-s1` is at 1.623. bb-mrv's
  option-index order is one draw from that spread, not a tuned order.
- At R6 on all 30 seeds, `bb-mrv` passes G1 against scb (0.468 [0.318, 0.895])
  but is slower in wall clock (1.373 [0.840, 2.478]); A22 saw 0.80 on seeds
  21..30 only. At R7, 0.976 (fail), wall 2.807.

## Reproduce

```
just experiments value-impact-order          # build, calibrate, test, check, analyze (about 45 min on 2 cores)
just experiments value-impact-order analyze  # tables from the committed rows
```

Wall clock (this run): test stage 33 min 44 s, check stage 5 min 36 s, on 2
threads (`results/stage-*.txt`).
