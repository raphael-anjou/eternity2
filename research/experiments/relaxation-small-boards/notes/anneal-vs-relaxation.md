# T1: restart annealing against the relaxation at 6x6, and annealing at 7x7

TOY BOARDS ONLY. Generated boards (frame 4, inner 5 at 6x6; frame 4, inner 7 at 7x7, seeds as
named, shared painter `scripts/far-ideas-geometry/common.py`). Nothing here is Eternity II
progress. This memo reports; it gives no verdict.

Test: T1 of `a separate re-check memo (not included)` (sections 2.1, 2.2, 2.4, 4). Run 2026-11-14,
02:06 to 02:23 CET.

## What was run

- Annealer: `scripts/geometry-isolation/iso.c` (the source of the re-check's `iso_ref`), copied to
  `anneal-vs-relaxation/iso_t.c` with only a timing patch (`iso_t.diff`): thread CPU seconds per
  run (`CLOCK_THREAD_CPUTIME_ID`, from before the random start board to the end of the walk),
  steps done, and the final board. On 5 runs at 3e5 steps the patched binary prints the same
  trajectories, endpoints and `solved_at` as `scripts/review-far-ideas/iso_ref` with the same
  arguments. Schedule as the re-check fixed it: `anneal RUNS STEPS SEED 1 0.8 0.2` (geometric
  0.8 to 0.2, one snapshot), as in `run_anneal_budget.sh` arms R4 and R5. Seeds are new
  (`make_jobs.py`).
- One run is one restart from a fresh random board. Solved means `M == 0`, re-verified in
  `analyze.py` from the final board: each tile once, every inner edge matched, every rim side
  grey, and the board equals one of the enumerated solutions (`final_dist == 0`). 20 of 20
  solved runs pass; no run failed verification.
- Statistic, per board: expected CPU seconds per verified solution = total thread CPU seconds of
  all runs (unsolved runs at their full budget) / solutions found. Interval: exact Poisson 95
  percent on the count.
- Machines: m1 (Apple M1, 8 cores, 8 single-thread jobs at once, nice 10) ran the grid.
  This Mac (Apple M4, 10 cores, load average about 95 to 127 from other work, 2 jobs, nice 10)
  ran the calibration. Wall clock: grid 14 min 41 s (55 processes), calibration 3 min 11 s.
- Before launching the 3e9-step runs: a probe of 8 concurrent 2e7-step runs on 7x7 seed 0 (seeds
  99900 to 99907, not part of the test, rows in `anneal-vs-relaxation/probe_m1/`) measured 1.53e7
  to 1.59e7 steps per CPU second, so a 3e9 run was estimated at about 194 s. Measured: 194.0 to
  194.2 s median.

## Calibration (seconds per step)

Board 6x6 seed 0, 2e8 steps, 20 runs on each machine:

| Machine | Steps per CPU second | Solved | Expected CPU s per solution |
|---|---|---|---|
| M4 (this Mac, where `s6m1` ran) | 1.224e7 | 7 of 20 | 40.8 [19.8, 102] |
| M1 (m1) | 1.522e7 | 8 of 20 | 28.6 [14.5, 66.2] |

Ratio M4 seconds per step / M1 seconds per step = 1.244. Columns "M4 equivalent" below multiply
the m1 figure by 1.244. No annealing ran on x86; x86 figures are compared in raw seconds,
machine named.

## 6x6, per board

Annealing on m1, 20 runs per board and per budget. Relaxation: `s6m1` (M4, structured
kernel) and x86 `s6.jsonl` (Intel Core Ultra 7 165U), copies taken at 02:23 CET, 19 rows and 42
rows; starts capped at about 1,800 CPU s are charged at their recorded seconds.

| Board (sols) | Anneal 2e6: solved, expected s/sol (M1) | Anneal 2e8: solved | median CPU s to solution (M1) | expected s/sol (M1) | expected s/sol, M4 equivalent | Relaxation mac (M4): solved of n, expected s/sol | Relaxation x86: solved of n, expected s/sol |
|---|---|---|---|---|---|---|---|
| s0 (7) | 0 of 20, none in 2.7 s | 8 of 20 | 8.58 | 28.6 [14.5, 66.2] | 35.6 | 3 of 5, 1,855 [635, 8,996] | 1 of 10, 17,691 [3,175, 698,744] |
| s1 (4) | 0 of 20, none in 2.6 s | 1 of 20 | 7.78 | 257 [46.1, 10,139] | 320 | 3 of 4, 1,145 [392, 5,550] | 1 of 10, 16,974 [3,046, 670,434] |
| s2 (14) | 0 of 20, none in 2.6 s | 7 of 20 | 9.13 | 33.4 [16.2, 83.0] | 41.5 | 2 of 4, 2,615 [724, 21,595] | 1 of 10, 16,785 [3,013, 662,967] |
| s3 (10) | 0 of 20, none in 2.3 s | 3 of 20 | 12.13 | 85.8 [29.4, 416] | 107 | 1 of 3, 4,939 [886, 195,064] | 0 of 10, none in 18,007 s |
| s4 (3) | 0 of 20, none in 2.1 s | 1 of 20 | 8.44 | 244 [43.8, 9,648] | 304 | 0 of 3, none in 5,400 s | 0 of 2, none in 3,602 s |

Pooled 2e8 on m1: 20 of 100 runs solved, 1,220.6 CPU s, 61 s per solution. Median CPU
seconds per 2e8 run (solved or not): 13.05 to 13.11 (M1). At 2e6 a run costs 0.10 to 0.13 s;
100 runs, 12.3 CPU s in all, no solution.

Exhaustive row-scan enumeration of each board, for scale (same binary, m1, one thread): 0.3
to 0.6 s at 6x6, 10.5 s and 54.5 s at 7x7.

## 7x7

| Board (sols) | Budget | Runs | Solved | CPU s in all (M1) | Final mismatches (count) |
|---|---|---|---|---|---|
| 7x7 s0 (2) | 2e8 | 100 | 0 | 1,292 | see below |
| 7x7 s1 (3) | 2e8 | 100 | 0 | 1,300 | see below |
| 7x7 s0 | 3e9 | 5 | 0 | 970 | 6, 3, 3, 4, 4 (best 4, 3, 3, 3, 3) |
| 7x7 s1 | 3e9 | 5 | 0 | 972 | 4, 4, 4, 3, 2 (best 3, 3, 4, 3, 2) |

All 210 7x7 runs together, final mismatches: 2 (3 runs), 3 (7), 4 (35), 5 (67), 6 (47), 7 (32),
8 (14), 9 (4), 10 (1); best along the walk: 2 (4), 3 (18), 4 (97), 5 (84), 6 (7).

## The kill lines

(a), verbatim: "the relaxation reaches solutions that local search does not" is dead at 6x6 if
annealing's expected CPU-seconds per verified solution is at or below the relaxation's on at
least four of five boards;

Numbers bearing on it (from `analysis.txt`):

- Annealing 2e8 against relaxation on M4 (`s6m1`), M1 seconds or M4 equivalent: at or below on
  5 of 5 boards. On s4 the relaxation has 0 solutions in 3 starts (5,400 s), so its figure is
  unbounded.
- Annealing 2e8 against relaxation on x86, M1 seconds: at or below on 5 of 5 boards; on s3 and
  s4 the x86 relaxation has 0 solutions (18,007 s and 3,602 s).
- Annealing 2e6: 0 solutions on every board (12.3 CPU s in all); at or below on 0 of 5 boards
  against either machine, with 1 board (M4) and 2 boards (x86) where both arms have 0 solutions
  and no comparison exists.
- Smallest gap at 2e8, M4 equivalent against `s6m1`: s1, 320 against 1,145 s per solution; the
  intervals overlap there (annealing [57, 12,612] after conversion, relaxation [392, 5,550]) and
  on s3 and s4.

(b), verbatim: "annealing has no funnel at 7x7" is dead if any of the 7x7 runs solves.

Numbers bearing on it: 0 of 200 runs at 2e8 and 0 of 10 runs at 3e9 solved, on boards 7x7 s0 and
s1. Lowest mismatch reached: 2, by 4 runs.

## Deviations

1. Machine. The design asks for the relaxation's machine and clock. The grid ran on m1 with
   the thread clock; the conversion to the M4 rests on 20 runs at one budget on one board
   (ratio 1.244). The M4 was under heavy load from other jobs during the calibration;
   thread CPU time does not count waiting, but contention for caches and the mix of performance
   and efficiency cores can still move it. The relaxation's own load at its run time is not
   known to me. No conversion exists for x86.
2. Per-run CPU time excludes the program's diagnostics outside the walk: the exhaustive
   enumeration and the 1,000 random-board baseline that `iso.c` runs once per process, plus
   process start. They are not part of the annealer.
3. Cap per start. Design point 4 of section 2.4 asks for an equal cap per start (1,800 CPU s).
   T1 fixes the annealing budgets instead (2e6 and 2e8 steps, about 0.13 s and 13 s on M1);
   annealing runs stop at their budget, the relaxation's at its cap.
4. Parameters (design point 5): the annealing schedule was fixed by the re-check before any run
   and not tuned; the relaxation's configuration was not fixed on boards outside this set
   either. Neither was tuned here.
5. Relaxation rows are still being written by another study; the figures use the rows present
   at 02:23 CET (19 on the M4, 42 on x86). Coverage is uneven (x86 s4 has 2 rows).
6. Seeds of the annealing are new (1000 to 4014 range, `make_jobs.py`); the re-check's rows were
   not reused.

## Not run

- Annealing on x86 (no calibration for the x86 relaxation rows).
- A calibration on more than one board or budget.
- An interval on the M4/M1 ratio.
- The 1,800-second equal-cap arm for annealing.

## Files

`anneal-vs-relaxation/`: `iso_t.c`, `iso_t.diff`, `make_jobs.py`, `jobs.txt`, `jobs_calib.txt`,
`jobs_probe.txt`, `run.sh`, `analyze.py` (run as `uv run --with scipy python analyze.py`),
`analysis.txt` (its output), `boards/` (copies of the geometry-isolation board files),
`raw_m1/` (m1 rows and `log.txt`), `raw_calib_m4/` (calibration rows and log),
`probe_m1/` (rate probe), `relaxation_rows/` (read-only copies of `s6m1.jsonl` and x86
`s6.jsonl`).
