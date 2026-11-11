# spectral-planted-recovery (agenda A1)

Pre-registered cheap test of agenda entry A1: do the leading eigenvectors of a
rarity-weighted piece compatibility matrix rank the planted neighbour of each
half-edge higher than the colour-rarity statistic they are built from?

- Pre-registration: [`PREREG.md`](PREREG.md) (committed before any code ran).
- Amendments: [`PREREG_AMENDMENTS.md`](PREREG_AMENDMENTS.md) (a 5-seed smoke
  run before the full run, and one added eigenvalue-gap diagnostic; the kill
  criterion and design are unchanged).
- Code: [`src/main.rs`](src/main.rs), a Rust crate with a path dependency on
  `../../../engine` (generator) and `serde_json`.
- Results: [`results/summary.json`](results/summary.json) and one jsonl line per
  puzzle in `results/raw/<config>.jsonl`.

## Outcome against the kill criterion

Kill criterion (verbatim): "True-neighbour mean rank not better than the
colour-rarity baseline by more than two standard errors."

Self-assessment: **killed.** At both primary configurations the spectral
method ranks the planted neighbour *worse* than the rarity baseline, by 5.9
and 7.9 paired standard errors, and it is also worse at both secondary
configurations. The verdict rule in PREREG.md (pass needs
`dbar > 2 SE` at both primary configurations) gives `killed`; the judge
decides.

## What was run

For each configuration, puzzle seeds 1 to 200:

1. `generate_solved_framed_bc(size, colours, seed, true, border)` gives the
   planted solution; the crate applies the exact scramble of
   `generate_framed` (unit-tested piece for piece against it) while keeping
   each piece's solution cell and rotation for scoring only.
2. On the scrambled piece list: `W[p][q] = sum of 1/n_c over side pairs of p
   and q with the same non-grey colour c` (n_c = half-edges of colour c).
3. For each of the `4n(n-1)` non-grey half-edges, the candidates are every
   (other piece, side) with the same colour. Each method scores the
   candidates; the planted neighbour's mid-rank `r` and normalised rank `r/m`
   are recorded.
