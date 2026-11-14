# real-puzzle-profile: results

Night 2026-11-12, track 10, label measurement. Official piece set, 16x16.
Nothing here is a record attempt or Eternity II progress. Grades: **measured**
(computed here from the committed raw files named), **reported** (published
elsewhere, not rerun), **argued** (reasoning only). All numbers below are
printed by `python3 -I scripts/analyze.py` (also in `results/summary.json`)
unless a raw file is named. Times in `PREREG_AMENDMENTS.md` headings are
approximate; the commit times and `results/run-log.jsonl` govern.

Machine: this Mac (10 cores), runs limited to 4 single-threaded processes at
a time, `nice`. Wall clock of the whole experiment: about 21:47 to 00:20; about 6.5 core-hours, of which 4.9 in the two 1e11 runs and the two killed gate201 runs.

## Corrections made after the judgment

Three wording fixes required by `JUDGE.md` (reviewed at `06cde25`); no number
or result file changed.

1. The M1 headline now states the budget-dependence flag (ratio 2.89 against a
   limit of 2), that the share is measured at 1e10 and 1e11 nodes only, that
   the reading at a record run's budget is argued, and the scope limits
   (`record` schedule only, this port only, `gate201` unfinished, `gate221`
   never reaching its first unlock).
2. The sentence saying the no-clue tree had no model figure was false: the
   price memo gives 10^50.03, and the measurement is +0.009 dex from it.
3. Gate (a) is 10 of 1,110 pairs (30 instances x 37 levels including the
   root), not 1,080.

## Headline

**M1, late-node share.** In the port of the repo's Blackwood-style engine
(record unlock list from depth 201, quota on, five clues), the share of nodes
at or after the first unlock is **4.5e-7 in the median over 6 seeds at 1e10
nodes per run (range 4.0e-7 to 2.0e-6)**, and 5.4e-7 and 1.4e-6 for two seeds
run to 1e11 (measured). The share is measured at 1e10 and 1e11 nodes only. By
the preregistered rule it is **flagged budget-dependent** (median at 1e10 over
median at 1e9 is 2.89, against a limit of 2), so "below the 5% line at a record
run's budget" is **argued, not measured**: the values are five to six orders of
magnitude below the 5% line of the lead's decision rule, but the 1e15 regime
was not reached. M1 covers only the `record` unlock schedule and this port of
`blackwood_bt` (no Blackwood colours, quota scan or restarts). `gate201` never
finished and `gate221` never reached its first unlock, so no statement is made
for those schedules. The DFS node histogram peaks at depth 161, the
depth of the widest level of the exact tree, and each node at depth 201 has
on average 35 to 71 nodes under it (measured).

**M2, five clues, row scan from row 15: 10^40.457 nodes to exhaustion, 95%
interval 10^40.440 to 10^40.473 (N = 1e6, 5 repetitions)** (measured). The
model price is 10^40.49 (reported, `price-and-ledger.REVIEW.md`): the
difference is -0.03 dex, inside every error bar. Centre clue only: 10^47.140
(model 10^47.14). The withdrawn 10^45.3 is 4.8 orders above the measurement.

## M2. Tree size by SMC

### Gates (A1 item 4), passed before any M2 number was read

| Gate | Statistic | Value | Rule | Verdict | Raw file |
|---|---|---|---|---|---|
| (a) 30 hard-ladder 6x6, rows 5..0, N = 200, 200 reps | (instance, depth) pairs > 3 s.e. from exact | 10 of 1,110 (0.90%; 30 instances x 37 levels including the root) | <= 2% | pass | `results/gate/gate-a.jsonl` |
| (a) | median abs log10(mean total / exact) | 0.0050 (max 0.023) | <= 0.05 | pass | same |
| (b) official frame, ring order, N = 1e5, 20 reps | mean complete frames / exact 40519056449429419697492635191544381440 | 0.99994 (z = -0.01) | within 3 s.e. | pass | `results/gate/gate-b.jsonl` |
| secondary: 40 generated boards (5x5, 6x6, with clue or frame fixed), N = 1000 | median abs log10 error; max; coverage of 95% t interval | 0.0023; 0.032; 95% | <= 0.05; <= 0.3; >= 80% | pass | `results/gate/gen-np1000.jsonl` |
| same, N = 100 | same | 0.0097; 0.054; 92.5% | not gated | | `results/gate/gen-np100.jsonl` |

