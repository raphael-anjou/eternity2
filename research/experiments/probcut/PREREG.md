# ProbCut statistical cutoffs for exact search: preregistration

Written 2026-11-11 before any calibration or test run. Branch
`devin/r2-probcut`, created from `devin/r2-hard-ladder` (85c5bf2). The kill
criterion below is fixed; any forced change goes in `PREREG_AMENDMENTS.md`
with what, why and when, keeping this text.

## Hypothesis

Buro's ProbCut (ICCA Journal 1995, DOI 10.3233/ICG-1995-18202) and
Multi-ProbCut (Buro 1999) cut a subtree when a shallow value predicts, with a
calibrated error model, that the deep value misses the bound. Exact Eternity II
search has no evaluation, but it has a shallow value: v_k(node) =
log2(1 + number of forward-checked nodes exactly k cells deeper) under the fixed
row-major fill order. Claim: v_k (k = 1..4) separates planted-path nodes from
their siblings beyond v_0 (the count of surviving immediate extensions that
forward checking already sees), enough that cutting nodes with v_k below a
calibrated threshold t(n, d, k, p) (planted-path cut probability p), with the
relaxation schedule p = 0.2, 0.05, 0.01, 0 (the last pass is the complete
DFS), reaches a first solution in fewer total nodes (probe nodes and failed
passes included) than the same DFS without cuts and than the ladder's best
baseline.

## Novelty check against `research/program/EXPLORED.md`

- Break-tolerant DFS (saturated): allows scheduled mismatches for record
  hunting; nothing here allows a mismatch. Exact first-solution search only.
- Restart portfolios, heavy tails (saturated): restarts escape early wrong
  commitments blindly. ProbCut keeps one deterministic DFS and decides per
  node, from a probe, whether to skip a subtree. A Luby-restart version of the
  same DFS is run as a baseline so the two are compared directly.
- Learned move ordering, corpus priors (saturated): those reorder children;
  ProbCut does not reorder, it cuts, and its threshold has a stated false-cut
  rate measured on planted labels.
- Beam search (saturated): beam prunes by width or score without a calibrated
  error rate and is incomplete. ProbCut cuts by a calibrated test and the
  relaxation schedule ends with a complete pass.
