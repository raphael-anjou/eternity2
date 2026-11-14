# pin-map: what is a known tile worth, by position, on small planted boards

Track D5, arc 2026-11-13. Track label: **measurement** (a calibration; there is
no kill line and no method claim). Generated-board results are never official
Eternity II progress.

## Question

On small boards built like the real puzzle, how many bits of exact search tree
does knowing the true tile of one cell remove, as a function of the cell; how
far from additive are two such pins; and which rule for placing k known tiles
shrinks the remaining exact tree fastest?

## Boards

Generator: `generate(n, cf, ci, seed)` from
`research/experiments/orders-and-pins/engine/src/puzzle.rs` (copied at commit
ba0b77b1): frame joints (between two frame cells along the rim) use colours
1..cf, every other inner joint cf+1..cf+ci, rim sides 0; pieces shuffled and
randomly rotated. Board "seed s" is the first attempt a = 0, 1, ... with
generator seed `s*1000+a` whose n^2 tiles are pairwise distinct up to rotation.
Rotations are allowed in the search. Because the rim is fixed and tiles rotate,
every unpinned board has its planted solution in 4 rotated copies: "unique" below
means 4 solutions unpinned, 1 once any tile is pinned.

| set | n | cf | ci | seeds | clues |
|---|---|---|---|---|---|
| M8 (main) | 8 | 4 | 11 | 1..20 | none |
| M8C (clue set) | 8 | 4 | 11 | 1..20 (same boards) | 5, see below |
| M10 (scale check) | 10 | 5 | 18 | 1..5 | none |
| N8 (nearer critical, singles only) | 8 | 4 | 10 | 21..25 | none |

How the colour counts were chosen (calibration seeds 9001.., disjoint from the
measured ones; rows in `calib/calib.jsonl`): unpinned row-scan trees at 8x8,
cf = 4: ci = 8 > 3e8 nodes on 3/3 boards; ci = 10: 7.9e7 and > 3e8 (2/3 past
3e8); ci = 11: 1.7e7 to 7.3e7, all 5 unique, 0.8 to 3.6 s per exact pass;
ci = 12: 8.8e6 to 3.5e7, unique. The Owen-critical interior count at 8x8 is
about 8 (the hard ladder's primary 8x8 rung), so ci = 11 is above critical: it
is the smallest count whose trees enumerate in seconds on one core, which the
greedy and pair measurements need (hundreds of exact passes per board). N8
(ci = 10, single-tile map only) checks that the single-tile pattern holds
nearer critical. 10x10, cf = 5: ci = 16 > 1.5e9 on the one board tried;
ci = 18: 1.7e8 and 3.4e8, unique, 6 to 13 s per pass. M10 uses ci = 18. This is
far above critical at 10x10 (about 10); 10x10 is a scale check of the
pattern, not a near-critical measurement.

Clues (M8C), placed like the real puzzle's five (16x16: (row,col) = (2,2),
(2,13), (13,2), (13,13) and the centre clue at row 7, col 8): at 8x8 the four
corner clues sit on the diagonal with one interior cell between them and the
frame, (2,2), (2,5), (5,2), (5,5), and the centre clue at (3,4). Cells
pos = row*8 + col: 18, 21, 42, 45, 28. Clues are fixed in every M8C tree,
including the reference one; worth in M8C is measured on top of the clues.

## Quantity

The full row-scan search tree (Track D3's `tree.rs`: cells in a fixed order,
a placement is valid iff rim sides are grey, other sides non-grey, and every
side facing a placed or fixed cell matches): exact node count (every consistent
partial board at free depth >= 1, the cost of exhausting) and the node count at
the first complete board, by exhaustive depth-first enumeration. Scan "fwd" =
row 0 first, left to right; "rev" = the exact reverse (last cell first).

Worth of a pin set S = log2(nodes of the reference tree) - log2(nodes with the
cells of S fixed to their planted tiles). Reference = unpinned (M8, M10, N8) or
clues only (M8C).

Engine: `engine/` (own binary, `cargo build --release -j 1`). `count_fast`
(per-depth child buffers) and `count_multi` (one pass over the S tree gives
the exact S+{p} tree for every candidate p, by tracking which pins stay
consistent with the partial board) are checked against direct enumeration with
the original `enumerate_first` on calibration boards
(`scripts/validate.py`, both scan directions, singles and pairs: node counts,
solution counts and first-solution counts must be identical) before any
measured run.

