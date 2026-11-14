# Coding theory: information-set decoding of a local-search incumbent

## Stall attacked

Two recorded stops that sit on either side of the same gap.

- Complete search at the critical colour count. `research/experiments/hard-ladder/README.md`: "counts at or just below the one-expected-solution point are the hardest, not the most informative." `research/program/AGENDA.md` (A22 result): "At the critical-colour rungs R6 and R7 G1 fails (0.80, 0.70) and `bb-mrv` is slower in wall clock (1.64x, 1.85x)", and from the same entry, "9x9 c=11 and 10x10 c=12 are censored for both solvers on every pair even at the extended 1e7 / 180 s cap". Stronger propagation buys almost nothing there: the search still has to guess its way down with no information about which value is right.
- Local search. `research/program/EXPLORED.md`: "Local search, ALNS, hyper-heuristics | saturated | 461 academic, 460 class here", and "Fake 480 and repair, repair seeded by exact prefix | mapped | dead ends". Local search reaches boards that are mostly right cheaply but cannot finish them.

The bridge is a measured fact in `research/experiments/sat-backdoors/README.md`: "Random subsets need T50 cells with nearly the same exponent (0.61 vs 0.59 in regime A)", where T50 is the subset size at which half of random subsets of correctly placed cells let propagation complete the whole board (T50/N = 0.30 at 6x6, 0.30 at 7x7, 0.25 at 8x8, 0.22 at 10x10). So complete search does not need to guess the whole board, only about a quarter of it, chosen at random, and correct. The mechanism attacks the first stall (complete search pays for guessing information it does not have) by taking those guesses from the second (an incumbent that local search has already made mostly right), instead of from branching.

## Field and mechanism

Coding theory, information-set decoding (ISD). To decode a noisy received word, pick a random information set I (positions that determine the whole codeword once known), assume the received word is error-free on I, re-encode from I alone, and accept if the result is a codeword consistent with the channel; otherwise draw a fresh random I. Per-trial success is the probability that I avoids every error, about f^|I| for agreement fraction f, so the method is efficient exactly when the information set is small relative to the length and the received word is mostly right. Ordered-statistics decoding adds the reliability step: draw I from the most reliable positions rather than uniformly.

Mapping to the puzzle:

- Received word: a full board from local search (parallel tempering or ALNS at a fixed budget), with some breaks.
- Information set: a random set I of t cells, t near the measured T50 for the size, whose placements (piece and rotation) are copied from the incumbent.
- Re-encoding: the ladder's propagating complete DFS (`bb-mrv`, A3 propagation with piece all-different) run on the remaining N - t cells with a small node cap. A wrong cell in I usually shows up as a fast contradiction; a stall is resolved by the capped DFS.
- Acceptance: any valid complete board, planted or not.
- Reliability (ordered statistics): a cell's reliability is how many of its four joints match in the incumbent; V2 draws I only from fully matched cells.

## Original paper

- E. Prange, "The use of information sets in decoding cyclic codes", IRE Transactions on Information Theory 8(5), 5-9, 1962. DOI 10.1109/TIT.1962.1057777.
- Reliability ordering (V2): M. P. C. Fossorier and S. Lin, "Soft-decision decoding of linear block codes based on ordered statistics", IEEE Transactions on Information Theory 41, 1379-1396, September 1995. DOI 10.1109/18.412683.

## Hypothesis

On critical-colour 6x6 and 7x7 ladder boards, local-search incumbents agree with a solution on a large enough share of cells, at their own positions, that a random t-subset of the incumbent with t near T50 is error-free often enough for propagation from it to finish a valid board. The end-to-end cost (incumbent plus trials) is then below complete search from an empty board, because the hard guesses are read from the incumbent instead of branched on. The prediction is quantitative: per-trial success should track the measured completion rate of correct t-subsets times the measured chance that a random t-subset of the incumbent is error-free.

## Nearest explored cell and missing mechanism

- Fake 480 and repair, repair seeded by exact prefix (mapped, dead ends); ALNS destroy-and-repair (saturated). These keep most of the board and re-solve a small region, usually a spatial window or a contiguous prefix. Missing mechanism: keep a small, sparse, random subset (about a quarter of the cells), re-solve everything else exactly, and redraw the subset on failure. The sparse random choice is what lets a mostly right board pass its correct cells into propagation without having to trust any contiguous region.
- A14 local branching (proposed): exact search in a Hamming ball of radius up to 8 around the incumbent. ISD lets up to N - t cells change (about 26 of 36 at 6x6), far outside any ball CP-SAT can certify.
- A3/A16 backdoors: A16 used backdoor membership to choose the first branching cell and died. ISD takes no branching decision from features; it takes values from an incumbent and relies only on A3's random-subset result.

