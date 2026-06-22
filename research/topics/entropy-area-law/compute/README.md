# compute — entropy and the area law

A Rust crate (path-depends on the shared engine) that computes the entropy density
of Eternity II's color-matching grammar: treat the 196 interior pieces as reusable
tiles and measure how fast an n-wide matched strip can grow.

- **Source:** [`src/main.rs`](src/main.rs)

It builds the row-transfer matrix for widths 1 and 2 and finds its dominant
eigenvalue by power iteration, giving the per-cell entropies h(1) and h(2) exactly,
plus the 1-D horizontal bound lambda_H. Widths 3 and 4, and the limit h_infinity,
come from a heavier offline width-n sweep and are carried as constants.

## Run

From this directory:

```sh
cargo run --release > ../results/entropy.json
```

Deterministic; reproduces the committed `../results/entropy.json` byte-for-byte.
The width-2 eigenvalue dominates the runtime (a 529x529 power iteration, about a
second).
