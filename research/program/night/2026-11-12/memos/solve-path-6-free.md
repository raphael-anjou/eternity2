# Solve path 6 (free lens): projection search with wide exact projections

Night 2026-11-12. One researcher, no assigned lens. Scripts and raw output:
`research/program/night/2026-11-12/memos/solve-path-6/` (`dm.py`, `run.sh`,
`profile.py`, `profile.out`, `results.txt`).

**Bottom line first.** The approach below is the only non-enumerative method
I found that is fully specified, exact at its fixed points, and unmeasured on
this puzzle. I measured it tonight on 4x4 to 8x8 boards. It works on all of
them, and its iteration count sits at 0.05 to 0.6 times the node count per
solution of a naive backtracker on the same instances. Extrapolated on that
evidence it is about 10^33 short of the real board at 1,000 core-years. One
measurable property could change that (the exponent falling with the width of
the exact projections). I give it a few percent. My probability of a full
solution is 10^-6, and I explain in section 2 why I think every method that
does not find a new exponent or a flaw in the instance faces the same wall.

## 1. The approach

**Object.** A point x in R^(256 x 4 x 23): one real vector over the colours
(grey included) for each of the four sides of each cell. A board is a 0/1
point (one-hot colour per side). Four sets:

- **A, layouts.** One-hot points where the 256 cells hold the 256 pieces, each
  once, in some rotation, grey on the rim. Seams are ignored.
- **R_k, bands.** One-hot points where every horizontal band of k rows is a
  fully matched tiling by oriented pieces, repeats allowed. K_k is the same for
  columns. Piece uniqueness is ignored.
- **B, seams.** Points where the two vectors facing each other across a seam
  are equal and the rim is grey. A linear subspace.

A solved board is exactly a point of A that lies in B. R_k and K_k contain
every solution, so adding them keeps the intersection.

**Projections, all exact.**

- P_A: one linear assignment. Score(cell, piece) = max over allowed rotations
  of the sum over sides of x[cell, side, colour of that side]. Hungarian or
  auction on a 256 x 256 matrix. Clue cells have a one-piece domain.
- P_R_k: a max-plus transfer matrix along the band (Viterbi). For k = 1 the
  state is the colour of the last east side: O(16 x 1024) per row. For k
  rows use a broken profile: state = the k frontier colours, 17^k states,
  about 2.7 transitions per state and cell (784 oriented interior pieces over
  17^2 colour pairs). k = 4 costs about 6 x 10^7 operations per sweep, k = 5
  about 10^9.
- P_B: average the two facing vectors on each seam, write grey on the rim.

**Iteration.** Divide and concur with one replica of x per non-linear set
(A, R_k, K_k). Divide D = project each replica on its set. Concur C = average
the replicas, then P_B. Update x <- x + beta (D(2C(x) - x) - C(x)) with
beta = 0.25 (the relaxed form; beta = 1 is plain Douglas-Rachford). Stop
when the layout produced inside D has zero mismatched seams, which is checked
exactly. There is no objective and no temperature. A fixed point is a
solution and nothing else is.

**Data structures.** x as a dense float array (256 x 4 x 23 per replica), a
table colour[state][side] for the 1,024 oriented pieces, a validity mask
(cell, state), a warm-started assignment solver, and per band a max-plus DP
table with back-pointers. `dm.py` is a 200-line numpy reference of k = 1.

## 2. Cost model for the real 16x16

**Per iteration (assumed, compiled code, one core, not measured).** Scores:
256 x 1,024 x 4 = 10^6 operations. Assignment: 10^6 to 10^7 with warm start.
Rows and columns, k = 1: 5 x 10^5. Total about 3 ms, so 300 iterations per
second. k = 4: about 0.3 s per iteration. Memory: a few megabytes (k <= 4).
Measured in numpy tonight: 6,500 it/s (A only, 6x6), 600 it/s (A + rows +
columns, 8x8).

**Budgets.** Three desktops for a year (assumed 48 cores): 1.5 x 10^9
core-seconds, 4.5 x 10^11 iterations. 1,000 core-years: 3.15 x 10^10
core-seconds, 10^13 iterations.

**Iterations needed.** I have no theory for it. The measured relation
(section 5) is I = rho x N, where N is the backtracker's nodes per solution
and rho was 0.05 to 0.6 at n = 5, 6, 7. For the official board `profile.py`
gives N = 10^47.7 for a raster row scan (first-moment model with exact-census
seam weights; it reproduces 14,702 solutions with the starter pinned, the
figure in `memos/theory-referee/01_census_first_moment.out`, committed). So
I is about 10^46 to 10^47, against 10^13 affordable: **33 to 34 orders of
magnitude short at 1,000 core-years, 35 on three desktops.** Assumptions:
rho does not grow with n; the first-moment tree is the right N; 3 ms per
iteration.

If I take the one favourable trend in the data at face value (rho of the
row-and-column variant fell from 0.19 to 0.079 to 0.045 over n = 5, 6, 7,
a slope of -0.027 per cell on seven instances with a naive baseline and
uncontrolled colour counts), I get 10^41 iterations. Still 28 orders short.

