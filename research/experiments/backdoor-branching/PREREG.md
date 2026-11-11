# Preregistration: backdoor-guided branching (round 2)

Written and pushed before any experiment of this study runs. Any forced
change goes in `PREREG_AMENDMENTS.md` (what, why, when); this file is never
edited after the push.

## Question

Round-1 test A3 (`research/experiments/sat-backdoors`, branch
`devin/r1-sat-backdoors`) found that on planted generated boards 6x6 to
10x10 fixing about N^0.59 planted cells lets propagation (rim unary, edge arc
consistency, piece all-different with singles and hidden singles, to
fixpoint) complete the board (b = 0.589, 95% CI [0.549, 0.626]). Those
backdoors were found with the solution in hand. This study asks:

1. Stage 1 (predictability): can backdoor membership be recognised from
   features computable at the root without the solution, better than the
   fail-first (domain-size) score?
2. Stage 2 (search): does a DFS that branches on the predicted backdoor cells
   first build a smaller search tree than dynamic fail-first (MRV) and
   impact-based selection with the same propagation, on the hard-instance
   ladder?

Hypothesis: yes to both; the learned score beats fail-first in within-board
AUC, and branching by it halves the median paired node count on the ladder.

## Novelty check (what the repo has already measured)

From `research/program/EXPLORED.md` ("Fixed / dynamic fill orders, comb
search": saturated; "Corpus priors, learned move ordering": saturated) and
the hard-ladder calibration (`research/experiments/hard-ladder/CALIBRATION.md`,
14 candidates on calibration seeds):

- Fixed orders: row-major (`hp-dfs`, `scb-naive_rowmajor`), spiral-in,
  border-first, snake.
- Dynamic orders: border-first MRV with class balance (`scb-border_first_lcv`,
  `_full`, `_random`), `scb-rare_color_first` (a no-op preset that behaves as
  its base), and border-first MRV with gacolor and AC-3 propagation
  (`scb-gacolor_ac3`, `_ns1`, `_lcv`, `_random`). The calibrated best baseline
  is `scb-gacolor_ac3_random`.
- Community: Zajc's 8x8 head-to-head (groups.io msg 5918, cited on
  `build/backtracking/fill-order`) found minimum-remaining-values ahead of
  scanline ahead of a max-reduction order.
- Learned move ordering in the repo (PRIOR, PALIMPSEST and the corpus-prior
  pages) learns value/placement preferences from record boards for 16x16
  score hunting, not a branching-cell rule for exact search.
- Impact-based search (Refalo, CP 2004): `grep` finds no measurement in the
  repo. It is therefore added here as a baseline.

The mechanism this study adds over those cells: a cell score trained on
measured strong backdoors (A3), so a win has to come from the backdoor signal
and not from dynamic ordering in general. Both close relatives (MRV and
impact) run with exactly the candidate's propagation, DFS skeleton, value
order and node counter.

## Propagation and DFS skeleton (shared by every `bb-*` solver)

- State: one bitset of (piece, rotation) options per cell (distinct
  orientations only), as in A3's `Model` (`sat-backdoors/src/lib.rs`, used as
  a path dependency, unchanged). Propagation `P` is A3's `Model::propagate`:
  rim unary, edge arc consistency on every shared edge, piece all-different
  with piece singles and hidden singles, iterated to fixpoint.
- DFS: propagate; on contradiction backtrack. If some unassigned cell has a
  singleton domain, assign the lowest-index such cell (forced move). Otherwise
  select a cell by the solver's rule among unassigned cells (all have
  |D| >= 2). Values in option index order (piece index, then rotation).
  Every assignment, forced or branching, calls `NodeCounter::try_assign`
  first (PROTOCOL.md node metric). Assigning sets the cell's domain to the one
  option and propagates from that cell. Lookahead probes (below) are never
  counted as nodes and never prune: their results only score cells, so every
  `bb-*` solver has exactly the same pruning power. Probe propagations are
  counted in `counter.propagation`.
