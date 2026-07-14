# Standalone engine calibration — vol-235 benchmark grid

Machine: 8-core Apple Silicon (`hw.ncpu=8`), macOS (Darwin 25.5.0). All
measurements below are single runs of `crates/bench-grid/wrappers/run_standalone.sh`
functions against `output/vol-235/variants_20260713T221027/variant_00.csv`
(official 16x16 E2 puzzle + 3 pinned corners, 8 hints total; one of the 10
corner-pinned benchmark variants). `verify_bucas` is the sole scoring
oracle — every score below is the independently-verified `canonical_score`,
not the engine's self-reported number (the two agreed in every run tested,
but only the verify_bucas number is trusted per repo convention).

Reproduce any row below with:

```bash
cd /Users/raphaelanjou/Documents/dev-projects/polytech/eternity2/v2
source crates/bench-grid/wrappers/run_standalone.sh
run_<engine> output/vol-235/variants_20260713T221027/variant_00.csv <seed> 60 /tmp/out.url
```

(must be run with `bash`, not `zsh` — the script uses `BASH_SOURCE`; `run_grid.py`
already invokes it via `bash -c`.)

## Single-core audit summary

| Engine | Threading mechanism | Verdict |
|---|---|---|
| `vol232_w1_producer_trie` | none — no `rayon`/`thread::spawn` anywhere in the binary (grep-verified) | inherently single-core |
| `blackwood_bt` | spawns exactly ONE `std::thread::Builder` worker (256MB stack, for the 256-deep recursive DFS) per restart, `.join()`s it synchronously before the next restart | single-core in practice (2 OS threads total, only 1 ever runs) |
| `verhaard_faithful_v2` | same pattern as `blackwood_bt` (same author lineage) | single-core in practice |
| `alns_only` | `run_alns` (single-chain entry point, what this wrapper calls) has zero rayon calls on the default `--repair-kind sa` path. Rayon exists in the crate only for `run_alns_portfolio`/`run_alns_pt` (unused here) and `RepairKind::Cp` (unused here — we pass no `--repair-kind`, default is `sa`) | single-core on the path used here |

All four were also empirically checked with `/usr/bin/time -l` (`user` time
≈ `real` time, 96–99% CPU) and with `ps -M <pid>` (2 OS threads max, i.e. at
most one ever runnable). `RAYON_NUM_THREADS=1` is exported by the wrapper
script defensively regardless.

**None of the 4 binaries are natively time-bounded except `alns_only`**
(which takes `--alns-budget-ms` directly). The other 3 either run a fixed
workload to completion (`vol232_w1_producer_trie`'s beam search) or a
node-capped restart portfolio (`blackwood_bt`, `verhaard_faithful_v2`). The
wrapper makes all 4 behave as ~60s-wall-clock engines via calibrated fixed
parameters (producer) or `timeout` + a checkpoint file the binary itself
maintains (blackwood/verhaard), or true time-boxing (alns).

---

## 1. Producer (`vol232_w1_producer_trie`)

Beam search, `--order comb:14` (the vol-234 comb fill-order that beat plain
row-major — see memory: "comb broke the old 455 raw ceiling"). Not natively
time-bounded: the binary runs the given `--beam` to full completion, no
timer. Calibration = finding a beam that completes in ~60s.

### Beam <-> wall-time table (single seed, variant_00)

| `--beam` | wall (s) | canonical_score |
|---:|---:|---:|
| 4,096 | 2.7 | (used as sub-stage for ALNS chain, see below) |
| 16,384 | 8.9–9.4 | 453 |
| 65,536 | 51.8 | 453 |
| 76,000 | 55.9 | 454 |
| 80,000 | 53.6–58.5 | 454–456 |
| 82,000 | 55.3 | 454 |

**Chosen: `--beam 80000`** (fixed constant in `run_producer`). Wrapped in
`timeout $((budget_s + 15))` as a hang guard only (never fires under normal
conditions).

### Exact command

```bash
target/release/vol232_w1_producer_trie \
  output/vol-235/variants_20260713T221027/variant_00.csv \
  --order comb:14 --beam 80000 --tol 0 --seed 1 \
  --emit-best /tmp/producer_s1.url
target/release/verify_bucas output/vol-235/variants_20260713T221027/variant_00.csv "$(cat /tmp/producer_s1.url)"
```

### Measured (via `run_producer`, budget_s=60)

| seed | wall (s) | CPU% | canonical_score | nodes (layers×beam) | nps (beam-nodes/s) |
|---:|---:|---:|---:|---:|---:|
| 1 | 56.2 | 96% | **456/480** | 19,840,000 | 353,213 |
| 2 | 53.6 | 98% | **454/480** | 19,840,000 | 370,357 |

Node accounting note: the beam producer has no native per-node counter. Its
search is exactly `layers` sequential frontier-expansion steps, each
expanding the surviving `beam_width` parents; `layers*beam_width` (both
printed on the engine's own CSV summary line) is reported as `nodes`, unit
`beam-nodes/s`. This is a beam-expansion count, not a search-tree-node count
— stated explicitly so it isn't confused with blackwood/verhaard's DFS node
counter.

---

## 2. Blackwood (`blackwood_bt`)

Depth-gated-break backtracker with a restart portfolio. Node-budget bounded
per restart (`--nodecap`), not time-bounded. `--restarts 1000` gives a large
portfolio; the wrapper wraps the whole thing in `timeout budget_s` and reads
the binary's own checkpoint file (`emit-dir/best_so_far.url`, refreshed on
every new-best) after the kill.

### Nodecap calibration

First tried `--nodecap 200000000` (200M): only 1–2 restarts complete in 60s,
so the binary's periodic `# progress` throughput line (emitted every 25
restarts) never fires before the `timeout` kill — no honest total-nodes
figure obtainable under a hard kill with this nodecap. Score was fine (432,
439) but node/nps reporting was blind.

