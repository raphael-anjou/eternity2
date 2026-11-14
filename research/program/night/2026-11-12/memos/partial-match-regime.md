# The partial-match regime: counting, bounds, and what to build

Night 2026-11-12. One memo, no experiment infrastructure. Scripts and their
outputs are in `research/program/night/2026-11-12/memos/partial-match-regime/`
(`model.py`, `brute_check.py`, `gap_check.py`, stdlib Python, run with
`python3 -I`, about 90 s in total on one core). Every number below is either
printed by one of those scripts (file named), or taken from the repo with its
path and a grade: **measured** (committed result), **reported** (someone
else's number), **argued** (reasoning or uncommitted run). The other memos in
this folder and every file of Track 8 were not read.

## 1. Answer in ten lines

1. The first-moment count can be made exact over a colour-shuffle ensemble, breaks included, and it reproduces four community numbers at once: 14,702 solutions with the starter, 4e-8 with five clues, 9.3e42 nodes per solution for a row scan, and Owen's 2.05e35 boards at 467 per perfect board (section 2.3).
2. Expected boards at each score, no clue pinned: 10^34.4 at 470, 10^31.7 at 471, 10^28.9 at 472, 10^12.3 at 478, zero at 479 (parity), 10^7.1 at 480. Each point costs a factor 10^2.7 in boards.
3. A depth-first search that allows 10 breaks anywhere pays 10^45.4 nodes per board, worse than strict search for a perfect board (10^43.0). Breaks only late, on Blackwood's unlock list, pay 10^23.3. The schedule is worth 22 decades; it is the algorithm.
4. Price of a point for that method: one break fewer multiplies nodes per board by 10^1.9 (list re-optimised) to 10^2.4 (list truncated), so 471 costs 80 to 235 times 470, and 472 about 10^4 times. Almost all of it is the ratio of late-admissible boards, a property of the puzzle and of the tail length, not of the node count.
5. The model's absolute cost is about 6 decades above the reported compute for 469. The colour-exhaustion quota of the record engine is worth 10^5.9 to 10^10.8 in the same model, so the gap is the quota. The pool's colour content is the largest lever in the regime.
6. A bound that drops piece distinctness is loose by about 0.2 break per remaining cell (model: 11 breaks at 56 cells left; measured tonight on 6x6 and 7x7: 2.2 to 2.75 at two and three rows). It starts to bite for a 470 target at 40 to 56 cells left, which is where the hand-made schedule unlocks its breaks.
7. So the ideal version of that bound (exact frontier, zero cost) re-derives the schedule and adds a factor 5.5 over the best list (0.4 point). A width-limited diagram on 16 columns, paid at each node, will do less. A perfect bound of any kind from depth 201 cannot beat the best list at all.
8. The literature the explored map omits (cost function networks, MaxSAT cores, decision diagrams, dual decomposition, quadratic assignment) has no published result on edge matching in the maximisation form that I could find, and its exact methods stop near 30 assignment variables on quadratic assignment. The 16x16 has 256.
9. What I would build is not a bound: a first-moment gate computed from the node's own pool (an adaptive schedule, about 10 table lookups per node), a colour staging computed by the same model, parity as the only sound per-node bound, and an exact solver for the last row.
10. Blind prediction for Track 8: median gap near 0 to 1 edge at one row left, 2 to 3 at two rows, 3 to 5 at three rows for W = 4096, larger for W = 64; little dependence on W on 6x6 above 512; the result will not transfer to the 16x16 in edges, only as gap per remaining cell.

## 2. Counting model and what it predicts

### 2.1 Objects and notation

Board 16x16, 256 cells, 480 joints. Pieces: 4 corners, 56 border pieces, 196
interior pieces. Joints split into 60 ring joints (between two border cells,
along the frame) and 420 interior-colour joints (56 between the ring and the
interior, 364 inside). Ring: 5 colours, each on 24 half-edges, 12 facing
clockwise and 12 anticlockwise. Interior: 17 colours on 840 half-edges, five
colours with n_c = 48 and twelve with n_c = 50 (`web/content/research/why/the-470-wall.mdx`,
`research/topics/the-470-wall/results/landscape.json`, measured).

A break is a joint whose two half-edges differ. N(k) is the number of complete
boards with exactly k breaks (score 480 - k). W is the number of
class-respecting placements: W = 4! 56! 196! 4^196 = 10^559.94 with no clue,
divided by 784 with the starter pinned, and 4! 56! 191! 4^191 with five clues.

### 2.2 The exact first moment, breaks included

For any instance, N(k) = W x P(a uniformly random placement has k breaks).
That is an identity. The model enters when the probability is computed.

**Ensemble.** Keep the piece shapes and the colour counts. Shuffle the 840
interior-colour half-edges uniformly over the 840 interior-colour sides, the
60 clockwise ring half-edges over the 60 clockwise-facing ring sides, and the
same for the anticlockwise ones. For a fixed placement the joints are fixed
pairs of sides, so the pairing of half-edges is a uniformly random perfect
matching of the 840 (interior) and a uniformly random bijection of 60 onto 60
(ring). The two are independent. Hence

E[N(k)] = W x sum over k_r + k_i = k of R(k_r) I(k_i),

with I(k) = D_I(k) / 839!! and R(k) = D_R(k) / 60!, where D_I(k) is the number
of perfect matchings with exactly k two-coloured pairs and D_R(k) the number
of bijections with exactly k two-coloured pairs. Both have closed forms by
inclusion and exclusion. Let A_m be the number of ways to mark m disjoint
one-coloured pairs,

A_m = [x^m] prod_c sum_j n_c! / (j! (n_c - 2j)! 2^j) x^j,

then A_m (839 - 2m)!! counts (matching, m marked one-coloured pairs), and

D_I(k) = sum_{m >= 420-k} (-1)^(m-(420-k)) C(m, 420-k) A_m (839 - 2m)!!.

For the ring, with r_j = [x^j] prod_c sum_i C(12,i)^2 i! x^i (rook numbers of
the same-colour blocks),

D_R(k) = sum_{j >= 60-k} (-1)^(j-(60-k)) C(j, 60-k) r_j (60 - j)!.

`model.py` evaluates both in exact integers (functions `interior_dist`,
`ring_dist`). At k = 0 this is the site's canonical formula
(`web/content/research/build/analysis/solution-counting.mdx`, the display with
(12!)^5/60! and prod (n_c-1)!!/839!!).

**Partial boards.** Fill in row scan. After d cells, a_c, a_e, a_i of them are
corner, border and interior cells, and e_r(d), e_i(d) ring and interior joints
are closed. The number of ordered choices of pieces is
A(d) = (4)_{a_c} (56)_{a_e} (196)_{a_i} 4^{a_i} (falling factorials). By
exchangeability, the e closed joints are a uniform subset of the full
matching's pairs, so the number of breaks among them is a hypergeometric
thinning of the full law:

H_I(e, j) = sum_k I(k) C(k, j) C(420-k, e-j) / C(420, e), and the same for H_R.

The expected number of partial boards at depth d with j breaks is

T(d, j) = A(d) x sum over j_r + j_i = j of H_R(e_r(d), j_r) H_I(e_i(d), j_i).

**A break rule.** If the search allows breaks only in some patterns (a
cumulative unlock list, at most one break per placement, and so on), let
pi_S(d; j_i, j_r) be the number of allowed patterns (a small dynamic program
over the cells). A specific pattern has probability H(e, j) / C(e, j), so

T_S(d) = A(d) x sum pi_S(d; j_i, j_r) x H_I(e_i, j_i)/C(e_i, j_i) x H_R(e_r, j_r)/C(e_r, j_r).

The boards the rule can reach are B_S = T_S(256), and the cost of the search is

Q_S = (sum_{d=1..256} T_S(d)) / T_S(256), nodes per board found.

Q_S is a ratio of expectations. It is the long-run cost of a depth-first search
that samples its openings uniformly (restarts), not a statement about one run.

**Distinctness.** It is exact here: placements are permutations, and half-edges
are drawn without replacement. Nothing is approximated inside the ensemble.

**What is neglected.** (a) The real instance is planted, not a typical shuffle:
it has at least 4 perfect boards and their neighbourhoods. (b) Everything past
the first moment (section 2.6). (c) Every heuristic of a real engine (section
2.5). (d) The five clue positions are treated as generic cells.

**Numerical check of the algebra** (`brute_check.py`, output in
`brute_check_output.txt` and `brute_check_400.txt`). `interior_dist` and
`ring_dist` equal brute-force enumeration of every matching and bijection on
nine small colour multisets, integer for integer. On 4x4 boards drawn from the
ensemble (3 ring and 3 interior colours), a real depth-first enumeration with
up to 2 breaks anywhere gives mean counts per (depth, breaks) that agree with
T(d, j): over 400 instances the largest deviation among the 27 cells compared
is 1.27 standard errors (for example depth 10 with 2 breaks: formula 18,881,
measured 19,088 +- 484).

### 2.3 Boards at each score

`model.py`, first table (`model_output.txt`). log10 of E[N(k)].

| score | k | no clue | starter | five clues | independent-edge model (470-wall page) |
|---|---|---|---|---|---|
| 480 | 0 | 7.06 | 4.17 | -7.39 | 1.28 |
| 479 | 1 | none | none | none | 5.12 |
| 478 | 2 | 12.29 | 9.39 | -2.16 | 8.66 |
| 477 | 3 | 14.98 | 12.08 | 0.53 | 12.02 |
| 476 | 4 | 17.81 | 14.92 | 3.36 | 15.26 |
| 475 | 5 | 20.61 | 17.71 | 6.16 | 18.40 |
| 472 | 8 | 28.92 | 26.02 | 14.47 | 27.38 |
| 471 | 9 | 31.65 | 28.75 | 17.20 | 30.26 |
| 470 | 10 | 34.36 | 31.47 | 19.91 | 33.10 |
| 469 | 11 | 37.05 | 34.16 | 22.60 | 35.89 |
| 467 | 13 | 42.38 | 39.48 | 27.93 | 41.36 |
| 465 | 15 | 47.61 | 44.72 | 33.16 | 46.69 |

Comparison with numbers already in the repo:

- 10^4.17 = 14,790 against the canonical 14,702 with the starter, and 10^-7.39 = 4.1e-8 against 4e-8 with five clues (`solution-counting.mdx`, reported, msg 11193). 10^7.06 = 1.15e7 with no clue, same page.
- The last column reproduces `landscape.json` (33.10 at 470, 18.40 at 475, 8.66 at 478, 1.28 at 480, measured). The exact count is 5.8 decades above it at 480, 1.3 above at 470 and 0.8 above at 460.
- N(467)/N(480) = 10^35.31. Owen's slip table gives 2.05e35 = 10^35.31 (`web/content/research/build/reduce/edge-slipping.mdx`, reported, msg 6412). Max's rule 2^(N-1) C(420, N) gives 10^27.84 at N = 13 and 10^22.34 at N = 10; it is 5 to 7 decades low because it counts only perturbations of perfect boards.
- k = 1 is exactly zero, as in Owen's table: every colour count is even, so the multigraph "colours joined by broken joints" must have all degrees even. The probability that k random interior breaks satisfy this is 7.4e-3 at k = 2, 4.0e-5 at k = 10 and tends to 2^-16 (`model.py`, line "P(all 17 interior colours even")). The independent-edge model has no such factor at k > 0 and no depletion factor at k = 0, which is why it is too low at 480 and nearly right at 470.
- Small boards, a real test. `research/experiments/near-codeword-spectrum/README.md` (measured, CP-SAT optima) gives the fewest breaks of any board far from the planted one and its turned copies. The first k with E[N(<= k)] >= 1, corner pinned, from `model.py`:

| board | model k* | measured far* minimum |
|---|---|---|
| 5x5, 13 colours | 7 | 6 (4 instances), 7 (15), 8 (11) |
| 5x5, 9 colours | 2 | median 2.5 |
| 6x6, 10 colours | 2 | 2 |
| 6x6, 14 colours | 7 | open, in [5, 12] |
| 7x7, 15 colours | 8 | open, lower bound 3 to 4, best found 15 |

On the three closed cases the first-moment threshold predicts the true minimum within one break. One break is about 1.4 decades of count there.

Reading for the frontier: with no clue or the starter only, the model expects
boards at every score from 478 down in astronomical numbers, so 470 is not a
ceiling of the piece set. With five clues it expects 3 boards at 477, about
2,300 at 476, and nothing uncorrelated with the planted board above that.

### 2.4 Nodes needed to find a board with at most K breaks

`model.py`, row scan, no clue. Strict search first, as a check: the tree peaks
at depth 161 with 10^48.8 nodes, totals 10^50.03 nodes, and costs 10^42.97 =
9.3e42 nodes per board. The community figure for scan-row is 9.28e42 nodes per
solution (`solution-counting.mdx`, reported, msg 8924).

Budget K = 10 (target 470), log10 values:

| rule for where breaks may go | boards reachable | nodes | nodes per board |
|---|---|---|---|
| anywhere | 34.36 | 79.80 | 45.44 |
| only in the first 56 cells | 26.18 | 73.90 | 47.72 |
| free from depth 151 | 30.47 | 70.93 | 40.46 |
| free from depth 201 | 27.15 | 57.07 | 29.93 |
| free from depth 231 | 22.38 | 50.03 | 27.65 |
| Blackwood's list shifted 40 cells earlier | 29.88 | 66.77 | 36.89 |
| Blackwood's list shifted 20 cells earlier | 28.64 | 58.53 | 29.89 |
| Blackwood's list (201, 206, ..., 237, 239) | 26.69 | 50.03 | 23.34 |
| Blackwood's list shifted 8 cells later | 25.39 | 50.03 | 24.64 |
| model-optimal list (185, 198, 205, 212, 217, 221, 227, 229, 233, 235) | 27.64 | 50.23 | 22.58 |

The list is from `web/content/research/lab/experiments/joshua-blackwood/solver.mdx`
(reported, his code); all rows allow at most one break per placement except
"anywhere" (two).

**Why late helps, in one line.** A break allowed among the e(d) joints closed
at depth d multiplies the nodes at that depth by about rho x e(d) / j, with
rho = (1 - p)/p = 16 for interior joints and 4 for ring joints, and multiplies
the reachable boards by rho x E_allowed / K. If breaks may fall before the
strict peak (depth 161), the peak itself, 10^48.8, is multiplied by
C(e, j) rho^j: 30 decades for "anywhere". If they may fall only after it, the
peak is untouched and only the boards grow. Breaks allowed early are the worst
case: they inflate the whole tree under them and reach fewer boards
(10^47.7 nodes per board, 4.7 decades worse than strict search).

**Why the unlocks must be spread.** Releasing all ten at depth 201 lets the
late tree re-expand (peak at depth 215, 10^57 nodes in total): 10^29.9 per board.
Releasing them one at a time keeps the late tree below the strict peak:
10^23.3. The cumulative list is worth 6.6 decades over a single gate, and
Blackwood's hand-made list is within a factor 6 of the model optimum. Under
his list the model puts only 0.09% of the nodes at depth 201 or later; under
the model-optimal list, 37%.

**Scaling with K.** log10 nodes per board:

| K | score | breaks anywhere | cumulative list, re-optimised per K | boards reachable (re-optimised) |
|---|---|---|---|---|
| 0 | 480 | 42.97 | 42.97 | 7.06 |
| 7 | 473 | 45.59 | 28.43 | 21.70 |
| 8 | 472 | 45.57 | 26.42 | 23.74 |
| 9 | 471 | 45.52 | 24.48 | 25.66 |
| 10 | 470 | 45.44 | 22.58 | 27.64 |
| 11 | 469 | 45.34 | 20.75 | 29.43 |
| 12 | 468 | 45.23 | 18.98 | 31.24 |
| 13 | 467 | 45.09 | 17.24 | 32.97 |

With breaks anywhere the cost is flat in K, near 10^45: every extra break adds
as many junk prefixes as boards. With a late list each extra break divides the
cost by 10^1.75 to 10^2.0.

### 2.5 The price of 471 and 472, and whose price it is

From the table, with the list re-optimised: 471 costs 10^1.90 = 80 times 470,
472 costs 10^3.84. If the list is only truncated (Blackwood's own move from 11
to 10 entries, same page, reported msg 10076), the model gives 10^2.37 = 235
per point (23.34 to 25.71) and 10^2.33 from 469 to 470 (taking the eleventh
unlock at depth 196, my assumption: the 469-era list is not on the page). Community rules of
thumb: about 100 per edge (Owen, msg 6391, reported) and about 20 per edge in
JSA's log of the public 2008 solver (`edge-slipping.mdx`, reported). The second
is a different quantity (the score histogram of one engine below its target);
its value is close to rho = 16, the cost of one more accidental match at the
bottom of the board, which I offer as a conjecture, not a derivation.

**Decomposition at 470.** Nodes barely move with K (50.14 to 50.23): the tree
is the strict tree. The price per point is the ratio of reachable boards,
10^1.98, and that ratio is rho x (late joints available) / K x a parity
factor: p = 1/17, a tail of about 100 joints, K = 10. It is a property of the
palette and of the tail, the same for any search that keeps its breaks in the
tail. The method's share shows elsewhere:

- boards that exist at 470: 10^34.4; boards a late list can reach: 10^27.6. The search can see one board in 10^6.7.
- the strict tree in front of the tail, 10^50, is paid whatever the score.

**Absolute level against reported compute.** The first 469 took "a few days on
about two hundred cores" (`web/content/research/build/faster/distributed-solving.mdx`,
reported, msg 10045). At the 18.9M nodes per second measured for that code on
one core (`solver.mdx`, measured here), that is about 10^15 nodes, a single
event, so uncertain by a decade either way. The model says 10^20.8 for K = 11.
The 470 has been found twice in about four years
(`web/src/components/research/views/RecordsView.tsx`, reported). So the bare
ensemble is about 6 decades too expensive.

