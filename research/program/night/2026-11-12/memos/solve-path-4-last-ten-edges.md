# Solve path 4. The last ten edges

Night 2026-11-12. Lens: what a 470 board is relative to a solution, and whether
the road to 480 runs through near-solutions. Scripts and raw outputs:
`research/program/night/2026-11-12/memos/solve-path-4/` (`01_isolation_profile`,
`02_small_boards`, `03_record_boards`, `04_leftover_ledger`, each `.py` with its
`.out`). Total compute tonight: about 6.5 minutes on one core, stdlib Python.

## 0. Verdict in one paragraph

A 470 board is not a solution with ten defects. By first-moment counting, of
about 10^33 boards at 470 only about 10^9 per solution lie within a few cells
of that solution, a share between 10^-20 and 10^-30; the rest sit 246 to 254
cells (of 256) from every solution. Between "a handful of cells changed" and
"almost everything changed" the counting predicts an empty band: no board with
10 or fewer mismatches is expected at 65 to 160 changed cells from a solution,
by 2 to 12 orders of magnitude. The same counting predicts every committed
rigidity verdict (20 of 20 SAT halo rows) at odds of 10^44 to 10^54 to one, so
rigidity is the null expectation and carries no information about where
solutions are. Consequence: improving 470 boards is a different problem from
solving, every exact neighbourhood of question (c) has an expected yield far
below one board, and no objective on complete boards can have a followable
landscape, because the isolation comes from the piece set and not from the
choice of score. What is left is search over perfect partial boards of the
five-clue instance, which I price at 10^35.6 to 10^40.7 nodes against 10^19
for 1,000 core-years: 17 to 22 orders short, or 6 to 8 "clue equivalents"
(55 to 72 bits). The one constructive idea the lens produced, a typical-set
prune on colour ledgers with an exact miss rate, is worth at most about 1.7
orders by my estimate. I did not find a way to solve the puzzle.

## 1. The approach

Stated so it can be built, in three parts. Only the third is a method; the
first two are decisions the counting forces.

**1a. Stop treating near-solutions as the road.** No repair, polish, k-best
assignment, Hamming-ball or chain neighbourhood started from a record board is
funded as a solving method. Section 2 gives the expected yield of each.

**1b. Work on the five-clue instance, on perfect partial boards only.** With
all five clues the target is one board (the planted one; the first-moment
count of others is 4 x 10^-8, `web/content/research/build/analysis/solution-counting.mdx`,
reported from community messages, not committed data). The search object is a
perfect prefix, never a complete scored board.

**1c. Typical-set pruning.** The planted solution is not an arbitrary perfect
board: its colour ledger has a known law. The official set has flat colour
totals (five interior colours on 48 half-edges, twelve on 50, each frame
colour on 24; `research/topics/the-470-wall/results/landscape.json`,
committed), which an independent painting would not produce (24.7 edges per
colour with a standard deviation of 4.8 under independent painting, against
24 or 25 observed). So the colour of each joint is, to first order, a uniform
arrangement of a fixed multiset. Then for any region R with E_R interior
joints, the number e_c of joints of colour c inside R in the true solution is
hypergeometric: mean E_R x 24.7/420 and variance v_0 = E_R p (1-p) (1-f),
f = E_R/420. A perfect prefix reached by search is not constrained to leave a
completable remainder, so its e_c has (to first order) the with-replacement
variance v_1 = E_R p (1-p). The ratio v_0/v_1 = 1-f falls to 0.4 at the widest
part of the tree. The prune: at each row boundary compute
Q = sum_c (e_c - mean)^2 / v_0 over the 17 interior colours and cut the prefix
if Q exceeds a fixed upper point of chi-square with 16 degrees of freedom
(32.0 at 99%). The per-test level is set so that the total miss rate over all
row boundaries is at most 5% (Bonferroni); the miss rate is fixed by
construction, not learned, and a sound ledger prune
(`research/topics/ledger-prune`) remains underneath. Build order:
(i) a flat-total painter for generated boards (shuffle a fixed colour
multiset); (ii) the Q statistic inside the ladder DFS of
`research/experiments/hard-ladder`; (iii) the same inside the engine's strict
solver for 16x16 pricing by Knuth probes.

One sentence for the design: search perfect prefixes of the five-clue puzzle
and discard those whose colour ledger the true solution would show with
probability below a fixed small level. The sentence has no "except", but its gain is small
(section 2c).

## 2. Cost model

