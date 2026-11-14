# Relaxation theory: does the degree-2 semidefinite lift stay exact as the board grows?

Desk study, 2026-11-13 evening. Draft, not independently checked. All boards are generated TOYS. Marks: [M] measured in this study (by me unless "lift study" is named), [D] derived, [C] cited from a link I opened, [C-u] cited, page not opened, [G] guessed.

## 0. Verdict

- The 6x6 prediction was committed at 20:00:44, before any 6x6 file of `scripts/geometry-sdp-lift/raw/` was opened: **still exact at 6x6, probability 60 percent**; first clear failure at 7x7 or 8x8; not exact on the real 16x16. The folder then held no 6x6 lift result: the solves had timed out or been killed (900 s, 3,600 s, 7.7 to 8 GB) [M, lift study]. The prediction is untested.
- No counting argument I can write explains the exactness at 5x5 or locates its end: counting equalities predicts failure already at 4x4, where the lift is exact [D, M]. The prediction's basis is weak.
- Exactness at 16 to 25 cells is not an averaging artefact: the lift removes 219 to 280 placements per 5x5 board that singleton arc consistency keeps [M]. It is still a small-board fact: every asymptotic result that fits this kind of instance predicts failure, and none gives a constant for a 2D board [C, D].
- Cheapest sound form: the lift is a face of the theta body of the conflict graph; factored, its matrix needs 30 GiB on the 16x16 at the rank that carries a guarantee (Y >= 0 must stay), against 183 GiB for one dense copy. The obstacle is exactness past 6x6, not storage.

## 1. Why it is exact at 5x5, and counting

**What the lift is [D].** Let G be the conflict graph: vertices are the P placements, edges join two placements in the same cell, of the same tile, or adjacent with mismatched sides. A solution is a stable set of size N; cells and tiles are cliques, so theta(G) <= N. The lift is the set of optimal points of Schrijver's nonnegative strengthening of the Lovász theta program of G. Two consequences, with Z written as a Gram matrix of vectors v_0, v_p. (i) |v_0 - sum_p v_p|^2 = 1 - sum_p x_p = 0 over any cell or tile, so the row-sum equalities of `lift.py` are implied. (ii) For an inner edge and a colour, the sum of v_p over one cell's placements showing that colour equals the same sum from the other cell: each edge carries one orthogonal splitting of v_0 by colour, and the lift contains the degree-2 lift of the edge-colour model.

**Counting 1: degrees of freedom against equalities [D].** A point of rank r has r(P+1) - r(r-1)/2 degrees of freedom against Z + (N+T-1) r + 1 equalities, Z being the structural zeros. The excess r(P+2-N-T) - r(r-1)/2 - Z - 1 is positive from r = 31 at 4x4 (P = 138, Z = 2,834), from r = 69 at 5x5 (P = 478, Z = 27,203) and from about r = 2,140 on the 16x16. Equalities leave room for non-integral points at every size, including those where the lift is exact. The inequalities (PSD, entrywise nonnegativity) do the work, and no count of this kind sees them. "Few degrees of freedom against many zeros" is not the explanation.

**Counting 2: what changes with n [D; `counts.py`].** Formulas for P and Z are in Appendix A; they give Z = 27,700 at 5x5 without pin against 27,203 measured with it.

| board (frame, inner colours) | P | P/N | zeros, share of pairs | R | cells with no rim neighbour |
|---|---|---|---|---|---|
| 4x4 (2, 4) | 144 | 9 | 28.6% | 1.2 | 0 |
| 5x5 (3, 4) | 484 | 19 | 23.7% | 1.6 | 1 |
| 6x6 (4, 5) | 1,296 | 36 | 18.6% | 2.1 | 4 |
| 7x7 (4, 7) | 2,916 | 60 | 14.6% | 2.7 | 9 |
| 8x8 (5, 8) | 5,776 | 90 | 11.3% | 3.3 | 16 |
| 16x16 (5, 17) | 156,816 | 613 | 2.7% | 9.3 | 144 |

The zero share is about 6/N. R = sqrt(P (1 - rho) / rho) / N, rho the zero share, is the theta value of a structureless random graph of the same size and density, divided by N ([C-u] Juhász 1982, Coja-Oghlan 2003). R near 1 means density alone nearly certifies "no stable set larger than N"; R of 9 means the certificate must come entirely from structure. Each-tile-once supplies 18 to 22 percent of the zeros at every size.

