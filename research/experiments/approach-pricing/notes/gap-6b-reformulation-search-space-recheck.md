# Review of gap 6b: a different search space

Second analysis report, 2026-11-13. Memo reviewed: `gap-6b-reformulation-search-space.md` and `gap-6b/`. My scripts and their outputs: `gap-6b-review/` (stdlib Python, 1 core, about 25 core-minutes in all, longest run 9 minutes). The memo was not edited.

## Verdict

**The "empty" verdict stands for what the memo actually covers, and that is less than what it says it covers.** I found no reformulation that comes within 90 bits of row scan, let alone 10 bits under it, and I built the strongest "guess that unlocks a test" I could. But:

- The verdict is worded for "every reformulation of the refine-and-test kind". The lemmas bound static sequences of fully determined regions, plus a list of priced examples. Searches whose cell order depends on the node, and guesses under which the order depends on the guess, are refine-and-test schemes that no lemma here bounds.
- L3's conclusion ("none can be guessed profitably") is an induction from four examples, not a consequence of its inequality. It survives my attempt to break it; it is not proved.
- L4's inequality is right; the sentence that uses it is wrong as written. A band is not the region with the fewest partial boards at most sizes, including sizes next to the peak.
- The ledger (`ledger16.py`) is not the model that was validated to 0.03 digits. Computed exactly, it puts row scan at 2^139.0, 4.6 bits above the measurement, and it is 19.4 bits off the canonical solution count at the full board. Three table prices move by 10 to 20 bits on the real colour counts. None of these moves changes a verdict, because every margin that matters is 90 bits or more.

## Rulings

