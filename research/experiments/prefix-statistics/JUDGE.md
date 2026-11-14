# JUDGE: prefix-statistics (arc 2026-11-13, Track D4)

Verdict: **keep**, as a measurement, with the scope below. The headline the
branch may carry is: on 48 planted 16x16 boards with the official colour
census, the census statistic S6 (and S9, the same finding) puts the planted
prefix about 2 sd above the consistent prefixes at depths 144 and 160, beyond
what one row of lookahead or subtree size explains; this refutes the lead's
claim "no cheap statistic has |delta| >= 1.4 at the widest depths" on planted
boards; and it is **not a usable pruning gain** (about 1 bit for one scan
direction at depth 144, losses for two or four directions). No transfer to
the official puzzle is shown.

Judged from the branch only (commits `e9ecd977` to `e91ae3ef`). Every number
below was recomputed by the judge's own scripts from the committed raw rows,
not read from `summary.json`.

## 1. Preregistration order and run log

- `PREREG.md` committed once, `e9ecd977` at 09:14:27 CET (08:14:27 UTC), never
  edited after; the working-tree file hashes to the committed SHA-256
  (`a6a28064...`).
- `results/run-log.jsonl`: 119 start lines, 115 end lines. Every start line
  carries the committed PREREG hash and commit; `dirty_paths` is 0 on all 119
  (the code that ran was committed code). First guarded line 08:20:33 UTC
  (gate), first test-stage line 09:13:27 UTC, both after the PREREG commit.
- All 48 test files have exactly one matching test-stage log line at head
  `5d2b1d4e` (after the A5 fix), `--np 500000`, exit 0. The four start lines
  without an end line are the two A5 stray launches and the two boards killed
  by the A6 stop (1113049, 1113050); their files are in `results/stray/`,
  disclosed, and enter no reading.
- Amendments A1 to A4 are committed before the first test run (A4 at 09:05
  UTC, test at 09:13 UTC). A2 relaxed G-exact eligibility after the gate had
  failed; it was disclosed, made before any test data existed, and the
  original reading is kept printable (`POP_FLOOR=0`). Accepted.
- Track label is measurement; no node-sequence hash check applies (no
  optimization claim).

## 2. Reproduction

- `psx` rebuilt from the committed source into a separate target dir; test
  board 1113029 rerun at the registered settings (N = 5e5, m = 1,000, four
  directions): **1,367 non-timing fields compared, 0 differences** against
  `results/test/board-1113029.jsonl`.
- `scripts/gates.py results/gate-a2` reprints G-exact PASS (bias 440/440, sd
  ratio 438/440) and G-rep PASS (51/54).

## 3. Recomputed numbers (judge's script, 48 boards, 100,000 board resamples, level 0.99833)

Validity (population >= 1,000, family ESS >= 200): 182/192 at 144, 189/192 at
160, 157/192 at 176 (18% invalid, so 176 is descriptive). Matches the README.

| statistic | 144 node | 160 node | 144 work | 160 work | survivors 144 | survivors 160 |
|---|---|---|---|---|---|---|
| S4 | +1.32 [0.93, 1.71] | +1.46 [1.12, 1.81] | +1.48 [1.08, 1.88] | +1.54 [1.20, 1.90] | +1.54 | +1.73 |
| S6 | **+1.96 [1.52, 2.43]** | **+2.15 [1.74, 2.56]** | +1.67 [1.22, 2.13] | +1.96 [1.57, 2.37] | +1.74 [1.32, 2.20] | +1.76 [1.38, 2.14] |
| S9 | -1.78 [-2.22, -1.35] | -1.98 [-2.39, -1.58] | -1.72 [-2.17, -1.29] | -1.93 [-2.34, -1.54] | -1.79 | -1.93 |
| S8 | +1.71 [1.57, 1.87] | +2.37 [2.14, 2.59] | +0.10 [-0.07, 0.29] | +0.64 | n/a | n/a |
| S5 | -0.21 | -0.19 | -0.13 | -0.18 | | |
| H0 | -0.13 [-0.37, 0.12] | +0.05 [-0.16, 0.27] | | | | |

All agree with the README to the second decimal (bootstrap ends within 0.01).
S6 per board: median +1.78 (144), +2.20 (160); boards below 0.7: 10.4% and
4.2%; boards below 0: none (minimum +0.41 and +0.16). At row level (single
direction) 7.7% and 7.9% of rows have S6 delta below 0. Per-row correlation
of the S6 and S9 deltas: -0.93 to -0.95 (96 to 160), so S9 is not independent
evidence. Positive control at 176: S8 +3.34 [3.02, 3.68], S10 +10 (clip):
pass. Size-bias table (`results/size-bias.txt`) reproduces.

Value reading for S6 (point estimates recomputed, independent code): depth
144, K = 1, p >= 0.5: **G = 1.02** (f = 0.063); K = 2: -0.04; K = 4: -1.88.
Depths 96 and 128: G = 3.43 / 9.73 / 10.06 and 2.34 / 3.49 / 0.17, but
**100% of rows at 96 and 98% at 128 have work ESS below 30** (median 2.9 and
13.1).

## 4. Rulings on the points the runner left to the judge

