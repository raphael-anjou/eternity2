# Research agenda

Candidate directions that are **not** cells of `EXPLORED.md`, or that attack
an explored cell with a mechanism its page does not cover. Each entry is a
hypothesis to kill cheaply, not a promise. The lead session (see
`.claude/commands/research-lead.md`) owns this file: it adds, ranks and closes
entries, and every closed entry gets a ledger line and, when the result is
worth reading, a lab page.

Entry format: **hypothesis** · **borrowed from** · **nearest explored cell and
how this differs** · **cheap test** (8x8, 10x10 or generated boards with a
known planted solution) · **baseline** · **kill criterion** · **budget**.

Status values: `proposed`, `running`, `killed`, `promoted` (passed the cheap
test, scheduled at larger scale), `published`.

## A1. Planted-solution recovery by spectral statistics · killed

- **Hypothesis.** The puzzle was painted from one solution. Pair statistics of
  the piece set (which pieces can sit next to which, weighted by colour
  rarity) carry a planted signal that a spectral method can recover above
  chance, the way planted partitions are recovered in community detection.
- **Borrowed from.** Planted-partition and stochastic block model recovery.
- **Nearest cell.** Belief-propagation ordering and survey propagation (dead
  ends): those propagate local marginals during search. This asks whether a
  global, search-free statistic ranks true neighbours higher.
- **Cheap test.** On 200 generated 8x8 and 10x10 puzzles with known solutions,
  rank each piece's candidate neighbours by the leading eigenvectors of a
  weighted compatibility matrix and measure the rank of the true neighbour.
- **Baseline.** Uniform ranking among compatible candidates, and colour-rarity
  ranking alone.
- **Kill.** True-neighbour mean rank not better than the colour-rarity
  baseline by more than two standard errors.
- **Budget.** One session, CPU only.

