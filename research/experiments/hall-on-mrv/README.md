# Hall matching on top of bb-mrv (A17 rebaselined)

Night shift 2026-11-12, track 4. Preregistration: [`PREREG.md`](PREREG.md)
(commit 860e368, committed before any test seed ran). One amendment: [`PREREG_AMENDMENTS.md`](PREREG_AMENDMENTS.md).
Track label: method. A separate judge decides; the outcome below is the
running session's self-assessment. Generated boards only; nothing here is
official Eternity II progress, nothing is extrapolated to 16x16.

## Self-assessed outcome (a separate judge decides)

**Killed by the preregistered rule (K2), and no material effect.** Hall
matching on top of `bb-mrv` removes 0 to 0.8% of the nodes at the median
pair, while computing the check costs up to about 5% wall clock (cost
control); the Hall arms' median wall ratios against `bb-mrv` are 0.989 to
1.036, above 1 at every rung where the nodes moved.

| criterion (PREREG.md section 6) | V1 `hall-v1` | V2 `hall-v2` |
|---|---|---|
| R6 label / wall ratio | NO-GAIN (1.000) / 1.024 | NO-GAIN (1.000) / 0.989 |
| R7 label / wall ratio | GAIN (0.993) / 1.028 | GAIN (0.992) / 1.025 |
| R8 label / wall ratio | GAIN (0.999) / 1.017 | GAIN (0.997) / 1.036 |
| K1 (NO-GAIN at every rung) | does not fire | does not fire |
| K2 (wall ratio > 1 at every GAIN rung) | **fires** | **fires** |
| variant | killed | killed |

Method: killed (both variants killed). The "no material effect" reading also
applies: every GAIN has a G1 ratio above 0.9 (0.992 to 0.999). No G1 PASS
anywhere. Source: `results/summary.json` (`kill`, `rungs.*.labels`,
`rungs.*.pairs`).

Why it is small: A17's 10% at R6 was measured against `scb`, which has no
piece all-different. `bb-mrv` already applies A3's naked piece singles,
hidden singles and no-cell wipeout at every node; those are Hall's condition
for sets of size 1, and almost every Hall failure on this ladder is already
caught by them. The larger Hall violations do occur (R7: 94,720 nodes where
`cost-v1` saw a failure that A3 missed, out of 11.2 million checks) but they
sit near the leaves and cut small subtrees.

## Order effects

The check never changes a domain and fails a node only when no completion
exists, so a Hall arm visits `bb-mrv`'s node sequence with failed subtrees
deleted and nothing reordered. Verified:

- `trace-check` (full traces, `results/trace/*.jsonl`): 120/120 instance-variant pairs pass (R6, R7, R8 calibration seeds 1001..1010 and R6 test seeds 1..30, both variants), with Hall nodes <= base nodes on all 120 and the same board on all 107 pairs where both solved; the deleted subtrees total 79 (V1) and 312 (V2) nodes on the R6 test seeds and 66,999 / 81,230 on the R7 calibration seeds.
- On every test instance at every rung: Hall nodes <= `bb-mrv` nodes (30/30
  per rung and variant), same returned board whenever both solve (30/30,
  23/23, 30/30), and the controls `hall-off`, `cost-v1`, `cost-v2` equal
  `bb-mrv` in status, nodes, backtracks, propagation count and board on
  90/90 instances, with `cost-vk` sequence hashes equal to `hall-off`'s
  (`results/summary.json`, `rungs.*.checks`).

So the node changes are pure pruning, not an order effect. In size they are
far inside the run-seed spread of `mrv-rand` (value order randomised, run
seeds 1 to 3): G1 against `bb-mrv` spans 0.891 to 1.281 at R6, 0.881 to 0.906
at R7 and 0.495 to 1.623 at R8, against Hall ratios of 0.992 to 1.000. By the
preregistered wording the Hall gains are "not distinguishable from an order
effect" in size.

## Per rung and arm

Cap 1e6 nodes for every arm, guard 600 s (never fired: 0 guard stops in all
2,070 test rows). Wall clock: Apple M4, 2 worker threads under `nice`,
machine shared with other tracks (load average 12 to 18 on 10 cores), arms
interleaved per instance; R6 is the median of 5 repetitions per pair, R7 and
R8 one run. KM quartiles are the 8th, 15th and 23rd order statistics
(censored = infinity).

