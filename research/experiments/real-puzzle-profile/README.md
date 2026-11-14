# real-puzzle-profile

Two measurements on the official Eternity II piece set (16x16, 256 pieces,
five clues), night 2026-11-12, track 10, label measurement. Neither is a record
attempt and no result here is Eternity II progress: they measure where the
cost of search sits.

- **M1.** Node histogram by depth of a Blackwood-style break-tolerant DFS (a
  port of the repo's `blackwood_bt`), and the share of nodes after the first
  break unlock. Measured at 1e10 and 1e11 nodes for the `record` schedule of this
  port only, and flagged budget-dependent by the preregistered rule; see
  `RESULTS.md`.
- **M2.** Size of the exact (no break) row-scan search tree, estimated by
  population SMC, validated against exact counts, compared with the model
  price 10^40.49.

Read in this order: `PREREG.md` (frozen first), `PREREG_AMENDMENTS.md` (A1 to
A8, each with its time and reason), `RESULTS.md` (tables, every number with
the raw file it comes from).

## Layout

| Path | What |
|---|---|
| `engine/` | std-only Rust crate, binary `rpp` (`bw`, `smc`, `enum`, `gate-a`, `gate-gen`, `paint`, `bwsmc`, `bwval`) |
| `run.sh` | every stage, each command wrapped by `../common/prereg_guard.py` |
| `scripts/frames.py` | decodes the frame stand-ins from `research/topics/record-boards/boards.json` |
| `scripts/analyze.py` | reads every raw file, prints the tables, writes `results/summary.json` |
| `results/run-log.jsonl` | the guard's append-only log (start and end of every run) |
| `results/gate/` | equivalence check, M2 gates, M1b gate |
| `results/m1/` | one JSON line per M1 run (`<schedule>-s<seed>.jsonl`), M1b SMC rows (`m1b-*.jsonl`) |
| `results/m2/` | SMC repetitions (`<config>-np1e5/6.jsonl`), painted null, frame stand-ins |

## Reproduce

```
just experiments real-puzzle-profile build
BW_BIN=<path to blackwood_bt> just experiments real-puzzle-profile equiv
just experiments real-puzzle-profile gate-a     # also gate-b, gate-gen
just experiments real-puzzle-profile m1          # 24 runs of 1e10 nodes, about 3 min each
just experiments real-puzzle-profile m2          # about 4 min
just experiments real-puzzle-profile paint
python3 -I research/experiments/real-puzzle-profile/scripts/analyze.py
```

M1 runs are deterministic given the seed (node histograms reproduce exactly).
SMC rows are deterministic given the seed and binary.
