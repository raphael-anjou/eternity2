# Dead-node refutation census

Night shift 2026-11-12, Track 6. **Label: measurement.** Preregistration:
`PREREG.md` (commit 41c069f, before any test-seed tree was sampled);
amendments: `PREREG_AMENDMENTS.md`. Generated boards only; nothing here is
Eternity II progress, and nothing here is a solver comparison.

**Result.** At the dead nodes `bb-mrv` keeps after propagation on R6 (6x6
c=5) and R7 (7x7 c=6), bounded-region checks prove few of them dead once each
node is weighted by its subtree size. At R7: 0.8% for exact cavities up to 3x3
(B-pool), 0.2% for 2-row bands (A), 1.7% for colour-only feasibility (E), 3.5%
for 2n-cell pockets (C), 10.3% for propagation partitioned on two cut edges
(D-cut), and 9.6% for two random edges. The mechanical stage-0 criteria
fire for T2 (cavities), T5 (bands), T3 (cut key) and ST (pocket lemmas); the
judge rules that three kills stand (T2 on K0a alone, T5, ST) and that T3 is
not decided at stage 0 (see the verdicts below). Every
share is an upper bound on node savings with unchanged branching. A gain claim
would also need the oracle's own work subtracted, which this census does not
do.

## What ran

- Trees: `bb-mrv` (A16 `BbSolver { rule: Rule::Mrv }`), replayed by a copy of
  its DFS with hooks (`compute/src/tree.rs`), cap 1e6, on hard-ladder test
  seeds 1..30 of R6 (primary n=6), R7 (primary n=7) and the null N7 (plus2
  n=7, c=8).
- Node identity gate: every instance row of every stage reproduces the
  committed `bb-mrv` row in status, nodes, propagation and backtracks: 450 rows
  (90 trees x 5 stages), 0 mismatches. Every replay pass has the same
  node-sequence hash as pass 1 (`hash`, `hash_p2`, `hash_p3` in the instance
  rows). Source: `results/summary.json` key `identity`.
- Sample: up to 200 dead nodes per tree, stratified by depth, seed per PREREG.
  R6: 13,966 dead nodes, 5,335 sampled. R7: 11,191,605 dead nodes (7 of 30
  trees censored at the cap), 6,000 sampled. N7: 78,220 dead nodes, 5,912
  sampled.
- Oracles B, A, E, D, C in that order, each its own stage and file
  (`results/run/<oracle>-<ladder>-n<n>.jsonl`), all on `m4` (Apple M4, 4
  threads), 20:34 to 21:12.

## Soundness gate

- 7,524 live states (planted path under bb-mrv's cell rule on all 90 trees,
  plus every node on the path to the solution bb-mrv found on the 83 solved
  trees): 0 violations for every oracle and arm
  (`results/gate/gate-*.jsonl`, `summary.json` key `gate`).
- Fuzz on 4x4 c=3 and 5x5 c=4 generated boards (`results/gate/fuzz-mac-local.jsonl`):
  cavity solver against naive enumeration 80,320 cases, band filter against
  naive band enumeration 41,913, colour DP against naive enumeration 11,903,
  all oracles against complete exact search 15,005 states (594 dead), the
  instrumented propagator against `Model::propagate` 16,539 steps: 0
  violations.

## Shares of sampled dead nodes proven dead

Mean over the 30 trees of the per-tree share, weighted by design weight times
subtree size (unweighted in brackets: design weight only). Source:
`results/tables.md`, `summary.json` key `rungs.<rung>.shares`.

| oracle | R6 | R7 | N7 (c=8 null) |
|---|---|---|---|
| B-pool r <= 3 (cavity, pool only) | 0.017 (0.054) | 0.008 (0.057) | 0.014 (0.057) |
| B-pool r <= 4 | 0.225 (0.510) | 0.061 (0.358) | 0.088 (0.305) |
| B-dom r <= 3 (cavity, propagated domains) | 0.108 (0.279) | 0.040 (0.240) | 0.088 (0.300) |
| B-dom r <= 4 | 0.508 (0.833) | 0.157 (0.674) | 0.249 (0.695) |
| A k=1 (control) | 0.000 (0.000) | 0.000 (0.000) | 0.000 (0.000) |
| A k=2 | 0.002 (0.006) | 0.002 (0.013) | 0.011 (0.043) |
| A k=3 | 0.011 (0.032) | 0.008 (0.049) | 0.045 (0.161) |
| E colour-only | 0.019 (0.058) | 0.017 (0.110) | 0.208 (0.602) |
| D-cut | 0.245 (0.553) | 0.103 (0.504) | 0.184 (0.557) |
| D-rand (control C1) | 0.251 (0.544) | 0.096 (0.480) | 0.164 (0.504) |
| D-poac, matched budget (control C2) | 0.144 (0.428) | 0.063 (0.415) | 0.149 (0.546) |
| D-poac on the MRV cell, unbudgeted | 0.037 (0.171) | 0.016 (0.167) | 0.038 (0.221) |
| C pocket (2n cells) | 0.070 (0.199) | 0.035 (0.225) | 0.056 (0.210) |

