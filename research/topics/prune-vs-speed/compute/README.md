# compute — prune beats speed

A small Rust crate (path-depends on the shared engine) that measures, with the
project's own depth-first solver, why reducing the search space beats raw speed.

- **Source:** [`src/main.rs`](src/main.rs)

It does two things:

1. **Hardness curve.** Solves generated 6×6 puzzles across colour counts (median
   over several seeds) and records the nodes the solver explores. The curve rises
   to a peak — the phase transition, about one expected solution — then falls.
2. **Lever comparison.** From the peak node count it derives the effective
   branching factor, then reports what a 1000× raw speedup buys (work ÷ 1000)
   versus what a 5% cut to the branching factor buys (work · 0.95^depth) — the
   exponential gap, in the puzzle's own numbers.

## Run

From this directory:

```sh
cargo run --release > ../results/prune-vs-speed.json
```

Deterministic (fixed seeds, fixed board): reproduces the committed
`../results/prune-vs-speed.json` byte-for-byte. The tree-size estimate's
branching factor and depth are illustrative; the node counts are real engine
measurements.
