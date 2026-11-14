# Judge: mrv-scaling-pilot (Track 1, night 2026-11-12)

Branch `night/2026-11-12-t1-9x9-pilot`, judged at 6762e14. Judged from the
branch only.

## Verdict: kill

The preregistered rule fires on both conditions (K1: M = 0.000 >= T = -0.83215;
K2: U = 6 > 4). Read the kill as the PREREG words it: "no evidence the ratio
keeps shrinking" at 9x9. It is **not** evidence that the ratio stopped
shrinking. K2 is the operative reason: 6 of 8 pairs are both node-censored at
1e8, so they are scored as ties and pull the upper median to 0 mechanically.
The pilot ran out of cap; it did not measure the ratio. Both informative pairs
favour bb-mrv (seed 103 r <= -0.106, seed 106 r <= -1.001, both upper bounds).

## 1. Preregistration and ordering

| event | time (CET) | source |
|---|---|---|
| PREREG.md committed | 18:51:02 | `git log --follow PREREG.md`: one commit, 012cfa1, never changed |
| A1 committed | 18:52:38 | 27d58c3 |
| build check committed | 18:52:44 | 620b1d3 |
| first test launch (s101, mac-m4) | 18:52:45 | row `started_at`, `heartbeat-mac-m4.log` |
| dell launches (s103..106) | 18:56:25 to 18:56:31 | rows, `heartbeat-dell.log` |
| mac-m1 launches (s107, s108) | 18:59:09 to 18:59:11 | rows, `heartbeat-mac-m1.log` |
| first result row committed | 19:19:49 | 53cfc7d (bb-mrv s106) |

Every result commit follows the PREREG commit, and every row's own
`started_at` follows it too (earliest 1 min 43 s after). PREREG and the first
launch are close in time, so I checked the row timestamps against the commit
as the protocol asks: consistent.

**Finding (protocol, not invalidating): no `results/run-log.jsonl`.** The runs
did not go through `research/experiments/common/prereg_guard.py`; that file is
absent from this branch (base `origin/main` 6247af0). So all 16 test rows have
no matching guard log line and no recorded PREREG hash. The README declares
this. I do not grade it invalid because (a) the guard did not exist where the
runner worked, (b) PREREG.md has one commit and no later edit, so there is only
one hash a guard could have recorded, (c) row `started_at`, heartbeat launch
lines and commit times agree and all post-date the PREREG commit, and (d) the
outcome is a kill, the direction a gamed prereg would not produce. The fix for
future tracks: base the branch on a commit that contains `prereg_guard.py` and
route the `test` stage through it.

Amendments: A1 (three machines, dell at SLOTS=8) and A2 (aborted duplicate
launch on mac-m1, killed after about 30 s with no row written) leave the
statistic, cap and censoring unchanged. A1's header says "about 19:00 CET" but
it was committed at 18:52:38; the commit time is the one that counts, and it
precedes every launch. A1's optional cross-machine re-run was not done; my
rerun below covers that gap for one run.

## 1b. Track label

Measurement. No engine, `mrv-baseline` or `hard-ladder` file differs from
6247af0 (`git diff --stat 6247af0 HEAD` outside the experiment directory shows
only `research/experiments/justfile`). My fresh `cargo build --release
--locked` of `../mrv-baseline/compute` gives sha256 `9a914907...`, identical to
the mac-m4 rows. No wave switched to a Track 0 binary. No node-sequence gate
applies.

## 3. Recomputed numbers (own script, from the four committed wave files)

16 rows, 8 seeds, both arms per seed, n = 9, c = 11, cap 1e8, `error` null on
all. No dropped seed, no subsampling.

| seed | machine | bb-mrv | scb | r (log10) |
|---|---|---|---|---|
| 101 | mac-m4 | censored 1e8 | censored 1e8 | 0 |
| 102 | mac-m4 | censored 1e8 | censored 1e8 | 0 |
| 103 | dell | solved 78,349,212 | censored 1e8 | -0.1060 (bound) |
| 104 | dell | censored 1e8 | censored 1e8 | 0 |
| 105 | dell | censored 1e8 | censored 1e8 | 0 |
| 106 | dell | solved 9,970,108 | censored 1e8 | -1.0013 (bound) |
| 107 | mac-m1 | censored 1e8 | censored 1e8 | 0 |
| 108 | mac-m1 | censored 1e8 | censored 1e8 | 0 |

