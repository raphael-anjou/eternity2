# Theory D: the formulation of the lift and the use of its output

2026-11-13. Draft, not independently checked. All boards are generated TOYS (5x5, frame 3, inner 4, the shared painter and pin, seeds 0 to 4): nothing here is Eternity II progress. Marks: [M] measured in this study by a script in `scripts/theory-d/`, [D] derived, [G] guessed. Nothing is cited: no page was opened.

Assessed at 16x16 first (project, 23:00): 32 GB, 10 cores, no dense P x P object.

## 0. The reading

1. **The aspect is nearly empty.** Of eight devices for a smaller lift, six are inexact or vacuous at 5x5. One is an exact rewriting that removes the conflict list, not the pair work. One, nonnegativity kept only within distance 2 (F2), fits 32 GB at 16x16; its 5x5 evidence is mixed: exact on 2 boards of 5, trending on 1, probably leaking on 1, moot on 1.
2. **Where exactness lives [M].** In Y >= 0 between placements of cells that share no constraint. With every zero and the full face kept, dropping Y >= 0 leaves 0.53 of the mass per cell off the solutions (converged, violation 6e-8). Keeping it between touching cells leaves 0.16 to 0.72 on four boards.
3. **A fact outside my aspect, found by the controls [M]: the full lift is not exact on every 5x5 toy.** On seed 3 (403 solutions) the full form settles with 0.0047 of the mass per cell on 10 placements that no solution uses, at violation 5e-9, at two penalties. The other four boards are exact. One solver only.
4. **No use of the output reaches the real board without exactness there.** A root filter gains from depth 0 and needs 5 to 17 percent of the placements removed.
5. **Odds of 10 bits on the real 16x16: 1 percent for F1, under 0.5 percent for F2, 0 for the rest.**

## 1. Ranking at 16x16

Sizes derived (Appendix A). Flops per evaluation of a factored form at rank 16 (4 x pairs x r). Memory: 0.5 GB of factor and history plus 8 bytes per pair multiplier.

| # | mechanism | pairs | flops | memory | fits 32 GB | at 5x5 | breaks first as P grows | odds |
|---|---|---|---|---|---|---|---|---|
| F1 | full lift; zeros as vector equalities; Y >= 0 on all pairs, in blocks | 1.2e10 | 8e11 | 96 GB; under 3 GB as pure penalty | as pure penalty | exact on 4 boards of 5 [M] | multiplier store at 14x14 (29 GB); evaluations; exactness | 1% |
| F2 | Y >= 0 within Chebyshev distance 2 | 1.3e9 | 8e10 | 11 GB | yes | mixed, section 2 [M] | iterations (2 times F1's or more at 5x5); sufficiency of the radius | under 0.5% |
| F3 | Y >= 0 between touching cells | 5.6e8 | 4e10 | 5 GB | yes | **leaks** 0.16 to 0.72 [M] | broken | 0 |
| F4 | theta form (zeros only) | 3.3e8 | 2e10 | 3 GB | yes | **leaks** 0.53 [M] | broken | 0 |
| F5 | far pairs as block sums over colour classes | 5.7e8 to 2e9 | 4e10 to 2e11 | 5 to 20 GB | yes | **leaks** 0.46 to 0.50 [M] | broken | 0 |
| F6 | partial lift: pairs within distance 1, tile sums linear | 5e8 | 3e10 | 4 GB | yes | **leaks** 0.53 [M] | broken | 0 |
| F7 | 2x2 windows glued on edges, tile sums linear (LP) | 7.7e8 variables | not priced | 6 GB per vector | marginal | **leaks** 0.38 to 0.81 [M] | broken | 0 |
| F8 | exact lift of a window, rest free | 1.9e8 per 5x5 window | small | small | yes | removes 2 to 8 placements [M] | vacuous by counting [D] | 0 |

"Leaks": a point feasible to the stated violation carries that mass per cell on placements no solution uses, which proves the form inexact on that board.

## 2. Smaller lifts (question 1)

**F1, an exact rewriting [D].** Write Z as the Gram matrix of v_0 and the v_p. For an inner edge and a colour c, let a(c) be the sum of v_p over one cell's placements showing c on the shared side, b(c) the same from the other cell. The 3.3e8 zeros are equivalent to: the sum of v_p over each cell and over each tile equals v_0; a(c) = b(c) for every edge and colour; Y >= 0 on those same pairs (proof: Appendix C). Implementation: V on a product of spheres (the low-rank memo's change of variable), (512 + 7,440) r linear equalities at O(P r) flops, and one uniform term min(0, V V^T) over all pairs, in row blocks. No conflict list (10 GB of indices at 16x16), no gather; nothing saved on the 1.2e10 pairs. Numerical check inconclusive (objective 4.61 against 4.47, both unconverged) [M].

