# Referee report on the derived claims of the eight field surveys (2026-11-13)

Scope: the claims tagged derived in `surveys/*.md`, not the citations. One core, stdlib only. Scripts and their outputs are in `DERIVATIONS.REVIEW.scripts/` (list at the end). "Model" below means the repo's first-moment model on the real colour census (`M12/solve-path-3/model.py`, copied verbatim as `m12_model.py`); with five clues and rows bottom-up it gives 10^40.52 = 2^134.6 against the measured 2^134.4.

## 1. Claims that are wrong or not established, and what replaces them

**W1. statistical-physics 3.1 and 4.2: "chi^2 = 1.03/14,702, so no statistic of the piece set has advantage above 0.4%". WRONG (the number and the conclusion; the identity is right).**
- The identity P_pl = P_0 x Z / E_0[Z], chi^2 = E_pl[Z]/E_0[Z] - 1, TV <= 0.5 sqrt(chi^2) is correct for a painter that is uniform over edge colourings with the exact census, with Z counted on labelled tiles. It survives conditioning both sides on "no duplicate tile, no symmetric tile" (rejection keeps the reweighting by Z). The direction and the Cauchy-Schwarz corollary are right.
- The plug-in E_pl[Z] = 1 + E[Z] + 0.03 is wrong. It assumes that solutions differing from the planted one almost everywhere contribute exactly E[Z]. They contribute E[Z](1 + delta) with delta of order 0.1, and chi^2 is about delta, not 1/E[Z].
- Proof by exhibit, in the survey's own null (census fixed, colours exchangeable over half-edge slots), `s_chi2.py`:
  - Statistic A, e_c = number of edge pieces whose inward colour is c. Null: 56 of 840 slots. Painted: 56 of 420 edges. The likelihood ratio is exact. chi^2 = 0.043, TV = 0.082 (300,000 null samples, mean of L = 1.0008).
  - Statistic B, equal-colour side pairs inside interior tiles: null 67.9 +- 8.1, painted 66.5 +- 7.9, TV about 0.07.
  - Statistic C, corner colours (exact enumeration): chi^2 = 0.021.
  - By data processing each is a lower bound for the full piece set. Together chi^2 >= about 0.10. So a statistic with advantage 8% exists, 20 times the claimed ceiling, and the bound 0.5 sqrt(chi^2) cannot be below about 0.16.
  - Toy 4x4, census fixed, exact Z on 3,945 painted boards with distinct tiles: E_pl[Z|D] = 59.65 +- 0.57 against E_0[Z|D] = 33.43, chi^2 = 0.78; the U4 form predicts 0.03. The excess E_pl - E_0 - 1 is 25, proportional to E_0, not a small constant.
- Separate arithmetic error inside U4 (theory-referee.md line 337): two non-adjacent edge pieces swap when they are twins on three sides, probability 1/425 per pair, expected 3.6 pairs, not C(56,2)(1/5)^4(1/17)^2 = 8.5e-3 (the two fit events are the same event, not independent). Same for corners: 6/25, not 6/625. Without the no-duplicate condition these terms alone give E_pl[Z] - E[Z] of order 100; with it they are zero. Either way "+0.03" is not a computed number.
- The real set is ordinary on all three statistics (L_A = 1.17, 78th null percentile; B = 70, z = +0.26).
- Replacement: "In the first-moment null, planted and null piece sets are distinguishable with advantage at least 8% by census-allocation statistics (chi^2 >= 0.10). An upper bound on chi^2 needs the full second moment, which nobody has computed. None of the distinguishing statistics found localises a solution." Do not write "nothing can be there".
- Scope errors even if the number were right: (a) it bounds DETECTION from the piece set alone. It says nothing about recovery: given the tiles, the planted solution is uniform among the Z solutions, so no statistic separates it from the others, and separating solution prefixes from dead prefixes is the search problem itself, which quietness does not address. What bounded chi^2 does give is transfer of rare events: an event of null probability p has planted probability at most sqrt(p(1 + chi^2)). (b) With the five clues given the bound is vacuous: E_0[Z_5] = 4.1e-8, so chi^2 is about 2.4e7. A statistic that uses the clue tiles and positions is not covered. (c) It is conditional on the uniform painter.

