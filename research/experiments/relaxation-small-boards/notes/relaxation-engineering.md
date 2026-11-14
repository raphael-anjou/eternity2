# Relaxation engineering: which solver, where the time goes, what Rust buys, and when to build

Desk and bench study, 2026-11-13 late evening. Draft, not independently checked. All boards are generated TOYS: nothing here is Eternity II progress. Marks: [M] measured in this study by a script in `scripts/relaxation-engineering/` (or by the named study), [D] derived, [C] cited from a page I opened, [C-u] cited but not opened, [G] guessed. The machine carried a load average of 20 to 30 on 10 cores throughout: wall times are best-of-several and still noisy; trust ratios over absolute times.

## 0. Verdict

- **Do not build a production solver now.** The method to build first is not the low-rank one. It is the convex splitting the lift study used, written matrix-free with facial reduction: 150 lines of NumPy, one dense eigendecomposition per iteration. It keeps the single optimum and gets entrywise nonnegativity for free. On all three 5x5 boards it reaches the exact reading (mass off solutions below 1e-5 per cell, ending below 1e-10) in 3,500 to 6,000 iterations, 110 to 190 s on one loaded core. SCS took 398 s, 702 s and did not converge in 1,100 s on the same boards; the low-rank code did not converge in 480 s [M].
- **Rust buys 7 times on the low-rank kernel as coded, about 2 times against NumPy written properly, and nothing on the splitting**, whose time is 93 percent inside LAPACK [M].
- **The number of iterations is the lever, not the flops.** One hand-set penalty changed the iteration count by more than 30 times at 4x4 [M]. Between 4x4 and 5x5 the iteration count grew like P to the power 1.6 to 3.1, depending on the solver [M, one size step].
- **Crossover against row scan: between 8x8 and 10x10 with optimistic iteration growth, between 10x10 and 12x12 with pessimistic growth** [D]. Up to 7x7, where exactness is cheap to measure, the lift never beats search. All of it is conditional on the lift staying exact, which the theory memo puts at 60 percent for 6x6, 20 percent for 8x8 and 2 percent for 16x16.
- **First step:** the existing prototype on the three 6x6 boards, 1.2 to 9 core-hours per solve, with the 10-minute cap lifted and 20 new lines.

## 1. Algorithm choice

Notation: P placements, Z zero pairs, F free pairs (Z + F = P(P-1)/2), m = 2n^2 sum equalities, r the factor rank, D = P + 1 - (2n^2 - 1) the order after facial reduction. The study plan's "0.12 P^2 zeros, 0.38 P^2 nonnegativity" is the 5x5 value; the shares drift to 0.014 P^2 and 0.487 P^2 on the 16x16 [D, section 4].

