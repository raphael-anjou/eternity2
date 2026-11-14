# Frame cores: what refutes a wrong frame of the real pieces, and how local is the reason

Arc 2026-11-13, track D1. Preregistered measurement (`PREREG.md`, first
commit of the branch; amendments A1 and A2 in `PREREG_AMENDMENTS.md`, both
committed before any test run). Real Eternity II pieces with the five
clues. Not a record attempt, no board produced. Results on generated boards are
never official Eternity II progress.

Grades: **measured** (by a run committed here, file named), **argued**
(arithmetic or reasoning, no run).

## Status

Test stage done. The detached driver (`scripts/driver.sh`) ran on the compute
host from 2026-11-13 08:37 to 14:45 CET; the results were copied back
unchanged. `results/summary.json` and `results/tables.md` were then
regenerated on this machine by the committed scripts from the raw rows; both
are byte-identical to the files the remote driver wrote (kept as
`results/summary.remote.json` and `results/tables.remote.md`). No test output
had been looked at when amendments A1 and A2 were written. These readings are
the runner's; the judge has not ruled.

## Readings

Every number below is in `results/summary.json` (key named) and
`results/tables.md`. All grades are **measured** unless marked argued. No
refuter returned `sat` on any test frame or arc.

- **Stage 1 kill line (PREREG, real H, 60 frames): met, cheap refutation of
  full frames is dead.** Lower median of r_X with 95% interval:
  R1-rows 0.89 [0.04, 1.00], R1-spiral 0.12 [0.04, 1.00], R2 1.00
  [1.00, 26.49] (`stage1["real/H"]`). None reaches a median of 100 with a
  lower end of 10. Censored pairs: 24, 22 and 14; plain search censored on 25
  of 60 frames (A1 item 2). Ratio of total times R1-rows / R1-spiral / R2:
  0.67 / 0.77 / 0.38.
- **Stage 2 reading (PREREG with A1, real H): the pass line holds by the
  letter of the preregistration on the median, and carries no evidence for
  nogood-driven frame search.** The median sits on frames near the clue
  corners (21 of the 23 near frames have a dead 3x3 clue block), the PREREG's
  own arithmetic bound is a mean over frames rather than a median, and the
  far stratum is undecided (see the strata bullet below). The line reads
  "passes the necessary condition"; the fail line (M_H > 134.4 or median R2 core of
  54 cells or more) is not met. M_H = median of b_H + log2 C = 36.75 bits,
  95% interval [32.90, 53.19], so below 120 with the upper end below 134.4.
  Median R2 minimised core 5 cells (quartiles 3 and 21). 16 of 60 frames have
  no proved nogood (censored; see the counter note below). As PREREG says, this is a necessary condition
  on a covering, not a method; overlap between nogoods is unmeasured.
  Secondary (PREREG form, median v_F - beta_H): 19.61 bits [16.54, 37.39].
- **A2 floors (real H): the A2 fail line (largest floor above 134.4) is not
  met.** F(0.4) 31.24, F(0.25) 34.48, F(0.1) 59.07, log2 mean of 2^v 71.94;
  largest 71.94 bits. A2 has no pass line (60 frames certify at most
  log2(60/3) = 4.3 bits of gain).
- **Near and far strata (A2, real H).** Near (every cell of the cheapest
  nogood within ring distance 3 of a corner): 23 frames, 0 censored, median v
  29.83 [28.30, 33.78], median R2 core 3 cells. Far (the rest, censored
  included; the frames away from the clue corners): 37 frames, 16 censored,
  median v 75.12 [42.64, inf], median R2 core 16 cells, largest A2 floor
  72.63. A2 fixed no separate line for the strata; applying the PREREG
  thresholds to the far stratum gives neither line: the median is below 120
  but the upper end of its interval is infinite, so the far stratum is
  undecided. Stage 1 by stratum: on near frames R2 has median ratio 62.88
  [0.61, 559.69] and total-time ratio 1285.65; on far frames 1.00
  [1.00, 14.65] and 0.27. R1 is below 1 on near frames (0.04, 0.05) and at 1
  on far frames.
- **Extra set realfar (A2 addendum, 60 H frames with no dead 3x3 clue block,
  no kill line).** Median v 86.57 [46.08, inf], 29 of 60 censored, median R2
  core 17.5 cells, largest A2 floor 89.69. Far stratum: 52 frames, median v
  infinite (more than half have no proved nogood), interval [58.43, inf].
  Counter censoring: when the frame counter returns null (no particle
  survived), `analyze.py` drops that nogood, so a frame left without a usable
  nogood counts as censored (v = inf) although a refuter proved it. This
  turns 2 real H frames (realH11, realH24) and 5 realfar frames (realHx35, 73,
  76, 97, 124) into censored frames, and accounts for 5 of the 29 "no proved
  nogood" frames in realfar. It is conservative for the median and does not
  change any number reported here.
  Stage 1: R1-rows, R1-spiral and R2 all have median ratio 1.00; R2 interval
  [1.00, 4.77], plain search censored on 37 of 60.
