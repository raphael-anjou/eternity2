# single-core-benchmark

Fifteen Eternity II solvers, each run once on ten corner-pinned variants of the
official puzzle, **single-threaded, 60 s per run**, every board re-scored by one
canonical scorer. The finding: **throughput does not equal score** — the beam
producer explores 150x fewer nodes/s than the Blackwood backtracker and still
finishes 20 points higher. The write-up is the research page
[`lab/experiments/raphael-anjou/single-core-benchmark`](https://eternity2.dev/research/lab/experiments/raphael-anjou/single-core-benchmark);
this directory is its runnable/committed backing.

## What is runnable here vs archived

The grid has two engine families. One is **ported in full and runs from this
repo**; the other lives in the private v2 vault and is represented by its
committed results only.

| family | ranks | where | reproducible here? |
|:--|:--|:--|:--|
| native (naive + CSP presets) | 5–15 | `engine/` (lifted from v2, self-contained) | **yes** — build & run below |
| standalone strong engines (`producer`, `blackwood`, `verhaard`, `alns`) | 1–4 | private v2 vault | no — committed `results/` are the published numbers; invocations archived under `scripts/archive-standalone/` |

## Layout

```
single-core-benchmark/
├── engine/              standalone cargo workspace: the native engine family,
│   ├── Cargo.toml       lifted verbatim from v2/crates (11 crates, serde-only)
│   └── crates/          run_algo (puzzle JSON in → bucas .url out) + gen_variants
├── variants/            the 10 corner-pinned puzzles: variant_NN.json (native)
│                        + variant_NN.csv (standalone) + manifest.json
├── results/             results.jsonl (150 runs), results.csv, run_meta.json,
│   └── urls/            REPORT.md, and one bucas .url per run
└── scripts/
    ├── run_grid.py      the grid runner (native family runs here)
    ├── make_report.py   results.jsonl → REPORT.md + CSV + ranking
    └── archive-standalone/   vault wrappers for the 4 private engines (provenance)
```

## Reproduce the native family

```bash
# from this directory
cargo build --release --bin run_algo --manifest-path engine/Cargo.toml

# one algorithm, one variant (see the full registry with `--list`)
engine/target/release/run_algo \
  --puzzle variants/variant_00.json --algo border_first_lcv \
  --seed 1 --budget-ms 60000 --emit /tmp/board.url

# the whole native grid (11 algos × 10 variants) + report
python3 scripts/run_grid.py --variants variants --out results/rerun \
  --budget-s 60 --seed 1 --parallel 6
python3 scripts/make_report.py --run results/rerun
```

`run_algo --list` prints the native registry. The scores are deterministic per
`--seed`; a longer `--budget-ms` only ever raises a score (the search keeps the
best board found so far), so short runs reproduce the shape, full 60 s runs
reproduce the committed numbers.

## Methodology

- **Corner-fixing.** Each variant is the official puzzle plus 3 pinned corner
  cells, so all ten share the 256-piece set + 5 clue hints but differ in 3 corner
  constraints. Emitted as both site-schema JSON (native) and CSV (standalone)
  from one generator — identical instances for every algorithm.
- **One scorer.** Every run emits a bucas `.url`; the score is the canonical
  matched-edge count (max 480), never the engine's self-report.
- **Single core.** `RAYON_NUM_THREADS=1`; the native presets are single-core by
  construction.

The four standalone engines' calibration (how each was pinned to one core and
~60 s) is documented in `scripts/archive-standalone/CALIBRATION.md`.