| candidate | flops per iteration | memory | iterations against accuracy | single optimum | Y >= 0 |
|---|---|---|---|---|---|
| A. conic splitting, general solver (SCS, as used) | one eigendecomposition, about 9 P^3, plus a sparse triangular solve | the solver's factor: 0.6 GB at 5x5, 6.5 to 8 GB at 6x6 [M, lift study] | 175 to 475 at 4x4; 3,025 to over 8,200 at 5x5, to 1e-5 [M, lift and theory studies]. Slower than interior point to high accuracy, no rate stated [C] | yes | yes, as cone rows |
| B. the same splitting, matrix-free, facially reduced (`admm.py`) | 9 D^3 plus O(P^2 D) | 5 dense matrices: 40 P^2 bytes [D] | 500 at 4x4 and 3,500 to 6,000 at 5x5 to the exact reading; 1,400 and 18,000 to violation 1e-7 and 2e-10 [M]. Linear rate, slope set by the penalty [M] | yes | free: a clamp |
| C. low-rank augmented Lagrangian (Burer-Monteiro, SDPLR, staircase) | 4 P^2 r + 3 P^2 + 4 Z r per evaluation [D, `bm.py`] | 8 (2 x 10 + 4) P r for the factor and L-BFGS, plus 16 bytes per active multiplier, up to 16 F = 6 to 8 P^2 bytes [D] | order eps^-4 gradient calls to a stationary point [C]. Converges at 4x4 in 25 s [M, theory]; at 5x5 14,119 to 28,500 evaluations without converging [M] | no. The guarantee needs "few equality constraints" [C]; with order P^2 active inequalities the safe rank is order P [D] | fits, but it is the whole P^2 r cost and a P^2 multiplier store |
| D. GPU low-rank (cuHALLaR, LoRADS) | as C, on GPU | as C | 1e-5 relative on stable-set programs with 10^6 vertices and 2 x 10^6 edges in 120 s, 8.4 x 10^6 vertices in 84 s, one H200 [C] | no | **no**: equalities only, inequalities are future work [C]. That is the theta form, inexact or 300 times slower on one 4x4 board [M, theory] |
| E. spectral bundle | a partial eigendecomposition of the dual slack: tens to hundreds of P^2 products, plus a small QP | 8 P^2 for the slack, 8 (Z + F) for the duals | eps^-3 in general, eps^-1 under strict complementarity, linear if in addition the bundle holds the optimal rank [C] | yes | yes, a sign on the duals |
| F. conditional gradient with sketching (SketchyCGAL) | one approximate top eigenvector | constraints + R P numbers; the constraint count is Z + F, so 4 P^2 bytes at least [D] | eps^-2 in theory, eps^-1 seen; experiments stop at relative 1e-1; "falters for SDPs with a large number of constraints" [C] | yes | yes |
| G. interior point, zeros eliminated | Newton system of order F: F^3 / 3 = 2 x 10^14 at 5x5 [D] | 8 F^2: 60 GB at 5x5, 3.7 TB at 6x6 [D] | 20 to 50 [G] | yes | yes |
| H. SDPNAL+ (semismooth Newton on the augmented Lagrangian) | several eigendecompositions and a conjugate-gradient solve per step | order 10 dense matrices [G] | solved 95 assignment-type relaxations with nonnegativity to 1e-6 [C] | yes | **built for it** |

- G is dead past 5x5. The zero pattern does not rescue it: the free pattern is 76 to 97 percent dense [D].
- F is dead for this use: the reading needs residuals near 1e-7 (section 2), and the storage advantage vanishes with order P^2 constraints [C, D].
- D does not apply as published. Adding F slack variables turns it into C.
- E is sound and fits, but its good rates need strict complementarity, unknown here [G: unlikely].
- C alone costs P^2 r per iteration instead of P^3. Its price is the guarantee. That price is zero for a solver (a support is checked by a trivial search) and total for a measurement of exactness (a stalled nonconvex run proves nothing).
- B and H keep the guarantee and handle Y >= 0 natively. H is the reference software for this class, in MATLAB [C-u]; with a licence it is a cross-check on 6x6, not something to port.

**Ranking for this program: B, then H as an existing cross-check, then C, then E; D, F and G are out.** I would build B now because the open question is exactness, and B answers it with a convex optimum. It needs facial reduction to converge at all (Appendix B). C becomes the production candidate only after B has shown exactness at a size where D^3 hurts.

## 2. Where the time goes today

`bm.py` (unmodified copy, timers around each group of statements), 5x5 seed 0, one BLAS thread [M, `profile_bm.py`]:

| bucket | r = 64 (245 s, 14,119 evaluations, 13.2 ms each) | r = 16 (121 s, 16,636 evaluations, 6.1 ms each) |
|---|---|---|
| gathers `V[ci] * V[cj]` over 27,203 pairs | 43.6% | 35.9% |
| scatter-add (rebuild a sparse matrix, multiply) | 11.2% | 12.9% |
| dense products (the only P^2 r work) | 7.3% | 8.4% |
| elementwise passes over the Gram matrix | 7.7% | 18.5% |
| SciPy L-BFGS-B outside the function | 23.4% | 15.1% |
| sums, sphere map, the rest | 6.5% | 9.1% |

**Implementation.** Half the time goes to gathering 2 Z r numbers for dot products that the Gram matrix, computed a few lines later, already contains. Reading Z scalars out of it and writing the pair weights back before the one product removes gather and scatter. In NumPy alone that is 3.0 times faster per evaluation at r = 64 [M, `kernel_numpy.py`]. A fifth is the bound-constrained driver, used here without bounds.

**Algorithm.** Three things, in order of weight.

