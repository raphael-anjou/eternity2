# single-core-benchmark

Fifteen Eternity II solvers, each run once on ten corner-pinned variants of the
official puzzle, **single-threaded, 60 s per run**, every board re-scored by one
canonical scorer. The finding: **throughput does not equal score** — the beam
producer explores 150x fewer nodes/s than the Blackwood backtracker and still
finishes 20 points higher. The write-up is the research page
[`lab/experiments/raphael-anjou/single-core-benchmark`](https://eternity2.dev/research/lab/experiments/raphael-anjou/single-core-benchmark);
this directory is its runnable/committed backing.

## What is runnable here vs archived

Every algorithm in the grid is **open source and runs from this repo.** All 15
solvers were lifted from the v2 vault into `engine/` (a single cargo workspace)
and are driven by the grid runner. There are two families by interface only:

| family | ranks | binary | variant input |
|:--|:--|:--|:--|
| native (naive + CSP presets) | 5–15 | `run_algo` (one binary, `--algo NAME`) | JSON |
| standalone strong engines (`producer`, `blackwood`, `verhaard`, `alns`) | 1–4 | one binary each, driven by `scripts/run_standalone.sh` | CSV |

Both speak the same puzzle-in / bucas-`.url`-out contract, and every board is
re-scored by the one canonical scorer (`verify_bucas`).

## Layout

```
single-core-benchmark/
├── engine/              one cargo workspace, all 15 solvers (17 crates):
│   ├── Cargo.toml       lifted verbatim from v2/crates
│   └── crates/          run_algo (native family) + the 4 strong engine bins
│                        (producer/blackwood/verhaard/alns) + verify_bucas +
│                        gen_variants
├── variants/            the 10 corner-pinned puzzles: variant_NN.json (native)
│                        + variant_NN.csv (strong engines) + manifest.json
├── results/             results.jsonl (150 runs), results.csv, run_meta.json,
│   └── urls/            REPORT.md, and one bucas .url per run
└── scripts/
    ├── run_grid.py      the grid runner (drives all 15)
    ├── run_standalone.sh   wrapper that drives the 4 strong-engine binaries
    ├── make_report.py   results.jsonl → REPORT.md + CSV + ranking
    └── archive-standalone/   the original vault wrappers (provenance)
```

## Reproduce

```bash
# from this directory: build every binary (native + the 4 strong engines)
cargo build --release --manifest-path engine/Cargo.toml

# one native algorithm, one variant (see the full registry with `--list`)
engine/target/release/run_algo \
  --puzzle variants/variant_00.json --algo border_first_lcv \
  --seed 1 --budget-ms 60000 --emit /tmp/board.url

# the whole grid (15 algos × 10 variants) + report
python3 scripts/run_grid.py --variants variants --out results/rerun \
  --budget-s 60 --seed 1 --parallel 6
python3 scripts/make_report.py --run results/rerun
```

`run_algo --list` prints the native registry; the strong engines are `producer`,
`blackwood`, `verhaard`, `alns`. Native scores are deterministic per `--seed`,
and a longer `--budget-ms` only ever raises a score (the search keeps the best
board so far), so short runs reproduce the shape and full 60 s runs reproduce the
committed numbers. The producer needs its full beam (~20 s) to reach 456; a
sub-20 s budget cuts it off and it reports 0, so give the strong engines the full
60 s.

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
