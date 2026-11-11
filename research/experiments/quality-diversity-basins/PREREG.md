# PREREG: quality-diversity search over basin descriptors (agenda A4)

Pre-registered 2026-11-11, before any calibration or experiment run. Branch
`devin/r1-quality-diversity-basins`. Changes after this commit go to
`PREREG_AMENDMENTS.md`; this file is not edited.

## Hypothesis

From `research/program/AGENDA.md`, A4: the open diversity problem is a
descriptor problem. MAP-Elites over a small descriptor grid (mismatch
positions, frame identity, colour-flow statistics) keeps many basins alive and
finds more distinct basins per core-hour than parallel tempering (PT).

Tested here as: on generated planted puzzles at 8x8 and 10x10, MAP-Elites
built on the same move kernel, scorer, temperature ladder and initial boards
as the repo's PT finds more distinct near-best basins than PT at an equal
number of move evaluations.

## Kill criterion

Verbatim from the agenda: **"No more distinct basins than PT at equal compute
on both sizes."**

Operationalisation:

- Statistic, per run: `B(k=2, delta=0.25)`, the number of distinct basins (see
  definition below) among the boards the run visited with score
  `>= best_known - 2`.
- Per puzzle: the mean of `B` over the two run seeds, for each method. Paired
  difference `D_p = mean_B_ME(p) - mean_B_PT(p)` over the 30 puzzles of a size.
- Test, per size: one-sided exact Wilcoxon signed-rank test of `D_p > 0`
  (zero differences dropped, mid-ranks for ties, exact null distribution of the
  rank sum by enumeration over sign assignments). Two sizes, Holm-Bonferroni at
  family-wise alpha = 0.05 (smaller p-value against 0.025, the other against
  0.05).
- Reading: **killed** unless MAP-Elites has significantly more basins on at
  least one size after Holm. If it is significant on one size only, the result
  is reported as "one size"; whether it wins on both sizes is reported
  separately.
- Quality guard (does not change the kill rule, only the survived label): on a
  size counted as a win, if the median over puzzles of the paired best-score
  difference (ME minus PT, puzzle means over seeds) is below -2 (worse by more
  than the primary k), the self-assessment is **inconclusive**, not survived,
  because more basins at a clearly worse score is not a win.

## Methods and baseline

Both methods are implemented in `engine/src/main.rs` on top of the single-core
benchmark's local-search crate
(`research/experiments/single-core-benchmark/engine/crates/localsearch`).

- **Shared parts.** Puzzle from the engine generator; scorer = matched interior
  edges (local-search `State::score`, cross-checked by an independent scorer);
  initial boards = 8 `greedy_fill_initial` boards seeded exactly as
  `run_pt_from` seeds its replicas; moves = `run_sa_steps_fixed_temp` (the
  kernel `run_pt_from` calls: single rotation, 2x2 and 3x3 block rotation,
  targeted swap, 2x2 region swap, random swap, 3-cycle), in bursts of
  `L = 20,000` steps at a fixed temperature; temperature ladder = the PT default
  (8 temperatures, geometric from 0.05 to 2.0).
- **Baseline: PT.** The repo's `run_pt_from` default path (greedy fill, no
  Houdayer, repair, kick or forbidden-edge penalty), n_replicas 8, t_min 0.05,
  t_max 2.0, with `inner_iters = 20,000` instead of the default 100,000 (small
  boards; this gives more exchange rounds per budget). Re-implemented as a
  sequential loop so it runs on one core and records every burst's best board;
  a unit test (`pt_matches_repo_run_pt_from`) checks that the best score and
  best board equal `run_pt_from`'s on two puzzles for a fixed number of
  rounds. One deviation: no early stop at a perfect board, so both methods
  spend the full budget.
- **MAP-Elites.** Archive = one elite per cell of the descriptor grid below.
  Initialised with the same 8 greedy boards. Each iteration: draw an occupied
  cell uniformly, draw a temperature uniformly from the PT ladder, run one burst
  of `L` steps from that elite's board, offer the burst's best board and its
  final board to the archive (a cell takes a board if empty, or if the board
  scores strictly higher than its elite).
- **Descriptor grid (4 x 6 x 4 = 96 cells), all invariant under the four
  board rotations.**
  - d1 (mismatch positions): share of mismatched seams with at least one
    endpoint on the outer ring, 4 equal bins on [0, 1]; a perfect board goes to
    bin 0.
  - d2 (frame identity): cyclic order of the four corner pieces modulo
    rotation (6 classes).
  - d3 (colour flow): number of distinct colours on mismatched seams divided
    by the colour count, 4 equal bins on [0, 1]; a perfect board goes to bin 0.
