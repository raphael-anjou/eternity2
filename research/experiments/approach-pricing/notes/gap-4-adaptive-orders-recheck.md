# Second analysis report: gap 4, fill orders that adapt to the board

Second analysis of `gap-4-adaptive-orders.md` (2026-11-13). Scripts and raw rows: `gap-4-review/`. Generated boards are toys and are labelled as such. Figures on the official 16x16 pieces are estimates of tree sizes, not progress toward a solution. Grades used below: measured exact (full enumeration), estimated (population estimate, section 1), proved, cited, argued.

## Rulings

| # | Claim in the memo | Ruling | Corrected statement |
|---|---|---|---|
| 1 | Verdict: the gap is empty as far as ordering goes; no adaptive order is worth 10 bits | **stands, with a narrower scope** | For smallest-domain cell rules (free, frontier, band, staircase, one-step lookahead), under forward checking and under arc consistency, on the official pieces with the five clues: every rule is within about 3 bits of row scan or far worse (13 to 49 bits worse for the free rules). Rules that finish rows one or two at a time cannot gain more than 5.3 and 6.7 bits whatever they do (lemma, section 2). Not covered: weighted-degree and other learning rules at size, restarts, tile and colour branching beyond the memo's own argument. |
| 2 | "The best any adaptive cell order can do with forward checking is about 1.1 bits" (verdict, 3.3) | **wrong as a bound; right as an order of size** | 1.1 bits is the best static staircase in the model, not a cap on adaptive rules (the memo says so itself in 3.4). Estimated on the official pieces under forward checking: band rule +0.35 bit, lookahead +0.6, two-row staircase -0.7, three-row staircase +2.9 (section 2). The three-row figure is above the memo's cap; it is not reproduced on a generated 16x16 (+0.45) and is zero under arc consistency. |
| 3 | The minimum is taken over staircases with up to 4 open rows | **stands**, but on my measurements, not on the memo's argument | Isoperimetry justifies staircases for the count of valid fillings only. For survivors the memo assumed it. What lies outside is worse on the official pieces: non-staircase band rules are within 0.6 bit of row scan, free rules 13 to 49 bits worse. |
| 4 | "Annealed min-domain order: 2^135.5, 1.0 bit worse than row scan" (table 3.3) | **wrong as a price of the smallest-domain rule** | That line follows the cell with the smallest expected domain, which is a fixed order. The actual rule on the official pieces: 2^170.5 with forward checking (36 bits worse), 2^157.8 with arc consistency (29 bits worse); restricted to the frontier of decided cells, 2^183 and 2^142. The error does not help the adaptive side. |
| 5 | Toys: staircase rules within 0.3 bit; free rules gain at 6x6 and lose 1.2 to 2.6 bits at 7x7 and 8x8 | **stands with a narrower scope** | At near-critical colour counts the crossover is later and the slope is steeper. Smallest domain anywhere, forward checking: +3.5, +2.1, +0.7, -1.3, -8.3, -11.2 bits at n = 5 to 10; arc consistency: +0.6, +0.7, +0.1, -1.0, -4.6, -6.8 (section 3). The sign of the trend is the memo's. |
| 6 | Arc consistency is worth about 4 bits on the toys and 5.2 in the program's arithmetic, flat with size | **stands** | 4.7, 4.8, 4.7, 4.5, 4.6, 4.7 bits at n = 5 to 10 near critical; 5.5 bits (branches tried) or 6.1 (surviving nodes) estimated directly on the official pieces. |
| 7 | Dynamic order under arc consistency at 16x16: "no model, no measurement" | **now measured (estimated)**; no sign of more than additive | On the official pieces under arc consistency: two-row staircase -0.1, three-row staircase +0.05, band +0.2, lookahead +0.7 (two runs), frontier -12.8, anywhere -28.9 bits against row scan. On toys the gain of a dynamic rule under arc consistency is a third or less of its gain under forward checking (section 4). |
| 8 | Conflict-driven rules: "the literature opened suggests a constant factor" | **unproven; one toy trend against, nothing at size** | Weighted degree under arc consistency, exact: +0.06 (5x5), +0.50 (6x6), -0.39 bit (7x7, 3 boards). No published test of a conflict-driven order on edge matching found. The memo's sentence "nothing found that prices a dynamic order on edge matching at the critical density" is wrong: Ansótegui et al. do (section 5). |
| 9 | D5: all 10 unpinned adaptive runs censored; D5 shows that an adaptive solver uses pins, not that an adaptive order shrinks an unpinned tree | **stands**, with two additions | The cap (2^16.6) sits 6.3 to 11.5 bits under the row-scan trees, so the censored runs say nothing in either direction. On pinned boards D5's adaptive solver is 8 to 10 bits under plain row scan, and on my own five-clue 8x8 boards that whole gap is arc consistency (15 bits), the order adding 0.1 bit. On near-critical five-clue 8x8 boards a frontier rule does add 4.7 bits; that does not carry to the 16x16 (section 6). |
| 10 | Tile-centred and colour-centred branching: 0 bits | **stands as argued**; not re-measured by me | No change. The memo's own toys (gain at 6x6, loss at 7x7) are the only data. |
| 11 | Odds: 1 percent for 10 bits, under 1 in 10,000 for 56 | **stands** for domain-size rules; I would not go below 2 percent overall | The untested part is learning rules at size (item 8). |

