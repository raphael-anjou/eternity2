# Thesis plan, draft 2 (2026-11-13)

Status: draft 2 (15:50). Section 9 adds the outcome of tracks D2 to D5 and of the seven independently re-checked gap studies; one figure of section 3 is corrected in place and marked. Sections 4, 7 and 8 are superseded by section 9 where they differ. Earlier status: draft 1, revised after the two re-checks of the eight surveys reported (`surveys/DERIVATIONS-recheck.md`, `surveys/CITATIONS-recheck.md`). Draft 0's obstruction O2 rested on a derivation the second analysis found wrong; it is rewritten below. Statements now carry the re-check's ruling where one exists. Grades used: measured (committed data, independently assessed), independently re-checked (argued, second reader agreed), survey (read from literature or derived today, not independently checked), open.

## 1. The question

Not "how do we solve Eternity II" but the question whose answer would tell us whether and how it can be solved:

**What is the cheapest way to find a solution of an edge-matching instance at the critical colour density, and what would have to be true of a method for it to beat backtracking by 60 bits?**

A thesis on this question succeeds without a solution if it delivers the measured price, the reasons, and a ranked map of what is left. A solution remains the goal and a long shot; the stated odds stay near one in a million on the routes examined.

## 2. Where we stand, in numbers

| Fact | Value | Grade |
|---|---|---|
| Row-scan search of the real puzzle, five clues | 2^134.4 nodes (10^40.46) | measured |
| First-moment model of that tree | within 0.03 digits | measured |
| Affordable at 1e8 nodes per second | about 2^61.5 for 1,000 core-years; 2^78 for a very large distributed effort | arithmetic |
| Gap | 56 to 73 bits, depending on the budget assumed | arithmetic |
| Share of the cost that is partial-frame multiplicity at the peak | about 30 of 39 digits | independently re-checked |
| Cheap information about the frame | about 17.4 bits, all at the four clue corners | independently re-checked, recomputed twice |
| Interior cost under a frame that passes the corners | about 10^13.6 nodes in the mean (not 10^8.3) | independently re-checked; D1 measures it |
| Trace of the construction in numbering or orientation | none: the numbering is a sort of the piece set | independently re-checked |
| Largest critical-density board solved blind in the literature | about 10x10 (a 2026 preprint's 16x16 used the known answer; read in full) | survey, citations audited |
| Static fill orders in the model with the real colour counts and clues | row scan 40.52 digits; anti-diagonal 50.39, spiral 51.99, frame-first 52.49 | independently re-checked model figure; Track D3 measures it |

## 3. The obstruction, stated so it can be attacked

Three statements. Each is a conjecture about this instance with a way to falsify it. Together they are the thesis's working explanation of why six paths and eight fields returned nothing.

**O1. Each tile once defeats tabulation.** Without the rule the board is a width-16 colour-only contraction: 2^62 to 2^66 states, 2^75.4 tile placements. With it, every exact method priced so far costs between 2^131.6 (model cost of a two-list join; corrected in draft 2: 2^126.6 was a list size, not a cost, see `notes/gap-2-lossy-representations-recheck.md`) and 2^134.4 (measured row scan). Proved in a survey and checked by its second analysis on 130 toy cases: any algorithm that tabulates fillings of regions by (boundary colours, used tiles restricted to a tracked subset) and sieves the remaining tiles by inclusion and exclusion over tile subsets, for any split, cell order or region decomposition, writes at least as many table entries as there are distinct (boundary, full used set) pairs among the strict fillings of the same regions; and no rank or representative-set argument that does not decide which states are live can compress that, because the compatibility matrix between partial fillings and completions is block-diagonal with permutation blocks, full rank over every field. Scope, exactly: one family of algorithms (region tables plus tile-subset sieves, oblivious linear compression), compared against the strict table of the same decomposition, not against 134.4 bits as such. It is not a lower bound for the instance. It says nothing about search that prunes with instance-specific reasoning, sieves not indexed by tile subsets, or lossy hashed states. The rule is not exotic (it is linear in lifted variables and hashable): what it defeats is tabulation. Grade: independently re-checked, toys only; the merge factor on the real tiles is argued to be about 1, not measured. Falsified by: a representation of partial boards on a 6x6 to 8x8 board whose size grows slower than the number of strict partial boards at the widest cut.

**O2. No statistic found so far points at the solution.** Draft 0 said the puzzle cannot be told from a random instance with more than 0.4% advantage. That was wrong: in the first-moment null the piece set is distinguishable with advantage of at least 8% (edge-piece colour counts, colour repeats inside tiles, corner colours), no upper bound is computed, and the bound concerned detection from the piece set, which says nothing about recovery and is vacuous once the clues are used. What stands: the search tree's level sizes match the random model to 0.03 digits (measured: about means, not about shape); the numbering and stored orientation are a sort of the piece set (independently re-checked); nineteen plus twelve statistics of the piece set show no trace of the arrangement (independently re-checked). What does not stand as a barrier: "restarts cannot change the exponent" is supported on 54 toy boards (median gain 0.00 bits) but the real tree's shape is unmeasured; "sampling needs the inverse probability" is a theorem for plain importance sampling only. So O2 is an empirical statement with a hole in it, not a theorem. Falsified by: Track D4, if a cheap statistic of a partial board shifts by 1.4 sd or more on the true prefix at the peak depths. Corrected value of such a tilt (second analysis): about [delta z_K + delta^2 / 2] / ln 2 bits with z_K the 1/K upper normal quantile, IF the K rerandomised runs are independent; at correlation 0.6 a hundred runs at delta = 1.4 give 2.8 bits, not 7.5, and fully dependent runs lose 3 bits. The four scan directions of one board cost 2^4.19 times the best one. So the correlation between scan directions is the number that decides whether any tilt is usable; D4 measures it.

**O3. No locality.** A wrong partial board is not wrong for a local reason. Evidence: propagation sees nothing past the corner windows (measured); the assembly theorem needs windows thousands of cells wide at this colour count (survey). Falsified by: D1, if wrong frames turn out to have small cores that are cheap to prove away from the clue corners.

If O1, O2 and O3 all hold, a method must get about 60 bits from somewhere that is neither algebra, statistics nor local reasoning. Two places remain in principle: information from outside the search (more known tiles: in the model about 9.6 bits for the best-placed tile and a median of 4.8 for a random cell, so 6 to 7 well-placed tiles would cover 56 to 66 bits (independently re-checked model figure; Track D3 measures it)), or a non-local, non-statistical structure nobody has named. Saying this plainly is part of the result.

## 4. Research questions and their experiments

Ranked by bearing on a solve. Each has a baseline and a kill line; odds are the program's.

| # | Question | Experiment | Cost | Kill line | Odds of a positive | State |
|---|---|---|---|---|---|---|
| RQ1 | Do wrong frames have small, cheap reasons away from the corners? (O3) | D1 frame cores, real pieces, with the bottom-arc break-even reading | running, about 12 h on the x86 laptop | quantile floors above 134.4 bits; no arc below break-even | 5% | results after 22:00 |
| RQ2 | Does a solver kept alive across frames reuse what it learned? (the one objection the nogood model does not price) | incremental solver on frames sharing a 44-cell arc; time per frame against frame index | 1 day | no downward trend over 1,000 frames | 5% | not scheduled |
| RQ3 | Is there any statistic that tilts toward the true prefix, and is it independent across scan directions? (O2) | Track D4: ten preregistered statistics on planted 16x16 boards against sampled prefixes, with controls | about 4 core-hours | no statistic with a shift of 1.4 sd at the peak | 5% for a tilt; 1% for a usable one | running |
| RQ4 | Is O1's merge factor about 1 on the real tiles, and does O1 extend beyond region tables? | measure distinct (boundary, used set) pairs against strict fillings on real-tile regions up to the size that can be enumerated; look for a counterexample family (lossy hashed states) | desk plus 1 core-hour | none: this is to establish, not to kill | n/a | barriers independently re-checked on toys; real tiles open |
| RQ5 | What is a known tile worth, and where? | first-moment model plus the validated estimator: bits gained per extra pinned tile, by cell | part of Track D3 | none: a calibration | n/a | running |
| RQ6 | Is row scan the best order on the real pieces? | price other static orders with the validated estimator; then an adaptive-order test with the seven design rules from the learning survey | part of Track D3, then days | no order more than 1 digit better | 3% for 1 digit after the re-check's model figures (row scan ahead by 9.9 digits); under 0.1% to close the gap | running |
| RQ7 | Message passing with an each-tile-once factor on frame marginals | 8x8 and 10x10 generated boards | core-minutes | under 3 bits beyond the corner bits | 2% for 10 bits | low priority |

RQ5 looks like it has no bearing on a solve, but it is the measurement that turns "we are 60 bits short" into "we are this many tiles short, and these cells are worth the most". It is also the number a reader outside the field understands at once.

Not pursued, with the reason on file: quantum (21 orders in constants), seed enumeration (odds a guess, a negative uninformative), score bound, decompositions and joins, difference map, local search toward high scores (those boards are 246 or more cells from a solution).

## 5. Papers

**Paper 1, writable now: "The price of Eternity II".** A measurement paper.
- Content: the measured row-scan tree on the real pieces with one, with five and with no clues; the estimator and its validation; the model's 0.03-digit agreement; the official set against boards painted with the same colour counts; where the cost sits (frame multiplicity; the corner bits); the families priced out, each with its number.
- What it needs before it is a paper: Track 10's branch merged and its run reproduced by a second analysis from the command in its README; the frame decomposition measured rather than argued (D1's stage 0 and the corrected interior cost); one independent recomputation of every figure in the abstract; related work from the eight surveys after the citation audit.
- Form on the site: a research page under the existing pipeline, with the PDF as an artefact. Venue is open (arXiv is the natural first step).