- **Equal compute.** Same number of move evaluations per run (`B` budget, a
  multiple of `8 L`; both methods execute exactly `B / L` bursts), one core per
  run, the same machine, at most 7 runs at once on the 8-core VM. Measured:
  executed evaluations, wall clock and process CPU seconds per run are
  recorded and compared, not assumed.

## Distinct basin definition

- Candidate boards of a run: the best board of every burst (the one board per
  burst that the kernel's best-tracking returns), deduplicated exactly.
- `best_known(p)`: the maximum best score over all four runs (2 methods x 2
  seeds) of puzzle `p`. Pooled across methods so that both methods are counted
  against the same reference.
- Near-best set: candidates with score `>= best_known - k`.
- Distance: fraction of cells holding a different (piece, rotation), minimised
  over the four rotations of the whole board (mirror images are not legal
  boards, so the symmetry group is C4).
- Clustering: leader clustering. Candidates sorted by score descending, then
  by the encoded board ascending; a candidate within distance `< delta` of an
  existing leader joins it, otherwise it becomes a leader. `B(k, delta)` = the
  number of leaders.
- Primary: `k = 2`, `delta = 0.25`. Sensitivity grid: `k in {0, 1, 2, 4}` x
  `delta in {0.10, 0.25, 0.50}`, each reported with the same per-size test
  (unadjusted, descriptive only).

## Boards, colours and generator

- Generator: `eternity2_engine::generate(size, colors, puzzle_seed)` (unframed,
  scrambled, solvable by construction; planted optimum = `2 n (n - 1)`
  matched edges: 112 at 8x8, 180 at 10x10).
- Sizes: 8x8 and 10x10. Nothing larger.
- Colour count, fixed by calibration before the main run (see below):
  candidates 8x8 `{8, 10, 12, 14}`, 10x10 `{10, 12, 14, 16}`.

## Calibration (run before the main grid, on disjoint seeds)

1. Budget. One PT run per size (first candidate colour count, puzzle seed 9001,
   run seed 1, 40M evaluations) measures evaluations per second. The budget `B`
   for that size is the largest value in `{40M x 2^i, i = 0..7}` whose
   projected single-run wall clock is at most 75 s.
2. Colours. For each candidate colour count in increasing order: 4 calibration
   puzzles (seeds 9001 to 9004) x run seed 1 x both methods at budget `B`. The
   chosen colour count is the smallest one whose pooled solve rate (runs that
   reach the planted optimum) is at most 25% (2 of 8 runs). If none qualifies,
   the largest candidate.
3. Output: `results/calibration.json` (committed). The main grid reads it; the
   reproduction recipe uses the committed file, so only step 1 depends on
   machine speed.

## Seeds and sample sizes

- Main puzzles: seeds 1 to 30 per size (30 puzzles per size).
- Run seeds: 1 and 2 for each method on each puzzle.
- Runs: 2 sizes x 30 puzzles x 2 seeds x 2 methods = 240 runs.
- Calibration puzzles 9001 to 9004 are never used in the main analysis.

## Metrics

Primary: `B(2, 0.25)` per run. Secondary, all reported per size:

- basins per measured core-hour (`B / (cpu_s / 3600)`) and the same Wilcoxon
  test on it;
- best score per run, paired ME minus PT difference, two-sided Wilcoxon;
- solve rate (runs reaching the planted optimum) per method;
- number of near-best boards (before clustering), archive fill (ME);
- executed evaluations, wall clock and CPU seconds per method;
- the sensitivity grid over `(k, delta)`.

## Analysis plan

`scripts/analyse.py` reads `results/basins.jsonl` (one row per run, written by
`qd_basins basins`) and writes `results/summary.json` plus the README tables.
Mean and median of `D_p`, a 95% bootstrap interval for the mean of `D_p`
(10,000 resamples of puzzles, seed 12345), the exact Wilcoxon p-value, Holm
decision, and the outcome label by the rules above.

## Compute budget

8-core VM (no GPU), at most 7 concurrent single-threaded runs. Target wall
clock per run at most 75 s by construction of `B`; main grid at most about
240 x 75 s / 7 = 43 min, calibration at most about 1 + 64 runs (under 15 min).
Total well under the 3 h limit. If the measured main-grid projection exceeds
2.5 h, the budget ladder step is lowered by one before the main grid starts
(that would be an amendment, logged before running).
