# PREREG: SAT backdoors on solvable small boards (agenda A3)

Pre-registration for the cheap test of agenda entry A3. Written and pushed
before any backdoor, propagation or baseline computation was run on any
instance. The kill criterion below is copied verbatim from the agenda and is
not changed after results are seen. Any forced change goes in
`PREREG_AMENDMENTS.md` (what, why, when), and this file stays as written.

## Hypothesis (from the agenda)

Solved small boards have small strong backdoors: a few cells whose correct
placement makes the rest solvable by propagation alone. If backdoor size grows
slowly with board size, a search over backdoor candidates beats cell-by-cell
search.

This test measures only the first half: the size of the smallest set of
planted placements after which the propagation rule below completes the board,
as a function of cell count N = n^2, on generated planted puzzles at
n = 6, 7, 8, 9, 10. It does not test whether a search over backdoor candidates
beats cell-by-cell search.

## Kill criterion

Verbatim from `research/program/AGENDA.md`, entry A3:

> **Kill.** Backdoor size grows roughly linearly in cell count.

### Operationalisation (binding)

- Per instance, `k` = the best available backdoor size: the exact minimum when
  the exact search certifies it, otherwise the greedy/local-search upper bound
  (defined below). Which one each instance used is recorded.
- Per size n, `kbar(n)` = mean of `k` over the 30 instances of the primary
  regime (regime A below).
- Fit by ordinary least squares `log kbar(n) = a + b * log(n^2)` over the five
  sizes n = 6..10. `b` is the growth exponent in cell count.
- Bootstrap: 2000 replicates; in each, resample the 30 instances of every size
  with replacement (independently per size), recompute `kbar(n)` and refit `b`.
  95% CI = 2.5th and 97.5th percentiles. Also bootstrap
  `D = kbar(10)/100 - kbar(6)/36` (change of the backdoor fraction k/N from the
  smallest to the largest size) the same way, with the same replicates.
  Bootstrap RNG: Python `random.Random(20261111)`.
- **KILL** if either holds on regime A:
  1. the upper end of the 95% CI of `b` is >= 0.9 (the data cannot exclude an
     exponent of 0.9 or more, which includes linear growth, b = 1), or
  2. the fraction k/N does not decrease with size: the 95% CI of `D` is not
     entirely below 0.
- **SURVIVE** otherwise (CI upper end of `b` below 0.9 and `D` significantly
  negative).
- If `kbar(n) = 0` at any size in regime A (propagation alone completes the
  empty board on every instance), the fit is undefined and the outcome is
  **inconclusive** for that regime.
- Five sizes is a small number of points. The exponent is a description of
  n = 6..10 only and is not extrapolated beyond it.
- Caveat fixed in advance: where `k` is a greedy upper bound, a kill on `k`
  shows that this greedy finds no sublinear backdoors, not that the minimum
  grows linearly. The calibration against the exact minimum (below) is the
  evidence for how far the upper bound sits from the minimum.

Regime B (second colour count) and the random-subset baseline get the same
statistics, reported as secondary results. They do not change the verdict.

## Instances

- Generator: `eternity2_engine::generator::generate_solved_framed_bc(n, colors,
  seed, true, border)` from `engine/src/generator.rs` (framed mode, explicit
  border-colour count). It returns the planted solution: piece i sits at cell i
  (row-major) in rotation 0. Pieces are used unscrambled; the propagation rule
  is invariant to piece labelling and initial rotation, so scrambling would not
  change any measured quantity.
- Sizes n = 6, 7, 8, 9, 10 (N = 36, 49, 64, 81, 100).
- Border colours: the hint study's E2-faithful recipe (`faithful_colors` in
  `engine/src/bin/hint_variants.rs`): 2, 2, 2, 3, 3 for n = 6..10.
- Interior colours, from Owen's one-expected-solution criterion
  `I(n) = (P! 4^P)^(1/(2P))`, `P = (n-2)^2` (the hardness-peak topic measured
  the 8x8 hard band at c = 6..8 with I(8) = 7.56 inside it):
  - **Regime A (primary, at the peak):** interior = round(I(n)).
  - **Regime B (secondary, over-constrained side):** interior = round(1.5 I(n)).

  | n | I(n) | border | A interior | A total `colors` | B interior | B total `colors` |
  |---|------|--------|-----------|------------------|-----------|------------------|
  | 6 | 5.216 | 2 | 5 | 7 | 8 | 10 |
  | 7 | 6.380 | 2 | 6 | 8 | 10 | 12 |
  | 8 | 7.558 | 2 | 8 | 10 | 11 | 13 |
  | 9 | 8.743 | 3 | 9 | 12 | 13 | 16 |
  | 10 | 9.935 | 3 | 10 | 13 | 15 | 18 |

- Seeds: 1..30 for every (n, regime). 2 regimes x 5 sizes x 30 = 300 instances.

## Propagation rule (fixed before running)

- **Options.** For every piece p, the distinct rotations by edge tuple (URDL);
  a rotation that reproduces an earlier rotation's edge tuple is dropped (lowest
  r kept). A cell's domain is a set of (piece, rotation) options; initially all
  options.
