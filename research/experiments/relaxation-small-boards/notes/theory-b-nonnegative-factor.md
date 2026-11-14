# Theory B: can a nonnegative factor make the entrywise condition free?

Desk and bench study B of four, 2026-11-13. Draft, not independently checked. All boards are generated TOYS (5x5, frame 3, inner 4, seeds 0 to 4, and the low-rank study's broken board): nothing here is Eternity II progress. Marks: [M] measured in this study by a script in `scripts/theory-b/`, [D] derived (Appendix A), [C] cited from a page I opened, [C-u] cited from memory or a search summary, [G] guessed. P placements, N cells, T tiles, K conflict pairs, F free pairs, r columns, C the 0/1 conflict matrix. An "evaluation" or "sweep" is one value-and-gradient call.

## 0. The reading first

**1. The nonnegative factor makes the condition free and the method useless. Do not build it.** A sweep costs order P r (1e8 flops, 60 MB on the 16x16) [D]. It solves 0 of 50 starts at 5x5 in each of four variants, and 0 of 2,000 in the simplest, where annealing solves 26 of 50 and the signed low-rank baseline 50 of 50 [M]. Three derived reasons: its feasible set at rank r is exactly the mixtures of at most r true solutions, at every size (the puzzle, not a relaxation); its columns do not interact (rank r is r unrelated soft-assignment descents); and the same-cell and same-tile zeros put a barrier around every board: exchanging two tiles is a descent direction only if it removes about 4 mismatches at once (formula checked on 400 boards, no failure [M]).

**2. The candidate left open, a signed factor with Y >= 0 only between cells within distance d, rounds at 5x5 as well as the full form, and so does no nonnegativity at all** [M, 20 starts per board, 10 at d = 0, same solver and starts]:

| nonnegativity kept on | solved | median evaluations | 90th percentile | solved within 15,000 | broken control |
|---|---|---|---|---|---|
| all free pairs (the full form) | 100/100 | 3,038 | 8,569 | 99% | 0/10 |
| cells within distance 2 (41% of free pairs at 5x5) | 100/100 | 3,052 | 9,039 | 99% | 0/10 |
| cells within distance 1 (6%) | 98/100 | 3,708 | 12,779 | 90% | 0/10 |
| none | 46/50 | 2,923 | 9,882 | 90% | 0/10 |

The full row reproduces the Rust study's 50 of 50 at 3,017. Dropping the far pairs costs nothing measurable at d = 2 and about nine points of the tail at d <= 1.

**3. But 5x5 cannot carry that to the 16x16.** In every row the run stops with 45 percent of the free pairs negative, violation near 0.05 and 0.2 of the mass per cell off the solutions: the rounding fires long before any relaxation holds (the Rust memo's reading). The convex lift with d <= 1 is not exact: its minimum-norm point keeps 0.44 to 0.52 of the mass per cell off the solutions on two boards (d = 2 did not converge in 600 s) [M]. And annealing solves half the 5x5 starts. What decides is 6x6, where annealing solves 0 of 1,000 and no low-rank run has converged.

**Ranking by cost on the 16x16** (P = 156,816, K = 3.3e8, F = 1.2e10; 32 GB, 10 cores), r = 16 [D]:

| form | flops per evaluation | memory | 5x5 result [M] | what breaks first as P grows |
|---|---|---|---|---|
| nonnegative columns (section 1) | 1e8 | 60 MB | 0/50 | nothing in cost; the solve rate is 0 at P = 478 |
| signed, no Y >= 0 | 2e10 (zeros, pairs stored) | 5 GB | 46/50 | evaluations to a solution (unknown past 5x5); not a relaxation |
| signed, Y >= 0 at d = 1 | 2e10 + 1e9 | 5.3 GB | 98/100 | the same |
| signed, Y >= 0 at d = 2 | 6e10 | 15 GB | 100/100 | pairs grow as P^2 n_d / N; the radius needed is unknown (d = 2 is 41% of a 5x5 and 5% of the 16x16) |
| masked nonnegative Gram, eigenvector cuts (I2) | 1e10 per cut, about P/3 cuts | 66 GB | not run | fails: negative eigenvalues proportional to P |
| signed, working set of inequalities (I1) | active set, 35 to 77% of F | 70 to 150 GB | (baseline) | fails: order P^2 |
| signed, Y >= 0 on all pairs | 1.7e12 | up to 192 GB | 100/100 | fails: P^2 flops; multipliers pass 32 GB near 13x13 |

## 1. The completely positive form, exactly

V >= 0 of shape P x r, Y = V V^T, x = diag(Y). No border is needed (A1).

| constraint of the lift | on the factor | count before | after |
|---|---|---|---|
| Y PSD, Y >= 0 on free pairs | by construction | cone, F | 0 |
| Y_pq = 0 on conflicts | all terms V_pk V_qk are nonnegative, so the K equalities are one scalar, sum_k v_k^T C v_k = 0: each column sits on a conflict-free partial board | K | 1, or 0 as the objective |
| row sums over cells and tiles | V^T (e_c - e_c') = 0: a column carries the same mass a_k in every cell and on every tile; linear | P (N + T) | (N + T) r |
| sums of x | sum_k a_k^2 = 1 | N + T | 1 |

**Proposition [D, A2].** V is feasible exactly when each nonzero column is a_k times the indicator of one complete solution. At r = 1 the feasible points are the solutions. No exactness question remains, at any size.

Known: the quadratic assignment problem "can equivalently be formulated as a linear program over the cone of completely positive matrices", and "it is hard to optimize over this cone" [C, Povh and Rendl]; strong membership in that cone is NP-hard [C, Dickinson and Gijben].

**Separation [D].** With the sums in linear form the problem is r independent copies of: minimise s^T C s over B = {s >= 0, cell sums 1, tile sums 1}, the polytope whose centre is the Sinkhorn centre X0.

**Cost [D].** Gradient 2 C s, and C is never formed: (C s)_p = (mass of p's cell minus s_p) + (mass of p's tile elsewhere) + over four sides (mass of the neighbour cell minus its mass showing p's colour there). One histogram per (inner edge, colour): about 40 P r flops with the Sinkhorn rounds [G for the constant], 24 P r bytes, no multiplier. Study A needs the same histograms (overlap).

## 2. What is lost

- **The convexity argument [D].** Feasibility is now the puzzle; a descent reaches local minima only.
- **The rank guarantee [D].** For a signed factor spurious minima disappear generically above rank sqrt(2m). Here the local minima at rank r are products of rank-1 local minima: columns add no escape direction. In the coupled variant (row sums penalised, V = W * W) a median of 1 column of 16 survives [M].
- **The barrier [D, A3; M].** Along the exchange of the tiles of cells x and y the slope at an integral board is 2 (4 - 2c + D): D the change in mismatches towards other cells, c = 1 if x and y are adjacent and mismatched. The 4 is the midpoint's cost: two half-tiles in each cell, each tile half in two cells. Among boards already improved by rotations (median 22 mismatches), 28 and 33 of 100 have no descending rotation or exchange edge, while a discrete exchange or rotation improves all 100 [M].
- **Against annealing.** A rank-1 nonnegative V is a deterministic soft-assignment descent with that barrier: the annealer's traps plus others. Rank r > 1 adds restarts. It ends at a median of 7 to 8 mismatches where the annealer ends at 0 to 2 [M].
- **Why the signed factor works where this fails [G].** With signs, Y_pq = 0 is orthogonality and needs no disjoint supports. By reading 2, the signs, not the entrywise condition, make the 5x5 rounding work.

## 3. Algorithms and relatives

- **Mirror descent with Sinkhorn** (s <- s * exp(-eta 2 C s), rescale) is the multiplicative update; with a rising inverse temperature it is softassign, which "incorporat[es] a method discovered by Sinkhorn" inside graduated nonconvexity [C, Gold and Rangarajan, indexed excerpt]. Both run [M].
- **V = W * W**: second-order points correspond, under conditions, to those of the constrained problem [C, Ding and Wright; C, Levin, Kileel, Boumal]. It removes the bound and no trap. Run in the coupled variant [M]. **Softmax rows per cell** free the cell sums only: same landscape [D].
- **Symmetric nonnegative factorisation** [C-u, Kuang, Ding, Park] and **completely positive factorisation** [C, Lai and Yoshise: stationary points only] fit or factor a given matrix. Here there is none.
- **Nonnegative low-rank SDP** works for clustering, "as simple and scalable as state-of-the-art NMF algorithms" [C, Zhuang, Chen, Yang, Zhang]: a planted problem with signal above noise, unlike this one (theory memo, item 5).
- **Motzkin and Straus** [C-u]: section 1 is their program with one simplex per cell and tile. The update's fixed point is naive mean field [G].

## 4. Intermediate options

- **I1. Working set of inequalities under a signed factor.** 35 to 77 percent of the free pairs carry a multiplier [M, low-rank study]: order P^2.
- **I2. Masked nonnegative Gram.** Y = N N^T with conflict entries overwritten by 0, N >= 0: zeros and signs by construction, PSD by eigenvector cuts ("a few PSD-only corrections"). A product Y u is order P r structured. But at random N around the centre the bordered matrix has 158 to 172 negative eigenvalues of 479, for r from 1 to 256 on three boards, and 0 at a mixture of solutions [M]: about P/3 corrections, not a few.
- **I3. Signed factor, nonnegativity only between nearby cells.** Reading 2 and 3. Pairs on the 16x16: 1.7e7 at d = 1 (16 of 17 pairs of adjacent cells are conflicts already), 6e8 at d = 2 (A4, A5). The zeros then dominate: 4 K r flops with pairs stored, or order P r^2 with second moments per (edge, colour) as a pure penalty [D; study A]. On 10 cores that is seconds per evaluation [G]; at the 5x5 median of 3,000 evaluations, hours per start, with the count's growth unknown. Study D holds the convex version.

## 5. Smallest experiments that confirm or kill

| claim | experiment | kill line | cost |
|---|---|---|---|
| a nonnegative factor does not solve 5x5 | done. To overturn: any update on V >= 0, any r | 5 of 10 starts on 3 of 5 boards within 1e5 sweeps | seconds |
| dropping far pairs keeps the solver | `bm_local.py` at 6x6 (4, 5), d = 0, 1, 2 and full, three boards, 10 starts | full solves and d <= 2 does not: the far pairs matter; nothing solves: the family stops at 6x6 | needs the project's 6x6 decision; minutes to an hour per start in the Rust solver [G] |
| I2 needs order P cuts | negative eigenvalues along a descent with cuts | under 10 at every iterate | minutes |
| convex lift at d = 2 | `local_nonneg.py` to residual 1e-7 | mass off solutions at or above 0.05 per cell on 2 boards | 30 minutes per board |

## 6. Not checked

Anything at 6x6 (out of scope). The Rust solver with a distance mask: I used `bm.py` with its settings (r = 16, 20 inner iterations, rounding every outer step); only the full-form control ties the two. Other ranks for the local forms. The barrier for cycles of three or more tiles. A nonnegative factor with noise or with repelling columns. I2 beyond its start point. The convex runs at residual 1e-7. Flop constants of the structured kernels. Full texts of the sources.

## Appendix A. Derivations

**A1. No border.** Feasible Y has Y e_c = x for the indicator e_c of a cell (row sums; inside the cell the other entries are zeros and Y_pp = x_p), and e_c^T Y e_c = 1. For PSD Y, Y - (Y e)(Y e)^T / (e^T Y e) is PSD: Y - x x^T.

**A2. Feasible set.** Y (e_c - e_c') = 0 and Y = V V^T give V^T (e_c - e_c') = 0: column k has the same sum a_k over every cell, and over every tile. Y_pq = 0 with nonnegative terms gives V_pk V_qk = 0 on every conflict pair, so column k has one nonzero entry per cell, equal to a_k, pairwise conflict-free, every tile once: a solution. The sum of x over a cell is sum_k a_k^2 = 1. The converse is direct.

**A3. Barrier.** f(s) = s^T C s, s0 integral, d = s' - s0 with the tiles of x and y exchanged (any rotations). Old placements ox, oy, new nx, ny: C(ox, nx) = C(oy, ny) = 1 (same cell), C(ox, ny) = C(oy, nx) = 1 (same tile), C(ox, oy) = c. Every other placement of s0 differs from the four in cell and tile and contributes the change D on its shared edges. Slope 2 s0^T C d = 2 (4 - 2c + D). Curvature d^T C d = 2 (C(nx, ny) + c - 4) < 0, so a tie escapes. Re-rotation: slope 2 (1 + D), curvature -2. A vertex whose every edge has positive slope is a strict local minimum; the edges of B are cyclic exchanges, a k-cycle carrying 2k by the same count [D for k = 2, G beyond].

**A4. Counts on the 16x16.** Interior cells hold about 784 placements. Adjacent cells: 784^2 / 17 free pairs per edge, 480 edges: 1.7e7. Cells at distance 2: about 1,000 cell pairs, nearly all of 784^2 free: 6e8. Per pair 4 r flops and 16 bytes with a multiplier. Zeros: 4 K r flops, 16 K bytes. I2: P/3 vectors of 8 P bytes. Full form: 4 P^2 r flops, 16 F bytes; 16 F = 32 GB at P near 64,000, between 13x13 and 14x14.

**A5. Cost and memory per evaluation at each size, r = 16 [D].**

| board | P | K | nonnegative columns | signed, d = 0 | signed, d = 2 | signed, all pairs |
|---|---|---|---|---|---|---|
| 5x5 | 478 | 2.7e4 | 3e5, 0.2 MB | 2e6, 1 MB | 4e6 | 1.5e7, 1.4 MB multipliers |
| 6x6 | 1,290 | 1.5e5 | 8e5, 0.5 MB | 1e7, 4 MB | 2e7 | 1.1e8, 11 MB |
| 8x8 | 5,776 | 1.9e6 | 4e6, 2 MB | 1.2e8, 30 MB | 3e8 | 2.2e9, 240 MB |
| 16x16 | 156,816 | 3.3e8 | 1e8, 60 MB | 2e10, 5 GB | 6e10, 15 GB | 1.7e12, 192 GB |

## Appendix B. Runs

Scripts in `scripts/theory-b/`, rows in `raw/`. Starts for the signed runs are the low-rank study's (`init_U`, rng 1000 r + start).

**Comparison, 10 starts per board** (`raw/cmp/`, table `raw/cmp_summary.txt`; m1, 4 processes beside study C's 4).

| method | script | solved, five boards | per board (seeds 0 to 4) | sweeps to a verified solution | end state of the others | broken |
|---|---|---|---|---|---|---|
| nonnegative column, mirror descent | `softassign.py` | 0/50 (and 0/2,000 on this Mac, `raw/soft_5x5.jsonl`) | 0 each | none | integral after about 200 sweeps, median 8 mismatches, least 2 | 0/10, 0/200 |
| nonnegative column, annealed (softassign) | `softassign.py ... anneal` | 0/50 | 0 each | none | one end point per board whatever the start: 2, 27, 18, 3, 4 mismatches (seeds 1 and 2 not integral at 3,000 sweeps) | 0/10 |
| coupled, r = 1 | `cp_coupled.py` | 0/50 | 0 each | none | median 10 mismatches, about 300 evaluations | 0/10 |
| coupled, r = 16 | `cp_coupled.py` | 0/50 | 0 each | none | median 7 mismatches, 1 live column of 16, about 3,600 evaluations | 0/10 |
| annealing, 2e6 proposals | `anneal_sa.py` (probe B's annealer) | 26/50 | 7, 2, 4, 9, 4 | median 61,700 sweeps of 25 proposals; set by the linear schedule, not a minimum | 2 mismatches (median) | 0/10 |
| signed baseline, `bm.py` r = 32, 300 inner iterations, rounding every 2 outer steps | `baseline_bm.py` | 16/50 at 180 s, 50/50 after rerunning the 34 others to 480 s | 10 each | median 16,065, range 3,844 to 36,513 | none | 0/10 at 180 s |

**Local nonnegativity under a signed factor** (`bm_local.py`, `raw/loc/`, table `raw/local_summary.txt`; 8 processes; limit 300 s, 150 s at d = 0, 60 s broken). `bm_local.py ... check` at d = 99 returns the value and gradient of `bm.SphereAL` exactly. Per board, solved and median evaluations:

| d | s0 | s1 | s2 | s3 | s4 |
|---|---|---|---|---|---|
| all | 20/20, 2,408 | 20/20, 4,232 | 20/20, 3,495 | 20/20, 1,907 | 20/20, 6,240 |
| 2 | 20/20, 2,322 | 20/20, 5,790 | 20/20, 2,976 | 20/20, 1,875 | 20/20, 6,862 |
| 1 | 20/20, 1,889 | 19/20, 4,447 | 20/20, 5,145 | 19/20, 2,127 | 20/20, 7,097 |
| 0 | 10/10, 2,277 | 10/10, 4,215 | 8/10, 6,855 | 10/10, 2,073 | 8/10, 6,802 |

Solved within 5,000 / 10,000 / 15,000 evaluations: all pairs 71, 94, 99 of 100; d = 2: 68, 92, 99; d = 1: 61, 81, 90; d = 0: 30, 41, 45 of 50. The two unsolved d = 1 runs stopped at the 300 s limit (about 32,000 evaluations), the four d = 0 runs at 150 s (about 15,700). At the stop, median over runs: violation 0.048 to 0.055; most negative free entry -0.019 (all pairs), -0.043 (d = 2), -0.053 (d = 1), -0.055 (d = 0); share of free pairs below -1e-6: 0.45 to 0.46 in every row; mass off solutions per cell 0.18 to 0.26.

**Convex lift with local nonnegativity** (`local_nonneg.py`: the engineering study's splitting with the clamp restricted; penalty 3,000, 600 s; `raw/local/`).

| board | d | clamped of free pairs | iterations | residual | objective (full lift: 8.79 on seed 1, 7.98 on seed 2) | mass off solutions per cell |
|---|---|---|---|---|---|---|
| seed 1 | 0 | 0 of 86,565 | 17,924 | 1e-5 | 4.39 | 0.52 |
| seed 1 | 1 | 5,322 | 27,248 | 7e-6 | 4.40 | 0.52 |
| seed 1 | 2 | 35,202 | 15,445 | 3e-4, not converged | 5.73, rising | 0.24, falling |
| seed 2 | 1 | 4,986 | 18,779 | 9e-6 | 4.99 | 0.44 |

An objective under the full lift's minimum cannot belong to a point of the solutions' hull, so d <= 1 is not exact on these boards [D from M].

**Other checks.** `verify_barrier.py` (seeds 0 and 1, 100 random boards and 100 rotation-improved ones each): 0 formula failures on every exchange edge; boards with no descending edge 0 and 0 (random), 33 and 28 (rotation-improved). `masked_gram.py` (seeds 0 to 2): negative eigenvalues 168, 170, 164, 160, 158 at r = 1, 4, 16, 64, 256 on seed 0 (172 to 160 and 169 to 158 on seeds 1 and 2), smallest eigenvalue -2.3 to -3.0 against a trace of 26; 0 at a mixture of 3 solutions. `cp_coupled.py ... gradcheck`: relative error 8e-10.

**Deviations.** Remote runs were first excluded, then allowed at about 23:04 (4 cores of m1) and 00:22 (8 cores). Everything remote is under `./theory-b/` (3 MB, left in place). This Mac carried a load of 55 to 138: two short runs overlapped my one background process for about a minute, a 400-start annealed run was stopped (it returns one point per board; `raw/softanneal_5x5.jsonl` holds seed 0 only), and the barrier check moved to m1. The first baseline pass had a 180 s limit on a shared machine and cut 34 of 50 runs; they were rerun from the same starts with 480 s. The d = 0 limit is half the others', hence the equal-budget counts. The annealed column is my own crude schedule. No 6x6 run. About 11 core-hours on m1, 10 core-minutes here.

## Appendix C. Sources

Opened (abstract pages): Povh and Rendl, https://optimization-online.org/?p=10012 ; Dickinson and Gijben, https://optimization-online.org/?p=11587 ; Zhuang, Chen, Yang, Zhang, https://arxiv.org/abs/2305.18436 ; Ding and Wright, https://arxiv.org/abs/2310.01784 ; Levin, Kileel, Boumal, https://arxiv.org/abs/2207.03512 ; Lai and Yoshise, https://arxiv.org/abs/2107.01538 .

Search summaries only: Gold and Rangarajan 1996 (IEEE PAMI; an excerpt of the body was returned); Kuang, Ding, Park 2012; Burer 2009; de Klerk and Pasechnik 2002; Groetzner and Dür 2020; Motzkin and Straus 1965 (memory). Sibling memos: `relaxation-theory.md`, `relaxation-engineering.md`, `lowrank-relaxation.md`, `lowrank-rust.md` (head only).
