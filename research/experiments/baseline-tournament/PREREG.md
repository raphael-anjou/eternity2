# Baseline tournament: preregistration (night shift 2026-11-12, Track 5)

Track label (NIGHT_BRIEF.md): **measurement**. No new solver, no tuning. Every
entrant runs with the configuration its own experiment committed. Generated
boards only; nothing here is official Eternity II progress, and nothing is
extrapolated to 16x16.

## Question

Which solver-like components already in the repo give the smallest search
trees on the hard ladder's comparison rungs, when every one of them is counted
by the same `NodeCounter::try_assign` (one node per (cell, piece, rotation)
assignment, PROTOCOL.md) under the same 1e6 node cap? The adopted baseline
(`bb-mrv`) was found as a control; this measurement looks for others on purpose.

## Rungs and instances (fixed, from `hard-ladder/instances/`)

| rung | board | file | seeds | count |
|---|---|---|---|---|
| R6 | 6x6 c=5 | `primary-test.jsonl`, n = 6 | 1..30 | 30 |
| R7 | 7x7 c=6 | `primary-test.jsonl`, n = 7 | 1..30 | 30 |
| R8 | 8x8 c=10 | `8-c10-test.jsonl` seeds 21..30 + `8-c10-fresh-test.jsonl` seeds 31..50 (A22's Q1 set) | 21..50 | 30 |

A solver receives only the `puzzle` object. Every returned board is verified by
the harness (`hard_ladder::run_one`). Cap C = 1,000,000 nodes per run (TEST_CAP).

## Counter and runner

All entrants run through `hard_ladder::run_one` (unchanged), one row per
(entrant, run seed, instance), in a runner crate that path-depends on the
entrants' own crates (`compute/`). No other experiment's file is edited. Where
an experiment's registry lives in its binary (not reachable as a library), the
runner re-declares the same constructor call (a one-line `SolverEntry`), and the
rerun-identity check below is the proof that it is the same solver.

Wall-clock guards: an entrant keeps the guard its experiment committed (A22
arms 600 s, A16 arms 240 s; every other entrant has none). A run stopped by its
guard is censored at C for the node statistics and flagged `time_censored`; the
count of such runs is reported per entrant and rung. A guard makes that row
depend on the machine; this is reported, not corrected.

## Entrants (complete solvers; ranked)

Source crate / constructor. "stoch" = uses a run seed: run seeds 1, 2, 3.

| # | entrant | source | constructor | stoch |
|---|---|---|---|---|
| 1 | bb-mrv | mrv-baseline (A22) / backdoor-branching (A16) | `BbSolver { rule: Rule::Mrv, guard: 600 s }` | no |
| 2 | scb-gacolor_ac3_random | hard-ladder registry | `Scb(EngineSolver::gacolor_ac3_random())` | yes |
| 3 | hp-dfs | hard-ladder registry | `EngineDfs { "row-major" }` | no |
| 4 | engine-dfs-spiral-in | hard-ladder registry | `EngineDfs { "spiral-in" }` | no |
| 5 | engine-dfs-border-first | hard-ladder registry | `EngineDfs { "border-first" }` | no |
| 6 | engine-dfs-snake | hard-ladder registry | `EngineDfs { "snake" }` | no |
| 7 | scb-naive_rowmajor | hard-ladder registry | `Scb(NaiveSolver::row_by_row())` | no |
| 8 | scb-naive_spiral | hard-ladder registry | `Scb(NaiveSolver::spiral())` | no |
| 9 | scb-border_first_lcv | hard-ladder registry | `Scb(EngineSolver::border_first_lcv())` | no |
| 10 | scb-border_first_full | hard-ladder registry | `Scb(EngineSolver::border_first_full())` | no |
| 11 | scb-border_first_random | hard-ladder registry | `Scb(EngineSolver::border_first_random())` | yes |
| 12 | scb-rare_color_first | hard-ladder registry | `Scb(EngineSolver::rare_color_first())` | no |
| 13 | example-dfs | hard-ladder registry | `ExampleDfs` | no |
| 14 | scb-gacolor_ac3 | hard-ladder registry | `Scb(EngineSolver::gacolor_ac3())` | no |
| 15 | scb-gacolor_ac3_ns1 | hard-ladder registry | `Scb(EngineSolver::gacolor_ac3_ns1())` | no |
| 16 | scb-gacolor_ac3_lcv | hard-ladder registry | `Scb(EngineSolver::gacolor_ac3_lcv())` | no |
| 17 | mrv-nohs | mrv-baseline (A22) | `MrvSolver { Free, random: None, hs: false, guard 600 s }` | no |
| 18 | mrv-bf | mrv-baseline (A22) | `MrvSolver { BorderFirst, None, hs: true, guard 600 s }` | no |
| 19 | mrv-rand | mrv-baseline (A22) | `MrvSolver { Free, Some(seed), hs: true, guard 600 s }` | yes |
| 20 | scb-ad | mrv-baseline (A22) | `GuardScb(mrvb fork gacolor_ac3_random().with_alldiff(true), 600 s)` | yes |
| 21 | fc-dfs | probcut (A12) | `probcut::SOLVERS["fc-dfs"]` | no |
| 22 | rm-dfs | probcut (A12) | `probcut::SOLVERS["rm-dfs"]` | no |
| 23 | fc-luby | probcut (A12) | `probcut::SOLVERS["fc-luby"]`, Luby unit from `params.json` | yes |
| 24 | probcut-v1 | probcut (A12) | `probcut::SOLVERS["probcut-v1"]`, committed `params.json` | no |
| 25 | probcut-v2 | probcut (A12) | `probcut::SOLVERS["probcut-v2"]`, committed `params.json` | no |
| 26 | cmp-hall-frontier | cut-multiset-pruning (A17) | `Scb(cmp fork gacolor_ac3_random().with_hall(scope 1, period 1))` | yes |
| 27 | cmp-hall-all-k4 | cut-multiset-pruning (A17) | same, scope 2, period 4 | yes |
| 28 | cmp-hall-all | cut-multiset-pruning (A17) | same, scope 2, period 1 | yes |
| 29 | cmp-hall-frontier-k4 | cut-multiset-pruning (A17) | same, scope 1, period 4 | yes |
| 30 | bb-impact | backdoor-branching (A16) | `BbSolver { Rule::Impact, guard 240 s }` | no |
| 31 | bb-learned | backdoor-branching (A16) | `BbSolver { learned_rule(), guard 240 s }` (committed `model.json`) | no |
| 32 | bb-single | backdoor-branching (A16) | `BbSolver { single_rule(), guard 240 s }` | no |
| 33-37 | engine-dfs-{column-major, spiral-out, diagonal, double-snake, random} | main engine via hard-ladder `EngineDfs` | path kinds not yet on the ladder; `random` uses the wrapper's fixed path seed 0 | no |
| 38-47 | scb-{border_first_parity, border_first_gacolor, chess_gacolor, gacolor_symbreak, chess_gacolor_ac3, joe_depth150, joe_depth150_bp, joe_depth150_bp_rec, joe_depth150_bp_rec_layered, verhaard_preferred} | single-core-benchmark `solver-engine` presets via hard-ladder `Scb` (run seed 0) | single-thread presets not yet on the ladder | no |
| 48 | scb-fork-off | mrv-baseline (A22) check arm | fork with hook off | yes |
| 49 | cmp-fork-off | cut-multiset-pruning (A17) check arm | fork with Hall off | yes |

`bb-mrv` is one entrant: A16 and A22 committed the same solver (A22 checked
60/60 identity). It is compared against both experiments' committed rows.

## Entrants in the separate incomplete table (never ranked against complete solvers)

| entrant | source | configuration | stoch |
|---|---|---|---|
| ktam-b3 | ktam-unassignment (A15) | `Unassign(Rule::Random{r: inf, k: 2})`, seed `run_seed * 1_000_003 + instance seed` (A15's driver formula) | yes |
| ktam-v1 | ktam-unassignment (A15) | `Unassign(Rule::Ktam{beta 2, r inf, k 1})` | yes |
| ktam-v2 | ktam-unassignment (A15) | `Unassign(Rule::Ims{tabu 0, noise 0.1})` | yes |

These evict pieces non-chronologically and cannot prove exhaustion; their node
count is their placements (A15 counted placements + removals as events; the
removal count is not the tournament's metric). Definitions: **incomplete** =
cannot certify that no solution exists; **planted-solution-aware** = reads the
test instance's planted board at run time. No ranked entrant reads the planted
board. ProbCut (A12) and bb-learned (A16) were calibrated or trained on
planted labels of other instances (calibration seeds, A3 boards); they are
complete and planted-blind at run time and stay in the ranked table, with that
fact in their row.

## Excluded components (reported as a table, with reason)

| component | reason |
|---|---|
| solver-engine `*_par` presets (border_first_lcv_par, border_first_full_par, border_first_gacolor_par, chess_gacolor_par, gacolor_symbreak_par, gacolor_ac3_par, gacolor_ac3_random_par, gacolor_ac3_ns1_par, gacolor_ac3_lcv_par, joe_depth150_par, joe_depth150_bp_par, joe_depth150_bp_rec_par, joe_depth150_bp_rec_layered_par, joe_depth150_bp_x_par, verhaard_preferred_par) | internally multi-threaded: no single serial node sequence, and they would exceed the 4 threads assigned |
| solver-engine blackwood_base / blackwood_raw (and `_par`) | break-tolerant schedule built for 16x16; accepts mismatching tries, which the node definition excludes |
| single-core-benchmark standalone binaries verhaard, blackwood, producer, alns | 16x16 CSV input only, not complete DFS (hard-ladder PREREG); producer and alns also withheld (`eligible = false`) |
| solver-verhaard crate | 16x16-specific (196 inner pieces, scaffold of about 80) |
| edge-solver crate (`EdgeSolver`) | variables are edges, not cells: it makes no (cell, piece, rotation) assignment to count without redefining a node |
| sat-encoder crate | CNF encoder for an external SAT solver; no search to hook |
| localsearch crate, repair-study, quality-diversity-basins | local search places pieces that disagree with neighbours; the node metric is undefined for it; incomplete |
| engine `break_solver` | accepts mismatches (break-tolerant) |
| dfs-study registry | compile-time sizes 8..16 even only with 16x16 rim indices (hard-ladder PREREG); break variants accept mismatches |
| sat-backdoors (A3) search | backdoor search fixes cells to the planted placement (needs the planted solution); its propagation is entered through bb-mrv and the A22 arms |
| frame-count-corners (A5) | counts frame placements of the 16x16 official ring; produces no board |
| near-codeword-spectrum (A10) | measures distance to the planted solution (needs it) |
| spectral-planted-recovery (A1) | statistics on planted boards, no search |
| bandsaw | analysis tool: brackets the endgame band of an existing 16x16 board, not a solver |
| hint-study, fill-order-mean-field | drive the main engine DFS with hints (covered by engine-dfs entrants) / a Python mean-field model, no search |
| A15 grid configurations other than the three selected | tuning candidates rejected on calibration by A15's own rule; only selected configurations were committed for test |
| hard-ladder test `FirstFit`, engine test fixtures | test scaffolding, not solvers |
| mrv-scaling-pilot | tonight's Track 1, not on origin/main |

## Statistics (fixed in advance)

Per entrant, run seed and rung: solved count out of 30 with Wilson 95% interval;
Kaplan-Meier quartiles of nodes with censored = +infinity (Q1 = 8th, median =
15th, Q3 = 23rd order statistic of 30, as PROTOCOL.md's 15th for the median);
median and total wall ms; time-censored count; error count; total inconsistent
tries.

**Ranking per rung** (complete entrants only): solved/30 descending, then KM
median ascending, then KM Q1 ascending, then KM Q3 ascending, then sum of nodes
(censored at C) ascending. Stochastic entrants are ranked on the run-seed
median of each of these statistics (median of solved counts, median of KM
medians with +infinity allowed, and so on) and their row reports the spread
(min to max over seeds). If seeds 2 and 3 of a stochastic entrant are not
reached, it is ranked on seed 1 and the row says so.

**Validity per entrant and rung.** An entrant with any `error` row or any
inconsistent try at a rung is listed as invalid at that rung (reason given)
and not ranked there.

**Rerun identity.** For every entrant, every committed row in
`research/experiments/*/results/**/*.jsonl` on the same instance (ladder, n, c,
seed), the same cap and the same run seed (A15 rows: same configuration) is
compared with the rerun on (status, nodes). A row whose committed or rerun copy
is time-censored is compared and, if it differs, reported as a guard
difference, not a node mismatch. Every other difference is listed as a finding.

**Screening against the adopted baseline (descriptive, no multiplicity
correction).** Per rung, each ranked entrant's paired PROTOCOL G1 against the
adopted baseline of that rung, run here: `scb-gacolor_ac3_random` (run seed 1)
at R6 and R7, `bb-mrv` at R8. Statistic: upper median of the 30 paired log
ratios with PROTOCOL's censoring table; 95% percentile bootstrap, 10,000
paired resamples, seed 20261111; "beats" iff exp(median) <= 0.5 and the CI
upper end < 0. Also reported: median paired wall ratio. A "beats" here is a
screening flag for a separately preregistered confirmation, not a claim.

## Kill criterion (validity of the measurement)

The tournament is **invalid** (killed) if either anchor fails to reproduce its
committed rows: `bb-mrv` against A22's Q1 rows (R6/R7 seeds 21..30 and the 30
R8 rows) and A16's rows (R6/R7 seeds 1..20), or `scb-gacolor_ac3_random` run
seed 1 against hard-ladder's committed R6/R7 rows (60) and A22's R8 rows (30).
Statistic: count of (status, nodes) mismatches on non-time-censored rows;
threshold: any mismatch > 0 kills. Otherwise the deliverable is the ranked
table per rung, the incomplete table, the excluded table and the identity
report.

## Compute, order and caps

Machine: `dell` (Intel Core Ultra 7 165U, 14 threads, 30 GB), 4 threads
(`--threads 4`, `cargo build -j 4`), shared with 8 single-thread runs of
another track. Wall clock is indicative only; nodes/s is not comparable with
other experiments. Budget: at most 10 core-hours; hard stop for launching new
work at 22:00 local on the Dell; whatever is not reached is reported as not
reached, by entrant and rung.

Run order (most-cited baselines first, then by expected cost):

1. Tier 1: bb-mrv; scb-gacolor_ac3_random seeds 1, 2, 3; hp-dfs.
2. Tier 2: entrants 4 to 16 (seed 1 for stochastic), then 17, 18, 19 (seed 1), 20 (seed 1).
3. Tier 3: 21 to 25 (seed 1), ktam-b3/v1/v2 (seed 1), 26 to 29 (seed 1), 30 to 32.
4. Tier 4: entrants 33 to 47.
5. Tier 5: seeds 2 and 3 of every stochastic entrant, in entrant order.
6. Tier 6: check arms 48 and 49 (seed 1).

Within a tier, rungs run R6, R7, R8 per entrant.
