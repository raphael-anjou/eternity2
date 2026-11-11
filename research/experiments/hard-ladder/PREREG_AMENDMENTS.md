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

## A3 (2026-11-11, before any shoulder-track run): two tracks, critical and shoulder

What: the ladder gets two tracks.

- Critical track: the preregistered primary colour set (Owen nearest integer,
  c = 5, 6, 8, 9, 10 for n = 6..10), unchanged, with its committed instances,
  calibration and baseline rows. The 6x6 and 7x7 baseline rows are committed;
  8x8 and above stays censored for the baselines and is reported as such. The
  kill-criterion evaluation in `README.md` above A3 is unchanged and stays the
  verdict for this track.
- Shoulder track (new): interior colour count c_shoulder(n) = c_primary(n) +
  D, with ONE integer offset D used at every size n = 6..10. A single offset
  keeps growth from 6x6 to 10x10 a function of size, not of a per-size
  difficulty choice. Frame colours stay 5.

Why: decision of the project lead and the user after the critical track's
condition 2 fired (best baseline 0/30 at 8x8 within 1e6 nodes; the committed
hardness-peak sweep solves 0/30 at 8x8 for every c <= 8 even at 1.5e7
placements, 23/30 at c = 9, 30/30 at c = 10). The critical track sits at or
below the generator's hardness peak, where no baseline curve beyond 7x7 can be
measured within a one-minute cap; the shoulder track is just above the peak,
where it can. Keeping both separates "hardest instances" from "instances on
which growth is measurable".

Rule for D (decided on calibration seeds only, before any shoulder test seed
is generated or run): D = the smallest integer >= 1 such that the best
calibrated baseline of the critical track (`scb-gacolor_ac3_random`, run seed
1, `CALIBRATION.md`) solves at least 5 of the 10 8x8 shoulder calibration
instances (seeds 1001..1010, c = 8 + D) within the calibration cap of
2,000,000 nodes. D is tried in the order 1, 2, 3, 4 and the search stops at
the first D that qualifies. If none of D = 1..4 qualifies, the shoulder track
is unusable as specified and that is reported; D is not raised further in this
session. Raw rows: `results/calibration/shoulder-d/d<D>.jsonl`; the chosen D
is written to `results/calibration/shoulder-d/selection.json` and fixed in the
code (`SHOULDER_D`) only after that run.

Instances: `instances/shoulder-test.jsonl` (seeds 1..30 per size) and
`instances/shoulder-calibration.jsonl` (seeds 1001..1010 per size), n = 6..10,
same generator call and scramble as the critical track:
`generate_solved_framed_bc(n, c_shoulder + 5, seed, true, 5)` then
`XorShift(seed ^ 0xA5A5A5A5)` shuffle and per-piece rotation. `hard_ladder
check-instances` regenerates and compares them with the other files.

Caps: test 1,000,000 nodes (as A1); calibration 2,000,000 nodes. Not raised.

Baselines on the shoulder test seeds, all five sizes: the best baseline and
`hp-dfs`. Baseline calibration is re-run on the shoulder calibration seeds
(all 14 PREREG candidates, all five sizes, cap 2e6, same selection rule as
PREREG), because the critical calibration took 1220 s, which fits the budget.
If that re-run selects a solver other than `scb-gacolor_ac3_random`, the
shoulder best baseline is the re-run's winner and the usability criterion is
evaluated on it; `scb-gacolor_ac3_random` is then also run on the shoulder
test seeds and reported. D is not revisited in either case. The README states
whether the choice changed.

Shoulder usability criterion (self-assessment; a separate judge decides). Let
m_n be the best baseline's Kaplan-Meier median nodes over the 30 shoulder test
instances at size n (15th order statistic, censored = +infinity). The
shoulder track is usable iff both hold:

1. Growth: the sequence m6, m7, m8, m9, m10 increases with size where finite:
   every pair of consecutive finite values is strictly increasing, and no
   finite value follows a censored one (a censored rung may only be followed
   by censored rungs). Censored rungs at 9x9 and 10x10 are allowed and are
   reported, not hidden. Test: deterministic comparison of point estimates.
2. Solve rate: the best baseline solves at least 15 of the 30 8x8 shoulder
   test instances within 1,000,000 nodes.

Reported, never gating: per (solver, size) solved/30 with Wilson 95%
interval, KM median and quartiles, median wall ms; fitted growth per cell
(OLS of log10 m_n on n^2 over rungs with finite m_n) with a 95% percentile
bootstrap CI over instances (10,000 resamples within each rung, seed
20261111, CI over the resamples in which every rung used in the point
estimate stays finite, that count reported) when at least 3 rungs are finite;
the fraction of resamples in which criterion 1 holds.

Protocol for candidates (added to `PROTOCOL.md`): G1 at rungs 6 and 7 on the
critical track; G1 at every shoulder rung 6..10 in order, dropping at the
first failed rung, and G2 over shoulder rungs 6..10.

Compute budget: about one hour of wall clock on 8 vCPU (D search at most 4 x
10 runs, shoulder calibration about 20 min, shoulder test grid about 15 min),
no AWS or paid services.

Effect on the critical track: none. Its files, instances and rows are
unchanged, `check-instances` still verifies them byte for byte.
