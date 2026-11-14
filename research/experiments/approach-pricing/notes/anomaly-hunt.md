# Anomaly hunt: measured results that did not fit

> **Status (2026-11-14):** Desk study, not independently checked. Scripts and outputs:
> `anomaly-hunt/` next to this file. Compute: about 26 core-minutes, one core,
> `nice -n 10`, longest run 6.7 minutes. Nothing committed, nothing remote.

Grades: **measured** (a script here ran it on committed rows, or a new run here),
**derived** (arithmetic here on numbers measured elsewhere), **cited** (read in a
README, ASSESSMENT, memo or review, not recomputed), **guessed**. Painted and generated boards
are toys or null instances. Nothing here is Eternity II progress.

## 1. The reading

**No anomaly survives.** 51 candidates examined (appendix). 14 disagree with their model
or baseline by more than noise at face value. After a null model (would a painted board
show the same?) and a count of comparisons, none is a trace of structure in the real
tile set. Nothing is worth a measurement for the solve.

What this study added:

1. **The one real-against-generated difference on file is an estimator artefact
   (measured).** The gap 4 second analysis found a three-row staircase 2.85 bits cheaper than row
   scan on the official pieces, absent on a generated board, with no mechanism. At ten
   times the population the same estimate is 135.2 and 135.7 bits, above row scan
   (134.4). The "gain" was a heavy-tailed estimator read at too small a population.
2. **The first-moment model has an identified residual, and it is the frame's
   (left, right) table (measured).** The exact frame count is 0.106 digits below the
   model. Against 150 painted rings that is -1.0 sd, and the count of edge pieces with
   the same frame colour on both sides predicts it (r = -0.82). The same count predicts
   the whole tree on 8 painted boards (r = -0.79, n = 8). It accounts for most of why the
   official tree sits 0.9 sd below its painted ensemble, of the 0.125-digit spread of
   Track 10's painted boards, and of why D3's five Part B boards look too tight. Worth 0 bits: the frame
   count is already inside the measured 2^134.4.
3. **Four shrugged-off numbers are explained from committed rows (measured):** the arc
   consistency dip at depth 112, the clue worths above the painted range, the single
   flagged cell of D4's reverse null, the scan-direction asymmetry.
4. **The official level profile was compared level by level (measured).** 185 widths,
   179 branching ratios, 156 dead-end shares, 164 dispersion ratios against 20 painted
   boards: no |z| above 3. The official set is on the same side throughout (smaller
   tree, more dead ends), one fact at about 0.9 sd.

Two items stay unexplained. Both are properties of painted boards, not of the real set,
and neither is worth more than about 2 bits (section 3).

## 2. Survivors

None. For the record, the six that looked strongest and how each ended:

