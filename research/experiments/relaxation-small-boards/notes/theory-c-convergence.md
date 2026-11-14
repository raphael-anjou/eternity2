# Theory C: why the iteration count is what it is, and what would cut it

Draft of 2026-11-13, not independently checked. All boards are generated TOYS (5x5, frame 3, inner 4, seeds 0 to 4): nothing here is Eternity II progress. Marks: [M] measured in this study by `scripts/theory-c/`, [D] derived, [C] cited from a page I opened, [C-u] cited, not opened, [G] guessed. Proposals are assessed at the real size (P = 156,816, 1.2e10 pairs, 32 GB, 10 cores): a dense P x P matrix or an eigendecomposition of order P fails there.

## 0. The reading first

1. **The two counts are different objects.** The splitting's thousands of iterations are a constant-speed drift: the residual sits near 3.1e-3 for 2,800 iterations while the fixed-point variable grows linearly in norm from 7.6 to 17.3; then the rank falls from 304 to 6 in 50 iterations and the residual drops to 1e-8 in 800 more [M]. The low-rank count is not a convergence count: when the rounding succeeds (8 of 16 baseline runs inside 12,000 evaluations, at 3,270 to 8,830) the factor has rank 30 to 32 of 32, the penalty is 106 to 1,125 in 7 of the 8, and 0.2 to 65 percent of the mass is still off the solutions [M].
2. **The degeneracy hypothesis is half right.** The reduced program has no interior point: in a face of order 433 the solutions span 2 to 32 directions (251 on the board with 403 solutions) [M]. But the optimum is not degenerate: at the splitting's fixed point, primal rank plus dual rank is 6 + 427 and 8 + 425 without objective (two boards) and 1 + 432 with a random linear one, each equal to 433, no eigenvalue within 1e-6 of zero [M]. Strict complementarity holds and the dual is attained.
3. **The slow tail and the penalty sensitivity of the splitting both belong to the minimum-norm objective** [M, D]. Without objective seed 1 reaches 1e-8 in 3,790 iterations and the iteration has no penalty parameter at all; with the norm (penalty 300) the residual is still 1e-4 at 6,000.
4. **Objective.** In the convex splitting a random linear objective gives a valid rounding in 2 runs of 3 (iterations 1,750 and 1,175; never by 6,000 on the third), where no objective and minimum norm never round in 6 runs: their limit is a mixture of all solutions [M]. In the low-rank form my prediction was wrong: inside 12,000 evaluations the linear objective, no objective and a three times heavier norm solve 0 of 5 each and the concave push 2 of 6, against 5 of 5 and 5 of 6 for the current objective on the same starts [M].
5. **Nothing I tested cuts the low-rank count by 10 times at 5x5.** Thirty inner iterations per outer step instead of 300: 6 of 6 solved, median 2,881 against 5,002 (5 of 6), under 2 times. A hint (one true placement pinned) does not help: 6 of 14 against 7 of 14. The ranking in section 4 is about growth with P: argued, not measured. Every count here is 4 to 16 runs.

## 1. Diagnosis

**A face of solutions [M, D].** When the lift is exact the feasible set is the convex hull of the solutions' rank-one matrices, so its points have rank at most the dimension of the solutions' span: 32, 6, 8, 251, 2 on seeds 0 to 4, in a face of order 433. Strict feasibility fails by 182 to 431 directions. An exact lift is by definition collapsed onto a tiny face: on the real board with about one solution the deficit is every direction but one [D]. The literature's price is a sublinear rate and a root-law error bound [C, Appendix E]; the trajectory does not confirm a root law (slopes from 0.1 to 0.9) [M]. Measured: at iteration 500 the largest violation is 1e-4 with 47 percent of the mass off the solutions; at 2,500, 1.9e-5 with 10 percent. Part of that is units (a typical entry of Y is 2.7e-3) [D]. An absolute tolerance says little before the collapse.

**What the plateau is [M, D].** The two projections disagree by a nearly constant amount and the iteration adds it to its variable at every step: it is building the dual certificate (a PSD matrix of rank 427 annihilating the six solutions) in steps of 0.0035. Count = distance to the fixed point over the residual. I expected that ratio to be units and tested the one free scale of a pure feasibility splitting, a congruence writing the lifted vector as (1, s x). It is not: s = 0.33, 1, 3 change residual and norm 80 times together and move the collapse by under 25 percent (3,675, 3,000, 2,900) [M]. The ratio is geometry, two sets touching along a face without crossing [G]. So the scale mismatch explains neither count nor sensitivity there.

