# A21 judge report: value order by propagation impact inside bb-mrv

Branch `night/2026-11-12-t3-value-impact`, judged at commit `596a3ad`. Judge
read the branch only (PREREG, code, raw rows), not the runner's session.

## Verdict: KILL

The preregistered A21 rule fires: G1 of `vi-impact` against `bb-mrv` fails at
every rung, so no rung has a node gain for the wall-clock gate to act on. The
kill also holds against the AGENDA's original baseline (random value order,
the `mrv-rand` composite): no rung shows a gain there either. The PROTOCOL.md
dropping rule drops the method at R6.

## 1. Preregistration order

`git log --follow` on `PREREG.md`: one commit, `a4507f5` (18:21:36). Code and
calibration rows follow in `4dc0caa` (18:27:21), test rows, check rows and
analysis in `596a3ad` (19:07:29). PREREG is unchanged after its commit; no
`PREREG_AMENDMENTS.md`. Calibration enters no decision and the method has no
free parameter to tune on it. Order valid.

## 1b. Track label

Label is **method** (value order changes the tree), so the optimization hash
check does not apply; the method targets (G1) apply and are what the verdict
uses. The implementation-check arm `vi-index` (probing code, index order)
equals `bb-mrv` on nodes, backtracks, status and `equals_planted` on 90 of 90
test instances (recomputed). `bb-mrv` does not emit a node-sequence hash, so
that equality rests on the four fields, not on a sequence hash; with nodes and
backtracks both equal on 90 instances this is sufficient for an order-only
change. Code read: `ProbeSolver::dfs` is `BbSolver::dfs` with probes added; the
node rule (one `try_assign` per option tried, failed options included, one
backtrack per return) is the same in both, so node accounting is equal between
arms. Probes never call `try_assign` and are reported in `propagation`/`probes`
(probes per node 1.00 to 1.02).

## 2. Numbers recomputed from the committed rows

Independent stdlib script over `results/test/*.jsonl` and
`results/check/*.jsonl` (own bootstrap, different RNG; point estimates are
exact, CIs agree to the third decimal).

| rung | vi-impact vs bb-mrv | CI | G1 | wall ratio | vs vi-probe-rand | vs mrv-rand |
|---|---|---|---|---|---|---|
| R6 6x6 c=5 | 1.075 | [0.779, 1.931] | fail | 1.143 | 1.297 [1.058, 1.621] | 1.325 |
| R7 7x7 c=6 | 0.829 | [0.544, 1.316] | fail | 0.814 | 1.000 | 1.000 |
| R8 8x8 c=10 | 1.096 | [0.669, 1.871] | fail | 1.141 | 1.128 | 1.127 |

Solved/30: R6 30 and 30, R7 23 and 23 (5 candidate-only, 5 baseline-only, 2
both censored), R8 30 and 30. At R7 the CI lower end (0.544) is above the
0.5 target, so no reading of the R7 data reaches G1.

Integrity checks on the raw rows: every arm has exactly 30 instances per rung
(seeds 1..30 at R6/R7, 21..50 at R8; none dropped), R6 has 5 repetitions per
(arm, instance) with identical status, nodes, backtracks and sequence hash
across repetitions, no error row, every censored row stopped at the 1e6 node
cap (no guard-stopped run), cap 1e6 on every row. Censored runs are scored as
PROTOCOL G1 requires (+inf when only the baseline solved), not as wins.

Reproduction cross-check claimed in the README, verified: `bb-mrv` (50
instances), `mrv-rand-s1` (50) and `scb-gacolor_ac3_random` (20) rows equal
A22's committed rows in `../mrv-baseline/results/raw/` on every overlapping
instance.

## 3. Rerun

Rebuilt from the committed source into a separate target dir (`cargo build
--release -j 2`), 2 threads. Rerun: R6 all 30 instances x {bb-mrv, vi-impact,
vi-probe-rand-s1, mrv-rand-s1}; R8 all 30 instances x {bb-mrv, vi-impact,
vi-probe-rand-s2}. All 210 rerun rows (120 at R6, 90 at R8) match the
committed rows exactly on status, nodes, backtracks, propagation,
`equals_planted`, and, for the probing arms, the node-sequence hash and probe
count. Rerun time 4 min 53 s on 2 threads (shared machine). R7 was not rerun (its censored runs make it
the slow rung on a shared machine).

## 4. Adversarial review

- Planted-solution leakage: none. The candidate sees only the domains and
  `Model::propagate`; `equals_planted` is computed after the run and is equal
  for `vi-index` and `bb-mrv`.
- Baseline strength: `bb-mrv` is the A22-kept 8x8 baseline; the AGENDA's
  random-order baseline is also run (3 seeds), and a probe-then-random twin
  isolates the ranking from the probing. Neither baseline is weak; the
  candidate loses or ties against all of them.
- Wall clock: irrelevant to the verdict (no node gain to gate). Wall ratios
  track node ratios, as expected with probes/node about 1.
- Subsampling: none; full prereg instance sets at every rung.
- Mis-specification, scope: the impact measured is the count of option bits
  removed (sum over cells). Refalo's impact is the relative reduction of the
  search-space product (sum of log domain sizes). The two orders can differ
  when removals fall on small versus large domains. The kill covers this
  measure, least-first, inside bb-mrv, at 6x6 c=5, 7x7 c=6 and 8x8 c=10. It
  does not cover a log-product impact, the most-constraining direction, or
  the AGENDA's 8x8 c=9 rung (the brief moved it to c=10). The README does
  not claim past this.
- Claims in the README: every number matches `tables.md` and my recompute.
  "Significantly worse than random among the same probed options" at R6 is
  supported (CI [1.058, 1.621] excludes 1) and is stated for R6 only.
- Minor: the README says the full recipe takes about 45 min, the justfile
  says about 1 h, PREREG budgets 1.5 h. Stage logs show 33:44 (test) plus 5:36
  (check) plus calibration. Not material.

## Status of claims this verdict leans on

Only committed results on this branch and A22's committed raw rows (measured).
No site "explored" or "dead end" page is used.

## Unplanned observations

- A single run seed of random value order passes G1 against `bb-mrv` at R8
  (`mrv-rand-s3`: 0.495 [0.409, 0.768], wall 0.486, recomputed), while
  `mrv-rand-s1` is at 1.623. G1 against one deterministic baseline can be
  passed by a reshuffle with no mechanism. Any future order-type method should
  carry run-seed controls, as this prereg did; a G1 pass without them is not
  evidence.
- `bb-mrv` vs scb at R6 on 30 seeds: node ratio 0.468 (PASS) but wall 1.373;
  at R7 node 0.976, wall 2.807. bb-mrv's node advantage over scb does not
  carry to wall clock at these small rungs (noisy, shared machine).
