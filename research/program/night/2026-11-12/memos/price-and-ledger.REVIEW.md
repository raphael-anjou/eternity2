# Referee review: the price of search and the cost ledger

Night 2026-11-12. Reviewed: `solve-path-5-price.md`, `solve-path-3-exponent.md`,
`solve-path-2-designed-instance.md`. Scripts and outputs of this review:
`research/program/night/2026-11-12/memos/price-and-ledger-review/`
(`rv_lib.py`, `01_price` to `07_real_set_small_regions`, each `.py` with its
`.out`). Compute: 3 minutes to rerun the three memos' scripts, 4 minutes for
my own, one core, stdlib Python. No solver run on the 16x16 and no sampling of
the real puzzle's tree. Web: three groups.io messages (9688, 9751, 6023)
returned HTTP 402 and were not read; the volunteer project's page was read.

Rerun result: all sixteen script runs of the three memos reproduce their
committed outputs line for line (the 60 second hill climb differs only in its
proposal count; path 3's 140 million node 6x6 run was not repeated).

## 1. Verdict table

| # | Claim | Verdict | P(true as stated) | Reason in one line |
|---|---|---|---|---|
| A1 | Path 5: five clues, 3.1e40 nodes to exhaustion, 1.5e40 to the first solution, range 1e39 to 1e42; one clue 1.4e47 and 9.3e42 | holds | 0.85 (that the real tree is inside the range) | My independent implementation gives 10^40.49, 10^40.19, 10^47.14, 10^42.97 |
| A2 | Path 3: 10^40.5 with five clues, peak 10^39.2 at depth 161 | holds with a correction | 0.85 for this row; 0.03 for its other scan direction | The quoted row is right to 0.03 digits. Its top-down figures (10^41.38, peak 10^40.66) are 0.76 and 1.32 digits too high: clue joints are closed late |
| A3 | Path 2: row scan with five clues is 2^150.5 = 10^45.3 | wrong | 0.02 | Same unit, same clues, same pieces. The match probability of the full board is spread evenly over joints; that alone adds 4.64 digits. Exact value in its own census and order: 10^40.66 = 2^135.1 |
| B1 | Small-board calibration (real trees 1.1 to 2.0 times the model) | holds | 0.95 | Recomputed on the 30 committed 6x6 instances: 1.369, sd 0.161 dex (path 5: 1.37, 0.16) |
| B2 | "Nothing suggests an error beyond 0.3 orders for a random 16x16" | not established | 0.5 | The model is unbiased on its own ensemble, but the spread between instances is 0.15 to 0.37 dex at 16 to 36 cells and nobody has its scaling to 256 |
| C1 | Ledger: correct frame given, interior costs 10^14.8 nodes (70 core-days) | holds with a correction | 0.85 (that it is below 10^15) | 10^14.8 carries the late-clue error and a poor interior order. Clue on board: 10^14.1 as scanned, 10^13.0 scanned from the other side, 10^8.3 by inward spiral |
| C2 | Ledger: split given, each half peaks at 10^9.9 instead of 10^39.8 | holds with a correction | 0.85 | 10^9.1 instead of 10^38.75 with the clue on the board |
| C3 | "Refuting a wrong frame costs as much as confirming the right one" | holds | 0.8 | Small boards, every frame enumerated: the true frame costs 1.1 to 3.7 times the mean frame; median frame is half the mean |
| C4 | Frame-first search is dearer than row scan in the model (path 5: 10^50.75) | holds with a correction | 0.9 that it is dearer | 10^37.6 frames times a mean of 10^8.3 is 10^45.9 with a spiral interior, 5.5 digits above row scan. Break-even needs a mean of 10^2.9 nodes per frame |
| D1 | Multiplier arithmetic, gap table | holds | 0.95 | Every row recomputed |
| D2 | Credible product 2.6 orders, ranking worth 25 to 150 | not established | 0.25 for the ranking range on this puzzle | The 25 was measured on unframed sub-solution hunts; the 10x10 figure needs the unranked solution rate, which no source in the repo gives |
| D3 | Remaining gap about 17 orders at 1e5 core-years, no combination closes it | holds with a correction | 0.9 that the gap is 15 orders or more | Without the unestablished ranking factor the gap is 19 to 20 orders, not 17 |
| E1 | Path 3: no seam join, quadrant split, join tree, merging or signature changes the exponent; best gain 0.8 digits | holds with a correction | 0.85 | Best gain is 1.0 digits with the clue on the board; at most 2.4 digits for any band join that stores both lists. The 10^31 join-tree bound covers materialising joins only |
| E2 | Path 2: zero-information theorem | holds | 0.99 as a theorem | It is Bayes' rule. Its use needs narrowing: it removes design traces, not solution marginals |
| E3 | Path 2: twelve statistics inside the null | holds | 0.95 | Reproduced exactly; the battery has no power against coarse stratification, as the memo says |
| F1 | Population estimator is unbiased for the width | holds | 0.95 | Simulated against exact widths: mean ratio 1.002 (s.e. 0.030) and 1.033 (s.e. 0.017) |
| F2 | "A few percent on the peak width" at N = 1e6; decision rule at 3 orders from the model | not established | 0.4 | The variance formula uses the wrong term, and the rule has no null for instance-to-instance spread |

## 2. The reconciled price

**Five clues, row scan starting at row 15 of the CSV coordinates (the rim row
next to the two clues of row 13), a node being one valid partial board with
the clues on the board from the start: 10^40.49 = 3.1e40 nodes to exhaustion,
1.5e40 expected to the first solution. Defensible range 10^39.5 to 10^41.5,
which I hold at about 0.8; path 5's wider 10^39 to 10^42 at about 0.9. One
clue: 10^47.14 to exhaustion, 10^42.97 per expected solution.**

What each memo's number measures:

| Memo | Number | What it is |
|---|---|---|
| Path 5 | 10^40.49 | The expected number of valid partial boards summed over the 251 free-cell levels, Owen's census (interior sides and inward edge sides pooled), exact probability of the joints closed, clues on the board. This is the price. |
| Path 3 | 10^40.52 | The same quantity plus five extra levels at the clue cells, and with clue joints closed only when the scan reaches the clue. Harmless in this direction (the clues are passed before the peak), not in the other. |
| Path 2 | 10^45.30 | Not a node count of any solver. Its ensemble is finer (inward edge sides are their own class, 17,130 solutions instead of 14,702) and that part is sound. But it charges every joint the geometric mean of the full-board match probability (1/16.70 inner, 1/17.20 seam, 1/4.39 ring). The exact cost of an early joint is 1/17.34 and 1/5.00; the last joint of each colour is free. Spreading moves those free joints to every depth. The error grows with depth: 0.8 digits after 16 cells, 4.9 after 160, 6.4 after 224, zero at 256. |

Decomposition of the five orders (`01_price.out`, section 3):

| Step | Total | Change |
|---|---|---|
| Path 2 as written | 10^45.30 | |
| Same census and order, exact probabilities | 10^40.66 | 4.64 digits (the error) |
| Pooled census | 10^40.62 | 0.04 digits (census choice, legitimate) |
| Scan from row 15 instead of row 0 | 10^40.49 | 0.13 digits (order) |

Unit, clue treatment, frame, piece distinctness and rotations are the same in
all three memos. Path 2's "placements" are valid partial boards, the same unit
as "nodes".

Consequences for path 2: 2^150.5 becomes 2^135.1 (its order) or 2^134.6 (the
better direction). "93 bits short" becomes 78.0 for three desktops and 73.6
for 1,000 core-years, which is path 3's 73. "Ten more correct placements"
becomes eight (`05_extras.out`: plus 8 gives 2^58.7, plus 9 gives 2^49.1).
The quadrant row of its section 6 moves from "5 to 46 bits short" to "0 to 31
bits short". Its 18-hint cross-check moves from 2^32.7 and 2^34.5 to 2^29.7
and 2^31.5, still inside the 2^33 to 2^36 it compares with only at the low
end.

## 3. Claim by claim

### A. The price

My implementation (`rv_lib.py`): the probability that e interior joints and f
frame joints are all monochromatic, in the ensemble where the colour multiset
of each class is a uniformly random arrangement over the slots of the class,
derived from scratch as a coefficient of a product of per-colour polynomials
and computed in exact integers. Checks: brute force over all arrangements of
tiny colour multisets agrees to six digits; a two-class version (seam joints
separate) agrees with Monte Carlo and reproduces path 2's closed form at the
full board (17,132 against its 17,130).

Results for the official set (`01_price.out`):

| Clues | Scan | Census | Total | Peak | at board cell |
|---|---|---|---|---|---|
| five | row 15 first | pooled | 10^40.49 | 10^39.20 | 161 (158 free cells) |
| five | row 15 first | class-aware | 10^40.53 | 10^39.23 | 161 |
| five | row 0 first | pooled | 10^40.62 | 10^39.34 | 134 |
| one | row 15 first | pooled | 10^47.14 | 10^45.86 | 161 |
| one | row 0 first | pooled | 10^47.21 | 10^45.86 | 161 |

98 percent of the five-clue nodes lie between free-cell depths 129 and 175,
as path 5 says. Other solutions with five clues: 4.1e-8 (pooled), 4.8e-8
(class-aware). McGavin's published checkpoints are reproduced by path 5's port
(rerun) and my totals match it to two decimals.

Why the range. Three terms, largest first:

1. Spread between instances and the designed set (section B below): plus or
   minus 1 order, argued from small boards.
2. Model against planted boards: plus 0.0 to plus 0.2 digits.
3. Node definition (hint levels, scan direction): 0.03 to 0.13 digits.

### B. Calibration

Recomputed (`02_calibration.out`): the 30 committed hard-ladder 6x6 c=5 test
instances, my enumerator, my model, row scan. Tree over model: geometric mean
1.369, sd of log10 0.161, range 0.56 to 2.91, 372 solutions in total. Path 5
reports 1.37, 0.16 and 372. Per depth the ratio climbs from 1.05 at depth 4
to 1.5 through the bulge and then explodes at the tail (5 at depth 32, 196 at
depth 36), where the planted solutions and their relatives are the whole
count.

The control neither memo ran: the same census with the colours reshuffled
inside each class and no planted solution, which is the ensemble the model is
an expectation over. There the model must be unbiased, and it is: mean
tree/model 1.16 (s.e. 0.10) on 30 draws of the committed censuses, and 1.02
(0.08), 0.96 (0.08), 0.95 (0.07) on 4x4, 5x5, 6x6 draws
(`04_smc_and_variance.out`). So the formula is exact for its ensemble and the
1.1 to 2.0 of the memos is entirely the planted tilt.

Two things follow that the memos do not say.

- **The planted excess has no established trend with size.** Path 5 writes
  that it shrinks with size; path 3 writes that it rose slightly. Both read
  boards whose colour counts change with size. The committed 6x6 instances
  (1.37 and 1.61) sit above path 5's own 6x6 rows (1.13 and 1.32) at the same
  size. The excess follows the number of planted relatives, not the side
  length. With five clues the 16x16 has one planted solution in a width of
  10^39, and the theory referee's overlap sum puts the relatives at about 0.03
  of one solution, so I expect an excess below 0.1 digits (argued).
- **The model's own ensemble is not concentrated at this size.** Sd of log10
  of tree/model across unplanted draws: 0.27 (4x4), 0.26 (5x5), 0.16 and 0.35
  (two 6x6 samples of 24 and 30). At the peak depth: 0.28, 0.37, 0.15. The
  first row alone gives 0.07 to 0.12. No growth with size is visible in three
  sizes, and three sizes cannot show one.

What the repo holds about the real pieces at any scale
(`07_real_set_small_regions.out`):

| Region, official set | Exact (committed) | Model, class-aware | Ratio |
|---|---|---|---|
| 2x2 corner block | 1,312 | 1,303 | 1.007 |
| 2x2 side block | 73,003 | 73,220 | 0.997 |
| 2x2 interior block | 4,059,952 | 4,062,000 | 1.000 |
| 3x3 corner block | 2,633,221 | 2,620,000 | 1.005 |
| 3x3 side block | 595,003,203 | 597,000,000 | 0.997 |
| All 60 frame cells | 10^37.608 | 10^37.714 | 0.784 |

At 4 to 9 cells the official set is within 0.7 percent of the class-aware
model (2.6 percent of the pooled one). At 60 cells, frame only, it is 0.78.
Between 9 interior cells and 256 there is no exact datum.

The 2008 sampling figure does not calibrate anything. The site's own table
(`fill-order.mdx`, line 282) records that its clue set and node definition
are unknown, and the 15 percent it found between orientations is not the
factor 1.35 (five clues) or 1.17 (one clue) the model gives between scan
directions. Agreement with 3.1e40 to within 25 percent is as likely a
coincidence of conventions as a confirmation. The "99 percent of time beyond
depth 132" sample on the complex-theory page is a depth histogram of one
billion iterations of one run; it describes a local subtree.

**Error bars at 16x16.** The model value is an expectation. For a typical
planted instance of the ensemble I put the sd of log10 width at 0.5 (twice the
largest small-board value, since the scaling is unknown), the planted excess
at 0 to 0.2, giving 10^40.5 plus or minus 1 at two sd. For the designed set
nothing more can be said from data: the theorem of path 2 makes it a typical
planted draw if the painter was exchangeable, the no-duplicate filter keeps
0.8 percent of draws (path 2: 247 of 30,000) and its effect on width is
unmeasured, and the one large exact count on the real pieces is 0.1 digits
off. A model validated at 36 cells is trusted at 256 on the strength of its
derivation (it is exact for the ensemble at every size), not on the strength
of the calibration. The unverified part is concentration, and it is
measurable (section 6).

### C. The ledger

**Derivation.** Path 3's "interior given the correct frame" is the model's
count after the 60 frame cells divided by the number of valid frames, that is
the mean interior tree over all valid frames. It is not a statement about the
true frame in particular, and the memo says so in its last sentence of the
section. Three corrections to the number (`03_ledger.out` 1b,
`06_interior_orders.out`):

| Interior order under a given frame, five clues | Mean tree per frame |
|---|---|
| Path 3 as computed (rows from row 1, clue joints closed late) | 10^14.78 |
| Same order, clues on the board | 10^14.09 |
| Rows from row 14 | 10^13.04 |
| Inward spiral | 10^8.34 |
| Spiral plus 259,000 hill-climb proposals | 10^8.34 |

The spiral wins by 4.7 digits because a region whose whole boundary is known
closes four extra joints per ring, and because it passes all four outer clues
in its first two rings. So the ledger's direction is right and its size is
understated: given the true frame the interior costs about 10^8.3 nodes in
the model, seconds, not 70 core-days. The split row: 10^9.11 peak and 10^9.79
total for the top half with its true pool, against 10^38.75 with the free
pool.

Under row scan the width factorises the same way (`03_ledger.out` 1c): at 160
free cells, 10^39.12 = 10^30.08 valid partial frames (35 frame cells) times
10^9.03 interior fills per partial frame. Thirty of the thirty-nine digits
are frame multiplicity. "Nearly all of the cost is which piece goes where on
the frame and in which zone" holds as arithmetic of the model.

**Small-board evidence.** Path 3's `ledger_check.py` reruns identically. Its
summary "2 to 5 times the model" describes means; per seed the ratio runs
from 0.75 to 11.3 and one 8x8 seed is censored at the cap. My own check
(`03_ledger.out` part 2) enumerates every valid frame of five planted 6x6
boards (46,000 to 110,000 frames each) and the interior tree under each:

| Board | Frames (model) | Mean tree per frame (model) | Median | Zero-node frames | True frame | Frame-first total / row scan (model ratio) |
|---|---|---|---|---|---|---|
| 6x6 (4/6) s1 | 110,080 (51,400) | 17.1 (15.5) | 10 | 17% | 35 | 8.8 (7.8) |
| 6x6 (4/6) s2 | 71,808 (71,900) | 37.5 (25.3) | 29 | 4% | 89 | 4.5 (5.1) |
| 6x6 (4/6) s3 | 46,208 (77,000) | 45.3 (37.3) | 24 | 8% | 103 | 3.3 (3.5) |
| 6x6 (4/5) s1 | 59,904 (71,900) | 540 (303) | 171 | 14% | 1,995 | row scan above cap |
| 6x6 (4/5) s2 | 49,408 (51,400) | 141 (89.6) | 74 | 5% | 154 | 1.2 (1.3) |

The mean tree per frame is 1.1 to 1.8 times the model; the true frame costs
1.1 to 3.7 times the mean frame; the median frame is about half the mean; the
heaviest 1 percent of frames hold 4 to 8 percent of the nodes. Path 3's "2 to
5 times" is the product of the first two factors. The model predicts the
ratio of frame-first to row scan within 15 percent on these boards.

**Against what the repo knows.**

- 10^37.608 valid frames (committed exact count). The model gives 10^37.714.
- "36 percent of perfect frames cannot start the interior" (fluid-frame page).
  This is a piece-set statistic and it reproduces without sampling: 269 of the
  289 ordered colour pairs occur on two clockwise-adjacent sides of some
  interior piece; with the official inward colours an inner-corner cell has no
  candidate with probability 0.094, and at least one of the four with 0.326
  (`03_ledger.out` 1f). The page reports 9.2 to 11 percent and 36 percent.
  Consistent, and consistent with the ledger: those frames are refuted in at
  most four lookups, and the model's mean already contains them (17 percent
  zero-node frames on a small board did not move the mean off the model).
