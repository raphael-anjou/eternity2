# Review of the four relaxation memos (theory, lift, engineering, convex 6x6)

independent re-check, 2026-11-14, 01:10 to 02:50. Memos reviewed, none edited: `relaxation-theory.md`,
`geometry-sdp-lift.md`, `relaxation-engineering.md`, `convex-6x6.md`. Also read: the run log from
19:43 to its end, the plan document section 9, and three later not independently checked memos
(`theory-c-convergence.md`, `theory-d-formulation-decoding.md` in its final state,
`theory-b-nonnegative-factor.md`). Scripts and raw outputs: `scripts/recheck-convex/`.
One process at a time, one thread, `nice -n 10`, 5x5 only, no process over 10 minutes, about
70 core-minutes. The machine carried a load of 35 to 130 and 9.7 GB of swap, so my iteration
counts per run are low (about 10,000 in 570 s). Everything here is on generated toy boards and
says nothing about Eternity II by itself.

Grades: **stands**, **stands with a narrower scope**, **wrong**, **unproven**.

## 0. Rulings

Prefixes: T theory memo, G lift memo, E engineering memo, C convex 6x6 memo, S the study log,
D and B the two later memos ruled on here.

| # | Claim | Ruling |
|---|---|---|
| X1 | "The lift is exact at 5x5" (T verdict and section 1, G section 4, E verdict, S 20:37 and 22:20, D before its final edit) | **Wrong as a sentence about the size.** Exactness is a property of a board. Of the five 5x5 (3,4) boards anyone ran, three (2, 6 and 8 solutions) have a converged largest-leak solve at zero, one (seed 0, 36 solutions) is undecided, and one (seed 3, 403 solutions) is not exact. Corrected sentence in section 1. |
| X2 | D item 3: on seed 3 the full lift keeps 0.0047 of the mass per cell on 10 placements in no solution | **Stands, and it is a real failure of exactness**, to numerical tolerance. Not an enumeration artefact (ruled out by an independent enumeration), not a convergence artefact (five checks, section 2.2, the last a point feasible to 1.5e-9 that keeps 1.05 units of mass off the solutions). Not proved to machine precision and not confirmed by a solver of another family. |
| X3 | "Mass off solutions per cell": definition and computation | **Stands.** Recomputed from stored points with an independent enumeration: identical to the last digit. Scope: it tests the support, not membership of the hull (section 2.1). |
| T1 | The test of exactness is the largest leak; it is 0.000 on 8 boards of 9 to 20 cells | **Stands with a narrower scope.** Solved by SCS at 1e-5, so "at most 2e-5 in total". The boards have 1 to 11 solutions and come from another painter and pin. Seven further 4x4 boards with 10 to 56 solutions were generated and skipped. |
| T2 | "Not an averaging artefact: the lift removes 219 to 280 placements per 5x5 board that singleton arc consistency keeps" | **Stands with a narrower scope.** Certified on seed 1 (280) in the memos and on seed 2 (257) by my run. On seed 0 (219) it rests on unconverged points. |
| T3 | The lift is a face of the theta body with entrywise nonnegativity; row sums are implied | **Stands** (rederived). |
| T4 | Counting 1: equalities leave room for non-integral points from rank 31, 69, about 2,140 | **Stands** (recomputed). It explains nothing, as the memo says. |
| T5 | The R scale: "everything measured exact has R <= 2.0", used to place 6x6 at 60 percent | **Unproven, and the stated regularity is now false.** A 5x5 board at R = 1.6 is not exact. R is monotone in the side, so it orders sizes and nothing else; and it is a scale for the theta VALUE, which is N at every size by the clique cover. |
| T6 | Odds 60 / 20 / 2 percent at 6x6 / 8x8 / 16x16 | **A estimate, not derived** (the memo marks it [G]). The direction of the decay is supported by analogy only. The quantity is undefined: exact on which share of boards? Section 3. |
| T7 | Literature: every asymptotic result that fits predicts failure | **Unproven as applied.** Pages not opened by me. The results cited are about refutation or detection on random sparse instances; none is a statement about the optimal face on a lattice, which the memo concedes. |
| T8 | 16x16 memory: 183 GiB dense, 30 GiB factored at rank 25,782, 0.9 TB as coded, 96 GB of multipliers | **Arithmetic stands** (recomputed). **"The rank that carries a guarantee" is wrong**: 25,782 counts the equalities of the form WITHOUT entrywise nonnegativity, which the same memo says must stay and which D measures inexact at 5x5. With it kept the store is 119 GiB and no rank below P is covered. |
| T9 | "The obstacle is exactness past 6x6, not storage" | **Stands with a narrower scope.** True of the factor alone. Every sound form priced in this study keeps order P^2 work or P^2 multipliers. And exactness is already an obstacle at 5x5. |
| T10 | Chordal decomposition: one block of at least 77,224 of 156,816 | **Stands** (the factor 4 is the clique blow-up of the rook's graph; checked). |
| T11 | Entrywise nonnegativity cannot be dropped | **Stands**, on one 4x4 board in the memo, and on D's converged 5x5 run (0.53 per cell). |
| G1 | "Both kill conditions pass on all three 5x5 boards" | **Stands with a narrower scope**: two boards. The memo's own next sentence withdraws seed 0. |
| G2 | On the two converged boards the point lies in the hull and puts "below 1e-5 per cell" off the solutions | **Stands** for the hull distance (recomputed 2e-6 and 2e-5). The mass on seed 2 is 1.8e-5, not below 1e-5. Points, not certificates. |
| G3 | Seed 1: all 325 placements certified dead | **Stands** (SCS status solved, 4e-5 in total; confirmed later by the splitting, 2e-7 in total). |
| G4 | "On three 5x5 toys and one 4x4 toy the lift is tight or nearly tight"; the support is "3 to 4 times smaller" than arc consistency's | **Stands with a narrower scope.** Two toys converged. The ratios are 4.5, 3.5 and 2.1. |
| G5 | "At 6x6 the baselines ran on all three boards" | **Wrong in a detail**: the LP ran on seed 0 only. |
| G6 | 6x6 solves decide nothing | **Stands.** |
| E1 | The splitting "reaches the exact reading on all three 5x5 boards in 3,500 to 6,000 iterations" | **Stands with a narrower scope.** One run per board at a penalty chosen per board (300 or 3,000). At 300, seed 0 is still at 4.6e-4 after 23,369 iterations. At 3,000 the objective barely acts, so the point is not the minimum-norm one and the run has not converged (residual 2.5e-5). |
| E2 | Rust kernel: 2.8 to 7.5 times against the code as written, 1.5 to 2.8 against one-pass NumPy, nothing on the splitting | **Stands as a kernel micro-benchmark** (ratios recomputed from the memo's table; not rerun). |
| E3 | "Iteration count grew like P to the power 1.6 to 3.1" | **Unproven as an exponent.** One size step, different boards, different penalties, a 500-iteration logging floor. The same raw files give anything from -1.1 to 3.3 (section 5). |
| E4 | Crossover against row scan between 8x8 and 12x12 | **Arithmetic stands** (table recomputed within 3 percent). Scope: one lift solve against a full enumeration, along one colour ladder, with a first-moment tree. The crossing size is set by the tree model, not by the exponent. |
| E5 | 916 GB for the splitting at 16x16 | **Stands with a unit correction**: 916 GiB, which is 984 GB. |
| E6 | The splitting "needs facial reduction to converge at all" | **Unproven.** The runs without it used penalties 0.1 to 10 for at most 1,400 iterations; with it, the same penalties do not converge either. |
| E7 | "A tolerance of 1e-5 cannot tell an exact lift from a leaky one", hence no single precision | **Stands with a narrower scope**: for this splitting, the constraint violation is not a convergence test (one trajectory). Not shown for a primal-dual solver. |
| E8 | The spectral bundle's good rates need strict complementarity, "unlikely" here | **Contradicted by a later measurement** (theory C: it holds on two boards). The ranking of that method should be redone. |
| C1 | The modified script reproduces the engineering trajectory on seed 1 | **Stands** (4.9585e-6 at iteration 4,500 in the log). |
| C2 | "First largest-leak certificate from method B": 8.3e-9 per cell at residual 1e-7 | **Stands**, one board, one run. It is a converged primal solve, with no dual bound computed. |
| C3 | The kill line: 0.05 per cell on 2 of 3 boards | **Stands with a narrower scope.** It is a line for stopping engineering, not a test of exactness: seed 3 at 5x5 leaks 0.045 at most so far and would pass it. |
| C4 | Deviation 1, the penalty rule | **Unproven and unsafe as combined with deviation 2.** Never fired, so the 6x6 runs were fixed-penalty runs at a value tuned on one 5x5 board. If it ever raised the penalty, the stop test would accept a point the objective had not acted on (section 6). |
| C5 | "The two stayed within a factor 2" | **Wrong for the leak run** (ratio up to 4.1), true for the minimum-norm run. Under the threshold of 10 either way. |
| C6 | The run table (one running, five queued) | **Stale.** All six were killed at 22:52 with no result. The memo was not updated. |
| S1 | Project, 22:20: "First certificate of exactness on a 5x5 board" | **Says more than the memo.** Not the first (the lift study certified the same board at 20:38), and it certifies zero leak on one board, not exactness at a size. |
| S2 | Project, 20:37: "Factored at the guaranteed rank (25,782) the matrix needs 30 GiB" and "The 5x5 exactness is real inference" | **Repeats T8's error**, and states as a fact what was then three points, one unconverged. |
| D1 | D item 2: exactness lives in nonnegativity between far-apart cells; without it 0.16 to 0.72 per cell leaks | **Stands with a narrower scope.** Shown on the four boards where the full lift reaches clean points. On seed 3 the three forms give the same point, so far pairs do nothing there. "Far" at 5x5 means distance 2. |
| D2 | D table: full lift "exact on 4 boards of 5" | **Stands with a narrower scope**: three by a converged largest-leak solve (seeds 1, 2, 4). Seed 0 is counted on a clean point at penalty 3,000; its largest-leak run has not converged after 28,645 iterations (0.0076 per cell and falling slowly). Undecided, not exact. |
| B1 | B item 2: the low-rank heuristic solves 46 of 50 at 5x5 with no entrywise nonnegativity | **Stands.** It supports "the heuristic's 5x5 success is not evidence about the lift". It does not support "nonnegativity is useless to it": at an equal budget of 5,000 evaluations it solves 30 of 50 against 71 of 100. |

## 1. Corrected statements that may be used

1. **Exactness at 5x5.** "On generated 5x5 (3,4) toys the degree-2 lift is not exact in general.
   Five boards were run. On three of them (seeds 1, 2 and 4, with 6, 8 and 2 solutions) a converged largest-leak
   solve finds no mass on any placement outside the solutions. Seed 0 (36 solutions) is
   undecided: clean points exist, the largest-leak solve has not converged.
   On seed 3 (403 solutions) the lift keeps weight on 10 placements that no solution uses:
   0.0047 of the mass per cell at its minimum-norm point (violation 5e-9), and a point feasible
   to 1.5e-9 carries 0.042 per cell there, with single placements at 0.21. One solver family;
   not yet confirmed by a primal-dual solver. The three boards certified clean have 2 to 8
   solutions."
2. **What "exact" means in these memos.** "No feasible point puts mass on a placement outside the
   solutions" (support). Equality with the convex hull of the solutions was checked only at single
   points (hull distance zero on seeds 1, 2 and 4), never by an optimisation.
3. **Odds.** "The theory memo's 60 / 20 / 2 percent are its author's estimate. They were formed
   before a 5x5 board was found inexact, on a scale that this board breaks. No 6x6 reading
   exists."
4. **Memory at 16x16.** "One dense copy of the matrix is 183 GiB (197 GB). The factor at rank
   25,782 is 30 GiB, but that rank is the equality count of the form without entrywise
   nonnegativity, which is inexact at 5x5; with nonnegativity kept by multipliers the store is
   about 119 GiB (128 GB), and no result guarantees a rank below P. The convex splitting needs
   916 GiB (984 GB)."
5. **Crossover.** "If the lift were exact and its iteration count grew like a power of P no
   larger than 2.5, one solve on 14 cores would pass a full row scan at 10x10 (5,10); for any
   power up to 7, at 12x12. This is arithmetic on a first-moment tree and one measured size step.
   It is not a forecast."
6. **Iteration growth.** "From 4x4 to 5x5 (P times 3.5) the iteration count rose 7 to 47 times in
   the pairings the memo chose. No exponent can be quoted from one step."
7. **Convex 6x6.** "Validated on one 5x5 board. Six 6x6 runs were started or queued and all were
   killed without a reading. The automatic penalty never acted."
8. **The low-rank heuristic.** "At 5x5 it finds solutions by rounding far from feasibility, and it
   still does with entrywise nonnegativity removed (46 of 50). Its success there says nothing
   about the lift."

## 2. "Exact at 5x5": the evidence

### 2.1 The measure

Code read: `scripts/geometry-sdp-lift/measure.py` (`kills`), `scripts/relaxation-engineering/admm.py`,
`scripts/convex-6x6/admm.py`, `scripts/theory-d/admm_var.py`. All four compute the same thing:
the sum of x over the placements that appear in no enumerated solution, divided by the number of
cells (25). The enumeration is the shared row-scan search of `scripts/far-ideas-geometry/common.py`.

Recomputed (`check_mass.py`, output `raw/check_mass.jsonl`): the placement list, the pin and the
solutions rebuilt without the memos' board code, with a search that scans the cells in reverse
order. Solutions: 36, 6, 8, 403, 2 on seeds 0 to 4, using 211, 93, 106, 373, 44 of 478
placements. Both match. The mass recomputed on eleven stored points agrees with the memos to the
printed digits (for example 0.004701379619391204 on seed 3, 2.86e-8 on the SCS point of seed 1).

Two limits of the measure, neither stated in the theory memo, which calls it "the test of
exactness":

- It tests the support. A feasible point made only of solution placements but outside the hull
  of the solutions would pass. On boards with one solution the two notions coincide. On the
  others the hull was checked at the points found (my recomputation: hull distance 0, 0 and 0 on
  the splitting's points of seeds 1, 2 and 4; 2e-6 and 2e-5 on the SCS points of seeds 1 and 2).
- "Per cell" divides by 25. On seed 3 the 0.0047 is 0.118 of total mass on ten placements, each
  holding 0.003 to 0.019. Under the largest-leak objective single placements reach 0.22.

### 2.2 Seed 3 (403 solutions): real, artefact of convergence, or artefact of enumeration?

**Enumeration: ruled out.** The painter rejects a tile set with two tiles equal up to rotation or
with a rotation-symmetric inner tile (`common.gen`), so no placement is a duplicate of a solution
placement under another name. My reverse-scan search finds the same 403 solutions and the same
373 placements. The ten leaking placements are in none: cells 6, 8, 13, 17, 18 with tiles 6, 11,
16, 17, 18 (five inner cells, five tiles).

**Convergence: five checks, all against an artefact.**

1. *Trajectory.* In `scripts/theory-d/raw/p_s3_full.jsonl` the violation falls from 5.5e-7
   (iteration 500) to 4.5e-9 (iteration 22,648) while the mass goes from 0.00501 to 0.00470 and
   flattens (0.00472 at 11,000). Over the last 11,000 iterations the mass moves 0.4 percent for a
   halving of the violation. On seed 0 with the same code and penalty, an unconverged run,
   the mass falls in proportion to the violation (0.0023 at 4e-7, 0.00046 at 7e-8). The two
   behaviours are distinct.
2. *Three forms, one point.* Full nonnegativity, distance 2 and distance 1 end at the same
   objective (4.0767) and the same ten placements. On every other board distance 1 leaks 0.16 to
   0.72.
3. *The other objective* (mine, `run_leak.sh`, the convex memo's script unmodified, 570 s): the
   largest-leak run RISES from 0.024 to 0.045 per cell and holds there (0.04535 at iteration
   5,400, 0.04506 at 9,843) with the violation at 1.5e-6 and the step still shrinking. On seed 1
   the same objective starts at 0.56 and collapses to 8e-9. The point was rechecked with
   constraints rebuilt independently (`verify_point.py`): 26,418 conflict pairs and 87,585 free
   pairs as in the memos, smallest eigenvalue -5e-16, largest violation 1.5e-6, the same ten
   placements, up to 0.22 each. Not converged: 0.045 is the value of an iterate, not the maximum.
4. *Objective gap* (`hull_minnorm.py`). The smallest squared norm over the hull of the 403
   solutions is 4.1243. The lift's minimum-norm point has 4.0767 at violation 5e-9: 0.048 lower.
   On seeds 1, 2 and 4 the same comparison gives +2e-4, +7e-6 and +3e-9 (the lift's point is on
   the hull's minimum, to the convergence of each run). An exact lift cannot go below the hull.
5. *Polish* (`polish_feasible.py`, my own implementation, no objective, no penalty). Started at
   the leak iterate, a plain feasibility iteration brings the violation from 1.5e-6 to 1.5e-9 at
   a steady linear rate over 18,800 iterations. The mass off the solutions goes from 0.0451 to
   0.0419 per cell and settles (0.04193 at 2.5e-9, 0.04191 at 1.5e-9): 1.048 units in total on
   the same ten placements, the largest at 0.211. Smallest eigenvalue -2e-15. Three decades of
   violation moved the mass by 7 percent, and the steps shrink geometrically. An artefact of
   tolerance would have to vanish with the violation; this does not.

**What is missing.** The feasible set has no interior, so a small violation does not bound the
distance to the feasible set in general; a proof needs an exactly feasible point or a solver of
another family. Theory D proposes the right run: seed 3 through the lift study's SCS model with
the largest-leak objective, about 20 core-minutes. SCS was outside my allowed tools.

**Ruling.** A real failure of exactness on this board, at the level of evidence above. It is the
only board of the five with hundreds of solutions, so "many solutions" is a candidate cause and
nothing more: one board.

### 2.3 Board by board

Mass off solutions per cell. "Leak solve" is the largest-leak objective.

| seed (solutions) | SCS point | SCS leak solve | splitting, minimum norm | splitting, leak solve | status |
|---|---|---|---|---|---|
| 0 (36) | 9e-4, inaccurate at 1,100 s | 2.4e-4 (0.0059 total), inaccurate | 3e-11 at penalty 3,000 (not converged, not the minimum-norm point); 4.6e-4 at penalty 300, falling with the violation | 0.0076 at violation 1e-6 after 28,645 iterations, falling slowly, not converged (mine, two 570 s legs) | undecided |
| 1 (6) | 3e-8, solved | 1.6e-6 (4e-5 total), solved | 2e-13 at residual 1e-7 | 8e-9 at residual 1e-7 (two studies, same run) | certified, two solver families |
| 2 (8) | 1.8e-5, solved | 2.7e-5 (6.8e-4 total), inaccurate | 3e-12 | 0 at residual 1e-7, 14,742 iterations, rank 8 (mine, second leg) | certified (splitting) |
| 3 (403) | not run | not run | 0.0047 at violation 5e-9 | 0.045 at violation 1.5e-6, not converged; polished to 0.042 at violation 1.5e-9 (mine) | not exact |
| 4 (2) | not run | not run | 1e-15 | 8e-9 at residual 1e-7, 6,028 iterations, rank 2 (mine) | certified (splitting) |

Seed 0 is undecided in both directions. Its clean points come from runs at penalty 3,000 that
had not converged; at penalty 300 the minimum-norm run sits 0.005 BELOW the hull's minimum norm
(5.1615 against 5.1665) at violation 7e-8 with 4.6e-4 per cell still off the solutions, and my
largest-leak run is at 0.0076 per cell after 28,645 iterations. All three are falling. It may be
a slow clean board or a small leak; nobody has a converged solve on it. It is the board of the
study log's first reading (19:43).

The three boards of the lift memo and the theory memo's SAC comparison are seeds 0 to 2. Seeds 3
and 4 entered with the low-rank study. No one chose the boards for their solution count.

### 2.4 Where exactness lives (theory D, item 2)

Read from `scripts/theory-d/raw/` (`fin.py`): with all zeros kept and no entrywise nonnegativity,
seed 1 ends at 0.527 per cell at violation 6e-8 after 14,682 iterations; with nonnegativity
between touching cells only, 0.500 (seed 1), 0.430 (seed 2), 0.162 (seed 0), 0.723 (seed 4) at
violations 4e-7 to 2e-6; the largest-leak objective on seed 1 at distance 1 gives 0.64. These are
near-feasible leaky points on boards where the full form is clean, so the weakened forms are
inexact there. The claim stands for those four boards. Three limits:

- On seed 3 the full form, distance 2 and distance 1 give the same leak. Far-pair nonnegativity
  is necessary on four boards and not sufficient on the fifth.
- "Far apart" at 5x5 is distance 2 on a board of side 5, as the memo says. Distance 2 itself is
  clean on seeds 1 and 2 only; on seeds 0 and 4 it had not converged (0.077 and 0.087).
- The share of free pairs within distance 2 is given as 73 percent in theory D and 41 percent in
  theory B. One of the two masks is not what its memo says. Not resolved here.

### 2.5 The low-rank heuristic and the lift (theory B, item 2)

B's table: 100 of 100 with all free pairs, 100 of 100 at distance 2, 98 of 100 at distance 1, 46
of 50 with none, where the last row had half the time limit. At equal evaluation budgets
(B, appendix): 71, 94, 99 of 100 against 30, 41, 45 of 50 within 5,000, 10,000 and 15,000
evaluations. Every run stops with 45 percent of the free pairs negative, violation near 0.05 and
0.18 to 0.26 of the mass per cell off the solutions.

So the heuristic at 5x5 is a local search with a rounding step that fires far from the feasible
set, and it works without the constraint that makes the lift exact. Its 5x5 success rate is not
evidence for or against the lift, and the study log's 21:50 heading "Low-rank form works at 5x5"
should read "the rounding heuristic finds solutions at 5x5". Nonnegativity is not worthless to
it: without it the early success rate is lower (30 of 50 against 71 of 100 at 5,000 evaluations;
by a two-sided exact test on those counts the difference is not secure, p about 0.2). Whether it
matters at 6x6 is untested.

## 3. The odds 60 / 20 / 2

**Derived or estimate.** Estimate. The memo marks the table [G] and calls its basis weak. The
study log's 20:37 entry reports them with that caveat. The engineering memo then quotes them four times
as the condition under its whole scaling table, without the caveat; that is where they harden.

**The R scale.** R = sqrt(P (1 - rho) / rho) / N is the theta value of a structureless graph of
the same density, over N. Three faults:

1. The lift's question is not the theta value. With a solution present the value is N at every
   size, because cells are cliques. Exactness is about the optimal FACE. R says how far density
   alone is from certifying the value, which is already certified.
2. R rises with the side (1.2, 1.6, 2.1, 2.7, 3.3, 9.3; recomputed). Any monotone function of the
   side would "separate" boards measured exact at sides 3 to 5 from sides not measured. The
   threshold 2.0 comes from a frameless 3x3 family that is not on the ladder.
3. The regularity "everything measured exact has R <= 2.0" no longer has a converse use: seed 3
   is a 5x5 at R = 1.6 and is not exact. Something other than size and density decides.

**Is decay with size sound?** As a direction, yes on two grounds that need no scale: the problem
is NP-complete, so a polynomial lift cannot be exact on all boards of all sizes unless P = NP
(worst case, not this distribution); and the one inexact board found is at the largest size
measured. As a rate, no. The literature items are lower bounds for refuting or detecting random
sparse instances; the memo applies them "in spirit" and says none is proved for a lattice or
gives a size. I did not open the pages; the attributions I could match from memory (theta below
Kesten-Stigum, planted clique, cardinality constraints, graph matching relaxations) are correctly
described as far as I recall them.

**What the number should be a probability of.** The memo's sentence is "exact at 6x6". Since
exactness varies between boards of one size (three certified clean, one undecided, one not, at
5x5), the claim must name a
share of boards and a family. The convex memo's kill line uses "2 of 3 boards", which is a
different event from the theory memo's.

**Relevance to the real board.** With its clues the real board has about one solution. All boards
measured clean have few solutions, and the inexact one has 403. If the solution count is the
cause, toys picked for few solutions are the relevant ladder and seed 3 is a side fact. That is a
hypothesis from one board; the planned 6x6 boards have 7, 4 and 14 solutions.

## 4. Memory and cost at 16x16

`recompute_figures.py`, output `raw/recompute_figures.txt`. P = 156,816, zeros 3.322e8, free
pairs 1.196e10 (theory memo's formulas; the table of P, zero share and R reproduces exactly).

| figure | memo | recomputed |
|---|---|---|
| one dense copy, 8 P^2 | 183 GiB | 183.2 GiB = 196.7 GB |
| rank from the equalities, m = zeros + P + 1 | 25,782 | 25,782 |
| factor at that rank, 8 P r | 30 GiB | 30.1 GiB = 32.3 GB |
| multipliers for Y >= 0, 8 F | 96 GB | 95.7 GB = 89.1 GiB |
| factor plus multipliers | not given | 119 GiB = 128 GB |
| SCS as coded, 72 F | 0.9 TB | 0.86 TB |
| splitting, five dense matrices | 916 GB | 916 GiB = 984 GB |
| chordal block | 77,224 | 77,224 |
| 10 P^3 | 3.9e16 | 3.9e16 |

**The guarantee.** The theory memo's verdict reads "factored, its matrix needs 30 GiB on the 16x16
at the rank that carries a guarantee (Y >= 0 must stay)". Its own section 4 table prices that
rank under "factored, no Y >= 0", and its Appendix D says the generic guarantee "is not known to
hold here". The verdict line joins two rows that the table keeps apart. With the inequalities
counted as constraints (most are active: theory C finds 585 to 23,591 of 86,500 free pairs
positive at 5x5), the count passes P(P+1)/2 and the bound is P. The engineering memo says this
correctly ("the safe rank is order P"). Also, a rank-one optimum exists whenever the board has a
solution, so the existence of a low-rank solution was never in doubt; the open point is whether a
local method finds it, and no theorem cited covers this program.

**Crossover.** The scaling table reproduces from its Appendix F inputs to within 3 percent (my
5x5 and 6x6 one-core times are 89 s and 70 min against 91 s and 71 min). With the memo's
assumptions, one solve on 14 cores beats row scan on 14 cores at 10x10 if the exponent is under
2.5, and at 12x12 if it is under 7.1; at 8x8 never. A row-scan tree 2^5 larger or smaller moves
those bounds by about 1. So the statement "between 8x8 and 12x12" is robust to the exponent and
rests on: the lift being exact there; a power law holding over 1.5 to 2 decades from one step;
the first-moment tree (28 bits between 8x8 and 10x10); the colour ladder (at 8x8, one colour
fewer moves the tree by 9 bits); "14 cores = one core / 6" [G]; one solve where a reading needs
two; and a 5x5 base of 4,500 iterations that is the best penalty per board. The 12x12 row on the
card (65 GiB for five matrices in 80 GB, before any workspace) is marginal.

## 5. The iteration-growth exponent

The memo: this splitting 500 to 3,500..6,000 gives 1.6 to 2.0 "or more"; SCS 175 to 3,025..8,200
gives 2.3 to 3.1. Recomputed, those four numbers are right. What they are worth:

- The 4x4 count of 500 is the first line of a log written every 500 iterations. The true count is
  anywhere under 500; at 100 the exponent would be 3.3.
- The 4x4 board in that pairing has one solution and a rank-one point; the 5x5 boards have 6 to
  36. The other 4x4 board (8 solutions) was run only at penalty 1 and took 14,500 iterations to
  the same threshold, more than any 5x5 run: paired with 5x5, exponent -0.7 to -1.1.
- The penalty was tuned per size (30, then 300 or 3,000). At a fixed penalty of 300, seed 0 at
  5x5 needs over 23,000 iterations.
- SCS at 4x4 is quoted as "175 to 475" in the same memo; with 475 the range is 1.5 to 2.3. The
  8,200 is a run that did not converge.
- To a residual of 1e-7 with the minimum-norm objective, the only matched pair is 1,387 (4x4) and
  35,602 (5x5 seed 1, from the convex memo's validation): 2.6.

What can be claimed: at the best penalty tried, a 5x5 board costs 7 to 47 times the iterations of
a 4x4 board, on one to three boards per size. The memo says this is its weakest number; the
scaling table still extends it over two decades. Theory C's later reading (the count is a drift
of constant speed toward a far dual point, then a collapse) fits the leak trajectories I read and
gives no law in P either.

## 6. The penalty rule in `convex-6x6.md`

Read: `scripts/convex-6x6/admm.py`, `raw/val5/`, `raw/val5_scaled_balance_failed/`.

- The textbook rule compares the primal residual with the penalty times the step. It parked the
  penalty at 4.7 and 9.4 and stalled (confirmed in the log: 9.375, mass 0.61 at iteration 6,100).
  The replacement compares the residual with the step itself.
- In the validation the replacement never fired. The ratio stayed between 0.62 and 1.25 in the
  minimum-norm run and between 0.63 and 4.1 in the leak run; the trigger is 10. The memo says so
  ("untested in action"). Consequence: the six 6x6 runs were fixed-penalty runs at 300, a value
  tuned on one 5x5 board with 6 solutions, in a method where the engineering memo moved the best
  penalty by 100 times across one size step and where 300 fails to converge seed 0 at 5x5 in
  23,000 iterations.
- The replacement has no feedback through the penalty. In the textbook rule, raising the penalty
  raises the dual residual, which stops the rise. Here neither side contains the penalty, so
  nothing shown bounds it short of the clamps at 1e-3 and 1e7.
- Combined with deviation 2 this is unsafe. The stop test uses the residual and the step, not the
  penalty times the step. The objective enters each iteration as c / penalty. At 300 the test is
  sound: at the stop the scaled dual residual is 2.6e-5, an error far under 0.05. If the rule
  drove the penalty to 1e5 or more, the objective's push per iteration would be under the 1e-7
  stop threshold's reach, and the run could stop "converged" at a feasible point the objective
  never moved. For a largest-leak run that is a false "exact". It did not happen in any run; it
  is a fault of the design.
- Fix before any rerun: fixed penalty with a small grid (the engineering memo's practice), or
  keep the rule and add the penalty times the step to the stop test with a threshold tied to the
  kill line. Theory C reports that with no objective there is no penalty at all; a largest-leak
  run cannot use that, but a first feasibility pass can.

The certificate on seed 1 is not affected: penalty 300 throughout, the last 85 iterations
contract by about 10 times per 100, and the SCS solve agrees.

## 7. Presented as measured, on a single run

| where | statement | basis |
|---|---|---|
| E verdict | one hand-set penalty changed the count by more than 30 times at 4x4 | one board, two runs |
| E verdict, appendix B | 3,500 to 6,000 iterations at 5x5 | one run per board, penalty chosen per board |
| E section 2 | violation 2.7e-5 with 0.14 of the mass still off the solutions | one trajectory |
| E section 3 | a second thread changes the eigendecomposition by about 10 percent | one pair of timings (0.405 and 0.363 s) |
| E section 3 | 92.7 percent of the time in one LAPACK call | 5x5 only; the convex memo's runs of the same code show 70 and 78 percent with its logging |
| E appendix B | facial reduction needed; Anderson acceleration gains at most 2 times | pilots of 1,100 to 2,500 iterations on one board; two 4x4 runs |
| E section 2 | the profile of `bm.py` | one run per rank |
| T appendix B | without Y >= 0, 0.003 per cell left on one 4x4 board | one board, two runs, both "inaccurate" |
| T appendix D | factored runs converge at 4x4 and not at 5x5 | two 4x4 boards; three runs on one 5x5 board |
| T section 3 | the lift refutes 8 of 8 dead prefixes | one 6x6 board |
| G section 3 | backend choice (Accelerate, MKL) | one board, one tolerance |
| G section 1 | entropy objective never converges | one 4x4 board |
| C reading | the leak objective converges "about three times faster" | one board; the minimum-norm run later ended at 35,602, so six times |
| C table | 1.72 s per iteration at 6x6 | iteration 100 of one run under load |
| D item 3 | seed 3 is not exact | one board, one solver family, two penalties |

The lift memo's seed 1 numbers are the exception: reproduced by an earlier run with another
backend, by the splitting in two studies, and by my recomputation.

## 8. The study log against the memos

Entries from 21:50 onward, plus the two earlier ones that summarise memos 1 and 2.

| entry | sentence | finding |
|---|---|---|
| 22:20 | "First certificate of exactness on a 5x5 board (the earlier readings were points)" | The memo says "first largest-leak certificate from method B". The study log's own 20:38 entry records the SCS certificate on the same board. And it is zero leak on one board. |
| 20:37 | "Factored at the guaranteed rank (25,782) the matrix needs 30 GiB on the 16x16 against 183 GiB dense" | Carries the memo's own conflation (ruling T8). |
| 20:37 | "The 5x5 exactness is real inference" | The memo had, at that time, three minimum-norm points, one unconverged, and says so. |
| 20:38 | "The lift sets 173 to 325 placements ... below 1e-4" | Faithful, but 173 is the unconverged board. |
| 21:50 | heading "Low-rank form works at 5x5" | Every run stopped at its time limit; what works is the rounding (see 2.5). The body of the entry is accurate. |
| 21:56 | the engineering summary | Faithful. It keeps "points, not certificates". |
| 22:52 | "the exactness question at 6x6 is open" | Faithful. |
| 19:43 | "a low-rank, matrix-free form removes the memory wall" | Before the range examined. Not supported by any memo: with nonnegativity kept, the multipliers are order P^2 (the Rust memo's 00:18 entry corrects it). |

the plan document section 9 predates these memos and says nothing about the lift.

## 9. What I could not check

- Any 6x6 quantity. No 6x6 reading exists anywhere in this study.
- Seed 3 with a primal-dual solver, or an exactly feasible leaky point. Until one exists, "not
  exact on seed 3" is numerical.
- Equality with the hull by optimisation on any board with several solutions.
- The time stamp of the 6x6 prediction (20:00:44, before any 6x6 file was opened).
- Every cited page. No web request was made.
- The Rust kernel benchmark, the `bm.py` profile, the eigendecomposition timings, the GPU figures:
  read, ratios recomputed from the tables, nothing rerun.
- The eight small boards of the theory memo's Appendix B: raw rows read, not rerun (SCS).
- The first-moment tree sizes behind the row-scan columns.
- The 73 percent against 41 percent for "distance 2" between theories D and B.
- Whether the solution count causes the failure on seed 3. One board.
- The lift memo's z scores and near-solution controls: read, not recomputed; they do not bear on
  the rulings above.

## Appendix. re-check runs

All in `scripts/recheck-convex/`, Python through `uv run --with numpy --with scipy python -I -B`.

| script | what | output |
|---|---|---|
| `check_mass.py` | independent placements, pin and enumeration (reverse scan) for 5x5 seeds 0 to 4; mass off solutions and hull distance on the stored points | `raw/check_mass.jsonl` |
| `recompute_figures.py` | sizes, R, memory, exponents, scaling table, crossover bounds (stdlib) | `raw/recompute_figures.txt` |
| `run_leak.sh` | the convex memo's `admm.py`, unmodified, largest-leak objective, penalty 300, 570 s, on seeds 3, 0, 2, 4 | `raw/leak_s*/`, `raw/leak_s*.out` |
| `verify_point.py` | every constraint of the lift measured on a checkpoint, with conflict pairs and face rebuilt independently | `raw/verify.jsonl` |
| `polish_feasible.py` | plain feasibility iteration, no objective, from the seed 3 leak point; own implementation | `raw/polish_s3.jsonl` |
| `hull_minnorm.py` | smallest squared norm over the hull of the solutions against the lift's minimum | `raw/hull_minnorm.jsonl` |

Notes. The largest-leak runs of seeds 0 and 2 did not converge in 570 s (8,670 and 11,101
iterations under load) and were each resumed once from their checkpoint for up to 570 s more, as
the script allows; no process ran over 10 minutes, but the two legs together are one solve of
about 19 and 11 minutes. I say so because the preset limit was per run. Seed 2 shows why a
time-limited leak run proves nothing: it held 0.24 per cell at iteration 11,101 and collapsed to
zero at 14,400. The reading on seed 3 does not rest on such a run alone (checks 1, 4 and 5).
A wait loop of mine polled a progress file every 5 s while a run was going; it used no
measurable CPU. Checkpoints (1.8 MB each) are left in `raw/leak_s*/`.