The record engine is not bare. It forces one ring colour and two interior
colours to be nearly exhausted by depth about 160 (the quota array reaches 119
at depth 160, and those three colours have 120 to 124 sides in all,
`solver.mdx`, reported). The model
prices this directly: the expected number of completions of one perfect prefix,
when the tail's half-edges are spread over fewer colours (`model.py`, function
`tail_yield`, K = 10, Blackwood's list, log10):

| tail starts at depth | 17 + 5 colours | 16 + 5 | 15 + 5 | 15 + 4 | 14 + 4 | 13 + 4 |
|---|---|---|---|---|---|---|
| 155 | -23.81 | -19.56 | -15.11 | -13.03 | -8.28 | -3.11 |
| 180 | -22.95 | -19.92 | -16.67 | -14.94 | -11.49 | -7.72 |
| 200 | -19.16 | -17.00 | -14.85 | -13.27 | -10.91 | -8.35 |

Two interior colours and one ring colour removed from the tail are worth
10^5.9 (tail counted from depth 200) to 10^10.8 (from depth 155). The 6
missing decades sit inside that range. I take this as the explanation, graded
argued: the calculation does not price what the quota costs in the first 160
cells. One more interior colour is worth 2.4 to 4.8 decades in the same table,
yet Blackwood reports "eliminating four colours early instead of three (no
gain)" (`solver.mdx`, reported, msg 10056). A count explains why that
variant is tight: the 196 interior pieces contain about 196 x (15/17)^4 = 119
pieces free of two given interior colours, 90 free of three, 67 free of four,
and the tail from depth 160 needs 70 interior pieces. His two interior colours
leave a choice of 70 among 119; a third leaves 70 among 90; a fourth leaves
fewer pieces than places. A further colour can only be exhausted later, when
the tail needs fewer pieces.

