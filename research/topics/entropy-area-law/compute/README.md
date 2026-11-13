# compute: entropy and the use-once rule

A Rust crate (path-depends on the shared engine) that computes, from the official
piece set, both halves of the entropy-area-law finding.

- **Source:** [`src/main.rs`](src/main.rs)

## Part 1: grammar entropy

Treat the 196 interior pieces as reusable tiles and measure how fast an n-wide
matched strip can grow. The crate builds the row-transfer matrix for widths 1 and
2 and finds its dominant eigenvalue by power iteration, giving the per-cell
entropies h(1) and h(2) exactly, plus the 1-D horizontal bound lambda_H. Widths 3
to 6 are carried as constants from the stdlib script
[`scripts/strip_entropy.js`](scripts/strip_entropy.js), which
agrees with the crate at widths 1 and 2; the output names that source. No value
of the limit h_infinity is reported, only the upper bound the smallest computed
h(n) gives (h(6) = 0.640).

## Part 2: block counts and the distinct-piece fraction (computed in-repo)

For an interior n×n block with a free outer border it counts, exactly:

- **A(n)**: colour-valid fillings with pieces reusable (the matching grammar), via
  a dense broken-profile transfer DP.
- **B(n)**: colour-valid fillings that use distinct pieces (true Eternity II), via
  a parallel distinct-counting DFS whose cost is bounded by A(n).

rho(n) = B(n)/A(n) is the realizable fraction, reported with the per-n
-ln(rho(n))/n^2. No exponent is fitted. Sampled fractions for patches four cells
wide (16 to 48 cells) are carried from two seeded runs of
[`scripts/distinct_fraction.js`](scripts/distinct_fraction.js) (seeds 1 to 6
and 101 to 106), and the 1e-3 crossing from
[`scripts/entropy_fields.py`](scripts/entropy_fields.py). An earlier version fitted an area-law exponent over n <= 3 and
extrapolated a collapse point from it; both are withdrawn (sampled fractions
fall far faster than the fit) and kept in the output under `withdrawn` with the
reason.

Exact through n = 3; n = 4 computes A(4) but B(4) is over the DFS budget (A(4) is
about 6.3e16, far past a few-minute enumeration), which is the wall. Cross-checked
at n = 2 against the in-repo `subgrid-placement-counts` reference table
(B(2) = 4 059 952).

## Run

From this directory:

```sh
cargo run --release > ../results/entropy.json
```

Deterministic; reproduces the committed `../results/entropy.json` (wall-clock
seconds aside). The carried values are produced by the scripts in
[`scripts/`](scripts/), each with its committed `.out`; `just
research-entropy-area-law-scripts` reruns them (strip eigenvalues exact, the
distinct-piece fractions sampled but seeded, so they reproduce given the seeds). The n = 3
distinct DFS dominates the runtime (about 12 minutes on 8 cores, roughly 1.3e11
nodes); everything else is seconds. The width-2 eigenvalue is a 529x529 power
iteration (about a second).
