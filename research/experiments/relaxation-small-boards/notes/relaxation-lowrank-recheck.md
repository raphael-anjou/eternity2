# Review of the low-rank relaxation line

independent re-check, 2026-11-14, on four memos: `lowrank-relaxation.md` (R), `lowrank-rust.md` with the
crate `lowrank-rust/` (U), `theory-a-structured-sweep.md` (A), `theory-c-convergence.md` (C), with
the growth run's rows (`growth-exponent/results/`) as later evidence (G) and the study log
(`the run log` from 21:50) checked against them (S). The memos were not edited. Scripts and raw
outputs: `scripts/recheck-lowrank/` (its `README.txt` lists them). At most 2 cores,
`nice -n 10`, no run over 10 minutes, nothing remote, no 6x6 solve; the machine carried a load of
40 to 105 throughout, so every second below is a loaded second and evaluation counts are the
column to trust. Every board is a generated toy. Nothing here is Eternity II progress.

Grades: **stands**, **stands with a narrower scope**, **wrong**, **unproven**.

## 0. Rulings

| # | Claim | Ruling |
|---|---|---|
| U1 | "The port is proved equal" to NumPy (1.46e-13 on 30 states, eight kernels) | **Stands with a narrower scope.** Rerun from a fresh build of the crate's source: 1.46e-13, PASS. The 30 stored references recomputed with the original `bm.py`: identical. What is proved equal is one function, the augmented Lagrangian and its gradient, at random states with multipliers on 30 percent of the free pairs (runs carry 68 to 86 percent) and penalties up to 1e4 (runs sit at the cap, 1e5). The optimiser (own L-BFGS and line search against SciPy's L-BFGS-B), the multiplier update, the violation, the rounding and the restart are not covered by the test. As a solver the port is not shown equal: at NumPy's settings it rounds inside 12,000 evaluations in 13 of 50 runs, theory C's NumPy loop in 8 of 16 (section 1). |
| U2 | "50 of 50" for the best configuration (S10) | **Stands**, and it survives the test it had not had. The 50 is the selection set: S9 solved 49, and S10 adds a restart rule whose period (100, 250, 1,000 tried) was chosen on the same starts after seeing the one failure. On 150 runs it was not selected on (ten new boards, ten new starts on each memo board), the unchanged configuration solves 150 of 150 inside 120 CPU-seconds, 148 before any restart (section 2). |
| U3 | "Same five boards and 10 starts throughout" | **Stands with a narrower scope.** Same boards and start indices. The starting points are not the same throughout: steps S0 to S4 read the rank-32 start files, S5 to S10 the rank-16 ones. The ten draws are shared by all boards (`default_rng(1000 r + start)`, whatever the board), so 50 runs are 10 random draws on 5 boards. S0 has 40 rows on four boards, not 39. |
| U4 | Median 3,017 evaluations, 3.3 s, per start | **Stands** as a pooled median. Fresh runs: median 3,453, mean 5,326. It is not a property of "a 5x5 board": per-board medians run from 2,040 to 12,247. The figure to quote is the mean, which is the work per solution: about 5,100 to 5,300 evaluations, about 6 CPU-seconds on this machine. |
| U5 | "5.0 times fewer evaluations", "about 145 times faster" than NumPy | **Stands inside the port only** (S1 to S10). Against NumPy it is **unproven**, as the memo half says: NumPy's evaluations to a first solution were never recorded in stage 1. The NumPy-side figures that exist are far below the port's S1 median of 15,188: 6,419 (the one early-stop test), 3,270 to 8,830 in 8 of 16 runs (theory C). Theory C's paired figure for the inner budget on NumPy is under 2 times (6 pairs), against 5.6 times in the port. |
| U6 | "What moved the count: the inner budget and the stopping rule"; rank 16, structured kernel, no nonnegativity multipliers, restart as further gains | First half **stands**. The rest moved nothing measurable in evaluations: 2,695 (S2), 2,748, 3,231, 3,274, 3,010, 3,017 (S10). The halving of the median seconds between S3 and S10 is **mostly load**: measured here under one load, S10's configuration is 1.3 times cheaper per evaluation than S3's, not 2.2 (section 2.4). |
| U7 | "Solved" is an exact check | **Stands.** `Board::is_valid` requires one placement of its own cell per cell, every tile at most once (25 tiles, 25 cells: exactly once), grey on every rim side, equal colours on every inner edge. It does not read the enumerated list. Independently rebuilt from the painter here: placements, grid, solution sets (36, 6, 8, 403, 2) all agree, and every accepted board in every row is in the enumeration (section 3). |
| U8 | Broken-board control, 0 of 10 | Unsolvability **stands**: two colours appear on an odd number of sides (a proof without search), and my own search finds 0 boards. "0 of 10" is the wrong unit and the control is weaker than it reads: acceptance is exact, so it can only catch a verifier bug. In that role it is strong: 447,993 roundings rejected in S10's ten runs, 5.1 million over all result files. It says nothing about the method (section 3). |
| U9 | "A heuristic that finds a board while the penalty dynamics still move, not a convergent certificate" | **Stands**, and the rows say more: this is a randomised search guided by a penalised relaxation and stopped by an exact test. Where the same dynamics do converge to a feasible point, the rounding fails (section 4). |
| R7, S | "Annealing 58 percent, this 100 percent"; R: "the low-rank form is not worse at 5x5" | **Wrong as a comparison.** One annealing run is 2e6 steps, 0.19 to 0.29 s here; one low-rank start is capped at 480 s and uses 5.8 s on average. Per verified solution on the same five boards: annealing with restarts 0.08 to 0.96 CPU-second, the low-rank form 2.8 to 8.0. Annealing is 8 to 35 times cheaper at 5x5, and a plain depth-first search lists every solution in at most 0.1 s (section 5). |
| U10 | Cost model t = P^2 (a + b r); 123 s per evaluation at 16x16 | Arithmetic **stands** (122.9 s). The model **stands at one more size**: 8.9 ms per evaluation measured at 6x6 over 2.2 million evaluations, whole run, against 8.3 ms predicted for the evaluation alone. At 16x16 it is a 107,000-fold extrapolation from a size where every array sits in cache: **unproven**, and best read as a floor. |
| U11 | "9.3 GB in the best form" at 16x16 | **Wrong** for the crate, **stands** as arithmetic for a design that is not written. As coded, the best configuration holds six dense P x P arrays (pair kind, conflict index, two masks, the multiplier matrix, and the Gram matrix that `fast_outer=1` forms at every outer step): 37 bytes per pair, 910 GB at 16x16. |
| U12 | "About 12 s on 10 cores" | **Unproven.** Ideal scaling assumed. The only thread measurements in the memo are slower than one thread. |
| U13 | Expectation "70 to 190 s per start" at 6x6 | **Wrong by a factor of 15 to 40** on the rows this Mac has produced (2,805 CPU-seconds per solution, 95 percent interval 1,360 to 6,980). The cost per evaluation was right to 6 percent. The evaluation count failed: growth was guessed between P and P^2, the rows give about P^4 (section 6). |
| U14 | "At the 5x5 count of 3,000 evaluations that is 10 hours per start on 10 cores" | Arithmetic **stands**; the sentence should not be used. The flat-count case is refuted by the first size step. |
| R1 | Kill line not met at 5x5 on five of five boards | **Stands with a narrower scope**, as the memo says: ranks 32 and 64 only, feasibility threshold fixed after tuning on seeds 0 and 1. |
| R2 | "Greedy rounding reads a solution in 38 of 38 runs at r = 32 and 37 of 50 at r = 64" | Greedy alone: 38 of 38 and **36** of 50. 37 is greedy or Hungarian (the table's definition). Total 75 of 88 by either, 74 by greedy, which is where the Rust memo's "74" comes from. |
| R3 | "The low-rank run behaves as a randomised search that lands on or near solutions" | **Stands.** It is the reading the later memos confirm. |
| R4 | Broken-board control: 12 of 12 stay infeasible (violation 0.026 to 0.23) | **Stands**, and unlike U8 it is a control of the relaxation: the violation is measured, not only the rounding. |
| R5 | 6x6 NumPy runs "decide nothing" | **Stands.** The Rust memo's gloss ("consistent with the slow inner budget") is moot: with the fast budget and 12 to 35 times more evaluations, 8 of 15 runs still end unsolved. |
| A1 | The 3.3e8 zero constraints collapse to 7,952 linear vector equalities, given nonnegativity on conflict pairs | **Stands.** Proof checked by hand in both directions; count recomputed (512 sum rows, 7,440 edge and colour groups). |
| A2 | "48 to 85 percent of pairs are active at every evaluation: no small active set" | **Stands with a narrower scope.** One board, one start, one rank, the structured sweep, and the share is of all pairs, not of free pairs (0.482 to 0.858, recomputed from its rows). Other 5x5 rows agree (68 to 86 percent of free pairs hold a multiplier at the stop in the Rust rows). At 6x6 the only figure is 45 percent in unconverged states. "Worse at 16x16" is the memo's guess. |
| A3 | Three forms with no P^2 term (18,000, 489 and 49 times cheaper per sweep) | Cost arithmetic **stands**. As solvers all three are **unproven**: none was run, the memo says so. The cheapest one is, by the memo's own derivation (checked), the puzzle itself written r times; the other two drop the free-pair nonnegativity. "Cheaper per sweep" compares sweeps of different problems. |
| A4 | "Fewer flops is not sooner solved" | One paired start. An anecdote, in line with U9. |
| C1 | "Strict complementarity holds": the degenerate-optimum hypothesis is refuted | **Stands with a narrower scope.** Shown: on two boards, at the splitting's last iterate, the ranks of the primal and dual semidefinite blocks add up to the face order with no eigenvalue within 1e-6 of zero. Not shown: complementarity of the nonnegativity block. The memo's own table has 565 to 3,614 free pairs with neither a positive entry nor a multiplier above 1e-8. One of the rows is unconverged (residual 1e-4). And the claim is about the convex splitting; it says nothing on the low-rank count. |
| C2 | "The tie between solutions is not why it is slow" (earlier wording of C1) | **Unproven** either way. Direct evidence is thin: where a hint leaves one solution (seeds 2 and 4) the low-rank form solves 1 of 5 against 3 of 5 without it; a linear objective, which breaks the tie in the splitting, collapses later on the one board where both ran (3 runs). No board with a single solution was run. |
| C3 | Linear objective, no objective, triple norm: 0 of 5; concave: 2 of 6 | Counts **stand** (re-tallied). Scope: every one of the "0 of 5" comparisons uses start index 0 only, one random draw shared by the five boards, the draw on which the baseline solves 5 of 5 against 3 of 11 on the others. Exact sign test p = 0.06 each. The uncensored first runs show these modes solving later (13,867 to 23,265), so the finding is "slower inside 12,000", not "does not solve". |
| C4 | Hint: "no effect", 6 of 14 against 7 of 14 | The memo says "no effect shown either way", which **stands**. "No effect" (run log, twice) is **more than the memo supports**: 2 pairs improve, 3 worsen. |
| C5 | "Nothing tested cuts the low-rank count by 10 times at 5x5" | **Stands.** |
| C6 | Ranked changes for 16x16 | Argued, not measured, and labelled so. Not ruled. |
| G1 | Growth of the evaluation count from 5x5 to 6x6 | Work per solution, capped runs counted as censored exposure: 317,000 evaluations at 6x6 (7 solutions in 2.22 million, 95 percent interval 154,000 to 787,000) against 5,083 at 5x5. Factor 62 (30 to 155). As a two-point exponent in P: **4.2, interval 3.4 to 5.1**. The project's "2.5 or worse" **stands as a lower bound** on the median, and understates the estimate (section 6). |

## 1. Corrected statements that may be used

1. **Equivalence.** "The Rust kernels compute the same objective and gradient as `bm.py` to 1.5e-13
   on 30 random states (rerun by the re-check). The Rust solver is a different optimiser on the
   same function; it was not compared with NumPy run for run."
2. **5x5 success.** "On 15 generated 5x5 boards (frame 3, inner 4; 2 to 405 solutions) and 200
   random starts, the Rust configuration reached an exactly verified solution in 200 of 200 runs,
   197 of them inside its first 1,000 outer steps. Work per solution: about 5,200 evaluations,
   about 6 CPU-seconds on a loaded core. Board medians run from 2,000 to 12,000 evaluations.
   The configuration was chosen on 50 of those runs and held on the other 150."
3. **What the method is.** "A randomised search: a penalised low-rank relaxation moves a
   fractional point, a greedy rounding is tested after every inner iteration by an exact check
   (about 2,450 tests per run), and the run stops at the first valid board. At that moment the
   relaxation is not solved (violation 0.03, 18 percent of the mass off the solutions, full
   rank). When the relaxation does converge, the rounding is usually lost."
4. **Against other methods at 5x5.** "On the same five boards plain annealing with restarts needs
   0.1 to 1 CPU-second per solution and the low-rank form 3 to 8. A depth-first search
   enumerates all solutions of each board in under 0.1 s. The low-rank form is the slowest of
   the three at 5x5." Never "annealing 58 percent, this 100 percent".
5. **Against other methods at 6x6.** "Not measured like for like by this review. The low-rank
   form costs about 2,800 CPU-seconds per solution on this Mac (interval 1,360 to 6,980); the
   far-ideas review reports restart annealing at one 6x6 solution per 30 to 70 CPU-seconds, and
   exhaustive search lists all solutions in 0.3 to 0.7 s. Its test T1 is the comparison."
6. **Growth.** "From 5x5 to 6x6 the evaluations per solution grew 62 times (30 to 155) while P
   grew 2.7 times: a two-point exponent of 4.2 in P (3.4 to 5.1), or 1.46 per added cell (1.36
   to 1.58). Two sizes cannot tell a power from an exponential, the two sizes differ in colour
   counts, and the restart period was tuned at 5x5. The cost model per evaluation held; the
   memo's expectation of 70 to 190 s per start was low by 15 to 40 times."
7. **Scale.** "123 s per evaluation at 16x16 on one core is a model floor, verified at 6x6 only.
   9.3 GB is the memory of an implementation that does not exist; the crate as written would
   need 910 GB. No evaluation count for 16x16 follows from the data: at the low end of the
   measured growth the count is of order 1e12." The "10 hours per start" line is withdrawn.
8. **Theory A.** "Every equality of the relaxation reduces to 7,952 linear vector equalities
   (derived, checked). The free-pair nonnegativity keeps a P^2 cost; at 5x5 half to five sixths
   of the pairs are active (one run, corroborated by the multiplier counts of other runs).
   Three cheaper forms exist on paper; none has been run as a solver."
9. **Theory C.** "On two 5x5 boards the semidefinite part of the convex optimum is strictly
   complementary. That does not settle whether ties between solutions slow the solver: the
   direct tests are 5 to 14 runs and show nothing either way. Swapping the objective made the
   low-rank form slower on the one starting draw tested. A hint showed no effect in 14 pairs."

## 2. "50 of 50": selection, and what an unselected sample gives

### 2.1 How many configurations, on which runs

`01_tally.py` reads every result row of the crate. Full-suite configurations (the same five
boards, start indices 0 to 9): 12 on the Mac (S0 on four boards, then eleven with the stopping
rule, S7 counting twice) and 14 on x86.
Mini-suite configurations (4 or 12 runs, taken from those same boards and start indices): 24.
The result files hold 63 distinct configuration records. The step table is a greedy path
through them: each step was kept or rejected on the 50 runs it is then scored on. That is
selection on the test set. Its usual cost is an optimistic headline; section 2.3 measures it.

Solved counts of the full-suite configurations with the stopping rule: 50 in nineteen of them,
49 in five (S4, S9, and x86's inner 300, rank 4 and penalty start 1,000), 47 of 47 in one
stopped early. No configuration with multipliers on the sums and conflicts fell below 49.

### 2.2 What S10 added to S9

`04_s10_detail.py`. 49 of the 50 runs have the same evaluation count in S9 and S10, to the
unit: the two are one algorithm until outer step 1,000. The fiftieth, seed 0 start 9, is S9's
failure: 600,575 evaluations in 480 s, ending at violation 7.4e-5 and rank 5 with 17
mismatches. It did not stall. It converged to a feasible blend of solutions, which the greedy
rounding cannot read. S10 abandons it at outer step 1,000 and solves from a fresh start at
31,287 evaluations. So "50 of 50" is "49 of 50, plus a restart period picked from three tried
values after seeing the failure". The memo reports each of those facts; its headline does not.

### 2.3 Unselected runs

Built from a copy of the crate's source in the scratchpad (`cargo build --release`), run with
S10's parameters unchanged and a 120 CPU-second cap (`08_fresh_summary.py`,
`raw/fresh_*.jsonl`).

| sample | runs | solved | before any restart | median evaluations | mean | inside 5,000 | inside 10,000 | max |
|---|---|---|---|---|---|---|---|---|
| memo rows (selection set) | 50 | 50 | 49 | 3,017 | 5,083 | 35 | 43 | 31,287 |
| ten new boards, painter seeds 5 to 14, starts 0 to 9 | 100 | 100 | 100 | 3,453 | 5,088 | 70 | 87 | 22,719 |
| memo boards, new starts 10 to 19 | 50 | 50 | 48 | 3,453 | 5,802 | 34 | 42 | 31,845 |

Every accepted board is in the enumeration of its board. The new boards have 4 to 405
solutions; their medians are 2,040, 2,458, 2,776, 3,051, 3,116, 3,605, 3,856, 4,655, 6,391 and
12,247 (the last on a board with 5 solutions).

The rate to quote. Unselected: 150 of 150, so the success rate inside 120 CPU-seconds is above 97.6
percent (two-sided 95 percent, runs taken as independent) on boards of this family. The runs
are clustered (15 boards, 20 draws), so the board-level statement is the firmer one: 15 boards
of 15 with every start solved. Inside a fixed work budget the rate is lower and is the number
to quote when a budget is implied: 69 percent inside 5,000 evaluations, 86 percent inside
10,000 (129 of 150), 99 percent inside 25,500 (one restart period).

The selection bias this time is small: the median moved from 3,017 to 3,453 and the mean did
not move. The headline held. It had not been earned by the 50 runs alone.

A reproduction note. `step.sh` as documented leaves the crate's default of 200 outer steps;
the memo's rows carry `max_outer=10000000`, which the configuration line does not list. Run as
documented, 29 of 100 new-board runs stop unsolved near 5,100 evaluations (my first attempt,
kept as `raw/mistake_max_outer200_boards.jsonl`, used nowhere).

### 2.4 Which steps moved what

Evaluations (load-independent): S1 15,188, S2 2,695, then 2,748, 3,612 (rejected), 3,231,
3,274, 3,010, 3,017. Everything after S2 is inside the noise of 50 runs with a 3-to-1 spread
between boards. The seconds column fell from 6.9 (S3) to 3.3 (S10) with the evaluation count
9 percent higher, so the whole fall is in milliseconds per evaluation, 2.46 to 1.14. The
memo's unloaded kernel benchmark does not show that gain (S3's kernel 0.98 ms at r = 32, S10's
1.10 ms at r = 16), and the memo states that the load fell from 50 to 100 down to 15 to 40
over those steps. Measured here under one load, one job at a time, the three configurations alternating on the same six starts, twice (`raw/loadcheck.txt`): 1.75 ms per evaluation for S3's configuration, 1.87 for S6's, 1.32 for S10's (whole run; evaluation alone 1.28, 1.50, 1.01). So dropping the nonnegativity multipliers buys about 1.3 to 1.4 times per evaluation, the structured kernel and rank 16 nothing, and the table's 2.2 times is mostly the load. From S2 to S10 the real gain is about 1.2 times in seconds per solution, against the 2.1 the median-seconds column shows.

