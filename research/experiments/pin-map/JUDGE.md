# pin-map: judge's verdict

Branch `night/2026-11-13-d5-pin-map`, head fd8089e9. Judged from the branch
only. Track label: measurement (calibration, no kill line), so there is no
method target to pass; the questions are whether the numbers are right,
whether the README says more than the data, and whether the extrapolation is
kept apart.

## Verdict: keep (numbers), with README corrections required

Every headline number I recomputed from the committed rows matches the
README to its stated precision (one rounding slip, below). A fresh build of
the engine reproduces the committed node, solution and first-solution counts
exactly on every rerun unit, and an enumerator I wrote myself (C, not derived
from the branch code) agrees on every pinned tree I gave it. Preregistration
order is clean. The measurements stand. Seven README sentences go beyond the
data and must be weakened before the page is used as a source; none changes
a number.

## Preregistration and run log

- `PREREG.md` committed at 559c16b8, 2026-11-13 09:49:37 UTC, before every
  result commit (`git log --follow`: one commit, no later edits).
- `results/run-log.jsonl`: first test-stage line (`boards`) 09:51:56 UTC,
  2 min 19 s after the PREREG commit; all 15 start lines carry
  `prereg_sha256` 05cbca62..., the hash of the committed file.
- Every result file has a guarded stage line. Two `p10` start lines have no
  end line; both are the stops disclosed in A1 and its addendum.
- `engine/` and `scripts/drive.py` are byte-identical from a3908d86 (before
  the first run) to head, so the code that ran is the committed code. Three
  start lines show `dirty_paths: 1` (mrv8, diag, second p10); the only
  tracked non-result files that changed in that window are `analyze.py` and
  `PREREG_AMENDMENTS.md`, neither of which produces rows.
- `calib/calib.jsonl` was not run through the guard; it uses seeds 9001..,
  disjoint from the measured seeds, and is what the PREREG cites.
- `diag` (M10 boards 2 and 5, two solutions) is not a preregistered stage;
  it is a disclosed diagnostic and feeds no headline number.
- Deviation not declared as one: PREREG item 2 asks for (median, mean) by
  distance; the README reports a worth-weighted "aggregate ratio" (sum of
  pair worths over sum of single worths) by distance and the median only
  overall. Means by group are below; they do not change the conclusion.

## Recomputed numbers (own script, committed raw rows)

All match the README unless marked.

Item 1, single tiles:
- M8 fwd: mean 2.085; rows 3.87, 5.36, 3.40, 1.48, 0.66, 0.55, 0.54, 0.81;
  top cells 10, 11, 12 at 6.42, 6.20, 6.14; corner/frame/interior 1.53,
  2.20, 2.07; median per-cell sd 0.282; log2 base mean 25.56.
- Direction: r(fwd, rev rotated 180) 0.9980 (M8), 0.9946 (M10); unrotated
  -0.6426, -0.2276.
- N8: r with M8 0.9598; rows 0..3 3.94, 5.83, 4.94, 2.69; rows 5..7 0.77,
  0.74, 0.85; trees 1.9e8 to 4.9e8.
- M10 fwd: rows 0, 1 4.29, 4.49; rows 3..8 0.17 to 0.28; frame 1.83,
  interior 0.84.
- M8C: rows 0, 1 3.80, 3.35, rest 0.15 to 0.59; frame 1.68, interior 0.77.
- Frame effect outside head rows: M10 rows 3..8 frame-column cells 0.47 to
  0.58 against interior 0.10 to 0.21; M8C rows 3..6 0.47 to 0.52 against
  0.04. Both edge columns carry it (not only column 0, the first cell of each
  row in the scan).
- Solution counts: M8 board 4 leaves 2 solutions for 60 of 64 single pins
  (README: "2 for some cells"); M8C board 4 leaves 2 for 55 of 59 (not
  mentioned); M10 boards 2 and 5 leave 2 for all 100. Every other board: 1.

Item 2, pairs (10,080 pairs, 5 boards): median ratio 0.957 (mean 0.937).
Aggregate / median / mean by group: adjacent 0.753 / 0.836 / 0.825;
distance 2 0.854 / 0.912 / 0.887 (README says 0.86, which is the
different-rows distance-2 cell 0.8595, not the distance-2 value); distance 3
0.914 / 0.937 / 0.920; distance >= 4 0.958 / 0.966 / 0.955; same row
0.847 / 0.922 / 0.878; different rows 0.942 / 0.959 / 0.945.