**What this says about the regime.** The quantity that sets the cost is the
number of completions per frontier prefix, and it is exponentially sensitive
to the colour content of the remaining pool: (c/(c-1))^(joints left) per
colour. The record engines' three hand-made devices (fill order, colour
quota, break list) are all ways of raising that yield. Bounds do not appear in
this list, and section 4 shows why.

### 2.6 How far off can the first moment be

- **Complete boards.** For k >= 5 the count is 10^20 or more boards that are almost pairwise unrelated: a board near a given 470 board that is also at 470 or better needs several accidental matches, so clusters are small. The colour counts are fixed in the ensemble, so the largest global fluctuation is absent. What remains is the dependence on which colours share a piece. Measured accuracy at small size: threshold right within one break on three palettes (section 2.3), about 1.4 decades. My estimate for the real instance: N(470) within 2 decades of 10^34.4 with probability 0.8.
- **Near the top it is wrong for a known reason.** At k <= 3 with clues, boards correlated with the planted solution dominate (the near band costs 2 breaks on small boards, same README). The ensemble count is a count of unrelated boards only.
- **Trees.** Here the second moment is large and it matters. The ensemble average of completions per depth-200 prefix is 10^-16.9, while a prefix whose tail pool is exactly balanced has 10^-19.2 (`model.py`, "ensemble average for comparison"). The mean is carried, by a factor 10^2.3, by prefixes whose pool is unbalanced by chance; at depth 155 the factor is 10^1.9. So nodes per board is the right long-run average for uniform sampling, a poor predictor of a single run, and exactly what a quota exploits. I put the absolute tree numbers within 3 decades of a quota-free engine, and the per-point ratios within 0.5 decade, each with probability about 0.6.

