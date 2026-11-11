# Judge report: hard-instance ladder (devin/r2-hard-ladder @ 85c5bf2)

## Verdict: kill

The ladder is valid as infrastructure, and its own preregistered kill criterion fired. Condition 2 (8x8 solve rate at least 15/30 on the primary colour set) fails. The best baseline solves 0/30 at the amended cap of 1e6 nodes. I reran it at the original preregistered cap of 1e7 nodes, and it solves 1/30 (primary, c = 8) and 0/30 (secondary, c = 7). So the kill does not depend on amendment A1. The committed numbers reproduce node for node. I found no planted-solution leakage, no node-accounting mismatch between solvers and no dropped censored rows. Two analysis-script bugs break the README's one-command candidate workflow. They are should-fix items for the follow-up session and do not change the kill.

## Preregistration and amendments

- `PREREG.md` was committed alone in 8b316e4 (13:00:39 UTC). The harness and instances came next in d36019c (13:05:50), with no results. Calibration, test results, CALIBRATION.md and PREREG_AMENDMENTS.md were all committed together in 85c5bf2 (13:31:57). The preregistration therefore came before any result.
- Git history cannot confirm that A1 and A2 were written "before any test-seed run", because the amendments and all results land in one commit. Two things make this matter little:
  - A1 lowers the test cap from 1e7 to 1e6. That can only lower solve rates, so it pushes toward a kill.
  - My rerun at the original cap of 1e7 also kills: 1/30 at 8x8 primary.
- A2 defers rungs 9 and 10, which leaves condition 1 only partly evaluated (6 < 7 < 8). Condition 2 alone is enough to kill, and it is fully evaluated. Neither amendment is a post-hoc rescue.
- The kill criterion has two parts: a strict median increase over 6..10, and an 8x8 solve rate of at least 15/30 at cap C on the primary colours, with censored instances counted as +infinity. This is a usability criterion for the requested ladder: hard but solvable rungs with search effort that grows with size. The binding statement I received is cut off after "Suggested core: G1 at", so I could not compare word for word beyond that point. What I can see matches.
- The runner refused the lead's instruction to report usable=true and reported the kill. That is correct.

## Recomputed numbers (own script over committed raw rows, plus reruns)

- **Committed raw rows.** `results/test/*/*.jsonl` and `results/calibration/primary/*.jsonl` give exactly the README tables. Recomputed best baseline (scb-gacolor_ac3_random), cap 1e6:
  - Primary: n6 c5 30/30, KM median 951; n7 c6 16/30, median 921,773; n8 c8 0/30 (Wilson 0.00 to 0.11).
  - Secondary: n6 c6 30/30, median 300; n7 c7 30/30, median 77,575; n8 c7 0/30.
- **hp-dfs**, same cap:
  - Primary: 30/30 (median 41,823), 0/30, 0/30.
  - Secondary: 30/30 (median 2,374), 20/30 (median 693,573), 0/30.
- **Growth and bootstrap.** Growth is x1.697 per cell (primary) and x1.533 per cell (secondary). The bootstrap fraction with a strict increase over 6..8 is 0.698 primary and 1.000 secondary (my own RNG stream; the committed values are 0.70 and 1.00).
- **Integrity checks.** Across all test rows: zero `error` rows, zero inconsistent tries, zero solved rows above the cap, and `native_nodes == nodes` on every solved registry row.
- **Calibration ranking.** Recomputed from the raw rows, it matches `ranking.json`: gacolor_ac3_random 18/50, gacolor_ac3, ns1 and lcv 17/50, the plain DFS family 10/50, border_first_random 9/50.
- **Determinism.** I reran the full test grid (n = 6..8, both colour sets, both solvers; `--out` to a scratch dir) at commit 85c5bf2. All 360 rows match the committed rows exactly in status, nodes and backtracks. `check-instances` passes. `cargo test --release` passes 9/9.
- **Original cap (my rerun).** At the preregistered 1e7 cap (TEST_CAP edited in a scratch worktree only):
  - Best baseline, 8x8 primary c = 8: 1/30 solved (s11 at 2,369,732 nodes), median wall 166 s per run.
  - Best baseline, 8x8 secondary c = 7: 0/30.
  - hp-dfs, 7x7 primary: 4/30.
  - Condition 2 fails at the original cap as well.