| rung | arm | solved/30 [Wilson 95%] | KM q25 | KM median | KM q75 | median wall ms | total wall s | guard stops | total nodes |
|---|---|---|---|---|---|---|---|---|---|
| R6 6x6 c=5 | bb-mrv | 30/30 [0.89, 1.00] | 317 | 460 | 1,092 | 7.5 | 0.4 | 0 | 22,509 |
| R6 6x6 c=5 | hall-off | 30/30 [0.89, 1.00] | 317 | 460 | 1,092 | 7.5 | 0.4 | 0 | 22,509 |
| R6 6x6 c=5 | hall-v1 | 30/30 [0.89, 1.00] | 317 | 458 | 1,090 | 7.7 | 0.4 | 0 | 22,430 |
| R6 6x6 c=5 | hall-v2 | 30/30 [0.89, 1.00] | 311 | 460 | 1,076 | 6.8 | 0.4 | 0 | 22,197 |
| R6 6x6 c=5 | cost-v1 | 30/30 [0.89, 1.00] | 317 | 460 | 1,092 | 7.6 | 0.4 | 0 | 22,509 |
| R6 6x6 c=5 | cost-v2 | 30/30 [0.89, 1.00] | 317 | 460 | 1,092 | 7.7 | 0.4 | 0 | 22,509 |
| R6 6x6 c=5 | mrv-rand-1 | 30/30 [0.89, 1.00] | 304 | 548 | 1,121 | 9.1 | 0.4 | 0 | 26,603 |
| R6 6x6 c=5 | mrv-rand-2 | 30/30 [0.89, 1.00] | 242 | 605 | 840 | 10.0 | 0.3 | 0 | 20,519 |
| R6 6x6 c=5 | mrv-rand-3 | 30/30 [0.89, 1.00] | 227 | 450 | 898 | 7.9 | 0.3 | 0 | 22,115 |
| R6 6x6 c=5 | scb | 30/30 [0.89, 1.00] | 620 | 951 | 1,940 | 6.9 | 0.3 | 0 | 50,200 |
| R7 7x7 c=6 | bb-mrv | 23/30 [0.59, 0.88] | 253,491 | 588,305 | 976,206 | 17401.4 | 486.6 | 0 | 17,601,059 |
| R7 7x7 c=6 | hall-off | 23/30 [0.59, 0.88] | 253,491 | 588,305 | 976,206 | 17416.6 | 487.3 | 0 | 17,601,059 |
| R7 7x7 c=6 | hall-v1 | 23/30 [0.59, 0.88] | 250,683 | 584,189 | 970,255 | 17612.0 | 503.7 | 0 | 17,505,488 |
| R7 7x7 c=6 | hall-v2 | 23/30 [0.59, 0.88] | 239,367 | 578,970 | 969,490 | 17637.8 | 503.6 | 0 | 17,474,005 |
| R7 7x7 c=6 | cost-v1 | 23/30 [0.59, 0.88] | 253,491 | 588,305 | 976,206 | 17797.1 | 500.2 | 0 | 17,601,059 |
| R7 7x7 c=6 | cost-v2 | 23/30 [0.59, 0.88] | 253,491 | 588,305 | 976,206 | 17879.2 | 499.5 | 0 | 17,601,059 |
| R7 7x7 c=6 | mrv-rand-1 | 27/30 [0.74, 0.97] | 204,974 | 396,842 | 870,772 | 12519.7 | 412.1 | 0 | 14,869,806 |
| R7 7x7 c=6 | mrv-rand-2 | 25/30 [0.66, 0.93] | 240,953 | 323,533 | 628,797 | 9904.2 | 383.8 | 0 | 13,692,175 |
| R7 7x7 c=6 | mrv-rand-3 | 26/30 [0.70, 0.95] | 171,005 | 378,131 | 770,480 | 13200.2 | 405.7 | 0 | 14,664,274 |
| R7 7x7 c=6 | scb | 16/30 [0.36, 0.70] | 272,281 | 921,773 | cens. | 8202.1 | 187.5 | 0 | 20,895,091 |
| R8 8x8 c=10 | bb-mrv | 30/30 [0.89, 1.00] | 37,222 | 69,036 | 162,813 | 3885.5 | 165.7 | 0 | 3,359,112 |
| R8 8x8 c=10 | hall-off | 30/30 [0.89, 1.00] | 37,222 | 69,036 | 162,813 | 3886.1 | 165.9 | 0 | 3,359,112 |
| R8 8x8 c=10 | hall-v1 | 30/30 [0.89, 1.00] | 37,218 | 68,881 | 162,747 | 3953.0 | 169.3 | 0 | 3,355,670 |
| R8 8x8 c=10 | hall-v2 | 30/30 [0.89, 1.00] | 36,914 | 68,632 | 162,586 | 4023.3 | 171.0 | 0 | 3,343,983 |
| R8 8x8 c=10 | cost-v1 | 30/30 [0.89, 1.00] | 37,222 | 69,036 | 162,813 | 4659.4 | 171.7 | 0 | 3,359,112 |
| R8 8x8 c=10 | cost-v2 | 30/30 [0.89, 1.00] | 37,222 | 69,036 | 162,813 | 4743.7 | 174.5 | 0 | 3,359,112 |
| R8 8x8 c=10 | mrv-rand-1 | 30/30 [0.89, 1.00] | 46,981 | 107,496 | 170,024 | 5945.1 | 189.1 | 0 | 3,816,721 |
| R8 8x8 c=10 | mrv-rand-2 | 30/30 [0.89, 1.00] | 25,774 | 69,526 | 181,472 | 4932.2 | 190.6 | 0 | 3,859,115 |
| R8 8x8 c=10 | mrv-rand-3 | 30/30 [0.89, 1.00] | 17,827 | 46,568 | 105,503 | 2507.3 | 110.7 | 0 | 2,247,424 |


