# Review of `nogood-frame-search-price.md`

Arc 2026-11-13. Independent re-check, one core, about 4 core-minutes of stdlib
Python (`python3 -I`). My scripts do not import `rv_lib`; they read
`engine/data/official_eternity2.csv` directly. They are not committed; paths
in section 6. The author's three scripts were rerun afterwards and their
outputs are identical to the committed `.out` files, line for line.

Grades as in the memo: **measured** (a script of mine, on the real pieces),
**reported** (a file or source I opened), **argued**.

## 0. For D1, today

I read `PREREG_AMENDMENTS.md` A1 in the D1 worktree (commit `929e0e1d`), which
is where the memo's input landed. Four things in it would make D1 read the
wrong thing. None needs a new population or invalidates a run already made;
the first three are changes to the reading, the fourth saves compute.

**0.1 Restricting to H does not remove the degenerate median. It moves it one
ring in.** 53.0% (s.e. 0.6) of the frames of H have at least one unfillable
3x3 clue block (the eight free cells around a corner clue, against the six
edge cells that touch them), refuted in 7 nodes in the median and 49 at most
(measured: 19,714 distinct H frames from 40 SMC runs, `r4_h_3x3.out`; per
corner 14.6%, 22.7%, 16.8%, 15.1%). So the median H frame again dies on a
corner-local core of at most 6 edge cells. Consequences for A1 as written:

- `M_H = median_f(...)` will sit at a few tens of bits (argued: I did not
  compute b_H of these cores), or flip between that and the real figure according to whether 30 or 31 of the 60 frames are of this
  kind (the expected count is 32). The pass line (below 120) would be met by
  construction a second time, and the kill line cannot fire even if the other
  47% of H sit at 150 bits.
- The stage 1 kill line has the same defect: it reads a median ratio. On a
  frame with a dead 3x3 block a fixed-order plain search may thrash (the
  spiral fills the 52 cells of ring 1 before it reaches (2,3), (3,2), (3,3);
  argued, not run) while the SAT refuter finds the local core at once. A
  median ratio above 100 would then say "plain search has a bad cell order
  on half of H", not "clause learning refutes frames cheaply".

Fix, analysis only. Read every threshold on a functional that bounds the
mean, which is what the memo's own section 1.2 requires and A1 did not take
up. For any share p, mean(X) >= p x (the upper p-quantile of X). So

    price >= max over p of [ Q_(1-p)(v_f) + log2 p ],

and with p = 1/2 this is the memo's "median minus one bit". Report it at
p = 0.4, 0.25 and 0.1 (floors 1.3, 2.0 and 3.3 bits under the 60th, 75th and
90th percentiles), together with log2 of the sample mean of 2^v. Kill when the
best of these floors exceeds 134.4. Drop the pass line or state it as "not
informative". Also report the split "cheapest nogood within ring distance 3
of a corner" against the rest; the second group is where the price is.
Stage 1: read the ratio of total times (sum over frames) and the same upper
quantiles, not the median.

