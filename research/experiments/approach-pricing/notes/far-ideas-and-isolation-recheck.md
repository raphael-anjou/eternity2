# Review of the three far-ideas memos and of "Are solutions isolated?"

Second analysis report, 2026-11-14. Memos reviewed, none edited: `far-ideas-geometry.md`,
`far-ideas-other-fields.md`, `far-ideas-different-question.md`, `geometry-isolation.md`.
Read against `THESIS_PLAN.md` (section 9), `ROUTES.md` and `the log.md`. Scripts and raw rows
of this review: `notes/review-far-ideas/`. One core, `nice -n 10`, no run over 10
minutes, about 35 core-minutes in all, nothing remote. Every board in the reruns is a
generated toy and says nothing about Eternity II by itself.

Grades: **stands**, **stands with a narrower scope**, **wrong**, **unproven**. In the far-ideas
tables a dismissal is also marked **measured**, **argued** or **asserted**.

## 0. Rulings

### 0.1 `geometry-isolation.md`

| # | Claim | Ruling |
|---|---|---|
| I1 | Annealing solved 580 of 1,000 at 5x5, 0 of 1,000 at 6x6, 0 of 400 at 7x7 | Counts **stand** (recounted from the raw rows). As a statement about annealing, **stands with a narrower scope**: one schedule, 2e6 steps (about 0.2 CPU-second) per run, no restart. The same code solves 6x6 boards as soon as either is changed (section 2.1). |
| I2 | "Funnelled fails outright"; "the funnel is visible only at 5x5" | **Wrong as worded, stands with a narrower scope.** The success rate per short run drops by about two orders from 5x5 to 6x6 (58 percent to under 1 percent). It is not zero: 3 of 500 at the memo's budget with a narrower schedule; 2 of 25 (memo's schedule) and 7 of 25 (narrower) at 100 times the budget; 2 of 3 at 1,000 times. |
| I3 | The size trend is read at "the same 2e6 steps at every size" | **Not an equal-effort comparison.** Steps per node of the exhaustive tree: 35 to 62 at 5x5, 0.5 to 1 at 6x6, 0.04 at 7x7. The memo concedes this for 7x7 only. The 7x7 annealing rows are **unproven** as evidence of anything but an exhausted budget (section 2.2). |
| I4 | Trap check: 405 of 419 annealed boards reach a solution inside {<= 4 mismatches} at 5x5, 22 of 734 at 6x6 | Counts **stand**. The comparison **stands with a narrower scope**: 431 of the 734 boards at 6x6 end at exactly 4 mismatches, the ceiling of the level set, and none of those reaches a solution at either size (0 of 431 at 6x6, 0 of 3 at 5x5). At equal mismatch count: 349 of 358 against 11 of 66 at 2 mismatches, 56 of 58 against 11 of 237 at 3. The trend is real; "3 percent" is diluted by the ceiling boards (section 2.3). |
| I5 | No board has exactly one mismatch (derived) | **Stands.** The parity argument is correct and the exact counts agree. |
| I6 | Component sizes, barriers, correlations, "21 of 38", the reading "in between" by the letter of the plan | **Stand** where compared with `summary.txt` (21 of 38, the trap totals, the anneal histograms). Correlations and component sizes not recomputed. |
| I7 | At 6x6 unrelated solutions are separated by a barrier of 4 or more, "exact on three boards"; first answer to the A10 two-solution measurement | **Stands with a narrower scope**: M1 moves, level sets up to 4, five boards with 3 to 14 solutions. It is "more than 4" on seeds 1, 3 and 4, exactly 4 on seed 0, unsettled on seed 2. Above 4 only a loose upper bound (7 to 10) exists, so the barrier height itself is not measured. |
| I8 | "A walk that lands at low mismatch count lands, with probability tending to 1, in a pocket that is not a solution's" (derived) | **Unproven.** The count of near-solutions per solution is not measured at 6x6 or 7x7 (the memo says so), and the limit is an extrapolation from two sizes. |
| I9 | The larger move sets do not open the landscape | **Stands with a narrower scope**: two 6x6 boards, level sets up to 4. |
| I10 | 16x16 extrapolation (pocket of a few hundred to a few thousand boards; fitness silent below about 10 mismatches) | Marked guessed by the memo, and it is. Not ruled. |

### 0.2 `far-ideas-geometry.md`