- Nearest mapped cell, arc consistency / Régin ("pruning fades about two
  cells out"): those cut only on proven emptiness. The missing mechanism is a
  statistical cut, calibrated on planted labels, that uses subtree-size
  information k cells out.

## Rungs, instances, caps (hard-ladder harness, `PROTOCOL.md`)

| rung | n | c | instances | cap | baselines |
|---|---|---|---|---|---|
| R6 | 6 | 5 | `hard-ladder/instances/primary-test.jsonl` (n = 6) | 1,000,000 | committed `scb-gacolor_ac3_random`, `hp-dfs` rows |
| R7 | 7 | 6 | same file (n = 7) | 1,000,000 | committed rows |
| R8 | 8 | 10 | `probcut/instances/8-c10-test.jsonl` | 1,000,000 | run here, same harness |

R8 instances: exactly the ladder's call `hard_ladder::instance::generate(8,
10, seed)` = `generate_solved_framed_bc(8, 15, seed, true, 5)` plus the
`XorShift(seed ^ 0xA5A5A5A5)` scramble; ladder name `8-c10`; checked by a
`check-instances` subcommand that regenerates and compares bytes. Test seeds
1..30 for every rung (30 >= 20, no cut needed). Calibration seeds 1001..1010
(R6/R7 from `primary-calibration.jsonl`, R8 generated the same way).
Node metric: `NodeCounter::try_assign` once per (cell, piece, rotation)
assignment, including every probe placement and every placement of a failed
pass. A candidate run is one `LadderSolver::solve` call; all passes share one
counter and the cap.

## Solvers (all in `probcut/compute`, plug-in interface)

- `fc-dfs` (the "same DFS", baseline 1 of the idea): row-major fill, candidate
  order piece index then rotation (identical orientations skipped, as hp-dfs),
  forward checking after each placement: the right neighbour and every pending
  cell of the next row must keep at least one unused consistent oriented piece.
- `fc-luby` (baseline 2 of the idea): `fc-dfs` with Luby restarts and a
  randomised value order (random cyclic start in each candidate list), run seed
  1; Luby unit chosen on calibration from {100, 1000, 10000} nodes by lowest KM
  median calibration nodes (ties: smaller unit).
- `scb-gacolor_ac3_random` (the ladder's calibrated best baseline, comparison
  baseline for G1) and `hp-dfs`.
- Candidate `probcut-v1`: `fc-dfs` plus probes at test depths d in {n, 2n, 3n}
  (filled-cell counts). At a node with d cells filled, count nodes exactly k
  cells deeper (probe stops early once the count reaches 2^t - 1, since the
  node then passes); cut the subtree when v_k < t(n, d, k, p). Schedule p =
  0.2, 0.05, 0.01, 0.
- Candidate `probcut-v2`: same probe and k, schedule p = 0.05, 0.

Threshold model (Buro's Gaussian form): t(n, d, k, p) = mu - z_p * sigma of
v_k over the planted-prefix nodes at depth d on the 10 calibration seeds of
that rung; z_0.2 = 0.8416, z_0.05 = 1.6449, z_0.01 = 2.3263; p = 0 means no cut.
k is selected per rung on calibration as the k in 1..4 with the highest pooled
(over the three depths) within-sibling AUC; ties go to the smaller k. The
selected k and thresholds are committed in `CALIBRATION.md` before any test
run. Labels: a node is on the planted path when its oriented edges equal the
planted board's edges in every filled cell (identical pieces are therefore
interchangeable).

## Stage 1 (prediction, calibration seeds only)

At each rung and d in {n, 2n, 3n}: the parent is the planted prefix with d - 1
cells; its children are the forward-checked children at cell d - 1 (sibling
set, kept if it has >= 2 children). For every child compute v_0..v_4 and
L = log2(1 + nodes exactly 8 cells deeper), the count capped at 2,000,000
nodes per child (capped values kept, flagged). Report:
(a) within-sibling AUC of v_k for the planted child (ties 0.5), mean over
sibling sets, with a shuffled-label control (planted label moved to a random
sibling, seed 7, 1000 shuffles); difference AUC(best k) - AUC(v_0) with a paired
percentile bootstrap 95% CI over sibling sets (10,000 resamples, seed 20261111);
(b) OLS of L on v_k over all children at that depth: R^2 and residual sigma;
(c) held-out false-cut rate of t(d, p) for p in {0.05, 0.2}: fit mu and sigma on
seeds 1001..1005, count planted-prefix v_k < t on 1006..1010, and the reverse;
report the pooled rate over the 10 held-out values.

## Primary metrics (test seeds, per rung)

Nodes to first solution (all passes and probes), solve count /30 with Wilson
95%, KM median and quartiles, median and total wall clock (8 runs in parallel,
same VM), first-pass solve rate, number of passes used, false-cut rate (the
fraction of test instances where the planted path fails at least one cut test
of pass 1, computed offline from planted labels after the runs; used for
reporting only).

G1 per `PROTOCOL.md` against `scb-gacolor_ac3_random` (upper median of paired
log-ratios, percentile bootstrap 10,000 resamples, seed 20261111, via the
ladder's `analyze.py` functions). Labels per rung:
PASS = median ratio <= 0.5 and CI upper end < 0; GAIN = CI upper end < 0 and
ratio > 0.5; NO-GAIN = CI includes or exceeds 0.
Two looks (v1, v2), each run once on test seeds. Holm: one-sided bootstrap
p_v = fraction of resamples with median log-ratio >= 0; smallest p compared
with 0.0125, the other with 0.025 (the 0.025 one-sided level matches the
protocol's two-sided 95% CI). Labels are reported unadjusted and Holm-adjusted;
the verdict uses the Holm-adjusted labels. G1 is also reported against
`fc-dfs`, `fc-luby` and `hp-dfs` (secondary, no verdict weight). G2 is reported
as computed by `analyze.py compare` on R6/R7 (the colour sets differ at R8, so
G2 over 6..8 is descriptive only).

## Kill criterion (fixed)

Stage-1 kill (the idea's own, R8 = 8x8): killed if, on R8 at every tested
depth d in {8, 16, 24}, the AUC gain of the best k over v_0 is < 0.05 or its
paired bootstrap 95% CI includes 0; OR the best-k regression R^2 < 0.2 at
every tested depth on R8; OR the pooled held-out false-cut rate on R8 exceeds
2p for p = 0.05 or for p = 0.2.

Stage-2 kill (the idea's own, adapted to the rungs available: its "8x8 and
9x9" becomes R7 and R8): killed if the KM median probe-inclusive nodes of
v1 is not <= 0.7x the better (lower KM median) of fc-dfs, fc-luby and
scb-gacolor_ac3_random on both R7 and R8 (a censored candidate median fails);
OR v1's first pass (p = 0.2) solves at most 15/30 on R8.

Round rules (from the lead): killed if NO-GAIN (Holm-adjusted, best of the two
variants) on all three rungs. G1 at R8 is part of the kill as the lead wrote it
("the idea's own kill criterion AND fails G1 at 8x8"); read together with the
lead's rule that GAIN without PASS is "promising, needs a variant", this
preregistration resolves it as: a NO-GAIN at R8 for both variants kills; a GAIN
at R8 without PASS is "promising, needs a variant" unless the idea's own
criterion fires; a PASS at R8 with no own criterion firing is "survived".
The stage-1 and stage-2 kills are evaluated independently: stage 2 runs even if
stage 1 fires, because the lead asked for all three rungs.

Solve rate guard: an incomplete pass that solves fewer instances is not a win;
because the last pass is complete, a loss of solves shows as censoring at the
cap and enters G1 as +infinity.

## Budget

About 90 minutes wall clock on the Devin VM (8 vCPU, 31 GiB). Expected:
calibration and stage 1 under 15 min; test runs: 3 rungs x 30 x (v1, v2,
fc-dfs, fc-luby) plus R8 scb-gacolor_ac3_random and hp-dfs, each capped at
1e6 nodes, under 15 min with 8 runs in parallel. No AWS, no paid services.
