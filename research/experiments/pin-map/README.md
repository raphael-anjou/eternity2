# pin-map: what a known tile is worth, by position, on small boards

Track D5, arc 2026-11-13. Label: measurement (a calibration, no kill line).
Generated boards only; nothing here is official Eternity II progress.

**Question.** On small planted boards built like the real puzzle, how many
bits of the exact search tree does knowing the true tile of one cell remove,
and how does that depend on where the cell is? Given k known tiles, which
placement rule leaves the smallest exact tree?

Everything below items 1 to 4 is exact (full enumeration, no sampling).
Item 5 is extrapolation. Preregistration: `PREREG.md` (commit 559c16b8),
two amendments: `PREREG_AMENDMENTS.md` (A1, 10x10 policies cut to board 1 of 5
for time; board 2 was completed afterwards, boards 3 to 5 were not run; A2, a
reporting deviation: item 2 gives a worth-weighted aggregate ratio by distance
where the PREREG asked for median and mean, and the median only overall).

## Setup in one paragraph

Boards: Track D3's planted generator (frame joints from cf colours, inner
joints from ci other colours, tiles pairwise distinct up to rotation,
rotations allowed). Sets: M8 = 8x8, cf 4, ci 11, seeds 1..20; M8C = the same
20 boards with five clues fixed like the real puzzle's (cells (2,2), (2,5),
(5,2), (5,5), centre (3,4)); M10 = 10x10, cf 5, ci 18, seeds 1..5; N8 = 8x8,
ci 10 (nearer critical), seeds 21..25, single-tile map only. Colour counts
were chosen on calibration seeds 9001.. (`calib/calib.jsonl`): ci 11 is the
smallest interior count at 8x8 whose trees enumerate in seconds (1.7e7 to
7.3e7 nodes); the Owen-critical count is about 8, so M8 sits above critical,
and N8 (trees 1.9e8 to 4.9e8) checks the pattern closer to it. Search tree:
row scan, row 0 first ("fwd"), or the exact reverse ("rev"); a node is every
consistent partial board (the cost of exhausting). Worth of a pin set =
log2(reference tree) - log2(tree with those cells fixed to their true tiles).
Unpinned boards have 4 solutions (the whole board rotated); one pin leaves 1,
except M8 board 4 (2 for 60 of 64 cells), M8C board 4 (2 for 55 of 59
cells) and M10 boards 2 and 5 (2 for every
single pin, 1 once all but one row or column is pinned:
`results/diag.jsonl`).

The engine (`engine/`, copied from `orders-and-pins/engine/src` at ba0b77b1)
adds `count_multi`: one pass over the tree with pin set S gives the exact tree
of S + {p} for every p, by tracking which pins stay consistent with the
partial board. `scripts/validate.py` checks it against direct enumeration by
the original enumerator on two calibration boards, both directions, singles
and pairs: node counts, solution counts and nodes-to-first-solution all
identical (`sh run.sh validate`, 0 mismatches).

## Figures

- `results/figures/F1-single-tile-map.svg` (and `.png`): mean bits per known
  tile for each cell, six panels (M8 fwd, M8 rev, M8C, N8, M10 fwd, M10 rev).
  `F1b-single-tile-rank` is the same as ranks.
- `results/figures/F2-remaining-vs-k.svg`: remaining exact tree (log2 nodes)
  against k for the seven placement rules, M8, M8C, M10.
- `results/figures/F3-greedy-choice-map.svg`: how often measured greedy picks
  each cell among its first 4, 8, 16 tiles.
- `results/figures/F4-pair-additivity.svg`: pair worth over the sum of the two
  single worths, by distance, same row against different rows.
- `results/figures/F5-bbmrv-vs-rowscan.svg`: the same pin sets under row scan
  and under bb-mrv run to exhaustion.

Tables behind them: `results/tables/*.csv`; every number below is in
`results/summary.json` (written by `scripts/analyze.py` from the raw rows
`results/*.jsonl`).

## Findings

1. **The worth of a tile is set by its place in the scan order, not by the
   board.** M8, scan from row 0: mean single-tile worth 2.08 bits, but row
   means fall from 3.87, 5.36, 3.40 bits (rows 0, 1, 2) to 0.54 to 0.81 bits
   (rows 4 to 7); the best cells are in row 1 (cells 10, 11, 12: 6.1 to 6.4
   bits). Scanning from the other side flips the map: the fwd mean map and
   the rev mean map rotated 180 degrees correlate at r = 0.998 (M8) and
   0.995 (M10); unrotated, r = -0.64 and -0.23. Corner, frame and interior
   averages are nearly equal at M8 (1.53, 2.20, 2.07 bits): frame against
   interior is not the main variable, scan position is (M8 rows 4 to 6 still show
   frame cells at 0.74 to 0.88 against interior 0.46 to 0.59). At M10 and M8C the
   frame averages exceed interior (1.83 against 0.84; 1.68 against 0.77), in part
   because the top frame row is the head of the scan. Outside the head rows there
   is one frame effect: frame cells keep more worth than interior cells (M10
   rows 3 to 8: 0.4 to 0.6 bits against about 0.1; M8C rows 3 to 6: 0.5
   against 0.0 to 0.1; F1). Spread over boards is
   small (median per-cell sd 0.28 bits at M8). N8 (ci 10) has the same shape
   (r = 0.96 with M8; rows 0..3 at 3.9, 5.8, 4.9, 2.7 bits, rows 5..7 at
   0.74 to 0.85). M10: rows 0, 1 at 4.3 and 4.5 bits, rows 3 to 8 at 0.17 to
   0.28. With the five clues (M8C) the head rows still dominate (3.8, 3.35)
   and the rest of the board drops to 0.15 to 0.6 bits.