| # | Claim | Ruling |
|---|---|---|
| G1 | Probe A: three geometric scores average 0.50, 0.46, 0.53 at the peak; "no signal" | Numbers **stand** (match `summary.txt`). Verdict **stands with a narrower scope**: five boards, two of them outside the memo's own design, three with 4 to 14 positives, no label-shuffle control, no 7x7. |
| G2 | Probe B: 0 of 30 runs solved; the flow "is a local search in disguise" | Count **stands**. "Dead as a solver" **stands with a narrower scope**: one schedule, one rounding, four runs per board on five of six boards. |
| G3 | Probe B: about 2 sd over search-sampled near-solutions, "at most a D4-sized tilt, about 1 bit" | **Unproven**, as the memo says. "No control could be generated at 6x6" was a limit of that probe's generator: the isolation run holds 66 boards with 2 mismatches at 6x6, and `geometry-sdp-lift.md` later ran the flow against annealed controls at 6x6 (z at most 1.26). |
| G4 | Idea 2 and idea B: the lift has about 2^34.5 entries on the 16x16, "beyond any solver" | Arithmetic **stands** (156,816 placements). "Beyond any solver" is **wrong** for matrix-free low-rank forms, which ran tonight. The price in bits on the 16x16 is not derived; the odds are guessed. |
| G5 | Idea B's prior: gap 6a says the second-order constraints carry nothing beyond propagation | **Overtaken.** `geometry-sdp-lift.md` (not independently checked) reports the lift tight on two 5x5 boards, removing 268 and 325 placements that arc consistency keeps. The memo's kill line was not met at 5x5; cost, not signal, is what remains against it. |
| G6 | Ideas 14 (here and in the other-fields memo): low-order Fourier coefficients are "the LP family, blind until depth 192" | Dismissal **asserted**, citation **wrong**: depth 192 belongs to gap 5's first-order conservation program, not to a second-order relaxation. The later lift memo measured the second-order LP weak on 5x5 toys (z 0.17 to 1.02), which supports the dismissal at toy scale only. |
| G7 | Idea 15: for the analog solvers "the analog time to solve grows exponentially (their own papers)" | **Wrong as worded.** The paper's abstract (opened) says the system finds solutions in polynomial continuous time at the expense of exponential fluctuations of its energy function. The dismissal is **asserted**. |
| G8 | Section 4: ideas 1, 3, 4, 7, 9, 10, 13, 14, 16, 18 "fail for the same reason" (a point computed from local matching) | **Stands with a narrower scope.** Two probes carry ten ideas. Idea 14 at order 2 is a second-order each-tile-once object, the class the same section says may escape. |
| G9 | Ideas 7, 8, 9, 11, 16, 17 are named duplicates of ROUTES rows; idea C is the A10 row's own next step | **Stands.** None is presented as new. |
| G10 | Odds (0.3, 0.5, 0.3 percent) | Guessed, labelled as such. Not ruled. |

### 0.3 `far-ideas-other-fields.md`

