# PREREG amendments: prefix-statistics

Each entry: what, why, when. Times are local (CET); the commit times govern.

## A1. Decoy control in rank form (before any gate or test run)

When: 2026-11-13, after the PREREG commit (`e9ecd977`), before the first
guarded run. No gate or test data existed.

What: negative control 2 (PREREG section 8) is read on the decoy's mid-rank
percentile in the sampled population (fraction below plus half the ties)
minus 0.5, averaged over directions per board, instead of on its clipped
delta. Pass: for each of the 10 statistics at each peak depth, the 0.99833
bootstrap interval of the mean over boards contains 0. The clipped-delta form
is still printed, as descriptive.

Why: for a statistic whose population is a point mass plus a rare tail (S10
and S7 at 160 and 176, where almost every sampled prefix has value 0), a
draw from the population has a clipped delta whose mean is not 0 (the rare
large positive values are clipped at +10 while the common value gives a
small negative delta). On the pilot this produced a degenerate interval at
-0.09 for S10 at 160. The mid-rank percentile of a draw from the same
population has mean exactly 0.5 whatever the shape, so it is the test the
control means.

## A2. S6 infeasible encoding; G-exact eligibility (after the gates, before any test run)

When: 2026-11-13, after the first gate runs (committed in `6a62f023` with
their reading in `results/gate/gates-reading.txt`), before any test board
was generated.

What the gates showed. G-rep passed (49 of 54 cells, 0.907, against 0.90).
G-exact failed on the sd ratio (481 of 561 cells, 0.857, against 0.90; the
bias criterion passed, 553 of 561). Two causes, both visible in the rows:

1. S6 used a sentinel of -1e6 when some colour's frontier demand exceeds its
   remaining supply. On the 6x6 exact enumeration a few such nodes make the
   exact sd of S6 enormous, so every S6 cell at depths 9 to 18 fails the sd
   ratio (the SMC sample rarely contains one). The same sentinel hit 16x16:
   in G-rep the N = 1e6 run drew such a prefix at depth 176 on both boards,
   and the S6 delta fell from 7.6 and 2.2 (N = 1e5 mean) to 0.06 and 0.03.
   This is a defect of the statistic's definition, not of the sampler.
2. At 6x6 depths 23 and 27 the SMC population is 2 to 24 particles (medians
   15 and 2), so the sample sd is near 0 for many statistics. The 16x16 test
   never enters that regime at the peak (populations about 100,000 at 144,
   80,000 at 160, 4,000 to 5,000 at 176 in the pilot).

What changes.

1. S6: a colour with r_c - f_c < 0 is clamped (the falling factorial runs to
   r_c!, and that colour contributes no internal pairs) instead of setting S6
   to -1e6. S6 stays a continuous census statistic; the sound infeasibility
   certificate it used to encode is not used.
2. G-exact eligibility: a (board, depth, statistic) cell is gated only if its
   median live population over the repetitions is at least 50 particles (in
   addition to the existing rules). All cells are still printed
   (`POP_FLOOR=0 python3 -I scripts/gates.py <dir>` reads the gate as first
   written).
3. Both gates are rerun with the corrected binary into `results/gate-a2/`; the
   first gate files stay committed as they were. The test runs only if both
   rerun gates pass.

Why this is not tuning toward a result: neither change touches the test
data (none exists), the readings, or the thresholds of the readings; the S6
change removes an outlier mechanism that would have pushed S6 toward a null
delta on any board where one sampled prefix is infeasible.

## A3. Accidental run of the `real` stage (disclosure)

When: 2026-11-13 08:25:59 to 08:27:17 UTC (run log ids a54a71fa9aac and
149fd83aac2e).

What happened: `run.sh` was edited (commit `b5a039d8`, adding the `calib`
stage) while the A2 gate run was executing it. Bash reads a script as it
runs, so the two job processes for G-exact boards 9001 and 9002 resumed
inside the new text after their gate command and executed the body of the
`real` stage with their own output path. The official-pieces population run
(deterministic, SMC seed 7) therefore ran twice, through the guard, and
overwrote `results/gate-a2/exact-9001.jsonl` and `exact-9002.jsonl`. The two
outputs are byte-identical and are kept, moved unedited to
`results/stray/real-into-exact-900{1,2}.jsonl`. Only the first line of one
was looked at (population means at depth 32). No test board was generated and
no reading uses these files.

What changes: G-exact boards 9001 and 9002 are rerun into `results/gate-a2/`;
the gate-rep A2 run, which never started, is run. The scheduled `real` stage
will rerun the same deterministic command into `results/real/real.jsonl`.
Rule kept from now on: no file of the experiment is edited while a stage
runs.

## A4. The design review's must-change list (before any test run)

