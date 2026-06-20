# engine-python

A **pure-Python** ISO re-implementation of the Eternity II engine — the same
algorithm as the website's Rust/WASM engine, in standard-library Python with no
dependencies.

> The algorithm itself is documented once, language-agnostically, in
> [`../ALGORITHM.md`](../ALGORITHM.md). Read that first; every function in
> [`eternity2.py`](eternity2.py) points back to one of its sections. This README
> only covers how to *run* the Python port.

## What's here

| File | What it is |
| --- | --- |
| [`eternity2.py`](eternity2.py) | The whole engine: XorShift RNG, generator, official set, all 9 path orders, the step-able backtracking `Solver`, and the scorer. One file, heavily commented. |
| [`spec.py`](spec.py) | Parity test against Rust golden data — 99 assertions, incl. exact node/attempt/backtrack counts. |
| [`demo.py`](demo.py) | A human-facing smoke test: generate + solve a small puzzle, then probe the official 16×16. |
| `official_eternity2.csv` | The canonical official piece set (copy of `engine/data/`). |
| `golden.txt` | Rust reference outputs (copy of the Lua port's golden data). |

## Run it

Needs only CPython 3.9+ (no `pip install` anything).

```sh
# Full parity suite vs the Rust engine (the real correctness check):
python3 spec.py

# Solve a generated puzzle + probe the official set:
python3 demo.py                # defaults: 6x6, seed 42, 5 colours
python3 demo.py 5 3 6          # 5x5, seed 3, 6 colours
python3 demo.py 3 2 3          # 3x3, seed 2, 3 colours (lots of backtracking)
```

Fewer colours than edges makes pieces share edge colours, so the search has to
**backtrack** — which is the interesting part to watch. With the maximum colour
count every placement is forced and the solve is trivial.

## Parity

`spec.py` reads `golden.txt` — outputs the Rust engine actually produced — and
checks the Python engine reproduces them exactly: generated-puzzle pieces (RNG +
construction), the official set summary and hints, all 9 path permutations at
three sizes, full 4×4 solver runs across every path (status, score, **and** the
exact node/attempt/backtrack counts), and a fixed 50,000-step probe of the
official set. Matching the counters proves the two engines walk the identical
search tree, not merely that they both solve.

```
$ python3 spec.py
...
99 passed, 0 failed
```

## Using it as a library

```python
import eternity2 as E

p = E.generate(size=5, colors=6, seed=3)        # solvable scrambled puzzle
path = E.build_path("snake", 5, 5, 0)            # a cell-visit order
solver = E.Solver(p, path)
while solver.step(100_000).status == "running":  # run in bounded bursts
    pass
print(E.score_board(p, solver.board_cells()))    # 40 (= 2*5*5 - 5 - 5)
```

The conventions (URDL edges, colour 0 = border, `cell = piece*4 + rot`) are the
same across every port — see [`../ALGORITHM.md`](../ALGORITHM.md) §2.
