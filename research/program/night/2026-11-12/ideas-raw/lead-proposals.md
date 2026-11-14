# Lead proposals (night 2026-11-12)

Written by the lead session at Raphaël's request. Status: unchecked. Each goes
through the same novelty check and tournament as the inventor proposals and gets
no priority. DOIs are from memory and must be verified by the scout.

All four aim at the partial-match regime (best score on a board nobody solves)
or at an explicit open problem, not at ladder node counts.

## L1. Relaxed decision diagrams as an upper bound for break-tolerant search

- **Stall attacked.** EXPLORED.md, "Break-tolerant DFS with scheduled late
  mismatches: saturated, 467 (Verhaard), 470 (Blackwood)". The record engines
  decide where breaks are allowed by a hand-made schedule. Nothing proves that a
  prefix can no longer reach the target score, so the search cannot prune on
  score, only on the schedule.
- **Field and mechanism.** Operations research: relaxed decision diagrams. Fill
  the board row by row; a DD layer is a cell, a state is the open frontier
  (edge colours facing unfilled cells) plus the set of used pieces. The exact
  diagram is too wide, so states are merged down to a fixed width W by relaxing
  the used-piece set (union of available pieces) while keeping the frontier
  exact or partly exact. The longest path in the relaxed diagram is a valid
  upper bound on the best score reachable from a prefix. Used inside the
  break-tolerant DFS, it turns a scheduled search into a branch and bound on
  score: prune when bound < target.
- **Original paper.** D. Bergman, A. A. Cire, W.-J. van Hoeve, J. N. Hooker,
  "Discrete Optimization with Decision Diagrams", INFORMS Journal on Computing
  28(1), 2016. Sequencing with all-different relaxations: A. A. Cire and
  W.-J. van Hoeve, "Multivalued Decision Diagrams for Sequencing Problems",
  Operations Research 61(6), 2013.
- **Nearest explored cell and missing mechanism.** LP / ILP relaxation bounds
  (mapped) and the exact tail endgame (plateaued). Missing: a bound that keeps
  the frontier colours exact and relaxes only piece distinctness, computed
  incrementally per node, with a width knob that trades time for tightness.
- **Cheapest falsifying test.** Generated 7x7 and 8x8 boards at critical
  colours, target = best known score (planted, so the optimum is known).
  Stage 0: bound gap (bound minus true best completion) as a function of
  remaining rows and width W in {64, 512, 4096}, on prefixes sampled from the
  break-tolerant DFS. Stage 1: nodes to prove "no board scores >= optimum + 1
  from this prefix" with and without the bound.
- **Baseline and control.** `BreakSolver` with its schedule; control: the same
  DD with W = 1 (a pure counting bound), to separate the diagram from any bound.
- **Kill.** Stage 0: median gap with 3 rows remaining above 4 edges at every W
  (the bound is then too loose to prune before the endgame, where the exact
  tail already works). Stage 1: node ratio above 0.5 or wall ratio above 1.
- **Budget.** 4 to 8 core-hours, CPU only.
- **Known risks, for the scout to check first.** (1) AGENDA.md records that in
  the fake-480 dead end "distinctness was found to bind globally". This bound
  relaxes exactly piece distinctness, so it may be loose everywhere; stage 0
  measures that before any solver is written. (2) The premise that record
  engines prune only on a break schedule is inferred from one line of
  EXPLORED.md, not from the engines' code or pages; if they already use a score
  bound, this is a variant and must name what that bound lacks.

Note on L2: the cryptanalysis inventor proposed the same mechanism
independently (`cryptanalysis.md`), after reading the BANDSAW page, and expects
its K1 to fire. The two are merged for the tournament; that file is the fuller
version.

## L2. Dissection instead of meet in the middle

- **Stall attacked.** EXPLORED.md, "Meet in the middle (BANDSAW): mapped, open,
  about 20x memory per extra mismatch", and the explicit open problem "the
  meet-in-the-middle memory wall".
- **Field and mechanism.** Cryptanalysis: dissection of composite problems.
  Split the board into four or seven bands instead of two. Guess the
  intermediate frontier between outer bands, solve the sub-problems by smaller
  meet-in-the-middle steps, and stream the matches instead of storing a full
  half. For a problem made of r independent blocks, dissection reaches the
  same time as meet in the middle with a fraction of the memory (for 7 blocks,
  memory of one block instead of three and a half).
