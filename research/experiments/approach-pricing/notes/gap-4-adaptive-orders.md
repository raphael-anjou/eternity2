# Gap 4: fill orders that adapt to the board

> **Status (2026-11-13):** independently re-checked, see `notes/gap-4-adaptive-orders-recheck.md`.
> **Re-check's rulings:**
> - Verdict "empty as far as ordering goes": stands, with a narrower scope. For smallest-domain cell rules (free, frontier, band, staircase, one-step lookahead), under forward checking and arc consistency, on the official pieces with the five clues, every rule is within about 3 bits of row scan or far worse (13 to 49 bits worse for the free rules). Rules that finish rows one or two at a time cannot gain more than 5.3 and 6.7 bits. Not covered: weighted-degree and other learning rules at size, restarts, tile and colour branching beyond the memo's argument.
> - "The best any adaptive cell order can do with forward checking is about 1.1 bits": wrong as a bound, right as an order of size. 1.1 is the best static staircase in the model. Estimated on the official pieces under forward checking: band +0.35 bit, lookahead +0.6, two-row staircase -0.7, three-row staircase +2.9 (not reproduced on a generated 16x16, +0.45, and zero under arc consistency).
> - Minimum over staircases with up to 4 open rows: stands on the re-check's measurements, not on the memo's argument (non-staircase band rules are within 0.6 bit of row scan, free rules 13 to 49 bits worse).
> - "Annealed min-domain order: 2^135.5, 1.0 bit worse than row scan": wrong as a price of the smallest-domain rule. The actual rule on the official pieces is 2^170.5 with forward checking (36 bits worse) and 2^157.8 with arc consistency (29 bits worse).
> - Toys (staircase within 0.3 bit, free rules gain at 6x6 and lose at 7x7, 8x8): stands with a narrower scope; near critical the crossover is later and the slope steeper (smallest domain, forward checking: +3.5, +2.1, +0.7, -1.3, -8.3, -11.2 bits at n = 5 to 10).
> - Dynamic order under arc consistency at 16x16, "no model, no measurement": now measured, no sign of more than additive (two-row -0.1, three-row +0.05, band +0.2, lookahead +0.7, frontier -12.8, anywhere -28.9 bits against row scan).
> - Conflict-driven rules, "a constant factor": unproven, one toy trend against, nothing at size (weighted degree under arc consistency, exact: +0.06 at 5x5, +0.50 at 6x6, -0.39 bit at 7x7). "Nothing found that prices a dynamic order on edge matching at the critical density" is wrong: Ansotegui et al. do.
> - D5: stands with two additions; the cap (2^16.6) sits 6.3 to 11.5 bits under the row-scan trees, so the censored runs say nothing either way. Tile-centred and colour-centred branching (0 bits): stands as argued, not re-measured.
> - Odds (1 percent for 10 bits): stands for domain-size rules; the second analysis would not go below 2 percent overall (learning rules at size are untested).
> **Read with:** `notes/gap-4-adaptive-orders-recheck.md` (rulings 2, 4, 7, 8 replace figures in the memo's verdict and table 3.3).

Draft, not independently checked (2026-11-13, afternoon). Each statement carries its grade: measured (exact counts on toy boards, this memo's scripts), derived (first-moment model, script given), cited, guessed. Toy boards are generated boards, not Eternity II.

## Verdict

**The gap is empty as far as ordering goes.** In the first-moment model with the real colour counts and the five clues, the best any adaptive cell order can do with forward checking is about **1.1 bits** below row scan (2^133.4 against 2^134.5, derived). Tile-centred and colour-centred branching price at **0 bits** at the peak (derived). On exact toy trees, the staircase rules stay within 0.3 bit of row scan from 6x6 to 8x8, and the free rules (smallest domain anywhere, exact-cover, tile, colour) gain at 6x6 and lose 1.2 to 2.6 bits at 7x7 and 8x8, with and without arc consistency (measured, few boards). The only gain of size in this neighbourhood is arc consistency itself, about 4 bits on the toys and 5.2 bits in the program's 16x16 arithmetic. That is a lookahead, it is already priced in the log, and it does not come from the order.

## 1. The gap, and what the evidence covers

Priced so far: static orders only. A static order is a chain of cell sets S_1, S_2, ...; level k has N(S_k) nodes, the number of valid fillings of S_k; the model gives row scan 40.5 digits and the others about 10 more (independently re-checked). The growth-law second analysis wrote that no argument covers a dynamic order (C1), and the learning survey said the same.

What the cited evidence does and does not say:

- **D5 (pin map).** It measures the worth of pinned tiles. Under bb-mrv the placement of 6 or more pins stops mattering at 8x8. It contains no comparison of unpinned trees: all 10 unpinned bb-mrv runs hit the 1e5 cap, and the independent assessor notes that bb-mrv counts a different quantity. So D5 shows that an adaptive solver uses pins wherever they are. It does not show that an adaptive order shrinks the tree of a board without extra pins. Six pins on 64 cells also put every cell within about three cells of a pin; on 256 cells with five clues that is not the case.
- **Arc-consistency follow-up of D4.** Survivor shares 0.43 at depth 96 down to 0.001 at 156. This is a propagation result on row-scan prefixes. It says nothing about which cell to fill next.
- **ROUTES rows touched:** "Other fill orders" (static only, weak evidence), "Propagating DFS lead" (flagged, lead not growing with n; plain DFS faster than bb-mrv at 9x9), "Exact cover / DLX" (claim without data), "Value order by impact" (killed), and the untried items A18 and E1 (evolved ordering). New here: a priced model for dynamic orders, tile and colour branching included, and exact toy counts for nine adaptive rules against row scan under the same propagation.

## 2. Literature

- Haralick and Elliott (1980), the fail-first principle: choose the variable most likely to fail. Not opened (unverified).
- Beck, Prosser, Wallace, "Failing First: An Update" (ECAI 2004), opened and read: https://www.dcs.gla.ac.uk/~pat/cpM/weekByWeek/week5/ff-again.pdf. Random binary problems, 20 to 70 variables. Four heuristics of increasing fail-firstness. Under forward checking, trying harder to fail first does not reduce effort; under MAC it does. Their figures 2 and 3 show the heuristics as near-parallel lines of log effort against size, and they state that size does not change the ranking. My reading of the figures (guessed): these heuristics move the constant, not the slope.
- Knuth, "Dancing Links" (2000), abstract opened: https://arxiv.org/abs/cs/0011047. The search switches between placing a piece and covering a square "according to a combined heuristic". This is the mixed cell and tile rule of the plan (MIX below).
- Pesant, Quimper, Zanarini, "Counting-Based Search" (JAIR 2012), abstract opened: https://arxiv.org/abs/1401.4601. Branch on the choice that keeps the largest share of solutions of each constraint. Nearest published relative of using an expected count as a branching score. No edge-matching test claimed in the abstract.
- Boussemart, Hemery, Lecoutre, Sais, "Boosting systematic search by weighting constraints" (ECAI 2004), dom/wdeg. Not opened (the PDF link returned 404); bibliographic record only (unverified).
- From the surveys of this arc, not reopened by me: Ansótegui et al. 2008 (MAC with colour filter is the reference solver; all-different nearly inert), Edmonds et al. (planted jigsaws: the static order changes the exponent, row-like from a corner is best).

Nothing found that prices a dynamic order on edge matching at the critical density.

## 3. The model

Notation: q = 17 interior colours, R free interior tiles, mu_j = 4R / q^j the expected number of candidates of an interior cell with j decided neighbours. At the peak of row scan mu_2 is 1.01 (derived, `model16.py`).

**3.1 Static part (independently re-checked model, reused).** N(S) depends on S only: tiles chosen times the probability that every joint inside S matches. For a staircase set (full rows, then partial rows of decreasing length) with k cells, r started rows and 16 columns, the number of joints inside is 2k - r - 16. So N(S) is the same for every staircase with the same k and r, and each extra started row costs one joint, a factor 17. A set of 158 cells on a 16-wide board has at least 16 open joints whatever its form (isoperimetry of the grid), so no form beats the row-scan prefix on N(S). Check: the minimum of N(S) over all staircases with up to four open rows gains 0.03 bits (derived).

**3.2 Forward checking.** A node is alive if every empty cell has a candidate. In the model the candidates of different frontier cells are near-independent Poisson counts, so

A(S) = N(S) x product over empty cells c touching S of (1 - exp(-mu_c)), with mu_c = N(S + c) / N(S).

Cells with one decided neighbour have mu near 17 and contribute nothing. Each staircase corner contributes 1 - exp(-mu_2), which is 0.64 at the peak. Row scan has one corner. **The corner trade:** one more simultaneous corner needs one more open row, costs a factor sqrt(17) = 4.1 in fillings on average over the row cycle, and buys a factor 0.64 in survival. Net 2.6 times worse per extra corner at the peak (derived). Up to five corners are free for a few cells just after row scan starts a new row (when the total deficit of the open rows stays under 16), which is where the small gain comes from. A cell with three decided neighbours is not a gain either: A of the notched set equals N of the set with the notch filled.

**3.3 First moment with a minimum over branching choices.** L = sum over k of the minimum of A(S) over the shapes with k cells. This is what the tree would cost if every branch sat on the best shape at every depth.

| 16x16, real colour counts, five clues on the board, rows bottom-up | log10 | bits |
|---|---|---|
| Row scan, all valid fillings | 40.49 | 134.5 |
| Row scan, forward-checking survivors | 40.27 | 133.8 |
| Minimum over staircases, up to 4 open rows | 40.16 | 133.4 |
| Annealed min-domain order (always the cell with the smallest expected domain, anywhere) | 40.79 | 135.5 |

So the prediction for a dynamic cell order with forward checking on the 16x16 is a gain of 1.1 bits at most over plain row scan, 0.4 over row scan with forward checking (derived). The winning shapes near the peak are row scan with one row left one cell short.

**3.4 Where the argument breaks.**

1. **Selection.** L is a lower bound for static orders in the model. It is not a bound for adaptive ones: each branch picks its own cell, and the mean of a minimum is below the minimum of the means. What limits it: the cell not chosen keeps its domain and must be paid before its row closes, and the number of valid fillings of any set the search completes does not depend on the order. Deferral gains only on branches that die in the meantime, and it can only span the open rows. Guessed ceiling: log2 of the number of cells that can be deferred, about 5 bits for three open rows. Measured on toys: 0.1 bit at 6x6, negative from 7x7 (section 4).
2. **Propagation beyond forward checking.** Under arc consistency the survival share is not a product over frontier cells. The model has no formula for it; it must be measured (D4's shares on row-scan prefixes are the only data).
3. **Annealed against the instance.** The real tree matches the model to 0.03 digits on row scan only. Other shapes are unmeasured on the real pieces (D3 Part A may cover static ones).
4. **Small boards.** At 6x6, 20 of 36 cells are frame cells; rules that exploit frame tiles look good there and nowhere else.
5. **Learning.** Nogoods change what a node is. Out of scope here.

**3.5 Tile-centred branching ("where does tile t go").** The expected number of places for one free interior tile is lambda = (sum of mu_c over empty interior cells) / R. Every empty cell with no decided neighbour offers four places. At the peak lambda is 217 against 1.01 for the row-scan cell; lambda drops under the cell's domain only with 14 rows full (derived, `model16.out`). A free edge tile has 23 places at the peak. "A tile has no place left" has probability about exp(-217) there. So tile branching, and the tile half of an exact-cover rule, is never selected before the last two rows on any compact shape. Making it bite needs every empty cell to have two decided neighbours, for instance alternate rows filled first, and that shape loses about 120 joints. Price: 0 bits.

**3.6 Colour-centred branching (colour of an open joint).** Branching on the colour of a joint of the next cell and then on the tile reaches the same leaves; it merges the tiles that share that colour, a factor (domain size) / (colours present), which is about 1 when mu is about 1. Branching on a joint between two empty cells multiplies the nodes by up to 17 now and divides two later domains by 17: neutral at best in the model. Interior colours are near-uniform (five colours on 48 half-edges, twelve on 50; five frame colours on 24 each), so there is no rare colour to start from (measured on the piece file). Price: 0 bits.

## 4. Toy check (exact trees, generated boards)

Rules, table and limits are in appendix A. Reading (measured, small samples): under forward checking the staircase rules are within 0.1 bit of row scan with two open rows, as the model says, and get worse as more rows are allowed open. Under arc consistency they stay within 0.3 bit (the one 8x8 board gives +0.11 and +0.28). The free rules (DOM, MIX, TILE, EDGE) gain at 6x6, where the frame is most of the board, lose 1.2 to 2.5 bits at 7x7, and DOM and MIX lose 2.6 and 2.5 at 8x8. The sign of the trend with n is the opposite of what a solve needs. Arc consistency is worth 4.3, 4.0 and 4.4 bits at 6x6, 7x7 and 8x8 and 5.2 bits in the program's 16x16 arithmetic: flat in bits, which is what a bounded lookahead gives (re-check's C1). The plain-DFS-beats-bb-mrv observation at 9x9 in ROUTES fits the same picture: a smallest-domain rule opens costly shapes.

## 5. Mechanisms, for the record

None is proposed as a route. Prices on the real 16x16:

1. **Staircase smallest-domain with forward checking (Y2).** Algorithm: among the empty cells whose upper and left neighbours are decided (or walls) and that lie in the first two unfinished rows, fill the one with fewest candidates; fail when any cell has none. Price: at most 1.1 bits (derived). Assumption: frontier domains are independent Poisson counts with the model's means.
2. **Exact-cover smallest-item rule with arc consistency (MIX-AC).** Algorithm: after propagation, count values per cell and places per free tile; branch on the smallest. Price: 0 bits from the tile half (lambda is 217 at the peak); the cell half is rule 1 or worse. Assumption: arc consistency does not cut the places of a tile by a factor of 200 at the peak.
3. **Joint-colour branching with arc consistency (EDGE-AC).** Price: 0 bits. Assumption: domains near the peak hold about one value per colour.

## 6. Experiments that would falsify "empty"

- **E1, the model on small boards, exact.** 8x8, 9x9, 10x10 at a fixed offset above the critical count (critical plus 2: about 9, 11, 12 interior colours), 20 boards each, ROW-AC as baseline, rules Y2, DOM, MIX, TILE, EDGE and dom/wdeg under AC, all solutions, tree size and work both recorded. Also the per-depth profile of ROW-FC against A(S_k) and of Y2-FC against the model minimum, to validate section 3.2 level by level (pass: within 0.3 digits per level, as row scan is). Kill line for this memo: some rule whose paired gain over ROW-AC rises by 1 bit or more per unit of n from 8 to 10, interval excluding zero. That slope would give 8 bits or more at 16. Cost: about 20 core-hours with a compiled engine (the 10x10 AC trees dominate); the design rules of the learning survey apply (score on work, fresh seeds, held-out size, per-depth profile).
- **E2, one step on the real size.** Take D4's sampled arc-consistent prefixes on planted 16x16 boards at depths 128 to 152. For each, compare the number of arc-consistent children of the row-scan cell with that of the best alternative (smallest domain among corners of a two-row staircase, the best tile, the best joint). Kill line: an alternative with under half the surviving children of row scan at depth 144, on average over prefixes. Cost: about 1 core-hour. This tests breaks 1 and 2 where they matter, and it is the cheapest test.

## 7. Odds

That an adaptive order (cell, tile, colour or mixed) yields 10 bits over row scan under the same propagation on the 16x16: 1 percent. That it yields 56 bits: under 1 in 10,000. The 5 bits of arc consistency are real in node counts and already known; at 13 bits of cost per check they are a net loss today.

## 8. Not checked

- Dynamic orders under arc consistency at 16x16: no model, no measurement. E2 is the missing number.
- Conflict-driven rules (dom/wdeg, impact, activity): not tested; they need failures to learn from, and the literature opened suggests a constant factor, but nothing here rules out more.
- 8x8 and beyond with enough boards; boards at the critical colour count.
- Whether an adaptive solver changes the worth of pins on the 16x16 (RQ5).
- Haralick and Elliott and dom/wdeg originals not opened.

## Appendix A: toy ladder

`toy_orders.c`: one propagation core, bitset domains over (tile, rotation), full enumeration of all solutions, tree size = branches tried. Solution counts agree across all uncensored rules on every board. Propagation: PLAIN, FC, AC (arc consistency on colours plus "a cell with one value removes that tile elsewhere"). Rules: ROW; Yk = smallest domain among staircase corners within k open rows; DOM = smallest domain anywhere; MIX = exact-cover rule over cells and tiles; TILE; EDGE = joint with fewest colours.

Bits gained over row scan under the same propagation (positive = smaller tree), mean and range over boards:

| rule | 6x6, 4+6 colours, 10 boards | 7x7, 4+8 colours, 3 boards | 8x8, 4+11 colours, 1 board |
|---|---|---|---|
| ROW-FC over ROW-PLAIN | 0.07 [0.03, 0.17] | 0.02 | 0.00 |
| Y2-FC | 0.10 [-0.04, 0.28] | -0.01 | -0.03 |
| Y3-FC, Y5-FC | -0.05, -0.12 | -0.39, -0.92 | not run |
| DOM-FC | 0.06 [-1.47, 1.85] | under -2.7 (2 of 3 censored) | not run |
| MIX-FC | 0.67 [-0.36, 2.37] | under -2.3 (1 censored) | not run |
| TILE-FC | under -5 (2 boards) | not run | not run |
| **ROW-AC over ROW-PLAIN** | **4.28 [3.87, 4.65]** | **3.96 [3.56, 4.49]** | **4.39** |
| Y2-AC, Y3-AC over ROW-AC | 0.04, 0.06 | -0.03, -0.08 | 0.11, 0.28 |
| Y5-AC over ROW-AC | 0.11 | -0.13 | not run |
| DOM-AC over ROW-AC | 0.10 [-0.87, 1.67] | -1.97 [-2.35, -1.56] | -2.60 |
| MIX-AC over ROW-AC | 1.24 [-0.21, 2.84] | -1.22 [-1.71, -0.70] | -2.52 |
| TILE-AC over ROW-AC | 1.24 [0.33, 3.01] | -2.45 [-2.76, -1.91] | cut by the timeout |
| EDGE-AC over ROW-AC | 0.29 [-0.40, 1.02] | -1.61 (2 boards) | cut by the timeout |

Limits: 7x7 rests on 3 boards and 8x8 on 1; the 7x7 seed 3 run was stopped by hand past the 15 minute limit and lacks its EDGE-AC line; the 8x8 run was cut by its 840 s timeout before TILE-AC and EDGE-AC; the colour counts are 1.4 to 3.7 above the critical counts, so these trees are easier than critical ones; my AC ignores Hall sets; no weighted-degree rule was tested.

## Appendix B: files

`gap-4/model16.py` and `model16.out` (16x16 model; imports the independently re-checked model tables read-only), `toy_orders.c` (toy enumerator), `run_ladder.sh`, `ladder.jsonl` (raw rows), `analyze.py`. Compute used: about 65 core-minutes, of which one process (7x7 seed 3) ran 33 minutes, over the 15 minute limit, and was stopped on the coordinator's instruction; later runs are under `timeout 840`.
