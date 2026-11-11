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

## A1. Planted-solution recovery by spectral statistics · proposed

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

## A3. SAT backdoors on solvable small boards · proposed

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

## A4. Quality-diversity search over basin descriptors · proposed

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

## A5. Exact count of frame placements with the corner constraint · proposed

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

## Ideas parked for hardware the VM does not have

- A tuned full-board GPU backtracker (open on the GPU page).
- An FPGA build of Field's design.

Both need the user's machine or rented compute.