## 3. What "solved" means, and the control

`03_validity.py` rebuilds everything from the painter without `board.py` or the stored
enumeration: the tiles; the 478 rim-consistent placements with the pin; the grid; the solutions
by a depth-first search written for the purpose; an edge-by-edge check of every stored solution.
All six boards agree (solution sets equal as sets: 36, 6, 8, 403, 2, and 0 for the broken one).

`Board::is_valid` (`src/board.rs`) was read line by line. It accepts a list of 25 placement
indices only if each index is a placement of its own cell, no tile repeats, every side facing
off the board is grey, and both sides of every inner edge are equal. With 25 cells and 25 tiles
"no tile repeats" is "every tile once". The pinned corner is enforced through the placement
list (cell 0 has one placement). The enumerated list is consulted after acceptance, only to set
a flag; that flag is true in every solved row of every file (1,495 solved runs in the memo's
files, 150 here).

The rows do not store the accepted board. So the chain is: exact check in the code (read),
plus membership in an enumeration I rebuilt independently. I could not re-verify an accepted
board edge by edge from a row. Storing the 25 indices in the row would close that.

The broken board. One side of tile 2 is recoloured from 6 to 4. Colours 4 and 6 then appear on
an odd number of sides, and every matched edge uses a colour twice, so no board matches every
edge. That is a proof, independent of any search; my search agrees (0 boards, 46,714 nodes).

