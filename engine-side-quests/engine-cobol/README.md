# Eternity II engine — COBOL

The Eternity II edge-matching solver and scorer, written in **COBOL**.

This is the pointed answer to "Rust is hard to maintain": here is the same
engine in a language that predates the puzzle, the web, and most of the people
who will ever read it — and it compiles and solves. It is a real port of the
core of [`../../engine/src/solver.rs`](../../engine/src/solver.rs): a row-major
backtracking depth-first search with an **explicit stack** (COBOL has no
comfortable recursion, so the search is a flat `PERFORM` loop over a depth
cursor — which is, conveniently, exactly how the Rust solver is structured: an
explicit machine, not a recursive function), followed by the same
bucas-convention scorer.

It is a standalone command-line program, not wired into the website.

## Build & run

You need [GnuCOBOL](https://gnucobol.sourceforge.io/) (`cobc`).

```sh
# macOS
brew install gnu-cobol

# Debian / Ubuntu
sudo apt-get install gnucobol
```

Then:

```sh
cobc -x -free eternity2.cbl -o eternity2     # compile
./eternity2 data/p_5_5_3.txt                 # solve one puzzle
```

Output (one machine-parseable line):

```
RESULT 5 5 3: status=SOLVED placed=25 score=40 nodes=197 attempts=10283 backtracks=172
```

## Input format

`./eternity2 <piece-file>`. A piece file is plain text:

```
<size> <colors> <seed>
<u> <r> <d> <l>      ← one piece per line, URDL, rotation 0
<u> <r> <d> <l>
...
```

The `data/*.txt` files were emitted from the Rust generator (so the same
`(size, colors, seed)` produces the same pieces), via
[`../../engine/src/bin/cobol_data.rs`](../../engine/src/bin/cobol_data.rs):

```sh
cd ../../engine && cargo run --quiet --bin cobol_data
```

## Parity — this is the whole point

`test.sh` runs the COBOL solver on every `data/*.txt` puzzle and compares its
`RESULT` line, field for field, against `golden.txt` (captured from the Rust
engine). It checks not just that both *solve*, but that the COBOL search
explores the tree **identically** — down to the exact node, attempt, and
backtrack counts:

```sh
./test.sh
```

```
  ok   4x4 c=4 s=11
  ok   5x5 c=5 s=3
  ok   5x5 c=6 s=42
  ok   3x3 c=3 s=1

4 passed, 0 failed
```

Regenerate the golden reference from the Rust crate if the engine changes:

```sh
cd ../../engine && cargo run --quiet --bin cobol_golden | grep '^RESULT' \
    > ../engine-side-quests/engine-cobol/golden.txt
```

## Conventions (identical to the Rust contract)

- Edge order **URDL** (up, right, down, left).
- Color **0** is the grey border; interior colors **1..22**.
- Rotation `r` is clockwise quarter-turns; `new[i] = old[(i+4-r) mod 4]`.
- A board cell holds `piece*4 + rot` (0-based) or "empty"; COBOL tables are
  1-based, so the boundary adds `+1` and the rest is reasoned about 0-based.

## Scope & honesty

This solves generated small/medium boards exactly. It does **not** carry the
official puzzle's 5 clue pieces (the generated inputs have no hints) and is not
meant to grind the full 16×16 — it is a faithful, runnable port of the search
core, written to make a point about language choice, not to beat the Rust/WASM
build under [`../../engine`](../../engine), which remains the real engine.