**0.2 The arc thresholds in seconds assume 1e8 nodes per core-second.** A1
item 4 hard-codes "60 s" and "65 s". Those come from the memo's convention.
Stage 2 of the same PREREG converts with the host's measured R0 rate r, and
the arc reading must do the same. Budgets in nodes, per arc that passes the
two bottom blocks (measured on the real pieces by SMC, `r3_arcs.out`, about
0.2 bits of noise; memo's model figures in brackets):

| Arc | log2 of arcs passing both bottom blocks | Budget, log2 nodes | At 1e8 nodes/s | At 3e7 nodes/s |
|---|---|---|---|---|
| 44 cells (k = 15) | 108.9 | 25.5 [25.2] | 0.49 s | 1.6 s |
| 42 cells (k = 14) | 105.4 | 29.0 [28.8] | 5.5 s | 18 s |
| 40 cells (k = 13) | 101.5 | 32.9 [32.6] | 78 s [65 s] | 260 s |
| 38 cells (k = 12) | 97.4 | 37.0 [36.7] | 22 min [19 min] | 75 min |
| 34 cells (k = 10) | 88.4 | 46.0 [45.7] | 7.9 days [6.7] | 26 days |
| 30 cells (k = 8) | 78.6 | 55.8 [55.6] | 20 core-years [17] | 67 core-years |

If r on the x86 laptop is 3e7, the 40-cell budget is 260 s, above the 120 s cap. Then
"no 40-cell arc refuted within 65 s" fires as a kill when nothing was
killed, and no outcome of the k = 13 arm can kill at all, since a mean of
times capped at 120 s cannot exceed 120 s. Either raise the k = 13 cap to
twice its budget in host seconds or state that the arm cannot kill.

**0.3 Neither arc reading line is sound.**

- "Refuted within 60 s on a quarter of the frames means below break-even":
  false in the favourable direction. The break-even of table 2 is a budget
  for the **mean over all arcs** (the memo's table header says so; its
  section 7 and A1 dropped the word). The other three quarters must still be
  disposed of, by extending them. Counter-example inside A1's own caps: a
  quarter refuted at 60 s, the rest censored at 38 and at 40 cells, full
  frames at 10 s each. Cost per 38-cell arc is at least
  0.75 x (120 + 17 x (120 + 64 x 10)) = 9,800 s, against a budget of 1,350 s.
  The rule says "below break-even"; the route is 2.9 bits above it.
- "No 40-cell arc refuted within 65 s kills the prefix route": it kills
  entry at 40 cells only. Entry at 42 and 44 cells has budgets of 5.5 s and
  0.49 s and is not measured. A refuter at 100 s on 40 cells and 1 s on 42
  cells is below break-even at 42. Nothing ties the two.

Sound reading, computable from the runs A1 already schedules, because each
frame's arcs are nested. Per frame, with T the cap and t_k the time (decided
or not), and t_16 the stage 1 R2 time on the full frame:

    c_13 = t_13 if decided, else T + 2^6.0 x t_16
    c_12 = t_12 if decided, else T + 2^4.1 x c_13
    c_10 = t_10 if decided, else T + 2^9.0 x c_12
    c_8  = t_8  if decided, else T + 2^9.9 x c_10

(the exponents are the log2 ratios of the counts in the table above, and
107.5 for H). Entry at level k is below break-even iff
mean_f(c_k) x r < 2^(budget_k). A censored t_16 makes c a lower bound. Each
frame's own longer arc stands for one uniformly drawn child, which is what
uniform sampling in H gives.

Sharper for the same compute: replace k = 10 and k = 8 by k = 14 and k = 15.
With a 120 s cap an arm can only kill where its budget is below the cap, and
that is 40 cells and longer. At 34 and 30 cells the cap is 4 to 7 orders
under the budget, so "not refuted" says nothing, and the mean number of
strip fills under such an arc is 2^39 and 2^43 (memo, script `03`, last
column), so "refuted" would need a proof about rows the arc does not touch.

**0.4 R0-arc is predicted censored at every k, on the same tree.** A plain
depth-first fill under a fixed arc is row scan without the sharing between
sibling arcs. Its mean tree per block-passing arc is 2^33.0 nodes by the end
of row 4, 2^42.4 by row 5 and 2^52.6 in all (argued, own pooled model,
`r2_model.out`): 88 s, 16 core-hours and 2.2 core-years at 1e8 nodes per
second. The four values of k give the same tree, because the search does not
reach the rows where the arcs differ before the cap. Running it once per
frame (k = 13) loses nothing and saves up to 6 core-hours of the 16. If
R0-arc does finish on many arcs, that is a heavy-tail finding in its own
right (most arcs die early and the mean is carried by few) and should be
reported as such, with the node count.

One figure for the README: D1 has 60 real H frames, so the sampling cap of
the memo's item 3 is log2(60/3) = 4.3 bits, not the 6.1 quoted for n = 200.

## 1. Rulings