**A nonnegative factor is not a relaxation [D].** With v_p >= 0 entrywise, Y_pp = x_p and the cell sums force each coordinate of v_p to 0 or to v_0's: every coordinate is an integral board (study B's ground).

**F2 to F4: which nonnegativity is needed [M, `admm_var.py`].** The engineering splitting with a mask on the clamp. Mass per cell off the solutions:

| board (solutions) | all pairs | distance <= 2 (73% of free pairs) | distance <= 1 (26%) |
|---|---|---|---|
| seed 0 (36) | 2e-11 | **0.077 at 17,003 iterations, slowing** (0.106 at the weaker penalty) | 0.162 |
| seed 1 (6) | 2e-11; largest-leak objective 8e-9 | 3e-11; largest-leak objective 0.000 | 0.500; largest-leak 0.64 |
| seed 2 (8) | 3e-12 | 4e-12 | 0.430 |
| seed 3 (403) | 0.0047 | 0.0047 | 0.0047 |
| seed 4 (2) | 1e-15 | 0.087 at 17,017 iterations, falling steadily | 0.723 |

Controls on seed 1 (Appendix B): Manhattan distance 3 (61,986 pairs) is exact. A random subset as large as distance 2 is at 0.17 to 0.19 after 13,600 iterations and falling, where distance 2 was exact by 11,000. Manhattan distance 2 (35,202 pairs) sits at 0.41, a random subset of that size at 0.48.

Reading.
- Distance 2 suffices on seeds 1 and 2, plausibly on 4, and probably not on seed 0, where the full form is exact in 8,500 iterations and distance 2 still holds 0.077 after 17,003, declining ever more slowly. Not a solid result.
- It costs iterations: 11,000 against 5,500 (seeds 1 and 2), over 17,000 against 11,000 (seed 4).
- 5x5 cannot test a radius: its nine inner cells are all within distance 2 of each other, and random subsets of equal size behave alike, only slower. Distance 2 covers 73 percent of the free pairs at 5x5, about 66 at 6x6, 40 at 8x8, 10 at 16x16 [D].

**F5 [M, `admm_agnn.py`].** Far pairs constrained only through block sums over colour classes: 0.50 and 0.46 on seed 1.

**F6 [M, D].** Same-tile zeros only between touching cells, tile sums linear, full cone: 0.528, converged. A relaxation of F3, so it had to leak: drop the long-range tile coupling and the chordal structure returns with the leak.

**F7 [M, `winlp.py`].** Largest mass off the solutions: 0.84, 0.78, 0.47 with edge pairs; 0.81, 0.74, 0.38 with 2x2 windows (seeds 1, 2, 0).

**F8 [M, D; `wincons.py`].** Window consistency (the support of an exact lift of the window alone) removes, beyond arc consistency, 2 to 8 placements with 2x2 or 3x3 windows, 7 to 127 with 4x4, 119 to 321 with 4x5, of 240 to 325. On the real pieces a 5x5 interior window has about 2^75 fillings with the rest free, 2^34 with two sides fixed [D]: a window lift there returns every placement (the plan document 9.3 item 1, not re-proposed).

**Symmetry, types, low rank:** nothing (gap 5, gap 6b; leaky points have rank 118 to 236 of 479 [M]).

## 3. Tighter at the same size (question 2)

On four boards nothing tighter is measurable at 5x5; seed 3 is the first testbed for a cut, unused here.

- Edge colour-count equalities, clique inequalities on cells, tiles and edge colours, colour totals: implied by F1's equalities [D].
- Parity and integer-balance laws: the real-valued part is implied, the integer part is not convex.
- Local triangle inequalities (about 1.5e9 candidate triples at 16x16 [D]), a cone on windows: local, where the measured deficit is between far cells [G: no effect]. A lift per pinned placement: P solves.