**W2. sat-cp-proof-complexity P4: "a freed arc of m cells has about m!/5^(m+1) arrangements, under 1 up to m = 12". WRONG by about 10 bits, in the direction that matters for reading D1.**
- m!/5^(m+1) is the unconditional expectation for m random pieces. The freed pieces came out of a valid run, so the count is at least 1 and is the number of Eulerian trails of their colour multigraph. On the real 56 edge pieces, 200 random valid runs per m (`s_arcs.py`, inward colours unconstrained, corners excluded):

| m | survey, log2 | measured median log2 | single arrangement |
|---|---|---|---|
| 3 | -6.7 | 0.0 | 79% |
| 6 | -6.8 | 1.0 | 19% |
| 12 | -1.3 | 7.8 | 0% |
| 13 | 0.0 | 9.3 | 0% |
| 20 | 12.3 | 22.2 | 0% |
| 30 | 35.7 | 46.1 | 0% |
| 30 cells in 10 arcs of 3 | 14.8 | 26.5 | 0% |

- Replacement: "a freed arc of m cells has about 2^10 x m!/5^(m+1) arrangements once m >= 8 (median 2^8 at m = 12, 2^22 at m = 20, 2^46 at m = 30); 30 scattered cells carry 2^26, not 2^15." The qualitative point (60 pieces on 60 cells is a bijection, so short freed arcs are not free: 79% of 3-cell arcs have a single arrangement) holds. The quantitative reading does not: a core that leaves 30 cells out leaves 2^26 to 2^46 frames standing, so b by volume is 26 to 46 bits below its full-frame value, not "within 15 to 35", and "a handful of alternatives" is wrong. b by volume must be computed by trail counting on the actual freed set, never from the formula.

**W3. monte-carlo 4: "SMC as a search costs 2^138.1, 3.7 bits worse than row scan". WRONG as arithmetic.**
- It charges W_peak particles at each of 256 depths. A population cannot exceed the level width. With population P the success probability is P/W_peak and a run costs the sum over depths of min(W_d, P, P W_d / max width so far). Model profile, `s_model.py`: expected nodes 2^137.2 at P = 10^6, 2^136.3 at 10^30, 2^135.7 at 10^36, and exactly the tree (2^134.6 in the model) at P = W_peak, where SMC is breadth-first search with 2^130 memory.
- Replacement: "0 to 2.6 bits worse than row scan, never better, and equal only with memory equal to the widest level."

**W4. cryptanalysis-coding D5 and T1: the extreme-pruning gain. NOT ESTABLISHED as stated; formula incomplete; the K = 4 example nets less than K = 1.**
- Derivation (Gaussian statistic, unit variance on both classes, K independent presentations, threshold t with Q(t - delta) = 1/K, all K trees paid): gain = (1/K)/Q(t) = [delta z_K + delta^2/2 + ln(1 + delta/z_K)] / ln 2 bits with z_K = Q^-1(1/K). The survey's "delta sqrt(2 ln K)/ln 2" drops delta^2/2 and overstates z_K; it is 4 to 5 bits low at delta = 3. The table entries themselves are reproduced (3.7, 6.7, 8.1, 11.4, 27.7).
- "One of them is expected to pass" is a 63% success probability, not 1.
- Most of the K = 4 figure is not extreme pruning: one presentation at 50% success already gains 2.6 bits at delta = 1.4.
- The four scan directions are not four equal trees: model totals 10^41.38, 40.52, 40.53, 41.47, so running all four costs 2^4.19 times the best one, not 2^2. Net K = 4 gain: 3.7 - 2.2 = 1.5 bits, below K = 1.
- Dependence: with correlation rho between presentations, success probability fixed at 0.5 and all runs paid (delta = 1.4): K = 4 gives 4.9 bits at rho = 0, 3.6 at 0.6, 1.6 at 1; K = 100 gives 7.5, 2.8, -3.0; K = 1 gives 3.6. Dependence turns extra presentations into pure cost. The whole gain above the one-shot classifier rests on independence, which one fixed instance does not supply.
- Cap not stated: a prune applied at depth d cannot gain more than total/N(d). At depth 128 that is 12.5 bits in the model, so the 17 to 28 bit entries need pruning at several levels, with success probabilities multiplying.
- Replacement: "gain = log2 of the likelihood ratio at the operating threshold; with one presentation and delta = 1.4 that is 2.6 to 3.6 bits; more presentations help only as far as they are independent and equally cheap, and here neither holds."

