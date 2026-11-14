# Amendments to PREREG.md

## A1, 2026-11-13, committed before any test-stage run

What: design input from the lead, based on an unrefereed memo
(`research/program/night/2026-11-13/memos/nogood-frame-search-price.md` on the
lead branch). No test-stage run had started when this was written: the run
log has no test line, and no test seed (20261113 and after, null boards 1101
to 1103) has been sampled. Stage 0 had been started and stopped; it is rerun
from scratch with the code of this commit. Nothing below changes a line that a
test seed has met.

1. **Population.** Already preregistered: stages 1 and 2 read on stratum H,
   frames that pass the four clue-block tests. The definition stays: the
   3-cell interior block next to each corner clue (for the corner at (0,0):
   cells (1,1), (1,2), (2,1), clue (2,2)) has at least one fill with distinct
   free interior pieces that matches the five border cells around it (corner,
   two cells along each side) and the clue. Sampled by construction (the test
   is a zero weight inside the SMC sampler), approximately uniform within H.
   **Change: b is counted within H.** For an H frame, b_H(S) =
   log2(N_H / N_H(S)), with N_H(S) the number of valid frames that pass the
   four tests and agree with the frame on S (same SMC counter with the test
   on, 20,000 particles, 3 runs). The stage 2 statistic on real H becomes
   M_H = median_f( min over nogoods of b_H(S) + log2 C(S) ), with no
   beta_H subtraction. The reading thresholds (134.4 and 120 bits, 54 cells)
   are unchanged. The PREREG statistic median(min v_F) - beta_H is still
   reported, as secondary. U frames keep b_F; their result is its own line and
   is read as a check of the memo's corner claim: the measured beta_H against
   17.45 bits, and how many U frames have their cheapest nogood within ring
   distance 2 of a corner.
2. **Censoring.** Already preregistered (a censored plain search gives the
   one-sided ratio 120 / t_X). Calibration decides whether plain search
   finishes H frames; the README states the censored share.
3. **The 54-cell line reads R2 minimised cores only.** Read sets of plain
   search depend on its fill order by construction, so the core-size part of
   the reading uses the size of the R2 minimised core (a frame without a
   proved R2 core counts as infinite). Read sets still enter v as nogoods, and
   their sizes are reported.
4. **New measurement: bottom arcs.** Track 10's row scan starts at row 15
   (`rows_bottom_up` in its `tree.rs`: rows 15 down to 0, left to right), so
   the arcs are bottom arcs: the bottom row plus k - 1 cells up each side,
   2k + 14 cells, for k = 13, 12, 10, 8 (40, 38, 34, 30 cells). For every real
   H test frame (60 frames), only the arc is given; the rest of the border and
   the interior are free. Refuters: R2 (same encoding, the arc as assumptions,
   no minimisation) and a new plain refuter R0-arc (depth-first over every
   free cell, border included, Track 10's bottom-up row order). R1 and the
   spiral are not run on arcs (the matching check and the spiral are defined
   for a full frame). Cap 120 s each. Also b_H and b_F of every arc.
   **Reading, per refuter:** if, for some k in {12, 10, 8}, the arc is refuted
   within 60 s on at least a quarter of the frames (15 of 60), the arc route
   is below the memo's claimed break-even (arcs of 39 to 40 cells, 109 to 111
   bits) and the README says so. If no 40-cell arc (k = 13) is refuted within
   65 s on any frame, the prefix route is dead for that refuter at this cap.
   Otherwise neither line is met. Stage 0 adds a check for partial frames: on
   the eight small boards, bottom arcs (k = 3 and 4) of every tested frame;
   `unsat` only when no brute-force solution agrees with the arc, `sat` only
   with a verified board.
5. **SAT solver state.** R2 is fresh per frame (a new solver loaded with the
   puzzle CNF for every frame; load time is recorded as `load_secs`). An
   incremental arm across frames is lowest priority and is run only if time
   remains after everything above; if it is not run, the README says so.

Null boards: the five clues are planted (the planted pieces at the official
clue cells), so their H stratum has the same clue-corner structure.

Budget: arcs add at most 60 x 4 x 2 x 120 s, about 16 core-hours worst case;
the whole test stage stays inside the 13:45 stop rule of PREREG.md.

## A2, 2026-11-13, committed before any test-stage run

What: the lead forwarded the referee's corrections to the memo behind A1
(`nogood-frame-search-price.REVIEW.md` on the lead branch, section 0). When
A2 was written no test-stage run had started and no test output existed (the
run log has no test line), so no row predates A2 and I had seen no test data.
Stage 0 was running (generated boards only). The original PREREG lines and
the A1 lines are kept and reported; A2 adds readings next to them.