Switched to **`--nodecap 20000000`** (20M): ~150–175 restarts complete in
60s at ~60 Mnode/s, so several `# progress` lines fire, giving an exact
binary-reported `nodes=` / `Mnode/s` figure right up to the last full
25-restart checkpoint before the kill. Score was equal-or-better (440 vs
432/439) because more restart diversity fits in the window. **Chosen for
the wrapper.**

### Exact command

```bash
timeout 60 target/release/blackwood_bt \
  output/vol-235/variants_20260713T221027/variant_00.csv \
  --nodecap 20000000 --restarts 1000 --seed 1 \
  --emit-dir /tmp/bw_scratch --emit-prefix bw
target/release/verify_bucas output/vol-235/variants_20260713T221027/variant_00.csv "$(cat /tmp/bw_scratch/best_so_far.url)"
```

### Measured (via `run_blackwood`, budget_s=60)

| seed | wall (s) | CPU% | canonical_score | nodes | nps (search-nodes/s) |
|---:|---:|---:|---:|---:|---:|
| 1 | 60.05 | 99% | **440/480** | 3,500,000,000 | 62,300,000 |
| 2 | 60.05 | 99% | **440/480** | 3,500,000,000 | 59,700,000 |

`timeout` cleanly SIGTERMs the process at budget_s; exit is non-zero
(expected — not treated as failure since the checkpoint file is what gets
banked). No orphaned processes observed after any run (`pgrep` checked).

---

## 3. Verhaard (`verhaard_faithful_v2`)

Same architecture as `blackwood_bt` (comb/border-first fill order + a
different depth-gated quota/slip schedule reconstructed from the Verhaard
"eii" binary). Same nodecap-calibration rationale and checkpoint pattern.
Default `--stage 3` already enables the full (quota + slip) engine — no
separate stage-4 exists distinct from stage 3 for this binary.

### Exact command

```bash
timeout 60 target/release/verhaard_faithful_v2 \
  output/vol-235/variants_20260713T221027/variant_00.csv \
  --nodecap 20000000 --restarts 1000 --seed 1 \
  --emit-dir /tmp/vh_scratch --emit-prefix vh
target/release/verify_bucas output/vol-235/variants_20260713T221027/variant_00.csv "$(cat /tmp/vh_scratch/best_so_far.url)"
```

### Measured (via `run_verhaard`, budget_s=60)

| seed | wall (s) | CPU% | canonical_score | nodes | nps (search-nodes/s) |
|---:|---:|---:|---:|---:|---:|
| 1 | 60.05 | 99% | **437/480** | 2,069,000,000 | 39,600,000 |
| 2 | 60.05 | 99% | **441/480** | 2,088,000,000 | 40,000,000 |

Verhaard's per-node throughput (~40 Mnode/s) is noticeably lower than
Blackwood's (~60 Mnode/s) at the same nodecap — consistent with Verhaard's
extra good-groups bookkeeping per placement (see
`vault/papers/vol-234/R8-VERHAARD-GOODGROUPS.md`).

---

## 4. ALNS (`alns_only`)

Time-bounded (`--alns-budget-ms`) but requires a **starting board** —
`alns_only` has no producer of its own. Two-stage chain inside the same
`budget_s` wall-clock window:

- **Stage A** — `vol232_w1_producer_trie`, fixed `--beam 16384`
  (`--order comb:14`), which completes in a MEASURED ~9s regardless of
  `budget_s` (it is not itself time-bounded; 16384 was chosen because it
  reliably lands well under any reasonable sub-budget while still landing
  in the "raw ~453-457" band the memory-recorded winning pipeline starts
  from — "comb14 B=16384 seed9 -> 457 raw -> ALNS -> 461").
