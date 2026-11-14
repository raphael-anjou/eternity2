# Partial-match ladder: preregistration

Track 7 of the 2026-11-12 night shift, added by the lead (origin: proposal L4
in `research/program/night/2026-11-12/ideas-raw/lead-proposals.md`, novelty
check "lead L4" in `NOVELTY.md`, judge remarks in `TOURNAMENT.md` sections 1,
3(c), 3(d), all on branch `night/2026-11-12-lead`).

**Track label: measurement.** This builds an instrument. It proposes no
solver and tunes none. Frozen before any test instance is generated or run.
Calibration output goes to `CALIBRATION.md`, defined below; forced changes go
to `PREREG_AMENDMENTS.md` (what, why, when). Nothing here touches the 16x16
board; nothing here is progress on the real puzzle.

## Purpose

Every judged experiment so far measures nodes to a first perfect solution on
small boards that have one. The open problem is the best score on a board where
no engine finds a perfect solution (470 of 480, a gap of 2.08%). This ladder
supplies planted boards (optimum known: every interior seam, 2n(n-1)) at sizes
and colour counts where the repo's score-maximising engines stall a few percent
below the optimum, and a fixed metric: best verified score at fixed work, and
work to reach optimum minus k for k = 1..6.

## Hypothesis

There is a rung (n in {8, 10, 12}, interior colour count c) at which, with the
budget pair fixed below, at least three of the five entrants stall with a
median relative gap in [2%, 4%], the best entrant stays below the optimum on at
least 25 of 30 test instances, and the instrument separates entrants whose
16x16 order is established.

## Instances

