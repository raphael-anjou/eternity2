# bandsaw

**BANDSAW is an exact analysis tool, not a solver.** It does not search for a
high-scoring board. It takes a board that already exists and *proves* the best
possible completion of its endgame band (the bottom four rows), by meeting in
the middle: enumerate the top halves and the bottom halves of the band
separately, then join them, so the true optimum falls out without searching the
whole band at once.

The output is a **bracket** `[lb, ub]` on the band's minimum mismatches, plus a
certified lower bound, not a board. It answers "given everything above row 12,
how good can the endgame get, provably?" That makes it a measurement of how much
score is still reachable in an endgame, and a way to prove a board's bottom is
already optimal.

The write-up is the research page
[`lab/experiments/raphael-anjou/analyses/bandsaw`](https://eternity2.dev/research/lab/experiments/raphael-anjou/analyses/bandsaw);
this directory is its runnable backing.

## Run it

BANDSAW runs on the shared benchmark engine (it reuses the same puzzle catalog
and loaders), so build from there:

```bash
# from research/experiments/, or via `just experiments bandsaw`
cargo build --release \
  --manifest-path single-core-benchmark/engine/Cargo.toml \
  --bin mini_e2 --bin dump_board_json

# a worked example: prove the endgame band of a committed official-puzzle board.
# 1. decode a board to placement JSON (any board on the official 16x16 catalog):
single-core-benchmark/engine/target/release/dump_board_json \
  "$(cat bandsaw/example/board.url)" > /tmp/entry_raw.json
python3 bandsaw/scripts/to_entry.py /tmp/entry_raw.json > /tmp/entry.json

# 2. run BANDSAW on its endgame band:
single-core-benchmark/engine/target/release/mini_e2 \
  --entry /tmp/entry.json --rung-ms 15000 --label-ms 1500
```

It prints the relax floor, the certified lower bound, a greedy upper-bound
label, and the bracket, e.g.:

```
{"entry":"...","relax_floor":10,"certified_lb":10,"greedy_label":51,"bracket":"[10,51]"}
```

`just experiments bandsaw` runs exactly this worked example and writes the result
to `results/`.

## What is deterministic

The **certified lower bound** and the **relax floor** are exact and reproduce
byte-for-byte on any machine for a given board and band. The **greedy label**
(the upper bound) is a time-bounded search (`--label-ms`), so its exact value
varies with the wall-clock budget and the host; only the certified half of the
bracket is a proof. Give it a longer `--rung-ms` to tighten the certified bound.

## Layout

```
bandsaw/
├── README.md
├── example/
│   └── board.url        a committed official-puzzle board (the worked example's input)
├── scripts/
│   └── to_entry.py      dump_board_json output -> mini_e2 --entry placement JSON
└── results/
    └── bracket.json     the committed bracket from the worked example
```

The engine code lives in the shared benchmark workspace
(`single-core-benchmark/engine/`): the band solver is
`crates/bench-audit/src/mini.rs`, driven by the `mini_e2` binary. It depends only
on the puzzle core and the deterministic generator, no external solver.
