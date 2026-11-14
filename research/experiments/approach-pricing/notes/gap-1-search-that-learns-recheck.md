# Review of "Gap 1: search that learns"

Second analysis report, 2026-11-13. The memo was not edited. Scripts and raw outputs:
`notes/gap-1-review/`. One core, `nice -n 10`, about 80 core-minutes in all, no run
over 15 minutes (two runs were stopped at 4 and 5 minutes because they were heading past the
limit, then split into shorter ones). Everything on generated boards is a toy and says nothing
about Eternity II by itself.

Grades used below: **stands**, **stands with a narrower scope**, **wrong**, **unproven**.

## 0. Rulings

| # | Claim in the memo | Ruling |
|---|---|---|
| V | "Empty for a solve. No form of learned reason is worth more than about 2 bits on the real 16x16." | **Unproven** as written. **Stands with a narrower scope**: see the corrected statement below. |
| 1 | Counting bound, graded "exact": T nodes pay only for reasons of at most log2 T bits | **Stands with a narrower scope.** It is a covering lemma for a store bolted onto a row scan, each reason cutting alone. It is not a bound on what a clause-learning solver gains, and inside its scope it excludes nothing by itself (section 1). |
| 2 | Depth bound: 10 bits needs nearly every node cut by free depth 113, 56 bits by depth 52 | **Stands** (recomputed from the committed profile: 29.8, 17.1, 10.7, 6.8, 2.4, 0.5 bits at depths 80, 96, 112, 128, 144, 158). Same scope as claim 1: the row-scan order is kept and a node is either visited or cut. |
| 3 | "What a reason is": window colours plus blocked tiles, monotone in the used set | **Stands** for a depth-first refutation. The bit counts are model shares (uniform colours, uniform used set), not measured shares. |
| 4 | M1: a free perfect store removes 0.48, 1.31, 1.97, 2.49 bits at 20, 40, 61, 80 bits | **Stands.** Reproduced with an independent script: 0.45, 1.25, 1.89, 2.44 (section 3). |
| 5 | M1's assumption: the depth-first reason is close to the smallest one | **Stands for the whole-tree figure, wrong locally.** After a sound greedy shrinking the figures are 0.47, 1.36, 2.11, 2.79. At depths 110 and 111 the share with a reason of at most 61 bits moves from 0.04 to 0.70 (section 2.1). |
| 6 | M2: 0.5 to 0.75 bits, "the best case of a persistent solver" | Numbers not re-run (code read, no fault found). The label is **wrong**: it is the reuse inside one row only. |
| 7 | M3: best case 0.9 bits with 2^91 reasons, nothing under 61 bits | **Unproven.** Model only, not re-run. It prices one proof technique (prove the frontier dead with k tiles released), not every reason that is small in tile-set terms. |
| 8 | Lookahead footnote: a free oracle of height 16 removes 3.9 bits, height 32 removes 6.6 | **Stands** for height 16 (3.79 here). Height 32 not recomputed. |
| 9 | "The best reason I could construct" as a proxy for what conflict analysis finds | **Unproven**, with two toy checks that point the memo's way (section 2.2) and one hole no script touches (section 2.3). |
| 10 | Ben-Sasson and Wigderson: size S gives width at most k + O(sqrt(n log S)); the theorem cannot certify 2^61 here | Attribution **stands** (paper opened). The numbers 590 and 2,600 use a constant of 1; the paper's proof gives 8, so they are 1,660 and 7,300. The comparison with "545 literals" is ill-formed. The conclusion **stands** and is stronger than stated: the corollary is vacuous on this instance (section 5). |
| 11 | "RQ2 as designed cannot kill": a downward trend is guaranteed for any solver that learns anything | **Wrong as stated.** One toy board of five shows no trend over 104 frames. What stands: a trend, when it appears, is not evidence of a gain, and the memo's replacement reading is the right one (section 4). |
| 12 | Section 5 experiment (prefix cores at depth 113) and its kill line | Design **stands with a narrower scope**. The kill line counts nodes at one depth, not bits, and can fire on a result worth 0.2 bits (section 2.1). |
| 13 | Section 6, step 5: no two tiles equal up to rotation, dominance is equality | **Stands** (256 tiles, 256 classes up to rotation, none symmetric; checked). The merge factor is cited from an not independently checked draft and was not checked here. |
| 14 | Section 6, last paragraph: for a pin, proof cost times 2^b stays at the price | **Stands for proofs paid separately.** That is the assumption a learner would break, so it cannot be a step of an argument that learning is empty. |
| 15 | Odds: 10 bits 1%, 56 bits under 0.1% | A judgment, not ruled. Nothing I ran argues for raising them; nothing in the memo derives them. |

