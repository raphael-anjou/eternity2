# Judge: Track 9, resumable runs for bb-mrv-fast (optimization), rejudge after the fix round

**Verdict: keep.** The three defects of the first judgment (D1 unchecked
checkpoint content, D2 errors exiting 0, D3 closed share 0 on a root
refutation) are fixed, the fix is preregistered as amendment A1 before any
run of the fixed code, the identity gates G-a, G-b and G-c pass again with the
fixed binary and reproduce the version 1 rows field for field, and the new
tamper gate G-e passes. The optimization label holds: no valid resume differs
from the uninterrupted reference. Two non-blocking findings (section 6).

This is the single rejudge the procedure allows. The first judgment (verdict
keep, defects D1 to D3) is `86f302a`; this file replaces it. Judged
2026-11-12 on `night/2026-11-12-t9-resumable-runs` at `0227330`, only the
commits after `86f302a`. Local only, two cores of a shared Mac, about 35
minutes. Nothing pushed.

## 1. Order, guard log, tested code

- `PREREG.md`: one commit, `7fda502` (20:46:45 CET), SHA-256 of the working
  file `711eaef7...`, unchanged since. `PREREG_AMENDMENTS.md` A1 and the fixed
  code are one commit, `299f8d3` (21:34:52 CET). Tests `e013e8c` (21:35:57,
  `compute/tests/` only). Results `0227330` (21:46:41).
- `results/run-log.jsonl`: the fix round is 38 start lines with a `run_id`,
  first at 20:34:56 UTC (21:34:56 CET, 4 s after the A1 commit), last at
  20:45:11 UTC. Every one carries `prereg_commit` `7fda502` and the committed
  SHA-256, has a matching end line by `run_id`, exit 0. Every committed file
  under `results/fix/raw` and `results/fix/tamper` (8 G-a, 24 G-b/G-c, 3 make,
  3 tamper) is the `--out` of one of them. File mtimes (21:34:57 to 21:46:12)
  all follow the A1 commit. The log is append-only over `86f302a` (0 removed
  lines); version 1 results, `results/summary.json` and the dell log are
  untouched.
- Tested code is committed code: a fresh `cargo build --release` of `HEAD`
  in my own target dir gives a binary byte-identical to the frozen
  `compute/target/gate-bin/resumable_runs` (SHA-256 `65b4ea51...`). This
  settles the `dirty_paths: 1` on some log lines.
- Hot path: `git diff a679005 299f8d3 -- compute/src/lib.rs` touches the
  format structs, `impl Checkpoint`, `checkpoint`/`write_checkpoint`, the
  error reporting in the cold `event`, the end of `replay`, the setup of
  `solve_w` and the root-refutation branch of `solve`. `dfs`, `branch`,
  `dfs_resume`, `propagate`, `choose` are unchanged. The only dependency
  change is `serde_json` `float_roundtrip`.

## 2. Track label (optimization): recomputed

My script over the committed fix rows (stdlib, own code):

| check | result |
|---|---|
| G-a, 150 instances (140 Track 0 gate set + 10 9x9): `FastSolver` vs `ResumableSolver` on status, nodes, backtracks, propagation, board, hash, hashed | 0 mismatches |
| G-a `fast` hashes vs Track 0's committed `fast` rows (`mrv-node-cost/results/raw/gate-*.jsonl`), 140 shared instances | 0 mismatches |
| G-a progress lines: node counts multiples of 1e5, closed share nondecreasing | 0 bad |
| G-b/G-c final segment vs G-a `fast` (7 fields) and vs uninterrupted closed share and deepest | 0 mismatches |
| per-segment node counts sum to the totals | 0 bad |
| classes | 108 / 16 / 6 solved in segment 1 / 2 / 3, 20 censored at C (10 at 7x7, 10 at 9x9) |
| every rerun row vs the committed version 1 row, 32 files | 368 rows, 0 differences |
| `analysis/analyze.py` and `fix_compare.py` rerun | outputs equal to committed `summary.json`, `fix/summary.json`, `fix/compare.json` |

Seeded rerun with my build, outputs in my scratch dir: G-a on primary 7x7
test 1..30 (0 mismatches, rows equal to the committed fix rows on every
field); G-b chain 333,333 / 666,666 / 1e6 on the same 30 (30, 21, 13 rows per
segment, all equal to the committed rows on the identity fields, segment,
per-segment nodes and depth); a schedule not in the gate, 9x9 plus2
calibration 1001..1003 checkpointed at 1e6 and resumed to 3e6 in one segment
(all three equal to the committed G-a `fast` row and uninterrupted closed
share and deepest).