| # | Claim | Ruling |
|---|---|---|
| F1 | "Of 32 ideas, 29 map onto a route already priced out (named per row) or have a crisp reason to be blind" | **Stands with a narrower scope.** Seven of the 29 lean on rows that ROUTES itself grades weak or flags (table in section 3.1). For those the dismissal is inherited, not established. |
| F2 | Idea A (colour merging): about 2^67.4 times more coarse solutions (derived) | **Stands** (420 x log2(19/17) = 67.4). |
| F3 | Idea A: "dead on cost (measured, toy)" | Numbers **stand** (ratio 1.6 to 99.7, median 6.9; rank 0.96 and percentile 0.28 on the 16 boards). Scope narrower: the cost measured is the enumeration of every coarse solution, while the idea samples a few. The memo's own kill line did not fire (mean rank 0.68 over the 24 boards against a line at 0.65); the verdict was reached on other grounds, which the memo states. |
| F4 | Idea A: "dead on distance (derived)" | **Unproven.** It rests on path 4's count, which ROUTES grades argued, not independently checked, with a flag. The stronger argument is the conservation statement of the different-question memo (Q1). |
| F5 | Idea B (print side channel): with rotations known the tree is "subcritical from the first interior row" | **Stands with a narrower scope.** Row 0 keeps its multiplicity (about 46 bits of edge-piece choices by first moment, my arithmetic) before the interior rows shrink it, so "worth everything" needs another order or about 10^14 nodes. The toy figure (17.2 to 19.8 bits on 36 cells, lower bounds on 8 of 10 boards) matches its output file. The idea is not new: ROUTES has the row and `how-the-puzzle-was-made.md` already asks for the photograph. It was closed on a recorded decision, not by the memo's kill line. |
| F6 | Idea C: "0 of about 35,000 wrong prefixes refuted" | **Stands with a narrower scope.** The prefixes are random valid prefixes other than the planted one; none is checked to be dead, and this board family has many solutions (8 to more than 732 with a clue in the same memo's rotation probe). The number of dead prefixes tested is unknown. |
| F7 | Idea C: "dead. Nonabelian finite quotients carry no information before the last handful of cells" | **Stands with a narrower scope**: S_6 with random involutions, by the covering control (full coset from 5 tiles on). For all finite quotients it is **unproven**: the covering theorem is unverified by the author and by me, it concerns simple groups, and solvable or large quotients are not addressed. |
| F8 | Item 8: about 2^23.7 tile quadruples close a 2x2 loop against 225 true loops | **Wrong figure, conclusion stands.** The estimate uses 256 pieces with 17 colours. The inner pieces give 4,059,952 blocks (2^21.95, measured in the different-question memo) against 169 interior loops. |
| F9 | Items 13 and 19: belief propagation and mean-field annealing are "priced out by theory at this density" | **Unproven.** The ROUTES row is "claim without committed data" (one 4x4, three settings) and THESIS_PLAN keeps RQ7 open at core-minutes. |
| F10 | Items 5, 20 (and item 15 of the different-question memo): local search is "killed", boards sit "246 or more cells from any solution" | **Stands with a narrower scope**: an expectation from an not independently checked model (ROUTES: "not a proof"). |
| F11 | Item 27 (cohomological consistency at k = 2): "the one row here with no measurement or theorem behind its kill" | **Stands**, and partly covered: gap 6a row 15 ran the field version of the second-order affine system (degree-2 linearisation determines 0 of 15 boards at 4x4 and 5x5). Refutation of wrong prefixes by it is untested. |
| F12 | Citations opened by the memo | Walshaw and Krzakala-Zdeborova say what the memo says. The Shalev preprint is about interleaved products; its abstract supports "products of classes become near uniform" only loosely. Pak's survey: the file downloads, I could not read it. Wikipedia: not opened by me. |

### 0.4 `far-ideas-different-question.md`

| # | Claim | Ruling |
|---|---|---|
| Q1 | Verdict: a relaxation that keeps the true board multiplies the solutions by about as much as it cheapens each one, "computed rather than assumed" | The statement **stands with a narrower scope** (first-moment model, a solver with no statistic). "Computed" is **wrong**: nodes per hole-solution is T / W and the count is W, so the product is T by construction. What carries the claim is the assumption that a found hole-solution is no nearer the planted board than chance, which the memo did not measure (its own one core-hour test was declined). |
| Q2 | Item 1 and table row 6: "the cheapest single bit costs about 2^124" | **Wrong as labelled.** 2^124 is the row-scan cost of exhausting the tree under the best single pin. It is one method's cost, not a floor, and a NO to "tile t at cell c in rotation r" carries about 0.002 bit, not one. "About 56 such answers" confuses bits of tree with answers. The source (D3 Part B) is measured, not independently assessed, on painted boards. |
| Q3 | Item 2: "every invariant computable from the piece set carries zero bits about the arrangement (cited, path 2 theorem)" | **Wrong as stated.** The second analysis of `how-the-puzzle-was-made.md` reworded "zero bits" to "at most 42.7 bits with no mechanism", and THESIS_PLAN O2 was rewritten for this reason. What stands: no statistic found is worth more than about one bit. |
| Q4 | Item 4 and section 2.2: 4,059,952 valid ordered 2x2 blocks; block scan 2.49 bits worse than row scan | Count **stands** (script read and rerun: 4,059,952; the first moment for 196 pieces gives 2^22.1). The order differences come from the memo's own uniform model, differences only: **stands with a narrower scope**. The idea has a ROUTES row (pattern databases, macro-tiles) that the memo does not name. |
| Q5 | Items 5, 6, 7, 12 and section 2.1 arithmetic (hole factors, nodes per solution with fewer clues, (17/9)^420) | **Stand** (rechecked by hand: 4.32 and 10.98 digits for 1x1 and 2x2 holes, 42.97 and 42.98, 10^116). |
| Q6 | Item 11: the last 60 cells have about 10^25.6 fillings "(W at free depth 60, measured)" | **Stands with a narrower scope**: a measured width at depth 60 applied to the tail by an assumed symmetry. Derived, not measured. |
| Q7 | Item 13: a bank of proven-dead frames or blocks cannot help, "one exhaustive search never revisits a subtree" | **Wrong argument.** A dead corner block refutes every frame that contains it; that is reuse without revisiting. This is the no-good row (undecided, flagged) and gap 1 (unproven in general). The verdict must come from there. |
| Q8 | Item 14: 886 G nodes per second gives 2^68.6 in ten years | **Wrong by 0.7 bit**: 2^39.7 x 2^28.2 = 2^67.9. The shortfall (66 bits) is unchanged. ROUTES marks the node rate as unreconciled. |
| Q9 | Section 2.3: outside information is "the one family the lens actually opens" | **Not new.** THESIS_PLAN section 3 (closing paragraph) and section 8 already name it; channel 3 is the photograph `how-the-puzzle-was-made.md` asks for. The value figures are cited correctly except one: 10^13.6 is the mean interior cost under a frame that passes the corners, not under the true frame. Odds guessed; sources hearsay grade, as the memo says. |
| Q10 | "Twenty ideas" | The list has 21. |

### 0.5 Summaries in `the log.md` and `ROUTES.md` that say more than the memos support

| # | Where | Sentence | Ruling |
|---|---|---|---|
| S1 | the log 18:52, heading | "the funnel is gone by 6x6" | **Too strong** (I1, I2). The memo's budget does not support "gone". |
| S2 | the log 16:07 | "one yes-or-no answer about the solution costs at least 2^124 nodes" | **Wrong.** The memo says "about", for one method, and its own label is wrong (Q2). There is no lower bound here. |
| S3 | the log 21:50 | "Annealing on the same boards: 58 percent", beside the low-rank form's 52 of 66 | **Unequal effort.** The annealing runs had about 0.2 CPU-second each, the low-rank runs a 480-second limit. |
| S4 | ROUTES, "Not run" | "the isolation probe shows the traps appear between 5x5 and 6x6" | **Stands with a narrower scope**: at level sets up to 4 mismatches. It does not show that a local search with reweighting or a longer budget fails at 6x6; plain annealing solves there. |
| S5 | the log 16:28 | "refuted 0 of about 35,000 wrong prefixes on toys, as theory predicts" | Narrower: prefixes not known to be wrong (F6); theory unverified (F7). |
| S6 | the log 16:28 | colour merging "under 0.01 percent" | The memo gives two figures (under 0.1 percent in section 2.A, under 0.01 in section 4). Both guessed. |
| S7 | the log 16:19, 16:28 (counts), 16:15 | probe summaries, "29 of 32", closure of the three outside bets | Match the memos. The 16:15 closure is a recorded decision; no kill line of either memo was executed. |

## 1. Corrected statements the program may use

1. **Annealing on toys.** With the memo's annealer (single-tile rotations and same-class swaps,
   geometric cooling from 2.0 to 0.05, 2e6 steps, about 0.2 CPU-second per run, no restart),
   580 of 1,000 runs solve at 5x5 and 0 of 1,000 at 6x6. With the same code at 6x6: 3 of 500 runs
   solve at the same budget on a schedule from 0.8 to 0.2; on the memo's schedule 1 of 150 at
   10 times the budget, 2 of 25 at 100 times and 2 of 3 at 1,000 times; 7 of 25 at 100 times on
   the narrower schedule.
   Plain annealing reaches 6x6 solutions; its success per short run is about two orders lower
   than at 5x5.
