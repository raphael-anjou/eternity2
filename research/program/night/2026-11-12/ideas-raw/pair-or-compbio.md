# Pair OR x computational biology: piece-priced frontier DP with Carrillo-Lipman safe banding as a node bound

## Stall attacked (with the quote and its file)

Critical-colour boards, where local propagation saves little, and the size wall where every solver is censored.

- `research/program/AGENDA.md`, A22 result: "At the critical-colour rungs R6 and R7 G1 fails (0.80, 0.70) and `bb-mrv` is slower in wall clock (1.64x, 1.85x), so it is not adopted there." and "9x9 c=11 and 10x10 c=12 are censored for both solvers on every pair even at the extended 1e7 / 180 s cap".
- `research/experiments/cut-multiset-pruning/README.md`: "Above the critical colour count the baseline's propagation buys no node reduction on this rung." The one global check tried (Hall matching) gave a 0.90 node ratio at 6x6 c=5 and NO-GAIN at 7x7 c=6.
- `research/program/EXPLORED.md`, Propagation: "signal fades two cells out".

Structural reason: every pruning test in the ladder solvers is local (AC-3, GAColor, piece singles) or global but geometry-blind (Hall matching ignores where colours sit). At the critical colour count a partial board is locally extensible almost everywhere; it dies on a conflict between the geometry of the whole empty region and the use-once rule for pieces. No current test sees both at once. This proposal builds a node test that does, and makes its cost safe to cut.

## Fields and mechanism (what each field contributes)

Mechanism, one sentence: at a DFS node, compute an exact lower bound on a Lagrangian relaxation of "complete the empty region strictly" by a frontier dynamic program over colour profiles, with the use-once constraint priced by per-piece multipliers, and restrict that DP to profiles that survive Carrillo-Lipman projection bounds; a positive bound proves the node dead.

- **Operations research contributes the pricing of the global constraint and the validity of the bound.** Relax "each remaining piece used exactly once" into the objective with free-sign multipliers lambda_p (Fisher 1981). Relaxed problem: tile the empty region so all edges match, pieces may repeat, cost = sum over cells of lambda of the piece placed, minus sum of lambda_p. Any strict completion has cost exactly 0, so L(lambda) = min relaxed cost <= 0 whenever the node is alive; L(lambda) > 0 for any lambda is a proof that the node is dead. Subgradient on g_p = (uses of p) - 1, warm-started from the parent node. By Geoffrion's property the bound equals the LP over conv(colour tilings) intersected with the piece counts, strictly stronger than the matched-edge LP on the LP page (which relaxes the tiling too) and than chain decompositions (local-polytope strength), because the frontier DP handles every 2x2 cycle exactly.
- **Computational biology contributes the safe truncation that makes the exact DP affordable.** The frontier DP state is the colour profile on the cut (about c^(n-1) x 5^2 states: 78k at 6x6 c=5, 1.2M at 7x7 c=6, about 5e9 at 9x9 c=11). Carrillo and Lipman faced the same object: an exact DP on a joint lattice too large to fill, whose objective is a sum over projections. Their mechanism: any joint path projects onto each pairwise sub-lattice, so a joint lattice point whose projection's forward + backward cost exceeds the slack U - (sum of the other projections' optima) cannot lie on an optimal path, and is never generated. Here the projections are column strips of width w (w = 2 or 3) of the empty region, each solved exactly by its own small frontier DP under the same lambda (horizontal agreement between strips relaxed, so the sum of strip optima is a valid lower bound on the joint optimum). The slack is U = 0 (a live node has relaxed cost <= 0). A joint profile at row boundary r is enumerated only if the sum over strips of [fwd_k + bwd_k of its restriction to strip k] is <= 0. The pruned states are provably off every strict completion under the current lambda, so the banded bound is identical to the unbanded one.
- **Why neither alone gives it.**
  - OR alone is the Lagrangian-patched tensor contraction already on `build/dead-ends` ("Tensor-network counting", amendment paragraph): it solved generated 6x6 in five minutes, then hit memory at full scale, and "the truncation needed to fit may discard too much for the fix to keep meaning". Its truncation is lossy. With cheap inner steps instead (message passing plus per-piece Lagrangian), the same page records non-convergence at 4x4. The OR half has no safe way to shrink the state space.
  - Biology alone is Carrillo-Lipman banding of a colour-only DP. Without piece prices, lambda = 0, the relaxed problem is the colour-only tiling that the dead-ends page calls the blind spot ("satisfying the color adjacencies is necessary but nowhere near sufficient"). At the critical colour count almost every node has a colour-consistent completion, so the band keeps everything and the bound proves nothing. The slack only bites once the prices make the use-once rule cost something.
  - Together: the prices give the slack something to measure; the banding makes the exact inner step affordable without discarding anything a solution could use. This addresses the stated reason the tensor-network patch stopped.

