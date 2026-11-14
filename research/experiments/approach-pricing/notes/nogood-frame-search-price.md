# The price of a frame search driven by learned nogoods

> **Status (2026-11-13):** independently re-checked, see `notes/nogood-frame-search-price-recheck.md`.
> **Re-check's rulings:**
> - The three headlines survive in substance: the pass line is void (more widely than the author says), the nogood route with a fill-order refuter has no price of its own, and D1 can kill and cannot confirm more than a few bits. Tables 1 and 2 and the decision table reproduce within 0.3 bits.
> - Restricting to H does not remove the degenerate median, it moves it one ring in: 53.0% (s.e. 0.6) of H frames have an unfillable 3x3 clue block, refuted in 7 nodes in the median. The kill line cannot fire as read on a median. Fix: read every threshold on price >= max over p of [Q_(1-p)(v_f) + log2 p], and drop the pass line.
> - The arc thresholds in seconds assume 1e8 nodes per core-second: a convention; at the 24 to 41 million measured for the repo's engines the budgets are 2.4 to 4.2 times longer. Neither arc reading line is sound; R0-arc is predicted censored at every k.
> - "3 cells in 70% of cases, 15 to 1,500 plain nodes, about 19 bits": holds with rewording. Per dead block; per frame the smallest core is 2 cells or fewer in 73% of frames; the node range is an artefact of one fixed cell order (at most 4 nodes in the mean with a suitable order).
> - "Kill line valid to one bit" (median minus one bit): holds and is too weak to use.
> - Typical border nogood, "arc of 39 to 40 cells, 109 to 111 bits": not established. 40 cells is an upper bound (Markov); among arcs that pass the two bottom blocks the bits are 99.5 to 101.5.
> - "w <= 6.4": not established. 6.4 is the expected value under the independence model, not a bound; it fails upward. Spread cores below 2^107.5: the bound holds, the walk in row-scan border order is 2^116.5.
> - "n frames support at most log2(n/3) bits": holds with rewording, "can certify", and n = 60 in D1 gives 4.3 bits (the memo's 6.1 is for n = 200).
> - Added measurement and reading rule (section 7): measurement useful, reading wrong. Its limit paragraph ("the refuted share is an upper estimate") is to be deleted; for the stand-alone procedure uniform arcs are the right population.
> - Persistent solver, unpriced: holds as an open objection, no bound exists.
> **Read with:** `notes/nogood-frame-search-price-recheck.md` (sections 0 and 3); the D1 reading rules are in `PREREG_AMENDMENTS.md` A1 of the D1 worktree, as the review cites.

Arc 2026-11-13, finished at 08:10 by the clock, before any Track D1 result
exists.
Thinking work, one core, about 1 core-minute of stdlib Python.

Grades used on every statement:

- **measured**: committed data, file cited.
- **reported**: a source I opened says so, file cited.
- **argued**: derived here. Where a number is attached, it comes from a scratch
  script that is not committed (paths in section 9), run on the independently re-checked
  first-moment library `night/2026-11-12/notes/price-and-ledger-review/rv_lib.py`
  or on the official piece file. Treat those numbers as argued until rerun.

Units. Prices are in bits (log2 of row-scan nodes). The row-scan price is
2^134.40 (measured: 10^40.457, `research/experiments/real-puzzle-profile/RESULTS.md`
on branch `night/2026-11-12-t10-real-puzzle-profile`). The class-aware model I
use gives 2^134.64 and the pooled one 2^134.52 (argued, script `01`, `03`), so
every model figure below carries about 0.2 bits of slack. Refuter costs are
converted at 1e8 nodes per core-second (reported: the rate used in
`night/2026-11-12/notes/price-and-ledger-recheck.md`, section D). 1,000
core-years is 2^61.5 nodes, 72.9 bits below the price.

## 0. Result first

I did not find a proof that the nogood route cannot beat row scan. I found
three things that hold whatever D1 measures, and they change how its numbers
must be read.

1. **The preregistered pass line is met by construction on uniform frames,
   and means nothing there.** All but 2^-17.45 of valid frames die on a clue
   corner block. The smallest core of such a death is 1 to 4 edge cells (3
   cells in 70% of cases), 9 to 13 bits, proved in 15 to 1,500 plain nodes.
   So b + log2(C) is about 19 bits on the median uniform frame (argued,
   exact enumeration on the official pieces, script `02`). These cores cover
   none of the 2^107.5 frames that pass the four blocks. The number that
   bounds a price is the mean of C x 2^b over the frames that pass, with b
   counted inside that set (section 1).
2. **With a refuter that fills cells in a fixed order, a nogood search is a
   tree search in another order, exactly.** Its price is the first-moment sum
   of the order "core cells first, then the refuter's order". Row scan is the
   member of this family whose cores are the nested bottom arcs. Any gain
   must therefore come from the refuter proving a partial frame for less than
   the break-even budgets of table 2 (for example 65 core-seconds for a
   40-cell bottom arc, 6.7 core-days for a 34-cell one, 1.3 core-seconds for a
   full frame that passes the four blocks). The nogood bookkeeping adds
   nothing by itself (section 2).
3. **What D1 can show is capped.** A sample of n frames with every one
   refuted cheaply bounds the unrefuted share only at 3/n, and that share has
   to be disposed of by a method already priced: inside a row scan it costs
   its share of the row-scan tree, in a frame-first search more. So a D1
   sample of n frames can support a gain of at most log2(n/3) bits (6 bits at
   n = 200), and that best case needs arcs sampled from the row-scan tree. D1 can kill the route. It cannot confirm an opening of
   more than about two orders of magnitude; anything larger is an
   extrapolation of a tail it did not sample (section 3).

## 1. Model

### 1.1 Objects

- Omega: the valid frames. |Omega| = 2^124.93 (reported: 10^37.608, exact
  committed count quoted in `price-and-ledger-recheck.md`, section C).
- H: the frames that pass the four clue corner blocks. |H| = 2^-17.45 |Omega|
  = 2^107.48 (argued, script `02`; it reproduces the re-check's counts 2,620,
  3,404, 2,669, 2,508 fillable chains of 57,131, reported in
  `theory-and-frame-recheck.md`, section 4a). The fraction is over 5-piece
  chains, not over frames, and treats the corners as independent.
- A border nogood nu = (S, a): a set S of border cells and a placement a on
  them such that no solution contains a, the other 60 - |S| border cells left
  free. This is what D1 extracts: its SAT refuter passes the 60 border
  placements as assumptions and keeps the failed ones after deletion
  (reported: `scripts/sat.py` in the D1 worktree, read at 07:50, uncommitted
  and liable to change).
- vol(nu): the number of frames of the population at hand that contain a.
  b(nu) = log2(population / vol). C(nu): the cost of proving nu alone.
- For a fixed order pi of the 60 border cells: N_j is the number of valid
  prefixes of length j, and beta_j = log2 N_j.

Assumptions, used where named.

- A1. A prefix of length j has |Omega| / N_j extensions. True on average by
  counting; the model's N_60 is 0.35 bits above the exact count (reported,
  same section C).
