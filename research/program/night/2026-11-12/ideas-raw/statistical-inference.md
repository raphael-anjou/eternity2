# A23. Adaptive multilevel splitting on local-search trajectories · proposed

Field: statistical inference (rare-event estimation by interacting particle systems).

## Stall attacked

Local search fails to solve small planted boards, yet the small-board landscape shows no energy barrier in front of the solution. Two judged results, read together, say this:

- `research/experiments/quality-diversity-basins/README.md` (A4 calibration): "The smallest candidates (8 at 8x8, 10 at 10x10) already gave 0 of 8 solves". That is parallel tempering with 320M move evaluations per run on planted boards.
- `research/LEDGER.md`, "Round 2, A10 near-codeword spectrum: killed": "on single-solution-class small boards the mid band is a cheap saddle, not a barrier; no board far from the planted solution has 0 breaks."
- Supporting, from a site page rather than a judged README (`web/content/research/build/local-search/parallel-tempering.mdx`): "Near the top of this puzzle the obstacle is of the entropic kind: improving boards sit beyond wide neutral plateaus whose exits are rare ... what the ladder cannot do is make a rare exit common."

If no energetic barrier blocks the way, the low-break path to the planted board exists but one trajectory rarely follows it to the end. That is a rare-event problem, and temperature does not make a rare event common. This proposal attacks that, and only that.

Caveat: A10 measured framed 5x5 and 6x6 boards with 13 and 14 colours, while A4 failed on unframed 8x8 with 8 colours. "No barrier" and "local search fails" have not been seen in the same regime yet. The test's calibration step puts both in one regime before any arm runs.

## Field and mechanism

**Field:** statistical inference, specifically rare-event estimation by interacting particle systems.

**Mechanism:** adaptive multilevel splitting (AMS) with the generalized tie rule.

