# Engine side-quests

Re-implementations of the Eternity II engine in deliberately *simpler* or
*older* languages. They exist because a maintainer balked at the real engine
being written in Rust (compiled to WebAssembly): "too hard to maintain." So
here is the same engine — same path-driven backtracking search, same scorer,
same conventions — in three other languages, each one validated **byte-for-byte
against the Rust engine's output**.

The real, fast engine remains the Rust/WASM build in [`../engine`](../engine)
and powers the website. These are standalone CLIs: a proof that the engine is
not locked to Rust, an education resource, and — for COBOL and Brainfuck — a
pointed answer to "Rust is unmaintainable."

## What's here

| Directory | Language | What it is | Validated on |
| --- | --- | --- | --- |
| [`engine-lua/`](engine-lua) | **Lua** | The serious one. A full pure-Lua port (types, RNG, generator, official set, all 9 path orders, step-able backtracking solver + scorer). No dependencies, no build step. | `spec.lua`: 99 assertions vs Rust golden data, incl. exact node/attempt/backtrack counts, 3×3 → 16×16. |
| [`engine-cobol/`](engine-cobol) | **COBOL** | A GnuCOBOL solver/scorer: explicit-stack row-major DFS (COBOL has no comfortable recursion). | `test.sh`: 4 puzzles match the Rust `RESULT` line field-for-field. |
| [`engine-brainfuck/`](engine-brainfuck) | **Brainfuck** | A *real backtracking solver* in 8 instructions over a byte tape, built via a kept-in-repo tape macro-compiler. | `test_solver.sh`: 3×3 boards match the Rust engine's exactly (incl. backtracking cases). |

Each directory has its own README with the full story, build steps, and tests.

## Run them all

```sh
./run-all.sh
```

This compiles and runs every engine and checks each against the Rust engine:

- **3×3** — Lua, COBOL **and** Brainfuck (the Brainfuck solver is only fast
  enough for 3×3, ~1 s).
- **4×4 and 5×5** — Lua and COBOL only. The Brainfuck solver *can* do 4×4
  (~100 s) but not as part of a quick run; bigger boards would take far too
  long.

The puzzles use few colors (5–6) on purpose: with the maximum color count every
placement is forced and the solve is trivial, so fewer colors are used to make
the search actually **backtrack** — which is the interesting part to watch.

`run-all.sh` gracefully skips any engine whose toolchain is missing
(`lua` / `cobc` / `rustc`).

## Per-engine tests

```sh
( cd engine-lua       && lua spec.lua )            # 99 parity assertions
( cd engine-cobol     && ./test.sh )               # 4 puzzles vs Rust RESULT
( cd engine-brainfuck && ./test_solver.sh )        # 3×3 boards vs Rust
( cd engine-brainfuck && ./test_solver.sh 4 )      # 4×4 (slow, optional)
```

## Toolchains

| Engine | Needs |
| --- | --- |
| Lua | `lua` 5.3+ (`brew install lua` / `apt-get install lua5.4`) |
| COBOL | GnuCOBOL `cobc` (`brew install gnu-cobol` / `apt-get install gnucobol`) |
| Brainfuck | Python 3 (compiler) and, for speed, `rustc` (the fast interpreter; a Python fallback exists) |

## A note on the conventions (shared by all three)

- Edge order is **URDL** (up, right, down, left), bucas convention.
- Color **0** is the grey border; interior colors are **1..22**.
- Rotation `r` is clockwise quarter-turns.
- A board cell holds `piece*4 + rotation` (0-based), so the data lines up with
  the Rust engine's output and the comparison is exact.
