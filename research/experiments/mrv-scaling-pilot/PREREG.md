# Track 1 preregistration: 9x9 high-cap pilot for the bb-mrv / scb node ratio

Night shift 2026-11-12, Track 1 of `research/program/NIGHT_BRIEF.md`. Branch
`night/2026-11-12-t1-9x9-pilot` from `origin/main` (6247af0). Committed
locally before any test run (tonight's rule: nothing is pushed). Any forced
change goes in `PREREG_AMENDMENTS.md` (what, why, when); this file stays
unchanged.

**Track label: measurement.** No new method and no optimization. Both arms are
the A22 code on `origin/main`, unchanged (the `mrv_baseline` binary of
`../mrv-baseline/compute`, built with `--locked`). Generated boards only;
nothing here is official Eternity II progress.

**This is a pilot.** Eight pairs at one size. It decides whether a 30-seed
9x9 / 10x10 scaling run is worth requesting; it is not that run, and no
scaling claim (slope over n) is made from it.

## Question and hypothesis

A22 (AGENDA, PR #26) measured the median paired log10 node ratio bb-mrv / scb
at -0.255, -0.379, -0.832 for n = 6, 7, 8 on the c = primary + 2 track, and
left 9x9 c=11 and 10x10 c=12 censored for both solvers on every pair at 1e7
nodes / 180 s. Question: does the ratio keep shrinking past 8x8?

H: at 9x9 c=11 with a 1e8 node cap, the median paired log10 node ratio
bb-mrv / scb is below the 8x8 value.

## Instances

- n = 9, c = 11 (A22 `plus2` track: `colours(Primary, 9) + 2`), test seeds
  101..108 (fresh: not 1..50, not 1001..1010; `pilot_instances check` asserts
  none is in the A22 ladder file).
- Generator: `hard_ladder::instance::make_instance(Ladder::PlusTwo,
  Role::Test, 9, seed)`, i.e. `generate(9, 11, seed)` =
  `generate_solved_framed_bc(9, 16, seed, true, 5)` plus the ladder scramble
  `XorShift(seed ^ 0xA5A5A5A5)` (hard-ladder amendment A4). No ladder file is
  edited.
- File: `instances/plus2-test.jsonl` in this directory (8 rows). The name is
  forced by the A22 binary's loader, which reads
  `$HARD_LADDER_ROOT/instances/<ladder>-<role>.jsonl`; runs set
  `HARD_LADDER_ROOT` to this directory. `compute/` (`pilot_instances check`)
  regenerates the file and requires byte equality, checks c = 11, and checks
  that the same call reproduces A22's committed row `n9-c11-s1` byte for byte.

## Arms

| arm | role | code |
|---|---|---|
| `bb-mrv` | candidate | A22 `mrv_baseline --solver bb-mrv` (A16 `BbSolver { rule: Rule::Mrv }`) |
| `scb-gacolor_ac3_random` | baseline | A22 `mrv_baseline --solver scb-gacolor_ac3_random`, run seed 1 |

Both count nodes through `NodeCounter::try_assign` (PROTOCOL.md node metric),
cap 1e8 nodes per run, identical for both arms.

Control: this is a measurement of an existing pair, with no new mechanism, so
the "everything except the mechanism" control does not apply. The control
that does apply is a build check: before the test runs, the binary built here
is run on A22's committed `plus2` 7x7 seeds 1..10 (cap 1e6, both arms) and
must reproduce A22's committed `nodes` and `status` exactly
(`results/check/`). If it does not, no test run starts and the mismatch is
reported.

## Caps and guards

- Node cap: 100,000,000 per run (`--cap 100000000`), both arms.
- Wall-clock guard: lifted. A22's arms stop on a 180 s / 600 s guard; here the
  guard passed to `--guard` is the number of seconds from the run's launch to
  **2026-11-13 05:15 local time (CET)**, which for any run launched tonight is
  above 8 hours, far beyond a plausible run (A22 measured 13k to 22k nodes/s, so
  1e8 nodes is about 1.3 to 2.1 h on that VM). Only the node cap is meant to
  censor a run.
- Preregistered exception: a run still going at 05:15 on 2026-11-13 is stopped
  by that guard. It is recorded as **time-censored** (status `censored` with
  `nodes_counted` < cap), reported separately from node-censored runs, and its
  pair is uninformative (scored below).
- A run is node-censored iff status `censored` and `nodes_counted` = cap (scb
  may overshoot by a few nodes; `nodes_counted` >= cap counts as node-censored).

## Scheduling and core budget

4 concurrent single-threaded runs (`--threads 1`, `RAYON_NUM_THREADS=1`,
`nice -n 5`; macOS has no `taskset`, so there is no core pinning), 4 cores of
an Apple M4 (4 performance + 6 efficiency cores) shared with other tracks.
Builds use `cargo build -j 4`. Pairs are dispatched together: a seed's two
runs start at the same time, when two of the four slots are free, in seed
order 101..108. Waves: wave k holds seeds 99 + 2k and 100 + 2k (k = 1..4);
rows go to `results/raw/wave<k>.jsonl`. Each row adds `binary`, `binary_sha256`,
`wave`, `started_at`, `ended_at`, `guard_s`, `host` to the A22 row.

## Binary rule (Track 0)

All waves start on the A22 binary from `origin/main`. A later wave (one whose
pairs have not started) may switch `bb-mrv` to tonight's Track 0 binary only if
the lead states that Track 0's judge returned keep. The binary path and sha256
are recorded per run. Since Track 0's gate requires an identical node sequence,
node counts stay comparable across waves; wall clock and nodes/s are not
compared across binaries. Nothing from the Track 0 branch is used before that
statement.

## Progress visibility

A 1e8 run writes its row only at the end. The run script appends a heartbeat
line every 5 minutes per live run to `results/heartbeat.log` (timestamp, run,
pid, elapsed, CPU time from `ps`), and commits `results/` at least every 60
minutes. Rising CPU time tells a long run from a stall.

## Statistic and kill criterion (binding)

For each seed i, with C = 1e8 and b_i, s_i the bb-mrv and scb nodes, the paired
log10 ratio r_i follows PROTOCOL.md G1 with log10 in place of ln:

| bb-mrv | scb | r_i |
|---|---|---|
| solved | solved | log10(b_i / s_i) |
| solved | node-censored | log10(b_i / C) (an upper bound on the true ratio; enters the median at that value) |
| node-censored | solved | +infinity (bb-mrv loses) |
| node-censored | node-censored | 0 (tie), counted as both-censored |
| either run time-censored | any | 0, counted as uninformative |

An `error` row (panic, unverifiable board, exhaustion of a planted instance)
invalidates the pilot until fixed and is reported.

Statistic M: the upper median of the 8 r_i (the 5th smallest), the
conservative choice for an even sample as in G1.

Reference: the A22 8x8 c=10 value, committed `median_ln_ratio` =
-1.9160979637943325 in `../mrv-baseline/results/summary.json`, i.e. log10
ratio T = -0.83216 (ratio 0.147). The brief rounds it to -0.833.

Kill (the pilot result is "no evidence the ratio keeps shrinking") if either:

- K1: M >= T (the median is not below the 8x8 value), or
- K2: U > 4, where U = number of pairs both node-censored plus number of
  pairs with a time-censored run (uninformative pilot).

Otherwise the pilot survives: "the ratio at 9x9 is below the 8x8 value on this
pilot", which supports requesting the 30-seed run, nothing more. If M falls in
[-0.833, -0.83216) it is reported as within rounding of the reference.

Caveat recorded now: the 8x8 reference was taken at cap 1e6 with 10 of 30 scb
runs censored there (scored log10(b/1e6), an upper bound), so it is itself
partly a bound; the 9x9 value at cap 1e8 is a different cap. The comparison is
what the brief asks for; it is not a like-for-like slope.

## Reporting (descriptive, not binding)

- All 8 pairs in full: seed, nodes, nodes_counted, status per arm, r_i.
- Percentile bootstrap 95% CI of the upper median, 10,000 paired resamples of
  the 8 pairs, seed 20261111 (coarse with 8 pairs; reported, not used).
- Per arm: solved count, nodes per second per run (`nodes_counted / wall`),
  wall clock per run, binary per wave.
- Paired wall-clock ratio bb-mrv / scb per pair (descriptive; machine shared
  with other tracks, so noisy).

## Compute budget

4 cores, about 16 runs of up to about 1 to 2 h each, in roughly 2 to 4 hours
of wall clock if bb-mrv finishes early, bounded by 05:15. The build check
costs a few minutes. No AWS, no 10x10, no 16x16.

## Determinism

Instances and seeds are fixed; bb-mrv has no RNG; scb uses run seed 1. Node
counts reproduce exactly unless the 05:15 guard fires; wall clock does not.