| # | Claim | Ruling | Memo | Mine |
|---|---|---|---|---|
| 1a | Share of valid frames that pass the four clue blocks | holds | 2^-17.45 (chain product, independence assumed) | 2^-17.41, s.e. 0.03, computed jointly on real frames by SMC; 17.48 if the counter's +4.7% gate offset is applied. Pairs: bottom 8.90 against 8.96 as a product, top 8.54 against 8.48, left 8.87 against 8.88 |
| 1b | Minimal core of a dead block: 1 to 4 edge cells, 3 cells in 70% | holds with rewording | 3.3 / 25.2 / 69.5 / 2.0% for 1 / 2 / 3 / 4 cells, per dead block | identical per dead block (independent code). Per frame, smallest core over the four corners: 1 cell 12%, 2 cells 61%, 3 cells 28% (2,051 frames) |
| 1c | 9 to 13 bits, 15 to 1,500 plain nodes, about 19 bits in all | holds with rewording | means 5.6 to 15.8 bits by size; 15 to 1,464 nodes | same bits. The node counts are an artefact of one fixed cell order; with the best of the six orders the proofs take 0 to 3.7 nodes in the mean. Sum about 9 to 14 bits |
| 2a | Lower bound cost >= E_F[C x 2^b], arithmetic mean over uniform frames | holds | (1) | derivation checked; the functional is the arithmetic mean |
| 2b | Median gives a floor one bit lower; kill line valid to one bit | holds, and is too weak to use | median - 1 | any quantile works: Q_(1-p) + log2 p. Needed because of 0.1 |
| 2c | Pass line met by construction on uniform frames | holds, and holds again inside H | "means nothing there" | 53.0% of H frames die on a 3x3 clue block in 7 nodes (section 0.1) |
| 3a | Row scan is the nested-arc member of the family | holds | identity | checked as a counting identity; exact only for a refuter that shares work between sibling arcs, which the memo says |
| 3b | Typical border nogood: arc of 39 to 40 cells, 109 to 111 bits | not established | 39 to 40 cells, 108.7 to 110.7 bits | 40 cells is an upper bound on where a uniform arc is dead (Markov). The first moment gives no lower bound. Among arcs that pass the two bottom blocks the bits are 99.5 to 101.5, not 109 to 111 |
| 3c | Cost sits on arcs of 31 to 37 cells | holds | free depths 129 to 175 | same, own pooled model: 1% at 129, 50% at 153, 99% at 175; total 2^134.52, widest level 158 |
| 3d | Orientation and clues | holds | row 15 first | correct. From row 0 the model gives 2^134.94 and a widest level at 132, so the side matters by 0.4 bits and Track 10's widest level (158) confirms row 15 |
| 3e | Table 2 | holds to 0.3 bits, at the stated convention | 65 s, 19 min, 6.7 days, 1.3 s | 78 s, 22 min, 7.9 days, 1.2 s on the real pieces (section 0.2). The walk of prefixes is inside (0.4 bits). The unrefuted remainder is not (section 0.3) |
| 4a | price about log2(2^beta_pos + 2^(b+w) C), 0 <= w | holds | (5), left side | exact given (1), with 2^b read as E[2^b] |
| 4b | w <= 6.4 | not established | upper bound | 6.4 is the expected value under the independence model, not a bound. It fails upward |
| 4c | Spread cores cannot go below 2^107.5 | holds | 107.5 | 107.5 is a bound; the walk in row-scan border order is 2^116.5 (widest level 2^114.2 at cell 56) |
| 4d | n frames support at most log2(n/3) bits | holds with rewording | 6.1 bits at n = 200 | "can certify", and n = 60 in D1: 4.3 bits |
| 5 | P2: 10^13.6 nodes per frame of H against 10^8.3 | holds for the mean | 10^13.6, spiral | 10^13.59 spiral, 10^15.66 rows 14 to 1, 10^16.72 rows 1 to 14. 10^8.34 is the mean over all valid frames. The true frame is in H |
| 6 | Persistent solver, unpriced | holds as an open objection | none | no bound exists; a floor and a design point can be derived (section 2.6) |
| 7 | Added measurement and reading rule | measurement useful, reading wrong | section 7 | section 0.2 to 0.4 |

## 2. Claim by claim

### 2.1 The corner blocks

**Block.** For the corner at (0,0): free cells (1,1), (1,2), (2,1), clue at
(2,2), touched by the four edge cells (0,1), (0,2), (1,0), (2,0). The corner
piece does not touch the block. Aliveness is a function of four inward
colours. All five clue pieces are out of the pool. Rotation convention
checked against `engine/src/types.rs` (`new[i] = old[(i + 4 - r) % 4]`).

**Chains (measured, exact, `r1_corners.out`).** 57,131 valid five-piece
chains per corner; alive 2,620, 3,404, 2,669, 2,508; product 2^-17.45. Same
as the memo and as last night's review.

