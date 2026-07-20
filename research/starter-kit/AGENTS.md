# AGENTS.md — Eternity II starter kit

Guidance for coding agents working in this folder. This is the reference; for a
one-shot "set it up and help me build" flow, see `PROMPT.md`.

## What this is

A Rust workspace that gives someone everything they need to build an Eternity II
solver *except the solver*. The user writes one `Solver`; the kit does scoring,
board generation, batch generation, format conversion, benchmarking, and the
run/compare iteration loop. **Do not re-implement any of that** — it already
exists and is tested.

## Setup

```bash
cargo build --release
cargo test --release        # 4 correctness tests must pass
```

Requires Rust stable 1.85+. No other toolchain. The kit depends on
`../experiments/common/crates/{e2-core,e2-io}` by path, so it must stay a sibling
of `experiments/` inside the repo.

## Where the user's idea goes

**The single extension point is the `Solver` trait in `src/solver.rs`.** To add a
solver:

1. Copy `examples/my_solver.rs` to a new example (e.g. `examples/my_idea.rs`).
2. Replace the body of `solve`. Start from `instance.seed_board()` (this applies
   pinned hints). Poll `budget.expired()` in the inner loop and return the best
   board found. Return a partial board if out of time — it scores what it scores.
3. Never score inside the solver. The kit re-scores every board canonically via
   `Instance::finish`, so the reported number is trustworthy by construction.

Everything else — the sweep runner, run directories, the compare tool — works on
any `Solver` unchanged.

## The iteration loop (the point of the kit)

```bash
cargo run --release --example sweep -- --n 40 --budget 3   # sweep a solver
cargo run --release --bin e2kit -- compare runs/<A> runs/<B>
```

`sweep` writes `runs/<solver>-<cfg>-<stamp>/` with `config.json`,
`results.jsonl`, `summary.json`. Two runs of the same config produce identical
scores (seeds are deterministic). `compare` pairs by seed and reports the mean
delta with its standard error.

## Common tasks

| Task | Command |
| --- | --- |
| Score a board | `cargo run --release --example score -- "<URL>"` |
| Generate one board | `cargo run --release --example generate -- 16 22 <seed> framed` |
| Batch-generate with clues | `cargo run --release --example generate_batch -- --n 100 --pins 5 --framed --out boards` |
| Convert formats | `cargo run --release --example convert -- "<URL>"` |
| Benchmark plumbing | `cargo run --release --example bench` |

## The anti-duplication rule (important)

Scoring, generation, and formats live in `e2-core` / `e2-io`. Route through them:

- **Score** → `e2_kit::score_url`, or `Instance::finish` / `e2-core::score_cells`.
  Never write a scorer. The canonical convention: matched interior edges, rim
  (colour 0) excluded. A solved 16×16 scores **480**.
- **Generate** → `e2_kit::generator` (`generate_framed` etc.). Real colour
  balance and determinism are already handled.
- **Formats** → `e2-io::format` and `Instance` (`from_site_json`, `to_csv`,
  `to_site`). Board-in-a-URL, CSV, `e2pieces.txt`, `board_edges`/`board_pieces`
  all exist. Never hand-roll a parser or a bucas encoder.

If you think you need a new scorer/parser/generator, you don't — find it in
`src/lib.rs`'s re-exports first.

## Scientific rigor (do not skip)

This puzzle's score has a **large seed-to-seed spread** (standard deviation
~12–20 points for a whole-board solver). Therefore:

- **Sweep ≥ 40 seeds** before claiming any improvement. A 1–2 point mean
  difference across a handful of seeds is noise.
- **Always report the standard deviation**, not just the mean. `summary.json`
  has it; the `compare` tool prints it and refuses to call a sub-noise delta a
  win.
- **Keep everything single-core.** The site's benchmarks are single-core; match
  that so numbers stay comparable. Do not add threads/rayon to a sweep.
- **Never trust a solver's self-reported score** — the kit re-scores canonically
  on purpose.

## Facts to state correctly

- A **480/480 solution is known to exist** (the organisers built the puzzle from
  one). Never write "nobody knows if 480 exists". Only its location is open.
- The public record is **470/480**; the strict five-clue record is lower. Don't
  invent record numbers — cite the site's [records page](https://eternity2.dev/research/records).

## Style

Match the existing code: `#![forbid(unsafe_code)]`, small functions, exact
commands in docs, no new dependencies beyond `serde`/`serde_json` unless the user
asks. Run `cargo clippy --release --all-targets` before finishing; the kit is
warning-clean and should stay that way.
