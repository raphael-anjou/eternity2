# Route audit: kills that rest on weak evidence

2026-11-14, night. Draft, not independently checked. Read-only audit of `ROUTES.md`, `THESIS_PLAN.md`, `EXPLORED.md`, `AGENDA.md`, the reviews, `the log.md` to 01:16 and the evidence behind the suspect rows. Two small recomputations, scripts in `route-audit/`. Toy results are never Eternity II progress.

Marks: [M] measured by the cited source on committed data; [M here] recomputed by me from committed rows; [D] derived; [C] cited from a memo or review, not rechecked; [G] guessed.

## 1. The reading

**Count.** 73 index rows plus 5 lines the index does not hold yet (everything after 19:17: the convex lift, the low-rank heuristic, the isolation probe, window refuters, the far ideas). 47 of the 78 are closures (killed, priced out, "empty stands", or stopped). Of those 47:

- **32 are sound** for the goal of a solve, 13 of them with an index row that is stale or worded past its evidence (section 3).
- **10 rest on weak evidence and bear on a solve.** They collapse into five questions (section 2).
- **5 rest on weak evidence and do not bear on a solve** (record regime: local search, genetic algorithms, beam, learned priors, repair). No test is proposed for them.

**Why most kills hold today.** The level profile gives one yardstick for every check, prior or learned reason attached to a row scan: if it removes bits(d) of the nodes at free depth d, its gain is at most the minimum over d of ceiling(d) + bits(d) [D]. Ceilings recomputed from Track 10's independently assessed file: 57.1 bits at depth 52, 44.8 at 64, 29.8 at 80, 17.1 at 96, 10.7 at 112, 6.8 at 128, 2.4 at 144 [M here]. So 56 bits needs 26 bits of nodes removed by depth 80 and 39 by depth 96; arc consistency removes 1.2 at depth 96 [C]. Every row closed on "it bites after depth 144" stays closed, whatever its original evidence: Hall matching, small-region checks, pattern tables, window lifts, balance laws, the exact tail.

**What the yardstick does not reach**: solvers that are not trees, and reformulations whose cost is polynomial if a property holds. The weak kills sit there, most from the last 30 hours.

**The three to reopen first.**

1. **Exactness of the convex lift at 6x6.** Stopped with its kill line unused. It is the only item on file whose pass, carried to 16x16, is priced above the 56-bit gap. About 6 to 170 core-hours. My odds: 40% it passes at 6x6, 1% it matters at 16x16.
2. **Growth of the non-tree solvers against the tree on the same boards** (difference map, annealing, low-rank descent, message passing). Each was closed at a cap; none has a growth figure next to the tree's. About 120 core-hours. Odds of 10 bits: under 1%.
3. **Learning solvers at 7x7 and on real-piece prefixes, with a propagation-only control** (THESIS_PLAN 9.4, the item this audit replaced). About 80 core-hours. Odds of 10 bits: 1%.

None has odds above the program's own figures. The audit changes which tests are worth their cost, not the overall reading.

## 2. The weak kills

Costs are core-hours on the three machines (32 cores).

### 2.1 Convex lift: closed without its kill line (not in the index)

**Rests on.** the log 22:52: "No 6x6 result of any kind was obtained ... The kill line for the convex 6x6 test stands unused; the exactness question at 6x6 is open." the log 01:12: "every gain tonight is a constant factor at toy size; the exponent decides 16x16 and the first data puts it at 2.5 or worse."

