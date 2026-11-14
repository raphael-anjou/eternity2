# Judge: blackwood_bt fidelity branch

Branch `night/2026-11-13-d2-port-fidelity`, commits b8d12486 to 07c96165
(only `blackwood_bt.rs` and `FIDELITY.md` change against 637a42be). Judged
against Blackwood's public source, `jblackwood345/EternityII_Solver` at
f544e42 (`Program.cs`, `Util.cs`), read independently of the author's notes.

## Verdict: keep

Every one of the seven claimed fixes is real and matches the published source.
Six of seven have a test that fails when the fix is reverted; the seventh's
order rule is untested (F2). The legacy switches reproduce the earlier binary
node for node except where item 6 bites, as claimed. One behaviour change is
unannounced: the no-break hot path is about 39% slower per node (F1).

## What was rerun (mac-m1, at most 6 threads, about 0.3 core-hour)

- `cargo test --release` on the bin: 12 of 12 pass.
- Mutation sweep, one revert at a time, rebuilding the tests each time:

| mutant | tests failing |
|---|---|
| no class filter in the no-break path (item 6) | 3 (no_break_path..., early_breaks..., published_order_searches...) |
| `legacy_break_path` on by default (item 2) | 3 |
| isolation on by default (item 3) | 1 |
| default keying legacy-port (item 1) | 1 |
| default order row-major (item 5) | 1 |
| `legacy_hints` on by default (item 4) | 1 |
| `legacy_quota_test` on by default (item 7) | 1 |
| ring rule removed (item 2) | 1 |
| cell rule removed (item 2) | 1 (only its `no_break_cell` assert) |
| ring and cell rules both removed | 2 |
| break candidates sorted the old way (item 2) | **0, survives** |

- Legacy reproduction against the pre-branch binary (built from 637a42be),
  new binary with all six legacy switches, 1 restart each: no-clue file seeds
  101 to 108 at 1e8 nodes, five-clue file seeds 501 to 506 at 1e8 and 501, 502
  at 1e9. 13 of 16 pairs are identical in nodes, deepest depth, best scores,
  completion histogram and emitted board URL, including runs deep in the break
  era (no-clue 101: 2,877,725 nodes past depth 200; 108: 4,594,151). The three
  that differ (no-clue 105, 106, 107) are item 6: the frozen Track 15 engine
  `fcq` without `--orientfix` reproduces the old binary's deepest depth on 106
  and 107 (240 and 25), and with it its per-depth histogram equals the new
  binary's exactly.
- Default engine (published order, keying, break rules), seeds 1 to 6 at 1e8
  on a centre-only file and on the no-clue file: 0 wrong-class placements in
  all 12 runs.
- Clean timing, seed 102 no clue, 1e8 nodes, fast path only, three repeats
  each run alone: old binary 1.05 s user every time, new binary with legacy
  switches 1.46 to 1.47 s. Identical node counts.

Independent checks (stdlib Python, on the published source and the repo CSV):
his 256 pieces map onto the repo's under the stated colour map with the same
chirality (0 mismatches); his centre 139 at rotation 2 is repo piece 138 at
rotation 0 at (7, 8), and its bottom and left are his 6 and 11, the colours his
`south_start`/`west_start` lists filter on; ring colours derived from the
pieces are {1..5}, the image of his `side_edges`, and no interior piece carries
one; {5, 20, 7} carry 24 + 50 + 48 = 122 sides, (22, 21, 20) 150; his
`heuristic_array` gives the same integers in f32 and f64 at every depth to 160
(119 at 160), so the port's f32 copy is not sitting on a rounding edge;
`BLACKWOOD_BOARD_ORDER` equals his `Get_Board_Order()` table entry for entry,
and in his order no cell ever has its top or right neighbour placed first, so
matching on left and bottom only loses nothing. The author's item 6 property
check (31 runs, raw files on mac-m1 under `~/e2-night/d2/prop`) recomputes
exactly: 31 of 31 histograms equal `fcq --orientfix`, 26 of 26 non-stranded
legal histograms identical, the five stranded runs and the six late counts
(1,271, 258, 25,120, 10,848, 71, 531) as stated.