- **Bottom arcs, A1 readings.** R2: at k = 12 (38 cells) 30 of 60 arcs are
  refuted within 60 s, at least the 15 required, so the A1 line "below the
  memo's claimed break-even" is met for R2. This is the same corner effect as
  in the strata: at k = 12 all 18 frames with a dead bottom 3x3 block are
  refuted, against 12 of the other 42. At k = 13 (40 cells) 34 arcs are
  refuted within 65 s, so the A1 "dead" line is not met for R2. R0-arc was
  run at k = 15 only (A2 item 4); neither A1 line can be read for it.
  Refuted within 60 s per arc: k = 15 (44 cells) 42, k = 14 (42 cells) 41,
  k = 13 34, k = 12 30 (R2); R0-arc at k = 15: 21. For R2, no arc refutation took
  longer than 60 s at any k (unsat count equals the within-60 s count). For
  R0-arc, 22 arcs were refuted, 21 within 60 s, the slowest in 107.26 s.
- **Bottom arcs, A2 nested cost: no level is below break-even.** log2 of
  mean c_k times r against the budget, in nodes: k = 15 (44 cells) 27.7
  against 25.5; k = 14 (42 cells) 31.4 against 29.0; k = 13 (40 cells) 35.4
  against 32.9; k = 12 (38 cells) 39.5 against 37.0. The budgets are the A2
  figures (40 cells 32.9, 38 cells 37.0, 34 cells 46.0), the same as in the
  lead's notes. The k = 12 budget (5.73e+04 s) is above its 120 s cap, so
  that level cannot kill and is stated, not read (A2 addendum). The nested
  cost (2.2 to 2.5 bits above every budget) is the reading that governs: the
  A1 line for R2 is the corner effect above. The 34-cell
  arc (k = 10) was replaced by A2 and not run. The A1 line for R2 and the A2
  nested cost point in opposite directions; both are reported as
  preregistered.
- **Uniform real frames, corner claim (A1).** 30 of 30 U frames refuted, 28
  of 30 with the cheapest nogood within ring distance 2 of a corner, median
  best plain nodes 3.5. beta_H measured 17.24 bits against the memo's 17.45.
  Median v on real U 14.43 bits [13.50, 24.13]. No line was fixed.
- **Null boards (no reading).** Stage 1 on H, lower median for R1-rows /
  R1-spiral / R2: 1101 0.09 / 0.12 / 4.92, 1102 1.00 / 1.00 / 1.00, 1103
  0.57 / 0.26 / 16.43, pooled 0.70 / 0.55 / 6.57 [1.00, 52.91]. Stage 2 median
  v on H: 1101 41.92, 1102 91.52, 1103 37.07, pooled 39.68 [34.99, 70.51];
  beta_H 17.85, 17.15, 17.56 bits. As on the real pieces, near frames are
  cheap (median v 31.39, 28.99, 32.22) and far frames dearer (119.33,
  infinite, 56.78).
- **Stage 0: every refuter passed; none was dropped.** (ii) 8 small boards,
  34 frames each, 272 instances per refuter, every verdict right; (iii) 272
  R2 against R0-rows pairs, 0 disagreements; 720 nogoods checked, 0 unsound;
  partial-frame arcs 544 instances per arc refuter, 0 errors. Counter on 6x6:
  median absolute error 0.0055 and maximum 0.049 in log10 (lines 0.05 and
  0.3). Real-piece N_F: SMC 37.603 against exact 37.608 in log10 (line 0.05).
  (i) True frames: no `unsat`; every `sat` verified. On the three 10x10
  boards R2 confirmed all three (56.8 to 349.9 s); on the three 16x16 boards
  all five refuters hit the 600 s cap, so the 16x16 confirm cost is censored
  (a lower bound only, A2 item 6) (`results/stage0/summary.json`).
- Node rate r (median of capped R0-rows runs): real 2.40e6 nodes/s over 40
  runs, null 2.53e6 over 44 (`node_rate`).

## Deviations and what did not run

- The run log (`results/run-log.jsonl`) has a start and an end line for each
  guarded stage, `test` (exit 0, 17265.2 s) and `test-far` (exit 0,
  3864.1 s). Both start lines carry PREREG sha256 `88c189b9...`, the hash of
  the committed `PREREG.md`. The compute host ran a copied tree, so the guard
  logged `git: unavailable (copied tree)` and no HEAD or PREREG commit.
- The test stage ended at 13:41, before the 13:45 stop rule; no job was
  stopped. The realfar set (stage `test-far`) ran 13:41 to 14:45, after
  13:45, inside the 22:00 window of the A2 addendum.
- The test stage took 4.8 hours wall against the 2.5 hours estimated in
  PREREG (the A1 and A2 arc runs were added after that estimate).
