# compute — record boards

A self-contained Rust crate that recomputes the matched-edge score of each bundled
record board straight from its e2.bucas.name edge string, so every score we
publish can be checked independently of how the board was found.

- **Source:** [`src/main.rs`](src/main.rs)
- **Board data:** [`../boards.json`](../boards.json) (embedded at compile time)

## Run

From this directory:

```sh
cargo run --release > ../results/verified-scores.json
```

Deterministic; reproduces the committed `../results/verified-scores.json`
byte-for-byte. For each board it prints the claimed score, the score recomputed
from the board edges, and whether they match (they do, for all boards).