A second candidate scale failed its own check [M; `lam.py`]: the squared correlation one side of an interior tile carries to another is 0.11 to 0.18 on every toy from 5x5 to 8x8 (flat, so it places no threshold) and 0.06 on the official pieces.

**What is measured [M].** The test of exactness is not where the minimum-norm point lands but the largest mass the lift can put on placements that are in no solution (linear objective). That maximum is 0.000 per cell on 8 boards of 9 to 20 cells (Appendix B), where the LP shadow allows 0.39 to 0.89 (0 on one). At 5x5 the lift study has three boards at the minimum-norm point only: 0.0009 (inaccurate solve), 3e-8 and 2e-5 per cell. Two corrections to in this study's headline: the LP's 18 percent is one board (the others give 53 and 45; 70 at 6x6), and "rank 32" is 28 at threshold 1e-3 and 20 at 1e-2 in the later run.

**Prediction [G, on the R scale].** Everything measured exact has R <= 2.0; 6x6 has 2.1. Hence: 6x6 exact with 60 percent, first clear failure at 7x7 (2.7) or 8x8 (3.3), failure on the real 16x16 (9.3) with 98 percent. R is a scale, not a theorem: the graph is not random, and no board with R above 2.0 was small enough for me to solve.

## 2. What the literature predicts

