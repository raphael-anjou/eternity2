# Relaxation of edge-matching on small toy boards (negative result)

Does a semidefinite relaxation of the placement problem, or a low-rank form of it, give a cheaper route to a full solution than search? On generated TOY boards of 4x4 to 6x6 (7x7 for one annealing control): no. Nothing here is Eternity II progress. All boards come from the project's painter (frame colours 3 or 4, inner colours 4, 5 or 7, seeds 0 to 4).

## What is established, and its status

| Result | Status |
|---|---|
| Convex lift on 5x5 (3,4) toys: certified clean (no mass outside the solutions) on seeds 1, 2, 4 (2 to 8 solutions); seed 0 undecided; seed 3 (403 solutions) not exact (about 0.042 per cell on ten placements in no solution, violation 1.5e-9) | Independently re-checked. Seed 3: one solver family only |
| Memory at 16x16: one dense copy 183 GiB; factor plus nonnegativity multipliers about 119 GiB; convex splitting 916 GiB | Independently re-checked |
| Low-rank form: a penalty-guided randomised search with an exact acceptance test, not a relaxation solver. 200 of 200 runs solved on 15 5x5 boards, about 5,200 evaluations per solution; 46 of 50 with nonnegativity removed, so it says nothing about the lift | Independently re-checked |
| Restart annealing against the low-rank search: 8 to 35 times cheaper per solution at 5x5, 3.6 to 63 times at 6x6; exhaustive search takes at most 0.1 s (5x5) and 0.3 to 0.6 s (6x6); annealing found nothing in 210 runs at 7x7 (rate below 26 percent, not shown zero) | 5x5 independently re-checked; 6x6: one pass, intervals overlap on three boards |
| Growth 5x5 to 6x6: 62 times more evaluations per solution, a two-point exponent of 4.2 in P (3.4 to 5.1) | Independently re-checked. Two sizes cannot separate a power from an exponential |
| Full 6x6 batch: 22 of 50 starts solved; 460,416 evaluations per solution against 5,083 at 5x5; exponent 4.54 (bootstrap 3.97 to 5.31) on two sizes | Not independently checked |
| Nonnegative factor: 0 of 50 starts at 5x5 in each of four variants (annealing 26 of 50) | Not independently checked (derivation re-checked) |
| Six of eight smaller lifts leak or are vacuous at 5x5 | Not independently checked |
| Exact structured sweep still needs about 4.8e12 flops and 100 GB per sweep at 16x16 | Not independently checked (cost arithmetic re-checked) |
| 6x6 convex leak runs (three boards) | Not converged: no 6x6 reading exists |

## Statements withdrawn or corrected

- "The lift is exact at 5x5" and "first certificate of exactness": not exact on seed 3; the certificate covers one board.
- 60 / 20 / 2 percent odds of exactness at 6x6, 8x8, 16x16: an underived estimate made before a board was found inexact.
- "30 GiB at the guaranteed rank" (16x16) and "9.3 GB, 10 hours per start": withdrawn; see the memory row (910 GB as coded).
- "Annealing 58 percent, low-rank 100 percent": efforts were unequal.
- "About 145 times faster" than NumPy at 5x5: unproven; the later tuning gain is mostly machine load (1.3 times per evaluation). "50 of 50": 49 of 50.
- "70 to 190 s per start" at 6x6 and "exponent 2.5 or worse": low by 15 to 40 times; 2.5 is a floor.
- Iteration growth between P^1.6 and P^3.1, "facial reduction needed": unproven or contradicted.
- A shorter restart interval at 6x6: withdrawn (median solve at step 605 of 1,000).

## Map

`notes/` holds the fifteen studies (two are the `*-recheck.md` independent re-checks). Where a note and the first table differ, the table governs. `scripts/<study>/` holds each study's code, `results/<study>/` its raw output, `lowrank-rust/` the Rust crate. Scripts keep their original relative paths; `scripts/<study>/raw` (and `results`, `logs`, `raw_m1`) are symlinks to `results/<study>/...`, so re-running a study script writes into `results/` (copy the folder first). `scripts/far-ideas-geometry/` (the painter) is shared.