Model throughout: class-respecting placements drawn uniformly, along-frame
joints matching with p_f = 0.2, all other joints with p_i = 0.0588,
independently (the model of `research/topics/the-470-wall`). My script
reproduces its committed table (log10 N = 33.10 at 470, 1.28 at 480). Known
defects of the model: it ignores colour parity and depletion, worth about +5.9
decades at 480 (0.02 naive against 14,702; solution-counting page, reported).
I carry that as an error bar of 6 decades on every count; no conclusion below
depends on fewer than 10.

### 2a. Question (a): distance from a 470 board to the nearest solution

Expected number of boards with at most k mismatches that differ from a given
solution exactly on a region S (the pieces of S rearranged inside S, every
cell changed): (number of rearrangements) x P(at most k mismatches among the
joints touching S). Summed over all rectangles and complements of rectangles
(914 region types), log10, from `01_isolation_profile.out`:

| cells changed | k <= 0 | k <= 4 | k <= 10 | k <= 20 | with shape-entropy bound, k <= 10 |
|---|---:|---:|---:|---:|---:|
| 1 | -2.2 | 2.8 | 2.8 | 2.8 | 3.6 |
| 3..4 | -4.5 | 0.9 | 5.5 | 6.4 | 7.3 |
| 9..16 | -11.5 | -4.0 | 3.1 | 12.5 | 5.6 |
| 17..32 | -23.9 | -14.6 | -4.0 | 10.4 | 0.3 |
| 33..64 | -32.3 | -21.9 | -9.8 | 6.8 | 0.4 |
| 65..96 | -47.3 | -35.3 | -20.3 | 1.6 | -12.0 |
| 97..128 | -45.8 | -33.0 | -16.9 | 6.5 | -10.2 |
| 129..160 | -39.8 | -26.5 | -9.8 | 14.7 | -2.3 |
| 161..192 | -30.8 | -17.2 | 0.0 | 25.4 | 5.8 |
| 193..224 | -18.1 | -4.3 | 13.2 | 39.2 | 16.0 |
| 251..256 | 0.8 | 14.8 | 32.7 | 59.1 | 33.2 |

Rectangles undercount shapes. The last column adds the exact surface entropy
of a 2D Ising droplet at the same boundary fugacity (17^-1/2 per unit of
boundary; 0.215 decades per unit length, Onsager), which is an upper bound for
a finite region. Scattered changes (no two changed cells adjacent) are counted
separately: 10^8.7 boards with at most 10 mismatches within 6 interior and 8
border cells of a solution.

Reading, for k <= 10:

- Near band: about 10^8.7 boards per solution, all within about 16 cells.
- Empty band: 65 to 160 changed cells, expected count 10^-2 to 10^-12 even
  with the generous shape bound. At 17 to 64 cells the bound leaves about one
  board; I do not claim that range is empty.
- Far band: essentially all 10^33 boards, at 250 or more changed cells.

Share of 470 boards that are near any solution: 10^(8.7 + 4.2 - 33.1) =
10^-20.2 with 14,702 solutions, 10^-24.4 with one; 6 decades lower if the
level-set count carries the solution-count correction. An unrelated board
agrees with a given solution on 4 x 1/4 + 56 x 1/56 + 196 x 1/784 = 2.25 cells
on average, so the typical 470 board is about 254 cells from a given solution
and about 246 to 250 from the nearest of 14,702 (maximum of that many Poisson
draws of mean 2.25).

The cheapest board at each distance (smallest k with expected count at least
one; rectangles only, then with the shape bound): 2 mismatches at 1 to 2
cells, 8/6 at 9 to 16, 13/10 at 17 to 32, 16/10 at 33 to 64, **20/16 at 65 to
96**, 18/15 at 97 to 128, 14/11 at 129 to 160, 11/8 at 161 to 192, 6/5 at 193
to 224, 0 at 251 and above. So relative to a solution the landscape is a
needle with a moat 15 to 20 mismatches high, and the floor on the far side (0,
other solutions and unrelated boards) is lower than the moat. This is the
overlap gap the site's `why/the-470-wall.mdx` states as a conjecture, obtained
here as a first-moment bound. A first moment proves absence in expectation
over the model, not for this instance.

### 2b. Question (c): exact non-spatial neighbourhoods, real sizes

Expected number of boards with at most 9 mismatches reachable from a 470
board by refilling a region that contains its mismatches (`01`, section C),
and what each neighbourhood really is:

| neighbourhood | nominal size | what the counting and tonight's data say | cost of exact search |
|---|---|---|---|
| one checkerboard class refilled by assignment, other class fixed | 2! x 28! x 98! x 4^98 = 10^242.7 | a cell whose four joints all match is frozen (two pieces never share a colour 4-tuple, so no other piece fits; measured: 0 perfect-fit alternatives in 10 class refills). Only the 7 to 15 cells of the class that touch a mismatch can move: effective size at most 15! x 4^15 = 10^21. Measured optimum = incumbent on 5 of 5 record boards, both classes, 0 cells moved | 128x128 Hungarian, 0.1 s in Python |
| k-best assignments (Murty), two-ply (AGENDA A9) | same | every non-incumbent assignment that moves a perfect cell loses at least one edge there and gains at most what the mismatch cells offer. Prediction: at record level its improving moves are cycles over mismatch cells only, which is A9's own second kill clause | k x O(n^3) |
| exact Hamming ball, radius r, any cells (AGENDA A14) | r = 8: 10^24; r = 16: 10^48 | compact regions are the best case; expected boards at 471 or better: 10^-7 (17 to 32 cells), 10^-12 (33 to 64), 10^-24 (65 to 96), 10^-13 (129 to 160), first above one at 193 or more cells freed. With the shape-entropy bound: 10^-2, 10^-2, 10^-15, 10^-6, above one from 161 cells | CP-SAT or SAT; committed halo times: 0.1 s at 62 freed cells, 23 s at 91, 1,193 s at 141, two timeouts at 133 and 148 (`research/topics/rigidity-sat-halo/results/halo-sat.json`) |
| exchange cycles along colour chains (AGENDA A6) | subset of the Hamming ball of the cycle length | same bound as the ball | cheap |
| the rows that hold the mismatches (rows 1..5 of the 470, 80 cells) | 10 border + 70 interior pieces | expected boards at 471 or better: 10^-30.7; perfect refills: 10^-55.8 | out of reach for SAT at this size on committed evidence |
| whole board as weighted CSP or MaxSAT with a hard all-different | everything | equals solving; the root LP bound is vacuous (478, notebook value on the site, not committed) | section 2c |

To reach 480 from a 470 board the freed region must contain every cell where
the board differs from a solution: about 250 cells. To reach 471 it must
exceed 160 to 190 cells. Below that, all of these are searches of a set that
is empty in expectation.

### 2c. Solving: the real price

Perfect-prefix counts in row-major order (`01`, section D, naive first
moment, log10): starter only 14.3, 20.7, 26.7, 32.0 after rows 1 to 4, 46.9
after row 8, 43.6 after row 12, sum over depths 49.1. Five clues: 14.3, 20.6,
20.8, 26.1, then 40.5 and 36.5, sum 42.6. The community anchor is 9.28 x 10^42
nodes per solution with 14,702 solutions, a tree of 10^47.1 with the starter
(McGavin, groups.io message 8924, as cited in solution-counting.mdx; reported,
not rerun). Scaling my five-clue to starter ratio onto that anchor:

- five clues, row-major: **10^40.7 nodes**. The two clues in row 13 act after
  the widest part of the tree, so they buy little.
- optimistic floor if all four optional clues acted before the plateau:
  10^47.1 / 784^4 = **10^35.6 nodes**. I found no order that achieves it: the
  two-front order (rows 0 to 2, rows 15 to 13, then the middle) costs 10^44.8
  because it carries two open seams, and frame-then-rings costs 10^48.6.
- budget: 3 x 10^8 placements per second per core (McGavin's engine, about
  295 million, `web/content/research/build/benchmarks.mdx:254`, reported).
  1,000 core-years = 9.5 x 10^18 nodes. Three desktops for a year (assumed 48
  cores) = 4.5 x 10^17 nodes.
- **gap: 16.6 to 21.7 orders at 1,000 core-years, 17.9 to 23.0 on desktops.**
  In information: 55 to 72 bits, 6 to 8 further correct placements of 9.6 bits
  each, placed before the plateau. Memory is not the limit for DFS; it is for
  any meet in the middle (10^20.6 perfect two-row bands at the cheap end).

