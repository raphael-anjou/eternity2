# Unplanned observations, night 2026-11-12

Leads for the morning, not results. Collected from runner and judge reports and
from the lead's own notes. Nothing here was acted on tonight unless it says so.

## From Raphaël, 18:30

- **The explored map may be wrong in places.** Until this week's rewrite the
  GPU page presented GPUs as a dead end when progress was available. The idea
  slot's novelty checks treat EXPLORED.md and the dead-ends page as ground
  truth, so a wrong "dead end" there silently kills good ideas. Acted on
  tonight: the tournament judge grades every dead-end claim its ranking leans
  on as measured, reported or argued, and only a measured stop can decide a
  pairing (section 5 of `ideas-raw/TOURNAMENT.md`). Follow-up for the morning:
  an audit of the dead-ends page and EXPLORED.md by evidence grade, starting
  with the entries graded reported or argued. EXPLORED.md cannot be edited
  during a night shift.

## From the idea slot (scouts)

- **The no-good page is richer than the explored map says.** EXPLORED.md
  compresses it to "no-goods outrun memory at 16x16". The page itself
  (`build/reduce/nogood-learning.mdx` lines 181 to 248) has a failure lookup
  table keyed on outer colours (Mocsi) and deletion-minimised unsatisfiable
  cores. Two inventors working from the map missed it.
- **`dead-ends.mdx` line 272 marks an untested idea inside a dead-end entry:**
  "Patch databases indexed by frontier colors and remaining pieces are untested
  here." Three proposals tonight land on that sentence.
- **The meet-in-the-middle bulge model is uncommitted** (`meet-in-the-middle.mdx`
  line 241): the argument that multi-way splits collapse is reasoning, with no
  committed measurement.
- **The break-tolerant engine has no score bound.** `engine/src/break_solver.rs`
  lines 15 to 24 allow one break per fixed position and track only the breaks
  used.
- **A partial-match instrument half exists.** The scaling-ladder lab page has
  planted boards at N = 8 to 14 with a proven ceiling and a verified score, on a
  fixed 22-colour palette with a 12 s budget.

## From the rehearsal (18:13 to 18:17)

- `runner.md` points at `research/program/PROTOCOL.md`, which does not exist;
  the file is `research/experiments/hard-ladder/PROTOCOL.md`.
- With 3 seeds the bootstrap of a median takes only 3 values, so a "CI below 0"
  only says all three ratios are below 1.

## From the idea tournament judge (18:38)

- **The site's evidence labels overstate some results.** `dead-ends.mdx` labels
  as "Measured" results that have no committed data: local cluster enumeration
  (near line 356) and Zajc's 2008 macro-tiles (near line 757). BANDSAW's "about
  20x memory per extra mismatch", which EXPLORED.md gives as where meet in the
  middle stopped, is uncommitted by its own page (`bandsaw.mdx` lines 108 to
  111). Of 21 explored or dead-end claims the ranking leaned on: 7 measured,
  8 reported, 6 argued.
- **6x6 c=5 is too easy to separate methods** (about 951 nodes), yet almost
  every proposal, and three of tonight's tracks, test it. A factor of 2 there is
  noise in wall clock.
- **The ranking criterion in the brief is biased toward node-count ideas**,
  because their first tests are the cheapest. Four of the top five do not bear
  on 470 against 480.
- **No node-count proposal preregistered a growth stage**, although the
  protocol proposal says only growth with n bears on the 16x16.
- **Inconsistent baseline number in circulation:** 7x7 scb is cited as 10/30
  (A15 entry) and as 16/30 within 1e6 nodes (hard-ladder README). Both appear in
  AGENDA.md; they are different caps or counters and should be labelled.
- **The Elo passes add nothing to a full round robin**: the rating order equals
  the win counts. Win counts are enough next time.
- **Lead L1's stage 1 was vacuous** ("prove no board scores optimum + 1" is true
  without search on a planted board); the judge replaced it with work to reach
  optimum minus k.

## From Track 3's runner (19:02) and judge (19:16, numbers recomputed by the judge)

- **Value order has a large run-seed spread at 8x8 c=10, and bb-mrv's index
  order is one untuned draw from it.** One seed of the random-value-order
  control (mrv-rand-s3) passes G1 against bb-mrv (0.495, CI [0.409, 0.768]);
  another (s1) sits at 1.623. A single-seed gain from any value-order change at
  that rung is not evidence. Consequence for A22: the adopted baseline's node
  count there is partly luck of its value order.
- **At 7x7 c=6 every random-order control solves more than bb-mrv** (25 to 27 of
  30, against 23).
- **bb-mrv against scb at 6x6 c=5 on all 30 seeds passes G1** (0.468, CI
  [0.318, 0.895]) while being slower in wall clock (1.373). A22 reported 0.80
  on seeds 21..30 only. At 7x7 it is 0.976 (fail), wall 2.807.
- **Judge's wording of the consequence:** a G1 pass against a single fixed-order
  baseline is not evidence for any ordering method unless it comes with controls
  over several random seeds. Worth adding to PROTOCOL.md as a rule (cannot be
  edited tonight): any arm that changes search order needs a run-seed control.
- **Runtime estimates disagree** across README, justfile and PREREG of Track 3
  (about 45 min, 1 h, 1.5 h). Cosmetic.

## From Raphaël, 20:02: long runs give no partial information

- **A capped run tells us only "more than the cap".** Ladder rows are written at
  the end of a run and carry nodes, backtracks and propagation counts, no depth
  and no intermediate state. Track 1's heartbeat logs elapsed and CPU time only.
  Three additions would fix this for the next long runs, none of which can be
  made tonight without restarting runs that are an hour in:
  1. a progress record every N nodes (nodes so far, deepest placement reached,
     current depth), written by the run wrapper, so a live run can be read;
  2. the fraction of the search tree already closed, computed from the DFS
     stack (at each level, branches finished over branches available). It is a
     real progress bar toward exhaustion, though not toward a first solution;
  3. a saved search stack at the cap, so a capped run can be continued to a
     higher cap later instead of being rerun from zero. The search is
     deterministic, so a continued run equals a longer one node for node.
- For Track 1 tonight the only live estimate is CPU time times the measured
  speed: the one finished run did 9,970,108 nodes in 794 s on machine-b, about
  12,600 nodes/s under load, so a run that does not solve reaches the 1e8 cap
  after roughly 2.2 hours there.

## From Track 4's runner (20:22) and judge (20:41, numbers recomputed by the judge)

- **At 7x7 c=6 all three random-value-order seeds beat bb-mrv** (G1 0.881 to
  0.906, 25 to 27 of 30 solved against 23, KM medians 324k to 397k against
  588k nodes). None is significant alone; all three point the same way. A
  second, independent run of the same pattern Track 3 saw: bb-mrv's fixed index
  value order looks like a poor draw, and a randomised or restarted value order
  may be a better baseline.
- **Why A17's 10% gain vanished:** it was measured against scb, which has no
  piece all-different. bb-mrv's existing filter already catches almost every
  Hall violation (94,720 extra failures in 11.2M checks at 7x7, cutting only
  small subtrees). A Hall follow-up would need Régin filtering, not this check.
- **scb is about 2.9x faster than bb-mrv in wall clock at 7x7** while solving
  fewer boards (16 against 23). Which is "the baseline" at 7x7 depends on
  whether nodes or time is counted.

## From Track 0's runner and judge (20:10, 20:30)

- **The 5x to 6x faster bb-mrv changes what a long run costs.** A 1e8-node run
  that took about 2 hours takes about 25 minutes, so the 30-seed 9x9 follow-up
  that Track 1 pilots is cheaper than assumed, for the bb-mrv arm only (scb is
  not sped up, and becomes the slow arm).
- **Identity at 9x9 is only checked on capped runs.** Any track that reports
  9x9 node counts from `bb-mrv-fast` should rerun the hash gate on a few of its
  own instances at its own cap.
- **The speed-up does not grow with board size** (6.1x at 7x7, 5.3x at 6x6 and
  8x8). The runner guesses state copying and arc consistency take over at 8x8;
  not profiled.
- **Absolute nodes/s from tonight should not be quoted**: the original did
  about 28k nodes/s at 8x8 on an idle machine and 21.5k during timing. Ratios
  held because arms alternated.