- The PREREG secondary L_H (b_F, minus beta_H) is computed by
  `scripts/analyze.py` as log2 of the mean of 2^v with v from b_H and no
  beta_H subtraction (the A1 form), so the column labelled "L (secondary,
  b_F)" in `tables.md` equals the A2 "log2 mean" column. It carries no
  reading.
- Not run: R1 and the spiral on arcs (A1), R0-arc at k = 14, 13, 12 (A2
  item 4), arcs of 34 and 30 cells (replaced by A2), the incremental solver
  arm (not scheduled, A2 addendum).
- `results/cal.log` is the driver's whole stderr, not the calibration log: it
  holds the sampler lines of the test and realfar stages (including far.sh's
  "60 kept"). It ends with five `Terminated` lines; the calibration step
  exited 0 (`results/PROGRESS.txt`) and the cause of those lines is not
  recorded.
  Calibration informs caps only and no calibration frame enters the test.

## Method

### Populations

- Real puzzle: `single-core-benchmark/engine/data/puzzles/size_16_official_eternity.csv`
  with its five clue columns.
- Null boards: `generate_solved_framed_bc(16, 22, seed, true, 5)` (5 frame
  colours, 17 interior colours), seeds 1101, 1102, 1103, piece ids and
  rotations scrambled, the planted pieces at the five official clue cells
  given as clues.
- Stratum U: valid frames (60 border cells, each border piece once, grey
  outward, all 60 border-to-border joints matching). Stratum H: valid frames
  that also pass the four clue-block tests (each 3-cell interior block next to
  a corner clue has a fill with distinct free interior pieces).
- Sampler (`fc sample`): sequential Monte Carlo over the ring, 5000
  particles, systematic resampling at every step, clue-block test as a zero
  weight for H, one frame per independent run. Approximately uniform (SIR,
  bias of order 1/particles), not exactly. Every sampled frame was checked
  valid by `scripts/common.py check_frame` (independent code).

### Refuters (single thread each)

- R0-rows, R0-spiral: plain depth-first interior fill (Rust, `engine/src/main.rs`).
- R1-rows, R1-spiral: R0 plus a matching (Hall) check after every assignment:
  the unfilled cells that touch a filled cell need distinct representatives
  among unused pieces.
- R2: Glucose 4.2.1 through PySAT 1.9.dev15, direct encoding (`scripts/sat.py`
  header), the 60 border placements passed as assumptions.

Caps: R0 and R1 120 s per frame; R2 600 s for the refutation, 300 s in total
(3 s per call) for deletion-based core minimisation, 600 s for the proof of the
minimised core alone.

### Nogoods, information, cost

- Nogood sources: the R2 minimised core (a proved nogood; placements kept
  because a call was capped are flagged, so its size is an upper bound), and
  the R0 read set (border cells next to an interior cell the search reached;
  the same tree refutes every frame agreeing on them).
- b(S) = log2(N_F / N_F(S)) bits, N by the SMC counter (`fc count`, 20,000
  particles, 3 runs; N_F and N_H with 10 runs).
- C(S): R0 nodes for a read set; for an R2 core, the seconds of a fresh solver
  proving the core alone unsatisfiable times r, the median node rate of the
  capped R0-rows runs on the same puzzle class (same host, same load).
- v = b + log2 C; per frame the minimum over its nogoods. For H frames b is
  counted within H (A1): b_H(S) = log2(N_H / N_H(S)). The PREREG form,
  b_F - beta_H with beta_H = log2(N_F / N_H), is reported as secondary.
- The detached driver `scripts/driver.sh` runs stage 0, calibration, the test
  stage (`scripts/test.sh`), the extra far-from-corner set (`scripts/far.sh`)
  and the analysis in sequence.

## Reproduce

```sh
just experiments frame-cores          # build, stage 0, test (through prereg_guard), analysis
# or, from research/experiments/frame-cores:
sh run.sh build; J=10 sh run.sh stage0; J=10 sh run.sh test
python3 -I scripts/analyze.py results/test results/stage0/summary.json > results/summary.json
python3 -I scripts/tables.py results/summary.json
```

Needs nix (cargo, rustc, python3 with python-sat). The R2 timings depend on
the host; the SMC counts and frame samples are seeded.

## Compute

Host x86, 12 threads, at most 10 used (one burst of 12 for about two
minutes during the first stage 0 attempt, when calibration and the stage 0
brute force overlapped). This Mac: editing, git and analysis only (the local
analysis took 8.5 s on one core).

Wall clock from `results/PROGRESS.txt` (driver started 08:37 CET): stage 0
wait 08:37 to 08:47, calibration 08:47 to 08:53, test 08:53 to 13:41
(17265.2 s in `results/run-log.jsonl`), realfar 13:41 to 14:45 (3864.1 s),
analysis 14:45. With 10 jobs in parallel the test and realfar stages used at
most 10 x 5.87 h, about 59 core-hours.