| # | Where | Observed | Model or baseline | Noise | Deviation | Comparisons | Explanation at the time | Tested? | End |
|---|---|---|---|---|---|---|---|---|---|
| A1 | `gap-4-review/real16_fc.jsonl`; review section 2 | staircase Y3, forward checking, official: 2^131.59 (13 runs, M = 12,000) | row scan 2^134.44 (5 runs); memo's cap 1.1 bits | run sd 0.64 and 0.33 bits | 2.85 bits, about 12 se nominal | 9 rules, 2 propagations | none ("I do not have a mechanism") | no | **killed here**: M = 120,000 gives 135.72, 135.22 (measured, `06_y3_m120000.jsonl`) |
| A29 | Track 10 gate (b), exact count | 10^37.608 frames | first moment 10^37.714 | exact; null sd 0.124 | -0.106 digits; z = -1.01 on 150 painted rings | 1 | not noticed | no | **explained here**: 14 same-colour edge pieces predict 37.607 (measured, `09_frame_null.out`) |
| A5 | `real-puzzle-profile` at `dc71ad56`, `paint-np1e5.jsonl` | painted mean 10^40.568 (median 40.61) | model 10^40.495 | sd 0.125, se 0.028 | +0.073 digits, 2.6 se | 1 | "planted excess or a census detail" (argued) | no | partly explained (frame table); official at -0.89 sd, 5 of 20 below (measured) |
| A16 | D4, `results/test-reading.txt`, ASSESSMENT section 3 | S6 on the planted prefix: +1.96 sd [1.52, 2.43] at depth 144, +2.15 at 160, 48 painted boards | 0 (the program's expectation); size bias (the reviewer's) | bootstrap over boards | 8 se from 0 | 30 (10 statistics, 3 depths); S9 is the same statistic (r = -0.93) | size bias of a prefix that has a completion | yes, in sign; size 1.5 to 1.8 times the prediction, untested | priced: 1.02 bits at 144 (cited) |
| A22 | D4, same file, "null in reverse" | S1 (candidates at the next cell), official, dir 0, depth 96: z = -3.2 | 48 generated boards | sd over boards | 3.2 sd | about 320 cells | chance among 320 | no | **not reproduced**: the same level against Track 10's 20 painted boards is z = -0.37 (measured, `07_branching.out`); 0.4 to 0.8 such cells expected by chance (derived) |
| A36 | `how-the-puzzle-was-made.md`, B18 | sum of the five clue pieces' numbers 1032 | 792.5 | sd 124 | +1.93 sd, p = 0.027 | 38 tails | chance, one tail of 38 expected | no | sharper readings here: smallest labels p = 0.013 (post hoc), sides in the five rare colours 4 of 20 (p = 0.29), sum of labels p = 0.11 (measured, `11_clue_colours.out`). The colour content of the clue pieces is ordinary |

## 3. The two unexplained items, priced

Neither is a survivor, and neither test is recommended.

**U1. The census tilt is 1.5 to 1.8 times what size bias predicts (A17).**
`wt-d4 results/size-bias.txt`: shift of S6 on the planted prefix over the population
variance is 1.22 at depth 32, 1.49 at 96, 1.70 at 128, 1.68 at 144, 1.78 at 176 (192 rows
per depth; cited). Pure size bias gives 1. At 144 the shift is 2.27 nats against 1.37
predicted, about 5 standard errors with a per-board sd near 1 on 48 boards (derived, not
recomputed from rows). The independent assessor called it "a quantity, not an anomaly". Nobody tested
why.

- If real: the number of completions of a prefix depends on its colour census more
  steeply than the configuration-model formula says, and more so with depth. A property
  of the planted ensemble (guessed: each tile ties four colours together, which S6
  ignores).
- Bits: the excess is already inside D4's measured value, 1.02 bits [0.72, 1.27] for one
  direction at depth 144 (ceiling 2.4). At 96 to 128 the reviewer's formula gives about 2
  bits (ceilings 17.1 and 6.8), with weights the independent assessor ruled degenerate. With
  rerandomised runs at correlation 0.6: 2.8 bits (cited, THESIS_PLAN section 3).
- Cheapest test: exact enumeration on 30 painted 6x6 or 7x7 boards, regress the log of
  the exact completion count of each prefix on its S6 at one level. Kill line: slope
  inside [0.8, 1.2]. About 0.5 core-hour.
- Odds that it is worth 10 bits on the real 16x16: under 0.1% (guessed). It cannot be
  checked on the real puzzle: the true prefix is unknown, and the official set's sampled
  prefixes look like the painted ones (11 statistics, about 320 cells, one flag, A22).

**U2. Part of the painted excess over the model (A5).** The frame-only null is +0.020
digits above the first-moment frame count (150 rings, se 0.010, measured), the full tree
+0.073 (20 boards). About 0.05 digits (0.17 bit) remain unassigned. No bits: a caveat on
"within 0.03 digits", which holds for the official set and not for the ensemble mean.
Test: Track 10's 20 painted seeds at N = 1e6 with the same-colour edge count as
covariate, 0.5 core-hour. Kill line: residual mean within 0.03 digits of zero.

## 4. What was explained here, with the numbers

**The staircase (A1, A2), measured.** `dyn.c` rebuilt unchanged. Official pieces,
forward checking, Y3:

| population | runs | log2 of branches tried |
|---|---|---|
| 3,000 (second analysis) | 5 | 131.35, range 129.70 to 132.05 |
| 12,000 (second analysis) | 13 | 131.59, range 130.70 to 132.92 |
| 12,000 (here, new seed) | 2 | 133.40, 131.63 |
| 120,000 (here) | 2 | 135.72, 135.22 |

Row scan with the same estimator: 134.44 at 12,000 (second analysis), 133.96 and 134.59 here.
The estimate rises with the population, so the low values were the estimator failing low
(the D3 independent assessor's F1 in a milder form: no early extinction, runs end at levels 183 to 190).
Two runs do not show convergence; the sound statement is "no gain shown". The null was
one generated board (+0.45); two more with five planted clues give an apparent gain of
+1.00 and -0.14 bits at M = 12,000 (3 runs each). Generated boards show the bias too.

**The frame table (A29, A5, A6, A51), measured.** `09_frame_count_null.py` reproduces
the exact official count (37.603 and 37.623 against 37.608). 150 painted rings: mean
37.733, sd 0.124; 18 of 150 below the official count. Slope -0.034 digits per same-colour
edge piece. On the 8 painted 16x16 boards of D3 the full tree has slope -0.045 per piece
(r = -0.79): boards with 6, 9, 9 such pieces have trees of 10^40.81, 40.54, 40.73, boards
with 11 to 14 have 10^40.41 to 40.52. The official set has 14 (painted mean 10.4, sd 2.9,
z = +1.22, the memo's B09). D3's Part B boards all have 11 to 14, which is why their five
trees span 0.10 digits where Track 10's twenty span 0.43 (permutation P = 0.006 for the
spread before the covariate). One null board rerun here at N = 1e6 gives 10^40.765
against 40.808 at N = 1e5, so the spread is the instances', not the estimator's.

**The dip at 112 (A21), measured.** `10_ac_dip.out`. Share of wrong prefixes that arc
consistency keeps, by scan direction (15 boards each, sd between boards 0.01 to 0.03):

| depth | dir 0 | dir 1 | dir 2 | dir 3 |
|---|---|---|---|---|
| 96 | 0.185 | 0.205 | 0.658 | 0.661 |
| 112 | 0.106 | 0.118 | 0.082 | 0.077 |
| 128 | 0.321 | 0.313 | 0.043 | 0.040 |
| 136 | 0.164 | 0.166 | 0.166 | 0.166 |

The pooled curve is the mean of two different curves that swap between direction pairs
and merge from 136 on. The directions differ only by where the centre clue lies relative
to the frontier (row 8 for two of them, row 7 for the other two). A clue-geometry effect
on painted boards, not noise and not tile structure (the cause is derived, not tested by
moving the clue).

**Clue worths (A13), measured.** D3's README has the official clue above the range of
five painted boards for 3 of 5 clues. The official worths were taken against the N = 1e5
baseline (10^40.412), the boards' against N = 1e6. Against the official N = 1e6 baseline
(10^40.453): z = +0.56, +0.37, -1.06, +0.94, +1.03.

**Scan directions (A12), measured.** Official minus model: -0.041 digits from the
bottom, -0.018 from the left, +0.083 from the right, +0.113 from the top (range 0.15).
The three null boards have ranges of 0.09, 0.17 and 0.24 across their four sides.

**Profile by level (A7 to A11), measured.** Official minus model: +0.05 digits at depth
40, 0 at 104, -0.04 at 158, -0.056 at 176. Against the 20 painted boards no depth has
|z| above 1.62; per 16-cell row the largest is -1.32. Branching per level: largest |z|
2.77 (level 61 to 62, the last cell of a row, a frame piece), 2 of 179 above 2. No depth
range departs from the null. The measured width never leaves the first-moment curve by
more than 0.07 digits (0.23 bit) before the estimator's extinction.

## 5. Dismissed, one line each

- A3, A4 searched orders "beat" row scan by 2.11 digits: estimator wall, tested by the independent assessor, same on null boards (cited).
- A14 model's greedy pins off by 11 digits: the optimiser selects the model's own errors; painted boards; wrong direction for a gain (cited).
- A18 to A20 D4 per-board spread, painter control, decoy: paired difference includes 0 in 15 of 18 cells; the decoy is a tie, verified by the independent assessor (cited).
- A23 D4 reverse null, S2 positive and S5 negative in most cells: board-level offsets of about 1 sd, repeated over correlated cells, 11 statistics (derived).
- A24 to A27 D1 far stratum: official median 75.12 bits, 16 of 37 censored; null boards 119.3, infinite, 56.8, 15 of 36 censored. Undecided on both, so no difference to explain. The corner pass reproduces on null boards (cited).
- A28 official frame count lowest of D1's four boards: same as A29.
- A30, A31, A34 engine and schedule effects: mechanisms tested by Track 15 and D2 (cited).
- A32, A33 extinction past depth 188; legal sample 2 decades under the model past depth 231: both act after depth 175, ceiling under 0.1 bit (derived).
- A35 no duplicate piece, p = 0.014: a design filter (cited).
- A37 rare colours are the first five labels, 1 in 6,188: labels sorted by count, post hoc (cited).
- A38, A39 the other 25 piece and tile-graph statistics, the seam: all inside (cited).
- A40 tensor components 9 against 4.8: P = 0.139 on 2,000 painted boards (measured).
- A41 to A43 ranks, cover invariant, "unexplained relations": inside the null or named by the second analysis (cited).
- A44 13 of 31 clue-pinned prefixes unfillable: clue-local; they die at the clue row, depths 16 to 48, at no cost (derived).
- A45 two of four shape descriptors outside the painted range: inside on the re-check's fresh seeds (cited).
- A46 to A49 parity identity, 2x2 blocks, the "8% advantage", near-twin cap: exact cause, z = -0.55, wrong null, orientation artefact (cited).
- A50 toy results: toys. A15: painted boards, not examined.

## 6. What I could not check

- **Convergence of A1.** Two runs at M = 120,000. Enough to kill the gain, not to price the rule.
- **The cause of the dip at 112** was not tested by moving the clue.
- **D1 raw rows.** The far-stratum figures are the independent assessor's. The five `Terminated` lines stay unexplained.
- **D4 raw rows.** U1's ratio and its standard error come from `size-bias.txt` and per-board summaries. In A22 I took S1 to be the first column of the reverse-null table, and the two nulls use different painters.
- **The true prefix.** Apart from five clue pieces nothing of the solution can be compared. Every real-against-null comparison here concerns the piece set or sampled wrong prefixes.
- **The null itself.** All painted nulls share one exchangeable painter. A designer who balanced colours by region would pass every test here (cited: `how-the-puzzle-was-made.md` section 4).
- **Not read in full:** LEDGER.md (grep, 4 hits, none relevant), EVIDENCE_AUDIT.md, the memos of gaps 1, 2, 6 and the relaxation arc (the log and a grep), tracks 0 to 9 and 11 to 15 beyond REPORT.md.

## Appendix: every candidate examined

Columns: where; observed against baseline; noise and deviation; comparisons in that
experiment; explanation at the time; tested (T) or only stated (S); end. "Here" means a
script in `anomaly-hunt/`.

| # | Where | Observed against baseline | Deviation | Comparisons | Explanation then | T/S | End |
|---|---|---|---|---|---|---|---|
| A1 | gap 4 review, `real16_fc.jsonl` | Y3 2^131.59 against row scan 2^134.44 | 2.85 bits, 12 se nominal | 18 | none | S | artefact, here (04, 05, 06) |
| A2 | same | Y2 mean 135.14, median 132.64, range 7 bits | skew | 18 | "heavy-tailed" | S | same estimator tail |
| A3 | D3 Part A, `a-official.jsonl` | s7-sa-beam 10^38.34 against 10^40.45 | 2.11 digits | 18 orders | estimator extinction | T (independent assessor) | artefact |
| A4 | same | s4-sa, s5-sa undecided | below row scan by 0 to 4 digits | 18 | same wall | T (independent assessor) | artefact |
| A5 | Track 10 `paint-np1e5.jsonl` | painted mean 40.568 against model 40.495 | 2.6 se | 1 | planted excess or census detail | S | partly the frame table, here (09, 13, 17) |
| A6 | same | official 40.457 against painted 40.568, sd 0.125 | -0.89 sd; pooled 28 boards -0.82 | 1 | "ordinary" | T | ordinary |
| A7 | `five-np1e6.jsonl` against painted | width at 185 depths | max z 1.62 (depth 15) | 185 | not done before | T here (01) | none |
| A8 | same | branching at 179 levels | max z 2.77, 2 above 2 | 179 | not done before | T here (07) | none |
| A9 | same against model | +0.05 at depth 40 to -0.056 at 176 | row blocks z up to 1.32 | 11 | "within 0.03 digits" (total) | T here (01) | inside null |
| A10 | `levels_E_M_zero_mean_var` | dead-end share above the null at most levels | mean z +0.77, 4 above 2, none above 3 | 156 | not done before | T here (15) | same fact as A6 |
| A11 | same | variance over mean of children | none above 2 | 164 | not done before | T here (15) | none |
| A12 | D3 Part A, 8 line scans | official minus model -0.04 to +0.13 by side | z -1.6 to -0.4 on 12 null sides | 4 | "wrong side costs 0.25 to 0.33" | T here (02) | ordinary |
| A13 | D3 README, stage bl | 3 of 5 official clue worths above the painted range | z up to 1.06 after baseline fix | 5 | "within 0.3 digits" | T here (03) | baseline mismatch |
| A14 | D3 b2 | greedy pins: model 10^6.65, measured 10^17.6 | 11 digits | 1 | clustered pins defeat the model | S | selection on model error; painted |
| A15 | D3 b1 | three interior cells 8.2 to 8.6 bits against 9.3 to 10.1 | about 1 bit | 56 cells | none | S | not examined |
| A16 | D4 test | S6 +1.96 and +2.15 sd at 144 and 160 | 8 se from 0 | 30 | size bias | T in sign | priced, 1.02 bits |
| A17 | D4 `size-bias.txt` | shift over variance 1.22 to 1.78 against 1 | about 5 se at 144 | 7 depths | "a quantity, not an anomaly" | S | unexplained, U1 |
| A18 | D4 design review | per-board spread; pilot board at 5 to 7.8 sd | heavy tail | 6 pilot boards | none | S | test: median 1.78, 10% below 0.7 |
| A19 | D4 painter control | fails 10 of 18 cells | paired difference includes 0 in 15 of 18 | 18 | rule ignores the planted mean's noise | T (study) | rule artefact |
| A20 | D4 decoy control | fails 1 of 30 | tie | 30 | tie artefact | T (independent assessor) | artefact |
| A21 | D4 `ac-followup.txt` | share kept 0.427, 0.096, 0.179 at 96, 112, 128 | far beyond board sd | 9 depths | "unexplained" | T here (10) | clue geometry by direction |
| A22 | D4 reverse null | S1, dir 0, depth 96, z -3.2 | 3.2 sd | 320 | chance | T here (07) | not reproduced |
| A23 | same | S2 mostly positive, S5 mostly negative | about 1 sd each | 11 statistics | none | S | board-level, 1 sd |
| A24 | D1 far stratum | official median 75.12 bits, 16 of 37 censored | null 119.3, inf, 56.8; 15 of 36 censored | 1 | undecided | T (independent assessor) | same on null |
| A25 | D1 stage 2 | pass on the median, 36.75 bits | corner frames | 1 | corner artefact | T (independent assessor, null boards) | artefact |
| A26 | D1 | refuter bimodal: under 1 s or never | n/a | 1 | solver behaviour | S | not structure |
| A27 | D1 `cal.log` | five Terminated lines | n/a | n/a | log is the whole stderr | S | process |
| A28 | D1 `tables.md` | official frames 37.603, nulls 37.71 to 38.01 | lowest of 4 | 1 | none | T here (09) | A29 |
| A29 | Track 10 gate (b) | exact 37.608 against model 37.714 | z -1.01 on 150 rings | 1 | none | T here (08, 09) | same-colour edge pieces |
| A30 | D2 | faithful defaults stop at depth 18 with five clues | n/a | 8 seeds | scan order reaches a clue neighbour late | T (independent assessor) | engine |
| A31 | Track 10 M1 | late share budget-dependent, ratio 2.89 | n/a | 6 seeds | zeros at 1e9 | T | engine |
| A32 | Track 10 M2 | extinction at 188 to 204, width 10^35 | tail unmeasured | 5 reps | tail share 10^-4 (argued) | S | after depth 175 |
| A33 | Track 8 independent assessor | legal sample 2 decades under the model past 231 | 2 digits | 1 | "open" | S | after depth 175 |
| A34 | Tracks 14, 15 | 0 of 2,040 runs reach depth 201 | n/a | 680 keyings | quota slack | T (Track 15) | engine |
| A35 | B01 | 0 duplicate pairs against 4.09 | p 0.014 | 38 tails | design filter | S | design |
| A36 | B18 | clue numbers sum 1032 against 792.5 | p 0.027 | 38 tails | chance | T here (11) | colours ordinary |
| A37 | E2 | rare colours are the first five labels | 1 in 6,188 | post hoc | labels sorted by count | S (second analysis) | labelling |
| A38 | B09, B13 | 14 same-colour edge pieces against 10.4 | z 1.22 | 38 tails | noise | S | see A29 |
| A39 | B02 to B17, B19; gap 5 review section 9 | 25 statistics | all |z| under 1.3 or share above 0.14 | 25 | n/a | T | none |
| A40 | gap 5 appendix A | 9 components against 4.8 | P 0.139 | 7 | "as in painted boards" | T here (14) | none |
| A41 | same | ranks 267, 256 against 251 to 275, 244 to 264 | inside | 7 | n/a | T | none |
| A42 | gap 5, covers | one extra invariant | on 30 of 30 painted | 1 | palette split | T (second analysis) | closed |
| A43 | gap 5 review section 3 | 4 and 8 relations unexplained in a first run | n/a | 4 | left rim column flux | T (second analysis) | named |
| A44 | gap 5 review section 7 | 13 of 31 clue-pinned prefixes unfillable | n/a | 31 | clue-local | T (second analysis) | 0 bits |
| A45 | gap 3 memo | 2 of 4 shape descriptors outside 11 painted boards | just outside | 4 | "edge of the range" | T (second analysis) | noise |
| A46 | gap 2 review | 19.0 bits between two rows | exact | 1 | parity identity | T | derived |
| A47 | far-ideas memo | 4,059,952 blocks against 4,066,387 | z -0.55 | 1 | ordinary | T | none |
| A48 | THESIS_PLAN O2 | "8% advantage" against free painting | n/a | 3 | flat counts are design | S | wrong null |
| A49 | the log 08:36 | near-twin cap 307 | n/a | 1 | stored orientation | T | artefact |
| A50 | the log, relaxation arc | 2 sd on 5x5; z 2.6 to 3.4 | toys | several | n/a | n/a | toys |
| A51 | D3 `b0.jsonl` against Track 10 | five boards span 0.10 digits, twenty span 0.43 | P 0.006 | post hoc | none | T here (13, 16, 17) | covariate: same-colour edge pieces |

Scripts (`anomaly-hunt/`): `01` profile against model and null; `02` scan
directions; `03` clue worths; `04` staircase rows; `05`, `06` staircase reruns (`dyn16`
is `gap-4-review/dyn.c` built with its own flags); `07` branching per level;
`08`, `09` frame count and its null; `10` dip by direction; `11` clue piece colours;
`12a` (board spec misparsed by the shell, no clue planted, kept and labelled) and `12b`
generated boards; `13`, `16`, `17` null samples and the covariate; `14` components;
`15` shape per level. `data/` holds the Track 10 rows extracted from `dc71ad56`.
