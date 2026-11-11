# Cut-multiset feasibility pruning (Hall matching) on the hard ladder

Round-2 test, preregistered in [PREREG.md](PREREG.md) (commit d54dfce, pushed
before any run). No amendments. A separate session judges the result; the
outcome below is this session's self-assessment.

## What was tested

The novelty check (PREREG.md section 1) found the count and parity half of
the idea already inside the baseline (`gacolor`: per colour, unused
half-edges minus open demand >= 0 and even) and the ring-cut half already
measured at zero on this ladder (NS-1, `scb-gacolor_ac3_ns1`). What remained,
and what was run, is the matching half: at a node, every empty cell in scope
must receive a pairwise distinct unused piece in a rotation still in its
AC-3-filtered domain (Kuhn augmenting paths over piece bitmasks). The
hand-built 5x5 example of PREREG.md (`compute/tests/example.rs`) is a dead
state that passes class balance, parity, gacolor, island, NS-1, the Eulerian
ring check and pairwise arc consistency, and is rejected by the check.

- `compute/engine-fork/`: verbatim copy of the single-core-benchmark
  `solver-engine` plus the optional hook (`HallMode`, `hall_feasible`,
  `hall_match`). Hook off reproduces the committed
  `scb-gacolor_ac3_random` rows exactly (60/60 instances of R6 and R7: same
  status, nodes and backtracks; `analysis/equivalence.py`).
- `compute/src/main.rs`: hard-ladder plug-in binary `cut_multiset` (entries
  `cmp-*`) and the R8 instance commands `gen-c10`, `check-c10`, `run-c10`.
- V1 `cmp-hall-frontier`: frontier cells (empty with an assigned neighbour),
  every node. V2, chosen on calibration seeds by the preregistered rule:
  `cmp-hall-all-k4` (every empty cell, every 4th node).

## Commands

```
just experiments cut-multiset-pruning          # everything, about 13 min of runs
just experiments cut-multiset-pruning check    # build, tests, instance checks, equivalence
# or: research/experiments/cut-multiset-pruning/run.sh [all|check|calibrate|test|analyze]
```

