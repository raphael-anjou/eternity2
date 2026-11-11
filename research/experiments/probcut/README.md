# ProbCut statistical cutoffs for exact search (round 2)

Pre-registered test (`PREREG.md`, commit 7ae11f4; amendment B1 in
`PREREG_AMENDMENTS.md`) of Buro's ProbCut / Multi-ProbCut carried over to exact
Eternity II search: at filled depths d in {n, 2n, 3n} of a forward-checked
row-major DFS, count the forward-checked nodes a few cells deeper, and cut the
subtree when log2(1 + count) falls below a threshold calibrated on planted
solution paths; relax the threshold pass by pass toward p = 0 (complete).
Built on the shared hard ladder (`../hard-ladder`): its harness, node counter
(`NodeCounter::try_assign`, one per (cell, piece, rotation) assignment, probe
placements and failed passes included), caps, seeds and `PROTOCOL.md`.

## What was run

- Rungs: R6 = 6x6 c=5 and R7 = 7x7 c=6 (ladder `primary` instances, committed
  ladder baselines), R8 = 8x8 c=10 (`instances/8-c10-*.jsonl`, generated with
  the ladder's call `generate_solved_framed_bc(8, 15, seed, true, 5)` plus its
  `XorShift(seed ^ 0xA5A5A5A5)` scramble; `probcut check-instances` verifies
  byte identity with the generator and every planted board).
- Seeds: calibration 1001..1010 (all tuning), test 1..30 (each candidate
  variant run once). Cap 1e6 nodes on test, 2e6 on calibration.
- Solvers (`compute/`, plug-in `SolverEntry`s): `fc-dfs` (row-major DFS with
  forward checking, the "same DFS"), `fc-luby` (Luby restarts, randomised value
  order, unit chosen on calibration), `probcut-v1` (p = 0.2, 0.05, 0.01, 0),
  `probcut-v2` (p = 0.05, 0); the ladder's `scb-gacolor_ac3_random` (verdict
  baseline) and `hp-dfs`, run here on R8 and taken from the ladder's committed
  results on R6/R7. `rm-dfs` (no forward checking) reproduces `hp-dfs` node for
  node (checked on calibration and in `cargo test`).
- Calibration (`CALIBRATION.md`, committed in 6d7c361 before any test run):
  Luby unit 10000; probe index k = 1 (counts 2 cells deeper) on every rung;
  mu and sigma of v_1 on the planted prefix per depth.

## Reproduce

```
just experiments probcut          # or: research/experiments/probcut/run.sh
cd research/experiments/probcut/compute && cargo test --release && cargo clippy --release --all-targets -- -D warnings
```

`run.sh` builds, regenerates and checks the R8 instances, runs stage 1 and the
calibration runs, writes `params.json` and `CALIBRATION.md`, runs all test
solvers, then `analysis/probcut.py report` (stdlib Python, reuses the ladder's
`analyze.py`). A clean clone of the branch reproduced every committed jsonl
(all fields except `wall_ms`), `params.json` and `CALIBRATION.md` exactly.

Hardware: Devin VM, 8 vCPU Intel Xeon, 31 GiB RAM, 8 runs in parallel.
Wall clock of the full recipe from a clean clone: 1 min 56 s (most of it
`scb-gacolor_ac3_random` on R8: 598 CPU-seconds over 30 instances).

## Stage 1 (calibration seeds, `CALIBRATION.md`, `results/calibration/stage1.json`)

The planted (d-1)-prefix has very few forward-checked children at these colour
counts: over 10 calibration puzzles and 3 depths per rung, only 9 (R6), 8 (R7)
and 6 (R8) sibling sets have 2 or more children. Within those sets the
deeper probes rank the planted child exactly as v_0 does: pooled AUC is
identical for v_0..v_4 on every rung (0.556, 0.562, 0.417). AUC gain of the
best k over v_0 on R8 is 0.000 at d = 8, 16, 24 (CI [0, 0]). Best-k R^2 on R8
is 0.12, 0.17, 0.61; held-out false-cut rates on R8 are 0.067 (p = 0.05) and
0.200 (p = 0.2). The stage-1 kill fires on its AUC clause. The test has little
power (6 sibling sets on R8); the measured statement is that whenever the
planted parent has siblings, a 2..5-cell subtree-size probe orders them no
differently from the immediate-extension count.

## Stage 2 (test seeds 1..30, cap 1e6, `results/RESULTS.md`, `results/summary.json`)