## 3. Literature: what exists for exactly this problem

Searched tonight (web). Citations marked **V** were checked against a
publisher, DBLP, Crossref or the paper itself; **U** are from memory and
unverified.

**On edge matching itself.**

- Maximisation is hard to approximate: J. Bosboom, E. D. Demaine, M. L. Demaine, A. Hesterberg, P. Manurangsi, A. Yodpinyanee, "Even 1 x n Edge-Matching and Jigsaw Puzzles are Really Hard", Journal of Information Processing 25, 682-694, 2017, arXiv:1701.00146 (V). They state that the earlier APX-hardness proof of A. Antoniadis and A. Lingas, "Approximability of Edge Matching Puzzles", SOFSEM 2010, LNCS 5901, 153-164, DOI 10.1007/978-3-642-11266-9_13 (V) is incomplete.
- Exact optimisation by integer programming: F. Salassa, W. Vancroonenburg, T. Wauters, F. Della Croce, G. Vanden Berghe, "MILP and Max-Clique based heuristics for the Eternity II puzzle", arXiv:1709.00252, 2017 (V). The abstract says the formulations "remain computationally intractable for medium and large sized instances". Already on the site (`build/exact/lp-relaxations.mdx`).
- Strict (perfect-match) constraint programming: T. Benoist, E. Bourreau, "Fast Global Filtering for Eternity II", Constraint Programming Letters 3, 2008 (V, HAL lirmm-00364330). C. Ansótegui, R. Béjar, C. Fernández, C. Mateu, "Edge Matching Puzzles as Hard SAT/CSP Benchmarks", CP 2008, LNCS 5202, 560-565 (V); journal version Constraints 18, 7-37, 2013, DOI 10.1007/s10601-012-9128-9 (V). M. Heule, "Solving edge-matching problems with satisfiability solvers", 2008 (U: a search did not confirm the venue).
- Heuristics on the matched-edges objective: P. Schaus, Y. Deville, "Hybridization of CP and VLNS for Eternity II", JFPC 2008 (V, 458/480 reported). T. Wauters, W. Vancroonenburg, G. Vanden Berghe, "A guide-and-observe hyper-heuristic approach to the Eternity II puzzle", J. Math. Model. Algorithms 11, 217-233, 2012, DOI 10.1007/s10852-012-9178-4 (V).
- I found **no** paper that applies a weighted-CSP solver, a core-guided MaxSAT solver, a decision-diagram branch and bound, or a dual-decomposition bound to edge matching or to Eternity II in the maximisation form. Eternity appears as a benchmark problem in the XCSP3 competitions (2022 proceedings, arXiv:2209.00917, V; I did not check in which track). Absence after one evening of searching: I put 0.75 on "nothing published".

**The structural match in the literature is quadratic assignment.** Maximising
matched edges with each piece used once is a quadratic assignment problem with
a sparse grid flow matrix and a rotation choice per piece (computer vision
calls the same problem graph matching). That literature is the one that
studies bounds which keep distinctness:

- Gilmore-Lawler bound: E. L. Lawler, "The Quadratic Assignment Problem", Management Science 9(4), 586-599, 1963, DOI 10.1287/mnsc.9.4.586 (V); P. C. Gilmore, J. SIAM 10, 1962 (U).
- State of exact solving: K. Anstreicher, N. Brixius, J.-P. Goux, J. Linderoth, "Solving large quadratic assignment problems on computational grids", Mathematical Programming 91, 563-588, 2002, DOI 10.1007/s101070100255 (V): nug30, 30 facilities, with an average of 650 worker machines.
- The 2026 state of the art inside a cost function network solver: G. Sewa, D. Allouche, S. de Givry, G. Katsirelos, P. Montalbano, T. Schiex, "Assignment Problems in Cost Function Networks", AAAI 2026, 40(17), 14322-14330, DOI 10.1609/aaai.v40i17.38447 (V, paper read). A soft-consistency propagator for AllDifferent built on the linear assignment problem, in toulbar2. With it, toulbar2 solves 33 of the 132 smallest QAPLIB instances in 1,200 s, against 22 for CPLEX and 25 for CP-SAT; a 20-facility instance takes 337 s and 0.9 million nodes.
- Dual decomposition with an assignment subproblem: L. Torresani, V. Kolmogorov, C. Rother, "A Dual Decomposition Approach to Feature Correspondence", IEEE TPAMI 35(2), 259-271, 2013, DOI 10.1109/TPAMI.2012.105 (V). P. Swoboda, C. Rother, H. Abu Alhaija, D. Kainmüller, B. Savchynskyy, "A Study of Lagrangean Decompositions and Dual Ascent Solvers for Graph Matching", CVPR 2017, DOI 10.1109/CVPR.2017.747 (V).