**W5. cryptanalysis-coding D3 and section 4: "every list method here pays more than the tree". WRONG against its own table.** The same survey quotes the two-list join at 131.9 bits with floor 126.6, both below 134.4. Replacement: "no list method is known to gain more than 2.5 bits (priced) to 7.8 bits (floor)". Also H_4 and H_8 do not reproduce: for four bands of four rows I get 476.2 bits (pools 2/0/0/2, 20/8/8/20, 42/56/56/42), not 488.5, and 689.7 for eight bands, not 716; list sizes 2^158.7 and 2^172.4, not 2^162.8 and 2^179.0. The trend (worse as k grows) stands.

**W6. cryptanalysis-coding D2 and D4: "a two-list band join pays at least the larger list, 10^38.1; representations give a list no smaller than the best single cut". NOT ESTABLISHED in that form.** List sizes are not the floor: a cut after row 14 has lists of 10^18.3 and about 10^20.4 (model, five clues). The floor is the cost of GENERATING the lists, which passes through each region's own widest level; min over band cuts of that is 10^38.16 (cut after row 7 or 9). "Mean of N(S) >= min of N(S)" is true and proves nothing about cost. The verdict (no gain) holds for a stronger reason the survey does not state: a random linear hash of the final used set cannot prune a region's enumeration before it ends, so R representations cost R full enumerations to keep 1/R of each.

**W7. cryptanalysis-coding analog A: "the density is the uniqueness bound of the code". WRONG wording.** As a random code the instance has 2^2217.7 regular vectors against 257^992 = 2^7941.6 syndromes, far past any uniqueness bound. The count of about 1 comes from the checks being low-entropy and structured (the piece checks remove 357.6 bits, the edge checks about 1,837), which is exactly why the ISD prices are meaningless as a barrier. The linear lift itself and 2^-1717.4 are correct arithmetic.

**W8. sat-cp-proof-complexity D2: "cutting planes buys well under one bit". NOT ESTABLISHED.** The evidence is a node share (Hall checks remove under 1% of nodes on 6x6 to 8x8 trees). That bounds Hall-set propagation along a row-scan tree at those sizes. It does not bound cutting-planes proof size, which is not tied to the tree, and no lower bound for cutting planes on this formula exists. Replace by: "Hall-set propagation along the search tree is measured inert at 6x6 to 8x8; nothing is known about cutting-planes refutations at 16x16."

**W9. monte-carlo B1: "the measured tree is balanced in the sense of Chen, Gomes, Selman". NOT ESTABLISHED from the cited evidence; the conclusion is supported on toys.** The theorem is for a complete b-ary tree. The 0.03-digit agreement is about the mean level sizes, not about the dispersion of sibling subtrees along the planted path, which is what the runtime distribution depends on. Direct test (`s_restart_toy.py`, 54 single-solution planted boards, 5x5 and 6x6, exact sibling subtree sizes, 6,000 random value orders each): E[T]/tree = 0.50 to 0.51; the best cutoff gains a median of 0.00 bits (means 0.04, 0.20, 0.02; one board of 54 above 1 bit, at 1.57); a cutoff at tree/10 loses 1.3 to 1.5 bits and at tree/100 loses 1.6 to 6.9 bits. Replacement: "on toy boards restarts never gain and short cutoffs lose more than 1 bit; the theorem does not apply to the real tree and the property is unmeasured at 16x16." "About 1 bit" is a floor on the loss, not an estimate.