**Corrected statement of the verdict.** For reasons that are (a) attached to nodes of the
five-clue row scan, (b) used one at a time as a cut, and (c) obtained from refutations at most
32 cells high: a free perfect store of those of at most 61 bits removes 1.9 to 2.1 bits of
nodes, and no store of such reasons, whatever their size, removes more than 6.6 bits (the
memo's own height-32 oracle). Ten bits therefore needs reasons for subtrees taller than 32
cells at free depth 128 or before. No script, in the memo or here, measures one of those on
the real pieces. On toys of side 4 to 6 a clause-learning solver shows no edge over a tree
that grows with size, and what it carries from one row-scan prefix to the next is worth 0.25
bits at 5,356 prefixes. That is evidence of absence at toy scale, not a proof that the gap is
empty.

## 1. The counting argument: a lemma, in a narrow scope

**What is true.** Take one level L of the row-scan tree. Suppose a search leaves at most T
nodes of L unvisited-but-uncut, and every other node of L is cut by one of at most T stored
predicates, each acting alone. Then the shares of those predicates sum to at least
1 - T/|L|, so the predicates with share below 2^-(log2 T + k) cover at most 2^-k of the
level. That is the memo's sentence, and it is exact.

**Where it stops being a theorem.**

1. *Reasons that act together.* A clause-learning solver does not cut a node only when the
   node falsifies one learned clause. It propagates over all of them. T clauses imply, by
   unit propagation, far more than T conjunctive nogoods, so "T nodes, T reasons" fails. A
   clause about cells that are not yet filled has share zero on the level (it cuts no node)
   and still shrinks the subtree under every node. For a resolution refutation of size T
   there is no need for any node of L to falsify any clause of the proof: the proof
   restricted to the node is a proof for the residual board, nothing more. So neither the
   counting bound nor the depth bound constrains a general refutation.
2. *Another order.* Both bounds are stated on levels of the row scan. A solver that
   branches in another order, or restarts, has no such levels. "Ten bits needs every node
   cut by depth 113" is a statement about methods that keep the row-scan prefix structure.
3. *Reasons that are not conjunctions.* For a cardinality or parity predicate the share is
   still defined and the lemma still holds, but the memo's arithmetic (4.09 bits per colour,
   0.6 to 1.2 bits per blocked tile) no longer says what a 61-bit reason can look like.
   Section 6, steps 3 and 4, enumerate conjunctions over the frontier only. One simple
   parity reason is void: the number of unused half-edges of a colour minus the number on
   the frontier is always even at a row-scan node (derived here). Hall-type reasons are
   reported inert on toys (ROUTES, rows "Hall matching" and the 2008 all-different column of
   the survey; not re-opened by me). No one has run a pseudo-Boolean solver.
4. *Extension variables.* Same as 3, with one more degree of freedom: the search can branch
   on the new variables, which changes the tree itself. No evidence either way. Standard
   solvers do not introduce them.

**Inside its scope it excludes nothing.** "Tile t at cell c" is a reason of 8 to 10 bits
at every level past c. A few hundred of them cover a level. The lemma is satisfied. What prices
them is the cost of proving each, and the memo's accounting there (last paragraph of its
section 6) charges each proof separately. That is tree-like accounting. A learner is, by
definition, the method that does not pay proofs separately.