Exact tree sizes in gate (a): 1.6e5 to 1.1e6; in the generated set: 120 to
1.15e9 (all enumerated completely). Gate (b) is the only exact 16x16 count in
the repo and the estimator lands on it to 6e-5 relative.

### Official set (measured)

Node = valid partial board after a placement on a free cell, grey exactly on
the rim, clues on the board from the start, order CSV rows 15 to 0, columns 0
to 15 (A1). Repetitions are combined by the mean of the estimates, then the
log.

| Config | N | reps | log10 mean T | 95% t interval (log10) | span of reps (dex) | widest free depth | log10 peak width | extinction depths | s / rep | Raw file |
|---|---|---|---|---|---|---|---|---|---|---|
| five clues | 1e6 | 5 | 40.457 | 40.440 to 40.473 | 0.035 | 158 | 39.16 | 191, 192, 192, 192, 192 | 32 | `results/m2/five-np1e6.jsonl` |
| five clues | 1e5 | 5 | 40.497 | 40.414 to 40.567 | 0.160 | 158 | 39.20 | 188 to 192 | 4.1 | `results/m2/five-np1e5.jsonl` |
| centre clue only | 1e5 | 5 | 47.140 | 47.061 to 47.208 | 0.143 | 160 | 45.87 | 195 to 204 | 4.5 | `results/m2/centre-np1e5.jsonl` |
| no clue | 1e5 | 5 | 50.039 | 50.017 to 50.059 | 0.039 | 161 | 48.76 | 196 to 200 | 4.8 | `results/m2/none-np1e5.jsonl` |

Against the model (reported: 10^40.49 five clues, 10^47.14 centre clue):
-0.033 dex (N = 1e6) and +0.007 dex (N = 1e5) for five clues; +0.0004 dex for
the centre clue. The A1 departure rule (|difference| > 1.0 dex) is not met:
**consistent with the model at this precision**. The no-clue tree has a model
figure: `research/program/night/2026-11-12/memos/solve-path-5-price.md` gives
10^50.03 (peak 10^48.76 at free depth 161), and the measurement, 10^50.039 with
the same peak, is +0.009 dex from it (reported model, measured value). The
no-clue tree carries the 4-fold corner symmetry (no clue fixes the
orientation).

The widest level is free depth 158, the review's "161 (158 free cells)". The
four optional clues are worth 10^6.68 (centre-only over five-clue totals).

**Extinction.** Every repetition's population dies at free depth 188 to 204
while the estimated width there is still about 10^35 (five clues, mean
log10 width 35.5 at free depth 190, 33.9 at 192): the population has collapsed
onto a few hundred families (diversity table below) whose descendants all die.
Widths past the extinction depth are therefore not measured. Their sum is
argued to be negligible: the mean width falls by about 0.2 to 0.3 dex per cell
from depth 165 on (38.76 at 165, 37.19 at 180, 35.53 at 190), so the
remaining 60 levels add at most about 10^36.5, a share of 10^-4 of the total
(argued). The sum to free cell 175 is 10^40.453 (N = 1e6), so 99% of the
estimate sits before 175 (measured).

**Weight diversity** (A1 item 6, N = 1e6, mean of 5 reps): at free depth 175
the 1e6 particles descend from 1,052 distinct depth-16 ancestors (1,504 of
depth 96), sum of squared family shares 0.0034 (an effective 300 families). At
depth 128: 2,732 distinct depth-16 ancestors, 0.0014. The fraction of particles
with no extension is 0.59 at depth 32, 0.13 at 64, 0.27 at 128, 0.0002 at 158
and 0.48 at 175. Per-level rows are in each JSON line
(`levels_E_M_zero_mean_var`, `diversity_depth_anc_distinct_sumsq`).

### Null: instance-to-instance spread (A1 item 7, measured)

