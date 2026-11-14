# A21 preregistration: value order by propagation impact inside bb-mrv

Written and committed before any test seed is run by any arm in this
experiment. Branch `night/2026-11-12-t3-value-impact`, from `origin/main`
(6247af0). Night shift 2026-11-12, Track 3 of `research/program/NIGHT_BRIEF.md`.
Forced changes go in `PREREG_AMENDMENTS.md` (what, why, when); this file stays
unchanged after its commit. Generated boards only; nothing here is official
Eternity II progress, and nothing is extrapolated beyond n = 8.

## Track label

**method** (PROTOCOL_TARGETS_PROPOSAL.md section 1): the value order changes, so
the search tree changes. Judged on nodes (G1) with wall clock as a gate.

## Hypothesis (AGENDA A21)

In `bb-mrv` (A16's `BbSolver { rule: Rule::Mrv }`, kept by A22 as the 8x8 c=10
baseline), ordering the options of the branching cell by the number of option
values their real propagation removes from the other cells' domains, least
first (least-constraining value measured by propagation, after Refalo, CP
2004), shrinks search trees against bb-mrv's option-index value order, and the
gain comes from the ranking, not from probing or from randomising the order.

## The mechanism, exactly

`vi-impact` is bb-mrv with one change, at every search node:

1. Cell choice unchanged: lowest-index free cell with |D| = 1, else min (|D|,
   index).
2. For the chosen cell c with options o_1 < ... < o_k (option index order, as
   bb-mrv), each option is **probed**: copy the domains d, restrict c to o_j,
   run `sat_backdoors::Model::propagate(d', [c])` (the same call bb-mrv's
   `assign` makes). The probe yields ok_j (no contradiction) and the reduced
   domains d'_j.
3. Impact_j = total_options(d) - total_options(d'_j) (number of option bits
   removed over all cells, the chosen cell included; the chosen cell's term
   |D_c| - 1 is the same for every option, so the order is the order by values
   removed from the other cells). A failed probe has impact +infinity.
4. Options are tried in ascending (impact, option index). Failed options sit
   last.
5. Trying an option is the bb-mrv step: `NodeCounter::try_assign()` (stop on
   false), then, if ok_j, recurse on the cached d'_j; if not ok_j, count a
   backtrack and continue. Every option is tried, failed ones included, so the
   method changes only the order, never the set of children.

### Probe accounting

A probe never calls `try_assign`. Each probe is one `Model::propagate` call and
is counted in the `propagation` column (`Stats::propagations`, root included).
The search reuses the probe's domains when it makes the assignment, so the
assignment itself runs no second propagation: in bb-mrv, `propagation` = nodes
+ 1; in probing arms, `propagation` = probes + 1. Probes of options that the
search never reaches (because a solution was found below an earlier option, or
the cap hit) are extra work and are visible in that column. `probes` is also
written to every row. Secondary work column: propagate calls per instance,
paired ratio against bb-mrv, reported next to nodes and wall clock.

## Arms (all on the same instances, same counter, cap 1e6 nodes)

