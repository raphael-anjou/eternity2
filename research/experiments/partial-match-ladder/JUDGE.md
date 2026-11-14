# Partial-match ladder: judge's verdict

Branch `night/2026-11-12-t7-partial-match-ladder`, judged at `c8b129b` by a
session that did not run the experiment. Track label: measurement (no solver
proposed or tuned, so no node-sequence hash check applies).

## Verdict: kill

The instrument as preregistered (these five entrants, the budget pair
4e6 n^2 move evaluations / 0.2 times that in placements, the hard ladder's
framed generator, n in {8, 10, 12}) is not usable. Criterion (1) fails at the
only rung calibration selected, and it fails by a wide margin: 2 of 5 entrants
stall in the [2%, 4%] band, and the next-closest entrant (`break-r1`) sits at
6.25%. No cell at n = 10 or 12 qualified, so there is no rung B to rescue it.

Scope of the kill: this entrant pool and budget pair. It does not cover a
partial-match ladder whose constructive entrant keeps searching after its first
complete board, or one built from the 16x16-native engines (`blackwood_bt`,
`verhaard_faithful_v2`, beam producer) once they run at small n. The failure is
largely a property of the entrant pool: `BreakSolver` stops at its first
complete board (at most 3.8e6 of 5.12e7 placements used on rung A), and ALNS
from a greedy start is the weakest entrant everywhere.

## Preregistration order

- `PREREG.md` committed once, `a796609` at 20:23:34; never edited since
  (`git diff 3e204d6 HEAD -- PREREG.md` is empty).
- Calibration rows finished (dell log 22:59:49) before `CALIBRATION.md` was
  committed (`3e204d6`, 23:00:09). Test instances committed in `10bb97d`, the
  same second; the instance file's birth time is 23:00:09. Ordering inside that
  second cannot be shown, but test seeds 7101..7130 were fixed in PREREG and the
  generator is deterministic, so nothing could be chosen from them.
- Test row files born 23:00:25 to 23:19:19, committed 23:19:46. All after PREREG.
- **Finding: no `results/run-log.jsonl`.** `research/experiments/common/prereg_guard.py`
  is not on this branch (it was added on the lead branch at 20:41, after this
  branch's PREREG commit), and the test runs at 23:00 did not go through it. The
  order above rests on commit order and file birth times only, as for tracks 1,
  2 and 6. Every test row has no matching log line. Not invalidating here
  (single PREREG commit, no threshold touched after any data), but the gap is real.
- Amendment clock labels do not match commits: A1 is headed 20:35 and A2 20:33
  but both were committed earlier (20:28:07, 20:31:12). A1 declared the
  fallback rate rule; A3 later used the full-budget rule after three probe runs
  had used their whole budget. Both rules give rho = 0.2 (recomputed below), so
  the choice after seeing data has no effect.
- A4 changed one prose sentence of `CALIBRATION.md` after the test; the table,
  rule and choice are unchanged (diff checked).

## Recomputed numbers (own script, own pure-Python verifier, from raw rows)

- Every board re-verified from the instance file: probe 32 rows, calibration
  240, test 150; 0 score mismatches, 0 over budget, every planted board scores
  the optimum. No duplicate or missing (entrant, instance) keys; 30 of 30 test
  seeds per entrant.
- rho: `break-r1` 6.387e5 placements/s (median of 3 full-budget runs),
  `pt` 3.833e6 evals/s, rho = 0.1666; the A1 fallback (pooled work/wall) gives
  0.1799. Both round to 0.2.
- Calibration: same 12-cell table; qualifying cells n8 c6 (2 in band, best `pt`
  2.68%) and n8 c10 (1 in band, `break-r1` 3.57%); rung A = n8 c6. Correct by the rule.
- Test, rung A (median gap): `break-r1` 6.25%, `break-r2` 13.39%, `sa` 3.57%,
  `pt` 3.57%, `alns` 11.61%. In band 2 of 5; best-of-five below optimum 30 of 30.
  Criterion (1) fails.
- Family pairs (break-r1, sa), (break-r2, sa), (alns, sa): `sa` wins 30/0 in
  each, median differences -1.79, -8.93, -8.48 points, p < 1e-4. Significant
  and every one reversed against the 16x16 family order.
- Determinism: the committed dell/mac-m1 rows are identical for all five entrants.

## Rerun

On a third machine (Apple M4, 2 threads): `break-r1`, `break-r2`, `alns` on all
30 test instances and `pt`, `sa` on seeds 7101, 7110, 7120, 7130. 98 of 98 rows
identical to the committed rows in status, score, work, budget, trajectory and
board. `cargo test` 4/4 and `pml check` on every instance file pass.

## Findings that do not change the verdict

1. **Rank ties on float differences** (`analysis/analyze.py` `ranks_abs` over
   share differences). Equal integer score differences can produce unequal float
   differences, so ties are split. On integer differences, `break-r2` vs `alns`
   gives W = -196, p = 0.032 (committed -183, 0.047). This is a descriptive pair
   only; the three family pairs are 30/0 and unaffected. Fix: rank
   `best_a - best_b` in integer score units.
2. **README overreach.** "The band is reachable at small size only for the two
   annealing entrants, and only at a colour count below the whole-board critical
   point" contradicts the calibration table: n8 c10 (c - c_crit = +2.68) has
   `break-r1` in band at 3.57% (4 calibration seeds). The sentence "at every cell
   at or above critical ... all entrants stall above 4%" is false for the same cell.
3. **Family-level reversal is confounded.** The README reads it as "these
   small-size implementations do not stand in for their families' 16x16 record
   engines". The data cannot separate implementation from board size. The 16x16
   side is measured on other implementations (single-core-benchmark, repair-study,
   dfs-study rows) plus reported records (EXPLORED.md: 470, 467). That is graded
   reported/other-implementation, and the verdict does not lean on it.
4. PREREG says "three entrants are linked" by the family pairs; the pairs link
   four (break-r1, break-r2, alns, sa). `summary.json` says 4. Wording only.
5. `sa` work is charged for the final run only; the doubling-grid prefix runs
   cost about 2x more compute, which is in its wall column. The prefix property
   holds (the `run_sa` schedule does not depend on `max_iters`).

## Unplanned observations

- At n8 c12 (first-moment expected solutions 4e-18, so the planted board is
  close to unique) `pt` reached the full 112 on calibration seed 7003, while its
  other three seeds sat at 99 to 102. A single perfect solve at a cell where every
  other median gap is above 10% points to high per-instance variance, which an
  instrument of this kind would have to account for.
- At n = 10 and 12 every entrant's best stalls 5% to 12% below the optimum at
  this budget, a wider gap than the 16x16 record gap (2.08%). These generated
  boards at this budget are harder relative to their optimum than the real
  puzzle is relative to 480. Planted optimum vs unknown optimum makes the
  comparison loose.
