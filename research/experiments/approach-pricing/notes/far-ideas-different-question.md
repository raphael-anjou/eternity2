# Far ideas: taking the question differently

Desk study, 2026-11-13, lens "change the question, not the search". One core, stdlib Python, under two minutes of compute in total. Scripts and outputs: `far-ideas-different-question/` next to this file (`python3 -I <script>`; they import the loader of `excluded/` and read copies of the Track 10 profile files from commit `dc71ad56`). Grades: **measured** (computed tonight from the real pieces or committed data), **derived** (arithmetic or a model computed tonight), **cited** (from a independently re-checked file of this program, named), **guessed** (my judgment). Nothing here is independently re-checked. Toy boards: none used.

## 0. Verdict

Most reframings in the plan collapse into one statement, which I computed rather than assumed: **a relaxation that keeps the true board among its solutions does not make the true board cheaper to reach, because it multiplies the solutions by about as much as it cheapens each one, unless the solver can tell the planted solution from the others (O2, worth about 1 bit) or enumerate them in compressed form (O1).** Holes, merged colours, fewer clues, endgames and super-tiles each fall to this or to "it is a static order" (sections 2.1 and 2.2). That leaves one family the lens actually opens: **information that sits outside the search**, held by people or printed on cardboard. It costs a letter or a photograph rather than core-years, and it is the only idea here I would bet on (section 2.3). My odds are in section 4. None of them is above the program's 1 percent prior by much, and most are far below it.

## 1. Twenty ideas, three lines each

Each item gives the reframing, what it would buy, and the first reason it probably fails.

1. **One bit for less than the search (decision before search).** Ask "is tile t at cell c, rotation r?" and self-reduce: about 56 such answers would bring the search within budget. Fails because a NO for a wrong pin costs the pinned tree, about 2^(134.4 - w), where a pin is worth w = 5.8 bits at the median cell and 10.05 at the best (cited, D3 Part B). The cheapest single bit therefore costs about 2^124 (derived). The only cheap bits known are the 17.45 corner bits, and those are already priced (cited, frame-information).

2. **A cheap certificate: an invariant of the solution checkable without it.** Colour counts, parities and balance laws would filter candidates. Fails because every invariant computable from the piece set is a function of the piece set, so it carries zero bits about the arrangement (cited, path 2 theorem). Invariants of the arrangement are not computable without the arrangement.

3. **Count instead of search: solutions mod 2 with a pinned tile (algebraic sieving).** Parity of a count answers one bit of the question in item 1. Fails because inclusion and exclusion over tile subsets has 2^256 terms. Determinant tricks need a bipartite structure that the adjacency constraints break (cited, gap 6a and path 1). This is gap 6a's "injective-only hole" seen from the decision side, and it remains unpriced.

4. **2x2 super-tiles: solve a 7x7 (interior) or 8x8 puzzle of blocks.** It would trade 4 tile levels for 1 block level with a precomputed table. Fails because it is a static order plus a lookup table. Section 2.2 computes it: block scan is 2.5 bits worse than row scan, and the lookup saves at most 1.9 bits, which row scan gets too (derived).

5. **Solutions with a hole as stepping stones.** Hole-solutions are cheap and plentiful: a hole of the last 36 to 41 cells gives one per 10^15 nodes. Fails because the planted board minus the hole is then 1 among about 10^25 of them, so the price is conserved exactly (section 2.1, derived from the measured profile).

6. **Find any of the about 14,702 starter-only solutions first.** It would hand over a full valid board. Fails because the measured trees give 10^42.97 nodes per starter-only solution and 10^42.98 per no-clue solution, against 10^40.46 for the five-clue one (derived from measured totals and cited counts). A random solution is also far from the planted one (cited, path 4: 246 or more cells apart).

7. **Merge colours (17 to 9) into an easy puzzle with many solutions, then refine.** The merged puzzle is far easier per solution. Fails on the same conservation: each merged joint matches with probability 1/9 instead of 1/17, so the merged puzzle has about (17/9)^420 = 10^116 times more solutions (derived, uniform model), and the true board is one needle among them.

