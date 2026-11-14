# Are solutions isolated? Level sets of near-solutions on toy boards (2026-11-13)

Draft, not independently checked. Every board here is a generated toy (the shared painter of gap 6a, 6b and the far-ideas probes; rim colours `cf`, inner colours `ci`, the first corner tile pinned at cell 0). Nothing here is Eternity II progress. Grades: measured (a run in `geometry-isolation/raw/`), derived, guessed.

## The reading first

**By the readings fixed in the plan: "in between".** At 6x6 the M1 component of a solution at k <= 2 has fewer than 100 boards on 21 of 38 solutions (most, pooled; three of five boards), which meets the first half of "isolated". The fitness-distance correlation over all sampled boards is 0.38 to 0.53 at 6x6, above the 0.2 line, which fails the second half. "Funnelled" fails outright: annealing solved 0 of 1,000 runs at 6x6.

**What the numbers say beyond the letter of the rule (measured, toys).** The 0.4 to 0.5 correlation comes from the gap between random boards (many mismatches, far from every solution) and annealed boards (few mismatches, slightly less far). Among boards with 8 or fewer mismatches it is 0.06 to 0.14 at 6x6 and -0.03 to 0.05 at 7x7. Near the bottom of the landscape, a lower mismatch count says almost nothing about closeness to a solution. The decisive measurement is the trap check (added to the plan, stated here): from every annealed board ending with 1 to 4 mismatches, a breadth-first search under M1 inside the level set {mismatches <= 4} asks whether any solution is reachable. At 5x5, 405 of 419 such boards reach a solution. At 6x6, 22 of 734 do. At 7x7 the annealer reached 4 mismatches in only 3 of 400 runs, and none of the 3 reaches a solution. The trend with size runs from connected at 5x5 to mostly disconnected at 6x6, inside the same mismatch band.

Plainly: at 5x5 one can "move through the space" to a solution, and the annealer does, in 58 percent of runs. From 6x6 on, the low-mismatch boards an annealer finds sit in small pockets cut off from every solution at k <= 4, and nearly as far from the nearest solution as a random board. Nothing in these toys points to a road that widens with size.

## Setup

Boards (measured counts, matching `far-ideas-geometry/scan*.out` and probe B where they overlap): 5x5 (cf 3, ci 4) seeds 0 to 4 with 36, 6, 8, 403, 2 solutions; 6x6 (cf 4, ci 5) seeds 0 to 4 with 7, 4, 14, 10, 3 solutions; 7x7 (cf 4, ci 7) seeds 0 and 1 with 2 and 3 solutions. "Solutions" means with the pin, so the four global rotations are removed. Inner edges: 40, 60, 84.

Board space: every cell holds a tile of its class; rim tiles sit in their one grey-out rotation, so the rim is always grey; inner tiles take any rotation. Mismatches are unmatched inner edges. Hamming distance counts cells whose (tile, rotation) differ.

Moves, all inside one class, pin excluded. M1: rotate one inner tile, or swap two tiles with each landing in any legal rotation. M2: M1 plus 3-cycles. M3: M1 plus any permutation of the four tiles of an inner 2x2 block in any rotations (this contains the rigid block rotation).

A derived fact that shapes the tables: **no board has exactly one mismatch.** The painter gives each inner edge one colour, so every colour appears an even number of times on the half-edges. On mismatched edges, each colour must therefore appear an even number of times, and a single mismatch makes two colours odd. Three mismatches are possible (a triangle a-b, b-c, c-a). The exact counts confirm it (j = 1 is 0 on every board counted), so k = 1 gives the same components as k = 0.

## Results per size (measured, toys)

Columns: (a) component of a solution under M1 at k = 2, 3, 4 (median boards over solutions), and how many solutions sit in a component with another solution, or reach a board more than N/2 cells away; (b) the barrier, the smallest k at which two different solutions share an M1 component (exact by BFS where no component was capped), and the beam upper bound on the pairwise M1 barrier (median over pairs); (c) annealer success, distance from near-solution endpoints to the nearest solution against random boards, the trap check, and the correlation over all sampled boards and over those with 8 or fewer mismatches; (d) exact counts.