- Wall-clock guard: a `bb-*` run that exceeds 240 s stops and returns
  `CapReached` (recorded as `censored` with `nodes_counted` < cap; called
  time-censored). Handling in the analysis: a time-censored candidate is
  censored (as PROTOCOL.md). A pair where the baseline is time-censored and the
  candidate solved scores 0 (tie), not ln(c/C), so the guard can never help
  the candidate.

## Solvers

| name | cell rule |
|---|---|
| `bb-mrv` | dynamic fail-first: minimum \|D(x)\|, ties lowest cell index |
| `bb-impact` | Refalo-style impact: minimum S(x) = sum over o in D(x) of exp(L_o - L), L = sum over cells of ln\|D\| before the probe, L_o after probing x = o with `P` (S term 0 if the probe fails); ties by \|D\| then index |
| `bb-learned` (variant V1) | maximum learned logistic score over the features below, z-scored over the candidate cells of the current node; ties lowest index |
| `bb-single` (variant V2) | maximum of the single non-domain-size feature with the highest Stage-1 training AUC, with the sign chosen on training; ties lowest index |

Ladder baselines (PROTOCOL.md): `scb-gacolor_ac3_random` (calibrated best)
and `hp-dfs`, committed rows at R6 and R7, run here at R8.

## Features (computed at a node over candidate cells x, unassigned, |D| >= 2)

1. `ln_dom`: ln \|D(x)\|.
2. `corner`, 3. `edge`: position class indicators (interior is the reference).
4. `frame_adj`: interior cell adjacent to the frame ring.
5. `degree`: number of in-board neighbours not yet assigned (at the root: the
   in-board neighbour count).
6. `rarity`: mean over o in D(x) of the mean over o's non-grey sides of
   1/m(colour), m = number of half-edges of that colour in the puzzle.
7. `impact`: mean over non-failing probes o in D(x) of (L - L_o) (one-step
   lookahead with `P`; 0 if all fail).
8. `fail_frac`: fraction of o in D(x) whose probe fails.

Control (reported, never used in a model): raw cell index (A3's exact search
enumerates subsets in lexicographic order, which favours low indices).

## Stage 1 design

- Data: A3's committed backdoors (`sat-backdoors/results/results.jsonl`,
  field `backdoor_cells`), 300 instances (regimes A and B, n = 6..10, seeds
  1..30), boards rebuilt exactly as A3 does:
  `generate_solved_framed_bc(n, border + interior, seed, true, border)` with
  A3's colour table. Features at the root (after `P`).
- Label: cell in the committed backdoor set. Candidate cells: all cells with
  \|D\| >= 2 at the root (A3 reports no root singletons).
- Split: train seeds 1..15, test seeds 16..30 (both regimes, all sizes; 150
  instances each).
- Model: logistic regression on within-instance z-scored features (sd 0 gives
  0), L2 penalty 1e-3 on weights, batch gradient descent from zeros, 3000
  steps, step 0.5, Python standard library. Deterministic.
- Statistic: mean over test instances of the within-instance AUC
  (Mann-Whitney, ties 0.5) for the learned score and for the fail-first score
  -\|D\|. Delta = AUC_learned - AUC_failfirst. 95% CI: percentile bootstrap
  over test instances, 10,000 resamples, seed 20261111.
- Also reported: AUC of every single feature (train and test), vs 0.5 with
  bootstrap CI; domain size with the sign chosen on training (a stricter
  comparison, not the kill test); the cell-index control; results split by
  A3's `k_source` (exact vs greedy upper bound) and by regime.

## Stage 2 design

