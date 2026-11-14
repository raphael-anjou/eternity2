# real-puzzle-profile: amendments

## A1. M2 follows the price review's sampling specification (2026-11-12, 21:53, before any run)

What. M2 of PREREG.md is replaced as follows. M1 is unchanged.

Why. PREREG.md was committed at 21:50; the lead's message of 21:51 says the
price review (`research/program/night/2026-11-12/memos/price-and-ledger.REVIEW.md`,
lead branch commit 627736f, claims F1 and F2, section F "Specification")
now exists and must be followed. No run of any kind (real pieces or
generated boards) had been made when this amendment was written.

1. Model figure to compare with: 10^40.49 nodes to exhaustion, five clues
   (range 10^39.5 to 10^41.5); 10^47.14 with the centre clue only. The
   10^45.3 figure is withdrawn (script error) and is not compared with.
2. Order: CSV rows 15, 14, ..., 0 (field 6 of the CSV is the row, field 5 the
   column), columns 0 to 15 within a row. The engine is generalised to any
   fixed cell order: a placement is checked against every neighbour that is
   already placed or fixed, grey must face the rim and only the rim. Clues
   are on the board from the start (their joints are checked when a neighbour
   is placed). A node is a valid partial board after a placement on a free
   cell; fixed cells are not nodes.
