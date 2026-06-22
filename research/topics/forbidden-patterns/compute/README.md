# compute — forbidden patterns

A small, self-contained Rust crate that produces this topic's results. It depends
on the shared engine (`engine/`) via a path dependency, so it reuses the official
piece set and edge conventions without copying any puzzle data.

- **Source:** [`src/main.rs`](src/main.rs)

For each small shape (two cells side by side, an L of three cells, a 2x2 square)
it counts, exactly and exhaustively, how many ways you can drop that many
distinct interior pieces into the shape so that every shared edge can be made to
match — versus how many ways are impossible no matter how you rotate the pieces.
No sampling, no randomness.

## Run

From this directory:

```sh
cargo run --release > ../results/feasibility.json
```

Output is deterministic — it reproduces the committed
`../results/feasibility.json` byte-for-byte. The 2x2 sweep walks all ~1.43
billion distinct quadruples and dominates the runtime (about 20 seconds); the
smaller shapes are near-instant.
