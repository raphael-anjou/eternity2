# A20: blocker eviction as the backjump rule inside bb-mrv

Track label: method. Preregistration: `PREREG.md` (committed before any test
seed ran). Generated boards only; nothing here is official Eternity II progress.

## Verdict

- **Method: KILLED.** Dropped at R6 by the PROTOCOL rung rule (G1 against scb
  fails: node ratio 4.68, CI [2.40, 6.55]) and K1 fires (G1 fails at R6 and
  R7; R7 solved, run-seed median, 4/30 against scb 16/30).
- **R7 gate: not passed**, so 8x8 c=10 was not run.
- **Mechanism (K2): KILLED at R7.** The random twin solves 3/30 (run-seed
  median) against the blocker's 4/30, inside the 2-solve margin, and the
  blocker is not significantly better than the twin (paired median ln ratio
  0, CI upper end 0.0, not below 0).

Every number below comes from `results/tables.md` and `results/summary.json`,
both written by `analysis/analyze.py test` from the raw rows in
`results/test/*/*.jsonl` (hard-ladder `RunRecord`) and `*.side.jsonl` (bj
per-run dead-end, eviction and revisit counters).

## Per rung and arm (run seed 1, cap 1e6 nodes, 2 threads)

| rung | arm | solved/30 | KM median nodes | total wall s |
|---|---|---|---|---|
| R6 6x6 c=5 | scb-gacolor_ac3_random | 30 | 951 | 0.3 |
| R6 6x6 c=5 | bb-mrv | 30 | 460 | 0.3 |
| R6 6x6 c=5 | bj-blocker | 30 | 4,334 | 11.5 |
| R6 6x6 c=5 | bj-random | 30 | 3,868 | 63.3 |
| R7 7x7 c=6 | scb-gacolor_ac3_random | 16 | 921,773 | 181.0 |
| R7 7x7 c=6 | bb-mrv | 23 | 588,305 | 471.0 |
| R7 7x7 c=6 | bj-blocker | 5 | > cap | 1,893.4 |
| R7 7x7 c=6 | bj-random | 2 | > cap | 2,149.6 |

Run seeds 2 and 3 (bj arms only): R6 blocker 30, 29; twin 30, 29. R7 blocker
2, 4; twin 4, 3. Medians over seeds 1..3: R6 30 and 30, R7 4 and 3.

## G1 (PROTOCOL form, run seed 1)

| rung | candidate | baseline | node ratio | 95% CI | G1 | wall ratio |
|---|---|---|---|---|---|---|
| R6 | bj-blocker | scb | 4.683 | [2.403, 6.547] | fail | 32.3 |
| R6 | bj-blocker | bb-mrv | 9.177 | [5.394, 15.204] | fail | 24.5 |
| R6 | bj-random | scb | 4.792 | [1.982, 6.239] | fail | 32.8 |
| R6 | bj-random | bb-mrv | 6.953 | [2.718, 15.771] | fail | 19.2 |
| R6 | bj-blocker | bj-random | 0.947 | [0.574, 3.670] | fail | 0.98 |
| R7 | bj-blocker | scb | 1.000 | [1.000, inf] | fail | 9.0 |
| R7 | bj-blocker | bb-mrv | inf | [3.813, inf] | fail | 3.96 |
| R7 | bj-random | scb | 3.173 | [1.000, inf] | fail | 9.6 |
| R7 | bj-random | bb-mrv | inf | [inf, inf] | fail | 4.22 |
| R7 | bj-blocker | bj-random | 1.000 | [1.000, 1.000] | fail | 0.97 |

The R7 blocker-vs-scb median of 1.000 is a tie produced by 13 both-censored
pairs; the blocker solves 1 board scb does not and loses 12.

## Mechanism diagnostics (sidecar, run seed 1)

| rung | arm | dead ends | fallback fraction | evictions per dead end | revisit fraction | cycled runs |
|---|---|---|---|---|---|---|
| R6 | bj-blocker | 203,240 | 0.244 | 1.08 | 0.365 | 0 |
| R6 | bj-random | 1,145,970 | 0.039 | 1.11 | 0.396 | 0 |
| R7 | bj-blocker | 19,195,508 | 0.285 | 1.06 | 0.257 | 5 |
| R7 | bj-random | 22,298,791 | 0.034 | 1.11 | 0.130 | 0 |

Cycling (preregistered definition: censored, more than half of repairs return
to an already-visited decision set): bj-blocker cycled on 5 (seed 1), 9
(seed 2) and 6 (seed 3) of 30 R7 runs and on 1 R6 run (seed 3, s18, 99.9%
revisits); bj-random never cycled. No run returned `Exhausted`, no `error`
row, and the 900 s guard never fired. The blocker set has one element in
almost every repair (1.06 evictions per dead end), and in about a quarter of
dead ends the failed cell has no direct blocker at all (the fallback evicts
the latest decision instead).

The tabu list is also cleared whenever a dead end leaves no decision standing: 4,169 times for the blocker arm and 10,922 times for the random twin over the 30 R7 runs (run seed 1), summed from the `tabu_clears` counter of the sidecar rows. Both arms are therefore partly a restart method, not a pure repair search.

## Checks

- Baselines rerun here reproduce committed rows node for node: scb at R6/R7
  equals `hard-ladder/results/test/primary/scb-gacolor_ac3_random.jsonl` on
  all 60 instances; bb-mrv equals A22's `q1-r6r7` rows on its 20 overlapping
  instances.
- bj determinism: calibration `t16` and `base` runs of bj-blocker give the same
  node counts and node-sequence hashes on all 20 instances.
- Calibration (tabu tenure, calibration seeds only): `results/calibration/calibration.md`.

## Reproduce

```
just experiments blocker-backjump build
just experiments blocker-backjump calibrate   # about 40 min on 2 threads
just experiments blocker-backjump r6
just experiments blocker-backjump r7          # about 40 min on 2 threads
just experiments blocker-backjump seeds23     # about 70 min on 2 threads
just experiments blocker-backjump analyze
```

Node counts are deterministic (fixed seeds); wall clock is not (Apple M4
laptop, 2 threads, `nice`, no pinning, arms interleaved per instance).
