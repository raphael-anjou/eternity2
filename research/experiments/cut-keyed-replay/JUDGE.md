# Judge report: cut-keyed replay (Track 12, night 2026-11-12)

**Verdict: kill.** The stage-0 kill of proposal T3 on K2 stands. A random key
with the same number of partitions, drawn off the cut line, keeps fewer
`bb-mrv` nodes than the cut key at both rungs, on every control seed, and at R7
on every one of the 30 instances. K1-IDEAS also fires (every arm costs more
propagation work than `bb-mrv`). K1 and K3 do not fire. No invalidity found.

## 1. Preregistration order

- `PREREG.md` committed alone in 9375c26 at 2026-11-13 00:34:32 +0100;
  `git log --follow` shows no later edit, and its sha256 (03bc8855...) equals
  the hash on every run-log line.
- Amendment 1 (adds generated soundness boards, removes nothing) committed
  00:39:28 with the check rows, then a 5-second follow-up commit changing its
  stated time from "00:55" (a time still in the future) to "00:38". Harmless:
  it touches gate 3 only, not any statistic.
- `results/run-log.jsonl`: first test-stage line 23:40:43 UTC = 00:40:43
  local, six minutes after the PREREG commit, `dirty_paths` 0. Both test files
  (R6, R7) have a matching start/end pair; no test row lacks a log line. File
  mtimes agree (`main.rs` 00:38:03, binary 00:38:06, `primary-n6` 00:40:46,
  `primary-n7` 00:54:42).
- Unverifiable from the branch alone: whether a draft of the 984-line crate
  ran on test seeds before 00:34 (the crate landed 5 minutes after PREREG).
  Mitigation: the replay is deterministic and has no free parameter that a
  peek could tune; the arms in the code match the PREREG text line for line.

## 1b. Track label

Labelled method, so the optimization hash test is not the gate. The node
identity gate holds anyway: on 60 of 60 trees status, nodes, propagation and
backtracks equal the committed `bb-mrv` rows, the node-sequence hash equals
the census row, and `prop_mismatch` is 0 (recomputed by my own script).

## 2. Rerun

Rebuilt from the committed source in a separate target dir, 2 cores on the Mac:

- R6, all 30 seeds: every row equal to the committed row in every field except
  wall-clock fields (all eight arms' surv, kills, calls, work, re-keys,
  mismatches).
- R7 seeds 9, 18, 27, 25, 15 (solved) and 29 (censored at 1e6): exact. The
  R7 file was produced on `dell`, so this also checks the remote binary
  matches the committed code.
- Gate 3 (`replay check`): 640 rows byte-identical to the committed file.
- `analysis/analyze.py` re-run: `summary.json` byte-identical (bootstrap CIs
  included).

## 3. Recomputed headline numbers (own script, from the jsonl rows)

| | R6 | R7 |
|---|---|---|
| cut, upper median node ratio | 0.4826 | 0.4856 |
| C1 (mean of 5 seeds) | 0.4438 | 0.4191 |
| C1 per-seed medians | 0.437 0.440 0.445 0.457 0.446 | 0.418 0.418 0.418 0.418 0.420 |
| median(C1) - median(cut) | -0.0388 | -0.0665 |
| paired C1 - cut, upper median | -0.0172 | -0.0592 |
| instances where C1 < cut | 21 / 30 | 30 / 30 |
| C2 POAC budgeted | 0.750 | 0.7025 |
| cut work ratio | 1.392 | 1.211 |
| C1 work ratio | 1.299 | 1.325 |
| C2 propagate calls vs cut | 28,104 vs 31,118 | 22.01 M vs 22.42 M |

K1: 0.486 <= 0.5, no. K1-IDEAS: 1.211 > 1, fires. K2: -0.039 and -0.067 both
<= 0.10, fires (two-sided reading also fires). K3: 0.702 > 0.486, no.
Soundness gates 1 and 2: 0 violations over 8 arms x 60 trees; gate 3: 0
violations, 1,425 (cut) to 1,954 (C2u) kill points checked; `mono` 0.

## 4. Adversarial checks

- **Random-order control:** present and partition-matched (C1, 5 seeds); the
  kill rests on it. Product mismatches 13 to 18 per seed at R6 and about
  14,200 to 14,500 of about 1.23 M selections (1.2%) at R7: too few to flip a
  gap that holds on all 30 R7 instances.
- **Node accounting:** both arms count surviving `try_assign`s under the same
  rule over the same tree (failing leaves included on both sides); split
  propagations are not nodes but appear in the work column. Equal.
- **Confound worth stating:** at R7 C1 re-keys more often (1.23 M vs 0.83 M)
  and spends 9% more work (1.325 vs 1.211), so part of its node edge at R7
  can be the extra re-splitting. This does not rescue T3: at R6 C1 beats the
  cut on nodes *and* work, and at R7 the cut is not better on nodes, which is
  what the hypothesis ("the gain comes from the cut") needs.
- **Leakage:** the planted solution is used only by soundness gate 2.
- **Censoring:** 7 censored R7 trees; ratios over the first 1e6 nodes, flagged.
  C1 beats the cut on both subsets (0.419 vs 0.495 solved, 0.419 vs 0.476
  censored). Not counted as wins anywhere.
- **Dropped seeds / subsampling:** none; 30 of 30 seeds per rung.
- **Wall clock:** no wall claim is made; every arm's work ratio is above 1 and
  the README says so.
- **Claims:** README stays inside the evidence (no 16x16 claim, stage 1 not
  run). One imprecision: "the replay ratio is an upper bound on node gain with
  the same branching" is exact, not a bound, when branching is held fixed; it
  is an optimistic estimate only for a real search that would also branch on
  the arm's own domains. Wording only, verdict unaffected.
- **Site pages:** the verdict does not lean on explored/dead-end pages. The
  census judge's ruling that the earlier T3 kill did not stand is *measured*
  (committed census results), and this run supersedes it.

## Unplanned observations

- Any two-edge partition prunes about half of `bb-mrv`'s nodes (all keyed arms
  0.42 to 0.49), against 0.70 for budgeted POAC on the next cell. The pruning
  is generic to partitioning on low-colour-set edges, not specific to the cut.
- With `bb-mrv`'s MRV order, "the line below the lowest assigned row" is not a
  frontier in the row-scan sense; the cut rule inherited from the census is
  close to an arbitrary line choice under this branching. A cut key may still
  matter under a row-scan solver; this run does not test that.
- C1 seeds give near-identical R7 medians (0.418 x4, 0.420): the control's
  variance across seeds is far smaller than its gap to the cut.
