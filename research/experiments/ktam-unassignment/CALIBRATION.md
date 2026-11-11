# A15 calibration (committed before any test-seed run)

Calibration instances only: the 20 R6 and R7 calibration boards of the primary
ladder (n = 6 c = 5 and n = 7 c = 6, seeds 1001..1010). Each config runs once
per instance at node cap (E + n^2) / 2, so every solve within E = 1e6 events
is observed. Selection rule (PREREG.md section 5): most solved within E, ties
by the smaller sum of ln(min(events, E)), then grid order.

Selected configs (also in `results/calibration/selection.json`, which the test
runs read):

| arm | selected | solved /20 (R6 /10, R7 /10) |
|---|---|---|
| B3 | `b3-rinf-k2` | 4 (4, 0) |
| V1 | `v1-beta2-rinf-k1` | 6 (6, 0) |
| V2 | `v2-t0-n0.1` | 8 (8, 0) |

For context only (not a selection input): the ladder's B1 calibration rows
solve all 10 R6 calibration boards within a few thousand nodes. The method
arms are far weaker than B1 already on calibration; no R7 calibration board
is solved within E by the selected configs. Wall clock for the whole grid
(54 configs x 20 runs, 8 threads): about 90 s.

Reproduce: `compute/target/release/ktam calibrate --arm B3|V1|V2` then
`python3 analysis/analyze.py select`.

## Full grids

### B3 (selected: `b3-rinf-k2`)

| config | solved /20 | R6 /10 | R7 /10 | sum ln(min(events, E)) |
|---|---|---|---|---|
| `b3-r1-k1` | 0 | 0 | 0 | 276.31 |
| `b3-r1-k2` | 0 | 0 | 0 | 276.31 |
| `b3-r1-k3` | 0 | 0 | 0 | 276.31 |
| `b3-r2-k1` | 0 | 0 | 0 | 276.31 |
| `b3-r2-k2` | 0 | 0 | 0 | 276.31 |
| `b3-r2-k3` | 0 | 0 | 0 | 276.31 |
| `b3-rinf-k1` | 3 | 3 | 0 | 272.87 |
| `b3-rinf-k2` **selected** | 4 | 4 | 0 | 271.16 |
| `b3-rinf-k3` | 4 | 4 | 0 | 272.17 |

### V1 (selected: `v1-beta2-rinf-k1`)

| config | solved /20 | R6 /10 | R7 /10 | sum ln(min(events, E)) |
|---|---|---|---|---|
| `v1-beta0.5-r1-k1` | 0 | 0 | 0 | 276.31 |
| `v1-beta0.5-r1-k2` | 0 | 0 | 0 | 276.31 |
| `v1-beta0.5-r1-k3` | 0 | 0 | 0 | 276.31 |
| `v1-beta0.5-r2-k1` | 0 | 0 | 0 | 276.31 |
| `v1-beta0.5-r2-k2` | 0 | 0 | 0 | 276.31 |
| `v1-beta0.5-r2-k3` | 0 | 0 | 0 | 276.31 |
| `v1-beta0.5-rinf-k1` | 6 | 6 | 0 | 266.45 |
| `v1-beta0.5-rinf-k2` | 5 | 5 | 0 | 265.76 |
| `v1-beta0.5-rinf-k3` | 4 | 4 | 0 | 263.29 |
| `v1-beta1-r1-k1` | 0 | 0 | 0 | 276.31 |
| `v1-beta1-r1-k2` | 0 | 0 | 0 | 276.31 |
| `v1-beta1-r1-k3` | 0 | 0 | 0 | 276.31 |
| `v1-beta1-r2-k1` | 0 | 0 | 0 | 276.31 |
| `v1-beta1-r2-k2` | 0 | 0 | 0 | 276.31 |
| `v1-beta1-r2-k3` | 0 | 0 | 0 | 276.31 |
| `v1-beta1-rinf-k1` | 6 | 6 | 0 | 265.25 |
| `v1-beta1-rinf-k2` | 4 | 4 | 0 | 265.62 |
| `v1-beta1-rinf-k3` | 5 | 5 | 0 | 266.89 |
| `v1-beta2-r1-k1` | 0 | 0 | 0 | 276.31 |
| `v1-beta2-r1-k2` | 0 | 0 | 0 | 276.31 |
| `v1-beta2-r1-k3` | 0 | 0 | 0 | 276.31 |
| `v1-beta2-r2-k1` | 0 | 0 | 0 | 276.31 |
| `v1-beta2-r2-k2` | 0 | 0 | 0 | 276.31 |
| `v1-beta2-r2-k3` | 0 | 0 | 0 | 276.31 |
| `v1-beta2-rinf-k1` **selected** | 6 | 6 | 0 | 262.02 |
| `v1-beta2-rinf-k2` | 6 | 6 | 0 | 264.78 |
| `v1-beta2-rinf-k3` | 4 | 4 | 0 | 268.87 |
| `v1-beta4-r1-k1` | 0 | 0 | 0 | 276.31 |
| `v1-beta4-r1-k2` | 0 | 0 | 0 | 276.31 |
| `v1-beta4-r1-k3` | 0 | 0 | 0 | 276.31 |
| `v1-beta4-r2-k1` | 0 | 0 | 0 | 276.31 |
| `v1-beta4-r2-k2` | 0 | 0 | 0 | 276.31 |
| `v1-beta4-r2-k3` | 0 | 0 | 0 | 276.31 |
| `v1-beta4-rinf-k1` | 2 | 2 | 0 | 274.83 |
| `v1-beta4-rinf-k2` | 2 | 1 | 1 | 275.12 |
| `v1-beta4-rinf-k3` | 2 | 2 | 0 | 276.15 |

### V2 (selected: `v2-t0-n0.1`)

| config | solved /20 | R6 /10 | R7 /10 | sum ln(min(events, E)) |
|---|---|---|---|---|
| `v2-t0-n0` | 0 | 0 | 0 | 276.31 |
| `v2-t0-n0.1` **selected** | 8 | 8 | 0 | 260.60 |
| `v2-t0-n0.3` | 7 | 7 | 0 | 254.33 |
| `v2-t16-n0` | 3 | 3 | 0 | 271.31 |
| `v2-t16-n0.1` | 7 | 7 | 0 | 263.45 |
| `v2-t16-n0.3` | 2 | 2 | 0 | 272.77 |
| `v2-t128-n0` | 5 | 5 | 0 | 273.44 |
| `v2-t128-n0.1` | 4 | 4 | 0 | 272.43 |
| `v2-t128-n0.3` | 6 | 6 | 0 | 269.29 |
