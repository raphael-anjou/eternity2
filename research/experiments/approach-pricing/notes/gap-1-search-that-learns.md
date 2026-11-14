# Gap 1: search that learns

> **Status (2026-11-13):** independently re-checked, see `notes/gap-1-search-that-learns-recheck.md`.
> **Re-check's rulings:**
> - Verdict ("empty for a solve, no form of learned reason is worth more than about 2 bits"): unproven as written, stands with a narrower scope. Corrected: for reasons attached to nodes of the five-clue row scan, used one at a time as a cut, and obtained from refutations at most 32 cells high, a free perfect store of those of at most 61 bits removes 1.9 to 2.1 bits of nodes, and no store of such reasons, whatever their size, removes more than 6.6 bits. Ten bits needs reasons for subtrees taller than 32 cells at free depth 128 or before, and no script measures one on the real pieces. "That is evidence of absence at toy scale, not a proof that the gap is empty."
> - Counting bound ("exact"): stands with a narrower scope. It is a covering lemma for a store bolted onto a row scan, each reason cutting alone, not a bound on what a clause-learning solver gains.
> - M1's assumption (depth-first reason close to the smallest): stands for the whole-tree figure, wrong locally. After a sound greedy shrinking the figures are 0.47, 1.36, 2.11, 2.79 bits; at depths 110 and 111 the share with a reason of at most 61 bits moves from 0.04 to 0.70.
> - M2 (0.5 to 0.75 bits, "the best case of a persistent solver"): the label is wrong, it is the reuse inside one row only.
> - M3 (best case 0.9 bits with 2^91 reasons): unproven, one proof technique priced, model only, not re-run.
> - "The best reason I could construct" as a proxy for what conflict analysis finds: unproven.
> - Ben-Sasson and Wigderson numbers 590 and 2,600 use a constant of 1; the paper's proof gives 8, so they are 1,660 and 7,300. The comparison with "545 literals" is ill-formed; the conclusion stands and is stronger than stated.
> - "RQ2 as designed cannot kill": wrong as stated. One toy board of five shows no trend over 104 frames; a trend, when it appears, is not evidence of a gain.
> - Section 5 experiment: design stands with a narrower scope; the kill line counts nodes at one depth, not bits, and can fire on a result worth 0.2 bits.
> - Odds (10 bits 1%, 56 bits under 0.1%): a judgment, not ruled.
> **Read with:** `notes/gap-1-search-that-learns-recheck.md` (its section 0 gives the corrected verdict). The memo's "two-list floor of O1, 2^126.6" is not a floor: see `notes/gap-2-lossy-representations-recheck.md` (ruling 5) and `surveys/DERIVATIONS-recheck.md` (W6).

Arc 2026-11-13, Desk study, finished 13:40 by the clock. Draft until a second analysis has ruled.
One core, about 9 core-minutes of stdlib Python in all, longest run 3 min 18 s.
Scripts and outputs: `notes/gap-1/`. Track D1 had no result when this was written.
What I could not check is in appendix C.

Grades: **measured** (a script here, on the official pieces), **derived** (argued here, with a
script where a number is attached), **cited** (a source I opened), **guessed**.

## 0. Verdict

**Empty for a solve.** No form of learned reason I could construct is worth more than about
2 bits on the real 16x16, before paying for the lookups. Three numbers carry the verdict.

1. A free and perfect store of every reason of at most 61 bits removes **2.0 bits** of nodes;
   at most 40 bits (what a machine can hold), **1.3 bits** (measured, real pieces, 106 depths).
2. A learner that keeps its reasons only while the search stays under one row removes
   **0.5 to 0.75 bits** on the rows that carry the price (measured, real pieces).
3. Making a reason small in tile-set terms costs more proof than it returns in reuse at every
   depth where a cut is worth anything; the best case in the model is **0.9 bits with 2^91
   stored reasons**, and nothing under a 61-bit limit (derived, first-moment model).

Why: 10 bits needs nearly every node cut by free depth 113 and 56 bits by depth 52
(measured profile), a budget of T nodes pays only for reasons of at most log2 T bits, and
at those depths the only reasons that small are those a depth-first search already finds
within a few cells.

My odds: 10 bits, 1%. 56 bits, under 0.1%.

## 1. The gap, precisely

The measured 2^134.4 is a row scan that forgets. The gap is any search that records why a
subtree died and cuts later nodes that contain the same reason.

What existing evidence covers.

- `nogood-frame-search-price.md` and its review: reasons on **border cells only**, with a
  refuter that fills cells in a fixed order. Result: such a search is a tree search in the
  order "core first", and row scan is the member with nested bottom arcs. It prices no
  reason that mentions interior seams or used tiles, and its section 5 names the hole: a
  solver kept alive between proofs (assumption A2).