**Frames (measured, `r1b_smc.out`).** The memo's fraction is over chains and
multiplies the corners. I counted frames instead: SMC over the 60 border
cells with each block test as a zero weight, 10 runs of 6,000 particles per
line, gated on the exact frame count (estimate over exact 1.047, s.e. 0.021).

| Set | Share of valid frames |
|---|---|
| block (0,0) alive | 2^-4.43 (chains 4.45) |
| block (0,15) alive | 2^-4.05 (4.07) |
| block (15,0) alive | 2^-4.44 (4.42) |
| block (15,15) alive | 2^-4.52 (4.51) |
| all four (H) | 2^-17.41, one s.e. 17.38 to 17.43 |

The sum of the four frame-weighted figures is 17.44. Weighting chains by
their frame extensions changes nothing, and the corners are independent to
the precision of the count (0.06 bits on the worst pair). 17.45 stands. Not
checked: that the twelve block pieces are distinct across the four blocks;
that can only make H slightly smaller.

**Core size.** A subset of the four edge cells is a core when the block has
no fill with the other cells free. I also ran the variant where a free cell
must be suppliable by an edge piece not already in the core: the shares move
by 0.1 point. Per dead block the memo's figures are exact. The headline "3
cells in 70% of cases" is per dead block. A uniform frame has 3.8 dead
blocks in the mean, and its smallest core is 2 cells or fewer in 73% of
frames. What D1's deletion loop returns is one minimal core, not the
smallest; it depends on the deletion order.

**Proof cost.** "15 to 1,500 plain nodes" is the author's fixed order
((1,2), (2,1), (1,1)) whatever the core. The 1,300-node figure for 2-cell
cores is that order filling two unconstrained cells before it meets the
core. With the best of the six orders the mean proof is 0.0, 0.7 to 1.0, 2.1
to 2.5 and 3.5 to 3.7 nodes for cores of 1 to 4 cells. The claim only gets
stronger: b + log2 C is 9 to 14 bits, not 19.

### 2.2 The bound and its functional

|NN| = sum_F 1 / #{F' : nu(F') = nu(F)} is exact, the denominator is at most
vol, so |NN| >= |P| E_F[1/vol] = E_F[2^b]. With A2, cost = sum over nogoods of
C, and the same step gives (1). This is the Horvitz-Thompson form of a sum
over nogoods sampled in proportion to their assigned frames. The right
functional is the **arithmetic mean of C x 2^b over uniformly sampled
frames**. Not a median, not a harmonic mean.

- The one-bit statement is Markov: P(X >= median) >= 1/2. Correct. The same
  line gives price >= Q_(1-p) + log2 p for every p, and section 0.1 shows the
  median is the one quantile D1 cannot use.
- (1) bounds the procedure that uses each frame's extracted core. As a bound
  on the route it needs the minimum of C x 2^b over all cores that contain
  the frame, jointly, not b_min alone. The memo says the first half.
- b counted inside H: correct, and A1 item 1 implements it correctly. The
  original `b - beta_H` of the PREREG is a valid but weaker bound.

### 2.3 Row scan in the same language

**Model.** My pooled implementation: total 2^134.52 (memo, pooled: 134.52),
widest level at free depth 158, beta_k identical to two decimals at every
row, mu < 1 first at free depth 196 on a 39-cell arc with beta 108.71.

**Real pieces (measured, `r3_arcs.out`).** Number of valid bottom arcs by
SMC: 2^110.57 at 40 cells against the model's 110.74; 2^106.45 at 38 against
106.61; full frame 2^124.89 against the exact 124.93. The two bottom blocks
remove 9.0 bits of arcs (memo 8.93, my frame count in 2.1 gives 8.90). So
table 2 moves by at most 0.3 bits.

**"Typical nogood of 39 to 40 cells."** mu_k is a mean over arcs. mu < 1 at
39 cells bounds the share of arcs with a strip fill from above. It does not
say arcs live until then: at 38 cells the mean is 10^2.8 fills per arc and
the share of arcs with any fill is unknown. The sentence supported is "a
uniform bottom arc of 40 cells has no strip fill, except for a share of at
most 10^-5.7". Row scan's own nogoods are a different population again (arcs
weighted by having a live parent). And this is strip-only reasoning: a
refuter that uses the rest of the board can have shorter arc nogoods.

**Is break-even the right comparison?** For the stand-alone prefix procedure
(walk the block-passing arcs of B_k with no interior, refute each alone),
yes, with three qualifications.