- **Third independent statement of the same finding (Track 4's judge):** value
  order alone moves 8x8 c=10 node counts by about 2x, and at 7x7 the random
  orders solve 25 to 27 of 30 against bb-mrv's 23. A22's adoption of bb-mrv at
  8x8 c=10 was judged on one fixed order. Follow-up worth a preregistration:
  bb-mrv with randomised value order and restarts as the baseline candidate,
  over at least 5 run seeds, at R7 and 8x8 c=10.
- **A judge can establish run order only up to "the last write".** When a
  binary exists before the PREREG commit, an earlier look at test seeds cannot
  be excluded from file times. A run wrapper that appends a line (time, git
  HEAD, command) to an append-only log before every test-seed run would close
  this.

## From Track 2's runner (20:53) and judge (21:08, numbers recomputed by the judge)

- **Under full propagation a dead end usually has no direct blocker.** In about
  a quarter of dead ends (0.24 at 6x6, 0.29 at 7x7) no placed piece directly
  blocks the emptied cell: it was emptied through cells nobody has decided yet.
  The "evict the blocker" rule from scheduling then has nothing to act on. This
  is a structural reason conflict-directed backjumping without explanations
  fails here: the conflict set is not local.
- **Evicting the direct blocker cycles; random eviction does not.** The blocker
  rule revisited one decision set up to 112,545 times and cycled on 5 to 9 of 30
  runs at 7x7; the random twin never cycled. The control matched or beat the
  candidate on several measures.
- **bb-mrv against scb at 6x6 on all 30 seeds passes G1** (0.468), a third
  independent rerun of that number tonight (Tracks 3, 4 and 2), against A22's
  0.80 on 10 seeds. A22's "G1 fails at R6" was a small-sample result.

## From the literature memo (21:05, `memos/literature-2026.md`, not yet reviewed)

- **A live volunteer project reports 466 under five clues.** The BOINC project
  at boinc.eternityathome.org shows "466/480, 5 official clues", four boards at
  466 dated 7 to 12 November, 259 boards at 465 or better in one week, 172
  computers and "1.1 T nodes/second" with CPU and GPU applications (page read
  by the lead on 2026-11-12). It is a dashboard report: validation method and
  node definition are not stated. If the boards hold, `/research/records`
  (strict five-clue 465) and the open problem "strict five-clue 465 to 466" are
  stale, and the "distributed farms: saturated" row of the map is wrong. Needs
  the boards themselves, verified in the site's viewer.
- **Weighted CSP has a propagator made for this problem shape.** toulbar2
  gained an all-different with linear-assignment bounds (Sewa et al., AAAI
  2026, metadata verified, content from a summary). Nobody has published a
  weighted CSP, MaxSAT branch and bound or decision-diagram run on edge
  matching.
- **A proved null model exists and the site does not cite it:** random jigsaw
  thresholds (Martinsson, arXiv:1701.04813) and a planted-jigsaw analysis
  (Edmonds, Edmonds, Mertz) that proves fill order changes the search exponent.
- **Two bibliography errors on the site** (`PapersView.tsx`): `rotations2017`
  (arXiv:1703.09421) is by Ebbesen, Fischer and Witt, not Bosboom and Demaine;
  `kovalsky2014` carries the wrong title (the paper is "A Global Approach for
  Solving Edge-Matching Puzzles", arXiv:1409.5957).
- **Verification caveat from the memo itself:** about a third of its 72 sources
  were confirmed through Crossref metadata only, because publishers refused
  automated access.
- **Judge's scope note on Track 2:** what ran is a repair search, not
  backjumping. In 24% to 31% of dead ends there is no direct blocker and the
  arm falls back to chronological eviction, so the mechanism as tested is about
  70% blocker eviction. A fair test of non-chronological backtracking here
  needs conflict sets built from propagation explanations.
- **Both eviction arms are partly restart methods:** the tabu list is cleared
  every time no decision is left standing (4,169 times for the blocker, 10,922
  for the twin over 30 runs at 7x7).

## From the growth-law memo (21:10, `memos/growth-law.md`, REVIEWED 21:34: its first claim is WRONG, see the review section below)

Claims, not results, until the referee reports (`memos/growth-law.REVIEW.md`).

- **No rung the program calls critical is critical.** With the frame's entropy
  counted, a whole generated board has one expected solution at 3.9, 5.3, 6.6,
  7.9, 9.3 colours for n = 6 to 10, against the interior-only values the ladder
  uses (5.2, 6.4, 7.6, 8.7, 9.9). Every ladder rung would then be 0.75 to 3.4
  colours above critical, while the real 16x16 sits at critical. With 5 frame
  colours no small board matches the 16x16 on both the colour ratio and the
  solution count; about 3 frame colours at n = 6 to 8 would.
- **Plain row-major DFS is predicted by a first-moment model with no fitted
  parameter** (within 0.84 to 1.9 on 14 rungs). The same memo's model of
  propagating solvers failed by 10 to 200 times.
- **bb-mrv's node lead over plain DFS does not grow with board size** (12, 14,
  7 at n = 6, 7, 8 on the +2 track). A22's "gain grows with n" would be an
  artefact of scb's border-first order.
- **Only two families change the exponential rate in the model:** merging
  states that share a frontier (blocked by piece distinctness) and two-sided
  search (about 6% lower rate at critical, memory equal to time). Local
  propagation and ordering would give a surface factor exp(O(n)).
- **Its own blind prediction failed:** bb-mrv at 9x9 c=11 predicted median
  2.3e7 nodes, measured above 1e8 on partial pilot data (2 of 6 solved).

## From the theory referee (21:20, `memos/theory-referee.md`, recomputed with scripts, not yet double-checked)

30 theory pages refereed: 8 sound, 14 sound with a caveat to add, 3 overstated,
3 wrong, 2 unverifiable from the repo. No page was edited.

- **`why/entropy-area-law.mdx` is wrong on its headline number** (lines 167,
  257, 260): the entropy limit is not 0.67. Recomputed h(1) to h(6): 1.6645,
  1.0497, 0.8449, 0.7425, 0.6810, 0.6400. The page's own theorem makes the limit
  an infimum, so it is at most 0.6400; the row increments converge to about
  0.435. The 0.67 is the width-5 value read as the limit. Its "collapse at 155
  cells" (lines 181, 193 to 195) is about a factor of three too large: the two
  committed exact points fit a birthday law that reaches 1e-3 near 46 cells.
  Both numbers are repeated on `walls-and-methods.mdx:79` and `index.mdx:95`.
- **`why/parity-defect-floor.mdx` lines 111 to 135** (badged proven): "at most
  76 one-move near misses" is wrong in count and in direction. Near-twin pairs
  (125 up to rotation) and in-place turns (29) give at least about 154 boards at
  478 from any solution. The 479 theorem itself is correct. Repeated on
  `theorem-sweep.mdx:121`.
- **`why/how-hard-is-this-instance.mdx`** puts the one-expected-solution
  crossing at N = 12 to 13 for 17 interior colours by dropping the 4(N-2) seam
  joints between frame and interior; with them it is at N = 16, as
  `phase-transition.mdx` says. (The growth-law memo found the same omission in
  the ladder's "critical" colour counts, independently.)
- **Two statistics measure the stored orientation of the CSV, not the piece
  set:** the "307 of 480 fixed-orientation cap" (402 +/- 9 under random fixed
  orientations) and the "114 near-twin pairs" used on three pages (125 up to
  rotation).
- **Overstated:** `sigma-cycles` (a random class-preserving permutation gives a
  largest cycle of median 121; the corpus median is 119, so the headline has no
  null model), `design-recipe`, `why-e2-is-hard`.
- **Reproduced exactly:** 14,702.27 expected solutions with the starter, 4.1e-8
  accidental solutions with five clues, the frame count (three methods agree:
  10^37.59 to 10^37.71), flux ranks, the forbidden-pattern table, the frame
  ANOVA.
- **Theory that could guide search (referee's view):** the depth profile of
  `complex-theory` (predicts 99.5% of nodes beyond depth 132, measured 99%); the
  phase line c*(N) for choosing benchmark boards; a census-exact completion
  count as a node score.
- 15 cross-page disagreements are tabled in the report's section 3.

## From solve path 2, "the instance was made by a person" (21:18, `memos/solve-path-2-designed-instance.md`, not yet reviewed)

- **A negative result with one narrow loophole.** If the maker's painter
  depends on the solved board only through its piece set, the piece set carries
  zero bits about the arrangement (stated as a theorem). Twelve piece-level
  statistics on the official set against 30,000 matched null boards all sit
  inside the null (largest |z| 1.37). The piece numbering is a pure sort.
- **The loophole:** a painter that balanced colours over coarse regions
  (halves or quadrants) would be invisible in the piece set and would carry
  29 to 118 bits. The memo puts quadrant balancing at about 1%.
- **A price, from an exact class-aware first moment:** row-scan search with
  five clues is about 10^45.3 placements. A year on three desktops affords
  about 2^57, a gap of 93 bits (28 orders of magnitude), or ten more correct
  placements known in advance. Its probabilities of a full solution by this
  route: 5e-5 on three desktops in a year, 1e-4 with 1,000 core-years.
- Its first-moment model gives 17,130 expected solutions with the starter
  against the site's 14,702 (the referee reproduces 14,702.27): to reconcile.

## From solve path 5, "the price of brute force" (21:25, `memos/solve-path-5-price.md`, not yet reviewed)

- **A price, with a range.** Model: Owen's complex theory in exact arithmetic,
  reproducing his published table. First solution with five clues: about
  1.5e40 nodes (range 1e39 to 1e42), 5e24 core-years at 1e8 nodes/s per core.
  Exhaustion with five clues: 3.1e40. One clue: 9.3e42 to a first solution,
  1.4e47 to exhaustion. The five-clue route is about 600 times cheaper, so the
  14,702 one-clue solutions do not help.
- **Calibration:** on exact enumeration of generated 4x4 to 6x6 boards and the
  60 committed 6x6 ladder instances, real trees are 1.1 to 2.0 times the model.
- **The gap:** three desktops with GPUs for a year reach about 1e18 nodes (22
  orders short); the BOINC project at its reported rate about 3.5e19 a year
  (20.6 orders); 1e5 core-years 3.2e20 (19.7 orders). Credible multipliers are
  worth about 2.6 orders together (subtree ranking by predicted solutions per
  node 25 to 150, reported by McGavin; per-core tuning about 2; GPU per euro 1
  to 3; propagation, Hall pruning, symmetry, meet in the middle and restarts at
  1 or below with evidence). Remaining gap: about 17 orders at 1e5 core-years.
  In the model that equals 56 bits about the solution, or six certain
  placements ahead of depth 160.
- **Fill order is a spent lever in the model:** row scan is the best of ten
  families and a hill climb found nothing below it.
- **Proposed first experiment, under one core-hour, NOT run (it is a 16x16
  measurement, which tonight's rules forbid without Raphaël's word):** estimate
  the real puzzle's tree width per depth by sequential Monte Carlo (1e6 partial
  boards resampled per cell, five clues, five seeds). Peak within 0.5 orders of
  1e39.2: search-based solving is closed on record. Peak 3 or more orders
  below: the designed set is atypical and the curve shows where, the first
  exploitable structural fact. Plain Knuth probes will not do (coefficient of
  variation 12.9 on a 6x6).
- **Site error found:** "4.9e32 years at 100M nodes/s" for the five-clue path
  (`going-fast.mdx` and the McGavin page); 3.1e40 nodes at that speed is about
  1e25 years.
- **To reconcile:** solve path 2 prices row-scan search with five clues at
  about 10^45.3 placements, five orders above this memo's 3.1e40. Two
  independent models, two units (placements, nodes); the merge must settle it.

## From Track 6's runner, the dead-node census (21:14), and its judge (21:26, numbers recomputed by the judge)

- **Small-region reasoning proves very little of what matters.** Weighted by
  subtree size, at 7x7 c=6: cavities up to 3x3 against the unused pool prove
  0.8% of dead nodes dead, up to 4x4 6.1% (15.7% with propagated domains);
  band consistency over 2 and 3 rows 0.2% and 0.8%; exact pockets 3.5%;
  partitioned propagation about 10%. Unweighted shares are much higher (up to
  67%): the checks catch small subtrees, which are cheap anyway. Every check
  proves almost nothing at shallow depth and almost everything deep, where the
  search is already cheap. This is the "signal fades" claim measured.
- **Where the key is does not matter; that it is an edge colour does.**
  Partitioning on two edge colours beats partitioning on a cell's values at
  matched budget (0.103 against 0.063), but a random pair of edges does as well
  as the cut pair.
- **Boundary keys recur often (46%) but a stored refutation is rarely reusable
  soundly (13.5%)**, and most of the cavity signal comes from propagated
  domains, which a table keyed on boundary and pool cannot hold.
- **The colour-only check is ten times stronger above critical than at it**
  (20.8% against 1.7%): a second, independent sign that "critical" and
  "above critical" boards differ in kind, not degree.
- One mechanical kill (cavity criterion K0c) fires on a tie between two
  infinite medians; its judge is asked whether that stands.
- Operational: the repository's secret scanner refused a commit because hex
  values under `key_*` fields looked like leaked keys; fields renamed `sig_*`.

## From Track 9's runner, resumable long runs (21:20), and its judge (21:28)

- **A 9x9 search closes almost none of its tree in 3 million nodes:** 0.06% to
  1.1% closed, against 6% to 14% for 7x7 runs capped at 1e6. A 1e8-node 9x9 run
  that hits the cap has barely started, which is what "censored" hid.
- **Checkpoints are tiny and cheap:** 1.3 to 2.3 KB, about 1 ms to write, under
  1.5 ms to resume by replaying decisions from the root.
- **Operational, seen twice tonight:** `~` inside a quoted `rsync -e "ssh -F
  ~/.ssh/..."` is not expanded; use `$HOME`. To add to the runner definition.
- **A silent failure to fix:** if the progress file's directory does not exist,
  progress records are dropped without an error.

## From solve path 3, "change the exponent" (21:19, `memos/solve-path-3-exponent.md`, not yet reviewed)

- **Negative: no decomposition in this family changes the exponent.** The best
  exact structure, a two-list seam join keyed on (piece set, seam colours),
  gains 0.8 digits over plain scan-order search, because each list must itself
  be generated by a search that climbs its own rising flank. Quadrants, join
  trees, frontier dynamic programming with state merging and colour-count
  signatures fail by the same count: some node of any join tree covers a third
  to two thirds of the board, with a list of at least about 1e31.
- **A second, independent price that agrees with path 5:** plain search with
  five clues 10^40.5 nodes (path 5: 3.1e40), peak 10^39.2 at depth 161. The
  model reproduces 14,702 and the committed frame count (10^37.71 against
  10^37.608). Path 2's 10^45.3 is the outlier of the three.
- **The ledger: the cost is almost all "which piece goes in which zone".** Given
  the correct frame, the interior costs about 10^14.8 nodes (about 70
  core-days). Given the correct split of pieces between top and bottom halves,
  each half peaks at 10^9.9. Checked on small boards: the frame-given and
  split-given collapses are real (6 to 10 orders below free search at 7x7 and
  8x8). What would remove most of the gap: ranking the true frame within the
  first 10^3.7 of the 10^37.6 valid frames, or 8 more true clues in rows 1 to 6
  (73 bits).
- **The site's meet-in-the-middle page:** its verdict holds, but its numbers are
  about 20 orders too high and its corner-anchoring argument is contradicted by
  a hand count.
- **Where the real puzzle sits in the published theory:** in Edmonds, Edmonds
  and Mertz's many-solutions regime (4n/q^2 = 2.71), not their easy
  width-1 regime; on Martinsson's lower threshold (16.98 against 17 interior
  colours).
- Its own two interface-state formulas were wrong on small palettes (state
  merging is a factor 2 to 4, not 1); it says neither changes the conclusion.
- Probabilities of a full solution by this route: 1e-6 on three desktops in a
  year, 1e-5 with 1,000 core-years.

## From solve path 1, "exact counting and algebra" (21:19, `memos/solve-path-1-counting-algebra.md`, not yet reviewed)

- **Negative: nothing in this lens is closer to a solution than depth-first
  search.** Colour-only contraction needs 10^19.85 frontier states and counts
  about 10^107.65 reused-piece boards per true solution (a factor e per cell).
  Inclusion and exclusion needs 2^256 contractions. An exact transfer matrix
  that carries the used set is breadth-first search.
- **The overcount of e per cell belongs to the puzzle:** every method either
  pays it, or tracks the used set and becomes the search tree. This is the same
  wall the other memos reach from different sides.
- **The "Lagrangian patch" did not reproduce.** The dead-ends page says a
  per-piece Lagrangian solved generated 4x4 and 6x6 boards. This memo's
  reconstruction solved 0 of 19 toy boards, and on 17 of 19 no price vector can
  isolate a true solution at all (LP margin 0). It may differ from what the
  notebook ran, and that run is uncommitted, so the page's claim stays
  unverified in both directions. It was one of the audit's "live direction"
  candidates.
- **Consequence for bounds:** a bound built on colour-only counting with the
  exact remaining pool cannot reject a partial board before depth 144 to 150,
  where the tree is within one order of its peak (from a first-moment profile
  with an assumed rescaling). This predicts Track 8's result.
- Probabilities of a full solution by this route: below 1e-7 on three desktops
  in a year, below 1e-6 with 1,000 core-years.

## From the partial-match memo (21:24, `memos/partial-match-regime.md`, REVIEWED 21:45)

The claims as the memo made them. The referee's rulings follow in the next section and override these.

- **How many boards exist at each score** (expected, no clue pinned): 10^34.4 at
  470, 10^31.7 at 471, 10^28.9 at 472, 10^12.3 at 478, none at 479 (parity),
  10^7.1 at 480. Each point costs a factor 10^2.7 in boards.
- **The price of the next record for the record method:** one break fewer
  multiplies the nodes per board found by 10^1.9 to 10^2.4. So 471 costs 80 to
  235 times what 470 cost, and 472 about 10^4 times. The model's absolute cost
  is about 6 decades above reported compute for 469, attributed to the record
  engine's colour quota, so the ratios are firmer than the levels.
- **The hand-made break schedule is worth 22 decades** (breaks anywhere: 10^45.4
  nodes per board; breaks on a late unlock list: 10^23.3).
- **A score bound has little to remove.** A bound that drops piece distinctness
  is loose by about 0.2 break per remaining cell and bites only 40 to 56 cells
  from the end, where the schedule already unlocks. An ideal one is worth at
  most a factor 5.5, about 0.4 point. Blind prediction for Track 8: median gap
  0 to 1 edge at one row left, 2 to 3 at two rows, 3 to 5 at three rows.
- **No published weighted-CSP, MaxSAT, decision-diagram or dual-decomposition
  result on edge matching in maximisation form.** The structural match is
  quadratic assignment, where exact methods stop near 30 variables; the 16x16
  has 256.
- **What it would build instead of a bound:** a first-moment gate computed from
  each node's own remaining pool (an adaptive schedule), colour staging from the
  same model, parity as the only sound per-node bound, an exact last-row solver.
- **What it says the program is missing most:** a measured histogram of where a
  record-style engine spends its nodes, by depth and breaks spent, against a
  calibrated model. Every 470-regime idea assumes a factor exists to remove
  without knowing where the work is.
- **Judge's rulings on Track 6:** three stage-0 kills stand (cavity summaries,
  band consistency, boundary-word lemmas). The kill of cut-keyed partitioning
  does not stand: its random control used a median of 16 partitions against the
  cut arm's 6, so it was not a matched control, and a one-shot share is not the
  proposal's replay ratio. A lesson for the kit: a mechanical verdict issued
  through a proxy needs the proxy named in the PREREG as a proxy.
