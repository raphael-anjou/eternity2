# prefix-statistics

Arc 2026-11-13, track D4, label **measurement**. Question RQ3 / obstruction
O2 of `research/program/night/2026-11-13/THESIS_PLAN.md`: does any cheap
statistic of a partial row-scan board tilt toward the planted solution's
prefix? Generated 16x16 boards with the official colour census and a planted
solution, five clues at the official cells. No board is produced; nothing here
is Eternity II progress.

Read in this order: `PREREG.md` (frozen first, commit `e9ecd977`),
`PREREG_AMENDMENTS.md` (A1 to A6, each with time and reason), this file, then
`JUDGE.md` (verdict KEEP as a measurement, and the rulings on the decoy cell
and the value rule that the readings below depend on).
Every number below is printed by a committed script from committed raw rows;
the file is named next to it.

## Result in one paragraph

On 48 planted 16x16 boards (the PREREG stop rule ended the test at 48 of 96,
A6), the census statistics S6 (configuration-model first moment of the
completions, 33 ns per evaluation) and S9 (colour entropy of the remaining
tiles, -0.93 to -0.95 correlated with S6 on the true prefix, so the same
finding) put the planted prefix about 2 sd from the sampled prefixes at the
widest levels: S6 node delta **+1.96 [1.53, 2.42] at depth 144 and +2.15
[1.75, 2.56] at 160** (Bonferroni 0.99833 bootstrap intervals). The shift
survives weighting the reference by subtree size (Reading 2) and restricting
it to prefixes alive one row ahead (Reading 3), so it passes
Readings 1, 2 and 3 at 144 and 160. **This is what the random model itself
predicts, not a contradiction of it**: the planted prefix is a uniform
prefix reweighted by its number of completions, and S6 is the first-moment
log of that number, so a shift of about one sd of S6 follows from that
model (design review, section 0). The
reviewer's prediction, written before the test (A4), was a shift / sd^2 of
about 1 (delta about equal to the sd of S6, worth about 2 bits at one
direction; the review's own calibration table already showed 1.22 to 1.56);
measured on the test boards 1.49 to 1.78 at depths 96 to 176, so the tilt is
that predicted size-bias plus about 50 to 80%. What it is worth: one prune at
depth 144 on one scan direction gains G = **1.02 [0.72, 1.27] bits** (A4
value reading), two directions -0.04, four directions -1.88; the larger
values the value rule prints at depths 96 and 128 are not gains: they rest on
work weights with a median effective sample of 3 and 13, below the floor of 30
that A4 set for work-weighted readings. Against the gap of about 60 bits the review cites,
this is a constant factor. The painter control on 6x6 boards fails its registered rule (in the
direction "planted prefixes are less tilted than other solutions"), so no
transfer beyond planted boards is claimed. One negative control (the decoy)
fails its registered rule in one degenerate cell, which by the letter of
PREREG section 8 suspends the readings. The judge ruled that failure
uninformative (a bootstrap tie artefact, post hoc, recorded as such in
`JUDGE.md` section 4a) and let the measurement stand; the readings below are
quoted under that ruling.

## Readings against the fixed lines

Ruling on the decoy cell (`JUDGE.md` section 4a): by the registered rule the
readings in this section are suspended; the judge rules the one failing cell
(S10 at 160) uninformative and lets them stand. The ruling is post hoc.

Source: `results/test-reading.txt` and `results/summary.json`, both printed by
`scripts/analyze.py results/test --real results/real/real.jsonl`.

**Validity (A4 change 1).** Valid rows (population >= 1,000 and family
effective size >= 200): 182 of 192 at 144 (5.2% invalid), 189 of 192 at 160
(1.6%), 157 of 192 at 176 (18.2%). Depth 176 is therefore demoted to
descriptive; the decision depths are 144 and 160.

**Reading 1, the lead's line** (|mean node delta| >= 1.4 and the 0.99833
interval excluding 0.7; valid rows, 48 boards):

