# Track 9: resumable long runs for bb-mrv-fast (optimization, night 2026-11-12)

Preregistration: [`PREREG.md`](PREREG.md) (commit `7fda502`, committed alone
before any run; code frozen at `68c79df`). Track label: **optimization**
(harness): the search does not change. Generated boards only; nothing here is
official Eternity II progress.

## Outcome (self-assessed; a separate judge decides)

All numbers below are in `results/summary.json`, produced by
`python3 -I analysis/analyze.py` from `results/raw/*.jsonl`.

| gate | instances | result | summary.json key |
|---|---|---|---|
| G-a: progress on (every 1e5), no checkpoint, vs unmodified `FastSolver<HashHook>`, cap 1e6 | 140 (Track 0's set) | 0 mismatches on hash, hashed count, nodes, backtracks, propagate calls, status, board; progress lines at exactly the expected node counts, closed share never decreasing | `gate.<set>.ga_mismatch`, `ga_progress_bad` |
| G-b: 3 processes, checkpoints at 333,333 and 666,666, end at 1e6 | 140 | 0 mismatches against G-a's `FastSolver` row (same fields) and against the uninterrupted run's closed share and deepest depth; progress lines identical to the uninterrupted run's (elapsed time aside); per-segment counts sum to the totals | `gate.<set>.gb_*` |
| G-c: same at 9x9 c=11, plus2 calibration 1001..1010, C = 3e6, checkpoints at 1e6 and 2e6 | 10 | 0 mismatches (G-a and G-b checks) | `gate.n9` |
| G-d: Mac checkpoints at 1e6 (9x9 seeds 1001..1004) resumed on `x86` (Linux 7.0, x86_64) to 3e6 in one segment | 4 | 0 mismatches against the Mac's uninterrupted rows, closed share and deepest depth included | `gate_d` |

G-b coverage by where the instance stopped (`gate.<set>.classes`), over the
140 instances: 108 solved before the first checkpoint, 16 between the first
and second, 6 after the second, 10 censored at C (R7 only; at 8x8 c=10 no
instance needed the third segment). At 9x9 all 10 are censored at 3e6.

**Timing** (`timing`): 8x8 c=10, 50 instances (8-c10 test 1..30,
8-c10-fresh 31..50), cap 1e6, A B B A A B B A A B, 5 runs per arm. Ratio
resumable (progress every 1e6, no checkpoint) over plain `bb-mrv-fast`,
per-instance minimum: median **1.008**, 95% CI **[0.998, 1.011]**. Upper end
at or below the preregistered line 1.03: **accepted**. Secondary
(per-instance median of 5): 1.005 [0.990, 1.013]. Every timed run of both
arms gave identical nodes and status. The timing stage (21:03:48 to
21:12:31 CET) ran next to the G-c stage (until 21:12:07) and then next to
the start of the supplementary kill test below (at most 2 of this track's
processes at once), and other tracks shared the machine;
absolute times from tonight should not be quoted, only the paired ratio.

**Checkpoint cost** (`checkpoint_cost`, 88 writes at a cap, 68 resumes;
binned by the depth of the stack): a checkpoint is 1.3 to 2.3 KB (median
1.4 KB at depth 10-19 on 7x7, 2.1 KB at depth 30-39 on 9x9), grows linearly
with depth (one entry per level, a few options each), and takes about 1 ms
to write (median 1.0 to 3.2 ms per bin; the maximum, 42 ms, is file-system
noise on a loaded machine). Replaying it at resume takes 0.06 ms (7x7, depth
10-19) to 0.35 ms (9x9, depth 30-39), at most 1.3 ms: the cost of about
depth propagate calls, negligible against any segment.

**Supplementary, not preregistered** (`results/raw/crash-s*.jsonl`,
`progress-crash.jsonl`, `results/ckpt/crash/`): 9x9 calibration seed 1005
started with cap 3e6 and a periodic checkpoint every 5e5, killed with
SIGKILL after 14 s (at about 1.6e6 nodes; the surviving checkpoint was at
1.5e6), then resumed to 3e6: hash, counts, status, closed share and deepest
depth equal the uninterrupted run.

Wall clock of the stages (`results/run-log.jsonl`, end lines): G-a and G-c's
G-a part 16.6 min, G-b and G-c's G-b part 8.3 min (the guard logs G-c under
the stage names `gate-a` and `gate-b`), timing 8.7 min. G-d ran on `x86`;
its guard log is `results/run-log-x86.jsonl` (no git checkout there, as
logged).

### Fix round after the judge (amendment A1)

