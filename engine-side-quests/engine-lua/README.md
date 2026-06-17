# Eternity II engine — pure Lua

A from-scratch port of the [Rust engine](../../engine) into **plain Lua**, with no
dependencies, no build step, and no toolchain. It is the *same* engine — the
same path-driven backtracking DFS, the same scorer, the same seeded generator,
the same official piece set, the same cell-visit orders — re-expressed in the
simplest embeddable language there is.

Why it exists: the Rust core is fast and correct, but Rust is a steep language
to pick up just to keep a puzzle engine alive. This version is here so that
maintenance never *requires* Rust or a WASM toolchain. If you can read Lua, you
can read and change the whole engine.

## Run it

You need a Lua interpreter (5.3 or newer — it relies on 64-bit integers and
bitwise operators).

```sh
# macOS
brew install lua

# Debian / Ubuntu
sudo apt-get install lua5.4
```

Then, from this directory:

```sh
lua demo.lua            # generate + solve a 6x6, then probe the official set
lua demo.lua 8 7        # generate + solve an 8x8 with seed 7
lua spec.lua            # full parity test suite against the Rust engine
```

Expected output of `lua demo.lua`:

```
Eternity II — pure Lua engine (Lua 5.5)
----------------------------------------------------
generated 6x6 (c=22, seed=42):
  status=solved  placed=36/36  score=60/60
  nodes=37  attempts=1416  backtracks=1
official Eternity II (16x16, 50,000-step probe):
  status=running  placed=47  best_placed=48  attempts=21297806
----------------------------------------------------
OK
```

(If your Lua isn't on `PATH` as `lua`, use `lua5.4` / `luajit` instead.)

## Layout

| File             | Mirrors (Rust)        | What it does                                   |
| ---------------- | --------------------- | ---------------------------------------------- |
| `eternity2.lua`  | `types.rs`, RNG       | edges/rotation, the XorShift RNG, `max_score`  |
| `puzzle.lua`     | `generator.rs`, `official.rs`, `paths.rs` | seeded generator, official set loader, the 9 path orders |
| `solver.lua`     | `solver.rs`           | step-able backtracking DFS + the board scorer  |
| `demo.lua`       | —                     | human-facing smoke test                        |
| `spec.lua`       | the crate's `#[test]`s | parity suite                                   |
| `golden.txt`     | —                     | reference outputs captured from the Rust engine |

## Parity — this is the whole point

`spec.lua` checks the Lua engine against `golden.txt`, a set of reference
outputs produced by the Rust engine itself. It does not merely check that both
engines *can solve* a puzzle — it checks that they explore the search tree
**identically**, down to the exact node / attempt / backtrack counts:

```
== generator parity ==
== official set parity ==
== path parity ==
== solver parity (generated 4x4 seed 11, all paths) ==
== official partial-run parity (fixed step budget) ==

99 passed, 0 failed
```

Matching the counts (e.g. `25021` nodes / `21297806` attempts / `24979`
backtracks on the official set after 50,000 steps) is a strong guarantee: the
two implementations are the same algorithm, not two programs that happen to
agree on the answer.

### Regenerating the golden data

If you change the engine and need fresh reference values, re-emit them from the
Rust crate:

```sh
cd ../../engine
cargo run --quiet --bin golden > ../engine-side-quests/engine-lua/golden.txt
```

(The `golden` binary lives in `../../engine/src/bin/golden.rs`.)

## Conventions (identical to the Rust / WASM contract)

- Edge order is **URDL** (up, right, down, left), bucas convention.
- Color **0** is the grey border; interior colors are **1..22**.
- A board is `cell -> piece*4 + rotation`, or `-1` for an empty cell, with
  `cell` and `piece` 0-based — so the data matches the Rust serde output.
- Rotation `r` is clockwise quarter-turns.

Lua is 1-indexed and the contract is 0-indexed; the port keeps every *domain*
index (cells, pieces, colors, rotations, board encodings) 0-based as values and
only adds the `+1` at the Lua-table boundary. See the header comment in
`eternity2.lua` for the rule.

## Performance note

This is a readability-first reference implementation, not a speed contender —
interpreted Lua is far slower than compiled Rust→WASM, so it is meant for
understanding, teaching, and small/medium boards rather than for grinding the
full 16×16 official puzzle. The fast path in production stays the Rust/WASM
build under [`../../engine`](../../engine); this exists to prove the engine is not
locked to Rust and to give a maintainer a version they can actually read.