1. Every outer round stopped on its 300-iteration cap [M]: no subproblem was ever solved, so each multiplier update starts from an unconverged point.
2. The penalty reaches its cap of 1e5 by round 40 and stays there [M]. With entries of Y near 2.5e-3 this is a badly conditioned problem for a 10-pair quasi-Newton step. L-BFGS is standard for this family [C-u, SDPLR]; the published GPU method adds Frank-Wolfe steps to leave spurious stationary points [C]; the staircase uses second-order steps and raises the rank [C-u]. First order is not the anomaly; unsolved subproblems at a fixed rank are.
3. The program is ill conditioned for any method. On the convex splitting (seed 1) at iteration 3,000 the largest violation is 2.7e-5 while 0.14 of the mass per cell is still off the solutions; 1,500 iterations later the violation is 8e-6 and the mass 5e-6 [M]. A tolerance of 1e-5 cannot tell an exact lift from a leaky one. Residuals near 1e-7 are needed, so single precision is out [G for the exact digit], and with it the Apple GPU route and consumer NVIDIA cards (1.25 TFLOPS in double on an RTX 4090 [C]).

## 3. What a compiled implementation buys

Crate `kernel-bench` (Rust, rayon, Accelerate through `cblas`) against `kernel_numpy.py` on identical inputs; checksums agree to 12 digits [M]. One evaluation is both kernels: the Z pair terms and the all-pairs nonnegativity term. Best time, one thread, double, milliseconds:

| size | NumPy as `bm.py` does it | NumPy, one pass | Rust, one pass | Rust single precision | Rust, 2 threads |
|---|---|---|---|---|---|
| 5x5, P 478, r 16 | 3.05 | 2.14 | 0.80 | 0.83 | 0.47 |
| 5x5, P 478, r 64 | 7.10 | 2.36 | 1.02 | 0.76 | 0.69 |
| 6x6, P 1,290, r 16 | 22.5 | 16.2 | 5.74 | 4.98 | 8.06 |
| 6x6, P 1,290, r 64 | 57.4 | 17.4 | 7.60 | 5.11 | 6.41 |
| synthetic, P 20,000, r 16 | 6,070 | 4,210 | 2,140 | 2,000 | 1,560 |
| synthetic, P 20,000, r 64 | 20,200 | 4,330 | 2,820 | 2,140 | 1,870 |

- **Rust against the code as written: 2.8 to 7.5 times (7.0 to 7.5 at r = 64).** Against NumPy restructured the same way: 1.5 to 2.8 times. Single precision adds between nothing and 1.5. Two threads add 0.7 to 1.7 on a loaded machine; I could not test 14 (limit of two cores).
- **Rust loops without a BLAS are 3 to 14 times slower than NumPy** on the all-pairs term (11.5 ms against 0.95 ms at 5x5, r = 64) [M]. NumPy's product is Accelerate, at about 118 GFLOPS on one thread here [M, `eig_time.py`]. A Rust solver must call the same library.
- Rust wins on the indexed kernel alone: 1.7 to 4.6 times in double [M, Appendix C]. That kernel only matters when a P x P block no longer fits: the 16x16.
- Whole low-rank loop at 5x5, r = 64: 13.2 ms per evaluation today; kernels in Rust 1.0 ms, plus sums and a plain L-BFGS: about 1.3 ms, **10 times** [D from the measured parts].
- The splitting spends 92.7 percent of its time in one LAPACK call [M]: compiled code buys under 8 percent. A second BLAS thread changed that call by about 10 percent at order 1,291, inside the noise [M].

## 4. Scaling table

Method priced: B, in double precision; every input is listed with its mark in Appendix F. Per-iteration times are measured up to order 2,917 and scaled as D^3 beyond. Iterations are 4,500 at 5x5 (measured: 3,500 to 6,000 on three boards) times (P / 478)^a, with a = 1 (optimistic, better than anything measured) or a = 3 (pessimistic, the top of the measured range). The card is one NVIDIA H100. Row scan runs at 1.6e7 nodes per second per core.

