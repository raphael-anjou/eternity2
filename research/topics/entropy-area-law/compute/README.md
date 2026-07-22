# compute: entropy and the area law

A Rust crate (path-depends on the shared engine) that computes, from the official
piece set, both halves of the entropy-area-law finding.

- **Source:** [`src/main.rs`](src/main.rs)

## Part 1: grammar entropy

Treat the 196 interior pieces as reusable tiles and measure how fast an n-wide
matched strip can grow. The crate builds the row-transfer matrix for widths 1 and
2 and finds its dominant eigenvalue by power iteration, giving the per-cell
entropies h(1) and h(2) exactly, plus the 1-D horizontal bound lambda_H. Widths 3
and 4, and the limit h_infinity, come from a heavier offline width-n sweep and are
carried as constants.

## Part 2: the area law (computed in-repo)

For an interior n×n block with a free outer border it counts, exactly:

- **A(n)**: colour-valid fillings with pieces reusable (the matching grammar), via
  a dense broken-profile transfer DP.
- **B(n)**: colour-valid fillings that use distinct pieces (true Eternity II), via
  a parallel distinct-counting DFS whose cost is bounded by A(n).

rho(n) = B(n)/A(n) is the realizable fraction. The area-law exponent alpha is fit
by least squares through the origin on ln rho(n) = -alpha n^2 over the exact range.

Exact through n = 3; n = 4 computes A(4) but B(4) is over the DFS budget (A(4) is
about 6.3e16, far past a few-minute enumeration), which is the wall. Cross-checked
at n = 2 against the in-repo `subgrid-placement-counts` reference table
(B(2) = 4 059 952).

## Run

From this directory:

```sh
cargo run --release > ../results/entropy.json
```

Deterministic; reproduces the committed `../results/entropy.json`. The n = 3
distinct DFS dominates the runtime (about 12 minutes on 8 cores, roughly 1.3e11
nodes); everything else is seconds. The width-2 eigenvalue is a 529x529 power
iteration (about a second).
