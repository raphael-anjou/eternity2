# Set up the Eternity II starter kit

You are helping a developer start building an Eternity II solver using the
**starter kit** in the [eternity2 repo](https://github.com/raphael-anjou/eternity2),
at `research/starter-kit/`. Follow these steps, and **ask the user questions
whenever a choice is theirs to make**.

## 1. Get the code in place

- If you are already inside the `eternity2` repo, `cd research/starter-kit`.
- Otherwise, clone it and enter the kit:
  ```bash
  git clone https://github.com/raphael-anjou/eternity2
  cd eternity2/research/starter-kit
  ```
- The kit depends on `../experiments/common/crates/{e2-core,e2-io}` by relative
  path, so it must stay inside the repo (do not copy the folder out on its own).

## 2. Build and verify

```bash
cargo build --release
cargo test --release      # 13 tests must pass (9 integration + 4 unit)
```

Rust stable 1.85+ is the only prerequisite. If the build fails, the most likely
cause is a missing sibling `experiments/` directory — confirm the repo is intact.

## 3. Read the reference

Read `AGENTS.md` in this folder. It has the anti-duplication rule (scoring,
generation, and formats come from `e2-core`/`e2-io` — never re-implement them),
the exact commands, and the scientific-rigor rules that matter on this puzzle.

## 4. Show the user it works

Run one of each so they see the plumbing live, and read the output back to them:

```bash
cargo run --release --example score       # scores a generated solved board (480)
cargo run --release --example generate    # generates one official-shaped board
cargo run --release --example my_solver   # runs the baseline solver once
cargo run --release --example verify -- "<board URL>"   # verify against the official set
```

## 5. Help them write their solver

The single extension point is the `Solver` trait in `src/solver.rs`. To start:

1. Copy `examples/my_solver.rs` (a complete greedy baseline) to a new example.
2. Ask the user **what approach they want** — backtracking DFS, constraint
   propagation, local search / repair, something learned? Don't assume; the
   [approaches map](https://eternity2.dev/research/build/approaches-map) surveys
   the options. Confirm before writing a lot of code.
3. Replace the body of `solve`. Keep it single-core. Never score inside the
   solver — the kit re-scores canonically.

## 6. Set up their iteration loop

Once they have a solver, show them the loop that makes iterating fast:

```bash
cargo run --release --example sweep -- --n 40 --budget 3
cargo run --release --bin e2kit -- compare runs/<A> runs/<B>
```

Explain the one rule that saves a week: this puzzle's score has a large
seed-to-seed spread (sd ~12–20), so they must sweep **≥ 40 seeds** and report the
standard deviation before believing any improvement. The `compare` tool enforces
this.

## Things to get right

- A **480/480 solution is known to exist**; only its location is open. Never say
  otherwise.
- Don't invent record numbers — cite the site's
  [records page](https://eternity2.dev/research/records).
- If you want a scorer, parser, or generator, it already exists in the kit — look
  at `src/lib.rs`'s re-exports before writing anything.

Ask the user what they'd like to build, and go from there.
