# PREREG: near-codeword spectrum (fewest breaks at each overlap with a planted solution)

Round-2 test. Written and pushed before any experiment instance was generated
or solved. The idea write-up (field: coding theory; MacKay and Postol 2003,
DOI 10.1016/S1571-0661(04)80768-0; Karimi and Banihashemi 2012, DOI
10.1109/TIT.2012.2205663) is the binding statement; its pilot numbers came
from a different throwaway generator and are not used anywhere here. The kill
criteria K1 to K3 below are copied from the write-up and then made exact. They
are not changed after results are seen. Any forced change goes in
`PREREG_AMENDMENTS.md` (what, why, when) and this file stays as written.

## Hypothesis (from the write-up)

Treat a full gray-legal board as a word, the planted solution as the
transmitted codeword and breaks (mismatched internal joints) as unsatisfied
checks. Let b_min(a) be the fewest breaks of any full legal board that differs
from the planted solution on exactly a cells. At palettes whose constraint
density mu matches Eternity II's (about 0.0094):

1. the barrier H_mid - H_far (fewest breaks in the mid-overlap band minus
   fewest breaks in the far band) is at least 2 breaks and grows from 5x5 to
   6x6 to 7x7;
2. the near-branch radius stays below a third of the cells;
3. solver boards with few breaks sit on the far branch (overlap near 0), never
   in the mid band.

This is analysis, not a solver. It moves no score.

## Definitions (binding)

- **Board.** A gray-legal placement of every piece: corner pieces on corners,
  edge pieces on edge cells, interior pieces inside, gray edges facing out
  (border-piece orientation is then forced), interior pieces in any of the 4
  rotations. Rotations of one piece that give the same edge tuple are merged.
- **Breaks.** Number of the 2n(n-1) internal joints whose two facing colours
  differ.
- **Pin.** Cell 0 (top-left corner) holds the planted cell-0 corner piece.
  This removes the 4-fold board rotation. Every CP-SAT board obeys it.
- **Distance a.** Number of cells whose placed URDL colour tuple differs from
  the planted tuple at that cell. A swap of two pieces with identical tuples
  does not count. Because of the pin, 0 <= a <= N - 1, N = n^2.
- **Bands** (N = n^2 cells; disjoint integer ranges):
  - near = [1, ceil(N/4) - 1]
  - mid = [ceil(N/4), floor(3N/4)]
  - far = [floor(3N/4) + 1, N - 1]
  - K2 band = [ceil(N/3), floor(3N/4)]

  | n | N | near | mid | far | K2 band |
  |---|---|------|-----|-----|---------|
  | 5 | 25 | 1-6 | 7-18 | 19-24 | 9-18 |
  | 6 | 36 | 1-8 | 9-27 | 28-35 | 12-27 |
  | 7 | 49 | 1-12 | 13-36 | 37-48 | 17-36 |
  | 8 | 64 | 1-15 | 16-48 | 49-63 | 22-48 |
  | 10 | 100 | 1-24 | 25-75 | 76-99 | 34-75 |

- **H_band** = min over a in the band of b_min(a), i.e. the optimum of
  "minimise breaks subject to a in the band". Barrier B = H_mid - H_far.
- **Overlap** = 1 - a/N.

## Instances

- Generator: `eternity2_engine::generator::generate_solved_framed(n, C, seed,
  true)` from `engine/src/generator.rs` (engine crate on `main`). Piece i sits
  at cell i in rotation 0; that board is the planted solution. Pieces are used
  unscrambled (the CP-SAT model is invariant to piece labels). Border colours
  = min(5, C - 1) = 5 at every C below.
- Primary (mu-matched) palettes, mu = 4 (n-2)^2 q_int^4 with q_int the
  probability that two random interior-band half-edges match (computed per
  instance from its colour census and recorded):

  | n | C | interior colours | approx mu |
  |---|---|------------------|-----------|
  | 5 | 13 | 8 | 0.009 |
  | 6 | 14 | 9 | 0.010 |
  | 7 | 15 | 10 | 0.010 |
  | 8 | 16 | 11 | 0.010 |
  | 10 | 18 | 13 | 0.009 |

- Contrast (looser) palettes, the no-gap control: 5x5 C=9, 6x6 C=10, 7x7
  C=11, 8x8 C=12 (4, 5, 6, 7 interior colours; mu about 0.14, 0.10, 0.08,
  0.06).
