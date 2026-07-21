# Engine ports

The Eternity II engine, reimplemented in languages other than the canonical
Rust — one folder per language. They fall into two groups:

- **Full ports** that match the Rust engine's *entire* functionality (RNG, all 9
  path orders, the step-able backtracking solver, exact node/attempt/backtrack
  counts): TypeScript, C, C++, Python.
- **Smaller studies** in deliberately simpler or older languages (Lua, COBOL,
  Brainfuck), written partly to prove the engine isn't locked to Rust and partly
  as a pointed answer to "Rust is unmaintainable."

Every implementation here is validated **byte-for-byte against the Rust engine's
output** (see `golden.txt` in the ports that carry one). None of these are
shipped or build-selectable — the site always runs the Rust/WASM engine in
[`../engine`](../engine). This collection is a study exhibit.

> **The algorithm is documented once, for all of them, in
> [`ALGORITHM.md`](ALGORITHM.md).** A maintainer who reads that one document can
> follow every port — the code in each language is a literal translation of it,
> same names, same steps. Start there.

The real, fast engine remains the Rust/WASM build in [`../engine`](../engine)
and powers the website.

## What's here

### Full ports

| Directory | Language | Validated |
| --- | --- | --- |
| [`typescript/`](typescript) | **TypeScript** (zero WASM) | `parity.mjs`: 106 checks vs Rust golden |
| [`c/`](c) | **C** → WASM (clang, freestanding) | `parity.mjs`: wasm-under-Node vs golden |
| [`cpp/`](cpp) | **C++** → WASM (clang++) | `parity.mjs`: wasm-under-Node vs golden |
| [`python/`](python) | **Python** | `spec.py`: 99 assertions vs golden |

### Smaller studies (simpler/older languages)

| Directory | Language | What it is | Validated on |
| --- | --- | --- | --- |
| [`lua/`](lua) | **Lua** | A full pure-Lua port (types, RNG, generator, official set, all 9 path orders, step-able solver + scorer). No deps, no build step. | `spec.lua`: 99 assertions, 3×3 → 16×16. |
| [`cobol/`](cobol) | **COBOL** | A GnuCOBOL solver/scorer: explicit-stack row-major DFS. | `test.sh`: 4 puzzles vs the Rust `RESULT` line. |
| [`brainfuck/`](brainfuck) | **Brainfuck** | A *real backtracking solver* in 8 instructions over a byte tape, via a kept-in-repo tape macro-compiler. | `test_solver.sh`: 3×3 boards match Rust. |

Each directory has its own README with the full story, build steps, and tests.

## Run them all

```sh
./run-all.sh
```

This compiles and runs every engine and checks each against the Rust engine:

- **3×3** — Lua, COBOL **and** Brainfuck (the Brainfuck solver is only fast
  enough for 3×3, ~1 s).
- **4×4 and 5×5** — Lua, COBOL and Python.
- **Full golden parity** — TypeScript, C, C++ and Python run their complete
  parity harnesses (every path/size + the official 16×16 probe).

The generated puzzles use few colours (5–6) on purpose: with the maximum colour
count every placement is forced and the solve is trivial, so fewer colours make
the search actually **backtrack** — the interesting part to watch.

`run-all.sh` gracefully skips any engine whose toolchain is missing
(`lua` / `cobc` / `node` / `python3` / a wasm-capable `clang`).

## Per-engine tests

```sh
node typescript/parity.mjs                # pure-TS port vs golden
node c/parity.mjs                 # C/WASM vs golden
node cpp/parity.mjs               # C++/WASM vs golden
( cd python    && python3 spec.py )          # Python vs golden
( cd lua       && lua spec.lua )             # 99 parity assertions
( cd cobol     && ./test.sh )                # 4 puzzles vs Rust RESULT
( cd brainfuck && ./test_solver.sh )         # 3×3 boards vs Rust
( cd brainfuck && ./test_solver.sh 4 )       # 4×4 (slow, optional)
```

## Toolchains

| Engine | Needs |
| --- | --- |
| TypeScript | `node` 18+ (the website's own toolchain; no extra deps) |
| C / C++ | a wasm-capable `clang`/`clang++` + `wasm-ld` (Homebrew LLVM + `lld`); `node` to run parity. The `.wasm` is committed, so the website builds without them. |
| Python | CPython 3.9+ (standard library only) |
| Lua | `lua` 5.3+ (`brew install lua` / `apt-get install lua5.4`) |
| COBOL | GnuCOBOL `cobc` (`brew install gnu-cobol` / `apt-get install gnucobol`) |
| Brainfuck | Python 3 (compiler) and, for speed, `rustc` (the fast interpreter; a Python fallback exists) |

## A note on the conventions (shared by every port)

These are spelled out in full in [`ALGORITHM.md`](ALGORITHM.md) §2, but in brief:

- Edge order is **URDL** (up, right, down, left), bucas convention.
- Color **0** is the grey border; interior colors are **1..22**.
- Rotation `r` is clockwise quarter-turns.
- A board cell holds `piece*4 + rotation` (0-based), so the data lines up with
  the Rust engine's output and every comparison is exact.
