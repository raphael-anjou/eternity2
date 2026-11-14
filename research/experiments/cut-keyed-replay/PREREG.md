# Cut-keyed trace partitioning: the deciding replay (preregistration)

Night shift 2026-11-12, Track 12 (added by the lead at 00:30; not in
NIGHT_BRIEF.md). Written before any test-seed tree is replayed.

**Track label: method** (stage-0 decision for proposal T3; no solver is run,
the tree is `bb-mrv`'s own). Proposal text: `research/program/night/2026-11-12/
ideas-raw/compilers.md` and `IDEAS.md` section T3 on branch
`night/2026-11-12-lead`. The census (`research/experiments/dead-node-census/`,
Track 6) issued a T3 kill on a one-shot proxy with a control not matched for
partition count; its judge (`dead-node-census/JUDGE.md`, section 6, T3) ruled
that kill does not stand and named this run as the deciding one.

## Hypothesis (copied from the proposal)

On critical-colour ladder boards, `bb-mrv` loses because its per-cell domains
forget how edge colour interacts with piece availability. Conditioning its
propagation on two cut-edge colours, kept alive down the descent, closes a
large share of `bb-mrv`'s failing subtrees at their roots, and the gain comes
from partitioning on the cut, not from more propagation in general.

## Trees (baseline)

- Solver: `bb-mrv` = `backdoor_branching::BbSolver { rule: Rule::Mrv }` (A16
  code, A22 binding arm), deterministic. The tree walk is the census crate's
  copy (`dead-node-census/compute/src/tree.rs`), used by path dependency;
  no census file is edited.
- Instances (hard-ladder, unchanged): R6 = primary 6x6 c=5, R7 = primary 7x7
  c=6, test seeds 1..30 each, 60 trees. Cap 1,000,000 nodes (TEST_CAP). Node =
  one counted `try_assign` (PROTOCOL node rule, `NodeCounter`).
- **Node identity gate** (before any statistic): on every tree the walk must
  reproduce the committed `bb-mrv` row exactly in status, `nodes`,
  `propagation`, `backtracks` (rows: `mrv-baseline/results/check/
  bb-mrv-a16-primary.jsonl` seeds 1..20, `mrv-baseline/results/raw/bb-mrv/
  q1-r6r7.jsonl` seeds 21..30), and the FNV-1a hash of its node sequence must
  equal the census instance row's `hash` (`dead-node-census/results/run/
  D-primary-n{6,7}.jsonl`). Additionally, at every tree node the instrumented
  propagation copy used for the work column must return the same verdict and
  the same domains as `Model::propagate`. One mismatch: stop and report.
- Development and timing only on calibration seeds (`primary-calibration`
  6x6 and 7x7) and on generated 4x4/5x5 boards. Nothing is tuned.

## The replay

All arms replay the same `bb-mrv` tree in one pass: same cell choice (computed
on `bb-mrv`'s own domains), same value order. An arm keeps its own state, a
set of live partitions (each a full domain vector), pushed and popped with the
DFS. For a tree node v = (cell c, option o) whose parent node is u:

- If u is not reached by the arm, or u's arm state is dead, v is **removed**.
- Else if o is in no live partition's domain at c (value pruned by the join),
  v is **removed**.
- Else v is **surviving** (counted). Transfer: in every live partition whose
  domain at c holds o, set c to {o} and run full propagation (A3
  `Model::propagate` semantics: arc consistency, piece all-different, hidden
  singles); partitions that wipe out are dropped for the rest of the subtree
  and never recomputed. If none survives, v's state is dead.
- Merge directive: if c is an endpoint of a key edge, the live partitions are
  joined (per-cell union of domains) and a new key is taken from the joined
  state by the arm's key rule; the joined state is split into the new key's
  partitions, each restricted and propagated. An arm with no key (fewer than 2
  undecided edges) tries to take one at every surviving node.
- The root state is the root domains split by the arm's key rule.

**Replay node ratio** of an arm on a tree = surviving nodes / `bb-mrv` nodes
(of the capped tree). On censored trees both counts are over `bb-mrv`'s first
1e6 nodes; the ratio is reported with the censoring flag.

### Cut key (the arm under test)

Undecided edge: an internal edge between two empty cells whose colour set
(colours offered on that side by both cells, under the state's domains) has
size at least 2. Scan horizontal grid lines starting with the line just below
the lowest row holding an assigned cell (line 0 when none), then the lines
above it upward, then the lines below it downward; take the first line with at
least 2 undecided edges, and on it the 2 with the smallest colour sets (ties
by lower cell index). Partitions: one per pair of colours (c1 in S1, c2 in S2),
both edges' cells restricted to options showing that colour on that edge, then
propagated. This is the census D-cut rule (`dead-node-census/PREREG.md`), made
persistent.

### C1, partition-matched random key (control; 5 control seeds 1..5)

Everything as the cut arm, except the key edges. At each key selection the
arm computes what the cut rule would choose on its own current state (line y,
product T = |S1| x |S2|). It then draws the two key edges uniformly among
unordered pairs of undecided edges (horizontal or vertical) **not on line y**
whose product |S1'| x |S2'| equals T, so the split creates the same number of
partitions; if no pair matches, uniformly among pairs minimising |product - T|
(mismatches counted and reported). If the cut rule finds no key, C1 takes
none. Random stream: splitmix64 seeded by (instance seed, control seed),
advanced at each selection in DFS order. The per-instance C1 ratio is the mean
over the 5 control seeds; per-seed medians are reported too.

### C2, POAC on the MRV cell at matched propagation budget (control)

A single domain store (no partitions). Transfer as above. Then Partition-One
AC on the cell `bb-mrv` branches on next at this node: each value of that
cell's domain in the C2 store is assigned and propagated; failing values are
removed; if all fail the state is dead; otherwise the store becomes the
per-cell union of the surviving branches. Budget: a bank of propagate calls,
credited at every tree node with the number of propagate calls the cut arm
made at that node (0 where the cut arm did not reach it), debited by every C2
propagate call (transfer included); POAC runs at a node only if the bank holds
at least the cell's domain size. So C2's total propagate calls never exceed the
cut arm's on the same tree. **C2u** (diagnostic, decides nothing): the same
without a budget.

## What is counted

- Nodes: surviving nodes per arm by the replay rule above (the ladder's node
  definition applied to the arm's search with `bb-mrv`'s branching).
- Work, second column (tonight's reviews: node counts can hide per-node cost):
  every propagate call of an arm is run through an instrumented copy of A3's
  propagation that counts work units = arc revisions + (cells x all-different
  sweeps). `bb-mrv`'s own work is the same unit summed over its tree.
  **Replay work ratio** = arm work / `bb-mrv` work. Also logged: propagate
  calls, splits, re-keys, mean live partitions per surviving node, wall ns per
  arm (indicative only).

## Soundness gate (before any ratio is reported)

1. Solved trees: on every tree where `bb-mrv` finds a solution within the cap,
   every arm must keep the whole solution path surviving and its state alive
   at the solution node.
2. Planted path: from the root, choose cells by `bb-mrv`'s rule and assign the
   planted option; for every arm, the planted option must be in the arm's
   joined domain at every step and the arm must stay alive to the full board.
3. Exhaustive check (census oracle `tree::completes`, exact complete search):
   on generated 5x5 c=4 and 5x5 c=5 boards (seeds 1..20 each, not ladder
   instances), every node an arm declares dead must have no completion from
   `bb-mrv`'s state at that node.
4. One violation is a bug: that arm's rows are not used and the violation is
   reported.

## Kill criteria (copied from the proposal, readings fixed here)

Statistic: per rung, the **upper median** over the 30 instances (16th order
statistic, PROTOCOL's convention for an even sample) of the per-instance
replay ratio; 95% CI by percentile bootstrap over instances, 10,000
resamples, seed 20261111.

- **K1 (proposal):** the median replay node ratio of the cut arm at R7 is
  above 0.5.
- **K1-IDEAS (the IDEAS.md correction, also read):** the median replay work
  ratio of the cut arm at R7 is above 1, or K1 fires.
- **K2:** C1's median replay ratio is within 0.1 of the cut arm's at both R6
  and R7. Reading fixed here: K2 fires iff median(C1) - median(cut) <= 0.10 at
  both rungs (C1 at most 0.1 worse than the cut key, which includes C1 being
  better). The literal two-sided reading |median(C1) - median(cut)| <= 0.10
  is reported beside it. Reason: the proposal's stated meaning of K2 is "the
  cut plays no part", which a random key that beats the cut also shows.
- **K3:** C2 reaches a median replay ratio at or below the cut arm's at R7
  (reading fixed: R7, the rung of K1 and of the census reading; R6 reported).

The proposal is killed at stage 0 if K1, K2 or K3 fires. K1-IDEAS is reported
as a verdict of its own. Secondary, descriptive: paired per-instance
difference C1 - cut (median, bootstrap CI), per-seed C1 medians, solved counts
are unchanged by construction (soundness), all ratios by censoring status.

## Out of scope

Stage 1 (the real search with partitions, wall clock, G1) is not run. No
board above 7x7. Generated-board results are never official Eternity II
progress. This bears on small-board node savings of a propagation state; it
does not bear on a 16x16 solution.

## Caps, machines, budget

Node cap 1e6 per tree. Runs on `dell` (Linux x86_64, at most 12 threads,
code under `~/e2-night/t12`) or 2 cores of the orchestrating Mac; machine
recorded per row. Node and work counts are machine-independent. Budget 4
core-hours; estimate 1.5 to 3. Every test-seed command goes through
`research/experiments/common/prereg_guard.py`. Deadline 03:30 on 2026-11-13;
an arm not finished on all 60 trees by then is reported as not reached.

## Analysis

`analysis/analyze.py` reads the committed jsonl rows, applies the identity
and soundness gates, and writes `results/summary.json`; every number in the
README traces to it.