| observation | cause | mark |
|---|---|---|
| splitting: thousands of iterations, then collapse | drift toward a far certificate; finite identification, then a linear rate [C, Liang, Fadili, Peyré] | M, D |
| splitting: slow tail | the minimum-norm objective | M |
| splitting: 30 times from one penalty | the objective's weight: at penalty 1 its prox shrinks x by 40 percent per iteration against the constraints | D |
| low rank: never converges formally | the multipliers must reach the same far certificate and the capped inner solves never let them; irrelevant to a solver | D; M Rust study: violation stalls near 2e-4 at penalty 1e5 |
| low rank: thousands of evaluations to a rounding, or none | two routes: an early window at penalty 100 to 1,000 (half my baseline runs), otherwise the penalty cap. Inner caps of 300, 100, 30 give medians near 5,000, 5,510 and 2,881 although the cap comes 3 and 10 times sooner: work, not schedule | M |
| low rank: fixed penalties, scalar sums stall | sums are the fragile class: raising pair penalties alone left sums violated by 0.08 to 0.26, 0 of 4 solved | M |

## 2. The literature, assessed at the real size (Appendix E)

The closest published design is a Riemannian low-rank augmented Lagrangian for doubly nonnegative relaxations [C, arXiv 2502.13849]: it states that Slater's condition fails for this class, carries the lifted sums as the affine A R = b e_1^T on the factor (`bm.py` penalises them), penalises only nonnegativity, and reports a smaller penalty and fewer outer and inner iterations. Row-by-row descent on unit rows claims an order of magnitude on diagonal-constrained programs [C, arXiv 1706.00476]. Anything built on a full eigendecomposition (the splittings, Anderson, restarts, SDPNAL+) fails at scale, and Anderson or restarts speed a contraction where the measured count is a drift.

## 3. The objective

**Exact lift [D, M].** The feasible set is a polytope whose vertices are the solutions. Minimum norm (and entropy [G]) pick an interior point: the best certificate (its support is the union of the solutions) and the worst point to round. A random linear objective has a unique optimum for almost every draw [C, arXiv 1005.1082], at a vertex, hence at a solution [D]. Measured once it is so, strictly complementary (1 + 432); the two other runs stopped at 6,000 iterations at rank 2 and 3, unconverged. Its price is a later collapse (3,800 or 5,100 against 3,000 on seed 1); the rounding, when it comes, comes before any collapse.

**Inexact lift [D].** The optimum of a random linear objective is then a fractional extreme point loaded on whatever the draw favours (the largest-leak objective is the worst draw by construction). The interior point still holds every solution in its support, which a pruning decoder needs. Linear to solve, interior to certify or prune.

**The low-rank form flips this [M].** Successes come early, at low penalty, far from feasible; the alternatives reach the penalty cap first. A concave push, a linear pull or a hint commit rows early, and in sphere coordinates a row at x = 0 has no first-order sensitivity, so commitments are slow to undo [D]. The squared norm is the one term pulling every x to the middle. Why three times the norm is also slower I cannot explain.

## 4. Ranked changes (low-rank form only)

No change below has a measured 10 times at 5x5.

| rank | change | argument | the one assumption | growth of the count with P |
|---|---|---|---|---|
| 1 | Exact row updates swept in board order instead of an all-rows L-BFGS step | successful runs spend thousands of quasi-Newton steps whatever the schedule. A sweep carries a change across the board in one pass, with no line search; published gain on unit-row programs: an order of magnitude [C]. Memory P r, a sweep costs one evaluation | the count is a relaxation time, not a search time | simultaneous updates: n steps at best, n^2 by diffusion, to cross the board, so P^(1/4) to P^(1/2) [D]. Sweeps: no such factor [G]. If it is a search time, exponential in N either way |
| 2 | Carry the cell and tile sums exactly (a linear projector: a 50 x 50 pseudo-inverse here, 512 x 512 at 16x16), penalise only the pairs | the published design reports a smaller penalty and fewer iterations from this move [C, no figures in the part I read]; every stall on record here is a sum stall [M; low-rank memo] | the sums, not the search, set the penalty the run needs | the sum penalty's curvature is the penalty times the cell size P/N, 19 at 5x5 and 613 at 16x16. Carried exactly, it stops growing: about root of 32, 6 times, fewer inner steps at 16x16 [D] |
| 3 | Scale the pair penalty with the board, as (P/N)^2 relative to 5x5 | a pair entry has size (N/P)^2: 2.7e-3 at 5x5, 2.7e-6 at 16x16. The window at penalty 100 to 1,000 [M] becomes 10^5 to 10^6 | the 5x5 balance of sums and pairs is right in relative units | unscaled: about 26 more outer steps and inner steps growing like P/N, 32 times at 16x16 [D]. Scaled: that factor goes. Both scale tests I ran at 5x5 came out null or negative, hence last |