## Per-item rulings

1. **Keying.** Real. His `heuristic_sides` {13, 16, 10} are {5, 20, 7} here
   (verified). Tested; the default-keying test fails on revert. The share
   reading for other keyings is the port's own design, not something his
   source says (he has only his keying, where share and count coincide); it
   is announced as such.
2. **Break path.** Real. Class-correct lists, one mismatch per piece, no
   mismatch on the piece's own ring side, no breaks at corners, his bottom row
   or left column, all match `Util.cs` 22 to 229 and `Program.cs` 172 to 308.
   Class and ring rules are tested and caught by mutation. The exact-fits-first
   order is right by reading (`Score = 100 * count - 100000 * breaks`) but no
   test catches its revert (F2). The cell rule is redundant on the official
   set, as FIDELITY says; its only test is an assert on the table itself.
3. **Isolation.** Real (his engine has no such rule). Tested.
4. **Centre piece.** Real, and the draft's description was correctly
   replaced: his centre is placed by the scan at depth 119 and counted then
   (`Program.cs` 155); the port now does the same. Tested.
5. **Scan order.** Real. Full table identical (checked here, the test only
   spot-checks). Removing `--border-intersperse` is announced; nothing in the
   repo passes it.
6. **Orientation defect.** Real, fixed unconditionally. Tested, and the
   per-run property check holds both in the author's files and in this
   judge's own old-versus-new runs.
7. **Quota per candidate.** Real (`Program.cs` 141 to 145, up to depth 160,
   loop ends at the first candidate short). Tested.

The "Remaining differences" list is accurate. One omission: his first corner
is fixed for the whole solve and never backtracked into (`Program.cs` 63, the
loop starts at index 1); the port searches depth 0 like any cell.

## Findings

- **F1 (unannounced behaviour change, wall clock).** The no-break path costs
  about 39% more per node than before (1.05 s to 1.46 s user per 1e8 nodes,
  same nodes). FIDELITY says only the break path's speed differs. Likely from
  the generic `fits_placed`/`wrong_class`/`required` checks and the always-on
  per-depth counter. Fix: state it in FIDELITY ("Who depends on the old
  behaviour": a benchmark re-run also measures a slower hot path), or restore
  the specialised checks.
- **F2 (untested fix).** Reverting the break-candidate order to the port's old
  one (privileged first, then fewer breaks) passes all 12 tests. The assertion
  in `legal_break_path_offers_only_physical_pieces_and_his_breaks` runs at a
  cell where both orders agree. Fix: a test at a cell where some break
  candidate has more privileged sides than some exact fit, asserting the exact
  fit comes first.
- **F3 (evidence not committed).** The item 6 property-check numbers rest on
  files that live only on mac-m1. They recompute exactly, but the branch does
  not carry them. Fix: commit `prop1_remote.sh` and the `prop/` outputs, or
  label the paragraph as reported.
- **F4 (weak tests, minor).** "legacy-port reproduces the old curve" compares
  `build_quota` with itself, and `his_heuristic_array` restates the port's own
  formula. Both claims hold by reading and by the independent check above.
- **F5 (minor).** `--priv a,b,c` alone now reads the quota as a share, where
  it used to be a count. Documented in FIDELITY item 1, but the header's list
  of switches that restore the old behaviour does not mention `--quota-count`.

## Not checked

Clippy and rustfmt; `--threads > 1`; stages 1 to 3; the benchmark variant
CSVs (three pinned corners) under the new default; break-path wall clock
beyond the runs above; Blackwood's C# engine itself (not run; his `Random`
makes node-for-node comparison impossible, so fidelity rests on source
reading plus the checks above).

## Unplanned observations

