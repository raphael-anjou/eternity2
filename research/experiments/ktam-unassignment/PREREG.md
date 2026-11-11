# A15 preregistration: non-chronological, bond-weighted unassignment (round 3)

Written and committed before any experiment run, calibration included. The
kill criterion below is binding and is never changed after results are seen;
any forced change goes to `PREREG_AMENDMENTS.md` (what, why, when) and this
file stays as committed.

## 1. Hypothesis

Growing the board by stable cooperative attachments and detaching weakly
bonded pieces anywhere (self-assembly kinetics: Winfree's kinetic Tile
Assembly Model, E. Winfree and R. Bekbolatov, "Proofreading Tile Sets", DNA
Computing 2003), or evicting the pieces that block a cell (B. R. Rau,
"Iterative modulo scheduling", MICRO-27, 1994, DOI 10.1145/192724.192731),
solves hard-band boards that chronological DFS does not, at the same event
budget.

## 2. Novelty check against `research/program/EXPLORED.md`

- Fixed / dynamic fill orders (saturated): they change which cell is filled
  next; every undo is still chronological (the most recent decision). Here the
  undo target is chosen by bond count (V1) or by the blocking relation at the
  dead cell (V2), anywhere on the board, and the rest of the partial board is
  kept.
- Break-tolerant DFS (saturated): it accepts mismatched seams for record
  hunting. Here the partial board is always mismatch-free (every placement
  agrees with every placed neighbour and the rim); only exact full solutions
  count.
- Min-conflicts local search, ALNS, annealing (saturated): they move on
  complete boards with mismatches and minimise a conflict count. Here the
  state is a consistent partial board; the search never holds a mismatch.
- Restart portfolios (saturated): restarts drop the whole board. Here a dead
  end drops a few pieces.
- Repair seeded by exact prefix (mapped, `repair-study`): repair on complete
  boards with mismatches.