3. Resampling: when E_k exceeds N, keep a uniform sample of N without
   replacement (the review's choice, replacing systematic resampling).
4. Gates, same binary, before any real-piece result is interpreted:
   (a) the 30 committed 6x6 instances of `research/experiments/hard-ladder/instances/primary-test.jsonl`
   (the `puzzle` object only; this is a counting check, not a solver
   comparison), order rows 5..0, N = 200, 200 repetitions each, exact widths
   by enumeration with the same child function. Statistic: the share of
   (instance, depth) pairs whose mean estimate is more than 3 standard errors
   from the exact width. Passes if that share is at most 2% (0.27% expected
   under normal errors; the margin allows for heavy tails), and the median over
   instances of |log10(mean total / exact total)| is at most 0.05. Any instance
   whose full enumeration exceeds 2e10 nodes is enumerated to the deepest level
   it can within that budget and checked to that level.
   (b) official set, order = the 60 frame cells in ring order (corner (0,0),
   along row 0, down column 15, back along row 15, up column 0), no interior,
   no clue: the mean estimate of the number of complete frames must equal
   40519056449429419697492635191544381440 (10^37.608, the exact count used by
   the review, from `research/experiments/frame-count-corners`) within 3
   standard errors, N = 1e5, 20 repetitions. If it differs by an exact
   symmetry factor (2 or 4), the convention is resolved and reported, not
   counted as a failure; any other failure fails the gate.
   PREREG.md's generated-board gate (5x5 and 6x6, with fixed cells) is kept
   as a secondary check (exercises fixed cells), reported but no longer the
   gate.
   If gate (a) or (b) fails, M2 is reported as not measured.
5. Real runs: five clues, five repetitions at N = 1e6 and five at N = 1e5;
   centre clue only and no clue, five repetitions at N = 1e5 each (cut first
   if wall time runs short). Repetitions are combined by averaging the
   estimates, then taking the log (the estimator is biased low in the log).
   Estimates run to full depth; the sum to free cell 175 is reported next to
   the total.
6. Recorded per level and run: E_k, M, r_k, log10 width estimate, fraction of
   particles with no extension, mean and variance of extensions per particle;
   at free depths 128, 158 and 175 the number of distinct ancestors at free
   depths 16, 32, 64 and 96 and the sum of squared family shares. Seed and
   wall time per run.
7. Null for instance-to-instance spread: instances painted from the official
   colour census (the 60 ring joints carry the official frame-colour joint
   multiset, the 56 seam and 364 inner joints a uniform deal of the official
   inner-colour joint multiset), rejected if any piece is duplicated or
   rotation-symmetric, five clues read off the painted board at the official
   cells. Target 20 instances, two repetitions each at N = 1e5, run only if
   the official runs finish before 00:15; otherwise not run, and reported as
   not run.
8. What counts as a real departure from the model. The review measures an
   sd of log10(tree/model) of 0.15 to 0.37 between instances at up to 36 cells
   and argues 0.5 at 16x16. Before seeing any number: the official estimate
   departs from 10^40.49 if |log10(estimate) - 40.49| > 1.0 (two of the
   argued sd) AND, when the null of item 7 was run, the official value lies
   outside the 2.5 to 97.5 percentile range of the null instances' log10
   estimates. Inside 1.0 dex: consistent with the model at this precision.
   The run is uninformative for T if the five N = 1e6 repetitions span more
   than 2 decades (max over min of the estimates), in which case only widths
   up to the median extinction depth are reported.
9. Frame stand-ins (configuration D): order changed to the inward spiral
   (the review's model figure is 10^8.34 nodes, the mean over frames for the
   true frame, by spiral). With a frame fixed, the interior tree under the
   spiral is enumerated exactly by depth-first search if it stays under 1e10
   nodes, else estimated with N = 1e5, five repetitions. These frames are
   stand-ins from 460-class boards, not the true frame, so the comparison with
   10^8.34 is only a scale check.

## A2. M1b, the whole-tree late share by SMC; exploratory frame runs (2026-11-12, 22:10)

What. Three additions, written after the first two M1 runs (record seeds 1
and 2) and all of M2 had been seen. They are labelled as added after data.

1. M1b. The DFS share at a budget B describes the one subtree the DFS sits in.
   The quantity the review needs is the share over the tree the engine
   searches. M1b estimates, by the same population SMC (now over the M1
   engine's own child function: quota test, unlock list, non-adjacency rule,
   clue pass-through, node counted on entry before the quota test, exactly as
   `search`), the number of entries per depth of the whole tree, for `record`,
   `gate221` and `record-noquota`, N = 1e5, 5 repetitions each. Statistic:
   sum of entries at depth >= first unlock over all entries, from the mean of
   the repetitions. Value order does not change the tree, so no seed of the
   opening is involved.
2. M1b validation gate (before M1b is read): 10 random prefixes to depth 190
   under `record` whose subtree has an entry at depth >= 200 and is fully
   enumerable within 2e8 nodes; exact entries per depth by recursion over the
   same child function, against SMC with N = 200 and 100 repetitions. Passes
   if the share of (prefix, depth) pairs more than 3 s.e. from exact is at most
   2% and the median |log10(mean total / exact total)| is at most 0.05. If it
   fails, M1b is reported as not measured.
3. The rpp binary was rebuilt at 22:08 to add M1b; the `bw` code path is
   unchanged, M1 runs after 22:08 use the rebuilt binary. The equivalence
   check against `blackwood_bt` is rerun with it (stage `equiv2`).
4. Frame stand-ins: both preregistered runs died at free depth 1 and 27,
   because the record boards do not honour the clues and the spiral meets the
   clue ring early. Exploratory (not preregistered): the same two frames with no
   clue, spiral order, exact enumeration capped at 1e10 nodes.

## A3. M1b validation prefixes come from a depth-first walk (2026-11-12, 22:12)

What. A2 item 2 asked for random prefixes to depth 190. A uniform random walk
dies before depth 190 in all 100,000 tries (the stage produced no row; its
start and end lines are in run-log.jsonl). The prefixes are instead the states
on entry to depth 190 met by a depth-first walk of the same tree in the
engine's own order (every 50th such state, from the first 400 kept, walk
capped at 5e9 nodes), keeping the first 10 whose subtree reaches depth 200
and is enumerable within 2e8 nodes. The pass rule is unchanged. These
prefixes are not independent draws from the tree (they share a long DFS
prefix); the gate tests the estimator on them, not their typicality.

## A4. M1b validation prefixes, second try (2026-11-12, 22:14)

