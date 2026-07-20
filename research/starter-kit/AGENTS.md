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
2. Replace the body of `solve(&mut self, instance, start, budget) -> SolveOutcome`.
   `start` is the board to continue from — `instance.seed_board()` by default (so
   pinned clues are in place), but any partial board can be handed in for
   seed-and-grow, band/column, or repair-a-known-board work. Poll
   `budget.expired()` and return the best board found.
3. Return the right **outcome**, because it is what stops a proven result being
   read as an ordinary score:
   - `SolveOutcome::complete(board)` — the search finished / filled what it could.
   - `SolveOutcome::improved(board)` — better than the start, stopped on budget.
   - `SolveOutcome::exhausted(board)` — a (sub-)space held nothing better; the
     *fact of exhaustion* is the result (LEDGER / tail-enumeration).
   - `SolveOutcome::bound(board, value, BoundKind::{LpUb,MipUb,GreedyRelaxed})` —
     the result is an **upper bound**, not a placed board. Never report a bound as
     a score; the type keeps them separate, so use it.
   Add `.with_nodes(n)` to record search work.
4. Never score inside the solver. The kit re-scores every board canonically via
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
| Verify / diff a board | `cargo run --release --example verify -- "<URL>" ["<URL B>"]` |

## The official puzzle

The real 256-piece set and the five official clues ship with the kit as
`data/official.json`, in the site's canonical `Puzzle`/`SiteInstance` format (see
`data/README.md`). Load the real instance with
`e2_kit::official_instance(with_clues)` — it goes through `e2-io`'s own
`SiteInstance` loader, so the kit does no parsing of its own. This is the board a
record attempt or a re-measurement runs on, not a generated seed. `verify` checks
a board against it: score, all-official-pieces, and clue compliance.

## The anti-duplication rule (important)

Scoring, generation, formats, and the official set live in `e2-core` / `e2-io` /
the kit's data. Route through them:

- **Score** → `e2_kit::score_url`, or `Instance::finish` / `e2-core::score_cells`.
  Never write a scorer. The canonical convention: matched interior edges, rim
  (colour 0) excluded. A solved 16×16 scores **480**.
- **Generate** → `e2_kit::generator` (`generate_framed` etc.). Real colour
  balance and determinism are already handled.
- **Official pieces / clues** → `e2_kit::official_instance(..)`. Don't re-embed or
  re-transcribe the piece set; it's shipped as `data/official.json` and
  test-locked to Eternity II's border census (4/56/196) and the five clue cells.
- **Formats** → `e2-io::format` and `Instance` (`from_site_json`, `to_csv`,
  `to_site`). Board-in-a-URL, CSV, `e2pieces.txt`, `board_edges` all exist. Never
  hand-roll a parser or a bucas encoder.

If you think you need a new scorer/parser/generator/piece set, you don't — find it
in `src/lib.rs`'s re-exports first.

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