Not copied: `.npy` checkpoints, binaries and `target/`, the generated `boards/` of both crates (66 MB and 84 MB, measured on the source), the isolation study's raw annealing rows (input of `scripts/recheck-lowrank/02_anneal_cost.py`), nested duplicates. Regenerate boards with `py/export.py 5 6` inside either crate.

## Rerun the small pieces

`RECIPE.txt` holds a `just experiments` recipe running all three (under two minutes each):

```
uv run --with numpy --with scipy python -I scripts/convex-6x6/admm.py 4 3 4 0 l2 300 20 OUTDIR   # convex lift, 4x4
cd lowrank-rust && uv run --with numpy --with scipy python -I py/export.py 5 16 && cargo build --release   # low-rank, 5x5
cc -O2 -o iso_t scripts/anneal-vs-relaxation/iso_t.c -lm                                               # annealer
```

Machines: "x86" (`check5_x86`) is machine-b, a 14-core low-power x86 laptop; "m1" an 8-core Apple M1; "m4", "Mac" a 10-core Apple M4 laptop (32 GB).

## Not exactly reproducible

Annealing and low-rank runs are stochastic: counts, not trajectories, should reproduce. The 6x6 batches (30 CPU minutes per start) and convex 5x5 solves (400 to 1,100 s) are not in the smoke run.

## Where each number comes from

Files are in `notes/` unless under `results/`; numbers are lines. "convex", "lowrank": the re-check notes.

| Number | File and line or key |
|---|---|
| Seeds 1, 2, 4 clean; seed 3: 403 solutions, ten placements, 0.042, 1.5e-9 | convex 64-71; `results/recheck-convex/raw/polish_s3.jsonl` (`mass_off_per_cell`) |
| 183, 119, 916 GiB; 30 GiB; 60 / 20 / 2 percent; P^1.6 to P^3.1 | convex 79-83, 76-78, 44; `results/recheck-convex/raw/recompute_figures.txt` 11-18 |
| 200 of 200, 15 boards, 5,200 evaluations | lowrank 55-59 |
| 46 of 50 | convex 60; lowrank 208 |
| 62 times, 4.2 (3.4 to 5.1); 8 to 35 times, 0.1 s (5x5) | lowrank 73-76, 289, 233-237; `results/recheck-lowrank/raw/02_anneal_cost_counts.txt` |
| 3.6 to 63 times (relaxation over M4-equivalent annealing s/solution: 1,145/320, 2,615/41.5); 0.3 to 0.6 s (6x6) | `anneal-vs-relaxation.md` 56-60, 66-67; `results/anneal-vs-relaxation/analysis.txt` |
| 210 runs at 7x7; below 26 percent = 1 - 0.05^(1/10), ten 3e9-step runs, none solved  | `anneal-vs-relaxation.md` 73-78; `analysis.txt` |
| 22 of 50, 460,416, 5,083, 4.54 (3.97 to 5.31); 30 CPU minutes; step 605 | `results/growth-exponent/results/growth.txt` 9, 20, 44-45; `growth-exponent.md` 7, 10, 44 |
| 6x6 convex not converged; 400 to 1,100 s | `convex-6x6.md` 7; `geometry-sdp-lift.md` 113 |
| 0 of 50, 26 of 50; six of eight lifts; 4.8e12 flops, 100 GB | `theory-b-nonnegative-factor.md` 7, 119; `theory-d-formulation-decoding.md` 9; `theory-a-structured-sweep.md` 19 |
| 9.3 GB, 910 GB, 10 hours, 70 to 190 s, 15 to 40 times, 2.5 floor | lowrank 29-32, 78-81, 373 |
| 58 percent; 145 times; 1.3 times; 49 of 50 | lowrank 27, 22-23, 114; `results/recheck-lowrank/raw/loadcheck.txt` |
