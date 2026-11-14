# five-clue-quota: amendments

## 1. Diagnostic row for G-on-flagged (2026-11-13, after the gates, before any test run)

What: one extra gate-stage row, `gate-on-2-9-11-noquota-1e8-s401` (keying
(2, 9, 11), fix on, no quota, cap 1e8, Track 14 seed 401, not a test seed).
Why: G-on-flagged requires each flagged row to leave the opening or be
explained; `gate-on-2-9-11-s401` (fix on, Q0) stayed at depth 18. The row
tests whether the quota holds it there. It does not (deepest 18 without any
quota too), see README. No test-stage choice depends on it.

## 2. Deconfounding arms and reading R5 (2026-11-13, after calibration, before any test run)

What and why: the lead relayed the Track 14 judge's finding F1 (JUDGE.md at
ee5391c8 on the Track 14 branch): in Track 14 the default keying ran with the
unscaled quota (119 of its 150 privileged sides by cell 160, 79.3%) while
every one-ring keying ran with the scaled quota (T - 3 of T, about 97.5%), so
keying and quota slack are confounded. Checked in Track 14's rows (measured):
`conf-D-s401` has `qscale` 1 and `quota` true with `--priv 22,21,20` and no
`--quotascaled` (unscaled); `conf-1-7-9-s401` has `qscale` 0.983 and
`conf-5-7-20-s401` 1.0, both with `--quotascaled`. The reference arm
D-off-Q0 is exactly Track 14's working arm (default keying, unscaled quota,
fix off); G-off reproduced its row.

Added arms (each fix off and fix on, 16 openings, same seeds and hosts):
- **D-off-QS, D-on-QS** (arm a): default keying with the scaled quota
  (`--quotascaled`, factor 147/119: 147 of 150 by cell 160, same curve shape).
- **P-off-QM, P-on-QM, K1-off-QM, K1-on-QM** (arm b): one-ring keyings with a
  slack-matched quota, new switch `--quotaslack`: the published curve times
  T / 150, so every threshold asks for the share of T that the unscaled curve
  asks of the default's 150 sides, up to integer truncation: 96 of 122 by
  cell 160 for P (78.7%), 95 of 120 for K1 (79.2%). `--quotaslack` with the
  default keying gives the unscaled curve exactly (checked with
  `--printquota`). The engine change is a new off-by-default switch; G-off is
  rerun on this Mac with the new binary (stage gate, rows `gate2-off-*`) and
  each remote host reruns its G-off row before its first test job.

Reading **R5** (fixed now, before these arms run), on the fix-off and the
fix-on pair separately, then stated jointly:
- "slack explains the result" if arm (a) has zero legal nodes at depth >= 201
  on at least 12 of 16 openings while some arm (b) (P or K1, same fix level)
  has a positive count on at least 8 of 16;
- "keying explains the result" if arm (a) has a positive count on at least 8
  of 16 and every arm (b) at that fix level is zero on every opening;
- otherwise "both or neither", stating which: "both" if (a) loses openings
  against D at the same fix level (fewer positive openings than D-<fix>-Q0)
  and some (b) gains at least one positive opening against its Q0 twin,
  "neither" otherwise.

Correction of cause (b) (judge F4): flagged runs lose legality in the top
border row (depth 1 to 8). Located in the code before the fix ran: in the
no-break path the top row's candidates are keyed on (grey top, left colour)
and the right side is checked only at the last column and the bottom side
only when the cell below is filled, so a corner piece (grey top and grey
right) can be placed mid-row, showing grey toward the next cell; the
following cells then match grey like a colour. `--orientfix` refuses it
(`wrong_class` checks all four sides). The analysis reports, for every
stranded fix-off test run, the depth of the first illegal placement (its
deepest legal depth, the cell where the illegal piece went).

Reduction (calibration, PREREG rule): calibration rows (seed 1 on dell, seed 2
on mac-m1, 26 arms) took 20.9 to 56.8 s each. Projected with 32 arms:
dell about 1,600 s per seed x 8 / 12 threads = 18 min, mac-m1 about 1,080 s x
5 / 7 = 13 min, this Mac about 1,600 s x 3 / 4 = 20 min. Start about 04:10,
end before 05:25: **no reduction**. Reduction order if needed later (lead):
K1 everywhere, then Q3, then 10 openings; never D-off-Q0 or arms (a), (b).