`JUDGE.md` found three defects (D1 unchecked checkpoint content, D2 errors
exiting 0, D3 closed share 0 on a root refutation). The fix is recorded in
`PREREG_AMENDMENTS.md` A1 (code `299f8d3`, written and committed before any
run of it); the checkpoint format is now version 2 (below). Numbers in this
section come from `results/fix/summary.json` (`python3 -I analysis/analyze.py
results/fix/raw results/fix/summary.json`) and `results/fix/compare.json`
(`python3 -I analysis/fix_compare.py`), both run by `./run.sh fix-analyze`.

| gate | result | key |
|---|---|---|
| G-a, G-b rerun with the fixed binary (140 instances) | 0 mismatches, 0 progress mismatches, 0 segment-chain errors; classes 108 / 16 / 6 solved in segment 1 / 2 / 3, 10 censored, as before | `summary.json` `gate.<set>` |
| G-c rerun (9x9, 10 instances) | 0 mismatches; 10 censored at 3e6 | `summary.json` `gate.n9` |
| every rerun row against the committed version 1 row (same file, same instance) | 368 rows in 32 files, 0 differences on status, nodes, backtracks, propagation, board, hash, hashed, closed share, deepest (and for G-b rows segment, per-segment counts, depth, resumable) | `compare.json` `cross_version` |
| G-e tamper suite: 7x7 primary test s1 at 4e5, 8x8 8-c10 test s5 at 333,333, 9x9 plus2 test s1 at 5e5 | 2202 alterations (718 / 661 / 823), **2202 refused** (exit 1 and an error row), 0 accepted; each unaltered copy resumed with exit 0 to the uninterrupted run's row | `compare.json` `gate_e` |
| G-d (cross-machine) | not rerun (below) | |

What the digest-independent checks catch alone (reported, not gated): the
same alterations with the digest recomputed, 1408 of 1436 refused. The 28
accepted are the fields no stack replay can recompute: `cap` +1, `deepest`
+/-1, the `hash` value, `segments` +1, `totals.backtracks` +/-1,
`totals.elapsed_s`, and on the 9x9 checkpoint dropping the last stack level
(depth 27: the dropped level has index 0, so it adds exactly 0 to the closed
share, and the level above it has a single option, so the replayed domains
at the new deepest level still hash to `state_fnv`; the accepted resume is a
different search, with one forced node counted twice. The cause is
structural, not rounding, and can occur at any depth). Every stack `index`
change was caught by the closed-share check on all three checkpoints. These
fields rest on the digest alone. Open follow-up: adding the stack depth and
last-level index to `state_fnv` would close this without the digest; it
changes behaviour, so it needs its own gate run.

Unit tests (`compute/tests/fix.rs`, calibration seeds, `cargo test
--release`): 5 tests, all pass (digest round trip and refusals, forged digest
against the load checks, D2 exit codes and the low-cap row through the
binary, D3 on a 3x3 puzzle refuted at the root, missing and impossible
progress directories).

**Hot path unchanged.** `git diff 86f302a 299f8d3 -- compute/src/lib.rs`
has hunks only at the format structs and `impl Checkpoint` (old lines
29-160), `checkpoint` / `write_checkpoint` (615-623), the body of the cold,
non-inlined `event` (664-678, failed writes recorded instead of ignored),
the end of `replay` (719), the setup of `solve_w` (812-848) and
`ResumableSolver::solve` (931-947). `Fast` (the solver state), `dfs`,
`branch`, `dfs_resume`, `propagate` and `choose` (old lines 223-544) are
untouched, so the preregistered timing was not rerun.

**G-d not rerun.** `x86` is reachable but has no Rust toolchain any more
(no `cargo`, no `~/.rustup`), so the fixed binary cannot be built there
without installing one. The change cannot affect what G-d tested: replay and
search are unchanged, the digest is FNV-1a over the bytes of a JSON text
whose integers are decimal and whose floats are printed and parsed exactly
by the same `serde_json` code on both machines (with `float_roundtrip`), and
the closed-share check compares an f64 computed by IEEE division and
addition, correctly rounded on x86_64 and aarch64 alike.

Guard log: the fix-round runs are the 38 start lines with a `run_id` in
`results/run-log.jsonl` (20:34:56 to 20:46:08 UTC, 11.2 min wall clock on
two lanes; every start paired with an end, every exit 0). Their `head` is
`299f8d3` or `e013e8c` (the second commit adds only the tests); a
`dirty_paths` of 1 on some lines is this README being edited while the runs
went, never `compute/` (the runs used the copy `compute/target/gate-bin/`
built from `299f8d3`).

### Closed share in practice

