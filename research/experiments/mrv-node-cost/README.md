# Track 0: bb-mrv node cost (optimization, night 2026-11-12)

Preregistration: [`PREREG.md`](PREREG.md) (commit `47aa9b5`, committed before
any test seed ran; code frozen at `512b10d`). One layout amendment, see [`PREREG_AMENDMENTS.md`](PREREG_AMENDMENTS.md). Track label:
**optimization**. Generated boards only; nothing here is official Eternity II
progress.

## Outcome (self-assessed; a separate judge decides)

- **Gate: PASS.** 140 of 140 instances (R6 40, R7 40, 8x8 c=10 60; every
  calibration and test seed, cap 1e6): the instrumented copy equals the
  unmodified `bb-mrv` on status, nodes, backtracks, propagate calls and board,
  and `bb-mrv-fast` equals the copy on the node-sequence hash and on all of
  those fields. 0 mismatches. Supplementary (cap 1e5, not binding): 9x9 c=11
  seeds 1001..1003 and 10x10 c=12 seed 1001, 4 of 4 identical.
- **Timing: every timed run of both arms reproduced identical nodes and
  status** (110 instances x 10 runs).
- **Wall-clock ratio fast / original** (per-instance minimum of 5 runs,
  A/B/B/A alternation, median over instances, 95% percentile bootstrap CI,
  10,000 resamples, seed 20261111):

| rung | instances | median ratio | 95% CI | speed-up | secondary (median of 5) |
|---|---|---|---|---|---|
| R6 (6x6 c=5) | 30 | 0.188 | [0.178, 0.203] | 5.3x | 0.191 [0.181, 0.198] |
| R7 (7x7 c=6) | 30 | 0.163 | [0.161, 0.166] | 6.1x | 0.164 [0.161, 0.166] |
| 8x8 c=10 | 50 | 0.190 | [0.188, 0.191] | 5.3x | 0.190 [0.189, 0.192] |

- Decision rule: gate passes and the CI upper end is below 1 at all three rungs, so the preregistered adopt rule holds (`verdict.adopt = true`).

**For Track 1:** binary `research/experiments/mrv-node-cost/compute/target/release/mrv_node_cost`,
solver `bb-mrv-fast`, same CLI as `mrv_baseline run` (`--solver --ladder
--role --sizes --seeds --cap --guard --threads --out`). Supports n <= 11.

## What changed (cost only)

`compute/src/lib.rs`. `RefSolver` is the verbatim copy of A16's `Rule::Mrv`
search plus one hook call per node. `FastSolver` keeps the search (forced cell
then min (|D|, index), ascending option order, one counted node per
`try_assign`, same backtrack and propagate counts) and changes the per-node
cost: 4 bits per piece in the option bitset (rotation rank kept, so the value
order is unchanged), fixed-size const-generic domains, a preallocated state
stack (no heap allocation per node), piece presence per cell by a nibble fold,
the all-different pass on 128-bit piece masks instead of a cells x pieces
scan, and offered colours by walking the bits of small domains. Propagation
computes the same fixpoint of A3's rules; the gate is the evidence.

## Numbers and where they come from

All from `results/summary.json`, produced by `python3 -I analysis/analyze.py`
from `results/raw/*.jsonl`.

| table entry | raw file(s) | summary.json key |
|---|---|---|
| gate R6 / R7 / R8 | `gate-r6-*.jsonl`, `gate-r7-*.jsonl`, `gate-r8-*.jsonl` | `gate.R6`, `gate.R7`, `gate.R8` |
| supplementary | `gate-supp-n9.jsonl`, `gate-supp-n10.jsonl` | `gate.supp-n9`, `gate.supp-n10` |
| timing ratios | `time-r6.jsonl`, `time-r7.jsonl`, `time-r8.jsonl` + `time-r8-fresh.jsonl` | `timing.R6/R7/R8.ratio_min_*`, `ratio_med_*` |
| verdict | all of the above | `verdict` |

Throughput during timing (nodes summed over instances / sum of per-instance
minimum wall clock): R6 73k vs 383k nodes/s, R8 21.5k vs 113k nodes/s,
R7 38k vs 235k nodes/s (`timing.*.nodes_per_s_*`). These absolute rates are depressed by
other tracks running on the same machine (gate runs at R8 gave about 28k
nodes/s for the original); the paired ratio is the claim, not the rate.

## Reproduce

```
just experiments mrv-node-cost          # = mrv-node-cost/run.sh all
# stages: build | gate | supp | time | analyze; existing raw files are skipped
compute/target/release/mrv_node_cost gate --ladder 8-c10 --role test --out /tmp/g.jsonl
compute/target/release/mrv_node_cost run --solver bb-mrv-fast --ladder primary --role test --sizes 7 --out /tmp/r.jsonl
```

Hardware: Apple M4 (4P + 6E cores), macOS, rustc 1.95.0, one thread, `nice`,
no core pinning; other tracks ran concurrently. Stage wall clock
in `results/stage-times.txt` (gate 27.8 min, supplementary 1.1 min, timing
74.4 min). Wall clock is not reproducible bit for bit; node counts and
hashes are.

## Limits

The gate covers n = 6..8 at cap 1e6 and four small checks at n = 9, 10 at cap
1e5. A speed-up is a constant factor: it does not change any node-count
result, any G1/G2 statistic, or anything about 16x16.
