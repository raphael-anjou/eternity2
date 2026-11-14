# real-puzzle-profile: preregistration

Night 2026-11-12, Track 10 (added by the lead at 21:48). Label: **measurement**.
No kill criterion (a measurement), but each measurement states below which
number would change a decision and what would make it uninformative.

Nothing here is a record attempt. No result of this experiment is Eternity II
progress: these are measurements of where search cost sits on the real piece
set.

## Fixed inputs

- Piece set and clues: `research/experiments/single-core-benchmark/engine/data/puzzles/size_16_official_eternity.csv`
  (256 pieces, five clues: piece 138 at (x=7, y=8) rot 0, 180 at (2,13) rot 1,
  207 at (2,2) rot 1, 248 at (13,13) rot 2, 254 at (13,2) rot 1; y = 0 is the
  top row; rotation convention of `blackwood_bt.rs`). Colour 0 is the border.
- Frame stand-ins (M2, part D only): the 60 border cells of the committed
  record boards `v129-palimpsest-463` and `v181-keyring-460` in
  `research/topics/record-boards/boards.json`, decoded from bucas letters
  (letter `a` + c is colour c, the convention of the export crate). A board is
  used only if every one of its 256 cells maps to a distinct piece of the CSV
  up to rotation; otherwise it is dropped and the drop is reported. These are
  stand-ins: no true frame is known, and a frame from a 460-class board very
  likely admits no perfect interior.
- Code: one std-only Rust crate in `engine/` (binary `rpp`), written tonight.

## M1. Node histogram by depth of a break-tolerant depth-first engine

Engine: a port of `research/experiments/single-core-benchmark/engine/crates/bench-audit/src/bin/blackwood_bt.rs`
(the repo's Blackwood-style engine: fixed top-left row-major scan, five clues
pinned, value order by three privileged colours {22,21,20} with a seeded
shuffle inside each privileged-count block, the piecewise-linear privileged
colour quota as a hard prune, a cumulative break budget unlocked by a depth
list, the non-adjacency rule for breaks). The port changes nothing in the
search; it adds a per-depth node counter and a configurable unlock list.
Equivalence check (stage `equiv`, run once before any M1 run): at seed 1,
record list, cap 1e8, the port and `blackwood_bt --stage 4 --restarts 1
--seed 1 --nodecap 100000000` (single attempt, shuffled opening, no
`--border-intersperse`) must report the same node total and the same deepest
depth. If they differ, the port is fixed before M1 runs; if it cannot be made
equal, M1 is reported as measured on the port only, with the difference.

Node: one call of the search on a non-clue cell, at the depth index d of that
cell (d = number of cells before it in the scan, 0..255), counted before the
quota test, as `blackwood_bt` counts it. Depth index d >= 201 is the region
where the record list allows the first break.

What the engine lacks against the real record engines (stated, not fixed):
Blackwood's own piece numbering and privileged colours (the repo's A/B choice
{22,21,20} stands in), his exact quota curve calibration, his border
interspersing and scan corner, restarts with tuned per-attempt caps, and any
engineering beyond about 4e7 nodes per second.

Schedules (cumulative unlock depths):
- `record`: [201, 206, 211, 216, 221, 225, 229, 233, 237, 239] (10 breaks, the
  list `blackwood_bt` uses; the repo's longer list adds 241 and 256, which
  cannot change any share below depth 241 and are left out).
- `gate201`: one gate, all 10 breaks allowed from depth 201.
- `gate221`: one gate, all 10 breaks allowed from depth 221 (the single late
  gate).
- `record-noquota` (secondary): `record` with the quota prune off, to show how
  much of the share is the quota's doing.

Seeds 1..6 per schedule (the seed sets the opening shuffle, as in
`blackwood_bt`). Node budget per run: **1e10 nodes**, with the cumulative
histogram also written at 1e7, 1e8 and 1e9 nodes (checkpoints), so the
dependence of the share on the budget is visible. 4 schedules x 6 seeds = 24
runs; at about 3e7 nodes per second that is about 2.2 core-hours. If wall
time runs short, `record-noquota` is cut first, then seeds 5 and 6 of
`gate201`; cuts are reported.

