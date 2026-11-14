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
test, scheduled at larger scale), `published`, `open` (a stage passed, the
next stage undecided).

Night of 2026-11-12 (sixteen judged tracks, ledger entries of that date):
results below name the track and its local branch; no branch of that night is
merged or pushed, so an entry's evidence is on that branch until it is. A
brief for a new track cites its row in `night/2026-11-13/ROUTES.md` (published form: `research/experiments/approach-pricing/routes.md`) or says
that none exists.

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

## A10. Near-codeword spectrum: fewest breaks at each overlap with a planted solution · killed

- **Hypothesis.** The curve b_min(a), the fewest breaks of any full board at distance a from the planted solution, tests the overlap-gap reading of the 470 wall for near-solutions: a barrier in the mid-overlap band that grows with size.
- **Borrowed from.** Coding theory, near-codewords / (a,b) trapping sets: D. J. C. MacKay and M. S. Postol, ENTCS 74 (2003), DOI 10.1016/S1571-0661(04)80768-0; enumeration after Karimi and Banihashemi, IEEE Trans. Inf. Theory 58(11), 2012, DOI 10.1109/TIT.2012.2205663.
- **Nearest cell.** The 470 annealed landscape (overlap gap shown for perfect solutions only, conjectured for near-solutions) and the SAT halo (no ground truth). Missing mechanism: break minimisation resolved by overlap with a known solution.
- **Cheap test.** CP-SAT on generated framed 5x5 and 6x6 (exact), 7x7 and 8x8 (bounded), plus where solver boards sit in overlap at 8x8 and 10x10.
- **Baseline.** Annealed first-moment prediction of b_min per band; a polished-random reachable null; a looser palette as no-gap control.
- **Kill.** K1: median barrier H_mid - H_far <= 0 at 5x5 and 6x6 or not growing; or at 7x7/8x8 a mid-band board as good as the best far-band board on 25% or more of instances. K2: b_min(a) <= 2 for some a >= n/3 at 6x6. K3: annealed model off by more than 2 breaks mid-band at 6x6.
- **Budget.** 10 to 14 core-hours, CPU only.

