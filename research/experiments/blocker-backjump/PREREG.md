# A20 preregistration: blocker eviction as the backjump rule inside bb-mrv

Written and committed before any test-seed run (night shift 2026-11-12,
Track 2, branch `night/2026-11-12-t2-blocker-backjump`, not pushed by rule of
the night). Only calibration instances (primary ladder, seeds 1001..1010) have
been run so far, for tuning and timing. Forced changes go in
`PREREG_AMENDMENTS.md` (what, why, when); this file stays unchanged.

**Track label: method.** The search tree changes (the backtracking rule), so
no node-sequence identity with the parent is claimed. The bj arms still hash
their node sequence per run (`node_seq_hash` in the sidecar) for determinism
checks only.

## Hypothesis

Inside `bb-mrv`'s propagating DFS (A3 propagation: rim unary, edge arc
consistency, piece singles and hidden singles to fixpoint; forced cell first,
else free MRV; option-index value order), undoing the decisions that directly
block the failed cell (the IMS rule that beat random eviction in A15), then
re-propagating from the root, solves more R7 boards and needs fewer nodes than
chronological backtracking and than the ladder's decision baseline, at the
same node cap. A random-eviction twin with identical propagation and
bookkeeping does not.

## Arms

All arms run through hard-ladder's `run_one` and `NodeCounter::try_assign`
(one call per (cell, piece, rotation) assignment, before propagation, whether
propagation refutes it or not; no lookahead in any arm).

