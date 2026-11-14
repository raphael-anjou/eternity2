# Drafts of arc 2026-11-13, not applied

## Track D2, draft LEDGER entry (from the plumber; two lines are its own summary, marked)

```
2026-11-13. D2 port fidelity (blackwood_bt), branch night/2026-11-13-d2-port-fidelity, head b9587fe5.
Outcome: kept after two judgments. Reproducibility: deterministic per seed; timings are ratios on a loaded host.
Fixed: items 1 to 7 of the port-vs-published-source differences now match his rules by default, each with a legacy switch (item 6 unconditional) [plumber's summary of FIDELITY's table]. Also fixed: the break path, the legal sort order (F2), the --quota-count header text (F5), and the first-corner gap, which is now listed in FIDELITY as not implemented.
First judgment: findings F1 to F5 and the first-corner gap, all answered in the round 7ca14eab to 1a4e5882. Second judgment (1a4e5882): keep. 23 of 23 run comparisons identical; 11 of 11 legacy runs reproduce the pre-branch binary; 12 of 12 histograms identical against the first judgment's binary; 13 of 13 tests pass.
Remaining cost: the no-break path is still 15 to 19% slower per node (new / old 1.15 to 1.19, seed 102, 1e8 nodes), documented in FIDELITY, not removed.
Five-clue finding: seeds 501 to 508, 1e8 nodes. New defaults reach depth 18 to 54 with 0 legal nodes at depth 201 or more; old defaults reach 33 to 215 with 1,529 (two seeds carry all of them). Six of eight seeds stop at depth 18 (the cell below the clue at x 2, y 13, which the scan reaches only at depth 34). No single legacy switch restores the old depths. His engine places only the centre piece, so five-clue rules are this port's extrapolation. Eight seeds, ranges only.
Consequence: the faithful defaults are for the no-clue and centre-clue settings; five-clue runs use the six legacy switches until a five-clue schedule is designed and measured.
Not checked: clippy, rustfmt, --threads > 1, break-path wall clock, quiet-host timing.
Files: bench-audit/src/bin/{blackwood_bt.rs, FIDELITY.md, JUDGE.md}, bench-audit/fidelity-results/{f1, item6, second-judgment}/.
```

## D4 draft ledger entry

```
## 2026-11-13 · Arc 2026-11-13, D4 prefix-statistics: S6 census tilt measured, not usable

Preregistered measurement (PREREG e9ecd977, amendments A1 to A6), judged KEEP as a measurement by a separate session. On 48 planted 16x16 boards with the official colour census and five clues (test stopped at 48 of 96 by A6), the census statistic S6 (configuration-model first moment, 33 ns per evaluation; S9 is the same finding, row correlation -0.93 to -0.95) puts the planted prefix above the sampled prefixes by +1.96 sd [1.52, 2.43] at depth 144 and +2.15 [1.74, 2.56] at 160 (Bonferroni 0.99833 bootstrap intervals), and passes Readings 1, 2 and 3. No board has the tilt reversed (minimum +0.41, +0.16). This refutes "no cheap statistic has |delta| >= 1.4 at the widest depths" on planted boards.
Value: not usable. One prune at depth 144, one scan direction: G = 1.02 bits [0.72, 1.27]; two directions -0.04; four -1.88. The registered rule's "yes" at depths 96 and 128 was rejected by the judge (work ESS median 3 and 13, below the A4 floor of 30); those cells are not gains.
Null model: the tilt is the size-bias the random model predicts (a prefix reweighted by its number of completions), measured at 1.5 to 1.8 times the first-moment prediction. A quantity, not an anomaly. No transfer to the official puzzle is shown.
Controls and rulings: negative control 2 (decoy) fails in one degenerate cell (S10 at 160, bootstrap tie artefact); by PREREG section 8 the readings are suspended, the judge ruled the failure uninformative (post hoc, recorded as such). The painter control on 6x6 fails 10 of 18 cells, in the direction that planted prefixes are less tilted, so transfer claims are not made. Judge recomputed every number from raw rows and reran board 1113029: 1,367 fields, 0 differences.
Side observation: arc consistency alone (S7, no search) refutes all 1,000 sampled prefixes in 189 of 192 rows at depth 160 and never the true prefix; whether a propagating scan pays 0.5 to 0.8 ms per node is unmeasured and proposed as the next track.
Learning: rank-bootstrap controls on a point-mass-plus-rare-tail statistic can fail by ties; future controls should use an exact test on the rare-tail count. The value rule needed the ESS floor it omitted.
Reproducibility: deterministic given seeds and binary except timing fields, `just experiments prefix-statistics` (about 4.8 core-hours on a Mac). Source commit: <fill at merge> (branch night/2026-11-13-d4-prefix-statistics). Files: research/experiments/prefix-statistics/.
Open: README corrections applied (6b642de1); `cargo fmt --check` fails on engine/src/{main,stats,tree}.rs (whitespace only, not applied so the committed code that ran stays the code on the branch).
```

## D5 draft ledger entry

