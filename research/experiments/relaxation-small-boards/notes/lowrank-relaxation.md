# A low-rank form of the semidefinite lift, measured on toy boards (2026-11-13)

Draft, not independently checked. Every board here is a generated toy (the shared painter; frame colours and inner colours as stated; the first corner tile pinned at cell 0). Nothing here is Eternity II progress. Grades: measured (a row in `scripts/lowrank-relaxation/raw/`), derived, guessed.

## The reading first

**Kill line: not killed at 5x5, with one caveat on "smallest r".** Ranks 2 to 16 were cut (dropped at 21:46, before any of those units ran), so "the smallest r for which any start succeeds" is the smallest rank that was run. At that rank, on every one of the five 5x5 boards, at least half the random starts reached a feasible point (largest violation of any constraint at most 1e-3) with mass off solution placements under 5 percent: 10/10, 10/10, 10/10 and 8/8 at r = 32 on seeds 0 to 3, and 7/10 at r = 64 on seed 4, where r = 32 was not run. Five boards of five meet the bar (measured).

**But the low-rank solver does not reproduce the full solve's point.** The points it reaches are feasible mixtures of a few solutions (numerical rank 1 to 14, median 4 to 5), not the minimum of the l2 objective: their squared norm is about 21 against 5.2, 8.8 and 8.0 for the full solve on seeds 0, 1 and 2, and the L-infinity distance to the full solve's x is 0.75 to 0.79 (medians). They sit in the convex hull of the solutions to within about 0.002 (median hull distance). So the comparison "entry by entry" fails; the low-rank run behaves as a randomised search that lands on or near solutions, and the greedy rounding then reads a solution in 38 of 38 runs at r = 32 and 37 of 50 at r = 64 (measured, toys, 480 s per run).

**6x6 decides nothing.** Ten 6x6 runs ended at the 1,650 s limit, all "not converged" under the rule fixed before reading them: violation 2e-3 to 2e-2, rank still full, 49 to 78 percent of the mass off solution placements, 18 to 29 mismatches after rounding. None reached the regime where 5x5 runs turn near-integral. The 90-minute runs and the early-stop runs were stopped at 22:51 before producing a row. 7x7 and 8x8 were not run. Whether the low-rank relaxation gets trapped where annealing does (0 of 1,000 at 6x6) is not measured.

## What was built

`bm.py`. Same relaxation as `scripts/geometry-sdp-lift/lift.py` with its "l2" objective (minimise the squared norm of x): Z = [[1, x^T], [x, Y]] = V V^T with the border row fixed at e_1 (a rotation gauge), V of shape P x r, so x_p is the first coordinate of row p and Y is the Gram matrix of the rows. Constraints, all of lift.py's: cell and tile sums of x equal 1, Y_pp = x_p, the row sums over every other cell and tile, Y_pq = 0 on every conflict pair (same cell, same tile, or adjacent with mismatched facing sides, as an index list built from structure), x >= 0, and Y_pq >= 0 on every free pair.

Two exact simplifications (derived; both leave the feasible set unchanged):

1. Y_pp = x_p says the row lies on the sphere of radius 1/2 centred at e_1/2, so v_p = (e_1 + u_p)/2 with u_p a unit vector, which is the 0/1 to plus-minus-1 change of variable. The diagonal constraint and x >= 0 then hold by construction; the search space is a product of P unit spheres.
2. The row sums are implied: the squared norm of (sum of rows in cell c minus e_1) equals the sum of x over c minus 1 once same-cell pairs are zero and the diagonal holds, so the cell sum of rows equals e_1 and every row sum follows; the same for tiles. They are not penalised, but every reported violation evaluates them explicitly (`max_violation`), with all the other constraints. The cell and tile sums are penalised in this vector form (N r and T r residuals).

Penalised: cell and tile sums, conflict zeros, entrywise nonnegativity. Method: augmented Lagrangian, inequalities in the shifted max(0, .) form, inner solve by L-BFGS on the sphere coordinates (300 iterations per outer step), penalty from 10 times 1.3 whenever the violation did not halve, capped at 1e5. The nonnegativity term is evaluated in row blocks over all pairs and the conflict pairs are subtracted back from the index list, so no P x P matrix or mask is formed. Checks (`check.py`, `raw/check.json`): gradient against finite differences (relative error 1e-6), every solution's rank-one point has violation exactly 0, a random board's rank-one point violates a zero, the free-pair count reproduces lift.py's 86,800, and the index-list exclusion agrees with a structure mask. Settings were tuned on 5x5 seeds 0 and 1 before stage 1 (`raw/tune_records.txt`); scalar sums and fixed penalties stalled at violations of 0.02 to 0.15.

