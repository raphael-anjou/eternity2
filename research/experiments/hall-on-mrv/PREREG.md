# Hall matching on top of bb-mrv (A17 rebaselined): preregistration

Night shift 2026-11-12, track 4 of `research/program/NIGHT_BRIEF.md`. Branch
`night/2026-11-12-t4-hall-on-mrv` from `origin/main` 6247af0. Written and
committed before any test seed runs. Forced changes go in
`PREREG_AMENDMENTS.md` (what, why, when); this file is not edited after its
commit. Generated boards only; nothing here is official Eternity II progress
and nothing is extrapolated to 16x16.

## 0. Track label

**Method.** The Hall arms change the search tree (they cut subtrees), so the
node sequence is not preserved and the track is judged as a method track
under `research/experiments/hard-ladder/PROTOCOL.md`. The controls below are
required to keep the node sequence of `bb-mrv` exactly (checked per seed).

## 1. What Hall matching adds on top of bb-mrv's all-different

`bb-mrv` (A16 `BbSolver { rule: Rule::Mrv }`, A22) propagates with A3's
`sat_backdoors::Model::propagate`. Its piece all-different part, applied to a
fixpoint together with colour arc consistency, is exactly three rules over
the piece sets P(c) = {pieces with some rotation still in the domain of c}:

1. naked piece single: if |P(c)| = 1, that piece is removed from every other cell;
2. hidden single: if a piece lies in P(c) for exactly one cell c, c is restricted to it;
3. no-cell wipeout: if a piece lies in no P(c), the node fails.

These are Hall's condition for sets of size 1 only (one cell with one piece,
one piece with one or zero cells). Hall matching asks for a system of
distinct representatives: every unassigned cell in scope gets a pairwise
distinct piece from its P(c). Because the unassigned cells and the unused
pieces are equally many, a cell-saturating matching over all unassigned
cells is a perfect matching, so the check is equivalent to Hall's condition
on every subset of either side. It is therefore not implied by rules 1 to 3:
three cells with P = {p, q}, {p, q}, {p, q} (p, q in no other cell, other
pieces elsewhere) pass rules 1 to 3 (no cell has one piece; p and q each sit
in three cells) and fail Hall (three cells, two pieces). Unit test
`compute/tests/units.rs::hall_rejects_three_cells_two_pieces`. The check
does not filter domains (no Regin filtering), so it can only fail a node.

Small check, run on calibration seeds before this file was committed
(`results/calibration/r6.jsonl`, `r8.jsonl`): at R6 seeds 1001..1010 the
cost-control arm (check computed at every node on the frontier, never acting)
found nodes where A3 had succeeded and Hall failed on 7 of 10 instances (1 to
24 per instance), and V1 cut 0 to 35 nodes per instance (bb-mrv 138 to 3,731
nodes). At R8 seeds 1001..1010 the cost arm counts 4 to 223 such nodes per
instance and V1 / V2 cut 0.06 to 1.6% of the nodes (one R7 calibration seed,
1001: 1,668 such nodes, V1 / V2 cut 0.5% / 0.7%; the R7 calibration run was
stopped after it to free the cores). So the check adds
something by construction and in practice, and the expected effect on bb-mrv
is small; this experiment measures it.

## 2. Hypothesis

Adding A17's Hall check to `bb-mrv` reduces the search tree enough to pass or
gain on PROTOCOL.md G1 against `bb-mrv` on at least one of R6, R7, 8x8 c=10,
at a median paired wall-clock ratio at most 1 there.

## 3. Arms (all run by this session on the same instances, same cap)

| arm | role | definition |
|---|---|---|
| `bb-mrv` | decision baseline, every rung | A16 `BbSolver { Rule::Mrv }`, unchanged code (path dependency) |
| `hall-v1` | candidate V1 | A17 V1: scope = frontier (unassigned cells with an assigned neighbour), check after every successful propagation |
| `hall-v2` | candidate V2 | A17 V2 (`cmp-hall-all-k4`, selected by A17's calibration): scope = every unassigned cell, check on every 4th successful propagation |
| `hall-off` | control (a), implementation check | this crate's solver with the check off; must equal `bb-mrv` node for node |
| `cost-v1`, `cost-v2` | control (b), cost | V1 / V2 check computed at the same nodes, result counted and never acted on; must equal `bb-mrv` node for node |
| `mrv-rand-1..3` | order-effect control | A22's `mrv-rand` (`MrvSolver { Free, random: Some(s), hs: true }`) at run seeds 1, 2, 3 |
| `scb` | adopted baseline at R6/R7 | `scb-gacolor_ac3_random`, run seed 1, A22's guarded wrapper; run at R6 and R7 only |

Both A17 variants carry over unchanged in meaning: scope and period are A17's;
the check runs where A17 ran it (after propagation succeeds, before
descending) and a failure is a propagation failure. No variant is tuned
here. The Hall routine is A17's `hall_match` (Kuhn augmenting paths, cells
sorted by popcount, u128 piece masks).

The check is the only difference between `hall-vk` and `cost-vk`, and
`cost-vk` differs from `hall-off` only by computing it. So `hall-vk` vs
`cost-vk` isolates the pruning, `cost-vk` vs `hall-off` isolates the check's
cost, and `hall-off` vs `bb-mrv` isolates implementation differences.

## 4. Instances, cap, seeds

