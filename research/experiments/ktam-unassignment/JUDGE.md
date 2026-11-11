# JUDGE: A15 non-chronological, bond-weighted unassignment (kTAM / IMS)

Branch `devin/r3-ktam-unassignment`, commit `617c131`, directory
`research/experiments/ktam-unassignment`. Independent judge; no change to the branch.

## Verdict: kill

The test is valid and its binding kill criterion fires. The method clause holds on both rungs that were run. At R6 and R7, V1 (kTAM detachment) and V2 (IMS eviction) are both NO-GAIN (Holm) against B1 on events. The best variant solves at most max(B1, B2, B3) + 3 boards (R6: 27 <= 33; R7: 0 <= 13). The mechanism clause does not fire, because at R6 V2 beats random eviction B3 (27 vs 19 solved, G1 ratio 0.43, p = 0.004). R8 was skipped correctly: its trigger (V1 or V2 GAIN or PASS against B1 at R6 or R7) did not fire.

The kill is far from the margin. At R6 the best variant needs about 100 times B1's events on paired instances. On four extra run seeds this ratio ranges from 54 to 145, and V2 also loses to plain row-major DFS (B2), which has no propagation. Across every committed and judge-run arm, no method run solves a board that B1 fails on, and that includes the exploratory equal-CPU check.

## 1. Preregistration and amendments

- `PREREG.md` was committed alone in `a7062a5` (14:58:43 UTC). The arms, calibration and first results came later, in `774d97d` (15:06:08). The test results came in `d6b6bea` (15:07:55). No file under `results/` predates the prereg.
- The kill criterion in PREREG.md section 8 matches the binding statement word for word in substance:
  - method clause: NO-GAIN against B1 on events on every rung run, AND best-variant solves <= max(B1, B2, B3) + 3;
  - mechanism clause: B3 within 2 solves AND not significantly worse on G1, on every rung run;
  - "GAIN without PASS" is reported as "promising".
  The code in `analysis/analyze.py` (kill_terms, method_killed, mech_killed) implements it as written.
- Event definition (section 4): an event is one placement (one ladder `try_assign` node) or one removal. This makes events equal to 2 * nodes - filled_at_end for every arm. Propagation is counted for no arm, and wall clock is reported. This satisfies the statement.
- `PREREG_AMENDMENTS.md` D1: the R8 instance files were committed after calibration and before any test run. This is immaterial. R8 was never run, and `ktam check-instances` regenerates the files byte for byte (passed in my rerun).
- `PREREG_AMENDMENTS.md` E1: the equal-CPU check at R7 was added after the verdict at the lead's request. It is labelled exploratory and non-binding, it does not change any label, and it points the same way as the binding result. This is not a post-hoc rescue.

## 2. Recomputed numbers

I wrote my own script (`recompute.py`, stdlib only) over the committed rows in `results/test/R{6,7}/*.jsonl`. It derives events independently and asserts the event identity on every method row: nodes + removals = events = 2*nodes - n^2. All 150 method rows passed. It recomputes KM quartiles (15th order statistic), G1 (upper median, 16th order statistic) and the bootstrap (10,000 resamples, `random.Random(20261111)`).

| rung | arm | solved within E /30 | KM median events | solved within 1e6 nodes |
|---|---|---|---|---|
| R6 | B1 | 30 | 1,866 | 30 |
| R6 | B2 | 30 | 83,610 | 30 |
| R6 | B3 | 19 | 565,948 | 24 |
| R6 | V1 | 23 | 481,968 | 27 |
| R6 | V2 | 27 | 221,284 | 30 |
| R7 | B1 | 10 | censored (q25 544,513) | 16 |
| R7 | B2, B3, V1, V2 | 0 | censored | 0 |

| rung | comparison | G1 median ratio | 95% CI median ln ratio | p one-sided |
|---|---|---|---|---|
| R6 | V1 vs B1 | 186.3 | 4.554 to 6.536 | 1.000 |
| R6 | V2 vs B1 | 100.4 | 3.295 to 5.493 | 1.000 |
| R6 | V1 vs B3 | 0.880 | -0.730 to 0.000 | 0.291 |
| R6 | V2 vs B3 | 0.430 | -1.398 to -0.332 | 0.0042 |
| R6 | V2 vs B2 | 2.887 | 0.388 to 1.535 | 0.9986 |
| R7 | V1, V2 vs B1 | 1.000 | 0 to +inf | 1.000 |
| R7 | V1, V2 vs B3 | 1.000 | 0 to 0 | 1.000 |