- `scb-gacolor_ac3_random`: decision baseline at R6 and R7 (hard-ladder scb,
  run seed 1, through A22's guarded wrapper `mrv_baseline::scb::GuardScb`).
- `bb-mrv`: the parent (chronological), and the decision baseline at 8x8
  c=10 (A16 `BbSolver { rule: Rule::Mrv }`, unchanged code).
- `bj-blocker` (candidate): same `Model::propagate`, same cell rule, same
  value order as bb-mrv. The chronological stack is replaced by an unordered
  decision set. At cell c the non-tabu options in c's current domain are tried
  in index order (one node each); the first whose propagation succeeds becomes
  a decision. If none succeeds, c is the failed cell x and the repair runs:
  over the non-tabu options o in x's root domain, B(o) = the decided
  orthogonal neighbours of x whose facing edge mismatches o, plus the decided
  cell already holding o's piece. Among o with |B(o)| >= 1 keep those of
  minimum |B(o)|, pick one uniformly at random (tie stream), and evict B(o).
  Then rebuild the state from the root domains with the remaining decisions
  and one `Model::propagate` call (asserted to succeed: removing decisions
  only loosens a monotone fixpoint; a failure is a panic and an `error` row).
  Code: `compute/src/lib.rs`.
- `bj-random` (mandatory matched control, everything except the mechanism):
  identical code path, identical B(o) computation and tie draw, identical
  tabu, fallback and re-propagation; the only difference is that it evicts
  |B(o)| decisions drawn uniformly at random (separate stream) from all
  current decisions instead of B(o).

Shared rules of both bj arms:

- Fallback: if no non-tabu option of x has |B(o)| >= 1 (x blocked only
  through propagation, not directly), evict the most recent decision. Counted
  per run (`fallbacks`) and reported as a fraction of dead ends.
- Tabu: every evicted (cell, option) cannot be re-decided at that cell during
  the next T nodes. T is chosen on calibration only (below) and is the same
  for both bj arms.
- Randomness: splitmix64 streams seeded from run seed XOR FNV(puzzle name);
  tie stream and twin stream are separate. Run seed 1 is binding; run seeds 2
  and 3 are run for both bj arms at R6 and R7 for the run-seed median.

### Completeness and cycling (decided before running)

The bj arms store no no-goods and are incomplete: they can revisit a decision
set and can never prove a board unsolvable. This is accepted, not patched:
every instance is planted (solvable), the comparison is at a fixed node cap,
and an incomplete method that cycles simply burns nodes until the cap. The
only anti-cycling device is the tabu tenure T (Rau's rule). If a dead end
occurs with no decision left, the tabu list is cleared (counted); if it was
already empty, the run returns `Exhausted`, which the harness records as an
`error` row (exhausted planted instance). Cycling is measured, not prevented:
after every repair the decision set is Zobrist-hashed; the sidecar records the
number of repairs that return to an already-visited decision set
(`repairs_revisiting_state`), the number of distinct states and the maximum
multiplicity. A run is reported as **cycled** if it is censored and more than
half of its repairs returned to an already-visited state. Every cycled run is
listed.

## Instances, rungs, seeds

| rung | ladder file | n | c | seeds | role |
|---|---|---|---|---|---|
| R6 | `primary-test` | 6 | 5 | 1..30 | binding |
| R7 | `primary-test` | 7 | 6 | 1..30 | binding |
| R8 (only if the R7 gate passes) | `8-c10-test` + `8-c10-fresh-test` | 8 | 10 | 21..30 + 31..50 | binding if run |

Calibration (tuning and timing only, never in a comparison):
`primary-calibration`, n = 6 and 7, seeds 1001..1010.

## Caps and guards

- Cap 1,000,000 nodes per instance for every arm (`TEST_CAP`).
- Wall-clock guard 900 s per run, every arm, expected never to bind (bj
  arms ran at roughly 18k to 50k nodes per second on calibration, so 1e6 nodes
  take about 20 to 60 s). If it fires, the run is time-censored and listed by
  name; a solved candidate against a time-censored baseline scores 0 (A22's
  rule).
- Tuning grid (calibration only): T in {0, 16, 128} nodes. Selection: on the
  20 calibration instances (R6 + R7), bj-blocker's most solved within 1e6,
  ties by the smaller sum over instances of ln(min(nodes, cap)), then the
  smaller T. bj-random uses the selected T (matched control); its results at
  every T are reported. Selected value and table: `results/calibration/`.
  Result of that rule, computed before this file was committed: **T = 16**
  (bj-blocker 11/20 calibration solved at T = 16, 6/20 at 128, 0/20 at 0).

## Execution

2 threads (`--threads 2`, `RAYON_NUM_THREADS=2`, `nice`), Apple M4 laptop
(4 performance + 6 efficiency cores, no pinning), so wall clock is noisy and a
guardrail column only. Arms are interleaved: for each instance the arms are
queued in an order rotated by the instance index, and two workers drain one
shared queue, so no arm runs entirely before another.

## Analysis (stdlib Python, `analysis/analyze.py`)

Per rung and arm: solved/30 with Wilson 95%, KM quartiles of nodes (censored
= +inf), median and total wall ms, time-censored count; for bj arms dead ends,
fallback fraction, mean evictions per dead end, revisit fraction, cycled runs.

G1 exactly as PROTOCOL.md: paired r_i = ln(c_i/b_i) (solved/solved),
ln(c_i/C) (candidate solved, baseline censored), +inf (candidate censored,
baseline solved), 0 (both censored); statistic the upper median (16th of 30);
95% percentile bootstrap of the median, 10,000 paired resamples, seed
20261111; PASS iff exp(median) <= 0.5 and CI upper end < 0. Wall ratio:
median of paired wall_c / wall_b with the same bootstrap.

Pairs reported at every rung run: blocker vs decision baseline (binding),
blocker vs bb-mrv (decision line, since the candidate is built on it),
random twin vs both baselines, blocker vs random twin (mechanism), bb-mrv vs
scb (context).

## Decision rules (binding, run seed 1 unless stated)

Write G1(X, Y, rung) for the PROTOCOL G1 result of X against Y.

1. **PROTOCOL rung rule.** G1(blocker, scb, R6) is evaluated first; if it is
   not PASS the method is dropped from the ladder at R6 (PROTOCOL.md "Rung
   order and dropping"). R7 is still run and reported (A20 rung).
2. **R7 gate (decides whether R8 runs).** Passes iff G1(blocker, scb, R7) is
   PASS, or blocker solves at least scb + 4 at R7 (run seed 1). R8 runs only if
   this gate passes; then G1(blocker, bb-mrv, R8) is binding.
3. **K1, method kill (A20 clause).** Killed iff G1(blocker, scb) is not PASS at
   R6 and not PASS at R7, AND blocker's R7 solved count (median over run seeds
   1..3) is at most scb's R7 solved count + 3.
4. **Overall method verdict.** KILLED if rule 1 drops it, or K1 fires, or R8
   was run and G1(blocker, bb-mrv, R8) is not PASS. Otherwise SURVIVES.
5. **K2, mechanism kill.** At R7 (and at R8 if run): killed iff the random
   twin solves at least blocker - 2 (run-seed medians at R7; run seed 1 at R8)
   AND blocker is not significantly better than the twin (95% CI upper end of
   the median paired ln ratio blocker/twin is >= 0, G1 pairing table).
6. Decision line against bb-mrv (G1(blocker, bb-mrv) at R6, R7): reported with
   CI, not binding.

Scope: generated boards, n = 6..8 only; nothing here is official Eternity II
progress.

## Core budget

2 cores, CPU only, no AWS. Estimate from calibration: R6 a few minutes; R7
4 arms x 30 at up to about 55 s per censored 1e6-node bj run (about 20 s for
bb-mrv), about 35 min on 2 threads; run seeds 2 and 3 (2 arms x 2 rungs x 30)
about 55 min; R8 if triggered about 60 min. Total under 3 hours wall on 2
cores. If time runs
out the order is R6, R7, run seeds 2 and 3, R8; anything not run is reported
as not run, and K1 falls back to the run-seed-1 count if seeds 2 and 3 are
missing.