| rung | solver | solved (Wilson 95%) | KM q25 / median / q75 nodes | median wall ms | total wall s | first pass solved | false-cut rate (pass 1) |
|---|---|---|---|---|---|---|---|
| R6 6x6 c=5 | scb-gacolor_ac3_random | 30/30 (0.89-1.00) | 620 / 951 / 1,940 | 6.2 | 0.3 | | |
| R6 | hp-dfs | 30/30 | 27,904 / 41,823 / 62,556 | 23.7 | 0.8 | | |
| R6 | fc-dfs | 30/30 | 25,961 / 38,866 / 56,883 | 4.2 | 0.2 | | |
| R6 | fc-luby | 30/30 | 32,290 / 74,830 / 144,530 | 7.6 | 0.3 | | |
| R6 | probcut-v1 | 30/30 | 26,858 / 171,321 / 248,763 | 14.7 | 0.5 | 27/30 | 0.70 |
| R6 | probcut-v2 | 30/30 | 26,946 / 41,315 / 77,397 | 4.8 | 0.2 | 30/30 | 0.07 |
| R7 7x7 c=6 | scb-gacolor_ac3_random | 16/30 (0.36-0.70) | 272,281 / 921,773 / > cap | 7347 | 173.1 | | |
| R7 | hp-dfs, fc-dfs, fc-luby | 0/30 (0.00-0.11) | > cap | 100 to 650 | 3 to 20 | | |
| R7 | probcut-v1 | 0/30 | > cap | 95.8 | 3.0 | 0/30 | 0.43 |
| R7 | probcut-v2 | 0/30 | > cap | 92.0 | 2.9 | 0/30 | 0.13 |
| R8 8x8 c=10 | scb-gacolor_ac3_random | 19/30 (0.46-0.78) | 363,902 / 505,364 / > cap | 17311 | 597.8 | | |
| R8 | hp-dfs | 19/30 | 360,947 / 509,862 / > cap | 698.6 | 20.2 | | |
| R8 | fc-dfs | 19/30 | 360,829 / 508,613 / > cap | 44.0 | 1.6 | | |
| R8 | fc-luby | 16/30 (0.36-0.70) | 306,878 / 820,909 / > cap | 75.2 | 1.8 | | |
| R8 | probcut-v1 | 13/30 (0.27-0.61) | 395,888 / > cap / > cap | 64.2 | 1.6 | 13/30 | 0.30 |
| R8 | probcut-v2 | 17/30 (0.39-0.73) | 408,107 / 550,557 / > cap | 55.6 | 1.6 | 17/30 | 0.13 |

Passes used: on R7 and R8 every candidate run stayed in pass 1 (a solve or
the cap came before pass 1 exhausted its cut tree), so the relaxation never
started; on R6 v1 needed 2 passes on 3 instances. Wall-clock numbers are not
comparable across implementations (scb runs full AC-3 per node).

G1 against `scb-gacolor_ac3_random` (upper median paired log-ratio, bootstrap
10,000, seed 20261111), Holm over the two variants:

| rung | variant | median ratio | 95% CI of median log-ratio | one-sided p | label (Holm) |
|---|---|---|---|---|---|
| R6 | v1 | 109.0 | [+4.10, +5.17] | 1.000 | NO-GAIN |
| R6 | v2 | 34.2 | [+3.13, +4.50] | 1.000 | NO-GAIN |
| R7 | v1 | inf | [0.00, inf] | 1.000 | NO-GAIN |
| R7 | v2 | inf | [0.00, inf] | 1.000 | NO-GAIN |
| R8 | v1 | 1.000 | [0.00, inf] | 0.981 | NO-GAIN |
| R8 | v2 | 1.000 | [-0.26, +0.52] | 0.818 | NO-GAIN |

Against the same DFS without cuts (`fc-dfs`, secondary): R6 v1 ratio 2.55
(CI [+0.45, +1.33]), v2 1.063 (CI [+0.03, +0.11]); R7 tie (all censored); R8
v1 1.000 (CI [-0.06, 0.00]) with 6 fewer solves, v2 1.000 (CI [0.00, +0.04])
with 2 fewer solves. G2 on R6/R7 is undefined (candidate medians censored at
R7). Full table, including `fc-luby` and `hp-dfs` comparisons:
`results/RESULTS.md`.

## Outcome against the kill criterion (self-assessment; a separate judge decides)

- Stage-1 kill (R8): fires (AUC gain of the best k over v_0 is 0 at every depth).
- Stage-2 kill: fires on both clauses. v1's KM median is censored on R7 and R8
  (not <= 0.7x the better baseline), and v1's first pass solves 13/30 on R8 (<= 15).
- Round rules: NO-GAIN (Holm) for both variants on all three rungs, including R8.

Self-assessed outcome: killed. The cuts did not reduce nodes to first
solution on any rung; on R8 they removed solutions (13/30 and 17/30 solved
against 19/30 for the same DFS) and on R6 they cost 6 % (v2) to 155 % (v1)
extra nodes against the same DFS. Within the measured range (k up to 4,
probe depth up to 5 cells, row-boundary tests, these colour counts), a
subtree-size probe carried no planted-path information beyond the
forward-checking count, and the Gaussian thresholds then act as random cuts
at the stated rate.

## Files

- `PREREG.md`, `PREREG_AMENDMENTS.md`, `CALIBRATION.md`
- `compute/` Rust crate (path deps on `../../../../engine` and `../../hard-ladder/compute`)
- `instances/8-c10-{test,calibration}.jsonl`
- `results/calibration/` stage-1 rows, Luby and fc-dfs calibration runs, `params.json`, `stage1.json`
- `results/test/` per-instance runs (`*.jsonl`), per-run pass diagnostics
  (`*.passes.jsonl`), planted-path probe values on test seeds (used only for
  the false-cut rate), G2 files
- `results/summary.json`, `results/RESULTS.md`