| # | Claim in the memo | Ruling | Corrected statement |
|---|---|---|---|
| 1 | A four-side colour pattern is owned by at most one tile (784 placements, 784 patterns) | **stands**, extended | True for all 256 tiles, not only the 196 interior ones: 1,024 oriented patterns, all distinct, no tile with a rotational symmetry (`r1_tiles.out`). Six tiles are the mirror image of another tile, which does not matter since reflection is not allowed. |
| 2 | Ledger totals 1860.1, 2217.7, 357.6 | **stands** | Reproduced. |
| 3 | C = 1856.1 bits of constraint, so E[Z] = 2^4.0 with no clue | **wrong for the real set** | With the real counts dealt without replacement (12 colours on 50 half-edges, 5 on 48, 5 frame colours on 24) the constraint is 1846.0 bits and E[Z] = 2^14.1. Replacing the frame term by the exact frame count on file (2^124.93 against 2^115.9 in this ensemble and 2^113.9 uniform) gives 2^23.1, against the canonical 2^23.46. The uniform ledger is 19.4 bits low at the full board. |
| 4 | "104.4 + 59 - 30, which is the measured peak of 130 bits at depth 158" | **wrong** | The hand sum is 133.4, not 130. The same ledger summed exactly with the five clues gives a widest level of 2^134.65 at depth 161 and a total of 2^139.0, against the measured total 2^134.4. The real-count ledger gives 2^127.8 and 2^132.4. It sits 2.0 to 2.3 bits under the figures on file with five, one and no clue alike, where the uniform ledger sits 4.4 to 4.6 over (the frame's left and right balance is in neither). |
| 5 | Section 7: "the census correction is small" (sum p^2 = 0.05890 against 0.05882) | **wrong** | Sum p^2 measures unequal colour frequencies only. The correction that matters is that the counts are fixed: 6.7 bits at depth 158 and 10.0 bits at the full board, plus about 9 bits of exact frame count. |
| 6 | Section 5 odds: the ledger is "validated to 0.03 digits on row scan" | **wrong attribution** | The 0.03 digits belong to the repo's census model. `ledger16.py` is 4.6 bits (1.4 digits) off on row scan. |
| 7 | L1: region count is the same under every description | **stands with a narrower scope** | A counting identity on these tiles, by ruling 1. It bounds a scheme only at a moment when a set of cells is fully determined and nothing else is. It says nothing about schemes that hold partial information on many cells; those are covered by priced examples only (rows B, D, E, F, G). |
| 8 | L2: per-cell check figures (0, 0.10, 0.07, 2.74, 6.73 bits) | **stands** | Ownership counts reproduced (735 three-side patterns, 269 and 276 two-side patterns). |
| 9 | L2: "edge orders reduce to cell orders"; toy E/C within 1.1 bits | **stands with a narrower scope**, now priced | The memo left the checkerboard order to experiment X1. In the first-moment model on the 16x16, "black cell, then the white cell above it" costs 2^169.8 (real counts) or 2^175.7 (uniform), 37 bits above row scan (`r4c_peak_orders.out`). The toy result is a small-board artefact, as the memo suspected. No proof covers every mixed order. |
| 10 | L2 and row B: matching test "worth about 25 bits", "2^863 to 2^900" | **unproven, immaterial** | The 25 bits have no derivation: an expected 2^4.6 perfect matchings bounds nothing, and the isolated-tile figure is a bound on something else. What is derivable: at most 2^863 interior colourings survive out of 2^858 committed on the 210 interior left-right edges, so the test may be worth almost nothing. Read the price as 2^860 to 2^928. |
| 11 | L3, the inequality | **stands** | Correct as stated, under its own hypothesis. |
| 12 | L3, the conclusion: "no side information is worth more than one bit per bit, so none can be guessed profitably" | **unproven; stands with a narrower scope** | See section 1 below. Corrected: a guess of I bits whose unlocked test removes t bits, leaving a tree of 2^R, costs 2^(I - t + R); it beats the baseline 2^B only if t > I - (B - R). For every commitment I could pair with a matching, census, balance or parity test, t falls short by 93 bits or more. |
| 13 | L3 example: orientation first, 32 equalities at 3.55 bits = 113, survivors 2^279, "each cheap" | **wrong number** | The 32 equalities are not independent: (N - S) + (E - W) has a fixed parity for each colour. On the real tiles the test is worth 94.2 bits, survivors 2^288, and each survivor still carries a 2^51.5 tree (top frame row). Price 2^339. |
| 14 | L4, the inequality (16 open edges for 64 to 192 cells) | **stands** | Proof in Appendix A checked line by line and correct. Exhaustive on 4x4 and 5x5. Agrees with the abstract of Bollobás and Leader. Covers every set, connected or not. |
| 15 | L4, its use: "the entropy of S depends only on its cell types, so no region of the peak's size checks more constraint than a band" | **wrong as stated** | See section 3. Corrected: every static cell order has a level at least max over k of min over k-sets S of N(S). Annealing puts that bottleneck at or below 2^127.55 (k = 152) against row scan's 2^127.79: row scan is within a fraction of a bit of any static order, as far as a search can tell. No certificate. |
| 16 | Row A (edge colourings plus matching): 134.4 minus under 1 bit | **stands** | Small sample on the real pieces (section 1): the matching test refuted 0 of 16,833 partial boards that forward checking had not already refuted, depths 96 to 160. |
| 17 | Row C and K: 118.3 and 184.1 bits | **stands, numbers move** | 116.2 and 180.1 with real counts. The memo's argument holds: a pool has 2^-11.8 expected tilings of a free 14x7 block, so about 2^180 pools pass even an exact test, which is therefore worth 11.8 bits of the 191.9 committed. |
| 18 | Row E: 2^71 with one pair merged, "one in 2^67 refines", 2^132 for fifteen classes, 2^476 for two | **wrong numbers, verdict stands** | One merged pair: one coarse solution in 2^49.5 (two 50-colours) or 2^47.5 (two 48-colours) refines, not 2^67: the uniform model forgets that the two colours keep their own counts. Fifteen classes: 2^113 (2^122 with the exact frame count). Two classes: 2^463 to 2^471. |
| 19 | Row F | see ruling 13 | |
| 20 | Row H: 2^361.6 relaxed solutions | **stands, number moves** | 2^371.7 with real counts, about 2^380.7 with the exact frame count. The ratio 2^-357.6 is unchanged. |
| 21 | Rows D, G, I, J, L | **not rechecked** beyond arithmetic | D: 706 = log2(196! / 14!^14), correct. J: 124.9 + 45.2 = 170, correct, and it already uses the exact frame count. I and L are ROUTES rows. |
| 22 | Section 6, "five exits" | **incomplete** | Add: node-dependent (dynamic) cell orders, and guesses under which the order depends on the guessed value. Both are refine-and-test and neither is bounded by L1 to L4. |
| 23 | Literature attributions | **stands**, one source still unread | Section 4. |

## 1. The guessing lemma

**What L3 proves.** If the test under a guess v refutes nothing the unconditioned test would not, the trees under all v together contain the unconditioned tree. True, and nearly a tautology. Two things do not follow from it.

First, the bound it gives is "the unconditioned cost of the same class of orders", and the only orders the memo can bound are static region sequences. If the search under v may choose its regions as a function of v, the sum over v of N(S_v given v) is not bounded below by any N(S). In the uniform model a generic guess is equally informative about every region, so no gain is expected; that is an expectation, not a proof.

Second, the hypothesis fails for any guess that is not a region, because such a guess makes global identities testable. I looked for the best one.

**Accounting.** Guess of I bits, test worth t bits on the guesses, tree 2^R under one survivor, baseline 2^B. Price 2^(I - t + R). Profitable only if t > I - (B - R): the test must repay everything the guess wastes. Baseline here: B = 132.4 (real-count ledger, five clues), so that both sides of each comparison use one ledger.

**Commitments built and priced on the official pieces** (`r3_guess_tests.out`; probabilities by local limit, checked against exact one- and two-coordinate dynamic programs to 1%):

| Commitment | I, bits | Test it unlocks | t, bits | R | Price | Against 132.4 |
|---|---|---|---|---|---|---|
| Checkerboard class of every tile (black or white cell) | 242.2 | Every edge joins a black and a white cell, so each of the 22 colours has as many half-edges on black cells as on white: 20 independent equalities, no orientation needed | 57.3 | 40.7 | 2^225.6 | +93 |
| Orientation of every interior tile | 382 | Directional census, 32 equalities tied by 16 parities | 94.2 | 51.5 | 2^339.3 | +207 |
| Axis of every interior tile | 191 | none | 0 | 57.9 | 2^248.9 | +117 |
| Pool of the top half, frame unknown | 191.9 | Colour counts against 42 free boundary sides | 0 exactly | | | |
| Pool of the top half, frame known | 191.9 | Same, 14 free sides | 0.001 (11 of 20,000 pools fail) | | | |
| Pool of a free 14x7 block, with an oracle for "this pool tiles it" | 191.9 | Exact | 11.8 | | about 2^180 survivors | |

The checkerboard guess is the best construction I have: it is the only split of the cells for which a colour balance needs no orientation, and its test is genuinely unavailable to the unconditioned search. It saves 91.7 bits of tree for 242.2 committed (0.38 bit per bit), needs a test worth 150.5 bits to break even, and has one worth 57.3. It loses by 93 bits.

**The same tests inside a region search** (the other reading of "a stronger feasibility test over the remaining tiles"):

- Colour-count parity and balance: an identity. Every colour total is even on the real set (`r1_tiles.out`), so the tiles left over always balance against the open frontier. Zero bits at every node.
- Directional census and checkerboard balance on the remaining pool: with r = 72 interior tiles left (the widest level), the first-moment number of orientation assignments meeting the census is 2^74.6 and of black and white splits meeting the balance 2^33.9. Neither test can refute there. They start to bite below about 30 tiles left, where the tree is already narrow.
- Hall matching of the frontier cells to the unused tiles: `r5_dives.out`. Randomised depth-first search with restarts on the real pieces with the five clues, 198 restarts, every board visited at nine depths tested. Refuted by matching and not by forward checking: 0 of 16,833 boards at depths 96 to 160; 18 of 2,733 at depth 168; 11 of 3,636 at depth 176. The sample is biased (boards a depth-first search meets first, many siblings), so it shows that the test does not fire ahead of the peak, not its value to a fraction of a bit. This is a small version of the memo's X2 and it does not meet X2's kill line.

**Not ruled out.** Tests that are none of the above (linear or semidefinite relaxations with the each-tile-once constraint), and guess-dependent orders. I have no candidate for either that prices below the baseline.

## 2. Uniform colours against the real counts

The real census: interior colours 12 x 50 and 5 x 48 half-edges (56 of the 840 are inward sides of frame pieces, very unevenly: 1 to 6 per colour); frame colours 5 x 24. I rebuilt the ledger with these counts dealt without replacement (`rlib.match_table`, exact rational arithmetic) and ran both ledgers through the same row-scan profile with clue cells handled exactly (`r2_ledger.out`).

| Quantity | Uniform (memo) | Real counts | Reference |
|---|---|---|---|
| log2 E[Z], no clue | 4.0 | 14.1 (23.1 with the exact frame count) | 23.46 canonical |
| Row scan total, five clues | 139.0 | 132.4 | 134.4 measured |
| Row scan total, one clue | 161.1 | 154.4 | 156.6 measured |
| Row scan total, no clue | 170.7 | 163.9 | 166.2 on file |
| Free 7x7 and 14x7 blocks | 118.3, 184.1 | 116.2, 180.1 | |
| Relaxed solutions (row H) | 361.6 | 371.7 | |
| One pair merged, coarse per fine (row E) | 67.4 | 49.5 or 47.5 | |
| Fifteen classes (row E) | 132.1 | 113.1 | |
| Orientation test (row F) | 113.6 | 94.2 | |
| Frames (row J) | 113.9 | 115.9 | 124.93 exact |

The gap between the ledgers grows along the scan: 3.0 bits at depth 64, 5.7 at 128, 6.7 at 158, 8.9 at 224, then reverses to +10.0 at the last level, because the final edges close for free when the counts are exact.

**Which of the twelve prices can be off by tens of bits.** Those that close all or nearly all 480 edges, or that compare two colour resolutions: E (18 to 20 bits on the refinement ratio and on the fifteen-class count, 5 to 13 on two classes), H (10 to 19), F (19, from the parity dependence, not from the counts), and exit 2's residual count if it is taken from the ledger and not from the canonical figure (the memo takes the canonical 24.5, correctly). J would be off by 11 bits but already uses the exact count. Band-scale prices (A, C, I, K and the row-scan baseline itself) move by 2 to 7 bits. B, D, G have margins of hundreds of bits and were not recomputed. No ruling flips.

## 3. The isoperimetric lemma

**The inequality.** Correct. The case analysis in Appendix A is complete: a full row and an empty column cannot coexist, so the unmixed rows and columns are all full or all empty, and in the empty case x + y >= 2 sqrt(xy) >= 2 sqrt(|S|) >= n. Exhaustive check: minimum 4 open edges for 4 to 12 cells on the 4x4, minimum 5 for 7 to 18 cells on the 5x5 (`r4_isoperimetry.out`). It holds for every set, so non-rectangular and disconnected sets do not escape the inequality.

**Its use.** They escape the use. The number of partial boards on S is not a function of the open-edge count: frame cells cost about 5.8 bits to fill against 9.6 for interior cells, a frame-to-frame edge pays back 2.3 bits against 4.1, and clue cells cost nothing. The memo's sentence treats the make-up of S as fixed; it is not. Searching over sets with five clues (`r4b_region_min.out`, `r4c_peak_orders.out`):

- At k = 192 the 192 free interior cells hold 2^84.1 partial boards with 56 open edges; the band holds 2^113.2 with 16. At k = 200 the gap is 45 bits.
- At k = 88 to 120 sets with 29 to 34 open edges and all four corners beat the prefix by 2 to 6 bits.
- Next to the peak, at every even size from 146 to 166 except 160, a non-band set beats the row-scan prefix, by 0.1 to 2.5 bits, with 20 to 28 open edges.

What survives is the bottleneck form: a static order passes through one set of each size, so its widest level is at least max over k of min over S of N(S). The search puts this at or below 2^127.55 (k = 152), 0.24 bit under row scan's widest level. Annealing gives upper bounds on each minimum, so the true bottleneck could be lower; I found nothing that suggests more than a bit or two. The abstract of Bollobás and Leader makes the same structural point: the extremal sets are not nested, so no single order is extremal at every size.

The claim "this is why the independently re-checked model finds row scan 9.9 digits ahead" is plausible and unproven; the lemma as used does not establish it.

## 4. Sources the author did not read

- **Bollobás and Leader, "Edge-isoperimetric inequalities in the grid", Combinatorica 11(4), 299 to 314, 1991.** Opened: the repository record with its abstract (https://digitalcommons.memphis.edu/facpubs/4558). The abstract states that a subset of [k]^n with between k^n/4 and 3k^n/4 elements has at least k^(n-1) boundary edges, which for n = 2, k = 16 is exactly the line printed by `ledger16.py` and the statement of L4. A secondary restatement of the main theorem (boundary at least min over r of |A|^(1-1/r) r k^(n/r - 1) for |A| <= k^n/2) agrees. Full text is paywalled; not opened. The attribution is accurate, and the memo does not rely on it.
- **"Fast Global Filtering for Eternity II", Constraint Programming Letters 3, 36 to 49, 2008.** The HAL page and PDF are behind a bot wall for me as well. Opened: the HAL metadata record (https://api.archives-ouvertes.fr/search/?q=halId_s:lirmm-00364330). Authors are listed there as Eric Bourreau and Thierry Benoist. The abstract concerns enumerating all solutions of an edge-matching puzzle with a model and data structures that give a filtering step in O(1) per node, compared with brute-force search and classical constraint programming; keywords: edge-matching puzzles, alldifferent, backjumping. The memo attributes nothing specific to it, so nothing is misattributed. It is the closest published work to row A (an all-different filter inside a tile search) and its body is still unread: the sentence "no board above 8x8 is reported solved with either model" has not been checked against it.

## 5. Left open, worth a study each

1. **Dynamic orders.** State in the memo that node-dependent cell orders are outside L1 to L4, and point to RQ6. A toy measurement of the best dynamic order against row scan at 7x7 and 8x8, with the ratio's trend, is the experiment that would test the verdict where it is weakest.
2. **A certificate for the bottleneck.** Replace the annealing estimate by an exact lower bound on min N(S) for 146 <= k <= 166: enumerate the cell-type make-up (corners, edge cells, interior cells, clue cells) and bound the closed edges of each type. Small integer program, one study. Until then "row scan is the best static order" is independently re-checked in a model for seven orders and estimated here for all.
3. **Swap the ledger.** `ledger16.py` should use the real-count table and the exact frame count. The remaining 2.0 bits between the real-count ledger and the measurement (the frame's left and right balance at partial depth) should be closed so that one ledger serves every gap memo.
4. **X2 with a level-uniform sampler.** My Hall sample is biased. Track 10's sampler on depths 96 to 160 would turn "0 of 16,833" into a price.
5. **Is D4's census tilt this fixed-count effect?** The 6.7 bits by which the real counts thin the tree at depth 158 come from the correlation between the tiles used and the colours left. D4 reported census statistics tilting toward the planted prefix at depths 144 and 160. If the two are the same effect, the tilt is already inside the measured 134.4 and is not exploitable. Worth one check before D4's tilt is priced as a method.
6. **Read the Constraint Programming Letters paper in full** from a browser, for row A and the 8x8 sentence.
7. **Checkerboard class first on toys.** The model says +93 bits on the 16x16. A 6x6 to 8x8 exhaustive run would confirm the sign and the 0.38 bit per bit; cheap, and it is the only guess-plus-test construction with a real test.

## Scripts

- `r1_tiles.py`: uniqueness of four-side patterns over all 256 tiles, partial-pattern ownership, real colour counts.
- `rlib.py`: shared helpers; exact matching probabilities for fixed colour counts.
- `r2_ledger.py`: uniform against real-count ledger; row scan with 0, 1, 5 clues against the measurements; rows C, E, H, J redone.
- `r3_guess_tests.py`: orientation, checkerboard and pool commitments with their tests, priced; the same tests on the remaining pool.
- `r4_isoperimetry.py`: exhaustive check of L4 on 4x4 and 5x5; first region search.
- `r4b_region_min.py`: region with the fewest partial boards for every size, both ledgers.
- `r4c_peak_orders.py`: focused search at sizes 146 to 166; black-first orders priced; two-class coarse count on the real tiles.
- `r5_dives.py`: Hall matching against forward checking on partial boards of the real puzzle.

Limits of this review: every price above is first-moment; the real-count ledger is itself 2.0 bits off the measurement; the region search is annealing, not exhaustive; the Hall sample is biased; rows I and L and the independently re-checked figures quoted from ROUTES were taken as given.
