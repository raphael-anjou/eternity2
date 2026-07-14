# Single-core Eternity II solver benchmark

**Grid:** 15 algorithms x 10 corner-pinned variants (**one run per puzzle**), **60s each, single core**, 6 in parallel.

**Diversity axis = the puzzle.** Each algorithm solves the SAME 10 instances (official E2 + 3 pinned corners, distinct corner arrangements). The score spread across the 10 variants is the reported distribution.

**Scoring:** every run emits a bucas `.url`; score is the canonical matched-edge count (max 480), the single source of truth (engines' self-reports are never trusted).

**Repro:** git `a360f07`, seed 1.


## Ranking (across the 10 corner variants)

Throughput (`nps`) is the engine's NATIVE unit -- search-nodes/s (DFS/CSP + backtrackers), beam-nodes/s (producer), or iters/s (ALNS). **These units are NOT comparable across families**; nps ranks throughput *within* a family only.

| rank | algorithm | mean | best | worst | median | std | n | fails | nps (native unit) |
|---:|:--|---:|---:|---:|---:|---:|---:|---:|:--|
| 1 | `producer` | 454.8 | **456** | 453 | 455.0 | 1.0 | 10 | 0 | 273K beam-nodes/s |
| 2 | `alns` | 452.9 | **455** | 451 | 453.0 | 1.1 | 10 | 0 | - |
| 3 | `verhaard` | 440.8 | **451** | 437 | 438.5 | 5.2 | 10 | 0 | 24.4M search-nodes/s |
| 4 | `blackwood` | 436.4 | **440** | 431 | 436.5 | 2.9 | 10 | 0 | 40.6M search-nodes/s |
| 5 | `naive_rowmajor` | 365.6 | **380** | 359 | 363.0 | 7.3 | 10 | 0 | 274K search-nodes/s |
| 6 | `border_first_lcv` | 182.7 | **349** | 54 | 114.5 | 135.7 | 10 | 0 | 279K search-nodes/s |
| 7 | `rare_color_first` | 182.7 | **349** | 54 | 114.5 | 135.7 | 10 | 0 | 281K search-nodes/s |
| 8 | `joe_depth150` | 168.6 | **249** | 53 | 243.5 | 93.6 | 10 | 0 | 5K search-nodes/s |
| 9 | `joe_depth150_bp` | 168.6 | **249** | 53 | 243.5 | 93.6 | 10 | 0 | 5K search-nodes/s |
| 10 | `border_first_full` | 124.2 | **344** | 55 | 55.0 | 113.0 | 10 | 0 | 11K search-nodes/s |
| 11 | `gacolor_ac3_random` | 103.1 | **160** | 65 | 94.5 | 30.1 | 10 | 0 | 8K search-nodes/s |
| 12 | `gacolor_ac3` | 76 | **96** | 56 | 70.0 | 14.1 | 10 | 0 | 9K search-nodes/s |
| 13 | `gacolor_ac3_ns1` | 76 | **96** | 56 | 70.0 | 14.1 | 10 | 0 | 9K search-nodes/s |
| 14 | `verhaard_preferred` | 76 | **96** | 56 | 70.0 | 14.1 | 10 | 0 | 8K search-nodes/s |
| 15 | `border_first_random` | 39.2 | **43** | 35 | 38.5 | 2.6 | 10 | 0 | 277K search-nodes/s |

## Score per variant (one 60s run each)

| algorithm | v00 | v01 | v02 | v03 | v04 | v05 | v06 | v07 | v08 | v09 | mean |
|:--|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `producer` | 456 | 453 | 456 | 454 | 454 | 455 | 454 | 456 | 455 | 455 | **454.8** |
| `alns` | 453 | 452 | 453 | 451 | 452 | 454 | 455 | 452 | 454 | 453 | **452.9** |
| `verhaard` | 437 | 438 | 439 | 451 | 441 | 437 | 439 | 451 | 438 | 437 | **440.8** |
| `blackwood` | 440 | 440 | 431 | 437 | 440 | 433 | 437 | 434 | 436 | 436 | **436.4** |
| `naive_rowmajor` | 374 | 359 | 364 | 366 | 380 | 359 | 359 | 374 | 359 | 362 | **365.6** |
| `border_first_lcv` | 346 | 349 | 68 | 341 | 55 | 55 | 55 | 54 | 343 | 161 | **182.7** |
| `rare_color_first` | 346 | 349 | 68 | 341 | 55 | 55 | 55 | 54 | 343 | 161 | **182.7** |
| `joe_depth150` | 245 | 249 | 244 | 245 | 53 | 55 | 55 | 53 | 244 | 243 | **168.6** |
| `joe_depth150_bp` | 245 | 249 | 244 | 245 | 53 | 55 | 55 | 53 | 244 | 243 | **168.6** |
| `border_first_full` | 55 | 344 | 68 | 339 | 55 | 55 | 55 | 55 | 161 | 55 | **124.2** |
| `gacolor_ac3_random` | 127 | 160 | 91 | 96 | 129 | 65 | 68 | 129 | 93 | 73 | **103.1** |
| `gacolor_ac3` | 93 | 69 | 96 | 67 | 87 | 63 | 94 | 64 | 56 | 71 | **76.0** |
| `gacolor_ac3_ns1` | 93 | 69 | 96 | 67 | 87 | 63 | 94 | 64 | 56 | 71 | **76.0** |
| `verhaard_preferred` | 93 | 69 | 96 | 67 | 87 | 63 | 94 | 64 | 56 | 71 | **76.0** |
| `border_first_random` | 38 | 36 | 40 | 42 | 43 | 39 | 43 | 38 | 35 | 38 | **39.2** |

## Best board found per algorithm

| algorithm | best score | bucas url |
|:--|---:|:--|
| `producer` | 456 | `producer__v00.url` |
| `alns` | 455 | `alns__v06.url` |
| `verhaard` | 451 | `verhaard__v03.url` |
| `blackwood` | 440 | `blackwood__v00.url` |
| `naive_rowmajor` | 380 | `naive_rowmajor__v04.url` |
| `border_first_lcv` | 349 | `border_first_lcv__v01.url` |
| `rare_color_first` | 349 | `rare_color_first__v01.url` |
| `joe_depth150` | 249 | `joe_depth150__v01.url` |
| `joe_depth150_bp` | 249 | `joe_depth150_bp__v01.url` |
| `border_first_full` | 344 | `border_first_full__v01.url` |
| `gacolor_ac3_random` | 160 | `gacolor_ac3_random__v01.url` |
| `gacolor_ac3` | 96 | `gacolor_ac3__v02.url` |
| `gacolor_ac3_ns1` | 96 | `gacolor_ac3_ns1__v02.url` |
| `verhaard_preferred` | 96 | `verhaard_preferred__v02.url` |
| `border_first_random` | 43 | `border_first_random__v04.url` |

## Reproduction (native family)

```bash
cd research/experiments/single-core-benchmark
cargo build --release --bin run_algo --manifest-path engine/Cargo.toml
python3 scripts/run_grid.py --variants variants \
  --out results/rerun --budget-s 60 --seed 1 --parallel 6
python3 scripts/make_report.py --run results/rerun
```

The four standalone strong engines (`producer`, `blackwood`, `verhaard`, `alns`) run from the private v2 vault and are not reproducible from this repo; their committed rows above are the published result.