No cut worth building; odds that one moves exactness at 8x8 by 10 points: 5 percent [G].

## 4. Decoding and use (question 3)

**Root filter, measured ceiling [M, `rootfilter.py`].** Row scan restricted once to the solutions' placements (seeds 0 to 2): 4.1, 6.6, 5.8 bits overall, 5.4 to 8.4 at the best level, 2.3 to 4.3 already at depth 8 of 25. Arc consistency at the root: 0.01 to 0.7 bits. This use meets the program's rule: a smaller branching factor at every depth, not a refutation.

**Price [D, M].** Removing a share f of placements independently of context is worth d log2(1/(1-f)) bits at depth d; at 5x5 the realised gain at the widest level is 0.22 to 0.33 of that. Ten bits by depth 128 needs f between 5.3 and 17 percent. The filter need only keep the solution's placements, not be sound.

**What inexact forms deliver [M, `kills.py`].** Placements under 1e-4 that arc consistency keeps, uncertified: 10 to 62 of 267 to 376 available on seeds 1, 2, 4; 110 to 127 of 240 on seed 0. On a board where propagation sees nothing past the clue corners I expect none [G]. As a value ordering an inexact point is one more statistic under O2.

**Pin and re-solve.** A toy concern: the real board has about one solution, so an exact point is integral.

**Once per node.** A solve of F2 is 2^41 to 2^44 row-scan nodes [D], and as a refutation it is a check that prunes: those bite after depth 144.

**Path to 16x16 without the full lift there:** F2, then the root filter. It needs the radius to stay sufficient [G, 20%], the lift exact at 16x16 [theory memo, 2%], a nonconvex solver that reaches the point [G, 50%].

## 5. The one computation (question 4)

At 5x5: (a) seed 3 through the lift study's SCS model with the largest-leak objective (about 20 core-minutes): is the full lift inexact on a 5x5 board? It bears on the theory memo's premise. (b) Seed 0 at distance 2 to residual 1e-7 (about 2 core-hours): a clean leak closes F2.

Past 5x5: the radius profile on the engineering memo's step 1 boards, the splitting at 6x6 and 7x7 with the clamp masked at distance 2 and 3 beside the full form. One line of code on runs already planned, closed in this study. If distance 2 leaks at 7x7, the lift needs all 1.2e10 pairs and F1 as pure penalty is the only form at 32 GB. Build nothing before that.

## 6. Not checked

- Any board above 5x5. Seeds 0, 3, 4 in F4 to F8 (seed 1 mostly, seeds 0 to 2 for F7, F8 and the root filter).
- Certificates: only seed 1 has largest-leak runs, and those are points of a first-order method, not dual certificates. Runs above violation 1e-7 are near-feasible points.
- Seed 3 by a second solver; which 10 placements carry the leak.
- F1's equivalence numerically and by a second reader. F2 in factored form. Any timing at 16x16 (flops are counts, at 1e11 per second for 10 cores).
- The parallel studies A, B, C were not read; overlap is possible on F1's kernel (A), the nonnegative factor (B), F2's iteration counts (C).

## Appendix A. Sizes [D]

Uniform colour counts, no pin; n = 5 (3,4), 6 (4,5), 8 (5,8), 16 (5,17).

| quantity | 5x5 | 6x6 | 8x8 | 16x16 |
|---|---|---|---|---|
| P | 484 | 1,296 | 5,776 | 156,816 |
| free pairs | 8.9e4 | 6.8e5 | 1.5e7 | 1.2e10 |
| zeros | 2.8e4 | 1.6e5 | 1.9e6 | 3.3e8 |
| matched pairs of side-adjacent cells | 5.3e3 [M] | 2.4e4 | 1.7e5 | 1.3e7 |
| cell pairs within Chebyshev 2, of all | 168 of 300 | 270 of 630 | 546 of 2,016 | 2,610 of 32,640 |
| free pairs within Chebyshev 2 (F2) | 6.3e4 [M] | about 4.5e5 | about 6e6 | 1.2e9 |
| F1 vector equalities (cells and tiles + edge colours) | 50 + 144 | 72 + 280 | 128 + 812 | 512 + 7,440 |
| 2x2 window fillings (F7) | 1.6e4 to 2.4e4 [M] | about 2.4e5 | about 2.6e6 | 7.7e8 |
| multipliers at 8 bytes, F1 / F2 | 0.7 MB / 0.5 MB | 5 MB / 4 MB | 120 MB / 48 MB | 96 GB / 10 GB |