**Table.** "Bound" is a lower bound on the fewest breaks of any completion of
a partial board. Costs are for the 16x16 with 784 values per interior cell;
the record engines spend about 10 ns per node (105M to 295M placements per
second, `RecordsView.tsx`, reported). The first three columns are established
in the cited papers; the last two columns are my extrapolation unless a paper
is named.

| family (key reference) | bound it gives | cost per evaluation | piece used once | reported on edge matching |
|---|---|---|---|---|
| Soft node and arc consistency, FDAC, EDAC (Larrosa and Schiex, Artif. Intell. 159, 1-26, 2004, DOI 10.1016/j.artint.2004.05.004, V; de Givry, Heras, Zytnicki, Larrosa, IJCAI 2005, 84-88, V) | the constant c0 after cost moves; at most the local-polytope LP | order e d^2 or more in the worst case, about 10^8 operations from scratch here, less incrementally | not in the bound; a board with reuse costs 0, so c0 stays 0 until a hard AllDifferent removes values | none found |
| VAC and OSAC (Cooper, de Givry, Sanchez, Schiex, Zytnicki, Werner, Artif. Intell. 174, 449-478, 2010, DOI 10.1016/j.artint.2010.02.001, V) | OSAC equals the local-polytope LP optimum; VAC is close to it | VAC several times EDAC; OSAC is an LP, preprocessing only | same as above | none found |
| Soft AllDifferent as a cost function (van Hoeve, CP 2004, LNCS 3258, 679-689, arXiv:cs/0407043, V; Lee and Leung, JAIR, arXiv:1401.4605, V, year 2012 from memory; Sewa et al. 2026, V) | assignment optimum over unary costs, moved into c0, with reduced costs per (cell, piece) | one linear assignment, O(n^3) from scratch, augmenting paths incrementally; microseconds to milliseconds | yes, but only against unary costs: the joints between two empty cells are not seen | none found; on quadratic assignment it reaches about 20 to 30 facilities |
| Core-guided MaxSAT (Morgado, Heras, Liffiton, Planes, Marques-Silva, Constraints 18, 478-534, 2013, DOI 10.1007/s10601-013-9146-2, V) | number of disjoint unsatisfiable cores among the 480 "joint matches" soft clauses | one SAT call per core, seconds to hours | yes, as hard clauses, so each core is a proof about real boards | none found for maximisation; the site uses a MaxSAT optimiser on sub-boards (`why/rigidity-wall.mdx`) |
| Relaxed decision diagrams (Bergman, Cire, van Hoeve, Hooker, INFORMS J. Comput. 28(1), 47-66, 2016, DOI 10.1287/ijoc.2015.0648, V; Cire and van Hoeve, Oper. Res. 61(6), 1411-1428, 2013, DOI 10.1287/opre.2013.1221, V) | shortest path in the diagram | about W x (values per cell) per layer: 10^8 to 10^9 operations for W = 4096 over 55 cells | only what the merged states keep; at small W, nothing | none found |
| Lagrangian prices on a diagram or on a frontier DP (Bergman, Cire, van Hoeve, Constraints 20, 346-361, 2015, DOI 10.1007/s10601-015-9193-y, V) | Lagrangian dual of "each piece once" over the relaxed diagram | 10 to 100 diagram passes | yes, priced; at best the LP over reuse-tilings with piece counts | none found; the site's Lagrangian tensor-network patch solved 6x6 in five minutes (`build/dead-ends.mdx`, argued) |
| Max-sum diffusion, TRW-S, dual decomposition (Werner, IEEE TPAMI 29(7), 1165-1179, 2007, DOI 10.1109/TPAMI.2007.1036, V; Kolmogorov, IEEE TPAMI 28(10), 1568-1583, 2006, DOI 10.1109/TPAMI.2006.200, V; Torresani et al. 2013, V) | local-polytope LP dual; with an assignment subproblem, the graph-matching relaxation | 10^6 to 10^7 operations per sweep if messages exploit the colour structure, tens of sweeps | only with the assignment subproblem | image jigsaws only: Cho, Avidan, Freeman, CVPR 2010, DOI 10.1109/CVPR.2010.5540212 (V), whose pairwise costs are informative, unlike 0/1 colours |
| LP and MILP on the cell model (Salassa et al. 2017, V; site) | LP optimum | minutes per LP at full size | fractionally | yes: integer optimum only on small boards; on the site the tail LP is vacuous at 4 free rows (124 of 124, `lp-relaxations.mdx`, argued) |

Two facts from this table matter later. First, every bound in it that is cheap
enough to run per node is, at best, the local-polytope LP, and the site's own
tail LPs show that LP at 0 breaks where the best completion found has 20. Second, the families
that carry distinctness into the bound do so through a linear assignment over
unary costs, which sees only the cells that touch the frontier.

The map of explored territory (`research/program/EXPLORED.md`) has no row for
any of this. The site cites Larrosa and Schiex once, on the arc-consistency
page, for the strict case.

## 4. How loose is a distinctness-relaxed bound, and where it bites

### 4.1 Estimate

Take a perfect prefix with r cells left, of which r_c, r_e, r_i are corner,
border and interior cells, and E_r joints still to close. The remaining pool
has exactly those pieces. Count completions with at most b breaks:

- each piece once: r_c! r_e! r_i! 4^{r_i} x P(at most b of the E_r joints break);
- reuse allowed: r_c^{r_c} r_e^{r_e} (4 r_i)^{r_i} x the same probability, up to the depletion terms.

The ratio is r^r / r! per class, about e^r / sqrt(2 pi r): 0.43 decade per
remaining cell. One more allowed break multiplies either count by about
rho x E_r / b, between 2.0 and 2.6 decades near b = 10. So the fewest breaks
with an expected completion differ by

gap(r) = (true minimum) - (reuse minimum), about 0.43 r / 2.3 = 0.19 r breaks,

less for the border row, where only 15 pieces compete for 15 cells.

`model.py` (function `tail_first_moment`, tail pool balanced over 17 + 5
colours, breaks anywhere in the tail) gives, for the 16x16:

| depth | cells left | true minimum | reuse minimum | gap |
|---|---|---|---|---|
| 120 | 136 | 17 | 0 | 17 |
| 150 | 106 | 19 | 2 | 17 |
| 170 | 86 | 20 | 5 | 15 |
| 184 | 72 | 21 | 7 | 14 |
| 200 | 56 | 20 | 9 | 11 |
| 216 | 40 | 19 | 11 | 8 |
| 224 | 32 | 17 | 10 | 7 |
| 232 | 24 | 16 | 10 | 6 |
| 240 | 16 | 11 | 7 | 4 |
| 248 | 8 | 7 | 5 | 2 |

Two readings. A typical perfect prefix of depth 200 is expected to need about
20 breaks to finish (a first-moment statement for a balanced pool), so a
470-class prefix is a rare one. And with more than about 110 to 130 cells left
the reuse relaxation has a perfect completion in expectation: this is the
"fake 480" entry, as a number.

**Measured tonight on small planted boards** (`gap_check.py`, `gap_output.txt`,
`gap_results.json`; 5 ring colours; prefixes are the first perfect partial
boards a randomised strict search reaches; "true" by iterative deepening with
each piece once, "reuse" with the remaining pieces reusable and the frontier
exact; 4 to 8 prefixes per row, so read these as orders of magnitude):

| board | rows left | cells | true (mean) | reuse (mean) | gap mean, median, range | first-moment true, reuse |
|---|---|---|---|---|---|---|
| 6x6, 5 interior colours | 1 | 6 | 1.25 | 1.12 | 0.12, 0, 0 to 1 | 6, 4 |
| 6x6, 5 | 2 | 12 | 7.38 | 4.62 | 2.75, 3, 2 to 5 | 6, 4 |
| 6x6, 5 | 3 | 18 | 5.25 (4 closed of 6) | 2.50 | 2.75, 3, 1 to 4 | 5, 2 |
| 7x7, 6 | 1 | 7 | 5.25 | 4.75 | 0.50, 1, 0 to 1 | 6, 4 |
| 7x7, 6 | 2 | 14 | 7.60 | 5.40 | 2.20, 2, 1 to 3 | 7, 4 |

The gap per remaining cell is 0.15 to 0.23 at two and three rows, against 0.19
from the estimate. The first-moment thresholds are right within about one
break except for the 6x6 one-row case, where most sampled prefixes were
prefixes of a perfect board. In the two unclosed 6x6 three-row cases the true
value is at least 6 and at least 5 against reuse 4 and 2. Using the reuse
bound to prune the "each piece once" search cut its nodes by 10^1.5 at two
rows on 6x6 (7 prefixes) and 10^0.85 at one row on 7x7 (4 prefixes): inside a
tail it is a useful bound.

### 4.2 Where it bites for a target of 470 or 471

A node at depth d with j breaks spent is pruned when the bound exceeds K - j.
For a typical pool, log10 of the expected number of reuse completions with at
most b breaks (`model.py`, "ideal reuse bound"):

| depth | cells left | b = 4 | b = 6 | b = 8 | b = 10 |
|---|---|---|---|---|---|
| 192 | 64 | -8.7 | -3.8 | 0.9 | 5.3 |
| 200 | 56 | -11.1 | -6.3 | -1.8 | 2.5 |
| 208 | 48 | -11.7 | -7.1 | -2.7 | 1.4 |
| 216 | 40 | -12.8 | -8.3 | -4.2 | -0.2 |
| 224 | 32 | -11.7 | -7.5 | -3.6 | 0.0 |
| 232 | 24 | -10.4 | -6.6 | -3.0 | 0.3 |
| 240 | 16 | -4.1 | -0.8 | 2.1 | 4.7 |

Negative means the bound prunes a typical node (Markov). For a 470 target the
exact-frontier reuse bound starts to prune typical nodes at about 56 cells
left once 2 breaks are spent, and at 40 cells left even with the whole budget
(marginally: the count is 10^-0.2 to 10^0.3 between 40 and 24 cells). For
471 (b one lower) it bites 4 to 8 cells earlier. Before about 64 cells left it
prunes nothing, for either target.

That depth is not an accident. Blackwood's list unlocks its first break at
depth 201, 55 cells from the end. A break list is this bound, evaluated once
for the typical pool and frozen into a table of depths.

**Its value in the terms of section 2.** `model.py` reruns the tree with every
node (d, j) kept with probability min(1, expected reuse completions with at
most K - j breaks), at zero cost:

| search | log10 nodes per board |
|---|---|
| Blackwood's list | 23.34 |
| Blackwood's list plus the ideal reuse bound | 23.34 |
| model-optimal list | 22.58 |
| model-optimal list plus the ideal reuse bound | 22.44 |
| list re-optimised with the bound: (185, 194, 195, 196, 197, 197, 199, 199, 201, 211) | 21.84 |
| the same list without the bound | 32.61 |

With the bound, the best policy is to release almost all breaks near depth 195
and let the bound do the scheduling. The gain over the best hand-made list is
10^0.74, a factor 5.5, about 0.4 point at 10^1.9 per point. That is the ceiling
for this family: exact frontier, no cost.

A perfect bound of any family, from depth d0 on, with the ten breaks free
after d0, would give (same script): 10^22.88 from depth 201, 10^21.22 from
181, 10^20.59 from 171, 10^18.89 from 151. From depth 201 a perfect oracle is
worse than the best list with no bound. To gain two decades a bound must be
near-perfect with 85 cells left, where the reuse relaxation is off by 15
breaks and the cell LP is vacuous.

**What a real diagram loses.** The frontier of a row scan is 16 colours plus
one; exact frontier states number up to 17^16. A diagram of width 10^3 to
10^6 keeps the frontier exact for a handful of cells and merges the rest, and
each merged frontier colour is a joint the bound can no longer charge. My
extrapolation: on 16 columns a width-limited diagram is exact for about one
row and loses most of the table above beyond two rows. Probability that a
width-limited diagram bound, cost included, gives a net factor 2 on the
16x16: 0.05.

### 4.3 Bounds that keep distinctness, and their cost