| statistic | class | 144 | 160 | signal |
|---|---|---|---|---|
| S1 next fits | search-equivalent | +0.88 [0.7, 1.0] | +1.00 [0.8, 1.2] | no |
| S2 frontier fits | search-equivalent | +0.83 [0.6, 1.1] | +0.86 [0.6, 1.1] | no |
| S3 Hall surplus | search-equivalent | +0.86 [0.6, 1.1] | +1.02 [0.8, 1.3] | no |
| S4 colour balance | census | +1.32 [0.94, 1.70] | +1.46 [1.12, 1.81] | 160 |
| S5 goodness | census | -0.21 [-0.38, -0.04] | -0.19 [-0.42, 0.03] | no |
| **S6 first moment** | census | **+1.96 [1.53, 2.42]** | **+2.15 [1.75, 2.56]** | 144, 160 |
| S7 AC domains | search-equivalent | +4.02 [3.9, 4.2] | +10.00 (clip) | 144, 160 |
| S8 block fills | search-equivalent | +1.71 [1.6, 1.9] | +2.37 [2.1, 2.6] | 144, 160 |
| S9 colour entropy | census | -1.78 [-2.21, -1.36] | -1.98 [-2.39, -1.58] | 144, 160 |
| S10 row lookahead | search-equivalent | +5.40 [5.1, 5.8] | +9.63 [9.4, 9.8] | 144, 160 |

**Kill line** ("a usable tilt exists" is killed if no statistic is a signal):
**not met**. Six statistics are signals at 144 or 160.