**The separation, exactly.** Tree-like resolution is what a search that forgets produces.
Clause learning with restarts matches general resolution up to a polynomial (cited in the
nogood review, not opened by me). The families that separate the two have one shape, and the
paper the memo did not open states it (Ben-Sasson and Wigderson, section 1, properties 1 to
4 of the pebbling contradictions): a refutation of width O(1), hence of size O(n), while
every tree has size exp(pebbling number). Their Theorem 3.1 gives the converse for trees:
tree size at least 2^(w - k), with w the refutation width and k the widest axiom. So the
separation is available to an instance exactly when its narrowest refutation is far narrower
than log2 of its best tree, because a proof of width w can only ever need about (2n)^w
distinct clauses and a tree re-derives each at every use.

For this puzzle: the row scan is a tree of 2^134.4, so a refutation of width about k + 145
exists, and that is all the theorem says. For the clause count to fit 2^61 through narrowness
alone, with 156,816 placement variables (2^17.3), nogoods could mention at most three
placements (2^49); four placements already make 2^64. Nogoods on so few placements that hold
without reference to the used set are almost absent away from the clues, which is O3
(propagation sees nothing past the corner windows; measured elsewhere, not here). So every
useful lemma carries a condition on the used set, and the separation applies to this puzzle
exactly to the extent that conflicts depend on few used tiles. Without each tile once it
does apply: the frontier contraction of O1 (2^62 to 2^66 states) is a DAG proof exponentially
smaller than any tree. With it, the memo measured the dependence on the real pieces: a
median of 35 blocked tiles and 53 bits for a 26-node subtree at the peak. After shrinking I
find 45 bits (section 2.1). That measurement is the memo's real argument. The counting bound
is not.

## 2. Is "the best reason I could construct" a fair proxy?

### 2.1 Real pieces: shrinking the memo's own reasons (measured, `r1_real.py`)

For each sampled node I took the memo's reason for its dead parent and shrank it greedily: a
blocked tile is dropped if the subtree is still dead with that tile available again, a window
colour is dropped if the subtree is still dead with any colour allowed there, and a step is
kept only if the bit count falls. Each test is a complete search, so the result is a valid
reason whatever proof one has in mind. Limits: subtrees of at most 1,500 nodes, each test
capped at 3,000 nodes and 32 cells; greedy, so subset-minimal at best, not minimum. This is
the memo's section 5 experiment with the cheapest possible refuter, on the nodes where that
refuter can finish.

| Reasons kept | Memo | Mine, memo's definition | Mine, after shrinking |
|---|---|---|---|
| at most 20 bits | 0.48 | 0.45 | 0.47 |
| at most 40 bits | 1.31 | 1.25 | 1.36 |
| at most 61 bits | 1.97 | 1.89 | 2.11 |
| at most 80 bits | 2.49 | 2.44 | 2.79 |

So the whole-tree price barely moves. Locally it moves a lot. Width-weighted share of nodes
under a parent with a reason of at most 61 bits: depths 100 to 113, 0.58 before and 0.77
after; depths 114 to 140, 0.54 and 0.58; depths 141 to 172, 0.75 and 0.78. At depths 110
and 111 the share goes from 0.04 to 0.70, and the median reason from 86 and 81 bits to 53
and 48.

Two consequences.

- The memo's kill line for its own verdict ("more than half of the nodes that script 02
  leaves uncut receive a reason of at most 61 bits") is almost met on depths 100 to 113 by
  this crude refuter (45% of the uncut nodes, width-weighted), for a whole-tree gain of 0.2
  bits. The kill line is mis-specified. It should be stated in whole-tree bits, or on the
  nodes whose parent's subtree is taller than 32 cells.
- What shrinking cannot reach is the tall subtrees: 11% of nodes at depths 100 to 113 and
  9% at depths 114 to 140 sit under a parent whose subtree passes 32 cells or 60,000 nodes.
  They are counted as uncut by the memo and by me. They are where ten bits would have to
  come from (section 2.3).