**W10. exact-algorithms B1, wording only.** The proof is correct, but (a) m_k in the statement is a maximum while the quoted 1.05 to 1.14 are averages, and (b) "never beats the tree" overreaches for non-sweeps. See ruling 1 for the corrected statement.

**W11. tilings T2: "about 6 bits per pin, so 9 to 11 pins". Holds only as an average; the count of pins is wrong for well-placed pins.** The 6.0 comes from two different instances. Model, real census, five clues (`s_model.py`): one extra correct pin is worth 9.6 bits at best (an interior cell ahead of the peak, log2(4 x 191) for the removed choice), median 4.8 and mean 5.0 for a random cell with the scan fixed (1.0 to 1.5 bits in the rows already past the peak), median 9.3 if the scan direction is re-chosen after the pin. Greedy best placement: 6 pins remove 57 bits, 7 remove 67.

**W12. learning-guided-search 4: fill-order ranking.** Row scan 51.37, spiral 55.84, frame-first 57.56, anti-diagonal 57.72 reproduce exactly in the uniform model. With the real census and five clues: row scan 40.52, anti-diagonal 50.39, spiral 51.99, frame-first 52.49. Row scan stays best by 9.9 digits; the order of the other three reverses (anti-diagonal becomes the best of them), so only "row scan beats these three" survives, not the ranking.

## 2. Rulings

| # | Claim | Ruling | My derivation or number |
|---|---|---|---|
| 1 | exact B1, partial sieve | **Holds with rewording** (stronger than stated) | Strong form: for any region (prefix of any cell order, or any region of a divide and conquer), sum over S of table entries >= D, the number of distinct (boundary colours, full used set) among strict fillings. Proof: a state of the S-run holds at most C(s, a) distinct strict (boundary, used set) triples; sum over S of C(u-a, s-a)/C(s, a) = sum of C(u, s)/C(u, a) >= 1. No m is needed; D = N/(average merge). Checked: 130 cases (3x3 and 4x4, orders row-major, frame-first, two-sided, random, checkerboard; five splits), inclusion-exclusion identity exact in all, smallest ratio 1.000 over all depths, 1x to 958x at the widest strict level (parity form 1x to 328x, never below 1.000). Scope: per-S or parity frontier tables with a tile-subset sieve. Any order is covered, but the comparison is with the strict table of the SAME order or regions; for a separator recursion that is the list join (126.6 to 131.9 bits), not 134.4. Not covered: sieves not indexed by tile subsets, determinant or extensor forms, hashed (lossy) used sets, search without tables. |
| 2 | exact B2, rank | **Holds with rewording** | The matrix is a permutation matrix over every field (determinant +-1). With boundary colours the right matrix is (boundary f, used A) x (completion boundary f', used B) = [f = f'][B = complement of A]: block diagonal with permutation blocks, still full rank. This is the instance-oblivious matrix. On the instance, rows of dead states are zero and the rank is the number of live states (about the number of solutions), so the true statement is "no compression that does not decide liveness". Also: the approximate rank of a permutation matrix is logarithmic (equality has constant randomised communication), which is the hashed join; it compresses the key, not the number of rows. B2 and cryptanalysis D3 are the same fact. |
| 3 | exact prices | **Holds** | 326.4, 327.0, 200.9, 258.9, 317.9, 333.9, 2^59 and 2^72.5, 0.235 to 0.289, 1.439 all reproduce. Notes: 317.9 uses 256 tiles where 312.9 uses 251; 0.525 is per cell (0.535 per free tile); C(256,135) = 2^251.1. The 2^75.4 is reproduced as TIME: 256 cells x 2^65.96 mid-row states x 784/289 fitting placements = 2^75.4 = 10^22.70 (unit = one tile placement, big-number adds not counted). Statistical-physics "2^66" is the mid-row STATE count (memory); 2^61.87 is the between-rows state count. Use 75.4 against tree nodes, 62 to 66 for memory. "Everything between 2^66 and 2^134" should read "between 2^75 and 2^134 in time". |
| 4 | stat-phys chi^2 | **Wrong** (number, conclusion); identity holds | W1. chi^2 >= 0.10, TV >= 0.08 in the survey's model. |
| 5a | SAT D1, unsat only through distinctness | **Holds with rewording** | 2^167.7 and 2^-109.9 reproduce. It is an expectation over the model for a typical wrong frame; satisfiability of the colour-only relaxation for a given frame is not shown, only expected. |
| 5b | SAT D2, cutting planes under a bit | **Not established** | W8. |
| 5c | SAT P4, arc formula | **Wrong** by about 10 bits | W2. |
| 6a | crypto D3, k-XOR hashability | **Holds** (identity); numbers for k = 4, 8 **wrong** | XOR = all-ones with sizes summing to 256 forces a partition: correct, and it corrects path 3. H_2 = 247.2 and 2^123.6 reproduce; H_4 = 476.2, H_8 = 689.7 (W5). |
| 6b | crypto D4, representations | **Holds with rewording** | W6. |
| 6c | crypto D5, extreme pruning | **Not established**; formula incomplete | W4. |
| 6d | crypto D6, syndrome decoding | **Holds** as arithmetic; "uniqueness bound" **wrong** | 156,816 unknowns, 992 checks, 2^-1717.4, 2^1870 reproduce. W7. |
| 7a | MC, SMC as search | **Wrong** | W3: 2^134.6 to 2^137.2. |
| 7b | MC, restarts lose 1 bit | **Holds with rewording** | W9. |
| 8a | learning, static orders | **Holds with rewording** | W12. |
| 8b | tilings, 6 bits per pin | **Holds with rewording** | W11: best 9.6, typical 4.8; 6 to 7 well-placed pins, not 9 to 11. |

