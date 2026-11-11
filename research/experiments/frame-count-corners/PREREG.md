# PREREG: exact count of frame placements with the corner constraint (agenda A5)

Written and pushed before any experiment in this directory is run. Feasibility
was estimated beforehand from state counts only (sizes of multiset layers on the
official ring, computed from the piece list; no count of frames was produced).

## Hypothesis

The ring-purity page samples about 10^37.6 valid frame placements and lists
the exact count with corners as open. A transfer-matrix count over the
directed ring multigraph with corner positions as a constraint is exact and
feasible. (Agenda A5, verbatim.)

## What is counted (normalisation, same as the committed Monte Carlo)

`research/topics/ring-purity/compute/ring_count.py` builds the directed ring
multigraph: vertices are the frame colours, each of the 60 border pieces is one
labelled arc (from the ring colour on the slot before its gray run to the colour
on the slot after it, clockwise; border pieces cannot flip, so the direction is
forced). It computes EC = arborescences x prod_v (out(v)-1)! by BEST
(3432 x (11!)^5): the number of Euler circuits as cyclic sequences of labelled
arcs, equivalently circuits with a fixed first arc. It samples uniform circuits
with a fixed first arc and records the fraction f whose four corner arcs sit at
cyclic positions 15 apart. Its `log10_valid_frame_placements` is
log10(4 f EC).

So the target quantity is

    N = number of placements of the 60 labelled border pieces on the 60 fixed
        border cells of the 16x16 board (corner pieces on the 4 corner cells,
        edge pieces on the 56 edge cells, orientation forced by the gray side)
        such that all 60 ring-facing adjacencies match.

Board rotations are not quotiented (a frame and its 90 degree rotation are two
placements); interior-facing colours are ignored. With corner cells at ring
positions 0, 15, 30, 45: N = 4 x N_cyc, where N_cyc is the number of cyclic
circuits with corners 15 apart, and f_exact = N_cyc / EC is the quantity the
Monte Carlo estimated (f_MC = 2.775e-5, s.e. 1.317e-6, 16,000,000 samples,
444 hits).

## Methods

- **M1, the method under test (Rust, transfer matrix).** For each assignment
  sigma of the corner pieces to the 4 corner cells, the ring splits into 4 sides
  of L edge arcs; side j is a walk from head(sigma_j) to tail(sigma_{j+1}).
  Edge pieces with the same (from, to) colour pair are interchangeable, so the
  DP runs over type-words: state = multiset of edge-arc types already used
  (packed into a u64), the current colour being implied by the state. One step
  appends one arc. Layers are kept as sorted key arrays per current colour and
  advanced by a k-way merge (no hash maps). Meet in the middle: forward over
  sides 0 and 1, backward (reversed arcs) over sides 3 and 2, joined on
  complementary multisets. W(sigma) = number of 4-tuples of side type-words
  using the edge-type multiset M exactly; N = prod_{types} m_ab! x
  sum_sigma W(sigma). Arithmetic: exact u128 with overflow checks on the
  reduced rings; on the official ring, residues modulo three fixed 61-bit
  primes p1 = 2^61-1 = 2305843009213693951, p2 = 2305843009213693921,
  p3 = 2305843009213693907 (largest three primes below 2^61, checked by
  Miller-Rabin) recombined by CRT. Bound used for CRT uniqueness:
  W_total = sum_sigma W(sigma) <= 4 EC / prod m_ab! (every placement is a
  distinct circuit), computed exactly before the run; the CRT is accepted only
  if this bound is below the product of each prime pair used, and the
  reconstruction from primes
  {1,2} must equal the one from {1,3} and from {2,3}.
- **M2, independent exact method (Python stdlib, BEST per side).** Number of
  type-words of a side with Parikh matrix P (an Euler trail s -> t of the
  multigraph P): kappa_t(P) x prod_{v != t, v present} (out_P(v)-1)! x
  out_P(t)! / prod_ab P_ab!, kappa_t by the matrix-tree theorem (exact Fraction
  determinant). Explicit convolution of the 4 side tables over all
  decompositions M = P_0 + P_1 + P_2 + P_3. Reduced rings only.
- **M3, brute force (Rust).** Depth-first enumeration of labelled placements,
  cell by cell around the ring, no type grouping and no BEST. Reduced rings
  only. Node cap 2e10 DFS nodes per instance.
