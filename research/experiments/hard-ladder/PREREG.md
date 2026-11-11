# Hard-instance ladder: preregistration

Frozen before any ladder instance was generated or any solver was run on one.
Only `hardness_peak --criterion <n>` (a closed-form calculation, no search) was
evaluated to fill in the colour table below. Any later forced change goes in
`PREREG_AMENDMENTS.md` (what, why, when); this file is not edited after the
commit that introduces it, except that calibration output is written to the
separate file `CALIBRATION.md`, which this document defines in advance.

## Purpose

Shared infrastructure for round 2: a fixed set of hard planted instances at
n = 6, 7, 8, 9, 10 on which new exact-search methods are compared against the
repo's best complete DFS by search effort (tree size), not wall clock.

## Hypothesis

At the Owen one-expected-solution interior colour counts (primary rounding
below) with 5 frame colours, the best complete DFS baseline selected by the
calibration rule below has a median nodes-to-first-solution that increases
strictly with board size from 6x6 to 10x10, and solves at least 50% of the 30
test instances at 8x8 within the node cap.

## Kill criterion (ladder usability)

The ladder is unusable (killed) if, on the primary colour set and the 30 test
seeds per size, with the per-instance node cap C = 10,000,000 nodes:

1. Monotonicity fails: let m_n be the median over the 30 test instances at size
   n of the best baseline's nodes-to-first-solution, where an unsolved
   (right-censored at C) instance counts as +infinity. Because every
   observation shares the same censoring point C, this is exactly the
   Kaplan-Meier median; it is finite when at least 15 of 30 are solved and
   "> C" otherwise. Kill if NOT (m6 < m7 < m8 < m9 < m10), where a finite value
   is less than "> C" and "> C" is not less than "> C" (two consecutive
   censored medians cannot show an increase, so they fail). Ties fail.
   Statistic: the five point estimates m_n. Threshold: strict increase at each
   of the four consecutive pairs. Test: deterministic comparison of point
   estimates on the fixed sample (no significance test gates the kill).
2. Solve rate fails: the best baseline solves fewer than 15 of the 30 test
   instances at 8x8 within C (solve fraction < 0.5).

Either condition kills. If killed, the cap or the colours must change before
candidates use the ladder; the README states which condition fired and which
fix applies. The preregistered fallback colour set is the secondary set below
(evaluated with the same rule and the same cap); a changed cap would be an
amendment.

Supporting statistics (reported, never gate the kill): the fraction of 10,000
bootstrap resamples (instances resampled with replacement within each size,
seed 20261111) in which the strict-increase condition holds; the OLS slope of
log10(m_n) against cell count n^2 over sizes with finite m_n with its 95%
percentile bootstrap CI.

## Sizes, colour counts, generator calls

Sizes n = 6, 7, 8, 9, 10 only. No 16x16 search of any kind.

Owen criterion from `research/topics/hardness-peak` (`hardness_peak
--criterion <n>`, I = (P! * 4^P)^(1/(2P)), P = (n-2)^2 interior pieces):

| n | P | criterion I | primary c (nearest) | secondary c (other neighbour) |
|---|---|---|---|---|
| 6 | 16 | 5.2155 | 5 | 6 |
| 7 | 25 | 6.3803 | 6 | 7 |
| 8 | 36 | 7.5579 | 8 | 7 |
| 9 | 49 | 8.7435 | 9 | 8 |
| 10 | 64 | 9.9346 | 10 | 9 |

Rounding rule: primary = nearest integer to I (round half up; no value is
within 0.05 of a half). Secondary = the other adjacent integer (floor(I) if the
primary is ceil(I), else ceil(I)). Both are preregistered. The kill criterion
is evaluated on the primary set; the secondary set is the fallback and is also
measured and reported.

Frame colours: 5 at every size (E2 uses 5 frame colours on its 60 frame seams,
12 seams each; the hardness-peak experiment also fixes 5). Total palette
passed to the generator is c + 5.

Generator call per instance (identical to `hardness_peak`, path deps on
`engine/`):

```
puzzle = generate_solved_framed_bc(n, c + 5, seed, /*framed*/ true, /*border_colors*/ 5)
rng = XorShift::new(seed ^ 0xA5A5_A5A5)
rng.shuffle(&mut puzzle.pieces)
for p in puzzle.pieces: p = rotated(p, rng.below(4))
```

The planted solution is recovered by tracking the permutation and rotation of
that same scramble and is written beside the solver-visible puzzle, for
verification only. Solvers receive only the scrambled `Puzzle` (no hints, no
planted board). A run is "solved" only if the returned board is verified
independently by the harness (every piece used once, every interior seam
matches, grey exactly on the rim); it need not equal the planted board.

Known faithfulness gaps (documented, not fixed): frame seams per frame colour
are 4(n-1)/5 = 4 to 7.2 at n = 6..10 versus 12 on E2; the generator paints
frame and interior seams uniformly at random within their bands, with no
rare-colour ring and no E2 colour census; no clue pieces; interior-colour
multiplicity follows from c, not from E2's 17 interior colours.

