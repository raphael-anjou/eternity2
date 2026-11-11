# PREREG amendments

The original `PREREG.md` (commit 8b316e4) is kept unchanged. Each amendment
states what changed, why, and when. The kill criterion's statistic, thresholds
and test are unchanged; only the cap value at which they are evaluated and the
rungs measured in this session change.

## A1 (2026-11-11, before any test-seed run): test cap 10,000,000 -> 1,000,000 nodes

What: `TEST_CAP` = 1,000,000 nodes per instance for every solver and size on
test seeds (was 10,000,000). The kill criterion is evaluated at C = 1,000,000.
Calibration (already run at its preregistered cap of 2,000,000 on calibration
seeds 1001..1010) and the selection rule are unchanged.

Why: instruction from the program lead, received after calibration and before
any test-seed run: each baseline run should take about a minute or less on one
core, censoring at 10x10 is accepted, and the baseline grid must finish well
under an hour. Calibration measured the strongest candidate (gacolor AC-3
family) at roughly 15,000 to 35,000 nodes/s per run on 9x9 and 10x10 with 8 runs
in parallel, so 1e7 nodes would take 5 to 10 minutes per run.

Effect, stated before the test runs: a smaller cap can only lower solve rates
and turn finite medians into "> C"; it makes the ladder easier to kill, not
harder.

## A2 (2026-11-11, before any test-seed run): rungs 9 and 10 deferred to candidate sessions

What: this session measures and commits baseline test curves at n = 6, 7, 8
only (primary and secondary colour sets, best baseline and hp-dfs). Rungs 9 and
10 are run by each candidate session through the same harness (same committed
instances, same `TEST_CAP`, same `NodeCounter`) for both the candidate and the
baseline, with `hard_ladder run --solver <baseline> --sizes 9,10`, when the
candidate reaches that rung. Those rows are deterministic in nodes, so every
session obtains the same baseline values.

Why: instruction from the program lead to unblock three waiting candidate
sessions.

Effect on the kill criterion: condition 1 (strict increase of m_n over
6..10) can only be evaluated here on 6 < 7 < 8; the 8 < 9 < 10 part is open
until the first candidate session commits the 9 and 10 baseline rows.
Condition 2 (8x8 solve rate) is fully evaluated here.