2. **Pairs are close to additive except when adjacent.** M8 boards 1..5,
   10,080 pairs: median ratio worth(pair) / (worth(p) + worth(q)) 0.957;
   aggregate ratio 0.75 for adjacent cells, 0.85 at distance 2, 0.96 at
   distance 4 and more; same row 0.85 against 0.94 for different rows
   (`results/tables/pairs_additivity_M8.csv`). Clustering costs 15 to 25
   percent of the two tiles' summed worth when the tiles touch (median 0.84,
   aggregate 0.75), little beyond.
3. **Best rule: put the tiles at the head of the scan; measured greedy does
   that and wins by a wide margin.** Measured greedy picks each cell by the
   exact tree with the true tile, so it is a per-board oracle bound, not a rule
   usable without the solution; the usable rule is (d), whose row ranking uses
   the measured boards (see "What is exact and what is not"). Tiles needed (median over boards):

   | rule | M8: cut 1e3 | M8: under 1e4 nodes | M8C: cut 1e3 | M8C: under 1e4 | M10: cut 1e3 | M10: cut 1e6 | M10: under 1e4 |
   |---|---|---|---|---|---|---|---|
   | (a) measured greedy | 2 | 2.5 | 3 | 2 | 2 | 6 | 3 |
   | (d) clustered in best rows | 3 | 3 | 4 | 3 | 2 | 9.5 | 4 |
   | (c) 2D lattice | 5 | 7 | 8 | 7 | 7.5 | not by 24 (2 of 2) | 14.5 |
   | (e) frame only | 5 | 7 | 7 | 6 | 7 | 13 | 9 |
   | (b) spread along scan | 7 | 9 | 10 | 7 | 10 | not by 24 (1 of 2) | 15.5 |
   | (g) random | 7 | 9.5 | 11 | 8 | 11 | not by 24 (2 of 2) | 21 |
   | (f) interior only | 10 | 11 | 12 | 10 | 17 | not by 24 (2 of 2) | 18 |

   M10 columns for (b) to (g) rest on boards 1 and 2 only (A1); greedy on all
   5. "not by 24 (j of 2)": j boards never reach the cut within 24 tiles.
   Medians are over boards (`results/tables/thresholds_*.csv`).
   A cut by 1e6 is not reachable at 8x8 by construction: the unpinned tree is
   2^25.5 (M8) or 2^20.8 (M8C) and any tree keeps at least one node per free
   cell (greedy flattens at 2^5.6, about 50 nodes, by k = 16), so a 1e6 cut
   would need almost the whole excess removed; greedy reaches it on 10 of 20
   M8 boards. Remaining tree at k = 4 (mean log2 nodes, M8, from 25.6):
   greedy 9.0, clustered 11.5, frame 18.4, random 18.7, scan-spread 18.8,
   lattice 20.9, interior 20.9 (`results/tables/policies_M8.csv`).
   **Clustered beats spread.** On row scan the "clustered pins waste
   information" intuition is wrong at small k: the head rows are worth so much
   more than anything else that filling them beats any even spread, despite
   the 15 to 25 percent pair overlap. Clustered does saturate: M8 stalls near
   2^8.9 from k = 8 on, while greedy keeps going (2^6.6 at k = 8, 2^5.6 at
   k = 16) by moving on to column 0, the first cell of each later row (F3:
   among the first 16 picks, column 0 is chosen on 30 to 70 percent of
   boards in every row; the first 4 picks are 84 percent in rows 0 and 1).