What the typical-set prune (1c) buys: by Stein's lemma the share of search
prefixes that pass a test tuned to keep the solution is about
exp(-D), D = 16 x 0.5 x [(1-f) - 1 - ln(1-f)] nats for 16 free colour
counts with variance ratio 1-f. That is 1.5 nats at f = 0.5 (x 4.7), 2.5 at
0.6 (x 12), 4.0 at 0.7 (x 56). Weighted by where the tree's mass sits (f
around 0.6): **1 to 1.7 orders**. Assumptions: with-replacement variance for
search prefixes (depletion makes the real ratio closer to one, so this is an
upper estimate), uniform arrangement of colours in the planted solution, and
independence across row boundaries not exploited.

What single sub-result would remove most of the gap:

1. An exact refuter of dead prefixes that acts by row 3 or 4. The isolation
   counting says why none is known: a dead prefix has, in expectation, about
   10^6 perfect fillings of each next row (6.4, 6.0 and 5.3 decades for rows
   2 to 4 in `01`, section D), and that number stays above one until about 68
   interior pieces remain, (m/196)^14 x 10^6.4 < 1, which is the last five
   rows. Its deadness is invisible to any bounded lookahead before that. The
   record boards agree: their mismatches fill rows 0 to 5. Probability such a refuter exists and is cheap: below 1%.
2. A statistic with a known law under the planted solution and a very
   different law under search, worth D of 40 nats or more. The colour ledger
   gives 2 to 4. Second-order ledgers (counts of corner colour pairs) are
   unmeasured; I put 3% on them reaching 10 nats and under 0.5% on 40.
3. Six to eight correct placements from outside the puzzle (a design
   regularity of the planted board). Nothing in my lens bears on it.

## 3. Why this is not already known to fail, and closest prior work

- **The 470 annealed landscape** (`web/content/research/why/the-470-wall.mdx`,
  data committed). It counts boards per score with no distance to a solution
  and leaves the gap as a conjecture ("no feasible computation checks one
  directly at full size"). The distance-resolved first moment of 2a is that
  computation, at the level of expectation.
- **Near-codeword spectrum, AGENDA A10**
  (`research/experiments/near-codeword-spectrum/`, committed, killed). On
  mu-matched 5x5 and 6x6 boards the mid band costs fewer mismatches than the
  far band (4 against 7 at 5x5), so "the mid band is a cheap saddle, not a
  barrier". My model reproduces those numbers (5x5: near 2, mid 4 to 7, far 7
  against measured 2, 4, 7; 6x6: mid 5 to 7, far 8 against 4 and [5, 12]) and
  says the sign flips with size: mid minus far is -3 to 0 at 5x5, -1 to +2 at
  8x8 (C = 16), 0 to +4 at 10x10 (C = 18), +3 to +6 at 12x12 (C = 19), +8 to
  +11 at 16x16. The kill is right at the sizes tested and does not transfer.
  What the earlier attempt lacked: a size-explicit model. At 6x6 my low end
  is one mismatch above the measurement.
- **Rigidity wall** (`why/rigidity-wall.mdx`; only the radius-4 SAT halo is
  committed). It reads rigidity as a property of record boards and lists
  "whether local rigidity follows from area-law distinctness" as open
  (`research/program/EXPLORED.md`). Section 5 shows the counting predicts all
  20 committed rows. One notebook claim on that page (218 freed cells refuted
  in 0.31 s) is at odds with the committed scaling (141 freed cells, 1,193 s)
  and should be rerun before anyone builds on it.
- **Checkerboard assignment.** P. Schaus and Y. Deville, "Hybridization of CP
  and VLNS for Eternity II", JFPC 2008 (record opened at
  research.dial.uclouvain.be/handle/2078.5/253676: a neighbourhood "explored
  in polynomial time by solving an assignment problem", 458/480). F. Salassa,
  W. Vancroonenburg, T. Wauters, F. Della Croce, G. Vanden Berghe, "MILP and
  Max-Clique based heuristics for the Eternity II puzzle", 2017,
  arXiv:1709.00252 (opened: the Tile Assignment neighbourhood and its "Black
  and White" checkers variant; 457 on the 16x16, with 460 quoted for T.
  Wauters, W. Vancroonenburg, G. Vanden Berghe, "A guide-and-observe
  hyper-heuristic approach to the Eternity II puzzle", J. Math. Model.
  Algorithms 2012, DOI 10.1007/s10852-012-9178-4, seen in a dblp listing, not
  opened). So the neighbourhood is 18 years old and stalls 10 to 13 edges
  below the record. New here: the frozen-cell identity that shrinks it from
  10^243 to at most 10^21 on a record board, and the measurement.