At the cap, R7 censored runs had closed 6% to 14% of their tree after 1e6
nodes (`gate.r7-test.share_at_cap`); 9x9 c=11 runs had closed 0.06% to 1.1%
after 3e6 nodes (`gate.n9.share_at_cap`). Read the limits below before using
either as a forecast.

## What it gives a long run

- **Progress records.** Every N nodes (default 1e6) one JSON line is appended
  and flushed: total nodes, backtracks, propagate calls, current depth,
  deepest depth reached, elapsed seconds, closed share. A two-hour run can be
  watched with `tail -f`.
- **Checkpoint and resume.** At the node cap (and optionally every M nodes)
  the solver writes a JSON checkpoint; `resumable_runs resume` continues the
  same search in a new process, on this machine or another one, up to a
  higher total cap. Totals are sums over segments; the running node-sequence
  hash continues across segments.
- **A row that says how far it got.** A capped row carries `closed_share`,
  `depth`, `deepest`, `checkpoint` and `resumable`: "censored at 1e8" becomes
  "censored at 1e8, closed share x, resumable from <path>".

### Closed share: definition and limits

At a progress or checkpoint point the stack has levels 0..L; level i was
entered with K_i options, of which j_i are exhausted (for i < L option j_i is
in progress; at level L option j_L is the next one to try). Closed share is
sum_i (j_i / K_i) * prod_{l<i} (1 / K_l), computed by Horner from the deepest
level up, s = (j_i + s) / K_i, in f64. Every operation is monotone, so the
computed value never decreases; it is 1 exactly when the search is exhausted.
The gate checks that it never decreased on any progress line.

What it is not:

- It measures progress toward **exhausting** the tree, not toward a first
  solution. A planted instance can solve at closed share 0.0001, and a run can
  sit at 0.9 with no solution in the remaining 10%.
- Under MRV the branching factors differ between levels and between subtrees
  (K_i is the size of whatever domain MRV picked), so equal slices of the
  share are not equal amounts of work. The share early in a run is dominated by
  the first few levels and moves in steps when a shallow option closes.
  **It is not a time estimate**: do not extrapolate "37% after 2 h, so 5.4 h
  in total".
- It is exact only in the sense of its definition (f64 rounding aside); it
  is not an estimate of the tree size (Knuth's estimator would be a different,
  noisy quantity).

### Checkpoint format (version 2)

Version 2 since the fix round (`PREREG_AMENDMENTS.md` A1). **Checkpoints
written before the fix (version 1, including every file under
`results/ckpt/`) cannot be resumed**: they carry no content digest, so an
altered stack index, total or hash in them could not be detected (the judge
showed such an edit resuming silently as a different search), and the
current build refuses them with a message saying so. Rerun such a search
from the start.