**Cost per evaluation** (P placements, r rank, K conflict pairs, A stored nonnegativity multipliers, m = 10 L-BFGS pairs; derived from the code): flops about 4 P^2 r + 3 P^2 + 4 K r; memory about 8 [(2m + 4) P r + 4 K + 2 A] bytes plus three 16 MB row blocks. The P^2 work comes only from the entrywise nonnegativity of the full relaxation. A is not small in practice: 35 to 77 percent of the free pairs carry a multiplier (measured at 5x5 and 6x6).

## Stage 1, 5x5 (frame 3, inner 4), 480 s per random start

Three-way classing as in stage 2: solved (greedy or Hungarian rounding is a solution), converged and wrong (not solved, violation at most 8.9e-4, the largest among the 5x5 runs that were successes and rounded to a solution), not converged (the rest). The 5x5 rows keep only the last three outer steps, so "stalled" cannot be tested there; every 5x5 run stopped at its time limit.

| board (solutions) | r | runs | feasible <= 1e-3 | success (kill line) | solved | converged and wrong | not converged | median mass off | median hull dist. | median L-inf to full solve | median norm^2 (full) | median evals | peak MB |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| s0 (36) | 32 | 10 | 10 | 10 | 10 | 0 | 0 | 0.0015 | 0.0018 | 0.84 | 16.1 (5.16) | 47,072 | 145 |
| s0 (36) | 64 | 10 | 9 | 9 | 9 | 1 | 0 | 0.0005 | 0.0004 | 0.83 | 19.8 (5.16) | 28,704 | 180 |
| s1 (6) | 32 | 10 | 10 | 10 | 10 | 0 | 0 | 0.0065 | 0.0058 | 0.74 | 23.9 (8.79) | 38,951 | 143 |
| s1 (6) | 64 | 10 | 5 | 5 | 8 | 0 | 2 | 0.055 | 0.039 | 0.71 | 19.4 (8.79) | 21,674 | 169 |
| s2 (8) | 32 | 10 | 10 | 10 | 10 | 0 | 0 | 0.0044 | 0.0039 | 0.79 | 22.9 (7.98) | 47,197 | 148 |
| s2 (8) | 64 | 10 | 6 | 6 | 7 | 0 | 3 | 0.025 | 0.018 | 0.74 | 21.1 (7.98) | 24,712 | 178 |
| s3 (403) | 32 | 8 | 8 | 8 | 8 | 0 | 0 | 0.0001 | 0.0002 | n/a | 17.4 | 39,418 | 150 |
| s3 (403) | 64 | 10 | 10 | 10 | 5 | 5 | 0 | 0.0003 | 0.0003 | n/a | 12.7 | 22,130 | 184 |
| s4 (2) | 64 | 10 | 7 | 7 | 8 | 0 | 2 | 0.013 | 0.016 | n/a | 23.7 | 22,469 | 180 |

Totals: r = 32, 38 runs, 38 solved; r = 64, 50 runs, 37 solved, 6 converged and wrong, 7 not converged (measured). Full solve for reference (lift study, SCS, one thread): 400 to 1,100 s and about 0.5 GB per board, mass off 0.09, 0.000003 and 0.002 percent per cell on seeds 0 to 2.

The two failure kinds at r = 64 (measured): seven runs stayed at full rank 64 with violation 3e-3 to 6e-3 and 44 to 71 percent of the mass off solutions (not converged; the same state every 6x6 run was in at its limit); six were feasible mixtures of 3 to 7 solutions on which rounding mixes solutions (five on seed 3 with 403 solutions, one on seed 0). r = 32 did better than r = 64 at equal wall time; r = 64 costs about twice as much per evaluation, so it completes fewer outer steps.

Plain annealing solved 580 of 1,000 runs at 5x5 (`geometry-isolation.md`); here 75 of 88 low-rank runs round to a solution. The budgets are not comparable (2e6 annealing steps against 20,000 to 47,000 evaluations of a P^2 r gradient), so this says only that the low-rank form is not worse at 5x5.