### 2.2 Toys: an off-the-shelf solver against the tree (measured, `r2_toy_sat.py`, `r5_cubes.py`)

Generated framed boards, one corner pinned, colour counts at or just past the critical
point by the first moment. Both sides do the complete job: every solution, then the proof
that there is no other. Tree: plain row scan, no propagation (`rowscan.c`). Solver: CaDiCaL
1.5.3 through python-sat on a direct encoding (placement variables, joint-colour variables,
each cell once, each tile once, pairwise).

| Board | Boards | log2(tree nodes / conflicts), median | log2(dead ends / conflicts), median | Wall time, solver / tree |
|---|---|---|---|---|
| 4x4, 2 frame and 3 inner colours | 8 | 5.1 | 3.5 | both under 0.1 s |
| 5x5, 3 and 4 | 8 | 6.1 | 4.7 | both under 0.5 s |
| 5x5, 2 and 5 | 8 | 6.5 | 5.3 | both under 0.5 s |
| 6x6, 3 and 5 | 4 | 6.1 (4.9 to 6.6) | 4.7 (3.4 to 5.3) | 28 to 130 times slower |
| 7x7, 3 and 7 | 1 | not decided | not decided | tree 57 s (2^31.3 nodes, 2^30.1 dead ends, 24 solutions); solver stopped unfinished after 13 min |

Reading. The solver needs far fewer conflicts than the tree has nodes, by a factor of 35 to
90. The factor does not grow from 5x5 to 6x6 (it grows by about one bit from 4x4 to 5x5).
The test named in my plan ("far fewer, with the ratio growing with size") is therefore half
met, and the half that would contradict the memo, the growth, is not seen on three sizes.
Three sizes and four boards at the largest are thin. The one 7x7 board decides nothing: at
the 6x6 rate of about 10,000 conflicts per second (not measured at 7x7) the solver had room
for about 2^23 conflicts before the time rule stopped it, so finishing would have needed a
ratio above 8 bits. All the run shows is that the ratio did not jump by two bits or more.

What this check does not control: the solver propagates and chooses its own order, and the
tree does neither. The repo's independently assessed figure for propagation alone against plain search is a
node lead of 5 to 16 with no growth from 6x6 to 8x8 (ROUTES, row "Propagating DFS"; not
re-measured by me), which is the size of the dead-end ratio above. To separate learning from
propagation with the same solver I ran a second check.

**Sharing between row-scan prefixes** (`r5_cubes.out`, one 6x6 board, 32 solutions, 191,692
conflicts for the whole board). The board is split into every valid row-scan prefix of k
cells. "Persistent" is one solver fed the prefixes as assumptions, in depth-first order.
"Fresh" is a new solver for each prefix, so nothing learned under one prefix serves under
another.

| k | Prefixes | Conflicts, persistent | Conflicts, fresh | log2(fresh / persistent) | log2(fresh / whole board) |
|---|---|---|---|---|---|
| 3 | 22 | 168,206 | 165,293 | -0.03 | -0.21 |
| 6 | 474 | 241,100 | 279,899 | 0.22 | 0.55 |
| 7 | 2,054 | 302,625 | 345,280 | 0.19 | 0.85 |
| 8 | 5,356 | 307,496 | 366,069 | 0.25 | 0.93 |

Cutting every channel between 5,356 branches costs the solver 0.25 bits against keeping
them, and 0.93 bits against letting it choose its own order on the whole board. On this
board, what the solver learns in one branch of the row scan is worth almost nothing in
another. That is the memo's claim, measured with a real solver, on one toy.

### 2.3 What neither the memo nor this review measures