8. **Split colours or double the tile set (512 tiles, two boards).** A larger symmetric puzzle sometimes has a cheaper path. Fails because grey rims forbid gluing two copies, and doubling adds solutions without adding constraints on the original. There is no map from solutions of the bigger puzzle to the original that is not the original search.

9. **The same tiles on a torus or another shape.** It would remove the frame bottleneck. Fails because 60 tiles carry grey, so no other shape shares the answer. A torus puzzle of the 196 inner tiles has a different, much larger solution set.

10. **The dual puzzle on vertices, or tiles cut into four triangles.** Different variables, perhaps better propagation. Fails because four sides identify one tile, so edge or vertex search is tile search (cited, gap 6b). Cutting into triangles drops "each tile once", which is the colour-only contraction (O1).

11. **Work backwards: enumerate end games and meet them.** The last 60 cells have only about 10^25.6 fillings, by the symmetry of row scan (W at free depth 60, measured). Fails because this is the two-list join, which is about 3 bits (2^131.6) at its best split (cited, gap 2 review). The prefix list must cross the peak at depth 158 wherever the meeting point is put.

12. **Near-solutions under another measure: valid everywhere except k cells left empty in the middle.** A compact hole is a softer target than a record board. Fails because a hole placed before the peak widens every later level: the tree grows by 6 to 48 bits for 1x1 to 6x6 holes (derived, model). Placed after the peak, it is item 5.

13. **A public bank of proven-dead frames, corner blocks or 2-row bands, searched once by the world.** It would make refutations permanent. Fails because one exhaustive search never revisits a subtree, so a bank helps only duplicated stochastic searches (record hunting, not solving). The frame table alone would hold about 10^32 entries after the corner test (derived: 10^37.6 frames x 2^-17.45).

14. **The volunteer network as the world's one search.** A BOINC fleet at the reported 886 G nodes per second (cited, ROUTES row) does 2^39.7 nodes per second, so 2^68.6 in ten years. Fails because it stays 66 bits short (derived), and banking does not help (item 13).

15. **Consensus of the record boards.** Placements shared by many 470 boards might be shared with the solution. Fails because records sit 246 or more cells from any solution (cited, path 4), and their consensus reflects the engines' biases.

16. **Planted-ness: the true board is atypical among the solutions of a relaxation.** It would tell the needle apart in items 5 to 7. Fails because the only measured tilt is about 1 bit, in one scan direction (cited, D4).

17. **Guess the generator and its seed.** Excluded by the plan as already priced (cited, how-the-puzzle-was-made section 6, about 1 in 1,000 by its author and struck as a guess by its second analysis). Listed only for completeness.

18. **The physical sheets.** If the pieces were laid out on the punch-out sheets in the generator's order or orientation before the sort, the layout is a trace of up to 1,850 bits. Fails because numbers on the backs must register with fronts, so a printer lays out by number. The community took its orientation convention "from the punch-out panels" (cited, digest msg 977 via how-the-puzzle-was-made), and the stored orientation is canonical, which suggests sorted, canonical sheets. A photograph settles it (section 2.3).

19. (Removed: contacting third parties for part of the reference board; see the README.)

20. **Physical or analogue parallelism (molecules, optics).** It would get past serial node rates. Fails because 2^134 states exceed Avogadro's number (about 2^79) by 55 bits.

21. **Solve the frame only, as a yes or no per candidate frame.** A cheap NO for wrong frames is a frame search. Fails because it is Track D1's question exactly (RQ1), not new.

## 2. The top three, developed

### 2.1 Solutions with a hole, and the conservation it exposes

**The reframing.** A hole-solution is a board valid everywhere except on a set H of empty cells, with |H| tiles left over. The planted board minus H is always one of them. If hole-solutions are cheap and the planted one can be reached from a hole-solution, the hole is a stepping stone.