Is 0 of 10 enough? For what the control can test, yes, with a large margin, but the unit is
the rounding, not the run: 5,149,332 rounded boards were checked on the broken board over all
result files and none was accepted. For what a reader expects a negative control to test, no
number of runs is enough, because the exact check makes a pass impossible. The NumPy memo's
control is the informative one: there the violation was measured and stayed at 0.026 to 0.23.
In the Rust rows the broken board ends at violation 0.021 to 0.051, the same range as the
solvable boards at the moment they are accepted (0.019 to 0.066): by the violation alone, a
broken board and a board about to be solved cannot be told apart. That is one more sign that
the relaxation is not what decides.

## 4. Relaxation solver or randomised search

The evidence, all from the memos' own rows:

- **Accepted far from feasibility.** At the first valid rounding (S10, 50 runs; the 100
  new-board runs are the same): largest violation 0.029 (0.019 to 0.066), 18 percent of the
  mass per cell off every solution, rank 16 of 16 (lowest 13), penalty at its cap.
- **Convergence loses the board.** S0 ran the faithful algorithm to its end: 40 of 40 runs held
  a valid rounding at some point, 28 still held one at the end (5 of 10 on seed 0, 4 of 10 on
  seed 3). S9's failure is a converged run (section 2.2). NumPy at r = 64 has six "converged
  and wrong" runs. Theory A's structured sweep reaches violation 1e-6 and never rounds.
