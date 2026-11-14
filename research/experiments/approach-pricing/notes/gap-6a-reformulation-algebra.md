# Gap 6a: a different question, through algebra and counting

> **Status (2026-11-13):** independently re-checked, see `notes/gap-6a-reformulation-algebra-recheck.md`.
> **Re-check's rulings:**
> - Verdict "empty": stands with a narrower scope. Second analysis: "No reformulation constructed here prices below the tree." It is a list of fifteen priced attempts and two partial barriers, "none found", not a barrier and not a proof that the lens is empty.
> - Lemma B: wrong as stated. Its proof step fails on the real tiles (a three-cell term with permanent 2 that fixes no colour on its first internal edge); replaced by B' and B'' in review section 2.2. Even repaired it proves nothing about cost, because a determinant never reads boards with a repeated tile.
> - Lemma B aside, a signed sum "is excluded as a polynomial-size object": unproven.
> - Lemma A: stands, proof checked. Its reading (any exact linear device is at least 2^n) stands with a narrower scope: true for modules over the exterior algebra, false in general (n = 2 has a device of dimension 2, not 4). General bound: C(n, k) at layer k, 2^246.7 at the widest layer for 251 tiles, not 2^251. The instance bound is the number of live states.
> - Duality, one dimension free and two not: stands as an argument, unproven as a barrier.
> - "Rim merges come almost all from one triple": stands with a narrower scope, true up to 1 x 4 (18 of 21); at 1 x 6 it is 3,680 of 5,572, a second mechanism (exchange of two dominoes) grows with the square of the length.
> - log2(m) about 0.006 bit, "under 0.02": wrong number, conclusion stands. About 0.033 bit by first moment once non-local exchanges are counted (m - 1 about 0.019 at the peak against the memo's 0.004). Quote "under 0.1 bit".
> - The row whose price rests on an unsupported guess, and would be below the tree if the guess failed, is row 15 (linearisation at degree 2 or 3).
> **Read with:** `notes/gap-6a-reformulation-algebra-recheck.md`; `notes/gap-6a-linearisation-degree.md` reports measurements on row 15 (not independently checked).

Draft, 2026-11-13 afternoon, not independently checked. Lens: reformulations in which the object computed is not a board. Scripts and outputs: `notes/gap-6a/` (`prices.py`, `merge_real.py`, `socle_check.py`, each `python3 -I`, about 40 core-seconds in total). Tags: [measured] = enumerated today on the real tiles; [derived] = my argument or arithmetic; [cited] = a source opened today, by me or by the survey named; [guessed] = no support beyond judgment.

## Verdict

**Empty, with one stronger barrier and one corrected number.** No algebraic or counting reformulation I can construct prices below the tree. Every one of them does one of three things: it carries the used set (and then is the tree, to within a merge factor now measured on the real tiles at 1.0000 to 1.0003 on small regions), it sums over something that is not the used set (and that index set has 2^339 or more live terms), or it moves each-tile-once into a matching where it is free and pays the same price on what replaces it. A footnote at best: under 0.02 bit.

## 1. The gap, and what the evidence covers

The claim under test is O1: each tile once defeats tabulation. The survey `surveys/exact-algorithms.md` and its second analysis (`DERIVATIONS-recheck.md`, ruling 1 and 2, section 4) establish it for region tables with a tile-subset sieve and for oblivious rank compression. The second analysis lists what is not covered: sieves not indexed by tile subsets, determinant or extensor forms, hashed used sets, search without tables. The survey's section 4.2 argues the determinant case in one paragraph without a barrier. Counting modulo a prime, self-reduction and the dual views (tiles as vertices, matchings, 2-factors) have no row in ROUTES. The merge factor on the real tiles is "argued to be about 1, not measured" (THESIS_PLAN, RQ4).

This memo covers those holes except search without tables, which is another study's lens.

## 2. Literature

Opened today (abstract pages unless said):
- Arvind, Chatterjee, Datta, Mukhopadhyay, "Fast exact algorithms using Hadamard product of polynomials", https://arxiv.org/abs/1807.04496. The multilinear part of f is the Hadamard product with the elementary symmetric polynomial; exact counting in time C(n, at most k/2). [cited]
- Pratt, "Waring rank, parameterized and exact algorithms", https://arxiv.org/abs/1807.06194. Ties multilinear detection to the Waring rank of the monomial; 4.075^d for approximate counting. [cited] That the rank of x_1...x_n is exactly 2^(n-1) is from memory (Ranestad and Schreyer), unverified.
- Curticapean, Dell, Marx, "Homomorphisms are a good basis for counting small subgraphs", https://arxiv.org/abs/1705.01595. Subgraph counts are linear combinations of homomorphism counts of quotient graphs. [cited] That the cost is governed by the largest treewidth among the quotients is from memory of the full text, unverified.
- Cai, Lu, Xia, "Holographic algorithms with matchgates capture precisely tractable planar #CSP", https://arxiv.org/abs/1008.0683. The dichotomy is for Boolean variables and symmetric constraints. [cited]
- Björklund, "Counting perfect matchings as fast as Ryser", https://arxiv.org/abs/1107.4466. Hafnian over any ring in 2^(n/2); no better is known. [cited]
- BEST theorem, https://en.wikipedia.org/wiki/BEST_theorem: Eulerian circuits of a directed graph are counted in polynomial time by a determinant. [cited]
- Through the survey, not reopened: Koutis and Williams (commutative algebras that detect k-multilinear terms have dimension at least 2^k / k), Björklund's determinant sums, Brand, Dell, Husfeldt (extensor coding).
- Not opened (the publisher refused the request): Nisan, "Lower bounds for non-commutative computation", STOC 1991. From memory: a branching program that reads its inputs in a fixed order and computes the symmetrised product of n variables needs width C(n, k) at layer k. Unverified.

I found no paper that applies any of these to edge matching.

## 3. The reformulations, priced

Baseline 134.4 bits. Contraction of the colour-only board: 2^75.4 tile placements (re-check's figure). So a sum whose terms are contractions must have fewer than 2^59.0 terms. "Colour-only boards per solution" is 2^357.6 in path 1's model and 2^338.8 after its toy correction (`prices.py` [B]). [derived, model]

The fifteen reformulations, each with its price and the step that costs it, are the table in Appendix A. In short: rows 1, 9 and 10 are the tree (134.4 bits less a merge factor worth under 0.02 bit); rows 2, 3, 6 and 7 are exact algebraic devices at 255 to 326 bits; rows 4, 5 and 8 are sums with 2^339 or more live terms; rows 11 to 13 are dual views with larger search spaces (1,957 bits against 1,860) or universes (512 bits); row 14 does not apply; row 15 fits only at degree 4 to 8.

Rows 2, 3, 6, 7 are oblivious prices; on the instance each falls back to row 1. All numbers: `prices.out`.

### Lemma A (extends the re-check's ruling 2 to every linear device) [derived, checked by brute force for n = 4]

Let L be the exterior algebra on n generators and M any L-module with a vector v such that the full product w = e_1 ... e_n does not kill v. Then a -> a.v is injective, so dim M is at least 2^n.

Proof. Let a be nonzero and e_A a monomial of least degree in a. Multiply by e_B with B the complement of A. Every other monomial e_A' of a has A' not contained in A, so it meets B and dies. Hence e_B a = +-c w with c nonzero. If a.v = 0 then w.v = 0, a contradiction.

Reading. Any device that represents tiles by linear operators, kills every repeat exactly and keeps the solution is at least as large as the used-set table, whatever the basis, the field or the vectors chosen (extensor coding with Vandermonde vectors included). On the instance, the vectors reached at one frontier are the images of the reached monomials under an injective map, so they are linearly independent: the dimension of the span is the number of distinct used sets. A device may escape only by not killing repeats (row 3, and then it must read a parity, which is the sign problem) or by not being linear.

### Lemma B (why the determinant does not absorb colours) [derived]

Write the count as a sum of permanents of 0/1 cell-by-tile matrices, each matrix a product of per-cell allowed sets S_c, with no cancellation between terms. A term counts only solutions if any two adjacent cells have all their allowed placements pairwise compatible, so all placements in S_c show one colour on each shared side. Each term therefore fixes the colour of every internal edge, and the terms are indexed by edge colourings. The live ones are those where every cell has a fitting tile: about the number of colour-only boards, 2^339 to 2^358. Scope: sums without cancellation. A signed sum in which non-solutions cancel between terms is not excluded by this lemma. It is excluded as a polynomial-size object (it would count modulo 2 in polynomial time), not at size 2^59; I have no lower bound there.

### Why duality does not help [derived]

In one dimension it does. An edge piece is an arc from its left colour to its right colour, a frame is an Eulerian trail, each piece once is the definition of a trail, and counting is a determinant (BEST) or a table over the 25 arc types. That is why the frame alone has an exact count of 10^37.6 (ROUTES, A5) at no cost. The device works because pieces of the same arc type are interchangeable for the ring. In two dimensions a tile is an arc in the horizontal colour graph and in the vertical one at once, and no two tiles share both. Rows 11 to 13 show the same thing three ways: each tile once becomes free, and the constraint that replaces it (closure around each board vertex, or commutation of the horizontal and vertical matchings) carries tile identities across a width-16 cut. A tile identity is 7.6 bits against 4.1 for a colour, so the frontier of the dual sweep is larger, not smaller.

## 4. Measured today: the merge factor on the real tiles

Rows 1 and 9 save exactly the merge factor m. The survey took m - 1 as 10^-11 per 2 x 2 block (first moment). I enumerated every strict filling of twelve small regions with the real tiles and counted fillings that share (tile set, outline) (`merge_real.py`, exact integer keys, 37 s). [measured]

The counts per region are in Appendix B: m - 1 is 0 on four regions, 6e-7 to 9e-6 on the interior and corner ones, and 1.4e-5 to 3.0e-4 on the rim ones.

Findings.
- The 10^-11 figure is wrong by six orders. The mechanism it missed: turn a block by a half turn. The internal edges stay matched, and the outline matches when it is centrally symmetric, which costs four colour equalities for a 2 x 2 or a 1 x 3 (17^-4 = 1.2e-5; predicted 49 fillings of 4.06 million, measured 60). [measured, mechanism derived]
- The rim merges come almost all from one triple, edge pieces 6, 7 and 51 (zero-based ids in `official.json`), which share inward colour 9 and can be cycled. [measured]
- The conclusion stands. A partial board at the peak has about 115 interior 2 x 2 windows, about 250 strips of three and one rim row. Summing the measured rates gives m - 1 of about 4e-3, that is log2(m) of about 0.006 bit; I quote under 0.02. [guessed extrapolation: assumes merges are local and independent; regions above nine cells not enumerated]
- A side effect worth knowing: a half turn of a 1 x 3 strip is an odd permutation of tiles. In a signed determinant sum over the rationals the two boards cancel, so a signed count can be zero when solutions exist. Random weights are needed, as in the literature. [derived]

## 5. Mechanisms and falsifying experiments

No mechanism is proposed. The plan asks for the experiment that would falsify each claim, so here are the two that would falsify the emptiness.

**F1. Merge factor at depth, real tiles.** Sample strict partial boards at depths 96, 128 and 158 with Track 10's estimator (commit `dc71ad56`), and for each sampled board enumerate exactly the alternative fillings of every window of up to 2 x 4 cells that keep tile set and outline. Measured: mean log2 of the number of boards equivalent to the sample. Baseline: 0.006 bit (section 4). Kill line for "m is about 1": mean above 1 bit at any depth. Cost: under 1 core-hour. My odds that it is killed: under 1%. It would close RQ4 on the real tiles.

**F2. A linear device smaller than the strict table.** On 5 x 5 and 6 x 6 generated boards (toys), give any proposed representation and count the dimension of the span of its states at the widest cut against the number of distinct (frontier, used set). Lemma A predicts equality for exact devices. Kill line: a ratio below 0.5 with every solution retained and no false positive. Cost: seconds. This is THESIS_PLAN's falsifier for O1, restricted to linear devices; I do not expect anyone to pass it.

## 6. Odds

- Some reformulation in this lens yields 10 bits: 0.3%. The mass is on a signed determinant sum with cancellation between terms (the hole in Lemma B) and on things I did not think of.
- 56 bits: under 1 in 100,000.

## 7. Why the gap is empty, in one argument

The count of solutions is the coefficient of x_1 ... x_256 in a polynomial computed by a width-16 planar network. Extracting a full-support multilinear coefficient costs the Waring rank of the monomial (2^255) when done by evaluation, the dimension of the exterior algebra (2^256) when done by any exact linear device (Lemma A), and one term per edge colouring (2^339 or more) when done by determinants without cancellation (Lemma B). The only discount available is the instance's sparsity, which is what the tree already is, and the only compression of the tree an algebraic device can add is the merge factor, measured here at a few parts per million per block. Dual views make the rule free only by moving tile identities onto the frontier.

What this does not exclude: non-linear or lossy devices with a sound error bound, signed sums with cancellation at intermediate size, and anything that uses the instance beyond its sparsity. None of these has a concrete candidate in this memo.

## 8. What I could not check

- Nisan 1991 and the Waring rank of the monomial are quoted from memory. Koutis and Williams was opened by the survey, not by me.
- The treewidth criterion in Curticapean, Dell, Marx: abstract only.
- Row 8's live-term count and row 15's degree claim are guesses.
- Row 3's claim that the trace estimator has the variance of path 1's sign estimator is argued (both sum squared sizes of parity classes), not computed.
- The merge factor is measured on regions of up to nine cells only. Whole-board merges that are not products of local ones are argued negligible, not measured.
- The colour-only board count (2^339 to 2^358) is path 1's model with its toy correction, not recomputed beyond the formula.
- Lemma B has had no second reader.

## Appendix A: the reformulations, priced

| # | Reformulation | Object computed | Price, bits | The step that costs it |
|---|---|---|---|---|
| 1 | Exterior algebra sweep, tiles as basis vectors, sparse storage | For each frontier, an element of the exterior algebra; repeats vanish by themselves | 134.4 minus log2(m), with log2(m) under 0.02 | The support is the set of distinct (frontier, used set). It is breadth-first search of the strict tree. Saving is the merge factor m only |
| 2 | Any exact linear device: a module on which a repeated tile acts as zero and the full product does not | A vector per frontier | 251 + 75.4 = 326.4 oblivious; row 1 on the instance | Lemma A below: such a module contains a copy of the whole algebra, dimension 2^251 |
| 3 | Clifford (Majorana) operators, squares equal to 1 | A spinor of dimension 2^125.5 per frontier | 326.4 exact, or a sign problem of 2^342 samples | A repeated tile acts as the identity, so the state is the parity vector. Reading the answer needs a trace: 2^125.5 start vectors, or random ones with the variance of path 1's sign estimator |
| 4 | Sum of determinants (survey 4.2), no table | N mod 2 = sum over edge colourings of det(A_chi) | 339 to 358 | Lemma B: without cancellation each term must fix every edge colour. Live terms = colourings with a fitting tile in every cell |
| 5 | Checkerboard half sieve (Björklund's bipartite trick) | Sum over white placements of a 128 x 128 determinant for black | as row 4 | White rows are unit vectors, but black rows depend on four white neighbours. Summing a white cell out gives back the sum over its four colours. No label set of size 2^128 appears |
| 6 | Weights in a group, x_t = g^(w_t) (hashed used set, exact) | Coefficient of one group element | at least 243 + 75.4 = 318 | Koutis and Williams: dimension at least 2^k / k with k = 251. With a smaller group the false positives are the 2^339+ colour-only boards |
| 7 | Hadamard product with the symmetric polynomial | Multilinear part of the board polynomial | 255.1 | C(256, at most 128) is the used-set automaton again |
| 8 | Homomorphism basis: Möbius inversion over partitions of the cells | Signed sum of counts of boards with prescribed equal cells | index set 2^1241.9; live terms about 2^339 to 2^358 [guessed: one board per kernel] | Each term glues distant cells, so its treewidth is no longer 16 |
| 9 | Counting modulo 2 or a prime | N mod p | row 1 | The only cancellation is between partial boards with the same (frontier, used set). Measured: 0 to 3 in 10,000 states |
| 10 | Self-reduction from a decision or counting oracle | A solution, from 2^11.3 to 2^17.6 oracle calls | oracle + 0 (later calls are cheaper) | No oracle below the tree is known. Decision is worth nothing by itself here: a solution is planted |
| 11 | Dual: tiles as vertices, each colour class a perfect matching of half-edges | A pairing of 960 half-edges whose glued surface is flat | space 1956.9 against 1860.1 for placements | Each tile once is free. The 225 vertex closures are now the constraint and they are the grid's width-16 structure with tile identities as bond values |
| 12 | Row systems: horizontal edges as an assignment problem | A partition of the tiles into 16 matched rows | 2^931.9 row systems; 2^84.2 candidates for one row | Existence of a row system is polynomial. The 240 vertical edges must remove 928.2 bits and couple all rows |
| 13 | Exact cover by vertex stars | 225 stars covering each tile corner once | 512 by the k-dimensional matching bound, and that omits edge agreement | Universe of 1,024 corners, 2^22.1 stars per vertex |
| 14 | Matchgates, Pfaffian orientation (the board is planar) | A Pfaffian | not defined | The tractable class is Boolean and symmetric. A cell tensor with four 17-valued legs and 784 arbitrary entries is not a matchgate; each tile once is not planar |
| 15 | Linearisation (XL, Nullstellensatz, polynomial calculus) on 156,816 lifted variables | Kernel of a Macaulay matrix | degree 4: 128.9 (dense elimination); degree 8: 122.8 to store; degree 9: 136.9 | Only degree 4 to 8 fits. Degree-D reasoning sees D placements at a time, which is local reasoning (O3) [guessed that the needed degree grows with the board side] |

## Appendix B: merged fillings of small regions, real tiles

| Region | Fillings | Merged pairs | m - 1 |
|---|---|---|---|
| interior 1 x 2 | 35,280 | 0 | 0 |
| interior 1 x 3 | 1,579,688 | 9 | 5.7e-6 |
| interior L of 3 | 1,579,465 | 1 | 6.3e-7 |
| interior 2 x 2 | 4,059,952 | 30 | 7.4e-6 |
| rim 1 x 3 | 6,600 | 1 | 1.5e-4 |
| rim 1 x 4 | 69,827 | 21 | 3.0e-4 |
| rim 2 x 2 | 73,003 | 1 | 1.4e-5 |
| rim 2 x 3 | 2,049,538 | 367 | 1.8e-4 |
| corner 2 x 2, 2 x 3 | 1,312 and 36,925 | 0 | 0 |
| corner 3 x 3 | 2,633,221 | 24 | 9.1e-6 |

## Appendix C: reproduce

```
cd scripts/gap-6a
python3 -I prices.py ../../../../../starter-kit/data/official.json       # under 1 s
python3 -I merge_real.py ../../../../../starter-kit/data/official.json   # about 40 s
python3 -I socle_check.py                                                # under 1 s
```

Outputs as run today: `prices.out`, `merge_real.out`, `socle_check.out`.