What. With A3's settings none of the 400 kept states at depth 190 had a
subtree reaching depth 200 (the stage ran and wrote no row). The prefixes are
now every state on entry to depth 196 met by the depth-first walk (up to
200,000), keeping the first 10 whose subtree has an entry at depth >= 206 (so
the break path after the unlocks at 201 and 206 is exercised) and is
enumerable within 2e8 nodes. Pass rule unchanged.

## A5. M1b gate as run fails its own rule; a replacement check (2026-11-12, 22:21)

What happened. The A4 gate found 5 qualifying prefixes, not 10 (the walk
captured 3,409 states at depth 196 within its 5e9-node cap; 5 had subtrees
reaching depth 206). Their subtrees are tiny (206 to 367 nodes, all ending at
depth 209), so with N = 200 the SMC kept every child and the zero error is
vacuous. By the A2 rule the M1b gate **fails**; this is recorded as such.

Replacement (decided after seeing that result, labelled so): two checks, both
must pass for M1b to be reported, and even then it is reported as validated
by a weaker gate than preregistered.
1. Child-function equivalence: from the root, the exact recursion over the
   M1b child function, in the engine's own order with seed 1, capped at 1e8
   nodes, must give the same per-depth node histogram as the M1 `bw` run
   `results/gate/equiv-port.jsonl` (same seed, same cap), up to a difference
   of at most 2 nodes in total from the cap convention.
2. Resampling active: the same 5 prefixes with N = 10 and 200 repetitions;
   the share of (prefix, depth) pairs more than 3 s.e. from exact at most 5%,
   and every |log10(mean total / exact total)| at most 0.1.

## A6. M1b not measured; M1 budget extension to 1e11 (2026-11-12, 22:27)

What happened. The M1b SMC runs (`results/m1/m1b-*.jsonl`) degenerate: with
the quota on, every repetition's population dies by depth 36 to 48 (uniform
resampling does not find the rare quota-satisfying branches that the
privileged-first value order of the DFS finds), and with the quota off the
population dies at depth 194, before the first unlock. Together with the
failed gate of A5, **M1b is reported as not measured**. The A5 replacement
check is not run (it cannot rescue a degenerate estimate). M1b rows stay in
`results/` as raw data.

Added (decided after seeing the M1 runs of `record` and `gate221` at 1e10,
labelled so): the preregistered budget-dependence rule flags `record` (median
share at 1e10 over 1e9 is 2.9, above the factor 2 of PREREG.md). To see the
next decade, `record` seeds 1 to 4 are rerun with a cap of 1e11 nodes (about
20 to 30 minutes each, four cores in parallel once the 1e10 runs end), with
the histogram also kept at 1e7, 1e8, 1e9 and 1e10. Reported next to, not in
place of, the 1e10 results. Also reported from the existing histograms: late
nodes per entry at depth 201 (sum of nodes at depth >= 201 over nodes at
depth 201), which measures how large the break subtree under one depth-201
node is in this engine.

## A7. Cuts for wall time (2026-11-12, 23:02)

What. The `gate201` runs at 1e10 nodes and the `record` runs at 1e11 run far
slower than the `record` runs at 1e10 (43 and 37 minutes of CPU so far,
against about 3 minutes for a `record` 1e10 run), which means they spend many
nodes in the slow break path. To report by 01:00: the M1 queue is stopped
(xargs killed at 23:01; the four running processes continue); `record-noquota`
(all 6 seeds) and `gate201` seeds 3 to 6 are cut, the first in the order
PREREG.md set, the second beyond it (PREREG.md only foresaw cutting seeds 5
and 6). `record-1e11` seeds 3 and 4 are cut. The four running processes get
until 00:15; any process still running then is killed, which leaves a start
line without an end line in `results/run-log.jsonl` and no row, reported as
not finished.

## A8. Two cores freed early (2026-11-12, 23:12)

The two `record-1e11` runs ended at 23:11. With their two cores, A7's cut of
`record-noquota` is lifted: seeds 1 to 6 run, two at a time, under the same
00:15 limit, through the `m1` stage with `ONLY_NOQUOTA=1` (queues only those
six; `gate201` seeds 3 to 6 stay cut). Then the exploratory `frames-noclue`.
