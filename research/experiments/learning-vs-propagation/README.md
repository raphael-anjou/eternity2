# Clause learning against propagation: toy boards and the real pieces

Status: both parts independently re-checked (`notes/recheck.md`, `real-prefixes/notes/recheck.md`; each measurement note carries the corrections). Generated toy boards (part 1) and wrong prefixes of the real pieces (part 2). Nothing here is Eternity II progress.

## Outcome

**The pre-stated stop line fired.** The test: "Kill line: the lead over the propagation-only control does not rise by 2 bits from 6x6 to 7x7, and not by 3 bits from depth 104 to 88." Clause learning does not buy search work over propagation on this encoding with these solvers. Figures in bits of log2 work.

- **Depth clause: fired, firmly, on summed work.** CaDiCaL's lead over row scan with arc consistency is -0.07 at depth 104, +0.14 at 96, at most -0.25 at 88: flat, not growing. The 88 figure has no lower bound (CaDiCaL was cut on the three largest prefixes); a cut can only lower it.
- The planned figure, the median over 40 prefixes, is 0.00 at 104 and at 88. It is uninformative: most prefixes are refuted at the root by every solver.
- **Size clause (toy boards): leans fired, not established.** Lead +2.19 at 6x6 and at most +4.28 at 7x7, a rise of at most +2.09 against 2, with no lower bound.
- The solver with learning switched off needs the fewest steps (summed at depth 88: 4.19 million conflicts, against 14.8 million branches for arc consistency). The learning solver needs more conflicts than it on 45 of 45 prefixes with work (-1.93, -1.68, at most -2.07 at 104, 96, 88).
- In time, with these programs, learning is 3.3 to 3.8 bits slower than arc consistency and about 9 bits slower than plain row scan.
- Worth for the full puzzle: 0 bits in counts over arc consistency (range -0.7 to +0.1), against a gap of 56 to 84 bits.

**Properties of these programs, not of the method:** every figure in seconds; the pairwise encoding of about 24 million clauses at depth 88; CaDiCaL 1.5.3 called through a Python wrapper (5,000 conflicts a call); an untuned arc-consistency solver (copies its state at every branch); clasp default's counts, which vary with the build.

**Properties of the method, as far as shown:** counts of conflicts or branches. Learning gives no lead over arc consistency and costs 1.7 to 2.1 bits against propagation alone on the same encoding. Learnt clauses average 511, 934 and 1,366 literals at depths 104, 96, 88 (longer as the prefix gets shallower). A propagator-based learner with short explanations was not run.

## Part 1: toy boards, 5x5 to 7x7

Ten boards per size (seeds 1 to 10), corner pinned, 3 frame colours, 4, 5, 7 inner colours. One thread, 3500 s limit. Arms: T plain row scan, AC row scan with arc consistency, WD weighted degree, U0 and U1 clasp with learning off and on in the same static order, L0 and L1 with lookahead, C clasp default, K CaDiCaL. Paired differences, 10,000-resample bootstrap.

- Bits due to learning, U1 minus U0, exhaustive: +1.31 at 5x5 (95% +1.06 to +1.49) and +2.03 at 6x6 (+1.33 to +2.26). No 7x7 value: U1 finished 0 of 10.
- 7x7 entries computed on cut runs are upper bounds, not measurements. 7x7 board 5 was dropped from the pairs against arc consistency because that run was killed with no output; CaDiCaL finished it in 430 s, the learner's best case, so the "+3.92" figure bounds 8 boards only.
- Size is confounded with tightness (inner colours 4, 5, 7).
- In seconds learning is about 3 bits slower at 6x6.

Details: `notes/measurement.md`, `notes/recheck.md`. Not run: 8x8, a cardinality solver, weighted degree with restarts, runs over 3500 s.

## Part 2: real pieces, depths 112 to 88

Official pieces, five clues, row scan from the top left. 40 sampled wrong prefixes at each depth (d free cells placed), refuted exhaustively by T, AC, K, C and U0, 1,200 s per run. 800 runs, 160 prefixes, 9.15 core-hours, no completion anywhere; 13 cut runs, all at depth 88 (K on 3 prefixes, C on 10). The first pilot used a reversed depth definition and was discarded before the full run (`real-prefixes/wrong-depth-pilot/`). The sampled prefixes miss the rare costly ones at 112 and 104 (2.2 to 2.7 bits below the population mean).

Details: `real-prefixes/notes/measurement.md` (design, tables, deviations, corrections), `real-prefixes/notes/plan.md`.

## Layout

- `scripts/`, `results/raw/machine-a|b`, `results/analysis/`, `notes/`: part 1.
- `real-prefixes/scripts/` (sources, queues, `recheck/`), `prefixes/` (the 160 prefixes), `results/raw/machine-a|b|c` and `pilot-corrected/`, `results/analysis/`, `notes/` (plan, measurement, recheck), `wrong-depth-pilot/`.
- The piece file is `engine/data/official_eternity2.csv` (byte-identical to the file the runs read).

## Rerun the analyses

From the repository root: `just experiments learning-vs-propagation`. It runs the toy analyses, the toy recomputation, the real-prefix analysis and its independent recomputation (stdlib only, `python3 -I`, a few seconds). The real-prefix analysis reproduces the committed `real-prefixes/results/analysis/analyze.out.txt` and `per_prefix.out.tsv` exactly, and the recomputation (its own parser) finds no mismatch against that table.

## Rerun the jobs

Part 1: `scripts/job.sh ARM BOARD`, queues `scripts/jobs.*.txt`, boards from `scripts/mkboards.py` (verified byte for byte).

Part 2, not rerun in this form: build with `sh real-prefixes/scripts/build.sh`, copy clasp 3.3.10 to `bin/clasp`, then `real-prefixes/scripts/worker.py QUEUE LABEL` (it expects `bin/`, `prefixes/`, `out/` next to it; copy the `scripts/` and `prefixes/` folders into one working folder first). Prefixes: `mkprefixes official.csv 40 112 104 96 88`. The independent rerun of counts is `scripts/recheck/rerun.py` (reported in `notes/recheck.md`).

## Hardware

machine-a: 8-core Apple M1 laptop (part 1: 8 jobs; part 2: 2 workers, 16 GB). machine-b: 14-core low-power x86 laptop (part 1: up to 10 jobs; part 2: 4 pinned workers). machine-c: 10-core Apple M4 laptop, 32 GB (part 2 only: 3 workers). CaDiCaL runs at 1,374 conflicts a second on machine-b against 2,628 and 3,148 on the Apple machines.
