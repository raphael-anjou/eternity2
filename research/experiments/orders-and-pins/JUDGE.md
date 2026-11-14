# orders-and-pins: judgment

Branch `night/2026-11-13-d3-orders-and-pins`, track D3, label measurement.
Judged from the branch only (PREREG, amendments, raw rows, code), not from
the runner's conversation.

## Verdict: keep, with the Part A headline corrected

- **Part B (RQ5): keep.** The single-pin map, the leave-one-out clue worths,
  the Markov lower bounds and the whole-tree check reproduce from the raw rows
  and from reruns. The labels are honest. One sentence overstates a bound
  (below).
- **Part A (RQ6): the committed reading "s7-sa-beam beats row scan, kill line
  not reached" is wrong and must be withdrawn.** The s7-sa-beam number is the
  mass of the first 42 levels of its tree. The population dies at a
  deterministic wall, and the first-moment model, which tracks the measured
  widths up to that wall, puts 99.9% of the tree beyond it. Correct reading:
  no order is shown to beat row scan; row scan from the bottom is the cheapest
  of the validly measured orders; the three annealed orders s4-sa, s5-sa and
  s7-sa-beam are **undecided** (the estimator cannot see past free depth 38
  to 48 in them). By A4 (M3), the kill line is reached for 15 of 18 compared
  orders and undecided for 3. This is a reading of the existing data, not a
  new run.

## 1. Preregistration order and run log

- `PREREG.md` committed alone in `4d9937af` at 09:30:23 +01:00 (08:30:23
  UTC). Its SHA-256 `48dc0dbd...` equals the committed blob and is the hash in
  every run-log line, local and mac-m1.
- First run-log line (gate6) 08:31:05 UTC; first test stage (b0) 09:19:32
  UTC. Every result commit (`84b4295d` onward) follows the PREREG commit.
  Calibration and smoke rows (file times 08:50 and 09:28 local) predate the
  PREREG and are declared there as calibration on board 100149 and seeds 9001
  to 9005; no test board or test seed is among them.
- Every test stage has a start and end line with exit 0: locally gb, g8, b0,
  bl, b1, bs, b4, b3 (head commits b938c282, ba0b77b1, 91a9e458, 0 dirty
  paths); on mac-m1 a-official, a-null, b2 (copied-tree mode, no git, the
  committed PREREG hash). The local skip lines for the three remote stages
  are 0.1 s no-ops, as A7 says.
- Amendments: A1 to A4 precede every test stage (A4 at 09:01:43 UTC, gb
  started 09:01:50). A5 (09:20 UTC) was written after b0 started; A6 (09:31
  UTC) after b0 finished and during bl. Both only add labels or reporting
  lines and say so. A4 (M1) changed a gate's consequence after the G6 outcome
  was seen, and says so; it did not touch a test number.
- Coverage: every plan unit of every stage has exactly one row (gate6 and g8
  write one row per instance/depth by design), no extra units, every row's
  seed equals its plan seed except the 10 s4-sa GT rows declared in A1. No
  dropped seeds, no subsampling.

Track label check (1b): measurement, not optimization; no node-sequence
reference to hash against.

## 2. Recomputed numbers (own scripts, from the committed raw rows)

Every number below matches the README to the printed precision.

Part A, official set, five clues, N = 1e6, 5 repetitions, log10 of the mean
total, 95% t interval:

| order | estimate [95%] | sd log10 | SMC extinction depths |
|---|---|---|---|
| rows-b-lr | 40.453 [40.422, 40.481] | 0.023 | 191 to 198 |
| s7-sa-beam | 38.339 [38.325, 38.353] | 0.011 | 42 to 48 |
| s5-sa | 36.484 [36.309, 36.609] | 0.115 | 42 to 46 |
| s4-sa | 40.612 [-, 40.921] | 0.314 | 38 to 43 |
| rows-t-rl | 40.784 [40.774, 40.794] | 0.008 | 191 to 200 |
| spiral-in | 44.013 [-, 44.424] | 0.635 | 100 to 111 |
| spiral-out | 50.304 [50.289, 50.317] | 0.011 | 140 to 151 |

s7-sa-beam "gain" 2.114 [2.069, 2.156]. Null boards: s7 "gains" 2.52,
2.29, 2.52 and s5 4.42, 4.16, 4.21, with s7 extinct at depth 41 to 43 on
every repetition of every board.

Part B: b0 SMC 10^40.515, 40.412, 40.464, 40.480, 40.438 (PPD 40.426 to
40.499). Single pin (56 cells): best (10, 1) 10.05 bits (9.13 to 11.34),
median 5.79, worst (0, 0) 1.01 bits. b4: all 10 whole-tree enumerations
complete, 1 or 2 solutions each, exact 10^8.06 to 10^10.17, every SMC and
PPD mean within 0.0047 dex of exact. Spread pooled medians k = 8: 10^22.52,
k = 10: 10^23.79, k = 16: 10^12.79, k = 24: 10^11.28, k = 48: 10^8.47.
Greedy k = 12 SMC/PPD disagreement up to 12.56 dex. Transfer: official
five-clue 10^40.453 inside the b0 range.