When: 2026-11-13, after the design review
(`research/program/night/2026-11-13/memos/d4-design-review.md` on the lead's
branch) and before any test board was generated. Instruction from the lead:
apply the review's eight items exactly; Reading 1 and the kill line stay;
A4 adds readings and removes none; where an item and the code disagree,
keep the quantity well defined and write the deviation here.

Reviewer's prior expectation, written before the test run (section 0 of the
review): in a planted ensemble the true prefix is a uniform consistent
prefix reweighted by its number of completions; S6 is the first-moment log
of that number, so for a roughly normal log-weight with sd sigma the true
prefix's S6 moves by about sigma^2, a delta of about 1 (calibration: shift /
sd^2 of 1.2 to 1.6). A tilt of about one sd of S6 is therefore what the
random model predicts, not a contradiction of it; S4 and S9 are readings of
the same census (true-prefix correlation with S6 about -0.9 for S9); and
such a tilt is worth about 2 bits at one direction (one prune near depth 96
to 128, success 0.5), less at the peak, and four directions lose bits.

1. **Level profile and family diagnostic.** Every (board, direction) writes
   its full width profile (log10 of the SMC estimate at every level, row
   `profile_log10_width`). Every depth row writes `fam_distinct` (distinct
   depth k - 32 ancestors in the whole population) and `fam_ess` (1 / sum of
   squared family shares). Deviation: the review asked to set Track 10's
   `ANC_DEPTHS` / `DIAG_DEPTHS`; those constants feed a diversity table this
   binary never prints, so the diagnostic is computed instead in `capture()`
   from the parent indices (same quantity, any target depth, Track 10's code
   untouched). Validity: a row is invalid for every decision if `fam_ess` <
   200 or population < 1,000 (an extinct row is invalid); a depth with more
   than 10% invalid rows is demoted to descriptive for all statistics.
   Byte identity: board 7007 is rerun with the A4 binary at its calibration
   settings and every pre-existing field other than timings (`ns_per_eval`,
   `stat_seconds`, `smc_seconds`, `ctx_seconds`, `seconds`) must have the
   same JSON text (`scripts/identity.py`, stage `a4check`). Checked on dir 0
   unguarded before this entry: 198 fields, 0 differences.
   **Deviation forced by this rule: population cap N raised from 1e5 to 5e5
   for the test boards and the official pieces.** On calibration boards 7008
   to 7011 at N = 1e5 the rule demotes every decision depth (invalid shares
   37.5% at 128, 56% at 144, 50% at 160, 100% at 176; median family ESS 168
   at 144): directions 2 and 3 pass a clue around depth 136 that funnels the
   population. At N = 3e5 (7008, 7009) directions 2 and 3 still gave 162 to
   294 at 144 and 160; at N = 5e5 (7009 dirs 2, 3) 272 and 399 at 144, 441
   and 452 at 160, 275 and 280 at 176; at N = 1e6 (7008 dir 2) 645, 718, 346
   but 87 s per direction and 3.7 GB per process, which breaks the budget.
   The threshold is the review's; only N moves. G-rep showed the deltas
   stable from N = 1e5 to 1e6, so the estimand is unchanged. Expected cost:
   about 2.5 core-minutes per board, about 4 core-hours for 96 boards. The
   PREREG hard stop (section 10) still applies.
2. **Census pruning separated.** Per row: `n_dead` (sampled prefixes with
   some colour r_c < f_c) and its work share; the run stops with an error if
   the true prefix is ever census-dead. S4, S6, S9 use reference mean, sd and
   percentiles over census-feasible sampled prefixes only (`*_feas` fields);
   the dead share is reported per depth as exact pruning, never inside a
   delta. The pre-existing fields are kept unchanged (byte identity).