No clause-learning solver has seen the real pieces. The toy checks stop at 6x6, where the
whole tree is 2^23 to 2^26 nodes and a frontier is 6 colours. The memo's own argument for
tall subtrees is M3 (a model of one proof technique) and step 3 of section 6 (arithmetic,
not scripted; my hand estimate of the unanchored-pair figure is 2^-23 against the memo's
2^-25, same order). The memo says this itself in step 6 and in appendix C. The verdict
"empty" is stronger than what those two paragraphs carry.

## 3. Reproduction on the official pieces

`r1_real.py` does not import the memo's `e2real.py`. It reads the CSV and the rotation
convention from the Track 10 crate, rebuilds the scan order and the node definition, and
runs its own population sampler (100,000 particles, four runs covering depths 80 to 185,
200 nodes per depth, against the memo's 200,000 and 1,500).

- Committed profile: total 2^134.39, widest level at free depth 158. As in the memo.
- Level widths: within 0.03 dex of Track 10 on depths 80 to 116, within 0.064 up to depth
  150, within 0.10 up to 172, 0.24 at depth 176, and up to 0.94 beyond (the population
  collapses to a handful of distinct nodes; those depths hold about 1% of the tree). The
  memo reports 0.09 up to depth 176 with twice the particles. Consistent.
- Bits removed: table in section 2.1. All four of the memo's figures are reproduced within
  0.08 bits.
- Per-depth shares (memo's appendix B, at most 61 bits): depth 100, 0.84 against my 0.865;
  depth 113, 0.83 against 0.86; depth 128, 0.41 against 0.47; depth 158, 0.61 against 0.62.
  Within sampling noise at 200 nodes.
- The memo's "0.30, 0.50, 0.76 on depths 80 to 113, 114 to 140, 141 to 172" are unweighted
  means over depths (mine: 0.29, 0.49, 0.76). Weighted by level width the first is 0.58,
  because the wide levels of that range are the ones next to the centre clue. The memo
  should say which mean it reports.
- Free oracle of height 16: 3.79 bits against the memo's 3.94.

The per-part summary blocks at the end of `r1_real.a.out` to `.d.out` are not whole-tree
figures. Read `r1_merge.out`.

Not reproduced: M2 (`03_row_memo.py`), M3 (`04_generalise.py`), the height-32 oracle, parts
B, D and E of `01_ceilings.py`.

## 4. RQ2

The memo's claim has three parts.

**"Frames that share the 44-cell bottom arc differ only in the top side."** Stands. It has a
consequence the memo does not draw (`r4_top_side.py`, real border pieces, leftover set drawn
uniformly, which is an approximation): the top row is a path through all 14 leftover edge
pieces, so of 400,000 random leftover sets only 81 (0.02%) admit any valid top row, and
those that do admit a median of 720 (mean 1,036, range 96 to 6,912). The mean over all arcs
is 0.21 (uniform arithmetic: 14! x 2 / 5^17 = 0.23). So "1,000 frames sharing a 44-cell arc"
is about the complete family of one extendable arc, all built from the same 16 border
pieces in different orders. The design is feasible, and it has no more than about a thousand
frames to offer per arc.

**"A downward trend is guaranteed for any solver that learns anything."** Wrong as stated.
Toy check (`r3_rq2_toy.py`, five generated 9x9 boards, Glucose 4, arc = bottom row and side
cells below row 1, every valid frame that extends the planted arc, shuffled):

| Board | Frames | Mean conflicts per frame, first block to last block of 10 frames (14 on board 2), persistent solver | Total, persistent | Plain fill under the arc | Fresh against persistent, first 30 frames |
|---|---|---|---|---|---|
| 1 | 72 | 628 to 26 | 13,173 | 18,804 | 509 against 341 |
| 3 | 104 | 117, 275, 256, 437, 105, 172, 290, 185, 66, 164 | 21,414 | 34,205 | 273 against 216 |
| 4 | 48 | 64 to 11 | 1,844 | 2,136 | 119 against 55 |
| 5 | 72 | 9 to 0 | 158 | 121 | 11 against 4 |
| 2 | 144 | 20 to 0 | 578 | 1,094 | 19 against 10 |

Board 3 shows no trend over 104 frames with a solver that does learn (it needs a fifth
fewer conflicts than a fresh solver on the first 30 frames). So RQ2's kill line can fire. A row-order refuter would share
everything below the top row; a clause-learning solver given the whole frame as assumptions
propagates from the top side at once and need not.

**"That trend is row scan found again; read the total against the plain fill under the
arc."** Stands, and the toys support it. On four boards of five the cost per frame falls,
and on all five the persistent total over every frame of the arc is 0.5 to 1.3 times one
plain fill under the arc by the same solver. The fall is amortisation of the shared part,
not a gain over it.

**Corrected statement.** RQ2's kill line is not blind, it is non-diagnostic in the other
direction: a downward trend is the expected outcome whether or not learning beats the plain
fill, so "survives" would mean nothing. Replace the kill line by the memo's reading: total
cost over all frames of one arc, against one plain fill under that arc by the same solver,
kill if the ratio is above a line fixed in advance (1/2 would do). Boards 2 and 5 are trivial (most frames die with no
conflict) and carry little weight. The planted arc is on a toy with 3 frame colours, where
frames that pass the corners are far more common than on the real board.

## 5. Ben-Sasson and Wigderson

Opened: the ECCC report TR99-022 of June 1999, the preprint of the JACM paper
(https://eccc.weizmann.ac.il/report/1999/022/). Pages 1 to 9 read. The JACM text itself was
not opened; the theorem numbers below are the preprint's.

- Theorem 3.1: w(F |- 0) <= w(F) + log S_T(F). Corollary 3.1: S_T(F) >= 2^(w(F |- 0) - w(F)).
- Theorem 3.2: w(F |- 0) <= w(F) + O(sqrt(n ln S(F))). Corollary 3.2:
  S(F) = exp(Omega((w(F |- 0) - w(F))^2 / n)). n is the number of variables, w(F) the widest
  initial clause, S and S_T the smallest general and tree-like refutations.

So the attribution in the memo is correct. Three corrections to what is built on it.

1. *The constant.* The proof of Theorem 3.2 sets d = ceil(sqrt(2 n ln S)) and ends at width
   k + d + b with b about sqrt(2 n ln S), so the form is S >= exp((w - k)^2 / (8 n)). The
   memo's "590" and "2,600" are computed with 1 in place of 8 (its script says so, its text
   does not). With 8: 1,660 literals on 8,160 variables and 7,300 on 156,816.
2. *The comparison.* "A row-scan state clause has at most 545 literals" mixes two
   encodings (17 seams times 17 values, plus 256 tile literals) and compares a clause width
   with a width above k. The clean form: in the placement encoding k is 764 to 784 (the
   clause "this cell holds some tile"), and the row-scan refutation has width at most
   k + 256. Then Corollary 3.2 gives S >= exp(256^2 / (8 x 156,816)) = 1.05. The paper says
   as much on its page 9: when w(F) is of the order of the number of variables the
   corollaries are useless.
3. *What was left out.* The half of the paper that bears on this gap is the other one:
   Theorem 3.1 and the pebbling family (general size O(n), width O(1), tree size
   exponential). It is the precise statement of when a learner beats a tree, and it is used
   in section 1 above.

The memo's sentence "the theorem cannot certify a bound of this size here" stands. It
should read "the theorem certifies nothing here".

Not opened by me: Smith 2005, Chu et al. 2012, Lecoutre et al. 2007, the handbook chapter,
Pipatsrisawat and Darwiche, Atserias et al. The memo's use of the first two was not checked.

## 6. Smaller points

- Section 0, "Why": "at those depths the only reasons that small are those a depth-first
  search already finds within a few cells" is the conclusion of steps 3 and 4 of section 6,
  both in a uniform model, one not scripted. It should be graded guessed or derived, not
  listed beside two measured facts.
- M2 measures deaths inside one row only (a node is alive if the row completes). A
  persistent solver also keeps reasons across rows. The toy table of section 2.2 is an
  independent sign that the total is small, at 6x6.
- M3's "independently re-checked ceiling, 7.8 bits at 10^38 entries" is O1's two-list floor. It is a
  ceiling for region tables, not for reasons.
- Section 5: "300 clean nodes certify at most log2(300 / 3) = 6.6 bits" is a fair rule of
  three.
- Budget arithmetic: 10^15 is 2^49.8 and 1,000 core-years at 1e8 per second is 2^61.5.
  Correct.

## 7. Left open, each worth a study

1. **Tall subtrees on the real pieces.** The only place ten bits can come from. Take the
   nodes at depths 100 to 128 whose parent's subtree passes 32 cells (9 to 11% of nodes,
   up to 48% just after the centre clue), give a clause-learning solver the residual board
   with the frontier colours and the used tiles as assumptions, and read the failed set and
   the conflicts. Kill line in whole-tree bits. This is the memo's section 5 with its
   target corrected; the short-subtree half is done here.
2. **Toy scaling past 6x6 with a propagation control.** Tree size by the Track 10
   estimator, conflicts with a budget of hours, at 7x7 and 8x8, and a third arm that
   propagates like the solver and does not learn. My check has no such arm.
3. **Prefix sharing at 7x7 and 8x8** (`r5_cubes.py` scaled up, more boards, deeper
   splits). It is the toy form of assumption A2 and of RQ2, and it is cheap. A slope of
   log2(fresh / persistent) against log2(prefixes) well above the 0.02 seen here would
   reopen the gap.
4. **A solver with native cardinality** (pseudo-Boolean, exactly-one per cell and per tile
   as constraints) on the same toys. The counting lemma is silent on such reasons and
   nothing in the repo has run one.
5. **RQ2 rewritten** with the total-against-plain-fill reading, and with the family size of
   an extendable arc (about a thousand frames) as a design fact.
6. **M3 on a small board**, as the memo's own appendix C asks.

## Appendix. Scripts and runs

All in `notes/gap-1-review/`. Python as `python3 -I` or
`uv run --with python-sat python3`.

| Script | What | Output | Time |
|---|---|---|---|
| `r1_real.py 100000 200 SEED 3000 d0 d1 1` | real pieces: sampler gate, memo's reasons, shrunk reasons | `r1_real.a.out` (80 to 112, seed 7), `.b.out` (113 to 148, seed 8), `.c.out` (149 to 166, seed 9), `.d.out` (167 to 185, seed 10) | 5, 8, 3, 1 min |
| `r1_merge.py` | whole-tree figures from the four parts | `r1_merge.out` | 1 s |
| `rowscan.c`, `toylib.py` | toy tree counter (build: `cc -O2 -o rowscan rowscan.c`, binary not kept), toy generator and encoding | | |
| `r2_toy_sat.py SOLVER n cf ci seed0 nseeds [budget]` | toy tree against solver | `r2_toy_sat.n4n5.out`, `.n6.out`, `.n7.out` (stopped by its time limit, no result line) | 1, 5, 14 min |
| `r3_rq2_toy.py` | RQ2 design on toys | `r3_rq2_toy.out` | 3 min |
| `r4_top_side.py 400000 1` | valid top rows per 44-cell arc, real border pieces | `r4_top_side.out` | 1 min |
| `r5_cubes.py cadical153 6 3 5 3 6000 3 6 7 8` | sharing between row-scan prefixes | `r5_cubes.out` | 6 min |

What I did not check: M2 and M3 by re-running; the memo's sources other than Ben-Sasson and
Wigderson; the merge factor cited from `gap-6a`; the real share of a reason among real nodes
(both the memo and this review use the uniform model for bits); any solver other than
CaDiCaL 1.5.3 and Glucose 4; any solver on the real pieces. The toy boards have 16 to 640
solutions even past the first-moment critical point, because small boards with few colours
carry duplicate tiles; they are near-critical by the first moment only.