## 3. Reruns (binary rebuilt from the committed source, seeds from the plans)

17 units, one core, `nice -n 10`; **17 of 17 identical** (total, extinction
depth, and for the enumeration solutions and nodes at the first):

| unit | total | extinction depth |
|---|---|---|
| a:s7-sa-beam:r0 (N = 1e6, mac-m1 row) | 2.131061e+38 | 43 |
| a:rows-b-lr:r0 (N = 1e6, mac-m1 row) | 3.047306e+40 | 191 |
| a:s5-sa:r1 (mac-m1 row) | 2.254664e+36 | 42 |
| a:s7-sa-beam:np1e5:r0 (mac-m1 row) | 2.227701e+38 | 41 |
| n:200115:s7-sa-beam:r0 (mac-m1 row) | 1.740414e+38 | 42 |
| b0:300061:k0:ppd:r0 | 5.397132e+40 | |
| bl:official:drop135:r0 | 2.661518e+43 | 192 |
| b1:300051:0,0:smc:r0 | 1.574064e+40 | 191 |
| b1:300097:10,1:smc:r2 | 1.664986e+37 | 190 |
| b1:300131:11,1:ppd:r0 | 1.111449e+37 | |
| bs:300051:k16:smc:r0 | 1.2135e+13 | 92 |
| bs:300051:k16:ppd:r0 | 7.121333e+12 | |
| bs:300117:k12:ppd:r1 | 1.986462e+16 | |
| b3:300061:k16-s1:smc:r3 | 2.210001e+13 | 87 |
| b2:300061:k12:smc:r0 (mac-m1 row) | 2.27899e+18 | 180 |
| b2:300061:k12:ppd:r0 (mac-m1 row) | 3.188988e+17 | |
| b4:300061:k48 (exact) | 131,134,387; 1 solution; 122,738,378 at first | |

The rows collected from mac-m1 reproduce on this Mac, so they are the
committed plans' rows, not something else.

## 4. Findings

### F1 (reshapes Part A). The s7-sa-beam "beat" is a truncation artefact