- Item 6 did not only strand runs: on no-clue seed 106 the old binary reached
  depth 240 through an illegal subtree, where the fixed engine reaches 231
  (24.3M of the old run's 1e8 nodes were in illegal subtrees). Results from the
  old engine can be inflated by the defect as well as deflated.
- The default engine goes deep fast: 11 of 12 default runs reached depth 244
  to 247 within 1e8 nodes (no clue and centre only), against 22 to 241 for the
  legacy settings, but at 3.5 to 62 s per 1e8 nodes against about 1.5 to 3 s.
  Their best scores (454 to 462) are greedy-tail completions, not native boards.

## Second judgment, 2026-11-13

Re-judged at 1a4e5882 after the fix round (7ca14eab to 1a4e5882, ten
commits counting the wording edit b86e4ef5 to this report, which changes no
number). Against 07c96165 the round touches only `blackwood_bt.rs`,
`FIDELITY.md`, this file and the new `fidelity-results/` folder. There is no
PREREG.md: this is a port-fidelity branch, not a preregistered experiment,
so the run-log and commit-order checks do not apply.

### Verdict: keep

Each of F1 to F5 and the first-corner gap is closed. The hot-path rewrite
changes no node sequence that could be checked (23 of 23 run comparisons below
are identical). F1 is closed by documenting the slowdown, not removing it:
the hot path is still about 15 to 19% slower per node than before the
branch, and FIDELITY says so.

### What was rerun (mac-m1, at most 7 threads)

The host was shared during the whole session: another job held four cores
(load average 14 to 22), so absolute seconds below run about 25% higher than
on a quiet machine. Ratios between binaries timed back to back hold up better
than absolute times. Binaries: `old` = 637a42be (the first judgment's build,
source hash checked), `j1` = 07c96165 (the binary of the first judgment,
source hash checked), `new` = 1a4e5882 built fresh. Raw outputs are on mac-m1
under `~/e2-night/d2-rejudge/` (`runs/`, `timing/`, `timing2/`, `r_*.sh`) and
were not committed: this round commits this file only.

- `cargo test --release` on the bin at 1a4e5882: 13 of 13 pass.

### Per-finding status

- **F1, closed (documented, not removed).** Seed 102, no clue, 1e8 nodes,
  one restart, binaries run one after another per repeat (old, j1 with the
  six legacy switches, new with the six legacy switches). Two sets: three
  repeats, then five.

  | set | old (user s) | j1 | new | new / old | j1 / old |
  |---|---|---|---|---|---|
  | 1 (3 repeats) | 1.22 to 1.28 | 1.70 to 1.74 | 1.39 to 1.57 | 1.19 | 1.37 |
  | 2 (5 repeats) | 1.37 to 1.41 | 1.81 to 1.93 | 1.56 to 1.64 | 1.15 | 1.36 |

  Node counts are 1e8 in all 24 runs, with the same deepest depth (26) and
  best score in each. The author's absolute 1.05 / 1.47 / 1.24 to 1.28 s
  could not be reproduced on the loaded host, but the ratios match his (1.40
  and about 1.2) to within the noise. FIDELITY "No-break path speed" and "Who
  depends on the old behaviour" now state the remaining 19%. Its account of
  what did not explain the remaining slowdown (node counter, per-candidate
  quota test, clue branch) is reported, not checked here. By reading, the new
  hot-path filter (key side showing grey off the frame, then `want_on` per
  other side) accepts exactly the candidates `wrong_class || !fits_placed`
  refused before. Two things hold this: key sides are always determined in
  both scan orders, and `budget_at` is refilled by `reset()`, which every
  attempt calls after the only test that edits `unlock`. The
  `illegal_placed` increment dropped from the fast path could only ever add
  0 there.
- **F2, closed.** With the legal sort reverted to the old order (privileged
  first, then fewer breaks; line 695 rewritten to match line 698), 1 of 13
  tests fails: `exact_fits_come_before_more_privileged_breaks`, at its
  assertion "every exact fit comes first". The other 12 pass.
- **F3, closed.** Recomputed with a separate script from the committed rows in
  `fidelity-results/item6/raw/`: blackwood_bt histogram equals `fcq
  --orientfix` in 31 of 31 runs; fix-off and fix-on legal histograms equal in
  26 of 26 non-stranded runs; the five stranded runs are f5 508 and 514,
  fn13 105 and 107, fn13m3 108 (2 to 8 legal nodes); the late legal counts
  are 1,271, 258, 25,120, 10,848, 71 and 531. From `fidelity-results/f1/raw/`:
  11 of 11 base/head pairs agree on everything but the new
  `illegal_placements=` field, the 5 emitted URLs match, and the 4 centre-file
  histograms are identical before and after the speed fix.
- **F4, closed.** `Program.cs` read again at f544e42 (raw fetch, read-only):
  `heuristic_array` lines 310 to 325, `max_heuristic_index = 160` (line 344).
  Evaluated here in f32 with the last segment promoted to double (its divisor
  `4.4615` is a double literal), and also in pure f64: both equal
  `HIS_HEURISTIC_ARRAY` at all 161 depths. `SHARE_150` equals the truncation
  of the unrounded f32 curve times 150 / 122 at all 161 depths. It does NOT
  equal floor(his integer array x 150 / 122), which differs at depth 17 (3
  against 2) and others. The literal is right for the port's design, but its
  doc comment "floor(curve * 150 / 122)" should say the curve is taken before
  his integer cast (minor, wording only).
- **F5, closed.** The header (lines 22 and 23) and FIDELITY item 1 (intro
  paragraph and table row) name `--quota-count`; the switch is parsed (line
  1508).
- **First-corner gap, closed.** FIDELITY "Remaining differences" now lists it
  (`Program.cs` 63, 68; not implemented), as the first judgment described.

### Regression checks

- Legacy switches against the pre-branch binary: new with the six legacy
  switches against the first judgment's `old` outputs (same files, same
  seeds), 1e8 nodes, one restart: no-clue 101, 102, 103, 104, 108 and five-clue
  501 to 506. **11 of 11 identical** in nodes, deepest depth, best complete and
  native scores, completion histogram and emitted board, including the
  break-era runs (no-clue 101 and 108 reach 241; five-clue 501 reaches 215).