- "Frame-first construction plateaus", "no border effect" (frame-is-not-the-
  basin, F = 1.24, p = 0.25). Consistent. A sampled perfect frame is the true
  one with probability 10^-37.6; under each wrong frame the model has 10^7 to
  10^13 partial interiors at the widest level and 10^-45 completions. A width
  128 beam sees the same statistics under every frame. These pages say
  nothing against "the interior under the true frame is easy", because they
  never held the true frame.

**The product over frames.** The sum over frames of the refutation cost is
the number of frames times the mean, whatever the skew, and the mean is a
first-moment quantity that the model gives directly. With the exact frame
count: 10^37.608 x 10^8.34 = 10^45.9 (spiral), 10^50.6 (rows). Row scan:
10^40.49. Frame-first is 5.5 digits dearer at the best interior order I
found, and equals row scan only if the mean cost per frame were 10^2.9 nodes.
The 36 percent of frames that die at once lower the median, not the mean.
Path 5's "frame first, then row scan: 10^50.75" is the row-interior variant
and I reproduce it exactly; with the spiral interior the same family costs
10^46.0.

**Reconciliation with "fill order is a spent lever".** No contradiction. The
ledger is a statement about side information, not about an order. Turned
into a search by enumerating frames it costs more than row scan, because row
scan only ever holds the 35 frame cells it needs and prunes them against the
interior as it goes. What the ledger does say is the size of the oracle that
would pay: 1,000 core-years (10^18.5 nodes) at 10^8.3 per frame tests
10^10.2 frames, so the oracle must rank the true frame in the first 10^10.2
of 10^37.6, which is 27.4 digits or 91 bits about the frame. Path 3's "first
10^3.7" was computed with 10^14.8. One caution on path 5's side: its ten
families contain one frame-first order, and that order was 4.7 digits from
the best of its family. "Row scan is the best of the families tried" is what
the table supports; I found nothing below it either.