### Paired comparisons

G1 ratio = exp(upper median of paired log node ratios), PROTOCOL.md censoring rules; wall ratio = exp(upper median of paired log wall ratios). Bootstrap 10,000, seed 20261111.

| rung | candidate | baseline | G1 ratio | 95% CI (ratio) | p(med>=0) | W/L/T | both cens. | cand only | base only | wall ratio | wall 95% CI |
|---|---|---|---|---|---|---|---|---|---|---|---|
| R6 6x6 c=5 | hall-off | bb-mrv | 1.000 | [1.000, 1.000] | 1.0000 | 0/0/30 | 0 | 0 | 0 | 0.982 | [0.957, 1.013] |
| R6 6x6 c=5 | hall-v1 | bb-mrv | 1.000 | [0.998, 1.000] | 0.9030 | 12/0/18 | 0 | 0 | 0 | 1.024 | [0.990, 1.056] |
| R6 6x6 c=5 | hall-v2 | bb-mrv | 1.000 | [0.991, 1.000] | 0.7099 | 14/0/16 | 0 | 0 | 0 | 0.989 | [0.951, 1.053] |
| R6 6x6 c=5 | cost-v1 | bb-mrv | 1.000 | [1.000, 1.000] | 1.0000 | 0/0/30 | 0 | 0 | 0 | 0.988 | [0.972, 1.056] |
| R6 6x6 c=5 | cost-v2 | bb-mrv | 1.000 | [1.000, 1.000] | 1.0000 | 0/0/30 | 0 | 0 | 0 | 1.040 | [0.971, 1.071] |
| R6 6x6 c=5 | mrv-rand-1 | bb-mrv | 1.281 | [0.802, 1.729] | 0.9549 | 11/19/0 | 0 | 0 | 0 | 1.141 | [0.798, 1.653] |
| R6 6x6 c=5 | mrv-rand-2 | bb-mrv | 0.891 | [0.520, 1.677] | 0.2848 | 17/13/0 | 0 | 0 | 0 | 0.924 | [0.511, 1.619] |
| R6 6x6 c=5 | mrv-rand-3 | bb-mrv | 1.131 | [0.805, 1.458] | 0.5767 | 15/15/0 | 0 | 0 | 0 | 1.006 | [0.736, 1.557] |
| R6 6x6 c=5 | scb | bb-mrv | 2.215 | [1.230, 3.700] | 0.9926 | 9/21/0 | 0 | 0 | 0 | 0.829 | [0.373, 1.308] |
| R6 6x6 c=5 | hall-v1 | cost-v1 | 1.000 | [0.998, 1.000] | 0.9030 | 12/0/18 | 0 | 0 | 0 | 1.007 | [0.995, 1.019] |
| R6 6x6 c=5 | hall-v2 | cost-v2 | 1.000 | [0.991, 1.000] | 0.7099 | 14/0/16 | 0 | 0 | 0 | 0.996 | [0.952, 1.016] |
| R6 6x6 c=5 | bb-mrv | scb | 0.468 | [0.318, 0.895] | 0.0174 | 21/9/0 | 0 | 0 | 0 | 1.210 | [0.836, 2.849] |
| R6 6x6 c=5 | hall-v1 | scb | 0.468 | [0.318, 0.895] | 0.0174 | 21/9/0 | 0 | 0 | 0 | 1.322 | [0.818, 2.862] |
| R6 6x6 c=5 | hall-v2 | scb | 0.468 | [0.315, 0.895] | 0.0174 | 21/9/0 | 0 | 0 | 0 | 1.305 | [0.816, 2.712] |
| R7 7x7 c=6 | hall-off | bb-mrv | 1.000 | [1.000, 1.000] | 1.0000 | 0/0/30 | 7 | 0 | 0 | 1.001 | [0.995, 1.006] |
| R7 7x7 c=6 | hall-v1 | bb-mrv | 0.993 | [0.992, 0.996] | 0.0018 | 23/0/7 | 7 | 0 | 0 | 1.028 | [1.013, 1.055] |
| R7 7x7 c=6 | hall-v2 | bb-mrv | 0.992 | [0.989, 0.993] | 0.0018 | 23/0/7 | 7 | 0 | 0 | 1.025 | [1.009, 1.038] |
| R7 7x7 c=6 | cost-v1 | bb-mrv | 1.000 | [1.000, 1.000] | 1.0000 | 0/0/30 | 7 | 0 | 0 | 1.024 | [1.010, 1.035] |
| R7 7x7 c=6 | cost-v2 | bb-mrv | 1.000 | [1.000, 1.000] | 1.0000 | 0/0/30 | 7 | 0 | 0 | 1.022 | [1.010, 1.044] |
| R7 7x7 c=6 | mrv-rand-1 | bb-mrv | 0.906 | [0.438, 1.428] | 0.2908 | 17/11/2 | 2 | 5 | 1 | 0.906 | [0.468, 1.175] |
| R7 7x7 c=6 | mrv-rand-2 | bb-mrv | 0.881 | [0.496, 1.190] | 0.1710 | 18/10/2 | 2 | 5 | 3 | 0.871 | [0.490, 1.190] |
| R7 7x7 c=6 | mrv-rand-3 | bb-mrv | 0.903 | [0.585, 1.000] | 0.1723 | 18/8/4 | 4 | 3 | 0 | 0.903 | [0.606, 1.012] |
| R7 7x7 c=6 | scb | bb-mrv | 1.499 | [1.000, 114.744] | 0.9945 | 9/16/5 | 5 | 2 | 9 | 0.351 | [0.324, 0.550] |
| R7 7x7 c=6 | hall-v1 | cost-v1 | 0.993 | [0.992, 0.996] | 0.0018 | 23/0/7 | 7 | 0 | 0 | 1.005 | [0.995, 1.016] |
| R7 7x7 c=6 | hall-v2 | cost-v2 | 0.992 | [0.989, 0.993] | 0.0018 | 23/0/7 | 7 | 0 | 0 | 0.998 | [0.979, 1.015] |
| R7 7x7 c=6 | bb-mrv | scb | 0.976 | [0.636, 1.000] | 0.4262 | 16/9/5 | 5 | 9 | 2 | 2.864 | [1.971, 3.095] |
| R7 7x7 c=6 | hall-v1 | scb | 0.970 | [0.631, 1.000] | 0.4262 | 16/9/5 | 5 | 9 | 2 | 3.041 | [2.141, 3.236] |
| R7 7x7 c=6 | hall-v2 | scb | 0.969 | [0.630, 1.000] | 0.4262 | 16/9/5 | 5 | 9 | 2 | 3.054 | [2.157, 3.267] |
| R8 8x8 c=10 | hall-off | bb-mrv | 1.000 | [1.000, 1.000] | 1.0000 | 0/0/30 | 0 | 0 | 0 | 1.001 | [0.996, 1.004] |
| R8 8x8 c=10 | hall-v1 | bb-mrv | 0.999 | [0.999, 0.999] | 0.0000 | 29/0/1 | 0 | 0 | 0 | 1.017 | [0.998, 1.044] |
| R8 8x8 c=10 | hall-v2 | bb-mrv | 0.997 | [0.996, 0.998] | 0.0000 | 30/0/0 | 0 | 0 | 0 | 1.036 | [1.017, 1.056] |
| R8 8x8 c=10 | cost-v1 | bb-mrv | 1.000 | [1.000, 1.000] | 1.0000 | 0/0/30 | 0 | 0 | 0 | 1.035 | [1.010, 1.036] |
| R8 8x8 c=10 | cost-v2 | bb-mrv | 1.000 | [1.000, 1.000] | 1.0000 | 0/0/30 | 0 | 0 | 0 | 1.051 | [1.029, 1.061] |
| R8 8x8 c=10 | mrv-rand-1 | bb-mrv | 1.623 | [0.890, 2.118] | 0.8292 | 13/17/0 | 0 | 0 | 0 | 1.618 | [0.877, 2.200] |
| R8 8x8 c=10 | mrv-rand-2 | bb-mrv | 0.847 | [0.399, 2.596] | 0.2935 | 17/13/0 | 0 | 0 | 0 | 0.830 | [0.397, 2.574] |
| R8 8x8 c=10 | mrv-rand-3 | bb-mrv | 0.495 | [0.409, 0.768] | 0.0000 | 25/5/0 | 0 | 0 | 0 | 0.506 | [0.404, 0.799] |
| R8 8x8 c=10 | hall-v1 | cost-v1 | 0.999 | [0.999, 0.999] | 0.0000 | 29/0/1 | 0 | 0 | 0 | 1.002 | [0.991, 1.011] |
| R8 8x8 c=10 | hall-v2 | cost-v2 | 0.997 | [0.996, 0.998] | 0.0000 | 30/0/0 | 0 | 0 | 0 | 0.996 | [0.972, 1.001] |