1. The walk is inside: sum_(j<=40) N_j is 2^101.94 against N_40 = 2^101.53.
2. The remainder is not inside. The budget is a mean over all arcs; arcs not
   refuted within a cap are extended, and the cost of that chain is the
   quantity of section 0.3.
3. The memo's stated limit of the measurement is backwards. For the
   stand-alone procedure the arcs are enumerated once each, so arcs of
   uniformly sampled H frames are the right population. The weighting by
   strip fills matters only for nogoods used inside a row scan, and there
   table 1 gives a gain ceiling of 0.0 bits at 36 cells and more. So table 2
   and "pruning inside row scan" are two different procedures: a 38 or
   40-cell arc nogood buys nothing inside row scan whatever it costs.

One number the memo does not give and D1 should have: the same model prices
a plain stand-alone refuter at 2^52.6 nodes per block-passing arc, so tying
with row scan at 40 cells needs a refuter 2^19.7 (about 10^6) times cheaper
than plain fill in the mean. That is the size of what the arc arm is looking
for.

### 2.4 The store

- **Left side of (5).** Exact, from (1).
- **w <= 6.4.** The ln|P| factor is the coupon-collector count for random
  sets of equal volume: after m sets a share exp(-m v / |P|) is uncovered,
  and the last frame needs m = (|P| / v) ln |P|. ln(2^124.93) = 86.6 (6.4
  bits); for H it is 74.5 (6.2 bits). Under the stated independence this is
  the expected value of w, not an upper bound. The assumption can fail both
  ways. Favourable: cores on one family of supports partition the frames and
  w = 0. Unfavourable: a search does not draw nogoods at random or greedily;
  it extracts one from each frame it meets, and if cores of neighbouring
  frames keep covering the same popular frames, the count is bounded only by
  (3). The set-cover theorem that looks like this (Johnson 1974, Lovász 1975,
  Chvátal 1979: a greedy cover is within 1 + ln d of the optimum, d the
  largest set) is about a cover chosen with the whole set system in view. It
  bounds the best store, not the one a search builds. (4) should read
  "2^b C <= E_proof <= (3)", with 6.4 bits as the independence figure.
- **A3.** Testing a prefix against nogoods on arbitrary supports is a subset
  query, not one node. Unfavourable; not priced.
- **The textbook result not opened.** It is the time and space of nogood
  recording. Source opened today: Frost and Dechter, "Dead-end driven
  learning", AAAI-94, Theorem 1: with ordering d and graph-based learning,
  space O((nk)^w*(d)) and time O((2nk)^w*(d)), w*(d) the induced width.
  Used correctly? The memo does not in fact need it. Its claim "the left side
  of (4) needs about 2^b stored entries" is a lower bound on the store and
  follows from its own count (1). The theorem is an upper bound and gives
  nothing here: the each-piece-once constraint joins every pair of cells, so
  w* = 255; with the grid alone and a row order w* is about 16 and (nk)^16
  is above the search itself. What stays unsupported is the sentence "above
  2^40 entries E_proof moves back toward (3)": no result I opened gives the
  cost of a bounded store between the two ends.
- **2^107.5.** A bound, not the cost. The widest level of the border walk
  with all four tests is 2^114.2 in row-scan border order and the walk is
  2^116.5. Still 18 bits under row scan, so the memo's sentence "what sinks
  frame-first search is the proof per frame, not the walk" stands.
- **log2(n/3).** The rule of three (zero events in n trials: 95% upper bound
  3/n). It caps what the sample can certify. It assumes independent uniform
  draws, which D1's sampler gives to order 1/particles. It is conservative
  in one respect (a stratified sample could certify more) and optimistic in
  another (it prices the refuted share at zero).

### 2.5 P2

Own model, five clues, mean interior tree under a full frame
(`r2_model.out`):

| Interior order | Mean over all valid frames | Mean over H |
|---|---|---|
| four blocks, then inward spiral | 10^8.34 | 10^13.59 |
| inward spiral | 10^8.34 | 10^13.59 |
| rows 14 to 1 | 10^13.04 | 10^15.66 |
| rows 1 to 14 | 10^14.09 | 10^16.72 |