- **Convert**: `dump_board_json <bucas-url>` (decodes the bucas URL back to
  a placement JSON) piped through `jq '{placement: .board}'` to match
  `alns_only`'s `load_cp_board` schema (`{"placement":[{piece_id,rotation}|
  {pos,piece_id,rotation}, ...]}` — sparse-with-`pos` form used here).
  ~1s overhead.
- **Stage B** — `alns_only --ops basic_lkh` (the memory-recorded winning
  preset: "ALNS `alns_only` ops=basic_lkh, repair=sa ... 457->459->460->461")
  for the ACTUAL remaining wall-clock: `budget_s - measured_stageA_wall -
  2s conversion overhead`, floor 5s. This measure-then-allocate approach
  (rather than a fixed a-priori split) avoids wasting budget when stage A
  finishes faster than any fixed allowance — an earlier version of this
  wrapper fixed a 22s producer sub-budget for a beam that actually finished
  in ~3s, silently discarding ~19s of the 60s window (found and fixed
  during calibration; see script comments).

### CRITICAL bugs found + fixed during calibration

1. **`alns_only`'s summary lines (`bucas: ...`, `ALNS: elapsed=... iters=...`)
   are `eprintln!`'d (stderr), not printed to stdout.** An early version of
   the wrapper grepped stdout and silently fell back to the stage-A board
   every single run (never actually picking up ALNS's output). Fixed by
   grepping the captured stderr log.
2. **`alns_only --puzzle` does NOT affect `load_cp_board`'s internal piece
   catalog load** — `load_cp_board` (used to parse `--cp-board`) hardcodes
   `../data/puzzles/size_16_official_eternity.csv` relative to CWD,
   independent of the `--puzzle` flag (which only affects the main
   scoring/ALNS puzzle load). This is safe here ONLY because piece
   ids/edges are identical across all 10 corner-pinned benchmark variants
   (only the 8 hint cells differ) — verified by checking the file actually
   exists at that relative path. **The wrapper explicitly `cd`s to
   `$V2_ROOT` before invoking `alns_only` and `dump_board_json`** (both
   binaries share this hardcoded-relative-path pattern) so `../data/...`
   resolves correctly regardless of the caller's cwd. This is a real
   pre-existing quirk in `alns_only.rs`/`dump_board_json.rs`, documented
   here rather than silently patched around, per repo convention (flagged
   for a future fix: the hardcoded path should honor `--puzzle` or take its
   own `--catalog-puzzle` flag).
3. **Non-English locale (`fr_FR.UTF-8` on this machine) made `awk`
   printf a comma decimal separator** (`58,48` instead of `58.48`),
   breaking every downstream numeric parse. Fixed with `export LC_ALL=C;
   export LC_NUMERIC=C` at the top of the wrapper script.

### Exact command (manual, 2-stage)

```bash
cd /Users/raphaelanjou/Documents/dev-projects/polytech/eternity2/v2   # cwd matters, see bug #2
target/release/vol232_w1_producer_trie \
  output/vol-235/variants_20260713T221027/variant_00.csv \
  --order comb:14 --beam 16384 --tol 0 --seed 1 --emit-best /tmp/base.url
target/release/dump_board_json "$(cat /tmp/base.url)" | jq '{placement: .board}' > /tmp/base_cpboard.json
target/release/alns_only \
  --cp-board /tmp/base_cpboard.json \
  --puzzle output/vol-235/variants_20260713T221027/variant_00.csv \
  --alns-budget-ms 48000 --seed 1 --ops basic_lkh
# bucas url is on the "bucas: ..." STDERR line
```

### Measured (via `run_alns`, budget_s=60)

| seed | stage A wall | stage A score | stage B (ALNS) elapsed | ALNS iters | final canonical_score | total wall | CPU% |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.20s | 453 | 48.0s | 32 | **454/480** | 57.3s | 99% |
| 2 | 9.15s | 453 | 48.0s | 32 | **453/480** | 57.2s | 99% |

`nodes`/`nps` for this engine are reported in `iters`/`iters-per-second`
(alns_only has no "search node" concept — its unit of work is one
destroy+repair iteration): seed 1 → `nodes=32 nps=0.67 nps_unit=iters/s`;
seed 2 → the same (32 iters / 48s). Only ~0.67 iters/s at ~48s budget is
consistent with `basic_lkh`'s `lkh_chain` op being expensive per call (each
repair does an LKH-style chain reconstruction) — this matches the memory
note that the historic 461-record pipeline needed ~20 MINUTES of ALNS, not
60 seconds, to walk 457→461. **A 60s grid budget for `alns_only` samples
only ~30 iterations — expect it to often report FLAT (no improvement over
the stage-A base board) at this budget, which is an honest, correctly
time-boxed result, not a bug.** This is noted explicitly so the grid's
consumers don't misread "no gain in 60s" as "ALNS doesn't work."

---

## Summary: all 4 at budget_s=60, variant_00, 2 seeds each

| Engine | seed 1 score | seed 2 score | wall range | single-core |
|---|---:|---:|---|---|
| `run_producer` | 456 | 454 | 53.6–56.2s | yes |
| `run_blackwood` | 440 | 440 | 60.05s (timeout-killed) | yes |
| `run_verhaard` | 437 | 441 | 60.05s (timeout-killed) | yes |
| `run_alns` (2-stage) | 454 | 453 | 57.2–57.3s | yes |

No engine in this grid is disqualified for multi-core use — all 4 confirmed
single-core (see audit table above).
