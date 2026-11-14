# Judge report: baseline tournament (night/2026-11-12-t5-baseline-tournament @ 240268a)

## Verdict: keep

Track label: measurement. The preregistered kill criterion is anchor identity,
and it does not fire: `bb-mrv` and `scb-gacolor_ac3_random` (run seed 1)
reproduce every committed row they are compared against, and so does every
other entrant with committed rows on these instances. The ranked tables
recompute exactly from the committed raw rows. The deliverable (ranked table
per rung, incomplete table, excluded table, identity report) stands.

Four sentences of the README reading go past the evidence or are wrong and must
be corrected before anything is published from it (section "Required
corrections"). None of them touches a preregistered statistic, so they do not
change the verdict.

## 1. Preregistration order

- `PREREG.md` is committed alone in `81860a4` at 19:09:11 +0100 and never
  edited afterwards (`git diff 81860a4 HEAD -- PREREG.md` is empty). Every
  results commit follows it.
- No `results/run-log.jsonl`. `prereg_guard.py` is not on this branch: it was
  added to main at 20:41 (e254e3f), after these runs started at about 19:12,
  and the branch base (6247af0) predates it. So the order rests on file times
  and commit order only. Evidence: the earliest result file
  (`raw/bb-mrv/s0-R8.jsonl`) was born at 19:13:04; the first stage-time line
  (19:15:49, 221 s) puts the first launch at about 19:12:08, three minutes
  after the PREREG commit. All 207 non-empty result files are born between
  19:13:04 and 22:17:34. A test result written before 19:09 and then
  overwritten would leave no trace; nothing suggests one, and the anchors make
  it moot for the solvers that matter (they are deterministic and match rows
  committed by other experiments days earlier).
- The runner crate was committed at 19:15:35 (`ad2479a`), after the first
  rows were produced. The only later change to `compute/` and `run.sh`
  (amendment 1) adds machine metadata from environment variables and the
  `part` stage; the search code is unchanged. The identity check below is the
  proof that the committed runner reproduces the committed rows.
- Four amendments, all dated and committed during the run, change machines,
  thread counts and the driver, never a statistic, threshold, entrant or cap.

## 1b. Track label

Measurement, no new method. Node-sequence hashes are not recorded by the
hard-ladder `RunRecord`; the equivalent check here is (status, nodes) identity
against committed rows, done in full (section 3).

## 2. Rigor and adversarial review (adapted)

- Null model / baseline: the comparisons are between existing solvers under
  one counter (`hard_ladder::run_one`, `NodeCounter::try_assign`) and one cap;
  no solver reads the planted board at run time. ProbCut and `bb-learned` were
  calibrated on planted labels of other instances; the rows say so.
- Equal node accounting: every entrant runs through the same `run_one`.
  `nodes_counted` exceeds `nodes` on 1,268 rows, all of them `censored` at
  exactly 1,000,000 (the solver overshoots the cap before it stops); no
  solved row is affected.
- Censoring: time-censored rows are counted as censored at C, never as wins.
  0 error rows, 0 inconsistent tries, 204 (entrant, run seed, rung) cells,
  every one with exactly the 30 preregistered instance seeds.
- Dropped seeds: none silently. Not reached (listed in `not-reached.txt`):
  run seeds 2 and 3 of `cmp-hall-all-k4`, `cmp-hall-all`,
  `cmp-hall-frontier-k4`; check arms `scb-fork-off`, `cmp-fork-off`.
- Budget: summed per-run wall clock is 19.10 h against the preregistered 10
  core-hours (recomputed). Reported in the README. A protocol deviation, not a
  validity problem for node counts.
- Wall clock: per-machine column, never compared across machines (`n/c`).
  Rows of the A16 arms depend on their 240 s guard and on machine load (see
  finding 4).

## 3. Recomputed numbers (own scripts, from the committed raw rows)

- Anchor identity: 0 (status, nodes) mismatches over 1,310 compared rows
  (1,130 hard-ladder-format rows in 44 committed files, plus 180 A15 rows in
  6 files), 0 guard differences. `bb-mrv` 130 of 130, `scb-gacolor_ac3_random`
  s1 110 of 110. Matches `results/identity.csv`.
- mac-m1 check: `scb-gacolor_ac3_random` s1 on mac-m1 equals the Dell rows on
  30/30 at each of R6, R7, R8.
- Solved counts, KM Q1/median/Q3 and G1 (median and bootstrap CI) for every
  cell match `results/tables.md` to the printed digit, including the
  bootstrap CIs.
- The three ranked tables (47 complete entrants per rung), recomputed with the
  preregistered rule (solved desc, KM median, Q1, Q3, sum of nodes; stochastic
  entrants on run-seed medians), are identical in order to the README.
- Headline values: R6 `bb-mrv` 30/30, KM median 460; R7 `scb-ad` seed-median
  27/30 (26 to 29), `mrv-rand` 26 (25 to 27), `bb-mrv` 23,
  `scb-gacolor_ac3_random` 18 (16 to 20); R8 `bb-mrv` 30/30, KM median 69,036,
  row-major DFS 24/30, KM median 483,034 (`rm-dfs` median wall 44 ms against
  `bb-mrv` 5,145 ms, same machine).
- Screening flags reproduce: R6 `scb-ad` s1/s2/s3 0.374/0.420/0.383, `bb-mrv`
  0.468 [0.318, 0.895], `mrv-rand` s2/s3 0.489/0.368; R8 `mrv-rand` s3 0.495
  [0.409, 0.768].

### Judge's rerun (this Mac, Apple silicon, 2 threads, runner built from 240268a)

14 (entrant, run seed, rung) cells, 420 rows, 0 (status, nodes) mismatches
against the committed rows: `bb-mrv` R6, R7, R8; `scb-gacolor_ac3_random`
s1 R6; `scb-ad` s1 R7 and s2 R6; `mrv-rand` s3 R8 (the one R8 screening
flag); `hp-dfs` R8; `cmp-hall-all` s1 R6; `ktam-v2` s1 R6; `fc-luby` s2 and
s3 R6 (confirms finding 1); `engine-dfs-random` R6; `scb-joe_depth150` R6.
Six of these cells were committed from mac-m1 and eight from the Dell, so the
rerun also crosses machines.

## 4. Findings

1. **`fc-luby` has two distinct run seeds, not three.** A12's seeding is
   `rng = (seed ^ name_hash) | 1`; seeds 2 and 3 differ only in bit 0, which
   `| 1` overwrites, so they are the same run. Committed rows confirm it:
   `fc-luby` s2 and s3 are row-identical at R6 and R8 (and R7, all censored).
   Its "(seeds 1,2,3)" label and its run-seed median (which is seed 2's value)
   overstate seed diversity. Does not affect any other entrant (checked: every
   other stochastic entrant's seeds differ wherever any instance is solved).
   This is a defect in A12's code, not this branch's; it would bite any future
   use of `fc-luby` with consecutive even/odd seeds.
2. **"Random value order helps at R7 and costs at R8" is not supported.**
   Paired G1 of `mrv-rand` (the run-seed control of `bb-mrv`'s value order)
   against `bb-mrv`: R7 0.906 [0.438, 1.428], 0.881 [0.496, 1.190],
   0.903 [0.585, 1.000]; R8 1.623 [0.890, 2.118], 0.847 [0.399, 2.596],
   0.495 [0.409, 0.768]. At R7 every CI reaches 1 (solve discordance per seed
   5:1, 5:3, 3:0); at R8 the seed-median ratio is below 1. The "cost" at R8 is
   `scb-ad` (1.18 to 1.52), which is a different solver, not `bb-mrv` with a
   random value order.
3. **Rank 1 against rank 2 is inside the run-seed spread at R6 and R8.**
   `bb-mrv` is one fixed tie-break order (a single draw). At R8 its KM median
   69,036 lies inside `mrv-rand`'s seed range (46,568 to 107,496); at R6 its
   460 sits inside `scb-ad`'s (422 to 472). The ranking is the preregistered
   ordering, but adjacent ranks between a deterministic entrant and a
   stochastic one with the same propagation are not a measured difference.
4. **The A16 arms are ranked at R7/R8 on a wall-clock guard.** `bb-impact`,
   `bb-learned`, `bb-single` are time-censored on 24, 28, 27 of 30 rows at R7
   and 22, 27, 26 at R8, under shared load, with `bb-single` R7/R8 on mac-m1
   and the other two on the Dell. Their R7/R8 solved counts and ranks measure
   the guard on that machine, not node efficiency. The README states the
   count; the ranked rows should carry the caveat inline.
5. **Screening flags against the R6/R7 baseline are conditional on its run
   seed 1.** `bb-mrv` against `scb-gacolor_ac3_random` at R6 gives 0.468
   (beats) against s1, 0.517 [0.243, 0.772] against s2 and 0.551
   [0.264, 0.996] against s3 (both fail the 0.5 threshold). At R8 `bb-mrv`
   beats `scb-gacolor_ac3_random` on every seed (s2 0.255, s3 0.273). The
   prereg names s1, so the flags are correct as computed; a confirmation
   should pair against all three baseline seeds.
6. **"within 0.3% of gacolor_ac3" holds for KM medians only.** The four
   `joe_depth150*` presets match `scb-gacolor_ac3` on 18/30 (R6), 17/30 (R7),
   7/30 (R8) rows, with per-instance node differences up to 0.37%, 1.08%,
   0.55%. KM medians differ by 0.27% (R6) and 0.23% (R8). The other duplicate
   claims hold node for node at every rung (`gacolor_ac3` = `ns1` =
   `verhaard_preferred`; `naive_rowmajor` = `hp-dfs` = `rm-dfs` =
   `example-dfs`; `naive_spiral` = `engine-dfs-spiral-in`).
7. **The plain-DFS result at R8 is about one cell order, not about plain DFS.**
   Row-major (and snake) DFS solve 24/30; column-major 20, double-snake 10,
   spiral-in, spiral-out, border-first, diagonal and the random path 0. The
   random path has one path seed (0), so there is no run-seed control on cell
   order. The README says "row-major"; keep it that narrow.

## Required corrections (README, before any site use)

- Replace "Random value order helps at R7 and costs at R8" with the paired
  numbers of finding 2 (no detectable effect of `bb-mrv`'s random value order
  at R7 or R8 on 30 instances and 3 run seeds).
- Mark `fc-luby` as "(seeds 1,2; 3 = 2, A12 seeding)" in the tables.
- Change "within 0.3%" to "KM medians within 0.3%; per-instance up to 1.1%".
- Add, next to the R6 and R8 tops, that the first two places are within the
  run-seed spread (finding 3), and that A16 R7/R8 ranks reflect the 240 s
  guard (finding 4).

## Claims graded

- Anchors reproduce, ranked tables, screening flags: measured (committed rows
  here, recomputed and partly rerun).
- "A22 measured 0.80 on seeds 21..30": reported (A22's number, not recomputed
  here).
- "Each flag needs its own preregistered confirmation": correct framing.

## Unplanned observations

- The A12 seeding collision (finding 1) also collides seeds 0 and 1 and any
  pair 2k, 2k+1.
- `nodes_counted` overshoots the cap on censored rows of the `scb-*` and
  A16/A17 entrants, by up to 6,715 nodes (`scb-chess_gacolor`; most entrants
  under 200; the solver keeps assigning for a while after the counter trips);
  harmless
  under the cap rule, but a solver whose propagation batch could carry a solve
  past C would be scored inconsistently. None here.
- Wall clock: row-major DFS solves 24/30 at R8 at about 1% of `bb-mrv`'s
  median per-run wall (same machine). On a wall-clock budget instead of a node
  budget the R8 ranking would change; that is a separate measurement.