Output per run (JSONL): nodes per depth at each checkpoint, best complete
score if any (480 minus breaks), deepest depth reached and breaks there, and
the shares of nodes at d >= first unlock of that schedule, d >= 201, d >= 216
(last 40 cells), d >= 200 (last 56 cells).

Primary statistic: the share of nodes at or after the first unlock under
`record`, at the 1e10 budget: median over the 6 seeds, with min and max.

Decision line (set by the lead): the score-bound line is worth pursuing if the
late-node share is above about 50%, not if below about 5%; between, the
measurement does not decide it.

Uninformative if: the share at 1e10 differs from the share at 1e9 by more than
a factor 2 in the median (budget-dependent: then the 1e15 regime of a record
run is not reached by extrapolation and the report says so), or no run reaches
depth 201.

## M2. Tree width by sequential Monte Carlo of the exact search tree

Tree: exact (no break) top-left row-major search. A node is a partial board
after a placement on a non-fixed cell that matches the border and every
already placed or fixed neighbour (left, top, and right or bottom when that
neighbour is fixed). Fixed cells (clues, stand-in frame) are checked against
their placed neighbours when the scan reaches them and do not count as nodes.
Width W_d = number of nodes with d placed cells. Tree size T = sum of W_d.

Estimator: population SMC. Population of Np partial boards; at each depth
every member is expanded, all children are counted (M), and Np children are
drawn uniformly with replacement by systematic resampling (all kept if M <= Np).
Estimate W_{d+1} = W_d_hat x M / |population_d|; W_0 = 1. This is unbiased per
repetition. Report per configuration: R independent repetitions, the mean of T
over repetitions with a 95% interval (t interval on the mean, and the 2.5 and
97.5 percentiles of the R values), log10 of each, the per-depth mean of W_d,
the depth of the widest level, and the extinction depth (deepest d with a
non-zero estimate) per repetition.

Validation gate (stage `validate`, generated boards only, before any real
M2 run is interpreted): planted boards from a seeded generator in `rpp`
(border colours confined to the frame), 5x5 with 3 frame and 4 interior
colours and 6x6 with 3 frame and 5 interior colours, generator seeds 1..10
each, no fixed cells; plus the same 6x6 boards with the planted centre-ish
cell (2,3) fixed as a clue, and with the planted frame fixed, to exercise the
fixed-cell path. Exact T by depth-first enumeration with the same child
function. A board whose enumeration exceeds 2e10 nodes is skipped and
reported. Estimator settings: Np = 1000, R = 100 (also reported, not gated:
Np = 100, R = 100).

Gate passes iff, over all validation boards with Np = 1000: the median of
|log10(mean T_hat / T)| is <= 0.05, no board has |log10(mean T_hat / T)| > 0.3,
and the exact T lies inside the 95% t interval on at least 80% of boards. If
the gate fails, M2 is reported as **not measured**.

Real configurations (each Np = 100,000, R = 30, seeds 1..30):
- A: no clue, all 256 pieces free.
- B: centre clue only (piece 138 at (7,8)).
- C: five clues.
- D: five clues plus a stand-in frame (each usable record board separately),
  scan of the 196 interior cells row-major.

If Np = 1e5 x R = 30 takes more than 40 minutes for one configuration, R is
cut to the number done in 40 minutes and that is reported.

What would change a decision: the memos price five clues, row scan, at
3.1e40 nodes to exhaustion (`solve-path-5-price.md`, range 1e39 to 1e42),
centre clue only at 1.4e47, and one model at 10^45.3 (five clues,
`solve-path-2-designed-instance.md`). If C's estimate is outside 1e39 to 1e42
with its 95% interval also outside, the model prices are revised; if it is
inside, the model is supported on the real pieces at that precision.

Uninformative if: the gate fails; or for a configuration the R repetition
estimates of T span more than 4 decades (5th to 95th percentile), in which case
only the per-depth profile up to the median extinction depth is reported, not T.

## Budget

4 cores of this Mac (taskset is not available on macOS; runs are limited to 4
concurrent processes, each single-threaded, with `nice`). Wall budget about
3 hours, report by 01:00. No remote machine.

## Grading

Every statement in RESULTS.md is graded measured (computed here from committed
raw files), reported (published elsewhere, not rerun) or argued.
