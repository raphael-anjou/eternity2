# Judge: frame cores (arc 2026-11-13, track D1)

Verdict: **keep**, as a measurement. The stage 1 kill holds: cheap refutation
of full frames is dead on real H at the preregistered line. The stage 2
"pass" reading is correct by the letter of PREREG + A1 but carries no
evidence for nogood-driven frame search. It sits on corner-local frames,
and the stratum that matters (far from the clue corners) is undecided.
Five corrections to the README are needed (findings 1 to 5 below). None of them makes the
result invalid.

Judged from the branch only, on this Mac, one core at most, `nice -n 10`.
Scripts are in the judge's scratchpad, not committed.

## 1. Preregistration order

- `PREREG.md` has one commit, `4fbbbf10` at 08:00:45 CET. A1 (`929e0e1d`,
  08:12:05), A2 (`0c51a824`, 08:32:30) and the A2 addendum (`3e3e8a62`,
  08:36:20) follow it. The last engine and script change before the test is
  `6817fe47` at 08:37:52. Results are committed in `b7e3b2e8` at 19:20:38.
- `results/run-log.jsonl`: the first test-stage line is
  `2026-11-13T07:53:16Z` (08:53:16 CET), 52 minutes after the PREREG commit
  and 17 minutes after the last amendment. Both start lines (`test`,
  `test-far`) carry PREREG sha256 `88c189b9...`, which equals `shasum -a 256
  PREREG.md` at HEAD. `results/PROGRESS.txt` agrees: test 08:53:16 to 13:41:01,
  test-far 13:41:01 to 14:45:25.
- Every test result file is written by `scripts/test.sh` or `scripts/far.sh`,
  and both ran only under the guard. No test row lacks a matching log line.
- Gap: the host ran a copied tree, so the guard recorded no HEAD and no
  PREREG commit. Exact reproduction at HEAD (section 3) shows the engine,
  sampler and counter that produced the rows are the committed ones.
- Track label is measurement, so step 1b (node-sequence hashes for
  optimization) does not apply.

## 2. Recomputed numbers

These come from my own script over the raw `refuters.jsonl`, `counts*.jsonl`,
`base.jsonl` and flags files. None of `analyze.py` was reused. Point
estimates match `results/summary.json`. My bootstrap intervals use a
different seed and resample count, so their upper ends differ within
bootstrap noise.

| quantity | branch | judge |
|---|---|---|
| node rate r, real (capped R0-rows, n) | 2.40e6 (40) | 2.39997e6 (40) |
| stage 1 real H, lower median R1-rows / R1-spiral / R2 | 0.89 / 0.12 / 1.00 | 0.892 / 0.123 / 1.000 |
| R2 interval | [1.00, 26.49] | [1.00, 34.26] |
| censored pairs R1-rows / R1-spiral / R2; plain censored | 24 / 22 / 14; 25 | 24 / 22 / 14; 25 |
| M_H (median b_H + log2 C), real H | 36.75 [32.90, 53.19] | 36.75 [32.90, 53.19] |
| censored frames, median R2 core | 16, 5 | 16, 5 |
| log2 mean of 2^v (largest A2 floor) | 71.94 | 71.94 |
| near / far frames, median v | 23 / 37, 29.83 / 75.12 | 23 / 37, 29.83 / 75.12 |
| realfar median v, censored, core, log2 mean | 86.57, 29, 17.5, 89.69 | 86.57, 29, 17.5, 89.69 |
| real U median v | 14.43 | 14.43 |
| beta_H real | 17.24 | 17.24 |
| arcs R2 unsat (all within 60 s), k = 15/14/13/12 | 42/41/34/30 | 42/41/34/30 |
| A2 nested log2 nodes vs budget, k = 15/14/13/12 | 27.7/31.4/35.4/39.5 vs 25.5/29.0/32.9/37.0 | 27.68/31.37/35.39/39.49 |
| null H median v 1101 / 1102 / 1103 | 41.92 / 91.52 / 37.07 | 41.98 / 91.50 / 37.06 |

The null rows differ in the second decimal because I used a per-board node
rate and the branch uses the pooled null rate (2.53e6). The branch's rate
follows PREREG ("same puzzle class").

## 3. Rerun at HEAD (engine built locally, single core)

- `fc real` and `fc gen 1101`: `real.puz` and `null1101.puz` are
  byte-identical.
- `fc sample`: real H (seed 20261113, 60), real U (20261114, 30), null 1101
  H and U, and the 240 realfar candidates (20261115) are all byte-identical
  to the committed frames.
- `fc flags` on the 90 real frames: byte-identical.
- `fc count`: `base.jsonl` (N_F, N_H, 10 reps each) and all 480 arc counts
  (seeds 51, 52) match every replicate exactly.
- `fc dfs`: 36 decided runs over real, realfar, null 1103 and the k = 15 arcs
  (R0-rows, R0-spiral, R1-rows, R1-spiral, R0-arc; up to 3.7e7 nodes).
  Verdict, node count and read set match exactly on all 36, with 0
  mismatches.
- R2 (`scripts/sat.py`, PySAT 1.9.dev22 here against 1.9.dev15 on the host),
  on 6 real H frames (the 4 with the most conflicts and 2 from the middle):
  verdicts and conflict counts are identical (374, 373, 267, 158, 12, 11).