- **Statistical cuts.** AGENDA A12 (ProbCut, killed): a learned probe of
  subtree size, AUC null. The typical-set prune differs in using a law known
  in closed form for the planted solution, with an exact miss rate, and no
  learning. `research/topics/ledger-prune` is the sound limit of the same
  ledger. I did not find the hypergeometric test in the repo or in the papers
  above; I am not sure it is absent from the community archive.
- **Theory.** D. Gamarnik, "The overlap gap property", PNAS 2021, DOI
  10.1073/pnas.2108492118, and D. Achlioptas and A. Coja-Oghlan, FOCS 2008,
  arXiv:0803.2122 (both cited by the site; not reopened tonight). F. Hamze et
  al., "From Near to Eternity: Spin-glass planting, tiling puzzles, and
  constraint satisfaction problems", Phys. Rev. E 97, 043303 (2018),
  arXiv:1711.04083 (abstract opened: planted tiling puzzles with tunable
  hardness; the abstract says nothing on near-solutions). E. Demaine and M.
  Demaine, Graphs and Combinatorics 23 (2007), DOI 10.1007/s00373-007-0713-4
  (NP-completeness; listing seen, not opened).

## 4. What is new

1. The distance-resolved first moment for the real 16x16 (table 2a), with a
   shape-entropy bound, and the numbers that follow: 10^-20 to 10^-30 of 470
   boards near a solution, an empty band at 65 to 160 cells, a moat of 15 to
   20 mismatches.
2. Rigidity as the null model: every committed halo verdict predicted, which
   answers an item on the explicit open-problems list in expectation.
3. The size-dependent reading of the A10 kill, with a predicted sign flip
   between 8x8 and 12x12.
4. The frozen-cell identity and the zero-move checkerboard optimum on five
   record boards.
5. The price of solving in clue equivalents (6 to 8), and the reason
   lookahead cannot see dead prefixes before the last five rows.
6. The typical-set prune (1c), with its Stein bound. Small, possibly new.

Nothing here is a new way to solve the puzzle.

## 5. Numerical evidence produced tonight

**Small planted boards** (`02_small_boards.out`, `02_small_boards_deep.out`).
My own painter, palettes of A10, all solutions enumerated exactly, final
boards of a plain annealer, distance to the nearest solution in cells.

| board | boards | where the low-mismatch boards sit | correlation mismatches vs distance |
|---|---:|---|---:|
| 5x5, 8 interior colours | 1,171 unsolved | 2 to 3 mismatches: 73 to 93% within 6 cells; 4 to 7: 59 to 77% in the mid band; 8 and more: 40 to 88% far | 0.62 |
| 6x6, 9 colours, 8,000 moves | 899 | none below 8 mismatches; from 10 up, 52 to 100% far | 0.27 |
| 6x6, 60,000 moves | 297 | 5 to 7 mismatches: 9 boards, spread over all three bands; 8 and more: 52 to 100% far | 0.33 |
| 7x7, 10 colours, 16,000 moves | 320 | none below 15 mismatches; 97 to 100% far | 0.15 |
| 7x7, 120,000 moves | 100 | none below 12; 89 to 100% far | 0.00 |

At 5x5 the boards a local search stops on are near-solutions when they have 2
or 3 mismatches, as the model says (near 2, mid from 4, far from 7). The link
between score and distance weakens with size and is gone at 7x7. Limits: my
annealer is weak (it solves 29 of 1,200 runs at 5x5 and 3 of 300 at 6x6), so
at 6x6 and 7x7 it never reaches the 2 to 7 mismatch range where the model
makes its sharper claim; the 7x7 result shows only that boards at 12 or more
mismatches are unrelated to solutions, which is the easy half. This is
consistent with the model and does not test the empty band. No claim about
16x16 follows from it.

**Record boards** (`03_record_boards.out`; the five public boards committed in
`research/topics/rigidity-sat-halo/compute/boards.json`, relabelled to the
official colours and checked to be rearrangements of the official set).

- Mismatches: all interior-colour except two frame joints on the 464; all in
  rows 0 to 5 on every board; 16 to 28 cells touch one.
- Pairwise distance, best of four turns: 250 to 253 cells of 256 for all ten
  pairs. The five best boards known share 3 to 6 cells with each other, the
  chance level plus clues.
- Checkerboard assignment, both classes, five boards: gain 0, cells moved 0,
  in all ten refills. Ties exist only at mismatch cells (5 to 73 cell and
  piece pairs fitting as well as the incumbent).
