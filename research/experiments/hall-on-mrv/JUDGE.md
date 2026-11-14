# hall-on-mrv: judge's verdict

Branch `night/2026-11-12-t4-hall-on-mrv`, track 4 (label: method). Judged
from the branch only, against `PREREG.md` (commit 860e368) and
`research/experiments/hard-ladder/PROTOCOL.md`.

## Verdict: kill

Both preregistered variants are killed by rule K2, so the method is killed.
Hall matching on top of `bb-mrv` is GAIN at R7 and R8 in nodes (G1 ratios
0.992 to 0.999, i.e. under 1% fewer nodes) and NO-GAIN at R6, and at every
GAIN rung its median paired wall ratio against `bb-mrv` is above 1. The kill
does not rest on a fragile wall-clock sign alone: at R7 both variants' wall
CIs exclude 1 ([1.013, 1.055], [1.009, 1.038]), the cost controls show the
check itself costs 2 to 5% wall clock, and the node gain is far below any
practical threshold (the preregistered "no material effect" reading applies
as well). This replaces A17's earlier GAIN against `scb` (node ratio 0.90 at
R6): against a baseline that already applies A3's size-1 Hall rules, the
effect vanishes at R6 and is under 1% at R7 and R8.

## Preregistration order

| event | time (2026-11-12, +0100) | evidence |
|---|---|---|
| solver sources last written | 19:18:59 / 19:19:50 | mtimes of `compute/src/lib.rs`, `main.rs` (unchanged since; worktree clean) |
| release binary built | 19:20:53 | mtime of `compute/target/release/hall_on_mrv` (cargo rebuild today is a no-op) |
| calibration rows R6, R8, R7 | 19:20:59, 19:23:17, 19:24:02 | mtimes; roles all `calibration`, seeds 1001..1010 |
| `PREREG.md` last written | 19:24:03 | file mtime |
| PREREG commit 860e368 | 19:24:09 | git |
| `run.sh` last written | 19:24:23 | file mtime |
| R6 test run | about 19:24:23.5 to 19:24:34 | `r6.jsonl` mtime 19:24:34 minus `/usr/bin/time` real 10.53 s in the untracked `r6.jsonl.log` |
| code + calibration commit f4ba41e | 19:24:30 | git |
| R6 test commit cdf5285 | 19:24:58 | git |
| R8 test run | about 19:24:34 to 19:37:09 | mtime minus real 754.78 s |
| R7 test run | about 19:37:09 to 20:13:35 | mtime minus real 2186.17 s |

Established: the last write of every committed test file finished after the
PREREG commit, and for R6 the run that produced it started about 14 s after
the PREREG commit (R8 and R7 started about 25 s and 13 min after). The R6
log's first line is the R6 test header and its timing footer matches the
file, so the committed R6 rows come from that post-PREREG run.

Not established: file mtimes record only the last write, so an earlier R6
test run, read and then overwritten, cannot be ruled out from the files. The
binary existed from 19:20:53, so the committed code could have produced test
rows before the PREREG text existed. This bounds what a late PREREG could
change: node counts are deterministic and the rerun below reproduces every
committed R6 test row (all 10 arms, 5 repetitions) node for node from the
committed code, and the decision rule in PREREG section 6 is PROTOCOL.md's
G1 plus A17's Holm labelling, written before R7 and R8 existed. The only
outcome-bearing PREREG text that an R6 peek could have shaped is K2 and the
"no material effect" reading; with R6 NO-GAIN for both variants, R6 cannot
drive K2 either way, so no conclusion here depends on the R6 order.

Minor: PREREG section 1 and the README say one R7 calibration seed (1001)
ran; `results/calibration/r7.jsonl` holds seeds 1001 and 1002 (4 arms each),
both finished at 19:24:02, before the PREREG file was saved.

## Numbers recomputed (own script, from the committed raw rows)

All point estimates, CIs and p-values below match `results/summary.json`
and `README.md` exactly.

| rung | candidate | G1 ratio [95% CI] | p(med>=0) | W/L | wall ratio [95% CI] | label |
|---|---|---|---|---|---|---|
| R6 | hall-v1 | 1.000 [0.998, 1.000] | 0.903 | 12/0 | 1.024 [0.990, 1.056] | NO-GAIN |
| R6 | hall-v2 | 1.000 [0.991, 1.000] | 0.710 | 14/0 | 0.989 [0.951, 1.053] | NO-GAIN |
| R7 | hall-v1 | 0.993 [0.992, 0.996] | 0.0018 | 23/0 | 1.028 [1.013, 1.055] | GAIN |
| R7 | hall-v2 | 0.992 [0.989, 0.993] | 0.0018 | 23/0 | 1.025 [1.009, 1.038] | GAIN |
| R8 | hall-v1 | 0.999 [0.999, 0.999] | 0.0000 | 29/0 | 1.017 [0.998, 1.044] | GAIN |
| R8 | hall-v2 | 0.997 [0.996, 0.998] | 0.0000 | 30/0 | 1.036 [1.017, 1.056] | GAIN |