- **Not measured by the census:** a band filter iterated with propagation down
  the search (the band filter removes about 10% of band-cell values at a node
  even when it empties nothing), and anything on scb trees.
- **Judge on the progress measure:** the closed share must not be read as work
  done, a time estimate or a chance of finding a solution: one 7x7 instance
  solves with 0.24% of its tree closed.
- **The first judge to read a guard log found the guard's own gaps** (no run
  id, no end line for a killed run, no record of uncommitted code): fixed in
  the guard the same hour. A killed run still leaves a start line with no end
  line, by construction; the judge definition says to read that as a kill.

## From solve path 4, "the last ten edges" (21:29, `memos/solve-path-4-last-ten-edges.md`, not yet reviewed)

- **A 470 board is not a damaged solution.** By first-moment counting, of about
  10^33 boards at 470 only about 10^9 per solution lie within a few cells of it
  (a share of 10^-20 to 10^-30). The rest sit 246 to 254 cells of 256 away from
  every solution. Record hunting and solving are different problems.
- **An empty band:** no board with 10 or fewer mismatches is expected at 65 to
  160 changed cells from a solution. An expectation over the model, not a proof
  for this instance.
- **Rigidity says nothing about where solutions are:** the same counting
  predicts all 20 committed SAT halo verdicts (10^-44 to 10^-54 expected
  perfect refills), so "record boards are locally optimal" is the null
  expectation.
