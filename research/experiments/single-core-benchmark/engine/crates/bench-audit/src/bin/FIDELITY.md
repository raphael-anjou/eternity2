# blackwood_bt against Blackwood's engine

`blackwood_bt.rs` is the repo's port of Joshua Blackwood's record engine
(github.com/jblackwood345/EternityII_Solver, GPL-3.0). Every check below was
made against commit f544e42 of that repository: `Program.cs`, `Util.cs`,
`Structs.cs`. No source was copied.

This port is the canonical one. The shared engines of 2026-11-12
(`memos/where-a-record-is-paid*/engine*/src/bw.rs`, and on their branches
`quota-keying` and `five-clue-quota`) are copies of it, frozen as evidence for
their results, and are not maintained.

## Colour numbering

His `Get_Pieces()` list maps onto this repo's piece file under exactly one
chirality-preserving colour bijection. His piece n is this repo's piece n - 1
with the same sides up to rotation, and both boards share their orientation
(his centre piece 139 at rotation 2 is this repo's piece 138 at rotation 0, at
y = 8, x = 7; his row r is this repo's y = 15 - r).

| his | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| repo | 1 | 6 | 11 | 16 | 4 | 9 | 14 | 19 | 2 | 7 | 12 | 17 | 5 | 10 | 15 | 20 | 3 | 8 | 13 | 18 | 21 | 22 |

Grey is 0 in both. His ring colours `side_edges` {1, 5, 9, 13, 17} are this
repo's {1, 2, 3, 4, 5}; his `heuristic_sides` {13, 16, 10} are {5, 20, 7}.
Source: `research/experiments/quota-keying/scripts/map_colours.py` and
`results/colour-map.txt` on branch `night/2026-11-12-t13-quota-keying`,
rechecked for this file (all 256 pieces match, centre piece and rotation
match).

## The differences

Numbering follows the draft issue of 2026-11-12 (`research/program/night/
2026-11-12/DRAFTS.md`), items 1 to 6; item 7 was missing from it. "Legacy" is
the switch that restores the port's earlier behaviour. With all six legacy
switches on, the engine reproduces the earlier port node for node, except that
item 6 cannot be turned off. With colours other than the two named keyings,
`--priv a,b,c` alone now reads the quota as a share; add `--quota-count` for
the earlier count reading (a seventh switch, needed only with `--priv`).

| # | difference | status | legacy switch |
|---|---|---|---|
| 6 | no-break path could place a piece showing grey toward an interior cell | fixed, unconditional | none |
| 2 | break path placed non-physical pieces, ignored his ring rule and cells | fixed | `--legacy-break-path` |
| 3 | isolation rule for breaks | fixed (off by default) | `--isolation` |
| 1 | colour keying and quota reading | fixed | `--keying legacy-port` (with `--priv`: `--quota-count`) |
| 4 | centre piece | fixed | `--legacy-hints` |
| 7 | quota tested per node instead of per candidate | fixed | `--legacy-quota-test` |
| 5 | scan order | fixed | `--order row-major` |

### 6. Wrong orientation in the no-break path (fixed)

His candidate lists are per piece class and per rotation (`Program.cs` 175 to
197 build them, 112 to 125 pick one per cell), so a piece's grey sides always
face the frame. The port looked candidates up by the colours of the two
matched sides and checked the others only where a neighbour or the frame was,
so a corner piece could take an edge cell, or an edge piece could show grey to
an empty interior cell. Fix: the no-break loop refuses any piece whose grey
sides are not exactly the cell's frame sides (`wrong_class`).

Tested by `no_break_path_never_places_a_wrong_class_piece` (Track 13's stranded
openings: seed 105 with (22, 21, 20) and seed 108 with (3, 12, 13), no clue,
earlier port settings; no wrong-class placement and the run leaves the opening)
and `wrong_class_matches_the_frame_sides_of_the_cell`. With the filter disabled
the first test fails (seed 105: 28,916 wrong-class placements in 1e6 nodes).

Raw rows and scripts: `../../fidelity-results/item6/` (committed after the
judgment; the judge recomputed the numbers below from the same files).

Property check against the frozen Track 15 engine (`fcq`, branch
`night/2026-11-12-t15-five-clue-quota`), 31 runs on m1, earlier port
settings: official 5-clue file, keying (22, 21, 20), seeds 501 to 516 at 1e7
nodes and 501, 506, 507 at 1e8; no-clue file, seeds 101 to 110 at 1e7; no-clue,
(3, 12, 13), seeds 105 and 108 at 1e7.

- In every run this port's per-depth node histogram equals `fcq --orientfix`'s
  exactly (all depths, including the break era).
- In every run that the defect did not strand (26 of 31), `fcq` without the fix
  and with a legal-node cap has a legal-node histogram identical to the fixed
  run's, at every depth, including the late counts where a run reached the
  break era (1,271, 258, 25,120, 10,848, 71 and 531 legal nodes past depth
  200). The fix removes only illegal subtrees.
- The five stranded runs (5-clue 508 and 514; no clue 105 and 107 with
  (22, 21, 20); no clue 108 with (3, 12, 13)) placed 2 to 8 legal nodes in
  1e8 to 1.3e8 nodes before the fix and run normally after it (deepest 38, 47,
  200, 26 and 100 at 1e7).
- The defect also inflated runs, not only stranded them (the judge's
  observation): on no-clue seed 106 the old binary reached depth 240 through
  an illegal subtree, where the fixed engine reaches 231 (24.3M of the old
  run's 1e8 nodes were in illegal subtrees). Any earlier result from this
  engine, its copies included (the night engines of 2026-11-12 and the
  benchmark row), can be biased either way by wrong-class placements: deepest
  depths and late-node yields as well as stranded openings.

### 2. Break path (fixed; `--legacy-break-path` keeps the old one)

His break-era lists (`Util.cs` 22 to 229, `Get_Rotated_Pieces` with
`allowbreaks`; chosen per cell in `Program.cs` 243 to 308) are per class, admit
at most one mismatch per piece, and refuse a mismatch where the piece's own
side is a ring colour (`side_breaks`, `Util.cs` 62, 71 and the like). Corners,
his bottom row and his left column only ever get lists built without breaks.
Candidates are tried in `Score` order (`Program.cs` 203 to 227, `Util.cs` 94):
every exact fit before any break, privileged-first within each group.

The port scanned every unused piece and rotation, so it could place a
wrong-class piece with a break; it had no ring rule and broke anywhere; and it
tried privileged-first, then fewer breaks. Default now: class-correct pieces
only, his ring rule (ring colours derived from the pieces: the colours next to
a grey side), no break at corners or at frame cells whose grey side is one of
the scan's two matched sides (his bottom row and left column), exact fits
first. On the official set the cell rule is implied by the class and ring
rules (the other matched side there is always a ring colour); it is kept
because his lists make it structural.

Tested by `legal_break_path_offers_only_physical_pieces_and_his_breaks`,
`legal_break_path_never_breaks_in_the_scan_first_column_or_at_corners` and
`early_breaks_place_only_class_correct_pieces` (breaks unlocked from depth 30:
no wrong-class placement; the legacy path places some). Each test also asserts
the legacy path shows the defect; with the legal path forced off all three
fail. The order rule is tested by `exact_fits_come_before_more_privileged_breaks`
(a cell where a break candidate carries more privileged sides than an exact
fit); it fails when the candidates are sorted the old way (added after the
judgment, which found that mutation surviving).

### 3. Isolation rule (off by default; `--isolation`)

The port refused a break against a cell that already carried one. His engine
has no such rule: the only per-piece limit is one mismatch (`Util.cs` 77 to
78), and the budget is cumulative (`Program.cs` 130, 136). Tested by
`a_break_may_touch_a_cell_that_already_carries_one` (fails with isolation on
by default).

### 1. Colour keying (fixed; `--keying legacy-port` keeps the old one)

Default keying is his set, {5, 20, 7} in this repo's numbering: ring colour 5
(24 sides), interior 20 (50) and 7 (48), 122 sides. The quota curve
(`Program.cs` 310 to 325) is his count on his 122 sides; by default it is read
as a share of the keying's sides T, quota = curve * T / 122, so it is exactly
his curve for his keying and asks the same fraction of any other keying's sides
(a count silently changes meaning when the keying changes: Track 15). The
port's old default was (22, 21, 20), 150 sides, with the curve as a count:
`--keying legacy-port`. `--priv a,b,c` sets other colours (share);
`--quota-count` forces the count reading. The Track 10 note that his labels
"transfer badly" tested {13, 16, 10} literally in this numbering, which is
three interior colours and not his set.

Tested by `default_keying_is_blackwoods_set_read_as_a_share` and
`share_quota_is_his_count_for_his_keying_and_scales_with_the_sides`. The
second compares against his `heuristic_array` written out as literals
(`Program.cs` 310 to 325, evaluated outside the port): the default quota and
legacy-port's count reading equal it integer for integer up to depth 160, and
the share of 150 sides equals a second literal table. Both fail with the old
default.

### 4. Centre piece (fixed; `--legacy-hints` keeps the old handling)

The draft said the port takes the centre from the piece file where his engine
places it. Checked against his source, the piece and rotation are the same, and
the neighbours are constrained the same way: his `south_start` and
`west_start` lists (`Program.cs` 195 to 197, 273 to 291) pre-filter the cells
below and left of the centre on the centre's sides, as the port's pinned
neighbour check does. The real differences were two: the port counted the
centre's privileged sides from depth 0 (he counts them when his scan places the
piece at depth 119, `Program.cs` 155), and the port stepped over the cell
without a node or a quota test. Default now: a pinned piece counts once the
scan reaches its cell, which is a node with the quota test. Every clue in the
CSV is handled this way; his engine has only the centre, so pass a centre-only
file to match him.

Tested by `hints_count_when_the_scan_reaches_them` (fails with the old
handling).

### 7. Quota test per candidate (fixed; `--legacy-quota-test`)

Not in the draft. His test (`Program.cs` 141 to 145) is per candidate and only
up to depth 160: a candidate at depth d is tried only if the privileged sides
already placed plus its own reach `heuristic_array[d]`, and the loop ends at
the first that falls short (the list is privileged-first). The port tested per
node at every depth: the sides already placed had to reach quota[d] before any
candidate, which asks for one more depth of the curve and spends a node on each
child that dies on arrival. Default now: his test.

Tested by `quota_counts_the_candidate_like_his` (a quota of 1 at depth 1 on a
colour no corner carries: his test lets the search through, the old one kills
every node at depth 1; fails with the old test by default).

### 5. Scan order (fixed; `--order row-major` keeps the old one)

His order is the table of `Get_Board_Order()` (`Util.cs` 333 to 371), embedded
as `BLACKWOOD_BOARD_ORDER`: rows from the bottom-left corner for depths 0 to
175, then the top five rows as a band (their first five columns row by row,
depths 176 to 200; columns 5 to 11 column by column, 201 to 235; the last four
columns as a staircase, 236 to 255). Every cell is matched on its left and
bottom sides, and the hot path is keyed on those. The port scanned rows from
the top-left, matched on top and left.

The draft described his order as interleaving border cells after depth 180.
His source has no separate interleaving: the band brings the frame cells of the
top rows in by geometry. That description matched blackwood_bt's own
`--border-intersperse` option, which was not his order and in this port skipped
cells (a cell with neither key neighbour placed was stepped over); it is
removed.

Tested by `published_order_is_his_board_order_in_this_frame` (spot checks
against his table, the default is his order; fails with the old default) and
`published_order_searches_class_correct_boards_with_and_without_clues`.
Solver::new asserts that every searched cell finds both matched sides placed
or on the frame.

## Remaining differences (not changed)

- **Node unit.** His node count is one per loop iteration, so backtracking
  visits count (`Program.cs` 74). The port counts one per cell entry. Node caps
  and rates are not comparable without a conversion.
- **Randomisation.** His engine reshuffles only the bottom-row list and the
  first corner per solve (`Program.cs` 59 to 63); the other lists are shuffled
  once per `Prepare_Pieces_And_Heuristics` call and shared by 63 threads times
  5 solves (`Program.cs` 199 to 239, 25 to 39). The port reshuffles every list
  per restart, within privileged-count blocks. His bottom row groups pieces
  only as privileged or not (`Program.cs` 61), not by count. Break-era ties are
  broken at random by him, by piece order here.
- **Portfolio and budget.** He runs 5e10 nodes per solve (`Program.cs` 96) on
  63 threads; the port's restart portfolio, node cap, greedy tail completion
  and board banking are its own.
- **First corner.** His first corner is drawn at random and fixed for the
  whole solve; the search starts at index 1 and never backtracks into index 0
  (`Program.cs` 63, 68). The port searches depth 0 like any other cell, so one
  restart can try several first corners. Not implemented.
- **Break-path speed.** His break-era lists are keyed like the no-break ones;
  the port scans every unused piece per break-era node (about 23 times the cost
  per node, Track 13). The search is the same; only wall clock differs.
- **No-break path speed.** This branch made the hot path slower per node; the
  judge measured 1.05 s against 1.46 s user per 1e8 nodes (legacy switches,
  same nodes). Profiling showed the generic per-candidate checks
  (`wrong_class` plus `fits_placed`, written with iterator adapters the
  compiler did not inline) and a per-node scan of the unlock list. The fix
  computes the two non-key side constraints once per node and reads the break
  budget from a per-depth table: now 1.24 to 1.28 s against 1.05 to 1.06 s
  for the pre-branch binary (m1, seed 102 no clue, 1e8 nodes, three runs
  each), so about 19% slower per node remains. Not located within the fix
  round: removing the per-depth node counter, the per-candidate quota test or
  the clue branch each changed nothing measurable, so it is spread over the
  larger solver and the extra per-node bookkeeping. Node sequences did not
  change: `../../fidelity-results/f1/`.

## Who depends on the old behaviour

`scripts/run_standalone.sh` (the `blackwood` row of this benchmark,
`results/REPORT.md`) runs the binary with its defaults, and
`just experiments single-core-benchmark` reaches it through
`scripts/run_grid.py` when the binary is built. A re-run now measures this
engine, not the one behind the committed row, with a no-break path about 19%
slower per node than the old binary (see above). That row was a
wall-clock-bounded run and never reproduced bit for bit; the six legacy
switches come closest, but item 6 stays fixed. No site page or wasm export
uses this binary (the site's break-tolerant solver is `engine/src/break_solver.rs`,
a different machine, untouched).

### Five clues

The second judgment (2026-11-13, `../../fidelity-results/second-judgment/`)
ran the official pieces with all five clues, seeds 501 to 508, 1e8 nodes, one
restart, old defaults against new defaults. The new defaults reach deepest
depth 18 to 54 and place no legal node at depth 201 or more. The old defaults
reach 33 to 215 and place 1,529 such nodes, all on two seeds (501 and 506).
On 6 of 8 seeds the new defaults stop at depth 18. In the published scan order
that is the cell at x 2, y 14, directly below the clue at x 2, y 13, which the
scan reaches only at depth 34. No single legacy switch restores the old depths
(`--keying legacy-port`, `--legacy-hints`, `--legacy-quota-test`, and row-major
order with `--legacy-hints`, were each tried). The published engine
places only the centre piece, so its rules with five clues are this port's
extrapolation, not something the published source settles. Limits: eight
seeds, ranges only, no ratio; every score is a greedy completion of a partial
board; the host was shared and loaded.

The faithful defaults are for the centre-clue and no-clue settings. For
five-clue runs use the legacy profile (`--keying legacy-port --order
row-major --legacy-break-path --isolation --legacy-hints
--legacy-quota-test`) until a five-clue schedule is designed and measured.