### D. The multipliers

Arithmetic (all recomputed): one core-year is 3.156e15 nodes at 1e8 per
second; 1.5e40 is 4.9e24 core-years; gaps 22.2, 21.7, 20.6, 19.7, 18.7, 18.0,
17.2, 11.7 orders; 2 x 2 x 100 = 400 = 2.6 orders; 17 orders = 56.5 bits =
5.9 placements at 2.89 orders each; the 1/Phi(-sigma) row. The site fix is
right: 3.1e40 nodes at 1e8 per second is 9.8e24 years.

The ranking factor is the whole product and it is not established for this
puzzle.

- The factor 25 "by depth 81" (msg 9751) is described on the site's own
  McGavin page (lines 286 to 291) as measured on unframed sub-solution hunts
  with a 13 or 14 ply lookahead. That is a rate of finding sub-solutions in a
  many-solution search, not the cost of a first solution in a one-solution
  tree.
- The 10x10: found at ranked row 92,907 of 19,864,204, against a predicted
  one success per 70,000 ranked rows. That confirms the prediction at the top
  of the ranking (95 percent range 1,800 to 258,000 rows). The gain over an
  unranked scan is the ratio of that rate to the average rate over all rows,
  and the average rate is not in any source in the repo. The solution found
  differs from the seed, so the tree holds at least two solutions, eight if
  all rotations are in it; with eight an unranked scan expects the first at
  one ninth of the tree and the gain is 24, with one it would be 106. Path 5's
  "40 to 150" assumes at most four.