- **Exact neighbourhoods around a 470 board search a set that is empty in
  expectation** until 160 to 190 cells are freed for 471, and about 250 for
  480. That covers AGENDA A9 (k-best checkerboard assignment), A14 (Hamming
  balls) and A6 (colour chains). Checked on the five public record boards: the
  exact checkerboard optimum is the incumbent, 0 cells moved in all ten
  refills. Checkerboard assignment itself is published (Schaus and Deville
  2008; Salassa et al. 2017, arXiv:1709.00252).
- **A third price that agrees:** 10^35.6 to 10^40.7 nodes for the five-clue
  tree; gap 17 to 22 orders at 1,000 core-years.
- **Its odds:** full solution 1e-6 (three desktops, a year) and 3e-6 (1,000
  core-years); a verified board above 470, 3% and 35%, resting on a guess that
  470 cost about 10 core-years with about 230 times more per extra edge.
- **The A10 kill does not transfer to 16x16:** the model reproduces its 5x5
  medians and predicts the sign flips between 8x8 and 12x12.
- **A site claim to rerun:** the rigidity page says 218 freed cells were refuted
  in 0.31 s, while the committed data shows 1,193 s at 141 freed cells.

## Review of the growth-law memo (21:34, `memos/growth-law.REVIEW.md`)

The referee recomputed everything; its own row-major DFS matches 159 of 159
committed `hp-dfs` rows node for node.

- **REFUTED: "no ladder rung is critical".** The memo's first moment assumed
  independent uniform colours; the generator gives exact colour counts and a
  ring whose seams pair a clockwise side with an anticlockwise side. By
  enumeration, 6x6 c=4 boards (which the memo put at critical) have 357
  solutions on average. Corrected critical colour counts: 4.65, 5.99, 7.32,
  8.63, 9.94 for n = 6 to 10. The ladder's primary rungs are within 0.7 colours
  of critical, as the ladder said. The recommendation of 3 frame colours falls
  with it. The corrected formula reproduces the site's 14,702 exactly, and
  gives 1.15e7 expected solutions with no clue and 3.9e-8 with five.
- **HOLDS: a parameter-free model predicts plain row-major DFS** within a factor
  2.1, always on the high side, on every above-critical rung from 6x6 to 9x9,
  including two rungs tested out of sample. Unusable below critical.
- **HOLDS: bb-mrv's node lead over plain DFS does not grow with board size**
  (paired medians 9.9, 15.7, 5.4 at n = 6, 7, 8 on the +2 track; about 2 at
  n = 9 on 4 informative pairs).
- **NEW AND IMPORTANT: plain row-major DFS solves 9x9 c=11 in seconds.** KM
  median 1.56e8 nodes, 18 of 30 solved under 2e8, at 2 to 6 s per instance.
  Tonight's pilot took 6,772 s for one bb-mrv solve of 7.8e7 nodes. bb-mrv
  saves about a factor 2 in nodes there and costs about a thousand times more
  per node. The ladder's node metric hid this: PROTOCOL.md counts assignments,
  not work. A17 had already noted it at 8x8 c=10 (plain DFS at 26 times lower
  wall time). Any 9x9 or 10x10 follow-up should use plain DFS, or the fast
  solver from Track 0, as the reference, and compare on work or time.
- **NOT ESTABLISHED: "propagation and ordering are only a surface factor".**
  Provable for a static order with bounded lookahead only, and at n up to 16 the
  surface allowance is as large as the bulk, so it forbids nothing measurable.
- **WRONG AS STATED: "reasoning that relaxes distinctness cannot refute a node
  before the peak".** Measured: a distinctness-free check of the next two rows
  refutes 37% (6x6 c=5) and 57% (7x7 c=8) of nodes more than one row before the
  peak. The weaker form (it cannot change the bulk rate) is given 0.8.
- **NOT ESTABLISHED: A22's trend as an artefact of border-first order.** At
  n = 8 border-first accounts for a factor 1.8 of the 6.0 and the missing piece
  all-different for 4.4.
- **HOLDS (algebra): two-sided search** gains only below a colour ratio of 1/2,
  and never beat first-solution DFS on any enumerated board. "Only two families
  change the rate" is a limit of the model, not a result.

## Review of the partial-match memo (21:45, `memos/partial-match-regime.REVIEW.md`)

A separate Fable session recomputed every number with its own scripts
(`memos/partial-match-regime-review/`, about 5.5 minutes of one core, no 16x16
solver run). Grades are the referee's.

- **HOLDS: the exact first moment with breaks.** Recomputed by a different method
  from the committed piece set: 14,702.27 solutions with the starter clue,
  4.097e-8 with five. Equal to full enumeration on 2x2, within noise on 3x3 and
  4x4. `model.py` can serve as the null model for counts.
- **HOLDS WITH A CORRECTION: boards per score.** 10^34.4 at 470, 10^31.65 at 471,
  10^28.9 at 472, a step of 10^2.7 per point. "None at 479" is true only for
  boards with a legal grey frame: the site's parity page reaches 479 by flipping
  a border piece (14 such pieces).
- **NOT ESTABLISHED: "471 costs 80 to 235 times 470".** For break lists re-tuned
  to the target the model gives 10^1.7 to 10^2.2 per point, but the figure moves
  from 1 to about 240 with where the engine spends its nodes. Three reported
  score ladders give 17 to 30 per point mid-ladder and 30, 64, 312 at the top
  step, on 11, 4 and 2 boards. Do not quote the 80 to 235 range.
- **HOLDS WITH A CORRECTION: breaks must be late.** Any single late gate beats
  "breaks anywhere" by about 20.6 of the 22 decades; the shape of the list adds
  1.5 to 2.3, not 6.6. A model statement; no real engine was measured.
- **NOT ESTABLISHED, AND THE MOST SERIOUS ERROR: "a score bound has little to
  remove".** The memo explains its 6-decade level error by the colour quota, then
  draws its conclusions about bounds from a tree where 0.09% of nodes come after
  the first unlock. With the quota in the model, the reported 10^15 nodes for a
  469 is matched only if 92 to 98% of nodes are late; there an ideal bound is
  worth 10 to 28 on Blackwood's list, and a perfect bound from depth 201 beats
  the best list by 5 to 9. **The score-bound line (Track 8, IDEAS.md rank 4) is
  not closed.** One measurement decides it: the node histogram by depth of a
  record-style engine on the real puzzle.
- **HOLDS WITH A CORRECTION: literature.** Twelve searches found no
  maximisation-form result on edge matching. "Quadratic assignment stops near
  30" is wrong as worded: exact methods have reached 35 to 42 on dense instances
  and 64 to 128 on symmetric ones.
- **Blind prediction for Track 8: not blind.** 10 of 12 cells fall in range, but
  Track 8's rows were committed 16 to 28 minutes before the memo's last save.
- Smaller errors: the eleventh unlock is assumed at depth 196 while the repo's
  list continues with 241 and 256; "14,790" is a rounding artefact; the colour
  supply test with breaks already exists (`research/topics/ledger-prune`), as do
  colour staging and the last-row solver.
- **Lead's note.** The deciding measurement is a 16x16 run, which tonight's
  limits forbid. It joins the tree-width sampling measurement as a question for
  the morning; neither was run.

## From solve path 6, the free one (21:47, `memos/solve-path-6-free.md`, not reviewed)

- **Approach tried: a difference map (divide and concur) instead of a tree.**
  Boards as real vectors of colour weights, three exact projections (every piece
  once; facing sides agree; bands of k rows tiled exactly with repeats allowed).
  The site's iterated-maps page lists the two-set version as never run on this
  puzzle; the largest published result found for the family is TetraVex 4x4
  (arXiv:1305.2657, opened).
- **Measured, small (8 CPU-minutes, seven instances with a reference):** every
  planted board from 4x4 to one 8x8 was solved exactly. Iteration counts track a
  naive backtracker's nodes (ratio 0.07 to 0.64); with row and column bands the
  geometric mean ratio falls 0.19, 0.079, 0.045 over n = 5, 6, 7. Weak evidence:
  colour counts not on the critical ladder, baseline without propagation.
- **Negative on its own lens:** at the measured ratio the real board needs 10^46
  to 10^47 iterations; 33 to 34 orders short at 1,000 core-years, 28 if the one
  favourable trend is taken at face value. The untested hypothesis that would
  matter: the exponent falling roughly as 1/k with band width (the author gives
  it 2 to 3%). A preregistered test costs under 10 core-hours (k = 0 to 3 on R6,
  R7, 8x8, 30 seeds, kill line written in the memo).