Rungs (all with the ladder's harness, instances, node counter, cap):

| rung | n | c (interior) | generator | test seeds | calibration seeds | cap |
|---|---|---|---|---|---|---|
| R6 | 6 | 5 | ladder primary | 1..30 | 1001..1010 | 1,000,000 |
| R7 | 7 | 6 | ladder primary | 1..30 | 1001..1010 | 1,000,000 |
| R8 | 8 | 10 | `generate_solved_framed_bc(8, 15, seed, true, 5)` + ladder scramble (`hard_ladder::instance::generate(8, 10, seed)`), ladder file `8-c10`, checked by `check-instances` | 1..30 | 1001..1010 | 1,000,000 |

Sample-size rule (fixed now): after the calibration runs of every solver at
every rung, project the test wall clock as sum over (solver, rung) of
3 x the calibration total wall (30 seeds vs 10) / 8 threads. If the
projection exceeds 50 min, every test run uses seeds 1..20 instead of 1..30
(the same subset for every solver; committed R6/R7 baseline rows are then
restricted to seeds 1..20). The decision and the projection are recorded in
the README.

Calibration seeds: correctness, timing and the sample-size rule only. No
weight, feature or rule is changed after Stage 1 other than through an
amendment.

Test looks: each variant (V1 `bb-learned`, V2 `bb-single`) runs once on the
test seeds of each rung; both runs are reported. Baselines `bb-mrv`,
`bb-impact` run once on the test seeds; `scb-gacolor_ac3_random` and `hp-dfs`
run once at R8 (committed rows at R6, R7).

### G1 per rung (PROTOCOL.md) with Holm over the two variants

Paired log-ratios r_i as in PROTOCOL.md (plus the time-guard rule above).
Statistic: upper median of r_i; ratio = exp(median). Bootstrap: 10,000
paired resamples of instances, seed 20261111 (fresh generator per
comparison). Bootstrap p-value p = min(1, 2 x fraction of resampled medians
>= 0). Holm at alpha = 0.05 over the two variants within each (rung,
baseline): the smaller p must be <= 0.025, the larger <= 0.05 and only if
the smaller rejected. A variant's CI is "below 0" iff Holm rejects.

Labels per (variant, rung, baseline): PASS = Holm rejects and ratio <= 0.5;
GAIN = Holm rejects and ratio > 0.5; NO-GAIN = Holm does not reject.

The strongest dynamic-ordering baseline: the label against the strongest
baseline at a rung is the worst label of the variant against `bb-mrv`,
`bb-impact` and `scb-gacolor_ac3_random` at that rung (PASS > GAIN >
NO-GAIN), i.e. the variant has to beat each of them. Labels against
`hp-dfs` and the unadjusted 95% CIs are reported too. PROTOCOL.md's dropping
rule (G1 at rung 6, then 8) is reported for the record; all three rungs run
regardless.

## Kill criterion (fixed now)

The idea is **killed** if any of:

- K1 (Stage 1): the lower end of the 95% bootstrap CI of Delta =
  AUC_learned - AUC_failfirst (mean within-instance test AUC) is <= 0.
- K2 (Stage 2 at 8x8): neither variant is PASS at R8 against the strongest
  dynamic-ordering baseline (as defined above).
- K3: both variants are NO-GAIN against the strongest dynamic-ordering
  baseline at all three rungs.

Otherwise it **survives**. A variant that is GAIN but never PASS is reported
as "promising, needs a variant" when K1 and K2 do not fire (K2 fires
whenever R8 is not PASS).

## Metrics reported per rung and solver

Solved/30 (or /20) with Wilson 95%, KM median and quartiles of nodes, median
and total wall clock (ms, 8 concurrent runs, same VM), propagation calls,
time-censored count, and the G1 table per baseline.

## Compute budget

About 90 minutes of wall clock in total on the Devin VM (8 vCPU), no AWS, no
paid services: Stage 1 feature dump (minutes), calibration (about 10 min),
test grid (at most about 50 min by the sample-size rule), analysis (minutes).

## Determinism

Instances and seeds are fixed; every `bb-*` solver is deterministic (no RNG),
so node counts reproduce exactly. The wall-clock guard is the one
nondeterministic element: it can only fire on runs that take over 240 s, and
such runs are reported by name.