## 3. Cross-survey inconsistencies (item 9)

| Quantity | Who says what | Right |
|---|---|---|
| Colour-only frontier | exact, crypto: 2^61.87 (2^65.96 mid-row); stat-phys: 2^61.9 and "2^66 to 2^134"; SAT D5 and D7: "17^14 = 2^57", "57 bits of state" | 2^61.87 between rows (SAT drops the two frame legs, 5^2 = 4.6 bits), 2^65.96 mid-row, 2^75.4 time |
| Cost of a list join against the tree | crypto text: "every list method pays more than the tree"; crypto table and exact 2b: 131.9, floor 126.6 | 131.9 priced, 126.6 floor, both below 134.4 |
| SMC as a search | MC: 2^138.1; implied by every other survey's use of the tree: 2^134.4 | 2^134.6 to 2^137.2 (W3) |
| Planted against null | stat-phys: "nothing can be there", 0.4%; learning 3: "matching the aggregate count does not exclude a signal that small; quiet planting is an asymptotic statement" | learning is right to hedge; chi^2 >= 0.10 (W1) |
| Value of a pin | tilings: 6.0 bits, 9 to 11 pins; stat-phys: five clues worth 31.8 bits (6.4 each); path 3: 9.6 bits for an early clue | all three are the same curve: 9.6 best, about 5 typical, 6.4 for the real clues; 6 to 7 best-placed pins close 56 to 66 bits |
| Pruning with low success rate | crypto T1: 3.7 bits (K = 4) to 11.4, independence assumed; MC C2: at most 8 to 10 bits and "re-randomisations are not independent here" | MC's caveat is right; with realistic costs K = 4 nets 1.5 bits (W4) |
| Remaining gap after a 10-bit gain | MC C2: "124.4 bits, still 2^58 or more above 1,000 core-years"; MC C1, learning, tilings, quantum: gap 56 to 66 bits from 134.4 | after 10 bits the gap is 46 to 56 bits; "2^58 or more" is an arithmetic slip |
| Restarts | MC: lose 1 bit, 135.4; ROUTES row quoted by crypto: "killed, reported" | toys: no gain, loss of 1.3 bits or more for short cutoffs; unmeasured at 16x16 |
| Tree peak | MC: free depth 158, 2^130.1 (measured); learning: depth 161 (no clue, uniform model); model with five clues: depth 161, 2^130.2 | consistent (158 free = 161 minus three clue cells already passed) |
| Per-cell rate, frame count, placements, clue selection | exact and stat-phys 0.525; SAT, learning, exact 2^124.9 = 10^37.6; learning 1,860 bits, stat-phys 1289.3 nats; stat-phys 48 bits | all consistent and reproduced (1860.1 bits; 23.46 + 24.54 = 48.0) |