**Reading 2, work-weighted** (work ESS >= 30; median work ESS 59 at 144, 187
at 160, so Reading 2 is informative at 144 only and cannot differ from
Reading 1 at 160 and 176): S4 +1.48 [1.10, 1.88] and +1.54 [1.19, 1.90], S6
+1.67 [1.23, 2.12] and +1.96 [1.57, 2.36], S9 -1.72 [-2.17, -1.30] and -1.93
[-2.34, -1.54] at 144 and 160 pass; S7 and S10 pass at 160 only; S8 falls to
+0.10 at 144 (the lookahead's separation is paid by its own subtree).

**Reading 3, survivors one row ahead** (128, 144, 160; at least 200
survivors in all 192 rows): S6 +1.68 [1.20, 2.16], +1.74 [1.32, 2.19], +1.76
[1.38, 2.14]; S9 -1.69, -1.79, -1.93; S4 +1.43, +1.54, +1.73: all pass. The
search-equivalent statistics collapse among survivors (S1, S2, S3, S8 between
-0.01 and +0.24; S10 about +1.1), as expected of statistics that read only
what search learns within a row. Two rows ahead is measurable at 128 only
(47.7% of one-row survivors pass; at 144 and 160 fewer than 200 do): S6
+1.56 [1.1, 2.0].

**Passes Readings 1, 2 and 3 at the same depth** (not a usable gain, see the
value reading): **S6 and S9
at 144 and 160** (one finding, S6 primary), S4 at 160. S5 (Selby-style
goodness) is not a signal anywhere (|delta| <= 0.5).

**Value reading for S6 (A4 change 5)**, capped gain of one prune, bootstrap
over boards, 95%:

| depth | median work ESS | K = 1, p >= 0.5 | K = 1, p >= 0.8 | K = 2, p >= 0.5 | K = 4, p >= 0.5 | cap (K = 1) |
|---|---|---|---|---|---|---|
| 96 | 3 | +3.43 [0.99, 10.00] | +0.47 [-0.09, 1.30] | +9.73 [4.33, 11.34] | +10.06 [9.62, 10.35] | 17.15 |
| 128 | 13 | +2.34 [1.19, 4.97] | +0.88 [0.48, 1.59] | +3.49 [1.20, 4.03] | +0.17 [0.05, 0.38] | 6.54 |
| 144 | 59 | **+1.02 [0.72, 1.27]** | +0.71 [0.30, 1.31] | -0.04 [-0.19, 0.17] | -1.88 [-1.97, -1.72] | 2.35 |

By the rule as registered ("usable" if the lower end of G exceeds 1 bit at
some depth) the answer is "yes", through 96 (K = 2, 4) and 128 (K = 1, 2). The
judge rejected that "yes" (`JUDGE.md` section 4b): the registered answer is
**not usable**, and the depth 96 and 128 cells are not gains and must not be
quoted as such. Those cells use a work-weighted reference with a median effective sample of
3 and 13 particles: the kept fraction f (0.0001 to 0.09) is estimated from a
handful of heavy particles, which is exactly what the A4 floor of 30 exists
to exclude in Reading 2. At the only value depth that clears that floor,
144, one direction gives 1.02 [0.72, 1.27] bits and more directions lose
bits. The reviewer's prediction (review table, section 3) was +1.9 and +2.1
bits at one direction at 96 and 128, +0.7 at 144, and losses for four
directions at 128 and 144; the one well-estimated cell (144) measures 1.02. **The
value result is "not usable"; the registered rule's "yes" is an artefact of
degenerate weights (judge ruling, `JUDGE.md` section 4b).**

**Per-board distribution (A4 change 6)**, S6 node delta: median +1.78 [q1
1.35, q3 2.49] at 144, +2.20 [1.46, 2.75] at 160; share of boards below 0.7:
0.10 and 0.04; below 0: 0.00 and 0.00. The real puzzle is one board; on 48
generated boards none had the tilt reversed.

**Direction dependence (A4 change 7)**, S6 node delta: opposite directions
r = +0.12 (96), +0.88 (128), +0.66 (144), +0.23 (160), +0.07 (176); adjacent
directions +0.06, +0.14, +0.10, +0.01, +0.01. Opposite scans at 128 cover
complementary halves of the board, so their census statistics are nearly the
same number; adjacent scans are close to independent.

## Controls

| control | rule | result | verdict |
|---|---|---|---|
| positive: S8, S10 at 176 (all live rows) | mean >= 1.4, interval above 0.7 | S8 +3.34 [3.03, 3.68]; S10 +10.00 (clip) | pass |
| negative 1: H0 hash, peak | interval contains 0 | -0.13 [-0.35, 0.11], +0.05 [-0.16, 0.27], +0.13 [-0.09, 0.35] | pass |
| negative 2: decoy mid-rank (A1), 10 statistics x peak | every interval contains 0 | 29 of 30 cells pass; S10 at 160: -0.004, a degenerate interval | **fail** (judge: uninformative, `JUDGE.md` 4a) |
| painter (A4 change 8), 6x6, 40 boards, 180 solutions median | planted mean inside the 95% interval of the other solutions' mean | fails in 10 of 18 (depth, statistic) cells | **fail** |
| null in reverse: official pieces vs generated boards | flag |z| > 3 | 1 flagged cell of about 320 (S1, dir 0, depth 96, z = -3.2); all others |z| <= 2.3 | official pieces distributed like the generated boards |

The decoy failure: at 160 almost every sampled prefix has S10 = 0, so the
decoy's mid-rank is nearly the same number in every row and the bootstrap
interval collapses to a point at -0.004 that excludes 0 by a tie artefact.
A1 adopted the rank form for exactly this shape and the pilot showed the
same cell; the registered rule nevertheless says the readings are suspended; the judge
ruled the failure uninformative and let them stand (`JUDGE.md` section 4a).

The painter failure (`results/painter/painter-reading.txt`): the registered
rule compares the planted mean with the narrow interval of a mean over about
180 other solutions per board and ignores the planted mean's own noise. The
paired difference (planted minus others) includes 0 in 15 of 18 cells; where
it does not (S4 at depths 9 and 23, S9 at 23) the planted prefix is **less**
tilted than the other solutions' prefixes, and for S6 the planted value is
below the others at every depth from 14 on (-0.09 to -0.31). So there is no
sign that planting creates the tilt; the rule still fails and no transfer
claim is made.

## Exact pruning and the arc-consistency observation

Census-dead sampled prefixes (some colour with more frontier demand than
remaining supply; `results/test-reading.txt`): none at depths up to 144,
5.2e-6 of the sample at 160 (1 row of 192), 3.2e-4 at 176 (48 rows). The
true prefix was never census-dead (the run asserts it).

Unplanned observation (`results/ac-observation.txt`, `scripts/ac_observation.py`):
arc consistency alone (S7, no search, tile uniqueness ignored) refutes **all
1,000 sampled prefixes in 189 of 192 rows at depth 160 and in 192 of 192 at
176**, and never refutes the true prefix (0 of 1,416 live rows). Up to depth 144
at least 2.2% (144) to 23% (64) of sampled prefixes stay consistent (a lower
bound from the true prefix's rank). S7 costs 0.84 ms at 144 and 0.49 ms at
160 per evaluation, about 2^13 plain nodes. Whether a propagating row scan
pays for this is a separate measurement (propagating-tree size by the same
estimator times ns per node), proposed as its own track.

## What is and is not shown

- Shown, on generated boards with the official colour census and a planted
  solution: a census statistic costing 33 ns tilts toward the planted prefix
  by about 2 sd at the widest levels, beyond what one row of lookahead or
  subtree size explains, consistently across boards (no board below 0).
- Shown: the size of the tilt is the size-bias the first-moment model
  predicts, times 1.5 to 1.8 (`results/size-bias.txt`).
- Not shown: that it transfers to the official puzzle. The official pieces'
  sampled prefixes look like the generated boards' (null in reverse), but
  the true prefix of the official puzzle is unknown, and a designer who
  balanced colours by region would shrink or reverse the tilt; the painter
  control fails its rule.
- Not shown: a gain worth having. The one well-estimated value cell is about
  1 bit for one direction at depth 144; directions are correlated (opposite
  pairs) and cost more than they return. Pruning at several depths was not
  measured.
- Not measured: depths 192 and later (the sampler dies near 190).

## Compute and wall clock

Guard log (`results/run-log.jsonl`, sum of ended runs): 4.23 core-hours
(test 3.49, calibration 0.25, gates 0.26, painter 0.10, official pieces
0.09, identity check 0.03). Unguarded calibration pilot and N probes, plus the
two boards stopped by A6 and the A5 stray runs: about 0.6 more. Total about
4.8 core-hours on this Mac, at most 2 processes at a time, `nice -n 10`.
First guarded run 09:20, last 12:07 local, 2026-11-13; test stage 10:13 to
12:03.


## Method in one screen

- Sampler: Track 10's population SMC (`real-puzzle-profile`, commit
  `dc71ad56`), imported unchanged except for additions marked in
  `engine/src/tree.rs`. Smoke check: identical totals and extinction depths to
  Track 10's committed `five-np1e5.jsonl` (seeds 201, 202). Test runs use
  N = 5e5 (A4), one repetition per (board, direction), m = 1,000 sampled
  prefixes per target depth.
- Boards: `generate_solved_framed_bc(16, 22, seed, true, 5)` (5 frame, 17
  interior colours, the official census), ids and rotations scrambled; test
  seeds 1113001 to 1113096; four row-scan directions each.
- Statistics (PREREG section 4): S1 next fits, S2 frontier fits, S3 Hall
  surplus, S4 colour balance, S5 Selby-style goodness of used tiles, S6
  configuration-model first moment, S7 arc-consistency domains, S8 fills of the
  next 4 cells, S9 colour entropy of the remaining tiles, S10 descendants one
  row ahead; H0 hash control. Census statistics (S4, S5, S6, S9) read only the
  multiset of used tiles and the frontier colours; the others read
  constraints within a bounded window or by propagation.
- Delta = (statistic on the true prefix - reference mean) / reference sd,
  clipped to [-10, 10], averaged over the four directions per board, mean over
  boards with a percentile bootstrap (100,000 resamples). Reference: the
  sampled prefixes, uniform (node delta); weighted by their estimated subtree
  size (work delta); restricted to prefixes alive one row ahead (Reading 3).
  S4, S6, S9 references exclude census-dead prefixes (A4).
- Decision family: 10 statistics x depths 144, 160, 176, Bonferroni level
  0.99833. Rows with population < 1,000 or family effective size < 200 are
  invalid; a depth with more than 10% invalid rows is descriptive only (A4).

## Reproduce

```
just experiments prefix-statistics build
just experiments prefix-statistics gate-exact   # GATE_DIR=gate-a2 for the A2 rerun
just experiments prefix-statistics gate-rep
just experiments prefix-statistics a4check      # byte identity of board 7007
just experiments prefix-statistics painter
just experiments prefix-statistics test         # 96 boards, about 2.5 min each per core
just experiments prefix-statistics real
just experiments prefix-statistics analyze      # uv run --with numpy
python3 -I research/experiments/prefix-statistics/scripts/gates.py research/experiments/prefix-statistics/results/gate-a2
python3 -I research/experiments/prefix-statistics/scripts/ac_observation.py research/experiments/prefix-statistics/results/test
```

Rows are deterministic given the seeds and the binary, except the timing
fields.

## Layout

| Path | What |
|---|---|
| `engine/` | std-only Rust crate plus the repo engine (generator), binary `psx` (`smc`, `measure`, `real`, `gate`, `painter`) |
| `run.sh` | every stage, each command through `../common/prereg_guard.py` |
| `scripts/analyze.py` | every reading of PREREG and A1, A4; writes `results/summary.json` |
| `scripts/gates.py`, `identity.py`, `painter.py`, `ac_observation.py` | gate, byte-identity, painter-control and arc-consistency readings |
| `results/run-log.jsonl` | the guard's append-only log |
| `results/test/` | one file per test board; `results/test-reading.txt` is the analysis output |
| `results/pilot/`, `pilot-rep/`, `calib/`, `gate/`, `gate-a2/`, `a4check/`, `painter/`, `real/` | calibration, gates, controls, official pieces |
| `results/stray/` | outputs of the two launch faults (A3, A5), kept unedited, not read |
