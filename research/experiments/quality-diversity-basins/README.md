# A4: quality-diversity search over basin descriptors vs parallel tempering

Agenda entry A4 (`research/program/AGENDA.md`). Pre-registration:
[`PREREG.md`](PREREG.md) (commit `6d116b4`, pushed before any run). No
amendments.

**Outcome (self-assessment): killed.** MAP-Elites found fewer distinct near-best
basins than parallel tempering on every size, on 58 of 60 puzzles (2 ties, 0
puzzles where MAP-Elites had more), and also reached a lower best score.

## What was run

- **Puzzles.** `eternity2_engine::generate(size, colors, puzzle_seed)` (unframed,
  planted solution of `2 n (n - 1)` matched edges). 8x8 with 8 colours, 10x10
  with 10 colours, puzzle seeds 1 to 30 per size.
- **Methods.** Both in [`engine/src/main.rs`](engine/src/main.rs), built on the
  single-core benchmark's local-search crate. Same scorer, same 8 greedy initial
  boards, same move kernel (`run_sa_steps_fixed_temp`, bursts of 20,000 steps),
  same 8-temperature ladder (0.05 to 2.0). Only archive and selection differ.
  - **PT**: the repo's `run_pt_from` default path, re-implemented as a
    sequential loop that records every burst's best board. The unit test
    `pt_matches_repo_run_pt_from` checks that it returns the same best score
    and board as `run_pt_from` on two puzzles. No early stop at a perfect board.
  - **MAP-Elites**: one elite per cell of a 96-cell grid (share of mismatches on
    the outer ring, 4 bins; corner-piece cyclic order, 6 classes; share of
    colours on mismatched seams, 4 bins). Each burst starts from a uniformly
    drawn elite at a uniformly drawn ladder temperature.
- **Budget.** Equal move evaluations per run, fixed by the pre-registered
  calibration: 320M at 8x8, 160M at 10x10 (16,000 and 8,000 bursts). One core
  per run, 7 concurrent runs.
- **Colours.** The calibration rule picks the smallest candidate colour count
  with a pooled solve rate at most 25% on calibration puzzles 9001 to 9004. The
  smallest candidates (8 at 8x8, 10 at 10x10) already gave 0 of 8 solves, so
  they were selected ([`results/calibration.json`](results/calibration.json)).
- **Basins.** Candidates = each burst's best board, deduplicated; near-best =
  score at least `best_known - k`, with `best_known` pooled over the 4 runs of a
  puzzle; leader clustering at placement distance `delta` (fraction of cells
  with a different piece or rotation, minimised over the 4 board rotations).
  Primary `k = 2`, `delta = 0.25`.
- **Test.** Per puzzle, mean over 2 run seeds; paired ME minus PT difference;
  one-sided exact Wilcoxon signed-rank per size; Holm over the two sizes at
  0.05.

## Commands

From the repo root, on a clean checkout:

```sh
just experiments quality-diversity-basins        # main grid + basins + analysis, writes results/rerun/
```

Calibration (machine dependent in its budget step, so the recipe reads the
committed `results/calibration.json` instead of re-running it):

```sh
cd research/experiments
cargo build --release --manifest-path quality-diversity-basins/engine/Cargo.toml
python3 quality-diversity-basins/scripts/run_grid.py calibrate \
    --out quality-diversity-basins/results --scratch quality-diversity-basins/results/scratch
```

The committed results were produced by the same three steps as the recipe with
`--runs results/runs` and outputs in `results/`. Exploratory analysis (not
pre-registered):

```sh
python3 quality-diversity-basins/scripts/exploratory_own_best.py quality-diversity-basins/results/runs 2 0.25
```

Reproduction check: `just experiments quality-diversity-basins` on a fresh
clone of the branch (commit `0a31297`) finished in 28 min 52 s wall clock and
produced a `basins.jsonl` identical to the committed one in all 240 rows
(every field except `wall_s` and `cpu_s`), so the summary statistics match.

## Hardware and compute

8-vCPU VM, `Intel(R) Xeon(R) Processor`, 31 GB RAM, no GPU. Rust 1.99.1.

- Calibration: about 3.5 min wall clock (2 probes + 16 runs).
- Main grid: 240 runs, 2,072 s wall clock with 7 concurrent runs.
- Measured per run (means): 8x8 PT 81.6 CPU s, ME 86.3 CPU s (ratio 1.06);
  10x10 PT 36.1 CPU s, ME 35.4 CPU s (ratio 0.98). Executed evaluations were
  exactly the budget for every run (320,000,000 and 160,000,000).
- Total main-grid CPU: 14,367 s (about 4.0 core-hours).

## Results

Files: [`results/basins.jsonl`](results/basins.jsonl) (one row per run: best
score, timings, archive fill, basin counts for every `(k, delta)`),
[`results/summary.json`](results/summary.json) (all statistics below),
[`results/calibration_runs.jsonl`](results/calibration_runs.jsonl). The per-run
board files (`results/runs/`, 34 MB) are not committed; the recipe regenerates
them deterministically.

Primary statistic, distinct basins at `k = 2`, `delta = 0.25` (puzzle means, 30
puzzles per size, 95% bootstrap interval of the mean paired difference):