## Cost control

`cost-vk` computes the Hall check at the same nodes as `hall-vk` and never
acts on it, so it has `bb-mrv`'s tree exactly. Median paired wall ratios
`cost-vk / bb-mrv`: R6 0.988 (V1), 1.040 (V2); R7 1.024, 1.022; R8 1.035,
1.051. The pruning recovers about that cost and no more: `hall-vk /
cost-vk` wall ratios are 1.007, 0.996 (R6), 1.005, 0.998 (R7), 1.002, 0.996
(R8), all with CIs containing 1. `hall-off / bb-mrv` is 0.982 to 1.001, so
the crate's solver matches A16's code in speed when the check is off.

## Exact commands

```
just experiments hall-on-mrv            # = research/experiments/hall-on-mrv/run.sh all
# stages: build | check | test | trace | analyze
compute/target/release/hall_on_mrv list
compute/target/release/hall_on_mrv run --rung r7 --role test --arms bb-mrv,hall-v1 --reps 1 \
  --threads 2 --cap 1000000 --guard 600 --out /tmp/x.jsonl
```

`run.sh trace` runs the same trace checks sequentially (they were run as two parallel loops, one per variant, with identical commands and outputs). `run.sh` skips raw files that already exist, so on a checkout with the
committed results `all` rebuilds, tests and re-analyses (identical
`summary.json`; node counts are deterministic, wall clock is not).