20 instances painted from the official colour census (frame joints from the
official frame-colour multiset, seam and inner joints a uniform deal of the
official inner-colour multiset; 2,634 of 2,654 draws, seeds 1 to 2,654,
rejected for a duplicate or rotation-symmetric piece, an acceptance rate of
0.75%, against 0.8% in the review), five clues read off the painted
board, N = 1e5, 2 repetitions each (`results/m2/paint-np1e5.jsonl`).

| | log10 T |
|---|---|
| null median | 40.61 |
| null 2.5 / 97.5 percentile (min / max of 20) | 40.33 / 40.76 |
| null sd (includes about 0.05 dex of estimator noise) | 0.125 |
| official set (N = 1e6) | 40.457, 5 of 20 null instances below it |

The official set is an ordinary member of its census ensemble on this
statistic (measured). The painted instances sit +0.12 dex above the model in
the median (see unplanned observations).

### Frame stand-ins (A1 item 9, A2 item 4)

`scripts/frames.py` decodes both preregistered boards: every cell maps to a
distinct CSV piece, neither honours the five clues, each has 3 broken
frame-to-interior joints. These frames are stand-ins from 460-class boards,
not the true frame.

| Frame | Clues | Order | Result | Raw file |
|---|---|---|---|---|
| v129-palimpsest-463 | five | spiral | dies at free depth 1 (1 node) | `results/m2/frame-v129-palimpsest-463-enum.jsonl` |
| v181-keyring-460 | five | spiral | dies at free depth 27 (481,683 nodes) | `results/m2/frame-v181-keyring-460-enum.jsonl` |
| v129-palimpsest-463 | none (exploratory, A2) | spiral | dies at free depth 1 (1 node) | `results/m2/frame-noclue-v129-palimpsest-463-enum.jsonl` |
| v181-keyring-460 | none (exploratory, A2) | spiral | more than 1e10 nodes, cap hit at 333 s, deepest free depth 140 of 196, widest level seen 1.4e9 at depth 95 | `results/m2/frame-noclue-v181-keyring-460-enum.jsonl` |

Uninformative as a check of the review's 10^8.34 (measured that they die or
run off; argued that this is expected): that figure is a mean over valid
frames with five clues, and a frame that carries breaks of its own toward the
interior, or does not fit the clues, is not a member of that set. With no
clue, one stand-in frame's interior tree exceeds 1e10 nodes under the spiral
(a lower bound, measured), which is a different configuration from the model
figure (no clue removes the four clues the spiral passes in its first rings).

## M1. Late-node share of a Blackwood-style DFS

Engine: `engine/src/bw.rs`, a port of `blackwood_bt.rs` (PREREG.md M1). The
equivalence check passed: at seed 1, cap 1e8, the port and `blackwood_bt`
report the same node total (100,000,000) and the same deepest depth (209)
(`results/gate/equiv-blackwood_bt.txt`, `results/gate/equiv-port.jsonl`); the
rebuilt binary gives the identical per-depth histogram
(`results/gate/equiv2-port.jsonl`). Node = a call of the search on a non-clue
cell at depth index d (0..255), counted before the quota test.