- Path 2's theorem and this multiplier coexist only because ranking by
  expected completions estimates solution marginals; it is not a design
  trace.

Double counting and omissions: "per-core tuning 2" is applied to a baseline of
1e8 nodes per second that is already the top of the reported range (the
committed measurement is 24 to 41 million for the break-tolerant engines,
`single-core-benchmark/results/REPORT.md`; the 105 to 110 million figure has
no committed result file); "GPU per euro 2" is a money multiplier applied to a
gap stated in core-years; parallel efficiency (0.5 to 1.0) is in the table
and not in the product. Net: the supported product is between 1 and 4 (0 to
0.6 orders); with the ranking factor at face value, 2.6. The gap at 1e5
core-years is 17 to 20 orders. The conclusion does not move.

Hardware rates. The volunteer project's page read 923 G nodes per second on
176 computers when I opened it, against 1.1e12 on 172 earlier the same day.
It defines no node. It reports 466/480 boards, so the application is a score
hunt that tolerates mismatches, and its nodes are not strict-search nodes.
5.2 to 6.4e9 per computer is two to three times the best measured single-GPU
figure in the memo. Use it for the order of magnitude of the installed base
only; the "20.6 orders" row carries at least plus or minus 0.5.

### E. The negative claims

**Path 3.** With the clue on the board the seam table changes
(`03_ledger.out` 1e): gain 1.00 digits at r = 7 and 0.97 at r = 8 (memo: 0.83
and 0.08). A bound that does not depend on how lists are generated: a
two-list band join that stores both lists pays at least the larger list,
10^38.1 at r = 8, so the gain cannot exceed 2.4 digits. The claim holds with
"0.8" replaced by "1.0, and at most 2.4".