- **Result (round 1, [PR #14](https://github.com/raphael-anjou/eternity2/pull/14)).** Killed by an independent judge. Spectral ranking puts the planted neighbour lower than colour-rarity ranking at 8x8 c=8 (paired difference -0.00472, SE 0.00059, z -7.94) and 10x10 (-0.00349, SE 0.00059, z -5.91), 200 puzzles each. Experiment: `research/experiments/spectral-planted-recovery/`.

## A2. Score versus attempts as an extreme-value curve · proposed

- **Hypothesis.** Record hunting is a maximum over attempts. Fitting
  extreme-value statistics to many short runs of one engine predicts how the
  best score grows with compute, and so tells whether 471 is a compute
  question or a method question for that engine.
- **Borrowed from.** Extreme-value theory, reliability engineering.
- **Nearest cell.** Restarts and heavy tails (runtime distributions to a
  solution). This models the distribution of the best score, not runtime.
- **Cheap test.** Stage 1 on generated 10x10 puzzles: 10,000 short
  break-tolerant runs; fit GEV / Gumbel on block maxima from the first 2,000,
  predict the best of the remaining 8,000, and check against them. Stage 2,
  only if stage 1 holds: the same split with one-minute runs of the
  Verhaard-style engine on 16x16 corner-pinned variants.
- **Baseline.** Naive extrapolation of the best-so-far curve.
- **Kill.** Prediction misses the held-out maximum by more than one edge.
- **Budget.** Stage 1 a few core-hours. Stage 2 about 170 core-hours
  (10,000 one-minute runs, holdout included).

## A3. SAT backdoors on solvable small boards · promoted

- **Hypothesis.** Solved small boards have small strong backdoors: a few
  cells whose correct placement makes the rest solvable by propagation alone.
  If backdoor size grows slowly with board size, a search over backdoor
  candidates beats cell-by-cell search.
- **Borrowed from.** Backdoor sets in SAT (Williams, Gomes, Selman).
- **Nearest cell.** Hint study (fixed official hints) and cube and conquer
  (splits on SAT variables chosen by lookahead). This measures the minimum
  set, on solved instances, as a function of size.
- **Cheap test.** For 6x6 to 9x9 generated puzzles, find the smallest set of
  placements after which unit propagation plus all-different completes the
  board; report the growth curve.
- **Baseline.** The same measurement on random cell subsets of equal size.
- **Kill.** Backdoor size grows roughly linearly in cell count.
- **Budget.** One session.

- **Result (round 1, [PR #15](https://github.com/raphael-anjou/eternity2/pull/15)).** Kept by an independent judge. Regime A growth exponent b = 0.589 (bootstrap 95% CI [0.549, 0.626], kill line 0.9). At 8x8 and above k is a greedy upper bound; boards are generated, not the E2 piece set; backdoors were found with access to the planted solution. Follow-up: A16. Experiment: `research/experiments/sat-backdoors/`.

## A4. Quality-diversity search over basin descriptors · killed

- **Hypothesis.** The open diversity problem is a descriptor problem: methods
  rediscover the same basins because they optimise one number. MAP-Elites over
  a small descriptor grid (mismatch positions, frame identity, colour-flow
  statistics) keeps many basins alive and finds more distinct ones per core-hour
  than parallel tempering.
- **Borrowed from.** Quality-diversity optimisation (MAP-Elites, novelty
  search) in evolutionary robotics.
- **Nearest cell.** Genetic algorithms (single objective, crossover) and the
  open diversity problem. This keeps the existing local moves and changes only
  the archive and selection.
- **Cheap test.** 8x8 and 10x10 generated puzzles: count distinct basins with
  score within k of the best, per core-hour, against parallel tempering at the
  same budget.
- **Baseline.** The repo's parallel tempering at equal compute.
- **Kill.** No more distinct basins than PT at equal compute on both sizes.
- **Budget.** One to two sessions.

- **Result (round 1, [PR #16](https://github.com/raphael-anjou/eternity2/pull/16)).** Killed by an independent judge. At equal move budgets MAP-Elites found fewer distinct near-best basins than parallel tempering on 58 of 60 puzzles (2 ties, 0 wins); mean distinct basins 8x8 PT 22.33 vs ME 5.63, 10x10 PT 5.30 vs ME 1.00. Experiment: `research/experiments/quality-diversity-basins/`.

## A5. Exact count of frame placements with the corner constraint · promoted

- **Hypothesis.** The ring-purity page samples about 10^37.6 valid frame
  placements and lists the exact count with corners as open. A transfer-matrix
  count over the directed ring multigraph with corner positions as a
  constraint is exact and feasible.
- **Borrowed from.** Transfer-matrix methods, BEST theorem with constraints.
- **Nearest cell.** Ring purity (sampled count).
- **Cheap test.** Validate the exact method against brute force on a reduced
  ring, then run it on the official frame.
- **Baseline.** The committed Monte Carlo estimate in
  `research/topics/ring-purity/results/ring_count.json`.
- **Kill.** Method disagrees with brute force on the reduced ring, or the
  state space does not fit in memory.
- **Budget.** One session. Small, closes a stated open problem.

- **Result (round 1, [PR #17](https://github.com/raphael-anjou/eternity2/pull/17)).** Kept by an independent judge. Exact count N = 40519056449429419697492635191544381440 (log10 N = 37.6077); f = N_cyc / EC = 2.913e-5 (N_cyc = N/4, EC the BEST-theorem Euler-circuit count), the quantity the Monte Carlo estimate measured, against its 2.775e-5 (SE 1.32e-6, z -1.04). Matches brute force on 34 reduced rings and an independent method on 45. Next: an independent cross-check on the official ring. Experiment: `research/experiments/frame-count-corners/`.

## A6. Non-spatial repair neighbourhoods · proposed

- **Hypothesis.** ALNS and MIP halos repair spatial windows. A neighbourhood
  defined by colour structure (all pieces on one alternating colour chain, in
  the spirit of Kempe chains in graph colouring) reaches boards that no
  spatial window of the same size reaches.
- **Borrowed from.** Kempe-chain moves in graph colouring and timetabling.
- **Nearest cell.** Local search / ALNS, sigma cycles. Check
  `why/sigma-cycles` first: if its relabelings already cover these chains,
  close this entry as a variant.
- **Cheap test.** On 10x10 generated boards first (committed record boards
  only after promotion), solve each chain neighbourhood exactly with the existing MaxSAT stage and
  count improving or score-preserving moves to a new basin.
- **Baseline.** Spatial windows with the same number of pieces.
- **Kill.** No more improving or basin-changing moves than spatial windows.
- **Budget.** One session.

## A7. Price set_2 at today's engine speed · proposed

- **Hypothesis.** Set_2's first-solution cost is the same order as set_1's
  2x10^17 nodes. At the JIT backtracker's measured speed, that is a concrete
  number of core-years, and a partial run gives a real lower bound.
- **Borrowed from.** Not new; this is pricing, the pre-requisite for any
  exact-search attempt.
- **Nearest cell.** Benchmarks, open problems.
- **Cheap test.** Two numbers, kept apart. (1) Knuth's random-probe estimate
  of set_2's full tree under the best fill order: the cost of an exhaustive
  search. The estimator is unbiased but heavy-tailed (most samples
  underestimate), so report the probe count and an upper confidence bound,
  not the mean alone. (2) The first-solution cost, from the distribution
  of time to first solution under randomized restarts on generated 10x10
  puzzles with set_2's colour profile, scaled by the engine's measured speed.
  (2) is an ensemble estimate, not set_2's own price: report it as such,
  check it first by predicting first-hit times on held-out generated puzzles,
  and report the spread across the ensemble. Report core-years with an
  uncertainty band for each.
- **Spending rule.** A compute request for set_2 cites the upper confidence
  bound of (1) and says it is an estimate, not a guarantee; (2) can motivate
  a request but never justifies one alone.
- **Baseline.** Set_1's measured first-hit cost, compared with (2) only.
- **Kill.** Not applicable; the output is a number. Promote only if the price
  is within a rented-compute budget the user approves.
- **Budget.** One session.

## A8. A measured GPU backtracker · proposed

- **Hypothesis.** The engine's break-tolerant DFS, ported to CUDA with one
  independent subtree per thread (or per warp), shared read-only tables and
  work stealing, runs at least 10x the placements per second of one CPU core
  of the same generation on the same puzzle, under the same counting
  convention.
- **Borrowed from.** GPU backtracking and branch-and-bound literature cited on
  the GPU page; not new as a method, but the project has no measurement.
- **Nearest cell.** Hardware: GPU (open). This closes the page's open problem
  with a first-party number instead of forum reports.
- **Cheap test.** Stage 1 on the VM (no GPU): write the per-thread search as
  host-and-device code, compile it for the CPU, and check that exhaustive runs
  on generated 6x6 to 8x8 puzzles find exactly the full solution set of a
  CPU oracle. The engine's `Solver` (`engine/src/solver.rs`) is size-generic
  but stops at the first solution, so extend it to resume after each solution
  (or write a separate enumerator), and cross-check the oracle's counts
  against an independent counter first. The benchmark engines are 16x16 only.
  Stage 2 on AWS, after repeating that check on the GPU: one g4dn.xlarge (T4) or g5.xlarge (A10G) for a few hours;
  placements per second and time to first solution on 10x10 generated
  puzzles against the CPU engine run in the same search mode on the same
  instance: the kernel's strict mode against `Solver`, its break-tolerant
  mode against `BreakSolver` (`engine/src/break_solver.rs`) with the same
  break schedule. Only if the 10x10
  ratio clears the kill threshold: best score at a fixed wall-clock budget on
  the 16x16 against the single-core benchmark engine on the same instance's
  CPU.
- **Baseline.** `research/experiments/single-core-benchmark` engine, same
  puzzle, same counting convention, same wall clock.
- **Kill.** Below 10x one core on throughput, or no gain in best score at
  equal wall clock (the GPU page then records the measured ratio).
- **Budget.** Stage 2 under $15 of the AWS reserve.

## A9. Murty k-best checkerboard assignments as a two-ply exact move · proposed

- **Hypothesis.** Fixing one checkerboard colour class and refilling the other is an assignment problem. Ranking the k best non-incumbent assignments (not just the argmax) and refilling the first class again gives an exact two-ply move that leaves rigid basins that greedy refill cannot.
- **Borrowed from.** Matching theory: K. G. Murty, "An Algorithm for Ranking all the Assignments in Order of Increasing Cost", Operations Research 16(3), 1968, DOI 10.1287/opre.16.3.682.
- **Nearest cell.** ALNS / Hungarian half-board refill (mapped): that takes the single optimum of one half. The missing mechanism is ranking beyond the optimum plus the second exact refill, a non-spatial move (compare A6, which uses colour chains).
- **Cheap test.** Generated 8x8 (several colour counts in the hard band) and 10x10, fixed-time arms: baseline ALNS, greedy refill, Murty two-ply.
- **Baseline.** The repo's ALNS and a one-ply refill arm at equal time.
- **Kill.** Murty arm does not beat the baseline on solve rate or on paired gap to the planted optimum by more than 2 standard errors on both sizes; or 90% or more of its improving moves are exchange cycles of 5 cells or fewer (already covered by the rigidity census).
- **Budget.** About 8 core-hours, CPU only.

## A10. Near-codeword spectrum: fewest breaks at each overlap with a planted solution · running

- **Hypothesis.** The curve b_min(a), the fewest breaks of any full board at distance a from the planted solution, tests the overlap-gap reading of the 470 wall for near-solutions: a barrier in the mid-overlap band that grows with size.
- **Borrowed from.** Coding theory, near-codewords / (a,b) trapping sets: D. J. C. MacKay and M. S. Postol, ENTCS 74 (2003), DOI 10.1016/S1571-0661(04)80768-0; enumeration after Karimi and Banihashemi, IEEE Trans. Inf. Theory 58(11), 2012, DOI 10.1109/TIT.2012.2205663.
- **Nearest cell.** The 470 annealed landscape (overlap gap shown for perfect solutions only, conjectured for near-solutions) and the SAT halo (no ground truth). Missing mechanism: break minimisation resolved by overlap with a known solution.
- **Cheap test.** CP-SAT on generated framed 5x5 and 6x6 (exact), 7x7 and 8x8 (bounded), plus where solver boards sit in overlap at 8x8 and 10x10.
- **Baseline.** Annealed first-moment prediction of b_min per band; a polished-random reachable null; a looser palette as no-gap control.
- **Kill.** K1: median barrier H_mid - H_far <= 0 at 5x5 and 6x6 or not growing; or at 7x7/8x8 a mid-band board as good as the best far-band board on 25% or more of instances. K2: b_min(a) <= 2 for some a >= n/3 at 6x6. K3: annealed model off by more than 2 breaks mid-band at 6x6.
- **Budget.** 10 to 14 core-hours, CPU only.

## A11. Coupled real replicas (local entropy) versus parallel tempering · proposed

- **Hypothesis.** Replicas at the same temperature coupled through their overlap (robust ensemble) bias search toward wide, dense regions of near-solutions and solve planted boards PT does not.
- **Borrowed from.** R. Monasson, PRL 75, 2847 (1995), DOI 10.1103/PhysRevLett.75.2847; replicated annealing / robust ensemble (Baldassi et al., PNAS 2016).
- **Nearest cell.** Simulated annealing and parallel tempering (saturated). Missing mechanism: overlap coupling between same-temperature replicas, not temperature swaps.
- **Cheap test.** Generated boards near the solve/no-solve crossing at 8x8 and 10x10, 120 test instances, equal time per arm.
- **Baseline.** The repo's PT and the same replicated sampler with coupling 0.
- **Kill.** Solved-count advantage over the better baseline below 10 instances of 120, or exact one-sided McNemar p >= 0.01.
- **Budget.** About 13 core-hours, CPU only.

## A12. ProbCut statistical cutoffs for exact search · running

- **Hypothesis.** A shallow subtree-size probe predicts whether a DFS subtree holds a solution well enough to cut it with a calibrated false-cut rate, past the depth where proofs stop.
- **Borrowed from.** M. Buro, "ProbCut", ICCA Journal 1995, DOI 10.3233/ICG-1995-18202; Multi-ProbCut (Buro 1999).
- **Nearest cell.** Arc consistency and break-tolerant DFS (saturated). Missing mechanism: statistical cuts calibrated on planted labels, with re-search at a looser threshold.
- **Cheap test.** Round 2 runs it on the hard-instance ladder (research/experiments/hard-ladder) against the ladder's strongest complete DFS.
- **Baseline.** Ladder DFS baselines; shallow value v_0 alone for the AUC stage.
- **Kill.** Probe AUC gain over v_0 below 0.05, R^2 below 0.2, or held-out false-cut rate above 2p; or median nodes to first solution (all passes included) not at most 0.7x the better baseline at 8x8 and 9x9; and the ladder's G1 gate at 8x8.
- **Budget.** A few core-hours, CPU only.

## A13. Synteny decomposition of local optima · proposed

- **Hypothesis.** Local optima of local search on planted boards are rearrangements of rigid blocks of the planted solution. If so, moves that translate and rotate whole blocks reach the solution where single-piece moves stall.
- **Borrowed from.** Comparative genomics, synteny blocks: P. Pevzner and G. Tesler, Genome Research 13(1), 2003, DOI 10.1101/gr.757503.
- **Nearest cell.** ALNS and the rigidity wall (mapped). Missing mechanism: decomposition against ground truth into rigidly moved blocks, then block-level moves.
- **Cheap test.** Stage 1 diagnostic on 8x8 and 10x10 local optima (displaced syntenic mass, planted-adjacency fraction); stage 2 block-move ALNS only if stage 1 passes.
- **Baseline.** Quenched-random chance level of planted adjacencies; the repo's ALNS.
- **Kill.** Stage 1: displaced syntenic mass below 10% of pieces in every cell, or planted-adjacency fraction within 2 standard errors of chance. Stage 2: block-move ALNS no better than ALNS by more than 2 standard errors on both sizes.
- **Budget.** 5 core-hours for stage 1, 5 to 10 for stage 2.

## A14. Local branching: exact Hamming-ball neighbourhoods · proposed

- **Hypothesis.** The rigidity wall is a spatial artefact: exact search over every board within Hamming distance k of the incumbent (any cells, not a window), with reversed cuts for diversification, improves incumbents that spatial halos prove locally optimal.
- **Borrowed from.** M. Fischetti and A. Lodi, "Local branching", Mathematical Programming 98, 2003, DOI 10.1007/s10107-003-0395-5.
- **Nearest cell.** Rigidity wall / SAT halo and MIP repair windows (mapped). Missing mechanism: non-spatial, size-bounded exact neighbourhoods and the reversed-cut diversification step.
- **Cheap test.** CP-SAT balls of radius up to 8 around ALNS/PT incumbents on generated 8x8 and 10x10.
- **Baseline.** ALNS and PT at equal time.
- **Kill.** No improvement over the better of ALNS and PT by more than 2 standard errors and no more planted solves on both sizes; or CP-SAT cannot certify the k=8 ball in 60 s on most 10x10 incumbents.
- **Budget.** About 25 core-hours, CPU only.

## A15. Non-chronological, bond-weighted unassignment (kTAM / iterative modulo scheduling) · proposed

- **Hypothesis.** Growing the board by stable cooperative attachments and detaching weakly bonded pieces anywhere (self-assembly kinetics), or evicting the piece that blocks a cell (iterative modulo scheduling), solves hard-band boards chronological DFS does not.
- **Borrowed from.** Winfree's kinetic Tile Assembly Model (E. Winfree and R. Bekbolatov, "Proofreading Tile Sets", DNA Computing 2003); B. R. Rau, "Iterative modulo scheduling", MICRO-27, 1994, DOI 10.1145/192724.192731. Two round-1 ideas merged here because both rest on the same mechanism.
- **Nearest cell.** Fixed/dynamic fill orders, break-tolerant DFS, min-conflicts local search. Missing mechanism: unassignment chosen by bond count or blocking relation, not by recency.
- **Cheap test.** 90 hard 8x8 boards and the 10x10 band, fixed event budget.
- **Baseline.** DFS and a generic non-chronological arm (random eviction / min-conflicts repair) at the same budget, to separate the mechanism from non-chronological unassignment in general.
- **Kill.** Solves fewer than max(baselines) + 5 boards at 8x8; mechanism killed if the generic arm comes within 4 boards.
- **Budget.** 1 to 4 core-hours, CPU only.

## A16. Backdoor-guided branching · running

- **Hypothesis.** A3's small backdoors can be recognised from root features without the solution, and branching on predicted backdoor cells first shrinks DFS trees below dynamic fail-first ordering.
- **Borrowed from.** Strong backdoors (Williams, Gomes and Selman, IJCAI 2003), via A3.
- **Nearest cell.** Dynamic fill orders and learned move ordering (saturated). Missing mechanism: a target (backdoor membership) measured on ground truth; impact-based and fail-first orderings are baselines.
- **Cheap test.** Round 2 on the hard-instance ladder.
- **Kill.** Backdoor-membership AUC not above fail-first with CI above 0, or the ladder's G1 gate fails at 8x8 against the strongest dynamic ordering.
- **Budget.** A few core-hours.

## A17. Cut-multiset feasibility pruning · running

- **Hypothesis.** A check that the colours demanded along the frontier of a partial board can be supplied by the unused pieces (count, parity and flow conditions) prunes DFS nodes that local arc consistency keeps.
- **Borrowed from.** Transfer-matrix counting over colour multisets (round-1 A5).
- **Nearest cell.** Flux invariants, border Eulerian propagator, parity-defect floor, LP/ILP. Round 2 checks implication by those first and stops if it is a duplicate.
- **Cheap test.** On the hard-instance ladder, nodes and wall clock.
- **Kill.** No significant node reduction on any of the round-2 rungs (6x6 c=5, 7x7 c=6, 8x8 c=10). A pass needs median paired node ratio at most 0.5 at 8x8.
- **Budget.** A few core-hours.

## Parked from the round-1 idea pass

- **Distinguished-point collision detection for local-search walks** (cryptanalysis; van Oorschot and Wiener, J. Cryptology 12(1), 1999). Its premise, that basin rediscovery wastes compute, overlaps A4. A4 then found that parallel tempering already reaches more distinct basins than an explicit diversity archive, so rediscovery is not shown to be the bottleneck; stays parked.
- **Abstraction refinement on piece distinctness** (program synthesis; Wang, Dillig and Singh, POPL 2018). Closest to the fake-480 dead end, where distinctness was found to bind globally; the session's own critique predicts kill (a). Parked until a cheap distinctness-core measurement exists.

## Ideas parked for hardware not available

- An FPGA build of Field's design (needs a board).