Lacks against the record engines (stated in PREREG.md): Blackwood's own colour
labels and quota calibration (the repo's {22,21,20} stands in), border
interspersing and scan corner, tuned restarts, his speed. Quirk kept: in the
no-break path a piece may show grey on an interior side.

### Per run (measured, `results/m1/<schedule>-s<seed>.jsonl`)

Share of nodes at depth >= the first unlock of the schedule, at the
checkpoints and at the cap ("final"; 1e10, or 1e11 for `record-1e11`); then,
at the cap, the share at depth >= 201, in the last 56 cells (d >= 200) and in
the last 40 (d >= 216); deepest depth reached and breaks used there; wall
seconds (four processes at a time, `nice`).

| schedule | seed | 1e7 | 1e8 | 1e9 | 1e10 checkpoint | final | d >= 201 | last 56 | last 40 | deepest | breaks | s |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| gate221 | 1 | 0.0e+00 | 0.0e+00 | 0.0e+00 |  | 0.0e+00 | 2.6e-08 | 6.4e-08 | 0.0e+00 | 205 | 0 | 187 |
| gate221 | 2 | 0.0e+00 | 0.0e+00 | 0.0e+00 |  | 0.0e+00 | 1.5e-08 | 3.8e-08 | 0.0e+00 | 205 | 0 | 191 |
| gate221 | 3 | 0.0e+00 | 0.0e+00 | 0.0e+00 |  | 0.0e+00 | 5.3e-08 | 1.2e-07 | 0.0e+00 | 205 | 0 | 215 |
| gate221 | 4 | 0.0e+00 | 0.0e+00 | 0.0e+00 |  | 0.0e+00 | 1.9e-08 | 4.3e-08 | 0.0e+00 | 205 | 0 | 211 |
| gate221 | 5 | 0.0e+00 | 0.0e+00 | 0.0e+00 |  | 0.0e+00 | 1.5e-08 | 3.6e-08 | 0.0e+00 | 205 | 0 | 208 |
| gate221 | 6 | 0.0e+00 | 0.0e+00 | 0.0e+00 |  | 0.0e+00 | 1.6e-08 | 4.3e-08 | 0.0e+00 | 204 | 0 | 218 |
| record | 1 | 0.0e+00 | 6.6e-06 | 1.2e-06 |  | 7.1e-07 | 7.1e-07 | 7.5e-07 | 0.0e+00 | 209 | 2 | 165 |
| record | 2 | 0.0e+00 | 0.0e+00 | 1.8e-07 |  | 4.1e-07 | 4.1e-07 | 4.3e-07 | 0.0e+00 | 209 | 2 | 168 |
| record | 3 | 0.0e+00 | 0.0e+00 | 2.3e-06 |  | 2.0e-06 | 2.0e-06 | 2.1e-06 | 1.1e-07 | 220 | 4 | 225 |
| record | 4 | 0.0e+00 | 0.0e+00 | 0.0e+00 |  | 4.8e-07 | 4.8e-07 | 5.1e-07 | 0.0e+00 | 207 | 2 | 220 |
| record | 5 | 0.0e+00 | 0.0e+00 | 7.6e-08 |  | 4.0e-07 | 4.0e-07 | 4.2e-07 | 0.0e+00 | 209 | 2 | 196 |
| record | 6 | 0.0e+00 | 0.0e+00 | 1.3e-07 |  | 4.1e-07 | 4.1e-07 | 4.4e-07 | 0.0e+00 | 209 | 2 | 204 |
| record-1e11 | 1 | 0.0e+00 | 6.6e-06 | 1.2e-06 | 7.1e-07 | 1.4e-06 | 1.4e-06 | 1.4e-06 | 6.8e-08 | 220 | 4 | 2786 |
| record-1e11 | 2 | 0.0e+00 | 0.0e+00 | 1.8e-07 | 4.1e-07 | 5.4e-07 | 5.4e-07 | 5.6e-07 | 4.2e-09 | 218 | 4 | 2792 |
| record-noquota | 1 | 0.0e+00 | 0.0e+00 | 9.4e-08 |  | 8.1e-08 | 8.1e-08 | 8.5e-08 | 0.0e+00 | 205 | 1 | 254 |
| record-noquota | 2 | 0.0e+00 | 0.0e+00 | 0.0e+00 |  | 2.2e-07 | 2.2e-07 | 2.3e-07 | 0.0e+00 | 209 | 2 | 251 |
| record-noquota | 3 | 0.0e+00 | 0.0e+00 | 9.2e-08 |  | 1.3e-07 | 1.3e-07 | 1.3e-07 | 0.0e+00 | 205 | 1 | 236 |
| record-noquota | 4 | 0.0e+00 | 0.0e+00 | 0.0e+00 |  | 1.9e-07 | 1.9e-07 | 1.9e-07 | 0.0e+00 | 209 | 2 | 243 |
| record-noquota | 5 | 0.0e+00 | 0.0e+00 | 3.5e-07 |  | 8.6e-08 | 8.6e-08 | 9.0e-08 | 0.0e+00 | 205 | 1 | 258 |
| record-noquota | 6 | 0.0e+00 | 0.0e+00 | 1.3e-06 |  | 1.6e-07 | 1.6e-07 | 1.6e-07 | 0.0e+00 | 205 | 1 | 257 |

### Per schedule: median [min, max] over seeds (measured)

| Schedule | seeds | share >= first unlock, at cap | share d >= 201 | last 56 | last 40 | deepest | nodes after 201 per node at 201 | histogram peak depth |
|---|---|---|---|---|---|---|---|---|
| gate221 | 6 | 0.0e+00 [0.0e+00, 0.0e+00] | 1.7e-08 [1.5e-08, 5.3e-08] | 4.3e-08 [3.6e-08, 1.2e-07] | 0.0e+00 [0.0e+00, 0.0e+00] | 205 [204, 205] | 2 [1, 2] | 161 [161, 161] |
| record | 6 | 4.5e-07 [4.0e-07, 2.0e-06] | 4.5e-07 [4.0e-07, 2.0e-06] | 4.7e-07 [4.2e-07, 2.1e-06] | 0.0e+00 [0.0e+00, 1.1e-07] | 209 [207, 220] | 46 [35, 71] | 161 [161, 161] |
| record-1e11 | 2 | 9.5e-07 [5.4e-07, 1.4e-06] | 9.5e-07 [5.4e-07, 1.4e-06] | 9.9e-07 [5.6e-07, 1.4e-06] | 3.6e-08 [4.2e-09, 6.8e-08] | 219 [218, 220] | 57 [53, 60] | 161 [161, 161] |
| record-noquota | 6 | 1.4e-07 [8.1e-08, 2.2e-07] | 1.4e-07 [8.1e-08, 2.2e-07] | 1.5e-07 [8.5e-08, 2.3e-07] | 0.0e+00 [0.0e+00, 0.0e+00] | 205 [205, 209] | 60 [43, 98] | 161 [134, 161] |

No run completed a board (`completions` = 0 in every row). Best partial:
depth 220 with 4 breaks (`record` seed 3; `record-1e11` seed 1).

### Reading against the preregistered rules

- **Primary statistic** (`record`, 1e10, median over 6 seeds): **4.48e-7**,
  range 4.0e-7 to 2.0e-6. Decision line: above 50% pursue, below 5% do not.
  Measured value: below 5% by a factor of about 10^5 (measured).
- **Budget dependence** (PREREG.md: uninformative if the median at 1e10
  differs from the median at 1e9 by more than a factor 2): the ratio is 2.9,
  so by the preregistered rule the `record` result is **flagged
  budget-dependent**. The 1e9 medians include zeros (two seeds had no node past
  201 by 1e9), which is what drives the ratio. The added 1e11 runs (A6) move
  the two seeds from 7.1e-7 and 4.1e-7 at their 1e10 checkpoint to 1.4e-6 and
  5.4e-7 at 1e11, factors 1.9 and 1.3 per decade (measured). Extrapolated at
  that rate (argued, not measured), four more decades to the 1e15 of a
  reported 469 run give at most about 1.9^4 x 1.4e-6, about 2e-5. To reach 50%
  the share would have to grow by a factor of about 10^5.5 over those four
  decades.
- `gate221`: no run reaches depth 221 in 1e10 nodes (deepest 204 or 205), so
  its share after the first unlock is 0 in every run: **uninformative** by the
  PREREG.md rule ("no run reaches the first unlock"), measured.
- `record-noquota`: 1.4e-7 (8.1e-8 to 2.3e-7), lower than with the quota.
- `gate201` (all 10 breaks from depth 201): **not finished**. Seeds 1 and 2
  ran 1 h 41 min of CPU each without reaching 1e10 nodes and were killed at
  00:15 (A7; start lines without end lines in `results/run-log.jsonl`, run ids
  668b8fd7eb33 and 9aebf95e2d3c; no row). A `record` run reaches 1e10 in about
  3 minutes, so these runs spend most of their time in the break path, which
  costs about 1,000 orientation tests per node. That they are slow suggests
  many nodes after depth 201 under this schedule, but no share was recorded
  (argued, unmeasured).

### Why the share is small (measured, with the argued step marked)

The DFS histogram peaks at depth 161 in every `record` run, the depth of the
widest level of the exact tree (M2), and 80% of the nodes sit between depths
146 and 173 (record seed 1). Nodes at depth 201 are about 1e-8 of all nodes;
under each one the engine visits 35 to 71 nodes (median 46) before the subtree
is exhausted. Late nodes are rare because few branches survive to depth 201
with zero breaks, not because the break region is small per branch. For the
late share to pass 50%, the subtree below one depth-201 node would have to
be about 10^8 times larger than measured (argued from the two measured
factors).

This addresses the review's question (`partial-match-regime.REVIEW.md`,
section E and "most serious error") for this engine: its model needs 92 to 98%
of nodes after the first unlock to match 10^15 nodes for a 469 under its quota
assumption; this engine, at 1e10 to 1e11 nodes, puts about 1e-6 there. Whether
Blackwood's engine (his colours, his quota, his tuning) behaves the same is
not measured here.