| board (frame, inner colours) | P | zeros | free pairs | equalities | one core, opt / pess | 14 cores, opt / pess | H100, opt / pess | row-scan tree | row scan, 1 / 14 cores |
|---|---|---|---|---|---|---|---|---|---|
| 5x5 (3,4) | 484 | 2.8e4 | 8.9e4 | 50 | 91 s / 93 s | 15 s / 16 s | 91 s / 93 s | 2^15.0 to 2^15.8 [M] | 4 ms / 0.3 ms |
| 6x6 (4,5) | 1,296 | 1.6e5 | 6.8e5 | 72 | 71 min / 8.6 h | 12 min / 86 min | 4 min / 30 min | 2^20.9 to 2^21.8 [M] | 0.2 s / 15 ms |
| 7x7 (4,7) | 2,916 | 6.2e5 | 3.6e6 | 98 | 33 h / 50 d | 5.4 h / 8.4 d | 10 min / 6 h | 2^25.9 [D] | 4 s / 0.3 s |
| 8x8 (5,8) | 5,776 | 1.9e6 | 1.5e7 | 128 | 22 d / 8.6 y | 3.6 d / 1.4 y | 2.6 h / 16 d | 2^32.8 [D] | 8 min / 33 s |
| 10x10 (5,10) | 17,424 | 1.1e7 | 1.4e8 | 200 | 5 y / 7,000 y | 0.8 y / 1,000 y | 9 d / 34 y | 2^60.4 [D] | 3,000 y / 220 y |
| 12x12 (5,12) | 41,616 | 4.3e7 | 8.2e8 | 288 | 170 y / 1e6 y | 28 y / 2e5 y | 0.85 y / 6,000 y | 2^96.5 [D] | 2e14 y / 2e13 y |
| 16x16 (5,17) | 156,816 | 3.3e8 | 1.2e10 | 512 | 3e4 y / 4e9 y | 6e3 y / 6e8 y | does not fit (916 GB) | 2^134.4, five clues [M] | 6e25 y / 4e24 y |

A reading needs two solves (the point, then the largest leak): double the lift columns.

**Crossover.** With optimistic growth the lift overtakes row scan between 8x8 and 10x10 (at 10x10: 9 days on one card against 220 years on 14 cores). With pessimistic growth it does so between 10x10 and 12x12 on CPUs, and already at 10x10 on the card (34 years against 220) [D]. At 8x8 (5,8) the best case is 2.6 hours against 33 seconds. With one colour fewer, (5,7), the tree is 2^41.6 and row scan takes 4 hours on 14 cores: a tie with the card's best case [D].

**All of this assumes the lift is still exact at the size in question.** The theory memo's odds are 60 percent at 6x6, 20 percent at 8x8 and 2 percent at 16x16; it gives none for 10x10, the first size where the table favours the lift. An inexact lift is not a slower solver, it is not a solver. Exactness is cheap to measure only at sizes where the lift has no practical value.

The low-rank form changes the 10x10 row, not the conclusion: 2 s per evaluation at r = 64 [M, scaled from P = 20,000] against 970 s per dense projection, with an iteration count that is unknown and was at least 6 times worse at 5x5 [M].

## 5. The plan

