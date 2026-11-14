# Gap 6b: a different search space (what is searched, and in what order information is committed)

> **Status (2026-11-13):** independently re-checked, see `notes/gap-6b-reformulation-search-space-recheck.md`.
> **Re-check's rulings:**
> - Verdict "empty for every reformulation of the refine-and-test kind": stands for what the memo actually covers, and that is less than what it says it covers. The lemmas bound static sequences of fully determined regions plus priced examples. Node-dependent cell orders, and guesses under which the order depends on the guess, are refine-and-test schemes that no lemma bounds (ruling 22, the "five exits" are incomplete). No reformulation found within 90 bits of row scan.
> - C = 1856.1 bits and E[Z] = 2^4.0 with no clue: wrong for the real set. With the real counts dealt without replacement the constraint is 1846.0 bits and E[Z] = 2^14.1; with the exact frame count 2^23.1, against the canonical 2^23.46. The uniform ledger is 19.4 bits low at the full board.
> - "104.4 + 59 - 30, the measured peak of 130 bits at depth 158": wrong. The hand sum is 133.4. The ledger summed exactly with the five clues gives a widest level of 2^134.65 at depth 161 and a total of 2^139.0, against the measured 2^134.4; the real-count ledger gives 2^127.8 and 2^132.4.
> - Section 7, "the census correction is small": wrong. The correction that matters is that the counts are fixed: 6.7 bits at depth 158 and 10.0 bits at the full board, plus about 9 bits of exact frame count.
> - Ledger "validated to 0.03 digits on row scan": wrong attribution. The 0.03 digits belong to the repo's census model; `ledger16.py` is 4.6 bits (1.4 digits) off.
> - L1: stands with a narrower scope, it bounds a scheme only at a moment when a set of cells is fully determined and nothing else is.
> - L2, edge orders reduce to cell orders: stands with a narrower scope, now priced. "Black cell, then the white cell above it" costs 2^169.8 (real counts), 37 bits above row scan; the toy result is a small-board artefact.
> - L2 and row B, matching test "worth about 25 bits", "2^863 to 2^900": unproven, immaterial. Read the price as 2^860 to 2^928.
> - L3 conclusion ("none can be guessed profitably"): unproven, stands with a narrower scope, an induction from four examples. Corrected: a guess of I bits whose unlocked test removes t bits, leaving a tree of 2^R, costs 2^(I - t + R) and beats the baseline 2^B only if t > I - (B - R); for every commitment paired with a matching, census, balance or parity test, t falls short by 93 bits or more.
> - L3 example, orientation first (32 equalities at 3.55 bits = 113, survivors 2^279): wrong number. The equalities are not independent; the test is worth 94.2 bits, survivors 2^288, price 2^339.
> - L4 inequality stands; its use ("no region of the peak's size checks more constraint than a band") is wrong as stated. Corrected: every static cell order has a level at least max over k of min over k-sets S of N(S); annealing puts that bottleneck at or below 2^127.55 against row scan's 2^127.79. No certificate.
> - Row C and K (118.3 and 184.1 bits): numbers move to 116.2 and 180.1 with real counts. Row E (2^71 with one pair merged, "one in 2^67 refines"): wrong numbers, verdict stands; one in 2^49.5 or 2^47.5, fifteen classes 2^113 (2^122 with the exact frame count), two classes 2^463 to 2^471. Row H: 2^371.7 with real counts, about 2^380.7 with the exact frame count. Rows D, G, I, J, L: not rechecked beyond arithmetic.
> **Read with:** `notes/gap-6b-reformulation-search-space-recheck.md`; the "126.6 (floor)" in this memo's table and text is not a floor, see `notes/gap-2-lossy-representations-recheck.md` (ruling 5) and `surveys/DERIVATIONS-recheck.md` (W6).

DRAFT, 2026-11-13, not independently checked. Second of two studies on gap 6; algebra and counting are left to 6a. Grades on each statement: measured (by a script here or a independently assessed track), derived (arithmetic in the first-moment model, script named), cited, guessed. Scripts and outputs: `gap-6b/`.

**Verdict: the gap is empty for every reformulation of the refine-and-test kind, and I propose no mechanism.** Changing what is searched relabels the same count. Four short lemmas and a table of twelve reformulations priced on the 16x16 follow; one comes below 134.4 bits (a join, already in ROUTES, 2.5 to 7.8 bits), most are hundreds of bits above. Section 6 lists exactly where the 2^134 does not reappear; each of those places already belongs to another gap.