3. **Reading 3 (survivors).** At 128, 144, 160: population members in a
   seeded random order (own RNG, the SMC's untouched) are tested for a
   descendant 16 cells ahead (S10 > 0, same 1e5 node cap; existence is
   checked depth-first and stops at the first descendant, which gives the
   same truth value as S10's capped count) until 1,000 are found or the
   population is exhausted. delta_surv over census-feasible survivors (all
   statistics computed, S4, S5, S6, S9 decisive); fewer than 200: unmeasured.
   Surviving fraction recorded. r = 2 (32 cells, 1e6 node cap) on the r = 1
   survivors, reported if at least 200 pass, else unmeasured. Reading 3 uses
   the reading-1 line (|mean| >= 1.4, 0.99833 interval excluding 0.7; the
   review gave no line, and this keeps the three readings comparable). A
   census statistic is a **usable tilt** only if it passes Readings 1, 2 and
   3 at the same depth, so only 144 and 160 can qualify.
4. **Reading 2 floor 30** (was 10), applied to both the all-prefix and the
   feasible work ESS; the median work ESS is printed beside the work table.
   The README states that at 160 and 176 the work weights are close to
   uniform (calibration median ESS 184 and 392 of 1,000), so Reading 2 cannot
   differ from Reading 1 there; its informative depths are 128 and 144.
5. **Value reading for S6** at 96, 128, 144. Per board, directions sorted by
   total T = A + B (A = nodes at levels <= k, B = below, from the profile).
   K = 1, 2, 4 cheapest directions, each pruned at k keeping the work
   fraction f; the true prefix is kept if its S6 work percentile (census-dead
   prefixes ranked lowest, field `s6_work_pct_deadlow`) is >= 1 - f; success
   on a board if some direction keeps it. G = log2(p) + mean over boards of
   log2(T_best / sum over the K runs of (A + f B)); f = smallest value of a
   400-point log grid on [1e-4, 1] with p >= 0.5, and again p >= 0.8.
   Deviation, to keep G well defined across boards of different size: the
   review's C (A + f B) is replaced by the summed per-direction costs and the
   ratio is averaged in log2 over boards (identical when the K directions
   have proportional profiles). Bootstrap over boards (4,000 resamples, f
   re-chosen in each), 95% interval. "Usable" in any summary requires the
   lower end of G above 1 bit at some depth (p >= 0.5). A pass of Reading 1 at
   160 or 176 alone is reported as "a tilt at a depth where it saves under
   0.3 bits".
6. **96 boards, 30 tests, no split.** Per statistic and peak depth: median,
   quartiles, share of boards with (sign-oriented) delta below 0.7 and below
   0. S6 is the primary census statistic; S4 and S9 are readings of the same
   quantity, with their per-row correlation with S6 reported by depth; a
   pass of S6 and S9 together is one finding.
7. **Direction correlation by pair type**, opposite (0-2, 1-3) and adjacent,
   at 96, 128, 144, 160, 176, for the node delta and for the kept-tail
   indicator (work percentile >= 0.9 in the tilted direction).
8. **Painter control, registered now** (stage `painter`, may run beside the
   test). 6x6 framed boards, 8 colours (2 frame), seeds 9101 to 9140, no
   clue, row scan, exact enumeration of the whole tree and of every
   solution. The colour count was chosen on probe seeds 9901 to 9904 (124 to
   240 solutions per board at 8 colours, 4 s each). Exact delta of S4, S6, S9 at depths 5, 9, 14,
   18, 23, 27 for the planted solution's prefix and for the prefixes of
   every other solution, rotations of the planted board excluded. Pass (no
   painter signature): at each depth and statistic, the mean over boards of
   the planted delta lies inside the 95% bootstrap interval of the mean over
   boards of the non-planted delta. No claim that a census tilt transfers
   beyond planted boards is written before this is in.

Additions to the code for A4: `Tree::exists_descendant`,
`Ctx::census_dead`, `Ctx::census3`, the `painter` subcommand, the survivor
search and family tags in `capture()`; `scripts/analyze.py` rewritten for
the A4 readings (the A1 decoy rank and every PREREG reading kept);
`scripts/identity.py`, `scripts/painter.py`.

## A5. Board-number formatting fault in the first test launch (disclosure)

When: 2026-11-13, test stage launched 10:13, stopped 10:20 local.

What happened: macOS `seq 1113001 1113096` prints seven-digit numbers in
exponent form (`1.113e+06`, `1.11301e+06`, ...). `psx` could not parse them
as a board seed and fell back to its default, board 1, so the first jobs
measured board 1 (not a test seed, never registered) under wrong file names
(three files, two of them for the same name, run log stage `test`). No
registered test seed was generated. The files are kept unedited in
`results/stray/seq-board1/`; nothing in them was read except the first
characters of one line to confirm `"board":1`.

What changes: `run.sh` uses `seq -f %.0f` for the test and calibration board
lists; `psx` behaviour is unchanged. The test stage is relaunched from an
empty `results/test/`.

## A6. PREREG hard stop applied at 48 test boards (disclosure, no rule change)

When: 2026-11-13 12:03 local, when the 48th test board finished.

PREREG section 10: "if after 48 test boards the projected total exceeds 8
core-hours, stop at the boards finished". At that point the guard log held
4.18 core-hours of ended runs, the unguarded pilot and probes about 0.5 more,
and the test boards had cost 252 s each on average (the machine was shared
with another track, load 11 to 24), so 48 more boards projected a total of
about 8.1 to 8.2 core-hours. The test stage was stopped. The two boards in
progress (1113049, 1113050) were killed before their `done` line; their
partial files are kept unedited in `results/stray/stopped/` and enter no
reading. The reading uses boards 1113001 to 1113048. The sigma the PREREG
power arithmetic tolerates at B = 48 is sqrt(48 / 40) = 1.10; the measured
board-to-board sd of the S6 delta at the peak is 0.91 to 1.17
(`results/summary.json`, `tables.node_peak_valid`). The official-pieces stage
(`real`) ran after the stop.