| bound | what it keeps | cost | tightness |
|---|---|---|---|
| Parity (Eulerian defect): future breaks >= ceil(odd colours / 2), plus the ring balance | exact global colour counts | O(1), two counters | exact reason why 479 does not exist. In the model it keeps 92% of nodes at depth 225, 10% at 235, 0.02% at 245, but only 4% of the late nodes are removed in total (`model.py`, 10^0.02), because the late tree lives at depths 205 to 230 |
| Colour supply against frontier demand, sum_c max(0, demand_c - supply_c) | pool colour counts | O(17) | the site's border-supply test: 0.0014% of tests in strict search (`dead-ends.mdx`, reported), untested with breaks; bites only when the pool has fewer than about 2 pieces per colour |
| Assignment of distinct pieces to the frontier cells, joints between empty cells ignored | distinctness, frontier joints | one small assignment, microseconds | measured tonight: 0.9, 0.9, 0.0 (6x6 at 1, 2, 3 rows) and 3.5, 1.0 (7x7 at 1, 2 rows) against true 1.25, 7.4, 5.25, 5.25, 7.6. Weaker than the reuse bound beyond one row |
| Soft AllDifferent with reduced costs in a cost function network (Sewa et al. 2026) | the same assignment, plus propagation into the binary costs | linear assignment per node, incremental | the strongest published way to carry distinctness; exact solving reaches about 20 to 30 facilities on quadratic assignment |
| Lagrangian prices per piece over a frontier DP or a diagram (Bergman et al. 2015; `ideas-raw/pair-or-compbio.md`) | distinctness priced, geometry exact inside the band | 10 to 100 DP passes; the DP is c^(n-1) wide, so bands only at 16 columns | unknown. Upper limit is the LP over reuse-tilings with piece counts. The site's Hall-matching test gave a node ratio 0.90 at 6x6 and none at 7x7 (`AGENDA.md` A17, measured), which is the price-free version |
| Exact tail (CP-SAT, or search with the reuse bound) | everything | one row proven in under 0.1 s, two rows not proven in 45 s (`build/exact/exact-tail-endgame.mdx`, measured) | exact for 16 cells |

None of these is tight beyond about two rows of the 16x16, and section 4.2
says tightness that late is worth less than a factor 6.

## 5. The algorithm I would build

**The premise I would not build on.** A per-node score bound is the wrong
centre for this algorithm. In the model the factor between the record
engine's schedule and an ideal bound is 5.5, the factor carried by the pool's
colour content is 10^6 to 10^11, and the price of a point is 10^1.9. I would
build the search around the quantity that carries the decades and keep sound
bounds where they are free.

**Design in one sentence.** A row-scan depth-first search with a global break
budget, in which the break list and the colour quota are both replaced by one
number computed at each node from its own pool, the first-moment log-count of
completions within the remaining budget, with parity as the only sound
per-node bound and an exact solver for the last row.

1. **Order of fill.** Row scan from one border, as the record engines (the trees of section 2 are for that order; the frontier is 17 colours wide and the strict peak is at depth 161).
2. **State kept per node, incrementally.** For each interior colour the number u_c of half-edges still to be paired (unused piece sides plus frontier colours); the same for the ring, split by facing; the counts of odd colours; the cells and joints left; breaks spent j. Placing a piece changes at most 4 of the u_c and 2 frontier entries.
3. **The gate.** G(node) = ln A_tail + ln P(at most K - j of the tail's joints break | the u_c), with A_tail the falling factorials of the pool and P from section 2.2 evaluated on the node's own counts. The strict term is sum_c L[u_c] - L[U] with L[n] = ln (n-1)!! from a table; the break term is a table in (joints left, K - j) times the parity factor. About 10 table lookups per node. Prune when G < theta(d). This is not sound: it is the model's adaptive version of the break list, and theta(d) is tuned so that the nodes kept per level stay flat (the model-optimal lists do exactly that).
4. **Colour staging.** In the supercritical part (depth below about 160) the same statistic, evaluated for the pool the node is heading to, replaces the hand quota: branches are kept when the pool they leave has high G. The model then chooses which colours to exhaust and by which depth, including later exhaustion of a fourth and fifth colour when the tail is short enough to allow it (section 2.5).
5. **Sound bounds, where they are free.** Parity at every node (two counters). At 16 cells left, an exact completion within the remaining budget (search with the reuse bound, or a precomputed table keyed on the frontier), replacing the last row of search.
6. **Where each is evaluated.** Staging below depth 160, gate from about depth 185, parity everywhere, exact tail at depth 240. Nothing heavier than a table lookup runs at a node rate of 10^8 per second.
7. **What it does to the break schedule.** It replaces it. The list of depths becomes the special case "theta from the typical pool".

**Expected effect, in the terms of section 2.** The gate removes part of the
factor between the best list (10^22.58) and a perfect late oracle (10^21.2 at
depth 181, 10^20.6 at 171): between 5 and 100, my central estimate 5 to 10,
half a point. Staging acts on the 10^-19 completions per prefix: each further
colour exhausted is worth 2 to 4 decades in the table of section 2.5 before
its cost in the prefix, which the model does not yet price and which
Blackwood's test says can cancel it. Probability that staging beyond the
record engine's three colours gains a decade on the 16x16: 0.25. Probability
that the gate alone gains a factor 3 at equal wall clock: 0.5.

**Cheapest falsifying experiments, in order.**

- **F0, one hour, no new solver.** Histogram of nodes by depth and by breaks spent for the Blackwood-style engine on the real puzzle (60 s, one core) and for `BreakSolver` on a ladder board. The model's claim is that the strict part dominates unless the list is tuned to balance it (0.09% late under Blackwood's list, 37% under the model-optimal list, in a quota-free model). If more than 90% of the real engine's nodes sit after its first unlock, the picture of section 2.4 is wrong for real engines and late bounds have more room than I say.
- **F1, calibration of the model, a few core-hours.** Planted framed boards at 10x10 to 12x12 with the colour count chosen by `model.py` so that the strict search costs far more than the budget and a break budget K in 3..8 costs 10^6 to 10^9 nodes per board. Run a cumulative-list search with many restarts and measure nodes per board found at each K. **Kill:** the measured slope differs from the model's by more than 0.5 decade per break, or the level is off by more than 2 decades at two or more K. Then the calculus is not a design tool and items 3 and 4 fall with it.
- **F2, the gate and one staged colour, same boards.** Arms: best list from the model; gate; gate plus one staged colour; control with theta from the typical pool (which must reproduce the list). **Kill:** work per board for the gate above 0.5 of the list's at every K, or wall-clock ratio above 1; for staging, a measured gain below one third of the predicted decades.

No 16x16 run is needed before F1 passes.

### Two neighbouring questions