2. **7x7.** At 100 times the memo's budget 0 of 4 runs solve and the end states are at 4 to 7
   mismatches; the equal-ratio budget (about 2e9 to 3e9 steps) was not run. With the memo's budget the 7x7 runs stop at about 8 mismatches, so the
   memo's 7x7 annealing and trap rows measure the budget.
3. **Level sets.** Among boards an annealer ends on at 6x6, those at 2 mismatches connect to a
   solution inside {<= 4 mismatches} in 11 of 66 cases (17 percent, up to 26 percent if the 6
   capped searches all connect), those at 3 in 11 of 237, against 97 percent at 5x5 for both.
   Boards at exactly 4 never connect, at either size, because they sit on the ceiling of the set.
4. **Against the relaxation heuristic.** No like-for-like comparison exists yet. The memo gave
   each annealing run about 0.2 CPU-second; the relaxation solves took 2 to 25 CPU-minutes. In
   this review, restart annealing found a 6x6 solution about every 30 to 70 seconds of one core on
   the narrower schedule and about every 200 to 300 on the memo's, pooled over the five boards
   (2 to 7 solutions per arm, another machine and load). Row-scan search enumerates
   every solution of these boards in 0.6 second.
5. **Far ideas, overall.** The three memos name no mechanism their authors rate above 1
   percent, and none carries a derived price of 10 bits on the 16x16. Several dismissals are
   inherited from ROUTES rows that are themselves not independently checked (path 4, belief propagation, the
   LP row, the difference map at its cap). "Priced out" should be said only of the rows that
   are.
6. **Decision bits.** "Exhausting the row-scan tree under the single best extra pin costs about
   2^124 nodes on painted boards (D3 Part B, not independently assessed)." Not a floor, and not the price of a bit.
7. **Relaxations keep the price.** "In the first-moment model, and for a solver with no
   statistic that tells the planted board from the rest, holes, merged colours, fewer clues and
   endgame lists leave the price at or above the row-scan tree." The first-moment independence
   between a found relaxed solution and the planted board is assumed, not measured.
8. **Nonabelian invariants.** "In S_6 with random involutions the class-product test cannot
   bite with five or more tiles left." Not "finite quotients are dead".
9. **Semidefinite lift.** "About 2^34.5 entries in the dense form on the 16x16." Not "beyond
   any solver".

## 2. Evidence: `geometry-isolation.md`

### 2.1 Is the annealer a fair one?

Code read: `geometry-isolation/iso.c`, `cmd_anneal`. Moves: with probability 0.3 on an
inner cell, a rotation; otherwise a swap of two tiles of the same class with fresh random
rotations for inner tiles. Acceptance: Metropolis on the mismatch count. Temperature:
geometric from 2.0 to 0.05 over the run. No restart, no reheating, the final board is reported
(not the best: 219 of the 1,000 runs at 6x6 ended above their best). The move set is the usual
one; a swap that also redraws both rotations is accepted less often than a swap that keeps the
better rotation, which is a mild handicap.

The schedule is the weak point. In the memo's own 5x5 rows the solving step has a median of
887,000 of 2,000,000 and a 10th to 90th percentile range of 763,000 to 1,040,000, which is
the temperature band 0.49 to 0.29. The run spends 14 percent of its steps in that band
(ln(0.49 / 0.29) / ln 40), about 280,000 steps, and is frozen after it (at 0.05 an uphill
move of one mismatch is accepted with probability 2e-9).

Reruns with the memo's code compiled unchanged (`iso_ref`), new seeds, the five 6x6 boards
(cf 4, ci 5). The narrower schedule (0.8 to 0.2) was fixed before any run and is not tuned.

| Arm | Schedule | Steps per run | Runs | Solved | Final mismatches (mode) |
|---|---|---|---|---|---|
| memo | 2.0 to 0.05 | 2e6 | 1,000 | 0 | 4 |
| R4 | 0.8 to 0.2 | 2e6 | 500 | 3 | 4 |
| R1 | 2.0 to 0.05 | 2e7 | 150 | 1 | 3 |
| R2 | 2.0 to 0.05 | 2e8 | 25 | 2 | 2 |
| R5 | 0.8 to 0.2 | 2e8 | 25 | 7 | 2 and 3 (8 each) |
| R3 | 2.0 to 0.05 | 2e9 | 3 (seeds 1, 3, 4) | 2 | 0 |

Per board: `review-far-ideas/raw/tally.txt`. Readings:

- "0 of 1,000" is specific to the schedule and the budget. It is a small number, not a zero:
  the 95 percent upper bound on the memo's own rate is 0.3 percent per run, and R4 measures
  0.6 percent (3 of 500; three of the five boards) at the same cost.
- On the memo's own schedule the success rate rises with the budget: 0 of 1,000, 1 of 150,
  2 of 25, 2 of 3 at 2e6, 2e7, 2e8 and 2e9 steps. The two 2e9 solves came near step 1e9, at
  temperatures of 0.28 and 0.34, inside the band where the 5x5 runs solved. The narrower
  schedule, which spends its whole run in that band, solves 7 of 25 at 2e8 steps. That is the
  ordinary behaviour of annealing on a rugged landscape with a reachable target, not the
  signature of a target cut off from the walk.
