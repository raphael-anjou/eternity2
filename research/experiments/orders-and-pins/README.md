# orders-and-pins

Arc 2026-11-13, track D3, label **measurement**. Two questions about the
price of the exact search tree on Eternity II-sized boards: is row scan the
cheapest static fill order on the real pieces (RQ6, Part A), and what is one
more known tile worth, and where (RQ5, Part B)? Not a record attempt; no
board is produced. Results on generated boards are not Eternity II progress.

**Status: complete.** Part A (official set, null boards) and b2 ran on
m1 (A7, driver ended 12:41, every step exit 0) and were collected at
19:16; the rest ran on this Mac.

Read in this order: `PREREG.md` (committed alone, `4d9937af`),
`PREREG_AMENDMENTS.md` (A1 to A8, each with time and reason; A4 applies the
design review M1 to M12), then this file. Every number below is printed by
`python3 -I scripts/analyze.py` (written to `results/summary.json`) from the
raw files named.

## The quantity (A4, M9)

Every tree size here is the **full tree**: every valid partial board, the
cost of exhausting, for row scan from the bottom (`rows-b-lr`: rows 15 to 0,
columns 0 to 15) with clues and pins on the board from the start and used
only when the scan reaches them. Node = a valid partial board after a
placement on a free cell. The expected cost to the first solution with one
solution and a random value order is about half of it (one bit less), about
T / (S + 1) with S solutions. A solver free to choose its order after seeing
the pins needs no more.

## Method

- Estimator: Track 10's population SMC (crate imported from
  `night/2026-11-12-t10-real-puzzle-profile`, commit `dc71ad56`, judged
  keep), unchanged in its SMC, child function and resampling. Additions in
  `engine/src/oap.rs`: any order file, fixed-cell list and board file; a
  depth-truncated enumerator; and **PPD** (planted-path decomposition): every
  node is on the planted path or leaves it first at one step j, each
  departure term estimated by an independent SMC started from the siblings of
  the planted child. Both estimators are unbiased; both can fail low.
- Model: the first-moment (bulge) model, `scripts/model.py`, a port of the
  refereed `solve-path-5/bulge.py`. It is a comparison column only.
- Boards: census-painted 16x16 boards (Track 10's judged painter: the
  official colour counts exactly, frame split kept), piece ids shuffled, five
  clues from the planted solution at the official cells. Part B boards:
  painter seeds 300051, 300061, 300097, 300117, 300131.
- Part B pin policies (A4, M10): **spread** (k pins at evenly spaced free
  depths 16 to 160 of the row scan, moved to the next middle cell not
  adjacent to a clue or pin; fixed before any test board was measured, but
  its depths were chosen after exploratory runs on calibration board 100149);
  **random** (uniform non-clue cells, two sets per board and k: "typical");
  **greedy in the unplanted model** (b2, a demonstration that the model's
  placement fails, never a best case).
- Reading of a configuration with both estimators (A4, M4 and M5): reported
  value = arithmetic mean of the SMC and PPD means (pooled); Markov 95% lower
  bound = log10(pooled) - 1.30. **Shown over** a budget iff the Markov bound
  is above it. **Shown under** only if validated (no early SMC extinction,
  sd of log10 totals at most 0.30 per estimator, means within 0.50 dex, the
  policy passed gate GB, pooled + 0.30 under). A secondary label **under,
  checked end-to-end only** (A5, defined after GB was seen) drops only the GB
  condition, for spread, because the whole-tree check b4 passed.

## Gates

| Gate | What | Result | Raw |
|---|---|---|---|
| g8 (A4, M7) | 30 hard-ladder 8x8 test instances x 5 pin sets, exact vs SMC (N = 100) and PPD (N = 10), 200 reps | SMC: 150 of 150 pairs within 3 se or 0.02 dex, median abs log10 error 0.0052, **pass**; PPD: 148 of 150 (98.7%), median 0.0082, **pass**. Tests code and unbiasedness, not the bottleneck regime | `results/gate/g8.jsonl` |
| GB (A4, M6) | calibration boards 100149, 100244: truncated exact trees under pins vs SMC (N = 1e5) and PPD (N = 1e4), 10 reps | greedy k = 12 at depth 72: SMC **fails** (+0.68 dex on 100149; every rep extinct before depth 72 on 100244), PPD **fails** (-0.197 on 100149, -0.005 on 100244). Greedy k = 16 at depth 56: both pass on both boards (errors at most 0.002 dex). Spread k = 16: **not testable**, no depth of 56, 72 or 88 completes under 3e9 nodes (A5). So GB passes for no policy | `results/gate/gb.jsonl` |
| b4 end-to-end (A4, M8) | spread k = 40 and 48, whole tree enumerated on all 5 boards (cap 2e10) | all 10 enumerations complete, every board passes, largest estimator miss 0.0047 dex (SMC and PPD both within 0.005 dex of exact): **pass at k = 40 and k = 48** | `results/b/b4.jsonl` |
| G6, GT (Part A) | see PREREG A3 and A1 to A4 | G6-total passes for all 15 named orders; G6-profile fails for spiral-out, antidiag, bands2, snake-cols-l; GT passes for all 19 after A2 | `results/gate/gate6.jsonl`, `gt.jsonl` |

