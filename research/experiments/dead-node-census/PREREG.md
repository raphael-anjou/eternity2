# Dead-node refutation census: preregistration

Night shift 2026-11-12, Track 6 (added by the lead; not in NIGHT_BRIEF.md).
Specification: `research/program/night/2026-11-12/IDEAS.md` ("Shared stage 0:
dead-node refutation census") and `ideas-raw/TOURNAMENT.md` section 3(a), on
branch `night/2026-11-12-lead`. Written before any test-seed tree is sampled.

**Track label: measurement.** This census decides stage 0 of five proposals.
It is not a solver comparison and claims no node gain. Every share it reports
is an upper bound on node savings with unchanged branching; a gain claim needs
the oracle's own work subtracted (PROTOCOL target 2), which this census does
not do.

## Question

At the dead nodes that `bb-mrv` keeps after propagation at the critical-colour
rungs, how many could a check over a bounded region of empty cells prove dead,
at what cost in work, and how often does the same refutation recur?

## Trees

- Solver: `bb-mrv` = `backdoor_branching::BbSolver { rule: Rule::Mrv }` (A16
  code, A22 binding arm), deterministic. The census crate path-depends on
  `sat-backdoors` (A3 `Model::propagate`), `backdoor-branching` (`options`,
  `assign`) and `hard-ladder` (instances, `NodeCounter`). It carries a copy of
  `BbSolver`'s DFS for `Rule::Mrv` with tree logging added (node ids, depths,
  subtree sizes, state snapshots). Neither the A22 crate nor `NodeCounter` is
  edited.
- Node identity gate (before any statistic is reported): on every instance the
  copy must reproduce the committed `bb-mrv` row exactly in status, `nodes`,
  `propagation` and `backtracks`. Committed rows: primary n=6,7 seeds 1..20 in
  `mrv-baseline/results/check/bb-mrv-a16-primary.jsonl`, seeds 21..30 in
  `mrv-baseline/results/raw/bb-mrv/q1-r6r7.jsonl`; plus2 n=7 seeds 1..30 in
  `mrv-baseline/results/raw/bb-mrv/q2-plus2.jsonl`. Additionally the per-seed
  hash of the node sequence (cell, option of every counted assignment) is
  logged so that every later pass can prove it replays the same tree. One
  mismatch: stop and report.
- Instances (hard-ladder, unchanged): R6 = primary 6x6 c=5, R7 = primary 7x7
  c=6, null N7 = plus2 7x7 c=8 (above critical), test seeds 1..30 each, 90
  trees. Cap 1,000,000 nodes (TEST_CAP). Development only on calibration seeds
  1001..1010 and on generated 4x4 and 5x5 boards.
- Node = one counted `try_assign` (PROTOCOL node rule). Node id = the counter
  value at that call (preorder). Subtree size s(v) = number of counted nodes in
  v's subtree including v.