- Would a better-tuned annealer change the sentence? Yes, it already does, without tuning.
  Whether a tuned one reaches the 5x5 rate at 6x6 is not measured here.

### 2.2 Is the comparison across sizes at equal effort?

No. The budget is 2e6 steps at every size while the problem grows:

| Size | Exhaustive row-scan nodes (memo, `enum`) | Anneal steps per node | Free placements space (log2, my arithmetic) |
|---|---|---|---|
| 5x5 | 3.2e4 to 5.8e4 | 35 to 62 | 68 |
| 6x6 | 1.9e6 to 3.7e6 | 0.5 to 1 | 123 |
| 7x7 | about 5.4e7 (2^25.7) | 0.04 | 197 |

An equal ratio to the exhaustive tree needs about 1e8 to 2e8 steps at 6x6 (arms R2 and R5)
and about 2e9 to 3e9 at 7x7. The memo says the fixed budget "handicaps 7x7"; the same holds
at 6x6 by a factor of about 60. At equal ratio the 6x6 success is 8 percent (memo schedule)
and 28 percent (narrower schedule) against 58 percent at 5x5, on 25 runs each. The trend with
size survives; its size does not.

7x7: at 2e8 steps (100 times the memo's budget, still about 15 times under the equal ratio)
0 of 4 runs solve on the two boards and the end states are at 4, 5, 6 and 7 mismatches,
against a median of 8 in the memo. Four runs decide nothing; the memo's 400 runs at 2e6
steps decide nothing about 7x7 either.

### 2.3 The trap check

Recount from the raw rows (`trap_by_headroom.py`, output `raw/trap_by_headroom.txt`):

| Size | Endpoint at 2 mismatches | at 3 | at 4 |
|---|---|---|---|
| 5x5 | 349 of 358 reach a solution (8 capped) | 56 of 58 (1 capped) | 0 of 3 |
| 6x6 | 11 of 66 (6 capped) | 11 of 237 (4 capped) | 0 of 431 |
| 7x7 | none | none | 0 of 3 |

The search is confined to boards with at most 4 mismatches. A board with exactly 4 can only
move to boards with 4 or fewer, and a frozen annealer endpoint is a local minimum by
construction, so those boards have almost no room (median component 4 to 12 boards). They are
59 percent of the 6x6 sample and under 1 percent of the 5x5 sample. The fair figure is the one
at equal mismatch count: 97 percent against 17 percent (2 mismatches) and 5 percent (3
mismatches). The memo's conclusion (the level set {<= 4} is connected at 5x5 and fragmented at
6x6) stands. Its bearing on solvers is narrower than the memo says: annealing at a working
temperature moves well above 4 mismatches, so a fragmented {<= 4} set does not stop it, as
section 2.1 shows.

### 2.4 Like for like with the relaxation heuristic

Later in the night a relaxation heuristic solved 6x6 toys after 15,000 to 140,000 gradient
evaluations, about 2 to 25 CPU-minutes per solved start (3 of 14 finished starts under a
1,800-second cap on one machine, one start at 1,345 seconds on another; program's tally,
not independently checked). The boards are the same family (`growth-exponent/boards/b6_s3` has P = 1,290 and
10 solutions, as 6x6 seed 3 here; I did not compare the tile lists).

Was the memo's budget per run smaller than that? Yes, by a factor of 600 to 7,500: 0.2
CPU-second against 120 to 1,500. All 1,000 annealing runs at 6x6 together cost about 200
CPU-seconds, about one cheap relaxation solve. At about 10 ms per gradient evaluation one
evaluation is worth about 1e5 annealing steps, so 15,000 to 140,000 evaluations are 1.5e9 to
1.4e10 steps.

A like-for-like comparison needs:

1. the same boards and the same pin;
2. the same success event, a verified solution, with every restart and every failed start
   charged;
3. the same clock on the same machine (CPU-seconds of one thread), or a common unit with the
   conversion measured;