1. **Floors instead of the median alone.** On real H, besides the median of
   A1, report for v = b_H + log2 C per frame the floors
   F(p) = Q_(1-p)(v) + log2 p at p = 0.4, 0.25, 0.1 (Q taken as the order
   statistic that leaves ceil(p n) frames at or above it, and log2 p replaced
   by log2(ceil(p n)/n)), and log2 of the sample mean of 2^v. A censored frame
   enters at log2(600 r), its b taken as 0 (a lower bound). **A2 reading:** the
   nogood route fails its necessary condition if the largest of these floors
   exceeds 134.4 bits. There is no A2 pass line: 60 frames can certify at most
   log2(60/3) = 4.3 bits of gain. Reason (referee, measured on the real
   pieces): 53.0% of H frames have an unfillable 3x3 clue block (eight free
   cells around a corner clue against the six border cells touching them), so
   the median of H can sit on corner-local cores whatever the rest does.
   Every stage 1 and stage 2 statistic is also reported in two strata: frames
   whose cheapest nogood has every cell within ring distance 3 of a corner
   ("near"), and the rest, censored frames included ("far"; the stratum that
   matters). Each test frame is flagged for a dead 3-cell and a dead 3x3 clue
   block by `fc flags` (no solver time). Stage 1 also reports the ratio of
   total times over frames (censored runs entered at their cap), per stratum.
2. **Arc thresholds in host nodes.** The arc budgets are the referee's, in
   log2 nodes per block-passing bottom arc: 44 cells 25.5, 42 cells 29.0, 40
   cells 32.9, 38 cells 37.0, 34 cells 46.0, 30 cells 55.8. They convert to
   host seconds with r, the stage 2 node rate. For each arc length the README
   states whether its budget in host seconds is under its cap (only then can
   the arm kill).
3. **Arc lengths and the nested cost.** No arc run had started, so k = 10 and
   k = 8 are replaced by k = 15 and k = 14: arcs of 44 and 42 cells (2k + 14),
   with the referee's budgets 25.5 and 29.0. Arcs run: k = 15, 14, 13, 12
   (44, 42, 40, 38 cells). R2 cap 120 s, except 450 s at k = 14 (about twice
   the 42-cell budget at the 2.5e6 nodes/s seen on the loaded host in
   calibration, so that this arm can kill). **A2 reading, per level, R2:**
   with T_k the cap and t_16 the stage 1 R2 time on the full frame,
   c_15 = t_15 if refuted, else T_15 + 2^e15 t_16; c_14 = t_14, else
   T_14 + 2^e14 c_15; c_13 = t_13, else T_13 + 2^e13 c_14; c_12 = t_12, else
   T_12 + 2^e12 c_13. The exponents are log2 ratios of the referee's counts of
   block-passing arcs (107.5 for H, 108.9, 105.4, 101.5, 97.4 bits for 44, 42,
   40, 38 cells): e15 = -1.4, e14 = 3.5, e13 = 3.9, e12 = 4.1. (The referee's
   own chain goes from 40 cells straight to the full frame with exponent 6.0;
   the inserted levels follow the same rule and are shown.) Entry at level k
   is below break-even iff mean_f(c_k) x r < 2^budget_k. A censored t_16 makes
   c a lower bound. The A1 arc readings are reported alongside, computed on
   the arcs that exist (k = 12 for "below", k = 13 for "dead").
4. **R0-arc once per frame**, at k = 15, cap 120 s (the plain tree is the
   same for every k until the cap; the referee predicts it censored). If it
   finishes on many arcs, the node counts are reported as a heavy-tail
   finding.
5. Incremental solver arm, if time allows: frames that agree on a 44-cell
   arc. Lowest priority.
6. Stage 0 (i) is also read against the corrected interior cost of a
   block-passing frame, about 10^13.6 nodes in the mean by spiral (lead's
   correction); a capped confirm is a lower bound only.

### A2 addendum, same day, before any test-stage run

Reason for every change below: compute available until 22:00 (the link to
the compute host drops; a detached driver, `scripts/driver.sh`, runs every
remaining step). No test output existed when this was written.

- Bottom arcs, k = 13 (40 cells): R2 cap 6400 s, about twice the 40-cell
  budget (2^32.9 nodes) at 2.5e6 nodes/s, on real H frames realH0 to realH29;
  120 s on realH30 to realH59. k = 15 (44 cells, budget about 19 s) keeps
  120 s and k = 14 (42 cells, about 215 s) keeps 450 s, both at least twice
  the budget. k = 12 (38 cells, budget about 15 hours) keeps 120 s and cannot
  kill: stated, not read. In the nested cost a capped row uses its own cap.
- Extra set "realfar" (stage `test-far`): 240 H frames sampled with seed
  20261115, flagged by `fc flags`, the first 60 with no dead 3x3 clue block
  kept (rejection from H, so approximately uniform within "H without a dead
  3x3 block"). Same refuters and caps as the main test. It enlarges the
  stratum away from the corners; it is reported as its own set with the same
  statistics and no separate kill line.
- The incremental-solver arm is not scheduled.