### 5x5 (cf 3, ci 4), five seeds

| seed, sols | comp. median k=2 / 3 / 4 | k=2: with another sol; reach > N/2 | barrier: first pair / all (M1) | beam UB median | anneal solved | near-sol dist median vs random | trap: reach a sol | corr. all / <= 8 mm |
|---|---|---|---|---|---|---|---|---|
| 0, 36 | 185 / 6,886 / 1,482,993 | 22/36; 31/36 | 2 / 4 | 5.5 | 168/200 | 15 vs 19 | 32/32 | 0.62 / 0.45 |
| 1, 6 | 72 / 1,718 / 583,461 | 2/6; 2/6 | 2 / 4 | 5 | 83/200 | 18 vs 20 | 112/117 | 0.36 / 0.16 |
| 2, 8 | 110 / 843 / 1,034,513 | 4/8; 5/8 | 2 / 4 | 6 | 85/200 | 17 vs 20 | 109/115 | 0.50 / 0.25 |
| 3, 403 | 79,542 / 1,270,485 / capped | 396/403; 399/403 | 2 / 3 | 5 | 199/200 | 9 vs 17 | 1/1 | 0.75 / 0.82 |
| 4, 2 | 44 / 518 / 278,054 | 0/2; 0/2 | 4 / 4 | 7 | 45/200 | 20 vs 21 | 151/154 | 0.30 / 0.12 |

At k = 4 every 5x5 board is one component holding all solutions. On seed 0 that component contains all 8,125 boards with 2 mismatches and 68,740 of the 69,063 boards with 3 mismatches: at 5x5 the level set {<= 4} is essentially connected. M2 and M3 change little: medians at k = 2 grow by at most 2x (M3 on seed 1), barriers move by at most one step (M2 joins all solutions at k = 3 on seed 0).

(d) Exact counts, row-scan DFS tolerating 3 mismatches: seed 0: 36 solutions, 0, 8,125, 69,063 boards at 1, 2, 3 mismatches; seed 1: 6, 0, 2,202, 26,611; seed 2: 8, 0, 3,841, 48,826; seed 4: 2, 0, 899, 13,772. Per solution: 226 to 480 boards at 2 mismatches and 1,900 to 6,900 at 3. Seed 3 did not finish in 300 s. At 4 mismatches only a lower bound exists: at least 185,442 on seed 0 (the K = 4 count stopped at 2e9 nodes).

### 6x6 (cf 4, ci 5), five seeds

| seed, sols | comp. median k=2 / 3 / 4 | k=2: with another sol; reach > N/2 | barrier: first pair / all (M1) | beam UB median | anneal solved | near-sol dist median vs random | trap: reach a sol | corr. all / <= 8 mm |
|---|---|---|---|---|---|---|---|---|
| 0, 7 | 274 / 2,024 / 478,544 | 6/7; 0/7 | 2 / 4 | 9 | 0/200 | 28 vs 31 | 12/158 | 0.48 / 0.06 |
| 1, 4 | 48 / 142 / 5,302 | 0/4; 0/4 | > 4 (exact) / > 4 | 8.5 | 0/200 | 28 vs 31 | 0/125 | 0.46 / 0.14 |
| 2, 14 | 124 / 396 / capped on 8 | 6/14; 2/14 | 2 / not settled (capped) | 9 | 0/200 | 27 vs 30 | 8/166 | 0.53 / 0.11 |
| 3, 10 | 56 / 179 / 10,395 | 4/10; 0/10 | 2 / > 4 (exact) | 10 | 0/200 | 28 vs 31 | 2/151 | 0.40 / 0.09 |
| 4, 3 | 51 / 157 / 6,152 | 0/3; 0/3 | 4 / > 4 (exact) | 10 | 0/200 | 29 vs 32 | 0/134 | 0.38 / 0.06 |

