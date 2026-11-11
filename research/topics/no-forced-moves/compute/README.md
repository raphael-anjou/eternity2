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

## Null model

`src/bin/null_model.rs` computes the same partner counts on 100 randomly painted
16x16 boards with Eternity II's palette (the engine's
`generate_framed(16, 22, seed, true)`: 5 frame colours, 17 interior colours),
alongside the official set, so the official numbers can be compared with a
random puzzle of the same size and palette:

```sh
cargo run --release --bin null_model > ../results/null-model.json
```

Deterministic for the seed range (1 to 100); reproduces the committed
`../results/null-model.json` byte-for-byte.
