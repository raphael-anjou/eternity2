# Preregistration: frame cores (arc 2026-11-13, track D1)

Track label: **measurement**. On the real Eternity II pieces with the five
clues. Not a record attempt; no board is produced. Generated-board results are
never official Eternity II progress.

## Question

Frame-first search is priced at 10^45.9 nodes = (10^37.6 valid frames) x
(10^8.3 nodes per frame), assuming a wrong frame costs as much to refute as the
right one to confirm and that each refutation kills exactly one frame. Two
things were never measured on the real pieces:

- (a) Does a clause-learning solver, or a matching (all-different) check,
  refute a wrong full frame much more cheaply than plain depth-first search?
- (b) How local is the reason? When a wrong frame is refuted, how many of its
  60 border placements does the refutation use, how much information do they
  carry, and what does proving that partial frame unsatisfiable cost?

## Necessary-condition arithmetic (argued, fixed before any run)

A nogood over a set S of border placements covers the valid frames that agree
with the frame on S, a fraction 2^-b(S) of all valid frames, where

    b(S) = log2( N_F / N_F(S) ),

N_F = number of valid frames, N_F(S) = number of valid frames that agree with
the frame on S. Both are estimated by the same sequential Monte Carlo counter
(section Methods). If every frame needs a nogood of b bits that costs C nodes
to prove, covering all frames needs at least 2^b nogoods, so the price is at
least about b + log2 C bits. With frames whose cheapest known nogood differs,
the bound is a mean, price >= E_f[ C_f 2^(b_f) ] over uniform frames f.

Restricted to the stratum H of frames that pass the four clue-block tests
(below), a nogood still covers at most N_F 2^-b frames of H, so covering H
needs at least N_H / (N_F 2^-b) = 2^(b - beta_H) nogoods, where
beta_H = log2(N_F / N_H) (about 17.2 bits by the frame-information memo).
The per-frame quantity for H is therefore b + log2 C - beta_H.

Row-scan price with five clues: 10^40.46 nodes = 2^134.4 (Track 10, judged).
A frame search driven by nogoods can only beat it if b + log2 C (adjusted for
H) is below 134.4 bits with real margin. A full-frame core has b of about 125
and C of about 2^27.6, total about 152.6 bits (the 10^45.9). **This is a lower
bound on any nogood covering, not a method**: overlap between nogoods is not
measured, and passing it shows only that the necessary condition is met.

Expected number of solutions per frame: the five-clue puzzle has about 1
solution (first-moment ensemble estimate, `/research/build/analysis/solution-counting`)
and 10^37.608 valid frames (exact count, `frame-count-corners`). A uniformly
sampled valid frame is right with probability about 10^-37.6, an H frame about
10^-32.4. Every sampled frame is treated as wrong; a `sat` verdict on a real
frame would be verified and reported as a solution.

## Populations and sizes

Real puzzle: `single-core-benchmark/engine/data/puzzles/size_16_official_eternity.csv`
with its five clue columns (the file Track 10 used).

Null boards: `e2_core::generator::generate_solved_framed_bc(16, 22, seed, true, 5)`
(the repo's framed generator: 5 frame colours, 17 interior colours, the real
colour structure), seeds **1101, 1102, 1103**. Piece ids and rotations are
scrambled with a seeded permutation (`fc gen`). The five clues are the planted
pieces at the five official clue cells. Their frames are wrong frames of a
board that has a planted solution.

Strata:

- **U**: valid frames (60 border cells, each border piece once, grey outward,
  all 60 border-to-border joints matching), sampled approximately uniformly.
- **H**: valid frames that also pass the four clue-block tests (each 3-cell
  interior block next to a corner clue has a fill with distinct free interior
  pieces). H is where the frame-first cost sits: the memo measured that U
  frames failing these tests die in about 10 nodes.