- **Dead node** (the sampled population): a node whose propagation succeeded
  and whose DFS subtree returned exhausted (no solution, not cut by the cap).
  Nodes refuted by their own propagation are not in the population ("dead ends
  that `bb-mrv` keeps after propagation"). Nodes open at the cap are neither.

## Sample

- Up to 200 dead nodes per instance; all of them when there are at most 200.
- Strata: depth (number of assigned cells). Equal allocation: quota
  q = floor(remaining / remaining strata); a stratum with fewer than q nodes is
  taken whole and the rest is reallocated, iterated to a fixed point;
  leftovers go one each to the deepest strata. Within a stratum, uniform
  without replacement by a splitmix64 stream seeded with
  `20261112 ^ (n << 40) ^ (c << 32) ^ seed`.
- Design weight w(v) = N_depth / n_depth.
- Per instance and oracle: weighted share = sum w s 1[proven] / sum w s;
  unweighted share = sum w 1[proven] / sum w.
- Per rung: mean over instances that have at least one dead node (the count is
  reported), 95% CI by percentile bootstrap over instances, 10,000 resamples,
  seed 20261111.

## Oracles

Each oracle receives the node's propagated domains `d` (A3 option bitsets),
its assigned-cell set, and its unused pool (pieces not on an assigned cell).
Each logs its own work in its unit below and its wall time in ns. Built and
run in this priority order, each as its own stage with its own committed rows:
B, A, E, D, C. An oracle not run on all 90 trees by 01:00 is reported as not
reached, never estimated.

**B. Exact k x k cavities, k = 2, 3, 4.** Windows: every k x k block of cells
inside the board that holds at least one empty cell, where some empty cell of
the block is edge-adjacent to an assigned cell. Cavity problem: give every
empty cell of the window an option, all window-internal edges match, every
piece used at most once in the window. Two variants:
- B-pool (primary, the proposal's own definition): an empty cell's candidates
  are the options of unused pieces that agree with the rim and with every
  assigned neighbour (inside or outside the window). This problem depends only
  on the boundary key and the pool, so its refutation is reusable under pool
  monotonicity.
- B-dom (upper reading): candidates are the node's propagated domains.
Solver: exact DFS with dynamic fewest-candidates cell choice and a piece
bitmask; work unit = option tests. Budget 200,000 option tests per cavity;
an exhausted budget counts as not proven and is logged.
r = the smallest k in {2, 3, 4} with an infeasible window; r = inf if none.
**Boundary key** of a refuting window: (k, the oriented edges of every
assigned cell in the window, and for every one of the 4k outer edge slots the
facing colour if the outside cell is assigned or the slot is rim, else open).
Raw key adds the window position; canonical key drops it and takes the minimum
over the 4 rotations. The key of node v is that of its first refuting window
at k = r (row-major scan). **Recurrence** of v: the key is presented (by some
window of the same size) at another propagation-surviving tree node u that is
neither an ancestor nor a descendant of v. Reported: share recurring (raw and
canonical), multiplicity, and pool-monotone reuse (a recurring u whose pool is
a subset of v's pool, where the stored refutation is sound).

**A. Band consistency, k = 1, 2, 3.** Every band of k consecutive full rows
and every band of k consecutive full columns. Cells keep their propagated
domains; piece distinctness is relaxed inside the band. Generalised arc
consistency on the band by a forward and a backward pass over its column
stacks (states = k-tuples of the colours on the edges between two stacks).
Proven dead if some band admits no filling. Work unit = stack transitions
evaluated. Also logged: share of band-cell values the passes remove, and the
rows-only result. k = 1 is the control (a one-row band is a path, on which the
existing arc consistency is already exact, so k = 1 should prove nothing).

**E. Colour-only feasibility.** Is there a filling of every empty cell from
its propagated domain with all edges matching, pieces reusable? Exact
broken-profile frontier DP in row-major order (state = the n open edge colours
plus the last east colour). Work unit = state x option transitions. Budget
4,000,000 states per layer; overflow counts as not proven and is logged.

**D. Partitioned propagation.**
- D-cut (compilers.md rule): undecided edge = an internal edge between two
  empty cells whose colour set (colours offered on that side by both cells)
  has size at least 2. Scan horizontal grid lines starting with the line just
  below the lowest row holding an assigned cell, then the lines above it
  (upward), then the lines below it (downward); take the first line with at
  least 2 undecided edges, and on it the 2 with the smallest colour sets (ties
  by lower edge index). For every pair of colours, restrict both edges'
  cells to options showing that colour on that edge and run A3
  `Model::propagate`. Proven dead if every partition wipes out.
- D-rand (control C1): the same with 2 undecided edges drawn uniformly
  (splitmix64 seeded by the node id and the instance seed) among undecided
  edges not used by D-cut.
- D-poac (control C2): budget B = the number of `propagate` calls D-cut made
  at this node (B = 0 when D-cut found no key). Cells in bb-mrv's order
  (forced first, then fewest options, then index); a cell is tried only if its
  whole domain fits in the remaining budget; each value is propagated; proven
  dead if every value of some tried cell fails. Also logged: unbudgeted POAC on
  the MRV cell.
- Work unit = `propagate` calls.

**C. Exact pockets.** During the replay, every failed propagation inside the
subtree of a sampled node is re-run through an instrumented copy of the A3
propagator (the A22 `Prop` copy with hidden singles on, which is tested equal
to `Model::propagate`) to find the cell whose domain emptied; wipe counts per
cell accumulate over v's subtree. Pocket: start at the empty cell with the
most wipes (ties: lower index); grow by adding the edge-adjacent empty cell
with the most wipes (ties: lower index) up to 2n cells or until no empty
neighbour remains. Exact check as B-pool (pool candidates, distinctness).
Budget 1,000,000 option tests. Logged: proven, pocket size, canonical key
(cells normalised by translation, minimum over 4 rotations, with the boundary
word) and its recurrence as for B (canonical only).

## Soundness gate (before any share is reported)

1. Planted path: for every instance, from the root, choose cells by bb-mrv's
   rule and assign the planted option, propagating; every state on the way is
   live. Also every tree node on the path to the solution bb-mrv found. Every
   oracle runs on every such node; none may declare it dead.
2. Fuzz against brute force: B and C cavity solver against naive enumeration
   on random 2x2 and 2x3 windows (both verdicts must agree); A against a naive
   band enumeration (relaxed distinctness) on 2-row bands of 4x4 boards (both
   directions); E against naive whole-board enumeration without distinctness on
   4x4 boards; all oracles on random reachable states of 5x5 c=4 boards against
   a complete exact search (proven dead must imply no completion).
3. One violation is a bug: that oracle is stopped and reported, its rows are
   not used.

## Readouts per oracle and rung (R6, R7, N7)

Weighted and unweighted share of sampled dead nodes proven dead (mean, 95%
CI), median work per check (own unit) and median wall per check, the same
divided by the node's subtree size (work scale), and for B and C the key
recurrence rates. Distribution of r for B. Also by depth band.

## Stage-0 kill thresholds, mechanical (copied from IDEAS.md and TOURNAMENT.md section 4)

**T2 Boundary-keyed cavity summaries (pair-glasses-compilers, GC).** Uses
B-pool. Killed if any holds:
- K0a: at R7, the weighted share of sampled dead nodes with r <= 3 is under
  0.30.
- K0b: at R7, the key recurrence (raw key, weighted share among B-pool-refuted
  sampled dead nodes) is under 0.10.
- K0c: r at R7 is not above r at N7: the weighted median of r (inf ordered
  last) at R7 is at most the weighted median at N7 (pooled over instances,
  weight w s normalised per instance).
B-dom is reported as an upper reading and decides nothing.

**T5 Band consistency (tiling-words, TW).** Uses A. Killed if:
- census kill: the k=2 weighted share is under 0.05 at both R6 and R7;
- mechanism kill (census version): the k=1 weighted share lies inside the 95%
  CI of the k=2 share at both R6 and R7.

**T3 Cut-keyed trace partitioning (compilers, CO).** Uses D. Killed if:
- K2: the D-rand weighted share is within 0.10 of the D-cut share at both R6
  and R7;
- K3: the D-poac weighted share is at or above the D-cut share at R7 (POAC at
  a matched budget proves at least as much).
- K1 (replay node and work ratio with persistent partitions) needs the full
  replay, which this census does not run: not decided here.

**ST Boundary-word lemmas (pair-synthesis-tiling).** Uses C. Killed if the
median over R7 instances of the local-core rate (unweighted share of sampled
dead nodes with s >= 200 that C proves dead) is under 0.10. The projected node
saving criterion needs lemma matching, which this census does not run: not
decided here. Canonical recurrence is reported.

**OC Piece-priced frontier DP (pair-or-compbio).** The census gives E only,
the lambda = 0 floor (OC control 1). No OC kill is decided here; E's share is
reported as the floor its K2 compares against.

## What is not in scope

No new solver, no node gain claim, no wall-clock comparison of solvers, no
8x8 or larger board. Generated-board results are never official Eternity II
progress. Baseline solvers: the tree is bb-mrv's own and is re-run here (node
identity gate); scb is not part of a census.

## Caps, machines, budget

Node cap 1e6 per tree. Oracle budgets as above. Runs on `dell` (Linux x86_64)
with at most 6 threads and only what the other tracks leave free (12 busy
threads in total), and `mac-m1` with at most 3; machine recorded per row.
Node and work counts are machine-independent; wall times are indicative.
Estimated budget: 3 to 5 core-hours. Deadline 01:00 on 2026-11-13.

## Analysis

`analysis/analyze.py` reads the committed jsonl rows and writes
`results/summary.json` and `results/tables.md`; every number in the README is
traced to one of them.
