# Cut-keyed trace partitioning: the deciding replay

Night 2026-11-12, Track 12. Label: method (stage-0 decision for proposal T3).
Preregistration: `PREREG.md` (commit 9375c26), amendments in
`PREREG_AMENDMENTS.md`. Every number below is in `results/summary.json`,
written by `analysis/analyze.py` from `results/run/primary-n{6,7}.jsonl` and
`results/check/check.jsonl`. Generated-board results are never official
Eternity II progress.

## Verdict (as preregistered; judged, kill stands, see `JUDGE.md`)

The proposal is killed at stage 0 by **K2**: a random key with the same number
of partitions, drawn off the cut line, prunes more nodes than the cut key at
both rungs. K1 does not fire (narrowly), K1-IDEAS fires on work, K3 does not
fire.

| criterion | statistic | value | threshold | fires |
|---|---|---|---|---|
| K1 | R7 upper median node ratio, cut | 0.486 [0.469, 0.500] | > 0.5 | no |
| K1-IDEAS | R7 upper median work ratio, cut (or K1) | 1.211 [1.169, 1.263] | > 1 | yes |
| K2 | median(C1) - median(cut), R6 and R7 | -0.039 (R6), -0.067 (R7) | <= 0.10 at both | yes |
| K2, literal two-sided | abs of the above | 0.039, 0.067 | <= 0.10 at both | yes |
| K3 | R7 median node ratio, POAC vs cut | 0.702 vs 0.486 | POAC <= cut | no |

## Replay ratios (upper median over 30 instances, 95% bootstrap CI)

Node ratio = surviving nodes / `bb-mrv` nodes on the same tree. Work ratio =
arm's propagation work units / `bb-mrv`'s (arc revisions + cells per
all-different sweep, instrumented copy of A3 propagation).

| arm | R6 node | R6 work | R7 node | R7 work |
|---|---|---|---|---|
| cut key | 0.483 [0.434, 0.504] | 1.392 [1.278, 1.474] | 0.486 [0.469, 0.500] | 1.211 [1.169, 1.263] |
| C1, matched random key (mean of 5 seeds) | 0.444 [0.409, 0.487] | 1.299 [1.214, 1.401] | 0.419 [0.414, 0.428] | 1.325 [1.294, 1.381] |
| C2, POAC on MRV cell, matched budget | 0.750 [0.730, 0.788] | 1.288 [1.236, 1.312] | 0.702 [0.673, 0.746] | 1.238 [1.206, 1.260] |
| C2u, POAC unbudgeted (diagnostic) | 0.676 [0.660, 0.716] | 1.300 [1.293, 1.327] | 0.637 [0.626, 0.642] | 1.286 [1.284, 1.289] |

- C1 per seed, R7 node medians: 0.418, 0.418, 0.418, 0.418, 0.420; R6: 0.437,
  0.440, 0.445, 0.457, 0.446. Every seed beats the cut key at both rungs.
- Paired per-instance difference C1 - cut: R7 median -0.059 [-0.067, -0.051],
  R6 -0.017 [-0.050, -0.005]. Difference of medians at R7: -0.067 [-0.081,
  -0.050]; at R6 -0.039 [-0.072, 0.011].
- POAC - cut (difference of medians): R7 +0.217 [0.176, 0.273], R6 +0.267.
- By censoring status at R7 (23 solved, 7 censored at 1e6): cut 0.495 / 0.476,
  C1 0.419 / 0.419, POAC 0.702 / 0.712.
- Propagate calls per `bb-mrv` call (median): R7 cut 1.28, C1 seed 1 1.34,
  POAC 1.27; R6 1.38, 1.18, 1.29. Wall per arm over the base recompute (one
  process carries all arms, indicative): R7 cut 1.25, C1 seed 1 1.35.

Partition diagnostics (pooled, R7): live partitions per surviving node, cut
2.26, C1 2.53 to 2.60; partitions made per split, cut 6.07, C1 5.89 to 5.92;
re-keys, cut 833,062, C1 about 1.23 to 1.25 million. C1 product mismatches
(no off-line pair with the cut key's partition count): 14,156 to 14,526 of
about 1.23 million selections at R7, 13 to 18 at R6. C2 spent 22.0 million
propagate calls against the cut arm's 22.4 million at R7 (budget held).

## Gates

- Node identity: 60 of 60 trees match the committed `bb-mrv` rows in status,
  nodes, propagation and backtracks, and the census node-sequence hash;
  `prop_mismatch` 0 at every tree node (the instrumented propagation equals
  `Model::propagate` in verdict and domains). `summary.json` `identity: []`.
- Soundness 1 (solved trees keep the solution) and 2 (planted path stays
  alive): 0 violations over 8 arms x 60 trees.
- Soundness 3 (exact search at every kill point on generated boards): 0
  violations; checked kill points per arm 1,425 (cut) to 1,954 (C2u).
- Monotonicity: no arm ever kept a partition alive where `bb-mrv`'s own
  propagation failed (`mono_total` 0).

## What this bears on

Small-board node savings of a persistent partitioned propagation state, with
`bb-mrv`'s branching held fixed. It does not bear on a 16x16 solution, and it
is not a search result: with branching held fixed the replay ratio is exact
(it would only be an optimistic estimate for a search that also branched on its
own domains), and the work ratio above 1 for every arm means none of them saves
propagation work on these trees.

## Corrections made after the judgment

Made after `JUDGE.md` (commit 1828059); no number or criterion changed.

- Wording: "the replay ratio is an upper bound on node gain with the same
  branching" was wrong. With branching held fixed the ratio is exact. It would
  only be an optimistic estimate for a real search that also branched on its
  own domains (edited in place above).
- Possible confound at R7: C1 re-keys more often than the cut key (about 1.23
  million against 0.83 million) and spends 9% more work (work ratio 1.325
  against 1.211), so part of its node edge at R7 may be the extra
  re-splitting. It does not rescue the proposal: at R6 C1 beats the cut key on
  nodes and on work, and at R7 the cut key is not better on nodes.
- Partition matching missed in 1.2% of C1 draws at R7 (14,156 to 14,526 of
  about 1.23 million selections found no off-line pair with the cut key's
  partition count; 13 to 18 at R6). Too few to flip a gap that holds on all 30
  R7 instances.
- Scope: under `bb-mrv`'s most-constrained-cell branching the cut line (the
  line below the lowest assigned row, inherited from the census) is close to
  an arbitrary line, so a cut key for a row-by-row solver is not tested here.

## Not done

Stage 1 (real search with partitions, wall clock, G1) was not run; the stage-0
kill makes it moot under the proposal's own rules. No growth track.

## Reproduce

```
just experiments cut-keyed-replay build
just experiments cut-keyed-replay check
just experiments cut-keyed-replay test 6 2
just experiments cut-keyed-replay test 7 12   # about 2 core-hours
just experiments cut-keyed-replay analyze
```

Runs: R6 on the orchestrating Mac (2 threads, 3 s), R7 on `x86` (12
threads, 769 s wall); summed per-tree wall 2.13 core-hours. Every test-seed
run is logged in `results/run-log.jsonl` by `../common/prereg_guard.py`.