## Measurements

1. **Single-tile map** (M8, M10, N8 fwd; M8, M10 also rev; M8C fwd): worth of
   each single cell, per board; mean and sd over boards per cell; corner,
   frame (non-corner) and interior averages; rank map; row means. Direction
   test: correlate the mean fwd map with the mean rev map rotated by 180
   degrees (Pearson r over cells) and with the unrotated rev map.
2. **Pairs** (M8 seeds 1..5, fwd): all 2,016 pairs. Additivity ratio =
   worth(pair) / (worth(p) + worth(q)); reported (median, mean) by Manhattan
   distance and by same row / different row.
3. **Placing k tiles**, k = 1..16 at 8x8 (M8 and M8C), 1..24 at 10x10, by the
   remaining exact tree (fwd scan):
   - (a) measured greedy: at each step add the cell whose addition gives the
     smallest exact tree (ties: lowest pos); all boards of the set.
   - (b) scan-spread: k cells at scan indices floor((i+0.5)*N/k), i < k
     (N = cells), shifted forward to the next non-clue cell in M8C.
   - (c) 2D lattice: a = ceil(sqrt(k)) rows, b = ceil(k/a) columns at
     floor((i+0.5)*n/a), floor((j+0.5)*n/b); if a*b > k keep k of them at
     evenly spaced indices of the row-major list; clue cells replaced by the
     nearest free cell (lowest pos on ties).
   - (d) clustered in the best rows: rows ranked by the set's mean row worth
     from item 1 (fwd), cells taken row by row in that order, inside a row by
     descending mean cell worth.
   - (e) frame only: k cells evenly spaced (floor((i+0.5)*F/k)) along the
     clockwise frame ring starting at (0,0) (F = frame cells).
   - (f) interior only: rule (c) on the (n-2)x(n-2) interior.
   - (g) uniformly random k-subsets of the free cells, 20 draws per k (seed
     `board_seed*100000 + k*100 + draw`, Python `random.Random`); k = 1 is
     also reported exactly as the mean over all cells from item 1.
   Outputs: remaining bits against k per policy (mean over boards, band =
   10th to 90th percentile over boards; for (g) the board value is the mean
   of log2 over draws); a table of the k each policy needs (median over
   boards, plus the count of boards that never reach it) to cut the tree by
   1e3, by 1e6, and to under 1e4 nodes; greedy choice frequency maps for its
   first 4, 8, 16 picks.
4. **Adaptive solver** (M8 seeds 1..10): the policies' pin sets from item 3
   ((a) as chosen under row scan, (b) to (f), (g) with 5 draws), k in
   {0, 1, 2, 3, 4, 6, 8, 12, 16}, solved by bb-mrv (the `Rule::Mrv` DFS of
   `backdoor-branching/compute/src/lib.rs` with A3 propagation, copied, run to
   exhaustion); nodes = options tried (the ladder's node metric), cap 1e5
   nodes per run, censored runs reported as such (>= cap). Question: does the
   order of policies under bb-mrv match row scan's, in particular is
   clustered (d) as good as spread (b, c) under bb-mrv?
5. Extrapolation paragraph for 16x16, labelled as such.

## Figures (results/figures/, SVG and PNG, tables as CSV)

F1 single-tile heat maps (M8 fwd, M8 rev, M8C, M10 fwd; numbers in cells),
F2 remaining bits against k for all policies (M8, M8C, M10), F3 greedy choice
frequency maps (first 4, 8, 16), plus F4 pair additivity by distance and
F5 bb-mrv against row scan per policy.

## Order of stages, caps, compute

Stages through `../common/prereg_guard.py`: `boards`, `s8`, `g8`, `p8`,
`pairs8`, `s8c`, `g8c`, `p8c`, `s10`, `g10`, `p10`, `n8`, `mrv8`. Row-scan
cap 2e10 nodes per tree (never expected to bind; a censored tree is reported
as such). One core of this Mac, `nice -n 10`, single-threaded. Budget about 2
core-hours, never above 4; stages left out for time are listed in the README
in this order of dropping: `mrv8` k > 8, `p10`, `g10`, `n8`. Resumable: every
unit is one jsonl row keyed by a label and finished units are skipped.

Not covered by PROTOCOL.md's G1/G2: this experiment compares no candidate
method against the ladder baseline; bb-mrv enters only as item 4's second
tree measure.