1. **Degree-2 sum of squares is the theta function, and it fails in the hard regime of planted colouring [C].** Banks, Kleinberg and Moore show that theta-based refutation of k-colourability of random regular graphs fails "well above the k-colorability transition" and "everywhere below the Kesten-Stigum threshold" (https://arxiv.org/abs/1705.01194). The thresholds are about 2k ln k and (k-1)^2 [C-u]. Montanari and Sen: the two-group block-model SDP detects down to the threshold up to a vanishing term (https://arxiv.org/abs/1504.05910). [D] Here a cell is a variable with about 600 values at critical density, the low end of that window, where the gap is widest.
2. **Random CSPs [C].** Kothari, Mori, O'Donnell and Witmer: if the predicate supports a t-wise uniform distribution, degree n / (Delta^(2/(t-1)) log Delta) fails to refute (https://arxiv.org/abs/1701.04521). [D] The cell constraint on its four edge colours supports a 1-wise uniform distribution but not a pairwise one (784 tile rotations against about 1,600 pair-marginal equations). So this theorem does not protect a degree-2 failure here: the question is an alphabet-dependent constant, as in item 1.
3. **Global counting constraints do not rescue low degree [C].** Kothari, O'Donnell and Schramm turn a global cardinality constraint into local ones and keep the lower bounds, up to degree of order n for bisection (https://arxiv.org/abs/1809.01207). Each-tile-once is 256 such constraints. [G] By analogy it does not rescue degree 2 either.
4. **Planted clique [C].** Degree-d SOS on G(n, 1/2) keeps value n^(1/2 - c sqrt(d / log n)) (https://arxiv.org/abs/1604.03084). [D] A solution is a clique of size N = 256 among P = 156,816 vertices; sqrt(P) = 396, and at density 0.973 the scale is R N = 2,370, nine times the solution's size.
5. **Assignment relaxations [C].** Non-isomorphism can need SOS degree of order n (https://arxiv.org/abs/1401.2436). For correlated random graphs the common convex relaxation "almost always fails to discover the optimal permutation" (https://arxiv.org/abs/1405.3133). The quadratic-assignment SDP is exact when noise is small against signal (https://arxiv.org/abs/2408.05942). [D] Here each side has one true partner among about 4 x 196 / 17 = 46 compatible sides: noise far above signal.
6. **Worst case [C].** For fixed templates, constant-level Sherali-Adams and linear-level Lasserre solve exactly the bounded-width problems (https://arxiv.org/abs/1612.01147). Edge matching is NP-complete ([C-u] Demaine and Demaine 2007), so the lift cannot be exact on all boards unless P = NP [D].
7. **This folder's surveys** (statistical physics, rows 1 to 3) place a planted instance with a unique solution in the hard phase [C-u, not reopened].

For about one solution at critical density: items 1, 3, 4 and 7 apply in spirit and predict failure for large n. None is proved for a 2D lattice (not an expander) and none gives a board size. The literature supports "not exact at 16x16" and is silent on 6x6 against 8x8.

## 3. Relation to what was measured

**Why PSD is exact where degree-2 linearisation only propagates [D].** Linearisation (gap 6a) uses equalities over a field and has no signs. "A sum of nonnegative terms is zero, so each is zero" does not exist in linear algebra; the LP shadow already has it, and PSD adds quantitative coupling between pairs that share no constraint. The two degrees are not on one scale.

**Is it more than arc consistency plus averaging? Yes at 5x5 [M; `sac.py`].** Singleton arc consistency with a cell-to-tile matching test leaves 219 of 267, 280 of 385 and 257 of 372 placements that are in no solution on the three 5x5 boards. The lift's minimum-norm point keeps 213, 93 and 105 placements above 1e-3 where the solutions use 211, 93 and 106 [M, lift study]. At 6x6 the same rule leaves 1,015, 934 and 816. This is real inference at 25 cells. [D] It is not path consistency either (PSD bounds a pair's weight without forcing a common support); the inference is quantitative, and I cannot say how it scales.

**What does not count as evidence [M; `sub.py`].** The lift is exact on the 6x6 interior with the true frame pinned (2 boards) and refutes 8 of 8 dead depth-18 prefixes of a 6x6 board; both are trivial for search (2 to 358 nodes). Pinning true tiles only adds equalities, so a leak there would have proved the full board not exact; none was found.

**The single measurement that separates the two readings.** Run the lift where there is nothing to average over: boards or prefixes with NO solution, which singleton arc consistency does not refute and plain search needs at least 10^3 nodes to refute. Report the share declared infeasible against the number of free cells. "Infeasible" there is global reasoning; "feasible" is the end of exactness. My attempt at 4x4 (colour-swapped tile sets) was uninformative: the LP already refuted all three.

## 4. Cheaper sound forms on the 16x16

P = 156,816, zeros Z = 3.32e8, free pairs F = 1.20e10 [D; `counts.py`]. "Breaks it" answers whether each-tile-once is the term that defeats the form.

| form | memory | flops per iteration | sound | each-tile-once breaks it |
|---|---|---|---|---|
| as coded (SCS, direct) | 72 F bytes = 0.9 TB before fill (measured: 0.5 GB at 5x5, 8 GB at 6x6) | 10 P^3 = 3.9e16 | yes | no, the order P does |
| matrix-free first order, dense projection | 8 P^2 = 183 GiB per copy | 3.9e16 | yes | no |
| facial reduction | order P + 1 - (N + T - 1): 0.3% smaller | same | yes | no |
| chordal decomposition | largest block at least 4 (I^2/2 + I/2) = 77,224 of 156,816 | one eighth of dense | yes | **yes** |
| symmetry reduction | no gain | no gain | yes | no (no automorphism, gap 5) |
| factored, rank r, no Y >= 0 | 8 P r: 0.07 GiB at r = 64, 30 GiB at r = 25,782 | gradient 2 Z r: 4e10 at r = 64, 1.7e13 at r = 25,782 | only at r >= 25,782 | no |
| factored, Y >= 0 kept | plus 8 F = 96 GB of multipliers | P^2 r / 2: 8e11 at r = 64 | as above | no |
| sketching (conditional gradient) | Z + P r numbers | one sparse eigenvector, 3e8 per multiply | yes, sublinear rate | no |

Notes.
- **Chordal [D].** Without the same-tile zeros the conflict graph is a strip with blocks of order 15 x 784 = 11,760. With them, contracting the four rotations of a (cell, tile) pair gives the rook's graph K_196 x K_196, of treewidth I^2/2 + I/2 - 1 ([C-u] Lucena 2007): one block holds half the matrix. This is obstruction O1 again. Chordal theory: https://arxiv.org/abs/2107.02379 [C].
- **Order [D].** Without each-tile-once the natural variables are edge colours: 480 edges x 17 colours, order about 8,160. The rule is why the matrix has order 156,816: 19 times larger, 7,000 times the dense flops.
- **Low rank, factored runs, the form without Y >= 0, GPUs:** Appendix D. In short: the rank with a guarantee is 25,782, and the guarantee is generic, not proved for this degenerate program [C, D]; my factored runs converge in 25 s at 4x4 and not within 10 minutes at 5x5 [M]; a published GPU low-rank solver handles stable-set programs of this size [C].
- At 6x6 one dense copy of Z is 13 MB [D]; the 6 to 8 GB is the solver's factorisation, not the problem.

## 5. Odds and experiments

| question | my odds [G] | smallest experiment that settles it | kill line | cost [D] |
|---|---|---|---|---|
| exact at 6x6 (4, 5) | 60% | largest off-solution mass (linear objective) on seeds 0 to 2, by a matrix-free splitting with a dense projection of order 1,297 | 0.05 per cell or more on 2 of 3 boards: not exact, stop all engineering | 2.2e10 flops per iteration, 10^4 iterations: about 1 core-hour per board, 50 MB |
| exact at 8x8 (5, 8) | 20% | the same at 7x7 then 8x8, three boards each | the same | 7x7: 2.5e11 per iteration, a core-day per board; 8x8: 1.9e12, a GPU or a week of one core |
| exact at 16x16 with five clues | 2% | not reachable directly: the ladder 6 to 10 at critical colour counts; only if 10x10 is exact, the factored form on a PAINTED 16x16 | any ladder size at 0.05 per cell ends it; at 16x16, failure to return the planted board | 10x10: 5e13 per iteration, GPU only; 16x16: 30 GiB, days of GPU [G] |

Not checked: any lift at 36 cells or more; the linear certificate at 5x5; the form without Y >= 0 beyond 4x4; the section 1 lemmas by a second reader.

## Appendix A. Formulas (no pin, uniform colours per edge class)

E = 4(n-2), I = (n-2)^2, c2(a) = a(a-1)/2, cf frame colours, ci inner colours.
- P = 16 + E^2 + 4 I^2, N = T = n^2.
- same cell: 4 c2(4) + E c2(E) + I c2(4I).
- same tile, other cell: 4 c2(4) + E c2(E) + I (c2(4I) - 6I).
- adjacent mismatch, other tile: 2(n-2)(n-3)(16 I^2 - 16 I)(1 - 1/ci) + 4(n-2)(4 E I)(1 - 1/ci) + 4(n-3)(E^2 - E)(1 - 1/cf) + 32 E (1 - 1/cf).
- R = sqrt(P (1 - rho) / rho) / N, rho = Z / c2(P).
- Barvinok-Pataki rank for m equalities: the least r with r(r+1)/2 >= m.
- Chordal block with the tile rule: at least 4 (I^2/2 + I/2).

## Appendix B. Runs (all in `scripts/relaxation-theory/`, one core, nice 10, each under 10 minutes)

| script | what | result |
|---|---|---|
| `sac.py` | AC, AC with matching, singleton AC, singleton AC with matching against the solution set, 5x5 and 6x6, seeds 0 to 2 | `raw/sac_*.json`; numbers in section 3 |
| `leak.py` | largest off-solution mass, LP and lift, small rectangular boards, framed or frameless; balanced colour swaps for boards with no solution | `raw/leak_batch1.jsonl`, `leak_batch2.jsonl`, `unsat_4x4.jsonl`; `leak_4x4.jsonl` |
| `sub.py` | the lift on a board with some cells pinned (true frame, true rows, dead prefixes); `theta` drops Y >= 0 | `raw/sub_true.jsonl`, `sub_dead18.jsonl`, `theta_4x4.jsonl`, `theta_4x4_long.jsonl` |
| `bm.py` | factored form, augmented Lagrangian, gradient checked against finite differences to 9 digits | `raw/bm.jsonl` |
| `counts.py`, `lam.py` | section 1 and 4 tables; side-to-side colour correlation | printed |

Largest off-solution mass per cell (SCS, eps 1e-5, status solved):

| board | P | solutions | search nodes | LP | lift |
|---|---|---|---|---|---|
| 4x4 (2,4) seed 0 | 138 | 8 | 556 | 0.395 | 0.000 |
| 4x4 (2,4) seed 1 | 138 | 2 | 370 | 0.824 | 0.000 |
| 4x4 (2,4) seed 2 | 138 | 2 | 477 | 0.844 | 0.000 |
| 4x4 (2,4) seed 3 | 138 | 1 | 579 | 0.888 | 0.000 |
| frameless 3x3, 5 colours, seed 0 | 257 | 2 | 256 | 0.778 | 0.000 |
| frameless 3x3, 5 colours, seed 1 | 257 | 11 | 751 | 0.858 | 0.000 |
| frameless 3x3, 6 colours, seed 0 | 257 | 3 | 297 | 0.000 | 0.000 |
| 4x5 (3,4) seed 0 | 254 | 2 | 860 | 0.892 | 0.000 |
| 6x6 seed 1, true frame pinned | 371 | 1 | 358 | not run | 0.000 |
| 6x6 seed 2, true frame pinned | 377 | 1 | 141 | not run | 0.000 |

Frameless 3x3: zero share 45 percent, P/N = 28.6, R = 2.0. `leak.py` uses its own painter and pin (planted tile of cell 0), so its 4x4 boards are not the lift study's 4x4 boards. Boards made by colour swaps are not checked for repeated or self-symmetric tiles.

Theta form without Y >= 0, 4x4 (2,4), lift study's boards: seeds 1, 2, 3 give 0.000; seed 0 (8 solutions) gives 0.0046 per cell at "solved (inaccurate)" after 90 s and 22,425 iterations, and 0.0030 per cell with 12 off-solution placements above 1e-3, still "inaccurate", after 480 s and 136,550 iterations. The full lift on the same board returns 0.000 in 475 iterations (1.4 s). So without Y >= 0 the program is either slightly inexact on that board or about 300 times slower to converge; I could not tell which.

Factored runs: see `raw/bm.jsonl`. 5x5 seed 0, rank 240: (a) leak objective, penalty to 1e5: residual 1e-4, leak 0.186 per cell at 210 s; (b) penalty capped at 2,000: residual 1.2e-3, leak 0.371 at 547 s; (c) no objective, random start: residual 1e-4, leak 0.125 at 303 s. None converged. Entries of Z are of order 2.5e-3 at 5x5 and 1.2e-2 at 4x4, so equal absolute residuals are not equal relative ones.

Compute used: about 65 core-minutes, the machine under a load average of 40 to 50 from other jobs (timings above are not benchmarks).

## Appendix C. Sources not opened

Juhász 1982 and Coja-Oghlan 2003 (theta of random graphs; arXiv math/0306266); Barvinok 1995 and Pataki 1998; O'Carroll, Srinivas, Vijayaraghavan, NeurIPS 2022; Zhao, Karisch, Rendl, Wolkowicz 1998; de Klerk and Sotirov 2010; Demaine and Demaine 2007; Lucena 2007; Barto and Kozik 2012. Each was seen in a search summary in this study; statements taken from them are marked [C-u].

## Appendix D. Notes to section 4

- **Low rank [C].** A solution of rank at most sqrt(2m) exists (Barvinok, Pataki [C-u]); with m = Z + P + 1 = 3.3e8, r = 25,782. Above that rank the factored problem "almost never has any spurious local optima" for a class of SDPs with few equalities (https://arxiv.org/abs/1606.04970); below it "second-order critical points are not generically optimal, even when the global solution is rank 1" (https://arxiv.org/abs/1812.03046); Max-Cut instances fail even above it ([C-u] O'Carroll et al. 2022). The lift is degenerate (hundreds of millions of active zeros), so the generic guarantee is not known to hold here [D].
- **Factored form in practice [M; `bm.py`].** A feasible factored point with mass off the solutions would prove non-exactness with no dual certificate. On two 4x4 boards the run reaches residual 1e-6 and leak 2e-6 in 25 s; a control without the tile rule leaks 0.94. At 5x5 it is slow: after 210 to 550 s the residual is 1e-4 to 1e-3 with 12 to 37 percent of the mass still off the solutions and falling. Near-feasible leaky points are plentiful at 5x5, so the solver tolerance needed to see exactness tightens with size. Engineering will meet this before memory.
- **Without Y >= 0 [M].** The theta form alone is exact on 3 of 4 boards at 4x4. On the fourth it still shows 0.003 per cell off the solutions after 136,550 iterations, where the full lift needs 475 (Appendix B). The entrywise condition cannot be dropped for free: plan on the factored form with Y >= 0 kept, at P^2 r / 2 flops per gradient.
- **GPUs [C].** SCS offers GPU back ends for the linear system only (https://www.cvxgrp.org/scs/linear_solver/index.html); its eigen-decomposition is not addressed there. cuHALLaR, a low-rank augmented Lagrangian on GPU, reports maximum-stable-set SDPs on graphs with 8.4 million vertices and instances with 3e8 constraints (https://arxiv.org/abs/2505.13719). The theta form of the 16x16 (1.6e5 vertices, 3.3e8 edges) is inside that published range. Sketching: matrix variables with over 10^14 entries on a laptop-class machine (https://arxiv.org/abs/1912.02949).
