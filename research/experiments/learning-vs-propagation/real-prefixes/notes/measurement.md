# Learning against propagation below real-piece prefixes, depths 112 to 88

2026-11-14. Status: independently re-checked (`recheck.md`); the corrections below apply. Official pieces, sampled wrong prefixes refuted exhaustively; not Eternity II progress. Every number below is in `results/analysis/analyze.out.txt` (full-run analysis) or `results/analysis/recheck.recompute.out.txt` (the re-check's own parser).

## Corrections from the independent re-check

The re-check (`recheck.md`, script `scripts/recheck/recompute.py`) found the counts sound and found these statements stronger than the data or incomplete. Read the sections below with them.

1. The plan predates the data by file time only, and after the first pilot. Only the pre-stated stop line (written before any real-prefix run) has an external record.
2. "All arms agree on the number of completions" is true and vacuous: every one of the 800 runs reports 0. It tests nothing about the encoding on the real pieces. What does support the encoding is that summed conflicts of clasp without learning sit within 0.04 bits of arc consistency's failed branches at depths 104, 96 and 88.
3. The mean subtree sizes 2^12.4 and 2^17.1 (depths 112, 104) and 2^18.3, 2^24.2 (96, 88) were quoted as the check of the depth definition. The first pair belongs to a scan in the opposite row order. For this scan a population estimate gives 2^8.5, 2^13.3, 2^18.6, 2^24.6; the sample gives 2^5.7, 2^11.1, 2^18.3, 2^24.5. The definition is confirmed at 96 and 88 (within 0.3 bits). At 112 and 104 the sample is 2.7 and 2.2 bits below the mean: 40 prefixes miss the rare prefixes that carry the cost.
4. The largest omission. The arm with the fewest steps is clasp with learning off and a static order: summed at depth 88, 4.19 million conflicts, against 14.8 million branches for arc consistency, at least 17.6 million conflicts for CaDiCaL and at least 25.8 million for clasp default. A lead of a solver arm over arc consistency's "branches tried" is therefore partly a change of unit (about 1.8 bits), and learning subtracts from it on every prefix with work.
5. "Prefixes where either solver did over 100 units: K over AC about +0.1" compares conflicts with branches tried. Against failed branches it is about -1.7.
6. The bound "d88_16: at most +1.08" for CaDiCaL over arc consistency comes from a run cut on the slowest machine (CaDiCaL at 1,374 conflicts a second there, against 2,628 and 3,148 on the two Apple machines; not "about 1,300"). It is weak for that reason only.
7. The "median log2 T" at depths 104 and 96 (6.2, 13.6) is the upper middle value of 40; the midpoint gives 5.5 and 13.2.
8. Clasp default counts are not fixed across builds: on a rerun one of five differed (`d88_31`: 50,222 against 52,914). The other four arms reproduced exactly (tree 160 of 160, arc consistency 157 of 157, clasp without learning 5 of 5, CaDiCaL 5 of 5).
9. Arc consistency against the plain tree in seconds (-5.46 bits in sums at 88 and -4.46 at 96; per machine at 88 -5.27, -5.91, -5.12) was absent from the seconds table. It is in `results/analysis/analyze.out.txt`, section "sums".

Tables below carry the corrected figures where one applies.

## Design

Part (b) of the pre-stated test (`../README.md`).

- Official pieces, five clues, row scan from the top left. A prefix at depth d has the first d non-clue cells placed consistently.
- 40 prefixes at each of depths 112, 104, 96, 88 (seed 1000*d + i), the same for every arm.
- Arms, exhaustive, one thread: T plain row scan (nodes); AC `dyn.c AC ROW` (branches tried; failed = tried - alive); K CaDiCaL 1.5.3 via python-sat (conflicts); C clasp 3.3.10 default (conflicts); U0 clasp, learning off, static order (conflicts). K, C, U0 read one CNF per prefix (pairwise direct encoding, 14 to 25 million clauses).
- 1,200 s wall inside every arm; counts printed on every exit. The five arms of a prefix ran in turn on one core, order shuffled per prefix.
- Work = count + 1; lead of X over Y = log2(work Y / work X); a cut run gives a bound.
- Check: all arms agree on completion counts on 9 pinned toy boards (`scripts/validate.py`). On the real pieces this check is vacuous (Correction 2).

## Deviations

1. Depth definition. The first pilot used a reversed definition (251 - d free cells placed) and was discarded before the full run: trees of 0 to 51 nodes, every solver at 0 or 1. Those data are in `../wrong-depth-pilot/` and used nowhere. The definition was then fixed to d placed; see Correction 3 for how well the sample matches the population.
2. Sampler. A single random descent dies before the target level, and randomised depth-first search thrashes under the first clue. Each prefix is one uniform draw from the last level of its own population descent (20,000 nodes a level): near uniform over the level, weights not computed.
3. Rows are scanned 0 to 15 (the profile it was compared with scans 15 to 0).
4. Machines. K needs 6.5 GB at depth 88, so: machine-c (Apple laptop) 3 workers, machine-a (8-core Apple M1, 16 GB) 2, machine-b (14-core low-power x86 laptop, two core types, pinned) 4. At 10:13:52 the 26 prefixes machine-a had not started (`scripts/queues/moved.from_machine-a.txt`) moved, 12 to machine-c and 14 to machine-b; the assignment is not the one fixed at launch. Final: 72, 10, 78 prefixes (machine-c, machine-a, machine-b). None ran twice.
5. K's limit counts from process start (search cut near 1,170 to 1,185 s); its conflict budget per call is 5,000.
6. No cross-machine count check on real prefixes in the full run; the corrected pilot's 12 prefixes reran on the same machine, 60 of 60 identical. The re-check reran 4 CaDiCaL prefixes that first ran elsewhere, all identical.

## Runs

800 runs, none without counts, none killed, no duplicate; 9.15 core-hours, 09:33 to 10:53. No prefix has a completion; all arms agree (vacuously). Every run finished except, at depth 88, K cut on 3 prefixes and C on 10.

## How many prefixes carry work

| Depth | AC, K, C, U0 all 0 | all at most 1 | AC and K both 0 | median log2 T | log2 summed T |
|---|---|---|---|---|---|
| 112 | 27 | 38 | 36 | 3.3 | 11.1 |
| 104 | 25 | 37 | 37 | 5.4 (upper middle value 6.2) | 16.4 |
| 96 | 11 | 18 | 18 | 13.2 (upper middle value 13.6) | 23.7 |
| 88 | 11 | 16 | 15 | 22.7 | 29.8 |

At 112 and 104 any median lead between solvers is set by prefixes refuted at the root. At 88 three prefixes hold 64% of the tree nodes.

## Counts

Medians over 40 prefixes, bits. Brackets: interval forced by cut runs. Parentheses: bootstrap 95%.

| Lead | 112 | 104 | 96 | 88 | 88 minus 104 |
|---|---|---|---|---|---|
| K over AC tried | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 (0 to 0) |
| K over AC failed | 0.00 | 0.00 | -0.78 | [-1.40, -1.32] | [-1.40, -1.32] (-1.58 to 0) |
| C over AC tried | 0.00 | 0.00 | -0.87 | [-1.42, -1.02] | [-1.42, -1.02] (-2.18 to -0.26) |
| U0 over AC tried | 0.00 | 0.00 | +0.94 | +1.76 | +1.76 (0 to +1.81) |
| AC over T | +3.32 | +4.94 | +5.70 | +5.80 | +0.86 (-2.05 to +2.51) |

Sums, log2(summed work Y / summed work X); "at most" where a cut run is in the sum:

| Lead | 112 | 104 | 96 | 88 |
|---|---|---|---|---|
| K over AC | +0.95 | -0.07 | +0.14 | at most -0.25 |
| C over AC | +0.83 | -0.29 | -2.03 | at most -0.80 |
| U0 over AC | +0.83 | +1.86 | +1.83 | +1.82 |
| K over U0 | +0.12 | -1.93 | -1.68 | at most -2.07 |
| AC over T | +4.25 | +6.60 | +5.94 | +6.02 |

Prefixes where AC or K did over 100 units (unplanned, descriptive): median K over AC tried is -0.09 (3 prefixes) at 104, +0.09 (18) at 96, [+0.02, +0.12] (24) at 88, range at 88 from -2.51 to +1.08. This compares conflicts with branches tried (Correction 5).

## Heavy prefixes, depth 88 (asterisk: cut, lower bound)

| Prefix | T | AC tried | AC failed | K | C | U0 | K over AC |
|---|---|---|---|---|---|---|---|
| 37 | 239.9 M | 3,314,612 | 984,093 | 3,816,631* | 2,347,758* | 953,248 | at most -0.20 |
| 16 | 203.9 M | 3,562,259 | 1,012,406 | 1,683,973* | 842,726* | 986,602 | at most +1.08 (Correction 6) |
| 03 | 174.5 M | 2,647,774 | 750,403 | 3,862,057* | 2,342,867* | 732,854 | at most -0.54 |
| 20 | 52.7 M | 938,839 | 284,209 | 851,067 | 1,923,931* | 277,840 | +0.14 |

Outliers of K over AC: -4.25 (d96_37, tree 3,523 nodes), -2.51 (d88_14); +3.83 (d112_29, tree 805 nodes), +1.51 (d96_05). `results/analysis/analyze.out.txt` lists all 13 cut runs and the eight heaviest prefixes per depth; `results/analysis/per_prefix.out.tsv` has every prefix.

## Seconds

These ratios compare programs, not methods: an untuned C arc-consistency solver copying a 37 KB state per branch (AC, about 21,500 branches a second); CaDiCaL 1.5.3 called from Python on a pairwise CNF (K, about 1,300 conflicts a second on the slowest machine, 2,628 to 3,148 on the others); clasp 3.3.10 on the same CNF (C about 1,600, U0 about 4,000); a plain C row scan (T, about 60 million nodes a second). Search-phase wall seconds, floored at 0.01 s; log2(seconds Y / seconds X), positive when X is faster.

| Programs | 96 median | 88 median | 88 sums |
|---|---|---|---|
| K against AC | -0.86 | [-3.16, -3.03] | at most -3.56 |
| C against AC | -1.50 | [-5.10, -4.58] | at most -4.77 |
| U0 against AC | 0.00 | -0.28 | -0.39 |
| K against T | -0.86 | [-8.65, -8.53] | at most -9.02 |
| AC against T | not computed | not computed | -5.46 (-4.46 at 96) |

Medians at 112 and 104 are all 0.00 (floor against floor). Summed search seconds at 88: T 15.5, AC 689, U0 901, K at least 8,146, C at least 18,806. Processor seconds agree with wall. CNF loading is excluded. Per machine at depth 88 the K against AC ratio is -3.32, -3.68, -3.78, so pooling machines does not distort it.

## The stop line

The pre-stated test: the lead of the learner over the propagation-only control "does not rise by ... 3 bits from depth 104 to 88".

K over AC:

- Planned statistic (median, branches tried): 0.00 at 104, 0.00 at 88, rise 0.00 (bootstrap 0 to 0). Both counts are zero on 37 of 40 prefixes at 104 and 15 of 40 at 88; three leads at 88 are upper bounds. Fired, and uninformative.
- Failure against failure: rise [-1.40, -1.32].
- Sums (the measure that carries cost): -0.07 at 104, +0.14 at 96, at most -0.25 at 88, with no lower bound at 88 (K cut on the three largest prefixes). Rise from 104 to 88 at most -0.18; none of 10,000 bootstrap resamples reaches 3.
- Prefixes with work (unplanned): about +0.1 at 96 and 88 against branches tried; about -1.7 against failed branches (Correction 5).
- Seconds, these programs: -3.0 to -3.2 in medians at 88, -3.3 to -3.8 per machine and in sums.

## Unplanned observations

- U0 conflicts match AC failed branches within 0.03 bits in the median at 88.
- Clasp default did more conflicts than U0: 3.9 bits in sums at 96, at least 2.6 at 88.
- Clasp's learnt clauses average 511, 934 and 1,366 literals at 104, 96, 88 (median over prefixes with work): longer as the prefix gets shallower.

## Not run

A cross-machine count check of every arm; a propagator-based learner with short explanations; a tuned incremental arc-consistency solver; any run over 1,200 s per arm.