Both figures are right and they answer different questions. 10^8.34 is the
mean over uniformly drawn valid frames, of which all but 2^-17.4 die in a
handful of nodes; it is the correct multiplier of the frame count. 10^13.59
is the mean over frames that pass the four blocks. The true frame passes
them, so last night's "given the true frame the interior costs about 10^8.3
nodes, seconds" (`price-and-ledger-recheck.md`, section C) is wrong for the
true frame by 5.25 digits: the model figure is 10^13.6 by spiral (4.5
core-days at 1e8), more by rows, and half of it to a first solution. Stage 0
of D1 should expect capped confirms on 16x16.

Reword P2: it is a statement about the mean. 53% of H frames have a dead 3x3
block, so whether plain search "finishes" on the typical H frame depends on
whether its order reaches that block early. The mean over the other 47% is
10^13.9.

### 2.6 The persistent solver

What can be derived.

1. Clauses learned under assumptions are implied by the formula alone, so a
   solver kept alive across frames is one CDCL run on the whole puzzle with
   border placements decided first. The covering bound (1) in units of fresh
   proofs does not apply to it. The memo is right.
2. Any procedure that presents frames one by one pays at least one unit per
   frame presented. So reuse can lower the proof factor only: the floor is
   the border walk (2^107.5 as a bound; widest level 2^114.2 and sum 2^116.5
   in the one order I measured), 18 to 27 bits under row scan and 46 to 55 above 1,000
   core-years. Below that floor the object is no longer a frame search.
3. A design point. The reuse that matters in a search is between frames that
   share a long prefix. Uniformly sampled H frames share none: a learned
   clause that covers 2^-b of the frames with b above 6 will not fire once in
   60 draws. So the incremental arm of A1 item 5 on the uniform sample
   measures reuse through interior lemmas only and understates reuse in a
   real enumeration. A flat curve there does not confirm A2. If the arm is
   run, feed it frames that agree on a 44-cell arc and differ above it.

What the literature gives, and no more. Clause learning with restarts
p-simulates general resolution (Pipatsrisawat and Darwiche 2011; Atserias,
Fichte and Thurley 2011; search summaries read today, papers not opened). So
the amortised cost is bounded below only by the shortest resolution
refutation of the puzzle with its solution blocked, and no lower bound is
known for this instance. With decisions forced in a fixed order the power
depends on the learning scheme: equivalent to ordered resolution with one
scheme, to general resolution with another (Mull, Pang and Razborov, SIAM J.
Comput. 2022, arXiv:1909.04135, abstract read through a search summary). I
found no result that bounds what reuse buys on edge matching. The repo's own
evidence on whole-instance SAT is from 2008 to 2013 solvers plus one CP-SAT
data point (`literature-2026.md`, section 3.4). I did not open those today.

### 2.7 The added measurement

Useful, and the right quantity for table 2. Its reading is not sound as
forwarded (section 0.2 and 0.3). Cheaper and sharper, in order: convert with
the measured rate; read the nested chain c_k, not a per-arc threshold; swap
k = 10 and 8 for k = 14 and 15; run R0-arc once per frame. An optional arm
that would separate two readings of a fast refutation: give the SAT refuter
only the bottom k rows as its formula. If it still refutes, the reason is in
the strip and the nogood is row scan's own; if only the full formula
refutes, the reason is above the arc.

## 3. Sentences that must change

1. Section 0 item 1 and headline 1, "3 cells in 70% of cases ... proved in 15
   to 1,500 plain nodes ... about 19 bits": say "per dead block", add the
   per-frame figure (smallest core 2 cells or fewer in 73% of frames), and
   replace the node range by "at most 4 nodes in the mean with a suitable
   cell order; 15 to 1,500 with the fixed order of script `02`".
2. Section 1.2, first bullet, "The program's kill line is therefore valid as
   stated, to one bit": add "on a population whose median is not a corner
   death; in H it is one (53% of frames have a dead 3x3 clue block), so the
   line must be read on an upper quantile: Q_(1-p) + log2 p".
3. Section 0 item 1, "These cores cover none of the 2^107.5 frames that pass
   the four blocks": true, and add that the same structure recurs inside H.
4. Section 1.3, "a factor of at most 87, or 6.4 bits" and formula (4): "a
   factor of about ln|P| (6.4 bits for Omega, 6.2 for H) if cores cover
   frames independently; less for nested or partitioning supports, and up to
   (3) for cores extracted adaptively".