Sampler (`fc sample`): sequential Monte Carlo over the ring, 5000 particles,
uniform choice among fitting pieces, systematic resampling at every step, the
clue-block test applied as a zero weight in stratum H, one frame drawn per
independent run. This is sampling-importance-resampling: approximately uniform
(bias of order 1/particles), not exactly uniform.

Test sizes and sample seeds:

| set | frames | seed |
|---|---|---|
| real H (primary) | 60 | 20261113 |
| real U | 30 | 20261114 |
| null board 1101 / 1102 / 1103, H | 20 each (60) | 20261121 / 20261122 / 20261123 |
| null board 1101 / 1102 / 1103, U | 10 each (30) | 20261131 / 20261132 / 20261133 |

Calibration (disjoint, already run, informs caps only): real frames from seeds
9001 to 9005 and 9101 to 9102, null board seed 901 with frame seeds 9103 and
9104. No calibration frame enters the test.

## Refuters (single thread each)

- **R0-rows**, **R0-spiral**: plain depth-first interior fill, every free
  interior piece in every distinct rotation, row order or inward spiral. One
  node per assignment that agrees with every assigned neighbour.
- **R1-rows**, **R1-spiral**: R0 plus, after every assignment, a matching
  check: the unfilled cells that touch a filled cell must have a system of
  distinct representatives among unused pieces (Kuhn augmenting paths). Cells
  that touch nothing accept every piece, so this is exactly the perfect
  matching (Hall) condition on remaining cells and pieces.
- **R2**: Glucose 4.2.1 through PySAT 1.9.dev15 (nixpkgs), direct encoding
  documented at the top of `scripts/sat.py` (placement variables, joint colour
  variables, placement implies joint colours, cell at-least-one, piece
  exactly-one by a sequential counter). The frame is passed as 60 assumption
  literals, never as clauses. CaDiCaL was the first choice; PySAT cannot
  interrupt it, so Glucose is the one solver.

Caps: R0 and R1 120 s wall per frame. R2: refutation 600 s; core minimisation
300 s per frame in total, 3 s per solver call; proof of the minimised core
alone 600 s. A capped run is censored, never a tie and never a refutation.

## Stage 0: validity (generated boards, before any test frame)

- (i) True frame given: 3 planted 10x10 boards (`generate_solved_framed`, 14
  colours, seeds 801 to 803, no clue) and 3 planted 16x16 boards (22 colours,
  seeds 801 to 803, five clues). Cap 600 s. No refuter may answer `unsat`.
  Every `sat` must pass the independent checker `scripts/common.py
  check_board`. A capped confirm is reported as a censored confirm cost.
- (ii) Small boards: 6x6 (10 colours, 4 frame colours) and 7x7 (12 colours, 4
  frame colours), seeds 1 to 4 each. `scripts/brute.py` enumerates every
  solution independently. Per board: 30 sampled frames plus every solution
  frame. Every refuter: `unsat` only on frames outside the brute-force set,
  `sat` only on frames inside it with a verified board. Every nogood (R2
  minimised core, R0 read set) is checked: no brute-force solution agrees with
  it.
- (iii) R2 against R0-rows on every decided small instance (at least 200):
  zero disagreements.
- Counter: on the 6x6 boards the SMC estimate of N_F(S) for every nogood and
  for S empty is compared with exact enumeration (`scripts/framecount.py`):
  median absolute error at most 0.05 and maximum at most 0.3 in log10.
  On the real pieces the SMC N_F must be within 0.05 of log10 of the exact
  40519056449429419697492635191544381440.

A refuter that fails a check is dropped from stages 1 and 2, not patched after
test runs. If the counter check fails, every b(S) is reported as unvalidated
and stage 2 has no reading.

## Stage 1: cost of one refutation

Per frame, t_X = wall seconds of refuter X on the full frame (R0, R1: the
search; R2: the solve call with the puzzle CNF already loaded; CNF encode and
load times are reported separately). Best plain P = min(t_R0-rows,
t_R0-spiral), censored only when both are censored.