| step | what | cost | kill line |
|---|---|---|---|
| 0 | Read the lab machine's three 6x6 SCS solves when they end | free | none: a converged point there replaces step 1 for that board |
| 1 | `admm.py` on 6x6 (4,5) seeds 0 to 2, after adding the linear leak objective, a residual-balanced penalty and a stop at residual 1e-7 (20 lines). The minimum-norm point, then the largest leak | 1.2 to 9 core-hours per board and objective [D], 60 MB; needs the 10-minute cap lifted | mass off solutions at or above 0.05 per cell on 2 of 3 boards at residual 1e-7: not exact at 6x6, **stop all engineering** (the theory memo's line) |
| 1b | Same code on a 5x5 board with no solution (the low-rank study's "broken" board) | 10 core-minutes | if the run reports a feasible point, the reading rule is wrong; fix before step 1 counts |
| 2 | Only if step 1 passes: refit the iteration exponent on the 6x6 logs, then 7x7 (4,7), three boards | 33 hours to 50 days per board on one core [D]; if the refit says over a week, one rented H100 day and a two-line change of the eigendecomposition call | same line |
| 3 | Only if step 2 passes: 8x8 (5,8) on the card | 2.6 hours to 16 days per board [D, G] | same line |
| 4 | Only if step 3 passes (theory memo: 20 percent): build the low-rank solver in Rust: double precision, a BLAS underneath, one-pass kernel, own L-BFGS with solved subproblems and rank increase, the splitting kept as certifier. Target 10x10 (5,10) | weeks | on 8x8 boards the splitting has certified, the low-rank point's support misses every solution |

Cost to the first decision: one hour of code and 7 to 54 core-hours.

Not yet: any Rust solver; any GPU port; Apple's GPU stack (no double precision); tuning the low-rank schedule; sketching, interior point or chordal splitting; anything at 16x16; single precision in a run whose result is a reading.

## Appendix A. Runs (all in `scripts/relaxation-engineering/`; at most 2 cores, nice 10, each under 10 minutes; about 90 core-minutes in total)

| script | what | output |
|---|---|---|
| `bmcopy/` | unmodified copies of `bm.py`, `board.py`, `common.py` with their SHA-256 | `COPIED_SHA256.txt` |
| `profile_bm.py` | section 2 | `raw/profile_5_3_4_0_r64.json`, `raw/profile_5_3_4_0_r16.json` |
| `kernel-bench/` (Rust), `kernel_numpy.py`, `run_bench.sh`, `bench_table.py` | section 3 | `raw/kernel_bench.jsonl`, `raw/kernel_bench_onepass.jsonl` |
| `admm.py`, `summarise_admm.py` | method B | `raw/admm_*.jsonl`, `raw/admm_summary.txt` |
| `eig_time.py` | eigendecomposition time at orders 479, 1,291, 2,917; two threads at 1,291 | `raw/eig_time_*.json`, `raw/eig_2threads_1290.json` |
| `counts_and_tree.py`, `scaling.py` | section 4 | `raw/counts_and_tree.json`, `raw/scaling.json` |

Deviations. The first Rust build used `-j 1` (a profile run held the other core), later builds `-j 2`. The first full benchmark used Gram blocks of 1 MB, which leaves 6-row blocks at P = 20,000 and made the Rust all-pairs kernel slower than NumPy there (4.39 s against 2.08 s); the one-pass runs use at least 64 rows and are the ones tabulated. The synthetic P = 20,000 uses 2e7 random pairs (0.05 P^2), not 0.12 P^2, to stay under 4 GB in NumPy. Synthetic pair lists contain repeats, which the NumPy one-pass version overwrites instead of summing, so only its time is comparable, not its checksum. The NumPy P = 20,000, r = 16 run peaked at 3.9 GB.

## Appendix B. Method B on toys [M]

Fixed-point map on one symmetric matrix v: Z = projection of v on the face (W R W^T, R PSD, W an orthonormal basis of the complement of the cell and tile vectors); Y = clamp and zero the off-diagonal entries of 2Z - v, set x from the three copies of each x_p with the sum equalities enforced by one small pseudo-inverse and the objective ||x||^2 / rho folded in; v = v + 1.6 (Y - Z). Start: the Sinkhorn centre.

| board | rho | facial reduction | iterations run | first logged iteration with mass off solutions under 1e-2 / 1e-3 / 1e-5 per cell | end state |
|---|---|---|---|---|---|
| 4x4 (2,4) seed 3, 1 solution | 30 | yes | 1,387 (2.9 s) | 500 / 500 / 500 | residual 1e-7, rank 1, the solution |
| same | 1,000 | yes | 1,631 | 500 / 500 / 1,000 | same |
| same | 1 | yes | 18,394 (40 s) | 16,000 / none / none | violation 2.8e-5, mass 5e-3 |
| 4x4 (2,4) seed 0, 8 solutions | 1 | yes | 18,500 | 4,500 / 7,500 / 14,500 | violation 5e-9, rank 7 |
| same, Anderson memory 10 | 1 | yes | 8,054 | 2,250 / 4,500 / none | mass 2e-5 |
| 5x5 (3,4) seed 1, 6 solutions | 300 | yes | 18,236 (570 s) | 4,500 / 4,500 / 4,500 | violation 2e-10, mass 8e-12, rank 6, objective 8.79068 against SCS 8.79073, largest difference to the SCS point 1.1e-3 |
| same | 3,000 | yes | 17,092 (570 s) | 3,000 / 3,500 / 3,500 | violation 1e-10, mass 1e-12, rank 6; objective 8.8207 (not yet the minimum-norm point: at this penalty the objective barely acts) |
| same | 1, 10, 30 | yes | 1,600 to 2,500 | none | mass 0.42 to 0.52 at violation 1e-4 |
| same | 0.1, 1, 10 | no | 1,100 to 1,400 | none | violation 1e-2 to 9e-2 |
| 5x5 (3,4) seed 0, 36 solutions | 3,000 | yes | 18,813 (570 s) | 3,500 / 3,500 / 4,000 | violation 1.7e-10, mass 3e-11, rank 30 (solution span 32); SCS stopped unconverged at 8,200 iterations with mass 9e-4 |
| 5x5 (3,4) seed 2, 8 solutions | 3,000 | yes | 18,175 (570 s) | 6,000 / 6,000 / 6,000 | violation 1.9e-10, mass 1e-12, rank 8 (solution span 8) |

Why B converges where a naive splitting does not [M]. Without facial reduction the same code sits at violation 1e-2 after 1,400 iterations. The reduction (every feasible matrix annihilates the vectors e_0 minus the indicator of a cell or of a tile, so the matrix lives on a face of order D) is the splitting published for the quadratic assignment relaxation [C, abstract only; the facial reduction detail is from memory of the paper]. It enforces every cell sum, tile sum and row sum at each iterate. Anderson acceleration with memory 10 gave at most 2 times fewer iterations at 4x4 and cost more per iteration [M]; I dropped it.

Logs are every 500 iterations (250 in the pilots), so "first logged" overstates by up to that much. 30 to 37 ms per iteration at 5x5, 92.7 percent in the eigendecomposition. SCS on the same boards: 8,200 iterations unconverged in 1,100 s (seed 0), 4,925 in 702 s (seed 1), 3,025 in 398 s (seed 2) [M, lift study].

Iteration growth from 4x4 to 5x5 (P times 3.46): this splitting, at most 500 to 3,500..6,000, exponent 1.6 to 2.0 or more at the best penalty tried at each size (which moved from 30 to 3,000); SCS, 175 to 3,025..8,200, exponent 2.3 to 3.1. One step, one to three boards per size: this is the weakest number in the memo and the one the scaling table leans on most.

These are readings at the point the method lands on with the minimum-norm objective barely acting, not certificates: the largest-leak solve was not run.

## Appendix C. Kernel benchmark, per kernel [M]

Best times in milliseconds, one thread unless stated. "Fused" is plain Rust loops with eight accumulators and no library.

| P, r, precision | pairs: NumPy | pairs: Rust | all pairs: NumPy | all pairs: Rust fused | all pairs: Rust + BLAS (64-row blocks) |
|---|---|---|---|---|---|
| 478, 16, double | 2.37 | 0.51 | 0.68 | 2.38 | 0.37 |
| 478, 64, double | 6.15 | 2.34 | 0.95 | 11.5 | 0.60 |
| 478, 64, single | 4.37 | 1.21 | 0.61 | 9.06 | 0.39 |
| 1,290, 16, double | 16.1 | 4.84 | 6.39 | 20.2 | 3.11 |
| 1,290, 64, double | 47.4 | 15.0 | 9.97 | 82.4 | 4.90 |
| 1,290, 64, single | 28.5 | 7.84 | 4.82 | 60.1 | 2.42 |
| 20,000, 16, double | 4,460 | 1,700 | 1,610 | 5,290 | 826 |
| 20,000, 64, double | 18,100 | 10,600 | 2,080 | 23,300 | 1,580 |
| 20,000, 64, single | 8,030 | 5,230 | 1,680 | 14,800 | 983 |

Two rayon threads against one: pairs 0.8 to 2.1, fused 1.1 to 2.0, BLAS blocks 0.9 to 1.7.

## Appendix D. Sources opened

- cuHALLaR: https://arxiv.org/abs/2505.13719 (abstract and full text: Table 1 hardware, Tables 3 and 5 stable-set times, the future-work paragraph on inequality constraints and double precision).
- HALLaR: https://arxiv.org/abs/2401.12490 (abstract: inner solver, Frank-Wolfe steps, 1e-5 on a stable-set program of 10^6 by 10^7 in under 20 minutes).
- SketchyCGAL: https://arxiv.org/abs/1912.02949 (abstract and full text: storage, eps^-2 and eps^-1, 1e-1 experiments, the limitations paragraph).
- Spectral bundle rates: https://arxiv.org/abs/2008.07067 (abstract and the contributions list).
- Inexact augmented Lagrangian complexity: https://arxiv.org/abs/1906.11357 (abstract).
- Burer-Monteiro landscape: https://arxiv.org/abs/1606.04970 (abstract).
- SDPNAL+: https://arxiv.org/abs/1406.0942 (abstract).
- SCS: https://arxiv.org/abs/1312.3039 (abstract).
- Splitting for the assignment relaxation: https://arxiv.org/abs/1512.05448 (abstract).
- GPU eigendecomposition on the H100: https://arxiv.org/html/2410.02170v1 (peak figures, cuSOLVER timings, RTX 4090 double-precision peak).

Not opened, cited from memory or a search summary [C-u]: Burer and Monteiro 2003 and SDPLR; the Riemannian staircase; LoRADS (arXiv 2403.09133, search summary only); the worst-case rate of two-block splitting; SDPNAL+ being MATLAB.

Two figures disagree across sources and I did not resolve it: the cuHALLaR paper tabulates about 34 TFLOPS peak double precision for the H200; the eigendecomposition paper uses 67 for the H100. The table above uses neither peak, only the measured 80.4 s.

## Appendix E. Not checked

- Any lift at 6x6 or above, by any method. Section 4 past its first row is extrapolation.
- Method B with the linear leak objective (so no certificate of exactness was produced here, only points with no leak), with an automatic penalty, or on a board with no solution.
- More than two threads; any GPU; MKL.
- The low-rank code after the cheap fixes of section 2. Its iteration count could drop a lot; I measured where its time goes, not how fast it could become.
- Whether strict complementarity holds for this program.
- The first-moment tree sizes at 7x7 (4,7), 8x8 (5,8), 10x10 and 12x12 against a real enumeration.

## Appendix F. Inputs of the scaling table

Method priced: B. Sizes from the theory memo's formulas (checked: formula 484 and 27,702 against measured 478 and 27,203 to 27,438 at 5x5; formula 1,296 and 156,243 against measured 1,290 and 154,766 at 6x6 [D, M]). Per-iteration time on one core: measured 0.026 s, 0.40 s and 4.7 s at orders 479, 1,291 and 2,917, then D^3 [M, D]. Fourteen cores: one core divided by 6 [G; the two-thread test gave nothing on this Mac, MKL on the lab machine should do better]. GPU: one NVIDIA H100 (80 GB), time 120 s x (D / 50,000)^3 with a floor of 20 ms, from a measured 80.4 s for the eigenvalues of an order-50,000 matrix with cuSOLVER on that card, where its tridiagonal reduction runs at 1.8 TFLOPS against a 67 TFLOPS peak [C], plus half again for the vectors [G]. Iterations: 4,500 at 5x5 (measured: 3,500 to 6,000 on three boards) times (P / 478)^a, with a = 1 (optimistic: better than anything measured) and a = 3 (pessimistic: the top of the measured range 1.6 to 3.1). Row scan: first-moment model plus 1.5 bits, 1.6e7 nodes per second per core [M, run log]. The model is 0.5 to 2.1 bits under the measured trees at 5x5 (3,4), 6x6 (4,5) and 7x7 (3,7), and 1.6 to 6.6 bits under at 8x8 (4,11), where the measured trees span 5 bits between boards; so the row-scan columns may be low by a factor of up to 30, which moves no conclusion; colour counts in the first column, 10x10 and 12x12 chosen as the fewest inner colours with under one expected extra solution [D]. The theory memo's 7x7 (4,7) and 8x8 (5,8) carry one colour more than that rule gives; at (4,6) and (5,7) the model trees are 2^31.3 and 2^40.1 before the 1.5-bit correction, and the lift's cost is the same [D].