**First-moment count, compact interior hole** (derived, `03_first_moment_orders_holes.py`). Relative to full solutions, a hole of k inner cells multiplies the expected count by 17^e(H) / (k! 4^k), where e(H) is the number of joints touching H. For an s x s square, e = 2s^2 + 2s. On the five-clue instance, the expected random hole-solutions per placement of H (uniform model with the real q = 0.058901, anchored to the independently re-checked 4.1e-8 for full boards) are:

| hole | k | log10 factor | expected random hole-solutions (five clues) |
|---|---|---|---|
| 1x1 | 1 | 4.32 | 10^-3.1 |
| 2x2 | 4 | 10.97 | 10^3.6 |
| 3x3 | 9 | 18.5 | 10^11.1 |
| 4x4 | 16 | 26.3 | 10^18.9 |
| 5x5 | 25 | 33.6 | 10^26.2 |
| 6x6 | 36 | 40.1 | 10^32.7 |

These are from the model's increments added to -7.39. The model's own absolute full-board figure is off by 5.6 digits, so these rows are increments on the independently re-checked anchor and carry that uncertainty. From a 2x2 hole on, random hole-solutions outnumber the planted one per placement.

**Price, by where the hole sits** (derived, model, five clues). A hole placed before the peak relaxes every later level. Total nodes rise by +8.1 (1x1 early) to +48 (6x6 early) bits, and by +6 to +10 bits for holes in the peak rows. Placed after the peak, the tree is unchanged and only the count of targets rises. The best case is the hole made of the last m scanned cells. Then the hole-solutions are exactly the valid partial boards at free depth 251 - m, and their number is the **measured** level width (`04_profile_late_hole.py`):

| hole = last m free cells | number of hole-solutions | nodes per hole-solution, about T / W |
|---|---|---|
| 61 | 10^35.53 (measured) | 10^4.9 |
| 75 | 10^37.85 (measured) | 10^2.6 |
| 36 to 41 | about 10^25.3 (measured to depth 191, extrapolated with the model, two shifts) | 10^15 |

So hole-solutions become affordable at a hole of about 36 to 41 cells, and at that size the planted board minus the hole is one of about 10^25.3. A blind search reaches it after about 10^25.3 x 10^15 = 10^40.3 nodes: the original price, to the precision of the extrapolation.

**Stepping stone.** Under the first moment, a hole-solution shares the planted prefix of depth d with probability about 1/W(d): 10^-14.3 at d = 16 (measured width). The found boards share nothing with the planted one beyond chance, so there is no gradient from a hole-solution towards it. Path 4's empty band (no boards with few defects at 65 to 160 changed cells) says the same for repair.

**The general statement this checks** (derived, first moment). Let R be any relaxation whose solution set contains the planted board, and let the solver have no statistic separating the planted board from the rest. Then reaching the planted board costs about T_R, the full R-tree. When R only removes constraints, T_R is at least T up to a re-ordering of levels. Holes (+0 to +48 bits), fewer clues (+2.5 digits per solution), merged colours and end-game lists are instances. The two ways out are the two obstructions already named: a statistic (O2, about 1 bit) or compressed enumeration (O1).

**What would have to be true to cut 10 bits.** Hole-solutions near the planted one would need to be more numerous than first moment predicts by 2^10. That is a clustering or condensation effect around the planted solution, which planted CSPs show only below their condensation threshold. Path 4 and A10 measured the opposite sign on small boards.

**Smallest experiment and kill line.** On 6x6 critical-ladder boards with known planted solutions (toys), enumerate all hole-solutions with the last 6 cells empty. Measure the distribution of their overlap with the planted board, against the first-moment 1/W(d) curve. Kill if the share agreeing on more than half the cells is within a factor 4 of the null. Cost about 1 core-hour. I would not run it: the result can only move the model, not the price on the 16x16.

### 2.2 2x2 super-tiles

**Measured.** The 196 inner pieces form **4,059,952** valid ordered 2x2 blocks (1,014,988 up to turning the block), all four inner joints matched, four distinct pieces (`01_blocks_2x2.py`). The naive first moment is 4,409,480. Boards painted from the official colour pools give 4,066,387 +/- 11,772 (60 boards, `02_blocks_null.py`), so the official set is ordinary (z = -0.55). All 289 two-colour rims occur. A given (left rim, top rim) pair admits 48.6 blocks on average before distinctness.