## 1. The gap, restated

The working explanation prices one object: the tree whose node is a strict partial board on a row-scan prefix (2^134.4, measured). The gap is the suspicion that this object is an artefact of the description "place tiles on cells", and that another description has a smaller space or a stronger cheap test.

Existing evidence covers: seven static cell orders (model, independently re-checked; D3 measuring), region tables with a used set (O1), two-list joins (126.6 to 131.9 bits), frame first, interior first (about 199 bits), the colour-only contraction (2^75.4 time), Hall matching inside a tile search (Track 4: under 1% of nodes). Each was priced one at a time; no row of ROUTES.md says why other descriptions all land in the same place. This memo supplies that.

## 2. Literature

- Edge-colour ("dual", "diamond") variables next to placement variables are the standard SAT and CP models of this puzzle (Heule 2008; Ansótegui, Béjar, Fernàndez, Mateu 2008). Cited through `surveys/sat-cp-proof-complexity.md`, whose citations were audited; not reopened by me. No board above 8x8 is reported solved with either model. Searching over edge colourings is not new; a count of why it cannot differ is.
- Unverified (not opened in full): Benoist and Bourreau, "Fast Global Filtering for Eternity II" (2008), https://hal-lirmm.ccsd.cnrs.fr/lirmm-00364330 (bot wall); Bollobás and Leader, "Edge-isoperimetric inequalities in the grid" (1991), https://digitalcommons.memphis.edu/facpubs/4558 (paywall). I rely on neither; the isoperimetric case needed is proved in Appendix A.
- Refuting an over-constrained random CSP is the test a reformulation would need (section 6, item 2). Opened today (abstracts): Raghavendra, Rao, Schramm, https://arxiv.org/abs/1605.00058 , spectral and sum-of-squares refutation of random k-XOR in time exp(Õ(n^δ)) at density n^((k/2-1)(1-δ)), "a smooth transition" down to brute force at the satisfiability threshold; Kothari, Mori, O'Donnell, Witmer, https://arxiv.org/abs/1701.04521 , sum of squares of degree about n / (Δ^(2/(t-1)) log Δ) cannot refute a random instance when the predicate supports a t-wise uniform distribution. Both are asymptotic, about ensembles with independent constraints and no permutation constraint. They say that near the threshold known refutation methods cost as much as search; they prove nothing here.
- Without rotation, one-row edge matching is an Eulerian path problem and polynomial (Bosboom et al., through `surveys/exact-algorithms.md`, audited): the one place where each tile once is tractable (row J).
- I found no paper that solves a colour-merged puzzle first and refines (two searches; absence, not proof).

## 3. The ledger and four lemmas

