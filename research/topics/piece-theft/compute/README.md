# compute — piece theft

A small Rust crate (path-depends on the shared engine) that measures the scarcity
behind dead-ends in a top-left-to-bottom-right solver. For every (north, west)
color demand an interior cell can have, it counts how many distinct interior
pieces could serve it in some rotation.

- **Source:** [`src/main.rs`](src/main.rs)

The demands are scarce (mean ~2.9 servers, many with just one), which is why a
single piece spent in the wrong place dooms a future cell: "piece theft".

## Run

From this directory:

```sh
cargo run --release > ../results/piece-theft.json
```

Exact, exhaustive, deterministic; reproduces the committed
`../results/piece-theft.json` byte-for-byte.

## Null model

`src/bin/null_model.rs` computes the same (north, west) server counts on 100 randomly painted
16x16 boards with Eternity II's palette (the engine's
`generate_framed(16, 22, seed, true)`: 5 frame colours, 17 interior colours),
alongside the official set, so the official numbers can be compared with a
random puzzle of the same size and palette:

```sh
cargo run --release --bin null_model > ../results/null-model.json
```

Deterministic for the seed range (1 to 100); reproduces the committed
`../results/null-model.json` byte-for-byte.