```
## 2026-11-13 · Arc 2026-11-13, D5 pin-map: what a known tile is worth, by position

Preregistered measurement (PREREG 559c16b8, amendments A1 and A2), judged KEEP; README corrections applied after the judgment. Exact enumeration on generated 8x8 (cf 4, ci 11), 8x8 with five clues, 8x8 ci 10 and 10x10 boards; no kill line. Judge recomputed every headline number from raw rows, reran 1,177 units with 0 mismatches, and an independent C enumerator agreed on 35 further trees.
Result: under a row scan the worth of a known tile is set by its place in the scan, not the board. M8 mean 2.09 bits; rows 0 to 2 at 3.87, 5.36, 3.40, rows 4 to 7 at 0.54 to 0.81; fwd map against rev map rotated 180 degrees r = 0.998 (M8), 0.995 (M10). A frame effect remains outside the head rows (M10 rows 3 to 8: 0.58 against 0.13; M8C: 0.54 against 0.04). Pairs: median additivity 0.957, adjacent 0.84 median (aggregate 0.75), so clustering costs 15 to 25 percent of the pair's summed worth.
Placement: measured greedy (a per-board oracle bound) reaches the 1e3 cut with 2 tiles at M8; the usable clustered-in-best-rows rule needs 3; random 7. At k = 4 (M8, mean log2 nodes from 25.6): greedy 9.0, clustered 11.5, random 18.7, lattice 20.9.
bb-mrv (M8, boards 1..10, cap 1e5): at 8x8, ci 11, the placement effect mostly disappears from k = 6 (all but frame-only) and k = 8 (all); lattice ties clustered at k = 4 and beats it from k = 6.
Deviations and corrections: A1 (10x10 policies rest on boards 1 and 2); A2 (aggregate pair ratio by distance reported where PREREG asked median and mean; both given in the amendment). Seven README sentences weakened to the judge's numbers. The 16x16 statements stay labelled extrapolation, not measured.
Not checked by the judge: bb-mrv reruns beyond boards 1 and 6, M10 rev map and boards 1, 3, 4, 5 singles, N8 boards 22..25, calibration rows, figures.
Learning: calibration, not a method; scan position, not board structure, drives single-tile worth under a fixed order, and the head-of-scan placement result does not transfer to an adaptive solver.
Reproducibility: deterministic (exact counts), `just experiments pin-map all` (about 1.6 core-hours). Source commit: <fill at merge> (branch night/2026-11-13-d5-pin-map). Files: research/experiments/pin-map/.
Open: `cargo fmt --check` fails on the engine (not fixed, the committed code that ran stays as is).
```

## D3 draft ledger entry

### 2026-11-13: orders-and-pins (D3), judged KEEP, Part A headline withdrawn
- Outcome: Part B (single-pin map, clue worths, Markov bounds) reproduces. Part A: "s7-sa-beam beats row scan" withdrawn. Its 10^38.34 is the mass of the first 42 levels; the SMC population dies at a deterministic wall (a cell with no placed neighbour just before a fully closed cell). Row scan from the bottom (10^40.45) is the cheapest validly measured order; s4-sa, s5-sa, s7-sa-beam are undecided (kill line reached for 15 of 18). Model Spearman 0.66 became 0.949 without s7-sa-beam (judge's reanalysis, not a preregistered reading; 0.934 over the 12 orders measured under A8). Prose fix: 10 spread pins do not clear 2^78 by the bound (only k <= 6 does).
- Reproducibility kind: deterministic (17 of 17 judge reruns identical); `just experiments orders-and-pins`, then `python3 -I scripts/analyze.py`.
- Amendment A8 (status rule, written after the outcome, on the judge's ruling; threshold 150 of 251, the judge's, which also demotes spiral-out to bounded below, still "does not beat"; kill line unchanged at 15 of 18; model Spearman 0.934 over 12 orders, judge's 0.949 over 13).
- Commit: see `git log` on night/2026-11-13-d3-orders-and-pins (withdraw commit follows judge).
- Files: research/experiments/orders-and-pins/{README.md,PREREG_AMENDMENTS.md,scripts/analyze.py,results/summary.json,JUDGE.md}

## D1 draft ledger entry

### 2026-11-13: frame-cores (D1), judged KEEP as a measurement
- Outcome: the stage 1 kill holds, cheap refutation of full frames is dead on real H at the preregistered line (lower median R1-rows 0.89, R1-spiral 0.12, R2 1.00; none reaches 100 with a lower end of 10). The stage 2 "pass" reading is correct by the letter of PREREG + A1 but carries no evidence for nogood-driven frame search: it sits on corner-local frames (21 of the 23 near frames have a dead 3x3 clue block), the PREREG's own bound is a mean over frames, and the far stratum is undecided (37 frames, 16 censored, median 75.12, infinite upper end). A1 "below break-even" for arcs is the same corner effect (k = 12: all 18 frames with a dead bottom 3x3 block refuted, against 12 of the other 42); the A2 nested cost (2.2 to 2.5 bits above every budget) governs.
- Corrections applied to the README (five): stage 2 headline demoted; arc readings reconciled; R0-arc refutation times (22 refuted, 21 within 60 s, slowest 107.26 s); counter censoring disclosed (2 real H and 5 realfar frames, 5 of the 29 "no proved nogood" frames in realfar); `results/cal.log` is the driver's whole stderr. No number or analysis changed.
- Judge checks: 36 DFS runs, counts, samples and flags reproduce exactly at HEAD; R2 verdicts and conflict counts identical on 6 frames; preregistration order intact (test started 52 minutes after PREREG, 17 after the last amendment).
- Reproducibility: deterministic counts and DFS, R2 verdicts reproduce; `just experiments frame-cores` (test stage 4.8 h wall on the host, shared 12 threads).
- Commit: 42d2050f on night/2026-11-13-d1-frame-cores (source commit at merge: <fill at merge>).
- Files: research/experiments/frame-cores/{README.md,JUDGE.md,PREREG.md,PREREG_AMENDMENTS.md,scripts/,results/}.
- Open: `cargo fmt --check` fails on the frame-cores engine (not fixed, the code that ran stays as is). Optional bound of the 7 counter-censored frames at b_H <= log2 N_H (about 107.7 bits), median both ways, not done.