4. **bb-mrv:** the adaptive solver erases most of the placement effect,
   and reverses part of it. M8 boards 1..10, the same pin sets, bb-mrv
   (A3 propagation, dynamic min-domain cell choice) run to exhaustion, cap
   1e5 nodes (`results/tables/mrv_vs_rowscan_M8.csv`, F5). Unpinned, all 10
   runs pass the cap. At k = 1 most runs are still censored (under the cap: greedy's
   first cell on 7 of 10 boards, clustered 4, scan-spread 1, random 11 of 50
   draws, lattice, frame and interior none). From k = 3 on only two runs are
   censored (one frame set at k = 3, one random draw at k = 4), and every
   policy converges to the floor of about one node per free cell (2^5.6 to
   2^6.4, nodes being options tried) by k = 8, all but frame-only
   already by k = 6, while under row scan the same sets still differ by up to 13 bits at k = 6
   (greedy 7.4 against interior 20.9). Order at k = 4, bb-mrv mean log2
   nodes: greedy 6.13, lattice 7.08, clustered 7.58, interior 8.28, random
   8.96, scan-spread 9.81, frame 11.10; row scan on the same boards: greedy
   9.08, clustered 11.68, frame 18.38, random 18.50, scan-spread 19.34,
   interior 20.94, lattice 20.97. The lattice set goes from last under row scan
   to second on means under bb-mrv (interior from sixth to fourth); per board it
   ties with clustered at k = 4 (better on 5 of 10 boards, mean -0.50 bits) and
   beats it from k = 6 (lattice and interior sit at 5.87 against 6.41 for
   clustered, better on 9 of 10 boards), and at k = 16 clustered is the worst
   set (5.88). Scan-spread does not beat clustered at k = 4 (clustered is better
   on 9 of 10 boards). At 8x8, ci 11, under bb-mrv, the placement effect of item
   3 mostly disappears from k = 6 (all sets but frame-only) and k = 8 (all
   sets); at k = 3 and 4 the sets still span 6.8 to 13.6 and 6.1 to 11.1 bits.
   The greedy set here is greedy for row scan, not for bb-mrv; a
   bb-mrv-greedy was not run.
5. Nodes to the first solution follow the tree: at M8 k = 4, greedy reaches
   the first solution after 2^8.1 nodes, clustered 2^10.3, random 2^17.3
   (mean log2, `summary.json` `at_k`).

## Extrapolation to 16x16 (not measured here)

This paragraph is extrapolation from 8x8 and 10x10, not a measurement.
For a row-scan (or any fixed-order) search on the 16x16, the small boards
say: the worth of a known tile is set by where its cell sits relative to the
part of the scan where the exact tree is wide, it is almost the same from
board to board (sd about 0.3 bits per cell here), and it collapses to a
fraction of a bit once the scan front has passed that part (rows 4 to 7 at
8x8 give 0.5 to 0.8 bits, rows 3 to 8 at 10x10 give 0.2 to 0.3). The
placement rule that should carry over is therefore "fill the cells the scan
reaches just before and inside its widest layers, starting with the first row
or two of that region, then the first cell of each following row", which is
what measured greedy did on most boards (a row's column-0 cell is among its
first 16 picks on 30 to 70 percent of boards); clustering inside that region is
cheap (pairs keep 75 to 95 percent additivity), spreading tiles evenly over
the board or the scan is not. The bits per tile should not carry over as
numbers: at 8x8 the whole unpinned tree is only 2^25.5 nodes and a single
tile can remove a quarter of its bits, while on the 16x16 the tree is many
orders larger and its widest layers lie far deeper in the scan, so one tile
removes a smaller share, and its absolute worth depends on how many layers
its constraints reach, which a small board cannot show. Three things cannot
be read from small boards at all: where the 16x16 tree is widest (that is a
measurement on the 16x16 itself), how the five official clues interact with
the widest layers there (at 8x8 the clue set kept the same head-of-scan
pattern), and whether an adaptive solver on the 16x16 also washes out
placement: at 8x8, ci 11, bb-mrv makes the placement of 6 or more known tiles
nearly irrelevant (the sets converge by k = 6 except frame-only; the spread at
3 tiles is 7 bits). A plausible guess, not measured, is that
propagation from the tiles reaches the whole small board; that need not happen
on a board with four times as many cells and near-critical colours. The 8x8
result rests on one adaptive solver, 10 boards and one colour count above
critical, so it says the large placement effect of item 3 is largely a property
of the row scan at that size, not that it is for the puzzle in general.

## What is exact and what is not

Exact: every node count, solution count and first-solution count in items 1
to 4 (complete enumeration; censored bb-mrv runs are marked and counted at
the cap, which understates them). Not exact: nothing is sampled, but the
results are for these generated boards, these colour counts (above critical
at both sizes, far above at 10x10) and these two tree measures. The policy
(d) ranking of rows uses the set's own mean single-tile map (all boards,
including the board measured), a mild leak that favours (d); greedy uses
only the board itself.

## Reproduce

```
just experiments pin-map build
just experiments pin-map validate
just experiments pin-map all        # about 1.6 core-hours on one core
just experiments pin-map analyze    # tables, summary.json, figures
```

## Compute

One core of an Apple-silicon Mac shared with two other tracks, every
process under `nice -n 10`, single-threaded. Guarded stages
(`results/run-log.jsonl`, sum of end-line durations): s8 357 s, g8 174 s,
p8 368 s, pairs8 602 s, s8c 6 s, g8c 15 s, p8c 34 s, s10 652 s, g10 318 s,
n8 181 s, mrv8 910 s, diag 0.2 s; total 3,617 s. Plus the two killed p10
runs (no end line; about 14 minutes together, boards 1 and 2), calibration
and validation (about 10 minutes). About 1.4 core-hours in all; wall clock
1 h 17 min from the PREREG commit (09:49 UTC) to the last run, about
1 h 50 min with setup and calibration. Not run:
p10 boards 3 to 5 (A1), a bb-mrv-greedy policy, bb-mrv on the clue set and
at 10x10.