Item 3, thresholds table: all 21 M8 and M8C cells and all 21 M10 cells
reproduced exactly (M10 policies (b) to (g) on boards 1 and 2, confirmed as
the only boards with all 600 rows; board 3 has 26 rows and is excluded).
Greedy reaches the 1e6 cut on 10 of 20 M8 boards. k = 4 mean log2 nodes
(M8): greedy 9.04, clustered 11.46, frame 18.42, random 18.68, scan-spread
18.75, lattice 20.89, interior 20.86. Clustered 8.95 at k = 8 to 8.87 at
k = 16; greedy 6.64 at k = 8, 5.62 at k = 16. First 4 greedy picks in rows
0 and 1: 0.838. Column-0 cell of row r among the first 16 picks: 0.70, 0.30,
0.55, 0.45, 0.50, 0.60, 0.50, 0.30. Item 5: first solution at k = 4, greedy
8.05, clustered 10.29, random 17.26.

Item 4, bb-mrv (890 runs): censoring as stated (k = 1 under cap: greedy 7,
clustered 4, scan-spread 1, random 11 of 50, others 0; from k = 3 on, one
frame run at k = 3 and one random draw at k = 4). k = 4 means 6.13, 7.08
(lattice), 7.58 (clustered), 8.28, 8.96, 9.81, 11.10; row scan on boards
1..10: 9.08, 11.68, 18.38, 18.50, 19.34, 20.94, 20.97. Paired per board:
at k = 4 lattice beats clustered on 5 of 10 boards (mean -0.50 bits),
interior beats clustered on 3 of 10; scan-spread beats clustered on 1 of 10.
At k = 6 lattice and interior beat clustered on 9 of 10 (mean -0.55 bits).

Compute: end-line durations sum to 3,617 s as stated.

## Reruns (fresh build, `cargo build --release -j 1`, nice 15, one core)

1,177 units checked, 0 mismatches (about 8.5 core-minutes): board files
regenerated byte-identical (M8 1 and 13, M10 2, N8 21); full singles passes
s8:1:fwd, s8:13:fwd, s8:7:rev, s8c:4, n8:21, s10:2:fwd (every cell's nodes,
solutions, first); greedy chains g8 board 3, g8c board 9, g10 board 4 (every
k, plus chain integrity: each k+1 base is the k base plus the argmin pick);
every k >= 2 policy tree of p8 board 5 and p8c board 11 (375 each); p10
board 2, k <= 6 (125); five pairs8 rows of board 2; every bb-mrv run of
boards 1 and 6 (89 each, nodes, solutions, first and propagations).

Independent enumerator (`myenum.c`, own code, same tree definition): base
tree of M8 board 1 fwd 64,893,775 nodes, 4 solutions (matches); cell 11
pinned 634,426 (matches). Further 35 trees, 0 mismatches on nodes,
solutions and first-solution node: single pins in fwd and rev (M8 boards 1,
4, 9, 17), greedy k = 1..8 on M8 board 3, all six deterministic and two
random policy sets at k = 4 and 12 on M8 board 14, the M8C clue-only tree and
three clue+pin trees on board 1, two M10 single pins (boards 2 and 5, both
2 solutions, the larger 472,360,298 nodes), and three pairs on M8 board 4.
This is the check that `count_multi` (one pass for all pins) is right
independently of the branch's own validation.

## README sentences that go beyond the data

1. Finding 1: "At M10 and M8C the frame averages exceed interior ... only
   because the top frame row is the head of the scan." Contradicted by the
   sentence added after it in fd8089e9 and by the data: excluding rows 0..2,
   M10 frame+corner 0.58 against interior 0.13, M8C 0.54 against 0.04. Delete
   "only because ..." (and "frame against interior is not the variable" for
   M8 should read "is not the main variable": M8 rows 4..6 also show frame
   0.74 to 0.88 against interior 0.46 to 0.59).
2. Finding 2: "Clustering costs about 15 to 25 percent of the second tile's
   worth when the tiles touch." The ratio is over the sum of both worths, so
   1 - 0.75 is 25 percent of the pair's summed worth, which for two equal
   tiles is about half of the second tile's worth. Fix: "costs 15 to 25
   percent of the two tiles' summed worth (median 0.84, aggregate 0.75)".
