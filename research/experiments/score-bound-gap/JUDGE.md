# Judge: score-bound gap (Track 8, night 2026-11-12)

Verdict: **keep** (stage 0 survives). K1 does not fire and K2 does not fire,
both recomputed independently from the committed rows. The pass sits exactly
at the K1 threshold, and the size trend goes the wrong way. Both are stated
plainly in the README. One README sentence claims more than the data shows
(see "Corrections").

Judged at branch head d5f2075, by a session that did not run the experiment.

## Preregistration order

- PREREG.md has a single commit, cd042da (2026-11-12 20:42:27), and it comes
  before every results commit. The calibration rows were committed in a3fba62
  (20:44:32). The first test rows were committed in f8087cd (20:54:17).
- **No `results/run-log.jsonl`.** `prereg_guard.py` was committed on the lead
  branch at 20:41:47 (e254e3f), one minute before this PREREG. It was never on
  this branch, whose base is 6247af0 (2026-11-11). This is a finding: no test
  row has a guard log line, so the order rests on commit order and file birth
  times. File birth times in the worktree: PREREG.md 20:42:17, every
  `results/test/prefixes-*` and `pools-*` file 20:44:07 or later, and every
  measure file later still (rows computed remotely and copied in). No test
  file predates the PREREG commit. This matches the precedent set for Tracks
  1, 2, 5, 6 and 7, so it does not invalidate the verdict.
- Code after PREREG: main.rs gained a `--source` filter (a3fba62) and nothing
  else in `compute/` changed. analyze.py was first committed in a3fba62, before
  any test row. Its later edits (983fed7, 3f1fd59, 31fee35, 53d0367) add
  tables and an exploratory bootstrap, and leave the K1/K2 logic as it was in
  a3fba62.
- Track label: measurement. No search's node sequence changes, so the
  optimization hashing rule does not apply.

## Numbers recomputed (my own script, committed raw rows)

- 8,063 prefixes in `results/test/prefixes-*`. All 8,063 were measured
  exactly once: 0 duplicate keys, 0 unmeasured, 0 extra. No seed or stratum
  was dropped silently. Strata with fewer than 240 rows have small pools, and
  the pool logs report them.
- Soundness: 0 rows with bound(W) < truth, 0 with a bound above the trivial
  bound, and trivial = prefix score + remaining edges on every row.
- Decision stratum (n7-c6, r = 3, 240 prefixes, 239 finished, 1 censored:
  n7-c6-s20 strict draw 3, truth_ub 75, bounds 84/83/82/81):

  | W | 1 | 64 | 512 | 4096 | trivial |
  |---|---|---|---|---|---|
  | M_lo = M_hi | 7 | 5 | 5 | **4** | 7 |
  | share with gap <= 4 | 0.013 | 0.19 | 0.39 | 0.69 | 0.004 |

  K1: M_hi(4096) = 4 <= 4, so it does not fire. K2: 7 - 4 = 3 > 1 under both
  variants, so it does not fire. In rank terms the median of 4 is not fragile:
  166 of 240 prefixes have a gap of at most 4, so 47 of them would need a
  larger gap to move it. The integer statistic, though, sits exactly on the
  threshold.
- Trend at W = 4096 (M_hi): at r = 2 the gap is 1, 2, 4 at critical colours
  and 1, 2, 3 at critical + 2 (n = 6, 7, 8). At r = 3 it is 3 and 4 at
  critical, and 3.5 and 5 at critical + 2. At 8x8 r = 3 the M_lo median is a
  valid lower bound on the true median, so "at least 5" holds.
- No decision prefix can reach max - 4. The largest truth (or truth_ub) is
  79 = 84 - 5. The median true deficit is 10 (schedule) and 7 (strict).
- Compute: 30.95 thread-hours, summed from the per-row wall times (mac-m1
  12.45, dell 18.50), against the 10 to 15 preregistered. This is a budget
  overrun with no effect on any count.
- `python3 -I analysis/analyze.py` regenerates `results/summary.json` byte for
  byte.

## Reruns (this Mac, Apple silicon, at most 2 threads)

- `sbg selftest 40`, built from the committed source: 240 cases pass, and the
  loose-case counts match PREREG.
