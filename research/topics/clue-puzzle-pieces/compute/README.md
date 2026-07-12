# compute — clue-puzzle solve and verify

A self-contained Rust crate that solves an Eternity II clue puzzle and checks the
result. It reads a piece file (`ID T R B L` or bare `T R B L` per line, URDL
edges, `0` = grey rim), encodes "place every piece so all edges match and grey
sits exactly on the rim" as SAT, solves with `kissat`, then independently
validates the model: all pieces used once, every internal edge matched, grey only
on the rim.

- **Source:** [`src/main.rs`](src/main.rs)
- **Data:** [`../data`](../data) (the recovered piece sets for all four clue puzzles)

SAT is used rather than a hand-rolled backtracker because it is decisive and
fast here: a naive row-major backtracker thrashes on Clue Puzzle 1 (a
solution-dense but ordering-hostile instance) and fails to find a solution even
after billions of nodes, while kissat settles it in a fraction of a second.

## Run

```sh
cargo build --release
# JSON record (solved + validated + matched-edge count):
cargo run --release -- ../data/clue3_jwortmann.txt 6 6
# optional: a self-contained flat-colour SVG of the solution:
cargo run --release -- ../data/clue4_jwortmann.txt 12 6 --svg > clue4.svg
```

Requires `kissat` on PATH. The topic-level [`../run.sh`](../run.sh) solves all four
puzzles and regenerates the web data modules (the site renders the solved boards
with the real Eternity II motifs).
