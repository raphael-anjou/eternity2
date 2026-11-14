# Theory A: the cost of one sweep, from structure (2026-11-13, late)

Draft, not independently checked. Every board is a generated TOY; nothing here is Eternity II progress. Marks: [M] measured in this study by a script in `scripts/theory-a/` (5x5 only, one core, nice 10, load 16 to 60), [D] derived, [memo] taken from a sibling memo, [G] guessed. Nothing is cited: I opened no link.

Assessed at the real scale first (project, 23:00): P = 156,816, 1.2e10 pairs, 3.3e8 conflict pairs, 32 GB, 10 cores.

## 0. The reading first

1. **Every equality of the relaxation collapses to O(P r), with no pair list** [D, M]. Given nonnegativity on the conflict pairs, the 3.3e8 zero constraints are equivalent to 7,952 linear vector equalities on the factor: cell sums, tile sums, and one equality per inner edge and colour. At 16x16 and r = 64 that is 3.5e8 flops and 4 MB of multipliers against 8.5e10 flops and 10.6 GB today.
2. **The entrywise nonnegativity of the free pairs does not collapse, cannot be made lazy, and is nearly the whole cost** [D, M]. Along a 5x5 run 48 to 85 percent of all pairs contribute to the gradient at every evaluation: no small active set. The exact structured sweep gains 1.4 times in flops at 5x5 (3 to 5 times in NumPy wall time) and 1.3 times at 16x16, where it still needs 4.8e12 flops per sweep and 100 GB of multipliers. **By the study's rule the exact relaxation in factored form is a failure at scale, with or without this memo.**
3. **The multiplier count is not cut** [D, M]. The Z free-sign multipliers go, but a sign-constrained multiplier appears on the same pairs: 94 percent of the conflict pairs carry one (median, one 5x5 run).
4. **Three forms have no P^2 term with a real constant.** Each pays elsewhere:

| rank at 16x16, r = 64 | flops per sweep | memory | what it gives up | what breaks first as P grows |
|---|---|---|---|---|
| 1. equalities only, on a nonnegative factor (study B's object) | 3.5e8 | 1.9 GB | convexity: its feasible set is exactly r weighted copies of the puzzle [D] | nothing in cost; the landscape |
| 2. equalities + zeros as sums of squares through r x r moments | 1.3e10 | 2.4 GB | free-pair nonnegativity (theta form: inexact or 300 times slower on 1 of 4 boards at 4x4 [memo]); pair multipliers | r^2 (fine to a few hundred) |
| 3. equalities + nonnegativity on cell, tile and adjacent blocks only | 1.3e11 | 4.7 GB | free-pair nonnegativity, as above | pair count 6 P^2 / N: block multipliers pass 32 GB near 24x24 [D] |
| 4. exact, structured | 4.8e12 | 100 GB (1.9 GB as a pure penalty) | nothing | **memory, at P near 89,000, between 12x12 and 16x16**; as a pure penalty, 5 to 10 s per sweep [G] |
| 5. today (`bm.py`) | 6.4e12 | 160 GB | nothing | memory, same place |

5. **Fewer flops is not sooner solved** [M, one paired start]. The structured sweep reached violation 1e-3 after 8,052 evaluations and 1e-6 later, at a feasible mixture of about 5 solutions that neither rounding reads as a board in 52,461 evaluations. Today's sweep rounded to a solution after 14,450 evaluations, still at violation 1e-2. Decoding is study D's.

## 1. What collapses (question 1)

Notation: v_0 = e_1, x_p = <v_0, v_p>, Y_pq = <v_p, v_q>, and the sphere |v_p|^2 = x_p holds by construction. Group sums, all computed in one pass of 6 P r additions: S_c (cell c), U_t (tile t), and for an inner edge e = (a, b) and colour k, A_{e,k} (placements of a showing k on e) and B_{e,k} (the same in b). M_g = sum of v_p v_p^T over a group g (r x r).

Identities, true at every V [D; checked to 1e-15 at a random 5x5 point, `identities.py`]:

- C1. Sum over p != q in cell c of Y_pq = |S_c|^2 - sum of x_p over c. Tiles: the same with U_t.
- C2. Sum over the mismatched pairs of edge e of Y_pq = <S_a, S_b> - sum over k of <A_{e,k}, B_{e,k}>.
- C3. Sum over p != q in c of Y_pq^2 = |V_c^T V_c|_F^2 - sum of x_p^2. Mismatched pairs of e: tr(M_a M_b) - sum over k of tr(M_{a,k} M_{b,k}). All pairs: |V^T V|_F^2.

Costs: C1 and C2 are O(P r) in total; C3 is O(P r^2), about 20 P r^2 with gradients [D, constant G]. Gradients cost the same: for p in a showing k on e, that of C2 is S_b - B_{e,k} and that of C3 is 4 (M_b - M_{b,k}) v_p.

What does not collapse [D]: a sum with a weight per pair (a multiplier matrix not constant on blocks costs its nonzero count times r) and a sum with a non-polynomial function per pair (the max(0, .) of an inequality). A polynomial of degree d in Y_pq over all pairs costs O(P r^d); the one-sided penalty is not one.

## 2. Aggregated equalities (question 3)

**Statement [D].** On the spheres, the feasible set of the full relaxation equals the set of V with
(L1) S_c = v_0 for every cell, U_t = v_0 for every tile;
(L2) A_{e,k} = B_{e,k} for every inner edge and colour;
(N) Y_pq >= 0 for every pair p != q.

Proof. Forward: (L1) is the low-rank memo's simplification (b), (L2) the theory memo's consequence (ii). Backward: by (L1), the sphere and C1, the same-cell entries sum to |v_0|^2 - <v_0, S_c> = 0; they are nonnegative, so each is zero; the same for tiles. Then A_{e,k} and A_{e,l} are sums over disjoint sets of orthogonal rows, so <A_{e,k}, A_{e,l}> = 0 for k != l, and by (L2) <A_{e,k}, B_{e,l}> = 0: the mismatched entries between colours k and l sum to zero and are nonnegative, so each is zero. Only the nonnegativity of the conflict pairs is used.

Counts [D; 5x5 M]: (L1) and (L2) are m = 2 n^2 + G rows, G = edge-colour groups: m = 194 at 5x5 (rank 148 with the right-hand side), 352 at 6x6, 996 at 8x8, 7,952 at 16x16. They are **linear in V**: L V = b e_1^T with L sparse, entries 0, 1, -1, at most 6 nonzeros per column [M].

**The multiplier count is not cut [D, M].** In Y-space the multiplier of (L1) is constant on a cell's block, so a free-sign multiplier lambda_pq of the old zero is recovered as (block constant) minus mu_pq with mu_pq >= 0 the multiplier of (N) on that pair: a general dual needs all of them. Measured on the structured run: 56 to 98 percent of the conflict pairs hold a nonzero mu (median 94), and 50 to 83 percent of the free pairs (median 80). The gain is flops and an index-free kernel, not a smaller dual.

Overlaps. D: the same rows shrink the face of the convex splitting from order 430 to 331 at 5x5 (P + 1 - 148). C: the equality block has the constant Hessian rho L^T L, so it can be preconditioned or projected exactly.

## 3. What stays P^2 (question 2)

**The term.** (N): the sum over pairs of max(0, mu_pq - rho Y_pq)^2. With the equalities structured, the sweep is one Gram product, one clamp and one product: 3 P^2 r + 3 P^2 flops, no pair list [D].

**Lazy active set: no [M].** The share of pairs with a nonzero term, the only pairs a sparse sweep keeps, logged at every evaluation: 48 percent at the start, 67 to 72 through the interior phase, 83 to 85 near feasibility (5x5 seed 0, r = 32). A working set built from one full scan per outer step (pairs with mu > 0 or Y below 1e-3) holds 46 to 99 percent of all pairs; with margin 0 it misses 8 to 57 percent of the pairs that turn active before the next scan. A neighbour-list bound on row displacement would make a lazy scheme exact [D], but it has nothing to save. Reason [D]: Y_pq = x_p x_q + <w_p, w_q>, w the part of the row off e_1, and |w_p|^2 = x_p (1 - x_p) dwarfs x_p^2 when x_p is near N / P: in the interior the sign of a free entry is close to a coin flip. N / P is 30 times smaller at 16x16 [G: worse there].

**Sampling with a guarantee: no [D].** A uniform sample of pairs gives an unbiased gradient of a pure penalty: no bound on the largest violation, no per-pair multipliers. Screening fails too: Cauchy-Schwarz certifies Y_pq >= 0 only when x_p + x_q >= 1.

**Restructured: only by changing the object.** What (N) says [D]: the rows of a cell are an orthogonal splitting of v_0 (section 2), and every row must have nonnegative coordinates in every cell's and every tile's splitting. Three ways out, ranked in section 0:

- *All splittings share one basis*: V >= 0 entrywise, v_0 a nonnegative unit vector. Then (N) is free and the whole program is (L1), (L2) and the sphere: about 35 P r flops. But then, in each cell, rows with disjoint supports sum to v_0, so every coordinate of v_0 is assigned to exactly one placement per cell, and (L1), (L2) make each coordinate a complete valid board: the feasible set is exactly the weighted families of r true solutions [D]. It is the puzzle, r times. Study B owns this; its sweep is O(P r).
- *Drop the free pairs* (theta form). Zeros then need their own term: C3 as a penalty in O(P r^2), or dense cell, tile and edge blocks with multipliers at 6 Kd r flops, Kd = 3.5e8.
- *Nonnegativity between group sums only* (G^2 r / 2 = 1.8e9 flops at 16x16): the lift of the edge-colour model, weaker; study D's.

## 4. Cost and memory per sweep (question 4)

The table for 5x5, 6x6, 8x8 and 16x16 is Appendix D [D, `costs.py`]; its 16x16 row is section 0's. The exact structured sweep is 1.4 times cheaper than today's at 5x5, 6x6 and 8x8, and 1.3 times at 16x16.

Measured at 5x5 (478 placements), median of 200 evaluations, NumPy, one thread, loaded machine [M]: today 4.1, 6.7, 9.0 ms at r = 16, 32, 64; exact structured 1.27, 1.40, 1.72 ms: 3.3, 4.8 and 5.2 times. The flop model says 1.4; the rest is the removed gathers and scatters, which the engineering memo's one-pass kernel already claims (3.0 times).

## 5. The smallest 5x5 experiment

The exact form is settled by arithmetic at 16x16, so the experiment worth running is on rank 2 of the table: `structured.py` with the all-pairs term replaced by the C3 penalty (formulas verified, solver not written). Five 5x5 boards, r = 32, 10 starts, 240 s, against `bm.py` and the exact structured sweep on the same starts; record evaluations to violation 1e-3, mass off solutions, first rounding that solves. Kill line, fixed now: if on 4 of 5 boards its feasible points carry more than 5 percent of the mass off solution placements, free-pair nonnegativity cannot be dropped, and no form of the relaxation survives at scale; only study B's object does. Cost: about 7 core-hours.

## 6. Not checked

- Anything above 5x5; the 16x16 figures are arithmetic on uniform colour counts.
- The moment, block and nonnegative-factor sweeps as solvers: only their identities were verified.
- The paired run and the active shares: one board, one start, one rank.
- Whether a block-constant dual exists at the optimum (it would cut the multipliers for real).
- Seconds per sweep at 16x16 assume 5e11 to 1e12 flops per second on 10 cores [G].
- No literature was consulted.

## Appendix A. Files and runs (`scripts/theory-a/`)

| file | what | output |
|---|---|---|
| `structure.py` | edge-colour groups, the sparse system L V = b e_1^T | imported |
| `identities.py` | C1 to C3 at a random point; conflict list = cell, tile and mismatch blocks (27,203); rank of L; mixtures of solutions satisfy L1, L2 | `raw/identities_seed0.json` |
| `structured.py check` | gradient of the structured augmented Lagrangian against central differences: 1.8e-6 relative (`bm.py`'s own check: 1e-6) | `raw/gradcheck.json` |
| `structured.py time r` | per-evaluation time, both sweeps, same point and multipliers | `raw/time_r16.json`, `time_r32.json`, `time_r64.json` |
| `structured.py run struct 32 0 240` | structured sweep, early stop, working-set log | `raw/run_struct_r32_s0.jsonl` |
| `structured.py run orig 32 0 240` | `bm.py`'s sweep, same loop | `raw/run_orig_r32_s0.jsonl` |
| `structured.py run struct 32 0 100` | repeat with mass off solutions, mismatches and rank logged | `raw/run_struct_r32_s0_100s_with_mass.jsonl` |
| `costs.py` | section 4 | `raw/costs.json` |

Compute: about 12 core-minutes, longest run 240 s of solver time.

## Appendix B. The paired run (5x5 seed 0, 36 solutions, r = 32, start 0) [M]

| outer step | structured: evaluations | violation | squared norm | active share | original: evaluations | violation | squared norm |
|---|---|---|---|---|---|---|---|
| 0 | 304 | 4.9e-2 | 4.24 | 0.48 | 315 | 3.8e-2 | 3.80 |
| 10 | 3,402 | 2.1e-2 | 5.32 | 0.68 | 3,530 | 1.2e-2 | 4.61 |
| 20 | 6,499 | 3.1e-3 | 8.08 | 0.71 | 6,733 | 5.2e-3 | 5.56 |
| 30 | 9,605 | 2.4e-4 | 8.60 | 0.72 | 9,941 | 5.4e-3 | 5.84 |
| 40 | 12,722 | 8.8e-5 | 8.54 | 0.68 | 13,173 | 1.0e-2 | 5.85 |
| 44 | | | | | 14,450 | 1.0e-2 | 8.36, rounds to a solution, stop |
| 80 | 25,145 | 1.2e-5 | 7.62 | 0.83 | | | |
| 168 | 52,461 | 1.5e-6 | 7.61 | 0.85 | | | |

Violations are `bm.max_violation`'s: the original constraint list, explicit zeros included, so the structured run is graded on constraints it never penalises directly. In the 100 s repeat the structured point had rank 15 at step 25, 5 at step 45, mass off solutions 3.6e-3 then 1.6e-4 per cell, and 13 greedy mismatches throughout. Wall times (241 s against 167 s) are not comparable: the structured run carries the working-set bookkeeping and the load moved from 16 to 60. The full solve's squared norm on this board is 5.16 [memo], so neither run is at the minimum.

## Appendix C. Counts (uniform colours, no pin) [D]

| board | K conflict pairs | Kd pairs in cell, tile and adjacent blocks | G edge-colour groups | m rows |
|---|---|---|---|---|
| 5x5 (3,4) | 27,702 (measured with pin: 27,203; Kd 32,760) | 32,934 | 144 | 194 |
| 6x6 (4,5) | 156,243 | 178,800 | 280 | 352 |
| 8x8 (4,9) | 1,904,616 | 2,051,184 | 868 | 996 |
| 16x16 (5,17) | 3.32e8 | 3.45e8 | 7,440 | 7,952 |

Flop models: today 4 P^2 r + 3 P^2 + 4 K r (`bm.py`); equalities 35 P r; exact structured 3 P^2 r + 3 P^2; blocks 6 Kd r; moments 20 P r^2. Memory: today 32 K + 16 x 0.77 F bytes of multipliers (the low-rank memo's share); exact structured 4 P^2 (dense symmetric); blocks 8 Kd; moments 16 G r^2.

## Appendix D. Flops per evaluation and memory, r = 64 [D]

Factor against today in brackets. Memory includes 8 x 24 P r bytes for the factor and ten L-BFGS pairs.

| board | P | today | exact, structured | blocks only | moments | equalities only (B) |
|---|---|---|---|---|---|---|
| 5x5 (3,4) | 484 | 6.8e7, 8 MB | 4.7e7 (1.4), 7 MB | 1.4e7 (4.9) | 4.1e7 (1.7) | 1.1e6 (63) |
| 6x6 (4,5) | 1,296 | 4.8e8, 29 MB | 3.3e8 (1.4), 23 MB | 7.2e7 (6.6) | 1.1e8 (4.4) | 2.9e6 (164) |
| 8x8 (4,9) | 5,776 | 9.1e9, 0.31 GB | 6.5e9 (1.4), 0.21 GB | 8.0e8 (11) | 4.9e8 (19) | 1.3e7 (706) |
| 16x16 (5,17) | 156,816 | 6.4e12, 160 GB | 4.8e12 (1.3), 100 GB | 1.3e11 (49), 4.7 GB | 1.3e10 (489), 2.4 GB | 3.5e8 (18,000), 1.9 GB |