Cell pairs within Chebyshev distance k: (((2k+1) n - k(k+1))^2 - n^2) / 2. F2 at 16x16: 1,950 inner cell pairs x 784^2, rim terms, and the same-cell and same-tile pairs (6.0e7 each). F3: 702 inner cell pairs x 784^2 plus the same.

Window counting on the real pieces: a w x w interior window with the rest free has about prod(196 - i, i < w^2) x 4^(w^2) / 17^(2w(w-1)) fillings: 2^74.6 at w = 5, 2^96 at w = 6. Each fixed boundary edge costs 4.09 bits. A cell adds log2(t) + 2 - 2 log2(17) bits, positive while t > 72 tiles remain.

Root filter, measured nodes (seeds 0 to 2): 2^11.7, 2^8.4, 2^9.2 on the solutions' placements against 2^15.8, 2^15.0, 2^15.0. f for 10 bits by depth d at efficiency e is 1 - 2^(-10 / (d e)): 5.3 percent (d = 128, e = 1), 7.0 (d = 96, e = 1), 16.5 (d = 128, e = 0.3).

## Appendix B. Runs

All in `scripts/theory-d/`, nice 10, one thread each, 5x5 only, `uv run --with numpy --with scipy python -I -B`. One process at a time and at most 10 minutes until 00:20; then up to 4 at once and at most 15 minutes (limit raised). `bmcopy/` copies the engineering study's copies of `bm.py`, `board.py`, `common.py`. About 6 core-hours, load average 10 to 37.

| script | what | output in `raw/` |
|---|---|---|
| `admm_var.py` | the engineering splitting with switches: nonnegativity radius, same-tile zero radius, tile vectors in the face, l2 or largest-leak objective | `admm_*`, `p_s*`, `q_s*` |
| `admm_mask.py` | nonnegativity on a Manhattan radius or a random subset | `p_s1_manh*`, `p_s1_rand*`, `p_s2_manh2` |
| `admm_agnn.py` | far pairs as block sums (F5) | `agnn_*` |
| `admm_agg.py` | zeros as edge-colour vectors in the face (F1, local pairs only) | `agg_*` |
| `winlp.py`, `wincons.py` | F7, F8 | `winlp.jsonl`, `wincons_*.jsonl` |
| `rootfilter.py`, `kills.py` | section 4 | `rootfilter.jsonl`, `kills.txt` |
| `fin.py`, `summ.py` | one line per run; trajectories | |

Every run (penalty 300 unless stated; mass per cell off the solutions, objective ||x||^2, largest violation, iterations):

