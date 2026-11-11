# A3: strong backdoors on planted small boards

Agenda entry A3 (SAT backdoors on solvable small boards). The design, the
propagation rule and the kill rule were fixed in `PREREG.md` (commit `41359dd`,
pushed before any experiment ran). No amendments were needed, so there is no
`PREREG_AMENDMENTS.md`.

## What was run

- **Boards.** `generate_solved_framed_bc(n, colors, seed, true, border)` from
  `engine/src/generator.rs`, n = 6..10, seeds 1..30, two colour regimes per
  size: 300 planted instances. Piece i sits at cell i in rotation 0 in the
  planted solution.

  | n | border colours | interior A (primary) | interior B (secondary) | total colours A / B |
  |---|---|---|---|---|
  | 6 | 2 | 5 | 8 | 7 / 10 |
  | 7 | 2 | 6 | 10 | 8 / 12 |
  | 8 | 2 | 8 | 11 | 10 / 13 |
  | 9 | 3 | 9 | 13 | 12 / 16 |
  | 10 | 3 | 10 | 15 | 13 / 18 |

- **Propagation** (exactly as pre-registered): per-cell domains of distinct
  (piece, rotation) options, rim unary filter, edge arc consistency on shared
  edges, piece all-different with piece singles and hidden singles, iterated to
  a fixpoint. "Completes" means every cell is a singleton and the board is a
  valid complete tiling; planted versus other completion is recorded.
- **Backdoor k.** Smallest set of cells fixed to their planted placement after
  which propagation completes. Exact minimum by iterative deepening over cell
  subsets where it finishes under the node cap (3,000,000 propagations for
  n <= 7, 500,000 for n >= 8); otherwise k is a labelled upper bound from a
  deterministic greedy (forward greedy, removal pass, 32 seeded removal
  restarts, first-improvement swap local search). `exact_certified` in each
  jsonl row says which. When capped, the certified lower bound is the last
  size the exact search fully excluded.
- **Baseline.** 200 seeded random cell permutations per instance: `T50` is the
  smallest prefix length that completes for at least half of them, and
  `p_rand(k)` is the fraction of random subsets of size k that complete.
- **Kill rule** (from PREREG.md, primary regime A): fit `log kbar = a + b log N`
  over n = 6..10 (N = n^2), 2000 bootstrap resamples of puzzles within size
  (seed 20261111). KILL if the 95% CI upper end of b is >= 0.9, or if the 95% CI
  of `D = kbar(10)/100 - kbar(6)/36` is not entirely below 0. SURVIVE otherwise.

## Commands

From the repo root (needs `just`, cargo, python3):

```
just experiments sat-backdoors
```

This builds the crate, reruns all 300 instances into `results/rerun/`, reruns
the analysis and `cmp`s both files against the committed `results/`. Without
`just`, from `research/experiments/`:

```
cargo build --release --manifest-path sat-backdoors/Cargo.toml
sat-backdoors/target/release/sat_backdoors --threads 8 \
    --out sat-backdoors/results/rerun/results.jsonl \
    --timing sat-backdoors/results/rerun/timing.jsonl
python3 sat-backdoors/analyze.py sat-backdoors/results/rerun/results.jsonl \
    sat-backdoors/results/rerun/summary.json
```

Tests and lint: `cargo test --release` and `cargo clippy --release --all-targets`
in this directory (3 tests: full fix completes to planted, completion is
monotone in the fixed set on samples, exact <= greedy and greedy completes).

## Hardware and wall clock

Devin VM, 8 vCPU Intel Xeon, 31 GB RAM, no GPU. Full run (300 instances, 8
threads): 10 min 8 s wall clock, 78 min 41 s CPU. Per-instance wall time is in
`results/timing.jsonl` (not part of the deterministic output); slowest cell of
the design is regime A n=7 (mean 53 s, max 88 s per instance, mostly the exact
search hitting its cap).

Reproduction check: a fresh clone of the branch at commit `785439f`, running
`just experiments sat-backdoors`, took 9 min 59 s wall clock and both `cmp`
checks passed (results.jsonl and summary.json byte-identical).

## Files

- `PREREG.md`: pre-registration.
- `src/lib.rs`: model, propagation, greedy, local search, exact search, baseline.
- `src/main.rs`: runner and colour schedule.
- `analyze.py`: summary, fits, bootstrap, verdict (standard library only).
- `results/results.jsonl`: one row per instance (300).
  `backdoor_cells` is the greedy upper-bound witness (its length is `upper_bound`), and `completed_valid` / `completed_is_planted` check that witness. When `k_source` is `exact` and `k < upper_bound` (47 rows), the exact search certifies that a k-cell backdoor exists but does not store it.
- `results/summary.json`: per-size table, fits, verdict, post hoc sensitivity.
- `results/timing.jsonl`: per-instance wall time.

## Results

Means over 30 planted puzzles per row. `k` = exact minimum where certified,
greedy upper bound otherwise; `exact` = how many of the 30 are certified;
`gap` = (greedy upper bound minus exact) distribution on certified instances.