| size | colours | budget (evals) | PT basins | ME basins | mean diff [95% CI] | ME>PT / PT>ME | p (one-sided, ME>PT) | Holm |
|---|---|---|---|---|---|---|---|---|
| 8x8 | 8 | 320,000,000 | 22.33 | 5.63 | -16.70 [-20.43, -13.20] | 0 / 30 | 1 | not rejected |
| 10x10 | 10 | 160,000,000 | 5.30 | 1.00 | -4.30 [-5.28, -3.35] | 0 / 28 | 1 | not rejected |

Per measured core-hour the gap is the same: 8x8 PT 1,019 vs ME 244 basins per
core-hour, 10x10 PT 538 vs ME 105 (one-sided p = 1 on both).

Secondary outcomes:

| size | PT best | ME best | median diff | p (two-sided) | PT solved runs | ME solved runs | near-best boards PT / ME | ME archive cells (of 96) |
|---|---|---|---|---|---|---|---|---|
| 8x8 (of 112) | 105.40 | 104.52 | -1.00 | 2.7e-07 | 0/60 | 0/60 | 252.1 / 32.5 | 27.8 |
| 10x10 (of 180) | 164.40 | 162.57 | -2.00 | 3.6e-07 | 0/60 | 0/60 | 189.6 / 11.0 | 17.9 |

Best score: PT higher on 25 of 30 puzzles at 8x8 (ME higher on 1, 4 tied) and
24 of 30 at 10x10 (ME higher on 1, 5 tied). No run of either method reached
the planted optimum.

Sensitivity over `(k, delta)` (mean PT / mean ME basins per puzzle; ME has
fewer in every cell, one-sided p for ME > PT is 1 in every cell):

| 8x8 | delta 0.10 | delta 0.25 | delta 0.50 |
|---|---|---|---|
| k = 0 | 1.52 / 0.27 | 1.23 / 0.20 | 1.18 / 0.20 |
| k = 1 | 11.88 / 3.15 | 6.13 / 1.78 | 5.02 / 1.27 |
| k = 2 | 52.35 / 11.20 | 22.33 / 5.63 | 13.15 / 3.12 |
| k = 4 | 268.22 / 47.75 | 129.95 / 21.48 | 71.50 / 7.77 |

| 10x10 | delta 0.10 | delta 0.25 | delta 0.50 |
|---|---|---|---|
| k = 0 | 1.10 / 0.25 | 0.90 / 0.22 | 0.90 / 0.20 |
| k = 1 | 5.32 / 1.02 | 2.53 / 0.47 | 2.12 / 0.42 |
| k = 2 | 15.18 / 2.55 | 5.30 / 1.00 | 3.55 / 0.77 |
| k = 4 | 67.92 / 9.28 | 16.32 / 3.37 | 7.00 / 2.32 |

Exploratory (not pre-registered,
[`results/exploratory_own_best_k2_d0.25.json`](results/exploratory_own_best_k2_d0.25.json)):
because ME's best score is lower, the pooled `best_known` threshold could hide
ME basins just below it. Counting each run against its own best score instead
(`k = 2`, `delta = 0.25`), PT still has more: 8x8 32.3 vs 14.9 (PT more on 29 of
30 puzzles), 10x10 8.6 vs 4.7 (PT more on 28 of 30).

## Outcome against the pre-registered kill criterion

Kill sentence: "No more distinct basins than PT at equal compute on both
sizes." Pre-registered reading: killed unless MAP-Elites has significantly more
distinct basins on at least one size (one-sided exact Wilcoxon, Holm at 0.05).

- MAP-Elites significantly more on at least one size: **no** (p = 1 on both;
  the paired difference is negative on every puzzle that is not tied).
- MAP-Elites wins on both sizes: **no**.
- Best score: MAP-Elites is lower on both sizes (median paired difference -1 at
  8x8, -2 at 10x10), so it is not trading score for diversity either.

**Self-assessment: killed** for this implementation (this descriptor grid, this
selection rule, these two sizes and colour counts, this budget). The judge
decides.

## Observations and limits

- The archive stayed sparse: about 28 of 96 cells at 8x8 and 18 at 10x10. The
  corner-order axis (6 classes) changes only when a move touches corners, and
  the two share axes concentrate in a few bins near the best score, so the grid
  did not spread the search the way the hypothesis assumes.
- Restarting each burst from a stored elite discards the SA trajectory. PT's
  replicas keep walking, so they emit a new distinct burst-best board far more
  often (888 vs 198 deduplicated candidates per run at 8x8, 699 vs 98 at
  10x10, kept within 4 of each run's best).
- This tests one MAP-Elites variant. Other descriptors, a larger archive,
  curiosity-weighted cell selection, or mixing PT replicas with an archive were
  not tested and are not covered by this result.
- Two sizes, one colour count each, generated unframed boards. Nothing here
  speaks to 16x16 or to framed boards.
- Wall clock per run under 7 concurrent jobs was noisier than the solo probe;
  compute was matched in move evaluations exactly and in CPU seconds to within
  6%.

## Files

- `PREREG.md`: pre-registration (unchanged after commit `6d116b4`).
- `engine/`: `qd_basins` crate (`run` one PT or ME run, `basins` cluster and
  count), tests in `src/main.rs`.
- `scripts/run_grid.py`: calibration and main grid runner (standard library).
- `scripts/analyse.py`: pre-registered analysis, writes `summary.json`.
- `scripts/exploratory_own_best.py`: the exploratory own-best count above.
- `results/`: calibration, per-run rows, summary, exploratory output.
