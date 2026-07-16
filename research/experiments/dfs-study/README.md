# DFS study

A dedicated study of **depth-first backtrackers** for Eternity II. One question,
asked carefully: among depth-first backtrackers, what does each fill order, each
heuristic, and the break mechanism actually buy?

Every algorithm here is a **from-scratch** reimplementation — no community
source, no dependency on the sibling `single-core-benchmark` engine — so the
software engineering is clean and the "what stacks on what" story is explicit:
each variant is one declared change over a parent.

## Layout

```
../common/              shared central lib (also used by the repair study)
  crates/
    e2-core/            board, pieces, the one canonical scorer, search stats
    e2-io/              shared IO contract + lossless converters (site JSON, CSV,
                        bucas URL, hints) — every study algorithm speaks this
engine/                 this study's own Cargo workspace (depends on ../common)
  crates/
    dfs-engine/         the composable DFS + the variant registry
    dfs-codegen/        NAIVE-CODEGEN: a 16x16-specialised row-major hot loop
    dfs-run/            run_dfs, dfs-convert binaries
variants/               the ten corner-pinned instances (shared with the benchmark)
scripts/
  run_grid.py           (variant x 10 instances) -> results.jsonl
  make_report.py        results.jsonl -> report.md
  make_site_json.py     results.jsonl -> web/src/data/dfs-study.json
results/                committed run outputs (results.jsonl, report.md, community.*)
```

## Run it

```
just experiments dfs-study                 # build + full 60s grid + report + site json
# or directly:
cargo build --release --manifest-path engine/Cargo.toml
./engine/target/release/run_dfs --list                       # every variant
./engine/target/release/run_dfs --matrix-json                # the registry as JSON
./engine/target/release/run_dfs --puzzle variants/variant_00.json --algo break-1 --budget-s 60
./engine/target/release/run_dfs_codegen --puzzle variants/variant_00.json --budget-s 60
./engine/target/release/dfs-convert info variants/variant_00.json
cargo test --manifest-path engine/Cargo.toml                 # scorer parity, soundness, breaks
```

## The variant families

- **baseline** — NAIVE-CLEAN (rawest row-major DFS) and NAIVE-CODEGEN (the same
  algorithm, hand-specialised), to price low-level engineering.
- **path** — row-major, bottom-up, spiral-in/out, border-first, Verhaard comb.
- **heuristic** — MRV cell choice, rare-colour values, forward-checking, AC-3,
  per-colour supply. Fixed path, one change at a time.
- **break** — the elite axis: depth-gated edge breaks (Blackwood / Verhaard
  schedules), single vs double per cell. Score = 480 − #breaks.
- **community** — McGavin's C and Blackwood's C#, cited as reference points
  (they run their own hardcoded instance, not these variants).

## Stats raised

Every run reports score, node throughput (search-nodes/s, never compared across
families), max depth reached, depth at timeout, breaks, and backtracks. No
engine's self-reported score is trusted — every board is re-scored by the one
canonical scorer, and every board carries a bucas `.url` verifiable in `/viewer`.

See `results/report.md` for the per-family measurements, `results/community.md`
for the community-engine runs, and the site pages under
`web/content/research/lab/experiments/raphael-anjou/dfs-study/` for the write-up.