End-to-end detail (`results/b/b4.jsonl`): exact trees 10^8.06 to 10^10.17;
1 or 2 complete boards each; nodes at the first complete board over the full
tree, in the enumerator's fixed value order: 0.09, 0.13, 0.94, 0.94, 0.93,
0.93, 0.40, 0.42, 0.35, 0.33 (300051 k40, k48; 300061; 300097; 300117;
300131). With one solution the mean over value orders is one half; the
spread here (0.09 to 0.94) is what one fixed order gives.

## Part B results (painted boards; full tree)

**Base (b0, five clues, SMC N = 1e6):** 10^40.515, 10^40.412, 10^40.464,
10^40.480, 10^40.438 (span 0.10 dex; PPD 10^40.43 to 10^40.50).

**One pin (b1, 56 cells, SMC N = 1e5, 4 reps; bits = log2(T0/T1), mean over
5 boards):**

- best cell (10, 1): **10.05 bits** (range 9.13 to 11.34 over boards);
- typical (median over the 56 cells): **5.79 bits**;
- worst (0, 0), a top corner the scan reaches last: **1.01 bits**.
- The map is the model's: interior cells in rows 8 to 14 are worth 9.3
  to 10.1 bits (three of them 8.2 to 8.6; model 9.58 = log2(784)), cells in rows 0 to 3 about 1.1 to
  1.5, bottom frame cells about 5.8. Spearman model against measured over
  the 56 cells 0.95, mean difference -0.07 bits. PPD cross-check on four
  cells: largest |PPD - SMC| 0.14 dex. No SMC repetition extinct early.
- The five clues themselves (bl, leave one out): the centre clue and the two
  bottom clues are worth 2.85 to 2.99 dex each (9.5 to 9.9 bits) on every
  board, the two top clues 0.39 to 0.47 dex (1.3 to 1.6 bits).

**Additivity:** spread k = 8: the measured gain is 0.88 [0.85, 0.92] of the
sum of its eight single-pin gains (5 boards): **sub-additive**. Greedy k = 4
(b2): 0.92 [0.87, 0.96], **sub-additive**.

**Greedy in the unplanted model (b2, a demonstration, not a best case):**
k = 4: pooled 10^29.3 to 10^30.4 (model 10^28.98); k = 12: pooled 10^17.1 to
10^19.2, median 10^17.6, against the model's 10^6.65. At k = 12 plain SMC
returns 10^6.9 and 10^7.4 on two boards (every repetition loses the few
surviving prefixes) while PPD gives 10^18.7 to 10^19.5 there: the two
estimators disagree by up to 12.6 dex. The model's placement is wrong by
about 11 dex at k = 12, and plain SMC with it.

**Several pins, pooled value, median over boards (bits gained over the
base):**

| k | spread | random (10 pairs) | random, model median |
|---|---|---|---|
| 1 | 10^37.61 (9.6) | 10^38.81 (5.4) | 10^38.75 |
| 2 | 10^37.10 (11.2) | 10^36.92 (11.8) | 10^37.19 |
| 4 | 10^32.09 (27.8) | 10^33.24 (24.1) | 10^33.89 |
| 6 | 10^28.15 (40.9) | | |
| 8 | 10^22.52 (59.5) | 10^28.75 (39.0) | 10^28.32 |
| 10 | 10^23.79 (55.4) | | |
| 12 | 10^14.95 (84.9), estimators up to 1.37 dex apart | | |
| 16 | 10^12.79 (92.0) | 10^19.24 (70.4) | 10^19.76 |
| 24 | 10^11.28 (97.0) | 10^14.56 (86.1) | 10^13.69 |
| 32 | 10^10.40 (99.9) | 10^11.01 (97.8) | 10^10.94 |
| 40 | 10^9.51 (102.9) | | |
| 48 | 10^8.47 (106.3) | 10^7.99 (107.8) | 10^7.43 |