## Cheapest falsifying test (6x6 or 7x7)

Ladder R6 (6x6 c=5) and R7 (7x7 c=6), test seeds 1..30, the committed instances.

Stage 1, diagnostic (no solver comparison yet):

1. Re-measure T50 and p_correct(t) on the ladder instances themselves: the completion rate of random t-subsets of the planted board under `bb-mrv` propagation, t on a grid around T50. A3 measured this on a different frame (2 border colours, not 5).
2. Produce incumbents with the repo's parallel tempering (as in `quality-diversity-basins`) at a fixed move budget, 3 run seeds per instance. Record breaks and positional agreement f with the planted solution, taking the best of the 4 board rotations (the A10 lesson).
3. For each incumbent and each t, run 1,000 trials with uniform I (V1) and 1,000 with reliability-restricted I (V2). Each trial gets propagation and then `bb-mrv` with a 100-node cap. Record the success rate p_isd(t).
4. Control C1, the spatial window: the same t, the same incumbent, I a random contiguous rectangle or row band (the ALNS or prefix shape). Control C2, the null incumbent: I drawn from a random full board with the same piece multiset. This should give 0 and checks that success is not an artefact of the capped DFS.

Stage 2, end to end, only if stage 1 passes: fixed wall clock per instance. ISD arm: PT for a share of the budget, then trials at the best t from stage 1 (chosen on calibration seeds 1001..1010, never on test seeds). Compared against `bb-mrv`, `scb-gacolor_ac3_random`, and PT continued for the full budget.

## Baseline and control

- Method baselines: `bb-mrv` and `scb-gacolor_ac3_random` from an empty board (complete search), and PT alone at equal wall clock (local search).
- Mechanism controls: C1, the spatial window of the same size from the same incumbent, which separates the information-set mechanism from ordinary destroy-and-repair; C2, the null incumbent; and the model prediction p_correct(t) times P(I error-free | f), which tests whether success comes from the stated mechanism.

## Kill criterion

- K1 (no signal): at both R6 and R7, the median over instances of the best p_isd(t) for V1 and V2 is below 1e-3, meaning more than 1,000 trials per hit, or the incumbents' median rotation-quotiented agreement f is below 0.5. This is the expected failure mode if local optima are planted blocks moved to other positions (A13's picture), because positional agreement is then low.
- K2 (mechanism): sparse random I does not beat the spatial window C1 on success rate by more than 2 standard errors (paired by incumbent) at both rungs. If it fails, whatever works is ordinary repair, already mapped.
- K3 (method, stage 2): on R7 at equal wall clock, the ISD arm solves fewer than max(`bb-mrv`, scb, PT-continued) + 4 of 30, or its median paired wall-clock ratio against the better complete solver is above 1 (an exact paired test; G1 rule from the ladder's PROTOCOL.md for the paired ratio).

## Budget

Stage 1: 60 instances x 3 incumbents x (2 variants + 2 controls) x about 8 values of t x 1,000 trials, each trial one propagation and at most 100 nodes. About 2 to 3 core-hours, plus PT incumbents at well under one core-hour. Stage 2: 60 instances x 4 arms x 30 s, about 2 core-hours. CPU only, no AWS.

## Closest prior work

- Within the project: repair seeded by exact prefix and the fake-480 repair (dead ends, `lab/.../repair-study`), ALNS (`build/local-search/local-search-alns`), A14 local branching, A3 backdoors.
- Outside it: large neighbourhood search (P. Shaw, CP 1998) with random destroy operators, and relaxation-induced neighbourhoods in MIP (RINS: E. Danna, E. Rothberg and C. Le Pape, Mathematical Programming 102, 2005), which fix the variables on which the incumbent and the LP relaxation agree and solve the rest.

## Why this is not a variant of it

LNS and RINS keep the large part of the incumbent and free a small part. The kept set comes from a spatial or structural rule (LNS) or from agreement with a relaxation (RINS), and nothing predicts how often a repair succeeds. ISD inverts the proportions: it keeps a small random part, sized from a measured quantity (T50, the size at which correct random subsets let propagation finish the board), and frees the rest. It also makes a falsifiable prediction from coding theory: success per trial is about f^t times the completion rate. This is the regime where the frame-dependent repairs stopped: when the incumbent is wrong in many scattered cells, every spatial window holds an error, while a sparse random subset of the same size can still miss all of them. Control C1 tests exactly this difference. If C1 does as well, the idea is a repair variant and dies under K2.