| board | form | pairs with Y >= 0 | mass off | objective | violation | iterations |
|---|---|---|---|---|---|---|
| s0 | full | 86,800 | 4.6e-4, falling | 5.1615 | 7e-8 | 23,369 |
| s0 | full, penalty 3000 | 86,800 | 2e-11 | 5.2186 | 2e-10 | 18,078 |
| s0 | distance 2 | 63,445 | 0.106 | 4.457 | 2e-6 | 22,428 |
| s0 | distance 2, penalty 3000 | 63,445 | 0.077 | 4.625 | 7e-7 | 17,003 |
| s0 | distance 1 | 22,573 | 0.162 | 4.263 | 4e-7 | 7,677 |
| s1 | full | 86,565 | 2e-11 | 8.7909 | 9e-10 | 12,725 |
| s1 | full, largest leak | 86,565 | 8e-9 | | 1e-8 | 5,985 |
| s1 | distance 3 | 83,442 | 1e-12 | 8.7907 | 2e-11 | 28,383 |
| s1 | distance 2 | 63,210 | 3e-11 | 8.7907 | 4e-10 | 17,744 |
| s1 | distance 2, largest leak | 63,210 | 0.000 | | 2e-8 | 10,861 |
| s1 | Manhattan 3 | 61,986 | 5e-11 | 8.7914 | 2e-9 | 14,337 |
| s1 | random, two draws | 63,210 | 0.186, 0.174, falling | 6.26, 6.39 | 1e-5 | 13,572, 13,951 |
| s1 | Manhattan 2 | 35,202 | 0.410 | 4.735 | 1e-5 | 13,985 |
| s1 | random | 35,202 | 0.478 | 4.549 | 4e-6 | 14,520 |
| s1 | distance 1 | 22,338 | 0.500 | 4.473 | 1e-6 | 13,834 |
| s1 | distance 1, largest leak | 22,338 | 0.642 | | 7e-6 | 16,826 |
| s1 | distance 1 + block sums by N (1,956 blocks) | | 0.500 | 4.477 | 1e-6 | 12,095 |
| s1 | distance 1 + block sums by N, W (15,608 blocks) | | 0.457, drifting | 4.628 | 6e-6 | 12,223 |
| s1 | none (theta form) | 0 | 0.527 | 4.386 | 6e-8 | 14,682 |
| s1 | F6 partial lift | 22,338 | 0.528 | 4.385 | 6e-8 | 14,495 |
| s1 | F1 rewriting, local pairs | 49,776 | 0.462 | 4.609 | 5e-6 | 22,080 |
| s2 | full | 86,229 | 3e-12 | 7.9761 | 1e-10 | 17,481 |
| s2 | distance 2 | 62,874 | 0.022, falling | 7.681 | 1e-6 | 40,424 |
| s2 | distance 2, penalty 3000 | 62,874 | 4e-12 | 8.0185 | 2e-10 | 17,890 |
| s2 | Manhattan 2 | 34,866 | 0.394 | 5.123 | 4e-6 | 19,913 |
| s2 | distance 1 | 22,002 | 0.430 | 5.036 | 9e-7 | 12,459 |
| s3 | full | 87,585 | 0.00470 | 4.0767 | 5e-9 | 22,648 |
| s3 | full, penalty 3000 | 87,585 | 0.00494 | 4.0802 | 2e-9 | 16,654 |
| s3 | distance 2 | 64,230 | 0.00470 | 4.0767 | 6e-10 | 22,638 |
| s3 | distance 1 | 23,358 | 0.00474 | 4.0767 | 2e-9 | 7,377 |
| s4 | full | 86,412 | 1e-15 | 15.5000 | 2e-12 | 11,078 |
| s4 | distance 2 | 63,057 | 0.087, falling | 13.015 | 9e-6 | 17,017 |
| s4 | distance 1 | 22,185 | 0.723 | 4.324 | 2e-6 | 7,615 |

At penalty 3000 the objective barely acts (engineering memo), so those objectives sit above the minimum. Seed 3: 403 solutions use 373 of the 478 placements; the full form's point has rank 227.

Window consistency, placements removed that arc consistency keeps (seeds 0, 1, 2): 2x2: 5, 2, 3. 3x3: 5, 6, 8. 4x4: 7, 18, 127. 3x5: 9, 14, 28. 5x3: 9, 21, 38. 4x5: 119, 272, 229. 5x4: 127, 321, 226.

## Appendix C. Proof of the rewriting (F1)

Assume Z PSD, Y_pp = x_p = v_p . v_0, |v_0| = 1.
- Zeros imply equalities. With same-cell zeros and the cell sum of x equal to 1, |v_0 - sum of v_p over the cell|^2 = 1 - 2 + 1 = 0; the same for a tile. For an edge: a(c) . a(c') = 0 for c != c' (same cell), the a(c) and the b(c) each sum to v_0, and a(c) . b(c') = 0 for c != c' (mismatch zeros). So a(c) . b(c) = a(c) . v_0 = |a(c)|^2, symmetrically |b(c)|^2, hence |a(c) - b(c)|^2 = 0.
- Equalities and nonnegativity imply zeros. v_p . (v_0 - sum over the cell) = 0 says the Y_pq over the other placements of the cell sum to 0; each is nonnegative, so each is 0. The same for a tile. From a(c) = b(c): a(c) . b(c') = b(c) . b(c') = 0 for c != c', a sum of nonnegative entries over the mismatched pairs, so each is 0.
- The nonnegativity used is on conflict pairs, which Y >= 0 on all pairs supplies.