A wrong "empty" was the costly error to look for. I did not find it. The memo's model is weaker than its verdict, and two of its lines are wrong (items 2 and 4), but no measurement at size came near 10 bits: the largest gain on the official pieces is 2.9 bits, for one rule, under forward checking only, and it vanishes under arc consistency.

## 1. What I ran

One solver written from scratch (`dyn.c`): bitset domains over (tile, rotation), three propagation levels (none; forward checking; arc consistency on joint colours with the rule "a cell with one value removes that tile elsewhere", the memo's definition), and these rules: ROW; Y2, Y3, Y5 (the memo's staircase rule); B2, B3 (smallest domain among all empty cells of the first 2 or 3 unfinished rows, staircase or not); L2 (takes the 6 best cells of B2, counts the surviving children of each, branches on the fewest; the trial propagations are not counted); DOMF (smallest domain among empty cells touching a decided cell); DOM (smallest domain anywhere); WDEG (domain over weighted degree, weights from wipeouts). Ties go to the cell with most decided neighbours or walls, then to the lowest index. The measure is the memo's: branches tried.

Two modes. Exact: full enumeration of all solutions. Estimate: a population of up to M partial boards per depth, drawn uniformly without replacement from the children of the previous sample; the node count of a depth is the previous count times children per parent. Each depth's estimate is unbiased, and depths with at most M nodes are enumerated exactly. This lets the same rules be run on the official pieces.

Checks of the tool:
- D5's board 1 (8x8, 11 interior colours), row scan without lookahead: 64,893,775 nodes and 4 solutions, equal to D5's engine and to its independent assessor's own enumerator.
- Official pieces, five clues, row scan from the last cell (the orientation whose widest level is 157 to 158, as in the plan): 2^134.36 (five runs, 133.6 to 135.3). The plan says 2^134.4.
- Estimate against exact on two 8x8 toys with a deliberately small population (M = 100, 30 runs; rules ROW, Y2, Y3, B2, and DOMF on one board): log2 of the mean within 0.23 bit of exact in all 9 cases (`valid8.jsonl`, `analyze_valid.py`).
- Solution counts agree across all exact variants on every board.

Limit of the estimator: for the staircase rules under forward checking on the 16x16 the runs spread over 6 bits (Y2: 131.3 to 137.1 at M = 12,000) while row scan spreads over 0.8. A few runs carry most of the mean. Because the estimate is unbiased, a run can sit far below the truth but the mean is unlikely to sit far above it (Markov: the mean exceeds 8 times the truth with probability at most 1/8). So these runs bound gains from above more firmly than losses.