- **Many draws per run.** A median of 2,452 exact tests per run; a run is accepted on the first
  hit.
- **At 6x6 it behaves as independent restarts with a constant hit rate.** On this Mac: 7
  successful 1,000-step segments, 79 abandoned, 8.1 percent per segment (3.3 to 16.1). At 5x5
  the first segment succeeds in 197 of 200 runs.
- **The constraint that makes the lift exact is not what solves 5x5** (theory B, not independently checked,
  run log 01:16: 46 of 50 with the free-pair nonnegativity removed, same median).

It is not blind search either. A greedy rounding of an arbitrary fractional point is valid
with negligible probability on boards with 2 to 405 solutions, and the broken board gives 0 in
5.1 million. The penalty dynamics carry the point into a region where roundings are near
solutions. The fair name is a penalty-guided randomised search with an exact acceptance test.
"Relaxation solver" suggests a certificate, a bound or a convergence guarantee; it has none of
the three, and the memo says so.

## 5. The comparison with annealing

"Annealing 58 percent, this 100 percent" compares success per run across runs of unequal size.

- An annealing run in the isolation memo is 2e6 steps. Timed here with that study's binary,
  20 runs per board, one loaded core: 0.19 to 0.29 s per run.
- A low-rank start is capped at 480 CPU-seconds and uses 5.8 on average: 20 to 30 annealing
  runs' worth. Give annealing that time and its success on the hardest of the five boards
  (seed 4, 22.5 percent per run) is 1 - 0.775^24 = 99.8 percent; on the others it is above
  99.999 percent.