- **Replicas.** N replicas run the same fixed-temperature Metropolis dynamics on full boards (A4's `run_sa_steps_fixed_temp` kernel).
- **Importance function.** The score (matched edges).
- **Trajectory rules.** Each replica is a trajectory with its own level, equal to its running maximum score. It stops when it reaches the target (a perfect board) or falls to a floor (start score minus Δ).
- **Each iteration:**
  1. Kill every replica whose running maximum equals the population minimum. All tied replicas are killed, which is the rule that keeps the estimator unbiased with discrete scores.
  2. For each killed replica, copy a surviving replica chosen uniformly at random, cut at the first step where that survivor's score went strictly above the killed level.
  3. Continue the copy from that exact board with fresh randomness.
- **Output.** Run until a perfect board appears, all replicas are tied (extinction), or the budget runs out. As a by-product, the method gives an unbiased estimate, p̂ = Π(1 − K_i/N), of the chance that one trajectory reaches the target.

If the path to the solution passes through intermediate score levels, each with a conditional success chance p_i, a single run succeeds with probability Π p_i. AMS needs about N·log(1/Π p_i) iterations instead of 1/Π p_i independent tries. If the exit gives no score signal (a pure plateau), all replicas tie and the population goes extinct at a measurable level. That outcome is itself a result.

## Original paper

- F. Cérou and A. Guyader, "Adaptive Multilevel Splitting for Rare Event Analysis", *Stochastic Analysis and Applications* 25(2), 417–443, 2007. DOI 10.1080/07362990601139628 (checked on Crossref).
- Tie handling and unbiasedness for discrete importance functions: C.-E. Bréhier, M. Gazeau, L. Goudenège, T. Lelièvre, M. Rousset, "Unbiasedness of some generalized adaptive multilevel splitting algorithms", *Annals of Applied Probability* 26(6), 3559–3601, 2016. DOI 10.1214/16-AAP1185, arXiv 1505.02674.

## Hypothesis

On planted 7x7 boards where parallel tempering solves at most 25%, the path from greedy boards to the planted solution splits into score levels whose conditional crossing probabilities are each not tiny. If so, AMS at equal move evaluations solves clearly more boards than PT and than the same number of independent Metropolis runs. The gain comes from choosing whom to kill by running maximum, not from cloning alone.

## Nearest explored cell and missing mechanism

- **Nearest cells:** simulated annealing and parallel tempering (saturated), restart portfolios (saturated), beam search (saturated), and the two replica agenda entries, A4 (killed) and A11 (proposed).
- **What is missing in all of them:**
  - Selection on a path quantity (a trajectory's running maximum) rather than on the current score or a Boltzmann weight.
  - Restarting copies from the exact board where a survivor first crossed the level.
  - Kill levels set by the population's own minimum rather than by a schedule.
  - The unbiased product estimator, which says per board how rare the solving event is.
- **Why that attacks this stall:** the stall is "temperature cannot make a rare exit common". AMS changes nothing about temperature. It turns one rare event into a chain of more common events.
- **Why it is not a restart schedule:** a restart throws away the state reached. AMS resumes from a level crossing and keeps going. The random-kill control below isolates this difference.

## Cheapest falsifying test (6x6 or 7x7)

**Boards.** Generated unframed boards from `generate(7, c, seed)` with known planted solutions.

**Stage 0, validity gate at 6x6.** Use a colour count where plain Metropolis reaches the perfect board in at least 5% of 2,000 independent runs. Compare AMS's p̂ (20 independent AMS runs) with the direct Monte Carlo estimate. Agreement within 2 standard errors is required. If they disagree, the implementation is wrong, and nothing else runs.

**Stage 1 calibration.** Pick the smallest c at which A4-configuration PT solves at most 25% of calibration puzzles 9001 to 9004 within M = 5e7 move evaluations, the same rule A4 used.

**Stage 1, the measurement.** Puzzle seeds 1 to 30 at 7x7, with 2 run seeds. All arms get equal move evaluations M, with the temperature T and Δ fixed on the calibration seeds only. Four arms:

1. **PT:** A4's ladder and kernel.
2. **IND:** N independent Metropolis runs at T from greedy boards. This is the population without selection.
3. **AMS:** N = 200 replicas, with the generalized tie rule.
4. **AMS-RAND:** identical cloning at first-crossing points, but the killed replicas are drawn uniformly. This is the matched control.

**Recorded per run:** solved or not, best score, the stop reason (solved, extinction level, or budget), and p̂.

## Baseline and control

- **Baselines:** PT (the repo's strongest local search) and IND (separates the mechanism from simply running a population of trajectories).
- **Control:** AMS-RAND. It separates selection by running maximum from branching at hitting points.
- **Diagnostic, not a gate:** the extinction level shows where in the score the rare event sits.

## Kill criterion

- **Method killed** if either holds:
  - AMS solves fewer than max(PT, IND) + 5 of the 30 puzzles at 7x7 (mean over run seeds).
  - The exact one-sided McNemar test against the better baseline gives p ≥ 0.01.
- **Mechanism killed** if AMS-RAND comes within 2 solves of AMS and is not significantly worse.
- **Structural reading** if 50% or more of AMS runs end by tie extinction below the perfect score. Then the exit is entropic in the score coordinate: score is the wrong importance function. That gets a ledger line with the extinction-level histogram, and the plan is not to swap in a hand-tuned importance function.

## Budget

About 3 core-hours, CPU only:

- **Arms:** 30 puzzles × 2 seeds × 4 arms × 5e7 move evaluations, around 1 core-hour at about 1e7 evaluations per second. That rate is an assumption; the calibration step measures it.
- **Calibration and stage 0:** under 2 core-hours.
- **AWS:** none.
- **Reuse:** A4's local-search kernel, plus one new replica manager of roughly 300 lines.

## Closest prior work

- D. Aldous and U. Vazirani, "'Go with the winners' algorithms", FOCS 1994, DOI 10.1109/SFCS.1994.365742 (checked on Crossref).
- Population annealing (J. Machta, Physical Review E 82, 026704, 2010; cited from memory, DOI not checked).
- Constructive sequential Monte Carlo, cited on the beam-width page (`pipelines/beam-width.mdx`, "Sequential Monte Carlo steering").

## Why this is not a variant of it

- **Go-with-the-winners** clones survivors by depth in a construction tree. At these colour counts that tree has one planted leaf, so the population would have to contain the solution's prefix at every depth. AMS works on full-board dynamics and selects on a trajectory's running maximum. It never needs a prefix of the solution.
- **Population annealing** resamples by Boltzmann weight while cooling, which is a temperature mechanism. That is the lever the stall says cannot make a rare exit common. AMS runs at fixed temperature and resamples by a path quantity, with copies resumed from first crossings.
- **Constructive SMC and beam search** build partial boards and belong to the saturated beam cell, which "stalls in the interior".
- **None of the three** gives an unbiased probability of the rare event per board. AMS does, so even a kill measures how rare the solving event is at 7x7, a number the 470-wall discussion lacks.

## Sources

- Cérou and Guyader 2007 (Crossref): https://api.crossref.org/works/10.1080/07362990601139628
- Bréhier et al. 2016, arXiv 1505.02674: https://arxiv.org/pdf/1505.02674
- Bréhier et al. 2016, Project Euclid: https://projecteuclid.org/download/pdfview_1/euclid.aoap/1481792593
- Aldous and Vazirani 1994 (Crossref): https://api.crossref.org/works/10.1109/SFCS.1994.365742
