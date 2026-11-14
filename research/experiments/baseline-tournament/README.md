# Baseline tournament (night shift 2026-11-12, Track 5)

Label: measurement. Preregistration: [`PREREG.md`](PREREG.md) (commit
`81860a4`, committed before any run). Amendments: `PREREG_AMENDMENTS.md` (1: tiers 4 to 6
moved to m1; 2 and 3: A16 arms on extra threads; 4: fixed thread quotas,
replacement x86 laptop driver `x86-driver-a4.sh`). Generated boards only; nothing here is official
Eternity II progress and nothing is extrapolated to 16x16. A separate judge
decides the outcome.

Every solver-like component of the repo that can be driven through the
hard-ladder `NodeCounter` (one node per `try_assign`, PROTOCOL.md) ran on R6
(6x6 c=5, primary seeds 1..30), R7 (7x7 c=6, primary seeds 1..30) and R8 (8x8
c=10, A22's set: `8-c10` seeds 21..30 plus `8-c10-fresh` seeds 31..50) under
the same 1,000,000 node cap, through the unchanged `hard_ladder::run_one`.

## Outcome against the preregistered criteria (self-assessment; a separate judge decides)

- **Kill criterion (anchor identity): not fired.** `bb-mrv` matches its
  committed rows on 130 of 130 compared rows (A16 R6/R7 seeds 1..20, A22's
  Q1 R6/R7 seeds 21..30 and the 30 R8 rows), `scb-gacolor_ac3_random` run
  seed 1 on 110 of 110 (hard-ladder's 60 R6/R7 rows, A22's 50 rows), 0
  mismatches (`results/tables.md`, first lines).
- **Rerun identity, every entrant with committed rows on these instances:**
  1,310 rerun rows compared against 50 committed files of 19 entrants, 0
  node mismatches, 0 guard differences (`results/identity.csv`). Entrants
  without committed rows on these instances (the tier-4 components, the A17
  Hall variants not committed at test, the A22 check arms) have nothing to
  compare.
- **m1 check (amendment 1):** `scb-gacolor_ac3_random` run seed 1 rerun on
  m1 equals the x86 laptop rows on 90 of 90.
- **Validity:** no entrant has an error row or an inconsistent try at any
  rung; all 47 complete entrants are ranked at every rung.
- **Not reached (deadline 22:00, `results/not-reached.txt`):** run seeds 2 and
  3 of `cmp-hall-all-k4`, `cmp-hall-all`, `cmp-hall-frontier-k4` (ranked on
  seed 1, marked "(seeds 1)"), and the two check arms `scb-fork-off`,
  `cmp-fork-off` (no rows).
- **Budget overrun:** the summed per-run wall clock is 19.1 core-hours against
  the preregistered 10 (about 10 of them are A16 runs stopped by their 240 s
  guard). Per-run wall clock under concurrency overstates CPU time, but the
  overrun is real.
- **Screening flags against the adopted baseline (G1 per PROTOCOL.md,
  descriptive, no multiplicity correction):** R6 against
  `scb-gacolor_ac3_random` s1: `scb-ad` on all three seeds (ratios 0.374,
  0.420, 0.383), `bb-mrv` (0.468, CI [0.318, 0.895]), `mrv-rand` on seeds 2
  and 3 (0.489, 0.368). R7: none (best `scb-ad` 0.536 to 0.625, CIs reach
  above 0.5 or to 1). R8 against `bb-mrv`: only `mrv-rand` seed 3 (0.495, CI
  [0.409, 0.768]); seeds 1 and 2 give 1.623 and 0.847, so the run-seed median
  does not beat it. Each flag needs its own preregistered confirmation.

## Ranked tables (complete solvers; nodes are the result)

Ranking rule (PREREG.md): solved/30, then KM median, Q1, Q3, sum of nodes;
stochastic entrants on the run-seed median (range of solved counts in
brackets). Wall clock is the median per-run wall on the machine named, under
shared load: indicative only, never compared across machines. Full columns
(Wilson intervals, quartiles, G1 per seed, wall ratios, sources):
`results/tables.md`.

#### R6

The top two places (`bb-mrv`, `scb-ad`) are within run-seed noise (see Corrections).

| rank | entrant | solved/30 | KM median nodes | median wall ms (machine) | time-cens |
|---|---|---|---|---|---|
| 1 | bb-mrv | 30 | 460 | 6 (x86) | 0 |
| 2 | scb-ad (seeds 1,2,3) | 30 (30-30) | 463 | 10 (x86) | 0 |
| 3 | mrv-rand (seeds 1,2,3) | 30 (30-30) | 548 | 12 (x86) | 0 |
| 4 | mrv-nohs | 30 | 583 | 14 (x86) | 0 |
| 5 | cmp-hall-all (seeds 1) | 30 | 702 | 13 (x86) | 0 |
| 6 | mrv-bf | 30 | 717 | 10 (x86) | 0 |
| 7 | cmp-hall-all-k4 (seeds 1) | 30 | 837 | 10 (x86) | 0 |
| 8 | cmp-hall-frontier-k4 (seeds 1) | 30 | 1,068 | 22 (x86) | 0 |
| 9 | cmp-hall-frontier (seeds 1,2,3) | 30 (30-30) | 1,194 | 10 (x86) | 0 |
| 10 | scb-gacolor_ac3_random (seeds 1,2,3) | 30 (30-30) | 1,291 | 9 (x86) | 0 |
| 11 | bb-impact | 30 | 1,317 | 1,314 (x86) | 0 |
| 12 | scb-gacolor_ac3_lcv | 30 | 1,351 | 14 (x86) | 0 |
| 13 | scb-gacolor_ac3 | 30 | 1,508 | 10 (x86) | 0 |
| 14 | scb-gacolor_ac3_ns1 | 30 | 1,508 | 18 (x86) | 0 |
| 15 | scb-verhaard_preferred | 30 | 1,508 | 7 (m1) | 0 |
| 16 | scb-joe_depth150 | 30 | 1,512 | 6 (m1) | 0 |
| 17 | scb-joe_depth150_bp | 30 | 1,512 | 6 (m1) | 0 |
| 18 | scb-joe_depth150_bp_rec | 30 | 1,512 | 6 (m1) | 0 |
| 19 | scb-joe_depth150_bp_rec_layered | 30 | 1,512 | 6 (m1) | 0 |
| 20 | bb-single | 30 | 3,522 | 1,994 (x86) | 0 |
| 21 | scb-chess_gacolor_ac3 | 30 | 4,887 | 13 (m1) | 0 |
| 22 | scb-border_first_full | 30 | 6,240 | 16 (x86) | 0 |
| 23 | scb-gacolor_symbreak | 30 | 6,507 | 5 (m1) | 0 |
| 24 | scb-border_first_gacolor | 30 | 6,511 | 5 (m1) | 0 |
| 25 | scb-border_first_parity | 30 | 6,539 | 6 (m1) | 0 |
| 26 | scb-border_first_lcv | 30 | 6,540 | 8 (x86) | 0 |
| 27 | scb-rare_color_first | 30 | 6,540 | 7 (x86) | 0 |
| 28 | bb-learned | 30 | 8,804 | 3,068 (x86) | 0 |
| 29 | engine-dfs-spiral-in | 30 | 18,863 | 11 (x86) | 0 |
| 30 | scb-naive_spiral | 30 | 18,863 | 12 (x86) | 0 |
| 31 | engine-dfs-border-first | 30 | 21,739 | 10 (x86) | 0 |
| 32 | fc-dfs | 30 | 38,866 | 7 (x86) | 0 |
| 33 | fc-luby (seeds 1,2; 3 = 2, A12 seeding) | 30 (30-30) | 40,137 | 9 (x86) | 0 |
| 34 | probcut-v2 | 30 | 41,315 | 5 (x86) | 0 |
| 35 | example-dfs | 30 | 41,823 | 42 (x86) | 0 |
| 36 | hp-dfs | 30 | 41,823 | 25 (x86) | 0 |
| 37 | rm-dfs | 30 | 41,823 | 3 (x86) | 0 |
| 38 | scb-naive_rowmajor | 30 | 41,823 | 38 (x86) | 0 |
| 39 | engine-dfs-snake | 30 | 42,889 | 31 (x86) | 0 |
| 40 | engine-dfs-column-major | 30 | 46,596 | 23 (m1) | 0 |
| 41 | engine-dfs-double-snake | 30 | 112,550 | 60 (m1) | 0 |
| 42 | probcut-v1 | 30 | 171,321 | 17 (x86) | 0 |
| 43 | engine-dfs-diagonal | 28 | 408,044 | 174 (m1) | 0 |
| 44 | scb-border_first_random (seeds 1,2,3) | 25 (25-26) | 402,761 | 252 (x86) | 0 |
| 45 | scb-chess_gacolor | 9 | > cap | 352 (m1) | 0 |
| 46 | engine-dfs-random | 0 | > cap | 447 (m1) | 0 |
| 47 | engine-dfs-spiral-out | 0 | > cap | 551 (m1) | 0 |

Incomplete at R6 (not ranked): ktam-b3 24 (24-24)/30, KM median 345,873; ktam-v1 25 (23-27)/30, KM median 159,436; ktam-v2 30 (29-30)/30, KM median 110,660

#### R7

The three A16 solvers (`bb-impact`, `bb-learned`, `bb-single`) are ranked here on their 240 s wall-clock limit, not on nodes (see Corrections).

| rank | entrant | solved/30 | KM median nodes | median wall ms (machine) | time-cens |
|---|---|---|---|---|---|
| 1 | scb-ad (seeds 1,2,3) | 27 (26-29) | 303,252 | 11,628 (x86) | 0 |
| 2 | mrv-rand (seeds 1,2,3) | 26 (25-27) | 378,131 | 13,982 (x86) | 0 |
| 3 | bb-mrv | 23 | 588,305 | 21,034 (x86) | 0 |
| 4 | mrv-bf | 22 | 650,537 | 22,835 (x86) | 0 |
| 5 | mrv-nohs | 19 | 827,773 | 26,120 (x86) | 0 |
| 6 | cmp-hall-frontier-k4 (seeds 1) | 18 | 653,790 | 10,402 (x86) | 0 |
| 7 | cmp-hall-frontier (seeds 1,2,3) | 18 (18-20) | 654,651 | 11,408 (x86) | 0 |
| 8 | scb-gacolor_ac3_random (seeds 1,2,3) | 18 (16-20) | 721,105 | 9,748 (x86) | 0 |
| 9 | cmp-hall-all (seeds 1) | 17 | 711,192 | 19,604 (x86) | 0 |
| 10 | cmp-hall-all-k4 (seeds 1) | 17 | 725,243 | 10,142 (x86) | 0 |
| 11 | scb-gacolor_ac3_lcv | 16 | 852,612 | 9,932 (x86) | 0 |
| 12 | scb-gacolor_ac3 | 14 | > cap | 10,094 (x86) | 0 |
| 13 | scb-gacolor_ac3_ns1 | 14 | > cap | 11,113 (x86) | 0 |
| 14 | scb-verhaard_preferred | 14 | > cap | 6,512 (m1) | 0 |
| 15 | scb-joe_depth150 | 14 | > cap | 6,300 (m1) | 0 |
| 16 | scb-joe_depth150_bp | 14 | > cap | 6,263 (m1) | 0 |
| 17 | scb-joe_depth150_bp_rec | 14 | > cap | 6,393 (m1) | 0 |
| 18 | scb-joe_depth150_bp_rec_layered | 14 | > cap | 6,322 (m1) | 0 |
| 19 | bb-impact | 6 | > cap | 240,008 (x86) | 24 |
| 20 | bb-single | 3 | > cap | 240,004 (m1) | 27 |
| 21 | scb-border_first_full | 3 | > cap | 2,533 (x86) | 0 |
| 22 | scb-gacolor_symbreak | 3 | > cap | 870 (m1) | 0 |
| 23 | scb-border_first_gacolor | 3 | > cap | 875 (m1) | 0 |
| 24 | scb-border_first_parity | 3 | > cap | 1,109 (m1) | 0 |
| 25 | scb-border_first_lcv | 3 | > cap | 1,162 (x86) | 0 |
| 26 | scb-rare_color_first | 3 | > cap | 1,109 (x86) | 0 |
| 27 | engine-dfs-column-major | 1 | > cap | 588 (m1) | 0 |
| 28 | bb-learned | 1 | > cap | 240,007 (x86) | 28 |
| 29 | engine-dfs-spiral-in | 1 | > cap | 805 (x86) | 0 |
| 30 | scb-naive_spiral | 1 | > cap | 1,078 (x86) | 0 |
| 31 | engine-dfs-border-first | 0 | > cap | 748 (x86) | 0 |
| 32 | engine-dfs-diagonal | 0 | > cap | 514 (m1) | 0 |
| 33 | engine-dfs-double-snake | 0 | > cap | 673 (m1) | 0 |
| 34 | engine-dfs-random | 0 | > cap | 816 (m1) | 0 |
| 35 | engine-dfs-snake | 0 | > cap | 850 (x86) | 0 |
| 36 | engine-dfs-spiral-out | 0 | > cap | 700 (m1) | 0 |
| 37 | example-dfs | 0 | > cap | 1,238 (x86) | 0 |
| 38 | fc-dfs | 0 | > cap | 140 (x86) | 0 |
| 39 | fc-luby (seeds 1,2; 3 = 2, A12 seeding) | 0 (0-0) | > cap | 136 (x86) | 0 |
| 40 | hp-dfs | 0 | > cap | 820 (x86) | 0 |
| 41 | probcut-v1 | 0 | > cap | 109 (x86) | 0 |
| 42 | probcut-v2 | 0 | > cap | 110 (x86) | 0 |
| 43 | rm-dfs | 0 | > cap | 67 (x86) | 0 |
| 44 | scb-border_first_random (seeds 1,2,3) | 0 (0-0) | > cap | 527 (x86) | 0 |
| 45 | scb-chess_gacolor | 0 | > cap | 286 (m1) | 0 |
| 46 | scb-chess_gacolor_ac3 | 0 | > cap | 425 (m1) | 0 |
| 47 | scb-naive_rowmajor | 0 | > cap | 1,063 (x86) | 0 |

Incomplete at R7 (not ranked): ktam-b3 0 (0-0)/30, KM median > cap; ktam-v1 0 (0-1)/30, KM median > cap; ktam-v2 0 (0-1)/30, KM median > cap

#### R8

The top two places are within run-seed noise, and the three A16 solvers are ranked on their 240 s wall-clock limit, not on nodes (see Corrections).

| rank | entrant | solved/30 | KM median nodes | median wall ms (machine) | time-cens |
|---|---|---|---|---|---|
| 1 | bb-mrv | 30 | 69,036 | 5,145 (x86) | 0 |
| 2 | mrv-rand (seeds 1,2,3) | 30 (30-30) | 69,526 | 7,455 (x86) | 0 |
| 3 | mrv-nohs | 30 | 102,584 | 5,121 (x86) | 0 |
| 4 | scb-ad (seeds 1,2,3) | 30 (30-30) | 113,087 | 18,972 (x86) | 0 |
| 5 | mrv-bf | 29 | 130,910 | 10,214 (x86) | 0 |
| 6 | fc-dfs | 24 | 482,370 | 53 (x86) | 0 |
| 7 | example-dfs | 24 | 483,034 | 1,190 (x86) | 0 |
| 8 | hp-dfs | 24 | 483,034 | 828 (x86) | 0 |
| 9 | rm-dfs | 24 | 483,034 | 44 (x86) | 0 |
| 10 | scb-naive_rowmajor | 24 | 483,034 | 946 (x86) | 0 |
| 11 | engine-dfs-snake | 24 | 487,206 | 854 (x86) | 0 |
| 12 | cmp-hall-all (seeds 1) | 24 | 497,523 | 41,258 (x86) | 0 |
| 13 | cmp-hall-frontier (seeds 1,2,3) | 24 (22-26) | 514,800 | 29,253 (x86) | 0 |
| 14 | scb-gacolor_ac3 | 23 | 491,738 | 37,078 (x86) | 0 |
| 15 | scb-gacolor_ac3_ns1 | 23 | 491,738 | 35,881 (x86) | 0 |
| 16 | scb-verhaard_preferred | 23 | 491,738 | 15,988 (m1) | 0 |
| 17 | scb-joe_depth150 | 23 | 492,873 | 15,726 (m1) | 0 |
| 18 | scb-joe_depth150_bp | 23 | 492,873 | 15,676 (m1) | 0 |
| 19 | scb-joe_depth150_bp_rec | 23 | 492,873 | 15,746 (m1) | 0 |
| 20 | scb-joe_depth150_bp_rec_layered | 23 | 492,873 | 15,758 (m1) | 0 |
| 21 | probcut-v2 | 22 | 504,634 | 62 (x86) | 0 |
| 22 | cmp-hall-frontier-k4 (seeds 1) | 22 | 551,930 | 37,925 (x86) | 0 |
| 23 | cmp-hall-all-k4 (seeds 1) | 22 | 565,955 | 30,392 (x86) | 0 |
| 24 | scb-chess_gacolor_ac3 | 20 | 494,015 | 12,638 (m1) | 0 |
| 25 | scb-gacolor_ac3_random (seeds 1,2,3) | 20 (19-24) | 555,342 | 30,781 (x86) | 0 |
| 26 | engine-dfs-column-major | 20 | 653,103 | 750 (m1) | 0 |
| 27 | probcut-v1 | 19 | 530,644 | 62 (x86) | 0 |
| 28 | scb-gacolor_ac3_lcv | 18 | 755,974 | 40,304 (x86) | 0 |
| 29 | fc-luby (seeds 1,2; 3 = 2, A12 seeding) | 16 (16-16) | 801,401 | 86 (x86) | 0 |
| 30 | engine-dfs-double-snake | 10 | > cap | 1,166 (m1) | 0 |
| 31 | bb-impact | 8 | > cap | 240,042 (x86) | 22 |
| 32 | bb-single | 4 | > cap | 240,009 (m1) | 26 |
| 33 | bb-learned | 3 | > cap | 240,009 (x86) | 27 |
| 34 | engine-dfs-border-first | 0 | > cap | 1,128 (x86) | 0 |
| 35 | engine-dfs-diagonal | 0 | > cap | 806 (m1) | 0 |
| 36 | engine-dfs-random | 0 | > cap | 802 (m1) | 0 |
| 37 | engine-dfs-spiral-in | 0 | > cap | 1,150 (x86) | 0 |
| 38 | engine-dfs-spiral-out | 0 | > cap | 1,034 (m1) | 0 |
| 39 | scb-border_first_full | 0 | > cap | 6,323 (x86) | 0 |
| 40 | scb-border_first_gacolor | 0 | > cap | 1,122 (m1) | 0 |
| 41 | scb-border_first_lcv | 0 | > cap | 1,710 (x86) | 0 |
| 42 | scb-border_first_parity | 0 | > cap | 1,442 (m1) | 0 |
| 43 | scb-border_first_random (seeds 1,2,3) | 0 (0-0) | > cap | 453 (x86) | 0 |
| 44 | scb-chess_gacolor | 0 | > cap | 285 (m1) | 0 |
| 45 | scb-gacolor_symbreak | 0 | > cap | 1,138 (m1) | 0 |
| 46 | scb-naive_spiral | 0 | > cap | 1,556 (x86) | 0 |
| 47 | scb-rare_color_first | 0 | > cap | 1,665 (x86) | 0 |

Incomplete at R8 (not ranked): ktam-b3 0 (0-0)/30, KM median > cap; ktam-v1 0 (0-1)/30, KM median > cap; ktam-v2 1 (0-3)/30, KM median > cap

## Reading (descriptive)

- R6: the A3-propagation family (`bb-mrv`, `scb-ad`, `mrv-*`) and A17's Hall
  forks take the top nine places; the adopted baseline `scb-gacolor_ac3_random`
  is tenth (seed-median KM median 1,291 nodes against 460 for `bb-mrv`). On 30
  seeds `bb-mrv` passes G1 against it at R6 (0.468); A22 measured 0.80 on
  seeds 21..30 alone.
- R7: `scb-ad` (27/30, range 26 to 29) and `mrv-rand` (26/30) solve more than
  `bb-mrv` (23/30) and the adopted baseline (18/30, range 16 to 20), without a
  G1 pass. `mrv-rand` (the run-seed control of `bb-mrv`'s value order) against
  `bb-mrv`, paired G1 per run seed: R7 0.906 [0.438, 1.428], 0.881 [0.496,
  1.190], 0.903 [0.585, 1.000]; R8 1.623 [0.890, 2.118], 0.847 [0.399, 2.596],
  0.495 [0.409, 0.768]. No detectable effect of the random value order at R7 or
  R8 on 30 instances and 3 run seeds.
- R8: `bb-mrv` stays first (30/30, KM median 69,036). Plain row-major DFS
  (`hp-dfs`, `rm-dfs`, `fc-dfs`, `example-dfs`, `scb-naive_rowmajor`) solves
  24/30 in about 1% to 23% of `bb-mrv`'s median wall clock per run, ranking
  above the former baseline `scb-gacolor_ac3_random` (20/30). This is a result
  about one cell order, not about plain DFS: row-major (and snake) solve 24/30,
  column-major 20, double-snake 10, and spiral-in, spiral-out, border-first,
  diagonal and the random path 0. The random path has one path seed (0), so
  cell order has no run-seed control.
- Duplicates confirmed node for node: `scb-gacolor_ac3` = `scb-gacolor_ac3_ns1`
  = `scb-verhaard_preferred` at every rung; the four `joe_depth150*` presets
  are identical to each other; only the KM medians are within 0.3% of
  `gacolor_ac3` (0.27% at R6, 0.23% at R8); per instance they differ by up to
  0.37% (R6), 1.08% (R7), 0.55% (R8), and match it on only 18/30, 17/30, 7/30
  rows;
  `scb-naive_rowmajor` = `hp-dfs` = `rm-dfs` = `example-dfs`; `scb-naive_spiral`
  = `engine-dfs-spiral-in`.
- The incomplete A15 arms (`ktam-*`) solve 24 to 30 of 30 at R6 and at most
  3 of 30 at R7 and R8; they are not ranked.

## Corrections made after the judgment

Made after the judge report (`JUDGE.md`, 1f4e211); none touches a
preregistered statistic or a result row.

- The earlier sentence "Random value order helps at R7 and costs at R8" is
  replaced by the paired numbers in the R7 reading above. The R8 "cost" seen
  in the tables is `scb-ad` (1.18 to 1.52), a different solver, not `bb-mrv`
  with a random value order.
- `fc-luby` has two distinct run seeds, not three. A12 seeds its generator with
  `(seed ^ name_hash) | 1`, so seeds 2 and 3 differ only in bit 0, which `| 1`
  overwrites: they are the same run (row-identical at R6, R7 and R8). Its
  run-seed median is seed 2's value. The defect is in A12's code
  (`research/experiments/probcut`) and is not fixed here; the same collision
  affects seeds 0 and 1 and any pair 2k, 2k+1.
- The `joe_depth150*` presets are within 0.3% of `gacolor_ac3` on KM medians
  only (see Reading); per instance they differ by up to 1.08%.
- First place and second place are within run-seed noise at R6 and R8.
  `bb-mrv` is one fixed tie-break order (a single draw). At R8 its KM median
  69,036 lies inside `mrv-rand`'s seed range (46,568 to 107,496); at R6 its 460
  lies inside `scb-ad`'s (422 to 472). Adjacent ranks between a deterministic
  entrant and a stochastic one with the same propagation are not a measured
  difference.
- At R7 and R8 the three A16 solvers (`bb-impact`, `bb-learned`, `bb-single`)
  are ranked on their 240 s wall-clock limit, not on nodes: they are
  time-censored on 24, 28, 27 of 30 rows at R7 and 22, 27, 26 at R8, under
  shared load (`bb-single` on m1, the other two on the x86 laptop). Their R7 and
  R8 ranks measure the guard on that machine, not node efficiency.
- Capped runs are scored at the cap of 1,000,000 nodes, but the solver keeps
  assigning for a while after the counter trips: `nodes_counted` exceeds the
  cap on 1,268 censored rows, by up to 6,715 nodes (`scb-chess_gacolor`; most
  entrants under 200). Harmless under the cap rule; no solved row is affected.
- The screening flags against the R6 baseline are conditional on its run seed
  1. `bb-mrv` against `scb-gacolor_ac3_random` at R6 gives 0.468 against seed
  1, 0.517 [0.243, 0.772] against seed 2 and 0.551 [0.264, 0.996] against seed
  3 (the last two do not pass 0.5). A confirmation should pair against all
  three seeds.
- Row-major DFS at R8 solves 24/30 in about 1% of `bb-mrv`'s median per-run
  wall (same machine). On a wall-clock budget the R8 ranking would change;
  that is a separate measurement.
- No `results/run-log.jsonl` exists: `prereg_guard.py` was added to main after
  these runs started and is not on this branch. Preregistration order rests on
  commit order and file times (judge report, section 1).

## Excluded components

The full list with reasons is the "Excluded components" table in
`PREREG.md`, unchanged: the `*_par` presets (internal threads), Blackwood and
Verhaard engines and standalone 16x16 binaries, the edge-variable solver, the
SAT encoder, local search (repair-study, quality-diversity-basins,
localsearch), the break-tolerant engine and dfs-study registry, the
planted-aware A3/A10/A1 tools, the A5 frame counter, BANDSAW, hint-study and
fill-order-mean-field, A15's non-selected grid configurations, test fixtures
and tonight's Track 1 pilot.


## How the numbers were produced

- Lockfile: `compute/Cargo.lock` is the crate's own (the crate declares an empty
  `[workspace]`, so the repository's `engine/Cargo.lock` does not govern it).
  The lockfile first committed was stale: it lacked the crate and its four
  path-dependency crates and carried an unused registry stack (ort, ndarray,
  openssl and others). Building rewrote it to what the graph actually
  resolves to, with no change to any retained version; that file is now
  committed and `cargo build --locked` passes.

- Runner: `compute/` (crate `baseline-tournament`, binary `tournament`). It
  path-depends on hard-ladder, mrv-baseline (and its engine fork),
  backdoor-branching, probcut, ktam-unassignment, cut-multiset-pruning's engine
  fork and the single-core-benchmark engine; no file of those experiments is
  edited. A17's entries live in its binary, so the runner repeats the same
  constructor call; the identity table is the proof that it is the same solver.
- Raw rows: `results/raw/<entrant>/s<run seed>-<rung>.jsonl` (hard-ladder
  `RunRecord` plus `entrant`, `rung`, `source`, `complete`, `stochastic`,
  `guard_s`, `time_censored`, `machine`, `cpu`, `load`).
- Analysis: `analysis/analyze.py` (stdlib only) writes `results/tables.md`,
  `results/summary.json` and `results/identity.csv`. Every number in this README
  is in `results/tables.md` or `results/summary.json`.
- Order and timing: `run.sh` (preregistered tier order, launch deadline 22:00);
  per-entrant wall clock in `results/stage-times.txt`, entrants not launched
  before the deadline in `results/not-reached.txt`.

```
just experiments baseline-tournament          # build + run (resumes; skips existing files) + analyze
just experiments baseline-tournament analyze  # tables from the committed raw rows
```

## Hardware

Machines: `x86` (Intel Core Ultra 7 165U, 14 threads, 30 GB; 4 threads,
plus a 1-thread process under amendments 2 and 3, while up to 8
single-thread runs of Track 1 shared it) and, from 19:52, `m1` (Apple M1,
4 performance + 4 efficiency cores; 3 threads plus a 2-thread process, shared
with Track 1). Every row records its machine. Runs 19:12 to 22:17 local. Built with cargo 1.99.0 inside Nix, `--release`, `-j 4`. Wall clock is
indicative only and nodes/s is not comparable with other experiments; node
counts are machine-independent except for rows stopped by a wall-clock guard
(A22 arms 600 s, A16 arms 240 s), which are flagged `time_censored`.
