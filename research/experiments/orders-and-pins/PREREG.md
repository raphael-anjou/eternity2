# orders-and-pins: preregistration

Arc 2026-11-13, track D3. **Track label: measurement.** Not a record attempt;
no board is produced; nothing here is Eternity II progress, and results on
generated boards are never official progress. The runner does not judge the
result.

Questions (`THESIS_PLAN.md`, lead worktree, arc 2026-11-13):

- **RQ6, Part A.** Is row scan the cheapest static fill order on the real
  pieces with the five clues?
- **RQ5, Part B.** What is one more known tile worth, and where?

Quantity, throughout: Track 10's size of the exact (no break) search tree of
a fixed-order backtracker. Node = a valid partial board after a placement on
a free cell (grey exactly on the rim, every joint with a placed or
pre-placed neighbour matching); pre-placed cells (clues, pins) are not
nodes. **Measured** by Track 10's population SMC estimator (and, in Part B,
by a second estimator defined in B2), **predicted** by the first-moment
(bulge) model. The model is the cheap proxy; the estimator is the
measurement; no model number is reported as measured.

## 0. Imported code, and everything done before this file

- `engine/` is Track 10's crate (`research/experiments/real-puzzle-profile/engine/`,
  branch `night/2026-11-12-t10-real-puzzle-profile`, commit
  `dc71ad56567570aecdcb75e474f163e5bba426e2`, judged keep). Changes, all
  additive, none to `smc`, the child function or the resampling:
  (1) a step with no rim side and no known neighbour scans every oriented
  piece (Track 10 panicked there); (2) the enumerator also reports the number
  of complete boards and the node count at the first one; (3) accessors and a
  depth-truncated enumerator in `tree.rs`; (4) `engine/src/oap.rs`:
  `paint-boards`, `tsmc` (Track 10's SMC on any order file, fixed-cell list
  and board), `tenum`, `tenumd` (truncated), `gate6`, and `tppd` (the
  planted-path decomposition of B2).
- Equivalence (not test seeds): `tsmc` with `orders/16/rows-b-lr.txt`
  reproduces Track 10's committed five-clue row (seed 201, N = 1e5) exactly
  (3.233816e+40); `gate6` on `rows-b-lr` with no clue reproduces Track 10's
  committed `gate-a.jsonl` exactly (all 30 exact width vectors, all SMC
  totals). PPD against exact enumeration on two 6x6 instances with two pins,
  400 repetitions at N = 50: 11,369 +- 185 against 11,168 and 7,975 +- 88
  against 7,812 (plain SMC 10,962 +- 227 and 7,865 +- 158).
- Smoke check (step 1 of the brief, seeds 9001 to 9005): official set, five
  clues, row scan, N = 1e5, 5 repetitions: 10^40.453 [40.351, 40.536];
  Track 10's 10^40.457 lies inside. `results/smoke/smoke-five-np1e5.jsonl`.
- Model: `scripts/model.py`, a port of `solve-path-5/bulge.py` (commit
  d3d727b4), float log arithmetic; reproduces 40.495 (five clues), 47.135
  (centre clue), 50.029 (no clue). Generalised for pins: pre-placed cells of
  any class; a joint between two pre-placed cells is not a random joint.
- `scripts/orders.py` (model only, deterministic, rerun gave the identical
  manifest): order files and `orders/manifest.json`. `scripts/pins_model.py
  rows-b-lr` (model only): `pins/model.json`. `scripts/plan.py`: every plan
  file, `plans/*.jsonl`, with each unit's command and seed.