Spread is not monotone in k (k = 10 above k = 8) because each k places its
pins at different depths. Per-configuration values, both estimators, sds and
early-extinction counts are in `results/summary.json`
(`part_b.policies.*.by_k.*.per_config`).

### Pins to a budget (full tree; first solution about half)

No configuration is **shown under** any budget: GB validates no policy (A5).
Brackets, per board for spread, and over the (board, set) pairs for random:

| budget | spread: largest k shown over | spread: smallest k under, checked end-to-end only | spread: shown under | random: largest k with half shown over | random: shown under | greedy (model): largest k shown over |
|---|---|---|---|---|---|---|
| 2^78 (10^23.5) | 6 on every board | 8, 16, 16, 24, 24 (boards 300097, 300051, 300117, 300061, 300131) | not shown within the grid | 8 | not shown within the grid | 4 (k = 12: neither) |
| 2^61.5 (10^18.5, 1,000 core-years) | 10 on every board | 16, 16, 16, 24, 24 | not shown within the grid | 16 | not shown within the grid | 4 (k = 12: neither) |
| 10^16 (about one year at 17 to 30 cores) | 10 on every board | 16, 16, 16, 24, 24 | not shown within the grid | 16 | not shown within the grid | 12 on 3 of 5 boards |
| 10^15 (about one month at 17 to 30 cores) | 10 on every board | 16, 16, 16, 24, 24 | not shown within the grid | 16 | not shown within the grid | 12 on every board |

So, on these boards and for this full-tree quantity: **10 spread pins are not
enough for 2^61.5, 10^16 or 10^15 on any board (Markov bound, sound; for 2^78
the bound shows only k <= 6, since the k = 10 lower bounds, 10^21.98 to
10^23.02, are below 10^23.48), 16 to
24 spread pins bring the tree under all four on every board by the pooled
estimate checked only against whole-tree enumeration at k = 40 and 48 and by
the agreement of the two estimators (not by GB).** For random pins: 16 are
not enough for 10^15, 10^16 or 2^61.5 on at least half the pairs, 8 not
enough for 2^78; nothing random is shown under. For the two owner budgets
(A6): spread bracket (10, 16 to 24], random bracket (16, not shown].

The survey figure "about 6 bits per tile, 9 to 11 tiles" (community
digests): the typical single tile here is 5.8 bits, the best 10; 9 to 11
spread tiles leave the tree above 2^61.5, 10^16 and 10^15 by the Markov bound
(k = 10: lower bounds 10^21.98 to 10^23.02), but not above 2^78 (10^23.48).

### What is and is not shown

- Shown: single-pin values to about 0.2 bits per cell, and the full map
  agrees with the model; clustered pins defeat both the model and plain SMC
  (calibration, GB); spread pins are sub-additive; Markov lower bounds
  (sound for any unbiased non-negative estimator) for every configuration.
- Not shown: any validated "under" at full size. Between 10^12 and 10^23 the
  "end-to-end only" values rest on the two estimators agreeing and on the
  whole-tree check at 10^8 to 10^10, not on an exact check in that range.
- Transfer to the real pieces (A4, M11): both checks pass. (i) The official
  five-clue tree, 10^40.453 (a-official, N = 1e6), lies inside the b0 range
  10^40.41 to 10^40.52. (ii) Each official clue's worth (bl, against the
  official N = 1e5 ladder value) lies within 0.3 dex of the same clue on the
  five boards: centre 2.963 (boards 2.848 to 2.995), (2, 2) 0.469 (0.396 to
  0.436), (2, 13) 0.412 (0.385 to 0.472), (13, 2) 2.996 (2.819 to 2.955),
  (13, 13) 2.973 (2.804 to 2.923). So the painted-board figures may be given
  as the estimate for the real pieces, with the caveats above; only the
  clues, never extra pins, are measured on the real set.
- Spread is the best policy tried, not an optimum.

## Part A (official set, five clues, N = 1e6, 5 repetitions)

