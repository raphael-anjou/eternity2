# A20 judge: blocker eviction as the backjump rule inside bb-mrv

Independent judgement of branch `night/2026-11-12-t2-blocker-backjump` (commits
`50e400e` to `660cf26`). Track label: method.

## Verdict: kill

The method is killed as preregistered: PROTOCOL rung rule at R6 (G1 against
scb fails, node ratio 4.68, CI [2.40, 6.55]) and K1 (G1 fails at R6 and R7,
R7 run-seed-median solved 4/30 against scb 16/30). The mechanism clause K2
also fires (twin 3/30 against blocker 4/30, paired CI upper end 0). The
verdict is valid; the scope of the kill is narrower than "blocker
backjumping" (see below).

## Preregistration order and the calibration parameter

What could be established:

- Commit order: `50e400e` code, run script and analysis (18:26:06), `418d125`
  PREREG.md (18:55:09), `fffd0c4` calibration rows (18:58:00), `baf2a56` R6/R7
  test rows (19:38:32), `660cf26` run seeds 2 and 3 (20:48:49). The branch
  reflog is linear, no amend or reset. PREREG.md has a single commit.
- File birth times in the worktree: the first test file (`results/test/r6/run.log`)
  was created at 18:58:10, 3 minutes after the PREREG commit; R7 at 18:59:01;
  seeds 2 and 3 from 19:38:25 to 20:15:40 (last write 20:48:11). Every test
  `run.log` header names tabu 16, cap 1e6, guard 900 s and the run seed.
- Grid and selection rule: fixed in code at `50e400e` (18:26:06), before any
  T = 16 or T = 128 calibration row existed (t16 created 18:30:54, t128
  18:39:18). `run.sh calibrate` loops `for t in 0 16 128`, and
  `analyze.py calibrate` selects by the key (-solved, sum ln min(nodes, cap),
  T), the rule PREREG.md states. Only the T = 0 calibration run had started
  (18:23:55) when that commit was made, and T = 0 is the arm that loses.
- Mechanical selection: from the committed calibration rows I get bj-blocker
  0/20 at T = 0, 11/20 at T = 16 (sum ln 213.86), 6/20 at T = 128, so T = 16,
  the committed `selected_tabu.txt` value. Every sidecar row of both bj arms at
  every test rung and run seed records `tabu: 16`, so candidate and control
  used the same value.

What could not be established:

- There is no `results/run-log.jsonl`. `prereg_guard.py` was committed at
  20:41:47 on the lead branch (`e254e3f`), is not an ancestor of this branch,
  and postdates every test run but the tail of R7 seed 3. Every test result
  therefore lacks a guard log line. This is structural, not misconduct, but
  the order rests on filesystem birth times, which a local actor could alter;
  result rows carry no timestamps of their own.
- PREREG.md was first created at 18:25:46 and last written at 18:55:08, after
  calibration finished, so its prose (including the decision rules) was
  finished with calibration results in view. The calibration seeds are
  disjoint from the test seeds and the kill rules match AGENDA A20's kill
  clause, so this does not bear on the test verdict.
- The release binary used for the runs was built at 18:23:22, before the
  source commit. My rebuild from the committed source reproduces the test
  rows (below), so the committed source is what produced them.

## Recomputed numbers (own script, committed raw rows)

All 25 result files: 30 rows per test file and 20 per calibration file, one
row per instance, cap 1,000,000 everywhere, `nodes == nodes_counted`, no
`error` row, censored rows exactly at the cap, guard never fired.

| rung | arm | solved/30 (seed 1) | seeds 2, 3 | KM median nodes |
|---|---|---|---|---|
| R6 | scb | 30 | | 951 |
| R6 | bb-mrv | 30 | | 460 |
| R6 | bj-blocker | 30 | 30, 29 | 4,334 |
| R6 | bj-random | 30 | 30, 29 | 3,868 |
| R7 | scb | 16 | | 921,773 |
| R7 | bb-mrv | 23 | | 588,305 |
| R7 | bj-blocker | 5 | 2, 4 | censored |
| R7 | bj-random | 2 | 4, 3 | censored |

G1 (upper median, 10,000 paired bootstrap resamples, seed 20261111):
R6 blocker/scb 4.683 [2.403, 6.547]; blocker/bb-mrv 9.177 [5.394, 15.204];
blocker/twin 0.947 [0.574, 3.670]; R7 blocker/scb 1.000 [1.000, inf] (1
blocker-only solve, 12 scb-only, 13 both censored); blocker/twin 1.000
[1.000, 1.000]. Every figure in README.md and `results/tables.md` matches.
Context, not preregistered as binding here: bb-mrv/scb at R6 is 0.468
[0.318, 0.895], a G1 PASS.

Baselines: scb rows equal `hard-ladder/results/test/primary/scb-gacolor_ac3_random.jsonl`
on all 60 instances; bb-mrv rows equal `mrv-baseline` A22 `q1-r6r7` on 20
overlapping instances and the A16 check file on 40.