Work per verified solution, the same five boards (`02_anneal_cost.py`; annealing from the
isolation rows, an unsolved run charged in full; low-rank from the S10 rows):

| board (solutions) | annealing: solved of 200 | steps per solution | CPU-s per solution | low-rank: evaluations per solution | CPU-s per solution | ratio |
|---|---|---|---|---|---|---|
| seed 0 (36) | 168 | 1.3e6 | 0.18 | 5,452 | 6.4 | 35 |
| seed 1 (6) | 83 | 3.8e6 | 0.44 | 3,903 | 4.7 | 11 |
| seed 2 (8) | 85 | 3.6e6 | 0.41 | 5,973 | 7.1 | 17 |
| seed 3 (403) | 199 | 8.7e5 | 0.08 | 2,479 | 2.8 | 35 |
| seed 4 (2) | 45 | 7.8e6 | 0.96 | 7,607 | 8.0 | 8 |

The two timings are not from the same hour (the low-rank seconds are the memo's, the
annealing seconds mine, both on a loaded machine), so read the ratio to a factor of 2. Its
direction is not in doubt. And the reference that should sit beside both: the isolation
study's enumerator lists every solution of these boards in 21,000 to 430,000 nodes, at most
0.1 s, and of the 6x6 boards in 1.9 to 3.8 million nodes, 0.3 to 0.7 s (its own rows; my
Python search reproduces the 5x5 node counts).