Stage wall clock on this machine (from `/usr/bin/time`): R6 10.5 s (5
repetitions, 10 arms), R8 755 s (9 arms), R7 2,186 s (10 arms), trace checks
about 8 min (both variants in parallel, one thread each), calibration (R8, 4 arms) 133 s. Total about 55 minutes of runs on
2 threads.

## Files

- `PREREG.md`: preregistration (commit 860e368).
- `compute/src/lib.rs`: `HallMrv` (A16 bb-mrv search plus the Hall hook),
  `hall_match`, `subtree_removal`. `compute/src/main.rs`: interleaved runner
  and `trace-check`. `compute/tests/units.rs`: the three-cells-two-pieces
  example, the trace comparison, and `hall-off` / cost arms equal to
  `BbSolver` on R6 calibration.
- `results/test/{r6,r7,r8}.jsonl`: one row per arm, instance and repetition.
- `results/trace/*.jsonl`: subtree-removal checks.
- `results/calibration/*.jsonl`: calibration rows used for the small check
  in PREREG.md section 1 (`r7.jsonl` holds two R7 seeds, 1001 and 1002, run before the
  calibration was stopped; see `PREREG_AMENDMENTS.md`).
- `results/summary.json`, `results/tables.md`: `analysis/analyze.py` output
  (stdlib only, 10,000 bootstrap resamples, seed 20261111).
