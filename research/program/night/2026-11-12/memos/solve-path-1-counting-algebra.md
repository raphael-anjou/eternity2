# Solve path 1: exact counting and algebra

Night 2026-11-12. Lens: treat Eternity II as a counting or algebraic object and
extract a solution from a computation that is not a tree search.

## Verdict first

This lens does not contain a path to a solution that is closer than plain
depth-first search, and depth-first search is about 23 orders of magnitude
short of 1,000 core-years. Every route I could construct lands on one of three
walls, each with a number:

| route | what it costs on the real 16x16 | status |
|---|---|---|
| Colour-only contraction (plain tensor network) | 10^19.85 frontier states (570 exabytes), and it counts about 10^107.65 boards with reused pieces per true solution | computed here |
| Inclusion and exclusion over pieces, or its sign form | 2^256 = 10^77.1 contractions, or a Monte Carlo whose variance needs at least 10^103 samples | computed here, identity verified on toys |
| Exact transfer matrix carrying the used set | the same number of states as the search tree has nodes (10^43 to 10^50), merging saves 5 to 14 percent on toys | computed here |
| Per-piece prices (the "Lagrangian patch") | no price vector isolates a true solution on 17 of 19 toy boards that have reused-piece boards (LP margin 0) | computed here |
| Uniqueness ladder with exact strip moves (the one new construction below) | predicted to freeze with 105 to 123 of 256 pieces constrained, leaving 10^58 to 10^66 reused-piece boards per solution | model only |

The useful output of this memo is therefore the cost map, four small-board
checks that back it, one new sampling construction with its predicted failure
point, and a one-line consequence for the other tracks tonight: any bound or
lookahead built on colour-only counting with the exact remaining pool is
vacuous until about 110 cells remain, by which depth the tree is already at its
plateau (section 2.4).

Scripts: `research/program/night/2026-11-12/memos/solve-path-1/` (`costs.py`,
`toy.py`, `lagrange.py`, `hull.py`). Compute used tonight: about 6 minutes of
one core, of which about 3.3 minutes were an oversized first run of `toy.py`
that was stopped with no output.

## 1. The approach

### 1.1 The object

Let cell tensors carry one formal variable per piece. For cell `c`,
`T_c(x) = sum over pieces p and rotations r of x_p * [p in rotation r shows the four colours on the legs]`.
Contract the 16x16 grid over the 480 shared colour legs (bond dimension 17
inside, 5 along the frame, 1 on the rim). The result is a polynomial

`Z(x) = sum over edge-matched boards t of prod_p x_p^(n_p(t))`,

where `n_p(t)` is how many times board `t` uses piece `p`. Edge matching is the
tensor network. "Each piece once" is the coefficient of `x_1 x_2 ... x_256`, a
permanent-like extraction. All exact-counting methods are ways to take that
coefficient:

1. **Plain contraction**, `Z(1)`: ignores it.
2. **Sign form of inclusion and exclusion**:
   `N_sol = average over x in {+1,-1}^256 of (prod_p x_p) * Z(x)`. With 256
   cells and 256 pieces, "every piece used an odd number of times" forces every
   count to be exactly 1, so the average projects exactly onto solutions. It is
   an unbiased Monte Carlo estimator with one signed colour-only contraction
   per sample.
3. **Exact transfer matrix**: sweep the board and carry `(frontier colours, used set)`.
4. **Tracked subset**: carry the used set for a subset `Q` of pieces only and let
   the others repeat. `Z_k` denotes the number of edge-matched boards in which
   `k` tracked pieces are each used exactly once. `Z_0 = Z(1)`, `Z_256 = N_sol`.

### 1.2 The one construction that is new: a uniqueness ladder with exact strip moves