**Controls** (measured). Positive: 12 runs started at a true solution's rank-one point (plus 1e-3 noise), r = 4 and 16, seeds 0 and 1: all 12 ended feasible (violation 8e-6 to 8e-5). Ten rounded to a solution. The other two (seed 1, r = 16, starts 0 and 1) did what the objective asks: minimising the squared norm of x pulls away from a single solution toward a mixture, and they ended at feasible mixtures of 2 and 3 solutions (rank 2 and 3, squared norm 14.0 and 10.9, hull distance at most 2e-4, mass off at most 0.02 percent). Seed 1's solutions differ in 9 to 10 cells, and a cell-by-cell rounding of such a mixture takes parts of different solutions (13 mismatches). That is a rounding failure on a correct point, not a feasibility failure. Negative: one side of the first inner tile recoloured to another inner colour (that colour then appears an odd number of times, so no board matches every edge; enumeration confirms 0 solutions). Swapping two sides of one tile, as first planned, left these few-colour boards solvable (3 to 36 solutions for every inner tile of seed 0), so it was not used. All 12 runs stayed infeasible (violation 0.026 to 0.23) and none rounded to a solution.

## Stage 2, 6x6 (frame 4, inner 5), 1,650 s per random start

Rule (fixed before reading): solved; converged and wrong (violation at most 8.9e-4 and not solved, or stalled: penalty at its cap through the last third of the run's wall time and less than 20 percent progress in the smallest violation over that third); not converged (still improving at the limit).

| board (solutions) | r | runs | host | solved | converged and wrong | not converged | final violation (median) | mass off (median) | greedy mismatches (median) | evals (median) |
|---|---|---|---|---|---|---|---|---|---|---|
| s0 (7) | 32 | 1 | x86 | 0 | 0 | 1 | 0.014 | 0.70 | 18 | 6,083 |
| s0 (7) | 64 | 1 | m1 | 0 | 0 | 1 | 0.0064 | 0.70 | 25 | 13,444 |
| s1 (4) | 32 | 1 | x86 | 0 | 0 | 1 | 0.014 | 0.71 | 26 | 6,077 |
| s1 (4) | 64 | 1 | m1 | 0 | 0 | 1 | 0.0033 | 0.64 | 24 | 13,109 |
| s2 (14) | 32 | 1 | x86 | 0 | 0 | 1 | 0.021 | 0.49 | 27 | 6,074 |
| s3 (10) | 64 | 3 | this Mac | 0 | 0 | 3 | 0.0053 | 0.60 | 27 | 15,343 |
| s4 (3) | 64 | 2 | this Mac | 0 | 0 | 2 | 0.0098 | 0.76 | 29 | 15,384 |

All ten are "not converged" and decide nothing (measured). Why (measured from the violation histories kept in each row): a 6x6 evaluation costs about 5 times a 5x5 one, so a run completes 41 to 48 outer steps in 1,650 s on this Mac and m1, and the penalty reaches its cap only in the last 6 to 8 of them; on x86 (loaded, 10 processes) it completes 19 and the penalty never passes 865. The 5x5 runs left the full-rank interior and turned near-integral shortly after the penalty passed about 1e4 (outer steps 40 to 50). Every 6x6 run was still in the interior: rank 32 or 64, squared norm 5.5 to 7.2, violation still moving. The rows are in `raw/stage2/` (this Mac), `raw/stage2_m1/`, `raw/stage2_x86/`, merged in `raw/stage2_30min/all_rows.jsonl`, classed in `raw/stage2_30min/classified.txt`.

Rounding: greedy, largest entry first. Walk the placements in decreasing x (ties by index) and take a placement when its cell is still empty and its tile still unused; every (cell, tile) pair of the same rim class has a placement, so this always fills the board. Probe B's Hungarian rounding (`measure.round_board`) is also reported; a run counts as solved if either is a solution.

## The early stop (round and stop at the first valid board)

Added at 22:48 after the Rust study's finding: every 2 outer steps, round the current x (greedy, then Hungarian) and stop at the first rounding that is a solution. One test run only (5x5 seed 0, r = 32, m1, `raw/earlytest/`): solved at outer step 20, after 32 s, while the violation was still 4.6e-3 and 10 percent of the mass was off solution placements (measured). So at 5x5 the rounding reads a solution long before the point is feasible, about 15 times sooner than the 480 s the stage 1 runs spent. The 6x6 runs with the early stop were launched at 22:49 and stopped at 22:51 with the rest; no 6x6 row exists. That a solution appears this early at 5x5 also means the 480 s stage 1 timings overstate the time to a solution there.

## Cost with size

Measured, one evaluation (value and gradient), x86 under load, median of 20 (`raw/costscale/cost.jsonl`):

| board | P | K | free pairs F | stored multipliers A | ms per eval r = 32 / 64 | peak MB r = 32 / 64 |
|---|---|---|---|---|---|---|
| 5x5 (3, 4) | 478 | 27,203 | 86,800 | 33,943 | 24 / 22 | 111 / 122 |
| 6x6 (4, 5) | 1,290 | 154,766 | 676,639 | 301,571 | 70 / 116 | 232 / 308 |
| 7x7 (4, 7) | 2,910 | 619,434 | 3,613,161 | 1,691,059 | 303 / 499 | 597 / 897 |
| 8x8 (4, 9) | 5,770 | 1,908,391 | 14,735,174 | 7,061,612 | 1,302 / 1,817 | 1,432 / 2,304 |

On this Mac, unloaded, the same evaluation took 2.6 ms at 5x5 and 14 ms at 6x6 (r = 16). Time grows close to P^2, as the formula says. The 8x8 board is frame 4, inner 9: on six seeds per inner count, inner 9 gives 1 or 2 solutions with the costliest search (5e8 to 3e9 nodes), inner 10 and 11 give 1 to 3 with less, and inner 8 did not finish enumerating in 900 s; seed 0 has 1 solution (`raw/enum8_progress.txt`, `boards/*.enum.json`). Only its evaluation cost was measured.

Derived for the 16x16, from the same formulas with uniform colours and the 5x5 share of stored multipliers (`scale.py`, `raw/scale.json`): P = 156,810, K about 3.3e8, F about 1.2e10, so 1.7e12 flops per evaluation at r = 16, and about 159 GB of memory, almost all of it the nonnegativity multipliers. Without storing them (a pure penalty on nonnegativity) and with the conflicts generated from structure, the state is under 1 GB, but the 1.7e12 flops per evaluation remain, and the number of evaluations needed is unknown: it was not measured past 5x5. No GPU note is given: stage 3 was conditional on stage 2 and did not run.

## Deviations and what did not fit

- Ranks 2 to 16 were not run (dropped at 21:46 to free capacity for 6x6), and the r = 32 units for seed 4 and two starts of seed 3 were cancelled in the queue. The kill line reading above uses the smallest rank run.
- No 5x5 run reached the stopping test (violation under 1e-5 with a settled objective); every one stopped at 480 s. The feasibility threshold 1e-3 was fixed after tuning and before stage 1.
- Same objective, not the same point: the reported x is wherever the run was at its limit, which is far from the l2 optimum. A solver that actually minimised the objective would need more than 480 s at 5x5; I did not measure how much more.
- Ground truth: the lift study re-ran its seed 0 solve during this work; I used its later files for seeds 0 to 2 (`raw/copied_files_sha256.txt`; the first seed 0 copy is kept as `raw/superseded_*`). Seeds 3 and 4 have no full solve.
- Stage 1 was restarted once (about 15 minutes lost) to save x per start. The remote tuning JSON files were deleted by an `rsync --delete`; their printed summaries are in `raw/tune_records.txt`. A rsync of the local PROGRESS file was overwritten by the remote one; per-unit logs and rows are complete.
- For about three minutes m1 ran 8 processes (limit 7) because the slot counter's process pattern missed `python3`; fixed. x86 ran 10 processes at 2 to 4 times the per-evaluation time of the Macs; its rows are labelled by host and its 6x6 r = 32 runs got the fewest steps of all.
- 7x7, the 6x6 controls, the 90-minute runs and stage 3 were not run (decided at 22:51).

## Files

In `scripts/lowrank-relaxation/`: `bm.py` (solver, rounding), `check.py` (code checks), `run_unit.py` (one board, one rank, a range of starts; one JSON row per start, x saved per start), `run_jobs.sh` and `run_pool.sh` (detached runners with PROGRESS files), `jobs_*.txt` (the job lists as run), `tune.py`, `summarise.py`, `stage1_reading.py`, `classify.py`, `costscale.py`, `scale.py`, `construct.py` (the painter's own solution without search, for 8x8; not used), `export_boards.py` and `iso.c` (a copy of the isolation study's enumerator with the array limit raised from 49 to 64 cells), and verbatim copies of `board.py`, `measure.py`, `common.py`. Outputs: `raw/stage1/` with `raw/stage1_summary.txt`, `raw/stage1_reading.txt`, `raw/stage1_classified.txt`; the 6x6 rows as above; `raw/costscale/`, `raw/scale.json`, `raw/earlytest/`, `raw/x86check/` (x86 against this Mac on the same 5x5 unit: same counts and checks, trajectories close but not bitwise equal). Rerun a unit: `uv run --with numpy --with scipy python -I -B run_unit.py 5 3 4 0 32 0 1 out.jsonl 480 random`.