- Boards, painted before this file; no estimator has run on a test board.
  Census painter = Track 10's `paint` (judged): ring joints get the official
  frame-colour joint multiset, seam and inner joints a uniform deal of the
  official interior joint multiset; duplicate or rotation-symmetric pieces
  rejected; then piece ids shuffled and pieces rotated (the tree does not
  depend on ids; the shuffle keeps the enumerator's value order from
  following the painted cell order). Five clues = the planted pieces at the
  official clue cells. Painter seeds, the first accepted from each start:
  calibration 100149, 100244 (start 100001); Part A null 200115, 200139,
  200181 (start 200001); Part B 300051, 300061, 300097, 300117, 300131 (start
  300001). Files `boards/<role>/board-<seed>.txt`. The census painter is used
  rather than the repo's framed generator because the framed generator draws
  each joint colour independently (colour counts vary between boards); the
  brief asks for the real colour counts, which the painter reproduces exactly.
- Calibration, on calibration board 100149 only: section 3.

## Part A (RQ6)

### A1. Orders (files `orders/16/<name>.txt`, 6x6 analogues `orders/6/`)

Cells (r, c): r = CSV row (field 6), c = column (field 5); pos = 16 r + c.
Clues (8, 7), (2, 2), (2, 13), (13, 2), (13, 13), skipped by every order.

| order | kind | model log10 T (five clues) | model peak (log10, free depth) | SHA-256 (first 12) |
|---|---|---|---|---|
| rows-b-lr | named, baseline | 40.495 | 39.20 at 158 | 623c44af253f |
| rows-b-rl | named | 40.495 | 39.20 at 158 | 02af306649b4 |
| rows-t-lr | named | 40.622 | 39.34 at 132 | 5d8c08307584 |
| rows-t-rl | named | 40.654 | 39.42 at 133 | 3f46ab745980 |
| cols-l-bt | named | 40.495 | 39.20 at 158 | e144af909b6a |
| cols-l-tb | named | 40.495 | 39.20 at 158 | 1f2412046797 |
| cols-r-bt | named | 40.622 | 39.34 at 132 | 0dcd35041f60 |
| cols-r-tb | named | 40.654 | 39.42 at 133 | 33d0e7a23158 |
| snake-rows-b | named | 40.495 | 39.20 at 158 | 2bd2434b7867 |
| snake-cols-l | named | 40.495 | 39.20 at 158 | 27012dbbe2de |
| spiral-in | named | 46.051 | 45.07 at 151 | 7fd7e14bcea7 |
| spiral-out | named | 50.388 | 49.48 at 104 | 0ca3b8750033 |
| frame-then-rows | named | 50.749 | 49.72 at 141 | 9b1e351c7ed7 |
| antidiag | named | 45.109 | 44.54 at 105 | fd4002587c3f |
| bands2 | named | 41.088 | 39.88 at 159 | 504e0c453203 |
| s5-sa | searched | 40.896 | 39.77 at 143 | ee2aa4bc30be |
| s6-beam | searched | 41.153 | 39.74 at 156 | fdb53c569986 |
| s4-sa | searched | 44.508 | 43.56 at 150 | 1da5ed769e2e |

Named: the eight line scans (rows from the bottom or top, columns from the
left or right, two directions along the line; `rows-b-lr` is Track 10's
order: rows 15 to 0, columns 0 to 15), boustrophedon by rows from the bottom
and by columns from the left, Track 10's clockwise inward spiral from (0, 0),
the outward spiral from the centre clue (square rings of growing distance,
each walked clockwise from the cell above the centre, off-board cells
skipped), the frame in Track 10's ring order then the interior by rows from
the bottom, anti-diagonals from (15, 0), two-row bands from the bottom
column by column (comb).

Searched (`scripts/orders.py`, model with the official colour census and the
five clues): seven searches. (1 to 5) simulated annealing on the model's
log10 tree size from rows-b-lr, cols-l-bt, snake-rows-b, spiral-out,
antidiag (seeds 11 to 15; 200,000 proposals; moves: one cell moved up to 40
positions, or a segment of 2 to 16 cells reversed; temperature geometric
from 0.5 to 0.001 dex; best order seen); (6) beam search over prefixes,
width 256, keeping the prefixes with the smallest running sum of model
widths; (7) annealing from the beam order (seed 16). Outputs equal to a named
order on the free cells, or to an earlier output, are dropped; up to five of
the rest are kept. Result: searches 1, 2, 3 returned their start (row scan
is a local optimum of the model under these moves, as the memo's hill climb
found); 7 returned the beam order; three new orders remain (table). No
searched order is below row scan in the model.