Hardware: Devin VM, 8 vCPU Intel Xeon, 31 GiB RAM, rustc 1.99.1, 8
instances in parallel (wall clock per run is measured under that load, as in
the ladder's committed baselines). Run wall clock: about 13 minutes in total
(equivalence 26 s, R8 baselines 77 s, calibration 441 s, test runs 221 s).

## Calibration (seeds 1001..10, cap 2e6; `results/calibration/selection.json`)

| solver | solved (R6+R7+R8) | KM median R6 / R7 / R8 |
|---|---|---|
| scb-gacolor_ac3_random | 27/30 | 1,133 / 613,022 / 460,275 |
| cmp-hall-frontier (V1) | 29/30 | 956 / 406,530 / 288,030 |
| cmp-hall-all | 28/30 | 1,025 / 410,709 / 262,766 |
| cmp-hall-frontier-k4 | 28/30 | 1,273 / 379,519 / 424,379 |
| cmp-hall-all-k4 (V2) | 28/30 | 962 / 335,031 / 307,895 |

## Test results (seeds 1..30, cap 1e6; `results/summary.json`)

Ratio = exp(upper median of paired log node ratio vs scb-gacolor_ac3_random),
CI = 95% percentile bootstrap of that log ratio (10,000 resamples, seed
20261111), p = one-sided bootstrap p, Holm over the two looks per rung. Wall
ratio = median paired wall-clock ratio vs the baseline.

| rung | solver | solved | KM median nodes [q1, q3] | median wall ms | total wall s | ratio | CI (log) | p | W/L/T | wall ratio | label |
|---|---|---|---|---|---|---|---|---|---|---|---|
| R6 6x6 c=5 | scb-gacolor_ac3_random | 30/30 | 951 [620, 1,940] | 6.2 | 0.25 | | | | | | |
| | hp-dfs | 30/30 | 41,823 [27,904, 62,556] | 23.7 | 0.83 | | | | | | |
| | V1 cmp-hall-frontier | 30/30 | 919 [595, 1,728] | 5.9 | 0.26 | 0.900 | [-0.200, -0.061] | 0.0002 | 24/6/0 | 1.00 | **GAIN** |
| | V2 cmp-hall-all-k4 | 30/30 | 837 [557, 1,508] | 5.5 | 0.22 | 0.876 | [-0.252, -0.028] | 0.0179 | 21/9/0 | 0.95 | **GAIN** |
| R7 7x7 c=6 | scb-gacolor_ac3_random | 16/30 | 921,773 [272,281, cens.] | 7,347 | 173.1 | | | | | | |
| | hp-dfs | 0/30 | cens. | 652 | 19.9 | | | | | | |
| | V1 cmp-hall-frontier | 18/30 | 654,651 [315,290, cens.] | 7,020 | 177.1 | 1.000 | [-0.079, 0.000] | 0.826 | 13/5/12 | 1.02 | NO-GAIN |
| | V2 cmp-hall-all-k4 | 17/30 | 725,243 [292,063, cens.] | 6,756 | 180.9 | 1.000 | [-0.140, 0.000] | 0.958 | 11/6/13 | 1.09 | NO-GAIN |
| R8 8x8 c=10 | scb-gacolor_ac3_random | 19/30 | 505,364 [363,902, cens.] | 15,367 | 552.1 | | | | | | |
| | hp-dfs | 19/30 | 509,862 [360,947, cens.] | 593 | 18.1 | | | | | | |
| | V1 cmp-hall-frontier | 22/30 | 449,692 [291,861, cens.] | 15,293 | 562.6 | 0.932 | [-0.168, 0.000] | 0.172 | 18/4/8 | 1.02 | NO-GAIN |
| | V2 cmp-hall-all-k4 | 22/30 | 445,508 [282,544, cens.] | 15,831 | 581.0 | 0.952 | [-0.245, 0.000] | 0.417 | 16/6/8 | 1.06 | NO-GAIN |

No error rows, no inconsistent `try_assign` calls, every solved board
verified by the harness. Both R6 labels survive Holm (p 0.0002 <= 0.0125 and
0.0179 <= 0.025).

## Self-assessment against the kill criterion

- Kill (NO-GAIN on all three rungs): **does not fire**; R6 is GAIN for both
  variants.
- PASS (G1 at R8): **not met**; R8 median ratio 0.93 / 0.95 with a CI that
  reaches 0 (the median pair is a both-censored tie or near it).
- Self-assessed outcome: **inconclusive, "promising, needs a variant"** in
  the round's wording, but the gain is small: about 10% fewer nodes at 6x6
  (where the baseline needs about 1,000 nodes), no significant median change
  at 7x7 and 8x8, and 8 to 17% lower node throughput at R7 and R8 that leaves wall clock
  roughly unchanged. Solve counts moved in the check's favour at R7 (16 to
  18 and 17 of 30) and R8 (19 to 22 of 30), which the paired median test
  does not register because about a third of pairs are censored on both
  sides; those counts are within the Wilson intervals and are not claimed.
- G2 (growth) not evaluated, as preregistered.

## Side observation (not part of the test)

At R8 c=10 the ladder's best baseline and plain row-major DFS solve the same
19/30 with the same KM median (about 5.1e5 nodes), and the baseline costs
about 26 times more wall clock per run. Above the critical colour count the
baseline's propagation buys no node reduction on this rung.

R8 is also off-protocol: `hard-ladder/README.md` requires an amendment and a
baseline calibration before candidates use 8x8 at c >= 10, and neither exists
for this rung. The R8 rows are therefore an uncalibrated rung against a
baseline that plain DFS matches; the verdict rests on R6 and R7 only.

## Files

- `PREREG.md`: preregistration and novelty check.
- `instances/8-c10-{test,calibration}.jsonl`: R8 instances (checked by `check-c10`).
- `results/test/primary/*.jsonl`, `results/test/8-c10/*.jsonl`: raw per-instance rows (test).
- `results/calibration/**`: calibration rows and `selection.json`.
- `results/summary.json`: all statistics and labels.
- `analysis/analyze.py`, `analysis/equivalence.py`: stdlib analysis.