Generator (path dependency on the hard ladder's crate, unchanged):
`hard_ladder::instance::generate(n, c, seed)` =
`generate_solved_framed_bc(n, c + 5, seed, true, 5)` then the hardness-peak
scramble (`XorShift(seed ^ 0xA5A5A5A5)` shuffle, one random rotation per
piece). c = interior colours, 5 frame colours. Solvers receive the scrambled
puzzle only, no hints. The planted board is stored beside it for the instance
check only. `pml check <file>` regenerates every row and verifies the planted
board at the optimum.

- Sizes: n = 8, 10, 12. Optimum 112, 180, 264.
- Calibration colour candidates: n = 8: c in {6, 8, 10, 12}; n = 10: {8, 10,
  12, 14}; n = 12: {10, 12, 14, 16}.
- Calibration seeds: 7001..7004 (4 per (n, c)), files
  `instances/cal-n<n>-c<c>.jsonl`.
- Test seeds: 7101..7130 (30 per selected rung), files
  `instances/test-n<n>-c<c>.jsonl`, generated only after `CALIBRATION.md` is
  committed. Test seeds are never used for any choice; calibration seeds never
  enter a test statistic.
- Run seed: 1 for every stochastic entrant (one run per instance). BreakSolver
  is deterministic.

Faithfulness gaps (stated, not fixed): generated boards are not the Eternity II
piece set; the planted optimum is the full edge count, while the 16x16's
optimum is unknown; frame-colour census, rare-colour ring and clue pieces of E2
are not reproduced.

## Verifier

`src/verify.rs::verify`, the only scorer whose number enters a result: a board
in the main engine's encoding (`piece * 4 + rotation`, row-major). An empty
cell (missing piece), an out-of-range or reused piece, or a rim violation is an
`error` row, never a score. Score = interior seams with equal colours. An
entrant that returns no complete board within the budget gets status
`no_complete_board` and share 0 in the primary metric (its partial-board score
is a secondary column). The harness re-verifies the returned board itself and
also checks that the last trajectory point equals the verified score; any
mismatch is an `error` row. Any `error` row in a test batch invalidates that
entrant at that rung until fixed (recorded as an amendment).

## Work counter and units

`src/work.rs::Work`, charged at one place per adapter:

- `placement` (constructive): one placement of an oriented piece on an empty
  cell made by the search (BreakSolver's own `nodes`). The adapter steps the
  solver with at most the remaining budget, so it never overshoots.
- `move_eval` (local search): one proposed move evaluated by the local-search
  crate's SA kernel (one SA iteration; for PT, `burst` iterations per replica
  burst; for ALNS, `iters x repair_step_budget`, an upper bound because an
  iteration whose destroy set is empty does no repair; destroy-operator and
  full-board rescoring cost is not counted).

Not comparable across families: a placement and a move evaluation are
different operations, so work-to-target numbers are compared only within a unit
(`sa`, `pt`, `alns` among themselves; `break-r1`, `break-r2` between
themselves). Cross-family comparison uses only the best score at the fixed
budget pair. Wall clock is a secondary column.

Budget pair per size: `W_eval(n) = 4,000,000 * n^2` move evaluations (8x8:
256e6; 10x10: 400e6; 12x12: 576e6) and `W_place(n) = rho * W_eval(n)`
placements. Exchange rate rho, measured once before calibration: on `dell`,
`break-r1` with 100,000,000 placements and `pt` with 100,000,000 move
evaluations on the 16 calibration instances at n = 8 (all four c), 6
concurrent runs; rho = (median over runs of placements per wall second of
`break-r1`, using only runs that used their whole budget) / (median move
evaluations per wall second of `pt`), rounded to one significant figure. The
probe and rho are written to `CALIBRATION.md`. If no `break-r1` run uses its
whole budget, rate = work_used / wall over all runs.

## Entrants (unchanged engines, committed configurations)

| name | engine | configuration |
|---|---|---|
| `break-r1` | `engine/src/break_solver.rs` `BreakSolver` | snake path (`build_path("snake", n, n, 0)`), no hints, break licence on every cell of the bottom row: the BreakIndexLab configuration (`web/src/components/research/BreakIndexLab.tsx`, snake, bottom `breakRows` rows; its note recommends one row first) |
| `break-r2` | same | bottom two rows licensed |
| `sa` | single-core-benchmark `localsearch::run_sa` | `SaConfig::default()` (T 2.0 to 0.05, cooling 0.999 per 5,000, re-anneal), `max_iters` = budget, seed = run seed |
| `pt` | `localsearch::run_pt_from` default path | `PtConfig::default()`: 8 replicas, T 0.05..2.0 geometric, 100,000-step bursts, greedy fill, no repair/kick/Houdayer/penalty; run as a sequential twin that returns the same best board (test `pt_twin_matches_run_pt_from`, the quality-diversity-basins pattern) and stops at the budget or a perfect board |
| `alns` | `localsearch::run_alns` | `alns_only` defaults: `winning5` ops, SA acceptance t = 1.0, segment 50, SA repair, CP fallback on; deterministic `repair_step_budget` = 100,000 (no committed value exists; fixed here equal to the PT burst, not tuned), `iter_budget` = budget / 100,000, start board = the local-search greedy fill with the run seed |

Trajectories (for work to optimum minus k): `break-*` has one point (the
completed board); `pt` records every new best at burst resolution, each
verified; `sa` is resolved on a doubling grid of budgets `W >> j` (down to
2^20) using prefix runs (the run with `max_iters = g` is the prefix of every
longer run: same seed, no clock), each verified, so its work-to-target is an
upper bound within a factor 2; `alns` uses the engine's own best-score history
(only the final board is verified; points are engine-reported).

Excluded, with reason:
- `blackwood_bt`, `verhaard_faithful_v2` (single-core-benchmark bench-audit):
  board size compiled in (`const N: usize = 256; const W: usize = 16`) with
  16x16-tuned break-unlock positions and quotas. Running them at n <= 12 needs a
  reimplementation.
- `producer` (beam): 16x16 CSV input and withheld pending publication.
- repair-study engines (incl. ACCEPT-ANNEAL) and dfs-study engines: built on
  `common/crates/e2-core`, `pub const N: usize = W * H` fixed at 16x16, or
  compile-time sizes with 16x16 rim indices.
- single-core-benchmark native strict DFS presets (`naive_rowmajor`, CSP
  presets): exact searches that return partial boards; under the complete-board
  rule they score only when they solve outright. Not score maximisers.
- `joe_depth150*`: depth gate tuned to 256 cells.
- `run_alns_portfolio`, `run_alns_pt`, multi-chain modes: multi-threaded.
- MAP-Elites (quality-diversity-basins): a killed experimental method, not a
  committed score-maximising configuration.

## Calibration (calibration seeds only, on `dell`)

All five entrants, all 12 (n, c) cells, 4 calibration instances each, the
budget pair above. Per cell: gap_e(i) = (optimum - best_e(i)) / optimum (1 for
no complete board); best entrant = highest median share; the cell qualifies iff
the best entrant's median gap is in [0.02, 0.04] and the best entrant is below
the optimum on at least 3 of 4 instances. Qualifying cells are ranked by (a)
number of entrants whose median gap is in [0.02, 0.04], descending, (b)
|best-entrant median gap - 0.03|, ascending, (c) larger n first. Rung A = the
top cell; rung B = the top cell with a different n. If no cell qualifies, rung
A = the cell whose best-entrant median gap is closest to 0.03 among cells with
best below optimum on at least 3 of 4, labelled "outside band", and criterion
(1) is still evaluated as written. `CALIBRATION.md` (table of all cells and the
choice) is committed before any test instance is generated.

## Test

Rung A first, then rung B if time allows: 30 test instances, all five
entrants, run seed 1, the budget pair of that size.

Cross-machine determinism (before any work is split across machines): one
calibration instance (n = 8, c = 10, seed 7001), each entrant, budget 2e7 of
its unit, on `dell` and `mac-m1`; identical `best_score`, `work_used` and
`trajectory` required. An entrant that differs runs wholly on one machine and
the discrepancy is reported. Every row records its machine.

## Metrics (per rung, per entrant)

- Share s_e(i) = best_e(i) / optimum (0 for no complete board); median, mean,
  quartiles; count at the optimum; count with no complete board; errors.
- Work to optimum minus k, k = 1..6: first trajectory work with score >=
  optimum - k, right-censored at the budget. Kaplan-Meier median = 15th order
  statistic of 30 with censored = +infinity; count reached.
- Wall ms (secondary; machine stated).

## Usability verdict (the instrument's own kill)

Evaluated on rung A (and separately on rung B if run). The instrument is usable
at a rung only if (1) and (2) pass and (3) does not fail.

**(1) Stall band.** Entrant e stalls in band iff the median over the 30
instances of gap_e(i) lies in [0.02, 0.04]. Pass iff at least 3 of the 5
entrants stall in band AND max_e best_e(i) < optimum on at least 25 of 30
instances.

**(2) Separation.** For each established pair (definition in (3)): paired
two-sided Wilcoxon signed-rank test on s_a(i) - s_b(i) over the 30 instances,
zero differences dropped, p-value from 100,000 random sign flips (seed
20261112; p = 1 if every difference is zero). Holm correction across the
established pairs at alpha = 0.05. Pass iff every established pair is
significant. All 10 entrant pairs are also reported (descriptive).

**(3) Order agreement with the 16x16.** A pair is *established at
implementation level* iff both entrants have committed, measured 16x16 results
from the same implementation, same budget and clue convention, with at least 8
of 10 paired variants in the same direction. Pass iff at least three entrants
are linked by established pairs and every established pair has the same
direction on the rung (sign of the median paired difference; a zero median
counts as disagreement). Fewer than three linked entrants: **undetermined**.

Known before any run: no entrant has a committed 16x16 result from its own
implementation (`BreakSolver`, `run_sa`, `run_pt_from` and `run_alns` from a
greedy start have none; see the table below). Criterion (3) is therefore
undetermined at implementation level, and (2) has no implementation-level
pair. Both are then evaluated at **family level**, labelled as such and never
reported as an implementation-level pass:

| family | entrants | committed 16x16 evidence (file:line) | grade |
|---|---|---|---|
| break-tolerant DFS | `break-r1`, `break-r2` | `research/experiments/single-core-benchmark/results/REPORT.md:18-19,58-59` (verhaard_faithful_v2 mean 440.8, blackwood_bt 436.4; 60 s, 1 core, 10 corner-pinned variants); `research/experiments/dfs-study/results/report.md:40-41` (BREAK-1 mean 431.3, worst 427) | measured (other implementations) |
| break-tolerant DFS | | `research/program/EXPLORED.md:20` (470 Blackwood, 467 Verhaard) | reported |
| ALNS | `alns` | `research/experiments/single-core-benchmark/results/boards/alns__v00.json` .. `alns__v09.json`, line 4 (451 to 455, mean 452.9; beam start then ALNS, 60 s, same variants; row withheld, `solvers.toml:197-212`) | measured (other pipeline: beam start) |
| annealing | `sa` | `research/experiments/repair-study/results/report.md:38` (ACCEPT-ANNEAL mean 376.6, best 394; destroy-repair with SA acceptance, 60 s, same variants) | measured (other implementation) |
| parallel tempering | `pt` | none committed (prose 457 in `web/content/research/why/sigma-cycles.mdx`, no result file) | none |

Family-level established pairs (measured, same 60 s convention, at least 8 of
10 variants in the same direction; worst of the higher family above best of the
lower one): break-tolerant DFS above annealing (worst 427 > best 394), ALNS
above annealing (worst 451 > best 394). ALNS against break-tolerant DFS is
**not** established: the measured 60 s rows put beam-plus-ALNS above (451 to
455 against at most 451), the reported records put break-tolerant DFS above
(470 against 463). So the family-level pairs are (`break-r1`, `sa`),
(`break-r2`, `sa`), (`alns`, `sa`), each with the higher member first; three
entrants are linked. Family-level (2) and (3) use exactly these pairs.

## Analysis

`analysis/analyze.py` reads the raw jsonl only and writes
`results/summary.json` and the README tables: everything above, plus every
error row. No number in the README is typed by hand.

## Compute budget and machines

`dell` (Linux x86_64) at most 6 threads and `mac-m1` (macOS arm64) at most 3,
both shared with other tracks; solver processes counted before each batch.
Estimate: probe about 3 min; calibration about 5.5 thread-hours (about 40 min
on 9 threads); each test rung 4 to 5 thread-hours. Report deadline 02:30;
delivery order: instances, verifier, counter; calibration table; rung A test;
verdict; rung B.