### A2. Measurement

- Official set, five clues, every order: N = 1e6, 5 repetitions (seeds
  700001 to 700090, plan order). Combination as Track 10: log10 of the mean
  of the totals; 95% interval = log10(mean +- t(4) sd / sqrt(5)).
- Baseline `rows-b-lr` is remeasured here with its own test seeds; Track
  10's 10^40.457 [40.440, 40.473] is reported beside it.
- Uninformative (Track 10's rule): if an order's totals span more than 2
  decades, it is reported as not measured.

### A3. Gates, per order (stages gate6, gt; before the order's number is read)

An order whose gate fails is reported as **not measured**, not patched.

- **G6, named orders.** The 30 hard-ladder 6x6 test instances, the order's
  6x6 analogue, twice: no clue; and the five clue analogues (3, 2), (1, 1),
  (1, 4), (4, 1), (4, 4) fixed from the planted board. N = 200, 200
  repetitions, exact widths by enumeration (cap 2e10). Track 10's rule per
  variant: share of (instance, depth) pairs more than 3 standard errors from
  exact at most 2%, median over instances of |log10(mean total / exact)| at
  most 0.05, every instance enumerated. Both variants must pass.
- **GT, every order, full size, real pieces.** The official five-clue tree in
  that order truncated at free depth K (K = the largest depth whose model
  cumulative node count is at most 10^9; `plans/gt-depths.json`: 8 for the
  line scans, spirals, frame-first and boustrophedons, 14 antidiag, 12
  bands2, 5 s5-sa, 19 s6-beam, 3 s4-sa), exact by enumeration (cap 3e9),
  against the per-depth mean of 10 SMC repetitions at N = 1e5 (seeds 600001
  onward). Pass iff at most 10% of the depths 1..K are more than 3 standard
  errors from exact, |log10(mean W_K / W_K)| <= 0.05 and |log10(mean sum /
  exact sum)| <= 0.05. GT tests the estimator in that order at full size but
  only over the first K levels; G6 tests whole trees on small boards.
- A named order must pass G6 and GT; a searched order GT (it has no 6x6
  analogue).

### A4. Readings (fixed by the lead)

- An order **beats row scan** iff its measured log10 size is lower than
  `rows-b-lr`'s by more than 1.0 AND the two 95% intervals do not overlap.
- **Kill line: no order beats row scan.**
- Reported whatever the outcome: the best measured order and its gain over
  `rows-b-lr` (difference and interval [lo_rows - hi_best, hi_rows -
  lo_best]); the spread among the eight line scans (the "wrong side" cost,
  in bits).
- **Model against measurement.** Measured minus model per order; Spearman
  rank correlation over the measured orders (average ranks for ties). The
  model "can replace the estimator for later static-order searches" iff
  Spearman >= 0.9 AND every order the model puts within 0.5 dex of its best
  is measured within 1.0 dex of the measured best AND no order the model puts
  more than 1.0 dex above row scan is measured below row scan. Else it
  cannot, and the report says which condition failed.
- **Null.** The same orders on the three Part A null boards, N = 1e5, 5
  repetitions (seeds 800001 onward). Per board: gains over `rows-b-lr`, the
  best order, Spearman with the official ranking. An official gain is
  "ordinary" if it lies within [min - 0.2, max + 0.2] of the three null gains
  (0.2 dex for estimator noise at N = 1e5), else flagged as specific to the
  official set. An order that beats row scan on the official set and on no
  null board is reported as such.

## Part B (RQ5)

Five Part B boards, five clues from the planted solution, order **rows-b-lr**
(the model's best row scan, tied in the model with the other bottom and left
line scans; Track 10's order). A pin = the planted piece, with its rotation,
at its planted cell.

### B1. What calibration showed, and why Part B has two estimators

On calibration board 100149 (section 3): with the model's greedy pins
(clustered in rows 11 to 15, the start of the row scan), plain SMC loses the
tree. At k = 12 every repetition died at free depth 21 to 53 and reported
10^7.2, the model said 10^8.9, and exact enumeration ran past 1e10 nodes
without finishing (k = 16: model 10^1.5, enumeration also past 1e10). The
partial enumeration shows why: the pins squeeze the width to a few states
around free depth 56 to 64 (the planted prefix and a few others), and the
unpinned rows above then re-expand to widths of 4e8 by depth 136 in the
first subtree alone. Plain SMC keeps a uniform sample and loses the few
pin-compatible prefixes; the model, which ignores the planted solution,
multiplies by 784 per pin and goes below one node. Neither is valid in this
regime. With pins spread along the order (section B3 "spread"), both
estimators agree (k = 16: SMC 10^13.0 to 10^13.1, PPD 10^13.0 to 10^13.2).

So every multi-pin configuration is measured by two estimators:

- **SMC**: Track 10's, N = 1e5.
- **PPD (planted-path decomposition)**: every node either lies on the
  planted path or leaves it first at some step j. W_d = 1 + sum_{j<d}
  D_j(d), D_j(d) = number of depth-d descendants of the siblings of the
  planted child at step j. Each D_j is estimated by an independent run of
  Track 10's population step started from those siblings
  (`oap::smc_from`), N = 1e4. Unbiased whenever SMC is; the planted path,
  which plain SMC loses at a bottleneck, is counted exactly. It does not
  remove the rare-event problem for non-planted prefixes that survive a
  bottleneck, so it can also be low.

Both estimators are unbiased in expectation and fail low in practice (they
miss rare surviving prefixes). Rule for a configuration with both: value =
the larger of the two log10 means; "under a budget" requires both 95% upper
bounds under it; if the two log10 means differ by more than 1.0 the
configuration is labelled **lower bound only**. Single-pin and k = 0
configurations use SMC (calibration: k = 1 behaves like k = 0, every
repetition reaches depth 185 or more, 95% half-width 0.05 dex at 5
repetitions), with PPD on a subset as a cross-check.

### B2. Configurations (sets in `plans/partb-sets.json`)

- **b0, base, k = 0.** SMC N = 1e6, 5 repetitions; PPD N = 1e4, 5.
- **b1, one pin at each cell of S1** (56 cells): the 6 x 6 grid r, c in {0,
  3, 6, 9, 12, 15} (36 cells, corners included); frame midpoints (0, 7),
  (15, 7), (7, 0), (7, 15); the model's best three and worst three single
  cells, ties by position ((11, 1), (11, 2), (11, 3); (2, 15), (0, 0),
  (0, 15)); the first 8 greedy cells and the 8 spread-8 cells (for the
  additivity test). SMC N = 1e5, 4 repetitions. PPD N = 1e4, 3 repetitions,
  on (11, 1), (11, 3), (14, 1), (13, 5).
- **b2, greedy in the model** (the brief's placement): add the cell with the
  largest model gain, ties by position. Sequence (11, 1), (11, 3), (11, 5),
  (13, 4), (14, 1), (13, 6), (14, 3), (14, 5), (14, 7), (14, 9), (14, 11),
  (14, 14), (15, 2), (15, 4), (15, 6), (15, 8). The model tree falls under
  2^78 nodes (10^23.48) at k = 6 and under 2^61.5 (10^18.51; 1,000
  core-years at 1e8 nodes per second) at k = 8. k = 1, 2, 4, 6, 8, 12, 16.
- **bs, spread** (added because the model's greedy is invalid in the
  clustered regime, B1): k pins at evenly spaced free depths of the row scan
  from 16 to 160 (k = 1: depth 88), each moved forward along the order to the
  first middle cell not adjacent to a clue or an earlier pin. k = 1, 2, 4, 6,
  8, 10, 12, 16, 24, 32, 40, 48.
- **b3, random**: uniform random sets of k non-clue cells,
  `random.Random(1000 (board index + 1) + 10 k + set)`, two sets per board
  and k; k = 1, 2, 4, 8, 16, 24, 32, 48.
- b2, bs, b3: SMC N = 1e5, 5 repetitions, and PPD N = 1e4, 4 repetitions,
  per configuration.
- **b4, end-to-end**: exact enumeration of the whole tree (cap 2e10 nodes)
  for spread k = 40 and k = 48 on all five boards (calibration board: spread
  48 at 10^8.2 by both estimators). Records the exact total, the number of
  complete boards and the node count at the first. The greedy sets are not
  enumerated (calibration: past 1e10 at k = 12 and 16).
- Seeds: one per repetition from a counter per stage (b0 900001, b1 910001,
  b2 920001, bs 925001, b3 930001 onward), as written in the plan files.

### B3. Quantities and readings

- **Bits per pin at a cell** = log2(T0 / T1) on each board (T0 SMC of b0, T1
  SMC of b1); per cell the mean over the five boards and the range. Best
  cell = largest mean, worst = smallest, typical = median over S1. Compared
  cell by cell with the model (Spearman, mean difference in bits). The PPD
  cross-check is reported (largest |PPD - SMC| in dex).
- **Pins to a budget**, for 2^78 and 2^61.5: for greedy and spread, per board
  the smallest k in the grid whose configuration is under the budget, and
  the k at which at least half the boards are; for random, per k the share
  of the 10 (board, set) pairs under the budget, and k50 = the smallest k
  with share >= 0.5. Not reached in the grid: "more than the largest k".
- **Additivity.** R_k = log(T0 / T_k) / sum over the set's cells of log(T0 /
  T1(cell)), per board, for greedy k = 2, 4, 8 and spread k = 8. Pooled:
  mean over boards with a t interval. Sub-additive iff the interval lies
  below 1; super-additive iff above; else additive within resolution.
- **End-to-end (b4).** Passes on a board iff the enumeration completes, finds
  at least one complete board, and the exact total lies inside each
  estimator's 95% interval or within 0.10 dex of its mean. The check passes
  at a k iff it passes on at least 3 of 5 boards. Reported with the node
  count at the first solution against the total. If it fails at both k, the
  Part B numbers below 10^12 are labelled unvalidated.
- **Model against measurement**: measured minus model per greedy k, per
  random k (model median of 200 sets), per single cell.
- **Real pieces (argued).** The model's single-pin map and its greedy k are
  the same for the official set (same census). Reported, labelled argued,
  next to the painted-board measurement of the same policy, which is the
  better argued figure for the real pieces where the model fails.
- The survey figure "about 6 bits per tile, 9 to 11 tiles" is not an input;
  the report compares the measurement with it.

## 3. Calibration and sample sizes (calibration board 100149 only)

Raw: `results/calib/calib.jsonl` (plan `plans/calib.jsonl`, script
`scripts/calib_summary.py`). Not test seeds; run before this file.

| configuration | N | reps | log10 T [95%] | sd of rep log10 | s / rep |
|---|---|---|---|---|---|
| rows-b-lr | 1e5 | 10 | 40.508 [40.484, 40.531] | 0.033 | 3.5 |
| rows-b-lr | 1e6 | 3 | 40.516 [40.505, 40.528] | 0.005 | 48 |
| rows-t-rl | 1e5 | 10 | 40.610 [40.560, 40.654] | 0.064 | 3.4 |
| antidiag | 1e5 | 10 | 45.123 [43.789, 45.414] | 0.341 | 2.4 |
| antidiag | 1e6 | 3 | 44.969 [-, 45.285] | 0.181 | 26 |
| spiral-in | 1e5 | 10 | 43.572 [42.992, 43.812] | 0.504 | 2.4 |
| spiral-in | 1e6 | 3 | 43.424 [-, 43.867] | 0.321 | 27 |
| spiral-out | 1e5 | 10 | 50.217 [50.203, 50.231] | 0.019 | 3.7 |
| frame-then-rows | 1e5 | 10 | 50.736 [50.593, 50.843] | 0.192 | 3.6 |
| greedy k = 1 | 1e5 | 10 | 37.633 [37.600, 37.664] | 0.047 | 4.5 |
| greedy k = 4 | 1e5 | 10 | 29.203 [28.028, 29.490] | 4.9 (bimodal: 5 reps die at depth 63) | 2.6 |
| greedy k = 12 | 1e5 | 10 | 7.227 (all reps extinct by depth 53; exact > 1e10) | 0.057 | 0.3 |
| PPD k = 0 | 1e4 | 3 | 40.548 [-, 40.869] | 0.198 | 16 |
| PPD greedy k = 4 | 1e4 | 3 | 27.143 [-, 27.810] | 0.81 | 8.3 |

Exploratory, not in the calibration file (commands in the README): spread
k = 8, 12, 16, 32, 48 on board 100149, SMC and PPD, 2 repetitions: 10^23.2,
10^15.5 to 10^17.5, 10^13.0 to 10^13.2, 10^10.6, 10^8.2.

Sample sizes chosen: official orders N = 1e6, 5 repetitions (row scan 95%
half-width projected 0.006 dex; the high-variance orders antidiag and
spiral-in about 0.2 to 0.3 dex, enough for a 1.0 dex reading and for ranks);
null boards N = 1e5, 5 (0.04 dex for row scan, up to 0.4 for the noisy
orders, hence the 0.2 dex margin in the null reading); singles N = 1e5, 4
(about 0.06 dex = 0.2 bits per cell); multi-pin sets 5 SMC + 4 PPD
repetitions (the wide spread of the bottleneck regime is reported, not
hidden).

Model departures already visible in calibration (spiral-in model 46.05
against 43.4 to 43.6 measured on board 100149; spiral-out 50.39 against
50.22) are reasons to measure, not results.

## 4. Compute budget

This Mac, at most 2 single-threaded processes (`JOBS=2`), each `nice -n
10`. Every stage writes one JSON line per unit, flushed, and resumes on
rerun (`scripts/drive.py`). All stages go through `../common/prereg_guard.py`
from `run.sh`: gate6, gt, a-official, a-null, b0, b1, b2, bs, b3, b4.

Estimated core-time: gate6 0.05 h, gt 0.4 h, a-official 1.0 h, a-null 0.25
h, b0 0.4 h, b1 1.3 h, b2 0.5 h, bs 0.6 h, b3 0.9 h, b4 0.5 h: about 5.9
core-hours, plus about 0.8 for calibration and checks already spent. Aim 6
to 7 in total, never above 12. Order of stages: gates, a-official, a-null,
b0, b1, b2, bs, b3, b4. If the owner takes the cores back, the run stops
and resumes; a stage cut short is reported with the units that ran, and cuts
start from b3's second set, then b1's PPD subset.

## 5. Analysis

`scripts/analyze.py` reads every raw file, prints every number of the
report and writes `results/summary.json`. If code and text disagree, the
text governs and the code is fixed under an amendment.

## 6. What would make this uninformative

- Part A stops and is reported as a failed reproduction if `rows-b-lr` fails
  its gates or its official estimate differs from 10^40.457 by more than 0.1
  dex.
- Part B: if the b0 trees of the five boards span more than 1 dex, bits per
  pin are reported per board only.