- **Owen criterion.** I recomputed I = (P! 4^P)^(1/(2P)) independently: 5.2155, 6.3803, 7.5579, 8.7435, 9.9346. This matches `PREREG.md`.
- **hardness-peak cross-check** (`hardness-peak-8x8.json`, seeds 1..30, cap 3e7 steps, about 1.5e7 placements):
  - 0/30 at c = 6, 7, 8.
  - 23/30 at c = 9 (KM median 8,474,080 placements); 30/30 at c = 10 (median 509,862).
  - It solves at c = 1..5: 12, 11, 14, 27 and 12 out of 30.

## Rigor-check (adapted to an experiment)

1. **Baseline strength and fairness.** The candidate pool covers every complete DFS in the repo that runs at runtime sizes. The exclusions (break-gated joe profiles, verhaard_preferred, 16x16-only binaries, dfs-study's compile-time sizes) are justified in the preregistration. Selection used calibration seeds only.
   - The margin of the selected gacolor_ac3_random over deterministic gacolor_ac3 is one instance. CALIBRATION.md says so, and the choice does not change the kill: no candidate solved any calibration 8x8.
   - **Node accounting.** The registry engines emit ValueTried right before their own `stats.nodes += 1` (solver-engine lib.rs:2794-2799). The sink replays each try against the partial board, with zero inconsistent tries. Propagation never places a piece without emitting ValueTried. scb-naive_rowmajor equals hp-dfs, and scb-naive_spiral equals engine-dfs-spiral-in, node for node. The example plug-in also equals hp-dfs on all 90 primary test rows; I checked this through an external crate.
2. **Claims against data.** The headline ("killed by its own criterion") is supported. One prose claim is wrong (README:23, below). The README does not claim the ladder is usable at 8x8.
3. **Exact search versus record hunting.** These are exact complete DFS only, with no 16x16 search. The scope is stated.
4. **Wall clock.** The selected baseline costs about 17x more wall clock per node than hp-dfs (8x8: 16 s against 0.93 s per 1e6 nodes). Its node win is not a wall-clock loss, though. At roughly equal wall clock on 7x7 primary, gacolor solves 16/30 (1e6 nodes, median 7.3 s), while hp-dfs solves 4/30 (1e7 nodes, median 5.1 s).
5. **Censoring.** Censored rows are kept. The KM median is the ceil(N/2)-th order statistic with censored = +infinity, which is exact under a common cap. G1 resolves censored pairs conservatively against the candidate.
6. **Planted leakage.** None found.
   - Solvers receive only `puzzle` (hints = 0). The scramble is the hardness-peak XorShift shuffle plus rotation.
   - `planted` is read only in runner.rs for `equals_planted`.
   - Solutions differ from the planted board on most instances (equals_planted: 3, 3, 11 and 6 out of 90 per file).

## Findings

Format: `file:line | severity | claim | problem | fix`

- `research/experiments/hard-ladder/README.md:23 | should-fix | "0/30 at every c <= 8" | False: the committed hardness-peak sweep solves 12, 11, 14, 27 and 12 out of 30 at c = 1..5. Only c = 6, 7, 8 are 0/30. The conclusion (move to c >= 10) still holds | Write "0/30 at c = 6, 7 and 8 (the hard band); c <= 5 is partly solvable"`
- `research/experiments/hard-ladder/analysis/analyze.py:28 | should-fix | README:168 one-command compare from a candidate directory | load() joins every relative path to the hard-ladder ROOT. From a candidate directory, `results/test/primary/NAME.jsonl` therefore resolves to `hard-ladder/results/...` and fails with FileNotFoundError (reproduced with an external plug-in crate). It only works with an absolute path, or when the candidate writes into hard-ladder/results as PROTOCOL.md:80 implies, so the README and PROTOCOL disagree | Resolve relative paths against the cwd (fall back to ROOT only if the path does not exist), and align README:166-169 with PROTOCOL.md:78-82`
- `research/experiments/hard-ladder/analysis/analyze.py:316 | should-fix | README:32-34 "A candidate can run G1 at rung 6 today" | compare crashes (TypeError: ols returns None) when the candidate file covers a single rung, because G2 fits OLS on one point. This is exactly the rung-6-only use the README recommends | Skip G2 with fewer than 2 rungs and report it as not evaluable; keep the G1 output`
- `research/experiments/hard-ladder/CALIBRATION.md:14-28 | nitpick | "14 candidates" | The table lists 13 rows. scb-border_first_random (rank 14, 9/50, 6x6 median 466,582) is missing, though it is in ranking.json | Add the row`
- `research/experiments/hard-ladder/PREREG_AMENDMENTS.md:8 | should-fix | A1 and A2 "before any test-seed run" | Not verifiable from git: amendments, calibration and test results share commit 85c5bf2. A1 favours a kill and the kill reproduces at the original cap of 1e7, so the outcome is unaffected | In future, commit each amendment in its own commit before the runs it governs`
- `research/experiments/hard-ladder/PROTOCOL.md:23-26 | should-fix | "A candidate that assigns without calling try_assign ... is invalid" | The harness cannot detect this for plug-ins. Only the registry wrapper replays tries, so a plug-in's node count is self-reported. A candidate could under-count, for example by doing assignments inside propagation without calling try_assign | Add an optional audit mode: the plug-in reports each assignment (cell, piece, rotation, depth) to the counter, the harness replays it like CapSink, and the protocol requires an audited run on calibration seeds`
- `research/experiments/hard-ladder/PROTOCOL.md:44-48 | nitpick | G1 evaluated at rungs 6, 8, 10 | At primary rung 6 the baseline needs about 951 nodes, so G1 there measures tiny trees. While rung 8 is censored for the baseline, a candidate can pass G1 at 8 only by solving at least 16/30. That makes the gate depend on cap choice rather than tree size | Fix the colour set first (next round); then gate on the hardest usable rungs`
- `research/experiments/hard-ladder/PREREG.md:69-75 | nitpick | nearest-integer rounding | The rounding direction alternates across sizes (6.38 rounds to 6, 7.56 rounds to 8), so the rungs sit at different distances from the hard band. That makes "monotone in n" partly a colour effect (secondary 7x7 c = 7 is 12x easier than primary 7x7 c = 6) | Next round: choose colours by a measured per-size difficulty target, not by rounding`

Reviewer misfires I checked and rejected:
- Engine-DFS cap overshoot is handled. Stepping is bounded by cap - nodes, and a full board at the cap is allowed one recording step.
- AC-3 propagation does not place pieces without ValueTried.
- G2's censored-baseline floor at C is conservative for the candidate, not lenient.

## Code and reproducibility

- `run.sh check` path: build, `check-instances` and 9 tests pass.
- The justfile recipe `hard-ladder` wraps `run.sh`.
- A full test-grid rerun reproduces every committed row bit for bit in nodes.
- The plug-in path works from an external crate (path dependency on `../hard-ladder/compute`, `main_with`). The instance root comes from CARGO_MANIFEST_DIR, so it does not depend on the cwd. The README compare command itself is broken (finding 2).

## Next-round test

Amendment A3 with a new colour set, preregistered before any run on the test seeds:
- **Colours.** Choose interior colours per size so that the best baseline (scb-gacolor_ac3_random) solves 50 to 90% of calibration instances within 1e6 nodes at each size. Use fresh calibration seeds 2001..2010, a grid of c from criterion+1 to criterion+5, and select the smallest c meeting the target at each size. Hardness-peak's 8x8 data (c = 9: 23/30 at 1.5e7 placements; c = 10: 30/30, median 5.1e5) suggests c around 10 to 11 at 8x8.
- **Kill criterion** (unchanged in form, on test seeds 1..30 at cap 1e6):
  - kill if NOT m6 < m7 < m8 < m9 < m10;
  - kill if the 8x8 or 10x10 solve rate is below 15/30.
- **Rungs.** Run all of 6..10 for both baselines in the same session.
- **Before candidates use it.** Fix the analyze.py path and single-rung bugs, and add the plug-in audit replay.