**What would remove most of the orders.** One sub-result: the exponent of I
falls with the band width k, roughly as 1/k. The mechanism would be that an
exact k-row projection removes the seams inside bands from the search, so the
map only has to agree on band interfaces and on piece uniqueness. With
I = N^(1/4) at k = 4 the count is 10^12 iterations at 0.3 s each, which is
10^11.5 core-seconds: ten times the 1,000 core-year budget, reachable with
engineering. I put this at 2 to 3 percent. Reasons against: piece
uniqueness is the constraint that carries the hardness (relaxed interior
tilings per frame number 10^52.9 and only a fraction 10^-83.6 of piece
multisets are all different, `profile.out`), and band projections do not touch
it. Reason for: k = 0 to k = 1 already cut iterations by 2 to 14 on the
harder instances tonight.

**Why I did not choose a reformulation of the enumeration.** Three
calculations, all from `profile.py` or elementary:

1. Any scan has branching r x 4 x p^2 per interior cell (r pieces left,
   p = 1/16.78 per seam). It exceeds 1 until 70 pieces remain. The interior
   hump is 10^32.9 and the full row scan costs 10^47.7 nodes per solution.
   Geometry cannot change this: every region has two seams per cell.
2. Restricting piece pools obliviously does not help. Let cell i be allowed
   only a(i) <= 196 - i pieces, chosen by a random priority order. A solution
   survives with probability prod a(i)/(196 - i), and the prefix count at depth
   i shrinks by prod_{j<i} a(j)/(196 - j). Cost per expected solution at depth
   i is the unrestricted prefix count times prod_{j>=i} (196 - j)/a(j), never
   below the unrestricted peak. Zoning pieces to regions, colour-class
   splitting and random sub-pools are all instances of this.
3. Handling uniqueness implicitly (inclusion-exclusion or fingerprinting over
   a colour-frontier transfer matrix) costs 2^196 = 10^59 times the frontier,
   more than the tree.

So a win has to come from information about the instance (guidance, design
flaws) or from a method whose exponent is not the tree's. The projection map
is a candidate of the second kind because nothing forces its running time to
follow the tree. Tonight's data say that at k <= 1 it does follow it.

## 3. Why it is not already known to fail

- **Published.** Aragón Artacho, Borwein and Tam, "Recent results on
  Douglas-Rachford methods for combinatorial optimization problems", J. Optim.
  Theory Appl. 163 (2014) 1 to 30, arXiv:1305.2657 (PDF opened): TetraVex
  "up to size 4 x 4 could be solved in an average of 200 iterations", citing
  P. Bansal's 2010 report. That is the largest published edge-matching result
  for this family that I found. Gravel and Elser, "Divide and concur",
  Phys. Rev. E 78, 036706 (2008), arXiv:0801.0222 (abstract opened): 3-SAT
  with scaling similar to WalkSAT, and sphere packing; no edge matching.
  Elser, Rankenburg and Thibault, "Searching with iterated maps", PNAS 104
  (2007) 418, doi:10.1073/pnas.0606359104: I could not open it tonight (HTTP
  403, and a captcha on the PMC copy), so what it does with edge matching is
  known to me only through `web/content/research/build/exact/iterated-maps.mdx`.
  Elser, "The complexity of bit retrieval", arXiv:1601.03428 (abstract
  opened): relaxed-reflect-reflect is the best known method on a problem with
  a unique planted solution, which is why a needle is not by itself fatal.
  Kovalsky, Glasner and Basri, "A global approach for solving edge-matching
  puzzles", SIAM J. Imaging Sci. 2015, arXiv:1409.5957 (abstract opened): a
  polynomial system with a sequence of convex relaxations; a different method.
- **On the site.** `iterated-maps.mdx` states that nobody ran the difference
  map on the full puzzle or the 10x10 ladder, and that the single test was a
  hill-climber on graph colouring. The audit grades the row "thin". The
  nearest dead end is "Gradient descent on a soft board" (336/480, four seeds,
  uncommitted): a descent on a score with rounding. This map has no score and
  no rounding step; its only resting points are exact solutions.