**Paper 2, after RQ4: "Each tile once: why the methods that work elsewhere do not work here".** The obstruction paper. The eight surveys condensed into one table of field, tool, price in bits on the 16x16, and the reason; the barriers that survive their second analysis; the small-board exact counts. Negative, and useful to anyone who thinks of trying one of these.

**Paper 3, depends on RQ1 and RQ2: frame refutation.** Either a positive result (cores are local and cheap: then a method paper) or a short negative note folded into Paper 2.

**Standing document: the routes index** (`ROUTES.md`, to be merged into `EXPLORED.md`), kept current, so that nothing is examined twice.

## 6. How the work is run

1. Read before inventing: a route enters the agenda only with its row in the routes index, the closest published work, and a price in bits.
2. Nothing is reported as a finding before a second reader has ruled on it. This applies to the program's own summaries.
3. Experiments are preregistered, on the real pieces when the question is about the real puzzle, and independently assessed by a analysis that did not run them.
4. Every direction carries stated odds and a kill line, and is closed on schedule.
6. The site says only what committed evidence supports, with its grade visible.

## 7. Next steps, in order

1. Tonight after 22:00: collect D1, report, independent assessor (RQ1). Recheck of the port fix round.
2. On the re-checks' reports: revise section 3, then plan RQ3, RQ5 and RQ6 as one preregistered measurement set on the lab machines (they share the estimator and the planted boards).
3. Start Paper 1's draft from Track 10's README and the corrected frame figures; list every number in it with its source file.
4. RQ4 exact counts on small boards.
5. RQ2 only if D1 leaves it worth asking.