Every number matches `results/summary.md` and the README.

**Full rerun.** I ran `run.sh all` in a separate worktree at `617c131` (2 min 26 s). It reran the build, 5 tests, clippy, the instance check, calibration (54 configs x 20 runs), the test runs and the analysis. All 64 committed raw jsonl files and `summary.json` reproduced exactly, apart from wall-clock fields. R8 was skipped by the trigger, as committed. B1 and B2 rows equal the committed hard-ladder rows node for node (0 mismatches).

**Judge diagnostics** (exploratory, R6 test seeds, not committed):

- **Run-seed robustness.** I changed `RUN_SEED` to 2, 3, 4 and 5 and reran B3, V1 and V2 with their selected configs.
  - Solves: B3 16 to 19, V1 19 to 22, V2 27 to 30.
  - V2 vs B1 G1 ratio: 77, 145, 101 and 54 (p = 1.0 each).
  - V2 vs B3: ratio 0.23 to 0.50, p from 0.0000 to 0.018.
  - V1 vs B3: ratio 0.68 to 1.0, p from 0.28 to 0.90; V1 never separates from B3.
- **Matched control for V2.** V2 with noise = 1 keeps the same evict-and-place move but picks the candidate at random instead of by minimum blocking cost. It solves 7/30, and V2 vs this control gives ratio 0.32 (p = 0.000). So the blocking-cost choice, not only the forced placement at the dead cell, accounts for V2's edge over B3.
- **Same algorithm, different RNG use.** V1 with beta = 0, R = inf, K = 2 is the same distribution as B3 `b3-rinf-k2`. It solves 14/30, against 19/30 for B3.

**Equal CPU** (committed exploratory rows). At 11.8 s CPU per instance at R7, B1 solves 16, V1 1, V2 1 and B3 0. The one board V1 and V2 solve (n7-c6-s5) is also solved by B1. Per node, B1 uses about 8 times V2's CPU (89k vs 691k nodes per CPU s).

## 3. Rigor check (experiment form)

- **Baseline fairness and strength.**
  - B1 is the ladder's strongest complete DFS, run unchanged through the registry, and it reproduces the committed rows.
  - B1 gets its AC-3/GAColor propagation free under E, while the method arms get their MRV candidate scans free. Both are uncounted, as the statement requires.
  - This bias does not decide the result in either direction. The method also loses to B2 at R6, and B2 has no propagation. At equal CPU at R7, B1 still wins 16 to 1.
  - B3 got fewer tuning configs (9) than V1 (36). This only affects the mechanism clause, which did not fire.
- **Leakage.**
  - The solver receives `inst.puzzle.clone()` only. `inst.planted` is read in `run_method` after `solve` returns, for `equals_planted` and the descriptive removal counts.
  - Candidate lists are ordered by piece index of the scrambled puzzle, and every choice among candidates and tied cells is random.
  - Every solve is checked with `hard_ladder::verify_board`.
  - No leakage path found.
- **Censoring.**
  - Every DFS solve within E is observed (node cap 1e6 >= (E + n^2)/2), and every method solve within E is observed (events stop at about 2e6).
  - G1 handles censoring as in PROTOCOL.md: both censored counts as 0, censored candidate vs solved baseline as +inf, solved candidate vs censored baseline as ln(c/E).
  - KM medians come out as "censored" where they should.
- **Node win hiding a wall-clock loss.** There is no node win. At R6, B1 is also faster in wall clock (median 6 ms vs 88 ms for V2). At R7, B1 is slower per run (6.7 s vs 1.0 s), but the equal-CPU check shows this does not reverse anything.
- **Claims no stronger than data.** The headline and README say "killed on the method clause" and scope the IMS-vs-random finding to 6x6 c=5 and to this family of searches without propagation. My run-seed rerun and matched control support that.
- **Scope.** Claims stay at R6 and R7 and do not extend to 8x8, 10x10 or 16x16. The README states that combinations with propagation or with a chronological core were not tested.

## 4. Findings