"Board evaluations" is not a usable common unit: an annealing step re-scores four edges, a
low-rank evaluation is a P^2 pass (about 9,000 annealing steps' worth of time at 5x5), and the
low-rank run tests about 2,450 complete boards. CPU-seconds per verified solution, one thread,
same machine, same cap, is the like-for-like measure. At 6x6 that comparison is the far-ideas
re-check's test T1, already started; nothing in the four memos reviewed here supports "the
relaxation reaches sizes where local search fails".

## 6. Cost model, the 6x6 expectation, and the growth

`07_growth_and_cost.py`, output in `raw/07_growth_and_cost.txt`.

**Recomputed.** t = P^2 (3.924e-9 + 16 x 6.715e-11): 1.14 ms at 478, 8.32 ms at 1,290, 42.5 ms
at 2,916, 8.66 s at 41,616, 122.9 s at 156,816. The memory rows follow from their formulas
(conflict list 7.9 GB, vectors 0.56, scratch 0.80, total 9.3; multipliers 241 GB if kept).
Small inconsistency between memos: R counts placements with the corner pinned (156,810; 2,910
at 7x7), U without (156,816; 2,916) while its 5x5 and 6x6 figures are pinned. Immaterial.

**Checked at 6x6.** The growth rows on this Mac (snapshot of 15 rows, 02:04): 19,634
CPU-seconds for 2,215,962 evaluations, 8.86 ms per evaluation for the whole run. The model
gives 8.32 ms for the evaluation alone, which was 75 percent of a run at 5x5. So the model is
right or slightly pessimistic at 6x6. One size step, P^2 times 7.3.

**Not checked beyond.** At 16x16 the factor (478 rows of 16 doubles, 61 kB) becomes 20 MB and
the 64-row block 80 MB per thread: the per-pair constant was fitted with everything in cache.
The memory figure describes a kernel-only design: `Board` holds `kind` (1 byte per pair),
`conf_idx` (4), `confm` and `freem` (8 each), `Mult` holds `pl` (8), and the solver forms
`gram` (8) at every outer step because the best configuration sets `fast_outer=1`. 37 P^2
bytes, 910 GB.

**The expectation.** Guessed growth P^1 to P^2 from the median 3,017: 8,142 to 21,973
evaluations, 68 to 183 s. Observed on the Mac: 2,805 CPU-seconds per solution (1,361 to
6,976). Factor 15 to 40 at the point estimate, 7 to 100 across the interval. Split: cost per
evaluation 1.06, evaluation count everything else. The run log line "wrong by an order of
magnitude" is at the bottom of that range.

**The growth, with censoring.** A capped run is neither a failure nor a success: it is
evaluations spent without a solution. With a restart every 1,000 outer steps the process is
close to memoryless at the scale of a run (successful segments end at outer steps 114, 267,
510, 709, 821, 963 and 970 of 1,000), so the estimate is total evaluations over solutions,
with an exact Poisson interval on the count.

| rows | solutions | evaluations spent | per solution | 95 percent | factor over 5x5 (5,083) | two-point exponent in P | per added cell |
|---|---|---|---|---|---|---|---|
| this Mac, read | 7 of 15 runs | 2.22e6 | 317,000 | 154,000 to 787,000 | 62 (30 to 155) | 4.2 (3.4 to 5.1) | 1.46 (1.36 to 1.58) |
| x86, as reported in run log, not read | 3 of 14 | about 4.5e5 | about 150,000 | 50,000 to 750,000 | 30 (10 to 148) | 3.4 (2.3 to 5.0) | 1.36 (1.23 to 1.58) |
| pooled | 10 of 29 | 2.67e6 | 267,000 | 144,000 to 559,000 | 52 (28 to 110) | 4.0 (3.4 to 4.7) | 1.43 (1.36 to 1.53) |

Against the three 5x5 boards whose solution counts (2 to 8) resemble the 6x6 boards' (3 to
14), the base is 5,828 and the Mac exponent 4.0 (3.3 to 4.9). The x86 row rests on the study log's
reading (two of the three solve counts, an approximate cap); I did not see those rows.