- **A fourth price, and it disagrees:** 10^47.7 nodes per solution for row-scan
  backtracking, against three near 10^40 and one at 10^45.3. The price review
  and Track 10's sampling of the real tree are meant to settle this.
- Stated odds of a full solution: 1e-6 on three desktops in a year, 3e-6 at
  1,000 core-years. Ideas not chosen: recovering the generator's seed (the
  author's only route above 1e-4), cavity-guided limited-discrepancy search.
- One source could not be opened (Elser, Rankenburg, Thibault, PNAS 2007: 403).

## Review of the price and the cost ledger (21:51, `memos/price-and-ledger.REVIEW.md`)

A separate Fable session reran all sixteen memo scripts (all reproduce) and
wrote its own implementation (`memos/price-and-ledger-review/`, about 7 minutes
of one core, nothing run on the 16x16). Grades are the referee's.

- **THE RECONCILED PRICE: 10^40.49 nodes** (3.1e40) to exhaust the row-scan tree
  with the five clues, 1.5e40 to the first solution, range 10^39.5 to 10^41.5.
  A node is a valid partial board with the clues on the board. With one clue:
  10^47.14 to exhaustion, 10^42.97 per solution. Paths 5 and 3 agree to 0.03
  digits. This is a first-moment model, calibrated on small boards only; Track
  10 samples the real tree.
- **WRONG: path 2's 10^45.3.** Its script spreads the full-board match
  probability evenly over joints and applies it to partial boards (exact only at
  depth 256). Corrected in its own terms: 10^40.66 = 2^135.1, so 78 bits short
  on three desktops, 73.6 at 1,000 core-years.
- **Lead's note on path 6's 10^47.7 (WRONG, corrected by the later referee):**
  the lead read it as the one-clue figure under another convention. It is not;
  it is the same script error as path 2. See the review of path 6 below.
- **HOLDS, AND UNDERSTATED: the cost ledger.** With the true frame given the
  interior costs 10^14.1 in row order, 10^13.0 from the other side and **about
  10^8.3 nodes by inward spiral** (model mean over frames). At the row-scan peak
  30 of the 39 digits are partial-frame multiplicity.
- **HOLDS: frame-first search is dearer, not cheaper.** 10^37.6 frames times a
  mean refutation cost of 10^8.3 is 10^45.9, 5.5 digits above row scan. Break
  even needs a mean of 10^2.9 nodes per frame. Refuting a wrong frame costs
  about as much as confirming the right one (all frames enumerated on five 6x6
  boards). The ledger sizes an oracle: 91 bits about the frame are worth 1,000
  core-years. It does not give an order.
- **NOT ESTABLISHED: "ranking is worth 25 to 150".** The 25 was measured on
  unframed sub-solution hunts. The remaining gap is 17 to 20 orders, 19 to 20
  without the ranking factor.
- **NOT ESTABLISHED: "no error beyond 0.3 orders for a random 16x16".** Instance
  spread is 0.15 to 0.37 dex at up to 36 cells; its scaling to 256 is unknown.
- **HOLDS WITH A CORRECTION: no join or merging changes the exponent.** Best
  gain 1.0 digit, at most 2.4 for any band join that stores both lists.
