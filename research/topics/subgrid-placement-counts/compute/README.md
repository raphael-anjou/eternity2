# compute — subgrid placement counts

A self-contained Rust crate that produces this topic's results. It depends on the
shared engine (`engine/`) via a path dependency, so it reuses the official piece
set and edge conventions without duplicating any puzzle data.

- **Source:** [`src/main.rs`](src/main.rs)

It enumerates, for each catalogued block position, the number of valid
distinct-piece fillings under each column variant (see [`../article.md`](../article.md)
for the exact definitions) and prints the table as JSON.

## Run

From this directory:

```sh
cargo run --release > ../results/reference-table.json
```

Output is deterministic — it must reproduce the committed
`../results/reference-table.json` byte-for-byte. The 4×4 corner columns dominate
the runtime (tens of millions of fillings); the 2×2 and 3×3 rows are
near-instant.