Medians. The 6x6 median is above 35,000 evaluations with 95 percent confidence (4 of 29 solved
inside it, upper bound 32 percent), which gives the earlier exponent of 2.5 as a floor. On the
Mac 7 of 15 are solved inside about 190,000 to 220,000; the median is near there, which would
be 4.2, but the interval on 7 of 15 contains one half. The exponent from work per solution is
the better-determined one.

Limits of "exponent".
- Two sizes. A power of P and an exponential in the cell count fit equally; for a search the
  second is the expected form. I give both columns; neither may be extrapolated as a law.
- The sizes differ in more than size: frame 3 and inner 4 colours at 5x5, 4 and 5 at 6x6.
- The restart period was tuned at 5x5, where the median solve comes at outer step 117. At 6x6
  the successful segments end at a median of 709 of 1,000, two of seven after step 960: the
  restart is cutting runs that were about to solve. The figure is for the configuration
  unchanged, which is what the growth run measured first; a longer period would
  probably lower it, by how much is not known.
- Fifteen runs: 8 capped, and board s1 supplies 3 of the 7 solutions. The interval treats
  runs as exchangeable across boards, which five boards cannot confirm.

What this does to the scale table: at the low end of the interval (3.4) the count at P =
156,816 would be about 2e12 evaluations. I give that number only to retire "10 hours per
start"; it is an extrapolation over 2 orders of magnitude in P from two toy sizes.

## 7. Theory A and theory C

**A.** The aggregation theorem is correct. Forward: with the conflict zeros and the cell sums
equal to v_0, |A_k - B_k|^2 = |A_k|^2 + |B_k|^2 - 2 <A_k, B_k> and each of the three terms
equals <A_k, v_0>. Backward: as the memo writes it, using only nonnegativity on conflict
pairs. The count 7,952 is 2 x 256 sum rows plus 60 rim-adjacent edges x 5 colours plus 420
edges x 17. The memo is exact about its limits (identities checked at one random point; the
three cheap sweeps not written as solvers; one paired run). The one place a reader goes wrong
is the cost table: its first row is not a cheaper way to solve the relaxation, it is a
different, non-convex problem whose feasible set is r copies of the puzzle, and theory B then
found it solves 0 of 50 (run log 01:16, not independently checked). "48 to 85 percent" is one trajectory
(`raw/run_struct_r32_s0.jsonl`, recomputed: 0.482 to 0.858 of all pairs).

**C.** The suite re-tallied (`09_theory_c_tally.py`) matches the memo's table A1 row for row.
Three points on what may be said.

1. *The refutation.* The test made is rank of Z plus rank of the dual equal to 433 with a
   spectral gap, on seeds 1 and 2. For a doubly nonnegative program strict complementarity
   also needs, for every free pair, either a positive entry or a positive multiplier. The
   last column of table A3 counts the pairs with neither: 1,226, 565, 848, 3,614, 792 without
   objective. The memo's Appendix B notes that the multipliers are not unique, so those pairs
   may be an artefact of the multiplier the iteration happened to reach; but then the claim is
   "not refuted on the orthant block", not "holds". The norm-objective row is read at a
   residual of 1e-4.