- **Original paper.** I. Dinur, O. Dunkelman, N. Keller, A. Shamir, "Efficient
  Dissection of Composite Problems, with Applications to Cryptanalysis,
  Knapsacks, and Combinatorial Search Problems", CRYPTO 2012. Ancestor:
  R. Schroeppel and A. Shamir, SIAM J. Computing 10(3), 1981.
- **Nearest explored cell and missing mechanism.** BANDSAW (two halves, one
  table). Missing: a multi-level split with guessed intermediate states. The
  known obstacle is piece distinctness across bands, which is not a block
  structure; the proposal treats a piece conflict as one more mismatch to be
  repaired or rejected at merge time, and measures how often merges die on it.
- **Cheapest falsifying test.** Generated 6x6 and 8x8 boards: enumerate all
  boards with at most b breaks (b = 0, 1, 2) by BANDSAW and by a 4-band
  dissection; compare peak memory and time, and count merge candidates
  rejected for piece reuse.
- **Baseline and control.** BANDSAW at equal time; control: 4 bands merged
  pairwise without the guess step (plain 4-list).
- **Kill.** Peak memory at equal time not at least 4x below BANDSAW at 8x8, or
  more than 99% of merges rejected for piece reuse (distinctness then defeats
  the block structure and the method does not apply).
- **Budget.** About 6 core-hours.

## L3. Price 471 for a record engine by rare-event splitting

- **Stall attacked.** AGENDA A2 is still only proposed: nobody knows whether
  471 is a compute question or a method question for a given engine. Naive
  extrapolation needs to see the event; a 471 has never been seen.
- **Field and mechanism.** Statistical inference: multilevel splitting on the
  depth reached by a break-tolerant DFS restart within its break budget.
  Levels are depths; a restart that reaches level k is cloned with fresh
  randomness in its remaining value order. The product of the conditional
  level-crossing rates is an unbiased estimate of P(one restart completes at
  score >= S), for S that direct sampling never reaches. This merges A2 with
  tonight's adaptive multilevel splitting proposal, applied to the engine that
  holds the records and not to local search.
- **Original paper.** F. Cérou and A. Guyader, "Adaptive Multilevel Splitting
  for Rare Event Analysis", Stochastic Analysis and Applications 25(2), 2007.
  P. Glasserman, P. Heidelberger, P. Shahabuddin, T. Zajic, "Multilevel
  splitting for estimating rare event probabilities", Operations Research
  47(4), 1999.
- **Nearest explored cell and missing mechanism.** Restart portfolios and
  heavy tails (saturated, runtime distributions). Missing: an estimator for
  events too rare to observe, with an unbiasedness check.
- **Cheapest falsifying test.** Generated 10x10 boards where the event is
  observable: estimate P(score >= S) by splitting with 1% of the budget, then
  measure it directly with the full budget. Only if that agrees: the same on
  16x16 corner-pinned variants for S = 460, 465 (observable), then S = 471.
- **Baseline.** Direct Monte Carlo, and A2's extreme-value fit.
- **Kill.** Splitting estimate outside the direct estimate's 95% interval on
  more than 2 of 10 boards, or relative variance not below direct sampling at
  equal budget.
- **Budget.** Stage 1 about 5 core-hours. The 16x16 stage is a request, not a
  night task.

## L4. A partial-match ladder

- **Stall attacked.** Every judged experiment of rounds 2 and 3 measures nodes
  to a first perfect solution on boards that have one. The open problem is the
  best score on a board where no engine finds a perfect solution. No shared,
  hack-proof instrument measures that regime, so method ideas that could move
  470 are judged on the wrong target.
- **Mechanism.** Not an import, an instrument (like the hard ladder was).
  Generated boards at sizes 8x8 to 12x12 at critical colours, where the planted
  board gives the known optimum but no engine reaches it in the budget. Fixed
  metric: best verified score at fixed work, and work to reach optimum minus k
  for k = 1..6, with the same counter and verifier as the ladder. Calibrated so
  that the repo's record-style engines stall 2% to 4% below the optimum, the
  relative gap of 470 against 480.
- **Cheapest test of usability.** The instrument is usable only if it orders
  the repo's existing engines the way the 16x16 does (break-tolerant DFS above
  ALNS above plain annealing). If the order differs, the small boards do not
  reproduce the regime.
- **Kill.** Rank correlation with the 16x16 ordering of at least 5 existing
  engines below 0.8, or no size and colour count where the engines stall in
  the 2% to 4% band.
- **Budget.** About 10 core-hours.