**(a) Decoy control (negative control 2) fails its registered rule in one
cell (S10 at 160).** Recomputed: mean mid-rank minus 0.5 = -0.0038, interval
[-0.0041, -0.0035]. Cause, verified: 0 of the 192 decoys at 160 have S10 > 0,
while the sampled populations make 1.45 such decoys the expected count; the
chance of seeing none under a correct pipeline is exp(-1.45) = 0.23. With no
rare decoy drawn, every row sits at the same tie value and the percentile
bootstrap collapses to a point. This is a property of the bootstrap on a
point-mass-plus-rare-tail statistic, not of the replay path or the reference
computation, which the other 29 cells, the hash control and the per-step
replay assertion all exercise. By the letter of PREREG section 8 the readings
are suspended; the judge rules the failure uninformative and lets the
measurement stand. This ruling is the judge's, it is post hoc, and it is
recorded as such. A1 was written knowing this cell degenerated on the pilot
and still left the rule exposed to it; that is a design defect for future
tracks, not a reason to discard 29 passing cells.

**(b) The value rule's "usable: yes" (A4 change 5).** Rejected. It is reached
only at depths 96 and 128, where nearly every row's work reference has an
effective size under 30, the floor A4 itself sets for every work-weighted
reading; the value rule omitted the floor, which is a registration gap. A
G of 10 bits at 96 with K = 4 directions against a K = 1 cap of 17 is the
signature of percentiles set by two or three heavy particles. At the only
depth with an adequate reference (144, median ESS 59) the lower end is 0.72,
below the 1-bit line. The registered answer is therefore "not usable", as the
README reads it.

**(c) Painter control fails (10 of 18 cells).** Confirmed from
`results/painter/painter-reading.txt`. The direction matters: where planted
and other solutions differ, the planted prefix is less tilted, so planting
does not manufacture the tilt; but the rule failed, and it only gates transfer
claims, none of which is made. Kept as stated.

**(d) A6 hard stop at 48 boards.** Guarded runs to the stop: 4.18 core-hours;
48 more boards at the measured 251.6 s: 3.35; total 7.53 guarded. The rule
triggers (projected > 8) only by adding the runner's estimate of about 0.5
unguarded pilot and probe hours, which the log cannot verify. The stop is
discretionary at the margin. It does not affect the verdict: at B = 48 every
decisive S6 interval clears 0.7 by 0.8 or more, and the measured sd (0.91 to
1.01) is within the 1.10 that B = 48 tolerates.

## 5. Adversarial checks

- Planted-solution leakage: the statistics are computed by `Ctx`, built from
  the scrambled pieces and the clue cells only (`Ctx::new(n, &p.pieces,
  &clue_pos)`); the planted placement is used only to replay the true prefix
  and to fix the five clues, which true and sampled prefixes share. No leak
  found.
- Unequal accounting: the true prefix and the decoy go through the same
  replay and `ctx.all` path; the reference is m = 1,000 uniform draws from one
  SMC population, as registered.
- Silently dropped seeds: none. Test seeds 1113001 to 1113048 are contiguous
  and complete; 1113049 to 1113096 not run (A6, disclosed).
- Null model: the README states plainly that the tilt is what the random
  model predicts (size-bias of a prefix with a completion), measured at 1.5 to
  1.8 times the first-moment prediction. The finding is a quantity, not an
  anomaly. G-exact's SMC-free exact deltas on 6x6 (S6 +1.0 to +1.7 at depths
  14 to 23) agree in sign.
- Node win hiding a wall-clock loss: the value reading is exactly that check,
  and it says the gain is about 1 bit at best before paying 33 ns per
  evaluation; not usable.
- Claims past the evidence: see section 6.

## 6. Required corrections before any site or ledger use

1. README "Result in one paragraph": "by the A4 definition it is a usable
   census tilt at 144 and 160" contradicts the value conclusion two lines
   later. Write "passes Readings 1, 2 and 3" and keep the word "usable" for
   the value reading, whose answer is no.
2. README "Read in this order" lists A1 to A5; A6 exists.
3. State the judge's ruling on the decoy cell (section 4a) wherever the
   readings are quoted, since by the registered rule they are suspended.
4. Never quote the 96 and 128 value cells as gains.

## 7. Grading of claims the README leans on but did not measure

- Reviewer's prediction "shift / sd^2 about 1, calibration 1.22 to 1.56":
  reported (design review memo, not on this branch).
- "Gap of about 60 bits the review cites": reported.
- The survey formula for pruned-search bits: argued, unrefereed (PREREG says
  so).

## Unplanned observations

- Arc consistency (S7, no search, tile uniqueness ignored) refutes all 1,000
  sampled prefixes in 189/192 rows at 160 and 192/192 at 176 and never the
  true prefix (`results/ac-observation.txt`, reproduced). Whether a
  propagating scan pays for 0.5 to 0.8 ms per node is unmeasured; the
  proposed follow-up track is the right next step and bears more on solving
  than the census tilt does.
- Opposite scan directions carry nearly the same census information at 128
  (r = 0.88), adjacent ones almost none; multi-direction schemes built on S6
  should pair adjacent scans.
- The decoy defect generalises: any registered rank-bootstrap control on a
  statistic that is a point mass plus a rare tail can fail by ties with
  probability of order exp(-expected rare draws). Future controls should use
  an exact test on the rare-tail count instead.

Judge scripts (not committed, stdlib or `uv run --with numpy`): run-log
check, independent delta and bootstrap recompute, value-reading recompute,
decoy tie analysis, rerun field comparison.