One UTF-8 JSON object, written to `<path>.tmp` and renamed. Fields:
`format` ("e2-bb-mrv-fast-checkpoint"), `version` (2), `solver`, `instance`
(id, ladder, role, n, c, seed), `model_fnv` (digest of the option tables and
root domains, so a checkpoint cannot be resumed on another instance or a
changed model), `segments`, `totals` (nodes, backtracks, propagation,
elapsed_s), `hash` and `hashed` (running node-sequence hash), `deepest`,
`closed_share`, `stack` (per level: `cell`, `options` as [piece, rotation] in
search order, `index`), `state_fnv` (digest of the deepest level's domains).
Pieces and rotations are the puzzle's own indices and every number is
decimal JSON (the 64-bit hashes are hex strings), so the file does not depend
on the machine's endianness or word size. `cap` is the total cap of the
segment that wrote the file. `digest` is FNV-1a 64 over the compact JSON of
the checkpoint with `digest` set to "": it detects any change of content
(corruption, a hand edit); it is not a signature, since anyone can recompute
it. Unknown fields are refused.

On load, in this order: format; version (version 1 gets its own refusal);
the digest, before any replay; then checks that hold for every checkpoint
the solver writes and need no puzzle: segments >= 1, nodes <= cap,
propagation = nodes + 1 (one propagate call per node plus the root's),
backtracks <= nodes, hashed = nodes, hash fields well formed, stack
non-empty and no deeper than the board, deepest >= stack depth, closed share
in [0, 1), every cell and index in range. After the replay below, the closed
share recomputed from the replayed stack must equal the stored one bit for
bit. These checks do not rely on the digest; what they cannot catch alone
(the hash value, backtracks, elapsed time, deepest, segments, cap, and
dropping a last stack level whose index is 0 under a forced level, which
leaves both the closed share and `state_fnv` unchanged) is covered only by
the digest (G-e below measures both).

Resume replays the decisions from the root (no domains are stored): at each
level `choose` must return the recorded cell, the domain must equal the
recorded option list, the recorded option is set and propagation must
succeed; the replayed deepest-level domains must hash to `state_fnv`. Replay
calls neither `NodeCounter::try_assign` nor the hook and its propagate calls
are not counted. Each segment uses a fresh `NodeCounter` with cap (total cap)
minus (nodes in the checkpoint) and calls `try_assign` exactly once per
assignment it makes.

### Row fields

`cap` is the total cap the segment stopped at; `segment_cap` is that segment's
own `NodeCounter` cap. `nodes`, `backtracks`, `propagation` are totals over
all segments so far; `*_segment` are this segment's. `wall_ms` is this
segment, `wall_ms_total` adds the elapsed time stored in the checkpoint.
`replay_ms`, `checkpoint_write_ms`, `checkpoint_bytes` give the costs.

## Commands for a long run

From `research/experiments/resumable-runs`, after `./run.sh build`:

```
B=compute/target/release/resumable_runs
# 1. Start: 9x9 c=11 plus2 test seed 3, total cap 1e8, progress every 1e6,
#    checkpoint every 1e7 (crash safety) and at the cap, no wall-clock guard.
nice $B run --ladder plus2 --role test --sizes 9 --seeds 3-3 --cap 1e8 --guard 0 \
  --progress runs/n9-s3.progress.jsonl --progress-every 1e6 \
  --checkpoint-dir runs/ckpt --checkpoint-every 1e7 --out runs/n9-s3.rows.jsonl
# 2. Watch it.
tail -n 1 runs/n9-s3.progress.jsonl
# 3. Continue the same search to a total of 3e8 (new process). Same machine
#    is the measured case; a version 2 checkpoint resumed on another machine
#    is argued (decimal JSON, IEEE arithmetic), not measured: only version 1
#    checkpoints were resumed across machines (G-d).
nice $B resume --from runs/ckpt/n9-c11-s3.ckpt.json --cap 3e8 --guard 0 \
  --progress runs/n9-s3.progress.jsonl --progress-every 1e6 \
  --checkpoint-dir runs/ckpt --checkpoint-every 1e7 --out runs/n9-s3.rows.jsonl
```

`--cap` is always the total over all segments. The progress file's
directory is created if missing; a progress file that cannot be opened fails
the segment before any node (error row, exit 1), and a failed progress or
periodic checkpoint write makes the row an error.

**Failures are loud.** `run`, `resume` and `gate-a` exit with code 1 when
any row they write has status `error`: a checkpoint that fails to load or
replay (it gets its own error row; the other checkpoints of a directory
still run), a cap at or below the checkpoint's node count (the row then
reports the checkpoint's totals and segment), an unverified board, an
exhausted search (the ladder instances are all solvable), or a failed
write. A driver script can rely on the exit code. If the process is killed, the
last periodic checkpoint is the restart point. The resumed totals are exact
(they start from the checkpoint's totals); the nodes searched after that
checkpoint are searched again, so the progress file then holds the killed
segment's lines above the checkpoint's node count twice (keep the later
ones). With
`--guard` left at its default (600 s), a guard stop is reported as
`guard_fired` and is not resumable; long runs should pass `--guard 0`.

## Not done (out of scope)

- **scb** (`hard-ladder/compute/src/solvers/scb.rs`, the slow arm of the 9x9
  comparison) is not resumable. It drives a `single-core-benchmark` engine
  through the `Solver` trait and only sees `ValueTried` events through an
  event sink; the engine's search stack, value order state, RNG and
  propagation state are internal to `solver-engine`. Making it resumable
  would take a resume API inside that engine (serialize its decision stack
  and per-level iterator positions, rebuild its domains by replay, restore its
  RNG), or a skip-replay that reruns the search from scratch and suppresses
  the first k events, which costs a full rerun and so saves nothing. Its cap
  also overshoots (it polls `should_continue` once per recursion), so segment
  boundaries would not be exact. Progress lines (nodes, depth) could be
  emitted from the sink, but not the closed share, because the events do not
  carry domain sizes.
- No change to `NodeCounter`, the hard-ladder crate, the A22 crates or the
  Track 0 crate.

## Reproduce

```
just experiments resumable-runs          # = resumable-runs/run.sh all
# stages: build | gate-a | gate-b | gate-c | time | analyze
# fix round (A1): fix-bin | fix-gate-a | fix-gate-b | fix-gate-c | tamper | fix-analyze
cd compute && cargo test --release       # unit tests (calibration seeds)
python3 -I analysis/analyze.py           # results/summary.json from results/raw/
```

Every run stage goes through `../common/prereg_guard.py`, which logs each
command to `results/run-log.jsonl` (append-only, committed).