5. Section 1.3, "Never below 107.5" and decision rows 2 and 5: keep as a
   bound, add "the walk itself is about 2^116.5 in row-scan border order".
6. Section 2, reading, "So row scan's typical border nogood is a bottom arc
   of 39 to 40 cells and 109 to 111 bits": "a uniform bottom arc of 40 cells
   is dead by strip fill alone except for a share of at most 10^-5.7; where
   arcs die before that is not given by a first moment. Among block-passing
   arcs these arcs carry 99.5 to 101.5 bits".
7. Table 2 header and every budget quoted in seconds: state the budget in
   nodes first; seconds only "at 1e8 nodes per core-second, a convention; at
   the 24 to 41 million measured for the repo's engines the budgets are 2.4
   to 4.2 times longer".
8. Section 7, "an arc of 38 cells or fewer refuted within a minute is below
   break-even by the ratio of its budget to 60 s": "the route entered at 38
   cells is below break-even iff the mean of the chain cost c_12 over arcs is
   under the budget", with the chain of section 0.3 of this review.
9. Section 7, "no arc of 40 cells refuted within 65 s is a kill of the prefix
   route for this refuter": "of entry at 40 cells; entry at 42 and 44 cells
   has its own budgets and is not measured".
10. Section 7, "Its limit: the arcs come from uniform frames of H, not from
    the row-scan tree ... so the refuted share is an upper estimate": delete.
    For the stand-alone procedure uniform arcs are the right population. The
    weighting applies to nogoods used inside row scan, where table 1 gives
    no gain at 36 cells and more.
11. P2: "in the mean", and "the median H frame has a dead 3x3 clue block".
12. Section 5, the proposed test of A2: add that uniformly sampled frames
    cannot show prefix-level reuse.
13. Section 0 item 3, "6 bits at n = 200": add "4.3 bits at D1's n = 60".

## 4. What stands

The memo's three headlines survive in substance. The pass line is void, and
more widely than the author says. The nogood route with a fill-order refuter
has no price of its own. D1 can kill and cannot confirm more than a few
bits. Table 1, table 2 and the decision table reproduce from an independent
model and, where I could count, from the real pieces, within 0.3 bits.

## 5. What I did not check

- The share of 38 and 40-cell arcs that have a strip fill on the real
  pieces. Nothing in this review or the memo measures it.
- Whether plain search thrashes on H frames with a dead 3x3 block (section
  0.1, stage 1). Argued from the cell orders only.
- Table 1's gain-ceiling column beyond its consistency with the shares I
  recomputed (7.8 and 0.7 bits at rows 8 and 10 agree).
- The window counts of section 1.3 (6) and decision row 7 beyond arithmetic.
- D1's sampler, counter and SAT encoding. I read `PREREG.md`,
  `PREREG_AMENDMENTS.md` and `scripts/arcs.py` only; `arcs.py` takes the
  bottom row and rows 16 - k to 14, which is the right side.
- Pipatsrisawat and Darwiche, Atserias et al., Mull et al.: known to me
  through search summaries today, not opened. The set-cover ratio was read on
  the Wikipedia page "Set cover problem", not in Lovász's paper. Frost and
  Dechter was opened (PDF, Theorem 1 read).
- The class-aware model. I reimplemented the pooled one; the two differ by
  0.12 bits on the total.

## 6. Scripts (scratch, not committed)

a scratch folder,
all `python3 -I`, stdlib, one core:

- `r1_corners.py` (25 s): exact chains, block aliveness, minimal cores in two
  variants, proof nodes for the fixed and the best cell order.
- `r1b_smc.py 10 6000` (77 s): frame counts by SMC with the block tests as
  zero weights; gate on the exact count; singles, three pairs, all four;
  per-frame smallest core.
- `r2_model.py` (1 s): own pooled first-moment model; row scan from both
  sides, arcs, budgets, plain stand-alone refuter, interior tree under a
  frame for four orders, over all frames and over H.
- `r3_arcs.py 8 6000` (90 s): number of valid bottom arcs on the real pieces
  with and without the bottom block tests; the border walk.
- `r4_h_3x3.py 40 3000` (32 s): 3x3 clue blocks on 19,714 sampled H frames.
- `auth/`: reruns of the author's `01`, `02`, `03`; identical to the
  committed outputs.
