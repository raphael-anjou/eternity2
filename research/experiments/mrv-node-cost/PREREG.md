# Preregistration: bb-mrv node cost (night shift 2026-11-12, Track 0)

Written and committed locally before any test seed runs. Branch
`night/2026-11-12-t0-node-cost` from `origin/main` (6247af0). Nothing is
pushed tonight. Forced changes go in `PREREG_AMENDMENTS.md` (what, why, when);
this file stays unchanged. Generated boards only; nothing here is official
Eternity II progress.

## Track label

**Optimization** (research/program/PROTOCOL_TARGETS_PROPOSAL.md section 1).
The search tree must not change. If the node sequence differs on any gate
seed, this is not an optimization: the track stops and reports, and no
wall-clock claim is made.

## Hypothesis

bb-mrv (A16 `BbSolver { rule: Rule::Mrv }`, the A22 baseline at 8x8 c=10)
spends most of its time per node on overhead that does not depend on the
search: heap allocation per node (`to_vec` of the domain state, the option
list, the free-cell list, the propagation queue and the all-different arrays),
a cells x pieces scan in every all-different pass, and a colour-by-colour
offer test. A solver that removes that overhead and computes the same
propagation fixpoint visits the identical node sequence and is faster in wall
clock at every rung.

## Arms

| name | what | where |
|---|---|---|
| `bb-mrv` | original, unchanged code: `backdoor_branching::BbSolver { rule: Rule::Mrv, guard: 600 s }`, exactly as A22 constructs it | path dependency on `../backdoor-branching/compute` |
| `bb-mrv-ref` | instrumented verbatim copy of the original search (gate only, never timed) | `compute/src/lib.rs` `RefSolver` |
| `bb-mrv-fast` | candidate | `compute/src/lib.rs` `FastSolver` |

All three run through hard-ladder's `NodeCounter`; no existing crate,
NodeCounter or instance file is edited.

### How the reference node sequence is obtained

`NodeCounter::try_assign` does not see the (cell, piece, rotation), so the
unmodified original cannot be hashed directly. `RefSolver` is a verbatim copy
of `Search::choose` (the `Rule::Mrv` path) and `Search::dfs` from
`backdoor-branching/compute/src/lib.rs`, calling the unmodified public
`backdoor_branching::{options, assign}` and `sat_backdoors::Model`, with one
added line: right after each successful `try_assign`, a hook receives
(cell, `opt_piece[o]`, `opt_rot[o]`). On every gate instance the gate runs the
unmodified `BbSolver` too and requires `RefSolver` to match it exactly on
status, node count, backtracks, propagate calls and the returned board, and
requires the number of hashed nodes to equal the node count. Only then is the
copy's hash the reference.

Hash: 64-bit FNV-1a over the bytes (cell u16 LE, piece u16 LE, rotation u8)
of every node in order, starting from the FNV offset basis.

### What the candidate changes (cost only)

Option bitsets re-laid out with 4 bits per piece (rotation rank inside the
piece kept, so ascending bit order equals the original option order),
const-generic fixed-size domains, a preallocated stack of states (one copy per
node as before, no allocation), piece presence per cell by a nibble fold, the
all-different pass on piece-presence masks (piece with no cell, piece
singles, hidden singles), offered colours by walking the set bits of domains
with at most 12 options, and a free-cell scan for min (|D|, index) without
building a list. Propagation computes the same rules (rim unary at the root,
edge arc consistency, piece singles, hidden singles) to fixpoint. The
rules are monotone and deflationary, so every application order reaches the
same greatest fixpoint and fails on the same states; the gate checks this
empirically rather than relying on the argument.

## Gate (binding)

Cap 1,000,000 nodes, guard 600 s (never expected to fire), one thread.
Every calibration and test seed of:

| rung | ladder | n | c | seeds |
|---|---|---|---|---|
| R6 | primary | 6 | 5 | test 1..30, calibration 1001..1010 |
| R7 | primary | 7 | 6 | test 1..30, calibration 1001..1010 |
| R8 | 8-c10 | 8 | 10 | test 1..30, calibration 1001..1010 |
| R8 | 8-c10-fresh | 8 | 10 | test 31..50 (no calibration file exists) |

140 instances. Gate passes iff on all 140: `ref == orig` (status, nodes,
backtracks, propagate calls, board, hashed count = nodes) AND
`fast == ref` (hash, hashed count, status, nodes, backtracks, propagate
calls, board). One mismatch fails the gate and stops the track.

Supplementary (not binding, reported): same three-way check on plus2
calibration 9x9 c=11 seeds 1001..1003 and 10x10 c=12 seed 1001 at cap
100,000, because Track 1 would run the candidate at 9x9.

## Timing (target)

Test seeds only: R6 (primary n=6, seeds 1..30), R7 (primary n=7, 1..30), R8
(8-c10 seeds 1..30 plus 8-c10-fresh 31..50, 50 instances). Cap 1e6, single
thread, `nice`, no core pinning (macOS). Per instance, 10 runs through
hard-ladder's `run_one` in the fixed order A B B A A B B A A B (A = bb-mrv,
B = bb-mrv-fast), so each arm gets 5 repetitions and neither arm is ever timed
as a block before the other. `wall_ms` is `run_one`'s timer around
`solve` (includes `Model::new` for both arms).

- Per-instance statistic (preregistered): the **minimum** of the 5 `wall_ms`
  per arm (least sensitive to interference from other processes on the
  shared machine). The per-instance median is reported as a secondary.
- Paired ratio r_i = min_fast / min_orig. Rung statistic: median of r_i.
  95% CI: percentile bootstrap, 10,000 resamples of instances, seed 20261111
  (Python `random.Random(20261111)`), upper median for even counts as in
  PROTOCOL.md.
- Every timed run must also reproduce identical nodes and status between the
  arms; any difference is a gate failure.

## Decision rule

- **Kill:** the gate fails on any of the 140 instances, OR at any rung the
  upper end of the 95% CI of the median ratio is >= 1.
- **Adopt for Track 1:** gate passes AND at every rung the CI upper end is
  < 1. Track 1 then uses binary `mrv_node_cost` (crate
  `research/experiments/mrv-node-cost/compute`), solver `bb-mrv-fast`.
- No speed claim is made beyond the measured rungs and this machine; the
  gate does not cover n >= 9 beyond the supplementary check.

## Machine, budget, disclosure

Apple M4 (4 performance + 6 efficiency cores), macOS, rustc 1.95.0, release
profile (opt-level 3, codegen-units 1). One core (`--threads 1`, `nice`),
`cargo build -j 2`, 2 hours wall clock in total. Other night-shift tracks run
concurrently on the same machine, so absolute wall clock is noisy; the
alternation and per-instance minimum are the guard against that.

Before this file was written, the candidate was developed and checked only on
calibration seeds (primary n=6 and 8-c10, 20 instances, 0 mismatches;
roughly 4.8x faster at 8x8). The code is frozen at the commit that follows
this one; the gate reruns those calibration seeds with the frozen code.

## Files

`results/gate.jsonl`, `results/gate-supp.jsonl`, `results/timing.jsonl` (raw),
`analysis/analyze.py` (stdlib), `results/summary.json`, `README.md`, recipe
`just experiments mrv-node-cost`.