## 8. What would change this plan

- D1 passes its necessary condition away from the corners: RQ1 and RQ2 move to the top and Paper 3 becomes the main line.
- The derivation second analysis breaks O1 or O2: that obstruction is rewritten before anything rests on it.
- RQ3 finds a statistic with a real shift: extreme pruning with rerandomisation becomes a method track, worth an estimated 4 to 11 bits.
- Tiles of the solution are obtained from outside the search: RQ5 says what they are worth.

## 9. Draft 2: what the day added (2026-11-13, 15:50)

Sources: `the log.md` from 12:15, the track branches D2 to D5 with their `ASSESSMENT.md`, and `notes/gap-*.md` with their `-recheck.md`. Grades as in the header. D1 and the second half of D3 are not in yet.

### 9.1 Tracks

| Track | Result | Grade |
|---|---|---|
| D2 port fidelity | Seven differences from the reference engine fixed; with five clues the faithful defaults do worse than the old ones | measured, independently assessed twice |
| D4 prefix statistics | One census statistic tilts about 2 sd toward the planted prefix at depths 144 and 160 on painted boards; worth 1.02 bits in one direction, nothing with two or four; no transfer claim | measured, independently assessed |
| D4 follow-up | Arc consistency leaves 43, 10, 18, 17, 6, 0.7 and 0 percent of sampled wrong prefixes at depths 96, 112, 128, 136, 144, 152, 160 on painted boards; about 5 bits of nodes on the official profile, at about 13 bits of cost per node at the measured speed | measured, not independently assessed; the 5 bits is the program's arithmetic, later matched by two re-checks (5.5 bits) |
| D5 pin map | Under row scan a known tile is worth most in the first rows scanned; touching pairs lose about half of the second tile's worth; best-first counts need the solution | measured, independently assessed |
| D3 Part B | On painted 16x16 boards: one extra pin worth 10.05 bits at the best cell and 5.79 at the median; ten spread pins leave at least 10^22 nodes; a 10^15 budget needs more than 10 and probably 16 to 24 spread pins | measured, not independently assessed; the upper end is checked end to end only |