| file:line | severity | claim | problem | fix |
|---|---|---|---|---|
| README.md:109 (also PREREG.md:171, results/summary.md "removed = planted") | minor | The fraction of removals that hit a planted placement measures selectivity. | This is nearly forced by the design, as the README partly says. In a censored or long run almost every placement is later removed, so the two fractions agree to 3 decimals. It also ignores the 4-fold board-rotation symmetry. The solved R6 final boards share 0 or 36 cells with the planted board (V2: 27 of 30 share 0 cells, B1: 3/30 equal planted), so most solutions are turned or other copies, and "lower bound" understates how uninformative the column is. | Measure, at each dead end, whether the removed piece agrees with any of the 4 turned planted copies, conditioned on the pieces on the board at that moment. Or drop the column. |
| PREREG.md:125 | minor | B3 separates the mechanism from non-chronological unassignment in general. | B3 is a clean control for V1 (V1 with beta = 0 is the same distribution as B3), but not for V2. V2 also places a piece at the dead cell after evicting, and B3 does not. The judge's matched control (V2 noise = 1: 7/30, V2 vs control ratio 0.32, p = 0.000) shows that V2's edge comes from the blocking-cost choice. So the README claim holds, but the preregistered design did not isolate it. | Declare a matched control per variant, for example a random-candidate IMS move for V2. |
| compute/src/driver.rs:21 | minor | The solve counts per arm are the outcome. | There is a single run seed. The same algorithm under a different RNG usage (V1 beta = 0 vs B3) gives 14/30 vs 19/30. The kill thresholds "+3" and "within 2" are inside this run-seed spread, although this run is not decided at the margin. | For margin-based clauses, run at least 3 run seeds per method arm and report the spread. |
| PREREG.md:145 | info | Each arm was tuned independently. | B3 had a 9-config grid against V1's 36. B3's R = 1 and R = 2 configs all solve 0, so the grid shape probably does not matter here. | Use equal grid sizes when a control decides a clause. |
| PREREG.md:125 | info | B3 is "random eviction / min-conflicts repair". | Only random eviction was implemented. The statement allows either option. | State in PREREG which option was chosen. |
| compute/src/driver.rs:174 | info | Events equal 2*nodes - filled_at_end. | The check is a `debug_assert`, so it is off in release runs. The 5x5 test checks it, and my script checked it on all 150 test rows of the method arms. | Make it a hard assert, or write the check into the analysis. |
| PREREG_AMENDMENTS.md:5 | info | D1: R8 files committed after calibration. | Immaterial: R8 was not run, and the generator check is byte for byte. | None. |
| PREREG_AMENDMENTS.md:14 | info | E1: equal-CPU check. | Added after the verdict but labelled non-binding, and it agrees with the binding result. | None. |

No blocking issues.

## 5. Adversarial review (four lenses)

- **Design validity.**
  - The event metric is the same for every arm: a placement is a ladder node, and a removal is one piece off the board.
  - The budget, cap and censoring are consistent.
  - The ladder baselines reproduce.
  - The one confound (propagation free for B1) is shown not to matter: B2 has no propagation and still beats the method, and the equal-CPU check agrees.
- **Scope.**
  - The kill covers this family: a consistent partial board, MRV with one-step lookahead, no propagation, and these two unassignment rules, on R6 and R7.
  - It does not cover unassignment combined with AC-3, or unassignment layered on a chronological core.
- **Prose.** The README and summary numbers match the raw rows. The removal-selectivity paragraph should also name the rotation symmetry (finding 1).
- **Code and reproducibility.**
  - A full clean rerun reproduces all raw rows bit for bit, apart from wall clock.
  - Tests and clippy pass.
  - The instance check passes.
  - The planted board is read only after `solve` returns.

## 6. Next-round test

The only open signal is V2's significant edge over random eviction inside a family that is about 100 times behind DFS. A test that could still change the program map is IMS-style eviction as the backjump rule of B1 itself.

- **Method.** At an AC-3 wipeout, evict the min-cost blockers of the dead cell and re-propagate, instead of backtracking chronologically. V2 is the incomplete variant. A complete variant keeps a nogood or chronological fallback.
- **Arms.** B1 unchanged, plus a random-eviction twin with the same propagation, as the matched control.
- **Rungs.** R7 and R8 test seeds.
- **Seeds.** 3 run seeds per arm.
- **Effort.** One budget of 1e6 events, with CPU time reported.
- **Kill.** Killed if NO-GAIN against B1 on G1 at both rungs and it solves no more than B1 + 3 on the run-seed median; the mechanism is killed if the random twin is within 2 solves and not significantly worse.