95% bootstrap intervals are in `results/tables.md`. The proven share rises
steeply with depth for every oracle (`by_depth_band_weighted`): at R7 D-cut
proves 0.9% of early, 27.7% of mid and 99% of late dead-node weight.

## Work per check (R7, medians)

Own unit; and wall time of the check over the wall time bb-mrv spent on the
node's subtree (from pass 1), which is the scale a gain claim would subtract.

| oracle | work | check ns / subtree ns |
|---|---|---|
| B-pool | 206,907 option tests | 3.84 |
| B-dom | 138,815 option tests | 1.15 |
| A k=2 / k=3 | 52,563 / 282,284 transitions | 0.83 / 2.03 |
| E | 20,480 transitions | 0.72 |
| C | 10,124 option tests | 0.39 |
| D-cut / D-rand / D-poac | 6 / 15 / 4 propagate calls | 0.50 / 1.00 / 0.43 calls per subtree node |

Budget exhaustions (counted as not proven): B-pool 212 cavities at R7, 829 at
R6, 779 at N7; E and C never overflowed.

## Recurrence (B-pool refuting keys, R7)

Among B-pool-refuted sampled nodes, the share whose refuting boundary key is
presented again at another tree node that is neither ancestor nor descendant:
raw 0.459 [0.413, 0.505], canonical 0.463; the share where the stored
refutation would also be sound (the other node's pool is a subset): raw 0.135,
canonical 0.135. Median count of other presenting nodes: 0 over all refuted
sampled nodes. C: 0.009 of refuted pockets recur among the other sampled nodes
(amendment 1: sampled nodes only).

## Stage-0 verdicts (mechanical, `summary.json` key `verdicts`)

| proposal | criterion | statistic | threshold | fires |
|---|---|---|---|---|
| T2 cavity summaries (GC) | K0a | R7 weighted share r <= 3: 0.008 | < 0.30 | yes |
| | K0b | R7 raw key recurrence: 0.459 | < 0.10 | no |
| | K0c | weighted median r: R7 inf, N7 inf | R7 not above N7 | yes (decides nothing: both medians are infinite) |
| T5 band consistency (TW) | census kill | k=2: R6 0.002, R7 0.002 | < 0.05 at both | yes |
| | mechanism | k=1 = 0 vs k=2 CI R6 [0.001, 0.004], R7 [0.001, 0.003] | k=1 inside CI at both | no |
| T3 cut-keyed partitioning (CO) | K2 | cut vs rand: R6 0.245 / 0.251, R7 0.103 / 0.096 | within 0.10 at both | yes (one-shot proxy, see ruling) |
| | K3 | R7 POAC 0.063 vs cut 0.103 | POAC >= cut | no |
| | K1 | replay with persistent partitions | not run here | not decided |
| ST boundary-word lemmas | local-core rate | R7 median over 30 trees: 0.000 | < 0.10 | yes |
| | projected saving | needs lemma matching | not run here | not decided |
| OC piece-priced frontier DP | none | E floor at R7: 0.017 | no kill preregistered here | n/a |

Judge's rulings (`JUDGE.md`). Killed at stage 0: T2, T5, ST.
- T2: criterion K0c compares two infinite medians (R7 and N7 both put most
  of the weight at r = inf) and decides nothing; the stage-0 kill rests on
  K0a alone.
- T3: K2 was evaluated on a one-shot weighted share at sampled nodes, with a
  random-key control (D-rand) not matched for partition count (median 16
  partitions against 6 for D-cut at R7). That is a proxy for the proposal's
  replay ratio, so T3 is not decided at stage 0: the kill does not stand. The
  replay with persistent partitions and a partition-matched control would
  decide it.

OC: not decided by this census; the lambda = 0 floor its K2 compares
against is 1.7% at R7 (11% unweighted).

## Reproduce

```
just experiments dead-node-census build
just experiments dead-node-census fuzz
just experiments dead-node-census gate 4
just experiments dead-node-census "stage B" 4     # then A, E, D, C
just experiments dead-node-census analyze         # results/summary.json, results/tables.md
```

Node and work counts are deterministic and machine-independent; wall times
are from `m4` and indicative only.