O2 is revised by D4: a statistic that points at the true prefix exists. It is the size-biased tilt the random model predicts plus 50 to 80 percent, and it is worth about one bit. O2 now reads: no statistic found is worth more than about one bit.

### 9.2 The seven gaps

Each gap is a place the three obstructions do not reach. One memo and one second analysis per gap.

| Gap | Ruling | Largest gain priced |
|---|---|---|
| 1 Search that learns | Unproven in general. Closed for reasons attached to row-scan nodes and used one at a time | about 2 bits (ceiling 6.6); an off-the-shelf clause-learning solver needs 2^5 to 2^6.5 fewer conflicts than tree nodes on 5x5 and 6x6 toys, flat with size |
| 2 Lossy representations | Empty stands for colour-blind sketches and rectangular region lists | join 3.0 bits; sound sketch at most 2.0 (re-check's bound, wants a second reader) |
| 3 Shape of the tree | Empty stands for blind restarts and portfolios over value orders and scans | 1 to 3 bits with oracle cutoffs; a loss blind |
| 4 Adaptive orders | Empty stands for row-shaped rules; free smallest-domain is 13 to 49 bits worse | under 3 bits; proved: finishing rows one or two at a time gains at most 5.3 or 6.7 bits |
| 5 Structure of the tile set | Empty stands: no colour automorphism, invariant group Z^2 + Z/2 | 0 bits before depth 192 |
| 6a Algebra and counting | None found among fifteen reformulations; not a barrier | linearisation needs degree 3 to 4 on 4x4 toys and is bound by memory on the 16x16 |
| 6b Search space | Empty stands for static reformulations; four sides identify one tile, so edge search is tile search | row scan within 0.24 bit of the best static order (no certificate) |

### 9.3 What the gaps have in common

1. Every test that refutes wrong boards (arc consistency, matching, integer balance laws, learned reasons) starts to bite between depths 144 and 192. The widest level of the measured tree is 158 and the cost before it is already within a few bits of the total. This is O3 seen from the other side, and it is the single explanation behind most of the day's results.
2. Each tile once makes every join an exact test on the used set. Approximation saves memory, not time.
3. Row scan is within a few bits of the best of everything named: static orders, row-shaped dynamic rules, restarts, edge-first and tile-first formulations.

### 9.4 Still open, with the reason

| Item | Why it is open | Cost | Program's odds of 10 bits |
|---|---|---|---|
| D1 frame cores (RQ1) | running, results after 22:00 | paid | 5% |
| Conflict-driven and cardinality solvers at 7x7 to 9x9 (gaps 1 and 4) | the only evidence is two sizes and flat | a few core-hours | 1% |
| The injective-only hole (gap 6a) | an object with one term per solution always exists; no model says whether it can be built without the solutions | half a day of desk work, no experiment | cannot be priced |
| Second reader for the 2-bit sketch bound (gap 2) | one derivation, weakly toy-checked | one study | not a gain; a proof |
| Certificate that row scan is the bottleneck optimum (gap 6b) | annealing only | one study | not a gain; a proof |

### 9.5 Corrections to earlier sections

- Section 2, last row and RQ6: row scan is ahead of other static orders by about 9.9 digits in the model; the gap 6b second analysis adds that no static order found beats it by more than 0.24 bit.
- Section 3, O1: the join costs 2^131.6, so the window between the cheapest exact method and row scan is 3 bits, not 8. The merge factor is now measured on the real tiles: 1.0000 to 1.0003 on twelve regions of up to nine cells, about 0.033 bit with non-local exchanges in the model.
- Section 3, closing paragraph: "6 to 7 well-placed tiles would cover 56 to 66 bits" was a model figure for additive, well-placed pins. D3 and D5 measure sub-additivity (0.88 at k = 8 spread, about 0.75 for touching pairs) and D3 brackets a 10^15 run at more than 10 spread pins.
- RQ2 as designed could not fail on frames sharing a 44-cell arc (gap 1 memo); the gap 1 second analysis says that is too strong, but the replacement test (cores at depth 113 from a clause-learning solver, about 5 core-hours) is the one to run if RQ2 is ever run.
- Section 5: Paper 2 now has its material. The table of field, tool, price and reason gains seven independently re-checked rows.

### 9.6 Stopping rule

Stated at 12:15 and extended at 13:04: the hunt for a method stops on D1's judgment and the seven independently re-checked gap memos, unless one of them prices a mechanism at 10 bits or more with a falsifying experiment under a day. The gap memos did not. D1 is the last condition.