Loopholes looked for:

- Cheaper list generation. I found one real case (the spiral, 4.7 digits)
  but it applies to regions bounded on all sides, which a band is not, and it
  cannot beat the list size.
- The join-tree statement "no join tree does better than about 10^31" is a
  bound for trees that materialise the full fill list of a region of 86 to
  171 cells, evaluated on bands and corner squares only. A pipelined join
  never materialises that list (row scan is one, and costs 10^40.5), so the
  bound is not a lower bound on all exact methods. It also sits 9 digits
  below the two-list figure; only the unproved "no hashable distinctness for
  three parts" argument closes that interval. Narrow the claim to: no
  materialising decomposition into bands or quadrants.
- Merging. The two wrong formulas (states per fill, occupied seams) do not
  touch the 16x16 conclusion. Merging needs two fills of the same piece set
  with the same seam. The model's count of other fills of the true top-half
  pool is 10^-34.8 (`03_ledger.out` 1d), and the official set has no
  duplicate and no rotation-symmetric piece (checked again here), so the
  smallest rearrangement is a block exchange. The factor 2 to 4 measured on
  small palettes comes from duplicate pieces, which those boards have and the
  official set does not. Still argued for the official set, not measured.

**Path 2, the theorem.** Correct: P(B | S) is proportional to mu(B) on
Sol(S), so a mu that depends on B only through S gives a uniform posterior.
Narrowing needed in three places.

