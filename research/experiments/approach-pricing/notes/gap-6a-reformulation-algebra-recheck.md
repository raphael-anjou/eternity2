# Second analysis report: gap 6a, reformulation through algebra and counting

2026-11-13, evening. Memo under review: `notes/gap-6a-reformulation-algebra.md`, read in full with its three scripts and outputs. One core, stdlib only, about 3 core-minutes. My scripts and their outputs are in `notes/gap-6a-review/` (list at the end). Tags as in the memo: [measured] on the real tiles, [derived] by me, [cited] opened today.

## Summary

The verdict "no reformulation in this lens prices below the tree" survives, but as "none found", not as a barrier. Of the two lemmas, A is correct and narrower than its reading, and B is wrong as stated and proves nothing about cost even once repaired. Every count and price in the memo's scripts reproduces. Three things change:

1. **Lemma B is replaced.** Its proof step fails on the real tiles (a three-cell term with permanent 2 that fixes no colour on its first internal edge). The statement that is true, and that covers signed sums, is B' in section 2. The hole is not cancellation. It is that a determinant never reads boards with a repeated tile, so the terms may be wrong there, and on that hole no rank argument can ever say anything.
2. **Lemma A's reading claims more than the lemma.** A linear device need not be a module over the exterior algebra. The bound that holds for every exact linear device is C(n, k) at layer k (2^246.7 at the widest layer for 251 tiles, not 2^251), and on the instance there is no lower bound at all beyond the number of live states.
3. **The merge factor is about five times the memo's estimate.** The memo counts only local refills. Two tile-disjoint dominoes with the same outline can be exchanged between distant places of a partial board: same used set, same frontier, seen by no window. On the real tiles that channel alone gives m - 1 of about 0.019 at the peak, against the memo's 0.004. Total about 0.033 bit, above the "under 0.02" the memo quotes. Still a footnote.

Nothing I looked at prices below 134.4 bits. The one row whose price rests on an unsupported guess, and would be below the tree if the guess failed, is row 15 (linearisation at degree 2 or 3).

## 1. Rulings

