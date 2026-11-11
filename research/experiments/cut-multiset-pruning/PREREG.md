# Cut-multiset feasibility pruning: preregistration

Round-2 test on the hard-instance ladder (branch devin/r2-hard-ladder,
`research/experiments/hard-ladder`, final commit 85c5bf2). Written and pushed
before any experiment on this branch. Any later forced change goes in
`PREREG_AMENDMENTS.md` with what, why and when; this file is not edited after
the first push.

## 1. Novelty check (done first)

Read: `single-core-benchmark/engine/crates/propagators/src/border_eulerian.rs`
and `propagators/src/lib.rs` (class_balance, parity, gacolor, island,
multiset_equality = NS-1), `research/topics/flux-invariants`,
`research/topics/parity-defect-floor`, `research/topics/forbidden-patterns`,
`web/content/research/build/dead-ends.mdx` (border-supply "lollypop" check,
Eulerian border closure, forbidden-pattern pruning, pattern databases),
`web/content/research/build/reduce/alldiff-regin.mdx`, the LP/ILP row of
`research/program/EXPLORED.md`, and the hard-ladder `CALIBRATION.md`.

What the idea proposes, and where each part already lives:

| Part of the proposed test | Existing code or measurement | Verdict |
|---|---|---|
| Per-colour parity and count inequality on the frontier: unused half-edges of colour c minus open frontier demand of c must be >= 0 and even | `gacolor_check` / `GaColorState` (propagators/src/lib.rs): exactly `supply(c) - open_demand(c) >= 0 and even`; it is **inside the baseline** `scb-gacolor_ac3_random` at every node | duplicate |
| Internal seam colour counts even | same gacolor slack parity; `parity_check` (checkerboard); parity-defect-floor census | duplicate |
| Straight cut between the border band and the interior (ring seam multiset must balance) | NS-1 `multiset_equality_check`; measured on this ladder as calibration candidate `scb-gacolor_ac3_ns1`: identical node counts to `scb-gacolor_ac3` on all 50 calibration instances (CALIBRATION.md) | duplicate, measured zero |
| Class-split count (edge pieces' fixed inward faces vs interior pieces) | algebra: under the baseline's border-first order this is a constant precondition during the ring phase and equals gacolor once the ring is placed | vacuous for the baseline |
| Other straight cut lines through the unfilled interior | which unused pieces fall on each side is free, so per-colour counts give only a total-parity condition already implied by gacolor | no pruning content |
| Rotation-parity (flux) law | flux-invariants: F2 rotation parity certificate; a different invariant (rotations, not colour coverage) | not this test |
| Bipartite flow between frontier slots and unused pieces with rotation (frontier Hall condition) | **not implemented** in any propagator of the single-core engine (no matching anywhere; piece uniqueness is only crossed off), not in the hard-ladder registry, not measured on any board here (`alldiff-regin.mdx` is a prose concept page with no repro) | **new for this repo and this baseline** |

So the count/parity half of the idea is a duplicate of gacolor (already in
the baseline) and the ring-cut half is a duplicate of NS-1 (measured zero on
this ladder). The part that is not implied and not measured is the matching
half: the frontier demand must be coverable by **pairwise distinct** unused
pieces, each in a rotation that fits all of its cell's known faces. This is
what is tested here. It is the feasibility (matching) half of Régin's
all-different filter restricted to the frontier, without the filtering half;
EXPLORED.md lists "Arc consistency, Régin all-different" as mapped, and the
family's recorded stop ("pruning fades about two cells out") is about arc
consistency, not about a measured matching check. Named mechanism the
baseline lacks: a global Hall condition across non-adjacent frontier cells.

### What it prunes that the existing propagators do not

gacolor counts half-edges with replacement (one piece may "serve" demands at
several cells); AC-3 checks pairs of adjacent cells (distinct pieces only
between neighbours); island checks that every unused piece has some home;
NS-1 and Eulerian look only at the ring; class balance counts classes. None
of them sees k non-adjacent frontier cells whose domains draw on fewer than k
pieces.

### Hand-built example (5x5, verified by `compute/tests/example.rs`)

Colours: 0 grey, 1 frame (all ring seams), a = 2, b = 3, c = 4. The ring is
placed with inward faces (a, b, a) on every side, so each interior corner
cell (1,1), (3,1), (1,3), (3,3) demands two adjacent a faces and each middle
cell demands b. Unused interior pieces (URDL): three of [a,a,c,c], two of
[b,c,a,c], two of [b,c,c,c], one [a,c,a,c], one [c,c,c,c]. Only the three
[a,a,c,c] pieces carry two adjacent a's, so the four corner cells cannot get
distinct pieces: the state is dead. The test computes the arc-consistent
domains (pairwise, distinct pieces) and asserts: no empty or singleton
domain; class_balance, parity, gacolor (a: 10 - 8 = 2, b: 4 - 4 = 0, c: 22 - 0,
all even), island, NS-1 and the Eulerian ring check all return Ok; the Hall
check on the 8 frontier cells, and on all 9 empty cells, rejects it.

## 2. Hypothesis

Adding the Hall check to the ladder's best baseline, at every node, reduces
the search-tree size (hard-ladder node metric) against the committed best
baseline `scb-gacolor_ac3_random`, enough to pass PROTOCOL.md G1 on at least
one rung, and in particular at R8.

## 3. Method

`compute/engine-fork/` is a verbatim copy of
`single-core-benchmark/engine/crates/solver-engine` (renamed package
`cmp-solver-engine`) with one hook: after the engine's Step-8 propagators
succeed, `hall_feasible()` builds, for every empty cell in scope, the set of
unused pieces with at least one row (piece, rotation) still in the cell's
AC-3-filtered domain, and runs Kuhn augmenting paths (cells sorted by domain
size); if some cell in scope cannot be matched the node is a wipeout. With
the hook off the fork must count node for node like
`scb-gacolor_ac3_random` (checked before any test run: fork-off on R6 and R7
test seeds must reproduce the committed rows exactly; otherwise stop and
amend). Everything else (border-first MRV, gacolor, AC-3, randomised value
order, run seed 1, harness, node counter, caps) is the baseline's.

Variants (two looks on test seeds, each run once):

- **V1 (fixed now)**: `cmp-hall-frontier`: scope = frontier (empty cells with
  at least one assigned neighbour), every node.
- **V2 (selected on calibration seeds only)**: chosen from
  {`cmp-hall-all` (every empty cell, every node), `cmp-hall-frontier-k4`
  (frontier, every 4th propagation), `cmp-hall-all-k4`} by: most calibration
  instances solved summed over R6, R7, R8 (seeds 1001..1010, calibration cap
  2,000,000), ties broken by the sum over rungs of log KM median nodes, then
  by total wall clock. The calibration rows of V1 and the baseline are run
  too and reported.

## 4. Rungs, instances, baselines

Harness, node counter, seeds (test 1..30, calibration 1001..1010), caps
(test 1,000,000 per instance, calibration 2,000,000) and protocol are the
ladder's.

- R6: 6x6 c = 5 (primary ladder, `instances/primary-test.jsonl`), baseline =
  committed `results/test/primary/scb-gacolor_ac3_random.jsonl` and `hp-dfs`.
- R7: 7x7 c = 6 (primary), committed baselines as above.
- R8: 8x8 c = 10, above the critical count. Instances: exactly the ladder's
  `generate(8, 10, seed)` (`generate_solved_framed_bc(8, 15, seed, true, 5)` +
  `XorShift(seed ^ 0xA5A5A5A5)` scramble), written to
  `instances/8-c10-{test,calibration}.jsonl` in this directory and checked
  byte for byte by `cut_multiset check-c10`; the committed ladder files are
  checked by `hard_ladder check-instances`. Baselines run here before the
  method: `scb-gacolor_ac3_random` and `hp-dfs`, cap 1,000,000, same harness.

Sample size: 30 test seeds per rung (the time estimate below fits; the floor
of 20 is not needed).

## 5. Metrics

Per rung, candidate and baselines: solved/30 with Wilson 95% interval, KM
median and quartiles of nodes, median and total wall clock (ms; 8 runs in
parallel on the 8-vCPU VM), nodes per second, and the paired wall-clock
log-ratio median, so a node win that costs wall clock is visible.

## 6. Analysis and labels

G1 per rung exactly as PROTOCOL.md (r_i table; statistic = upper median of
the 30 r_i; percentile bootstrap, 10,000 paired resamples, seed 20261111),
against `scb-gacolor_ac3_random`.

Holm correction for the two looks, per rung: one-sided bootstrap p = fraction
of resamples whose median r is >= 0. Sort the two variants' p; the smaller is
significant iff p <= 0.025 / 2, the other iff the first is and its p <=
0.025. "CI below 0" for a variant means Holm-significant (unadjusted, this is
PROTOCOL's "upper end of the 95% CI < 0").

Per rung and variant:
- **PASS**: exp(median r) <= 0.5 and CI below 0 (G1).
- **GAIN**: CI below 0 and exp(median r) > 0.5.
- **NO-GAIN**: otherwise.

All three rungs are run for both variants whatever the earlier results; the
ladder's dropping rule is reported but does not stop runs. G2 (growth over
6..10) is not evaluated: the three rungs mix critical and above-critical
colour counts and rungs 9, 10 are not run; this is stated in the report.

## 7. Kill criterion and outcome

- **Killed** iff the method (best variant per rung) is NO-GAIN on all three
  rungs R6, R7, R8.
- **PASS (survived)** requires G1 PASS at R8 (median paired ratio <= 0.5 and
  Holm-corrected CI below 0) for at least one variant.
- Otherwise (GAIN on some rung, or PASS only at R6/R7): **promising, needs a
  variant** (reported as inconclusive, not killed).
- Any `error` row (panic, unverifiable board, exhaustion of a planted
  instance) invalidates the variant at that rung until fixed.

## 8. Compute budget

Devin VM, 8 vCPU, 31 GiB, 8 runs in parallel. Baseline wall clock per run is
at most about 16 s at the 1e6 cap (ladder README); the Hall check costs one
matching of at most 64 cells per node, assumed at most 10x per node, so a
rung of 30 runs is at most about 10 minutes per variant. Total, including
calibration of the V2 grid, under 90 minutes. If a variant exceeds 20
minutes on one rung, the remaining runs continue and the overrun is
reported; nothing is cut after seeing results.