- "Zero bits" means zero bits from the making process. With five clues
  Sol(S) has one element, so S determines the whole arrangement. The theorem
  is about information, not about the cost of extracting it.
- Consequence 1 and item 1 of "what is new" retire "classifiers of true
  neighbours trained on generated boards". Under the theorem such a
  classifier estimates the marginals of Sol(S), which are not uniform over
  side pairs and are exactly what the ranking of path 5 uses. What is retired
  is the hope of telling the planted solution from other solutions.
- The hypothesis covers rejection rules that read the pieces. It does not
  cover a clue-cell choice made after seeing the board, or judges who saw the
  board. Both are undocumented, as the memo says.

I found no counter-example to the theorem. The numbering check reproduces:
each class is contiguous, sorted by minimal rotation, stored in minimal
rotation, with 0 duplicates and 0 symmetric pieces.

**Path 2, the statistics.** `01_piece_set_traces.py` at 30,000 boards and
`03_stratified_painters.py` at 3,000 reproduce every figure. The conditional
columns rest on 247 boards, so their sd carries about 5 percent noise. The
battery cannot see two or four strata, which the memo states and measures.

### F. The proposed first measurement

**Unbiased?** Yes for the width at each depth and for their sum, when each
level keeps either all extensions or a uniform sample of fixed size. Checked
on committed instance n6-c5-s1 against the exact width at depth 20
(`04_smc_and_variance.out`): mean estimate over exact 1.002 (s.e. 0.030) at
N = 50, 1.033 (s.e. 0.017) at N = 200. It is not unbiased in the logarithm:
at N = 50 the median run is 0.85 of the truth and the mean of log10 is low by
0.06. Seeds must be combined by averaging the estimates, then taking the log.

**Variance.** Path 5 uses the squared coefficient of variation of the
offspring count. The term that governs the estimator is the squared
coefficient of variation, over the nodes of depth k, of the number of their
descendants at the target depth D, summed over k and divided by N. Measured
exactly on the 30 committed instances with D = 17: the sum is 15 to 39
(median 23), single terms 0.04 to 4. The simulation gives relative variances
0.36 and 0.055 against the formula's 0.64 and 0.16 (the formula is
conservative because levels narrower than N are enumerated exactly).

At depth 160 the per-level term is exp(sigma_k^2) - 1, where sigma_k is the
sd of the log of the number of depth-160 descendants among depth-k nodes.
That is the same sigma as in path 5's section 6. If sigma stays below 1.5,
the sum over 160 levels is below 1,400 and N = 1e6 gives 4 percent. At
sigma = 2.45 (the value path 5 reads from the 10x10) over 100 levels the sum
is 4e4 and the sd is 20 percent, still fine on a log scale. At sigma = 3.5 it
is 2e7 and the estimate is noise below N = 1e9. So 1e6 particles suffice if
and only if prefix heterogeneity is below about sigma = 3, and the two
proposals of path 5 (ranking gain large, tree measurable with 1e6 particles)
cannot both come out at their favourable end. The run has to measure sigma
itself.