- **What differs here.** Band and column projections by transfer matrix
  (the page and the published formulations use the layout set and per-seam
  sets only, as far as I could check without Elser's code); a relaxed step;
  and a measurement against a backtracker on the same instances.

It is known to be suspected of failing: the page quotes a list member who
expects the iteration count to be of the order of a backtracker's node
count. Section 5 supports that expectation at k <= 1.

## 4. What is new

The wide exact projections (R_k, K_k) inside divide and concur for edge
matching, and the hypothesis that the exponent depends on k. The cost model
tied to measured rho. The oblivious-restriction lemma of section 2, if the
program does not already have it (I did not find it). The two-set map itself
is not new.

## 5. Evidence produced tonight

Planted boards with a ring/interior colour split, no clue pinned, pieces
distinct up to rotation (`dm.py gen`). Baseline: naive row-scan backtracking,
full tree, nodes divided by solutions. About 8 CPU-minutes on one core, 2.6 of
them lost to a generator loop. Full output in `results.txt`. Medians of 3 to
5 random starts, beta = 0.25:

| n | colours (ring, other) | seed | DFS nodes per solution | A only | A + rows + columns |
|---|---|---|---|---|---|
| 5 | 3, 5 | 0 | 1,303 | 195 | 351 |
| 5 | 3, 5 | 1 | 8,823 | 5,657 | 2,287 |
| 5 | 3, 5 | 2 | 4,527 | 937 | 465 |
| 6 | 4, 8 | 0 | 11,977 | 1,246 | 970 |
| 6 | 4, 8 | 1 | 4,386 | 322 | 299 |
| 6 | 4, 7 | 0 | 28,842 | 5,506 | 2,613 |
| 6 | 4, 7 | 1 | over 300,000 (capped) | 2 of 3 runs over 60,000 | 4,226 (1 of 3 over 20,000) |
| 7 | 4, 11 | 1 | 55,323 | 17,136 (1 of 3 over 50,000) | 2,515 |
| 8 | 5, 12 | 0 | not run | not run | 14,948 and 20,736 |

Findings:

- Every instance tried was solved exactly, up to 8x8 (one instance, two
  starts). The published figure for this family is 4x4.
- Plain Douglas-Rachford (beta = 1) cycles: on 5x5 seed 1 all five starts
  stalled at 2 or 3 mismatches for 20,000 iterations. beta = 0.25 removed
  the stall on that instance (5 of 5 solved).
- Iterations track the backtracker's nodes per solution. Ratio for A only:
  0.15 to 0.64 (n = 5), 0.07 to 0.19 (n = 6), 0.31 (n = 7). With rows and
  columns: geometric mean 0.19, 0.079, 0.045.
- Rows and columns cut iterations by 2 to 14 on the five harder instances
  and did not help on the three easy ones (0.6 to 1.3 times), at 5 times the
  cost per iteration in numpy.

This contradicts the hope that the map escapes the tree at k <= 1. It does
not contradict the k hypothesis, which was not tested (k = 1 only). Limits:
8 instances with a reference, colour counts not on a controlled critical
ladder, a baseline without propagation or cell ordering (a better baseline
raises rho), and an iteration costs 10^3 to 10^4 backtracker nodes. Nothing
here is evidence about the 16x16.

## 6. First experiment (under 10 core-hours)

Port `dm.py` to the engine crate with k = 0, 1, 2, 3 (broken-profile
Viterbi). Instances: the ladder's critical rungs R6, R7 and 8x8, 30 test
seeds each, preregistered. Measure median iterations to an exact solution
(censored at a fixed cap) and `bb-mrv` nodes to a first solution on the same
seeds. Fit log10 I = a + s_k x cells for each k, and log10 nodes = a' + s_0'
x cells.

**Kill, written before the run.** The approach is dead if either holds:
(a) s_3 > 0.5 x s_0' (the map's exponent at the widest band is not below half
of the backtracker's); (b) s_k does not fall by at least 25 percent from k = 0
to k = 3 with a bootstrap 95 percent interval excluding zero change. Passing
both does not show the 16x16 is reachable; it buys a second stage at 9x9 and
10x10 with k = 4.

## 7. Probabilities

- One year, three desktops, no paid compute: **10^-6**.
- 1,000 core-years: **3 x 10^-6**.

They are nearly equal because a factor 20 in compute is nothing against 33
orders. Both are the probability that the exponent falls with k steeply
enough (2 to 3 percent for a real effect, far less for one large enough),
times the chance that it survives from 8x8 to 16x16. The main reason they are
not higher: at three sizes the map's iteration count followed the
backtracker's node count, and the real board needs a departure of 10^33 from
that relation.

## 8. Two ideas considered and not chosen

**Generator archaeology.** The board was painted from a solved grid by a
program written around 2006. If the painter drew colours or shuffled pieces
with a seeded generator of 32 to 48 bits, the solution is recoverable by
enumerating seeds and comparing the resulting piece multiset with the official
one (10^10 to 10^14 cheap trials per candidate recipe). The unknown is the
recipe, not the compute. Not chosen: it belongs to the designed-instance lens,
and I have no data on the generator. It is the only route I see with a
probability above 10^-4.

**Cavity-guided limited-discrepancy search.** Belief propagation on cell
marginals with exact matching marginals for uniqueness and exact frame
counts, used to rank children; search by discrepancies. Cost: sum over
k <= D(1 - q) of C(D, k) x 1.7^k marginal computations for D = 124 critical
decisions and ranking accuracy q. q = 0.97 gives 10^8 runs, q = 0.90 gives
10^19. Chance ranking is q = 0.37. Not chosen: with 14,702 expected
solutions the planted one is statistically hidden, marginals over the
solution set are flat to about 11 percent, and I expect q near chance. It is
a guided depth-first search, which the brief excludes.