- SAT and CP survey, D5: a prediction that conflicts stay within a small factor of nodes.
  Track 6: 13.5% of boundary refutations reusable, on a 7x7 toy.
- RQ2: never run. D1: reasons on frames, fresh solver per frame, blind to reuse by design.

What nothing covered before today: the size, in bits, of the reason for a dead node of the
real row-scan tree, and what a store of such reasons would cut.

## 2. Two facts that frame everything (both exact)

**Counting bound (derived; the covering bound of the nogood memo, applied to tree levels).**
Call b(v) = -log2 of the share of the nodes of a level that contain reason v. A search that
visits T nodes learns at most T reasons, and each cuts at most a share 2^-b of the level.
So a level is cut only by reasons with b at most log2 T: **61 bits** for 1,000 core-years,
**50 bits** for 10^15. The bound counts reasons ever learned, so forgetting does not help.

**Depth bound (measured, `01_ceilings.out` part A, Track 10 widths).** Removing everything
below free depth d for nothing saves 0.5 bits at d = 158, 2.4 at 144, 6.8 at 128, 10.7 at
112, 17.1 at 96, 29.8 at 80. So 10 bits needs nearly every node cut at depth 113 or before,
20 bits at 91, 56 bits at 52. A reason that fires at the peak is worth under one bit however
cheap it is.

**What a reason is here (derived).** A row-scan node is (frontier: 16 seam colours and one
left colour, about 66 bits; used set U). Its subtree depends on nothing else. A dead subtree
of height h reads the left colour and the next h + 1 frontier colours (at most 16, whether
or not it wraps to the next row), and it depended on a set R of used tiles: those that
fitted some cell of the refutation but were taken. The reason is "these colours, and R
inside U". It is monotone in U, so more general than a clause that names where each blocked tile sits. Its bits: log2 17 = 4.09 per inner
colour, and -log2 P(R inside U), which is 0.6 to 1.2 bits per tile at depths 158 to 112
(`01_ceilings.out` part D). The tile part is cheap per tile. The window is what costs:
4.09 (h + 2) bits, so 61 bits allows height 12 at most and 40 bits height 7.

## 3. Literature

- **Caching states in permutation problems.** Smith, CP 2005
  (https://www.dcs.gla.ac.uk/~pat/cpM/jchoco/rehersalProblem/smith2005.pdf, opened, text
  read). Key = (set of values used so far, current value). Backtracks fall by 60% to 75% on the
  hardest unsolvable instances of a card game, because many orders of one card set reach
  the same state. Here two arrangements of the same
  tile set with the same outline almost never exist (cited from today's draft
  `gap-6a-reformulation-algebra.md`, script output `merge_real.out`: merge factor 1.000000
  to 1.0003 on real-tile regions up to 3x3; not independently checked).