| # | Claim | Ruling | Corrected statement or number |
|---|---|---|---|
| 1 | Verdict: empty | **Stands with a narrower scope** | "No reformulation constructed here prices below the tree." It is a list of fifteen priced attempts and two partial barriers, not a proof that the lens is empty |
| 2 | Lemma A, statement and proof | **Stands** | Proof checked line by line; exhaustive over GF(3) for n = 3 and 20,000 random integer elements for n = 6, no failure (`r_lemmaA.py`) |
| 3 | Lemma A, reading: any exact linear device is at least 2^n | **Stands with a narrower scope** | True for modules over the exterior algebra (or any algebra with square-zero generators that commute up to nonzero scalars). False in general: n = 2 has a device of dimension 2, not 4. General bound: C(n, k) at layer k. See 2.1 |
| 4 | Lemma A, reading: "on the instance the dimension of the span is the number of distinct used sets" | **Stands with a narrower scope** | Holds for a faithful exterior module. For linear devices in general the instance bound is the number of live states (the earlier re-check's ruling 2). The memo's "may escape only by not killing repeats or by not being linear" omits a third escape: operators under which products of distinct tiles that occur in no solution also vanish |
| 5 | Lemma B | **Wrong** as stated | Counterexample on the real tiles; replaced by B' and B'' in 2.2 |
| 6 | Lemma B, aside: a signed sum "is excluded as a polynomial-size object" | **Unproven** | It would follow from parity-P hardness of counting edge-matching solutions, which the memo does not cite, and that is a worst-case statement about the family, not about this instance |
| 7 | Duality: one dimension is free, two are not | **Stands** as an argument, **unproven** as a barrier | BEST checked [cited]. No duplicate tiles [measured]. "The frontier of the dual sweep is larger" is one sweep's accounting, not a bound on dual methods |
| 8 | Appendix B, all twelve regions | **Stands** | Every count reproduced exactly by an independent enumerator (`r_merge.py`) |
| 9 | "10^-11 is wrong by six orders"; half-turn mechanism | **Stands** | 17^-4 x 4,059,952 = 48.6 predicted, 60 measured; cycle types confirm half turns (16 classes of one swap, 14 of two swaps in the 2 x 2) |
| 10 | "Rim merges come almost all from one triple" | **Stands with a narrower scope** | True up to 1 x 4 (18 of 21). At 1 x 6 it is 3,680 of 5,572: a second mechanism, exchange of two dominoes, grows with the square of the length (3, 90, 1,853 classes at lengths 4, 5, 6) |
| 11 | log2(m) about 0.006 bit, "under 0.02" | **Wrong** (number), conclusion stands | About 0.033 bit by first moment once non-local exchanges are counted. Quote "under 0.1 bit". See section 3 |
| 12 | Signed count can vanish when solutions exist | **Stands** | The 1 x 3 half turn is one transposition (9 classes, cycle type (2)) |
| 13 | F1 | **Stands with a correction** | As designed it enumerates windows only and would miss the dominant channel. Add: for each sampled board, hash the outline of every domino and tromino and count tile-disjoint twins |
| 14 | F2 | **Wrong** as a falsifier | A projection onto live states passes its kill line trivially. Add: "built without knowledge of the solutions, in time below the strict table" |
| 15 | Section 7, "in one argument" | **Unproven** | Each clause is a price of one method. "The only discount available is the instance's sparsity" is O1 restated, not derived |
| 16 | Odds (0.3% for 10 bits) | not ruled | A judgment. I have no number that moves it |

## 2. The two lemmas

### 2.1 Lemma A

The proof is right. A least-degree monomial e_A exists; every other monomial of a has a set not contained in A, so it meets the complement B and dies; e_B a is a nonzero multiple of w. This is the standard fact that the top form spans the socle. It also holds verbatim when the generators commute, or commute up to nonzero scalars.

What it does not cover: a device only has to give zero on words with a repeated tile and nonzero on full words. Nothing forces its operators to anticommute. Smallest case [derived, `r_lemmaA.py` part b]: two tiles, a space with basis p, q, X1 sends q to p and p to 0, X2 sends p to q and q to 0, start vector p + q, readout the coordinate sum. Both words with a repeat give 0, both orders give 1, dimension 2. Here X1 X2 + X2 X1 is the identity.

The bound that covers every exact linear device, in any field, with any operators, is Nisan's rank argument: the matrix (ordered prefix of length k, ordered suffix) with nonzero entries exactly where no letter repeats has rank at least C(n, k) (checked for n up to 6, 0/1 and generic nonzero values, part c). The used-set table itself is a linear device that reaches it. So:

- Corrected reading: "an exact oblivious linear device has dimension at least C(n, k) at layer k, which is the used-set table of that layer; the sum over layers is 2^n." Row 2 becomes 246.7 + 75.4 = 322.1 at the widest layer rather than 326.4. No consequence.
- On the instance the same argument gives only the rank of (reached states) x (completions), which is the number of live states. A linear device far smaller than the strict table is therefore not excluded; what is excluded is building one without deciding liveness. This is the earlier re-check's ruling 2, and the memo's "falls back to row 1" is an upper bound on cost, not a lower one.

### 2.2 Lemma B

The proof says: a term counts only solutions if all allowed placements of adjacent cells are pairwise compatible. That is sufficient, not necessary. Only pairs that occur together in some perfect matching must be compatible, and the same tile allowed in two cells never does.

Counterexample [measured, `r_twins.py` part 2]: bottom rim, three cells, allowed tiles {6, 7}, {7, 51}, {51, 6}. The term has exactly two perfect matchings, (6, 7, 51) and (7, 51, 6). Both are valid fillings with the same outline. The first internal edge is colour 1 in one and 5 in the other. The pair (6 in cell 1, 51 in cell 2) is allowed and incompatible, and lies in no matching. In general the union of any two solutions whose difference is one cycle of tiles is a valid term, so terms are not indexed by edge colourings.

More important, even a repaired Lemma B bounds nothing. A sum of permanents without cancellation that counts only solutions needs at most N terms (one per solution). The figure 2^339 is not the number of terms a positive sum must have. It is the number of terms with no empty row in one particular sum, the sum over edge colourings. And that particular sum, enumerated edge by edge with a stop when a tile repeats, is the strict tree. So the price of row 4 is "the tree, or 2^339 if enumerated blindly", like rows 2, 3, 6 and 7.

Two statements that are true and that I can prove:

**B' (colour-exact sums, cancellation allowed) [derived, toy-checked].** Let G be the colour tensor: G(one placement per cell) = 1 if all 480 internal edges match, tiles free to repeat. If G is a sum of m terms, each a product of one vector per cell, over any field and with any signs, then m is at least L, the number of edge colourings for which every cell has a fitting placement. Proof: flatten white cells against black cells of the checkerboard. Every edge joins a white and a black cell, so a white placement fixes the whole colouring; the matrix is block diagonal with all-ones blocks, one per such colouring, rank L; each term has rank 1. On four generated 2 x 2 toys the rank equals L exactly (35, 56, 56, 47; `r_lemmaB.py`). L is the memo's "live terms", 2^339 to 2^358 in path 1's model.

**B'' (edge-local sums) [derived, measured].** If the index set is a product of one index per edge and each cell's vector reads only its own four indices, each interior edge needs 17 values even when same-tile pairs are left unspecified: rows from one half of the interior tiles against columns from the other half already show all 17 colours on both sides.

**What stays open, stated exactly.** A determinant only reads injective placements. So the terms need to reproduce G on boards without repeats only, and may be anything on the rest. Restricted that way, every flattening is block diagonal in the used set with all other blocks free, and its smallest completion has the rank of its largest block, which is at most the number of solutions (toys: 4 to 10 against 35 to 56). Hence no rank or communication argument of the Lemma A kind can close this hole, with or without cancellation: the object "few terms, correct on strict boards" always exists (one term per solution) and the only question is whether it can be built without the solutions. That is a question about uniform construction, and neither the memo nor I have a model for it or a candidate.

## 3. The merge factor

All of Appendix B reproduces. Two additions [measured].

- Rim strips of 5 and 6 cells: 725,643 fillings with 366 merged pairs (m - 1 = 5.0e-4) and 7,404,390 with 5,572 (7.5e-4). The triple (6, 7, 51) accounts for 275 and 3,680. The rest is mostly two dominoes changing places: 90 and 1,853 classes, 4.1e-5 to 4.3e-5 per pair of domino slots.
- Twin census (`r_twins.py`): tile-disjoint pairs of fillings with equal outline. Rim dominoes: 19 pairs among 613 fillings, 5.1e-5 per pair of slots, in agreement with the strips. Interior dominoes: 14 pairs among 35,280, 2.3e-8 per pair of slots (two relative orientations). Edge piece plus the interior tile above it: 15 among 2,548, 2.3e-6.

At depth 158 a row-scan board has about 29 rim domino slots, 227 interior ones and 30 mixed ones. First moment, slots independent and uniform over fillings: 0.0176 + 0.0006 + 0.0009 = 0.019, plus the memo's local 0.004, so m - 1 is about 0.023 and log2(m) about 0.033 bit [derived from measured rates; the extrapolation is mine and is a model]. The memo's assumption "merges are local" is the error: the dominant term is quadratic in the number of rim slots. The conclusion (m is 1 for every purpose) is unchanged, and RQ4 should quote 0.03 bit as an estimate and "under 0.1 bit" as the claim.

## 4. The fifteen reformulations

"Price" means the number in Appendix A reproduces and means what the row says. `r_numbers.py` recomputes all of them with exact integers.

| # | Price right? | Bottleneck right? | Note |
|---|---|---|---|
| 1 | yes, with log2(m) about 0.03, not under 0.02 | yes | The sweep can also lose states to sign cancellation (ruling 12) |
| 2 | 326.4 reproduces; 322.1 for a layered device | yes, by Nisan's rank, not by Lemma A alone | Section 2.1 |
| 3 | 326.4 yes. "2^342 samples" is path 1's figure at 14,702 solutions (I get 2^343.8; 2^357.6 with about one solution) | yes | Random start vectors are still 2^125.5 long, so the alternative is path 1's scalar sign estimator, not a spinor |
| 4 | yes as the blind sum; the pruned sum is row 1 | Lemma B no; B' yes | Section 2.2 |
| 5 | **no** | partly | "As row 4" is not derived. White strict and black colour-only on the tiles white left gives 2^174.4 live terms per solution by first moment (2^302.4 if black may use any tile). Still far above 2^59 |
| 6 | 318.4 reproduces as arithmetic | yes | Koutis and Williams prove Omega(2^k / sqrt k) in their Theorem 4.2 and write 2^k / k in the introduction; both are asymptotic with no constant, so "at least 243 bits" should read "of order 243 to 247 bits" |
| 7 | 255.1 yes | yes | The abstract's general bound is n^(k/2 + c log k); the binomial is for the named counting corollaries. With n = k = 256 the binomial is the kinder figure |
| 8 | index set 2^1241.9 yes; live terms **no** | yes | "One board per kernel" undercounts: a term is live for every partition that refines the kernel of some colour-only board, about 2^105 partitions per typical board. The guess is a lower bound, which is the safe direction |
| 9 | yes | yes, for sweeps keyed by (frontier, used set) | 0 to 3 in 10,000 holds to 1 x 4; 7.5 in 10,000 at rim 1 x 6 |
| 10 | yes | yes | |
| 11 | 1956.9 and 1860.1 reproduce | plausible, unproven | These are sizes of search spaces, not prices. The row shows no gain; it does not show a loss |
| 12 | 931.9, 84.2, 928.2 reproduce | yes | "Existence of a row system is polynomial" is unproven (trails of exact length with a rotation choice) and immaterial, since a solution is planted |
| 13 | 512 loosely | yes | Björklund's bound needs four fixed classes; rotation mixes the corner classes, so the applicable bound is no better than 512. 225 stars cover 900 of the 1,024 corners; 124 lie on the rim |
| 14 | not applicable, correctly | yes | Abstract confirms Boolean inputs and symmetric real-valued constraints |
| 15 | monomial counts reproduce; "only degree 4 to 8 fits" is **wrong** | **unproven** | With elimination at exponent 2, degrees 1 to 4 are under the tree (34.5, 67.0, 98.4, 128.9 bits) and degree 5 is at 158.8. At exponent 2.37 degree 4 is at 152.7. "122.8 to store" at degree 8 is not a cost of solving anything. The row's verdict rests entirely on the guess that the needed degree grows with the side |

## 5. Citations

All opened today unless said.

| Source | Memo's claim | Ruling |
|---|---|---|
| Arvind et al., arXiv:1807.04496 | Hadamard product with the symmetric polynomial; time C(n, at most k/2) | Confirmed with a narrower scope: binomial bound for the counting corollaries, n^(k/2 + c log k) for general circuits |
| Pratt, arXiv:1807.06194 | Waring rank; 4.075^d approximate counting | Confirmed |
| Ranestad and Schreyer, arXiv:1104.3648 | rank of x_1 ... x_n is 2^(n-1) (from memory) | Confirmed from the abstract: for n + 1 variables with equal exponents the rank is the product of (d_i + 1) over all but one, here 2^n |
| Curticapean, Dell, Marx, arXiv:1705.01595 | cost governed by the largest treewidth among quotients (from memory) | Confirmed in the full text: Theorem 1.1, time k^O(k) n^(t+1) with t the largest treewidth in the spasm |
| Cai, Lu, Xia, arXiv:1008.0683 | Boolean, symmetric | Confirmed |
| Björklund, arXiv:1107.4466 | hafnian over any ring in 2^(n/2) | Confirmed. "No better is known" is the memo's addition |
| BEST theorem | polynomial count of Eulerian circuits | Confirmed |
| Koutis and Williams (full text) | dimension at least 2^k / k | Confirmed as the introduction's wording; the theorem has sqrt k; a footnote extends it to the non-commutative setting. It is a bound for black-box evaluation over all circuits |
| Nisan, STOC 1991 (copy at csa.iisc.ac.in) | width C(n, k) at layer k for the symmetrised product (from memory) | Confirmed with rewording. Theorem 1: the size of a non-commutative algebraic branching program is the sum over k of rank(M_k(f)), and level k has at least rank(M_k(f)) vertices. Lemma 2: that rank is C(n, k) for the permanent and the determinant. The model is non-commutative programs, not "inputs read in a fixed order", and the symmetrised product of n variables is a corollary (rank C(n, k), my part c), not a statement in the paper |

## 6. Numbers recomputed

Reproduced exactly: 59.0; 357.6 (3.4 + 76.6 + 277.6); 338.8; 326.4; 318.4; C(256, 128) = 2^251.7; sum of C(256, i) for i up to 128 = 2^255.07; Bell(256) = 2^1241.9; 1860.1; 1956.9; 1856.1; half-edge census (5 x 24, 5 x 48, 12 x 50, total 960); 4.086, 4.107, 2.322; 84.2; 931.9; 928.2; 22.1; 156,816 variables and all nine monomial counts; 11.3 and 17.6; every row of Appendix B.

Mislabelled, not wrong: 338.8 is path 1's rounded floor 10^102. The toy correction the memo names (10^1.1 to 10^1.9) gives 2^351.3 to 2^354.0. The memo also mixes 251 and 256 tiles between rows 2, 6 and 7 and section 7.

## 7. Left open, worth a study

1. **Row 15 at low degree, on toys.** The only place in the memo where a price below the tree (67.0 or 98.4 bits at exponent 2) is dismissed by a guess. On generated 4 x 4 and 5 x 5 boards with one solution (144 and 484 variables), find the smallest degree at which the linearised system pins the solution, over GF(2) and over a large prime. Kill line for the guess: degree 2 suffices at both sizes. My odds: under 1%; O3's measurement that propagation sees nothing past the corner windows points the same way. Cost: minutes at 4 x 4, about an hour at 5 x 5 with sparse elimination.
2. **F1 with non-local exchanges.** Section 3's 0.033 bit is a first-moment figure. Measure it on sampled partial boards at depths 96, 128, 158 by hashing domino and tromino outlines. This closes RQ4 properly. Under one core-hour.
3. **The injective-only hole of B'.** Not a compute task. A half-day Desk study to state a uniform model (which index sets and which per-cell rules count as "built without the solutions") under which a bound can be proved or a candidate found. Without that model the hole cannot be closed by any argument in this lens, as section 2.2 shows.

## 8. What I did not do

- I did not recheck 134.4 or 75.4 (independently re-checked in `DERIVATIONS-recheck.md`), nor path 1's toy correction.
- The 0.033 bit is a model extrapolation from measured twin rates; slot counts at depth 158 are approximate.
- Row 5's 2^174.4 and row 8's 2^105 are first-moment figures of mine, not enumerated.
- The parity-P hardness of edge-matching counting was not looked up.
- Rows 11 to 13: I checked the arithmetic and looked for a cheaper sweep; I did not prove that none exists.

## Scripts (`notes/gap-6a-review/`, all `python3 -I`)

- `r_numbers.py <official.json>`, `r_numbers.out`: every price in section 3 and Appendix A, exact integers. Under 1 s.
- `r_merge.py <official.json>`, `r_merge.out`: independent enumeration of the twelve regions plus rim 1 x 5 and 1 x 6, with cycle types. 100 s.
- `r_twins.py <official.json>`, `r_twins.out`: twin census for non-local exchanges, and the Lemma B counterexample. 5 s.
- `r_lemmaA.py`, `r_lemmaA.out`: Lemma A with signs, the dimension-2 device, the rank at each layer. 5 s.
- `r_lemmaB.py <official.json>`, `r_lemmaB.out`: B' on generated 2 x 2 toys, B'' on the real tiles. 15 s.