**Why that is not enough.** The 01:12 data are evaluation counts of the low-rank heuristic. Theory B shows that heuristic solves 46 of 50 at 5x5 with the constraint removed that makes the lift exact, so it is "closer to a penalty descent with early stopping than to the relaxation" [C, the log 01:16]. The convex lift is a different object: polynomial cost if exact. `relaxation-engineering.md` prices it at 3e4 to 4e9 core-years on the 16x16 against 6e25 for row scan, conditional on exactness [C, its author's derivation, not independently checked]: 54 to 71 bits [D]. Evidence against exactness already on file: theory D found one 5x5 toy of five (seed 3, 403 solutions) with 0.0047 of the mass per cell off the solutions, converged, one solver [C]; the theory memo's own literature review predicts failure at large n [C].

**Test.** Boards: the three 6x6 toys already set up (frame 4, inner 5, seeds 0 to 2), `convex-6x6-scripts/admm.py`, largest-leak objective to 1e-7, resumed from the checkpoints. Measured: mass off solution placements per cell. Baseline: the LP on the same boards (18 percent of weight off solutions at 5x5 [C]) and the 5x5 certificate (8e-9 per cell [C]). Kill line, as fixed at 22:20 before any 6x6 result: above 0.05 per cell on 2 of 3 boards. First, the 5x5 seed 3 leak solve to convergence (20 core-minutes [C]). Cost: 16,000 to 118,000 iterations at 0.4 to 1.7 s, three runs: 6 to 170 core-hours, 1 to 2.5 days of wall time on the M1 [D from the memo's range].

**Worth.** 0 bits from a 6x6 pass alone. 54 to 71 bits if exactness held to 16x16 [C]. As a root filter without exactness: 4.1 to 6.6 bits at 5x5, toys [C].

**Odds.** Pass at 6x6: 40% [G] (author 60%, lowered for the seed 3 leak). Exact or usable at 16x16: 1% [G] (author 2%).

### 2.2 Non-tree solvers closed at a cap (four closures, one question)

**Rests on.**
- Difference map, index row "killed at the cap". Independent assessor: "the kill is of this method at this cap (beta 0.25, 20,000 iterations)". At 6x6 it needed a median of 12,156 iterations where `bb-mrv` needed 460 nodes; at 7x7 `bb-mrv`'s median is 588,305 nodes [C, Track 11 ASSESSMENT]. A cap of 20,000 is 29 times under the baseline's own count, so 0 of 30 was the expected outcome even with the same growth as the tree [D]. The independent assessor says so ("A larger iteration cap could give finite n = 7 medians") and still asks for "a different mechanism, not a larger cap"; beta was calibrated over four values, none clearly better.
- Annealing to a solution (geometry-isolation, not in the index): "0 of 1,000 at 6x6" at about 0.2 CPU-second per run. The re-check's reruns solve 7 of 25 at 2e8 steps [C, raw rows; the review file still has unfilled placeholders].
- Low-rank descent: 5x5 median 3,017 evaluations; at 6x6 on the Mac 4 of 8 starts solved (24,379 to 140,075) and 4 capped at 1,800 s [C, `growth-exponent/results/s6mac.jsonl`]. Fitted against P, never against the tree.
- Survey and belief propagation, index row "killed (per EXPLORED)": "one toy instance and three settings", a 4x4 [C, dead-ends page]. RQ7 (message passing with an each-tile-once factor) was never run.

**Why that is not enough.** Such a solver is independently assessed by how its work grows against the tree's on the same boards. No such pair of numbers exists. Indicative only, different boards [D]: the gap 4 re-check's ladder has the exhaustive row-scan tree growing 4.1 bits from 5x5 to 6x6 (2^18.0 to 2^22.1 [M here]); the low-rank count grows at least 5.5 bits over the same step (median above 140,075, half censored). That is no sign of slower growth, and no measurement of it.

**Test.** One ladder: generated critical boards, 5x5, 6x6, 7x7, 10 each, one painter. Measured: CPU work to a first verified solution, uncensored up to 300 times the baseline's time. Arms: annealing (the re-check's better schedule), difference map at band 0 with three values of beta, the Rust low-rank solver unchanged, message passing with decimation. Baseline: row scan with arc consistency, median over 100 random value orders, same boards. Kill line: an arm is dead if its median work grows from 6x6 to 7x7 by at least as many bits as the baseline's; it survives only below 0.75 of the baseline's growth with a bootstrap interval that excludes 1. Cost: about 120 core-hours [G: 26 difference map, 40 annealing, 54 low-rank].

**Worth.** Not bounded by the level profile. If an arm kept 0.75 of the tree's bits per cell to 256 cells: 34 bits less, minus the 12 to 16 bits it is behind at 5x5, so about 20 bits [G, linear extrapolation].

**Odds.** One arm survives: 5%. 10 bits on the 16x16: 0.5 to 1% [G].

### 2.3 Learning: clause learning, cardinality reasoning, conflict-driven orders

**Rows.** Blocker eviction ("killed (these implementations)"), no-good learning (undecided), SAT and cube and conquer (flagged), gap 1 ("unproven in general").

**Rests on.** Gap 1 review: 4x4 to 6x6, 4 to 8 boards a size, log2(tree nodes / conflicts) 5.1, 6.1, 6.5, 6.1; the one 7x7 board "stopped unfinished after 13 min"; "the solver propagates and chooses its own order, and the tree does neither"; "No one has run a pseudo-Boolean solver." Track 2 independent assessor: "Does not cover: ... Conflict sets built from propagation explanations." Gap 4 review: weighted degree is "the only untested family" (three boards at 7x7).

**What I could add.** On the gap 4 re-check's committed exact rows, arc consistency alone is worth 4.6 and 4.7 bits at 5x5 and 6x6, and with a smallest-domain order 5.4 and 5.25 (12 boards each) [M here, `propagation_control.py`]. Set beside the solver's 6.1, about 5 of its 6 bits are propagation and order, about 1 is learning [D, indicative: other boards, branches against conflicts]. That supports the kill at toy size and says nothing on growth. The only learning datum on the real pieces is D1's refuter on frames (Glucose; lower median ratio 1.00; every refutation within 0.35 s or none in 600 s [C]), not on prefixes.

**Test.** (a) Toys at 7x7, 8 boards, complete job (all solutions and the proof), 2 hours a run: CaDiCaL, one cardinality solver, weighted degree with restarts under arc consistency; controls: plain row scan and row scan with arc consistency. (b) Real pieces: 40 wrong prefixes at each of free depths 112, 104, 96, 88, where the mean subtree is 2^13 to 2^25 nodes [D from the profile], same arms as refuters against exact subtree counts. Kill line: the program over the propagation-only control does not rise by 2 bits from 6x6 to 7x7, and not by 3 bits from depth 104 to 88. Cost: about 80 core-hours [G].

**Worth.** For reasons attached to row-scan nodes: 2 bits, ceiling 6.6 [C, gap 1]. Beyond that family: 10 bits needs nearly every node cut by depth 113, 56 bits by depth 52 [C, confirmed here]. A conflict costs 2^5 to 2^7 nodes at 6x6 [C], so a time gain needs the program to outgrow that as well.

**Odds.** Lead rising with size: 5%. 10 bits: 1% [G].

### 2.4 The stratified-painter loophole (row "Traces of the maker's program")

**Rests on.** `solve-path-2`: a guessed 1% prior, an assumed usable share of 0.5 to 0.75 of 94 to 118 bits, and an experiment "under 10 core-hours" never run.

**Derived here** (`strata_gain.py`, first-moment model, multinomial colours, Track 10's profile). Under row scan a quadrant budget removes 2.8 bits of nodes at depth 80, 8.4 at 96, 21.9 at 112, and gains 23.6 bits in all (tree 2^110.8); a top and bottom balance gains 17.5; a horizontal and vertical balance 1.1 [D]. The usable share is 0.26, not 0.5 to 0.75. Even with a true quadrant prior the tree stays 33 to 49 bits over budget.

**Verdict.** Sound now, for a better reason than guessed odds. No test. Not priced: a quadrant-first order under the prior (one desk hour with D3's model).

### 2.5 Linearisation degree

**Rests on.** "measured, not independently checked"; degree 3 at 5x5 "did not fit in memory"; "Neither formal closing condition is measured."

**Test.** Degree 3 at 5x5 on the 30 GB Linux machine, 5 boards, sparse elimination. Kill: degree 3 fails on 3 of 5. Cost: 10 to 20 core-hours, memory-bound [G]. Worth if degree 3 held at every size: 36 bits (2^98.4 [C]); at degree 4, 5 bits. Odds: 25% at 5x5, under 0.1% at 16x16 [G], since 4x4 already needs degree 4 on a third of the boards.

## 3. Sound, but the index row says more or less than the evidence

1. **Other fill orders.** Now: "measured and independently assessed on the real pieces (D3): 15 of 18 static orders do not beat row scan; 3 undecided because the estimator dies; the model puts them above row scan."
2. **Propagating DFS lead** ("undecided"). Now: "settled at 16x16 by the gap 4 second analysis: smallest-domain rules are 12.8 to 49.3 bits worse on the official pieces. Under arc consistency that is one run at M = 500 (9 and 15 s) [M here], which an unbiased estimator still makes decisive: probability at most 2^-12.8 of such an overestimate [D]. The small-board lead is propagation (4.6 bits), not order (0.6)."
3. **Restarts.** Now: "independently re-checked (gap 3): median 0.5 bits, maximum 4.83, blind schemes on painted boards", not "per EXPLORED".
4. **Frame-first enumeration.** Replace 10^8.3 by 10^13.59 for frames that pass the corners, and add D1 stage 1 (measured).
5. **Nogood-driven frame search** ("open") and **D1** ("running"). Now: "independently assessed keep: nested arc cost 2.2 to 2.5 bits above break-even at k = 12 to 15; far stratum undecided and a necessary condition only." Ten bits would need arcs refuted 2^12.5 times cheaper [D].
6. **D3 Part B** ("open, not independently assessed"). Now: independently assessed keep at 19:31, Part A headline withdrawn.
7. **Arc consistency follow-up** ("killed at the measured speed"). Now: "worth 5.2 to 5.5 bits even if free; speed is not the reason." The measured speed is one implementation that rebuilds every domain from scratch per prefix.
8. **LP / ILP.** Now: "LP measured weak on toys; the 420 to 440 figure is one 2009 solver. ILP is a learning solver: see gap 1."
9. **Exact cover / DLX.** Now: "a smallest-domain tree search; priced by gap 4 on the real pieces; tile branching argued only."
10. **Tensor networks and per-piece prices.** Add: "the site's notebook claim (4x4 and 6x6 solved) is uncommitted and did not reproduce (0 of 19); not independently checked."
11. **Difference map.** Now: "not measured near critical: cap 29 times under the baseline at 7x7; dead by censoring, no exponent."
12. **Small-region reasoning.** Add: "on the real pieces a window gains fillings per cell while more than 72 tiles remain (theory D, counting), so no window check bites before depth about 180."
13. **Traces of the maker's program.** Replace "29 to 118 bits" by "23.6 bits under row scan for quadrants (derived 2026-11-14)".

## 4. In the documents, absent from the index, never run

Bearing on a solve only. Costs and odds are their authors'.

| Item | Source | Stated cost, odds |
|---|---|---|
| Lift at 6x6; lift on boards with no solution ("the single measurement that separates the two readings"); nonnegativity within distance 2 at 6x6 and 7x7 | `convex-6x6.md`, `relaxation-theory.md`, `theory-d` | days; 20 core-min to 2 h; under 0.5% |
| RQ2 persistent solver across frames, and its replacement (cores at depth 113) | THESIS_PLAN 4 and 9.5 | 5 core-hours |
| RQ7 message passing with an each-tile-once factor | THESIS_PLAN 4 | core-minutes, 2% |
| Cardinality solver against CaDiCaL on D1 frames | `surveys/sat-cp-proof-complexity.md` T2 | minutes, 2% |
| Learned ranker of partial frames | `surveys/learning-guided-search.md` 4.2 | a few GPU-hours, 5% for 6 bits |
| Pass rates of extreme pruning on painted 8x8 | `surveys/monte-carlo-rare-events.md` C2 | 1 core-hour, 3% |
| Runtime distribution of randomised backtracking on the critical ladder | same, C1 | a few core-hours, 5% |
| Dispersion of tree size over randomised dynamic orders with arc consistency, real pieces | gap 3 review | not costed; kill under 11 bits |
| Hall test redone with a level-uniform sampler; bottleneck integer program | gap 6b review | one study |
| Non-abelian group on the 54 regions in a computer algebra system | gap 5 review | half a study |
| Cohomological consistency at k = 2 ("no measurement or theorem behind its kill") | `far-ideas-other-fields.md` 27 | a day of coding |
| Fourier on the symmetric group at order 2 (dismissal "asserted, wrong citation") | far-ideas geometry 14 | none |
| Selby and Riordan goodness as a value order | `surveys/tilings-jigsaws.md` T1 | a few core-hours, under 1% |
| Agenda entries A23, A29 to A34 on the docs branch | reported by a helper analysis; not opened by me | n/a |

Also missing: the evening's results themselves, and two unfinished second analysis reports (`far-ideas-and-isolation-recheck.md` has unfilled placeholders; the convex and low-rank reviews have scripts and no report).

## 5. What I could not check

- **Track independent assessor files.** Opened by me: Track 11 and D1 (head), Tracks 2 and 6 (scope lines). Read through a helper analysis: Tracks 1, 2, 4, 6, 11, 12, D1, D3, D4, D5. Not opened by anyone tonight: Tracks 3, 5, 7 to 10, 13 to 15, D2. Those rows are graded from the index, REPORT and the log.
- **Reviews and evening memos** were read through two more helper studies, with my own spot checks of the gap 1 table, gap 4 sections 1, 5, 6 and 8, theory D's head, `convex-6x6.md` and the engineering table. Other quotes are theirs.
- **No experiment was rerun.** My two scripts read committed rows.
- **The 2.2 and 2.3 cross-reads compare different boards and units.** They are indicative and marked so.
- **The strata figure** is a first-moment model with multinomial colours and independent strata, row scan only.
- **The gap 4 estimator** was validated on five rules, not on the free smallest-domain rule; my Markov reading of its single runs assumes it is unbiased there.
- **Site pages**: only the dead-ends entries on survey propagation and tensor networks and the iterated-maps page. The rest is taken from `EVIDENCE_AUDIT.md`.
- **Not verified:** the docs branch, the surveys' citations, the 54 to 71 bit figure (not independently checked).

## Appendix: every row examined

Verdicts: S sound; S* sound, row stale or overstated (section 3); W weak (section 2); R weak, record regime, no bearing on a solve; n not a closure.

| # | Row | Index status | Verdict | Basis |
|---|---|---|---|---|
| 1 | Row-scan price | priced out | n | measurement, total 10^40.457 reproduced here |
| 2 | Other fill orders | killed per EXPLORED | S* | D3 independently assessed |
| 3 | Propagating DFS lead | undecided | S* | gap 4 real pieces, Markov |
| 4 | Value order by impact | killed | S | measured; a value order cannot shrink a refutation tree |
| 5 | Blocker eviction | killed (these implementations) | W | 2.3 |
| 6 | Hall matching | killed / kept small | S | 0 of 16,833 on real pieces; profile |
| 7 | ProbCut | killed | S | D4: no statistic above 1 bit |
| 8 | Backdoors | A16 killed | S | measured, independently assessed |
| 9 | Arc consistency, Régin | killed (mapped) | S | profile: 5.2 to 5.5 bits if free |
| 10 | Small-region reasoning | killed | S* | profile; window counting |
| 11 | Cut-keyed partitioning | killed | S | matched control |
| 12 | No-good learning | undecided | n | see 2.3 |
| 13 | Restarts | killed per EXPLORED | S* | gap 3 independently re-checked |
| 14 | Forbidden-pattern pruning | killed for strict search | S | a solve is strict search |
| 15 | Colour-ledger prune | open | n | priced at 5.6 bits |
| 16 | Survey and belief propagation | killed per EXPLORED | W | 2.2 |
| 17 | Meet in the middle | priced out | S | gap 2 independently re-checked, 3.0 bits |
| 18 | BANDSAW soft interface | open | n | record regime |
| 19 | Exact tail | killed as record lever | S | profile: the tail is free |
| 20 | Pattern databases | killed per EXPLORED | S | O1 independently re-checked; merge factor measured |
| 21 | Frame-first enumeration | priced out | S* | D1 stage 1; corrected interior |
| 22 | Cheap frame information | killed | S | D1 stage 1 on real pieces |
| 23 | Nogood-driven frame search | open | S* | D1 independently assessed |
| 24 | D1 | running | n | independently assessed keep |
| 25 | Frame as basin | killed per EXPLORED | n | record regime |
| 26 | SAT, cube and conquer | killed for full board | W | 2.3 |
| 27 | LP / ILP | killed per EXPLORED | S* | LP measured weak; ILP in 2.3 |
| 28 | Exact cover / DLX | killed per EXPLORED | S* | gap 4 |
| 29 | Difference map | killed at the cap | W | 2.2 |
| 30 | Tensor networks, prices | priced out | S* | state counts; not independently checked |
| 31 | Weighted CSP, MaxSAT | open | n | no run |
| 32 | First-moment counts | kept | n | |
| 33 | Growth law | kept | n | |
| 34 | Ladders as proxy | killed as proxy | S | independently assessed |
| 35 | Frame count A5 | kept | n | |
| 36 | Boards per score | kept | n | |
| 37 | 470 board as damaged solution | priced out | S | counting; not independently checked |
| 38 | Overlap gap A10 | killed | S | within its scope |
| 39 | Theory pages | kept | n | |
| 40 | Spectral recovery A1 | killed | S | independently assessed |
| 41 | Traces of the maker | killed | W | 2.4, now priced |
| 42 | Break-tolerant DFS | kept | n | |
| 43 | Port fidelity T13 | kept | n | |
| 44 | D2 (section 7 row) | running | n | |
| 45 | Five-clue keying | kept / killed | n | port only |
| 46 | Where a record is paid | undecided | n | |
| 47 | Score bound | undecided | n | profile: late |
| 48 | Local search | killed per EXPLORED | R | reported |
| 49 | Quality-diversity A4 | killed | S | independently assessed |
| 50 | Genetic algorithms | killed per EXPLORED | R | reported |
| 51 | Beam | killed per EXPLORED | R | uncommitted |
| 52 | Learned priors | killed per EXPLORED | R | argued |
| 53 | Repair | killed per EXPLORED | R | uncommitted |
| 54 | Public 466 | stale on site | n | |
| 55 | Hardware | undecided | n | arithmetic: 2.5 orders |
| 56 to 60 | Tools (5 rows) | kept | n | |
| 61 | D2 | kept | n | |
| 62 | D4 | kept | n | |
| 63 | D4 follow-up | killed at measured speed | S* | profile |
| 64 | D5 | kept | n | |
| 65 | D3 Part B | open | n | independently assessed since |
| 66 | Gap 1 | unproven | W | 2.3 |
| 67 | Gap 2 | empty stands | S | one-reader bound |
| 68 | Gap 3 | empty stands | S | independently re-checked |
| 69 | Gap 4 | empty stands | S | real pieces; learning orders in 2.3 |
| 70 | Gap 5 | empty stands | S | 0 bits before depth 192 |
| 71 | Gap 6a | none found | S | worded as it should be |
| 72 | Gap 6b | empty stands | S | no certificate |
| 73 | Linearisation degree | not formally closed | W | 2.5 |
| 74 | Convex lift (not in index) | stopped | W | 2.1 |
| 75 | Low-rank descent (not in index) | "local optimum" | W | 2.2 |
| 76 | Annealing, isolation (not in index) | "funnel gone by 6x6" | W | 2.2 |
| 77 | Window refuters (not in index) | vacuous | S | counting, theory D |
| 78 | Far ideas (not in index) | none above 1% | S | authors' odds; three closed by decision |