4. a cap per start equal for both (1,800 CPU-seconds was the relaxation's);
5. parameters of both fixed on boards outside the test set;
6. the report as expected CPU-seconds per verified solution with an interval, plus the share
   of starts solved under the cap;
7. the exhaustive search next to both, since it finishes these boards in under a second.

What this review can say, short of that: pooled over the five boards and counting every run,
restart annealing found a 6x6 solution about every 32 seconds of one core in arm R4 (3
solutions), 69 in R5 (7), 208 in R3 (2), 307 in R1 (1) and 311 in R2 (2), wall time of a
single process on a loaded machine. The relaxation's rows give
roughly 7,000 to 8,000 CPU-seconds per solution with the capped starts charged (11 capped at 1,800, three
solved; another machine). On these numbers annealing is not behind the relaxation at 6x6.
The counts are small and the machines differ, so this is a reason to run the comparison, not a
result.

Files: `run_anneal_budget.sh` (the driver, arms in the order run), `iso_ref` (the memo's
`iso.c` compiled unchanged), `tally.py` and `raw/tally.txt`, `trap_by_headroom.py` and
`raw/trap_by_headroom.txt`, `raw/log.txt` (exit codes and wall seconds), one JSON-lines file
per arm and board in `raw/`.

## 3. Evidence: the far-ideas memos

### 3.1 Dismissals: measured, argued or asserted

Only the items where the grade matters are listed. "Named row" means the memo cites its ROUTES
row and the row is measured or independently re-checked; those are not repeated here.

| Memo, item | Idea | Dismissal | Note |
|---|---|---|---|
| geometry 1, other fields 19 | soft assignment, Hopfield-Tank, graduated assignment | measured (probe B, 0 of 30) in one memo, asserted in the other ("known to break down well below 256", no source) | Toy scale, one schedule. |
| geometry 2 | semidefinite lift | argued on cost; the signal part was overtaken (G5) | |
| geometry 3 | geometric tilt on the level | measured, five boards | The 7x7 rung and the shuffle control of its own design were not run. |
| geometry 5 | level-set topology | argued, then measured by the isolation memo | Section 2. |
| geometry 6, 10, 12, 18 | Morse counts, tropical, Whitney, hyperbolic picture | argued | Each reduces to enumeration or to the measured level widths; I find no hole. |
| geometry 13 | springs and rigidity | asserted ("exponentially many local minima") | Same family as idea 1. |
| geometry 14, other fields 14 | Fourier on the symmetric group | asserted, wrong citation (G6) | Toy support from the lift memo. |
| geometry 15 | analog dynamics with memory | asserted, source misquoted (G7) | See test T2. |
| other fields 3 | phase retrieval, RAAR, RRR | inherited from the difference-map row | ROUTES: killed at the cap, flagged, reopening test named. That other projection schemes behave like the difference map is asserted. |
| other fields 5, 20; question 15 | fragment assembly, potential game, consensus of records | inherited from path 4 | Argued, not independently checked. |
| other fields 6 | row sets before row orders | guessed | First moment agrees: a random interior row set has about 2^4 valid orders under horizontal matching (14! x 4^14 x 2 / 17^15), so sets and sequences differ by a few bits per row. |
| other fields 13 | iterative decoding, belief propagation | asserted ("priced out by theory") | F9. RQ7 is the existing test. |
| other fields 18 | auctions | inherited from path 1 (not independently checked) | "Auction prices plus score-band counting" is still in ROUTES' untried list. |
| other fields 26, C | nonabelian tiling groups | measured for S_6 with a control; theorem unverified | F6, F7. Not a duplicate: it is the reopening condition of the gap 5 row. |
| other fields 27 | cohomological consistency | none (the memo says so) | F11. |
| other fields 28, A; question 7 | colour merging | derived (conservation) plus a toy in the wrong regime | The two memos price it at one pair (2^67) and at 17 to 9 colours (10^116); both arithmetic checks pass. |
| question 1 | decision bits | derived, mislabelled (Q2) | |
| question 2 | cheap certificate | wrong statement (Q3) | |
| question 3 | counting mod 2 | named as gap 6a's open hole | Correctly left unpriced. |
| question 13 | bank of dead frames | wrong argument (Q7) | |
| question 4, 5, 6, 8 to 12, 16, 20, 21 | super-tiles, holes, fewer clues, other shapes, duals, backwards, hole measure, plantedness, analog parallelism, frame yes or no | derived or named row | Arithmetic rechecked where shown in section 0. |

### 3.2 Ideas presented without their ROUTES row

- Different-question item 4 (2x2 super-tiles): row "Pattern databases, macro-tiles, local
  cluster enumeration" (killed per EXPLORED, claim without committed data; "macro-tiles are a
  real constant-factor win"). The memo's count and null are new material for that row.
- Different-question section 2.3 and item 19 (outside information): THESIS_PLAN sections 3
  and 8; ROUTES row "Traces of the maker's program". Channel 3 is also other-fields idea B.
  Counted once, the three memos hold one outside-information idea, not two.
- Different-question item 13 (public bank): row "No-good learning and shared failure tables".
- Different-question item 20 and other-fields item 2 are the same idea (molecular
  parallelism) with the same arithmetic.
- Other-fields item 18 (auctions): present in "Looks untried" (IDEAS rank 13); the memo
  dismisses it without saying the route has no run.

### 3.3 Ideas kept alive: is the price in bits derived or guessed?

| Idea | Price on the 16x16 | Grade |
|---|---|---|
| geometry A (tilt on the level) | none claimed; "what must be true" uses the D4 second analysis formula | condition derived, odds guessed, measured AUC 0.5 on toys |
| geometry B (lift) | none; a size (2^34.5 entries) | size derived, bits not derived, odds guessed |
| geometry C (level sets) | none; meant as a negative | n/a |
| other fields A (merging) | 2^67.4 more coarse solutions; 0.07 bit per cell needed | derived |
| other fields B (print) | "the whole problem" | derived with the row-0 caveat (F5); toy 17 to 20 bits measured as lower bounds; odds guessed |
| other fields C (groups) | none | n/a |
| question 2.3 (outside information) | a frame, two rows or about 20 cells suffice | cited from D3 Part B (measured, not independently assessed, painted boards); one figure mis-cited (Q9) |

No idea in the three memos carries a derived price of 10 bits or more together with a
mechanism. That matches the memos' own verdicts.

### 3.4 Citations opened for this review

| Source | Used for | What it says |
|---|---|---|
| arXiv:0901.2130, Krzakala and Zdeborova, "Hiding quiet solutions in random constraint satisfaction problems" | other fields A | Abstract: for some problem classes many properties of the planted ensemble match the random one. As cited. |
| Walshaw 2004, Annals of Operations Research 131, 325 to 372 | other fields A | Abstract: partitioning, colouring, travelling salesman; the framework "cannot be considered as a panacea". As cited. |
| arXiv:1601.00795, Shalev | other fields C | Title: "Simple groups, interleaved products, complexity and conjectures of Gowers and Viola". The abstract mentions the distribution of products of conjugacy classes with effective uniformity results; its named almost-uniform statement is about interleaved products of dense subsets. Loose support, not a covering statement. |
| arXiv:1208.0526, Ercsey-Ravasz and Toroczkai | geometry 15, ROUTES "Not run" | Abstract: solutions found "in polynomial continuous-time", at the expense of exponential fluctuations of the energy function. The memo's "analog time grows exponentially" is not what it says. ROUTES' "time or precision" is acceptable for precision only. |
| Pak, "Tile invariants: new horizons" (UCLA link) | other fields 25, C | The link returns a PDF; I could not extract its text. Not verified. |

## 4. Cheap tests worth running

**T1. Annealing against the relaxation at 6x6, like for like, and annealing at 7x7.** Under a
core-hour. Design in section 2.4. Restart annealing with the narrower schedule fixed here
(0.8 to 0.2, 2e6 and 2e8 steps), the five 6x6 boards of the growth run, 20 repeats per board,
CPU-seconds to the first verified solution, same machine and thread clock as the relaxation
rows. At 7x7: two boards, 100 runs at 2e8 steps and 5 runs at 3e9.
Kill lines: (a) "the relaxation reaches solutions that local search does not" is dead at 6x6
if annealing's expected CPU-seconds per verified solution is at or below the relaxation's on
at least four of five boards; (b) "annealing has no funnel at 7x7" is dead if any of the
7x7 runs solves.

**T2. None for the analog solver.** Its "Not run" reason loses one leg (S4) and its source was
misquoted (G7), but a digital simulation of it is a weighted local search, and T1 already says
what plain local search does at these sizes. If T1(b) fires, the reason should be rewritten;
no separate run.

**Considered and not recommended.** Cohomological consistency at k = 2 (cheap in compute, a
day of code, and the field version already determines nothing at 4x4 and 5x5); belief
propagation with an each-tile-once factor (RQ7 already exists with its kill line; the memos
add no reason to move it); the hole-solution overlap test of the different-question memo (it
can move the model, not the price); the class test with verified-dead prefixes (the covering
control already explains the null); probe A's 7x7 rung (mean AUC 0.50 on five boards and a
prior under 1 percent).

