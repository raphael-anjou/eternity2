# Eternity II engine — Brainfuck

A real, running **backtracking solver** for Eternity II, in **Brainfuck**.

This is the end of the "Rust is too hard to maintain" road. Brainfuck has eight
instructions (`+ - < > [ ] . ,`) and a single byte tape — no integers, no
arrays, no `if`, no functions, no variables. And yet `solver.bf` is the *same*
explicit-stack depth-first search as [`../../engine/src/solver.rs`](../../engine/src/solver.rs):
it reads a puzzle, runs a row-major backtracking search with piece-availability
tracking and edge matching, unwinds the stack on dead ends, and prints the
solved board. It produces byte-for-byte the same solution as the Rust engine.

It is sized for small boards (3×3, 4×4, 5×5) — the size where a Brainfuck
backtracker actually finishes — and is a standalone CLI, not wired into the
website.

## How it's built (and why the compiler stays)

Nobody hand-writes ten million Brainfuck instructions. `solver.bf` is
*generated* by a tiny compiler:

```
bfc.py          a ~250-line tape macro-compiler: named registers, arrays with a
                runtime index (linear scan), copy/add/sub, equality, if/while —
                each compiling to a standard Brainfuck idiom.
solver_src.py   the engine, written against that API (reads like the Rust/Lua
                port): table build, the DFS loop, fit-checking, place/backtrack.
build.py        solver_src.py -> solver.bf
```

We keep the compiler in the repo on purpose: it is what makes the Brainfuck
**maintainable**. To change the engine you edit `solver_src.py` and recompile —
you never edit the `.bf` by hand. The shipped `solver.bf` is genuine,
self-contained Brainfuck that runs in any compliant interpreter; the compiler is
how you *regenerate* it, not a runtime dependency.

## Build & run

You need Python 3 (for the compiler) and, for speed, `rustc` (for the fast
interpreter — optional, there is a Python fallback).

```sh
# Compile the engine to Brainfuck (default 3×3):
python3 build.py                      # writes solver.bf

# Build the fast interpreter:
rustc -O bf.rs -o bf

# Solve a puzzle (board printed as raw bytes = piece*4+rot per cell):
./bf solver.bf < data/bf_3_3_2.bin \
  | python3 -c 'import sys; print(*sys.stdin.buffer.read())'
# -> 14 3 29 11 6 16 33 21 26   (identical to the Rust engine)

# Watch a longer run make progress (heartbeat to stderr):
./bf --trace solver.bf < data/bf_3_3_2.bin > /dev/null
```

Other board sizes (the build is parameterised):

```sh
E2_BF_SIZE=5 python3 build.py        # writes a 5×5 solver.bf
```

The Python interpreter `bf.py` runs the same programs (`python3 bf.py
solver.bf < input`), just ~100× slower — fine for the unit tests, slow for a
full 5×5 search.

## Parity — this is the whole point

`test_solver.sh` compiles the solver, runs it on every `data/bf_3_3_*.bin`
puzzle, and compares the solved board to the board the **Rust engine** produced
(`*.board.txt`). The puzzles include backtracking cases (seed 2 takes 6
backtracks, seed 5 takes 3), so a pass proves the Brainfuck stack unwinds
correctly, not just that it walks straight to an answer.

```sh
./test_solver.sh        # 3x3, ~1s each
./test_solver.sh 4      # 4x4, ~100s each (same code, bigger search)
```

```
  ok   bf_3_3_1 -> 4 18 12 1 33 11 25 21 29
  ok   bf_3_3_2 -> 14 3 29 11 6 16 33 21 26
  ok   bf_3_3_5 -> 0 14 27 33 9 20 19 4 28
  ok   bf_3_3_7 -> 6 18 9 31 13 0 26 35 23

3x3: 4 passed, 0 failed
```

A 4×4 solves to the same board the Rust engine finds
(`1 47 35 52 14 29 8 59 4 61 25 40 37 16 22 51` for seed 11) — it just takes
~100 s, because the search is bigger and every step is interpreted Brainfuck.

The compiler's own primitives are unit-tested too:

```sh
python3 test_bfc.py            # 26 tests: scalars, eq, if/while, indexed arrays
python3 test_solver_helpers.py # 32 tests: divmod, <, x/y, row index, align
```

## Input / output format

- **Input** (stdin, raw bytes): one `SIZE` byte, then `N = SIZE²` pieces, each 4
  bytes in URDL order (up, right, down, left), rotation 0. The `data/*.bin`
  files come from the Rust generator (`../../engine/src/bin/bf_data.rs`), so the
  same `(size, seed)` yields the same pieces — that's what makes the comparison
  exact.
- **Output** (stdout, raw bytes): the solved board, one byte per cell =
  `piece*4 + rotation` (0-based), row-major. Empty/edge bytes never appear in a
  solved board.

## Conventions (identical to the Rust contract)

- Edge order **URDL**; color **0** is the grey border; interior colors **1..22**.
- Rotation `r` is clockwise quarter-turns.
- Board cell = `piece*4 + rot`; inside the Brainfuck the board stores `that + 1`
  (0 means empty, since the tape starts zeroed) and the final output subtracts 1.

## Performance

This is interpreted Brainfuck, so it is slow on purpose-built terms: a 3×3
solves in about a second, a 4×4 in roughly a minute and a half. The dominant
cost is runtime-indexed array access — Brainfuck has no random access, so every
`arr[i]` is a linear scan. The rotated-edge table (the hottest array) is
therefore stored split into four per-direction arrays so each lookup scans a
quarter as many cells, which cut the per-step cost ~8× and is the difference
between a 4×4 finishing in ~100 s and not finishing observably at all. Beyond
~4×4 the search space grows faster than is worth waiting for; 3×3 is the size
where everything (including the full `test_solver.sh`) runs comfortably.

## Scope & honesty

This is a faithful, *running* port of the search core for small boards — the
point is the language, not the scale. It carries no clue pieces (the generated
inputs have none) and is not meant for the full 16×16 official puzzle: a
Brainfuck backtracker over 256 pieces would run effectively forever. The real
engine remains the Rust/WASM build under [`../../engine`](../../engine); this
exists to make a point about what "unmaintainable language" actually means.