- On the reduced rings M1 is run on all 24 corner assignments; on the official
  ring M1 uses the rotation symmetry N = 4 x sum over the 6 assignments with
  corner piece 0 on cell 0 (it is checked on every reduced ring that the 24-way
  sum equals 4 x the 6-way sum).

## Boards, colour counts, generator calls, seeds

- Reduced rings: the 4(n-1) border pieces of
  `eternity2_engine::generate_framed(n, colors, seed, true)` (framed palette:
  min(5, colors-1) border colours). Ring of 4 corners and 4(n-2) edges.
  Cells (n, colors): (5,3), (5,4), (6,3), (6,4), (6,6), (7,4), (7,6), (8,5),
  (8,6). Seeds 1, 2, 3, 4, 5 for every cell: 45 instances. The ring colours are
  read from the generated pieces (gray = 0), never assumed.
- Official: the 60 border pieces of `engine/data/official_eternity2.csv`
  (through `eternity2_engine::official_puzzle()`), 5 frame colours, 4 corners,
  56 edges, L = 14.
- Nothing is random in M1, M2 or M3; the only seeds are the generator seeds.
  No full 16x16 search is run; only the official frame ring.

## Kill criterion

Verbatim from the agenda: "Method disagrees with brute force on the reduced
ring, or the state space does not fit in memory."

Operationalisation:

- **K1 (disagreement).** Statistic: for each reduced instance where M3
  finished under its node cap, the exact integers N_M1 and N_M3. Test: exact
  equality. Threshold: one instance with N_M1 != N_M3 kills.
- **K2 (memory).** The official M1 run runs on this VM (31 GiB RAM, no swap)
  with peak resident memory measured by `/usr/bin/time -v`. Threshold: the run
  is killed for lack of memory, an allocation fails, or peak RSS exceeds
  30 GiB. Any of these kills.
- If fewer than 30 of the 45 reduced instances finish M3 under the node cap,
  the validation is too thin and the outcome is inconclusive (not survived).
- If the official run does not finish within the compute budget below for a
  reason other than memory, the outcome is inconclusive.
- Otherwise the result survives.

## Secondary checks (reported, do not change the kill decision)

- M1 vs M2 on all 45 reduced instances (exact equality); any mismatch is
  reported and investigated.
- 24-way sum = 4 x 6-way cyclic sum on every reduced instance.
- Official: f_exact = N_cyc / EC versus the committed Monte Carlo:
  z = (f_MC - f_exact) / 1.317e-6. Report z and whether f_exact lies in the
  95% interval f_MC +- 1.96 s.e. = [2.517e-5, 3.033e-5]. |z| > 3.29 is
  reported as a discrepancy between the two methods that the write-up must
  address; it is not a kill under the agenda wording.
- Official: CRT agreement across the three prime pairs.

## Baseline

The committed Monte Carlo estimate in
`research/topics/ring-purity/results/ring_count.json`: fraction 2.775e-5,
s.e. 1.32e-6 (binomial), log10 placements 37.5866. Not recomputed; compared
against as committed. EC is recomputed exactly from the piece list by BEST in
the summary script and checked against its `arborescences` and
`log10_euler_circuits_BEST` fields.

## Metrics

Per reduced instance: n, colors, seed, frame colours, edge-type multiset,
N_M1 (u128), N_M2, N_M3 (or capped), M3 nodes, wall times, max layer size.
Official: N (exact integer), N_cyc, log10 N, f_exact, z, per-assignment W,
max layer size (states), peak RSS, wall clock.

## Analysis plan

1. Run M1, M2, M3 on the 45 reduced instances; write one jsonl line each.
2. Apply K1. 3. Run M1 on the official ring three times (one per prime) under
`/usr/bin/time -v`; apply K2. 4. CRT, then the secondary checks.
5. Summary JSON and README with the outcome against K1/K2.

## Compute budget

This VM: 8 cores, 31 GiB RAM, no GPU. About 3 hours of wall clock in total.
Expected from the layer-size estimate: peak about 5e8 states per layer on the
official ring (16 bytes per state in residue mode), tens of minutes for the
official run. No AWS, no paid service.

## Amendment rule

The kill criterion above is fixed. A bug fix or a forced deviation goes into
`PREREG_AMENDMENTS.md` with what, why and when; this file is not edited after
it is pushed.
