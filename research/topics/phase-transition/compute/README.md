# compute — phase transition (color split)

A tiny, self-contained Rust crate that reads the official Eternity II set from the
shared engine (`engine/`, path dependency) and sorts its colors by where they
appear: on interior pieces, or only on the frame.

- **Source:** [`src/main.rs`](src/main.rs)

It confirms the set's design split: 17 interior colors and 5 colors that live only
on the border band.

## Run

From this directory:

```sh
cargo run --release > ../results/color-split.json
```

Instant and deterministic — it reproduces the committed
`../results/color-split.json` byte-for-byte.