**As a puzzle.** The 14x14 interior is exactly a 7x7 of blocks with 289 super-colours. The solution count is unchanged (a bijection). Search over blocks in row-major block order is a static cell order with intra-block levels skipped by table lookup. In the uniform first-moment model (derived, `03`):

| order | log10 total nodes | bits vs row scan |
|---|---|---|
| row scan (model) | 42.05 | 0 |
| 2x2 block scan, all levels | 42.80 | +2.49 |
| 2x2 block scan, block levels only (table) | 42.27 | +0.72 |
| row scan, every 4th level only (same lookup on 4-cell runs) | 41.49 | -1.87 |

The model is 1.6 digits high in absolute terms against the measured 40.46. Only the differences are used, and they agree with gap 6b's "row scan within 0.24 bit of any static order". The super-tile view is worth at most the 1.9 bits of collapsing levels into lookups, and row scan can take that without blocks. As a 64-piece puzzle of given super-tiles it does not exist: the partition of 256 tiles into blocks is the unknown, chosen from about 10^6 candidate blocks.

**What would have to be true to cut 10 bits.** The planted solution's 49 blocks would need to be distinguishable from the other 10^6. That is O2 at the block level. A1 (spectral pair statistics) killed the pair version.

**Smallest experiment and kill line.** None worth running. If anyone insists: on the 16x16 model with the real piece counts, price the best block order with gap 6b's annealer, and kill at under 5 bits. My expectation is a negative number.

### 2.3 Information from outside the search (passage removed)

This passage discussed obtaining parts of the reference board from third parties. It is not part of the published record; the three prices that depend on it are carried by the pin study cited in `question-and-numbers.md`.

## 3. What I computed

| # | computation | result | grade | script |
|---|---|---|---|---|
| 1 | valid 2x2 blocks of the inner pieces | 4,059,952 ordered (1,014,988 up to rotation); first moment 4,409,480; painted null 4,066,387 +/- 11,772, z = -0.55 | measured | `01`, `02` |
| 2 | block scan against row scan, uniform first-moment model | +2.49 bits all levels; +0.72 with table lookups; row scan with the same lookup -1.87 | derived (model, relative only) | `03` |
| 3 | nodes per solution with fewer clues | five clues 10^40.46; starter only 10^42.97; none 10^42.98 | derived from measured totals and cited counts | `04` |
| 4 | late hole needed for one hole-solution per 10^15 nodes | last 36 to 41 cells; then about 10^25.3 hole-solutions, so the planted one costs about 10^40.3 | derived (measured profile to depth 191, model past it) | `04` |
| 5 | compact holes: count factor and tree growth | 2x2 hole: 10^3.6 random hole-solutions per placement; tree +8 to +48 bits for early holes of 1x1 to 6x6 | derived (model) | `03` |
| 6 | price of one decision bit | about 2^124 at best (2^(134.4 - 10.05)), 2^128.6 at the median cell | derived from cited D3 figures | none |

Model caveat: my uniform first-moment model is 1.6 digits high on the five-clue total and 5.6 digits low on the full-board expectation. The independently re-checked model handles the flat colour quotas; mine does not. I use it only for differences between orders and for hole increments, and I anchor absolute counts to the measured profile or to the independently re-checked 4.1e-8.

## 4. Odds (guessed)

- Every computational reframing in section 1: under 0.1 percent each for 10 bits, by the conservation argument in 2.1. For items 1, 3 and 21, the residue is exactly the open questions D1 and gap 6a already hold, not new odds.

## 5. Not checked

- The late-hole extrapolation past free depth 191 rests on a model whose shift changes by 7 digits between depth 191 and 251. The bracket 36 to 41 cells is model-dependent; the conservation conclusion is not.
- Block-order figures come from my uniform model, not the independently re-checked one.
