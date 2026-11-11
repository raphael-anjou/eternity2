# fill-order-mean-field

Checks the mean-field candidate law quoted on `/research/build/backtracking/fill-order`
against the official piece set, and computes the k-profiles (placed neighbours at
visit time) of row scan and an inward spiral on a bordered 14x14 interior.

Run from the repo root (pure Python 3, seeded, about 3 minutes on one core):

    python3 research/experiments/fill-order-mean-field/mean_field.py

Output: `results/mean-field.json`.

- `p_interior` is the sum of squared colour frequencies over the 17 interior
  colours of the 196 interior pieces (0.0589); `p_all22` is the same over all 22
  non-grey colours of the full set (0.0482). The law is `4 R p^k` with R = 196.
- `exact_mean` is the exact number of (piece, rotation) candidates for a cell with
  k placed neighbours at the start of interior fill, where each neighbour is a
  random interior piece in a random rotation (Monte Carlo over the neighbours,
  exact count over candidates).
- On a bordered board every edge is counted, so row scan and the inward spiral
  have the same k-histogram (169 cells at k=2, 26 at k=3, 1 at k=4).
