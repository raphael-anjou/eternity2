# compute: record boards

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

## Community boards given as pieces and rotations

[`verify_pieces.py`](verify_pieces.py) is a standard-library Python verifier for a
board stored as 256 pairs of official piece number and clockwise rotation (the
files under [`../community/`](../community/)). From
`engine/data/official_eternity2.csv` alone it checks that every official piece is
used once, that the five official clue pieces sit at their official cells and
rotations, that grey sides lie only on the frame, and counts matched internal
edges out of 480. It also checks that the stored `board_edges` string equals the
edges recomputed from the pieces. From the repository root:

```sh
just research-record-boards-pieces
```

This re-verifies stored boards (artifact verification); it does not search.
Deterministic; reproduces the committed `../results/community-verified.json`
byte-for-byte.