**The ledger [derived, `ledger16.py`; totals agree with the re-check's 1860.1, 2217.7, 357.6].** A full arrangement carries H = 1860.1 bits (4! x 56! x 196! x 4^196). The 480 inner edges carry C = 1856.1 bits of constraint (420 x log2 17 + 60 x log2 5). Any exact scheme commits the H bits in some order and checks the C bits in some order. In the first-moment model the number of live nodes at a stage is 2^(bits committed minus bits checked). Row scan commits log2(4r) bits per interior tile (r tiles left) and checks two edges, 8.17 bits. The surplus log2(4r/289) is positive while r > 72: summed over the first 124 interior tiles it is 104.4 bits; the top frame row and side columns add about 59, the three clues passed remove about 30 (hand sum, plus or minus 3), which is the measured peak of 130 bits at depth 158. A tile costs 9.6 bits to name and an edge pays back 4.09: that is the whole price.

**L1. The count belongs to the region, not to the description [derived; measured on toys].** Let S be a set of cells and N(S) the number of strict partial boards on S. On the official tiles a four-side colour pattern is owned by at most one tile (measured, `real_domains.py`: 784 oriented placements, 784 distinct patterns, 0 shared). So "the colours of all edges touching S", "the tiles on S", "the tiles on S without rotation plus one consistent rotation each", "the pool of S plus one tiling of it" are in bijection, and any scheme that has determined the content of S at some stage has at least N(S) nodes there, minus what its test refutes. Toy check (`toy_reformulations.py`, four generated boards, 4x4 and 5x5, exhaustive): an edge-colouring search in cell-completing order has, at each cell-complete level, exactly the node count of the tile search with the same test.

**L2. The check is convex in the number of known sides, so edge orders reduce to cell orders [derived; real-tile figures measured].** For an interior cell with s sides coloured, the check "some tile fits" is worth, on the official tiles: 0.00 bits at s = 1, 0.10 (adjacent) or 0.07 (opposite) at s = 2, 2.74 at s = 3, 6.73 at s = 4 (`real_domains.out`; the uniform model gives 0, 0.10, 2.76, 6.74). Each coloured side costs 2.04 bits. Checks arrive only when cells are nearly complete, so the cheapest way to spend edge commitments is to complete cells one at a time, which is a tile order. Spreading them is ruinous: with all 240 left-right edges coloured first, 928 bits are committed against about 25 bits of matching test (the expected number of perfect matchings is 2^4.6; no isolated tile alone costs 19.4 bits), leaving about 2^863 to 2^900 nodes.

**L3. Guessing first conserves the cost [derived; an inequality, no model].** Let φ be any function of the solution with I bits (a seam colouring, a pool, the orientations, a pin, a coarse colouring). A scheme that enumerates the values v of φ and searches under each is itself a refine-and-test scheme. If its test under v refutes a node only when the unconditioned test would, or when the node contradicts v, every node of the unconditioned search appears under at least one v, so the sum over v of the costs is at least the unconditioned cost of the same class of orders. A guess of I bits therefore saves at most I bits per value, on average over values, and nothing in total. Exchange rates agree: a pin of 9.6 bits buys 9.6 at best and 4.8 at the median (independently re-checked); the axis of every tile (196 bits) lowers the interior surplus from 104.4 to 12.1 bits (0.47 bit per bit); all orientations (392 bits) remove the peak entirely (0.27 bit per bit); the top and bottom pools (191.9 bits) bring each half to 2^33 (ROUTES). No side information is worth more than one bit per bit, so none can be guessed profitably.

**L4. The row band is the isoperimetric optimum at the peak [proved, Appendix A].** Any set of k cells of the 16x16 with 64 <= k <= 192 leaves at least 16 inner edges open. A band of full rows leaves exactly 16. The entropy of S depends only on how many corner, edge and interior cells it holds, so no region of the peak's size checks more constraint than a band does. This is why the independently re-checked model finds row scan 9.9 digits ahead of the other static orders, and it bounds every scheme covered by L1.

Together: a refine-and-test scheme either determines regions as it goes and pays N(S) on regions no better than bands (L1, L4), or commits information that determines no region, cannot check edges, and pays more (L2). Guessing first does not leave the class (L3).

## 4. Twelve reformulations, priced [derived unless marked]

Space = bits committed before the first strong test. "Reappears" says where the 134.4 comes back.

| # | What is searched | Space, bits | Feasibility test and its cost | Price on the 16x16 | Where the 2^134 reappears |
|---|---|---|---|---|---|
| A | Edge colourings, cell-completing order, test = perfect matching cells to tiles | 1856.1 in all | bipartite matching, 256 x 256, about 2^13 per node at the speed measured for arc consistency | 134.4 minus under 1 bit | Exactly: bijection with the tile tree at every cell-complete level (L1). Matching removes under 1% of nodes on a strong baseline (Track 4, independently assessed), 0.7 to 1.2 bits over plain search on 5x5 toys |
| B | Edge colourings, all left-right edges first | 928 | matching, worth about 25 bits | about 2^863 to 2^900 | Does not need to: one edge closed per tile instead of two (L2) |
| C | Which tiles go in which half, then each half | 191.9 for the split | exact: tile the half, 2^33 once the pool is right | 2^184 pools pass the tiling test for a free 14x7 block | N(S) again: a pool that tiles a block is a filling of it (118.3 bits for a 7x7 block either way) |
| D | Row of each tile first | 706 | colour balance between consecutive rows, under 20 bits per interface | above 2^400 | Set membership commits log2 14 bits per tile and closes no edge |
| E | Coarse colours first, then refine (multigrid) | 420 for two classes | class counts by matching | 2^476 coarse solutions for two classes, 2^646 for six, 2^132 for fifteen, 2^71 with one pair merged | A coarse puzzle is an under-constrained puzzle. Its solutions are plentiful and easy to find, and one in 2^67 (one merged pair) to 2^472 refines |
| F | Orientation of every tile first | 392 | 32 directional census equalities, 3.55 bits each, then a tree with no peak | 2^279 survivors, each cheap | L3: the peak is traded for 279 bits of guess |
| G | One colour at a time (where its 24.7 edges sit) | 132 for the first colour, 209 to seat its tiles | none: any seating is consistent | above 2^340 after one colour | Highest entropy, zero constraint first |
| H | Sample exact solutions of the colour-only relaxation, hope for strict | 361.6 (number of relaxed solutions) | the contraction: 2^75.4 time, 2^62 to 2^66 memory, itself 25 bits over the budget | 2^357.6 samples | A relaxed sample misses 94 of 256 tile types and agrees with a given solution on 0.33 cells in expectation; repair is the whole problem |
| I | Guess a seam of 16 edges (relax a cut), solve both sides | 61.9 | two lists of 2^61.7 per seam and a join | 126.6 (floor) to 131.9 | ROUTES "meet in the middle"; the only row below 134.4, by the join, not by the search space |
| J | Frame through its inward word, or interior first with the frame as an Eulerian test | 124.9 frames | interior search, 10^13.6 nodes per frame (independently re-checked mean) | 170; interior first about 199 (survey) | Frame anchoring is worth more than frame multiplicity costs |
| K | Position-free fragments, bottom-up assembly | 118.3 for 7x7 blocks, 184.1 for 14x7 | join of fragments | above 184 before the last join | Free boundaries on all sides; same count as row C |
| L | Solution as a fixed point (difference map, message passing) | none | iteration | Track 11: 0 of 30 near critical at 7x7 and 8x8 | Independently assessed row of ROUTES; nothing to add from this lens |

Toy confirmation of rows A, B and E: Appendix B.

## 5. Mechanisms, experiments, odds

I propose none. The nearest candidates are footnotes: row A (under 1 bit of nodes for 13 bits of cost per node, independently assessed in Track 4); row I (2.5 to 7.8 bits, in ROUTES, about 2^123 memory); row E with one merged colour pair as a pre-filter (prunes nothing: every node of the fine tree is a node of the coarse one).

Smallest experiments that would falsify the emptiness claim:

- X1, lemma L1 and L2 on larger boards. Generated critical 7x7 and 8x8 (toys), 30 boards each, a C port of `toy_reformulations.py`. Measured: exhaustive node counts of edge-colouring search under ten edge orders (the four here plus six random) and of a coarse-then-refine search, all with the matching test. Baseline: tile row scan with the same test. Kill line for this memo: any formulation with a median node count below one eighth of the baseline at 8x8, and a ratio that falls from 7x7 to 8x8. Cost: about 3 core-hours.
- X2, the matching test on the real pieces. Take the prefixes Track 10's sampler already draws at depths 96 to 160; apply the matching test with partial sides. Baseline: the arc-consistency shares of the log 13:00 (about 5.2 bits of nodes). Kill line for row A: the test removes 10 bits or more of nodes ahead of depth 158 beyond arc consistency. Cost: 2 core-hours.

My odds [guessed]: a reformulation of the refine-and-test kind yields 10 bits, 0.5%; 56 bits, below 1 in 100,000. The 0.5% is the chance that the first-moment ledger, validated to 0.03 digits on row scan only, misprices a description nobody has measured.

## 6. Where the 2^134 does not reappear

The lemmas have five exits. A method must use one.

1. **A join instead of a tree.** Lists can be intersected without enumerating pairs. Worth 2.5 to 7.8 bits, bounded by O1's list sizes. Row I.
2. **A test that sees past the determined region.** At the peak the residual instance (98 cells, their tiles) has expected solution count about 2^-155 (130 bits of surplus plus the 24.5 of the five-clue count), and no cheap test notices. A test that refuted over-constrained residuals early would remove the peak; that is refutation of a random CSP about a fifth past its threshold, where the literature above has no sub-exponential method. This is O3, gaps 1 and 3.
3. **Merging nodes** that L1 keeps apart, exactly (O1 says factor about 1) or with loss (gap 2).
4. **Information from outside**, at no more than one bit per bit (L3): pins remain the best currency.
5. **Schemes that are not refine-and-test**: algebra (6a), fixed points and local dynamics (row L, priced).

A sixth exit would be a description in which L1 fails: a checkable statement about the solution that does not determine a region and is not coarse in the sense of row E. I looked among edge sets, tile sets, colour classes, orientations, colour layers and adjacency pairings and found none: a search that failed, not a proof.

## 7. What I could not check

- L1 to L4 are stated in the first-moment model. It is measured for row scan only. For rows B, D, E, G the margins are hundreds of bits, so the verdict does not depend on precision; for row A it rests on Track 4 and on toys up to 5x5.
- L3 assumes the conditioned test is no stronger than "unconditioned test plus consistency with the guess". A guess that unlocks a qualitatively stronger test is exit 2, not covered.
- Row prices use the no-clue uniform ledger; clue terms were added by hand. The real interior census gives sum p^2 = 0.05890 against 0.05882 (measured), so the census correction is small.
- Benoist and Bourreau and Bollobás and Leader were not read in full.

## Appendix A. Proof of L4's inequality

Let S be a set of cells of the n x n grid with n^2/4 <= |S| <= 3n^2/4. Call a row mixed if it meets both S and its complement; a mixed row contains at least one open left-right edge, a mixed column at least one open up-down edge. If all rows are mixed, or all columns are, there are at least n open edges. Otherwise some row and some column are each full or empty; a full row and an empty column cannot coexist, so both are full or both are empty. If both are empty, S lies in the x mixed-or-full rows and y columns that are not empty, every such row and column is mixed or full, and since an empty row and column exist none is full, so all x + y are mixed: at least x + y >= 2 sqrt(xy) >= 2 sqrt(|S|) >= n open edges. If both are full, apply the same to the complement. For n = 16 this is 16 open edges for 64 <= k <= 192.

## Appendix B. Toy runs (generated boards, not Eternity II)

Exhaustive node counts (all solutions), log2 of the widest level. T = tile row scan; TM = T plus the matching test M; E/R, E/I, E/H, E/C = edge-colouring search with M under four edge orders (cell-completing row scan; row interface first; all left-right edges first; edges around the black cells of a checkerboard first). COARSE = interior colours merged into two classes, non-ring edges only: coarse colourings passing M (20,000 sampled) against the number that can be refined. Measured, `toy_reformulations.out`, about 12 core-minutes.

| Board (toy) | Solutions | T | TM | E/R | E/I | E/H | E/C | COARSE survivors | refinable at most |
|---|---|---|---|---|---|---|---|---|---|
| 5x5, 3 frame and 4 interior colours, seed 1 | 24 | 14.2 | 13.4 | 13.4 | 16.9 | above 17.9 (capped) | 14.5 | 2^15.3 | 2^4.6 |
| 5x5, 3 and 5, seed 2 | 8 | 12.2 | 11.5 | 11.5 | 16.1 | above 18.1 (capped) | 12.3 | 2^13.7 | 2^3.0 |
| 5x5, 3 and 5, seed 3 | 16 | 12.3 | 11.1 | 11.1 | 14.9 | above 18.3 (capped) | 12.2 | 2^14.6 | 2^4.0 |

Readings, all on toys:
- E/R equals TM at every cell-complete level on the three boards (and on one 4x4 board): L1's bijection, checked level by level. In total nodes E/R is 1.8 times TM, because it has two levels per cell.
- The matching test is worth 0.7 to 1.2 bits over plain tile search at this size.
- No edge order is below TM. Orders that delay cell completion cost 3.4 to 5.4 bits (E/I) and more than 6.4 to 7.2 bits (E/H, capped at 1.5 million nodes).
- E/C is within 1.1 bits of TM on 5x5. This does not contradict L2: on a 5x5 board 16 of 25 cells are frame cells with a known rim side, so a checkerboard order completes cells almost as fast as a scan. The board is too small to separate these two; experiment X1 exists for that.
- A two-class coarse puzzle has 2^10 to 2^11 times more surviving colourings than refinable ones even at 5x5; the 16x16 figure of row E is the model's.
- Three boards are an illustration, not a sample.

## Appendix C. Scripts

- `ledger16.py`, `ledger16.out`: every figure of sections 3 and 4 (uniform first-moment model), under 1 s.
- `real_domains.py`, `real_domains.out`: domain sizes by number of known sides on the official tiles; pattern uniqueness.
- `toy_reformulations.py`, `toy_reformulations.out`: exhaustive node counts under six search spaces on generated 4x4 and 5x5 boards.
