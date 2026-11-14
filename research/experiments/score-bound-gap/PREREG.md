# Score-bound gap: preregistration

Night shift 2026-11-12, Track 8 (added by the lead; specification: entry T4
of `research/program/night/2026-11-12/IDEAS.md`, proposal L1, novelty check
"lead L1", tournament corrections). Branch `night/2026-11-12-t8-score-bound-gap`.

**Track label: measurement** (stage 0 of a method proposal). No solver is
built and no search gain is claimed. The node sequence of no search is
changed, so the optimization hashing rule does not apply.

## Question

Can a relaxed decision diagram (DD) prove that a prefix of a break-tolerant
row-major search can no longer reach a target score? Stage 0 measures only
how tight the bound is: the gap between the DD upper bound and the true best
completion of the prefix, as a function of rows remaining and width W.

## Hypothesis

On 7x7 boards at the critical colour count, with 3 rows remaining, a width
W <= 4096 DD that relaxes piece distinctness bounds the best completion score
within a median of 4 matched edges, and clearly better (by more than 1 edge)
than the same DD at W = 1.

## Scoring convention

matched-edges, as in `engine::score_board` and `BreakSolver`: the score is the
number of matched interior edges (maximum 2n(n-1): 60 at 6x6, 84 at 7x7, 112
at 8x8). The rim is structural, never scored: a piece may sit at a cell only
in an orientation whose grey sides face exactly the rim (the hard rule of
`BreakSolver::break_cost`).

## The bound

Cells are filled in row-major order; a prefix fills cells 0..d-1 (d = (n-r)n
for r rows remaining). Layer t of the DD is cell d+t. A node holds

- `fr`: for each column x, the set of colours that the next unfilled cell of
  column x may see on its top side, and one set for the left side of the next
  cell (exact nodes have singleton sets);
- `all`: pieces used on every path to the node; `some`: pieces used on some
  path (both include the prefix pieces);
- `val`: the largest number of remaining edges matched on a path to the node.

Root: the prefix's exact frontier, `all = some =` prefix pieces, `val = 0`. At
cell `pos`, a piece is unavailable if it is in `all`, or, when |`some`| equals
`pos` (the number of filled cells), if it is in `some`. A transition places
an available piece in any structurally valid orientation, gains 1 for the top
side if its colour is in the column's set (rows >= 1) and 1 for the left side
if its colour is in the left set (columns >= 1), and replaces the column set
and the left set by the singletons of the piece's bottom and right colours.
Bound = prefix score + max `val` over the last layer.

**Merge rule (exact).** After a layer is generated, nodes with identical
(`fr`, `all`, `some`) are combined (max of `val`; this is no relaxation). If
more than W nodes remain: step 1, all nodes with identical `fr` are merged
(this relaxes pieces only). If more than W nodes still remain: step 2, nodes
are sorted by `val` descending, then by `fr` ascending (lexicographic on the
nine u16 masks); the first W-1 are kept and all the others are merged into one
node (this relaxes frontier colours). A merge of nodes u_1..u_k gives
`fr` = union per slot, `all` = intersection, `some` = union, `val` = max.

**Proof that the bound is valid for every W.** Let C be any completion of the
prefix (distinct unused pieces, valid orientations) with score S(C) = prefix
score + matched remaining edges. Claim: for every layer t there is a node v in
the layer such that the first t cells of C are *admitted* by v: (a) for every
slot, the true colour C shows there lies in v's set; (b) every piece of
v.`all` is used by the prefix plus C's first t cells (call this set U_t, of
size d+t), and U_t is a subset of v.`some`; (c) v.`val` >= the number of
remaining edges matched by C among its first t cells. At t = 0 the root
admits it with equality. Step: if v admits C at t, the piece p of C at cell
d+t is not in U_t, so it is not in v.`all` (by b); and if |v.`some`| = d+t then
U_t, a subset of v.`some` of the same size, equals it, so p is not in
v.`some`. Hence p is available at v, and the transition placing p in C's
orientation exists. Its gain is at least C's matched edges at that cell
(by a, a matched side has its colour in the set), and the child admits C at
t+1 (sets are replaced by C's own colours; `all` grows by p, which is in
U_{t+1}; `some` grows by p). Combining identical states keeps the max `val`.
A merge node m of u_1..u_k with u_i admitting C also admits C: its sets are
supersets, `all` is a subset of u_i.`all` (subset of U_t), `some` is a
superset of u_i.`some` (superset of U_t), and `val` >= u_i.`val`. So a node of
the last layer has `val` >= the remaining edges matched by C, and the bound is
>= S(C) for every C, in particular for the best one. Nothing in the argument
depends on W or on which nodes are merged, so the bound is valid for every W
and every merge order. When no merge happens (W at least the exact width) the
diagram is the exact state graph and the bound equals the optimum (checked by
the self-test).

**Controls.** W = 1 (one node per layer, so no frontier or piece information
survives a layer with more than one child: a pure counting bound), and the
trivial bound "prefix score + every remaining edge".

## The truth