- Nearest prior literature outside this repo: incomplete dynamic backtracking
  (S. Prestwich, "Combining the scalability of local search with the pruning
  techniques of systematic search", Annals of OR 115, 2002) and decision
  repair (N. Jussien and O. Lhomme, "Local search with constraint
  propagation and conflict-based heuristics", AIJ 139, 2002) unassign
  variables non-chronologically, mostly at random. That generic mechanism is
  arm B3 here, so the test separates the specific choice (bond count, blocking
  relation) from non-chronological unassignment in general.

Missing mechanism: unassignment chosen by bond count or blocking relation,
not by recency.

## 3. Rungs, instances, generator calls, seeds

| rung | n | interior c | instances | test seeds | calibration seeds |
|---|---|---|---|---|---|
| R6 | 6 | 5 | `../hard-ladder/instances/primary-test.jsonl` / `primary-calibration.jsonl`, n = 6 | 1..30 | 1001..1010 |
| R7 | 7 | 6 | same files, n = 7 | 1..30 | 1001..1010 |
| R8 | 8 | 9 | `instances/8-c9-test.jsonl` / `8-c9-calibration.jsonl` (this directory) | 1..30 | 1001..1010 |

R8 instances use exactly the ladder's call `hard_ladder::instance::generate(8,
9, seed)` = `generate_solved_framed_bc(8, 14, seed, true, 5)` plus the
`XorShift(seed ^ 0xA5A5A5A5)` shuffle and per-piece rotation. They are
generated and committed with the code, before any run, and `ktam check-instances`
regenerates them and compares byte for byte (and checks the ladder files with
the ladder's own generator). Sample size: 30 test instances per rung, every
arm on every instance once.

Staging (binding): R6 and R7 are run first. R8 is run only if V1 or V2 is GAIN
or PASS (Holm-adjusted, against B1, section 7) at R6 or R7. If R8 is skipped
the README says so and why.

## 4. Effort: events

An **event** is one piece placement or one piece removal. A placement is one
(cell, piece, rotation) assignment into an empty cell that agrees with the rim
and with every placed orthogonal neighbour, counted with the ladder's
`NodeCounter::try_assign` (so placements are exactly ladder nodes). A removal
takes one placed piece off the board. A swap or an eviction is the sum of its
removals and placements.

Every arm starts from the empty board, places only into empty cells and
removes only placed pieces, so for every arm

    events = placements + removals = 2 * nodes - filled_at_end,

where `nodes` is the ladder node count and `filled_at_end` the number of
placed cells when the run stops (n^2 for a solved run). For the method arms
removals are counted directly and a test asserts the identity; for the DFS
baselines B1 and B2 (which never place a piece without a counted node) the
event count of a solved run is derived as 2 * nodes - n^2.

**Budget:** E = 1,000,000 events per instance, common to all arms. Every arm is
run through the ladder harness at the ladder's test cap of 1,000,000 nodes,
which is at least (E + n^2) / 2 nodes, so every solve within E is observed. A
run is solved-within-E iff the board verifies and 2 * nodes - n^2 <= E;
otherwise it is censored at E. Secondary (no verdict weight): the same
analysis on the ladder's native node budget (1e6 nodes), which is the
committed-ladder comparison.

Propagation: no arm's propagation counts as events (B1's AC-3 work, B2's fit
tests and the method arms' candidate scans alike); each arm's native
propagation work is a secondary column, and wall clock is reported.

## 5. Arms

All method arms (V1, V2, B3) share one placement machinery:

- State: a consistent partial board (no mismatched seam ever; grey exactly on
  the rim).
- Step: among empty cells, choose one with the fewest consistent unused
  oriented candidates (MRV); ties by the most bonds an attachment there would
  make (placed orthogonal neighbours plus rim edges: cooperative attachment),
  then uniformly at random. If its count is >= 1, place a uniformly random
  candidate (one placement event).
- Dead end: the chosen cell x has 0 candidates. The arm's unassignment rule
  runs (below), then the step loop continues. No restarts, no chronological
  stack.
- Stop: full board (solved; verified by the harness) or `try_assign` returns
  false (censored).
- Randomness: one XorShift stream per run, seeded from the instance seed and a
  fixed run seed (1); deterministic given seeds.

Arms:

- **B1** `scb-gacolor_ac3_random`: the ladder's strongest complete DFS (run
  seed 1), through the ladder registry, unchanged. R6/R7 rows must equal the
  committed ladder rows node for node (checked).
- **B2** `hp-dfs`: plain row-major DFS, ladder registry.
- **B3** `rand-evict`: generic non-chronological unassignment. At a dead end at
  x, remove K pieces chosen uniformly at random (without replacement) among
  placed pieces within Manhattan distance R of x (R = inf: anywhere).
- **V1** `ktam-detach` (kTAM-style bond-weighted detachment): at a dead end at
  x, remove K placed pieces within Manhattan distance R of x, drawn without
  replacement with probability proportional to exp(-beta * b_i), where b_i is
  the piece's current bond count (matched seams with placed neighbours plus
  its rim edges). Weakly bonded pieces leave first (kTAM off-rate
  proportional to exp(-b G_se)).
- **V2** `ims-evict` (IMS-style eviction of the blocking pieces): at a dead
  end at x, consider every oriented piece (p, r) that fits x's rim; its cost
  is the number of placed orthogonal neighbours of x whose seam with (p, r)
  mismatches plus 1 if p is placed elsewhere. With probability `noise` pick a
  uniformly random such candidate, else a minimum-cost one (ties at random),
  skipping candidates whose placement at x is tabu. Remove the blocking pieces
  (the mismatching neighbours and p's current copy) and place (p, r) at x.
  Each removed (cell, piece, rotation) is tabu at that cell for the next T
  placements (Rau's rule of not re-scheduling an operation into the slot it
  was just evicted from).

Tuning grids (calibration seeds only; each arm tuned independently, so the
generic arm B3 gets the same tuning effort as the method):

| arm | grid |
|---|---|
| B3 | R in {1, 2, inf} x K in {1, 2, 3} (9 configs) |
| V1 | beta in {0.5, 1, 2, 4} x R in {1, 2, inf} x K in {1, 2, 3} (36 configs) |
| V2 | T in {0, 16, 128} x noise in {0, 0.1, 0.3} (9 configs) |

Selection: on the 20 calibration instances of R6 and R7 (seeds 1001..1010,
primary ladder), at event budget E, maximise the number solved within E;
ties by the smaller sum over instances of ln(min(events, E)); then the first
config in grid order. Selected configs are committed in `CALIBRATION.md`
before any test-seed run. Each arm is then run once per test instance with
its selected config. The same configs are used at R8 (no R8 retuning).

## 6. Metrics

Per rung and arm: solved within E out of 30 with Wilson 95% interval; Kaplan-
Meier median and quartiles of events (censored = +inf, ceil(N/2)-th order
statistic, as the ladder); the same on ladder nodes at the 1e6 node budget
(secondary); median and total wall clock (ms, 8 runs in parallel on the same
VM); native propagation work. Method arms (V1, V2, B3): number of dead ends,
removals (detachments / evictions), and, computed by the driver after the
solver returns (the solver never sees the planted board), the fraction of
removals whose (cell, piece, rotation) equals the planted board's, next to
the same fraction for placements (base rate). Descriptive only; boards with
other solutions make this a lower bound on "correct" removals.

## 7. Analysis plan

G1 per `../hard-ladder/PROTOCOL.md` on events with C = E: per instance r_i =
ln(c_i / b_i) if both solved within E, ln(c_i / E) if only the candidate
solved, +inf if only the baseline solved, 0 if both censored. Statistic: upper
median of the 30 r_i (16th order statistic); 95% CI by percentile bootstrap,
10,000 paired resamples of instances, `random.Random(20261111)` freshly seeded
per comparison. One-sided bootstrap p = fraction of resamples with median r
>= 0.

Labels per rung and variant against a reference arm:
- **PASS**: exp(median r) <= 0.5 and Holm-significant;
- **GAIN**: Holm-significant and exp(median r) > 0.5;
- **NO-GAIN**: otherwise.

Holm over the two variants (V1, V2) per rung and reference: the smaller p is
significant iff p <= 0.0125; the other iff the first is significant and its
p <= 0.025. Unadjusted labels are reported next to them; verdicts use the
Holm labels.

Comparisons: each variant against B1 (verdict); against the better of B1 and
B3 on that rung (better = more solved within E, ties by lower KM median
events, then B1); against B3 (mechanism clause); against B2 (reported).
Secondary: the same tables on ladder nodes at 1e6 nodes (no verdict weight).

The **best variant** at a rung is the one with more solved within E, ties by
lower KM median events, then V1.

## 8. Kill criterion (binding)

- **Method killed** iff on every rung run, both V1 and V2 are NO-GAIN
  (Holm) against B1 on events, AND the best variant solves at most
  max(B1, B2, B3) + 3 instances (solved within E, out of 30).
- **Mechanism killed** (as opposed to non-chronological unassignment in
  general) iff on every rung run, B3 solves at least (best variant solves - 2)
  AND the best variant is not significantly better than B3 (G1 of best
  variant against B3: the unadjusted one-sided p > 0.025, i.e. the upper end
  of the 95% CI of median r is not < 0). B3 doing better than the variant, or
  significantly better, satisfies "comes within" and fires this clause.
- A GAIN without PASS is reported as "promising, needs a variant".

Self-assessed outcome mapping: method killed, or mechanism killed -> killed
(the README says which clause fired). Otherwise PASS against B1 at some rung
-> survived; GAIN without PASS -> inconclusive ("promising, needs a variant").
A separate judge decides.

Any `error` row (panic, unverified board, exhausted search on a planted
instance) invalidates that arm at that rung until fixed; a fix and rerun is
recorded in `PREREG_AMENDMENTS.md`.

## 9. Compute budget

CPU only, Devin VM (8 vCPU), 8 runs in parallel. Estimated: calibration 54
configs x 20 runs, test 5 arms x 30 x 2 rungs, R8 5 arms x 30 if triggered.
Target about 90 minutes wall clock for everything; if a smoke check on one
calibration instance shows the grid will not fit, the grid is shrunk in a
recorded amendment before calibration, not after. No AWS, no paid services.

## 10. Reproduction

`just experiments ktam-unassignment` (or `research/experiments/ktam-unassignment/run.sh`)
builds, runs tests and clippy, checks instances, calibrates, runs the test
arms, and analyses. Deterministic in events and nodes given the seeds; wall
clock is not.
