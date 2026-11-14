# Clause learning against propagation on toy boards, 5x5 to 7x7

Status: independently re-checked (`notes/recheck.md`; the measurement note carries its corrections). Generated toy boards only; nothing here is Eternity II progress. The pre-stated stop line is undecidable on this data.

## Question

On generated framed edge-matching boards, how many bits of search work does clause learning save, compared with a solver that only propagates? Does the lead grow with size? Pre-stated test: "Kill line: the lead over the propagation-only control does not rise by 2 bits from 6x6 to 7x7, and not by 3 bits from depth 104 to 88."

## Design

Ten boards per size (seeds 1 to 10), corner pinned, 3 frame colours, 4, 5, 7 inner colours. Score: log2(row-scan tree nodes / work) per board, paired differences, 10,000-resample bootstrap. One thread, 3500 s limit.

| Arm | Solver | Work counted |
|---|---|---|
| T | plain row scan (`rowscan.c`) | nodes |
| AC | row scan with arc consistency (`dyn.c`) | branches tried |
| WD | weighted degree under arc consistency (no restarts) | branches tried |
| U0 | clasp, learning off, static order | conflicts |
| U1 | clasp, learning on, same order | conflicts |
| L0 / L1 | clasp, failed-literal lookahead, learning off / on | conflicts |
| C | clasp default | conflicts |
| K | CaDiCaL 1.5.3 (python-sat) | conflicts |

## Results

Median bits, tree nodes over work (10 boards each unless noted):

| Arm | 5x5 | 6x6 | 7x7 (finished of 10) |
|---|---|---|---|
| AC | +3.77 | +4.39 | +4.07 (9) |
| U0 | +5.59 | +5.71 | +5.46 (6; 4 cut) |
| U1 | +6.89 | +7.73 | none finished (0) |
| K | +6.16 | +6.97 | +8.17 (4 finished; 5 cut, 1 killed; bound) |

All arms finished 10 of 10 at 5x5 and 6x6.

- Bits due to learning, U1 minus U0, paired, exhaustive: **+1.31** at 5x5 (95% +1.06 to +1.49) and **+2.03** at 6x6 (+1.33 to +2.26). There is no 7x7 value: U1 finished 0 of 10 boards.
- 7x7 table entries computed on runs cut at the limit (for example +6.06 for U1 minus U0) are upper bounds, not measurements: a cut run stopped at a smaller count than finishing would need.
- Pre-stated test: **undecidable on this data.** The depth clause needs the real-piece part, which was never run. The size clause: median lead of CaDiCaL over row scan with arc consistency +2.19 at 6x6 and at most +4.28 at 7x7, a rise of at most +2.09 against a threshold of 2, with no lower bound (a cut solver may need any number of further conflicts).
- The fault found by the re-check: 7x7 board 5 was dropped from the paired figure because the arc consistency run was killed with no output, although CaDiCaL finished it in 430 s. It is the learning solver's best case (at least about 8 bits, inferred). The measurement note's "+3.92" therefore bounds only the 8 boards it used.
- Worth for the full puzzle: 0 to 2 bits in time (the re-check's estimate). In seconds learning is about 3 bits slower at 6x6 and on the 3 finished 7x7 pairs.
- Size is confounded with tightness (inner colours 4, 5, 7).

Per-arm finished and cut counts at 7x7 are in `notes/measurement.md`.

## Not run

8x8; the real-piece part (40 wrong prefixes at free depths 112 to 88); a cardinality solver; weighted degree with restarts; any run over 3500 s. Some 7x7 jobs were lost when one machine restarted.

## Rerun the analysis

From the repository root: `just experiments learning-vs-propagation`. It runs `scripts/analyze.py` and `scripts/analyze_extra.py` (stdlib only, `python3 -I`) on `results/raw/` and prints to the terminal; committed outputs: `results/analysis/`.

## Rerun the jobs

Verified: `scripts/job.sh ARM BOARD` (ARM in T, AC, WD, U0, U1, L0, L1, C, K, with an `f` suffix for first-solution clasp arms; BOARD such as `6_3`); queues are `scripts/jobs.*.txt`, started with `sh scripts/launch.sh QUEUE PARALLEL` or `scripts/pool.sh`. `job.sh` expects `clasp`, `rowscan`, `dyn8` in `scripts/` and boards in `scripts/boards/`.

Boards: `python3 -I scripts/mkboards.py N 3 CI 1 10` with (N, CI) = (5, 4), (6, 5), (7, 7). Checked: this regenerates every `.rs` and `.cnf` file byte for byte against the files the runs used, and the `.dyn` files apart from one comment line.

Unverified: solver builds. clasp 3.3.10 and 3.4.1 were used; CaDiCaL 1.5.3 comes through python-sat. Inferred, not run: `cc -O2 -DMAXC=64 -DW=4 -o scripts/dyn8 scripts/dyn.c` and `cc -O2 -o scripts/rowscan scripts/rowscan.c`.

## Hardware

machine-a: 8-core Apple M1 laptop, 8 jobs at a time; machine-b: 14-core low-power x86 laptop, up to 10 jobs; ran 7x7 U0, U1 and first-solution runs.
