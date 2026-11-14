# mrv-scaling-pilot: 9x9 c=11 high-cap pilot, bb-mrv against scb

Night shift 2026-11-12, Track 1. Label: measurement. Preregistration:
[`PREREG.md`](PREREG.md) (commit 012cfa1, 18:51:02 CET), amendments A1 and A2
in [`PREREG_AMENDMENTS.md`](PREREG_AMENDMENTS.md). Generated boards only;
nothing here is official Eternity II progress.

Question: at 9x9 c=11 (A22 `plus2` track, test seeds 101..108, cap 1e8 nodes
per run), is the median paired log10 node ratio bb-mrv / scb-gacolor_ac3_random
below the A22 8x8 c=10 value T = -0.83215?

## Verdict (preregistered rule, applied literally)

**Kill: "no evidence the ratio keeps shrinking".** Both kill conditions fire.

| criterion | statistic | threshold | value | fires |
|---|---|---|---|---|
| K1 | M = upper median (5th smallest) of the 8 r_i | M >= T = -0.83215 | M = 0.000 (ratio 1.0) | yes |
| K2 | U = both node-censored pairs + pairs with a time-censored run | U > 4 | U = 6 | yes |

Descriptive, not binding: percentile bootstrap 95% CI of the upper median
(10,000 paired resamples, seed 20261111) is [-0.106, 0.000]. M is not in the
rounding band [-0.833, -0.83216).

Reading (JUDGE.md): K2 decides. K1 fires as a side effect of the ties, since
6 of 8 pairs are both node-censored at 1e8 and score 0, which pulls the upper
median to 0 whatever the true ratio. The kill is "no evidence the ratio keeps
shrinking", not evidence that it stopped; the pilot ran out of cap and did not
measure the ratio. Both informative pairs (103, 106) favour bb-mrv, as upper
bounds. A follow-up needs an scb cap near 1e9 (about 14 to 23 h per run at the
measured rates) or per-solver Kaplan-Meier medians instead of ties. scb is
randomized and each r is conditional on the single scb run seed (1) per instance.

Source: `results/summary.json` (`M_upper_median_log10`, `U_uninformative`,
`K1_fires`, `K2_fires`, `verdict`, `boot95_log10`), produced by
`analysis/analyze.py` from the four raw files below.

## Pair counts

| class | pairs | seeds |
|---|---|---|
| both solved | 0 | |
| one solved (bb-mrv solved, scb node-censored) | 2 | 103, 106 |
| one solved (scb solved, bb-mrv node-censored) | 0 | |
| both node-censored | 6 | 101, 102, 104, 105, 107, 108 |
| time-censored (05:15 guard) | 0 | |
| error or missing | 0 | |

## Per-pair table

Nodes are `nodes` (equal to `nodes_counted` on every row). r is the
preregistered log10 score; for seeds 103 and 106 it is log10(b / 1e8), an upper
bound on the true ratio.

| seed | machine | bb-mrv nodes | bb-mrv status | bb-mrv wall s | scb nodes | scb status | scb wall s | r (log10) |
|---|---|---|---|---|---|---|---|---|
| 101 | m4 | 100,000,000 | node-censored | 6,729 | 100,000,000 | node-censored | 4,915 | 0 |
| 102 | m4 | 100,000,000 | node-censored | 6,834 | 100,000,000 | node-censored | 5,203 | 0 |
| 103 | x86 | 78,349,212 | solved | 6,772 | 100,000,000 | node-censored | 7,720 | -0.106 |
| 104 | x86 | 100,000,000 | node-censored | 8,823 | 100,000,000 | node-censored | 7,890 | 0 |
| 105 | x86 | 100,000,000 | node-censored | 8,825 | 100,000,000 | node-censored | 7,653 | 0 |
| 106 | x86 | 9,970,108 | solved | 794 | 100,000,000 | node-censored | 8,282 | -1.001 |
| 107 | m1 | 100,000,000 | node-censored | 5,374 | 100,000,000 | node-censored | 3,570 | 0 |
| 108 | m1 | 100,000,000 | node-censored | 5,562 | 100,000,000 | node-censored | 3,587 | 0 |

Per-run nodes/s and wave are in `results/tables.md` and `results/summary.json`
(`runs`). Both arms of every seed ran on the same machine at the same time
(A1), so the paired wall ratio bb-mrv / scb in `summary.json` (`pairs[].wall_ratio`,
0.10 to 1.55) is within one machine.