Cost controls `cost-vk / bb-mrv` wall: R6 0.988, 1.040; R7 1.024, 1.022;
R8 1.035, 1.051. `hall-vk / cost-vk` wall: 1.007, 0.996; 1.005, 0.998;
1.002, 0.996. Total nodes `bb-mrv` / `hall-v1` / `hall-v2`: R6 22,509 /
22,430 / 22,197; R7 17,601,059 / 17,505,488 / 17,474,005; R8 3,359,112 /
3,355,670 / 3,343,983. Solved: R6 30/30 all, R7 23/30 for `bb-mrv` and
both Hall arms, R8 30/30.

Integrity checks recomputed: 2,070 test rows, 0 errors, 0 guard stops, no
repetition disagreeing in status or nodes (R6, 5 reps); `hall-off`,
`cost-v1`, `cost-v2` equal `bb-mrv` in status, nodes, backtracks,
propagation and board on 90/90 instance-rungs, cost `seq_hash` equal to
`hall-off`'s on all; Hall nodes <= `bb-mrv` nodes and same board when both
solve on every test instance; the 120 committed trace-check rows all pass
subtree removal with no board difference.

## Rerun (committed code, 2 threads, shared machine)

Run 20:23 to 20:39 with `compute/target/release/hall_on_mrv run`, cap 1e6,
guard 600 s, load average about 14 to 21 on 10 cores.

| subset | rows | node-for-node match (status, nodes, backtracks, propagation, board, seq hash, Hall checks and fails) | rerun wall ratio vs `bb-mrv` (V1, V2) | committed |
|---|---|---|---|---|
| R6 full: 30 instances, all 10 arms, 5 reps | 1,500 | 1,500 / 1,500 | 1.054, 0.999 | 1.024, 0.989 |
| R8 full instance set, arms `bb-mrv`, `hall-v1`, `hall-v2` | 90 | 90 / 90 | 1.011, 1.028 | 1.017, 1.036 |
| R7 full instance set, arms `bb-mrv`, `hall-v1`, `hall-v2` | 90 | 90 / 90 | 1.009, 1.014 | 1.028, 1.025 |

Every rerun row matches its committed row exactly. The wall-clock sign at
the GAIN rungs (R7, R8) holds for both variants (ratio above 1, Hall slower
on 18 to 24 of 30 instances); the R7 magnitude is smaller in the rerun
(about 1% instead of 2.5%), which with three arms instead of ten is a
different interleaving and is within the noise of a loaded machine. R6 is
NO-GAIN either way, so its wall sign does not enter K2.

## Rigor and adversarial checks

- Track label holds: method. The node sequence is changed (pruning), the
  method targets are applied, and the order question is answered both by
  construction (the check never touches a domain) and by trace checks.
- Order-effect control present: `mrv-rand` at run seeds 1 to 3. The Hall
  node changes (ratios 0.992 to 1.000) are inside the run-seed spread
  (R6 0.891 to 1.281, R7 0.881 to 0.906, R8 0.495 to 1.623) but are not an
  order effect by the trace checks. Irrelevant to a kill.
- Node accounting is equal between arms: the Hall check runs after
  `NodeCounter::try_assign`, so a Hall-pruned node is still counted. Same cap
  (1e6) for every arm; no censored run counted as a win (censoring follows
  PROTOCOL.md; 7 R7 pairs are both censored and score 0).
- The baseline is the strongest available (`bb-mrv`, kept at R8 by the
  round-3 judge, a reported result from AGENDA.md, not remeasured here);
  `hall-off` shows the crate's re-implementation matches it in
  nodes and speed.
- No planted-solution leakage: `equals_planted` is computed after the solve
  and not read by any solver.
- No seed dropped or subsampled: 30 instances per rung as preregistered.
- Wall-clock design: arms interleaved per instance with rotated arm order, two
  workers taking consecutive jobs, 5 repetitions at R6, one at R7/R8 as
  preregistered. Load average 12 to 18 on 10 cores makes single wall ratios
  noisy, which is why the conclusion leans on the R7 CIs and the cost arms
  as well as the K2 point estimates.
- Claims past evidence: the README's explanation of why the gain is small
  (size-1 Hall rules already in A3, larger violations near the leaves) is
  argued, partly supported by the cost-arm fail counts; it is labelled as an
  explanation, not a result. No claim beyond 8x8 c=10.