- T = -1.9160979637943325 / ln 10 = -0.832151 (A22 `Q1 R8 8x8 c=10 s21-50`
  and `Q2 n=8 c=10`, `median_ln_ratio`, checked in
  `../mrv-baseline/results/summary.json`).
- M (5th smallest) = 0.000; U = 6; K1 and K2 fire.
- Bootstrap (10,000 paired resamples, seed 20261111): [-0.106, 0.000].
- Core-hours (sum of `wall_ms`): 27.37.
- Paired wall ratio bb/scb: 0.096 to 1.551.

All match `results/summary.json` and the README.

### Reruns on this Mac (2 cores, fresh build)

- `pilot_instances check`: pass (instance file equals the generator; A22 row
  n9-c11-s1 reproduced; no pilot seed in the A22 ladder).
- Build check, plus2 7x7 seeds 1..10, cap 1e6, both arms: 20 of 20 rows equal
  A22's committed `nodes` and `status`. The committed dell and mac-m1 check
  files also match A22 (0 mismatches each).
- **bb-mrv seed 106, cap 1e8: solved, 9,970,108 nodes, 9,970,027 backtracks,
  651 s.** Identical to the dell row (9,970,108 / 9,970,027). This is the
  cross-machine (x86 Linux vs arm64 macOS) determinism check A1 left undone.
- scb seed 106 at cap 1e7: censored at 10,000,000 (native 10,000,013), so no
  solution in the first 1e7 nodes, consistent with the deterministic 1e8
  censored row. The full 1e8 scb rerun (about 1.5 h) was not done.

## 2 and 4. Rigor and adversarial review (inline, four lenses)

**Design.** The statistic scores both-censored pairs as 0, so a pilot whose cap
is too low for one arm returns M = 0 by construction and trips K1 whatever the
true ratio. The prereg anticipated this with K2, which is the condition that
actually carries the verdict. The baseline is the A22 scb arm with one run seed
(seed 1) per instance; scb is randomized, so each r_i is conditional on that
seed. A22 did the same, and here no number is compared with a different scb
seed, so this is a limitation, not a bias. The 8x8 reference was taken at cap
1e6 with 10 of 30 scb runs censored; the prereg states this caveat.

**Node accounting.** Both arms count through `NodeCounter::try_assign` with the
same cap; scb overshoots natively by 16 to 132 nodes and the row's `nodes` is
clamped to the cap, which the prereg allows. Equal accounting.

**Leakage.** The instance rows carry the planted board (A22 format), but
`equals_planted` is false on both solved runs and the solvers are the
unchanged A22 code that A22's judge already passed. No sign of leakage.

**Wall clock.** bb-mrv is slower per node on every machine (11.3k to 18.6k
nodes/s against 12.1k to 28.0k for scb; on both-censored pairs it needs 1.12
to 1.55x scb's wall for the same 1e8 nodes). No node win hides here since there
is no node win claimed. The README's further point, that plain row-major DFS
solves these 9x9 boards in seconds at roughly 1e8 nodes, is **reported** (from
`growth-law.REVIEW.md` on the lead branch), not measured here, and the README
labels it so. My verdict does not lean on it.

**Scope and prose.** README claims match the evidence: no scaling claim, kill
worded as "no evidence", generated boards only, cross-machine wall times not
compared. Deviations (no run-log, heartbeat file name, skipped cross-machine
rerun) are disclosed. No em dashes in README or PREREG.

## What would make a 9x9 ratio informative

The pilot shows the cap, not the ratio, is binding: scb censored 8 of 8 at
1e8. A follow-up needs either a cap of about 1e9 for scb (about 14 to 23 h per
run at the measured rates) or a scoring rule for both-censored pairs that is
not a tie (for example Kaplan-Meier medians per arm). Given that plain DFS is
reported to be three orders of magnitude faster per node at this size, a node
ratio between these two solvers is unlikely to bear on solving; I would not
spend the 30-seed run on it.