Solutions in the same component at k = 2 are the small families that probe A described (pairs at 4 to 12 cells). Excluding those families, the first barrier between unrelated solutions is 4 or more. On seed 1 (four solutions, no family) no two solutions connect at k <= 4 under any of M1, M2, M3, exactly (no component capped), and the beam bound is 7 to 10. At k <= 2 the component almost never reaches beyond N/2 (2 of 38 solutions); at k = 4 it does on every solution, but on seeds 1, 3 and 4, 9 of 17 solutions sit in a k = 4 component with no other solution in it (components of 2,886 to 174,722 boards on those seeds). The larger move sets do not open the landscape: on seeds 0 and 1, M2 and M3 leave every barrier unchanged and grow the k = 4 components by at most 3.4x (M3 seed 0: 1,638,319 against 478,544).

Of the annealed boards with at most 4 mismatches, 22 of 734 connect to a solution inside the level set. The other 712 sit in components of median size 4 to 12 boards (largest 152,540; 10 searches stopped at the 200,000 cap on seed 2 and count as not reaching). Their distance to the nearest solution is 27 to 29 cells of 36, against 30 to 32 for random boards.

### 7x7 (cf 4, ci 7), two seeds, M1 only

| seed, sols | comp. median k=2 / 3 / 4 | k=2: with another sol; reach > N/2 | barrier: first pair / all | beam UB median | anneal solved | near-sol dist median vs random | trap: reach a sol | corr. all / <= 8 mm |
|---|---|---|---|---|---|---|---|---|
| 0, 2 | 120 / 299 / 5,453 | 2/2; 0/2 | 2 / 2 | 2 | 0/200 | 44 vs 46 | 0/2 | 0.16 / 0.05 |
| 1, 3 | 131 / 326 / 16,374 | 2/3; 0/3 | 2 / 4 | 4 | 0/200 | 43 vs 45 | 0/1 | 0.27 / -0.03 |

Both 7x7 boards have their solutions in one family (pair Hamming distances 4 and 14 of 49), so the low barriers here are within a family, not between unrelated solutions. The annealer's final boards have a median of 8 mismatches, against 4 at 6x6 and 0 to 2 at 5x5. The budget is the same 2e6 steps at every size, so this size step is partly a budget effect (see the cuts below).

### Figure

`geometry-isolation/fig_mismatch_vs_distance.png`: for each size, the mean and the 10th to 90th percentile of the distance to the nearest solution against the mismatch count, over annealer snapshots and 1,000 random boards per board, with the random-board mean dashed. At 5x5 the curve drops toward 0 below about 4 mismatches. At 6x6 it sits about 3 cells under the random line down to 2 mismatches; at 7x7 about 1 to 2 cells under it. The funnel is visible only at 5x5.

## The trend with board size, and what it says about 16x16

| | 5x5 | 6x6 | 7x7 |
|---|---|---|---|
| annealer solved (fixed budget) | 580/1,000 | 0/1,000 | 0/400 |
| annealed boards at 1 to 4 mm that reach a solution inside {<= 4} | 405/419 (97%) | 22/734 (3%) | 0/3 |
| near-solution distance as a share of the random-board distance (medians) | 0.53 to 0.95 (0.79 to 0.95 without seed 3, 403 solutions) | 0.90 to 0.91 | 0.96 |
| correlation, boards with <= 8 mm | 0.12 to 0.82 | 0.06 to 0.14 | -0.03 to 0.05 |
| correlation, all sampled boards | 0.30 to 0.75 | 0.38 to 0.53 | 0.16 to 0.27 |
| k = 4 component holds all solutions | 5 of 5 boards | 1 of 5 (one more not settled) | 2 of 2 (one family each) |

Every row moves the same way: the landscape near the bottom decorrelates from the solutions as the board grows. The 5x5 boards are small enough that the 4-mismatch level set is one connected piece. From 6x6 on it breaks into many pockets, most of them without a solution. Derived, not measured: the count of near-solutions per solution grows with the number of inner edges (at 5x5 already 226 to 480 boards at 2 mismatches per solution), while the solutions' own components at k <= 3 stay at a few hundred boards. A walk that lands at low mismatch count therefore lands, with probability tending to 1, in a pocket that is not a solution's. This agrees with path 4's empty band and with idea C's prediction in `far-ideas-geometry.md`. It also gives the A10 "two-solution measurement on 6x6 and 7x7" its first answer: at 6x6, unrelated solutions are separated by a barrier of 4 or more (exact on three boards); at 7x7 no board had two unrelated solutions.

