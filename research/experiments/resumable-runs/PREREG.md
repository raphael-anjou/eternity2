# Preregistration: resumable long runs for bb-mrv-fast (night shift 2026-11-12, Track 9)

Written and committed locally before any test seed runs. Branch
`night/2026-11-12-t9-resumable-runs`, branched from
`night/2026-11-12-t0-node-cost` (0860e32, Track 0 judged keep). Nothing is
pushed tonight. Forced changes go in `PREREG_AMENDMENTS.md` (what, why, when);
this file stays unchanged. Generated boards only; nothing here is official
Eternity II progress.

## Track label

**Optimization** (harness). The search must not change: with progress records
on, and across any number of checkpoint and resume cycles, the node sequence,
node count, backtracks, propagate calls, status and board must equal
`bb-mrv-fast`'s. Any difference on any gate instance reclassifies the track:
it stops and reports, and no timing claim is made.

No hypothesis about solving is tested. The claim is: (1) identity under the
gate below, (2) a bounded wall-clock overhead (timing section).

## What is built

Crate `research/experiments/resumable-runs/compute`, binary `resumable_runs`.
It path-depends on `mrv-node-cost/compute` (unchanged; used for the
unmodified `FastSolver`, `NodeHook` and the FNV-1a `HashHook`), on
`hard-ladder/compute` (unchanged, `NodeCounter`, instances, `run_one`,
`verify_board`) and on `sat-backdoors` / `backdoor-branching` (unchanged).
`FastSolver`'s search loop is private to its crate, so `ResumableSolver` is a
copy of it (`propagate`, `choose`, `offered`, `presence` copied verbatim) with
a changed `dfs`: per-level bookkeeping (cell and option index), an event
check before each `try_assign` (progress record or periodic checkpoint), and a
resume entry. There is no separate verbatim-copy step: gate G-a compares
`ResumableSolver` directly with the unmodified `FastSolver<HashHook>` from
the Track 0 crate, node-sequence hash included, on Track 0's whole gate set.

### Progress records

Every N nodes (total over segments; `--progress-every`, default 1,000,000),
just before the `try_assign` of node N*k + 1, one JSON line is appended (and
flushed) to the progress file: instance, segment, total nodes, total
backtracks, total propagate calls, current depth (cells assigned on the
current path), deepest depth reached so far (over all segments), elapsed
seconds (summed over segments), closed share.

### Closed share (exact definition)

Let the search stack have decision levels 0..L. Level i was entered with a
domain of K_i options (the chosen cell's domain at that level, in the solver's
option order), and j_i of them are exhausted: their subtrees are fully closed
(searched to the end, or refuted by propagation). At a progress or checkpoint point, levels
0..L-1 each have option j_i in progress, and level L is about to try option
j_L (not yet counted). Then

    closed = sum over i = 0..L of (j_i / K_i) * prod over l < i of (1 / K_l)

computed in f64 by Horner from the deepest level up,
s = 0; for i = L down to 0: s = (j_i + s) / K_i. Each operation is monotone
in its inputs under round-to-nearest and every partial s is at most 1, so the
computed value never decreases along the search. It is 1 exactly when the
root level is exhausted (status exhausted), and is reported as 1 then.

Limits, stated in the README: it measures progress toward exhausting the
tree, not toward a first solution (a run can solve at closed = 0.001 or never
solve at 0.99); under MRV the branching factors K_i differ between levels and
between subtrees, so equal shares of the tree are not equal amounts of work,
and closed share is not a time estimate.

### Checkpoint and resume

A checkpoint is written at the node cap (always, when `--checkpoint` or
`--checkpoint-dir` is given) and, optionally, every M nodes without stopping
(`--checkpoint-every`). It is taken at the same point as a progress record,
just before a `try_assign`. Format: one UTF-8 JSON object, written to
`<path>.tmp` and renamed (atomic on one file system):

```
{ "format": "e2-bb-mrv-fast-checkpoint", "version": 1,
  "solver": "bb-mrv-fast",
  "instance": {"id", "ladder", "role", "n", "c", "seed"},
  "model_fnv": "<16 hex>",          // FNV-1a over the solver's option tables and root domains
  "segments": k,                    // segments completed, including this one
  "totals": {"nodes", "backtracks", "propagation", "elapsed_s"},
  "hash": "<16 hex>", "hashed": <u64>,   // running node-sequence hash (Track 0's FNV-1a) and count
  "deepest": <int>, "closed_share": <f64>,
  "stack": [ {"cell": c, "options": [[piece, rot], ...], "index": j}, ... ],
  "state_fnv": "<16 hex>"           // FNV-1a over all domains at the deepest level
}
```

`stack[i].options` is the full option list of level i in search order, so
the remaining options are `options[index..]`. For i < L the option at
`index` is in progress (already counted); for the last level it is the next
one to try (not counted). Every integer is a JSON number (all below 2^53
except the hash, which is a hex string); pieces and rotations are the
puzzle's own indices, independent of the internal bit layout, so the file is
portable between machines and endiannesses.

Resume rebuilds the propagated domains by replaying the decisions from the
root: at each level i < L, `choose` must return `stack[i].cell`, the cell's
domain must equal `stack[i].options` in order, the option at `index` is set
and `propagate` must succeed; at level L, `choose` and the domain are checked
the same way, and the replayed deepest-level domains must hash to
`state_fnv`. Any check failing aborts the resume with an error (no silent
fallback). Replay calls neither `try_assign` nor the hook and its propagate
calls are not counted. This is exact because `choose` and `propagate` are
deterministic functions of the domains; the gate checks it.