4. Methods: `rarity` (W entry, the kill baseline), `spectral_primary`
   (rank-k reconstruction from the k largest eigenpairs of W, k = ceil(sqrt N):
   8 at 8x8, 10 at 10x10), exploratory `spectral_k{1,2,4,8,16,32}`,
   `combined` (rarity with spectral tie-break), `cand_rarity` (candidate
   piece's own colour rarity), `uniform_sim`, the analytic uniform expectation,
   and `id_distance` (piece index distance, the leak detector's positive
   control).
5. Leakage checks: every method rerun after an independent re-permutation and
   re-rotation; methods run on the unscrambled solved order; tiling check of the
   planted solution.

| id | size | colours (border + interior) | role | half-edges per puzzle | mean candidates m |
|---|---:|---|---|---:|---:|
| n8_c8 | 8 | 8 (2 + 6) | primary, E2-faithful ratio | 224 | 26.56 |
| n10_c12 | 10 | 12 (3 + 9) | primary, E2-faithful ratio | 360 | 29.11 |
| n8_c14 | 8 | 14 (2 + 12) | secondary, sparser interior | 224 | 16.23 |
| n10_c21 | 10 | 21 (3 + 18) | secondary, sparser interior | 360 | 16.43 |

The primary colour counts are the hint study's `faithful_colors` recipe
(border multiplicity about 12, interior about 24; it gives E2's 5 + 17 at
16x16).

## Commands

From the repo root (needs `just` and a Rust toolchain):

    just experiments spectral-planted-recovery

which runs, from `research/experiments/`:

    cargo test --release --manifest-path spectral-planted-recovery/Cargo.toml
    cargo run --release --manifest-path spectral-planted-recovery/Cargo.toml -- \
        --out spectral-planted-recovery/results --seeds 200

Output is deterministic given the seeds (no timestamps in result files). A
fresh clone of the branch (commit 1b4206e) ran the recipe in 4 min 25 s
(tests, build and run) and reproduced `results/` byte for byte (`diff -r`).

## Hardware and wall clock

One process, single-threaded, on an 8 vCPU Intel Xeon VM with 31 GB RAM
(2026-11). The full run took 211 s (n8_c8 168 s; the other three 5 to 21 s
each). The unit tests take about 38 s in release mode, plus compile time.

## Results

Normalised rank `r/m` of the planted neighbour, mean over 200 puzzles of the
per-puzzle mean over all half-edges (lower is better; uniform ranking gives
`mean((m+1)/(2m))`, a bit above 0.5). Kill test columns: paired
`d_i = NR_i(rarity) - NR_i(spectral_primary)`, SE = sd(d)/sqrt(200).

| config | role | uniform (analytic) | uniform (sim) | rarity | spectral_primary | combined | dbar | SE | dbar/SE | pass |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| n8_c8 | primary | 0.5188 | 0.5205 | 0.5263 | 0.5310 | 0.5309 | -0.0047 | 0.0006 | -7.94 | no |
| n10_c12 | primary | 0.5174 | 0.5183 | 0.5251 | 0.5286 | 0.5284 | -0.0035 | 0.0006 | -5.91 | no |
| n8_c14 | secondary | 0.5340 | 0.5338 | 0.5427 | 0.5483 | 0.5476 | -0.0055 | 0.0011 | -5.11 | no |
| n10_c21 | secondary | 0.5313 | 0.5326 | 0.5401 | 0.5439 | 0.5436 | -0.0037 | 0.0009 | -4.27 | no |

Raw mean rank (secondary statistic, same paired test) agrees in sign and
verdict:

| config | uniform | rarity | spectral_primary | dbar | SE | dbar/SE |
|---|---:|---:|---:|---:|---:|---:|
| n8_c8 | 13.780 | 13.980 | 14.106 | -0.126 | 0.016 | -7.94 |
| n10_c12 | 15.056 | 15.297 | 15.393 | -0.095 | 0.017 | -5.66 |
| n8_c14 | 8.615 | 8.698 | 8.796 | -0.097 | 0.022 | -4.50 |
| n10_c21 | 8.716 | 8.849 | 8.911 | -0.063 | 0.015 | -4.20 |

Neither rarity nor the spectral method beats uniform ranking on all
half-edges: both are worse than the analytic uniform expectation at every
configuration (rarity by 5.6 to 8.3 paired SE, spectral by 8.1 to 10.2;
`paired_vs_uniform_analytic` in the summary). In absolute terms every method
sits within 0.02 of uniform: the planted neighbour lands near the middle of
its candidate list.

### Strata (interpretation only, pre-registered)

`deep`: the half-edge's piece and its planted neighbour are both interior
pieces. `rim`: at least one of them is a border or corner piece.

| config | stratum | uniform | rarity | spectral_primary | cand_rarity | dbar (rarity - spectral) | dbar/SE |
|---|---|---:|---:|---:|---:|---:|---:|
| n8_c8 | deep | 0.5189 | 0.4554 | 0.4476 | 0.4448 | +0.0078 | 13.67 |
| n8_c8 | rim | 0.5188 | 0.6081 | 0.6273 | 0.6023 | -0.0192 | -17.75 |
| n10_c12 | deep | 0.5163 | 0.4805 | 0.4732 | 0.4607 | +0.0072 | 10.60 |
| n10_c12 | rim | 0.5193 | 0.5986 | 0.6197 | 0.6098 | -0.0212 | -19.23 |
| n8_c14 | deep | 0.5392 | 0.5038 | 0.4868 | 0.4664 | +0.0169 | 13.34 |
| n8_c14 | rim | 0.5281 | 0.5877 | 0.6191 | 0.6101 | -0.0314 | -18.47 |
| n10_c21 | deep | 0.5337 | 0.5163 | 0.5005 | 0.4804 | +0.0159 | 13.88 |
| n10_c21 | rim | 0.5274 | 0.5794 | 0.6154 | 0.6141 | -0.0360 | -25.93 |

Reading (an interpretation, not part of the verdict): every method that sees
W, and `cand_rarity`, which scores a candidate only by its own colours,
does better than uniform on `deep` half-edges and worse on `rim` ones. W
grows with the number of non-grey sides a candidate has, so it prefers
interior pieces: right for deep half-edges, wrong for a border piece's
along-frame sides and for the inward sides whose planted neighbour is a
border piece. The spectral method sharpens this piece-class preference
(better than rarity on `deep`, worse on `rim`, net worse). On `deep` it does
not beat `cand_rarity`, a score with no pair information at all, at any
configuration. So the gain on `deep` is accounted for by the border and
interior split, the outcome PREREG.md named in advance, and no pair-level
planted signal shows up beyond it in this test.

### Exploratory k sweep (not in the verdict)

`dbar` (rarity minus spectral, normalised, all half-edges) and dbar/SE:

| config | k=1 | k=2 | k=4 | k=8 | k=16 | k=32 |
|---|---|---|---|---|---|---|
| n8_c8 | +0.0057 (4.3) | -0.0087 (-5.9) | -0.0067 (-5.7) | -0.0047 (-7.9) | -0.0036 (-6.0) | -0.0010 (-1.5) |
| n10_c12 | +0.0062 (5.6) | -0.0038 (-3.2) | -0.0023 (-2.1) | -0.0042 (-5.5) | -0.0033 (-6.8) | -0.0033 (-6.3) |
| n8_c14 | +0.0063 (5.3) | -0.0052 (-3.7) | -0.0027 (-1.9) | -0.0055 (-5.1) | -0.0074 (-8.8) | -0.0049 (-5.6) |
| n10_c21 | +0.0070 (6.6) | -0.0013 (-1.1) | +0.0022 (1.7) | -0.0023 (-2.4) | -0.0061 (-8.6) | -0.0060 (-8.5) |

k = 1 beats rarity at all four configurations. This does not rescue the
hypothesis: the rank-1 reconstruction `lambda_1 u_1[p] u_1[q]` ranks a
fixed piece's candidates by the Perron vector entry `u_1[q]` alone, a
per-piece centrality with no pair information, and it lands at the uniform
expectation (n8_c8: 0.5206 against 0.5188 uniform; n10_c12: 0.5188 against
0.5174), like `cand_rarity` (0.5179, 0.5170). It beats rarity only because
rarity is below uniform. Choosing k after seeing this table would also be
the multiple-comparison problem PREREG.md rules out.

### Top-1 rate (secondary)

Fraction of half-edges where the planted neighbour scores strictly above
every other candidate: rarity 0.003 to 0.016, spectral_primary 0.014 to
0.039, uniform_sim 0.036 to 0.069. With constant per-band weights, rarity
scores take few distinct values and the planted neighbour usually ties with
other candidates, so strict top-1 is low for it by construction (mid-rank
handles the ties in the main statistic); neither method reaches the uniform
top-1 rate.

## Leakage checks

| config | re-scramble max diff, rarity | spectral_primary | spectral_k16 | spectral_k32 | puzzles with zero gap at k16 / k32 | id_distance, solved order | id_distance, scrambled | id_distance, re-scrambled |
|---|---:|---:|---:|---:|---|---:|---:|---:|
| n8_c8 | 6.7e-16 | 6.7e-16 | 0.036 | 0.018 | 38 / 42 | 0.1574 | 0.5214 | 0.5186 |
| n10_c12 | 1.1e-15 | 8.9e-16 | 0.032 | 0.023 | 171 / 44 | 0.1310 | 0.5163 | 0.5177 |
| n8_c14 | 7.8e-16 | 6.7e-16 | 0.000 | 0.010 | 0 / 12 | 0.1832 | 0.5369 | 0.5333 |
| n10_c21 | 1.2e-15 | 1.1e-15 | 0.000 | 0.020 | 0 / 27 | 0.1550 | 0.5304 | 0.5311 |

- Re-scramble: rarity, spectral_primary, combined, cand_rarity and
  spectral_k{1,2,4,8} agree within 1.4e-15 on every puzzle (pass, threshold
  1e-6). spectral_k16 and spectral_k32 fail the 1e-6 threshold on some
  puzzles, and only at the k where some puzzles have
  `lambda_k = lambda_{k+1}` (zero gap): the rank-k reconstruction is then
  not unique and Jacobi returns an input-order-dependent basis of the
  degenerate eigenspace. That is numerical non-uniqueness of those two
  exploratory scores, not leaked order. The primary k has a nonzero gap on
  every puzzle (minimum 0.0029).
- Positive control: `id_distance` ranks the planted neighbour at 0.13 to 0.18
  on the solved order (where piece index encodes the layout) and at the
  uniform level (0.52 to 0.54) on the scrambled and re-scrambled puzzles, so
  the metric would have exposed leaked order. Rarity and spectral_primary give
  the same values on the solved order as on the scrambled puzzles (they are
  invariant to relabelling).
- The planted solution tiles (all adjacencies match, rim grey) for all 800
  puzzles before and after both scrambles.

## Limits

- One spectral construction (rank-k reconstruction of one rarity-weighted
  matrix, largest algebraic eigenvalues) was tested; normalised Laplacians,
  half-edge-level matrices, non-backtracking or Bethe Hessian operators were
  not. The result says this method recovers no planted signal here, not that
  no spectral method can.
- Two sizes (8x8, 10x10) and two colour counts per size, all from the engine's
  framed generator. Its painting gives every colour the same half-edge count
  within each band, identical across all 200 puzzles of a configuration
  (`colour_census_half_edges` in the raw files: 28 per colour at n8_c8; 24
  per border and 32 per interior colour at n10_c12; 28 and 14 at n8_c14; 24
  and 16 at n10_c21). The rarity weights are therefore one constant per band,
  so `W` is a two-weight colour-match count on these puzzles. A painter with
  unequal colour frequencies could behave differently. The official 16x16
  set was not used.
- The candidate set is colour-only, as pre-registered; it ignores the grey-side
  geometry that rules out many rim candidates directly.
