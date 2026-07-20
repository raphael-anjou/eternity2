# The Eternity II starter kit

A ready-to-use Rust workspace for building your own [Eternity II](https://eternity2.dev)
solver or running your own research. Everything the plumbing needs is already
here — score a board, generate boards with real colour balance, batch-generate
with pinned clues, convert between every format, benchmark, and a full
**solve → sweep → compare** iteration loop. You write only the part that is your
idea: one `Solver`.

Nothing here re-implements scoring, generation, or formats. Those come from the
blog's shared crates ([`e2-core`](../experiments/common/crates/e2-core),
[`e2-io`](../experiments/common/crates/e2-io)) by path, so a score from the kit
is the *same* score the site and every other engine produce — the one canonical
rim-excluding matched-edge count.

> **New here?** Read the [why it's hard](https://eternity2.dev/research/why),
> [build a solver](https://eternity2.dev/research/build), and
> [formats](https://eternity2.dev/research/build/formats) pages first. A
> **480/480 solution is known to exist** (the organisers built the puzzle from
> one); only its location is open.

## Using a coding agent?

Paste this into Claude Code, Cursor, or any agent and it sets the kit up and
helps you write a solver:

```
Follow the instructions from
https://raw.githubusercontent.com/raphael-anjou/eternity2/main/research/starter-kit/PROMPT.md
and ask me questions as needed.
```

Or point your agent at [`AGENTS.md`](./AGENTS.md) directly.

## Get it

The kit is one folder in the [eternity2 repo](https://github.com/raphael-anjou/eternity2).
Clone the repo and work in place — the two path dependencies reach into
`../experiments/common/crates`, so keep that sibling directory alongside the kit.

```bash
git clone https://github.com/raphael-anjou/eternity2
cd eternity2/research/starter-kit
cargo build --release
```

You need Rust stable (1.85+). Nothing else.

## The 60-second tour

```bash
cargo run --release --example score                 # score a generated solved board (480)
cargo run --release --example generate              # one official 16×16 board
cargo run --release --example my_solver             # run the baseline solver once
cargo run --release --example sweep -- --n 40       # sweep it over 40 seeds
cargo run --release --bin e2kit -- --help           # the CLI: gen | compare | score | verify
```

---

# Cookbook

Every block below compiles and runs. `use e2_kit::*;` gives you the whole kit.

## Score any board

The one canonical scorer (`e2-core`'s `score_cells`), rim excluded. Works on an
`eternity2.dev` / `e2.bucas.name` URL or a bare `board_edges` blob.

```rust
use e2_kit::score_url;

let url = "https://e2.bucas.name/#board_w=16&board_h=16&board_edges=aaaa…";
if let Some(score) = score_url(url) {
    println!("score: {score}");   // matched interior edges; rim never counts
}
```

Run it: `cargo run --release --example score -- "<URL>"`.

## Generate a board with real colour balance

`framed = true` confines five border colours to the frame and balances the
interior colours, exactly like the real puzzle. Deterministic per
`(size, colors, seed)`.

```rust
use e2_kit::{generator, instance_from_generated};

let puzzle = generator::generate_framed(16, 22, /* seed */ 7, /* framed */ true);
let instance = instance_from_generated("my-board", &puzzle);
// `instance` carries the piece set; score, solve, or convert it from here.
```

Run it: `cargo run --release --example generate -- 16 22 7 framed`.

## Generate 100 boards, with clues pinned

The "give me a batch, different seeds, hints pinned" tool. Each board is written
as site-schema JSON (the same shape the benchmark's `variant_NN.json` uses), so
you can read any of them straight back.

```bash
# 100 boards, seeds 1..100, 5 solution cells pinned as clues, into a folder:
cargo run --release --example generate_batch -- --n 100 --pins 5 --framed --out boards

# or an explicit seed range, streamed as one JSON object per line:
cargo run --release --example generate_batch -- --seeds 1..50 --jsonl > boards.jsonl
```

Read one back in Rust:

```rust
use e2_kit::Instance;
let instance = Instance::from_site_json("boards/3.json").unwrap();
```

Pinned hints are guaranteed to be genuine solution cells — the kit's test suite
proves a pinned board still solves to 480.

## Convert between every format

One lossless hub: read a board in any format, print it in all the others
(`board_edges`, `board_pieces`, `e2pieces.txt`, `eternity2.dev` and legacy bucas
URLs, and — from a site-JSON instance — CSV). Same converters the site uses.

```bash
cargo run --release --example convert -- "<board URL>"      # edges in, every form out
cargo run --release --example convert -- boards/3.json      # full instance → CSV too
```

## The official puzzle, and verifying a board

The real 256-piece set and the five official clues ship with the kit as
`data/official.json`, in the site's canonical `Puzzle` format. Load the real
instance (through `e2-io`'s own loader, so the kit parses nothing itself):

```rust
use e2_kit::official_instance;

let puzzle = official_instance(true); // true = pin the five official clues
// score any candidate board against the true set, canonically:
let out = puzzle.match_board(&cells);
println!("{} / 480", out.score);
```

Before you claim anything, verify it — re-measure, never trust:

```bash
cargo run --release --example verify -- "<board URL>"          # score + official-set + clue check
cargo run --release --example verify -- "<URL A>" "<URL B>"    # diff two boards cell by cell
```

`verify` re-scores from the board's own edges, confirms every cell is a distinct
official piece, checks the five clues, and (for two boards) reports how many cells
differ. A record claim has to survive this.

## Benchmark the plumbing

The kit has no search engine — that's your part — so this measures what you build
*on*: generation and scoring throughput, single-core, native. Run with
`--release`.

```bash
cargo run --release --example bench
#   ~9700 boards/s generated   (official 16×16/22 framed)
#   ~3.1M scorings/s           (canonical scorer on a full board)
```

---

# The iteration loop

Built for *many* iterations. You change only your solver; the runner, the run
directories, and the compare tool never change.

## 1. Write a `Solver`

One trait, one method. Copy [`examples/my_solver.rs`](./examples/my_solver.rs)
(a complete, working greedy baseline) and replace the body of `solve`:

```rust
use e2_kit::{Board, Budget, Instance, SolveOutcome, Solver};

struct MySolver;

impl Solver for MySolver {
    fn name(&self) -> String { "my-solver".into() }

    fn solve(&mut self, instance: &Instance, start: &Board, budget: Budget) -> SolveOutcome {
        let mut board = start.clone();   // `start` already carries any pinned clues
        // ... your search here; poll budget.expired().
        SolveOutcome::complete(board)    // or ::improved / ::exhausted / ::bound
    }
}
```

You never score yourself — the kit re-scores every board canonically, so your
reported number is trustworthy by construction. Return the honest **outcome**:
`SolveOutcome::complete` only for a full board, `::improved` for a best-effort
partial, `::exhausted` if you proved nothing better exists, or `::bound` for an
upper bound (which the kit keeps out of the score statistics — never averaged as
a score). See `examples/bound.rs` for a worked bound.

## 2. Sweep it over a seed grid

```bash
cargo run --release --example sweep -- --n 40 --budget 3
```

This generates one board per seed, runs your solver single-core with a time
budget, re-scores, and writes a **reproducible run directory** under `runs/`:

```
runs/my-solver-<cfg>-<stamp>/
  config.json      # exact parameters — re-running reproduces the scores
  results.jsonl    # one row per seed: score, breaks, elapsed, board URL
  summary.json     # n, mean, median, best, standard deviation
```

## 3. Compare two runs

Change your solver, sweep again, then ask "did it actually help?":

```bash
cargo run --release --bin e2kit -- compare runs/<A> runs/<B>
# or the Python twin:
python3 scripts/compare.py runs/<A> runs/<B>
```

It pairs cells by seed and reports the mean delta **with its spread**, and tells
you when a difference is too small to be real.

## The one rule that will save you a week

On this puzzle the score varies a lot from seed to seed — the standard deviation
across seeds is roughly **12–20 points** for a whole-board solver. So a one- or
two-point difference in mean between two runs is almost always **noise**. Sweep
at least **40 seeds** before believing any improvement, always report the
standard deviation, and keep everything **single-core** (the site's benchmarks
are single-core, so your numbers stay comparable). The `compare` tool enforces
this: it refuses to call a sub-noise difference a win.

---

## Layout

| Path | What it is |
| --- | --- |
| `src/solver.rs` | the `Solver` trait, `Budget`, and `SolveOutcome` — **your extension point** |
| `src/runner.rs` | the sweep runner (`sweep`, `SweepConfig`) |
| `src/run.rs` | run-directory types (`RunConfig`, `CellResult`, `Summary`) |
| `src/lib.rs` | re-exports + helpers (`instance_from_generated`, `pin_solution_hints`, `score_url`, `official_instance`) |
| `data/official.json` | the real 256-piece official set + clues (site `Puzzle` schema) |
| `examples/my_solver.rs` | **copy this** — a worked baseline solver |
| `examples/sweep.rs` | run a solver across a seed grid |
| `examples/bound.rs` | a solver that returns an upper *bound*, not a board |
| `examples/verify.rs` | re-score + official-set check + two-board diff |
| `examples/{score,generate,generate_batch,convert,bench}.rs` | the cookbook, runnable |
| `bin/e2kit.rs` | CLI: `gen`, `compare`, `score`, `verify` |
| `scripts/compare.py` | the compare tool, in Python |
| `tests/kit.rs` | correctness guards (scoring, pinned hints, official set, outcome type) |

## Testing your own work

```bash
cargo test --release     # the kit's own correctness tests
cargo clippy --release --all-targets
```

## Licence

MIT OR Apache-2.0, same as the rest of the repo. Build whatever you want on it.