Counting across segments: each segment runs with a fresh `NodeCounter` whose
cap is (total cap) - (nodes before this segment), and calls `try_assign`
exactly once per assignment it makes. Totals are the sum over segments.

### Row format

One JSON line per run segment: instance metadata, `solver`
("bb-mrv-fast-resumable"), `segment`, `cap` (total cap this segment stopped
at), `segment_cap`, `status` (solved | censored | error), `nodes`,
`backtracks`, `propagation` (totals) and `nodes_segment`,
`backtracks_segment`, `propagation_segment`, `board`, `hash`, `hashed`,
`closed_share`, `depth`, `deepest`, `checkpoint` (path or null),
`resumable` (true iff a checkpoint was written), `resumed_from`,
`wall_ms` (this segment), `wall_ms_total`, `replay_ms`,
`checkpoint_write_ms`, `checkpoint_bytes`. A capped row thus reads
"censored at cap, closed share x, resumable from <path>".

## Gate (binding)

Cap C = 1,000,000, guard 600 s, one thread. Instances, Track 0's gate set
(140): primary n=6 test 1..30 and calibration 1001..1010; primary n=7 test
1..30 and calibration 1001..1010; 8-c10 test 1..30 and calibration
1001..1010; 8-c10-fresh test 31..50.

- **G-a.** Per instance, in one process: `FastSolver<HashHook>` (unmodified,
  Track 0 crate) and `ResumableSolver<HashHook>` with progress records every
  100,000 nodes (finer than the default, to exercise the event path) and no
  checkpoint. Equal: hash, hashed count, nodes, backtracks, propagate calls,
  status, board. Also checked: progress lines have nondecreasing closed share
  and nodes equal to multiples of 100,000.
- **G-b.** Three processes per batch: run to C/3 = 333,333 with checkpoint,
  exit; resume every checkpoint in a new process to 2C/3 = 666,666, checkpoint,
  exit; resume to C = 1,000,000. The final row of each instance (the segment
  where it solved, or the last one) must equal G-a's `FastSolver` row on
  hash, hashed, nodes, backtracks, propagate calls, status and board, and the
  uninterrupted resumable run's closed share and deepest depth. Reported: how
  many instances solve before the first checkpoint, between checkpoints, after
  the second, and are censored at C.
- **G-c.** The same G-a and G-b checks at 9x9 c=11 on the plus2 calibration
  seeds 1001..1010 (10 instances), C = 3,000,000, checkpoints at 1,000,000 and
  2,000,000.
- **G-d** (only if `dell`, Linux x86_64, is reachable): checkpoints written on
  this Mac at G-c's first checkpoint (1,000,000 nodes) for at least 3 of the
  9x9 instances are copied to `dell`, resumed there to 3,000,000 in one
  segment, and the final rows must equal the Mac's uninterrupted rows on the
  same fields.

One mismatch in G-a, G-b, G-c or G-d fails the gate and stops the track.

## Timing (target)

8x8 c=10: 8-c10 test seeds 1..30 and 8-c10-fresh test 31..50 (50 instances),
cap 1e6, one thread, `nice`, no pinning. Arms: A = plain `bb-mrv-fast`
(`FastSolver<NoHook>`, unmodified), B = `ResumableSolver<NoHook>` with
progress records every 1,000,000 nodes to a file and checkpoint capability
compiled in, no checkpoint taken. Per instance 10 runs through hard-ladder's
`run_one` in the order A B B A A B B A A B (5 per arm). Per-instance
statistic: the minimum `wall_ms` per arm; ratio r_i = min_B / min_A;
statistic: median of r_i (upper median for even counts); 95% CI percentile
bootstrap, 10,000 resamples of instances, Python `random.Random(20261111)`.
Every timed run of both arms must give identical nodes and status.

**Acceptance line:** upper end of the 95% CI of the median ratio <= 1.03.
Above it, the overhead target is missed (reported as such; the gate verdict
stands on its own).

Checkpoint cost is reported separately from the G-b/G-c rows: write time
(ms) and file size (bytes) at each checkpoint, and replay time at resume, as a
function of the checkpoint depth.

## Out of scope (listed as not done)

The scb solver (hard-ladder crate, the slow arm of the 9x9 comparison) and any
change to `NodeCounter`.

## Machine, budget, disclosure

Apple M4 (4 performance + 6 efficiency cores), macOS, shared with other
tracks tonight: `cargo build -j 2`, single-threaded runs, at most 2 at once,
`nice`. Absolute times from tonight are not to be quoted; only the paired
ratio is a claim. G-d: `dell` for a few minutes, 1 thread. Budget about 3.5
hours wall clock in total.

Before this commit no code for this track had been written or run. Development
runs use calibration seeds only.

## Files

`results/raw/*.jsonl` (rows, progress lines), `results/ckpt/` (checkpoints),
`results/run-log.jsonl` (prereg guard log, never edited),
`analysis/analyze.py` (stdlib), `results/summary.json`, `README.md`,
`run.sh`, one recipe `just experiments resumable-runs`.