The ladder `Z_0 > Z_1 > ... > Z_256` has ratios near `1/e` (each new "exactly
once" constraint keeps a Poisson(1) fraction), so no single step is a rare
event. That is the standard setting for count-then-sample by telescoping
products (Jerrum, Valiant, Vazirani 1986), provided each ensemble can be
sampled.

Algorithm (population annealing over the number of constrained pieces):

- **State.** A population of `P` fully edge-matched boards with repeats allowed
  (480 of 480 edges matched, always), stored as 256 (piece, rotation) bytes each.
- **Stage k to k+1.** Add one piece to the tracked set. Keep the particles in
  which it appears exactly once (about `P/e` of them), resample back to `P`.
- **Rejuvenation move.** Pick a horizontal strip of `w` rows. Resample the whole
  strip from the exact conditional distribution of the colour-only model given
  the rows above and below, by a column-wise transfer matrix with state
  `17^w` (the `w` horizontal legs crossing the sweep line) and forward counting
  then backward sampling. Pieces tracked and currently placed outside the strip
  get fugacity 0. Tracked pieces inside the strip are handled by Metropolis
  rejection on "each exactly once". Cost per move: `16 * 17^w * (about 3 fitting pieces)`
  operations, 2 * 10^10 for `w = 7`, and 8 * 17^7 = 3.3 GB per count table (one
  per column if kept for the backward pass, or recomputed in blocks).
- **Stop.** A particle that survives stage 256 is a solution.

This uses the tensor network for what it is good at (exact, large, always
matched rearrangements in polynomial time) and moves uniqueness into a ladder
of mild constraints. Section 2.5 gives the reason it should still fail.

## 2. Cost model for the real 16x16

Inputs, all from `research/starter-kit/data/official.json` through `costs.py`
(exact census of the committed piece list): 4 corners, 56 edge pieces, 196
interior pieces; 5 frame colours with 24 sides each, never on an interior piece;
17 interior colours with 43 to 49 sides each on interior pieces; match
probabilities 1/5.00 (frame to frame), 1/16.98 (interior to interior), 1/17.23
(edge piece to interior). All counts below are first-moment estimates and
assume the pieces are statistically exchangeable within their type.

**2.1 Colour-only contraction.** The grid has treewidth 16, so no sweep order
beats a row cut: 5^2 * 17^14 = 10^18.62 states between rows and 10^19.85 with
the extra leg inside a row. At 8 bytes that is 10^20.76 bytes. A 64 GB desktop
holds 10^9.9 states, so the single full-scale attempt recorded on
`web/content/research/build/dead-ends.mdx` (lines 187 to 197, uncommitted, died
near 3 GB) was keeping at most a 10^-10 fraction of the frontier. Time is about
10^22.7 operations, 3 orders over 1,000 core-years.

**2.2 What the contraction would count.** Boards with reuse per true solution:
`prod over types of n^n / n!` = 10^1.03 * 10^23.05 * 10^83.58 = 10^107.65, that
is 10^0.434 per open cell (the base is e). With the reported 14,702 solutions
(`web/content/research/build/analysis/solution-counting.mdx`, a model value,
not a count) the contraction would return about 10^111.8. Toy boards show the
law's slope is right and its constant is high by 10^1.1 to 10^1.9 (section 5),
so read 10^107.65 as "between 10^102 and 10^108".

**2.3 Inclusion and exclusion.** 2^256 = 10^77.1 contractions of 10^22.7
operations each. In sign form the estimator has second moment
`sum over parity classes of (class size)^2 >= Z(1)`, so the number of samples for
unit relative error is at least `Z(1) / N_sol^2`, about 10^103. On 4x4 toys the measured number is 7 to 13 times `Z(1)/N_sol`,
well above that bound (section 5). This is a sign problem of 100 orders.

**2.4 Exact transfer matrix, and where colour-only lookahead starts to see.**
States are `(frontier, used set)`. On toys the number of states is within 5 to
14 percent of the number of partial boards (section 5), so the exact transfer
matrix is breadth-first search with a hash table. The naive first-moment
row-major profile from `costs.py` peaks at 10^50.0 partial boards at depth 161
and sums to 10^51.3; the site reports about 10^45 at the plateau and
9.28 * 10^42 nodes per solution for tuned orders (reported, McGavin, groups.io
message 8924, cited on the solution-counting page). Budgets: three 16-core
desktops for a year visit 10^17.2 to 10^18.2 nodes at 10^8 to 10^9 nodes per
core-second; 1,000 core-years visit 10^18.5 to 10^19.5. Gap: 23.5 orders at
best. Meet in the middle does not help at full scale: a half board has about
10^47.1 strict tilings and an 8x8 corner quadrant 10^32.9, so even one list is
out of reach, and two half lists must hold 10^90 pairs between them to contain
one solution.

The expected number of reused-piece completions of a depth-`k` partial board,
with the pool restricted to its unused pieces, is
`N_sol / T(k) * r^r / r!` with `r = 256 - k`. With the naive profile rescaled to
end at 14,702 (the rescaling is spread linearly over depth, an assumption) this
is 10^30 at depth 96, 10^9.7 at depth 128, 10^0.9 at depth 144 and 10^-3.7 at
depth 152. A colour-only bound or count over the remaining region therefore
cannot reject a partial board before depth 144 to 150, where the profile is
within one order of its peak. The same arithmetic for a closed hole: an `s x s`
interior hole with its own `s^2` pieces has fewer than one expected colour-only
filling for `s <= 10`, and strict search on such a hole starts with fewer than
1.4 candidates per cell. Where the tensor network is exact, search is already
trivial.

**2.5 The ladder.** A strip move is useful only if the strip has alternatives
that respect the tracked constraints. `costs.py` section 5: a `w`-row strip
between two fixed rows has 10^9.1 (w=4), 10^23.0 (w=6), 10^30.0 (w=7) and
10^36.9 (w=8) colour-only fillings. Requiring a fraction `q` of the pieces to be
used exactly once multiplies this by
`r!/(r-qr)! / 196^(qr) * (1-q)^(r-qr)` with `r = 14w`. The product falls below
one alternative at `q* = 0.34` (w=6), `0.41` (w=7, 3.3 GB) and `0.48` (w=8,
56 GB). Past that point the population cannot be refreshed, lineages collapse,
and the remaining `256 - k*` constraints cost `e` each: 10^66 particles for
w=7, 10^58 for w=8. So the ladder as specified is 40 or more orders short and
is worse than depth-first search.

**The single assumption that would remove most of the orders:** that strip
moves stay ergodic far beyond the annealed freeze estimate, up to about 230
tracked pieces (then `e^26 = 10^11` particles would do). The estimate treats
hard constraints, one move type and a typical configuration; soft penalties
and tempering could move it, but the block boundary cost (two rows of 16 fixed
colours, 10^-39) is geometric and does not move. I put this assumption below
1 percent.

## 3. Why it is not already known to fail, and the closest prior work

- **On the site.** "Tensor-network counting" (`build/dead-ends.mdx` line 175):
  plain contraction is argued away correctly; the amendment reports a per-piece
  Lagrangian penalty solving generated 4x4 and 6x6 boards, with nothing
  committed. I could not reproduce that with the most direct reading (max-sum
  contraction, one price per piece, subgradient steps): 0 of 8 boards at 3x3,
  0 of 8 at 4x4, 0 of 2 at 5x5, 0 of 1 at 6x6 (`lagrange.py`). `hull.py` gives the reason: the
  all-ones usage vector lies on or inside the convex hull of the reused-piece
  usage vectors, so a linear objective cannot make it the unique maximiser.
  The notebook version must have differed (a nonlinear penalty, a beam that
  tracks pieces, or boards with very few reused-piece boards, as in the 2 toy
  boards where prices do work). Its scaling is bounded by 2.1 and 2.2 either way.
- **"Build a fake 480 and repair it"** (`build/dead-ends.mdx` line 664, reported,
  no data): starts from one reused-piece board and swaps real pieces in
  greedily. The ladder differs by never breaking an edge, by sampling each
  intermediate ensemble instead of repairing one board, and by exact strip
  moves. Section 2.5 predicts the same end.
- **Kourtis, Chamon, Mucciolo, Ruckenstein, "Fast counting with tensor
  networks", SciPost Phys. 7, 060 (2019), arXiv:1805.00475.** Exact #CSP by
  contraction. Works when every constraint is local. Piece uniqueness is a
  global all-different, which is what 2.2 prices.
- **Amini, Fomin, Saurabh, "Counting subgraphs via homomorphisms", SIAM J.
  Discrete Math. 26(2), 2012, DOI 10.1137/100789403**, and **Brand, Dell,
  Husfeldt, "Extensor-coding", STOC 2018, arXiv:1804.09448.** The known ways to
  turn homomorphism counts (here: colour-only boards) into injective counts cost
  `2^k` or `4^k` for `k` vertices. Here `k = 256`. This is route 2.3; nothing
  in those papers lowers the exponent when the pattern is as large as the host.
- **Bosboom, Demaine, Demaine, Hesterberg, Manurangsi, Yodpinyanee, "Even 1 x n
  edge-matching and jigsaw puzzles are really hard", J. Information Processing
  25 (2017), arXiv:1701.00146.** Edge matching stays NP-hard at frontier width
  1, so the hardness lives in the used set and not in the grid width. That is
  why no bounded-treewidth sweep rescues the exact transfer matrix.
- **Frías-Pérez, Mariën, Pérez-García, Bañuls, Iblisdir, "Collective Monte Carlo
  updates through tensor network renormalization", SciPost Phys. 14, 123 (2023),
  arXiv:2104.13264.** Tensor contraction as a collective Monte Carlo proposal
  for Ising models. The strip move is the same idea with exact contraction.
- **Jerrum, Valiant, Vazirani, "Random generation of combinatorial structures
  from a uniform distribution", Theoretical Computer Science 43 (1986), DOI
  10.1016/0304-3975(86)90174-X.** Counting and sampling are equivalent for
  self-reducible problems; the ladder is that reduction over constraints.
- **Martinsson, "A linear threshold for uniqueness of solutions to random jigsaw
  puzzles", Combinatorics, Probability and Computing 28 (2019), arXiv:1701.04813.**
  First-moment counting for random puzzles, the model behind 2.2.

I found no paper that counts edge-matching solutions by tensor contraction,
model counting or decision diagrams (one search, not exhaustive). The five
arXiv papers were opened tonight; the SIAM and Theoretical Computer Science
entries were checked through publisher and index listings only.

## 4. What is new

- The sign identity `N_sol = E[(prod x_p) Z(x)]` as an exact statement for this
  puzzle, with its measured variance. It is inclusion and exclusion rewritten;
  the "odd implies once" step is specific to a board with as many cells as
  pieces.
- The e-per-cell law as a price list: 10^107.65 for the board, the depth (144
  to 150) at which colour-only lookahead first carries information, and the
  hole size (10 x 10) below which the contraction is exact in expectation.
- The LP result that per-piece prices cannot isolate a solution once
  reused-piece boards are abundant. It explains the degenerate dual noted in
  tonight's `ideas-raw/pair-cryptanalysis-matching.md` with data.
- The uniqueness ladder with exact strip moves. I have not found it in the repo
  or the literature. My own model says it fails.

Nothing here is a new way to solve the puzzle.

## 5. Numerical evidence produced tonight

Generated boards (own generator in `toy.py`: random edge colouring, separate
frame palette, pieces distinct under rotation). `N` = true solutions,
`Z` = colour-only boards.

**A. Sign identity** (`toy.py` part A, exact, all sign vectors summed through a
parity-mask transfer matrix). 10 of 10 boards (seven 3x3, three 4x4): the
all-odd class equals `N` exactly (for example 80 = 80 and 336 = 336 at 4x4).
Samples needed for unit relative error: 1 to 12 at 3x3; 9.6 * 10^4 and
9.7 * 10^4 at 4x4 with 3 interior colours, where `Z/N` is 8.0 * 10^3 and
7.7 * 10^3; 2.2 * 10^3 on a 4x4 with 4 interior colours, where `Z/N` is 295.
The variance is larger than the overcount, as claimed.

**B. Overcount law** (`toy.py` part B). Mean `log10(Z/N)` against the predicted
`sum log10(n^n/n!)`: 3x3: 0.79 against 2.06 (8 boards). 4x4: 3.55 against 4.68
(8). 5x5: 6.74 against 8.33 (3). 6x6: 11.00 against 12.92 (1 board, `N = 4`).
Growth per size step is 2.8, 3.2, 4.3 observed against 2.6, 3.7, 4.6 predicted.
The exponential law holds; the prediction is high by a factor 13 to 83. This
contradicts the constant in my estimate, not the slope. A likely cause is that
boards with repeats have unbalanced colour supply and match less often than
the exchangeable model assumes.

**C. Exact transfer matrix merging** (`toy.py` part C). Partial boards per
distinct `(frontier, used set)` state: 1.136 overall at 4x4, 1.065 and 1.068 at
5x5; 1.05 to 1.09 at the peak depth. No compression.

**D. Ladder ratios** (`toy.py` part D). 4x4, 3 interior colours:
`Z_k` = 641304, 249384, 110328, 50844, 20436, 9668, 4728, ... , 80. The first
six ratios are 0.39, 0.44, 0.46, 0.40, 0.47, 0.49 (prediction for pools of 4 to
8 pieces: 0.39 to 0.42; for large pools 0.37). Later ratios are erratic because
the counts are small.

**E. Per-piece prices** (`lagrange.py`, `hull.py`). Subgradient max-sum
contraction: 0 of 19 boards solved (2,000 rounds per board at 3x3, 3,000 at 4x4,
time-limited to about 1,500 rounds at 5x5 and about 50 at 6x6); best boards use
7 of 9, 11 of 16, 17 of 25 and 20 of 36 distinct pieces. LP over all usage vectors: margin 0
on all 8 boards at 3x3 (2 colours), 2 of 2 at 3x3 (4 colours) that have
reused-piece boards, 3 of 3 at 4x4 (3 colours, 20,369 to 41,325 vectors), and
4 of 6 at 4x4 (4 colours); positive margin on 2 of 6 at 4x4 (4 colours). My
reconstruction may differ from the notebook's, so this refutes linear prices,
not the notebook claim.

None of this says anything about the real puzzle beyond checking the
arithmetic used in section 2.

## 6. First experiment

Measure the freeze point of the uniqueness ladder against the annealed
prediction, on generated 8x8 boards.

- **Boards.** 10 generated 8x8 boards, 3 frame colours, 8 interior colours, and
  10 with 9 interior colours.
- **Method.** Section 1.2 with `P = 10^3`, strips of `w = 3, 4, 5` rows, ten
  sweeps per stage, random ladder order, three seeds per board (one seed for
  `w = 5`). Baseline: the
  repo's strict solver on the same boards at equal CPU time.
- **Measured.** `k*`: the stage at which fewer than 1 percent of strip moves
  change the board. The number of distinct surviving lineages per stage. Whether
  any particle reaches stage 64.
- **Prediction written now** (`costs.py` section 8): `k*` = 12, 25, 36 of 64 for
  `w` = 3, 4, 5 at 8 colours; 1, 12, 21 at 9 colours.
- **Kill.** The approach is dead if, at `w = 4` (the strip-to-board ratio closest
  to what fits in memory at 16x16), the median measured `k*` is below 45 of 64
  on the 8-colour boards, or if the ladder solves fewer than 5 of the 10
  8-colour boards that the strict solver solves in the same time. A measured
  `k*` within 8 of the prediction also confirms the price list in 2.5.
- **Budget.** A strip move at 8x8 with `w = 5` is about 8 * 8^5 * 3 = 8 * 10^5
  operations; 64 stages * 10 sweeps * 4 strips * 10^3 particles is 2 * 10^12
  operations per run, about 0.2 core-hours in compiled code. 20 runs at `w = 5`
  and 60 each at `w = 4` (8 times cheaper) and `w = 3` come to about 6
  core-hours.

## 7. Probabilities

- One year, three desktops, this approach: below 10^-7.
- 1,000 core-years: below 10^-6.

Main reason they are not higher: the reused-piece overcount of e per cell is a
property of the puzzle, not of an algorithm. Every method here either pays it
(10^58 to 10^108) or tracks the used set and becomes the search tree (10^43).
The residual is my chance of being wrong about the freeze estimate by more
than a factor two in `k*`, times the chance that the rest then works.

## 8. Best idea rejected

Colour quotients with exact multiplicities. Merge the 17 interior colours into
2 classes: the 196 interior pieces fall into 6 necklace types, and "each piece
once" becomes "each type exactly `n_t` times", a count vector with roughly 10^7.5
values (hand estimate) in place of 2^196 sets. Frontier 2^17, so the quotient puzzle is exactly
countable on a desktop, with uniqueness intact and colours coarse: the opposite
corner from the tensor network. Rejected because the 2-class quotient has an
astronomical number of solutions and 3 classes already need 24 types (10^20
count vectors), so it cannot be refined toward the real puzzle; as a pruner it
is the cut-multiset idea that AGENDA A17 measured as a small gain.
