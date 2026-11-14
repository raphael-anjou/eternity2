# Cavity summaries at the point-to-set scale (physics of glasses x compilers)

## Stall attacked

Critical-colour boards, where propagation stops paying. Quotes:

- `research/experiments/mrv-baseline/README.md` (A22, judged): "At the critical-colour rungs R6 and R7 G1 fails (0.80, 0.70) and `bb-mrv` is slower in wall clock (1.64x, 1.85x), so it is not adopted there." At R7 (7x7 c=6) bb-mrv solves 8/10 with a KM median of 528,610 nodes; 9x9 c=11 and 10x10 c=12 are "censored for both solvers on every pair even at the extended 1e7 / 180 s cap".
- `web/content/research/build/dead-ends.mdx` (Treat the mismatches as physical defects): what breaks "is the joint four-sided constraint over all remaining pieces at once. No single-cut, per-color, or pairwise-energy reduction captures it." Same page (Survey propagation): constraint influence "seems to 'fade out' about two cells away".

Structural reading: every inference the ladder solvers use (AC-3, GAColor, piece all-different, Hall matching from A17) is pairwise or global-count. A dead DFS node at the critical colour count is dead because of a joint constraint over a block of cells larger than one edge and smaller than the board, so the solver discovers it only by enumerating that block, and it rediscovers the same block refutation under every prefix that leaves the same block boundary. The mechanism attacks both halves: the scale at which a dead node becomes refutable, and the repetition of that refutation across prefixes.

## Fields and mechanism

**Physics of glasses contributes the scale and the test that a scale exists.** The point-to-set correlation length: freeze a configuration outside a cavity and ask from which cavity size on the frozen exterior determines (or rules out) the interior. Montanari and Semerjian proved that relaxation time is bounded below by a function of this length, which is the formal version of "local dynamics cannot see constraints longer than xi". Here the cavity is a k x k window of empty cells touching the placed region of a DFS node, the frozen exterior is that node's placed cells plus its unused piece pool, and the measured quantity is the refutation length r: the smallest k at which some window's cavity problem is infeasible. The glass picture predicts r is a finite scale at the critical colour count, above what pairwise propagation reaches (about one cell) and well below the board. That is a falsifiable statement about these boards, not a metaphor, and if it is false the mechanism has nothing to act on.

**Compilers contribute the reuse mechanism.** IFDS (Reps, Horwitz and Sagiv) computes each procedure's summary edge (entry fact to exit fact) once, on demand, and reuses it at every call site, independent of calling context. Here a cavity is the procedure, its boundary key B (the colours on placed edges facing the window, plus border flags and window shape and position class) is the entry fact, and the summary is "infeasible" or "feasible with witness filling F". The summary is reused across every DFS prefix that presents the same B, with one soundness rule that makes context-insensitive reuse valid despite piece distinctness: the pool only shrinks along a branch, so "infeasible with pool P0" implies infeasible with any P subset of P0, and "feasible with witness F" holds whenever F's pieces are all still unused. Otherwise the cavity is solved exactly by a small DFS and stored.

**Why neither alone.** The point-to-set length alone is a diagnostic: it says where refutations live but gives no way to avoid paying for them again at every prefix. Summary tabulation alone has no principled unit: keyed on the whole remaining subproblem it is no-good learning and subproblem caching (mapped, "no-goods outrun memory at 16x16", the pool makes keys unique); keyed on unconditioned clusters it is the cluster-enumeration dead end ("counts blow up before they help"). The glass scale is what bounds the key to a window whose boundary repeats across prefixes and whose cavity problem is small; the IFDS summary with pool-monotone soundness is what turns a one-off refutation into a reusable one.

Note on the import: IFDS's polynomial guarantee needs distributive transfer functions, which piece distinctness does not give. What is borrowed is the summary-edge construction and context-insensitive reuse, made sound here by pool monotonicity, not the distributive reduction.

## Original papers

- Physics of glasses: A. Montanari and G. Semerjian, "Rigorous inequalities between length and time scales in glassy systems", Journal of Statistical Physics 125, 23-54 (2006), DOI 10.1007/s10955-006-9175-y, arXiv cond-mat/0603018. Point-to-set length as used there follows J.-P. Bouchaud and G. Biroli, J. Chem. Phys. 121, 7347 (2004), DOI 10.1063/1.1796231.
- Compilers: T. Reps, S. Horwitz and M. Sagiv, "Precise interprocedural dataflow analysis via graph reachability", POPL 1995, pp. 49-61, DOI 10.1145/199448.199462.

## Hypothesis

At the critical colour count (ladder R6 6x6 c=5, R7 7x7 c=6), most dead nodes of bb-mrv's tree are refuted by some k x k cavity with k in 2..3 given the node's own boundary and pool, the same boundary keys recur across many prefixes, and a DFS that consults cavity summaries keyed on the boundary alone solves R7 in fewer total nodes (cavity sub-search nodes included) and less wall clock than the better of bb-mrv and scb.