3. Finding 3 title: "Best rule: ... measured greedy". Measured greedy picks
   each cell by the exact tree with the true tile, so it is a per-board oracle
   bound, not a rule usable without the solution. Say so; the usable rule is
   (d), whose row ranking leaks the measured board (disclosed).
4. Finding 4: "The large placement effect of item 3 is mostly a property of
   the row scan, not of the puzzle." One adaptive solver, 10 boards, one
   colour count above critical. Scope it: "at 8x8, ci 11, under bb-mrv".
5. Finding 4: "after 3 to 4 known tiles the count matters and the place
   barely does". At k = 3 bb-mrv policy means span 6.79 to 13.58 bits, at
   k = 4 6.13 to 11.10 (frame and scan-spread 3.7 to 5 bits above greedy).
   The convergence the data show is by k = 6 (all but frame) and k = 8 (all).
6. Finding 4: "The lattice set goes from last under row scan to second under
   bb-mrv". On means only; per board lattice beats clustered 5 of 10 at
   k = 4. Write "ties with clustered at k = 4, beats it from k = 6".
   "Clustered is no longer better than spread" holds for the lattice set from
   k = 6, not for scan-spread at k = 4 (clustered better on 9 of 10 boards).
7. Extrapolation: "which is what measured greedy did on every board" and
   "bb-mrv makes any 3 to 6 known tiles nearly equivalent because propagation
   from them reaches the whole small board". Greedy's first 16 picks include
   a row's column-0 cell on 30 to 70 percent of boards, not all; the 3-tile
   spread is 7 bits; propagation reach was not measured, so "because" is
   argued. Fix: "what measured greedy did on most boards"; "nearly equivalent
   from 6 known tiles"; drop or label the mechanism as a guess.

Minor: setup "M8 board 4 (2 for some cells)" should read 60 of 64, and M8C
board 4 (55 of 59) is missing; "0.86 at distance 2" should be 0.85; the
`main.rs` header says `tree.rs` "gains `count_fast` only" but it also gains
`count_multi`.

## Is the extrapolation kept apart?

Mostly. Item 5 is its own headed section, labelled "not a measurement", and
its numbers are taken from items 1 to 4 correctly (sd 0.3, rows 4..7 0.5 to
0.8, rows 3..8 0.2 to 0.3, pair additivity 75 to 95 percent, a quarter of
the bits at 8x8). Two leaks: Finding 4's last sentences (points 4 and 5
above) are generalisation inside the measurement section, and the
extrapolation's two overstated sentences (point 7) restate measured facts
more strongly than the measurements show. The 16x16 claims themselves
("widest layers lie far deeper", "one tile removes a smaller share") are
argued, not measured, and labelled as such.

## Checklist items

- Random-order control: not applicable (no ordering method claimed). The
  fixed-order baseline is the object of study, and item 4 is the needed
  second tree measure.
- Planted-solution leakage: inherent and intended (pins are planted tiles);
  greedy's oracle character is point 3.
- Node accounting: same tree definition for every arm under row scan; bb-mrv
  counts options tried, a different metric, never compared in absolute terms
  with row scan except as rankings.
- Censored runs: bb-mrv censored runs counted at the cap and marked; no
  conclusion rests on k <= 2 bb-mrv means.
- Dropped seeds: p10 boards 3..5 (A1, disclosed, policies (b) to (g) at 10x10
  rest on 2 boards; the README says so in the table note).
- Wall clock: not a claim here.
- Symmetry check (unplanned): the 4-fold rotated reference does not explain
  the late-row worth at M8. Conditional on one row-1 pin (symmetry already
  broken), rows 4..7 cells keep 0.53 bits on average against 0.66 unconditional
  (1,280 pairs, boards 1..5).
- Site pages: no verdict here leans on the explored or dead-end pages. The
  "Owen-critical about 8 at 8x8" figure is reported (from the hard ladder),
  not measured here.

## What I did not check

- bb-mrv reruns cover boards 1 and 6 only; I did not reimplement bb-mrv.
- The 10x10 rev map, boards M10 1, 3, 4, 5 singles, and N8 boards 22..25
  were not rerun (only recomputed from rows).
- Figures were not regenerated or inspected pixel by pixel.
- The calibration rows were not rerun.