Guessed extrapolation: on the 16x16 with about one solution under the five clues, the solution's basin at bounded k is a pocket of a few hundred to a few thousand boards among a near-solution population that is astronomically larger, and fitness says nothing about distance below about 10 mismatches. "Moving through the space" toward the solution is not supported by any measurement here beyond 5x5.

## Cuts and departures from the plan, stated

- **Basins (a), (b):** M1 on all 12 boards. M2 and M3 on all five 5x5 boards and on 6x6 seeds 0 and 1 only (coordinator's cut). The 6x6 seed 2 M2 run was stopped mid-way and its partial output is kept apart (`raw/cut_partial_basin_6_4_5_2_m2.jsonl`), not used. The 7x7 M1 basins finished (cap 1e6, not hit). Explored-set cap 2e6 boards; it was hit at k = 4 on 5x5 seed 3 (all move sets), 6x6 seed 0 M2, and 8 of 14 solutions of 6x6 seed 2 M1. A solution met inside an earlier explored set is not re-run; its component and its farthest distance come from that set.
- **(b)** is exact for k <= 4 where no component was capped; above 4 only the beam bound exists. The beam searches monotone paths only (each step puts one more cell into the target's placement, width 200, at most 30 pairs per board), so it is an upper bound and probably loose.
- **(c)** 200 runs per board (the minimum asked), 2e6 steps each at every size, geometric temperature 2.0 to 0.05, M1 proposals (30 percent inner rotations, otherwise same-class swaps with random inner rotations), random starts, 50 snapshots per run. The fixed budget handicaps 7x7: its endpoints stop at about 8 mismatches, so the 7x7 rows on near-solutions rest on 3 boards. The trap check was not in the plan; I added it because it measures the question directly. Its BFS is capped at 200,000 boards; a capped search counts as not reaching a solution (10 cases, all on 6x6 seed 2, and 9 on 5x5 seeds 1 and 2).
- **"Correlation over all sampled boards"** depends on the sample: the pooled sample here is 50 snapshots per run plus 1,000 random boards. I report it as the plan defines it and next to it the value restricted to boards with 8 or fewer mismatches, which is the part of the space a search would be in.
- **(d)** Exact counts only at 5x5, up to 3 mismatches, on four of five seeds. The 4-mismatch count did not finish (lower bound only), the 6x6 count to 2 mismatches did not finish in 300 s, and nothing was counted at 7x7. A plain Knuth estimator and a hybrid (random prefix, exact subtree) were tried and dropped: their variance was useless at this budget (the Knuth estimate for 5x5 seed 0 was 57 boards at 4 mismatches, against an exact lower bound of 185,442). So the count of near-solutions per solution at 6x6 and 7x7 is not measured. The BFS gives lower bounds only (for example, at least 1,836 boards at 2 mismatches on 6x6 seed 0).
- Compute: one process at a time, `nice -n 10`, each run under `timeout 900` or less, about 50 core-minutes in all. One stretch, early on, had a waiting shell loop alongside the single compute process; no two compute runs overlapped.

## Files

In `geometry-isolation/`: `export_boards.py` (writes `boards/` from the shared painter), `iso.c` (all measurements: `enum`, `basin`, `beam`, `anneal` with the trap check, `count`, and the dropped `knuth` and `hybrid`), `run_all.sh`, `run_rest.sh`, `run_final.sh` (the three drivers, in the order run; the second and third record the cuts in their headers), `raw/` (every JSON line output; `raw/superseded/` holds the aborted 4-mismatch count and the Knuth trial), `raw_*_log.txt` (exit codes and times), `summarise.py` and `summary.txt` (all per-board numbers in this memo), and `fig_mismatch_vs_distance.png`. To rerun: `bash run_all.sh basin`, then `bash run_final.sh`, then `uv run --with numpy --with matplotlib python -I summarise.py > summary.txt`.