## 3. The defects, retested by hand on a version 2 checkpoint

7x7 primary test s1 at 4e5 (depth 26, last index 1 of 2), the first judge's
case, written by my build:

| edit | before (v1) | now |
|---|---|---|
| unaltered, pretty-printed, keys reordered | | resume, exit 0, hash `38a73fb0ba6192f7` = uninterrupted |
| last `index` 1 to 0 | silent different search | refused, exit 1, error row |
| `totals.nodes` +1000, `backtracks` +7, `hash` zeroed, `hashed` 5 | silent | refused, exit 1 |
| cap 4e5 and 3e5 on a 4e5 checkpoint | exit 0, nodes 0 | exit 1, error row with nodes 400,000 and the checkpoint hash |
| version 1 file | | refused with the version 1 message |
| trailing garbage, BOM | | refused |
| duplicate key with a different value | | refused (digest) |
| duplicate key repeating the same value | | accepted, same search (harmless) |

Directory resume with one corrupt file among four: the three good files
resume (rows equal to the committed ones), the bad one gets an error row,
exit 1 (D2). Progress path under a missing directory: created, exit 0; under
a regular file: error row before any node, exit 1. D3: unit test passes
(`cargo test --release`: 5 passed).

G-e rerun by me on a checkpoint not in the gate (7x7 test s7 at 666,666,
segment 2 of my G-b chain): 658 / 658 alterations refused, unaltered copy
identical; forged digest 413 / 424 refused. Committed G-e recomputed from the
rows: 2202 / 2202 refused (every one exit 1 and status error, no duplicate
labels), 3 / 3 unaltered copies identical, forged 1408 / 1436.

## 4. Timing

Not rerun by the branch (hot path unchanged, verified above). My spot check,
`time` subcommand, 8-c10 test 1..10, same A B B A A B B A A B design, shared
machine: per-instance min ratio median 0.996 (0.994 to 1.000), nodes and
status identical in all 100 runs. The preregistered result from the first
round (1.008, CI [0.998, 1.011]) stands.

## 5. Claims against evidence

- Identity, integrity, exit codes: measured, committed, reproduced.
- G-d for version 2 (cross-machine resume): **argued, not measured**. `dell`
  has no toolchain now; the README's argument (decimal JSON, same `serde_json`
  with `float_roundtrip`, IEEE division) is sound but it is an argument. The
  README's long-run recipe says "new process, any machine": true for version
  1 by G-d, untested for version 2. A one-off resume on another machine is
  the cheap way to make it measured.
- Closed share: limits stated (not a time estimate, not a solution
  probability). No overclaim found.

## 6. Findings (non-blocking)

**F1. Wrong explanation of the one forged stack alteration that passes.**
README line 82 says dropping the last stack level of the 9x9 checkpoint
passes the digest-independent checks because "that level's share of the tree
is below f64 resolution". I reproduced it on a regenerated plus2 test s1
checkpoint at 5e5: the last two levels are (index 0 of 1) and (index 0 of 3).
The dropped level contributes exactly 0 to the closed share (index 0), and
the level above is forced (one option), so the replayed domains at the new
deepest level equal the stored `state_fnv`. The accepted resume is a
different search (hash `8a004a552cdd9354` vs `3e1373e99063187e`, one forced
node counted twice). It needs a forged digest, so it is outside G-e and does
not change the verdict; but the cause is structural, not rounding, and it
can occur at any depth. Fix: correct the sentence (here and in "what they
cannot catch alone" further down), and if the digest-independent layer is
meant to stand on its own, add the stack depth and last-level index to
`state_fnv`.

**F2. Version 2 cross-machine resume unmeasured** (section 5).

## 7. Use for a long 9x9 run

Safe to rely on, with the README's own caveats: `--guard 0` (a guard stop is
not resumable), check the exit code (now meaningful) and the row status,
progress lines past the last checkpoint appear twice after a kill (keep the
later ones), version 1 checkpoints under `results/ckpt/` cannot be resumed.
The digest detects corruption and hand edits, not deliberate forgery.

## Unplanned observations

- The fix round reproduced every version 1 row to the bit, including the
  closed share floats, after switching `serde_json` to `float_roundtrip`: the
  float printing used in the rows did not change.
- The forged-digest case in F1 shows MRV forced levels (one option) are
  common enough deep in a 9x9 stack to defeat a state hash taken only at the
  deepest level.