Not ranked: Anderson, restarts, adaptive penalties, semismooth Newton and staircases improve an asymptotic rate a solver never reaches. Objective swaps and hints are measured slower or neutral. The shorter inner solve (under 2 times) is the Rust study's to tune.

**At the real size [D].** One evaluation is 1.7e12 flops (low-rank memo), tens of seconds on 10 cores: 4,500 evaluations are days, and years with the factors above. The obstacle is the growth of the count, which nobody has measured past 5x5.

## 5. The smallest experiment for each

1. Row sweeps: in a copy of `bm.py`, minimise the same augmented Lagrangian one row at a time (a few Newton steps on its sphere), in board order. Five boards, ten starts. Kill: median above 1,500 sweeps.
2. Sums exact: compose the variables with the linear projector on the sums and penalise the diagonal instead (P residuals replace 50 r). Same suite. Kill: median not under 2,000 evaluations.
3. Penalty scale: invisible at one size. Smallest test is 4x4 against 5x5 (P/N 9 against 19), baseline schedule against scaled, ratio of the two counts. Kill: same ratio.

## 6. Not checked

- Any size but 5x5 (out of scope): every growth law is an argument.
- Ten starts per board: 1 to 4 were reached.
- Strict complementarity beyond seeds 1 and 2, and on an inexact lift; linear objectives run to convergence or at other scales.
- Row sweeps, the Riemannian variety, second-order methods: not implemented.
- The Hessian spectrum in the rounding window (relaxation time against search time).

## Appendix A. Measurements