Model profile (`scripts/model.py`, the runner's own) against the mean
measured widths, per free depth, official set:

| order | model total | measured | wall (min extinction) | model mass to wall | model mass past wall | max abs(model - measured) before wall |
|---|---|---|---|---|---|---|
| rows-b-lr | 40.495 | 40.453 | 191 | 40.495 | 34.05 (0.0%) | 0.09 |
| s7-sa-beam | 41.138 | 38.339 | 42 | 38.213 | 41.137 (99.9%) | 0.16 |
| s5-sa | 40.896 | 36.484 | 42 | 36.345 | 40.896 (100%) | 0.09 |
| s4-sa | 44.508 | 40.612 | 38 | 40.301 | 44.508 (100%) | 0.08 |
| spiral-in | 46.051 | 44.013 | 100 | 43.223 | 46.051 (99.9%) | 0.14 |

On every order where SMC reaches the end, model and measurement agree within
0.1 to 0.2 dex per depth (0.46 for spiral-out). For s7-sa-beam they agree
within 0.16 dex at every depth up to the wall, and the measured total
(38.34) equals the model's mass up to the wall (38.21), not the model's
total (41.14). The wall is structural: the annealed orders place scattered
unconnected cells (s7 starts (14, 4), (0, 1), (1, 2), (0, 15), (15, 1)), so
an unconstrained cell (680 children for every particle, zero variance) is
followed by a four-sided cell (0.6% survive), then several three-sided
cells. The width falls 4 dex in three steps (model 10^38.2 at depth 40,
10^34.2 at 43; measured 10^38.3 and 10^34.4), so a population of 1e6 thins
to thousands, then tens of particles (seed 700081: 6,095, 347, 44, 0) and dies there on every seed and board, while the
true tree (by the model that matches it up to that point) re-expands to a
peak of 10^39.9 at depth 118. Stability across seeds and between N = 1e5
and 1e6, which is what A4 (M1) tests, cannot detect a wall every seed hits.
The README's own diagnostic paragraph says the same and calls the data
undecided; the table and the bolded kill-line sentence contradict it. The
model is not a measurement (PREREG), so the sound statement is "undecided",
and the argued statement is "almost surely above row scan" (model 41.14
against 40.50, with the model validated on the depths the estimator sees).

Fix: in `scripts/analyze.py` `part_a`, an order is not "measured" if its SMC
repetitions go extinct before the last free depth minus some margin while the
row-scan baseline does not (for example, minimum extinction depth below 150 of
251); such an order falls to the Markov reading, which makes s7-sa-beam
undecided. In the README: s7-sa-beam status "bounded below (LB 37.04),
undecided"; the kill line "reached for 15 of 18 compared orders, undecided
for s4-sa, s5-sa, s7-sa-beam"; drop "beats". Record the change as an
amendment written after the outcome was seen.

### F2 (Part A model reading). The model's failure is the s7 artefact

The README says the model fails the "can replace the estimator" rule
(Spearman 0.66 over 14 measured orders). Recomputed: 0.667 with s7-sa-beam,
**0.949 without it** (13 orders), and conditions 2 and 3 then hold as well
(every model-near-best order is a line scan measured within 0.33 dex of row
scan; nothing the model puts above row scan by 1 dex is measured below it).
After F1 the reading flips to "the model can replace the estimator for
static line-scan-like orders"; it should not be read as validated for
orders whose estimator is wall-limited, where the model is the only number.
This is a judge's reanalysis, not a prereg reading; report it as such.

### F3 (Part B prose). One bound overstated

"10 spread pins are not enough for any of the four budgets on any board
(Markov bound, sound)" and "9 to 11 spread tiles leave the tree above every
budget by the Markov bound (k = 10: lower bounds 10^22.0 to 10^23.0)" are
false for 2^78 = 10^23.48: every k = 10 lower bound (10^21.98 to 10^23.02)
is below it. The README's own table (largest k shown over 2^78: 6) is right.
Fix: "not enough for 2^61.5, 10^16 or 10^15 on any board; for 2^78 the
bound shows only k <= 6".

### F4 (Part B scope, already labelled). The 10^11 to 10^23 range is unvalidated

Spread k = 12 to 48 SMC repetitions all die early (extinction depth 20 to
109). At k = 40 and 48 the exact enumeration shows that is the tree really
ending; between 10^12 and 10^23 nothing exact checks it, and the two
estimators can fail low together. The README says so ("under, checked
end-to-end only", "not shown: any validated under at full size"). Keep that
wording; do not cite "16 to 24 spread pins bring the tree under 2^61.5"
anywhere without it. A judge's extra enumeration at k = 24 (smallest board)
was started and stopped when the core budget was cut to one; it is not
evidence either way.

### Checks that came back clean

- Planted-solution leakage: Part A uses no planted information. Part B
  pins are planted pieces by design (the quantity asked); PPD uses the
  planted path by design and is gated (g8 pass, GB greedy k = 16 pass, b4
  pass). The census painter keeps the official colour counts, and the
  transfer check (official five-clue tree and each clue's worth inside the
  painted-board ranges) passes on recomputation.
- Node accounting is the same tree definition in every arm (pre-placed
  cells are not nodes); full tree, value order irrelevant except for the
  nodes-at-first-solution column, which the README labels as one fixed
  value order.
- No random-value-order control is needed: the quantity is the full tree of
  a static cell order, which does not depend on value order.
- No wall-clock claim is made; nothing is a speed result.
- Baseline is not weak: row scan is Track 10's order and is remeasured with
  its own seeds; reproduction of Track 10's 10^40.457 within 0.004 dex.
- No site "explored" or "dead end" page is relied on.

## 5. What the result supports

- Row scan from the bottom (and its three model-equivalent line scans and
  boustrophedon) is the cheapest measured static order on the real pieces
  with five clues, 10^40.45; scanning from the wrong side costs 0.25 to 0.33
  dex (0.8 to 1.1 bits). Measured.
- The three annealed orders are not measured; the model puts them 0.4 to 4.0
  dex above row scan. Argued.
- One planted tile is worth up to 10 bits (rows 8 to 14 interior), 5.8 bits
  typically, 1 bit at the top corners; spread pins are sub-additive; ten
  spread pins leave the full tree above 10^18.5 on every board. Measured
  (Markov bounds sound).
- 16 to 24 spread pins bring the full tree to about 10^11 to 10^13. Estimated
  by two agreeing estimators, checked exactly only at 40 and 48 pins.

## Unplanned observations

- The engine's SMC dies at a deterministic wall whenever an order places an
  unconstrained cell just before a closed cell; any future order search that
  scatters cells needs an estimator that can cross a valley (more particles
  at the valley, or a planted or stratified path) before its numbers are
  read.
- The first-moment model matches measured per-depth widths to 0.1 to 0.2
  dex on every order where the estimator reaches the end, including orders 4
  to 10 dex above row scan; it is a better order-ranking tool than the README
  credits.

Scripts used by this judgment (scratchpad, not committed): independent
recompute, wall diagnostic (model against measured per depth), seeded rerun.
