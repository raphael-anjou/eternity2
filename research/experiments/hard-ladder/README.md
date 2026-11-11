# Hard-instance ladder (n = 6..10, Owen colour counts)

Shared infrastructure for round-2 exact-search candidates: fixed planted
instances, a solver-independent node counter, calibrated DFS baselines, a
plug-in harness, and a comparison protocol (`PROTOCOL.md`). Candidates are
compared by search-tree size (nodes), with wall clock and solve rate reported
next to it. Preregistration: `PREREG.md` (commit 8b316e4), amendments
`PREREG_AMENDMENTS.md` (A1 test cap 1e6, A2 rungs 9/10 deferred to candidate
sessions).

## Outcome against the kill criterion (self-assessment; a separate judge decides)

**Killed at the preregistered colour sets and the amended cap.** Condition 2
fired on both colour sets: the best baseline solves **0 of 30** 8x8 test
instances within C = 1,000,000 nodes (primary c = 8 and secondary c = 7;
Wilson 95% upper bound 0.11; threshold 15/30). Condition 1 (strict increase of
the median) holds on the measured rungs 6 < 7 < 8 for the best baseline on both
sets (8 is "> C" after a finite 7); rungs 9 and 10 were not run here (A2).

Which fix applies (PREREG: "the cap or the colours must change"): the colours,
not the cap. The committed hardness-peak sweep, run with the same generator,
scramble and plain DFS on the same seeds 1..30, solves **0 of 30** at 8x8 c = 8
even with 1.5e7 placements, and 0/30 at every c <= 8; it solves 23/30 at c = 9
(KM median 8.5e6 placements) and 30/30 at c = 10 (KM median 5.1e5). The
hardness peak of this generator at 8x8 therefore sits at or below Owen's
criterion (7.56): counts at or just below the one-expected-solution point are
the hardest, not the most informative. A usable 8x8 rung within a one-minute
cap needs c >= 10 at 8x8 (scaled correspondingly at other sizes), which is a
new colour set and needs its own amendment and calibration before candidates
use it. That change is not made here.

What is usable now: the harness, node counter, instances, protocol, plug-in
interface and the committed 6x6 and 7x7 baseline rows. A candidate can run
G1 at rung 6 today; G1 at rung 8 cannot pass against a baseline censored on
every instance unless the candidate solves at least half of the 8x8
instances (both-censored pairs count as ties, log-ratio 0).

## Instances

`instances/<ladder>-<role>.jsonl`, one row per instance: `puzzle` (what a
solver receives: pieces scrambled and rotated, no hints) and `planted` (the
generator's board, `piece*4 + rotation` row-major, used only by the harness to
check `equals_planted`). Generator: `generate_solved_framed_bc(n, c + 5, seed,
true, 5)` then the hardness-peak scramble (`XorShift(seed ^ 0xA5A5A5A5)`
shuffle + per-piece rotation), verified by test to be byte-identical to the
hardness-peak code. `hard_ladder check-instances` regenerates and compares.

| n | Owen I | primary c | secondary c | frame colours | test seeds | calibration seeds |
|---|---|---|---|---|---|---|
| 6 | 5.2155 | 5 | 6 | 5 | 1..30 | 1001..1010 (primary) |
| 7 | 6.3803 | 6 | 7 | 5 | 1..30 | 1001..1010 |
| 8 | 7.5579 | 8 | 7 | 5 | 1..30 | 1001..1010 |
| 9 | 8.7435 | 9 | 8 | 5 | 1..30 | 1001..1010 |
| 10 | 9.9346 | 10 | 9 | 5 | 1..30 | 1001..1010 |

Rounding: primary = nearest integer, secondary = the other neighbour. Frame
gap: the generator confines 5 colours to the border band as in E2, but does
not reproduce E2's per-colour frame census, its rare-colour ring fraction or
its clue pieces (see `PREREG.md`).

## Node metric

One node per (cell, piece, rotation) assignment the search makes that agrees
with the rim and every assigned neighbour (`src/counter.rs`,
`NodeCounter::try_assign`). Propagation work, fit tests and backtracks are
separate columns. For the main engine this is its placement count; for the
single-core-benchmark registry it is the number of `ValueTried` events, which
a test checks equal to the solver's native `FinalStats.nodes`, and the sink
replays every try against the partial board (`inconsistent` = 0 on every
row). Cross-solver checks: `scb-naive_rowmajor` equals `hp-dfs` and
`scb-naive_spiral` equals `engine-dfs-spiral-in` node for node on all 50
calibration instances; `hp-dfs` reproduces four committed hardness-peak rows
exactly (test `hp_dfs_matches_committed_hardness_peak_rows`).

Cap: 1,000,000 nodes per instance on test seeds (A1), 2,000,000 on
calibration seeds. Unsolved runs are right-censored at the cap and kept; the
median is the Kaplan-Meier median (ceil(N/2)-th order statistic with censored
= +infinity).

## Baselines (test seeds, cap 1,000,000)

Calibration (`CALIBRATION.md`, 14 candidates, 1220 s) selected
**scb-gacolor_ac3_random** (gacolor AC-3 propagation, randomised value order,
fixed run seed 1). `hp-dfs` is the plain row-major DFS of hardness-peak.

Primary colours:

| solver | n | c | solved/30 | Wilson 95% | KM median nodes | Q1 | Q3 | median wall ms |
|---|---|---|---|---|---|---|---|---|
| scb-gacolor_ac3_random | 6 | 5 | 30 | 0.89-1.00 | 951 | 620 | 1,940 | 6 |
| scb-gacolor_ac3_random | 7 | 6 | 16 | 0.36-0.70 | 921,773 | 272,281 | > cap | 7,347 |
| scb-gacolor_ac3_random | 8 | 8 | 0 | 0.00-0.11 | > cap | > cap | > cap | 16,074 |
| hp-dfs | 6 | 5 | 30 | 0.89-1.00 | 41,823 | 27,904 | 62,556 | 24 |
| hp-dfs | 7 | 6 | 0 | 0.00-0.11 | > cap | > cap | > cap | 652 |
| hp-dfs | 8 | 8 | 0 | 0.00-0.11 | > cap | > cap | > cap | 930 |

Secondary colours:

| solver | n | c | solved/30 | Wilson 95% | KM median nodes | Q1 | Q3 | median wall ms |
|---|---|---|---|---|---|---|---|---|
| scb-gacolor_ac3_random | 6 | 6 | 30 | 0.89-1.00 | 300 | 158 | 469 | 2 |
| scb-gacolor_ac3_random | 7 | 7 | 30 | 0.89-1.00 | 77,575 | 40,824 | 132,947 | 1,279 |
| scb-gacolor_ac3_random | 8 | 7 | 0 | 0.00-0.11 | > cap | > cap | > cap | 13,659 |
| hp-dfs | 6 | 6 | 30 | 0.89-1.00 | 2,374 | 1,323 | 4,713 | 1 |
| hp-dfs | 7 | 7 | 20 | 0.49-0.81 | 693,573 | 535,019 | > cap | 497 |
| hp-dfs | 8 | 7 | 0 | 0.00-0.11 | > cap | > cap | > cap | 620 |

Fitted growth (OLS of log10 KM median on cell count, finite medians only,
two points so no CI): best baseline x1.70 per cell (primary, 6 to 7) and x1.53
per cell (secondary); hp-dfs x1.55 per cell (secondary). Bootstrap fraction of
resamples with a strict increase over 6..8: 0.70 (primary) and 1.00
(secondary) for the best baseline. Full numbers: `results/summary.json`;
per-instance survival curves: `results/survival.csv`; raw rows:
`results/test/<ladder>/<solver>.jsonl`.

Comparison with past results: at 8x8 c = 8 (primary) hp-dfs solves 0/30 at
1e6 nodes and hardness-peak's committed sweep solves 0/30 at 1.5e7 placements
on the same seeds and the same instances, so the setups agree; the difference
is only the cap (hardness-peak budgets 3e7 DFS steps = placements +
backtracks, this ladder counts placements). The scaling-ladder topic uses 22
colours at every size and a wall-clock budget, so its curves are not
comparable node for node.

## Plug-in interface

A candidate lives in its own experiment directory and depends on this crate
by path:

```toml
[dependencies]
hard-ladder = { path = "../../hard-ladder/compute" }
```

```rust
use hard_ladder::{LadderSolver, NodeCounter, Outcome, Puzzle, SolverEntry};

struct MySolver;
impl LadderSolver for MySolver {
    fn solve(&mut self, puzzle: &Puzzle, counter: &mut NodeCounter) -> Outcome {
        // For every (cell, piece, rotation) assignment, call
        // counter.try_assign() first; if it returns false, return Outcome::CapReached.
        // Count propagation in counter.propagation, backtracks in counter.backtracks.
        // Return Outcome::Solved(board) with board[cell] = piece*4 + rotation (row-major).
        unimplemented!()
    }
}

fn main() -> std::process::ExitCode {
    hard_ladder::cli::main_with(&[SolverEntry {
        name: "my-solver",
        description: "what it does",
        run_seed: 0,
        make: |_seed| Box::new(MySolver),
    }])
}
```

The minimal complete example is `compute/src/solvers/example.rs`
(`example-dfs`, a row-major DFS); the test
`example_solver_counts_exactly_like_hp_dfs` checks that it counts exactly
like the main engine, and `plug_in_entry_runs_through_the_harness` runs an
externally defined entry through the harness. Run a registered solver on the
same instances and caps (from the hard-ladder directory, or set
`HARD_LADDER_ROOT`):

```
cargo run --release -- run --solver my-solver --role test --ladder primary
python3 ../hard-ladder/analysis/analyze.py compare results/test/primary/my-solver.jsonl \
    ../hard-ladder/results/test/primary/scb-gacolor_ac3_random.jsonl
```

The harness verifies every returned board (pieces used once, grey rim, all
seams match); the planted board is never passed to a solver. For rungs 9 and
10 (A2) also run the baseline through the same binary:
`run --solver scb-gacolor_ac3_random --sizes 9,10`.

## Reproduce

```
just experiments hard-ladder          # calibrate + test 6..8 + analyse (about 25 min, 8 vCPU)
just experiments hard-ladder check    # build, tests, instance check (minutes)
```

or `research/experiments/hard-ladder/run.sh [all|check|calibrate|test|analyze]`.
Node counts are deterministic given the seeds; wall clock is not.

Hardware and wall clock: Devin VM, 8 vCPU, 31 GiB RAM, rustc 1.99 (cargo
1.99.1), 8 runs in parallel. Calibration 1220 s, test grid (6..8, two colour
sets, two solvers) 152 s. Wall-clock columns are per-run with 8 concurrent
runs.