**Specification (not run; needs the owner's go-ahead).**

1. Order: rows 15, 14, ..., 0, columns 0 to 15, in the (row, col) of
   `engine/data/official_eternity2.csv` (fields 6 and 5). The five clues are
   on the board from the start with their CSV rotation. A node is a partial
   board with grey exactly on the rim and every joint between placed cells
   matched, clue joints included.
2. Level k (free cell k): enumerate every extension of the M particles; E_k
   is their number; r_k = E_k / M. Keep all if E_k is at most N, else a
   uniform sample of N without replacement. Width estimate: the product of
   the r values. Stop after free cell 175.
3. Gates before any run on the official set, same binary: (a) the 30
   committed 6x6 instances, N = 200, 200 seeds each, mean estimate within 3
   s.e. of the exact width at every depth; (b) on the official set with the
   order "the 60 frame cells in ring order" and no interior, the estimate
   must return 10^37.608 within 3 s.e. This is the one exact 16x16 target the
   repo holds.
4. Runs: official set, five seeds at N = 1e6 and five at N = 1e5. Null: 40
   instances painted from the official colour census (12 joints per frame
   colour; inner colours dealt over the 56 seam and 364 inner joints),
   rejected if any piece is duplicated or rotation-symmetric, five clues read
   off the painted board at the official cells; two seeds each at N = 1e6.
   The same with the centre clue only if time allows.
5. Record per level and per run: E_k, M, r_k, log10 of the width estimate,
   the fraction of particles with no extension, the mean and variance of
   extensions per particle; at free depths 128, 158, 175 the number of
   distinct ancestors at depths 16, 32, 64, 96 and the sum of squared family
   shares for each; the seed and the wall time.
6. Statistics: T_peak = log10 of the seed-mean width estimate at free cell
   158; T_sum = log10 of the sum over levels of the seed-mean estimates.
7. Estimator validity, all three required: the five seeds at N = 1e6 span at
   most 0.2 in log10 of the estimate at 158; T_sum at N = 1e6 and at N = 1e5
   differ by at most 0.2; the effective number of depth-64 ancestors (one
   over the sum of squared family shares) seen from depth 158 is at least
   1,000. If any fails, the result is "estimator not converged" and no width
   is reported. Agreement between seeds alone is not enough: every run can
   miss the same rare heavy prefixes, and the N comparison is the check for
   that.
8. Decision. "The designed set is atypical" if and only if the estimator is
   valid on the official set and on at least 36 of the 40 null instances, and
   the official T_sum lies outside the range of the null T_sum values
   (two-sided p = 0.049). Report the distance in null sd. Otherwise "typical
   at this resolution". Separately: the null median against 10^40.49 tests
   the model at 256 cells, and the null sd is the missing error bar of
   section B.
9. Cost: 90 runs of about 5e9 candidate tests, an hour or two of one core in
   compiled code.

Path 5's thresholds (within 0.5 orders: price stands; 3 orders off: atypical)
compare with the model only. Without the null sd a 3 order deviation could be
two sd or twenty.

## 4. Errors found

1. **`solve-path-2/02_bits_needed.py`, lines 69 to 71 and 106 to 110**
   (memo lines 147 to 157, 172 to 173, 191 to 194, 319 to 321, 369 to 371,
   and "about 93 more bits" in the verdict). The full-board match probability
   is divided evenly among joints and applied to partial boards. Fix: use the
   exact probability of the joints closed so far (`rv_lib.p2_table`,
   `p_frame_table`). Corrected numbers: 2^135.1 = 10^40.66 (five clues, its
   order), 2^156.9 = 10^47.25 (centre clue), 78.0 and 73.6 bits short, eight
   extra placements, gap 10^23.5 instead of 10^28. The docstring's "exact at
   depth 256" is true and is the reason the error went unseen: the solution
   count checks, the tree does not. This is the most serious error.
2. **`solve-path-3/model.py`, `profile()`, lines 132 to 152.** `placed`
   starts empty and a pinned clue is added only when the scan reaches its
   cell, so the joint between a clue and a neighbour placed earlier is closed
   up to 16 levels late. Fix: initialise `placed` with the fixed cells.
   Affected memo figures: line 113 (after rows 2, 8, 13: 20.4, 39.3, 30.1
   become 18.0, 38.1, 27.6), line 119 (10^40.66 at 135 and 10^41.38 become
   10^39.34 at 134 and 10^40.62), line 117 (10^47.83 becomes 10^47.21),
   lines 141 to 146 (best gain 0.83 becomes 1.00; r = 8 gain 0.08 becomes
   0.97), lines 19 and 25 (10^39.3 and 10^39.8 become 10^38.1 and 10^38.75;
   10^9.9 becomes 10^9.1), lines 219 and 223 (10^14.8 becomes 10^14.1 in its
   order; "25.7 of 40.5 digits" becomes 26.4), lines 239 to 244 (10^3.7
   becomes 10^4.4 in its order, 10^10.2 with the spiral).
3. **`solve-path-3-exponent.md`, lines 23 to 24 and 219.** The frame-given
   interior is priced with a row scan. Under a known frame an inward spiral
   costs 10^8.34. Fix: quote 10^8.3 and name the order.
4. **`solve-path-5-price.md`, lines 290 and 293.** The ranking factor is
   taken from a measurement on a different search and from one success whose
   baseline rate is not known. Fix: grade it "not established for the
   five-clue puzzle", carry the product as 0 to 2.6 orders and the gap as 17
   to 20.
5. **`solve-path-5-price.md`, lines 444 to 449.** The precision estimate
   uses the offspring-count variance. Fix: the descendant-count term of
   section F, and the validity gates.
6. **`solve-path-5/e2_price.py`, line 19** (memo line 110). The printed peak
   depth adds the number of clues to a free-cell index: "163" is board cell
   161, or 158 free cells.
7. **`solve-path-5-price.md`, line 149.** The 2008 sampling row is listed as
   a calibration with ratio 0.74 to 0.87. Fix: remove it from the calibration
   table or mark it as not comparable, as the site table already does.
8. **`solve-path-5-price.md`, lines 59 to 61 and 166 to 170, and
   `solve-path-3-exponent.md`, line 356.** Opposite size trends of the
   planted excess are asserted from boards whose colour counts change with
   size. Fix: state no trend.
9. **`solve-path-3-exponent.md`, table at lines 346 to 351.** The 5x5 row
   says 5 seeds; six were drawn and the one with the largest predicted tree
   was skipped by a cap on the model value (`small_check.py`, line 150).
   Selection on the predictor. Small effect; say so.
10. **`solve-path-3-exponent.md`, line 395.** "2 to 5 times the model"
    describes means; per seed it is 0.75 to 11.3 with one seed censored.

## 5. What the program may rely on tomorrow

1. Row scan with five clues costs 10^40.5 nodes in the first-moment model
   (10^40.2 to the first solution), reproduced by three independent
   implementations; use the range 10^39.5 to 10^41.5.
2. The model is exact for its ensemble at every size, runs 1.4 to 1.6 high on
   planted 6x6 boards for a reason that vanishes at 16x16, and matches the
   official pieces to 1 percent on 4 to 9 cells and to 0.1 digits on the
   frame.
3. Given the true frame the interior is cheap (10^8 to 10^14 nodes by
   order); given the true top and bottom pools each half is 10^9 to 10^10.
   The cost of the puzzle is frame and zone assignment.
4. Enumerating frames does not buy that: 10^45.9 at best, 5.5 digits above
   row scan. Band and quadrant joins gain at most 2.4 digits.
5. Hardware and tuning are worth under one order; the gap to 1e5 core-years
   is 17 to 20 orders; a design trace in the piece set carries nothing under
   an exchangeable painter.

## 6. What it must not rely on yet, and the cheapest measurement for each

| Not yet | Cheapest measurement |
|---|---|
| That the official set's width is within one order of the model beyond 9 cells | The population estimator of section F with its null of 40 painted instances; an hour or two of one core, needs the owner's go-ahead |
| That the spread between instances stays near 0.3 dex at 256 cells | The same null runs give it directly. Without the go-ahead: exact unplanted trees at 7x7 and 8x8 near the critical colour count with the compiled engine, 30 draws each |
| That ranking by predicted completions is worth 25 to 150 on this puzzle | Path 5's section 6 protocol on generated 16x16 boards (no real-set sampling); and read msgs 9688 and 9751 for the unranked rate and the search they measured |
| Path 2's bit budget (93 bits, ten placements, quadrant row) | Rerun `02_bits_needed.py` with exact probabilities; seconds. Corrected values are in section 2 |
| Path 3's top-down figures, seam table and 10^14.8 | One-line fix in `model.py`, rerun `bulge16.py`; seconds |
| That the spiral advantage under a known frame is real outside the model | Exact enumeration under the planted frame on 8x8 and 10x10 planted boards, spiral against rows; minutes in Python |
| That merging is absent on the official set | Count pairs of valid two-piece blocks with identical outer boundary in the official set; a piece-set statistic, seconds |
| The volunteer project's node rate as a strict-search rate | Ask the project what a node is, or run its CPU application on a board with a known tree |
| 1e8 strict nodes per core-second | Commit a result file for the JIT backtracker on a fixed hard board |