| arm | role | value order |
|---|---|---|
| `bb-mrv` | parent, decision baseline at every rung | option index (A16 code, unchanged, via the `backdoor-branching` crate) |
| `vi-impact` | candidate | probe all options, ascending impact, ties by index |
| `vi-probe-rand-s{1,2,3}` | control with everything except the ranking | probe all options, failed options last in index order, the non-failed ones shuffled (Fisher-Yates, scb's splitmix64 generator as in A22's `mrv-rand`, seeded by the run seed) |
| `mrv-rand-s{1,2,3}` | control named by the brief (A22's `mrv-rand`) | no probes, random order (A22 `MrvSolver { random: Some(seed) }`, unchanged code, run seeds 1, 2, 3) |
| `scb-gacolor_ac3_random` | adopted baseline at R6/R7, reported | A22's `GuardScb` wrapper, run seed 1 (R6 and R7 only) |
| `vi-index` | implementation check, never evidence | same code as `vi-impact` (probes, cached domains) with the option-index order |

Implementation check (binding as a validity gate): `vi-index` must equal
`bb-mrv` in nodes, backtracks, status and `equals_planted` on every test
instance of every rung. A mismatch on any instance invalidates the run until
fixed (recorded in `PREREG_AMENDMENTS.md`). Probing arms also record an FNV-1a
hash of their node sequence (cell, option per `try_assign`) per run, so the
check is reproducible and `vi-impact` reruns can be confirmed identical.

## Rungs and instances (fixed, from hard-ladder `instances/`)

| rung | board | file | seeds | n |
|---|---|---|---|---|
| R6 | 6x6 c=5 | `primary-test` | 1..30 | 30 |
| R7 | 7x7 c=6 | `primary-test` | 1..30 | 30 |
| R8 | 8x8 c=10 (above critical, amendment A3) | `8-c10-test` 21..30 + `8-c10-fresh-test` 31..50 | 21..50 | 30 |

Calibration (timing and correctness only, no tuning; this method has no free
parameter): `primary-calibration` n = 6, 7 and `8-c10-calibration`, seeds
1001..1010, arms `bb-mrv`, `vi-index`, `vi-impact`. Calibration rows enter no
comparison and no decision.

## Caps, guard, cores

- Cap 1,000,000 nodes per run through `NodeCounter::try_assign`, identical for
  every arm (hard-ladder `TEST_CAP`).
- Wall-clock guard 600 s per run, every arm. Expected never to fire; it is not
  the binding stop. Every guard-stopped run is listed by name in the report; a
  guard-stopped run is censored, and a pair with a guard-stopped baseline and a
  solved candidate scores 0 (as in A22).
- 2 cores: 2 worker threads, `cargo build -j 2`, `RAYON_NUM_THREADS=2`. Apple
  M4 (4 performance + 6 efficiency cores), no core pinning available on macOS.

## Wall clock (a gate here, not only a column)

Arms are interleaved on the same instance: a job is (instance, repetition);
the worker runs every timed arm on that instance back to back, in an order
rotated by (instance index + repetition) so no arm is always first. No arm is
ever timed in a block before another. Repetitions: 5 at R6 (runs last
milliseconds), 1 at R7 and R8. Per (arm, instance) wall = median over
repetitions. Node counts are deterministic and identical across repetitions
(checked). `vi-index` runs in a separate untimed pass.

Wall statistic W_n: median over the 30 instances of wall_candidate /
wall_baseline (censored runs use their measured time), 95% percentile bootstrap
CI, 10,000 resamples of instances, seed 20261111.

## Statistics

G1 exactly as PROTOCOL.md: per instance r_i = ln(c_i / b_i) if both solved;
ln(c_i / C) if only the candidate solved; +inf if only the baseline solved; 0
if both censored. Statistic: upper median (16th order statistic of 30). 95% CI:
percentile bootstrap, 10,000 paired resamples, seed 20261111. G1 PASS iff
exp(median r) <= 0.5 and the CI upper end of median r < 0.

Control composite: for a 3-seed control, per instance the median of the three
run-seed node counts with censored = +inf (the 2nd order statistic); the
composite is censored iff that median is +inf. Its wall is the median of the
three walls.

Per rung and arm: solved/30 with Wilson 95%, KM median and quartiles of nodes
(censored = +inf), median and total wall ms, propagate calls per node, probes.

## Decision rules (binding)

For each rung n in {R6, R7, R8}:

- G1_n: PROTOCOL G1 of `vi-impact` against `bb-mrv` (30 pairs).
- W_n: median paired wall ratio `vi-impact` / `bb-mrv`.

**Kill (A21 criterion):** KILL iff at every rung either G1_n is not PASS, or
G1_n is PASS and W_n > 1. Equivalently, SURVIVE iff at least one rung has G1_n
PASS and W_n <= 1.

**Mechanism attribution (required for a SURVIVE to be credited to the
ranking):** at every rung where SURVIVE holds, G1-form statistic of
`vi-impact` against the `vi-probe-rand` composite has upper median r < 0 and
CI upper end < 0. If SURVIVE holds without this, the verdict is "survives,
gain not attributable to the impact ranking".

**PROTOCOL.md dropping rule, also reported:** G1 against `bb-mrv` at R6, then
R8 (the 8x8 rung in use); the method is dropped at the first fail.

Reported, not binding: G1 and W of `vi-impact` against the `mrv-rand`
composite (all rungs) and against `scb-gacolor_ac3_random` (R6, R7); G1 of
each control against `bb-mrv`; propagate-call ratio against `bb-mrv`.

## Novelty note

A16 measured probe-based impact for **cell** choice (`Rule::Impact`, killed);
A22 measured random value order (`mrv-rand`, node ratio 1.62 at R8). No value
order by propagation impact has been measured in this repo (grep of
`research/experiments` and `research/LEDGER.md` for value-order impact rules
before writing this file). Learned move ordering (EXPLORED, saturated) used
corpus priors, not per-node propagation.

## Compute budget

2 cores, about 1.5 hours: R6 9 arms x 30 x 5 reps (seconds); R7 9 arms x 30,
up to about 30 s per censored run (about 40 min); R8 8 arms x 30 (about 15
min); check pass about 15 min. Order if time runs short: R8, R7, R6 main
passes, then the check pass; anything not run is reported as not run.

## Determinism

Instances fixed; `bb-mrv`, `vi-impact`, `vi-index` have no RNG; controls use
fixed run seeds 1, 2, 3; scb run seed 1. Node counts reproduce exactly unless
the guard fires; wall clock does not.