- Null model for the SAT halo: my freed-cell counts equal the committed ones
  on all 20 rows; log10 of the expected number of perfect refills is -43.7 to
  -54.2. The 18 UNSAT verdicts and 2 timeouts are what a random board with
  the same mismatches would give.
- Mismatch colour parity: on all five boards every colour appears an even
  number of times on mismatched half-edges (0 odd colours). The mismatch set
  is an Eulerian multigraph on colours.
- Parity ledger across the 60 straight half-board cuts: the bound "mismatches
  at least half the number of odd colours" reaches 3 to 5 against 10 to 16
  actual.

**A check that contradicted me** (`04_leftover_ledger.out`). I expected the
pieces left for the mismatch band to show an unusual colour ledger. Against
random piece sets they do, by 7 to 11 standard deviations. That is an
artefact of the null: any region tiled with few mismatches has even,
pair-inflated colour totals. Against the right control, the mismatch-free
mirror band of the same boards, the statistic is 162.2 on average for the
mismatch bands and 161.0 for the perfect bands, spread 2.1 decades over the
ten bands. No selection effect is visible with five boards. This is why the
typical-set estimate in 2c is small and why I do not claim more for it.

## 6. First experiment

Under 10 core-hours, generated boards only.

- Painter with flat colour totals at 8x8 (C = 16) and 10x10 (C = 18), 60
  instances each, planted solution known.
- Arm A: the ladder's strongest propagating DFS
  (`research/experiments/hard-ladder`). Arm B: the same with the Q test at
  every row boundary, per-test level 1 - 0.05/R for R row boundaries (total
  miss rate at most 5%). Paired by instance, node-counted, same node cap.
- Also record, for prefixes visited by arm A, the variance of e_c per depth
  against v_0, and Q of the planted prefix at each depth.

Written before the run. **Kill the typical-set prune if** the median paired
node ratio B/A is above 0.5 at both sizes, or the planted solution is cut on
8 or more of 60 instances at either size (a true 5% rate gives 8 or more with
probability about 1%, so that outcome means the hypergeometric law does not
hold for the painter or the test is miscoded). **Kill it
as a solving lever in any case if** the measured variance ratio implies fewer
than 5 orders at 16x16 by the formula of 2c; I expect this second line to
fire. The run is still worth its cost: a 10x to 50x node cut on five-clue
search would be the largest sound-in-probability prune on the site, and the
variance ratio is a number nobody has.

A second experiment tests the negative claim itself and should follow: at
12x12 with C = 19, where the model puts the far floor (6) below the mid band
(9 to 12), take boards at 6 to 8 mismatches from the break-tolerant engine
and measure distance to the planted board and its turns. Prediction: bimodal,
either within about 16 cells or beyond 108, nothing between. A mid-band board
at 8 or fewer mismatches on 3 or more of 30 instances refutes section 2a.

## 7. Probabilities

- Complete solution, this approach, one year, three desktops: **10^-6**.
- Complete solution, 1,000 core-years: **3 x 10^-6**. Compute moves the gap
  by 1.3 of 17 to 22 orders; the number is the chance that an idea worth 15
  or more orders appears along this line and is right.
- Main reason they are not higher: the target is one isolated board among
  10^35.6 to 10^40.7 partial boards, sound lookahead cannot see deadness
  before the last five rows, and the only statistic with a known law is worth
  under 2 orders.
- Verified board above 470, same approach used as a record engine (perfect
  prefixes, typical-set prune, exact tail): **3%** on three desktops, **35%**
  with 1,000 core-years. Assumptions: boards with mismatches confined to an
  80-cell tail thin out by 2.4 decades per edge (C(170, k) x 16^k), so one
  more edge costs about 230 times the compute at a fixed method; the 470
  cost on the order of 10 core-years (the site says "weeks of many-core
  time", not a number; this is my guess and the largest uncertainty).

## 8. The best idea I rejected

Relax piece uniqueness instead of edge matching: walk the space of fully
matched tilings with repeated pieces and minimise the number of duplicates.
It is attractive because that space is locally flexible (an 8x8 window has
about 784^64 / 17^144 = 10^8 perfect refills), which complete boards never
are. Rejected by the same counting: a refill draws about three duplicates per
four cells, tilings with D duplicates number about N_sol x (196^2/2)^D / D!^2
(4.3 decades per duplicate) and none is within reach of a solution, since
replacing one piece of a solution needs an identical piece. The community run
of this idea stalled near 463 (`build/dead-ends.mdx`, line 664, reported).