**Can small planted boards reproduce the regime?** Partly, and the model says
when. The regime is defined by three conditions, none of which mentions the
optimum being unknown: the perfect board costs far more than the budget
(strict nodes per board far above the nodes available); the boards at the
reachable score are numerous and unrelated to the planted one (E[N(k)] far
above 1); and each further point costs a fixed number of decades set by
rho x (tail joints) / K. When the second condition holds, the planted optimum
is as invisible as the 16x16's, so "100% by construction" is harmless. On 6x6
and 7x7 at critical colours none of the three holds: the planted board is
found in 10^3 to 10^6 nodes, the tail has 20 joints, K is 0 to 2 and parity
makes K = 1 empty. From 10x10 up, with the colour count chosen so that strict
search is out of reach, all three can be matched; `model.py` computes the
table for any size and palette. Two things will not transfer: the value of
colour staging, which needs many colours and a tail of 50 cells or more, and
the absolute fluctuations, which are larger on small boards.

**What an instrument for this regime must measure.** Not the best score at a
fixed time, which is one draw of an extreme. It must report: (1) nodes per
board found at each score, with counts large enough for a rate and a Poisson
interval, so that the slope in decades per point is a measured number;
(2) the histogram of nodes by depth and by breaks spent, which is the model's
T(d, j) and the only way to see where a method spends its work; (3) the share
of nodes after the first break is allowed; (4) the colour counts of the pool
at that depth, for prefixes that led to a board and for those that did not,
since that is where the yield is decided; (5) work in two units, nodes and
wall clock, with the cost of any bound counted; (6) every board re-scored by
a verifier that shares no code with the engine; (7) the model's prediction
next to each number, as the null.

## 6. Prediction for Track 8, made blind

Written before reading any file of `night/2026-11-12-t8-score-bound-gap`. I
know only what `IDEAS.md` T4 says: a relaxed diagram that keeps frontier
colours and relaxes distinctness, widths 64, 512, 4096, gap by rows remaining,
6x6 and 7x7, prefixes sampled from the break-tolerant search.

Prediction, gap = bound's shortfall in edges, median over prefixes:

| rows left | 6x6, W = 4096 | 6x6, W = 64 | 7x7, W = 4096 | 7x7, W = 64 |
|---|---|---|---|---|
| 1 | 0 | 0 to 1 | 0 to 1 | 1 to 2 |
| 2 | 2 to 3 | 3 to 5 | 2 to 4 | 4 to 6 |
| 3 | 3 to 4 | 4 to 6 | 4 to 5 | 5 to 7 |

- The gap grows by about 0.2 edge per remaining cell at large width and does not close with W, because it comes from distinctness, not from merging. On 6x6 the results at W = 512 and W = 4096 will be within one edge of each other. On 7x7, W = 4096 still merges frontier states and sits 1 to 2 edges above its large-width limit.
- At W = 64 most merges relax frontier colours (my guess for the reported share: above 70% on both sizes) and the bound falls toward 0 at two rows and more.
- T4's stage-0 kill ("median gap with 3 rows left above 4 edges at every W") fires on 7x7 with probability 0.55 and on 6x6 with probability 0.3.
- If the prefixes are prefixes of the planted board, or dead-end prefixes whose best completion is within 2 breaks, the gap will be 0 to 2 everywhere by construction (the gap cannot exceed the true value). In that case the measurement is uninformative and my table does not apply.
- If the diagram keeps an exact used-piece set on unmerged paths, the one-row gap is 0 at every W.

**What the result would tell.** Whether width matters next to distinctness at
these sizes, and a gap per remaining cell to set against 0.19. **What it would
not tell.** Whether the bound prunes a 16x16 search. Three rows are half of a
6x6 board and 19% of the 16x16; the 16x16's three-row gap is about 10 edges in
the model; its frontier cannot be kept exact at any feasible width; and
section 4.2 caps the value of even the ideal version at a factor 5.5. A
passing gap on 6x6 would be evidence for a tail solver, not for a search
bound.

## 7. What I am unsure of, with probabilities

| statement | probability |
|---|---|
| N(470) of the real instance, no clue, is within 2 decades of 10^34.4 | 0.8 |
| The per-point price of the cumulative-list family on the 16x16 is between 10^1.5 and 10^2.6 | 0.7 |
| The 6-decade gap between the bare model and the reported 469 compute is mostly the colour quota | 0.6 |
| More than half of the real record engine's nodes are spent after its first unlock depth (I do not know; F0 decides) | 0.45 |
| Some sound per-node bound gives a net factor 10 on nodes per 470-class board at equal wall clock | 0.07 |
| The ideal-reuse-bound ceiling (5.5) is right within a factor 4 for a quota-free engine | 0.6 |
| The 0.19 break per cell looseness holds within a factor 1.5 on the 16x16 | 0.65 |
| No published soft-constraint, MaxSAT, diagram or dual-decomposition result exists on edge matching in maximisation form | 0.75 |
| The JSA "about 20 per edge" equals rho for the reason I gave | 0.4 |
| The model's slope survives F1 on 10x10 to 12x12 | 0.6 |

Known weak points of the derivations. The tree model is a uniform-sampling,
quota-free search on an average instance; its absolute costs are 6 decades
from reported compute and I close that gap with a calculation that prices the
quota's benefit and not its cost. The reuse counts use independent edges with
p = 1/colours and a balanced pool; real late pools are unbalanced, which makes
both counts larger. The "kept with probability min(1, expectation)" rule is a
Markov bound applied to a typical pool. The small-board gap table rests on 4
to 8 prefixes per row and on one way of sampling prefixes. The node-cost
figures for the literature families are order-of-magnitude extrapolations, not
measurements.

## 8. Three concrete recommendations for the program

1. **Measure before building any bound: run F0.** One histogram of nodes by depth and breaks for the record-style engine, set against `model.py`. It costs an hour and decides whether T4, L1 and every "score bound" idea have a factor to remove on the 16x16. If Track 8 reports a small gap on 6x6, do not promote T4 to a search bound on that evidence; keep it as a candidate for the exact two-row tail.
2. **Adopt the exact first-moment calculator as the program's null model and calibrate it (F1).** It is about 550 lines, reproduces four community numbers and one committed small-board result, and gives tree sizes, optimal unlock lists, the value of a colour, and the looseness of a relaxation for any size and palette. The partial-match instrument (L4) should be designed from it: sizes and colour counts chosen so the three conditions of section 5 hold, output in nodes per board by score.
3. **Put the pool's colour content on the agenda as the main lever, and the literature on the map.** One agenda entry for the first-moment gate and computed colour staging (F2), with Blackwood's "four colours, no gain" as the prior to beat. One row in the explored map for "soft-constraint optimisation, quadratic assignment bounds", status thin, with the table of section 3 and the statement that exact methods there stop near 30 assignment variables; the useful import from that literature is a tail solver (toulbar2 with the 2026 AllDifferent propagator, or CP-SAT, on two-row tails), not a full-board bound.