**A1. Low-rank form, evaluations to the first verified rounding** (`bm_obj.py`, r = 32, `bm.py`'s loop and schedule, greedy rounding tested after every inner iteration and verified by counting mismatches; SciPy L-BFGS-B; every run censored at 12,000 evaluations; each row compares a mode with the baseline on the (board, start) pairs both finished; `table.py`, `raw/suite/`).

| mode | pairs | solved, mode | median evaluations, mode | solved, baseline | median evaluations, baseline |
|---|---|---|---|---|---|
| current objective (baseline) | 16 | 8 | 10,415 | 8 | 10,415 |
| concave, minus the squared norm | 6 | 2 | over 12,000 | 5 | 5,002 |
| pair constraints rescaled ("eq") | 4 | 0 | over 12,000 | 4 | 4,493 |
| inner cap 100 | 5 | 5 | 5,510 | 5 | 4,294 |
| inner cap 30 | 6 | 6 | 2,881 | 5 | 5,002 |
| hint (centre cell of solution 0 pinned) | 14 | 6 | over 12,000 | 7 | 9,160 |
| random linear, no norm | 5 | 0 | over 12,000 | 5 | 4,294 |
| no objective | 5 | 0 | over 12,000 | 5 | 4,294 |
| three times the norm | 5 | 0 | over 12,000 | 5 | 4,294 |

Uncensored, from the first runs (cap 25,000): linear solved at 13,867 to 23,265 (4 runs), no objective at 14,280 and 16,125, concave at 4,897, 5,255, 13,847, 22,231, 24,170 and once not at all, all but two of these at the penalty cap of 1e5. Rescaled pairs: 4 of 4 unsolved at 25,000 with violations 0.08 to 0.26. Baseline runs that solve do so at penalty 106, 106, 179, 512, 665, 1,125, 1,125, 9,173, rank 30 to 32, violation 3.8e-3 to 1.0e-2. Baseline by (seed, start): start 0 solved on all five boards (3,270 to 6,320); starts 1 to 3 solved 3 of 11. The Rust study's own table shows the same two populations on the same boards (a few runs at 4,400 to 8,950, most at 14,000 to 24,500); I did not reconcile the proportions (different L-BFGS).

**A2. Hints** (posed question; the placement that solution 0 puts on the centre cell is pinned, its conflicting placements deleted; same starts; "none" = not solved inside 12,000 evaluations).

| board (solutions) | solutions left by the hint | placements deleted | starts | solved, no hint | evaluations, no hint | solved, hint | evaluations, hint |
|---|---|---|---|---|---|---|---|
| seed 0 (36) | 14 | 169 | 3 | 1 | 6,320, none, none | 0 | none, none, none |
| seed 1 (6) | 3 | 162 | 3 | 1 | 3,277, none, none | 2 | 5,710, 5,833, none |
| seed 2 (8) | 1 | 163 | 3 | 2 | 3,270, none, 5,285 | 1 | 11,874, none, none |
| seed 3 (403) | 14 | 147 | 3 | 2 | 5,709, 6,184, none | 3 | 5,222, 5,206, 5,617 |
| seed 4 (2) | 1 | 155 | 2 | 1 | 4,294, none | 0 | none, none |

Totals: 7 of 14 without the hint, 6 of 14 with it. The effect does not track the number of solutions left in the expected direction: where one solution is left (seeds 2 and 4) the hinted runs solve 1 of 5 against 3 of 5; where 14 or 3 are left they solve 5 of 9 against 4 of 9. On seed 0 one hinted run reached a nearly feasible mixture (violation 1e-4, rank 6, mass off 0.3 percent) that does not round. Small numbers: no effect shown either way.

**A3. Convex splitting as a diagnostic** (`dr_obj.py`, `admm.py`'s iteration with the objective as a switch, 6,000 iterations at most; residual = distance between the two projections; complementarity read at the last iterate; `raw/dr_obj.jsonl`).

| board | objective | scale s | iterations run | first valid rounding | mass off under 1e-2 / 1e-5 | end residual | rank of Z | dual rank | eigenvalues within 1e-6 | free pairs: primal positive / multiplier over 1e-8 / neither |
|---|---|---|---|---|---|---|---|---|---|---|
| seed 1 (6) | none | 1 | 3,790 | never | 2900 / 3000 | 9e-09 | 6 | 427 | 0 | 1,482 / 83,857 / 1,226 |
| seed 1 (6) | linear, tau 0.1, draw 5 | 1 | 6,000 | 1750 | 3750 / 3800 | 3e-06 | 1 | 432 | 0 | 300 / 83,466 / 2,799 |
| seed 1 (6) | norm, rho 300 | 1 | 6,000 | never | 4225 / 4400 | 1e-04 | 6 | 427 | 0 | 1,482 / 83,432 / 1,651 |
| seed 1 (6) | none | 3 | 4,635 | never | 2850 / 2900 | 1e-08 | 6 | 427 | 0 | 1,482 / 84,235 / 848 |
| seed 1 (6) | none | 0.33 | 4,078 | never | 3625 / 3675 | 1e-08 | 6 | 427 | 0 | 1,482 / 81,469 / 3,614 |
| seed 2 (8) | none | 1 | 6,000 | never | 5450 / 5600 | 5e-07 | 8 | 425 | 0 | 1,725 / 83,939 / 565 |
| seed 2 (8) | linear, tau 0.1, draw 5 | 1 | 6,000 | 1175 | 3100 / 3150 | 1e-03 | 2 | 431 | 0 | 390 / 83,825 / 2,014 |
| seed 1 (6) | none | 10 | 5,205 | never | 2900 / 3000 | 1e-08 | 6 | 427 | 0 | 1,482 / 84,291 / 792 |
| seed 1 (6) | linear, tau 0.1, draw 6 | 1 | 6,000 | never | 5050 / 5100 | 6e-04 | 3 | 430 | 0 | 873 / 83,959 / 1,733 |


Face order 433 in every row, so rank of Z plus dual rank equals 433 everywhere. Seed 1 without objective: residual 3.1e-3 to 5.4e-3 from iteration 200 to 2,500, norm of the variable 7.9 to 17.38, then stable to four digits.

**A4. Structure counts** (`degen.py`, `raw/degen.jsonl`): seeds 0 to 4 have 36, 6, 8, 403, 2 solutions spanning 32, 6, 8, 251, 2 directions of a face of order 433 (deficit 401, 427, 425, 182, 431); 211, 93, 106, 373, 44 placements are in a solution; at the full mixture 5,537, 1,482, 1,725, 23,591, 585 of about 86,500 free pairs are positive.

## Appendix B. Derivations

- **Conditioning and redundancy.** Facial reduction removes the sum equalities exactly (rank 46 of 50; the redundant four are harmless) [M]. What stays is the pairs: 27,438 zeros and about 85,000 active nonnegativities at the optimum against 93,961 entries of the face matrix [M]. More active constraints than dimensions: the multipliers are not unique. That primal degeneracy coexists with strict complementarity but removes the nondegeneracy semismooth Newton needs for its fast rate [C-u].
- **Deficit.** Feasible Z annihilates e_0 minus the indicator of each cell and tile (face of order 433 = 479 - 46). If the lift is exact, Z is a convex combination of z_s z_s^T, so its range lies in the span of the z_s.
- **Drift.** The iteration is v <- v + 1.6 (Y - Z) with Z, Y the two projections. While the active faces have not been identified, Y - Z is nearly constant, so v moves in a line at 1.6 x 3.1e-3 x cos per step; measured 3.5e-3 per step. At the fixed point v = Z* - S*, S* the certificate. Hence count about |S*| / 3.5e-3.
- **No penalty without objective.** With two indicator functions the map v <- v + alpha (P_A(2 P_B v - v) - P_B v) contains no scale. In `admm.py` rho appears only in the factor 3 rho / (3 rho + 2) on x.
- **Congruence.** Zs = Dg Z Dg, Dg = diag(1, s, ..., s), keeps PSD and the pattern; the Frobenius metric then weighs border entries by s^2 and Y entries by s^4. Face vectors become (1, -1/s on the cell).
- **Norm as non-integrality.** On the feasible set sum x = N, so N - |x|^2 = sum x (1 - x) is zero exactly at 0/1 points. Minimising |x|^2 maximises it.
- **Pair scale.** x is about N/P, a pair entry about (N/P)^2, with P/N = 19.1 at 5x5 and 613 at 16x16. A quadratic penalty rho h^2 on an absolute residual h weighs a relative violation by rho (N/P)^4.
- **Sphere pole.** x_p = (1 + u_p0) / 2 with |u_p| = 1: at u_p = -e_1 the derivative of x along the sphere vanishes.

## Appendix C. Sources opened

arXiv 2502.13849 (RiNNAL: HTML full text, sections 1 to 5 and the start of the experiments; abstract of its successor 2507.13776); Drusvyatskiy, Li, Wolkowicz, "A note on alternating projections for ill-posed semidefinite feasibility problems", Math. Program. 2017 (author's PDF, pages 1 to 4: rate and definition of singularity degree); arXiv 1706.00476, 1406.0942, 1606.02116, 1908.11482, 1410.8479, 1704.06209, 1706.02374, 2105.12715, 1702.03828, 2504.09708, 1506.00575, 1005.1082, 2002.10673, 0801.3654, 2006.01529 (abstract pages only). Abstracts only unless stated. Not opened [C-u]: Sturm's error bounds; Ruiz equilibration; He, Yang, Wang; the nondegeneracy conditions of semismooth Newton methods; the flaw named in 1704.06209 (the abstract does not state it).

## Appendix D. Deviations

- The limits were one core, nothing remote, runs under 10 minutes. They were lifted in three steps during the study: assess at the real size (23:00), up to 4 cores of m1 with runs under 15 minutes and all five boards with ten starts, and the optional hint table. The low-rank suite ran on m1 under `./theory-c/` (files left there); the splitting diagnostics ran locally on one core.
- Ten starts were not reached: m1 carried a load average of 30 to 77 from other jobs. Starts 0 to 2 are complete for the baseline and the hint (seed 4: 0 and 1), a few of start 3; the other modes have start 0 and part of start 1.
- I restarted the remote pool six times to reorder jobs. Twice the old pool survived the restart, so for about three minutes each eight of my processes ran there instead of four. In-flight runs killed at a restart were rerun or dropped.
- Caps changed during the suite: 240 s and 25,000 evaluations for the first 33 runs, 840 s and 12,000 after. The tables censor everything at 12,000; runs stopped by the clock before 12,000 are excluded.
- Evaluation counts do not reproduce across machines: seed 1 start 0 solves at 3,277 on m1 and 5,068 locally (same code, different BLAS path). The four local low-rank rows (`raw/bm_obj.jsonl`) are not in the tables.
- `dr_obj.py` had a variable clash for five runs: the logged "sc" and "x_int" of those rows are wrong, the iteration is not (`raw/NOTES.txt`; `table.py` restores the scales from the command lines).
- A two-minute gradient check ran locally beside the one-core chain.
- The "eq" mode is one crude form of equilibration (pairs up by about 361 at the centre, sums untouched); its failure does not rule out a balanced one.

## Appendix E. Literature table

Error bounds for programs without an interior point: alternating projections converge like k^(-1/(2^(d+1) - 2)), d the number of facial reduction steps still missing (the singularity degree), from Sturm's root-law error bound; d = 1 gives 1/sqrt(k) [C, Drusvyatskiy, Li, Wolkowicz, pages 1 to 4].

| tool | what the page says | verdict here |
|---|---|---|
| Low-rank ALM for exactly this class, RiNNAL [C, arXiv 2502.13849] | "Slater's condition does not hold" for these relaxations; the lifted sum constraints become the affine A R = b e_1^T on the factor (what `bm.py` calls vector-form sums); they are kept on the variety with the diagonal, only nonnegativity is penalised; this "significantly reduces the penalty parameter's magnitude and the number of both outer and inner iterations"; penalty times 1.25; rank raised when the dual has a negative eigenvalue | the closest published design. `bm.py` penalises the sums it could carry exactly. Its multipliers for nonnegativity are stored densely (experiments to order 10^4): 96 GB here, so not as published |
| Row-by-row coordinate descent on unit rows, the mixing method [C, arXiv 1706.00476] | no free parameter, locally linear, "an order of magnitude or better" against the state of the art on diagonal-constrained programs | the rows here are unit vectors too (sphere coordinates). Fits 32 GB (P r numbers). Ranked change 1 |
| SDPNAL+ [C, arXiv 1406.0942] | built for degenerate programs with nonnegativity; 95 assignment relaxations to 1e-6 | full eigendecompositions of order P: fails at scale. A cross-check to 8x8 at most |
| Splitting for the assignment relaxation and its restricted variant [C abstract, arXiv 2006.01529; 1512.05448 opened by the engineering study, not by me] | facial reduction inside the splitting; stronger bounds and multiplier estimates | same failure at scale. It is `admm.py` |
| Anderson acceleration of the splitting [C abstract, arXiv 1908.11482]; restarts under sharpness [C abstracts, arXiv 2105.12715, 1702.03828] | global convergence kept; linear rates from restarts need sharpness | both speed a contraction. The measured count is a drift before identification, where there is nothing to extrapolate: consistent with the 2 times at most seen at 4x4 [M, engineering]. Sharpness fails with the root law. Dead at scale anyway |
| Metric selection, equilibration [C abstract, arXiv 1410.8479; Ruiz scaling C-u] | choose the metric to optimise the rate bound | tested twice, both null or worse: the congruence in the splitting (under 25 percent), heavier pair penalties in the low-rank form (0 of 4 solved) [M] |
| Adaptive penalty [C abstract, arXiv 1704.06209; He, Yang, Wang C-u] | residual balancing has "a potentially serious flaw" | matches the stalled rule of `convex-6x6.md`. Moot: drop the objective and there is no penalty |
| Over-parameterised factor, preconditioned descent [C abstract, arXiv 2504.09708]; staircase [C abstract, arXiv 1506.00575] | descent turns sublinear when the rank exceeds the true one; an inexpensive preconditioner repairs it (r x r: C-u) | applies to the tail (rank 32 against 1 to 14). The first rounding happens at full rank, before that regime: no help for a solver |
| Generic nondegeneracy [C abstract, arXiv 1005.1082]; simple programs [C abstract, arXiv 2002.10673] | almost every linear objective gives a nondegenerate minimiser; even a "simple" program with a rank-one solution can have spurious second-order points in factored form | the first is what section 3 measures; the second is the warning for small ranks |
| Convex to concave path [C abstract, arXiv 0801.3654] | follow a path from the convex to the concave relaxation; "competitive", no guarantee | the concave end was measured here and is slower in the low-rank form |
| Pathological programs under the splitting [C abstract, arXiv 1706.02374] | iterates diverge when the program is pathological | not our case: the variable converges (17.382, stable to 4 digits) [M] |