2. *The thin suite.* Baseline: 8 of 16 inside 12,000, and by start index 5 of 5, 1 of 5, 1 of
   5, 1 of 1. The objective swaps were run on start 0 only. Start 0 is one random draw, reused
   on every board, and it is the draw the baseline likes: comparing a variant with the
   baseline there is comparing it with the baseline's best case. Five discordant pairs of
   five gives p = 0.06 per mode. What may be said: on that draw the three swaps did not solve
   inside 12,000 where the baseline did; with a larger cap they solved at 14,000 to 23,000.
3. *The inner budget.* Theory C: 300 to 30 gives 5,002 to 2,881 on 6 pairs (under 2 times,
   SciPy). The Rust memo: 300 to 20 gives 16,432 to 2,756 on 50 (5.6 times, its own L-BFGS).
   Both end near 2,800. The disagreement is in the 300-iteration baseline, which is three
   times worse in the port. Neither memo reconciles it; theory C flags it. The gain "5.6
   times from the inner budget" is therefore partly the port recovering ground it had lost.
   Theory B's NumPy rows with the short budget (100 of 100, median 3,038; not independently checked) say the
   end point is the same in both codes.

## 8. The study log against the memos

| run log entry | Statement | Against the memo |
|---|---|---|
| 21:50 | "Annealing on the same boards: 58 percent" beside 52 of 66 | Unequal effort (section 5). Already withdrawn at 01:55 for 6x6; the 5x5 pairing should go too. |
| 22:53 | "Greedy rounding gives a solution in ... 37 of 50 at r = 64" | 36 by greedy; 37 by greedy or Hungarian. The memo's sentence has the same slip. |
| 23:22 | "48 to 85 percent of pairs are active at every evaluation, so an active set saves nothing" | The memo: one board, one start, one rank. The log drops the scope. |
| 23:22 | "nonnegative factor (18,000 times cheaper ...)" | Per sweep, of a different problem. The log's parenthesis does say "feasible set exactly r copies of the puzzle"; the ratio should not be quoted without it. |
| 23:58, title | "the tie between solutions is not why it is slow" | Stronger than the memo ("the degeneracy hypothesis is half right") and than the evidence (C1, C2). |
| 23:58, 01:12 | "Hints: no effect at 5x5"; "theory C measured no effect" | The memo: "no effect shown either way" on 14 pairs. |
| 00:18, title | "all starts solved" | True of the 50 after a rule added for the one that was not. Now also true of 150 unselected runs (section 2.3), so the title can stay if it cites those. |
| 00:18 | "What moved it: ... rank 16 with the structured kernel; nonnegativity as a pure penalty; restart after 1,000 stalled outer steps" | The memo's evaluation column shows no movement from these three (2,748 to 3,017), and it attributes part of the seconds to the load. Only the inner budget and the stopping rule moved the count. |
| 00:18 | "lost a solution it had found in 11 of 39 runs" | 12 of 40 in the rows; the memo has the same count as the log. |
| 00:18 | "9.3 GB" | Of a design, not of the crate (U11). |
| 00:18 | "The NumPy 6x6 runs that failed used the slow 300-iteration inner budget" | True and, after the growth rows, not the reason. |
| 00:21 | "10 hours if flat, 137 days if linear in P, 124 years if quadratic" | Arithmetic right (12 s per evaluation is the ideal ten-core figure). All three cases are below the measured growth. |
| 01:12 | "the first data puts it at 2.5 or worse" | A floor from the median. Estimate 4.2 (3.4 to 5.1). |
| 01:12 | "wrong by an order of magnitude" | 15 to 40 times. |
| 01:12 | "The cost model per evaluation held; the evaluation count did not" | Stands (1.06). |
| 01:12 | "every gain in this study is a constant factor at toy size" | Stands. |

## 9. What I could not check

- The x86 growth rows: not on this machine. Entered as reported.
- Any accepted board edge by edge: rows do not store it (section 3).
- The NumPy solver's evaluations to a first solution at its original settings: never recorded;
  measuring it is 50 runs of about a minute and would settle U5.
- Annealing at 6x6 at equal CPU: no 6x6 solve was allowed here; the far-ideas review and its
  test T1 carry it.
- The 16x16 cost per evaluation and the ten-core scaling: nothing above 6x6 exists.
- The 6x6 growth with a retuned restart period, on boards with matched colour counts, or at a
  third size. The exponent above is for the configuration unchanged, on two sizes.
- The share of active pairs above 5x5 at a solve point; theory A's three cheap forms as
  solvers; theory C's complementarity on the orthant block, on more than two boards, or with a
  converged norm-objective run.
- The x86 sweep rows were re-tallied, not rerun. The Mac step rows were not rerun on their
  own starts; the fresh runs of section 2.3 are the check.
- Timings: the machine was at a load of 40 to 105. Seconds here are good to a factor of 2.
