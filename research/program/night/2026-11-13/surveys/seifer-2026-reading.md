# Reading: Seifer 2026, "A Pipeline for Solving Edge-Matching Puzzles and Their Implications for Protein Folding"

> **Status (2026-11-13):** not refereed. (`surveys/CITATIONS.REVIEW.md` item 5 audits the quantum survey's use of the same preprint, from its abstract, not this reading.)
> **Referee's rulings:** none, no referee report exists for this reading.
> **Read with:** `surveys/CITATIONS.REVIEW.md` (item 5); `surveys/quantum-unconventional.md`.

Shahar Seifer (Chemical and Biological Physics, Weizmann Institute of Science). bioRxiv, posted 24 May 2026, doi:10.64898/2026.05.23.727379, v1 (the only version). 9-page PDF plus a 3-page supplement, both read in full. The public GitHub repository and its instance files were also inspected (section 5).

## Verdict

- **What it establishes:** on a 64-tile (8x8) generated puzzle, D-Wave's hybrid BQM solver returned a complete solution when the whole 28-piece frame was given. On a planted 16x16 instance with a known answer, a human-steered pipeline (hybrid annealer runs, deduction, a transformer, then 64-thread classical backtracking for 1 to 4 days) recovered that known answer.
- **What it does not establish:** a solve of Eternity II, or a blind solve of anything at 256 tiles. The 16x16 instance is not the Eternity II tile set (only 128 of its 256 tiles are genuine E2 tiles, and it contains 5 duplicate tiles). The known answer was used inside the pipeline at least at step one (Fig. 2a was built by intersecting annealer output "with the complete solution"). The 16x16 annealer runs started from the correct full border. No experiment measures the quantum share of the hybrid solver.
- **One sentence for a literature review:** "Seifer (2026, bioRxiv preprint, not peer reviewed) reports a hybrid D-Wave plus classical pipeline that recovers the planted solution of a 16x16 edge-matching instance modified from Eternity II, using the known solution to filter intermediate states and starting the 16x16 annealer runs from the correct border; it does not solve Eternity II, and the unaided hybrid solver fully solved only a 64-tile puzzle, and only with its frame given."

## 1. The test puzzle of claim (c)

**Size.** 16x16, 256 positions. Sec. III intro: "a 16×16 board, referred to as the 'virtual' puzzle, is examined. This puzzle is comparable in complexity to the Eternity II puzzle and uses the same set of clues, except that its full solution is known in advance."

**How it was generated.** It was not randomly generated. Sec. III intro: "The layout of the virtual puzzle was generated from a partial Eternity II configuration, with mismatched pieces adjusted to produce a consistent final arrangement." In other words, the author took a real-tile E2 arrangement with mismatches and repainted edges until every edge matched. This plants a solution by construction.

I checked the published files (`code/Matlab/origin6.mat` for the real arrangement, `code/Matlab/origin_virtual.mat` for the virtual puzzle):
- `origin6.mat` is the genuine Eternity II tile set. It is isomorphic to the official piece list up to colour relabelling, with identical colour-frequency profiles. The arrangement matches 267 of 480 edges.
- `origin_virtual.mat` matches all 480 edges, and its outer edges are all grey.
- Only **128 of its 256 tiles are genuine E2 tiles**. The other 128 were repainted.
- Only 251 of its tiles are distinct up to rotation: **5 tiles are duplicates**. One duplicate is the official clue piece that the official list numbers 207, which appears both at its clue square and at a border-adjacent square.
- The five official clue pieces do sit at the official clue squares, so "uses the same set of clues" holds for the clue tiles.

**Colours.** 22 non-grey colours, the same count as E2: 5 border colours and 17 interior colours in the real set. In the virtual set the frequencies are uneven: interior colours appear 38 to 58 times, against 48 to 50 in E2. One border colour also turns up on an interior edge (26 occurrences instead of 24).

**Distinct, used once, rotation.** Each physical piece is constrained to be used exactly once (Sec. II.A, Eq. 7, "each physical piece appears only once across the entire board"). Rotation is allowed: "each position initially has 1,024 possible assignments (piece plus orientation)" (Sec. I). Because of the 5 duplicates, tiles are not all distinct.

**By what measure "comparable in complexity".** None is given. The phrase is asserted twice (abstract, Sec. III intro) with no metric behind it: no solution-count estimate, no backtracking-cost comparison, and no comparison on the same solver between this instance and the real one. The only complexity numbers are "combinatoric spans" (products of option counts) in Table 1. Those are measured on this instance alone, as subsets of a single ring.

**Critical colour density?** On colour count, yes: 22 colours with E2's 5 plus 17 split, so it is near the critical regime by that crude measure. Two caveats. First, the instance is a hand-repaired planted board, not a draw from a random ensemble at that density. Second, half the tiles were repainted with no reported control over the colour statistics, and duplicates were introduced. The expected number of solutions and the actual difficulty were never computed, so "comparable to Eternity II" is unsupported either way.

## 2. What "partial initialisation" means

The abstract says the problem "requires partial initialization". The text shows several layers of given information.

**(a) The five official clues.** Every 16x16 run starts from them (Fig. 2a: "Initial step with 5 clues exposed"). This matches the real puzzle.

**(b) Border pieces restricted to border squares.** Sec. I: "Eliminating options for edge pieces and incorporating initial clues results in roughly 100,000 logical variables." The supplement (S4) also says: "We allow also the correct corner pieces to be present at start." That sentence refers to the gradient-descent test.

**(c) The known solution used to filter step one.** Sec. III.B: "A major practical difficulty is determining which locations are missing the correct option, a task that can be solved only through repeated independent annealer runs. For the sake of evaluation, the logical conjunction of two option matrices was used, which was intersected with the complete solution. This produced the exposed pieces shown in Fig. 2a." So the 33 border pieces exposed at the pipeline's start come from intersecting annealer output with the answer. In the published file `AI_example/savestate_virtual_step1.mat` (the documented start, "step1 - is the starting configuration shown in Fig.2a"), 38 positions are fixed (33 border, 4 clues, centre), and all 38 are correct. The raw annealer outputs behind it are far weaker:
- QA7220 keeps the correct tile among its candidates at only 71 of 256 positions.
- QA7239 does so at only 62.
- Each leaves about 45 to 73 candidates per position and excludes the correct tile at the other 185 to 194 positions.

**(d) The correct full border for the 16x16 annealer runs.** Fig. 2b, Sec. III.B: "starting from a pre-exposed L1 configuration (file QA7281, with 76,422 spins and 14 min run time)". The soft-constraint run QA7050 (Fig. 2c) uses the same 76,422-spin setup.

In the published data, "L1" is the outer border ring of 60 squares. The prose instead calls L1 "the first ring of tiles adjacent to the outer frame", but the data contradicts that: `savestate_L1_virtual.mat` fixes all 60 border squares plus 12 squares in the next ring, the 4 clues and the centre (77 positions, all correct), and `savestateVirtual_3lines.mat` fixes 157 = 60 + 52 + 44 + 1. I am reading the paper's "L2" as the first interior ring.

The paper says this border was recovered by the AI step: "The resulting pieces are the green-shaded ones shown in Fig. 2b, selected from a handful of other possible choices." It does not say how the correct choice was picked from that handful. The AI pipeline was itself seeded from the solution-filtered step-one state (README: "d - Logical conjunction with the starting configuration").

**(e) Faults "corrected" before the final classical stage.** Sec. III.B: "Faults, locations missing the correct option, are indicated by orange crosses. Most faults can be removed through comparison to the hard-constraint solution. After correcting all such locations, the combinatorial span decreases dramatically (Table 1)." Fig. 2d: "Complete solution obtained by nucleation from step c, after faults removed."

A "fault" is defined against the known solution, and only "most" can be removed by comparing runs. "Correcting all such locations" therefore implies the answer was consulted for the rest. The paper does not say this explicitly, and the corrected state is not among the published files, so I cannot measure how much was restored.

**What the solver actually had to find.** QA7281 started from 77 known-correct positions with 179 open. It ended with 238 single-choice positions, of which 90 are correct: the 77 given plus 13 new, 12 of them in the first interior ring ("12 persistent pieces found in L2"). The other 148 placements are wrong. QA7050 (soft) ends with 106 positions that have no candidate at all, and only 128 positions that still hold the correct option (60 of them the given border).

In fraction terms: the 16x16 annealer was handed 30% of the board (77 of 256) and contributed about 13 further correct placements. The remainder was found by classical nucleation, starting from a state that had been fault-corrected against the answer.

## 3. What "solved" means

**Criterion.** The pipeline output is compared with the planted solution ("Comparison with the known complete solution showed...", Sec. III.B). No independent verifier is described. The final board (Fig. 2d) is the planted answer, which is fully matched by construction.

**Who solved it.** Classical code. Sec. III.B: "Finally, starting from the corrected soft-constraint configuration, a 64-thread nucleation procedure was run... One thread reached the full solution after 24 hours, and another after four days." Nucleation is described in Sec. II.B as "an exhaustive search to grow a consistent assembly of matching pieces. Deduction fills in pieces that must follow from earlier selections. If stalling occurs, the process backtracks". That is depth-first backtracking with propagation.

**Runs and success rate.** The final stage succeeded in 2 of 64 threads (1 day and 4 days). Upstream there were "repeated" annealer runs, but no success rate over independent pipeline repetitions is reported: the whole pipeline was carried through once, on one instance, with manual steps (the README documents keyboard-driven GUI operations: "w", "y", "d", "f", "h"). The published nucleation snapshots log progress against the answer (file names such as `known169`, the number of positions still holding the correct tile).

**Wall times reported for the annealer.**
- QA7220: 10 min.
- QA7239: 20 min, 132,195 spins.
- QA7281: 14 min, 76,422 spins.
- QA7050: 16.7 min.
- 8x8 runs: 4.7 s and 14 s.

The published 8x8 script shows `sampler.sample(bqm, time_limit=4.757812e+00)`, so the "4.7 seconds of computation" is the requested hybrid time limit, not a measured QPU time.

**Quantum share.** It is not measured. The solver is `LeapHybridSampler(solver='hybrid_binary_quadratic_model_version2p')`. The paper itself says: "The D-Wave hybrid solver partitions the QUBO into subproblems that are either fitted to hardware or attempted classically [9]" (Sec. I). No QPU access time, no ablation against the same hybrid workflow with the QPU disabled, and no comparison against a classical QUBO heuristic of matched budget are reported. The only classical baseline at QUBO level is Ocean's simulated quantum annealing on 8x8, which "repeatedly failed to find a solution, determining less than 50% of the pieces" (Sec. III.A). The author concedes: "the hybrid quantum annealer does not outperform classical solvers in isolation" (Sec. III.B).

The claimed advantage is a reduction in "combinatoric span" from about 1.2x10^123 to 6.9x10^101 over the first interior rings with the border given (Table 1). That figure is evaluated after clearing wrong selections using the answer ("To evaluate the combinatorial reduction, I cleared selections in faulty locations based on prior knowledge", said of the classical baseline).

**8x8 result (claim a).** Sec. III.A: a complete solution "beginning from 28 pre-exposed frame pieces" (the entire 8x8 frame), so the solver placed the 36 interior tiles. From 4 corners only, the result was partial: "83 out of 84 edges were correctly matched", with faults "due to repeated use of the same piece". Both runs were single runs on one generated 8x8 instance ("generated by randomly assembling edge-matched pieces"). Its colour count is not stated.

## 4. Claims about the real Eternity II

The paper does not claim to have solved Eternity II. It frames the 16x16 work as a path toward it (Sec. III.B: "a pipeline capable of achieving a complete solution to the 16×16 Eternity II puzzle should include the following steps", a four-step plan). It states the real solution "has not been revealed" (Sec. I).

The repository does contain runs on the real tiles: `Results/EternityIIPuzzle/` holds D-Wave savestates (`QA7051`, `QA7227` ... `QA7284`, all `_real16x16`), nucleation snapshots and weighted-swap outputs. The paper reports no result from them, and no score or board from them appears in the text. The nucleation file names on the real puzzle (for example `goingon_real25_th3_167_170`) appear to encode partial depths in the 150 to 170 range, which is not near a solution. The paper says nothing about them.

Two side claims in the paper are unsupported:
- "For the scale of the Eternity II puzzle no classical method competes effectively with the hybrid quantum annealer as a standalone strategy" (Sec. I). No experiment backs it.
- "With 23 distinct edge types": E2 has 22 colours plus grey.

## 5. Code, instances, solutions

Published at https://github.com/Pr4Et/EdgeMatchingPuzzleSolver under GPL-3.0. Commits run from 31 Dec 2025 to 1 Jul 2026. The repository holds:
- MATLAB sources (`e2solverz10.m`, `miniSolver10.m`, `Dwave_Hamiltonian_test_prepare.m`), compiled Windows executables, the Python transformer with trained weights, and a TSPP demo.
- The virtual instance (`origin_virtual.mat`), the real tile arrangement (`origin6.mat`), its planted solution (`Initiallt_Exposed/Complete/savestate_full_virtual.mat`), the 5-clue, border and 3-ring starting states, the D-Wave output option matrices for every named run, the AI_example chain, nucleation snapshots, and the 8x8 instances with their auto-generated D-Wave Python scripts and answers.

What is missing for verification:
- the fault-corrected state that seeded the final 64-thread nucleation;
- a log or final state of the two successful threads;
- any QPU timing data.

The instance is checkable: I confirmed its 480/480 planted solution and its tile statistics from these files. The "solved" claim reduces to recovering that planted board, and the fully blind version (no oracle filtering or correction) is not reproducible from what is published because it was never run.

## 6. Status

Not peer reviewed. bioRxiv v1 only, posted 24 May 2026. The bioRxiv API reports category "biophysics", type "new results", license CC BY-NC-ND 4.0. bioRxiv's publication link API returns no published version ("no articles found for published version"), and the page shows no "Now published in" notice. The PDF header reads: "this preprint (which was not certified by peer review)".

The venue is a biology preprint server, chosen because of the protein-folding framing in Sec. IV. The optimisation and quantum-computing content has had no review from either community.

## Sources

- Full text and PDF: https://www.biorxiv.org/content/10.64898/2026.05.23.727379v1.full and `.full.pdf`
- Supplement: https://www.biorxiv.org/content/biorxiv/early/2026/05/24/2026.05.23.727379/DC1/embed/media-1.pdf
- bioRxiv API: https://api.biorxiv.org/details/biorxiv/10.64898/2026.05.23.727379 and https://api.biorxiv.org/pubs/biorxiv/10.64898/2026.05.23.727379
- Sciety listing: https://sciety.org/articles/activity/10.64898/2026.05.23.727379
- Code and data: https://github.com/Pr4Et/EdgeMatchingPuzzleSolver (cloned at commit 030e6a2)
- Tile-set comparison against the official piece list in `engine/data/official_eternity2.csv` (colour-relabelling isomorphism search; scripts kept in the session scratchpad only).
