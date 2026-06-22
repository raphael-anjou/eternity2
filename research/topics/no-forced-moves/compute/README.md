# compute — no forced moves

A small Rust crate (path-depends on the shared engine) that counts, for each of
the 196 interior pieces, how many other interior pieces could sit immediately to
its right (right edge matching their left edge, under some rotation of each).

- **Source:** [`src/main.rs`](src/main.rs)

The point: no piece is ever down to one option, so there is no "only one piece
fits here" lever to pull. It emits min/max/median/mean partner counts, the number
of forced pieces (zero), and a histogram.

## Run

From this directory:

```sh
cargo run --release > ../results/partner-counts.json
```

Exact, exhaustive, deterministic; reproduces the committed
`../results/partner-counts.json` byte-for-byte.
