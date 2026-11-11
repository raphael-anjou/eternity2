# Calibration (calibration seeds 1001..1010 only, primary colours, cap 2,000,000 nodes)

Command: `compute/target/release/hard_ladder calibrate --threads 8` then
`python3 analysis/analyze.py calibration` (writes `results/calibration/ranking.json`).
Raw rows: `results/calibration/primary/<solver>.jsonl`. Wall clock 1220 s on
8 vCPU (8 runs in parallel). Selection rule from PREREG.md: most calibration
instances solved, ties broken by KM median nodes at n = 10, 9, 8, 7, 6. Every
candidate was eligible (no errors, zero inconsistent tries).

Median = Kaplan-Meier median over the 10 seeds of a size (5th order statistic,
unsolved = +infinity). Wall s = sum of per-run wall clock.

1220s
| rank | solver | eligible | solved/50 | solved n=6 | solved n=7 | solved n=8 | solved n=9 | solved n=10 | median n=6 | median n=7 | median n=8 | median n=9 | median n=10 | wall s |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | scb-gacolor_ac3_random | yes | 18 | 10 | 8 | 0 | 0 | 0 | 1,133 | 613,022 | > cap | > cap | > cap | 1896 |
| 2 | scb-gacolor_ac3 | yes | 17 | 10 | 7 | 0 | 0 | 0 | 1,054 | 652,941 | > cap | > cap | > cap | 2241 |
| 3 | scb-gacolor_ac3_ns1 | yes | 17 | 10 | 7 | 0 | 0 | 0 | 1,054 | 652,941 | > cap | > cap | > cap | 2110 |
| 4 | scb-gacolor_ac3_lcv | yes | 17 | 10 | 7 | 0 | 0 | 0 | 922 | 821,647 | > cap | > cap | > cap | 1920 |
| 5 | scb-border_first_full | yes | 10 | 10 | 0 | 0 | 0 | 0 | 5,128 | > cap | > cap | > cap | > cap | 333 |
| 6 | scb-border_first_lcv | yes | 10 | 10 | 0 | 0 | 0 | 0 | 5,255 | > cap | > cap | > cap | > cap | 111 |
| 7 | scb-rare_color_first | yes | 10 | 10 | 0 | 0 | 0 | 0 | 5,255 | > cap | > cap | > cap | > cap | 116 |
| 8 | engine-dfs-spiral-in | yes | 10 | 10 | 0 | 0 | 0 | 0 | 23,394 | > cap | > cap | > cap | > cap | 88 |
| 9 | scb-naive_spiral | yes | 10 | 10 | 0 | 0 | 0 | 0 | 23,394 | > cap | > cap | > cap | > cap | 115 |
| 10 | engine-dfs-border-first | yes | 10 | 10 | 0 | 0 | 0 | 0 | 30,781 | > cap | > cap | > cap | > cap | 81 |
| 11 | engine-dfs-snake | yes | 10 | 10 | 0 | 0 | 0 | 0 | 51,141 | > cap | > cap | > cap | > cap | 87 |
| 12 | hp-dfs | yes | 10 | 10 | 0 | 0 | 0 | 0 | 51,146 | > cap | > cap | > cap | > cap | 77 |
| 13 | scb-naive_rowmajor | yes | 10 | 10 | 0 | 0 | 0 | 0 | 51,146 | > cap | > cap | > cap | > cap | 100 |

Selected best baseline: **scb-gacolor_ac3_random** (single-core-benchmark
`gacolor_ac3` preset with randomised value order, fixed run seed; 18/50). The
margin over the deterministic gacolor_ac3 variants is one instance (8 vs 7
solved at 7x7), inside sampling noise; the rule is applied as preregistered.

Cross-checks visible in this table: `scb-naive_rowmajor` and `hp-dfs` report
identical node counts on every instance (the registry's event-counted metric
equals the main engine's placement count), as do `scb-naive_spiral` and
`engine-dfs-spiral-in`. No candidate solved any 8x8, 9x9 or 10x10 calibration
instance within 2,000,000 nodes.