- **Rim (unary).** A side facing off the board must carry colour 0 (grey). A
  side facing another cell must carry a non-zero colour.
- **Edge arc consistency.** For adjacent cells A, B sharing A's side d and B's
  opposite side: remove option o from D(A) when no option of D(B) has, on the
  opposite side, the colour o has on side d. Applied in both directions.
- **All-different, piece singles.** If every option of D(c) belongs to one
  piece p, remove all options of p from every other cell.
- **All-different, hidden singles.** If piece p has options in exactly one cell
  c, restrict D(c) to the options of p. If some piece has options in no cell, or
  a domain is empty, the state is a contradiction (cannot happen when only
  planted placements are fixed, since every rule is sound for the planted
  solution; a contradiction is logged as a bug).
- Iterated to fixpoint with a worklist.
- **Fixing a cell** c to its planted placement sets D(c) to the single option
  (piece c, the canonical rotation equal to rotation 0).
- **Completes** = at fixpoint every domain is a single option. The resulting
  board is then checked (every seam matches, rim grey, every piece once) and
  compared with the planted solution; both flags are recorded per instance.
  Since all rules are sound, a completed board must be the planted one unless
  two pieces are identical, which the generator's uniqueness pass prevents.
- The rules are monotone, so completion is monotone in the fixed set: any
  superset of a completing set completes. The random baseline and the removal
  greedy rely on this; a unit test checks it on sampled sets.

## Backdoor search

- **Root.** Propagate the empty set; record singleton count and whether it
  completes (then k = 0).
- **Greedy upper bound (deterministic).** Minimum over:
  1. Forward greedy: repeatedly fix the non-singleton cell that maximises the
     number of singleton cells after propagation (ties: fewest total options,
     then lowest cell index) until complete; then a reverse pass removes, in
     insertion order, every cell whose removal keeps completion.
  2. 32 removal restarts: start from all cells; visit cells in a seeded random
     order (engine `XorShift`, seed `1_000_003 * seed + 101 * n + regime_id`
     plus the restart index, regime_id A=0, B=1), drop each cell whose removal
     keeps completion.
  3. Local search on the best set S: for each c in S and each d not in S (both
     in index order), if `S - {c} + {d}` completes, run the removal pass on it;
     accept the first strictly smaller set and restart the scan. Stops when a
     full scan finds no improvement or after 100,000 propagations.
- **Exact minimum.** Iterative deepening over k = 0, 1, ..., UB-1: enumerate
  k-subsets in increasing cell-index order by DFS, propagating incrementally,
  and skipping as a next choice any cell already singleton under the current
  fixes (such a cell adds nothing, and the subset without it is enumerated
  elsewhere, so the search stays exact). If a level finds a completing set,
  the minimum is that k; if all levels below UB are refuted, the minimum is UB.
  Node cap (one node = one fix plus propagation): 3,000,000 per instance for
  n <= 7, 500,000 for n >= 8. If the cap is hit, the instance records a
  certified lower bound (deepest fully refuted level + 1) and uses the upper
  bound as `k`.
- **Calibration.** On instances where the exact minimum is certified, report
  the distribution of UB minus exact, and the fraction with UB = exact, per
  (n, regime).

## Baseline: random cell subsets

- Per instance, 200 seeded random permutations of the cells (engine `XorShift`,
  seed `7_000_001 * seed + 101 * n + regime_id` plus the permutation index).
  For each, binary search for the threshold T = the smallest prefix length that
  completes (valid by monotonicity; each prefix of a uniform permutation is a
  uniform random subset of that size).
- Reported per instance: `T50` = smallest k with at least half of the 200
  thresholds <= k (the random-subset size completing with probability 1/2),
  mean T, and `p_rand(k)` = fraction of thresholds <= k at the instance's own
  backdoor size k (the fraction of random subsets of equal size that complete).
- Same growth fit and bootstrap for `T50` as for `k`, and the ratio
  `kbar / T50bar` per size.

## Metrics recorded per instance (raw jsonl)

n, regime, seed, colours (border, interior, total as returned), root singleton
count, root completes, forward-greedy size, best removal-restart size, final UB,
local-search propagations, exact status (certified / capped), exact minimum or
certified lower bound, exact nodes, `k` and its source, completed-board checks
(valid, equals planted), the 200 random thresholds, `T50`, mean T, `p_rand(k)`.
Wall-clock times go to a separate timing file and are not part of the
deterministic results.

## Analysis plan

`analyze.py` (Python standard library only) reads the jsonl and writes
`results/summary.json`: per (n, regime) mean, median, sd of `k`, `k/N`, `T50`,
`p_rand(k)`, exact-coverage and calibration; the fits and bootstrap CIs for
`b` (backdoor and `T50`), `D`; and the verdict computed by the rule above.
README.md reports a results table and the self-assessed outcome.

## Compute budget

The VM: 8 cores, 31 GB RAM, no GPU. Instances run in parallel, one per thread;
results are sorted by (regime, n, seed) so output is deterministic given the
seeds. Target: at most about 3 hours of wall clock. If a timing check of the
code on seeds outside 1..30 projects more than 3 hours, the exact-search node
caps (and only they) are lowered, and the change is written to
`PREREG_AMENDMENTS.md` before the main run.