- New defaults against `j1` (07c96165) defaults, 1e7 nodes, `--hist`: no-clue,
  centre-only and five-clue files, seeds 1 to 4 each. **12 of 12 per-depth
  histograms identical**, with equal deepest depth, best score and
  `illegal_placements=0`.

### Supplementary measurement (not part of the verdict)

Official pieces, all five clues (the first judgment's `five.csv`), seeds 501
to 508 for every arm, 1e8 nodes, one restart. (a) `old` with its defaults;
(b) `new` with the six legacy switches; (c) `new` with its defaults (published
keying {5, 20, 7}, quota as a share, legal breaks, published scan order,
clues counted when reached, per-candidate quota). The old binary prints no
per-depth histogram, so legal accounting for (a) and (b) comes from the
frozen Track 15 engine `fcq` run on the same file and seeds (fix off for a,
`--orientfix` for b). It reproduces them: equal nodes and deepest depth
against (a) on 8 of 8, and an identical histogram against (b) on 8 of 8. (c)
reports `illegal_placements=0` on all 8, so all its nodes are legal.
"Late" = legal nodes at depth 201 or more. User seconds, loaded host.

| seed | a deep | a late | a legal share | a best | a s | b deep | b late | b best | b s | c deep | c late | c best | c s |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 501 | 215 | 1,271 | 0.991 | 443 | 3.59 | 215 | 1,271 | 443 | 3.72 | 54 | 0 | 372 | 3.53 |
| 502 | 200 | 0 | 0.985 | 431 | 2.92 | 200 | 0 | 431 | 3.21 | 18 | 0 | 369 | 3.57 |
| 503 | 33 | 0 | 0.971 | 381 | 2.04 | 33 | 0 | 381 | 2.18 | 18 | 0 | 382 | 3.62 |
| 504 | 199 | 0 | 0.986 | none | 2.29 | 199 | 0 | none | 2.57 | 18 | 0 | none | 3.49 |
| 505 | 197 | 0 | 0.991 | 425 | 3.09 | 197 | 0 | 425 | 3.35 | 18 | 0 | 373 | 3.64 |
| 506 | 205 | 258 | 0.992 | 425 | 3.42 | 205 | 258 | 425 | 3.66 | 18 | 0 | 377 | 3.77 |
| 507 | 197 | 0 | 0.990 | 425 | 3.68 | 197 | 0 | 425 | 3.82 | 18 | 0 | 361 | 3.81 |
| 508 | 33 | 0 | 8 nodes | none | 1.97 | 40 | 0 | 382 | 2.07 | 33 | 0 | 378 | 3.61 |

(b)'s legal share is 1 except 501 (99,999,775 legal) and 506 (99,999,964),
where the legacy break path places wrong-class pieces (216 and 36). Every
score in every arm is a greedy completion of a partial board (`native=false`);
no arm completed a board by search, and "none" means no completion was
emitted (score 0). Totals: late legal nodes a 1,529 (range 0 to 1,271), b
1,529, c 0; deepest a 33 to 215, b 33 to 215, c 18 to 54; user seconds a
23.0, b 24.6, c 29.0.

On equal seeds the faithful defaults do **worse** with five clues, per node
and per second. They place no legal node past depth 200 in 8 x 1e8 nodes,
against 1,529 for the old engine. On six of eight seeds they never get past
depth 18, and they spend slightly more time per node. The earlier rough
reading of "about 9 times fewer" understates the gap on these seeds; with two
seeds carrying all of the old engine's late nodes, eight seeds cannot put a
ratio on it. (b) equals (a) wherever item 6 does not strand (a); on 508 it
does, and (b) runs normally.

Where the depth-18 wall comes from (unplanned ablations, same seeds and cap,
raw on mac-m1 under `runs/x-*`): depth 18 in his order is cell x 2, y 14,
directly below the clue at x 2, y 13, which the scan reaches only at depth 34.
Published order with every other legacy switch on stops at 18 on the same
four seeds (502, 503, 504, 507) and reaches 198 to 199 on the others. Row-major
order with every other default reaches only 60 to 110. No single switch
(`--keying legacy-port`, `--legacy-hints`, `--legacy-quota-test`, row-major
plus `--legacy-hints`) restores the old depths. So two things combine: the
published order meets a clue's neighbour 16 cells before the clue, and the
default keying, quota and clue counting together keep the search shallow.
His engine places only the centre piece (`Program.cs` 177, 178), so five-clue
behaviour under "his rules" is the port's extrapolation, not something his
source settles.

### New findings (minor, none blocking)

- **N1.** `SHARE_150` doc comment: say "the unrounded curve" (see F4).
- **N2.** FIDELITY does not say that the defaults with the five clues search
  far shallower than the old engine (table above). Anyone who reruns a
  five-clue result with the new defaults gets this engine. One line under
  "Who depends on the old behaviour" would cover it, with the note that his
  engine never ran with five clues.

### Not checked

Clippy and rustfmt; `--threads > 1`; the break path's wall clock under the
new hot path; the author's profiling claims for the remaining 19%; a quiet-host
absolute timing; Blackwood's C# engine itself.

### Unplanned observations

- The default engine's five-clue histograms at 1e7 (regression check above)
  also stop at depth 18 on all four seeds 1 to 4, so the wall is not specific
  to seeds 501 to 508.
- Contrast with the first judgment: with no clue or the centre only, the
  defaults reached 244 to 247 within 1e8 nodes. Adding the four corner-area
  clues reverses the picture against the old engine.
