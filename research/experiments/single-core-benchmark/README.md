# single-core-benchmark

Seventeen Eternity II solvers, each run once on ten corner-pinned variants of the
official puzzle, **single-threaded, 60 s per run**, every board re-scored by one
canonical scorer. The finding: **throughput does not equal score** — the beam
producer explores 150x fewer nodes/s than the Blackwood backtracker and still
finishes 20 points higher. The write-up is the research page
[`lab/experiments/single-core-benchmark`](https://eternity2.dev/research/lab/experiments/single-core-benchmark);
this directory is its runnable/committed backing.

On the published page, Raphaël's engines carry an `anjou-` name prefix and the two
community backtrackers (blackwood, verhaard) stay bare; that prefix is applied by
`scripts/make_site_json.py` and is **site-side only** — the engine, the registry
(`run_algo --list`), and `results/` use the canonical unprefixed names below.

## Why single-core

Single-core is a **choice this experiment enforces**, not a limitation of the
engines. It makes the comparison fair: every solver gets one core and 60 s, so a
row's score reflects its search, not how many threads its author wired up.

It used to be both. When these results were measured the four strong engines
genuinely had no threading — the wrapper's own notes said the beam producer was
*"inherently single-core"*. That is no longer true: all four now accept
`--threads N` (`--chains N` for `alns`) and scale to many cores. So
`scripts/run_standalone.sh` now passes `--threads 1` **explicitly** rather than
relying on a default, and the published numbers in `results/` are unchanged and
still reproducible — `--threads 1` is bit-identical to the pre-threading
binaries, verified down to the emitted board URLs, not just the scores.

A multi-core grid — same 60 s, more cores — is a separate experiment, not a
re-run of this one, and is future work.

## What is runnable here vs archived

Every algorithm in the grid is **open source and runs from this repo.** All 17
solvers were lifted from the v2 vault into `engine/` (a single cargo workspace)
and are driven by the grid runner. There are two families by interface only:

| family | ranks | binary | variant input |
|:--|:--|:--|:--|
| native (naive + CSP presets) | 5–17 | `run_algo` (one binary, `--algo NAME`) | JSON |
| standalone strong engines (`producer`, `blackwood`, `verhaard`, `alns`) | 1–4 | one binary each, driven by `scripts/run_standalone.sh` | CSV |

Both speak the same puzzle-in / bucas-`.url`-out contract, and every board is
re-scored by the one canonical scorer (`verify_bucas`).

## Layout

```
single-core-benchmark/
├── engine/              one cargo workspace, all 17 solvers (17 crates):
│   ├── Cargo.toml       lifted verbatim from v2/crates
│   └── crates/          run_algo (native family) + the 4 strong engine bins
│                        (producer/blackwood/verhaard/alns) + verify_bucas +
│                        gen_variants
├── variants/            the 10 corner-pinned puzzles: variant_NN.json (native)
│                        + variant_NN.csv (strong engines) + manifest.json
├── results/             results.jsonl (170 runs), results.csv, run_meta.json,
│   └── urls/            REPORT.md, and one bucas .url per run
└── scripts/
    ├── run_grid.py      the grid runner (reads solvers.toml)
    ├── run_standalone.sh   wrapper that drives the 4 strong-engine binaries
    ├── make_report.py   results.jsonl → REPORT.md + CSV + ranking
    ├── make_site_json.py   results.jsonl → web site JSON (reads solvers.toml)
    ├── manifest.py      shared solvers.toml loader (one parser for both scripts)
    └── archive-standalone/   the original vault wrappers (provenance)

solvers.toml (at the experiment root) is the SINGLE SOURCE OF TRUTH for the grid:
one [[solver]] per row with its name, author, family, I/O format, native unit and
calibration. run_grid.py and make_site_json.py both read it, so there is no
hand-kept algo/family/unit table to drift (that drift is what once silently
dropped two solvers). Adding a solver to the grid = one [[solver]] entry + its
runnable engine (a native module in the bench-grid registry, or a standalone
binary with a run_<name> wrapper). run_grid.py cross-checks the native entries
against `run_algo --list` and errors on any mismatch.
```

## Reproduce

```bash
# from this directory: build every binary (native + the 4 strong engines)
cargo build --release --manifest-path engine/Cargo.toml

# one native algorithm, one variant (see the full registry with `--list`)
engine/target/release/run_algo \
  --puzzle variants/variant_00.json --algo border_first_lcv \
  --seed 1 --budget-ms 60000 --emit /tmp/board.url

# the whole grid (17 algos × 10 variants) + report + site JSON
python3 scripts/run_grid.py --variants variants --out results/rerun \
  --budget-s 60 --seed 1 --parallel 6
python3 scripts/make_report.py --run results/rerun
python3 scripts/make_site_json.py --run results/rerun
```

The default build is self-contained and fetches nothing over the network. (The
learned value-ordering path in `solver-engine` — which pulls the `ort` ONNX
runtime and downloads a native library at build time — is behind the off-by-
default `learned-order` cargo feature; no published preset uses it. Build with
`--features learned-order` only if you want the learned model.)

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