- Prefix sampler, n7-c6 schedule and strict (4 runs x 1e6 steps x 30
  instances): output **identical** byte for byte to the committed prefix
  files.
- Full decision stratum re-measured (240 prefixes, including the censored
  one at the 1e9 cap): **240/240 identical** in truth, truth_ub, oracle
  nodes, k_infeasible, and every W's bound, DD nodes, transitions and merge
  counters. The committed rows came from mac-m1.
- n8-c8 r = 1, 2, both sources, rows from dell (x86_64): **293/293
  identical**. The counts are deterministic across machines.
- Independent exhaustive search (my own Python, no repo code) on every r = 1
  prefix at n <= 7 (837 prefixes): the oracle's truth matches on all 837, and
  the W = 4096 bound equals the exact optimum on all 837 (the diagram never
  merges with one row left).
- Independent prefix checks on all 8,063 prefixes: structural validity,
  distinct pieces, prefix score, remaining edges. Strict prefixes have 0
  mismatches. Schedule prefixes mismatch only at the scheduled break cells.

## Adversarial review

- **Leakage.** The bound and the oracle never read the planted board. The
  planted maximum appears only as a target value in the pruning table.
- **Oracle direction.** A witness that fails rescoring panics the run, so a
  reported truth is always reachable. An invalid oracle prune could only
  lower the truth, which would inflate the gap against the pass. Code review
  of both lower bounds (colour counting and the top check) found them valid.
- **DD validity.** The proof in PREREG holds as written in `dd.rs`: the
  availability rule uses `some` only when |some| equals the number of filled
  cells, and merges take union, intersection, union, max. The soundness gate
  agrees empirically.
- **Weak control.** The median of (trivial - W=1 bound) is 0 on every stratum
  with r >= 2. In practice W = 1 is the trivial bound. So K2 measures the
  diagram against no bound at all. That was preregistered, but "3 edges better
  than the counting control" means "3 edges better than nothing".
- **Population.** Every decision prefix is deeply dead (deficit at least 5).
  The gap near the optimum, where a branch and bound with a good incumbent
  spends its time, is not measured. The README says so.
- **Cost.** A W = 4096 bound costs a median 1.3e6 transitions (about 0.2 s) at
  7x7 r = 3. Nothing here says it pays inside a search. Stage 1 must show
  that, and the README says so.
- **Claims past the evidence.** One, below.

## Corrections required (wording, no verdict change)

1. README, size section: "With many rows left the diagram adds little: at
   r >= 4 on 6x6 and 7x7 its median gap is within one edge of the trivial
   bound". This holds at the **critical** rungs only. The direct per-prefix
   median of (trivial - W=4096 bound), which needs no truth, is:
   - n6-c5: 1 at r = 4, 0 at r = 5.
   - n7-c6: 1 at r = 4, 0 at r = 5 and 6.
   - n6-c7: **3** at r = 4, 1 at r = 5.
   - n7-c8: **3** at r = 4, **1.5** at r = 5, 0 at r = 6.
   - n8-c8: 2 at r = 4. n8-c10: 3 at r = 4.

   Restrict the sentence to the critical colour counts. Better, quote this
   direct difference instead of comparing censored M_lo medians. The same
   unrestricted sentence appears in the lead's STATUS entry at 00:28 and must
   not be carried into the ledger or a site page as written.
2. README Outcome: say that the W = 1 control equals the trivial bound at the
   median on every stratum with r >= 2.

## Scope of the keep

A width-4096 relaxed DD bounds the best completion of deeply dead
break-tolerant row-major prefixes within a median of 4 matched edges, 3 rows
from the end of a 7x7 critical board, on generated boards. It carries no
search gain, no wall-clock claim and no 16x16 claim. The size trend (gap per
remaining cell rising from n = 6 to 8) argues against extrapolating it.
Stage 1 should be preregistered with a wall-clock arm, and with prefixes
drawn near the optimum as well as dead ones.

## Sources graded

- The "reported" referee memo cited in the README
  (`memos/partial-match-regime.REVIEW.md`) is used only as context. No part of
  this verdict depends on it.