## Nearest explored cell and missing mechanism

Nearest cells: no-good learning and shared transposition tables (mapped: no-goods outrun memory at 16x16), pattern databases and local cluster enumeration (mapped dead ends), and A20 (blocker eviction, proposed). The pattern-database entry itself says "Patch databases indexed by frontier colors and remaining pieces are untested here." Missing mechanism: a summary keyed on a bounded cavity boundary only (not the whole remaining subproblem, not an unconditioned cluster), made sound across prefixes by pool monotonicity, with the cavity size taken from a measured refutation (point-to-set) length rather than chosen. Not A11 (no replicas), not annealing or tempering (no stochastic dynamics at all), not a fill order (MRV is unchanged), not a speedup (node counts include every cavity sub-search node).

## Cheapest falsifying test (6x6 or 7x7)

Stage 0, a measurement, no new solver. On ladder R6 and R7 instances (fresh seeds, preregistered as a hard-ladder amendment like A4, since A16 and A22 used seeds 1..30), run bb-mrv to cap 1e6 and log every node. Classify a node dead when its subtree is exhausted without a solution. Sample up to 200 dead nodes per instance; for each, compute r, the smallest k in 1..n for which some k x k window of empty cells touching the placed region has an infeasible cavity problem (exact small DFS, current pool, window-internal distinctness), r = inf if none below the whole remaining board. Also log, for the smallest k covering half the refutations, how often each boundary key B recurs across distinct prefixes. Descriptive side measure: the planted point-to-set function, the fraction of k x k cavities uniquely filled given the planted exterior, per k.

Stage 1, only if stage 0 passes: four arms on the same R6 and R7 test seeds, cap 1e6 total nodes where each cavity sub-search node counts as a node:
1. bb-mrv;
2. bb-mrv + cavity lookahead at the stage-0 k, no table (isolates lookahead from reuse);
3. bb-mrv + cavity summaries (table with pool-monotone reuse);
4. bb-mrv + summaries keyed on random non-contiguous cell sets of the same size (matched control for the point-to-set geometry).
scb-gacolor_ac3_random runs as the second baseline.

## Baseline and control

- Baseline: the better of bb-mrv and scb-gacolor_ac3_random per rung, from the committed A22 and hard-ladder harness (at R6 and R7 neither clearly beats the other).
- Controls: arm 2 (lookahead without reuse) separates the compilers half; arm 4 (random-shape keys of equal size) separates the glass half. Null model for stage 0: the same r measurement on the A22 `plus2` track at 6x6 c=7 and 7x7 c=8, where propagation already pays; the glass reading predicts a smaller r there.

## Kill criterion

Stage 0 (kills the whole idea before any solver is written):
- K0a: at R7, fewer than 30% of sampled dead nodes have r <= 3 (no finite cavity scale; bounded summaries cannot fire).
- K0b: at R7, the share of cavity queries whose boundary key repeats across distinct prefixes is below 10% (no reuse; what is left is plain lookahead, not this proposal).
- K0c: r at R7 is not larger than r on the `plus2` null at 7x7 c=8 (the scale is not tied to the critical colour count, so it does not attack the stated stall).

Stage 1:
- Method kill: G1 NO-GAIN against the baseline at R7 with cavity nodes counted, or median paired wall-clock ratio above 1 at R7.
- Compilers-half kill: arm 3 not significantly better than arm 2 in nodes at R7 (paired, Holm over the two mechanism contrasts).
- Glass-half kill: arm 4 within 2 solves of arm 3 at R7 and not significantly worse in nodes.

## Budget

Stage 0: under 1 core-hour (logging plus small exact cavity solves at k <= 4 on 6x6 and 7x7). Stage 1: 1 to 3 core-hours, CPU only, no AWS. No 8x8 or larger until stage 1 passes at R7.

## Closest prior work

G. Chu, M. Garcia de la Banda and P. J. Stuckey, "Automatically exploiting subproblem equivalence in constraint programming", CPAIOR 2010, LNCS 6140, pp. 71-86, DOI 10.1007/978-3-642-13520-0_10: caching of remaining subproblems in CP search with automatically derived keys (and their follow-up on subproblem dominance). In this project: no-good learning and shared transposition tables (mapped), and the pattern-database and cluster-enumeration dead ends.

## Why this is not a variant of it

Chu et al. key on the entire remaining subproblem (by projection) and reuse its full solve result; on edge matching the remaining piece pool is part of that key, so keys almost never repeat, which is exactly the recorded stop of no-good learning here. This proposal keys on a bounded window's boundary only, reuses a refutation of a part rather than a solve of the whole, and gets soundness from pool monotonicity rather than from subproblem equivalence or dominance. The window size is not a free parameter: it is the measured refutation length, and stage 0 kills the idea if that length does not exist as a finite scale at the critical colour count. Pattern databases compute admissible bounds over patches at the root (no information there by the dead-end page's own computation); this computes exact infeasibility at interior nodes, under the node's own pool, which that page lists as untested.