`oracle.rs`, independent of `dd.rs` (it imports nothing from it): iterative
deepening on k, the number of mismatched remaining edges. For k = k0, k0+1,
..., a row-major DFS over unused pieces looks for a completion with at most k
mismatches, pruned by two lower bounds on future mismatches (colour counting
with distinctness kept and geometry relaxed; and a column whose next cell
needs a top colour that no unused piece can show at that cell). The first
feasible k is the optimum: truth = prefix score + remaining edges - k. The
witness completion is rescored independently. Node cap per prefix (all
iterations summed): **1e9 for r <= 3, 1e8 for r >= 4**. A capped prefix is
censored: its truth is only known to be <= truth_ub = prefix score + remaining
edges - (largest k proven infeasible + 1).

Verified before PREREG on calibration-free generated boards: `sbg selftest 40`
compares oracle, DD (W in {1,2,3,8,64,512,4096}) and an unpruned exhaustive
search on 240 random prefixes of 4x4 and 5x5 boards with 1 or 2 rows left
(oracle = brute force on all; DD >= brute force on all; unmerged DD = brute
force on all; trivial bound loose on 236, DD at W=1 loose on 204, at W=4096 on
15).

## Instances, prefixes, seeds

Instances: the hard-ladder test files, seeds 1..30 per configuration (never
used by this experiment before this commit; development used the calibration
files only). Rungs, in run priority order:

| rung | file | n | c (interior colours) |
|---|---|---|---|
| 6x6 critical | `primary-test.jsonl` | 6 | 5 |
| **7x7 critical (decision)** | `primary-test.jsonl` | 7 | 6 |
| 6x6 critical + 2 | `plus2-test.jsonl` | 6 | 7 |
| 7x7 critical + 2 | `plus2-test.jsonl` | 7 | 8 |
| 8x8 critical | `primary-test.jsonl` | 8 | 8 |
| 8x8 critical + 2 | `8-c10-test.jsonl` | 8 | 10 |

Prefixes (`sample.rs`): `BreakSolver` (the repo's engine, unmodified) on a
row-major path, two sources: **schedule** (one licensed break cell per row
y = 1..n-1, at column (3y) mod n) and **strict** (no break cell: prefixes
perfect so far, alive or dead). For each instance and source, 4 runs (run
seeds 1,2,3,4; each relabels the pieces by a seeded shuffle so the DFS
explores a different subtree), 1,000,000 `step(1)` calls each. Every
placement that completes a row with r rows left (r = 1..n-1) adds the prefix to
the pool of stratum r (deduplicated). K = 4 prefixes per (instance, source,
stratum) are drawn uniformly without replacement, sampling seed 20261112 mixed
with the instance seed, r and the source. So 8 prefixes per instance per
stratum, 240 per stratum per rung (fewer if a pool is smaller; reported).

## Measured quantities

Per rung, rows remaining r, W in {1, 64, 512, 4096}, and the trivial bound:
gap = bound - truth (median, quartiles, share at 0, at <= 1, 2, 4); for targets
T = max, max-1, max-2, max-4 (max = 2n(n-1), reached by the planted board),
the share of dead-for-T prefixes (truth < T, or truth_ub < T when censored)
that the bound prunes (bound < T); work (DD nodes, transitions) and wall
time; frontier-relaxation share = nodes absorbed by step 2 / nodes absorbed by
steps 1 and 2, and the share of layers with a step-2 merge; oracle coverage
(share of prefixes not censored). Results are reported pooled over the two
sources and split by source.

## Soundness gate

Any prefix with a finished oracle where bound(W) < truth, or any bound above
the trivial bound, is a bug: the run stops (`measure` exits with status 3) and
the experiment reports the violation instead of gaps.

## Kill criteria (population: 7x7 critical test prefixes with r = 3, both sources pooled)

Medians are the usual median (mean of the two middle values for an even
count). Each is computed twice: M_lo with censored prefixes at their gap lower
bound (bound - truth_ub) and M_hi with censored prefixes at +infinity.

- **K1 (the idea is killed at stage 0):** median gap > 4 edges at every W in
  {1, 64, 512, 4096}. Fires if M_lo(W) > 4 for every W. Does not fire if
  M_hi(W) <= 4 for some W. Otherwise: undetermined at the coverage reached.
- **K2 (the diagram adds nothing):** median gap(W=1) - median gap(W=4096) <= 1.
  Fires if it holds under both M_lo and M_hi; does not fire if it fails under
  both; otherwise undetermined.

## What a pass would and would not mean

A pass says the bound is tight on 7x7 prefixes three rows from the end. It
says nothing yet about the 16x16 board (frontier of 16 edges over 22 colours)
and nothing about search gains (stage 1, not run here). The only part of the
result that bears on larger boards is how the gap changes from 6x6 to 7x7 (and
8x8 if reached) at a fixed number of rows remaining; it is reported as such.
Generated-board results are never official Eternity II progress.

## Analysis plan

`analysis/analyze.py` reads `results/test/*.jsonl` and writes
`results/summary.json` and the tables of the README. No other test-seed
analysis decides the verdict.

## Budget and machines

Remote only, per the lead's quotas: `mac-m1` 2 threads, `dell` 2 threads after
about 21:10; each process at a stated thread count. Estimate: about 10 to 15
thread-hours in total, dominated by censored oracle runs at r = 3 and the
r >= 4 strata. Wall clock target: results by 02:00. If time runs out, rungs are
dropped from the end of the table above and reported as not reached.
