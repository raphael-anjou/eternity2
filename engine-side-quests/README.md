# Engine side-quests

Re-implementations of the Eternity II engine in languages other than the
canonical Rust. They fall into two groups:

- **ISO ports** that match the Rust engine's *full* functionality (RNG, all 9
  path orders, the step-able backtracking solver, exact node/attempt/backtrack
  counts) — TypeScript, C, C++, Python. Three of these (TS/C/C++) are also
  **browser-switchable**: the website can be built against any of them.
- **Side-quests proper** — deliberately *simpler* or *older* languages (Lua,
  COBOL, Brainfuck), written partly to prove the engine isn't locked to Rust and
  partly as a pointed answer to "Rust is unmaintainable."

Every implementation here is validated **byte-for-byte against the Rust engine's
output**.

> **The algorithm is documented once, for all of them, in
> [`ALGORITHM.md`](ALGORITHM.md).** A maintainer who reads that one document can
> follow every port — the code in each language is a literal translation of it,
> same names, same steps. Start there.

The real, fast engine remains the Rust/WASM build in [`../engine`](../engine)
and powers the website.

## What's here

### ISO ports (full functionality)

| Directory | Language | Browser-switchable? | Validated |
| --- | --- | --- | --- |
| [`../web/src/engine-ts/`](../web/src/engine-ts) | **TypeScript** | ✅ `VITE_ENGINE=ts` (zero WASM) | `parity.mjs`: 100 checks vs Rust golden |
| [`../web/src/engine-c/`](../web/src/engine-c) | **C** → WASM (clang, freestanding) | ✅ `VITE_ENGINE=c` | `parity.mjs`: wasm-under-Node vs golden |
| [`../web/src/engine-cpp/`](../web/src/engine-cpp) | **C++** → WASM (clang++) | ✅ `VITE_ENGINE=cpp` | `parity.mjs`: wasm-under-Node vs golden |
| [`engine-python/`](engine-python) | **Python** | CLI only | `spec.py`: 99 assertions vs golden |

The three switchable ports live under `web/src/` (next to the build that
consumes them) and all implement the *identical* surface as the Rust/WASM
engine. See [`../web/README.md`](../web/README.md) for the `VITE_ENGINE` switch.

### Side-quests (simpler/older languages)

| Directory | Language | What it is | Validated on |
| --- | --- | --- | --- |
| [`engine-lua/`](engine-lua) | **Lua** | A full pure-Lua port (types, RNG, generator, official set, all 9 path orders, step-able solver + scorer). No deps, no build step. | `spec.lua`: 99 assertions, 3×3 → 16×16. |
| [`engine-cobol/`](engine-cobol) | **COBOL** | A GnuCOBOL solver/scorer: explicit-stack row-major DFS. | `test.sh`: 4 puzzles vs the Rust `RESULT` line. |
| [`engine-brainfuck/`](engine-brainfuck) | **Brainfuck** | A *real backtracking solver* in 8 instructions over a byte tape, via a kept-in-repo tape macro-compiler. | `test_solver.sh`: 3×3 boards match Rust. |

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
node ../web/src/engine-ts/parity.mjs                # pure-TS port vs golden
node ../web/src/engine-c/parity.mjs                 # C/WASM vs golden
node ../web/src/engine-cpp/parity.mjs               # C++/WASM vs golden
( cd engine-python    && python3 spec.py )          # Python vs golden
( cd engine-lua       && lua spec.lua )             # 99 parity assertions
( cd engine-cobol     && ./test.sh )                # 4 puzzles vs Rust RESULT
( cd engine-brainfuck && ./test_solver.sh )         # 3×3 boards vs Rust
( cd engine-brainfuck && ./test_solver.sh 4 )       # 4×4 (slow, optional)
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