| regime | n | N | k mean (sd) | exact | greedy gap on certified | k/N | T50 | T50/N | k/T50 | p_rand(k) mean (max) |
|---|---|---|---|---|---|---|---|---|---|---|
| A | 6 | 36 | 4.90 (0.48) | 30 | 0:18, 1:12 (mean 0.40) | 0.136 | 10.87 | 0.302 | 0.45 | 0.000 (0.000) |
| A | 7 | 49 | 7.03 (0.81) | 6 | 0:3, 1:2, 2:1 (mean 0.67) | 0.144 | 14.53 | 0.297 | 0.48 | 0.0005 (0.005) |
| A | 8 | 64 | 7.33 (0.55) | 0 | n/a | 0.115 | 15.77 | 0.246 | 0.47 | 0.0003 (0.005) |
| A | 9 | 81 | 7.93 (0.69) | 0 | n/a | 0.098 | 16.50 | 0.204 | 0.48 | 0.000 (0.000) |
| A | 10 | 100 | 9.70 (0.65) | 0 | n/a | 0.097 | 22.10 | 0.221 | 0.44 | 0.000 (0.000) |
| B | 6 | 36 | 2.43 (0.50) | 30 | 0:29, 1:1 (mean 0.03) | 0.068 | 6.20 | 0.172 | 0.39 | 0.018 (0.095) |
| B | 7 | 49 | 2.67 (0.55) | 30 | 0:26, 1:4 (mean 0.13) | 0.054 | 7.10 | 0.145 | 0.38 | 0.005 (0.030) |
| B | 8 | 64 | 3.67 (0.71) | 28 | 0:17, 1:11 (mean 0.39) | 0.057 | 10.27 | 0.160 | 0.36 | 0.0008 (0.005) |
| B | 9 | 81 | 3.33 (0.48) | 30 | 0:16, 1:14 (mean 0.47) | 0.041 | 9.27 | 0.114 | 0.36 | 0.0005 (0.005) |
| B | 10 | 100 | 4.23 (0.57) | 21 | 0:19, 1:2 (mean 0.10) | 0.042 | 11.33 | 0.113 | 0.37 | 0.0008 (0.010) |

In all 300 instances propagation from the empty board fixes no cell (no root
singletons), and every backdoor completion was checked to be a valid board equal
to the planted solution (`completed_is_planted`). Random-subset completions are
not checked one by one; since the fixed cells are planted and the propagation
only removes options that no solution extending the fixed cells uses, the
planted option survives in every domain, so any all-singleton fixpoint is the
planted board.

Fits (bootstrap 95% CI, 2000 resamples):

| regime | b (backdoor k) | D (k/N, n=10 minus n=6) | b (random T50) | D (T50/N) | verdict |
|---|---|---|---|---|---|
| A (primary) | 0.589 [0.549, 0.626] | -0.039 [-0.045, -0.034] | 0.606 [0.545, 0.678] | -0.081 [-0.100, -0.059] | survive |
| B (secondary) | 0.523 [0.448, 0.603] | -0.025 [-0.030, -0.020] | 0.584 [0.465, 0.717] | -0.059 [-0.080, -0.036] | survive |

### Post hoc sensitivity (not pre-registered)

In regime A, k at n >= 8 is a greedy upper bound for every instance, while
n = 6 is exact. Overstating the large sizes can only raise the fitted exponent,
so on that side the primary fit is conservative. The other side is n = 7, where
24 of 30 values are upper bounds and sit on a weight below the mean of log N:
setting those 24 to their certified lower bound (6) gives b = 0.648, still
below 0.9. Setting every capped instance to its certified lower bound gives
b = -0.31 for regime A, which only shows that the lower bounds at n >= 8
(k >= 4) are too weak to constrain anything. In regime B, where 269 of 300
values are exact, the same substitution gives b = 0.467.

## Outcome against the pre-registered kill criterion

Kill criterion (verbatim): "Backdoor size grows roughly linearly in cell count."

Self-assessment: **survived** on the pre-registered primary statistic. In
regime A the 95% CI of the exponent is [0.549, 0.626], entirely below the 0.9
threshold, and the backdoor fraction k/N falls from 0.136 at 6x6 to 0.097 at
10x10 with a CI of D entirely below 0. The secondary regime B gives the same
verdict.

What this does and does not show:

- Five sizes (36 to 100 cells), one generator, two colour schedules, one
  propagation rule. The exponent describes this range; it is not a law and is
  not extrapolated to 16x16.
- Regime A at n >= 8 reports upper bounds, not minima. The greedy overshoots
  the exact minimum by 0.4 cells on average at n=6 and 0.67 at n=7 (6
  certified instances), so the true curve at large n is at or below the one
  reported.
- The sublinear growth is not specific to chosen backdoors. Random subsets
  need T50 cells with nearly the same exponent (0.61 vs 0.59 in regime A), and
  the backdoor stays a near-constant 0.44 to 0.48 of T50 across sizes. What the
  chosen set buys is a constant factor of about 2: random subsets of size k
  complete with probability at most 0.5% in regime A.
- The colour count grows with n in both schedules (it follows the E2-faithful
  ratio), so size and colour count are not separated here.
- The backdoor is found with the planted solution in hand. The second half of
  the hypothesis (a search over backdoor candidates beats cell-by-cell search)
  was not tested: a solver that does not know the solution must also guess the
  placement of each backdoor cell, and this run measures neither that cost nor
  any search comparison.