Cycling (preregistered definition), recomputed: bj-blocker 5, 9, 6 of 30 at R7
(seeds 1, 2, 3), 1 at R6 seed 3; bj-random 0 everywhere.

## Rerun

Rebuilt from the committed source at `660cf26` (`cargo build --release -j 2`,
separate target directory) and rerun with `--threads 2`, tabu 16, run seed 1:
all 30 R6 instances for all four arms, and five R7 instances for all four
arms, drawn before the rerun by `random.Random(20261112).sample(range(1, 31), 5)`
= seeds 7, 13, 17, 26, 27. Compared fields: status and nodes for every arm;
for the bj sidecars also node-sequence hash, dead ends, fallbacks, evictions,
revisits and tabu clears. Result: **210 of 210 rows identical** (180 at R6,
30 at R7). Limit: none of the five R7 instances is a bj solve (all ten bj runs
are censored on both sides), so the rerun confirms determinism and the
censoring of those runs, not any R7 bj solve.

## Fairness of the comparison with complete solvers

- Same counter: all four arms go through hard-ladder `run_one` and
  `NodeCounter::try_assign`; the bj arms call it once per (cell, option)
  tried, before propagation, like bb-mrv. Same cap (1e6) and guard (900 s).
- Repairs: a repair makes no new assignment. It removes decisions and
  re-propagates the kept ones from the root; that re-propagation is not
  charged as nodes. Tabu-skipped options cost no node either. Both choices
  favour the candidate, which still loses by 4.7x (scb) and 9.2x (bb-mrv) on
  nodes at R6, and by 32x and 25x in wall clock. There is no node win hiding a
  wall-clock loss; it is a loss on both.
- Incompleteness: no run returned `Exhausted`, so no incomplete-search
  artefact was scored. Censored runs are scored as losses as PROTOCOL.md
  requires.

## Is the kill an artefact of the anti-cycling device or the fallback?

- Anti-cycling: T = 0 solves nothing (pure cycling), so the tabu matters. But
  at R6 seed 1 neither bj arm cycled on any run and both still lose to scb and
  bb-mrv by a factor of 5 to 9 in nodes. At R7, even if the 5 cycled blocker
  runs were all counted as solved, 10/30 is below scb's 16. The method kill
  does not depend on the cycling runs. The grid has three values only; a
  tenure between 16 and 128 was not tried (argued: the calibration trend
  16 > 128 > 0 at R7 is 1, 0, 0 of 10, which leaves no room for a 12-solve gap
  to close).
- The tabu list is cleared whenever a dead end occurs with no decision left.
  This happened 4,169 (blocker) and 10,922 (twin) times over 30 R7 runs,
  which makes both arms partly a restart method. Not reported in README.md.
- Fallback: in 24 to 31% of blocker dead ends the failed cell has no direct
  blocker (it was emptied by propagation from further away) and the arm evicts
  the latest decision instead; the twin falls back on 3.4%. So "the mechanism"
  as run is about 70% blocker eviction and 30% latest-decision eviction. This
  dilutes K2 but cannot explain the method loss, since the random twin loses
  as badly.
- K2 power: at R7 both bj arms are nearly all censored (counts 2 to 5 of 30),
  so the paired median is a forced tie. K2 fires as preregistered, but the
  evidence is "no detectable difference between two failing arms", not
  "blocker choice carries no signal". At R6, where both solve 30/30, the
  paired ratio is 0.947 with CI [0.574, 3.670]: no difference either way.

## What the kill covers and what it does not

Covers: incomplete repair search inside bb-mrv's propagation, with an
unordered decision set, no stored no-goods, direct-blocker eviction (edge
mismatch or piece clash with the failed cell), latest-decision fallback,
re-propagation from the root, tabu tenure 16 nodes, at R6 and R7, cap 1e6.

Does not cover:

1. Conflict-directed backjumping proper (Prosser 1993, cited by AGENDA A20):
   a complete chronological stack that jumps to the deepest culprit and keeps
   everything below it. The arm run here is a repair search, not CBJ.
2. Conflict sets built from propagation explanations (which decisions removed
   each value of the failed cell). Here B(o) sees only orthogonal neighbours
   and the piece holder, which is why about 30% of dead ends fall back.
3. Dynamic backtracking or any variant with no-good storage, which removes
   the need for a tabu device.
4. IMS as Rau defines it, where the displaced operation is scheduled into the
   freed slot: here the chosen option o is not committed at x after eviction;
   MRV picks the next cell.
5. Tabu tenures other than 0, 16 and 128, and R8 (not run, as preregistered).

## Findings

- No invalidity found.
- Missing guard log (structural, see above). Fix for future runs: merge
  `prereg_guard.py` before the first test seed; for this branch no fix is
  possible after the fact.
- README.md omits the tabu-clear counts; minor.

## Method

Own script over committed rows (solved counts, KM medians, G1 with bootstrap,
cycling, fallback fraction, baseline reproduction), calibration selection
recomputed, rebuild from the committed source with `cargo build -j 2` into a
separate target directory and rerun of a seeded subset with `--threads 2`.
Rigor-check and adversarial-review checklists applied inline by one reader,
without subagents.