## 5. What I could not check

- The correlations, component sizes, barriers and beam bounds of the isolation memo beyond the
  rows compared with `summary.txt`. The BFS and beam code was not read.
- Whether the growth run's 6x6 boards are tile for tile the isolation boards.
- The relaxation's solve times: taken from the log and one raw row, not independently checked here.
- Probe A and probe B code: outputs compared with the memo's tables, code not read line by
  line. The class-product code was read; its group arithmetic was not re-derived.
- The 4,059,952 block count was rerun with the memo's script, not recounted with an
  independent one. Its null (60 painted boards) was not rerun.
- The first-moment order model of the different-question memo (its script 03) and the
  late-hole extrapolation.
- Sources the memos mark unverified (Liebeck and Shalev, Conway and Lagarias, Winfree,
  Son et al., Gardner et al., Cai et al., O Conghaile, Kondor, Zhao et al.). Wikipedia was not
  opened, to keep the puzzle's name out of web requests. The community digests (messages 901
  and 977): the raw archive is not on this machine.
- The narrower schedule used here was chosen once, by eye from the 5x5 solving band. It is not
  an optimum, and the reruns are 25 runs per arm at the large budgets.

## Ruling on T1, 2026-11-14

Test T1 of section 4 was run by a separate analysis that did not independent assessor
(`notes/anneal-vs-relaxation.md`, rows in `notes/anneal-vs-relaxation/`). Everything below is
on generated toy boards and says nothing about Eternity II by itself. My recount:
`review-far-ideas/t1_recount.py`, output `raw/t1_recount.txt` (a few CPU-seconds).

### What I checked

| Check | Result |
|---|---|
| Annealer | `iso_t.c` differs from `geometry-isolation/iso.c` by a header comment and four timing and printing lines; no search line changed. **Stands.** |
| Schedule and budgets | `run.sh` calls `anneal RUNS STEPS SEED 1 0.8 0.2`; `jobs.txt` has 2e6 and 2e8 at 6x6, 2e8 and 3e9 at 7x7. As fixed in section 4. **Stands.** |
| Boards | The three board files I compared are byte-identical to the isolation boards. |
| Solved boards | Re-verified with my own checker from each stored final board and the tile list (each tile once, rim grey exactly on the rim, every inner edge matched): 20 of 20 claimed solutions on the M1 laptop and 7 of 7 in the calibration are valid; no unsolved row is a valid board. **Stands.** |
| Annealing figures | Recounted from the rows: 8, 1, 7, 3, 1 of 20 at 2e8 with 28.6, 256.7, 33.4, 85.8, 244.3 CPU-seconds per solution on the M1 laptop; 0 of 100 at 2e6 in 12.3 CPU-seconds; 7x7: 0 of 200 at 2e8, 0 of 10 at 3e9. **Stand.** |
| Calibration | 1.224e7 against 1.522e7 steps per CPU-second, ratio 1.244, one board, one budget, heavy load. **Stands with a narrower scope**; see the margin below. |
| Relaxation rows | `growth-exponent/results/s6mac.jsonl` is unchanged since the copy (19 rows, byte-identical). Recount: 3 of 5, 3 of 4, 2 of 4, 1 of 3, 0 of 3 solved; 1,855, 1,145, 2,615, 4,939 CPU-seconds per solution and none in 5,400. **Stand.** |