**Wall times across the three machines are not comparable.** m4 (Apple M4),
x86 (Intel Core Ultra 7 165U, Linux) and m1 (Apple M1) differ in CPU and
were shared with other tracks; the node counts are the only cross-machine
quantity. Node counts are machine-independent: the build check (A22 `plus2`
7x7 seeds 1..10, both arms, cap 1e6) gives node-identical rows on all three
machines and matches A22's committed rows (`results/check/`,
`results/remote/<machine>/check/`).

## Raw data

| file | rows | machine | seeds |
|---|---|---|---|
| `results/raw/wave1-m4.jsonl` | 4 | m4 | 101, 102 |
| `results/remote/x86/raw/wave2-x86.jsonl` | 4 | x86 | 103, 104 |
| `results/remote/x86/raw/wave3-x86.jsonl` | 4 | x86 | 105, 106 |
| `results/remote/m1/raw/wave4-m1.jsonl` | 4 | m1 | 107, 108 |

All 16 rows: n = 9, c = 11, ladder `plus2`, role `test`, cap 100,000,000,
solvers `bb-mrv` (run seed 0) and `scb-gacolor_ac3_random` (run seed 1),
status `solved` or `censored`, `error` null. Each machine used its own build of
the A22 `mrv_baseline` binary (sha256 per row: m4 `9a914907...`, x86
`4e08b7f8...`, m1 `54752120...`); no wave switched to a Track 0 binary.
Per-run launch and finish lines are in `results/heartbeat-m4.log` and
`results/remote/<machine>/heartbeat-<machine>.log`.

## Cost

- Wall clock: first launch 18:52:45 CET (seed 101, m4), last row 21:23:34
  CET (bb-mrv seed 105, x86): 2 h 31 min.
- Core-hours (16 single-threaded runs, sum of `wall_ms`): 27.37, of which
  m4 6.58, x86 15.77, m1 5.03. The build checks (a few minutes per
  machine) are not included.

## Reproduce

From the repository root:

```
just experiments mrv-scaling-pilot analyze   # summary.json and tables.md from the committed rows (seconds)
just experiments mrv-scaling-pilot build     # build the A22 binary and pilot_instances
just experiments mrv-scaling-pilot check     # instance file check and build check (minutes)
just experiments mrv-scaling-pilot test      # all 16 runs on one machine, 4 slots (hours)
```

The placement used tonight (A2) is the same `test` stage with environment
variables, run from this directory on each machine:

```
MACHINE=m4 SEEDS="101 102"         SLOTS=4 ./run.sh test
MACHINE=x86   SEEDS="103 104 105 106" SLOTS=8 ./run.sh test
MACHINE=m1 SEEDS="107 108"         SLOTS=4 ./run.sh test
```

Node counts and statuses reproduce exactly (bb-mrv has no RNG, scb uses run
seed 1); wall clock does not.

## Deviations from the protocol

- The runs did not go through `research/experiments/common/prereg_guard.py`,
  so there is no `results/run-log.jsonl`. The guard was not on this branch
  (based on `origin/main` 6247af0); it reached the lead branch at 21:28 CET
  (ca02860), after the last run. PREREG.md (18:51:02) and amendment A1
  (18:52:38) were committed before the first launch (18:52:45), and PREREG.md
  has not changed since 012cfa1.
- The heartbeat file is `results/heartbeat-<machine>.log`, not
  `results/heartbeat.log` as PREREG.md says.
- A1's optional cross-machine re-run of one pair's cheaper arm was not done;
  the three-machine build check above is the only cross-machine determinism
  evidence.

## Context found after the runs

Reported by tonight's review `research/program/night/2026-11-12/memos/growth-law.REVIEW.md`
(section F) on the lead branch, not measured here: plain row-major DFS (the
review's `enum.c`, which reproduces committed `hp-dfs` rows node for node)
solves 9x9 c=11 with a Kaplan-Meier median of 1.56e8 nodes on A22's seeds 1..30
(18 of 30 under a 2e8 cap) and 1.61e8 on this pilot's seeds 101..108 (5 of 8
under 2e8, 3 of 8 under 1e8), in 2 to 6 s per instance. Here one bb-mrv solve
(seed 103, 7.8e7 nodes) took 6,772 s. At this rung node counts hide a per-node
cost that differs by more than three orders of magnitude between plain DFS and
these two solvers, so a node ratio between bb-mrv and scb says nothing about
which search is cheapest in time.