## M1b. Whole-tree late share by SMC: not measured

A2 added an SMC estimate of the whole break-tolerant tree. Its gate failed by
its own rule (A5: 5 qualifying prefixes, not 10, and those subtrees were small
enough that the SMC never resampled, so the zero error is vacuous), and the
estimator degenerates on this tree (A6): with the quota on, every repetition
dies by depth 36 to 48 (raw: `results/m1/m1b-record.jsonl`,
`m1b-gate221.jsonl`), because uniform resampling does not find the rare
quota-satisfying branches the privileged-first DFS finds; with the quota off
it dies at depth 194, before the first unlock (`m1b-record-noquota.jsonl`).
No M1b number is reported as a measurement.

## Unplanned observations

- The painted null instances (official census, one planted solution each)
  have a median tree of 10^40.61, 0.12 dex above the model's 10^40.49, and 15
  of 20 sit above the official set's 10^40.457 (measured). Small, inside the
  review's argued 0.5 dex, but one-sided: either a planted excess or a census
  detail (the painter deals inner colours uniformly over seam and inner joints,
  the model pools them) the model does not carry (argued).
- The SMC population collapses at free depth 188 to 204 while the true width
  is still about 10^35; the five-clue total is unaffected (99% of it lies
  before depth 175) but any quantity in the last 60 cells, including M1b, is
  out of reach of uniform resampling at N = 1e6 (measured).
