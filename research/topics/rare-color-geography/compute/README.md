# compute — rare color geography

A small Rust crate (path-depends on the shared engine) that sorts the official
set's 22 colors by where their edges appear: on the frame (edge and corner pieces)
or in the interior.

- **Source:** [`src/main.rs`](src/main.rs)

It finds the five rare colors that live only on the border band, each on exactly
24 edges, and emits per-color frame/interior edge counts for the rest.

## Run

From this directory:

```sh
cargo run --release > ../results/color-geography.json
```

Exact and deterministic; reproduces the committed `../results/color-geography.json`
byte-for-byte.
