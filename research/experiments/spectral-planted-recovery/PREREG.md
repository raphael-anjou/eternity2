# Pre-registration: spectral-planted-recovery (agenda A1)

Written and committed before any experiment code is run. The kill criterion
below is fixed; any later change goes into `PREREG_AMENDMENTS.md` with what,
why and when, and this file stays as it is.

## Hypothesis (from research/program/AGENDA.md, A1)

The puzzle was painted from one solution. Pair statistics of the piece set
(which pieces can sit next to which, weighted by colour rarity) carry a planted
signal that a spectral method can recover above chance, the way planted
partitions are recovered in community detection.

Borrowed from: planted-partition and stochastic block model recovery. The
question is whether a global, search-free statistic (leading eigenvectors of a
weighted compatibility matrix) ranks true neighbours higher than the
colour-rarity statistic it is built from.

## Kill criterion

Verbatim from the agenda:

> **Kill.** True-neighbour mean rank not better than the colour-rarity
> baseline by more than two standard errors.

Operationalisation (fixed here):

- Unit of analysis: one puzzle. For each puzzle `i` and method `M`, compute
  `NR_i(M)`, the mean over all half-edges of that puzzle of the normalised
  rank of the true planted neighbour (definitions below; lower is better).
- Paired difference per puzzle: `d_i = NR_i(rarity) - NR_i(spectral)`
  (positive means the spectral method ranks the true neighbour higher).
- Statistic: `dbar = mean(d_i)`, `SE = sd(d_i) / sqrt(n)` with the sample
  standard deviation (n - 1 denominator), `n = 200` puzzles per configuration.
- A configuration **passes** when `dbar > 2 * SE`. Otherwise it **fails**
  (the kill condition holds for that configuration).
- The verdict uses the two **primary** configurations (8x8 and 10x10 at the
  E2-faithful colour count, below):
  - both pass: **survived**;
  - both fail: **killed**;
  - one passes and one fails: **inconclusive**.
- The spectral method in the kill test is the primary one (`spectral_k`,
  k = ceil(sqrt(N)), defined below). Other k values, the combined score and
  the secondary configurations are reported but do not enter the verdict.
- Raw (unnormalised) mean rank gets the same paired test as a secondary
  statistic. If raw and normalised disagree, the normalised one decides.

## Puzzles

- Generator: `eternity2_engine::generate_solved_framed_bc(size, colors, seed,
  true, border_colors)` gives the planted solution (piece `i` at cell `i`,
  rotation 0). The scrambled puzzle the methods see is produced by exactly the
  scramble `generate_framed` applies (`XorShift::new(seed ^ 0xA5A5_A5A5)`,
  `shuffle` of the piece list, then `rotated(p, rng.below(4))` per piece), but
  applied to a list that also carries each piece's solution cell and rotation,
  so the planted solution is kept for scoring only. A unit test checks that
  with the default border count this reproduces `generate_framed` piece for
  piece. `generate_framed` itself is not called directly because it only
  exposes the default border-colour count `min(5, colors - 1)`, which at 8x8
  gives 5 border colours and 3 interior colours, far from Eternity II's ratio.
- The methods receive only the scrambled piece list (edge colours URDL). They
  never see the solution cell, rotation, or the piece order of the solved
  board.

Configurations (frame-restricted colours on, `framed = true`):

| id | size | colours | border | interior | role |
|---|---:|---:|---:|---:|---|
| n8_c8 | 8 | 8 | 2 | 6 | primary (E2-faithful) |
| n10_c12 | 10 | 12 | 3 | 9 | primary (E2-faithful) |
| n8_c14 | 8 | 14 | 2 | 12 | secondary (sparser interior) |
| n10_c21 | 10 | 21 | 3 | 18 | secondary (sparser interior) |

The primary colour counts are the hint study's density-preserving recipe
(`faithful_colors` in `engine/src/bin/hint_variants.rs`: border multiplicity
about 12, interior about 24, interior at least 3x border), which is exactly
Eternity II's 5 + 17 at 16x16. The secondary configurations keep the border
count and roughly halve the interior colour multiplicity (8x8: 84 deep seams
over 12 colours, 7 each; 10x10: 144 over 18, 8 each), giving smaller candidate
sets. 21 is the largest count at 10x10 that keeps 3 border colours within the
22-motif cap of the generator.

Seeds: puzzle seeds 1 to 200 inclusive for every configuration (200 puzzles
per configuration, 800 in total). This is the sample size in the agenda; the
compute is seconds, so no reduction is needed.

## Definitions

- Colour 0 is the grey rim. `n_c` is the number of half-edges (piece sides)
  of colour `c` in the whole piece set.
- Half-edge: a (piece `p`, side `d`) with a non-grey colour `c`. Every
  non-grey side of every piece is scored, so each planted adjacency is scored
  twice (once from each side). Per puzzle there are `4 n (n - 1)` half-edges
  (224 at 8x8, 360 at 10x10).
- Candidate set of `(p, d)`: every (piece `q`, side `j`) with `q != p` and
  colour(q, j) = c. Each such pair is exactly one placement of `q` (the
  rotation that turns side `j` to face `p`). No other filter (no grey or
  geometry check): the candidate set is purely colour-based, `m = n_c - k_p`
  where `k_p` is the number of sides of `p` with colour `c`.