- **Subproblem dominance by projection keys.** Chu, Garcia de la Banda, Stuckey,
  Constraints 2012 (https://users.monash.edu.au/~mbanda/papers/constraints12.pdf, opened).
  For all-different the sound key is the set of values taken by the fixed variables
  (their Example 4, key2). That is the used set: the literature's own answer to "a key
  smaller than the state" is O1's state. They note lazy clause generation learns reasons
  that mention only the constraints involved in the failure, which is the reason of
  section 2.
- **Transposition tables for CSP.** Lecoutre, Sais, Tabary, Vidal, AAAI 2007
  (https://mlanthology.org/aaai/2007/lecoutre2007aaai-transposition, abstract only):
  records reduced states to cut similar subnetworks. No edge-matching data. Unverified
  beyond the abstract.
- **Clause learning and resolution.** As in the survey (handbook chapter 7, not opened by
  me today): a clause-learning solver is bounded below by the shortest resolution
  refutation, and no lower bound applies to this instance.
- **Width against size.** Ben-Sasson and Wigderson, JACM 2001: a refutation of size S gives
  one of width at most k + O(sqrt(n log S)). Known to me through a search summary only,
  **unverified**. Numbers (`01_ceilings.out` part E, derived): to certify 2^61 the
  refutation width would have to exceed k by 590 literals on a seam-colour encoding and by
  2,600 on a placement encoding, while a row-scan state clause has at most 545 literals.
  The theorem cannot certify a bound of this size here.

## 4. Mechanisms, priced

### M1. A global store of small reasons

Algorithm. Row scan. When a subtree rooted at scan position p dies with height h, store
(p's column, left colour, the h + 1 frontier colours read, R as a bit mask) if its bits are
at most B. At each node, look up the column and colours for h = 0 to H, and cut when a
stored R is inside the current used set. Reasons are valid in any interior row without a
clue.

Price (measured, `02_real_reasons.py`, `01_ceilings.out` part C). I sampled the real
five-clue tree at every free depth from 80 to 185 by SMC (200,000 particles, 1,500 nodes per
depth; level widths within 0.09 dex of Track 10 up to depth 176). For each node I explored
its parent's subtree completely and computed the reason. Weighted by the measured widths,
a **free and perfect** store (every reason already known, lookups at no cost) leaves:

| Reasons kept | Bits of nodes removed |
|---|---|
| at most 20 bits | 0.48 |
| at most 40 bits | 1.31 |
| at most 61 bits | 1.97 |
| at most 80 bits | 2.49 |

Median dead parent at the peak (depth 158): 26 nodes, height 6, 35 blocked tiles, 53 bits.
A real store must first learn each reason and pay a subset test per lookup, so the net is
at or below zero (guessed).

Assumption it rests on: the reason a depth-first refutation yields is close to the smallest
one. A cleverer refuter could find smaller reasons for the same node; section 5 is the test.

### M2. A learner that lives under one row (the best case of a persistent solver)

Algorithm. While the search stays above one row-start state, the frontier below and the
used tiles of lower rows are the same for every node, so they cost no bits. Keep a table
(cell, left colour) to the set of current-row tiles whose use the death depended on; cut
when a stored set is inside the tiles now in the row. Drop the table when the row-start
changes. No global memory.

Price (measured, `03_row_memo.out`, 400 row-start states per row, whole row subtree
enumerated): gain **0.73, 0.75, 0.60, 0.47, 0.19 bits** on board rows 8, 7, 6, 5, 4, which
hold nearly all of the tree. Why so little: a row-start has only 15 to 550 nodes in its row,
spread over 17 left colours and 16 cells, so a context seldom comes back.

Assumption: reuse between depth-first neighbours is largest inside one row.

### M3. Reasons made small in tile-set terms

Algorithm. Keep the whole frontier but declare k of the used tiles "not needed": prove the
frontier dead with those k tiles available again. The reason then covers every node with
that frontier whose used set contains the remaining ones. At k = all, this is a two-list
join on the seam.

Price (derived, `04_generalise.out`, independently re-checked first-moment profile). Real nodes are sparse
among (frontier, used set) pairs, so a reason covers a second real node only once about
37 to 50 tiles are released; by then the proof tree has grown by 29 to 37 bits at depths 80
to 128. Net at the point of first reuse: -37 bits at depth 80, -34 at 113, -29 at 128, -14
at 158. Only at and after the peak does the trade turn positive (reason 87 bits, proof
2^38.7), where a cut is worth under a bit. Whole search, best depth and k: **0.93 bits,
with 2^91 reasons**; zero when reasons are limited to 61 bits. Also: at depths up to 128,
a full-frontier reason stays dead in the mean only while it keeps 85 to 105 bits. So no
reason of 61 bits with a full window exists there in the model. The independently re-checked ceiling for
the whole family is the two-list floor of O1, 2^126.6, 7.8 bits, at about 10^38 stored
entries.

Assumption: the first moment decides whether a generalised frontier is dead.

### Not learning, but found on the way

A free perfect oracle for "dies within H cells" removes 3.9 bits at H = 16 and 6.6 at
H = 32 (measured, same table), in line with today's not independently assessed 5 bits of nodes for arc
consistency. It is lookahead (ROUTES section 1, "Arc consistency"), it costs about the
subtree it saves, and I price its cheap form at 2 bits net at best (guessed). A footnote.

## 5. Falsifying experiments

**For the verdict (and M1's assumption): prefix cores at depth 113.**
Board: the real pieces. Sample 300 nodes at free depth 113 with the sampler of
`02_real_reasons.py`. For each, give a clause-learning solver the residual board with one
assumption per frontier colour and one per used tile ("tile t is below the frontier"), and
minimise the failed set by deletion. Measure the bits of the minimised reason (function
`reason_bits`) and the conflicts. Baseline: the depth-first reason of the same node from
script 02 (at depth 113, 83% of nodes already sit under a parent with a 61-bit reason,
because the centre clue is in that row; the test is on the other 17%, and on depth 100 to
109 where the share is lower). Kill line for my verdict: more than half of the nodes that
script 02 leaves uncut receive a reason of at most 61 bits within 2^20 node-equivalents.
Then the gap reopens and a second stage of 3,000 nodes is needed, since 300 clean nodes
certify at most log2(300/3) = 6.6 bits. Cost: about 5 core-hours.

**For M2 and RQ2 as planned.** RQ2's design (frames sharing a 44-cell arc, time per frame
against index, kill on "no downward trend") cannot kill. Frames that share the 44-cell
bottom arc differ only in the top side, which a row-order refuter reads in its last row,
so everything below is shared and a downward trend is guaranteed for any solver that
learns anything. That trend is row scan found again (derived). The sound reading is the
total time for all frames on the arc against the plain fill under the arc (2^52.6 nodes
in the mean, review section 2.3). If RQ2 runs, read it that way. M2 says what to expect
at row scale: under one bit.

**For M3.** None proposed. Its ceiling is independently re-checked and needs 10^38 entries.

## 6. Why the gap is empty: the argument in order

1. T nodes pay for reasons of at most log2 T bits (exact).
2. Ten bits or more need the cuts at depth 113 or before (measured).
3. A reason is frontier colours plus blocked tiles. With fewer than the full window it is
   dead only when the refutation is a short chain from the current cell: a window with a
   free end has 17 times more fillings than an anchored one, and at the peak an unanchored
   pair of cells is dead with probability about 2^-25 (derived, uniform model, not
   scripted). Short chains are what M1 measures: 2 bits in all.
4. With the full window the reason needs 85 bits or more to be dead in the mean at those
   depths, above any budget, and releasing tiles costs more proof than reuse (M3).
5. Symmetry and dominance: no two tiles are equal up to rotation, the clues remove the
   board rotations, nodes of one depth have used sets of equal size so dominance is
   equality, and equal states do not recur (merge factor 1.0003 at most, cited).
6. What this does not exclude: a short refutation that is not a chain and not a join, found
   by a solver and invisible to the first moment. No lower bound forbids it (section 3).
   The experiment of section 5 looks for it directly. D1 looks for it on frames.

True small reasons exist in abundance: "tile t at cell c" is false for all but about one
placement, at 9.6 bits. Proving one costs the whole tree with that pin, and the best pin is
worth about 10 bits (D3, painted boards), so proof cost times 2^b stays at the price. That
is the nogood memo's headline 2 again.

## Appendix A. Scripts

All `python3 -I`, stdlib, one core, in `notes/gap-1/`.

- `e2real.py`: official pieces, Track 10's scan order and node definition, candidate
  index, measured widths (read from commit `dc71ad56`).
- `02_real_reasons.py 200000 1500 11 $(seq 80 185)` (3 min 18 s) gives
  `02_real_reasons.all.out`; `02_real_reasons.out` is an earlier run with another seed
  and 21 depths (77 s), kept as a repeat.
- `01_ceilings.py 02_real_reasons.all.out` (1 s) gives `01_ceilings.out`.
- `03_row_memo.py 100000 400 5` (31 s) gives `03_row_memo.out`.
- `04_generalise.py` (about 1 min) gives `04_generalise.out`; imports the re-check's
  `rv_lib.py` read only.

## Appendix B. Per-depth shares (measured, seed 11, 1,500 nodes per depth)

Share of nodes whose parent's dead subtree has a reason of at most B bits, or a height of
at most H.

| Free depth | B 40 | B 61 | H 16 | H 32 | Median reason of a dead parent (bits) |
|---|---|---|---|---|---|
| 100 | 0.82 | 0.84 | 0.85 | 0.94 | 22 |
| 113 | 0.53 | 0.83 | 0.89 | 0.90 | 38 |
| 128 | 0.33 | 0.41 | 0.62 | 0.79 | 55 |
| 144 | 0.47 | 0.63 | 0.90 | 0.99 | 43 |
| 158 | 0.30 | 0.61 | 0.97 | 1.00 | 53 |
| 166 | 0.81 | 0.93 | 0.99 | 1.00 | 23 |

Means over depth ranges: B 61 saves 0.30 of nodes on depths 80 to 113, 0.50 on 114 to 140,
0.76 on 141 to 172. The shares look large and the gain is small because a gain is the log
of the share that remains, and a quarter to two thirds remains on the levels that count.
Depths 100 and 113 are high because the centre clue sits in that row (113) or just above
it (100) and kills most subtrees within a few cells; plain row scan already pays only
those few cells.

## Appendix C. What I could not check

- D1's results (not available). If D1 finds frame cores that are small in real-node
  measure and cheap, point 6 above is where this memo is wrong.
- The claim that a depth-first reason is near-minimal. Not tested; it is the experiment of
  section 5.
- Reasons of a clause-learning solver on the real pieces. No SAT call was made.
- The unanchored-window figure (2^-25) and the cost of a lookup: arithmetic, no script.
- M3 is model only. The model matches the measured total to 0.25 bits here, but its
  treatment of a released tile pool was not checked on a small board.
- Sampling: nodes of one depth share ancestors (SMC). Level widths pass the gate, shares
  carry no error bars; depths 177 to 185 drift to 0.28 dex and hold 1% of the tree.
- Ben-Sasson and Wigderson, Lecoutre et al. (beyond the abstract), and the handbook
  chapter were not opened today.
- `gap-6a` is a draft of today without a second analysis; I cite one of its script outputs.