## 4. Findings

### Reshapes the reading (README must change its headline wording)

1. **The stage 2 "pass" says nothing about a frame search.** The PREREG's own
   arithmetic bound is a mean over frames, price >= E_f[C_f 2^(b_f)]. The
   reading line uses the median instead. On real H the median falls on the
   corner-local half: 23 frames are "near", and 21 of them have a dead 3x3
   clue block (29 of 60 H frames have one). Those frames die by 2- to
   3-cell nogoods at about 30 bits. The far stratum (37 frames, 16
   censored) has median 75.12 with an infinite upper end. The realfar set
   (no dead 3x3 block) has a far median that is infinite (more than half the
   frames have no proved nogood). The A2 floors are lower bounds only
   (censored frames enter at log2(600 r) = 30.4 bits), so "fail line not
   met" is not evidence of passing. The README reports all of this. But the
   bullet heading "the pass line is met" should be demoted to: met by the
   letter of PREREG, driven by corner-local frames, and the far stratum is
   undecided. It is not evidence that nogoods beat the 134.4-bit row scan.
2. **Arc A1 "below break-even" is the same corner effect.** At k = 12 all 18
   frames with a dead bottom 3x3 block are refuted, against 12 of the other
   42. Every R2 arc refutation takes 1.4 s or less at k = 12 and 13. At
   k = 13, the 30 frames given a 6400 s cap gained no refutation past the
   first second. A2's nested cost (no level below break-even) is the
   reading that accounts for this. The README states both readings and
   should say which one governs.

### Integrity (fix in the README; no rerun needed)

3. **False sentence.** "No arc refutation took longer than 60 s at any k
   (unsat count equals the within-60 s count)" is wrong for R0-arc: 22
   unsat, 21 within 60 s, the slowest 107.26 s (`realH*k15`, R0-rowsup-partial
   rows; `summary.json` `arcs["k15/R0-rowsup-partial"]` shows unsat 22,
   within 60 s 21). It is true for R2 only.
4. **Undisclosed censoring by the counter.** In `analyze.py` a nogood
   whose SMC H-count comes back null (no particle survived) is dropped. A
   frame left without a usable nogood then counts as censored (v = inf),
   although a refuter proved it. This affects 2 real H frames (realH11,
   realH24) and 5 realfar frames (realHx35, 73, 76, 97, 124). It is
   conservative for the median and harmless to the readings on real H. But
   it accounts for 5 of the 29 "no proved nogood" frames in realfar. The
   README's "more than half have no proved nogood" is therefore partly a
   counter failure, not a refuter failure. Fix: state it. Optionally bound
   these frames at b_H <= log2 N_H (about 107.7 bits) and report the median
   both ways.
5. **`results/cal.log` is the driver's whole stderr, not the calibration
   log.** It holds the sampler lines of the test and realfar stages,
   including far.sh's "60 kept". The README's "calibration ... its cause is
   not recorded" note should say what the file is.

### Checked and clean

- No dropped seeds: 90 / 30 / 30 / 30 / 60 frames with 5 refuter rows each,
  240 R2 arc rows and 60 R0-arc rows. Realfar keeps the first 60 of 240 by
  the committed rule. My rerun reproduces the 240 candidates.
- No `sat` on any test frame or arc. No planted-solution leakage path: null
  frames are SMC samples, and refuters see only the scrambled puzzle.
- Censoring follows PREREG in both stages. No censored run is counted as a
  win. In stage 1, both sides censored gives ratio 1.
- Node accounting is the same across R0 and R1 (same `dfs` binary, same
  counter). The stage 1 statistic is wall time, so the R1 matching cost is
  charged. Converting R2 seconds into DFS nodes through r is preregistered.
  It is an exchange rate, not a node count. The README says so in Method.
- Null boards: the corner-local cheapness reproduces (near medians 31.4,
  29.0, 32.2), so it comes from the clue-corner structure, which the
  framed generator shares. It is not a quirk of the real pieces.
- Stop rule: test ended 13:41, before 13:45. Realfar ran under the A2
  addendum, which was committed before any test run.

## 5. Site pages

This verdict does not rest on the explored or dead-end pages. The 10^45.9
frame price, the 134.4-bit row-scan price (Track 10), the 17.45-bit memo
beta_H and the 53% dead-3x3 share are reported numbers from other tracks or
memos. beta_H = 17.24 and the 48% (29/60) dead-3x3 share are measured here.

## Unplanned observations

- R2 on full frames is bimodal. Every refutation finishes in at most 0.35 s
  (real H, 46 frames) or 0.91 s (realfar, 36 frames); every other frame hits
  600 s. Arcs behave the same way. A CDCL solver either sees a local
  contradiction at once or does not finish, which is a stronger statement
  than the medians.
- On uniform frames (U), R2 loses to plain search: its lower median ratio is
  0.0, because those frames die in a handful of DFS nodes while R2 takes tens
  of milliseconds per call.
- The host ran about 1.1 to 2.3 times slower per run than this Mac on the
  same node counts (10 jobs on 12 threads). r is a loaded-host rate, as
  PREREG says.