Ratio r_X = t_P / t_X, with censoring:

| P | X | r_X |
|---|---|---|
| decided | decided | t_P / t_X |
| censored | decided | 120 / t_X (a lower bound) |
| decided | censored | t_P / cap_X (an upper bound) |
| censored | censored | 1, counted and reported as censored pairs |

Statistic: the lower median of r_X over the frames of a set (order statistic
floor((n+1)/2) of the ascending ratios), 95% interval by percentile bootstrap
over frames, 10,000 resamples, seed 20261113.

**Kill line (fixed by the lead), evaluated on real H:** cheap refutation of
full frames is dead if no X in {R1-rows, R1-spiral, R2} has median r_X >= 100
with the lower end of its interval >= 10. The same statistic is reported, with
no reading, on real U and on the null sets.

## Stage 2: locality of the reason

Nogood sources for a refuted frame:

- the R2 minimised core: failed assumptions of the refutation, then
  deletion-based minimisation (drop a placement when the rest is proved
  unsatisfiable within the per-call cap; keep it otherwise). Every core is a
  proved nogood. Placements kept because the call was capped are flagged
  `undecided`, so the core size is an upper bound;
- the R0-rows and R0-spiral read sets: border cells adjacent to an interior
  cell the search reached. The same tree refutes every frame that agrees on
  them, so they are nogoods proved at the run's own node count. R1 is not a
  source (its matching check reads every cell that touches the frame).

Per nogood S: size, b(S) as above (SMC, 20,000 particles, 3 runs, mean of the
estimates of N; N_F and N_H with 10 runs), and C(S):

- R0 read set: the run's nodes.
- R2 core: seconds of a fresh solver proving the core alone unsatisfiable
  (border cells outside the core and the interior free), times r, where r is
  the median node rate (nodes per second) of the R0-rows runs that hit their
  cap on the same puzzle class (real or null) in the test stage, same host and
  load. If that proof is capped, the R2 core is not used.

v(S) = b(S) + log2 C(S). Per frame v_f = the minimum over its sources; the
frame's core is the minimising nogood. A frame with no source has v_f = +inf
(censored). Median: mean of the two middle order statistics (infinite if
either is).

- **Primary, real H:** M_H = median_f(v_f) - beta_H, beta_H from the same
  counter on the same puzzle.
- Real U: M_U = median_f(v_f), no adjustment.
- Null sets: the same, per board and pooled.

95% intervals: percentile bootstrap over frames, 10,000 resamples, seed
20261113 (beta_H held fixed).

**Reading (fixed by the lead), on real H:** if M_H > 134.4 bits, or the median
core size is 54 cells or more, the nogood route fails its necessary condition
and is dead as far as this measurement reaches. If M_H < 120 with the upper
end of its interval below 134.4, it passes the necessary condition, and the
README says exactly that and no more (a lower bound on a covering is not a
method; overlap between nogoods is unmeasured). Otherwise undecided.

Also reported per frame, with no reading: core size, whether the core cells
form one contiguous arc of the ring, the sides it touches, whether it contains
a corner cell, its ring distance to the nearest corner, and the R2 conflict
counts. Secondary (argued, a lower bound only): L_H = log2(mean_f 2^(v_f)) -
beta_H with each censored frame entered at log2(600 r), its b taken as 0.

## Compute

Host dell (12 threads), at most 10 used, `xargs -P 10`, one thread per job.
This Mac: editing and git only. Estimated: stage 0 about 1 hour, test stage
about 2.5 hours. If the test stage is still running at 13:45 local, unfinished
jobs are stopped and their frames count as censored for that refuter; this is
reported.

## Outputs

`results/` holds the raw JSON lines of every refuter run, the nogood counts,
`run-log.jsonl` from `common/prereg_guard.py`, and `summary.json` from
`scripts/analyze.py`. Every number in the README traces to one of them.
