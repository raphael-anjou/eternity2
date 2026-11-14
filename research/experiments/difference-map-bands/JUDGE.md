# JUDGE: difference-map-bands (track 11, night 2026-11-12)

**Verdict: kill.** The preregistered kill rule fires on both arms, (a) and (b),
through the censoring fallback that PREREG.md wrote down before the run. The
band-width exponent was not measured: the kill is of this method at this cap
(beta 0.25, 20,000 iterations) on the primary ladder, and the only rung where
anything is measurable (n = 6) shows no band effect either.

Judged from the branch only (commits 961d1aa to c4b026f), on 2026-11-13.

## 1. Preregistration order

- PREREG.md: commit 9a0c4a3, 2026-11-12 22:58:52 UTC, the file alone. Its
  SHA-256 at that commit and at HEAD is `8f784d3f...cdbc3`; no later commit
  touches it.
- Before it: 961d1aa (Rust port, memo-instance exporter, memo comparison rows
  on the memo's own nine instances; no ladder test seed). After it, 1 s later:
  81ac339 (cycle detection, calibration rows, pipeline, analysis); calibration
  files hold seeds 1001..1010 only. 405c1c7 (bash 3.2 fix) at 22:59:02 UTC.
- `results/run-log.jsonl`: 52 start lines, 52 matching end lines, all exit 0.
  First start 2026-11-12T22:59:06Z, 14 s after the PREREG commit. Every line
  carries the committed PREREG hash, HEAD 405c1c7 and `dirty_paths` 0, so the
  code that ran is the committed code. Each of the 52 part files has exactly
  one start line naming it as `--out`, and no log line lacks its part file.
  The earliest part-file mtime is 23:29 UTC. The run-log seconds sum to
  6.81 core-hours, as the README states.
- No amendments.

## 1b. Track label

Method track. No optimization claim, so the node-sequence hash check does not
apply.

## 2. Rigor check and adversarial lenses (run inline, not fanned out)

- Design: the k = 0 arm is the right control for the band hypothesis (same
  starts of the A replica, cap, step, run seeds). Not an ordering method, so the
  random-order control does not apply. All 30 instances x 3 run seeds are
  present for every (rung, k); no seed dropped, no subsampling. The planted
  board is read only after the run, to fill `equals_planted`.
- Code: A projection = Hungarian on the summed one-hot scores (exact Euclidean
  projection, since every embedding has 4p ones); B = facing-side averaging plus
  grey rim (orthogonal projection on an affine subspace, so P_B of the replica
  mean is the exact concur projection); band DP state = h lane colours plus one
  vertical colour, top of each band free, left rim seeded at state 0, right rim
  enforced by the valid-orientation lists. I found no bug. The memo comparison
  (calib/memo-compare.txt) agrees with the memo's dm.py within start-to-start
  scatter.
- Scope: the README says "a statement about this cap and budget, not a measured
  exponent" and stops at 8x8. The title and the outcome do not overreach.
- Baseline: `bb-mrv` from mrv-baseline, code untouched on this branch. It is
  itself censored at primary n = 8 (0/30), so s' is a lower bound, as PREREG
  says; this does not matter because every s_k is +infinity.

## 3. Recomputed (own script, from results/raw/*.jsonl)

All match `results/summary.json` and the README:

| rung | arm | instances solved | runs solved | KM median | us/iter |
|---|---|---|---|---|---|
| primary 6 | dm-k0 / k1 / k2 / k3 | 21 / 23 / 22 / 15 | 57 / 69 / 64 / 50 of 90 | 12,156 / 9,077 / 7,936 / 14,049 it | 31 / 90 / 173 / 1,040 |
| primary 6 | bb-mrv | 30 | 30 | 460 nodes | |
| primary 7 | dm-k0..k3 | 0 each | 0 / 0 / 0 / 1 | censored | 62 / 186 / 283 / 2,285 |
| primary 7 | bb-mrv | 23 | 23 | 588,305 nodes | |
| primary 8 | dm-k0..k3, bb-mrv | 0 each | 0 | censored | 125 / 373 / 769 / 6,133 |
| 8-c10 | dm-k0 / k1 / k2 | 0 each | 0 / 0 / 1 of 90 | censored | |
| 8-c10 | bb-mrv | 30 | 30 | 95,128 nodes | |

s_0 = s_1 = s_2 = s_3 = +infinity; s' = 0.1165, bootstrap 95% [0.1080, 0.1195]
(my resampler reproduces the committed interval). Kill (a) fires
(+inf > 0.058), kill (b) fires (no finite fall). All 242 solved boards pass my
own independent checker against the instance files.

Rerun on 2 cores, binaries rebuilt from HEAD into a separate target dir: 90
rows (dm-k0 n6 seeds 1-10 x 3 runs, dm-k1 n6 seeds 1-4, dm-k2 n6 seeds 1-2,
the single solved dm-k3 n7 run (s26, run 1, 12,139 it), the single solved
dm-k2 8-c10 run (s5, run 2, 15,315 it), dm-k0 n8 seeds 1-2, bb-mrv n6 seeds
1-30 and n7 seeds 1-4). Status, iterations, DP transitions, best and terminal
mismatches, cycle fields, boards and `bb-mrv` node counts are identical in
all 90 rows.

## 4. Findings

- **No band effect where it is measurable.** At n = 6, paired bootstrap of the
  KM median ratio k / k0 (5,000 resamples): k1 [0.41, 1.36], k2 [0.43, 1.30],
  k3 [0.77, +inf]. Per instance k3 beats k0 on 9 and loses on 13. Bands cost
  3x to 33x more per iteration, so in wall time every k >= 1 loses to k = 0.
- **Censored runs get worse with k**, measured: median best mismatches at n = 8
  are 9, 10, 11, 13 for k = 0..3, and no censored run (of 1,108) cycles.
- **Wall-clock against the baseline.** At n = 6 the KM median wall is 3.3 ms
  for `bb-mrv` against 376 ms (k0) to 14,678 ms (k3). In the easy regime
  (8-c10) the map solved 1 of 270 runs while `bb-mrv` solved 30 of 30.
- **Caveat on the cap (does not change the verdict).** In wall time the DM cap
  is smaller than the baseline's for k <= 2: 20,000 iterations at n = 7 is
  1.2 s (k0) to 5.7 s (k2) per run, against a `bb-mrv` median of 11.2 s and a
  cap of about 18.6 s. A larger iteration cap could give finite n = 7 medians
  at low k. It would not rescue the hypothesis: the band trend at n = 6 and in
  the censored dynamics points the wrong way, and the per-iteration cost of
  wide bands grows fast.
- **Cosmetic.** README censored-run terminal medians round half-integers
  (n = 7: 16.5, 19.5 printed as 17, 20; 8-c10 k1 34.5 printed 35). No effect.

## 5. Site pages

The verdict leans on no explored or dead-end page. The memo the hypothesis
comes from (solve-path-6-free.md) is argued and unrefereed; its numbers were
reproduced here only to validate the port.

## What reopening would need

A different mechanism, not a larger cap: a band projection that lowers the
iterations at n = 6 with a bootstrap interval below 1 against k = 0, before any
scaling claim is attempted.
