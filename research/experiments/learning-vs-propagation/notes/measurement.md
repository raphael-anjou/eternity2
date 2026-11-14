# Learning against propagation on toy boards, 5x5 to 7x7

2026-11-14. Status: independently re-checked (`notes/recheck.md`); the corrections below apply. Generated toys only; not Eternity II progress. Raw outputs are in `results/raw/`, scripts in `scripts/`, and the analysis outputs (`results/analysis/analyze.out.txt`, `results/analysis/analyze_extra.out.txt`, which hold every number below) in `results/analysis/`.

## Corrections from the independent re-check

The re-check (`notes/recheck.md`, script `scripts/recheck/recompute.py`) reproduces the finished and cut table and the paired medians below, and finds these statements stronger than the data. Read the sections below with these corrections.

1. Ruling: the pre-stated stop line is undecidable on this data. Its depth clause (real pieces, depths 104 to 88) was never run. For the size clause (CaDiCaL over row scan with arc consistency) the median lead is +2.19 at 6x6 and at most +4.28 at 7x7, a rise of at most +2.09 against a threshold of 2, with no lower bound.
2. The "+3.92 using 8 boards" figure bounds those 8 boards only. 7x7 board 5 was dropped from every pair against arc consistency because the arc consistency run was killed with no output; CaDiCaL finished that board in 430 s, so it is the learning solver's best case (a lead of at least about 8 bits, inferred from the arc consistency rate, not counted). "Runs killed with no counts are dropped" is therefore not neutral. Board 4 (CaDiCaL killed, no counts) is unknown.
3. The bound on the 7x7 bootstrap interval (+0.35 to +2.73) and the 7x7 medians in the main table (+11.50, +10.53, +7.46 and the other cut-based entries) are computed on runs stopped early: upper bounds, not measurements.
4. "Each finished 7x7 learning run whose board AC also finished took longer than AC" excludes the one 7x7 board where learning won in seconds (board 5, by at least 3 bits). "Every learning arm slower at 6x6" is true of medians only; single boards reach +2.12 and +3.48 bits in learning's favour.
5. "U0 and U1 differ only by learning": they also differ by backjumping, restarts and lemma deletion.
6. "Identical counts on 12 test runs" across clasp versions is reported, not measured: no raw file supports it.
7. U0 seconds at 7x7 come from a different machine than the arc consistency seconds.
8. The 6x6 to 7x7 axis also changes tightness (inner colours 4, 5, 7; median solution counts 60, 296, 36), so a size effect is not separated from it.
9. Worth for the full puzzle: 0 to 2 bits in time (the re-check's estimate); in seconds learning is about 3 bits slower at 6x6 and on the 3 finished 7x7 pairs.

## Reading first

Bits due to learning, same solver (clasp) and static order, learning on (U1) against off (U0), paired per board, exhaustive: median +1.31 at 5x5 (bootstrap 95% +1.06 to +1.49, 10 boards) and +2.03 at 6x6 (+1.33 to +2.26, 10 boards), a change of +0.72 (-0.08 to +1.09). At 7x7 U1 finished 0 of 10, so there is no 7x7 value; the +6.06 analyze.py prints measures the cut, not learning (see "Cut runs").

Under failed-literal lookahead (L1 against L0): +0.01 and +0.92; none finished at 7x7.

In seconds every learning arm was slower than its control at 6x6 (log2 seconds ratio U1 against U0 -3.81, CaDiCaL against arc consistency -3.15), and each of the 4 finished 7x7 learning runs whose board AC also finished took longer than AC.

## Design

Boards: generator and CNF encoding in `toylib.py`, corner pinned, 3 frame colours, 4, 5, 7 inner colours at 5x5, 6x6, 7x7, seeds 1 to 10.

Arms, all on the same 30 boards:

| Arm | What | Work counted |
|---|---|---|
| T | plain row scan, `rowscan.c` (prints also nodes at first solution) | nodes |
| AC | row scan with arc consistency, `dyn.c`, `AC ROW` | branches tried |
| WD | weighted degree under arc consistency, `dyn.c AC WDEG` (no restarts) | branches tried |
| U0 | clasp `--no-lookback --lookahead=no --heuristic=None`: unit propagation, chronological backtracking, static order (variable index order, placements cell by cell) | conflicts |
| U1 | clasp `--heuristic=None`: same order, learning on | conflicts |
| L0 | clasp `--no-lookback` (forces failed-literal lookahead, heuristic Unit) | conflicts |
| L1 | clasp `--lookahead=atom --heuristic=Unit`, learning on | conflicts |
| C | clasp default (learning) | conflicts |
| K | CaDiCaL 1.5.3 via python-sat | conflicts |

Learning off is `--no-lookback`: no lemmas, backjumps or restarts ("Lemmas: 0", "Analyzed: 0", "Backjumps: 0" on a 4x4 check and every run). With it clasp's default heuristic forces lookahead, hence `--heuristic=None`; U0 and U1 differ by learning (and by backjumping, restarts and lemma deletion; Correction 5). CaDiCaL cannot switch learning off.

All arms enumerate every solution (K by blocking clauses); counts agree with the tree on every finished run. First-solution counts (clasp `-n 1`, K and T at their first solution) exist at 5x5 and 6x6, and at 7x7 for K and U0 only.

Ratio: log2(T nodes / work) per board; paired differences; 10,000-resample bootstrap over boards.

Limits: one thread a job, 3500 s internal limit, `timeout 3600`. machine-a (8 jobs) ran everything except U0, U1 and first-solution runs at 7x7, which ran on machine-b (up to 10). clasp 3.3.10 (machine-a build) and 3.4.1 (nixpkgs, machine-b) were reported to give identical counts on 12 test runs (not backed by a raw file; Correction 6).

## Finished and cut, per arm and size

At 5x5 and 6x6 every arm finished 10 of 10, first-solution runs included. At 7x7:

| Arm | Finished | Cut at 3500 s | Cut by the machine-b reboot | Killed at 3600 s, no counts | Not run |
|---|---|---|---|---|---|
| T | 10 | 0 | 0 | 0 | 0 |
| AC | 9 | 0 | 0 | 1 (board 5) | 0 |
| WD | 10 | 0 | 0 | 0 | 0 |
| U0 | 6 | 2 (3, 5) | 2 (9, 10) | 0 | 0 |
| U1 | 0 | 5 (1 to 4; 5 at 3452 s) | 5 (6 to 10) | 0 | 0 |
| L0 | 0 | 10 | 0 | 0 | 0 |
| L1 | 0 | 10 | 0 | 0 | 0 |
| C | 2 (5, 7) | 8 | 0 | 0 | 0 |
| K | 4 (5, 6, 7, 9) | 5 | 0 | 1 (board 4) | 0 |
| U0, first solution | 4 | 1 | 1 | 0 | 4 |
| U1, L0, L1, C, first solution | 0 | 0 | 0 | 0 | 10 each |

## Tables (exhaustive, median log2(tree nodes / work), boards used, of which cut)

| Arm | 5x5 | 6x6 | 7x7 |
|---|---|---|---|
| tree, log2 nodes | 16.15 | 25.27 | 30.75 |
| AC | +3.77 (10, 0) | +4.39 (10, 0) | +4.07 (9, 0) |
| WD | +4.90 | +5.32 | +5.64 (10, 0) |
| U0 | +5.59 | +5.71 | +5.46 (10, 4) |
| U1 | +6.89 | +7.73 | +11.50 (10, 10) |
| L0 | +7.55 | +6.99 | +7.46 (10, 10) |
| L1 | +7.58 | +8.19 | +10.53 (10, 10) |
| C | +7.06 | +7.94 | +8.02 (10, 8) |
| K | +6.16 | +6.97 | +8.17 (9, 5) |

Paired differences, bits (median, bootstrap 95%):

| Pair | 5x5 | 6x6 | 7x7 with cut runs | 7x7, both finished |
|---|---|---|---|---|
| U1 minus U0 | +1.31 (+1.06, +1.49) | +2.03 (+1.33, +2.26) | +6.06 (10 cut) | none |
| L1 minus L0 | +0.01 | +0.92 | +3.15 (10 cut) | none |
| K minus U0 | +0.37 | +0.92 | +2.59 (9 boards, 7 cut) | +3.28 (2 boards) |
| C minus U0 | +1.34 | +1.84 | +2.41 (10, 9 cut) | +4.34 (1 board) |
| K minus AC | +2.32 (+1.85, +3.11) | +2.19 (+1.80, +3.60) | +3.92 (8, 5 cut) | +3.89 (3 boards) |
| C minus AC | +3.32 | +3.20 | +3.68 (9, 8 cut) | +5.55 (1 board) |
| U1 minus AC | +3.06 | +3.34 | +7.38 (9, 9 cut) | none |
| U0 minus AC | +1.68 | +1.28 | +1.26 (9, 3 cut) | +1.22 (6 boards) |

First solution, U1 minus U0: +0.77, +1.15. Seconds, both finished, log2(control / learner): U1 against U0 0.00, -3.81; K against AC -2.00, -3.15, -2.98 (3 boards). Median seconds of finished 7x7 runs: T 42, AC 408, WD 260, U0 2144, C 477 (2), K 1644 (4).

## Cut runs: how they enter, and the bias

analyze.py takes a cut run's counter at the moment it stopped (clasp prints statistics on the limit or the reboot signal; K writes its totals) as the work, so the ratio is log2(tree nodes / conflicts so far). Finishing needs at least that many, so a cut ratio is an upper bound that overstates the lead. Runs killed with no counts (AC board 5, K board 4) were dropped here; the re-check shows this is not neutral (Correction 2).

A paired difference "learning minus control" is inflated when the learner is cut and deflated when the control is. At 7x7 the learners are cut, and they are much slower per conflict: median rate on cut runs U0 18,553, L0 3,058, C 2,419, K 2,611, L1 357, U1 256 conflicts per second. U1's 10 cut runs stopped at 0.29 to 0.99 million conflicts, while U0 reached 21 to 77 million on the same boards (finishing on 6); five of the U1 runs were cut by the reboot at 966 to 2526 s, earlier still. So the 7x7 figures for U1 minus U0 (+6.06), L1 minus L0 (+3.15), U1 minus AC (+7.38) and every cut-based K or C figure lean upward by an amount these data do not bound.

Pairs where both finished avoid that bias but select the easy boards (K finished 4 of 10, C 2), so they too lean upward, on 1 to 3 boards.

## The kill line, quoted, with the numbers

Pre-stated test, section 2.3: "Kill line: the lead over the propagation-only control does not rise by 2 bits from 6x6 to 7x7, and not by 3 bits from depth 104 to 88."

With the pre-stated control (row scan with arc consistency) and solver (CaDiCaL): lead at 6x6 +2.19 (10 boards, none cut); at 7x7 +3.92 using 8 boards of which 5 cut (an upper bound for those 8 only; see Corrections 1 and 2), change +1.73 (bootstrap 95% +0.35 to +2.73); on the 3 boards where both finished +3.89, change +1.70. A rise of 2 bits would need +4.19 at 7x7. With clasp default as the learner: +3.20 to +3.68, change +0.48 (-0.63 to +2.11), 8 of 9 cut. With clasp static-order learning: +3.34 at 6x6, no finished 7x7 run. With the learning-off control (clasp, learning off): U1 minus U0 +2.03 at 6x6, no finished 7x7 run. The depth clause has no numbers: part (b) was not run.

## Deviations from the pre-stated test

- Part (b), 40 wrong real-piece prefixes at free depths 112 to 88, was not run.
- 10 boards a size, not 8; limit 3500 s a run, not 2 hours.
- No cardinality solver. Weighted degree is dyn.c's WDEG without restarts. Added: four clasp arms and clasp default.
- Colours: 3 frame colours; the pre-stated test's 4.6 and 4.7 arc-consistency bits came from a ladder with 4 frame colours.
- `dyn.c` is a copy changed in two places: it reads a corner pin, and it drops a tile turn equal to an earlier turn (symmetric tiles; before this it counted 2 to 8 times the solutions on 15 boards). AC and WD at 5x5 and 6x6 were rerun.
- Machine-b rebooted at 04:30:03 (system restart), nothing relaunched: 7x7 U0 boards 9, 10 and U1 boards 5 to 10 cut early; first-solution U0 board 6 cut, U0 boards 7 to 10 and all 40 of U1, C, L0, L1 never run.

## Not run

8x8; part (b); a cardinality solver; weighted degree with restarts; 7x7 first-solution runs for U1, C, L0, L1; any run over 3500 s, which the 7x7 learners need.