## 2. Point 1: is the staircase restriction justified, and what bounds a truly dynamic rule

**What the memo's number is.** L = sum over depths of the minimum over staircase shapes of survivors. It is a statement about fixed shapes. A dynamic rule puts different nodes of one depth on different shapes, so L is neither an upper nor a lower bound for it. The memo concedes this in 3.4 point 1 and then writes "at most" in the verdict. The restriction to staircases is sound for valid fillings (each open joint costs a factor 17, and the 6b second analysis found row scan within 0.24 bit of the best set). For survivors it is assumed.

**A lemma that does bound a class of dynamic rules (proved).** Call a propagation monotone if a partial board it refutes stays refuted when more cells are filled; no lookahead, forward checking and arc consistency all are. Take any rule, static or dynamic, that fills a block of rows completely before it touches the next block, in any order inside the block and with any dependence on the node. Let B be the cells of the first j blocks. Every valid filling of B that survives propagation is a node of the tree: all its sub-fillings are valid, and by monotonicity all survive, so the rule's own path reaches it. No filling of B is reached twice. So the rule has exactly row scan's node count at every block end, and its tree is at least the sum of row scan's counts at block ends. Its gain over row scan is at most log2(sum over all levels / sum over block-end levels).

From the estimated level profile of row scan on the official pieces (`analyze_profile.py`, two runs each):

| blocks of | no lookahead | forward checking | arc consistency |
|---|---|---|---|
| 1 row | 5.9 to 6.0 bits | 5.2 to 5.3 | 4.0 |
| 2 rows, best alignment | 7.4 | 6.7 | 6.5 |
| 4 rows, best alignment | 12.2 | 12.1 | 11.3 |

So no rule that works inside one row at a time, or inside pairs of rows, can be worth 10 bits, whatever information it uses, weighted degree and lookahead included. The lemma says nothing for rules that keep a sliding window or roam freely; those are measured below. It is a cap, not an estimate: the measured rules gain far less.

**The dynamic rule the plan asks about, on the real colour counts (estimated, official pieces, five clues).** Log2 of branches tried, mean over runs, range of runs in brackets, gain over row scan under the same propagation:

| rule | forward checking | gain | arc consistency | gain |
|---|---|---|---|---|
| ROW | 134.44 [134.05, 134.85] | | 128.86 [128.09, 129.42] | |
| Y2 (memo's mechanism 1) | 135.14 [130.59, 137.40] | -0.7 | 128.94 [128.75, 129.12] | -0.08 |
| Y3 | 131.59 [130.70, 132.92] | +2.85 | 128.81 [128.38, 129.08] | +0.05 |
| Y5 (M = 3,000) | 132.04 [131.09, 133.14] | +1.9 | not run | |
| B2 | 134.08 [133.48, 134.39] | +0.35 | 128.63 [127.95, 128.99] | +0.23 |
| B3 (M = 3,000) | 134.54 [133.18, 135.03] | -0.6 | not run | |
| L2 (lookahead) | 133.87 [133.20, 134.21] | +0.56 | 128.15 [128.02, 128.26], 2 runs | +0.7 |
| DOMF (frontier) | 183.29 [177.4, 185.5] | -49.3 | 141.70, 1 run | -12.8 |
| DOM (anywhere) | 170.53 [167.4, 171.9] | -36.6 | 157.80, 1 run | -28.9 |

Populations: forward checking M = 12,000 (5 runs; Y2 and Y3 13 runs) unless marked, L2 M = 4,000 (3 runs); the M = 3,000 rows (5 runs) are set against ROW at the same M (133.97); arc consistency M = 2,000 (3 or 4 runs), L2 M = 600, DOMF and DOM M = 500. Files `real16.jsonl`, `real16_fc.jsonl`, `real16_ac.jsonl`.

Reading. The smallest-domain rule anywhere on the board, the genuinely dynamic rule, costs 2^170 with forward checking and 2^158 with arc consistency: it starts from the four corners and the clues, opens many fronts, and pays for every open joint. Restricting it to the frontier of decided cells is worse still under forward checking, because it grows five blobs around the clues. Rules that keep the row-scan shape and choose inside it are at row scan's level within the noise. The three-row staircase under forward checking is the one line above the memo's 1.1 bits: +2.85, thirteen runs within 2.2 bits of each other. I do not have a mechanism for it and it does not generalise: the two-row staircase on the same board is at -0.7 with runs spread over 7 bits, the five-row one at +1.9, and all of them are at zero under arc consistency. On a GENERATED 16x16 with the same colour numbers (`toy16.jsonl`, M = 12,000, 4 runs each) Y2 and Y3 are at 0.0 and -0.4 bit with only a corner fixed, and at +0.46 and +0.45 with five more planted tiles in the clue cells. The one-sided bound of section 1 applies to the official figure: with a mean of 2^131.59 over thirteen runs, a true Y3 tree under 2^124.4 (a 10-bit gain) has probability under 1 percent.

**The memo's lines against these.** Row scan with forward checking, survivors against valid fillings: memo 0.75 bit, estimated 0.6 (134.36 against 133.72 surviving nodes). Staircase minimum: memo 1.1 bits, estimated -0.7 (two rows), +2.85 (three rows), +1.9 (five rows). Min-domain order: memo -1.0 bit, estimated -36.6. Arc consistency: the program's arithmetic 5.2 bits, estimated 5.5.

## 3. Point 2: exact trees on near-critical toys, and the trend with size

**Colour counts.** For generated boards with 4 frame colours and one corner tile fixed, the first-moment count of fillings of an unplanted board is E = 3! (4(n-2))! ((n-2)^2)! 4^((n-2)^2) / (4^(4(n-1)) ci^(2(n-1)(n-2))). The critical interior count solves E = 1: 2.82, 4.22, 5.61, 6.99, 8.36, 9.71 for n = 5 to 10 (`crit.py`). The ladder uses the smallest whole count with E under 1: 3, 5, 6, 7, 9, 10. The same formula with 5 frame colours gives 17.06 for the 16x16, which has 17, so this is the regime of the real puzzle. The memo's 6, 8, 11 at n = 6, 7, 8 are 1.8, 2.4 and 4.0 above. Measured solution counts on my boards: 1 to 97 at 6x6, 68 to 2,210 at 5x5 (planted boards with so few colours have many solutions; 5x5 is on the many-solution side), 1 to 3 at 7x7 with 7 colours. Every board has the planted tile of cell 0 fixed, which removes the four-fold turn symmetry and favours no rule (all start there).

**Exact (full enumeration).** Paired gain in bits over row scan under the same propagation; mean, standard deviation over boards, range, boards won.

| rule | 5x5, 3 colours, 12 boards | 6x6, 5 colours, 12 boards | 7x7, 7 colours, 3 boards |
|---|---|---|---|
| FC-ROW over no lookahead | +0.53 sd 0.08 | +0.15 sd 0.08 | not run |
| FC-Y2 | +0.19 sd 0.07 [+0.07, +0.29] | +0.10 sd 0.05 [+0.01, +0.19] | not run |
| FC-B2 | +0.85 sd 0.20 [+0.55, +1.20] | +0.46 sd 0.13 [+0.31, +0.77] | not run |
| FC-DOMF | +2.87 sd 0.71 [+1.81, +3.84] 12/12 | +1.29 sd 0.69 [+0.41, +2.83] 12/12 | -0.03 sd 0.47 [-0.50, +0.45] 1/3 |
| FC-DOM | +3.50 sd 0.56 [+2.58, +4.28] 12/12 | +2.10 sd 0.96 [+1.06, +3.92] 12/12 | not run |
| **AC-ROW over no lookahead** | **+4.68 sd 0.42** | **+4.80 sd 0.38** | not run |
| AC-Y2 over AC-ROW | +0.08 sd 0.06 | +0.11 sd 0.08 | not run |
| AC-B2 | +0.27 sd 0.13 [+0.14, +0.52] | +0.33 sd 0.22 [+0.11, +0.89] | +0.12 sd 0.09 [+0.03, +0.21] 3/3 |
| AC-DOMF | +0.46 sd 0.32 [+0.01, +0.88] | +0.41 sd 0.40 [-0.01, +1.26] | +0.08 sd 0.08 [0.00, +0.16] 3/3 |
| AC-DOM | +0.61 sd 0.27 [+0.12, +0.95] 12/12 | +0.67 sd 0.66 [-0.18, +1.89] 11/12 | -0.37 sd 0.36 [-0.78, -0.09] 0/3 |
| AC-WDEG | +0.06 sd 0.29 [-0.27, +0.51] 6/12 | +0.50 sd 0.50 [-0.15, +1.51] 11/12 | -0.39 sd 0.36 [-0.80, -0.14] 0/3 |
| L2 (5 boards, the stage was stopped with its batch) | not run | FC +0.39 sd 0.19; AC +0.88 sd 0.16 | not run |

7x7 exact is at 7 interior colours (1.4 above critical), because the trees at 6 colours are 2^28 to 2^33 and do not fit the 15 minute limit at my solver's speed. No run was censored.

**Estimated at the ladder count of each size** (M = 2,000, 2 runs per board and variant; 8 boards at n = 7 and 8, 6 at n = 9, 5 at n = 10), with the exact rows of n = 5 and 6 and the official 16x16 for comparison. Mean paired gain over row scan, standard deviation over boards in brackets.

| n (interior colours) | FC-DOM | FC-DOMF | FC-B2 | FC-Y2 | AC-DOM | AC-DOMF | AC-B2 | AC-Y2 | AC over none |
|---|---|---|---|---|---|---|---|---|---|
| 5 (3), exact | +3.50 (0.56) | +2.87 (0.71) | +0.85 (0.20) | +0.19 (0.07) | +0.61 (0.27) | +0.46 (0.32) | +0.27 (0.13) | +0.08 (0.06) | 4.68 (0.42) |
| 6 (5), exact | +2.10 (0.96) | +1.29 (0.69) | +0.46 (0.13) | +0.10 (0.05) | +0.67 (0.66) | +0.41 (0.40) | +0.33 (0.22) | +0.11 (0.08) | 4.80 (0.38) |
| 7 (6) | +0.73 (0.76) | +0.82 (0.64) | +0.33 (0.17) | +0.01 (0.17) | +0.06 (0.62) | +0.24 (0.27) | +0.21 (0.14) | +0.07 (0.13) | 4.73 (0.46) |
| 8 (7) | -1.33 (0.90) | -0.19 (0.50) | +0.25 (0.14) | +0.06 (0.10) | -0.97 (0.84) | -0.07 (0.35) | +0.14 (0.19) | +0.04 (0.14) | 4.47 (0.37) |
| 9 (9) | -8.29 (1.36) | -1.98 (0.84) | +0.04 (0.27) | -0.10 (0.13) | -4.62 (0.70) | -0.57 (0.29) | +0.10 (0.18) | -0.14 (0.20) | 4.57 (0.36) |
| 10 (10) | -11.22 (2.20) | -4.78 (0.71) | +0.23 (0.42) | +0.09 (0.45) | -6.76 (1.45) | -1.12 (0.43) | +0.03 (0.06) | -0.01 (0.13) | 4.69 (0.40) |
| 16, official | -36.6 | -49.3 | +0.35 | -0.7 | -28.9 | -12.8 | +0.23 | -0.08 | 5.5 |

**Trend.** The free rules gain on the smallest boards and lose more with every size: smallest domain anywhere goes from +3.5 to -11.2 bits between n = 5 and 10 under forward checking and from +0.6 to -6.8 under arc consistency, monotone, with the spread across boards well under the change per size from n = 7 on. The frontier rule follows two sizes later. The rules that stay in a band go to zero from above (+0.85 to about +0.2 under forward checking, +0.3 to about 0 under arc consistency). The official 16x16 continues each column. "Dynamic orders matter more as instances get harder" does not hold here in the direction of a gain: nearer critical the small-board gains are larger than the memo's (its DOM was +0.06 and +0.10 at 6x6), but the slope with size is negative and steeper. The reason is the one the memo gives: a smallest-domain rule opens fronts, and on a large board each open joint costs 4 bits while the small domains it chases are worth a fraction of a bit.

## 4. Point 3: dynamic order under arc consistency, more than additive?

The reason to expect more: under forward checking a domain size reflects only the decided neighbours, while under arc consistency it carries implications from further away, so the smallest domain is a better predictor of failure. Beck, Prosser and Wallace find exactly this on random problems (more fail-first pays under MAC, not under forward checking). The reasons to expect less: arc consistency already removes the nodes a good choice would have killed, and on this problem its reach is short (a cell with one decided neighbour keeps about 46 values covering every colour on every side, so nothing propagates through it).

Measured, it is less than additive everywhere:
- Toys, exact: the gain of DOM under arc consistency is 0.61 against 3.50 under forward checking at 5x5, 0.67 against 2.10 at 6x6; B2 0.27 against 0.85 and 0.33 against 0.46. The one exception is the lookahead rule at 6x6 (0.88 under arc consistency against 0.39), a rule that spends extra propagations to choose.
- Official 16x16, estimated: Y2 -0.08, Y3 +0.05, B2 +0.23 bit under arc consistency (runs within 1.3 bits of each other, so these are zero within about half a bit), lookahead about +0.7 on two runs at M = 600 (a third run hit its time limit and is discarded).
- Arc consistency itself: 5.5 bits on the official pieces, 4.5 to 4.8 on the ladder, no slope.

So the combination the memo left open prices at 0 to 1 bit on top of arc consistency's 5.5.

## 5. Point 4: conflict-driven orders

**My toy measurement (exact).** Domain over weighted degree under arc consistency, weights kept over the whole enumeration, no restarts: +0.06 bit at 5x5 (wins 6 of 12), +0.50 at 6x6 (11 of 12), -0.39 at 7x7 with 7 colours (0 of 3). Same shape as the other free rules, on few boards at the largest size. The population estimator cannot run a rule whose choice depends on the history of the search, so there is no 16x16 figure. The lemma of section 2 covers any such rule confined to one or two rows at a time.

**Literature, opened by me or by a helper under my instruction (each marked).**
- Ansótegui, Béjar, Fernández, Mateu, "How Hard is a Commercial Puzzle: the Eternity II Challenge" (an earlier version of their CP 2008 and Constraints 2013 papers; text extracted and read by me): https://repositori.udl.cat/server/api/core/bitstreams/0b6533fe-54e5-4070-85fe-80f7d35837d8/content. Table 2, median seconds over 100 generated framed instances near the phase transition, look-ahead solver with LEX, DOM (minimum domain) or CHESS (a static chessboard spiral). One colour set shared by frame and interior: 7x7 LEX 235, DOM 20, CHESS 42; 8x8 LEX over 2e5, DOM 12,125, CHESS 52,814. Separate frame colours (the Eternity II type), DOM against CHESS: 6x6 15 against 0.5; 7x7 520 against 5,249, 18,193 against 137, 9,464 against 4,181, 581 against 6,906, 8,387 against 510 depending on the colour split. Their best solver on the two-set boards is MAC with a colour all-different filter under the static CHESS order; MAC with dom/deg and Minion are the slowest on most columns. So the published record on edge matching is: minimum domain beats lexicographic by 12 times at 7x7 and by more than 16 times at 8x8 on one-set boards (3.6 and over 4 bits, time to a first solution), and against a good static order on two-set boards the sign depends on the colour split, with factors up to 130 either way. No weighted-degree or other learning heuristic is tested. The published versions were not opened.
- Beck, Prosser, Wallace, "Failing First: An Update" (ECAI 2004; opened by the helper, the memo's link): random binary problems only; "the ranking of the heuristics is not influenced by problem size"; more fail-first helps under MAC and not under forward checking. The memo's reading is fair.
- Michel and Van Hentenryck, "Activity-Based Search for Black-Box Constraint Programming Solvers" (abstract opened by me, tables by the helper): https://arxiv.org/abs/1105.6314. Activity-based search is "more robust" than impact-based search and weighted degree; on magic squares the ratio of weighted degree to activity in choice points falls with size (about 400, 13, 8, 5 at orders 7 to 10). Not a permutation-of-tiles problem.
- Balafoutis and Stergiou, "Evaluating and Improving Modern Variable and Revision Ordering Strategies in CSPs" (opened by me, first 100,000 characters): https://ar5iv.labs.arxiv.org/html/1008.0659. Conflict-driven heuristics are "in general faster and more robust" than impact-based ones on frequency assignment, random, Boolean and a few structured classes. No quasigroup, tiling or edge-matching class, and no comparison with a static order.
- Not opened (404, access denied or not found): Boussemart et al. 2004 (the dom/wdeg original), Refalo 2004, Heule 2008, Benoist and Bourreau 2008, Schaus and Deville 2008. Nothing is claimed from them.

What the literature supports: on small one-set boards a smallest-domain rule is worth 4 bits over a lexicographic order in time to a first solution, which matches my 5x5 and 6x6 rows; nothing published reaches the sizes where my ladder turns negative; nothing published tests a learning order on edge matching. "A constant factor" is the memo's guess, not a finding.

## 6. Point 5: the reading of D5

Checked against `wt-d5/research/experiments/pin-map/results/mrv8.jsonl` (`d5_check.py`).
- 10 unpinned bb-mrv runs, 10 censored at 100,000 nodes. The memo is right, and so is its note that bb-mrv counts options tried.
- The cap is 2^16.6 and the row-scan trees of the same boards are 2^23.0 to 2^28.1. A solver 6 to 11 bits better than row scan would still have been censored. The runs carry no information about unpinned boards in either direction; "censored" should not be read as "no better".
- On the pin sets where both solvers finished, log2(row scan) - log2(bb-mrv) has median +3.8 at 1 pin (23 pairs, 87 runs censored, which flatters bb-mrv), +7.3 at 2, +8.4 at 3, +9.3 at 4, +9.9 at 6, +8.1 at 8, +4.1 at 12. D5 did not separate propagation from order. I did, on eight 8x8 boards with 11 interior colours and five clues in D5's clue cells (exact, `clues8.jsonl`): row scan without lookahead 2^21.2, row scan with arc consistency 2^6.0, smallest domain with arc consistency 2^5.9. Arc consistency is worth 15.2 bits there and the order 0.1. At 7 interior colours, near critical, on six such boards (estimated, M = 2,000): arc consistency is worth 7.4 bits, and on top of it the frontier rule gains 4.7 bits (sd 0.95, 6 of 6 boards), smallest domain anywhere 1.3, the band rule 0.3. This is the one setting in this report where an adaptive order is worth several bits: a small near-critical board with one clue per 13 cells, where every empty cell is close to a clue. On the official board, one clue per 51 cells, the same frontier rule is 12.8 bits worse than row scan.
- So D5's item 4, at its colour count, is a propagation result: "arc consistency uses pins wherever they are". An order effect of several bits appears only near critical with dense clues. The memo's caution about clue density (every cell within three cells of a pin at 8x8, not at 16x16) is borne out: on the official board the rules that start from the clues are the worst I measured.

## 7. Smaller points

- The memo's table of toys mixes boards with four symmetric solutions and no pin; mine fix one corner. The differences between our 6x6 rows come from the colour count, not from this.
- Memo 3.4 point 1 guesses a ceiling of about 5 bits for selection. The block lemma gives 5.3 bits as a proved cap for the one-row case under forward checking, so the guess was a good one, for a reason the memo did not have.
- Memo section 6, experiment E2 (one step at depth 144 on planted boards) is superseded by the full-depth estimates on the official pieces. E1 is partly done here (sizes 5 to 10, rules ROW, Y2, B2, DOMF, DOM; not TILE, EDGE, MIX; weighted degree to 7x7 only).
- The memo's appendix reports its own overrun of the time limit. No process of mine reached 15 minutes. The longest, the lookahead estimate under arc consistency, was cut by its own 600 s cpu limit during its third run (that run is discarded). I stopped two things by hand: an 8x8 DOMF enumeration after about 4 minutes (no row written), and the first 16x16 batch once it had moved on to runs I no longer needed (its rows are kept in `real16.jsonl`).
- I reran the memo's `model16.py`: output identical to `model16.out`, 5 seconds.

## 8. Left open, worth a study or not

1. **Learning orders at size (worth a study, small).** Weighted degree, with and without restarts, exact at 8x8 and 9x9 at the ladder counts with a compiled engine, 20 boards, against ROW under arc consistency. My three 7x7 boards say -0.4 bit; the lemma caps any one-row or two-row variant at 4.0 and 6.5 bits. Kill line as in the memo's E1. This is the only untested family.
2. **The three-row staircase under forward checking on the official pieces (not worth a study by itself).** +2.85 bits, unexplained, absent on a generated board and under arc consistency. If someone wants the mechanism: the level profile of Y3 against ROW from `dyn.c` with PROFILE set, and Y4 to Y8 to see where it saturates (Y5 is already lower). A forward-checking gain that arc consistency erases is not a route, since arc consistency is worth 5.5 bits by itself.
3. **Lookahead as a rule (fold into the arc-consistency cost question, not a study).** About +0.7 bit under arc consistency for six extra propagations per node. It is a stronger propagation paid at the node, and the program has already priced that trade as a loss.
4. **Tile and colour branching on near-critical boards (not worth a study).** Only the memo's toys, at non-critical counts. The argument (217 expected places for a tile at the peak) is strong and nothing here weakens it.
5. **A proof for sliding-window and free rules.** The lemma covers block rules only. For the rest the evidence is the estimates above. A lower bound for all cell orders would be a result for the thesis, not a route.

## 9. Files, compute, limits

`gap-4-review/`: `dyn.c` (solver), `build.sh`, `crit.py` (colour counts), `run_ladder.sh`, `run16.sh`, `run16b.sh` (every process under `timeout 840`, `nice -n 10`, an internal cpu limit, and a branch cap in exact mode), `analyze_ladder.py`, `analyze16.py`, `analyze_profile.py`, `analyze_valid.py`, `d5_check.py`; raw rows `exact.jsonl`, `smc.jsonl`, `valid8.jsonl`, `clues8.jsonl`, `real16*.jsonl`, `toy16.jsonl`, `profile_*.txt`. Binaries are not kept; `sh build.sh` rebuilds them.

Compute: one core, one process at a time, 63 cpu-minutes in the recorded rows, about 80 with the profile runs, the stopped runs and a few unrecorded probes of under a minute each.

Limits of this report. The 16x16 figures are estimates from populations of 500 to 12,000 partial boards per depth, good to about half a bit for row scan and the arc-consistency rows, and to 2 or 3 bits for the staircase rules under forward checking. The ladder beyond 7x7 is estimated, not enumerated, and rests on 5 to 8 boards per size. 7x7 exact has 3 boards. My arc consistency, like the memo's, ignores Hall sets. Tile, colour and exact-cover branching were not re-measured.