- R6: `hard-ladder/instances/primary-test.jsonl`, n = 6 (c = 5), seeds 1..30.
- R7: same file, n = 7 (c = 6), seeds 1..30.
- R8: 8x8 c=10, A22's 30 instances: `8-c10-test.jsonl` seeds 21..30 and
  `8-c10-fresh-test.jsonl` seeds 31..50.
- Calibration (used only for the small check above and for timing):
  `primary-calibration.jsonl` n = 6, 7 and `8-c10-calibration.jsonl`, seeds 1001..1010.
- Cap: 1,000,000 nodes through `NodeCounter::try_assign` for every arm.
- Wall-clock guard 600 s per run, which must not bind (bb-mrv on R7 runs at
  well under 1e6 nodes per 600 s here); every run a guard stops is reported
  and counted as censored.

## 5. Runs and wall clock

Hardware: Apple M4 (4 performance + 6 efficiency cores), shared tonight with
other tracks (about 9 of 10 cores busy), no pinning possible on macOS.
Two worker threads (`--threads 2`, `RAYON_NUM_THREADS=2`, under `nice`).

Interleaving: per rung, jobs are ordered repetition, then instance, then arm,
with the arm order rotated by (instance index + repetition); the two workers
take jobs in that order, so arms on one instance run back to back and no arm
always runs first. R6 runs every (instance, arm) 5 times (runs last
milliseconds) and uses the median wall clock per pair; R7 and R8 run once.
Node counts are deterministic; any repetition that disagrees in status or
nodes is an error. Order of rungs: R6, R8, R7.

## 6. Statistics and decision rule

All against `bb-mrv` on the 30 instances of a rung.

- G1 exactly as PROTOCOL.md: r_i table with cap C = 1e6, statistic = upper
  median (16th order statistic) of the 30 r_i, 95% percentile bootstrap CI
  with 10,000 paired resamples, seed 20261111. G1 PASS iff exp(median) <= 0.5
  and the CI upper end < 0.
- Labels per variant and rung, A17's rule with Holm over the two variants:
  one-sided bootstrap p = fraction of resamples with median r >= 0; the
  smaller p is significant iff <= 0.0125, the other iff the first is and its
  p <= 0.025. PASS = G1 PASS and significant; GAIN = significant, CI upper
  < 0, ratio > 0.5; NO-GAIN otherwise.
- Wall ratio: per instance ln(wall_cand / wall_bb-mrv) (R6: medians over the
  5 repetitions); statistic = exp(upper median), same bootstrap for the CI.
- **Kill (binding).** A variant is killed iff (K1) it is NO-GAIN at R6, R7
  and R8, or (K2) at every rung where it is GAIN or PASS its median paired
  wall ratio against `bb-mrv` is above 1. The method is killed iff both
  variants are killed.
- Because the check only removes subtrees (section 7), every r_i <= 0 by
  construction, so a label of GAIN can come from a very small reduction.
  Preregistered reading, not a change to the rule: a surviving variant whose
  every GAIN rung has a G1 ratio above 0.9 and a wall-ratio CI containing 1
  is reported as "not killed by the rule, no material effect". Only a G1 PASS
  with wall ratio <= 1 would be reported as a result worth adoption.
- Also reported (not binding): solved/30 with Wilson 95% interval, KM median
  and quartiles (8th, 15th, 23rd order statistics, censored = infinity),
  median and total wall clock, guard stops; G1 and wall ratios of `hall-vk`
  vs `cost-vk` (pruning at equal cost), `cost-vk` vs `bb-mrv` (cost of the
  check), `hall-off` vs `bb-mrv`; and at R6/R7 `bb-mrv` and `hall-vk` against
  `scb`. G2 is not evaluated (three rungs with mixed colour regimes).

## 7. Order effects

Claim by construction: the check never changes a domain, and it fails a node
only when no completion exists below it (every solution extending the node
uses options still in the propagated domains and gives a perfect matching).
So the Hall arm visits the `bb-mrv` node sequence with whole failed subtrees
deleted and nothing reordered; when both solve, they return the same board.
Checks:

1. `trace-check`: full traces of `hall-off` and each Hall arm, verified to be
   "base sequence minus subtrees below Hall-pruned nodes", on all calibration
   instances of R6, R7, R8 and on the R6 test instances.
2. On every test instance and rung: Hall nodes <= `bb-mrv` nodes, same board
   hash when both solve, and `cost-vk` / `hall-off` identical to `bb-mrv` in
   status, nodes, backtracks, propagation count and board, and `cost-vk`
   sequence hash equal to `hall-off`'s. Any failure stops the analysis and
   goes in `PREREG_AMENDMENTS.md`.

Order-effect comparison requested by the lead: per rung, the G1 ratios of
`mrv-rand-1..3` against `bb-mrv` give a spread [min, max]. A Hall G1 ratio
inside that spread (not below its minimum) is reported as "not
distinguishable from an order effect" in size, even though by check 1 it
contains no reordering.

## 8. Budget

2 cores, about 75 minutes of runs measured from calibration speed (R8 about
20k nodes/s per thread under tonight's load): R6 under 2 min with 5
repetitions, R8 about 15 min (9 arms), R7 about 45 min (10 arms), trace
checks about 10 min. If a stage runs past twice its estimate, it continues
and the overrun is reported; nothing is cut after seeing results.