**How the capped and missing relaxation runs enter.** Every finished start is charged at its
recorded seconds, capped starts at 1,800, which is the statistic section 2.4 asks for. The
batch is unfinished: boards have 3 to 5 of the planned starts. Rows are written when a start
ends, so within a cohort the solved starts appear before the capped ones; the truncation, if it
biases anything, lowers the relaxation's figure. Starts still running are not charged at all,
which also favours the relaxation. Neither bias works against the ruling below.

### Kill line (a)

As written: "the relaxation reaches solutions that local search does not" is dead at 6x6 if
annealing's expected CPU-seconds per verified solution is at or below the relaxation's on at
least four of five boards.

**It fires.** At 2e8 steps, in M4-equivalent seconds against the relaxation on the M4:

| Board | Annealing | Relaxation | Factor |
|---|---|---|---|
| s0 | 35.6 (40.8 measured on the M4 itself) | 1,855 | 52 |
| s1 | 320 | 1,145 | 3.6 |
| s2 | 41.5 | 2,615 | 63 |
| s3 | 107 | 4,939 | 46 |
| s4 | 304 | none in 5,400 | at least 18 |

Five of five on the point estimates, four needed. Limits, none of which reverses it:

- The clock conversion would have to be wrong by a factor of 3.6 to flip the closest board and
  by 46 or more to flip a second one; the calibration measured 1.24. On s0 both figures were
  measured on the same machine.
- The intervals overlap on s1, s3 and s4, where annealing has 1, 3 and 1 solutions and the
  relaxation 3, 1 and 0. The line was written on expectations and fires on them; board by
  board, only s0 and s2 are separated.
- The equal-cap arm was not run. It cannot favour the relaxation: 1,800 CPU-seconds hold about
  110 annealing restarts of 2e8 steps on the M4, and the pooled success per restart is 20 of
  100.
- The 2e6 arm (0 of 100, 12 CPU-seconds in all) decides nothing: at the rate of my arm R4
  (3 of 500) it expected 0.6 solutions. The line fires for the 2e8 budget, the better of two
  budgets fixed in advance.
- Against the x86 laptop rows no clock conversion exists. In raw seconds the gap is larger (17,000
  per solution or none), so they do not weaken the ruling; I do not rest it on them.

### Kill line (b)

As written: "annealing has no funnel at 7x7" is dead if any of the 7x7 runs solves.

**It does not fire.** No run solved. The claim is not killed and is not established either.

What the 7x7 result shows:

- Plain annealing on this schedule did not find a solution of either 7x7 board in 4,534
  CPU-seconds (0 of 200 at 2e8, 0 of 10 at 3e9), where exhaustive row-scan search enumerates
  every solution in 10.5 and 54.5 seconds. As a solver at 7x7 it loses to exhaustive search by
  a factor of at least 70 on these two boards.
- The success per run falls with size at a matched ratio of steps to exhaustive-tree nodes:
  58 percent at 5x5 (memo's schedule), 20 of 100 at 6x6, 0 of 10 at 7x7.

What it does not show:

- That the rate at 7x7 is zero. The 95 percent upper bounds are 1.5 percent per 2e8 run and 26
  percent per 3e9 run. Zero of 10 has probability 0.11 if the 7x7 rate equalled the 6x6 rate of
  0.20, so the equal-ratio arm does not even separate 7x7 from 6x6.
- That solutions are cut off. End states improve with budget (mode 5 mismatches at 2e8; 2 to 4
  on 9 of 10 runs at 3e9) and reach 2, the lowest non-zero count possible.
- Anything at the 2e8 budget about equal effort: it is 3.7 steps per exhaustive-tree node on
  seed 0, against 35 to 62 at 5x5.
- Anything beyond two boards with 2 and 3 solutions, each one family.

### What may be said

1. "On five 6x6 toys, restart annealing at 2e8 steps found a verified solution every 29 to 257
   CPU-seconds (20 of 100 runs), against 1,145 to 4,939 CPU-seconds and one board without a
   solution for the relaxation heuristic on the available rows, clocks matched by a measured
   factor of 1.24. The relaxation shows no edge over plain local search at 6x6."
2. "Both lose to exhaustive search, which enumerates these boards in 0.3 to 0.6 second."
3. "At 7x7 annealing found no solution in 210 runs and about 4,500 CPU-seconds; its rate there
   is below 26 percent per 3e9-step run and is not shown to be zero."

### What may not be said

1. "The funnel is gone by 6x6", or that annealing fails at 6x6 (sections 0.1 and 2.1, now on
   120 more runs).
2. That the relaxation solves 6x6 boards annealing cannot, or that its 6x6 solves are evidence
   of a non-local mechanism. This says nothing about the exactness of the lift, which is a
   separate question.
3. "Annealing has no funnel at 7x7" as a finding, or "isolated at 7x7". The budgets do not
   carry it.
4. That annealing beats the relaxation "by a factor of 50": the factor is 3.6 to 63 by board,
   on 1 to 8 solutions per cell, with an unfinished relaxation batch.
5. Any of this about the 16x16.

### Not checked

The relaxation rows' own solution verification (I used their `solved` flag); the x86 laptop clock;
the M4 load during the relaxation runs; whether a tuned relaxation or a tuned annealer changes
the factor (neither was tuned).