- **Result (round 2, [PR #25](https://github.com/raphael-anjou/eternity2/pull/25)).** Killed by a second independent judge; the first ruled the run invalid because the corner pin does not remove the 4-fold board rotation, so the far band held a turned copy of the planted solution and K1c was undetermined. With distance quotiented by rotation (amendment A6, committed before the fix-run solves), K1a fires on closed data: 5x5 C=13 median mid band 4 breaks against far band 7 (B* = -3, 30/30 optimal), 6x6 C=14 median B* in [-7, -1]. The judge reproduced all 60 5x5 optima with an independent CP-SAT model. No primary instance has a 0-break board far from the planted solution, so the kill covers single-solution-class small boards only: the mid band is a cheap saddle, not a barrier. Next: the same measurement between two distinct solutions on 6x6 and 7x7 palettes with 5 to 50 expected solution classes (kill if median H_between - H_near <= 0 or fewer than 10 usable instances per size). Experiment: `research/experiments/near-codeword-spectrum/`.

## A11. Coupled real replicas (local entropy) versus parallel tempering · proposed

- **Hypothesis.** Replicas at the same temperature coupled through their overlap (robust ensemble) bias search toward wide, dense regions of near-solutions and solve planted boards PT does not.
- **Borrowed from.** R. Monasson, PRL 75, 2847 (1995), DOI 10.1103/PhysRevLett.75.2847; replicated annealing / robust ensemble (Baldassi et al., PNAS 2016).
- **Nearest cell.** Simulated annealing and parallel tempering (saturated). Missing mechanism: overlap coupling between same-temperature replicas, not temperature swaps.
- **Cheap test.** Generated boards near the solve/no-solve crossing at 8x8 and 10x10, 120 test instances, equal time per arm.
- **Baseline.** The repo's PT and the same replicated sampler with coupling 0.
- **Kill.** Solved-count advantage over the better baseline below 10 instances of 120, or exact one-sided McNemar p >= 0.01.
- **Budget.** About 13 core-hours, CPU only.

## A12. ProbCut statistical cutoffs for exact search · killed

- **Hypothesis.** A shallow subtree-size probe predicts whether a DFS subtree holds a solution well enough to cut it with a calibrated false-cut rate, past the depth where proofs stop.
- **Borrowed from.** M. Buro, "ProbCut", ICCA Journal 1995, DOI 10.3233/ICG-1995-18202; Multi-ProbCut (Buro 1999).
- **Nearest cell.** Arc consistency and break-tolerant DFS (saturated). Missing mechanism: statistical cuts calibrated on planted labels, with re-search at a looser threshold.
- **Cheap test.** Round 2 runs it on the hard-instance ladder (research/experiments/hard-ladder) against the ladder's strongest complete DFS.
- **Baseline.** Ladder DFS baselines; shallow value v_0 alone for the AUC stage.
- **Kill.** Probe AUC gain over v_0 below 0.05, R^2 below 0.2, or held-out false-cut rate above 2p; or median nodes to first solution (all passes included) not at most 0.7x the better baseline at 8x8 and 9x9; and the ladder's G1 gate at 8x8.
- **Budget.** A few core-hours, CPU only.

- **Result (round 2, [PR #20](https://github.com/raphael-anjou/eternity2/pull/20)).** Killed by an independent judge. Its own stage-2 criterion fired: V1 solved 13/30 on the first R8 pass, at or below the kill line of 15. At 8x8 c=10 G1 is NO-GAIN for both variants against `scb` and `fc-dfs`; the cuts removed planted-solution branches (V1 13/30, V2 17/30 solved vs `fc-dfs` 19/30) and cost nodes at 6x6. The stage-1 AUC null is structural (row-boundary probes shorter than one row cannot see the sibling difference), so it is not evidence for the "signal fades two cells out" wall. A variant would need probes of n to 2n cells at mid-row nodes, at least 60 sibling sets, cuts on the ladder's strongest DFS. Experiment: `research/experiments/probcut/`.

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

## A15. Non-chronological, bond-weighted unassignment (kTAM / iterative modulo scheduling) · killed

- **Hypothesis.** Growing the board by stable cooperative attachments and detaching weakly bonded pieces anywhere (self-assembly kinetics), or evicting the piece that blocks a cell (iterative modulo scheduling), solves hard-band boards chronological DFS does not.
- **Borrowed from.** Winfree's kinetic Tile Assembly Model (E. Winfree and R. Bekbolatov, "Proofreading Tile Sets", DNA Computing 2003); B. R. Rau, "Iterative modulo scheduling", MICRO-27, 1994, DOI 10.1145/192724.192731. Two round-1 ideas merged here because both rest on the same mechanism.
- **Nearest cell.** Fixed/dynamic fill orders, break-tolerant DFS, min-conflicts local search. Missing mechanism: unassignment chosen by bond count or blocking relation, not by recency.
- **Cheap test.** 90 hard 8x8 boards and the 10x10 band, fixed event budget.
- **Baseline.** DFS and a generic non-chronological arm (random eviction / min-conflicts repair) at the same budget, to separate the mechanism from non-chronological unassignment in general.
- **Kill.** Solves fewer than max(baselines) + 5 boards at 8x8; mechanism killed if the generic arm comes within 4 boards.
- **Budget.** 1 to 4 core-hours, CPU only.

- **Result (round 3, [PR #23](https://github.com/raphael-anjou/eternity2/pull/23)).** Killed by an independent judge on the method clause; the mechanism clause did not fire. Run at 6x6 c=5 and 7x7 c=6 first, 1e6 events per instance (one event = one placement or removal). R6 solved/30: ladder DFS `scb-gacolor_ac3_random` 30 (KM median 1,866 events), plain DFS 30, random eviction 19, kTAM detachment (V1) 23, IMS blocker eviction (V2) 27; G1 against the ladder DFS NO-GAIN for both (paired ratios 186 and 100). R7: ladder DFS 10 (16 within 1e6 nodes), every other arm 0. R8 not triggered. V2 beats random eviction at R6 (ratio 0.43, p 0.004; holds over the judge's run seeds 2 to 5 and a matched control), so choosing which piece to remove by its blocking relation carries signal, but without propagation it loses to complete propagating DFS by two orders of magnitude. Exploratory equal-CPU check at R7 (11.8 s per instance): ladder DFS 16/30, V1 and V2 1/30 (lower bounds: most method runs hit their node cap at about 80% of that CPU). Randomised restarts on the ladder DFS were suggested and are not taken up (restart portfolios are saturated in EXPLORED.md). Follow-ups: A19, A20, A21. Experiment: `research/experiments/ktam-unassignment/`.

## A16. Backdoor-guided branching · killed

- **Hypothesis.** A3's small backdoors can be recognised from root features without the solution, and branching on predicted backdoor cells first shrinks DFS trees below dynamic fail-first ordering.
- **Borrowed from.** Strong backdoors (Williams, Gomes and Selman, IJCAI 2003), via A3.
- **Nearest cell.** Dynamic fill orders and learned move ordering (saturated). Missing mechanism: a target (backdoor membership) measured on ground truth; impact-based and fail-first orderings are baselines.
- **Cheap test.** Round 2 on the hard-instance ladder.
- **Kill.** Backdoor-membership AUC not above fail-first with CI above 0, or the ladder's G1 gate fails at 8x8 against the strongest dynamic ordering.
- **Budget.** A few core-hours.

- **Result (round 2, [PR #24](https://github.com/raphael-anjou/eternity2/pull/24)).** Killed by an independent judge: K1 does not fire but K2 and K3 do. Stage 1: backdoor membership is predicted with test AUC 0.637 against 0.373 for fail-first (Delta +0.264, CI [0.233, 0.295]), but the model adds only +0.010 (CI includes 0) over domain size with the sign learned on training, and within interior cells its AUC is 0.518, so the signal is mostly position class plus domain size. Stage 2 on 20 test seeds per rung: branching first on predicted backdoor cells is NO-GAIN against `bb-mrv` (A3 propagation plus MRV) at every rung; R6 node ratio 17.2 (learned) and 8.1 (impact feature), R7 1/20 solved against 15/20, R8 (8x8 c=10) 1 to 2/20 against 20/20, and the loss holds on nodes alone without the 240 s guard. Side observation, not preregistered: `bb-mrv` beats the ladder baseline `scb-gacolor_ac3_random` at all three rungs (R8 20/20 vs 12/20, KM median 95k vs 468k nodes, faster in wall clock). Next: preregister `bb-mrv` as a ladder baseline candidate on fresh seeds (kill if G1 fails at 8-c10 or the wall-clock ratio against scb is above 1), and if it survives re-baseline A12 and A17 on it. Further backdoor work should use backdoors for decomposition or restarts, not as a first-branch rule. Experiment: `research/experiments/backdoor-branching/`.

## A17. Cut-multiset feasibility pruning · killed

- **Hypothesis.** A check that the colours demanded along the frontier of a partial board can be supplied by the unused pieces (count, parity and flow conditions) prunes DFS nodes that local arc consistency keeps.
- **Borrowed from.** Transfer-matrix counting over colour multisets (round-1 A5).
- **Nearest cell.** Flux invariants, border Eulerian propagator, parity-defect floor, LP/ILP. Round 2 checks implication by those first and stops if it is a duplicate.
- **Cheap test.** On the hard-instance ladder, nodes and wall clock.
- **Kill.** No significant node reduction on any of the round-2 rungs (6x6 c=5, 7x7 c=6, 8x8 c=10). A pass needs median paired node ratio at most 0.5 at 8x8.
- **Budget.** A few core-hours.

- **Result (round 2, [PR #21](https://github.com/raphael-anjou/eternity2/pull/21)).** Kept by an independent judge as a small gain. The count/parity half was already in `gacolor` and the ring-cut half measured at zero, so only Hall matching (every empty cell gets a distinct unused piece, Kuhn augmenting paths) was tested. 6x6 c=5: GAIN, paired node ratio 0.900 (V1, p 0.0002) and 0.876 (V2, p 0.018), Holm-corrected. 7x7 c=6 and 8x8 c=10: NO-GAIN. Wall clock neutral to slightly worse. 8x8 c=10 does not separate pruning methods (plain DFS matches the baseline in nodes at about 26x lower wall time). Next variant: Hall plus Regin filtering on the frontier with a prune-invariant value order, on an 8x8 rung between c=8 and c=10; kill if NO-GAIN there or wall ratio above 1 against the better of the baseline and plain DFS. Experiment: `research/experiments/cut-multiset-pruning/`.
- **Result (night 2026-11-12, Track 4, branch `night/2026-11-12-t4-hall-on-mrv`, local, not merged).** Killed by an independent judge when rebaselined on `bb-mrv` (A3 propagation, piece all-different, free MRV), which already applies Hall's condition for size-1 sets. V1 and V2, Holm over 2 variants, paired G1: 6x6 c=5 NO-GAIN (1.000 for both), 7x7 c=6 GAIN (0.993 and 0.992), 8x8 c=10 GAIN (0.999 and 0.997): under 1% fewer nodes, no G1 PASS. At every GAIN rung the wall ratio against `bb-mrv` is above 1 (R7 1.028 [1.013, 1.055] and 1.025 [1.009, 1.038]); the check itself costs 2 to 5% of wall clock; K2 fires for both variants. Round 2's 0.90 at R6 was against `scb`, which lacks piece all-different, and vanishes against `bb-mrv`; it stands as measured and is not adopted. The "next variant" above (Hall plus Regin on an 8x8 rung between c=8 and c=10) was not run; it would now have to beat `bb-mrv` on work, not nodes. Experiment: `research/experiments/hall-on-mrv/`.

## A18. Exact frame marginals: where border pieces sit across all valid frames · proposed

- **Hypothesis.** Exact per-slot and per-adjacency frequencies over all valid frames (A5 counts 4.05e37 on the official ring) are non-uniform enough that the planted frame's placements rank above chance, beyond what colour rarity already gives, and ordering the border by them shrinks DFS trees.
- **Borrowed from.** Forward-backward marginals on a transfer matrix (hidden Markov models; L. R. Rabiner, "A tutorial on hidden Markov models and selected applications in speech recognition", Proc. IEEE 77(2), 1989, DOI 10.1109/5.18626), applied to the A5 counter. Pieces of the same edge-arc type are interchangeable in M1, so per-piece marginals reduce to per-type marginals divided by the type multiplicity.
- **Nearest cell.** Belief-propagation ordering and survey propagation (dead ends: signal fades two cells out) and A1 spectral pair statistics (killed: nothing beyond colour rarity). Missing mechanism: exact rather than approximate marginals, restricted to the frame subproblem where they are computable, scored on ground truth.
- **Cheap test.** Generated 8x8 and 10x10 boards with a planted solution: exact frame marginals, then the rank of each planted placement against a colour-rarity ranking of the same slot, paired per board. If that survives, a border ordering by marginal inside the ladder DFS. The official-ring marginals are computed as a descriptive side result only.
- **Baseline.** Colour-rarity ranking (the A1 baseline) for recovery; the ladder's strongest propagating DFS with its own border order for nodes.
- **Kill.** Paired planted-placement rank no better than colour rarity at both sizes (CI including 0), or no significant node reduction on any ladder rung. Most valid frames do not extend to a full solution, so frame-only signal may not transfer; that is the expected failure mode.
- **Budget.** Under 2 core-hours on reduced boards; a few runs of the A5 counter (about 9 min per prime, 17.5 GiB peak on the official ring) for the descriptive official marginals. No AWS.

## A19. Ablation of the ladder's best DFS · proposed

- **Hypothesis.** One component of `scb-gacolor_ac3_random` (border-first MRV, AC-3 after every placement, the GAColor per-colour pairing check, class balance, random value order) carries most of its 44x node lead over plain DFS at 6x6 c=5, and the same component explains its lead at 7x7 c=6.
- **Borrowed from.** Component ablation, standard in CP solver evaluation. A measurement, not a new method: it tells later candidates which inference to build on.
- **Nearest cell.** Arc consistency and dynamic ordering (saturated as methods); not previously ablated on node-counted, hardness-matched instances.
- **Cheap test.** Remove one component at a time (and add one at a time to plain DFS) on ladder R6 and R7 test seeds, 1e6 nodes and CPU time per instance, 3 run seeds where the order is random.
- **Baseline.** The full solver and plain row-major DFS.
- **Kill.** Not a method, so no kill: the outcome is the per-component paired node and CPU ratio table. Dropped if no single removal changes nodes significantly at either rung (the lead is then spread over components).
- **Budget.** Under 1 core-hour.

## A20. Blocker eviction as the backjump rule inside propagating DFS · killed

- **Hypothesis.** Inside the ladder's propagating DFS, undoing the placement that blocks the cell whose domain was wiped out (the IMS rule that beat random eviction in A15), then re-propagating, solves more 7x7 and 8x8 boards than chronological backtracking at equal events and CPU.
- **Borrowed from.** B. R. Rau, "Iterative modulo scheduling" (MICRO-27, 1994); conflict-directed backjumping (P. Prosser, "Hybrid algorithms for the constraint satisfaction problem", Computational Intelligence 1993).
- **Nearest cell.** No-good learning (mapped: no-goods outrun memory at 16x16) and A15 (killed: eviction without propagation). Missing mechanism: blocker choice by conflict, without stored no-goods, on top of full propagation.
- **Cheap test.** Ladder R7 and 8x8 c=9 test seeds, 3 run seeds per arm, 1e6 events plus CPU time.
- **Baseline.** The ladder DFS, and a twin that evicts a random placed piece with identical propagation (matched control).
- **Kill.** NO-GAIN against the ladder DFS on G1 at both rungs and solves at most baseline + 3 on the run-seed median; mechanism killed if the random twin comes within 2 solves and is not significantly worse.
- **Budget.** 1 to 3 core-hours, CPU only.
- **Result (night 2026-11-12, Track 2, branch `night/2026-11-12-t2-blocker-backjump`, local, not merged).** Killed by an independent judge on the method clause; the mechanism clause also fires. 6x6 c=5 and 7x7 c=6, 1e6 nodes, tabu tenure 16 chosen on calibration seeds only. R6 solved/30: `scb-gacolor_ac3_random` 30 (KM median 951 nodes), `bb-mrv` 30 (460), blocker eviction 30 (4,334), random-eviction twin 30 (3,868); G1 fails against `scb` (4.68 [2.40, 6.55]) and `bb-mrv` (9.18 [5.39, 15.20]), wall 32.3x and 24.5x, so the method is dropped at R6. R7 solved/30: `scb` 16, `bb-mrv` 23, blocker 5 (run-seed median 4), twin 2 (median 3). K2 fires but has low power at R7 (both arms nearly all censored): it shows no detectable difference between two failing arms, not that blocker choice carries no signal (R6 paired ratio 0.947 [0.574, 3.670]). As run, 24 to 31% of blocker dead ends fall back to evicting the latest decision, and the tabu list is cleared when no decision is left, so both arms are partly restart methods. 8x8 not run. The kill covers this repair-search implementation only; it does not cover conflict-directed backjumping proper (Prosser 1993), conflict sets from propagation explanations, dynamic backtracking or stored no-goods, Rau's IMS placement rule, other tenures, or 8x8. Reopen only with conflict sets built from propagation explanations, scored on work. Experiment: `research/experiments/blocker-backjump/`.

## A21. Value order by propagation impact · killed

- **Hypothesis.** In the ladder's propagating DFS, trying first the piece and rotation whose propagation removes the fewest values from neighbouring domains (least-constraining value measured by real propagation, not by colour counts) shrinks trees at 7x7 c=6 and 8x8 c=9 against its random value order.
- **Borrowed from.** Impact-based search (P. Refalo, "Impact-based search strategies for constraint programming", CP 2004).
- **Nearest cell.** Fixed and dynamic fill orders, learned move ordering. Novelty check against those first; record whether a value-impact order was already measured.
- **Cheap test.** Ladder R6, R7 and 8x8 c=9 test seeds, nodes and CPU time per instance.
- **Baseline.** The ladder DFS with its random value order (3 run seeds).
- **Kill.** NO-GAIN on every rung, or a node GAIN whose CPU ratio is above 1 at every rung.
- **Budget.** Under 2 core-hours.
- **Result (night 2026-11-12, Track 3, branch `night/2026-11-12-t3-value-impact`, local, not merged).** Killed by an independent judge: G1 of `vi-impact` (value order by option bits removed by propagation, least first, inside `bb-mrv`) fails against `bb-mrv` at every rung, so no node gain exists for the wall-clock gate. Node ratios 6x6 c=5 1.075 [0.779, 1.931], 7x7 c=6 0.829 [0.544, 1.316], 8x8 c=10 1.096 [0.669, 1.871]; wall 1.143, 0.814, 1.141. Against the probe-then-random twin: 1.297 [1.058, 1.621] at R6 (worse than random), 1.000 at R7, 1.128 at R8; against `mrv-rand` 1.325, 1.000, 1.127. Scope: count of option bits, least first, inside `bb-mrv`; not a log-product (Refalo) impact, not the most-constraining direction, not 8x8 c=9 (the brief moved the rung to c=10). Judge's observation: run seed 3 of random value order passes G1 against `bb-mrv` at 8x8 c=10 (0.495 [0.409, 0.768]) while seed 1 sits at 1.623, so any order-type method needs run-seed controls (draft protocol rule P1). Experiment: `research/experiments/value-impact-order/`.

## A22. `bb-mrv` as a ladder baseline: fresh seeds, scaling, ablation · promoted

- **Hypothesis.** The A16 side observation holds on fresh seeds: `bb-mrv` (A3 propagation with piece all-different, plus free MRV) beats `scb-gacolor_ac3_random` in nodes and wall clock, and its node ratio shrinks with board size (a smaller effective branching factor rather than a constant factor).
- **Nearest cell.** Propagating complete DFS (the ladder baseline). Not a new method: a baseline check that decides what every later candidate is compared against.
- **Cheap test.** Ladder R6, R7 seeds 21..30; 8x8 c=10 seeds 21..50; a fixed c = critical + 2 track for n = 6..10, seeds 1..30; four one-change ablations.
- **Kill.** Not adopted if G1 fails at 8x8 c=10 on seeds 21..50 or the median paired wall-clock ratio there is above 1; "gain grows with n" only if the slope CI is entirely below 0.
- **Result (round 3, [PR #26](https://github.com/raphael-anjou/eternity2/pull/26)).** Kept by an independent judge, scoped to the 8x8 c=10 rung: 30/30 solved against 20/30, KM median 69,036 vs 555,342 nodes, G1 ratio 0.147 (CI [0.076, 0.313]), wall ratio 0.194; node-identical on the judge's rerun and G1 PASS for scb run seeds 1 to 5. At the critical-colour rungs R6 and R7 G1 fails (0.80, 0.70) and `bb-mrv` is slower in wall clock (1.64x, 1.85x), so it is not adopted there. Scaling: no evidence of a growing gain, because 9x9 c=11 and 10x10 c=12 are censored for both solvers on every pair even at the extended 1e7 / 180 s cap (binding slope +0.089 per n). Exploratory only: over n = 6..8 the median log10 ratio goes -0.26, -0.38, -0.83. Ablations at R8 (node ratio against the parent): no hidden singles 1.11, border-first order 1.83, random value order 1.62; adding A3's piece all-different to scb cuts its nodes to 0.23 but costs per node (wall 0.90). Next: a separately preregistered scaling run with per-size caps of 1e8 to 1e9 nodes on the +2 track, and re-baselining A17 on `bb-mrv` at 8x8 c=10. Experiment: `research/experiments/mrv-baseline/`.
- **9x9 follow-up (night 2026-11-12, Track 1, measurement, branch `night/2026-11-12-t1-9x9-pilot`, local, not merged).** Killed by the preregistered rule, read as "no evidence the node ratio keeps shrinking", not "it stopped". Seeds 101..108, cap 1e8, `bb-mrv` against `scb-gacolor_ac3_random`: M = 0.000 against T = -0.83215 (K1), U = 6 of 8 pairs uninformative against a limit of 4 (K2, which decides); 6 pairs censored on both solvers; seed 103 r <= -0.106, seed 106 r <= -1.001 (upper bounds), both favouring `bb-mrv`. The cap, not the ratio, was binding. The judge would not spend a 30-seed run on this ratio, because plain row-major DFS is reported (a referee's runs) to solve these 9x9 boards in 2 to 6 s, about three orders of magnitude cheaper per node.
- **Baseline status after the night (Tracks 0, 5).** `bb-mrv-fast` (Track 0) is node-identical to `bb-mrv` on 140 gate instances and 5.3x to 6.1x faster; use it wherever only counts, statuses and boards are read. The 47-entrant tournament (Track 5) reproduces both ladder anchors with 0 mismatches over 1,310 rows; at R6 and R8 the first two places lie inside run-seed spread, and at R8 row-major DFS solves 24/30 at about 1% of `bb-mrv`'s wall clock (one cell order: column-major 20, five other orders 0). Open question carried forward: which reference to use above 8x8. The night's recommendation (not yet a protocol rule) is plain DFS or `bb-mrv-fast`, scored on work or time (draft rules P7, P10).

## Idea slot of the night 2026-11-12 (A23 to A31)

Thirteen proposals were novelty-checked and ranked in `night/2026-11-12/IDEAS.md`
(full cards in `ideas-raw/`). The night's census (Track 6), replay (Track 12),
score-bound gap (Track 8) and partial-match ladder (Track 7) decided several
of them at stage 0. Entries below keep the IDEAS rank; numbers come from each
track's judge. Proposals with no baseline or kill criterion written stay in
`IDEAS.md` and `ROUTES.md` ("looks untried") and are not entries here.

## A23. Revocable branching cells by proof-number search (IDEAS rank 1) · proposed

- **Hypothesis.** At R7, MRV's chosen cell often has a much larger refutation tree than another of the 3 smallest-domain cells; df-pn over revocable splits, with `bb-mrv`'s propagation, finds the cheaper refutations at a net gain in work.
- **Borrowed from.** Proof-number search (Allis et al., AIJ 66, 1994); df-pn (Nagai 2002).
- **Nearest cell.** Dynamic fill orders (one committed cell per node). Missing mechanism: effort moved between refutations. Not touched by the census.
- **Cheap test.** Stage 0 headroom oracle (r = MRV cell refutation size over the best of 3, 300+ refuted nodes, R6 and R7), then df-pn k=3 at R6 and R7, then a growth stage on the +2 track n = 6..8, counting work (nodes plus propagations).
- **Baseline.** The better of `bb-mrv` and `scb` per rung; controls df-pn k=1 and dom/wdeg on `bb-mrv`.
- **Kill.** K0: median r at most 1.25 and r at least 2 at under 20% of nodes, both rungs. K1: R7 paired work ratio not at most 0.7 or CI including 1. K2: k=3 not below k=1 (Wilcoxon p at least 0.01). K3: wall ratio above 1. Growth slope CI not below 0.
- **Budget.** Stage 0 under 2, stage 1 about 4, growth about 3 core-hours.

## A24. Boundary-keyed cavity summaries (IDEAS rank 2) · killed

- **Hypothesis.** Most of `bb-mrv`'s dead nodes at R7 are refuted by a k x k cavity (k at most 3) given its boundary and pool, and those keys recur, so tabulated refutations cut work.
- **Result (night 2026-11-12, Track 6, branch `night/2026-11-12-t6-dead-node-census`, local, not merged).** Killed at stage 0 on K0a only: cavities with r <= 3 prove 0.0083 of weighted R7 dead-node weight (0.057 unweighted; 0.040 with propagated domains) against a threshold of 0.30. K0c compared two infinite medians and decides nothing (judge). Scope: `bb-mrv` parent, 6x6 and 7x7 critical rungs; a share is an upper bound on savings with unchanged branching.

## A25. Cut-keyed trace partitioning (IDEAS rank 3) · killed

- **Hypothesis.** `bb-mrv` loses at the critical rungs by joining cut-colour cases; partitions keyed on two cut-edge colours, persistent through the descent, close failing subtrees at their roots.
- **Result (night 2026-11-12, Track 12, branch `night/2026-11-12-t12-cut-keyed-replay`, local, not merged).** Killed at stage 0 on its own K2 with a partition-matched control: upper median node ratio, cut key 0.483 (R6) and 0.486 (R7); random key off the cut line with the same partition count 0.444 and 0.419, better on all five seeds at both rungs and on 30 of 30 R7 instances; every arm costs more propagation work than `bb-mrv` (R7 cut 1.211). Track 6's earlier kill did not stand (one-shot share, unmatched control); this replay supersedes it. Any two-edge partition prunes about half of `bb-mrv`'s nodes, generic to low-colour-set edges. Untested: a cut key under a row-scan solver, where the cut line is a real frontier.

## A26. Relaxed decision diagrams as a score bound in break-tolerant DFS (IDEAS rank 4) · open

- **Hypothesis.** A width-W relaxed decision diagram (frontier colours exact, piece distinctness relaxed) bounds the best score reachable from a prefix tightly enough to prune break-tolerant DFS before the endgame.
- **Borrowed from.** Bergman, Cire, van Hoeve, Hooker, IJOC 28(1), 2016.
- **Nearest cell.** Break-tolerant DFS with scheduled late breaks; the repo's `BreakSolver` has no per-node bound.
- **Stage 0 result (night 2026-11-12, Track 8, branch `night/2026-11-12-t8-score-bound-gap`, local, not merged).** Passed on the threshold: median gap exactly 4 edges at W=4096, 7x7 c=6 with 3 rows left (K1 fires above 4), 3 edges better than W=1, which equals the trivial bound. The gap grows with size (1, 2, 4 at 2 rows left for n = 6, 7, 8) and was measured only on deeply dead prefixes. Undecided, not killed: the night overturned two attempts to close it (a model row giving 92% to 98% late nodes, a port measurement giving 4.5e-7) and the record-cost referee saw 0.4% to 53% of legal nodes late across seeds and clue settings. Whether a record engine spends enough of its search in the tail is not known.
- **Next test.** Stage 1: work to reach optimum minus k (k = 1..4) on 8x8 c=8 boards where `BreakSolver` stalls, with and without the bound, prefixes drawn near the optimum as well as dead ones, and a wall-clock arm. Before any 16x16 build: the late-node share of a faithful port of the published engine (see A32 and the 2026-11-13 port-fidelity track).
- **Baseline.** `BreakSolver` with its schedule; control W = 1.
- **Kill.** Stage 1 work ratio above 0.5 or wall ratio above 1 at any k.
- **Budget.** 4 to 8 core-hours for stage 1.

## A27. Band consistency by composed tile transducers (IDEAS rank 5) · killed

- **Hypothesis.** GAC over a k-row band, built from the cells' current domains, removes values arc consistency keeps.
- **Result (night 2026-11-12, Track 6).** Killed at stage 0: k=2 bands prove 0.0023 (R6) and 0.0018 (R7) of weighted dead-node weight against 0.05; even k=3 stays at 0.0113 and 0.0077. Scope: `bb-mrv` parent; the `scb` parent, with weaker domains, was not measured.

## A28. Boundary-word lemmas learned from refutations (IDEAS rank 12) · killed

- **Hypothesis.** Refutations of large dead subtrees yield small verified lemmas (boundary words of a pocket that cannot be filled) that generalise and prune elsewhere.
- **Result (night 2026-11-12, Track 6).** Killed at stage 0 for the `bb-mrv` parent: 0 of 1,447 sampled R7 dead nodes with subtree size 200 or more are proven by a pocket of up to 2n cells (0 of 88 at R6, 0 of 456 at the 7x7 c=8 null), against a median local-core rate of 10%. The `scb` parent was not measured.

## A29. Adaptive multilevel splitting on local-search trajectories (IDEAS rank 6) · proposed

- **Hypothesis.** Local search fails on small planted boards because the exit to the solution is rare, not because a barrier blocks it (A10); splitting trajectories at level crossings of the score turns one rare event into a chain of common ones and solves boards PT does not.
- **Borrowed from.** Adaptive multilevel splitting (rare-event estimation by interacting particle systems). Full card: `night/2026-11-12/ideas-raw/statistical-inference.md`.
- **Nearest cell.** Simulated annealing and parallel tempering (saturated per EXPLORED, a status graded argued) and restart portfolios. Missing mechanism: resuming from a level crossing with selection, isolated by a random-kill control.
- **Cheap test.** Stage 0 validity gate at 6x6 (AMS estimate within 2 s.e. of direct Monte Carlo); calibration of c on seeds 9001..9004 by A4's rule; 7x7 puzzle seeds 1..30, 2 run seeds, equal move evaluations for every arm.
- **Baseline.** A4-configuration PT and N independent Metropolis runs; control AMS-RAND (same cloning, uniformly drawn kills).
- **Kill.** Method killed if AMS solves fewer than max(PT, IND) + 5 of 30 at 7x7 (mean over run seeds) or the one-sided exact McNemar test against the better baseline gives p >= 0.01; mechanism killed if AMS-RAND comes within 2 solves and is not significantly worse.
- **Budget.** About 3 core-hours.

## A30. Piece-priced frontier bound with safe banding (IDEAS rank 7) · proposed

- **Hypothesis.** A frontier dynamic programme with exact cycle structure and Lagrangian piece prices proves dead a share of `bb-mrv`'s dead nodes that colour-only feasibility and Hall matching miss.
- **Borrowed from.** Lagrangian relaxation (operations research) with banded alignment (computational biology). Full card: `night/2026-11-12/ideas-raw/pair-or-compbio.md`.
- **Nearest cell.** LP/ILP relaxations (mapped), tensor-network counting, A17 Hall matching (killed on `bb-mrv`). The census (Track 6) measured only its floor: colour-only feasibility proves 0.017 of weighted R7 dead-node weight (0.110 unweighted), so it is not decided.
- **Cheap test.** Stage 1a: the bound at sampled dead nodes of the census trees at R6 and R7, subtree-weighted, with four one-component controls.
- **Baseline.** `scb-gacolor_ac3_random` and `bb-mrv`, same seeds and node rule; controls lambda = 0 (colour-only), row and column chains only, Hall matching at the same nodes, unbanded against banded.
- **Kill.** K1: weighted share proven dead below 0.3 at depths 6 and 12 combined, or not above the Hall control by more than 2 s.e. K2: lambda = 0 within 2 s.e. of the priced bound. K3: the chain-only control within 2 s.e. of the frontier bound. Read against the census: every small-region oracle tested so far proved almost none of the weight before the peak.
- **Budget.** About 10 core-hours (the card's estimate).

## A31. Evolve `bb-mrv`'s cell and value ordering code (night 2026-11-12 idea slot, E1) · proposed

- **Hypothesis.** A search over programs for the two ordering hooks of `bb-mrv`, scored automatically on work, finds an ordering that beats MRV with index value order at the critical rungs and on growth with n.
- **Borrowed from.** AlphaEvolve (Novikov et al., arXiv:2506.13131); open implementation OpenEvolve.
- **Nearest cell.** Corpus priors and learned move ordering, dynamic fill orders. Missing mechanism: search over the ordering program itself against a scorer outside its reach. Tracks 2 and 3 measured two hand-written points of this space (both killed).
- **Cheap test.** Evolution sees calibration seeds only (R6, R7, 8x8 c=10) and scores solves within a fixed work budget; selection on test seeds plus 9x9, never seen in evolution. Harness, node counter and verifier out of the evolved code's reach.
- **Baseline.** `bb-mrv`; `scb` at R6 and R7. Controls: the best of an equal number of random mutations without selection; A21 as the hand-written reference; run-seed controls on order (A21's judge).
- **Kill.** The selected program fails G1 on work against `bb-mrv` at 7x7 c=6 test seeds, or its growth slope CI is not below 0, or its test-seed gain is under half its calibration-seed gain.
- **Budget.** About 500 programs at about 2 core-minutes each (15 to 20 core-hours) plus model usage, to be estimated and approved before launch.

## Other candidates from the 2026-11-12 night with a baseline and a kill

## A32. Five-clue quota share sweep on the faithful port · proposed

- **Hypothesis.** On the real pieces with five clues, the share of the keying's privileged sides the quota demands by cell 160 has an optimum that beats the port's default by at least 2x in legal deep yield per second.
- **Origin.** Track 15 (night 2026-11-12, branch `night/2026-11-12-t15-five-clue-quota`, judged keep): the same absolute quota lets the default keying live (12 of 16 openings reach legal depth 201) and kills the published keying (2 of 16); a matched share lets both live (12 to 14 of 16); a tight share kills the default (0 of 16). No arm beat the default, and which segment of the curve binds was not shown. Track 14 (killed) is the confounded result this explains. Nobody has swept the share.
- **Nearest cell.** Break-tolerant DFS with scheduled late breaks (record family, reported). Missing: the quota stated as a share and swept, on a port whose differences from the published engine are fixed.
- **Cheap test.** Five clues, the port with the orientation fix on (free: 0.983 [0.95, 1.04] of yield per second, no non-stranded run changed), 16 held-out openings per share point, 1e9 nodes per run; measure legal nodes at depth 201 or more per second.
- **Baseline.** The default keying under its own quota (12 to 13 of 16 openings positive in Track 15).
- **Kill.** No share point beats the default by 2x with a bootstrap interval excluding 1.
- **Budget.** About one evening on two machines (Track 15 used 5.06 thread-hours for 32 arms x 16 openings). Depends on the port-fidelity work started 2026-11-13 (track D2, branch `night/2026-11-13-d2-port-fidelity`, independently assessed twice and kept on 2026-11-13; with five clues the faithful defaults reach depth 18 to 54 against 33 to 215 for the old defaults, so five-clue runs use the legacy switches until a five-clue schedule is measured).

## A33. Partial-match ladder with entrants that keep searching · proposed

- **Hypothesis.** A partial-match instrument (best verified score at fixed work on planted-optimum boards) becomes usable when its constructive entrants keep searching after their first complete board, or are the 16x16-native engines run at small n.
- **Origin.** Track 7 (night 2026-11-12, branch `night/2026-11-12-t7-partial-match-ladder`, killed for its pool and budget): 2 of 5 entrants in the [2%, 4%] gap band against 3 required; `BreakSolver` stopped at its first complete board after at most 3.8e6 of 5.12e7 placements; ALNS from a greedy start was weakest everywhere. The judge scoped the kill to that pool and budget.
- **Cheap test.** Track 7's PREREG and calibration rule on n in {8, 10, 12}, with `BreakSolver` replaced by an entrant that continues after a complete board and with `blackwood_bt`, `verhaard_faithful_v2` or the beam producer added where they run at small n.
- **Baseline.** Track 7's committed rows (rung A medians: `sa` 3.57%, `pt` 3.57%, `break-r1` 6.25%).
- **Kill.** Track 7's criterion (1): fewer than 3 of 5 entrants stall in the [2%, 4%] band on a qualifying rung, or no cell at n = 10 or 12 qualifies.
- **Budget.** About 10 thread-hours (Track 7 used about 9.2).

## A34. Border candidates ordered by first-ring fill count · proposed

- **Hypothesis.** The true border word admits more first-ring fills than shuffled words passing the same corner tests (z = 1.40 against valid frames on generated boards, about 2 ranking bits, confirmed by the referee), and ordering border candidates by that count cuts nodes to the first solution.
- **Origin.** `night/2026-11-12/memos/frame-information.md` section 10, refereed in `theory-and-frame.REVIEW.md`. The author would not run it first; stated odds of bearing on a full solution under 0.5%.
- **Cheap test.** 100 generated planted boards with a unique solution at the largest size the compiled strict engine exhausts in under a minute (expected 9x9 or 10x10 with clues); same row scan and engine, (A) default piece order, (B) border candidates sorted by the first-ring count of the corner window they complete.
- **Baseline.** Arm A.
- **Kill.** Geometric mean of nodes(A) / nodes(B) below 2.0, or its 95% interval including 1, or the ratio at the larger of two sizes below the ratio at the smaller.
- **Budget.** About 3 core-hours.

## Arc of 2026-11-13 to 2026-11-14 (A35 to A55)

Results of the two-day arc closed on 2026-11-14 (ledger entries of those dates; evidence in `research/experiments/approach-pricing/`, `relaxation-small-boards/`, `learning-vs-propagation/` and the experiment folders named below; the routes index is `research/experiments/approach-pricing/routes.md`). The hunt for a method stopped on 2026-11-14 with no route priced at 10 bits or more; closed entries first, then what stays open and what would reopen the question. Everything on generated boards is a toy result, never Eternity II progress.

### Closed by the arc

## A35. Cheap refutation of wrong frames of the real pieces · killed

- **Hypothesis.** A wrong 60-piece frame can be refuted about 100 times cheaper than plain search, with a core of fewer than 54 cells, so frame-first search with learned cores beats row scan.
- **Result (frame-cores, ledger 2026-11-13).** Killed: lower medians 0.89, 0.12 and 1.00 against a line of 100. The stage 2 pass (36.75 bits [32.90, 53.19], median core 5 cells) is a corner artefact (21 of 23 near frames die on a 3x3 clue block). Nested arc cost 27.68, 31.37, 35.39 and 39.49 bits, each 2.2 to 2.5 bits above its break-even budget. The far stratum is undecided (see A44).
- **Kill reference.** `research/experiments/frame-cores/` (README and results); `research/experiments/approach-pricing/notes/nogood-frame-search-price.md` and its re-check.

## A36. A static scan order that beats row scan · killed

- **Hypothesis.** Some static fill order (comb, border-first, centre-out, rings, searched orders) shrinks the exact tree by 10 bits or more.
- **Result (orders-and-pins, ledger 2026-11-13).** None of 18 static orders is shown to beat row scan: the kill line is reached for 15 and 3 are undecided (the estimator's population dies); a searched order's 2.11-digit win is withdrawn. Row scan is within 0.24 bit of the best static order found by annealing (no certificate; see A47).
- **Kill reference.** `research/experiments/orders-and-pins/`; `research/experiments/approach-pricing/notes/gap-6b-reformulation-search-space.md` and its re-check.

## A37. Linearisation at degree 2 as a polynomial-price reformulation · killed

- **Hypothesis.** Linearising the board equations at degree 2 pins the solution at a price near 2^67.
- **Result (ledger 2026-11-13).** Killed: degree 2 determines 10 of 10 boards at 3x3, 0 of 10 at 4x4, 0 of 5 at 5x5. Degree 3 at 5x5 did not fit in memory, so the row is not formally closed (see A48).
- **Kill reference.** `research/experiments/approach-pricing/notes/gap-6a-linearisation-degree.md`.

## A38. A relaxation solver (convex lift, low-rank form, nonnegative factor) as a way to solve the board · killed

- **Hypothesis.** A relaxation of the board (degree-2 semidefinite lift, or its low-rank and nonnegative-factor forms) is exact or rounds to solutions at a cost that grows slowly enough to reach 16x16.
- **Result (ledger 2026-11-14).** Closed as a negative on toy boards. The lift is certified clean on three of five 5x5 boards, undecided on one and not exact on one (about 0.042 of the mass per cell on ten placements in no solution); it costs 400 to 1,100 s per 5x5 solve and 119 GiB or more at 16x16. The low-rank form is a randomised search that annealing beats by 8 to 35 times per solution at 5x5 and 3.6 to 63 times at 6x6; its work grows with an exponent of 4.5 in P (3.97 to 5.31, two sizes). The nonnegative factor solved 0 of 50 starts at 5x5. Exactness of the lift at 6x6 stays open (A41).
- **Kill reference.** `research/experiments/relaxation-small-boards/`.

## A39. Search that learns, as a time gain on the row-scan tree · killed for the narrow family priced

- **Hypothesis.** Learned reasons (clauses, nogoods, shared failure tables) are worth 10 bits or more on the real 16x16.
- **Result (ledger 2026-11-13, 2026-11-14).** About 2 bits (ceiling 6.6) for a free, perfect store of small reasons attached to row-scan nodes; a general "empty" is unproven. A clause-learning solver needs 2^5 to 2^6.5 fewer conflicts than tree nodes on 5x5 and 6x6, which is not a time gain. Toy learning measurement: +1.31 bits at 5x5 and +2.03 at 6x6 over a propagation-only control, no 7x7 value, stop line undecidable (A43).
- **Kill reference.** `research/experiments/approach-pricing/notes/gap-1-search-that-learns.md` and its re-check; `research/experiments/learning-vs-propagation/`.

### Open after the arc

Entries below are open items and reopening conditions from the arc's report. Where the source gave no kill criterion, the one written here was set when the entry was added, before any run.

## A40. Reopening condition: a published method with a class-level gain · open

- **Hypothesis.** A method outside this repo's map shows a gain that is not a constant factor on a row-scan tree.
- **Nearest cell.** Every row of the routes index; the ceiling table is the first filter.
- **Cheap test.** Price it against the routes index and the ceiling table of a perfect free cut (57.1, 44.8, 29.8, 17.1, 10.7, 6.8, 2.4 bits at depths 52, 64, 80, 96, 112, 128, 144), then run the falsifying experiment on small boards.
- **Baseline.** Largest gain priced in the arc: about 5.5 bits of nodes (arc consistency), at a per-node cost of about 13 bits at the measured speed.
- **Kill.** Priced below 10 bits, or it only bites after depth 128, or the falsifying experiment fails.
- **Budget.** Under a day for the experiment.

## A41. Exactness of the convex lift at 6x6 · open

- **Hypothesis.** The degree-2 lift with entrywise nonnegativity is exact on generated 6x6 boards under a largest-leak objective.
- **Nearest cell.** A38. Differs: the 6x6 largest-leak solves, which ran but did not converge.
- **Cheap test.** Three 6x6 boards, converged to residual 1e-7, one solver family, then a second solver family; the penalty rule fixed first.
- **Baseline.** 5x5 seed 3 leaks about 0.042 per cell; three 6x6 runs stopped unconverged with mass off solutions per cell 0.843, 0.865, 0.730, and lower bounds of 24, 13.5 and 13 hours to the target.
- **Kill.** Mass off solutions above 0.05 per cell on 2 of 3 boards at residual 1e-7. A reading before convergence decides nothing. Even a pass is worth 0 bits on its own.
- **Budget.** 6 to 170 core-hours (the unconverged runs suggest the high end).

## A42. Second solver on the 5x5 seed 3 leak, and hull equality · open

- **Hypothesis.** The seed 3 leak at 5x5 is a property of the lift, not of one solver family.
- **Nearest cell.** A38.
- **Cheap test.** A primal-dual solver on seed 3; equality of the lift with the hull checked by optimisation; the 73 against 41 percent comparison for the distance-2 mask.
- **Baseline.** 403 solutions confirmed by an independent enumeration; a point feasible to 1.5e-9 keeps 0.042 per cell on ten placements.
- **Kill.** The second solver drives the leak below 1e-6 per cell (the leak was a first-solver artefact).
- **Budget.** About 20 core-minutes for the first item.

## A43. Learning solvers at 7x7 and on real-piece prefixes, with a propagation-only control · open

- **Hypothesis.** A learning solver's lead over propagation alone rises with size.
- **Nearest cell.** A39.
- **Cheap test.** The learning-vs-propagation design extended: 7x7 with a lower bound on cut runs, 8x8, a cardinality solver, weighted degree with restarts, and the real-piece part (40 wrong prefixes at free depths 112 to 88).
- **Baseline.** +1.31 bits at 5x5 and +2.03 at 6x6 from learning; CaDiCaL over row scan with arc consistency +2.19 at 6x6 and at most +4.28 at 7x7 (rise at most +2.09, no lower bound). In seconds the learners lose at every size.
- **Kill.** The stated line: the lead over row scan with arc consistency does not rise by 2 bits from 6x6 to 7x7, and not by 3 bits from depth 104 to 88.
- **Budget.** About 80 core-hours; the program's odds of 10 bits at 16x16 are 1 percent.

## A44. Far stratum of frame cores · open

- **Hypothesis.** The far stratum of wrong frames (those passing the corner blocks, far from the true frame) resolves below the nested break-even cost, which would reopen frame search driven by nogoods.
- **Nearest cell.** A35.
- **Cheap test.** Raise the cap on the 16 censored of 37 far frames, or sample more, on the real pieces.
- **Baseline.** 37 frames, 16 censored, median 75.12 bits, upper end infinite; nested arc cost 27.68 to 39.49 bits, 2.2 to 2.5 bits above break-even. A necessary condition only.
- **Kill.** Nested arc cost above its break-even budget at every measured arc length once the cap is raised.
- **Budget.** Not costed (the first stage used about 59 core-hours).

## A45. Growth of non-tree solvers against the tree on the same boards · open

- **Hypothesis.** Annealing, difference map, low-rank form or message passing grows more slowly than the tree on the same boards.
- **Nearest cell.** A38 and the local-search rows of EXPLORED.
- **Cheap test.** Each arm and the tree on the same boards at 6x6 and 7x7, work counted in evaluations or CPU seconds.
- **Baseline.** Restart annealing finds a verified solution every 29 to 257 CPU-seconds at 6x6 and nothing in 210 runs at 7x7 (rate below 26 percent per 3e9-step run, not shown to be zero); low-rank exponent 4.5 in P on two sizes; exhaustive search 0.3 to 0.6 s at 6x6.
- **Kill.** A non-tree solver's work grows by 0.75 or more of the tree's from 6x6 to 7x7.
- **Budget.** About 120 core-hours; one arm surviving 5 percent, 10 bits 0.5 to 1 percent.

## A46. Second reader for the 2-bit sketch bound · open

- **Hypothesis.** A sound colour-blind sketch table of the used set saves at most 2.0 bits (a table costs at least 2^126.7 against 2^128.7).
- **Nearest cell.** The lossy-representations row (gap 2).
- **Cheap test.** An independent derivation, then a toy check on more than the 8 boards used.
- **Baseline.** The two-list join gains 3.0 bits (2^131.6); the 2.0 bound is one derivation, weakly toy-checked.
- **Kill.** A counterexample, or a sound sketch that saves more than 2.0 bits.
- **Budget.** One study; a proof, not a gain.

## A47. Certificate that row scan is the best static order at the bottleneck · open

- **Hypothesis.** No static order has a lower bottleneck level than row scan.
- **Nearest cell.** A36.
- **Cheap test.** A certificate or counterexample on the level profile of the real pieces.
- **Baseline.** Row scan within 0.24 bit of the best order found by annealing (no certificate); an order that finishes rows one or two at a time gains at most 5.3 or 6.7 bits.
- **Kill.** An order found at least 1 bit below row scan at the bottleneck.
- **Budget.** One study; a proof, not a gain.

## A48. Linearisation at degree 3 on 5x5 · open

- **Hypothesis.** Degree 3 pins the solution at 5x5, so the needed degree does not grow past 3.
- **Nearest cell.** A37.
- **Cheap test.** Generated 5x5 boards on a machine with about 30 GB, over GF(2) and a large prime.
- **Baseline.** Degree 3 at 4x4 pins 6 or 7 of 10 boards (the rest need 4); at 5x5 the system has 16.2 million monomials and stopped on memory at rank 2.35 million.
- **Kill.** The degree needed at 5x5 is above 3 or above that at 4x4.
- **Budget.** 10 to 20 core-hours; the program's odds are 25 percent at 5x5 and under 0.1 percent at 16x16.

## A49. Injective-only hole in algebraic reformulation · open

- **Hypothesis.** An object with few terms that is correct on boards without repeated pieces can be built without knowing the solutions.
- **Nearest cell.** The algebra-and-counting row (gap 6a); an object with one term per solution always exists.
- **Cheap test.** A desk construction, then a toy check.
- **Baseline.** No model prices it; fifteen reformulations priced, none below 2^134.4.
- **Kill.** A proof that any such object needs the solutions, or a size lower bound at or above the tree.
- **Budget.** Half a day of desk work; cannot be priced in bits.

## A50. Weighted degree at 8x8 and 9x9, pseudo-Boolean solvers, spread of tree size over randomised dynamic orders, non-abelian tile group · open

- **Hypothesis.** One of four untested items inside the closed gap rows moves a price by 3 bits or more: conflict-driven weighted degree, cardinality (pseudo-Boolean) reasoning, the dispersion of exhaustion-tree size over randomised dynamic orders on the real pieces (needs no solution), or the non-abelian group of the unplaced tiles.
- **Nearest cell.** Adaptive-orders, search-that-learns, shape-of-the-tree and tile-set-structure rows.
- **Cheap test.** Weighted degree on 8x8 and 9x9 toys; the dispersion study on the real pieces; a computer-algebra run for the group.
- **Baseline.** Weighted degree +0.5 bits at 6x6 and -0.4 at 7x7; quotients of the group up to order 12 refute 0 of 32 undecided prefixes and 0 of 54 known-untileable ones.
- **Kill.** Gain below 3 bits at both sizes, or no refutation beyond forward checking.
- **Budget.** Not costed.

## A51. Low-rank form re-check items · open

- **Hypothesis.** The low-rank form's cost to a first solution is changed by a retuned restart period or a third size.
- **Nearest cell.** A38.
- **Cheap test.** NumPy evaluations to a first solution; the replicate rows on the second machine written up; 50 runs of about a minute for the first.
- **Baseline.** 5,083 evaluations per solution at 5x5, 460,416 at 6x6 (censored median 270,051); median solve at step 605 of 1,000, so a shorter restart interval would cut off most solves.
- **Kill.** A third size puts the exponent below 3.4, or no tuning lowers the 6x6 cost by 2 times.
- **Budget.** About an hour for the first item.

## A52. Census tilt and painted-board excess (anomaly hunt items) · open, not recommended

- **Hypothesis.** The census tilt at 1.5 to 1.8 times the size bias, or the unexplained painted-board excess (about 0.05 digit), carries a usable signal.
- **Nearest cell.** The prefix-statistics results (D4).
- **Cheap test.** 0.5 core-hour each.
- **Baseline.** Nothing survives a painted-board null and a count of comparisons; neither item is worth more than about 2 bits; odds under 0.1 percent.
- **Kill.** No gain at ten times the population.
- **Budget.** 0.5 core-hour each.

## A53. Reopening condition: tiles of the solution become available · open

- **Hypothesis.** An outside source reveals the approach behind the five-clue 466 or any tiles of the solution.
- **Nearest cell.** The pin study (pin-map, orders-and-pins).
- **Cheap test.** Price the tiles with the pin study on painted boards.
- **Baseline.** Best extra pin about 10 bits (10.05), median 5.79; a 10^15 run needs more than 10 and probably 16 to 24 spread pins; touching pairs lose about half of the second tile's worth.
- **Kill.** Fewer than 10 spread pins, or pins that are not spread, leave the run above 10^15 nodes.
- **Budget.** Not applicable until it happens.

## A54. Reopening condition: hardware gains of many orders of magnitude · open

- **Hypothesis.** A hardware change closes the gap of 56 to 73 bits (17 to 22 orders of magnitude).
- **Nearest cell.** The hardware rows of EXPLORED.
- **Cheap test.** Re-price the row-scan tree at the new cost per node.
- **Baseline.** Every credible multiplier together is about 2.5 orders; a quantum backtracking constant sits 21 to 22 orders above a 2029 roadmap machine (survey, not independently checked).
- **Kill.** A multiplier below 15 orders of magnitude (it would reopen plain search, not any method here).
- **Budget.** Not applicable.

## A55. Reopening condition: a closure is overturned by a re-check or a run · open

- **Hypothesis.** A re-check or a run overturns one of the arc's closures: A41 (lift exact at 6x6), A44 (far stratum below break-even), A45 (non-tree solver grows by less than 0.75 of the tree's), A43 (learning lead rises with size).
- **Nearest cell.** The four entries named.
- **Cheap test.** As each entry states.
- **Baseline.** As each entry states.
- **Kill.** As each entry states.
- **Budget.** As each entry states.

## Parked from the round-1 idea pass

- **Distinguished-point collision detection for local-search walks** (cryptanalysis; van Oorschot and Wiener, J. Cryptology 12(1), 1999). Its premise, that basin rediscovery wastes compute, overlaps A4. A4 then found that parallel tempering already reaches more distinct basins than an explicit diversity archive, so rediscovery is not shown to be the bottleneck; stays parked.
- **Abstraction refinement on piece distinctness** (program synthesis; Wang, Dillig and Singh, POPL 2018). Closest to the fake-480 dead end, where distinctness was found to bind globally; the session's own critique predicts kill (a). Parked until a cheap distinctness-core measurement exists.

## Ideas parked for hardware not available

- An FPGA build of Field's design (needs a board).