- Seeds and sample sizes:
  - Stage 1 (exact): 5x5 C=13 and 6x6 C=14, seeds 1..30 each. Contrast 5x5
    C=9 and 6x6 C=10, seeds 1..10 each.
  - Stage 2 (bounded): 7x7 C=15 and 8x8 C=16, seeds 1..10 each. Contrast 7x7
    C=11 and 8x8 C=12, seeds 1..5 each.
  - Stage 3 (solver position): 8x8 C=16 and 10x10 C=18, seeds 1..20 each
    (8x8 seeds 1..10 are the stage-2 instances).
- Development and debugging use only seeds >= 1000; nothing from those runs is
  reported.
- Per instance the number of rotation-duplicate pieces is recorded (the
  generator's uniqueness pass is bounded and may leave some).

## Model (CP-SAT, OR-Tools 9.15.6755 pinned in requirements.txt, Python venv via uv)

- Booleans x[c, p, r] for every gray-legal (cell, piece, rotation) with
  duplicate rotations merged. Exactly one per cell; exactly one per piece
  (summed over cells and rotations). Pin x[0, planted] = 1.
- Side colour of cell c on side s with colour k: the linear sum of the x with
  that colour on that side.
- Per joint j and colour k possible on both sides: m[j,k] <= side colour sums
  of both endpoints. Breaks = sum_j (1 - sum_k m[j,k]), minimised.
- Distance: a = N - sum over c of the x whose tuple equals the planted tuple
  at c. Constraint lo <= a <= hi.
- Every returned board is re-checked in plain Python (legality, piece
  permutation, recomputed breaks and a); a mismatch is a bug and stops the run.
- Per solve we record status, objective (UB), best bound (LB), wall time and
  the witness board. "Exact" = status OPTIMAL (UB = LB).

### Solve schedule

- Stage 1, every stage-1 instance: bands near, mid, far and the K2 band, each
  `max_time_in_seconds = 120`, `num_workers = 4`. Primary palettes only: the
  full curve b_min(a) for every a = 1..N-1 (a = lo = hi), each 30 s x 2
  workers. Curve points are descriptive; the kill statistics use the band
  solves. Consistency check: where all curve points of a band closed, their
  minimum must equal the band optimum.
- Stage 2, primary: near 60 s x 4 workers, mid and far 300 s x 4 workers each
  (equal budget for mid and far). Contrast: mid and far 60 s x 4 workers.
- CP-SAT search is randomised and parallel; optimal values are reproducible,
  witness boards and the bounds of time-limited solves are not bit-for-bit
  reproducible. CP-SAT `random_seed` = 1000*n + seed.

## Baselines

1. **Annealed first-moment (random-code spectrum) prediction** b_ann, computed
   exactly (no sampling) in Rust for the 5x5 and 6x6 palettes:
   E(a, b) = sum over cell subsets S of size a not containing cell 0 of
   F(S) * P_S(b), where F(S) = D(s_c) D(s_e) R(s_i) counts fillings of S in
   which every cell of S changes (s_c, s_e, s_i = corner, edge, interior cells
   in S; D = derangement number; R(k) = sum_j (-1)^j C(k,j) (k-j)! 4^(k-j),
   arrangements of k interior pieces with rotations in which no cell keeps its
   own piece in rotation 0), and P_S(b) = probability that exactly b of the
   joints touching S mismatch when each touched along-frame joint matches
   independently with probability p_f and every other touched joint with
   probability p_i. p_f and p_i are the instance's collision probabilities
   (sum over colours of squared half-edge shares) on the along-frame half-edges
   and on all other non-gray half-edges. The subset sum is a row-by-row
   transfer-matrix DP over row masks. b_ann(a) = smallest b with E(a,b) >= 1;
   b_ann(band) = min over a in the band of b_ann(a). The generator gives the
   same census for every seed of one (n, C), so p_f and p_i are checked to be
   seed-independent and b_ann is then one number per palette; if not, it is
   computed per instance.
2. **Reachable null (stage 3).** Uniformly random legal placement polished by
   plain greedy descent (first strictly improving swap or rotation, moves
   scanned in a seeded random order, until no improving move exists).
3. **Contrast palette** per size, as listed above (the no-gap control).

## Stage 3 (solver position)

Per stage-3 instance, three samplers, 32 runs each, deterministic budgets
(no wall-clock limits):

- **Engine break-tolerant DFS** (`eternity2_engine::BreakSolver`, snake path,
  hints off, break cells = the last 2n cells of the path). Run r shuffles the
  piece list with the engine `XorShift(1_000_003 * seed + 1009 * n + r)` (the
  mapping to planted pieces is kept). Budget 20,000,000 steps. If the run ends
  Solved, its full board is recorded; otherwise the run records no board.
- **Local search.** The starter kit ships no local search (only greedy and
  backtracking examples), so a plain simulated annealing is used instead:
  random legal start, moves = swap two same-class cells (border pieces
  re-oriented to stay legal, interior pieces keep their rotation) with
  probability 1/2, else rotate one interior cell; Metropolis acceptance;
  geometric temperature from 2.0 to 0.2 over 20,000,000 move attempts; RNG
  seeded as above with r + 100.
- **Reachable null** as in baseline 2, RNG seed r + 200.

Every state a sampler visits (each accepted move, and the final board) is
scored and its distance a is computed after rotating the whole board so that
the planted cell-0 corner piece sits at cell 0 (the CP-SAT pin). Per run we
record min breaks at every a (vector of length N + 1) and the best break count
b*. The overlap histogram (10 bins of overlap) is over all visited states with
breaks <= b* + 2 (visit counts) and, separately, over distinct per-run best
boards. At 8x8 seeds 1..10 the per-a minima feed the K1 portfolio.

## Kill criteria (written before any run; operationalised here)

General rule for bounded values: where a quantity is an interval [LB, UB]
(time-limited solve), the criterion is evaluated at both ends (the end most
favourable and the end least favourable to the hypothesis). It is **decided**
when both give the same verdict, and **undetermined** otherwise. Barrier
bounds per instance: [LB_mid - UB_far, UB_mid - LB_far].

**K1 (overlap gap).** Fires if any of:

- K1a: at 5x5 or at 6x6 (30 primary instances each) the median barrier
  B = H_mid - H_far is <= 0.
- K1b: the barrier does not grow from 5x5 to 6x6: one-sided Wilcoxon
  signed-rank test of d_s = B(6x6, seed s) - B(5x5, seed s), s = 1..30
  (pairing by seed index, which is an arbitrary pairing of independent
  instances), alternative "greater", `scipy.stats.wilcoxon(d,
  alternative="greater", zero_method="wilcox", method="approx",
  correction=True)`; fires if p > 0.1 (also if every d_s = 0). Mann-Whitney U
  on the two unpaired samples is reported as a secondary check only.
- K1c: at 7x7 or at 8x8 (10 primary instances each), on >= 25% of instances
  (>= 3 of 10) the best mid-band board found has breaks <= the best far-band
  board found at equal budget. Best = min over the CP-SAT UB for that band
  (300 s x 4 workers each) and, at 8x8 seeds 1..10, the stage-3 per-a minima
  over that band. Certificates only, no LB needed.

**K2 (decoding radius).** Fires if at 6x6 the median over the 30 primary
instances of m2 = H_(K2 band) = min over a in [12, 27] of b_min(a) is <= 2.

**K3 (annealed predictor).** Fires if at 6x6 |median over the 30 primary
instances of (H_mid - b_ann(mid))| > 2.

**Outcome.** Survived if K1, K2 and K3 are all decided and none fires. Killed
if any criterion is decided as firing (its numbers recorded). Inconclusive if
none fires but at least one is undetermined. The judge decides; this is the
self-assessment rule.

## Metrics reported (descriptive, not part of the verdict)

- Per size and palette: median and IQR of H_near, H_mid, H_far, H_K2, B; share
  of instances with H_mid > max(H_near, H_far) (the hypothesis's "strictly
  above both").
- 5x5 and 6x6 primary: median curve b_min(a) and the annealed b_ann(a) curve.
- Near-branch radius r(b) per instance: the largest a with b_min(a) <= b on
  the curve before the first a where b_min(a) > b, for b = H_mid - 1.
- Contrast palettes: the same band statistics.
- Stage 3: share of low-break visited states (b <= b* + 2) and of per-run best
  boards in each band, per sampler, against the reachable null.
- Wall clock per stage, CP-SAT closure rates.

## Limits stated in advance

- Exact claims only at 5x5 and 6x6 (9 and 16 interior pieces; mu matching is
  crude there). Above 6x6 the test can only kill (find a cheap mid-band board),
  not confirm.
- The barrier at these sizes is at most a few breaks; nothing is extrapolated
  to 16x16.
- Generated planted puzzles may carry structure the official set lacks.

## Compute budget

VM, 8 cores, no AWS. Estimated wall clock: stage 1 about 30 to 40 min, stage 2
at most about 2 h (two 4-worker solves at a time; far-band solves that close
early finish sooner), stage 3 about 15 min. Total about 3 h wall, about 20
core-hours at most. If a stage would overrun, the cut is recorded in
`PREREG_AMENDMENTS.md` before its results are read.

## Code and outputs

All under `research/experiments/near-codeword-spectrum/`: Rust crate (path dep
on `../../../engine`) for instances, the annealed DP and stage 3; Python
scripts for CP-SAT and analysis; raw per-solve jsonl, summary json, README.
Recipe `just experiments near-codeword-spectrum`.