- The true neighbour is the (piece, side) that faces `(p, d)` in the planted
  solution. It is always in the candidate set.
- Rank: candidates are scored by the method (higher score = more likely);
  the true neighbour's rank `r` is 1 + (number of candidates with a strictly
  higher score) + (number of other candidates tied with it) / 2 (mid-rank).
  Two scores tie when `|a - b| <= 1e-9 * max(1, max |score| in the set)`.
- Raw rank `r`; normalised rank `r / m`.
- Also recorded per method: top-1 rate (true neighbour strictly above all
  other candidates).

## Methods

All methods are functions of the scrambled piece list only and are invariant
to piece relabelling and to piece rotation.

- **Weighted compatibility matrix** `W` (N x N, N = n^2 pieces):
  `W[p][q] = sum over sides i of p and j of q with colour(p,i) = colour(q,j)
  != 0 of 1 / n_c`, and `W[p][p] = 0`. Symmetric, non-negative.
- **Rarity baseline (the kill baseline)**: score of candidate `(q, j)` for
  half-edge `(p, d)` is `W[p][q]`, the rarity-weighted number of ways `p` and
  `q` can touch. It is the same matrix the spectral method uses, without the
  spectral step.
- **Spectral method (primary)**: eigen-decompose `W` (cyclic Jacobi, written in
  the crate, checked by reconstruction residual). Take the `k` eigenpairs with
  the largest algebraic eigenvalues and score candidate `(q, j)` by the rank-k
  reconstruction `S_k[p][q] = sum_{l=1..k} lambda_l u_l[p] u_l[q]`.
  Primary `k = ceil(sqrt(N))`: 8 at 8x8, 10 at 10x10.
- **Secondary, reported, not in the verdict**:
  - `spectral_k` for k in {1, 2, 4, 8, 16, 32} (exploratory sweep; a pass at
    some k only is not survival, given the multiple comparisons);
  - `combined`: rarity score with the primary spectral score as a tie-break
    (`W[p][q] + 1e-6 * S_k[p][q] / max|S_k|`);
  - `cand_rarity`: candidate piece's own rarity, `sum over non-grey sides j'
    of q of 1 / n_c(j')` (a second reading of "colour rarity alone");
  - `uniform`: analytic expectation `mean((m + 1) / (2m))` for the normalised
    rank and `mean((m + 1) / 2)` for the raw rank, plus a simulated uniform
    ranking (i.i.d. random scores from `XorShift::new(seed ^ 0x00C0_FFEE)`).

## Leakage checks

The main failure mode is solution order leaking into a method.

1. Re-scramble: every puzzle is also re-permuted and re-rotated with an
   independent seed (`XorShift::new(seed ^ 0x5EED_1234)`); every method is rerun
   on it. Pass: per-puzzle mean normalised ranks agree within 1e-6 for every
   method except the simulated uniform ranking. The maximum absolute
   difference is reported.
2. Positive control for the leak detector: an `id_distance` score
   (`-|index(q) - index(p)|` on the list the method receives) is run on the
   scrambled puzzle and on the unscrambled solved-order puzzle. On the solved
   order it must rank true neighbours well (it can see the layout); on the
   scrambled puzzle it must sit at the uniform expectation. This shows the
   metric would detect leaked order.
3. Each instance checks that the recovered planted solution tiles: every
   planted adjacency joins equal colours, every rim side is grey.

## Strata (secondary, interpretation only)

Border structure is a known local constraint, not a planted global signal: a
border piece's inward side shares interior colours with interior pieces, so a
method that separates border from interior pieces can down-rank impossible
candidates. To see whether any gain is only that, every statistic is also
reported on two strata of half-edges:

- `deep`: `p` and its true neighbour are both interior pieces (no grey side);
- `rim`: at least one of the two is a border or corner piece.

The verdict is on all half-edges as pre-registered. If spectral beats rarity
on all half-edges but not on `deep`, the README says the gain is explained by
the border and interior split.

## Metrics reported

Per puzzle (raw jsonl, one line per puzzle per configuration): seed, sizes,
colour census, number of half-edges, mean candidate-set size, and for each
method and stratum: mean raw rank, mean normalised rank, top-1 rate; leakage
check differences; tiling check.

Per configuration (summary json): means and SDs of the above, paired
`dbar`, `SE`, `dbar / SE` for spectral vs rarity (normalised and raw, all
strata, each k), and the pass or fail and overall verdict as defined above.

## Analysis plan

1. Build the crate, run unit tests (Jacobi, scramble reproduces
   `generate_framed`, tiling check, rank with ties).
2. Run the four configurations, 200 seeds each, single process.
3. Compute the paired test per configuration; apply the verdict rule to the
   two primary configurations.
4. Report everything in README.md, including the secondary analyses, without
   promoting any secondary result to the verdict.

## Compute budget

One session, CPU only, no paid services. Expected run time: well under ten
minutes on the VM (8 cores, 31 GB). Hard cap: 3 hours wall clock; if exceeded
the design is shrunk by amendment before results are read.

## Implementation

Rust crate in `research/experiments/spectral-planted-recovery/` with a path
dependency on `../../../engine` and `serde_json`. Deterministic given the
seeds; no timestamps inside result files. Reproduced by
`just experiments spectral-planted-recovery`.