- `gate201` runs are about 30 times slower per node than `record` runs
  (unfinished after 101 CPU minutes against 3 for 1e10 `record` nodes): the one
  schedule where the late region may dominate is the one this experiment could
  not finish (argued from wall time).
- The no-clue tree with the stand-in keyring frame exceeds 1e10 nodes under the
  spiral, far above the model's 10^8.34 for a frame with five clues; the
  configurations differ, so this is not a contradiction (measured lower bound,
  argued reading).

## What was not done

- M1b (whole-tree late share by SMC): not measured (gate failed, estimator
  degenerate; A5, A6).
- `gate201` seeds 3 to 6 and `record-1e11` seeds 3 and 4: cut for wall time
  (A7). The two `gate201` seeds that started did not finish (killed at 00:15).
- `gate221`: ran, uninformative (never reaches its gate at 1e10).
- The review's full specification was not run in full: N = 1e6 only for the
  five-clue configuration, no sampling of the stop at free cell 175 as a
  separate run (the sum to 175 is reported from the full runs), the null with
  2 repetitions at N = 1e5 rather than N = 1e6, no centre-clue null.
- No Blackwood-faithful engine: M1 measures the repo's port, not his code.
- The interior cost given the true frame: no true frame exists; the stand-ins
  are uninformative.

## Commands

```
cd research/experiments/real-puzzle-profile
./run.sh build
BW_BIN=<blackwood_bt> ./run.sh equiv     # and equiv2 (rebuilt binary)
./run.sh gate-gen; ./run.sh gate-a; ./run.sh gate-b
JOBS=2 ./run.sh m1
./run.sh m2; ./run.sh paint; ./run.sh frames; ./run.sh frames-noclue
./run.sh m1b-val; ./run.sh m1b
SEEDS="1 2" ./run.sh m1-1e11; SEEDS="3 4" ./run.sh m1-1e11
python3 -I scripts/analyze.py
```