## Seeds and sample sizes

- Test seeds: 1..=30 per size (30 instances per size per colour set; the
  primary 8x8 test instances coincide with hardness-peak's c = 8 instances).
- Calibration seeds: 1001..=1010 per size (10 per size), primary colours only.
  Calibration seeds are never used for test statistics; test seeds are never
  used for baseline selection.
- Solver run seed: 0 for deterministic solvers; 1 for solvers with random
  ordering. Every result is a pure function of (n, c, instance seed, solver,
  run seed, cap).

## Common node metric

One node = one assignment of an oriented piece (piece, rotation) to an empty
cell made by the search, where the oriented piece agrees with every
already-assigned neighbour and with the rim. It is counted when made, whether
or not propagation subsequently refutes it. Not nodes: candidate tests rejected
by the edge check, domain values removed by propagation, undo steps. Rotations
giving an identical oriented piece are deduplicated by every included solver.
Secondary columns: backtracks, candidate fit tests (`attempts`, plain DFS) or
other solver-native propagation counters where available, and wall-clock ms
(measured with 8 runs in parallel on the VM; secondary only).

Implementation: the main engine's `Solver` counts exactly this (`nodes` in its
`Report`); runs are stepped so the count never exceeds C. The single-core
registry engine (`solver-engine`, `solver-naive`) emits one `ValueTried` event
per tried value; the harness counts those through an event sink, verifies each
against the currently assigned ancestors (inconsistent tries are counted
separately and must be zero for the solver to be eligible), and stops the run
at C. An equivalence test checks that the sink count equals the solver's own
`FinalStats.nodes` on uncapped runs.

Cap: C = 10,000,000 nodes per instance, every size, every solver, test runs.
Calibration cap: 2,000,000 nodes. An instance unsolved at the cap is recorded
as right-censored at C with its full row kept (never dropped).

## Baselines

1. Plain DFS from hardness-peak: main-engine `Solver`, row-major path, no
   hints, no piece shuffle (`hp-dfs`). Always included.
2. Best complete DFS: selected on calibration seeds only from these candidates
   (all complete exact DFS that run at runtime board sizes):
   - main engine `Solver` with paths `row-major` (= hp-dfs), `spiral-in`,
     `border-first`, `snake`;
   - single-core registry presets: `naive_rowmajor`, `naive_spiral`,
     `border_first_lcv`, `border_first_full`, `border_first_random`,
     `rare_color_first`, `gacolor_ac3`, `gacolor_ac3_ns1`, `gacolor_ac3_lcv`,
     `gacolor_ac3_random`.
   Excluded with reason: `joe_depth150`, `joe_depth150_bp` (break-gated
   profiles, not exact search); `verhaard_preferred` (needs a phase-0
   preferred-piece list; with an empty list it is `gacolor_ac3`); blackwood,
   verhaard, producer, alns binaries (16x16 CSV only, not complete DFS);
   dfs-study registry (compile-time sizes 8..16 even only, with 16x16
   rim indices, so it cannot run 6, 7, 9 without reimplementation).
   A candidate that errors, panics, returns an unverifiable board, or records
   any inconsistent try at any calibration instance is ineligible.

Selection rule (applied to the 50 calibration instances, cap 2,000,000):
rank eligible candidates by (a) number of calibration instances solved,
descending; ties by (b) median nodes at n = 10 (censored = +infinity),
ascending, then n = 9, 8, 7, 6. The top candidate is the best baseline. The
calibration table and the selected solver are committed in `CALIBRATION.md`
before any test seed is run.

## Analysis plan

Per (solver, colour set, size): solved count, solve fraction with Wilson 95%
interval, median nodes (Kaplan-Meier, equal to the censored order statistic
here), quartiles where defined, empirical survival curve (fraction unsolved
versus node count), median wall ms. Growth: OLS of log10(median nodes) on
cells n^2 and on n over sizes with finite medians, with 95% percentile
bootstrap CI (10,000 resamples, seed 20261111). Kill evaluation exactly as
above. Comparison rules for later candidates live in `PROTOCOL.md`.

## Run order

1. Generate and commit instances (primary, secondary; test and calibration).
2. Calibration: all candidates, calibration seeds, cap 2e6. Commit
   `CALIBRATION.md`.
3. Primary test ladder: best baseline and hp-dfs, cap 1e7.
4. Secondary test ladder: best baseline and hp-dfs, cap 1e7.
5. Analysis, README.

## Compute budget

About 3 hours wall clock on this VM (8 vCPU Intel Xeon, 31 GiB), 8 runs in
parallel, no AWS or paid services. Worst-case projection, from the committed
single-core-benchmark rates (propagating presets near 1e4 nodes/s on 16x16,
assumed at least 5e4 nodes/s on boards of at most 100 cells): calibration
about 25 min, each test ladder at most about 30 min. If the run is slower than
projected the order above is kept; nothing is shrunk after this commit except
by a recorded amendment.
