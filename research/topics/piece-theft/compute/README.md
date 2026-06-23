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