Raw: `results/a/a-official.jsonl`, null `results/a/a-null.jsonl` (both
collected from m1). Status by A4 (M1) and A8: measured iff G6-total and GT
pass, the sd of the five log10 totals is at most 0.10, the N = 1e5 ladder is
within 0.10 dex, and no repetition dies before free depth 150 of 251 (the judge's threshold); else
bounded below by the Markov bound (estimate - 1.30). A8 was added after the
outcome was seen, on the judge's ruling (JUDGE.md, F1).

| order | model | estimate [95%] | sd | ladder minus N=1e6 | status | reading |
|---|---|---|---|---|---|---|
| s5-sa | 40.896 | 36.484 [36.309, 36.609] | 0.115 | -0.150 | bounded below (LB 35.18) | undecided |
| s7-sa-beam | 41.138 | 38.339 [38.325, 38.353] | 0.011 | +0.001 | bounded below (LB 37.04) | undecided |
| rows-b-lr | 40.495 | 40.453 [40.422, 40.481] | 0.023 | -0.040 | measured | baseline |
| snake-rows-b | 40.495 | 40.454 [40.423, 40.482] | 0.023 | -0.011 | measured | does not beat |
| rows-b-rl | 40.495 | 40.455 [40.426, 40.483] | 0.023 | +0.025 | measured | does not beat |
| cols-l-bt | 40.495 | 40.469 [40.448, 40.488] | 0.016 | -0.031 | measured | does not beat |
| snake-cols-l | 40.495 | 40.478 [40.461, 40.494] | 0.013 | 0.000 | measured | does not beat |
| cols-l-tb | 40.495 | 40.483 [40.437, 40.523] | 0.035 | -0.010 | measured | does not beat |
| s4-sa | 44.508 | 40.612 [-, 40.920] | 0.314 | -0.287 | bounded below (LB 39.31) | undecided |
| cols-r-bt | 40.622 | 40.700 [40.667, 40.731] | 0.025 | +0.053 | measured | does not beat |
| rows-t-lr | 40.622 | 40.718 [40.698, 40.737] | 0.016 | 0.000 | measured | does not beat |
| cols-r-tb | 40.654 | 40.743 [40.710, 40.774] | 0.026 | +0.031 | measured | does not beat |
| rows-t-rl | 40.654 | 40.784 [40.774, 40.794] | 0.008 | +0.014 | measured | does not beat |
| bands2 | 41.088 | 41.044 [40.990, 41.092] | 0.040 | +0.014 | measured | does not beat |
| s6-beam | 41.153 | 41.181 [41.153, 41.207] | 0.022 | -0.043 | measured | does not beat |
| spiral-in | 46.051 | 44.013 [-, 44.424] | 0.635 | -0.472 | bounded below (LB 42.71) | does not beat |
| antidiag | 45.109 | 44.933 [44.736, 45.068] | 0.127 | -0.172 | bounded below (LB 43.63) | does not beat |
| spiral-out | 50.388 | 50.304 [50.289, 50.317] | 0.011 | 0.000 | bounded below (LB 49.00) | does not beat |
| frame-then-rows | 50.749 | 50.631 [50.450, 50.759] | 0.127 | +0.049 | bounded below (LB 49.33) | does not beat |

- Baseline: rows-b-lr 10^40.453 [40.422, 40.481]; Track 10's 10^40.457 is
  0.004 dex away, so the reproduction passes.
- **Kill line by the A4 (M3) reading: reached for 15 of 18 orders,
  undecided for s4-sa, s5-sa, s7-sa-beam.** No order is shown to beat row
  scan; row scan from the bottom is the cheapest of the validly measured
  orders. The s7-sa-beam figure (10^38.34) is the mass of its first 42 levels,
  not its tree.
- **Mechanism (the judge's finding).** The annealed orders place scattered
  cells, so a cell with no placed neighbour (680 children per particle) comes
  just before a fully closed cell (0.6% survive). The population of 1e6 thins
  to tens of particles within three levels and dies at a depth fixed by the
  order, on every seed and board, while the first-moment model puts 99.9% of
  the tree beyond that wall.
- **Read this with the following diagnostic, which the A4 status rule does
  not contain.** In every N = 1e6 repetition of the three searched orders
  s4-sa, s5-sa and s7-sa-beam, the population dies at free depth 38 to 48
  while the estimated width there is still 10^34 to 10^39 (row scan dies at
  191 to 198, with widths of 10^33 to 10^35 after a peak of 10^39 at depth
  158). The s7 estimate is the mass of its first 40 or so levels; the tree
  past the step where no sampled particle can continue is not seen. That is
  the skew failure the design review describes (an unbiased estimator that
  fails low), and stability between N = 1e5 and 1e6 does not detect it when
  every repetition hits the same wall. The null boards show the same pattern
  (s5-sa "gains" 4.2 to 4.4 dex and s7-sa-beam 2.3 to 2.5 on all three). The
  sound statement is the Markov bound: s7-sa-beam's tree is above 10^37.04
  (95%), which is below row scan's, so these data do not decide whether it
  is cheaper than row scan. GT, the only full-size exact check of these orders, stops
  at free depth 5.
- Eight line scans (all measured): the best is rows-b-lr; the worst,
  rows-t-rl, is 1.10 bits above it (the "wrong side" cost). rows-t-rl's
  official gain (-0.33 dex) lies outside its null range (-0.13 to -0.08,
  widened by 0.2) and is flagged as specific to the official set; every other
  order's gain is ordinary.
- Model against measurement (12 measured orders after A8, which also
  demotes spiral-out): Spearman 0.934; conditions 1 to 3 hold. Before A8, with s7-sa-beam counted as measured (14
  orders), the preregistered reading was Spearman 0.66 and the rule failed.
  The judge's reanalysis (not a preregistered reading, over 13 orders, spiral-out
  kept) is that the failure was the s7-sa-beam truncation: 0.949 without it, so the model can stand in for
  the estimator on static line-scan-like orders, and is not validated for
  orders whose estimator hits a wall, where it is the only number. Over all
  19 point estimates Spearman is 0.70. The model is 2.0 dex too high on
  spiral-in.
- Null boards: the best order is s5-sa on all three; Spearman with the
  official ranking 0.92, 0.91, 0.86.

## Collection from m1 (A7), done at 19:16

The commands that were run:

```
cd research/experiments/orders-and-pins
mkdir -p results/remote-m1
rsync -a -e "ssh -F $SSH_CONFIG -o BatchMode=yes" \
  m1:work/d3/research/experiments/orders-and-pins/results/ results/remote-m1/
rsync -a -e "ssh -F $SSH_CONFIG -o BatchMode=yes" \
  m1:work/d3/research/experiments/orders-and-pins/PROGRESS.txt results/remote-m1/
cp results/remote-m1/a/a-official.jsonl results/remote-m1/a/a-null.jsonl results/a/
cp results/remote-m1/b/b2.jsonl results/b/
python3 -I scripts/analyze.py
```

The copies in `results/a/` and `results/b/` are byte-identical to the
collected files (checked with `cmp`); the duplicates under
`results/remote-m1/a` and `b` were then removed. `results/remote-m1/`
keeps the remote guard log (`run-log.jsonl`, copied-tree mode, PREREG SHA-256
48dc0dbd...), `PROGRESS.txt` and the driver logs.

## Reproduce

```
just experiments orders-and-pins build
just experiments orders-and-pins plans    # model, orders, plans (deterministic)
just experiments orders-and-pins all      # gb g8 b0 bl b1 bs b4 b3 b2 a-official a-null
python3 -I research/experiments/orders-and-pins/scripts/analyze.py
```

Remove `REMOTE_STAGES` to run every stage locally. Each stage is resumable
and goes through `../common/prereg_guard.py`, which logs to
`results/run-log.jsonl`. SMC and PPD rows are deterministic given seed and
binary (checked identical between this Mac and m1).

## Compute

This Mac (10 cores, at most 2 processes, `nice -n 10`). Stage wall times
from the run log: gate6 199 s, gt 558 s, gb 1,041 s, g8 20 s, b0 1,414 s, bl
510 s, b1 4,100 s, bs 973 s, b4 596 s, b3 1,990 s: about 11,400 s of wall
clock at 2 processes, about 6.3 core-hours, plus about 0.8 core-hours of
calibration and checks. On m1 (4 processes, from PROGRESS.txt):
a-official 35.7 min, a-null 5.8 min, b2 3.2 min, about 3.0 core-hours.
Total about 10.1 core-hours (cap 12). Wall clock: 08:45 to 13:00, collection
and analysis 19:16 onward.

## Layout

| Path | What |
|---|---|
| `engine/` | Track 10's crate plus `oap.rs` (binary `rpp`) |
| `scripts/` | `model.py`, `orders.py`, `pins_model.py`, `plan.py`, `drive.py`, `analyze.py`, `calib_summary.py` |
| `orders/`, `pins/`, `plans/`, `boards/` | inputs, all generated before the test stages |
| `results/` | raw rows per stage, `run-log.jsonl`, `summary.json` |