- A2. Each proof is paid in full: no clause learned in one proof shortens
  another. D1 as written opens a fresh solver per frame (reported, `sat.py`),
  so its C is this quantity.
- A3. A stored nogood is tested against a prefix for one node.

### 1.2 (a) The covering bound, and the statistic it needs

Let a search certify every frame of a population P (P = Omega or H) with a
set NN of nogoods, and let nu(F) be the nogood that disposes of frame F. The
frames disposed of by one nogood are at most vol(nu). So

    |NN| = sum over F in P of 1 / #{F' : nu(F') = nu(F)}
        >= sum over F in P of 1 / vol(nu(F))
         = E_F[ 2^b(nu(F)) ],            F uniform in P,

and in the same way, under A2,

    cost >= E_F[ C(nu(F)) x 2^b(nu(F)) ].                       (1)

(argued; exact.) Four consequences for the reading.

- The bound is a **mean of 2^b C**, carried by the frames with the largest
  cores. A median gives a floor one bit lower: the mean of a positive
  variable is at least half its median, so
  price >= median(b + log2 C) - 1. The program's kill line is therefore valid
  as stated, to one bit. The pass line (below 120) is not informative: a
  median says nothing about the mean from above.
- **The population matters.** On Omega the median frame has a corner core of
  about 19 bits (section 0, item 1). Those nogoods cover Omega minus H and
  not one frame of H. For the frames of H the bound is (1) with P = H and b
  counted inside H: b_H = -log2( #{F in H containing a} / |H| ). D1's batch
  script counts every nogood in the whole population (reported:
  `scripts/batch.sh` calls `count --stratum U --subsets`). For a core that
  avoids the 16 corner-window cells, b_H is about b_U (argued, independence).
  For a core that contains k complete corner windows, b_H is b_U minus about
  4.4k, so b_U overstates the floor by up to 17.45 bits.
- (1) is a bound for a search that uses the nogood D1 extracted for each
  frame. A real search disposes of F with the first stored nogood that
  contains it, which may be larger than F's own core. The rigorous bound has
  b_min(F), the least b over all cores the refuter can produce that contain
  F. A deletion-minimal core is not a minimum core, and cells left undecided
  at the time cap stay in it (reported, `sat.py`, field `core_undecided`).
  So D1's b is an upper estimate of b_min and the "kill" is a kill of this
  extractor at this budget, not of the route.
- Placement nogoods undercount volume when the reason is interior only. The
  interior sees an edge cell only through its inward colour (reported:
  `frame-information.md`, section 1). An inward letter carries 3.94 bits, a
  placement 5.81 (argued, script `01`: 17 seam colours, counts 1 to 6). For
  cells in a contiguous run the frame joints already pin the piece to about
  3.5 bits a cell, so the difference is small; for scattered cells it is up
  to 1.9 bits a cell.

### 1.3 (b) A concrete procedure and its cost

**Procedure A(pi).** Depth-first search over valid border prefixes in the
fixed order pi. The four corner lists are applied when their window is
complete. A prefix is cut when it contains a stored nogood. At a full frame
the refuter is called: a solution ends the search, otherwise the core is
stored and the search returns to the deepest core cell.

For a frame F let j(F) be the position of the deepest cell of the nogood that
disposes of it, q_j the share of frames with j(F) = j, and Q_j the share with
j(F) >= j. Cost = E_enum + E_proof.

- **Enumeration.** A prefix of length j is visited when none of its shorter
  prefixes was cut. Under A1 and A3:

      E_enum = sum_j N_j x Q_j.                                 (2)

- **Proofs, without a store** (backjumping only). Every prefix of length j
  whose frames are disposed of at level j needs its own proof. Under A1 the
  frames with j(F) = j fill q_j N_j prefixes of length j, so

      E_proof = sum_j q_j N_j Cbar_j  =  E_F[ C x 2^beta_j(F) ].  (3)

- **Proofs, with a store.** The number of proofs is the number of distinct
  nogoods, which is at least (1). It equals (1) when the nogoods in use do
  not overlap (one cell set for all frames). If they overlap as independent
  sets of equal volume, covering |P| frames takes ln|P| times more: a factor
  of at most 87, or 6.4 bits, for P = Omega (argued; the independence is an
  assumption, not checked). So

      2^b C  <=  E_proof  <=  about 2^(b + 6.4) C,   and never above (3).   (4)

  The store must hold the nogoods for the whole search unless their cells are
  a prefix of pi: a nogood whose first cell sits at position p is met again
  under each of the N_(p-1) prefixes before it. So the left side of (4) needs
  about 2^b stored entries. Above roughly 2^40 entries it is not available on
  the machines of this program, and E_proof moves back toward (3) (argued; I
  did not open a source on the time and space of nogood recording today).

Putting (2) to (4) together:

    price of A(pi)  ~  log2( 2^beta_pos + 2^(b + w) x C ),   0 <= w <= 6.4,   (5)

where beta_pos stands for log2 of (2): the bits of the prefixes that have to
be walked before a nogood applies. The refuter's cost multiplies the covering
term only. Position enters through enumeration only.

**The role of position, exactly.** The sentence "a core spread over all four
sides prunes nothing in a prefix order" is true of E_enum for an order chosen
without regard to the cores: then Q_j stays near 1 to the last cells and
E_enum is the whole population, 2^107.5 for H. It is not true of E_proof,
which a store still cuts from 2^107.5 C to about 2^b C. And 2^107.5 is below
the row-scan price by 26.9 bits: walking every frame of H costs less than row
scan. What sinks frame-first search is the proof per frame, not the walk.

**The role of contiguity, exactly.** Two separate effects.

1. Bits per cell. A window of L consecutive border cells holds 3.6 bits a
   cell at L = 10 and 2.9 at L = 30, because frame joints tie neighbours. L
   scattered edge cells hold about 5.7 bits each as placements: 56.8 bits for
   10 cells, 110.6 for 20 (argued, script `01` and arithmetic). For the same
   cell count a contiguous core is a far larger nogood.
2. Shared support. What an order can exploit is not the position of one
   core but whether the cores of different frames lie on the same cells.
   Nested supports (row scan's bottom arcs) make E_enum small in the order
   that follows the nesting. Contiguous cores at varying places on the ring
   need no shared support: enumerate the valid windows of length L at each of
   the 60 starts, prove each, then chain the survivors around the ring. Cost
   about 60 x N_win(L) x Cbar, that is

       price of the window procedure ~ 5.9 + log2 N_win(L) + log2 Cbar,   (6)

   provided every wrong frame has a refutable window of length L (argued).
   Scattered cores on unshared supports have no such scheme, and only (5)
   applies.

Window counts (argued, script `01`, corner-centred windows, block tests not
applied): L = 16: 51.5 bits; 20: 63.5; 24: 75.0; 30: 87.7; 36: 98.7.

### 1.4 (c) Where the truth lies, and what decides it

Between (1) and (3). The gap is beta_j(F) - b(F), the information carried by
the prefix cells that are not in the core. It is zero when the cores are
prefixes of one order. Three things decide where a given D1 outcome falls,
and all three can be read from the core lists D1 already writes, with no new
run:

- the span of each core along the ring (the shortest run of consecutive
  border cells that contains it), which feeds (6);
- whether the cores of different frames share cells: take the cells in
  decreasing order of their frequency in the cores as pi and evaluate (2);
- 2^b_H against the store that can be held, which says whether the left side
  of (4) is reachable.

My expectation, argued and not measured: on H the cores are long and near
the row-scan shape, because on the official pieces arc consistency over the
whole interior sees nothing past the corner windows (reported,
`theory-and-frame-recheck.md`, section 3) and the 3x3 corner blocks add 0.23
to 0.34 bits a corner (reported, `frame-information.md`, section 4). The
first-moment curve itself is silent on what a clause-learning refuter sees
(reported, same review, ruling F3), so this is a prior, not a result.

## 2. What row scan already does, in the same language

Row scan from row 15 places the bottom side, then for each row a left edge
cell, 14 interior cells and a right edge cell. After k rows the border cells
on the board are the **bottom arc** B_k: the bottom side and k - 1 cells up
each side, 2k + 14 cells.

**A dead end as a nogood.** A row-scan node at the end of row k is a pair
(p, y): p a valid placement on B_k, y a valid fill of the 14(k - 1) interior
cells under it (clues on the board). Write Z_k(p) for the number of such y.

- A single dead node (p, y) is a nogood on the cells of p and y together. It
  is not a border nogood.
- If Z_k(p) = 0 then (B_k, p) is a border nogood: any solution containing p
  would restrict to a fill of the strip. Row scan has proved it at the moment
  the last fill under p's parent arc dies in row k. (argued; exact.)
- The level sum is an identity: the number of row-scan nodes at the end of
  row k is the sum of Z_k(p) over all arcs p. So row scan is procedure A in
  the order "bottom side, then the two sides upward in step", with a refuter
  that fills the strip row by row and shares its work between sibling arcs.
  Its cores are the nested arcs B_k.

**More generally (item 2 of section 0).** Let a refuter prove a nogood on S
by filling cells in a fixed order sigma, with no memory between proofs. The
proofs for all placements on S together visit exactly the valid partial
boards of the order "S, then sigma". So N(S) x Cbar is the first-moment sum
of that order, which is a tree search of the whole puzzle. (argued; exact
for a fixed S. With S varying by frame it is a tree search with a dynamic
order.) D1's plain refuters R0 are of this kind, R1 adds a matching check
(reported, header of `engine/src/main.rs` in the D1 worktree, uncommitted).
For R0 the nogood route has no price of its own. The only source of gain
is a refuter whose proof of a partial frame is smaller than the fill tree
under it.

**The profile, split by arc** (argued, scripts `01` and `03`, class-aware
model, five clues; N_k from the frame-only model). mu_k is the mean of
Z_k over all valid arcs.

Table 1.

| Rows k | Arc cells | beta_k (bits) | log10 mu_k | Share of the price below row k | Gain ceiling (bits) |
|---|---|---|---|---|---|
| 3 | 20 | 59.9 | 2.41 | 1.000 | 62.6 |
| 4 | 22 | 65.8 | 5.75 | 1.000 | 47.4 |
| 5 | 24 | 71.5 | 8.54 | 1.000 | 32.2 |
| 6 | 26 | 77.1 | 10.72 | 1.000 | 19.1 |
| 7 | 28 | 82.5 | 10.98 | 1.000 | 12.0 |
| 8 | 30 | 87.7 | 10.36 | 0.996 | 7.8 |
| 9 | 32 | 92.8 | 10.27 | 0.90 | 3.4 |
| 10 | 34 | 97.6 | 9.15 | 0.37 | 0.7 |
| 11 | 36 | 102.2 | 6.78 | 0.017 | 0.0 |
| 12 | 38 | 106.6 | 2.84 | 1.7e-5 | 0.0 |
| 13 | 40 | 110.7 | -5.71 | 5e-13 | 0.0 |
| 16 | 60 | 124.9 | -45.0 | 0 | 0.0 |

Cross-check: at 160 free cells the script gives 35 arc cells and 2^99.9
arcs, the 10^30.08 of `price-and-ledger-recheck.md` section C (reported).

Reading.

- **The core row scan already exploits.** mu falls below one at free depth
  196, in the thirteenth scanned row: an arc of 39 cells carrying 108.7
  bits. At the end of row 13 (40 cells, 110.7 bits) at most 10^-5.7 of arcs
  have a strip fill (Markov on the first moment, exact inequality in the
  model). So row scan's typical border nogood is a bottom arc of 39 to 40
  cells and 109 to 111 bits, and it has spent all but 5e-13 of its price by
  then.
- **Where its cost sits.** 1% to 99% of the nodes lie between free depths
  129 and 175, on arcs of 31 to 37 cells (90 to 104 bits); the median node
  sits on a 33-cell arc with 10^10.4 strip fills per arc on average. (argued
  for the split. Two points of the depth profile are measured by Track 10,
  `RESULTS.md`: the widest level at free depth 158, and 99% of the total
  before free cell 175. The model gives both.)
- **Gain ceiling.** If border nogoods are used as pruning inside row scan,
  a nogood on B_k cannot remove the nodes above the end of row k. The price
  cannot fall below the sum to row k. The last column is that ceiling,
  reached only if every arc is refuted there for nothing.

**So a D1 core is news only if** it fits inside a bottom-type arc (around any
one side) of fewer than 39 cells, and is proved for less than the break-even
budget of table 2. A core whose span is 40 contiguous cells or more around
one side is the row-scan nogood again. A core that is not inside any such
arc can only be used through (5) or (6).

Table 2. Break-even budget for proving one bottom arc alone, the arc having
passed the two bottom clue blocks (share 2^-8.93 of arcs, argued from the
chain counts). Stand-alone prefix search in the row-scan border order:
price = beta_k - 8.93 + log2 Cbar_k. Break-even is price = 134.40. Cbar is a
mean over arcs. (argued, script `03`.)

| Rows k | Arc cells | Break-even log2 C | In core-time at 1e8 nodes/s |
|---|---|---|---|
| 16 (full frame, two blocks) | 60 | 18.4 | 3.4 ms |
| 15 | 44 | 25.2 | 0.39 s |
| 14 | 42 | 28.8 | 4.5 s |
| 13 | 40 | 32.6 | 65 s |
| 12 | 38 | 36.7 | 19 min |
| 11 | 36 | 41.1 | 6.6 h |
| 10 | 34 | 45.7 | 6.7 days |
| 8 | 30 | 55.6 | 17 core-years |
| 6 | 26 | 66.3 | 28,000 core-years |

Each bit below the budget is a bit off the price. A full frame searched with
all four corner lists (not a bottom prefix) has its own line:
price = 107.48 + log2 Cbar_H, break-even at Cbar_H = 2^26.9, **1.3
core-seconds per frame of H in the mean**. This matches the review's
"break-even at 10^2.9 nodes per frame" taken over all frames (reported).

## 3. Decision table

All rows are for frames of H unless said. b_H counted inside H. C in node
equivalents. "Cap" is the sampling cap: with n frames sampled and all
refuted, the share not refuted is below 3/n at 95%. Disposing of it costs its
share of the row scan in the best case (nogoods used as pruning inside row
scan, arcs sampled by their weight in the tree) and the plain per-frame cost
otherwise, so the price shown is supported at best down to
134.4 - log2(n/3) (argued).

| # | What D1 reports | Price it implies (bits) | Needs | Verdict |
|---|---|---|---|---|
| 1 | Uniform frames: median core 1 to 4 cells in one corner window, b 9 to 13, C 15 to 1,500 nodes, sum near 19 | 134.4, unchanged | nothing | Looks like an opening, is none. Predicted here. Two of the four windows are already in the price and the other two cannot be brought in (section 4) |
| 2 | H: no core, or core of 54 cells or more; CDCL mean cost per frame Cbar_H | 107.5 + log2 Cbar_H: 124.1 at 1 ms, 134.4 at 1.3 s, 140.0 at 60 s | mean, not median; A2 | Opening only if the MEAN is under 1.3 core-seconds. A median above 2.6 core-seconds kills it (mean >= median / 2). Floor 107.5 |
| 3 | H: cores inside a bottom arc B_k, k <= 12 (38 cells or fewer), cost C_k | beta_k - 8.93 + log2 C_k: 97.7 + log2 C at k = 12, 88.7 + log2 C at k = 10, 78.8 + log2 C at k = 8 | the same side for all frames; A1; A2 | Real opening when C_k is under the budget of table 2. This is the only shape that is directly "row scan, improved" |
| 4 | H: cores of 40 to 53 cells around one side | 101.8 + log2 C at 40 cells and up | as row 3 | Row scan's own nogood. News only if proved in under 65 s (40 cells), 4.5 s (42), 0.4 s (44) |
| 5 | H: small cores (8 to 25 cells) spread over three or four sides, supports not shared | log2( 2^107.5 + 2^(b_H + w) C ), w in 0 to 6.4. Never below 107.5 | a store of 2^b_H nogoods; A3 | b_H <= 40: real, worth at most 26.9 bits (8.1 orders), 46 bits short of 1,000 core-years. b_H > 40: looks like an opening, the store does not exist; cost returns to (3) |
| 6 | H: small cores spread over the sides, supports shared (the same m cells in most cores) | log2( 2^b(support) x (1 + C) ) | order with the shared cells first | Real. Equivalent to row 3 with another arc; compare with table 2 at equal bits |
| 7 | H: short contiguous cores, span L <= 20 anywhere on the ring | 5.9 + log2 N_win(L) + log2 C: 57.4 + log2 C at L = 16, 69.4 at 20, 80.9 at 24, 93.6 at 30 | every wrong frame has such a window; (6) | The only outcome that could close the whole gap (needs L <= 16 and C <= 2^4, or L = 14 and C <= 2^10). The prior is against it: an arc of 20 cells that passes its two blocks has about 2^17 strip fills in the mean (20-cell bottom arc, script `03`) |
| 8 | H: median b_H + log2 C above 135.4 | at least 134.4 for this extractor | b counted in H, not in the whole population | Kill of this refuter and budget. Not a kill of the route (cores are minimal, not minimum; A2) |

Rows 2, 3, 5 and 7 are real openings in arithmetic. Under the cap none of
them can be confirmed by D1 beyond log2(n/3) bits: 6.1 bits at n = 200, 8.4
at n = 1,000. Rows 1, 5 with b_H above 40, and any reading of a median as a
price, only look like openings.

Three predictions that D1 can falsify the hour it reports (argued):

- P1. On uniform frames the median minimised core is 2 or 3 edge cells of one
  corner window (25% and 70% of dead blocks), 9 to 13 bits as placements.
- P2. Plain search does not finish on frames of H within a cap of minutes:
  the mean interior tree over H is 10^8.34 x 10^5.25 = 10^13.6 nodes by
  spiral (reported figures combined, `frame-information.md` section 2), 4.5
  core-days at 1e8 nodes per second. If its median is far below, the tree
  sizes are heavy-tailed and stage 1's "100 times cheaper on the median
  frame" compares medians of a distribution whose mean sets the price.
- P3. For a row-order plain refuter on frames of H, the border cells it reads
  are at most 40 to 42 around the side it starts from, for all but about
  2^-10 of frames (table 1, row 13, with the 2^-8.93 block share). A spiral
  refuter reads all four sides in its first ring, so its "core" is 52 cells
  or more by construction and must not be read against the 54-cell kill line.

## 4. The clues

Read from the committed piece file `engine/data/official_eternity2.csv`
through `rv_lib.load_official()` (script `01`): the five clues are at
(row, col) (2,2), (2,13), (13,2), (13,13) and (8,7). The same four corner
positions are reported in `frame-information.md`, section 1. So **four** clues, not three, sit two
cells in from a board corner on the diagonal, and the fifth is central and
touches no border cell. Each corner clue closes a 2x2 block of three free
cells against four edge cells. They do make the cores near them small: 95.4%,
94.0%, 95.3% and 95.6% of the 57,131 border chains at a corner have an
unfillable block, and the smallest core among the four touching edge cells is
1 cell in 3%, 2 in 25%, 3 in 70% and 4 in 2% of those (argued, exact
enumeration, script `02`, all five clue pieces out of the pool). Is that
inside 10^40.46? For the two blocks the scan meets first, yes: those two
clues are worth 5.77 digits to the row scan (reported,
`theory-and-frame-recheck.md`, section 4b). For the two it meets last, no:
they are closed at scan cell 239 of 256, are worth 0.86 digits through the
piece pool only, and closing them first costs 0.64 digits (reported, same
section). In the language of section 1 this is position: their cores have
their deepest cell at level 14 to 16 of the row-scan border order, behind
114 to 125 bits of prefix. A frame-level search can use all four, and that is
where the 2^107.5 floor of rows 2 and 5 comes from; that floor is not inside
10^40.46, and it is 26.9 bits below it, but it is reachable only with a
proof cost per frame of H near one node.

## 5. The strongest objection to this model

Assumption A2. The model prices a search as (number of nogoods) x (cost of
one proof from scratch). A clause-learning solver that stays alive across
frames keeps interior lemmas (clauses on joint colours and interior cells)
that are not border nogoods and that shorten every later proof. Then C is
not a constant, the covering bound (1) no longer bounds the cost in units of
fresh proofs, and the object is "CDCL on the whole puzzle with border
variables decided first", whose price none of the formulas here gives. D1
measures C with a fresh solver per frame, so it measures the model's
quantity and is blind to this effect.

What would break the model: feed the frames of H to one persistent solver
and record conflicts per frame against the number of frames already refuted.
A sustained fall (for example a halving per fixed number of frames, with no
floor in sight) means amortised C is far below fresh C and the tables above
overprice the route. A flat curve means A2 holds at this scale.

Lesser objections, in order: the first moment is used for where arcs die,
and a mean above one does not say most arcs live (it is used here only as
Markov bounds and as exact sums); A1 is exact only on average; the
conversion of solver seconds to nodes at 1e8 per second is a convention, and
a factor 10 either way moves every budget of table 2 by 3.3 bits; the
independence behind w <= 6.4 and behind b_H ~ b_U is assumed.

## 6. What I did not do

- No run on the real pieces beyond the corner chains. No SAT call.
- I did not prove that no refuter beats the budgets of table 2. That is the
  open question D1 bears on, at k = 16 only as designed.
- I did not check D1's sampler or its count of b. I read its uncommitted
  scripts once, at 07:50; they may have changed.
- The window counts ignore the corner block tests (conservative by at most
  4.4 bits per corner window inside the window).
- The time and space claim for a nogood store is from memory, not from a
  source opened today.

## 7. The measurement I would add to D1, if only one

**Truncate each frame of H to its bottom arc and refute the arc alone.** For
each sampled frame of H keep the placements on B_k for k = 13, 12, 10 and 8
(40, 38, 34 and 30 cells: the bottom side and the cells up each side), leave
the rest of the border free, and run the same refuter with the same cap.
D1's frames file already accepts subsets, so this is four more solver calls
per frame. Report, per k, the share refuted within the cap and the cost.

Why this one. It measures the quantity of table 2 directly (the price of
the concrete procedure of section 1.3 in row scan's own order), with no
dependence on which minimal core the deletion loop happens to return and
none on the store. Reading: an arc of 38 cells or fewer refuted within a
minute is below break-even by the ratio of its budget to 60 s (19 times at
k = 12, 2^13 at k = 10); no arc of 40 cells refuted within 65 s is a kill of
the prefix route for this refuter. Its limit: the arcs come from uniform
frames of H, not from the row-scan tree, where arcs are weighted by their
number of strip fills; arcs with many fills are plausibly harder to refute,
so the refuted share is an upper estimate (argued).

## 8. Headline statements and grades

1. On uniform frames the median core is a clue corner window of 1 to 4 edge
   cells and about 19 bits with its proof, so the preregistered pass line is
   met by construction; the bound on a price is the mean of C x 2^b over the
   frames that pass the four blocks, b counted among them, and a median gives
   only a floor one bit lower. **Argued** (exact counting argument; corner
   numbers by exact enumeration on the official pieces, script uncommitted).
2. With a fill-order refuter a nogood frame search is a tree search in the
   order "core first", row scan being the case of nested bottom arcs; its
   typical nogood is a 39 to 40 cell arc of 109 to 111 bits and its cost sits
   on arcs of 31 to 37 cells; a refuter is news only below the budgets of
   table 2 (65 s at 40 cells, 6.7 core-days at 34, 1.3 s per full frame of
   H). **Argued** (identity plus the first-moment model; the model's total is
   measured to 0.03 digits by Track 10, its split by arc is not measured).
3. With a store the price is log2( 2^beta_pos + 2^(b + w) C ): spread cores
   on unshared supports cannot go below 2^107.5 and need 2^b stored nogoods;
   only short contiguous cores or shared supports go lower; and n sampled
   frames support at most log2(n/3) bits of gain. **Argued** (the overlap
   factor w and the store size rest on an independence assumption and on an
   unopened textbook result).

## 9. Scratch scripts (not committed)

a scratch folder,
each `.py` with its `.out`, all `python3 -I`, importing `rv_lib.py` read
only:

- `01_profile.py`: clue positions, row-scan profile split by arc, window
  counts, seam colour census (1 s).
- `02_corner_cores.py`: smallest core of each clue corner block over its
  four edge cells, 57,131 chains per corner (15 s).
- `03_thresholds.py`: break-even budgets and gain ceilings per row, both
  censuses (1 s).