## 4. The strongest correct statement of why each-tile-once is the obstruction

Without the each-tile-once rule the board is a width-16 colour-only contraction costing 2^62 to 2^66 states and 2^75.4 tile placements; with it, every exact method priced so far costs between 2^126.6 (model floor of a two-list join) and 2^134.4 (measured row scan). The rule itself is not exotic: it is linear in the lifted variables and hashable for any number of parts. What it defeats is tabulation. Proved and checked on 130 toy cases: any algorithm that tabulates fillings of regions by (boundary colours, the used tiles restricted to a tracked subset) and sieves the remaining tiles by inclusion-exclusion over tile subsets, in per-subset or parity form, for any split and any cell order or region decomposition, writes at least as many table entries as there are distinct (boundary, full used set) pairs among the strict fillings of those same regions; and that number cannot be reduced by any rank or representative-set argument that does not decide which states are live, because the compatibility matrix between partial fillings and completions is block-diagonal with permutation blocks, full rank over every field. On the real tiles distinct states and strict fillings coincide to within a merge factor argued (not measured) to be about 1, so this family cannot beat strict enumeration of the same regions. Exact scope: this is a statement about one family of algorithms (region tables plus tile-subset sieves, and oblivious linear compression), compared against the strict table of the same decomposition, not against 134.4 bits as such. It is not a lower bound for the instance, it says nothing about search that prunes with instance-specific reasoning, about sieves that are not indexed by tile subsets, or about lossy hashed states, and it uses neither the planted solution nor the clues. The supporting claim "planted and random sets are statistically indistinguishable" must not be attached to it: in the first-moment null they are distinguishable with advantage at least 8%, and what is true is only that no statistic found so far points at the solution.

## 5. What I did not do

- No upper bound on chi^2 at 16x16 (needs the full second moment). The lower bound 0.10 treats statistics A, B, C as independent; A alone (0.043, TV 0.082) is exact up to Monte Carlo error.
- No check that the colour-only relaxation is satisfiable for a specific wrong frame.
- Arc counts ignore corners and inward colours; with interior assumptions in the core the counts drop.
- Restart test is on toy trees of 10^3 to 10^6 nodes, first corner pinned to remove board rotations.
- The survey's H_4 = 488.5 could not be reproduced; its band pools are not stated, mine are in W5.
- The merge factor m on the real tiles (B1 reading) is path 3's argument, not checked here.
- "Row scan is the best static order" is checked against six other orders only.

## Scripts (`DERIVATIONS.REVIEW.scripts/`, all `python3 -I`)

- `s_arith.py`: prices, entropies, ISD, arc formula, extreme-pruning table and dependence (items 3, 5, 6).
- `s_model.py <csv>`: fill orders, scan directions, band cuts, SMC cost, pin values (items 6, 7, 8); uses `m12_model.py` (verbatim copy of `M12/solve-path-3/model.py`).
- `s_b1_toy.py`, `s_b1_toy.out`: B1 on toy boards, 150 s (item 1).
- `s_chi2.py <csv>`, `s_chi2.out`: planted against null, real census and toy 4x4, 55 s (item 4).
- `s_arcs.py <csv>`, `s_arcs.out`: trail counts of freed border arcs on the real edge pieces, 2 min (item 5).
- `s_restart_toy.py`, `s_restart_toy.out`: runtime distribution under random value order, 5 s (item 7).