- **HOLDS: the zero-information theorem of path 2** (it is Bayes' rule). It
  rules out design traces, not solution marginals, so it does not retire
  neighbour classifiers.
- **Sampling estimator:** unbiased against exact widths in simulation (1.002,
  1.033), biased low in the log. The memo's variance formula uses the wrong
  term and its 3-order decision rule has no null for instance spread. Sent to
  Track 10.
- A second script error: path 3's `profile()` closes a clue's joints only when
  the scan reaches the clue cell, which inflates its top-down totals and the
  10^14.8.
- Three groups.io messages could not be opened (HTTP 402).

## From Track 1's wrap-up (22:35, not judged yet)

- Per node, the ladder DFS `scb` is faster than `bb-mrv` on every machine at
  9x9 c=11 (about 20k against 14.7k nodes per second on this Mac), and on the 6
  pairs where both hit the cap bb-mrv took 1.12 to 1.55 times scb's wall time.
- Both bb-mrv solves found a solution other than the planted one.
- The earlier runner's fetch loop was still alive 70 minutes after its session
  was gone; the remote scripts' own git commits failed (copied tree, no git).

## From Track 1's judge (22:48)

- **A scoring rule that manufactures a kill.** Counting "both hit the cap" as a
  tie at 0 makes K1 fire whenever the cap is too low for one solver. A follow-up
  needs an scb cap near 1e9 (14 to 23 h per run) or per-solver Kaplan-Meier
  medians. Candidate protocol draft.
- **The judge would not request the 30-seed 9x9 scaling run**: if plain DFS
  solves these boards in seconds (reported by the growth-law review), a node
  ratio between bb-mrv and scb says nothing about solving speed.
- Tracks started before 21:28 have no guard log because they branched before
  the guard existed. Later tracks branch from the lead branch, which has it.

## Review of the literature memo and of solve path 6 (22:54, `memos/literature-and-path-6.REVIEW.md`)

A separate Fable session opened the sources and reran path 6's scripts
(`memos/literature-and-path-6-review/`, 7.7 core-minutes). Grades are the
referee's. With this, every memo of the night except the theory referee and
the frame-information memo has had a second reader.

Literature:
- **HOLDS (reported): the BOINC page shows 466/480 "with 5 official clues".**
  No board can be downloaded and no validation method is given, so 0.6 that the
  boards honour the five clues. Its rate reads "886 G nodes / second", with an
  undefined node; probably the known five-clue fleet under a BOINC client.
- **WRONG: "the record moved this month".** A five-clue 466 has been public
  since at least 2026-10-08 (the memo's own source 10). The site's strict 465
  is probably stale; verify a board before changing it.
- **HOLDS: the toulbar2 AllDifferent paper (AAAI 2026).** NOT ESTABLISHED that
  it reaches our sizes: it proves optimality on 33 of 132 QAPLIB instances
  below size 100, and an edge-matching cell has up to 1,024 values.
- **WRONG: "row-major scanning is optimal for a structural reason".** The cited
  planted-jigsaw manuscript says the opposite of a single row: grow a rectangle
  with both sides at least two.
- **HOLDS: two site bibliography errors** (`rotations2017` authors;
  `kovalsky2014` title, year, venue and blurb: the paper is on edge matching,
  solves 12x12 and names the 16x16). Safe to fix; not edited tonight.
- **NOT ESTABLISHED, no counterexample found:** nobody has published weighted
  CSP, MaxSAT, decision diagrams or pseudo-Boolean on edge matching (0.7); no
  peer-reviewed paper on the puzzle after 2018 (0.75). Write "none found after
  two searches", not "never tried".

Solve path 6:
- **HOLDS: its measurements reproduce**, and its conclusion (the map follows
  the tree at band width up to 1) survives.
- **WRONG: its price 10^47.7 and "33 to 34 orders short".** Same script error
  as path 2 (the full-board match probability spread evenly over seams). Exact
  value of that quantity: 10^42.97. Corrected gap: 26 to 27 orders at 1,000
  core-years. The check on 14,702 could not see the error.
- **WRONG: "instances near the critical colour count".** They were 1.4 to 5.4
  colours above it; nothing above 5x5 was near critical.
- **NOT ESTABLISHED: the ratio to a backtracker (0.05 to 0.6) and its fall with
  n.** Against a smallest-domain-first DFS it is 0.13 to 2.9; the trend is
  confounded with over-colouring and n = 7 is one instance.
- **Pattern worth a protocol line:** two memos made the same modelling error and
  both "validated" it on the solution count, the one depth where the wrong and
  right models agree. A model of a search tree must be checked against an
  enumerated tree profile, not against its endpoint.
- Track 11 was sent the corrections before any test seed (primary rungs near
  critical, bb-mrv baseline, cycle detection, do not quote 10^47.7).
- **Lead's own error:** the brief to this referee gave the critical colour
  counts for n = 5 to 9; they are for n = 6 to 10. The referee caught it.

## From the frame-information memo (23:13, `memos/frame-information.md`, REVIEWED: conclusion kept for the tests it ran, two supporting statements fail, see the last review)

The question: is there any computable information about the frame that costs
less than the search it replaces? Answer: no. About 25 core-minutes of scripts
(`memos/frame-information/`, `01` to `07`).

- **The frame holds about 20 cheap bits, all at the four corners.** On the
  official pieces, exactly: 57,131 valid five-piece border chains per corner;
  the 2x2 clue block is fillable for 2,637 to 3,499 of them, 17.17 bits over the
  four corners at about 10 nodes each. These 17 bits are already inside the
  10^40.49 price. 91 more would be needed to reach 1,000 core-years.
- **Away from the corners a test sees nothing until about 150 of the 191 free
  interior cells are filled**, when the whole per-frame cost is spent. On five
  6x6 boards with every frame enumerated, a sound refuter holds 1.3 to 2.0 of
  11.5 to 14.6 bits at 90% of the per-frame cost; half the bits arrive after
  99.9% of it. The middles of the sides refute nothing (first ring by transfer
  matrix: the full ring and the corner windows pass the same 0.5045 of words).
- **Fast refutation is dead on count alone:** one node per frame is 10^37.6
  nodes, and 10^32.4 frames survive the corner tests.
- **Zero-bit argument (argued):** the multiset of inward colours is the same
  for every frame, so any statistic that ignores their order carries nothing.
- **One small signal:** on planted 16x16 boards of official shape, the true
  border word admits more first-ring fills than shuffled words that pass the
  same corner tests (1.62 standard deviations, s.e. 0.15), about 2 ranking
  bits, a factor of about 4 at best. Null words were shuffled letters, not
  valid frames, and pieces were reusable: weak.
- Proposed test (3 core-hours, kill line written, the author would not run it
  first): border candidates ordered by that count at 9x9 or 10x10. Stated odds:
  bears on a full solution under 0.5%, on passing 470 about 1%.
- **Lead's reading:** the border question is closed with a measurement for
  local and corner tests. Not covered: stronger refuters (clause learning,
  all-different propagation), belief propagation, boards between 6x6 and 16x16.
- Operational: the lead's `git add` of the whole night folder committed this
  session's first scripts while it was still working. Harmless, but sessions
  should own their folders until they report.

## From Track 7's runner (23:21, not judged yet)

- **(Corrected by the judge: one entrant is in band above critical at 8x8 with
  10 colours.) The 2% to 4% stall band appears mostly below the critical colour
  count** on small boards (the in-band rung has 1.1e7 expected solutions). At or above
  critical, and at n = 10 and 12, every entrant stalls further out.
- **A control beats the record families on small boards:** plain annealing and
  parallel tempering beat the break-tolerant DFS and ALNS on 30 of 30
  instances. Two caveats from the runner: `BreakSolver` stops at its first
  complete board and ignores the rest of its budget, so it is not a stand-in
  for the record backtrackers; ALNS started from a greedy board, not a beam
  board as in the committed 16x16 runs.
- **Small boards may not rank methods for the 16x16 at all.** JUDGED: the
  reversal is real in the data (annealing wins 30 to 0 in all three family
  pairs) but the judge rules that it cannot separate implementation from board
  size, and its verdict does not rely on it. Not evidence against small-board
  ladders yet.
- All five entrants were bit-identical across x86_64 Linux and arm64 macOS.
- Operational: a budget truncated to 0 (rows deleted unanalysed, amendment),
  a file written to /tmp on machine-b (moved, deleted), a probe started over the
  thread limit (stopped), two rsync destination slips from a zsh modifier.

## From Track 5's runner (23:23, not judged yet)

- **The adopted baseline at R7 is not the best entrant.** `scb-ad` solves 27 of
  30 and `mrv-rand` 26, against 23 for bb-mrv and 18 for the adopted
  `scb-gacolor_ac3_random`. No G1 pass, screening only.
- **(Weakened by the judge.) Random value order against bb-mrv:** paired ratios
  0.88 to 0.91 at R7 with every interval reaching 1, and 1.62, 0.85, 0.50 over
  three seeds at 8x8. What the data support: bb-mrv's single fixed order sits
  inside the spread of random orders (8x8: 69,036 nodes inside 46,568 to
  107,496), so a bb-mrv number carries run-seed-sized noise that its tables do
  not show. Not: random order helps.
- **Plain row-major DFS at 8x8 c=10** solves 24 of 30 at 1% to 23% of bb-mrv's
  wall time per run, and outranks the former baseline on nodes. Second sighting
  tonight that the node metric hides per-node cost.
- At R6 on 30 seeds bb-mrv passes G1 against the adopted baseline (0.468); A22
  had 0.80 on 10 seeds.
- Several registered entrants are node-for-node duplicates of one another
  (`scb-gacolor_ac3`, `_ns1`, `scb-verhaard_preferred`; four `joe_depth150*`
  presets). The main engine's `spiral-out` and `random` orders solve 0 of 30
  even at R6.

## From Track 7's judge (23:35)

- At n = 10 and 12 every entrant stalls 5% to 12% below the optimum at this
  budget, wider than the real puzzle's 2.08% record gap. Small boards at these
  budgets are not in the regime of 470 against 480.
- Scores vary a lot by instance: one 8x8 12-colour board was solved perfectly by
  parallel tempering while its other three seeds sat 10 to 13 edges short.
- A partial-match ladder worth a second attempt needs a constructive entrant
  that keeps searching after its first complete board, or the 16x16 engines
  made size-generic.

## From Track 5's judge (23:36)

- **A seeding bug in the probcut experiment (A12):** `fc-luby` seeds with
  `(seed ^ hash) | 1`, so run seeds 2k and 2k+1 are the same run. Any
  multi-seed result with that solver has fewer distinct seeds than it says.
  Not fixed tonight (another experiment's code); follow-up.
- **The plain-DFS result at 8x8 is about one cell order:** row-major solves 24
  of 30, column-major 20, five other orders 0.
- **On a wall-clock budget the 8x8 ranking would likely flip** (row-major DFS at
  about 1% of bb-mrv's time per run). A separate measurement; candidate
  protocol draft together with P-series entries on work versus nodes.
- Capped runs overshoot the 1e6 cap by up to 6,715 nodes but are scored at the
  cap. The committed crate lockfile is not the one the build resolves to.

## From Track 10's runner (00:16, real 16x16 pieces, not judged yet)

- **The price model survives its first contact with the real pieces.** Sampled
  tree size with five clues: 10^40.457 (interval 40.440 to 40.473), model
  10^40.49. Centre clue only: 10^47.140 against 10^47.14. No clue: 10^50.039.
- **The official piece set is ordinary** among 20 boards painted from its own
  colour counts (median 10^40.61, sd 0.125; 5 of 20 below it). No sign in tree
  size that the designers made it unusually hard or easy.
- **Late nodes are rare in the record-style engine:** 4.5e-7 of nodes at or
  after depth 201 at 1e10 nodes (rising 1.3 to 1.9 times per decade of budget;
  argued extrapolation 2e-5 at 1e15). The histogram peaks at depth 161. This
  contradicts the partial-match referee's inference that matching the reported
  cost of a 469 needs 92 to 98% of nodes late; one of the two is wrong, or they
  count different things. To reconcile after the judgment.
- **Each node at depth 201 leads to only 35 to 71 more.** Late nodes are rare
  because few zero-break branches reach depth 201, not because the endgame is
  small. So a score bound has little to cut in this engine, and the cost of a
  record is paid in the middle of the board.
- **Sampling cannot see the last 60 cells:** the population collapses at depth
  188 to 204 while the true width is still about 10^35. Totals are unaffected.
- The single-late-gate schedule runs about 30 times slower and did not finish:
  the one schedule where late nodes might dominate is not measured.
- Record boards cannot serve as frame stand-ins: neither honours the clues.

## From Track 8's runner (00:28, not judged yet)

- **The score bound loosens as boards grow.** Gap at 2 rows left: 1, 2, 4 edges
  at 6x6, 7x7, 8x8; per remaining cell 0.083, 0.143, 0.250; share of the
  trivial gap closed 86%, 75%, 60%. With 4 or more rows left it is within one
  edge of the trivial bound at the critical colour counts only (judge: 2 to 3
  edges of difference above critical). Nothing supports a tight bound at 16x16.
- **A pure counting bound (width 1) adds almost nothing over no bound.**
- Together with Track 10 (4.5e-7 of the record engine's nodes are late), both
  halves of the score-bound question now point the same way: the bound is
  loose where it would act, and little of the search is there. IDEAS.md rank 4
  (decision-diagram score bound) should be marked not worth a build, pending
  the two judgments.
- Scope limit from the runner: the sampled prefixes are deeply dead (7 to 10
  edges below the planted maximum); prefixes close to an optimum were not
  measured.
- Operational: a first remote queue was launched with a cap 10 times the
  preregistered one and killed before any measurement; a `pkill -f` on machine-b
  matched its own ssh command line.

## A five-clue 466 board verified (00:31, side check, measured)

- **The site's strict five-clue record of 465 is stale.** The board
  `FiveClue_466_2c037e70f7e9` from the public repository
  `gearashi/EternityIISolver` (`data/record466.json`, fetched 2026-11-13,
  attributed there to "Jef" with `stats.eternityathome.org` as origin) was
  checked with independent scripts (`checks/verify-466/`): 256 pieces each used
  once, the piece multiset equal to the official set up to rotation under a
  colour relabelling, 5 of 5 official clues honoured in the repo's own
  definition, 466 of 480 internal edges matched, no grey side off the rim.
- Limits: the colour relabelling was found by search (one valid bijection; the
  score and the piece multiset do not depend on it, the clue check does); the
  first-publication date (2026-10-08 per the review) was not re-checked;
  attribution is the source's metadata. The downloaded data is not committed.
- The repo has no command that verifies a board given as pieces and rotations
  (only edge strings). Worth adding with the records update.
- Not done tonight: `/research/records` is not edited. Draft for the morning.

## From Track 10's judge (00:35)

- **JUDGED: the first-moment price model matches the real piece set** at three
  clue settings, to 0.03, 0.00 and 0.01 dex. The price of a full solve by row
  scan with five clues is 10^40.46 nodes, measured.
- The M1 share must not be generalised beyond the `record` schedule and this
  port. With the quota off the late share is lower, so the quota is not why it
  is small.
- The sampler keeps about 300 effective families and dies by depth 188 to 204;
  five repetitions cannot rule out a heavy subtree it never visits. Accepted
  because two population sizes agree, the frame count is hit exactly and the
  model is independent.
- The painted null sits 0.12 dex above the model (15 of 20 instances above the
  official set): small, one-sided, a caveat for the model.
- For any future port: compare per-depth histograms or deepest depth at several
  caps, not the node total at the cap (equal by construction). Protocol draft.

## From Track 8's judge (00:48)

- The pass is on the threshold exactly; 166 of 240 prefixes have a gap of at
  most 4, so the median is stable, but the margin is zero by construction of
  the rule.
- **The width-1 (counting) control equals the trivial bound** at the median on
  every stratum with 2 or more rows left. "3 edges better than the control"
  means 3 edges better than no bound at all.
- **The gap was measured only on deeply dead prefixes.** A stage 1 would need
  prefixes close to the optimum and a wall-clock arm (one bound costs about
  0.2 s at 7x7 with 3 rows left).
- **Lead's reading of the score-bound line after both judgments:** the bound is
  sound and beats counting, it loosens with size, and on the real pieces the
  record-style engine spends about 4.5e-7 of its nodes where the bound acts
  (one schedule, budget-dependent). Not worth a build for that engine. It stays
  open only for a search whose cost sits in the tail, which nobody has shown
  to exist; the session on where a record is paid is asked exactly that.

## From Track 12's runner (00:55, not judged yet)

- **A matched random control beats the arm under test.** Random partition keys
  remove more nodes than cut-line keys (R7: 0.419 against 0.486). The cut plays
  no part; what prunes is persistent two-edge partitioning itself.
- **A large node saving that is a work loss.** Any persistent two-edge
  partitioning removes 52 to 58% of bb-mrv's nodes on the replayed tree, at 1.2
  to 1.4 times the propagation work. Fourth sighting tonight that node counts
  alone mislead (draft rule P7).
- The census judge's refusal to accept the proxy kill was right in procedure
  and the kill now stands on the proposal's own run.

## From Track 12's judge (01:04)

- JUDGED: any partition on two edges removes about half of bb-mrv's nodes
  (0.42 to 0.49), against 0.70 for single-cell lookahead at the same budget.
  The pruning comes from partitioning on edges with few possible colours, not
  from where the cut sits, and it costs more work than it saves.
- Under most-constrained-cell branching the "frontier" cut line is close to an
  arbitrary line. A cut key for a row-by-row solver was not tested.
- First track of the night whose preregistration order is established by the
  guard's run log and not by file times alone (with Tracks 9 and 10).

## From the memo on where a record is paid (01:15, `memos/where-a-record-is-paid.md`, REVIEWED: its headline number is wrong, see the review below)

Claims until the referee reports (`memos/where-a-record-is-paid.REVIEW.md`) and
Track 13 re-measures them. Two openings, at most 3e8 nodes per run, no clue.

- **Track 10's late-node share does not describe a record-style engine.** The
  port privileges three interior colours (150 sides) against a quota that stops
  at 119, so no colour is ever used up. The record engine as the site describes
  it removes one frame-ring colour and two interior colours (124 sides). Track
  10's numbers stand for the engine it ran; the lead's reading "the record
  engine spends 4.5e-7 of its nodes late" was wrong in scope.
- **With the corrected keying:** a break-free arrival at depth 201 costs 2.5e4
  to 7.8e4 nodes, not 3.5e7 to 1.1e8 (450 to 4,600 times fewer); 54 to 66% of
  nodes are late, 8 to 20% on legal boards only. Seven of nine tries were still
  in the opening at 1e8 nodes.
- **79 to 95% of late nodes are on boards that are not physical** (the break
  path places a piece of the wrong class or rotation at the price of one
  break). Refusing them saves 1.75 to 2.7 times.
- **The partial-match referee's 92 to 98% is also not a record engine:** it is a
  property of one row of a model.
- **Stage costs on legal boards (no clue):** opening about 3e7 nodes once per
  restart; depths 60 to 200 cost 2.5e4 to 7.8e4 nodes per arrival; the legal
  tail 9e3 to 6e4 per arrival, dying by depth 237. Finished boards per arrival
  are model only (one per 10^11 to 10^15).
- **Candidates for a cheaper 471, model prices:** nothing survives as an
  order-of-magnitude gain with good odds. Best: finishing and staging the
  colour exhaustion (model 16.3 to 16.6 in log10 nodes against 18.0; author's
  odds 0.12 for a factor 10 on the real pieces, 0.03 for passing a record).
  Anything acting only after depth 201 is capped at 1.1 to 1.9 times; a better
  arrival generator at 2.2 to 10 times; tail-first, two-sided and pocket
  geometries are 4 to 20 digits worse.
- **Nothing here bears on a full solution** (author's statement): colour
  steering spends boards to buy yield, and at 480 there are none to spend.
- **Two concrete defects claimed in the repo's own Blackwood-style engine:**
  its default privileged colours (chosen on depth at 30 million nodes) and its
  break path admitting wrong-class pieces. If they hold, the project's own
  record runs used a mis-keyed engine. Track 13 measures both.
- Not checked by the memo: which colours Blackwood's 13, 16, 10 are in this
  repo's numbering; five clues (the corrected keying never left the opening
  with clues on the board in three tries).

## Review of the memo on where a record is paid (01:36, `memos/where-a-record-is-paid.REVIEW.md`)

A separate Fable session read Blackwood's public source (five versions), reran
all eight scripts (they reproduce) and ran 15 engine runs of its own
(`memos/where-a-record-is-paid-review/`, 9 core-minutes). Grades are the
referee's.

- **WRONG: "the keying alone moves the cost of an arrival by 450 to 4,600".**
  The memo set the port with five clues at 1e10 nodes against no-clue runs at
  1e8. The missing control (the port's default keying, no clue) costs 3.3e5 to
  4.2e5 nodes per arrival. Like for like the gain is 3 to 20, and 0.8 to 2.5
  when stuck tries are charged. Removing the clues accounts for a factor 90 to
  330.
- **ESTABLISHED, from source: what the published record engine does.**
  Privileged colours {13, 16, 10} in Blackwood's numbering are this repo's
  {5, 20, 7}: one ring colour (24 sides) and two interior (50 and 48), 122
  sides, quota 119 by cell 160. It builds legal boards only (no wrong-class or
  wrongly turned piece; a break is one mismatch on an interior colour). It pins
  piece 139 in the centre. It has no isolation rule. Its last 80 cells are
  filled as a five-row band in columns.
- **HOLDS: neither 4.5e-7 nor 92 to 98% describes a record-style engine**, and
  legal boards are the right accounting. Track 10's M1 stands for its port with
  five clues and says nothing about the record engine.
- **NOT ESTABLISHED: any single late share.** With the published triple and no
  clue: 25%, 53%, 88% of all nodes on three seeds (legal: 3.6%, 8.2%, 53%).
  With the centre piece pinned: 3% and 5% (legal 0.4% and 1.4%).
- **WRONG: "anything after depth 201 is capped at 1.1 to 1.9 times".** The
  formula assumes the reachable boards are unchanged; the memo's own row with
  12% late nodes gives 8.7 for perfect knowledge from 201. **So the score-bound
  line is, again, not closed.** The cap on a better arrival generator runs from
  1.9 to 250 across the range of late shares.
- **NOT ESTABLISHED: "471 costs 120 to 230 times 470".** In the model, cutting
  the last unlock costs 59 to 66 and cutting the first 120 to 230.
- **NOT ESTABLISHED: "one more colour is worth 25 to 50 times"** (0.15).
- **HOLDS: the tail model is the exact first moment** (no joint-spreading
  error) and reproduces; tail-first, two-sided and pocket geometries are dead
  by several decades in direction if not in exact size.
- **HOLDS: the two fidelity fixes** for the repo's Blackwood-style port: key on
  5, 20, 7 and refuse wrong-class pieces. They make the port faithful; the
  stated gain does not follow. Already in the repo: `schedule_builders.rs:173`
  has the 122 pool, the site page says "one border colour and two interior",
  `blackwood_bt` takes `--priv` but nothing passes 5, 20, 7.
- **Open:** the legal sample falls 2 decades faster than the model past depth
  231, and the absolute cost of a 470 is unexplained.
- **Lead's note.** Second time tonight that a number of mine about the score
  bound had to be withdrawn within two hours. What can be said: where a record
  run spends its nodes is not known for the published engine, the measured
  shares vary by two orders across seeds, and Track 13 now has the right arms.

## From Track 11's runner (01:45, not judged yet)

- **The difference map fails on near-critical boards.** 0 of 30 at 7x7 c=6 and
  8x8 c=8 at every band width, where bb-mrv solves 23 of 30 at 7x7. The memo's
  solves were all on boards well above the critical colour count.
- **Wider bands made it worse**, against the hypothesis: median best
  mismatched seams of censored runs 9, 10, 11, 13 for k = 0 to 3 at 8x8, and
  fewer solves at 6x6 for k = 3 (15 of 30) than k = 0 (21 of 30). The exponent
  hypothesis itself was not measured (every slope is infinite by censoring).
- **The over-coloured 8x8 c=10 arm was harder for the map than the critical
  8x8 c=8**, the opposite of the backtracker. The map and the tree do not share
  a difficulty scale.
- No censored run cycled (0 of 1,148); they wander.

## From Track 11's judge (01:51)

- **The primary 8x8 rung separates nothing:** bb-mrv itself solves 0 of 30
  there at the 1e6 cap. Future ladder tracks should not count on it.
- The memo's boards were much easier than the ladder's: the map solves the
  memo's 8x8 with 12 colours 4 times in 5 and the ladder's 8x8 with 10 colours
  once in 270 runs. Its iteration counts must not be quoted for hard boards.
- The kill does not cover a larger cap, another step size, or over-coloured
  boards.

## Review of the theory referee and of the frame-information memo (02:13, `memos/theory-and-frame.REVIEW.md`)

A separate Fable session recomputed the disputed quantities by other routes
(`memos/theory-and-frame-review/`, 7 core-minutes). Grades are the referee's.
With this, every memo of the night has had a second reader.

Theory referee: **all six rulings hold.** The first memo of the night to keep
its headlines.
- **HOLDS: `entropy-area-law`, the limit is not 0.67.** Recounted cell by cell:
  at most 0.640, close to 0.435. The "area law" for the distinct-piece fraction
  also fails: sampled 0.326, 0.025, 0.0004 at 16, 32, 48 cells against the
  page's fit (0.49 at 16, 0.118 at 48); below 1e-3 near 46 cells, not 155.
- **HOLDS: `parity-defect-floor`, "at most 76" is wrong.** At least about 154:
  125 near-twin pairs and 29 turns, confirmed by brute force on three painted
  boards.
- **HOLDS WITH A CORRECTION: `how-hard-is-this-instance`.** No crossing at
  N = 12 to 13. With the five clues the bench is below one expected extra
  solution at every size up to 16.
- **HOLDS: the 307 of 480 and the 114 near-twins are artefacts of the stored
  orientation** (every interior piece is stored smallest colour on top). Any
  "stored orientation" statistic is about the file, not the puzzle.
- **HOLDS: `sigma-cycles`, `design-recipe`, `why-e2-is-hard` overstated.**
- **The theory referee was too lenient elsewhere:** 3 of 5 sampled "sound"
  pages are one grade too kind (`irreducible-hard-region`: its budget test has
  no power; `complex-theory`: its plateau spans 20 decades; `rigidity-wall`:
  marked proven on uncommitted runs). So "24 pages pass" is given 0.5.
- The review's section 6 lists sentence-level site corrections it stands
  behind, with line numbers, and those it does not. Drafts only; no page
  edited.

Frame information:
- **HOLDS WITH A CORRECTION: the corner counts.** 57,131 chains per corner,
  reproduced; with the other clue pieces out of the pool, 17.45 bits, not 17.17.
- **MOSTLY WRONG: "these bits are already inside the price".** True for two
  corner blocks only; the far two close at scan cell 239 of 256. Closing them
  first costs 0.64 digits, so the practical conclusion stands for another
  reason.
- **WRONG: "at 90% of the cost a sound refuter holds 1.3 to 2.0 bits" on 6x6.**
  That was one refuter, a plain fill. Arc consistency plus all-different refutes
  94.4% to 100% of wrong 6x6 frames with zero search nodes. The 6x6 boards are
  not evidence about 16x16 either way.
- **HOLDS on the real pieces, measured by the referee:** arc consistency over
  all 196 interior cells kills 0.4313 of shuffled border words against 0.4293
  on the first ring alone. Propagation and belief propagation see nothing past
  the corner windows.
- **HOLDS: the 2 ranking bits are real** on generated boards (z = 1.40 against
  real valid frames), half of it in the four corner cell counts.
- **HOLDS WITH A CORRECTION: "no cheap frame information beyond about 20
  bits"** (0.75), for counting, corner lists, first-ring counts and arc
  consistency. **Open:** all-different reasoning and clause learning on partial
  frames. That is the one frame question left with no measurement.

## From Track 13's runner (02:37, real 16x16 pieces, not judged yet)

- **Colour keying is worth about nothing on total nodes.** Published (5, 20, 7)
  against the port's default (22, 21, 20), every node charged, 10 openings:
  1.31 [0.69, 2.43] with no clue, 0.74 [0.42, 1.27] with the centre clue.
  Every keying tried is within about a factor 2 of the default.
- **With five clues the published keying never reached depth 201** in 10 runs
  of 1e9 nodes (deepest 52 to 191); the default keying made 135 arrivals. The
  published engine was built for the centre-clue record; this is the first
  sign that its keying does not transfer to the five-clue problem.
- **Fidelity matters more than keying.** Refusing non-physical pieces in the
  break path gives 7.63 [4.82, 13.9] times more legal deep nodes per node
  spent; the full fidelity variant (legal breaks, ring rule, no isolation rule)
  gains 4.47 [3.06, 6.41] at the centre clue and 10.8 [7.0, 19.4] with no clue.
  The repo's port wastes most of its late work on boards that cannot exist.
- **Staging further colours is dead** (kill line: 0.051 and 0 against 3). The
  runner argues the design partly built the kill in.
- **Late-node shares vary by more than a decade between seeds** in every arm
  (fidelity engine: 0.25 to 0.55 with no clue, 0.013 to 0.22 at the centre).
  No single share describes a record engine; arrival counts at a fixed cap
  depend on where the cap falls.

## From Track 13's judge (02:46)

- JUDGED: on the real pieces, colour keying changes the node cost of an arrival
  by a factor within 0.4 to 2.4 (at 1e9 nodes per run, this port).
- **A per-node gain that is a wall-clock loss.** The published keying takes
  about 1,500 s per 1e9 nodes against about 64 s for the default on machine-b, and
  makes 1.25 arrivals per second against 19.2. Fifth sighting tonight that
  nodes alone mislead; here on the real puzzle. Part of the gap is this port's
  break path.
- **A defect in the port's no-break path:** it can place a wrongly oriented
  piece in the first few moves, after which a run spends its whole budget stuck
  at depth 22 to 66 with 2 to 5 legal nodes. Most "never left the opening" runs
  are this, not keying. It may also explain part of what the memo read as an
  "opening cost".
- The staging kill is a statement about one schedule under one scan order.
- **List of differences between the repo's Blackwood-style port and the
  published engine, now established:** colour keying; non-physical pieces in
  the break path; an isolation rule the original lacks; scan order and border
  interleaving; the last 80 cells as a column-filled band; and the
  wrong-orientation defect. The project's own record runs with this engine
  family should be read with that list in hand.

## From Track 14's runner (03:57, real 16x16 pieces, five clues, not judged yet)

- **The whole one-ring keying family is dead with five clues on this port:**
  0 legal nodes at depth 201 or deeper for all 680 keyings (2,040 screening
  runs) and for the top 5, the published keying and 5 random keys on 20
  held-out openings. The port's default (three interior colours) is the only
  arm with any yield.
- **Probable cause is the configuration, not the colours (argued by the
  runner):** unflagged runs mostly stop at depth 46 to 47, just past the clue
  at (13, 2), where the early quota (59 privileged sides by cell 48) prunes;
  depth falls as a keying's side count rises. And 25% of one-ring runs are
  stranded at depth 18 to 22 by the port's orientation defect, which a ring
  colour in the keying triggers by sorting edge pieces first at interior cells.
- So the measured fact is about this port with a quota curve scaled from the
  centre-clue engine. The open question it leaves: a quota curve shaped for the
  five-clue board (lower or later early quota), on a port without the defect.
- No five-clue arm got as deep as Track 13's centre-clue runs at the same
  budget (209 against 238 to 246).
- Operational: a second code shipment overwrote the remote guard logs; nothing
  was lost because they had been fetched and committed first.

## From Track 14's judge (04:04)

- JUDGED: on this port, with five clues, no one-ring keying under the scaled
  quota reaches legal depth 201 (0 of 2,040 runs, and 0 on held-out openings).
- **The comparison with the default keying is confounded with quota slack**
  (79% of privileged sides required against 97.5%). The published keying runs
  at Blackwood's own quota, so its failure here points at how this port differs
  from his engine at least as much as at the colours.
- The fidelity engine cuts the default's deep-node yield about 9 times against
  Track 13's plain engine (different seeds, rough): much of the plain engine's
  apparent yield came from boards that cannot exist.
- The lead's brief to Track 15 repeated the runner's unverified account of
  where the defect acts. Corrected by message before Track 15's runs.

## From Track 15's runner (04:24, real 16x16 pieces, five clues, not judged yet)

- **Quota slack, not colour, decided Track 14.** Swap the slack and the ranking
  swaps: the default keying under a tight quota is dead (0 of 16 openings,
  never past depth 35); published-style keyings under a loose quota reach depth
  201 on 12 to 14 of 16 openings. At matched slack no keying is shown to matter
  (within 1.3 to 1.6 times, every interval includes 1).
- **The published quota (119 of 122 sides by cell 160) is too tight for the
  five-clue board on this port:** the published keying with its own quota
  reaches depth 201 on 2 of 16 openings. Argued by the runner: the tight late
  requirement is the lever, not the early curve.
- **The orientation defect is located and fixed at no cost:** in the top row a
  corner piece could be placed mid-row showing grey to its right, because that
  side was only checked at the last column. With the fix the search is exactly
  the legal search (histograms match the committed legal histograms).
- Some stranding is lack of look-ahead above a clue (a prefix that leaves the
  cell above the clue at cell 34 with no candidate is re-enumerated for the
  whole budget), not the defect.
- One arm (halving the early curve) accidentally built a wall at cell 96 and
  measures nothing.

## From Track 15's judge (05:48, judged KEEP)

- **What matters is the share of its sides a quota demands, not the count**
  (measured, this port, five clues): the published keying under the default's
  absolute curve reaches depth 201 on 2 of 16 openings, under a curve matched
  in share on 14 of 16. The default under a tight share: 0 of 16.
- The runner's "the tight late requirement is the lever" goes past the data:
  the dead arms die at median depth 75.5 and before depth 35, long before
  cell 160. Which part of the curve binds is open.
- At matched slack the default keying is still ahead on point estimate
  (0.61 and 0.75, intervals include 1). No ranking swap.
- The orientation fix changes nothing past depth 201 in any of the 233 runs
  that were not stranded. It can become the reference.
- Guard gap: 92 of 96 local log lines record one modified file without naming
  it (the engine and job scripts were byte-identical to the commit). The guard
  should record the names. Small fix, not applied tonight to keep the guard
  frozen while runs referenced it.