## Original papers

- OR: Marshall L. Fisher, "The Lagrangian Relaxation Method for Solving Integer Programming Problems", Management Science 27(1):1-18, 1981, DOI 10.1287/mnsc.27.1.1 (verified; reprinted Management Science 50(12S), 2004, DOI 10.1287/mnsc.1040.0263).
- Computational biology: Humberto Carrillo and David Lipman, "The Multiple Sequence Alignment Problem in Biology", SIAM Journal on Applied Mathematics 48(5):1073-1082, 1988, DOI 10.1137/0148063 (verified).

## Hypothesis

At the critical colour count (ladder R6 = 6x6 c=5, R7 = 7x7 c=6), a substantial share of the dead nodes that the ladder DFS keeps after propagation are dead because geometry and piece use clash, and the piece-priced frontier bound proves them dead at depths where propagation does not. Applied at every node, it shrinks the tree to first solution by at least 2x against the strongest ladder DFS at R6 and R7, while Carrillo-Lipman banding keeps at most a few percent of the profile states, so the bound has a path to 9x9 instead of the c^(n-1) wall.

## Nearest explored cell and missing mechanism

- **Nearest cells:** "Tensor-network counting, lattice reduction" (mapped, dead end) with its Lagrangian amendment on `build/dead-ends`; "LP / ILP relaxations, relaxation bounds" (mapped: 420 to 440, ILP wall at 8x8); A17 Hall matching (kept, small gain at R6 only); A5 and A18 (transfer matrix on the frame only).
- **Missing mechanism:** a safe, solution-preserving truncation of the exact contraction (Carrillo-Lipman projection slack), which the tensor-network attempt lacked and which is the reason it recorded for stopping. Secondary missing piece: none of those attempts used the bound as a node-death test inside a propagating DFS; they were standalone solvers or score ceilings. The LP cell is not re-entered: the relaxation here keeps tilings integral by DP, so its bound is the convex hull of tilings, not the fractional-piece LP whose "fake matches" the LP page records.

## Cheapest falsifying test (6x6 or 7x7)

All on committed `hard-ladder` instances, test seeds 1..30, generated boards with a planted solution; nothing on 16x16.

- **Stage 0, soundness (minutes).** On 4x4 and 5x5 generated boards and on every node along the planted path at R6: the bound must never be positive on a live node, and the banded L must equal the unbanded L to floating tolerance. Any violation is a bug, not a result.
- **Stage 1a, signal at R6 (about 1 core-hour, unbanded).** Run the baseline DFS (`scb-gacolor_ac3_random`, run seed 1, cap 1e6) and record the tree. Label nodes dead (subtree fully explored without a solution) or alive. Sample up to 20 dead nodes per depth at depths 6, 12 and 18, all of which passed propagation. Run the bound (20 subgradient iterations from lambda = 0) and record which it proves dead, weighting each by its subtree size.
- **Stage 1b, tree size at R6 (about 5 core-hours, unbanded).** If 1a passes, evaluate the bound at every node of the recorded tree (warm-started lambda, 10 iterations) and compute exactly the tree to first solution with killed subtrees removed. The DFS order is fixed by the seed, so the pruned run visits a known subset. Node count = surviving nodes plus one node per bound evaluation (conservative). Report CPU next to it.
- **Stage 2, R7 with banding (about 4 core-hours).** Width-2 and width-3 strip projections. Same exact pruned-tree computation for nodes at depth <= D, D set so at most 2,000 evaluations per instance, against `scb-gacolor_ac3_random` and `bb-mrv`. Report the fraction of profile states the band keeps per evaluation. At the 9x9 c=11 root, report the band fraction only (descriptive, no kill).

