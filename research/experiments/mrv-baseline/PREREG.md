# A22 preregistration: bb-mrv as a hard-ladder baseline

Written and pushed before any run (no instance of this test has been solved
by any arm in this session). Branch `devin/r3-mrv-baseline`, created from
`origin/devin/r2-backdoor-branching` (A16, PR #24). Any forced change goes in
`PREREG_AMENDMENTS.md` (what, why, when); this file stays unchanged.

## Origin and hypothesis

Unplanned side observation in A16 (`../backdoor-branching/JUDGE.md`): `bb-mrv`
(A3's `sat_backdoors::Model::propagate`: per-cell option bitsets, edge arc
consistency, piece all-different with piece singles and hidden singles, to
fixpoint; forced move = lowest-index cell with |D| = 1 first; otherwise free
MRV, min (|D|, index); option-index value order) beat the ladder baseline
`scb-gacolor_ac3_random` (border-first MRV, random value order run seed 1,
AC-3, GAColor, class balance) on test seeds 1..20 at R6/R7/R8. Those seeds are
used and are not evidence here.

- H1 (Q1): on fresh seeds bb-mrv needs fewer nodes than scb AND is not slower
  in wall clock, at 8x8 c=10.
- H2 (Q2): the paired node ratio bb-mrv/scb shrinks as n grows on a fixed
  above-critical colour track (smaller effective branching factor), rather
  than staying flat (constant factor).
- Q3 (descriptive only): which component carries the gain (ablations).

## Instances (generator call unchanged)

All from `hard_ladder::instance::generate(n, c, seed)` =
`generate_solved_framed_bc(n, c + 5, seed, true, 5)` + the ladder scramble.
New ladders are added to hard-ladder's `Ladder` enum and `cli.rs`
(`check-instances` covers them), recorded as hard-ladder
`PREREG_AMENDMENTS.md` A4 before any test run. Existing ladder files stay
byte-identical (checked by `check-instances` and `git diff`).

| set | n | c | seeds | file |
|---|---|---|---|---|
| Q1 R6 | 6 | 5 | 21..30 (test) | `primary-test` (existing, seed subset) |
| Q1 R7 | 7 | 6 | 21..30 (test) | `primary-test` (existing, seed subset) |
| Q1 R8 | 8 | 10 | 21..30 + 31..50 (30) | `8-c10-test` subset + new `8-c10-fresh-test` |
| Q2 | 6 | 7 | 1..30 test, 1001..1010 calibration | new `plus2-*` |
| Q2 | 7 | 8 | same | new `plus2-*` |
| Q2 | 8 | 10 | 21..50 (the Q1 R8 set) | as above |
| Q2 | 9 | 11 | same as 6 | new `plus2-*` |
| Q2 | 10 | 12 | same as 6 | new `plus2-*` |

Colour rule for the Q2 track, fixed now: c(n) = `colours(Primary, n)` + 2
(primary = nearest integer to Owen's criterion: 5, 6, 8, 9, 10 for n = 6..10),
giving 7, 8, 10, 11, 12. No colour search. The 8x8 member reuses the Q1 R8 set
(its c = 8 + 2 = 10 matches the rule). Calibration seeds are used only to check
timing and correctness (cap 2e6, bb-mrv and scb); they enter no comparison
and no decision below.

## Arms

Binding pair: `bb-mrv` (A16's `BbSolver { rule: Rule::Mrv }`, unchanged code)
and `scb-gacolor_ac3_random` (hard-ladder's `Scb` wrapper around the
single-core-benchmark `EngineSolver::gacolor_ac3_random()`, run seed 1).
Reference: `hp-dfs` (plain row-major DFS). Both binding arms are driven through
a wrapper in this directory that only adds a wall-clock guard (below); with
the guard not firing, node counts equal the unwrapped solvers (checked against
committed hard-ladder `scb`/`hp-dfs` rows on R6/R7 seeds 21..30 and A16 `bb-mrv`
rows where seeds overlap).

Ablations (exploratory, Q1 instances only, labelled exploratory; each changes
exactly one thing from its parent):

- `mrv-nohs`: bb-mrv without the piece-side all-different rules (hidden
  singles, and the "piece with no remaining cell" wipeout). Kept: rim unary,
  edge arc consistency, piece singles (a cell whose domain holds one piece
  removes that piece from every other cell; needed so no piece is used twice).
  Implemented as a reimplementation of `Model::propagate` with a flag; a test
  checks that flag on equals `Model::propagate` exactly.
- `mrv-bf`: bb-mrv with border-first order: forced cells first as in bb-mrv,
  then min (class, |D|, index), class corner 0 < edge 1 < interior 2 (scb's
  `BorderFirstMrv` key).
- `mrv-rand`: bb-mrv with scb's random value order: per node, Fisher-Yates
  over the option list with scb's generator (`next_random`, splitmix64 state
  seeded from run seed 1, j = next % (i + 1)).
- `scb-ad`: scb with A3's piece all-different added: a verbatim copy of the
  single-core-benchmark `solver-engine` with one optional hook after AC-3:
  repeat to fixpoint {a piece with no unplaced cell able to take it -> wipeout;
  hidden single: a piece with exactly one such cell -> restrict that cell to the
  piece's rows; piece single: a cell whose domain holds one piece -> remove it
  from every other unplaced cell; rerun AC-3 after any change}. Hook off must
  reproduce `scb-gacolor_ac3_random` node for node (tested on R6/R7 21..30).
  If this fork cannot be made correct within the budget, it is dropped and the
  drop is recorded in `PREREG_AMENDMENTS.md`; no other arm replaces it.

## Node rule (same for every arm)

One node = one `NodeCounter::try_assign`, i.e. one (cell, piece, rotation)
assignment the search makes, counted before propagation, whether propagation
refutes it or not. Every cell is placed by the search exactly once per path:
forced cells (|D| = 1) and cells reduced to one option by hidden singles are
placed as explicit search steps and count one node each, in every arm alike
(bb arms place them first; scb places them when its order reaches them).
Propagation never places a cell by itself without a node. Lookahead does not
exist in any arm.

## Caps and wall clock

- Cap 1,000,000 nodes per test run for every arm and set (hard-ladder
  `TEST_CAP`).
- Wall-clock guard 600 s per run on every arm at cap 1e6 (expected never to
  fire; if it fires, the run is time-censored, reported by name, and a pair
  with a time-censored baseline and a solved candidate scores 0, as in A16).
- Extended cap rule (Q2): at any Q2 size where fewer than 10/30 instances are
  solved by both bb-mrv and scb at 1e6, also run that size at 10,000,000 nodes
  for bb-mrv and scb only, with a 180 s guard per run (budget bound), same
  time-censoring rule. Both caps are reported.
- Wall clock: 8 concurrent runs on the same VM (8 vCPU), one arm at a time.

## Analysis (stdlib Python, `analysis/analyze.py`)

Per set and arm: solved/N with Wilson 95%, KM quartiles of nodes (censored =
+inf), median and total wall ms, nodes per second, propagation calls per node
(bb arms: `Model::propagate` calls; scb arms: one `place_and_propagate` per
node by construction, plus scb's native pruned-row count as a separate unit),
time-censored count.

G1 (PROTOCOL.md exactly): paired log ratio r_i bb-mrv vs scb (solved/solved
ln(c/b); solved/censored ln(c/C); censored/solved +inf; both censored 0),
upper median, 95% percentile bootstrap of the median, 10,000 paired resamples,
seed 20261111. PASS iff exp(median) <= 0.5 and CI upper end < 0. Reported at
R6, R7 (10 pairs, descriptive) and R8 (30 pairs, binding), and for the
ablations vs their parent (descriptive).

Wall clock: median of the paired ratios wall_bb / wall_scb (all pairs,
censored runs use their measured time), 95% percentile bootstrap CI (10,000,
seed 20261111).

Q2: per size, the paired log10 node ratio with the same censoring table
(log10 instead of ln), its upper median and bootstrap CI, and the number of
both-censored pairs. Slope statistic: OLS slope of the per-size upper-median
log10 ratio against n over n = 6..10 (5 points). CI: 10,000 bootstrap
replicates (seed 20261111), resampling instances with replacement within each
size independently, recomputing the 5 medians and the slope; a replicate with
any +inf median has slope +inf. Binding slope uses the cap 1e6 rows at every
size. Secondary (reported, not binding): the same slope with the 1e7 rows
substituted at sizes where the extended cap ran.

## Decision rules (binding)

- Kill: bb-mrv is NOT adopted as a ladder baseline if G1 is not PASS at 8x8
  c=10 on seeds 21..50, OR if the median paired wall-clock ratio bb-mrv/scb
  there is above 1. Otherwise: adopted (survives).
- Q2 claim: "the gain grows with n" only if the 95% CI of the binding slope
  lies entirely below 0; otherwise "no evidence of a growing gain".
- Q3: descriptive only; no keep/kill rides on it.

Scope: generated boards, n = 6..10 only; nothing here is official E2
progress; no extrapolation to 16x16 beyond what the slope supports.

## Compute budget

CPU only, Devin VM, 8 vCPU, no AWS. Estimate: binding Q1 runs a few minutes;
ablations about 20 min; Q2 at 1e6 about 25 min; extended cap at most
2 sizes x 2 arms x 30 x 180 s / 8 = 45 min; total about 90 min. If time runs
out, the order is: binding Q1, Q2 at 1e6, extended cap, ablations; anything
not run is reported as not run.

## Determinism

Instances and seeds are fixed. bb arms have no RNG except `mrv-rand` (fixed
seed); scb arms use run seed 1. Node counts reproduce exactly unless a guard
fires; wall clock does not.