## Baseline and control

- **Baselines:** `scb-gacolor_ac3_random` (the ladder DFS that wins at R6/R7) and `bb-mrv` (A22), same seeds, same node rule (`NodeCounter`).
- **Controls (each isolates one component):**
  1. lambda = 0, the colour-only frontier DP: isolates the OR half; the tensor-network blind spot.
  2. Lagrangian over row chains plus column chains only (no cycle-exact frontier): isolates the exact DP from LP-strength bounds.
  3. Hall matching at the same nodes (A17's check): the global check already measured, geometry-blind.
  4. Unbanded against banded on the same nodes: isolates the biology half. Its contribution is state count and CPU only, by construction.

## Kill criterion

Written before any code; the binding test is at R6 and R7 only.

- **K1 (no signal).** Stage 1a: subtree-weighted fraction of sampled dead nodes the bound proves dead below 0.3 at depths 6 and 12 combined, or not above the Hall-matching control by more than 2 standard errors.
- **K2 (OR half inert).** The lambda = 0 control proves dead within 2 standard errors of as many sampled nodes as the priced bound. The prices then add nothing and the idea is the colour-only blind spot.
- **K3 (exact DP inert).** The chain-decomposition control is within 2 standard errors of the frontier bound. The bound is then LP-strength, which is mapped.
- **K4 (no search gain).** The paired G1 median node ratio (evaluations counted) is above 0.5 against `scb-gacolor_ac3_random` at R6 in stage 1b, or NO-GAIN against the better of the two baselines at R7 in stage 2.
- **K5 (biology half fails).** At R7 the band keeps more than 25% of profile states at the median evaluation. The mechanism then has no route past c^(n-1) and is killed as a scaling method, even if K4 passes; a K4 pass is then recorded as a 6x6/7x7 finding only.
- **Not a kill at this stage, but a promotion gate:** the measured wall-clock ratio at R7 (bound CPU plus surviving nodes at the baseline's per-node CPU) must be at most 1 before any 8x8 or 9x9 run is requested.

## Budget

About 10 core-hours, CPU only, VM: stage 0 under 0.5 h, 1a about 1 h, 1b about 5 h, stage 2 about 4 h. One implementation session (Rust, path-dep on `hard-ladder` for instances, tree recording and `NodeCounter`) and one separate judge. No AWS.

## Closest prior work

1. The Lagrangian-patched tensor-network contraction recorded on `web/content/research/build/dead-ends.mdx` ("Tensor-network counting", amendment): exact contraction plus per-piece Lagrangian penalty, used as a standalone solver; solved generated 4x4 and 6x6, ran out of memory around 3 GB at full scale, with lossy truncation flagged as possibly fatal.
2. Same page, "Belief propagation" entry: message passing plus per-piece Lagrangian, non-convergent at 4x4.
3. A17 Hall matching (`research/experiments/cut-multiset-pruning/`): a global feasibility test inside the ladder DFS, small gain at R6, none at R7.
4. In the literature, Lagrangian relaxation with DP subproblems is standard OR (Held-Karp style bounds). Carrillo-Lipman bounds are standard in exact MSA (the MSA program of Lipman, Altschul and Kececioglu, 1989; not cited as a source here).

## Why this is not a variant of it

- Against (1): the missing mechanism is the one that page says stopped it. The truncation is no longer lossy; Carrillo-Lipman slack discards only profiles provably off every strict completion under the current prices, so the bound and the proof it gives are unchanged by the truncation. The use is also different: a death certificate at DFS nodes, judged by tree size against the strongest ladder DFS, not a standalone contraction solver.
- Against (2): the inner step is an exact DP, not message passing, which the dead-ends page itself names as the deciding factor ("The accuracy of the inner marginals is the whole game").
- Against (3): Hall matching sees piece counts but not geometry; this bound sees both, and control 3 measures the difference on the same nodes.
- Against the LP cell: the relaxation keeps tilings integral (convex hull of exact colour tilings), and control 2 kills the idea if it is no stronger than an LP-strength chain bound.
- Against A5/A18: those are exact transfer-matrix computations on the frame ring, without piece prices and outside search. A13, A14 and A9 are local-search or neighbourhood moves; this is a pruning test inside complete search.
