# Score-bound gap (stage 0 of a relaxed-decision-diagram score bound)

Night shift 2026-11-12, Track 8. Label: **measurement**. Preregistration:
`PREREG.md` (commit cd042da, committed before any test seed ran).
Generated-board results are never official Eternity II progress.

## Question

The record engines for the 16x16 board run a break-tolerant DFS that allows
mismatches only where a hand-made schedule says so; the repo's `BreakSolver`
works the same way and carries no score bound. Can a search prove that a
prefix can no longer reach a target score? This experiment measures only the
first stage: how tight one candidate bound is. No solver is built and no
search gain is claimed.

The bound is a width-W relaxed decision diagram (DD) over the remaining cells
in row-major order. Its states keep the frontier colours (relaxed to colour
sets when merged) and the used pieces (relaxed to "used on every path" and
"used on some path" when merged). It is a valid upper bound on the best total
score (matched-edges convention) of any completion of the prefix, for every W;
the proof is in `PREREG.md`. Controls: W = 1 and the trivial bound "prefix
score plus every remaining edge". The truth is the exact best completion
computed by an independent iterative-deepening search (`compute/src/oracle.rs`,
no code shared with the DD).

## Outcome against the preregistered criteria (self-assessment; a separate judge decides)

**Soundness gate: passed.** No prefix with a finished oracle had a bound below
its true best completion, at any W, and no bound exceeded the trivial bound
(8,063 rows; `soundness_violation_rows` = 0 and
`soundness_recheck_failures` = 0 in `results/summary.json`, the second
recomputed from the raw numbers). Before any test seed, `sbg selftest 40`
matched the oracle against an unpruned exhaustive search on 240 small-board
prefixes, and the unmerged diagram equalled the exhaustive optimum on all of
them.

**Decision stratum** (7x7 critical, c = 6, 3 rows remaining, 240 prefixes,
both sources pooled, oracle coverage 239/240):

| W | 1 | 64 | 512 | 4096 | trivial |
|---|---|---|---|---|---|
| median gap (M_hi = M_lo) | 7 | 5 | 5 | 4 | 7 |

- **K1 (killed at stage 0 if the median gap is above 4 at every W): does not
  fire.** M_hi(4096) = M_lo(4096) = 4. The pass is at the threshold, not
  clear of it: Q1 to Q3 is 4 to 5, 69% of prefixes have a gap of at most 4
  and 3% a gap of at most 2, none has a gap of 0 (exploratory bootstrap 95% CI
  of the median: [4, 4]).
- **K2 (the diagram adds nothing if W = 4096 is not better than W = 1 by
  more than 1 edge): does not fire.** Median gap 7 at W = 1 against 4 at
  W = 4096, a difference of 3 under both censoring variants.

The W = 1 control equals the trivial bound at the median on every stratum
with 2 or more rows left (the median of trivial minus W = 1 bound is 0), so
"3 edges better than the control" means 3 edges better than no bound at all.

So the idea survives stage 0 at 7x7, narrowly. The trend with board size
(below) is the part that bears on larger boards, and it goes the wrong way.

### Gap by rung and rows remaining

Median gap (bound minus truth) per W, M_hi with M_lo in brackets when they
differ ("inf" when more than half the prefixes are censored). "cells left" is
r times n; "gap per cell left" divides the W = 4096 median by it. Quartiles
are over finished prefixes. All values are pooled over the two prefix sources.

| rung | r | cells left | prefixes | coverage | W=1 | W=64 | W=512 | W=4096 | trivial | W=4096 Q1-Q3 (finished) | W=4096 gap per cell left | W=4096 gap 0 | W=4096 gap <= 2 | W=4096 transitions (median) | W=4096 ms (median) | W=4096 frontier-relax share |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| n6-c5 | 1 | 6 | 240 | 1.000 | 2 | 0 | 0 | 0 | 5 | 0-0 | 0.000 | 1 | 1 | 104 | 0.01 | 0.00 |
| n6-c5 | 2 | 12 | 240 | 1.000 | 6 | 3 | 2 | 1 | 7 | 0-1 | 0.083 | 0.36 | 0.99 | 82792.5 | 7.32 | 0.12 |
| n6-c5 | 3 | 18 | 240 | 1.000 | 6 | 4 | 4 | 3 | 6 | 3-4 | 0.167 | 0 | 0.24 | 743700.5 | 78.87 | 0.21 |
| n6-c5 | 4 | 24 | 240 | 0.896 | 5 | 4 | 4 | 4 | 5 | 3-4 | 0.167 | 0 | 0.05 | 1981169 | 197.13 | 0.22 |
| n6-c5 | 5 | 30 | 240 | 0.871 | 3 | 3 | 3 | 3 | 3 | 2-3 | 0.100 | 0.02 | 0.28 | 3747647 | 323.09 | 0.20 |
| n6-c7 | 1 | 6 | 216 | 1.000 | 0 | 0 | 0 | 0 | 0 | 0-0 | 0.000 | 1 | 1 | 104 | 0 | 0.00 |
| n6-c7 | 2 | 12 | 230 | 1.000 | 6 | 2 | 1 | 1 | 7 | 0-1 | 0.083 | 0.48 | 0.97 | 96163.5 | 9.08 | 0.21 |
| n6-c7 | 3 | 18 | 240 | 1.000 | 8 | 5 | 4 | 3.5 | 8 | 3-4 | 0.194 | 0.08 | 0.22 | 801711.5 | 76.03 | 0.30 |
| n6-c7 | 4 | 24 | 240 | 0.487 | inf [7] | inf [5] | inf [5] | inf [4] | inf [7] | 3-4 | inf | 0.03 | 0.05 | 2072662.5 | 193.39 | 0.31 |
| n6-c7 | 5 | 30 | 240 | 0.379 | inf [5] | inf [4] | inf [4] | inf [4] | inf [5] | 2-4 | inf | 0.04 | 0.12 | 3914922 | 352.22 | 0.31 |
| n7-c6 | 1 | 7 | 232 | 1.000 | 2 | 0 | 0 | 0 | 6 | 0-0 | 0.000 | 1 | 1 | 316 | 0.01 | 0.00 |
| n7-c6 | 2 | 14 | 240 | 1.000 | 7 | 4 | 3 | 2 | 8 | 1-3 | 0.143 | 0.03 | 0.66 | 217344 | 30.8 | 0.28 |
| n7-c6 | 3 | 21 | 240 | 0.996 | 7 | 5 | 5 | 4 | 7 | 4-5 | 0.190 | 0 | 0.03 | 1339740.5 | 195.43 | 0.31 |
| n7-c6 | 4 | 28 | 240 | 0.287 | inf [5] | inf [5] | inf [5] | inf [4] | inf [5] | 4-4 | inf | 0 | 0 | 3313284.5 | 384.03 | 0.29 |
| n7-c6 | 5 | 35 | 240 | 0.167 | inf [4] | inf [3] | inf [3] | inf [3] | inf [4] | 3-4 | inf | 0 | 0.01 | 6091552 | 629.98 | 0.26 |
| n7-c6 | 6 | 42 | 239 | 0.117 | inf [2] | inf [2] | inf [2] | inf [2] | inf [2] | 2-2 | inf | 0.02 | 0.11 | 9757718 | 939.18 | 0.23 |
| n7-c8 | 1 | 7 | 149 | 1.000 | 0 | 0 | 0 | 0 | 0 | 0-0 | 0.000 | 1 | 1 | 316 | 0.03 | 0.00 |
| n7-c8 | 2 | 14 | 237 | 1.000 | 8 | 4 | 3 | 2 | 8 | 0-3 | 0.143 | 0.27 | 0.67 | 229917 | 56.05 | 0.35 |
| n7-c8 | 3 | 21 | 240 | 0.900 | 9 | 7 | 6 | 5 | 9 | 4-5 | 0.238 | 0 | 0.01 | 1338999 | 426.33 | 0.39 |
| n7-c8 | 4 | 28 | 240 | 0.013 | inf [6] | inf [5] | inf [4] | inf [4] | inf [7] | 4-4 | inf | 0 | 0 | 3332666 | 1185.79 | 0.37 |
| n7-c8 | 5 | 35 | 240 | 0.004 | inf [5] | inf [4] | inf [4] | inf [4] | inf [5] | 4-4 | inf | 0 | 0 | 6192131 | 2303.95 | 0.35 |
| n7-c8 | 6 | 42 | 240 | 0.017 | inf [4] | inf [4] | inf [4] | inf [4] | inf [4] | 2-3 | inf | 0 | 0.01 | 9909506.5 | 3632.82 | 0.32 |
| n8-c8 | 1 | 8 | 53 | 1.000 | 2 | 1 | 0 | 0 | 7 | 0-0 | 0.000 | 1 | 1 | 854 | 0.07 | 0.00 |
| n8-c8 | 2 | 16 | 240 | 1.000 | 9 | 5 | 5 | 4 | 10 | 3-4 | 0.250 | 0 | 0.09 | 418329.5 | 109.09 | 0.39 |
| n8-c8 | 3 | 24 | 240 | 0.362 | inf [8] | inf [6] | inf [6] | inf [5] | inf [8] | 5-5 | inf | 0 | 0 | 2007849.5 | 788.62 | 0.41 |
| n8-c8 | 4 | 32 | 240 | 0.000 | inf [6] | inf [5] | inf [4] | inf [4] | inf [6] | n/a-n/a | inf | 0 | 0 | 4865495 | 2020.6 | 0.37 |
| n8-c8 | 5 | 40 | 240 | 0.000 | inf [4] | inf [4] | inf [4] | inf [4] | inf [4] | n/a-n/a | inf | 0 | 0 | 8896125.5 | 3683.93 | 0.34 |
| n8-c8 | 6 | 48 | 240 | 0.000 | inf [3] | inf [3] | inf [3] | inf [3] | inf [3] | n/a-n/a | inf | 0 | 0 | 14187972 | 5503.76 | 0.29 |
| n8-c8 | 7 | 56 | 240 | 0.000 | inf [2] | inf [2] | inf [2] | inf [2] | inf [2] | n/a-n/a | inf | 0 | 0 | 20653344 | 7628.9 | 0.26 |
| n8-c10 | 1 | 8 | 48 | 1.000 | 0 | 0 | 0 | 0 | 0 | 0-0 | 0.000 | 1 | 1 | 854 | 0.04 | 0.00 |
| n8-c10 | 2 | 16 | 179 | 1.000 | 9 | 5 | 4 | 3 | 10 | 0-4 | 0.188 | 0.27 | 0.37 | 419931 | 55.77 | 0.43 |
| n8-c10 | 3 | 24 | 240 | 0.104 | inf [9] | inf [7] | inf [6] | inf [5] | inf [9] | 5-5 | inf | 0 | 0.01 | 2003035.5 | 309.89 | 0.48 |
| n8-c10 | 4 | 32 | 240 | 0.000 | inf [7] | inf [5] | inf [4] | inf [4] | inf [7] | n/a-n/a | inf | 0 | 0 | 4891981 | 817.2 | 0.44 |
| n8-c10 | 5 | 40 | 240 | 0.000 | inf [6] | inf [4] | inf [4] | inf [3] | inf [6] | n/a-n/a | inf | 0 | 0 | 9011674.5 | 1358.58 | 0.40 |
| n8-c10 | 6 | 48 | 240 | 0.000 | inf [5] | inf [4] | inf [4] | inf [4] | inf [5] | n/a-n/a | inf | 0 | 0 | 14351986.5 | 2078.81 | 0.37 |
| n8-c10 | 7 | 56 | 240 | 0.013 | inf [4] | inf [4] | inf [4] | inf [4] | inf [4] | 0-3 | inf | 0 | 0 | 21020856.5 | 2854.77 | 0.34 |

### Pruning of dead prefixes (rows remaining 2 and 3)

Share of prefixes whose truth is below the target that the bound also puts
below it (opt = 2n(n-1), the planted maximum).

| rung | r | target | dead prefixes | W=1 | W=64 | W=512 | W=4096 | trivial |
|---|---|---|---|---|---|---|---|---|
| n6-c5 | 2 | opt | 240 | 0.64 | 1 | 1 | 1 | 0.5 |
| n6-c5 | 2 | opt-1 | 240 | 0.52 | 1 | 1 | 1 | 0.49 |
| n6-c5 | 2 | opt-2 | 240 | 0.39 | 0.95 | 0.99 | 0.99 | 0.3 |
| n6-c5 | 2 | opt-4 | 231 | 0.05 | 0.55 | 0.8 | 0.97 | 0 |
| n6-c5 | 3 | opt | 240 | 0.55 | 0.98 | 1 | 1 | 0.49 |
| n6-c5 | 3 | opt-1 | 240 | 0.4 | 0.73 | 0.94 | 0.98 | 0.37 |
| n6-c5 | 3 | opt-2 | 240 | 0.05 | 0.46 | 0.59 | 0.8 | 0 |
| n6-c5 | 3 | opt-4 | 226 | 0 | 0.03 | 0.12 | 0.32 | 0 |
| n6-c7 | 2 | opt | 172 | 0.78 | 1 | 1 | 1 | 0.7 |
| n6-c7 | 2 | opt-1 | 172 | 0.67 | 0.99 | 0.99 | 0.99 | 0.63 |
| n6-c7 | 2 | opt-2 | 150 | 0.61 | 0.99 | 1 | 1 | 0.47 |
| n6-c7 | 2 | opt-4 | 142 | 0.08 | 0.86 | 1 | 1 | 0 |
| n6-c7 | 3 | opt | 227 | 0.57 | 1 | 1 | 1 | 0.52 |
| n6-c7 | 3 | opt-1 | 227 | 0.42 | 0.98 | 1 | 1 | 0.41 |
| n6-c7 | 3 | opt-2 | 222 | 0.07 | 0.64 | 0.95 | 0.97 | 0 |
| n6-c7 | 3 | opt-4 | 219 | 0 | 0.14 | 0.44 | 0.63 | 0 |
| n7-c6 | 2 | opt | 240 | 0.65 | 1 | 1 | 1 | 0.5 |
| n7-c6 | 2 | opt-1 | 240 | 0.55 | 1 | 1 | 1 | 0.5 |
| n7-c6 | 2 | opt-2 | 240 | 0.5 | 0.97 | 1 | 1 | 0.48 |
| n7-c6 | 2 | opt-4 | 239 | 0.13 | 0.59 | 0.75 | 0.94 | 0 |
| n7-c6 | 3 | opt | 240 | 0.55 | 0.98 | 1 | 1 | 0.5 |
| n7-c6 | 3 | opt-1 | 240 | 0.49 | 0.72 | 0.95 | 0.99 | 0.47 |
| n7-c6 | 3 | opt-2 | 240 | 0.38 | 0.5 | 0.59 | 0.8 | 0.35 |
| n7-c6 | 3 | opt-4 | 240 | 0.01 | 0.2 | 0.35 | 0.45 | 0 |
| n7-c8 | 2 | opt | 198 | 0.73 | 1 | 1 | 1 | 0.61 |
| n7-c8 | 2 | opt-1 | 198 | 0.64 | 1 | 1 | 1 | 0.61 |
| n7-c8 | 2 | opt-2 | 186 | 0.65 | 0.99 | 1 | 1 | 0.62 |
| n7-c8 | 2 | opt-4 | 180 | 0.26 | 0.81 | 0.93 | 0.98 | 0 |
| n7-c8 | 3 | opt | 240 | 0.53 | 1 | 1 | 1 | 0.5 |
| n7-c8 | 3 | opt-1 | 240 | 0.51 | 0.97 | 1 | 1 | 0.5 |
| n7-c8 | 3 | opt-2 | 240 | 0.4 | 0.62 | 0.93 | 1 | 0.38 |
| n7-c8 | 3 | opt-4 | 240 | 0.01 | 0.41 | 0.49 | 0.52 | 0 |
| n8-c8 | 2 | opt | 240 | 0.65 | 1 | 1 | 1 | 0.5 |
| n8-c8 | 2 | opt-1 | 240 | 0.53 | 1 | 1 | 1 | 0.5 |
| n8-c8 | 2 | opt-2 | 240 | 0.52 | 1 | 1 | 1 | 0.5 |
| n8-c8 | 2 | opt-4 | 240 | 0.4 | 0.69 | 0.88 | 1 | 0.37 |
| n8-c8 | 3 | opt | 240 | 0.53 | 1 | 1 | 1 | 0.5 |
| n8-c8 | 3 | opt-1 | 240 | 0.5 | 0.93 | 1 | 1 | 0.5 |
| n8-c8 | 3 | opt-2 | 240 | 0.5 | 0.56 | 0.78 | 0.97 | 0.5 |
| n8-c8 | 3 | opt-4 | 240 | 0.03 | 0.48 | 0.5 | 0.5 | 0 |
| n8-c10 | 2 | opt | 143 | 0.88 | 1 | 1 | 1 | 0.84 |
| n8-c10 | 2 | opt-1 | 143 | 0.85 | 0.99 | 0.99 | 0.99 | 0.84 |
| n8-c10 | 2 | opt-2 | 134 | 0.9 | 0.99 | 0.99 | 0.99 | 0.9 |
| n8-c10 | 2 | opt-4 | 128 | 0.84 | 0.98 | 0.98 | 0.98 | 0.72 |
| n8-c10 | 3 | opt | 239 | 0.55 | 1 | 1 | 1 | 0.5 |
| n8-c10 | 3 | opt-1 | 239 | 0.5 | 1 | 1 | 1 | 0.5 |
| n8-c10 | 3 | opt-2 | 239 | 0.49 | 0.79 | 0.98 | 1 | 0.49 |
| n8-c10 | 3 | opt-4 | 238 | 0.05 | 0.51 | 0.51 | 0.57 | 0 |

### Trend with board size (median gap, M_hi)

| track | r | W | n=6 | n=7 | n=8 |
|---|---|---|---|---|---|
| critical | 1 | 1 | 2 | 2 | 2 |
| critical | 1 | 64 | 0 | 0 | 1 |
| critical | 1 | 512 | 0 | 0 | 0 |
| critical | 1 | 4096 | 0 | 0 | 0 |
| critical | 1 | trivial | 5 | 6 | 7 |
| critical | 2 | 1 | 6 | 7 | 9 |
| critical | 2 | 64 | 3 | 4 | 5 |
| critical | 2 | 512 | 2 | 3 | 5 |
| critical | 2 | 4096 | 1 | 2 | 4 |
| critical | 2 | trivial | 7 | 8 | 10 |
| critical | 3 | 1 | 6 | 7 (cov 0.996) | inf (cov 0.362) |
| critical | 3 | 64 | 4 | 5 (cov 0.996) | inf (cov 0.362) |
| critical | 3 | 512 | 4 | 5 (cov 0.996) | inf (cov 0.362) |
| critical | 3 | 4096 | 3 | 4 (cov 0.996) | inf (cov 0.362) |
| critical | 3 | trivial | 6 | 7 (cov 0.996) | inf (cov 0.362) |
| critical+2 | 1 | 1 | 0 | 0 | 0 |
| critical+2 | 1 | 64 | 0 | 0 | 0 |
| critical+2 | 1 | 512 | 0 | 0 | 0 |
| critical+2 | 1 | 4096 | 0 | 0 | 0 |
| critical+2 | 1 | trivial | 0 | 0 | 0 |
| critical+2 | 2 | 1 | 6 | 8 | 9 |
| critical+2 | 2 | 64 | 2 | 4 | 5 |
| critical+2 | 2 | 512 | 1 | 3 | 4 |
| critical+2 | 2 | 4096 | 1 | 2 | 3 |
| critical+2 | 2 | trivial | 7 | 8 | 10 |
| critical+2 | 3 | 1 | 8 | 9 (cov 0.900) | inf (cov 0.104) |
| critical+2 | 3 | 64 | 5 | 7 (cov 0.900) | inf (cov 0.104) |
| critical+2 | 3 | 512 | 4 | 6 (cov 0.900) | inf (cov 0.104) |
| critical+2 | 3 | 4096 | 3.5 | 5 (cov 0.900) | inf (cov 0.104) |
| critical+2 | 3 | trivial | 8 | 9 (cov 0.900) | inf (cov 0.104) |

### Decision stratum by source (7x7 critical, r = 3)

| source | prefixes | coverage | true deficit (median) | W=1 | W=64 | W=512 | W=4096 | trivial |
|---|---|---|---|---|---|---|---|---|
| schedule | 120 | 1.000 | 10 | 7 | 5 | 5 | 4 | 7 |
| strict | 120 | 0.992 | 7 | 7 | 5 | 5 | 4 | 7 |

## How the gap changes with board size (the only part that bears on larger boards)

At a fixed number of rows remaining, the median gap of the W = 4096 bound
grows with n on both colour tracks (table "trend" below). At the critical
colour counts with 2 rows left it is 1, 2 and 4 edges (n = 6, 7, 8); with 3 rows left
it is 3, 4 and at least 5 (at 8x8 only 36% of these prefixes finished; 5 is the median under M_lo and the median of the finished ones). Part of the growth is that a row is longer (n cells), and part is
that the same W keeps a smaller fraction of a frontier that has n + 1 slots;
per remaining cell the gap still rises (W = 4096, critical colours, 2 rows
left: 0.083, 0.143 and 0.250 per cell left at n = 6, 7, 8; 3 rows left: 0.167,
0.190 and at least 0.21). The bound closes most of the trivial bound's gap where few cells are
left, and less of it as n grows: at 2 rows left W = 4096 removes 86%, 75% and 60% of
the trivial gap (n = 6, 7, 8). Nothing here supports extrapolating a tight
bound to the 16x16 (a frontier of 17 slots over 22 colours, with 256 pieces):
the direction measured is the wrong one, and the 16x16 is far outside the
range.

With many rows left the diagram adds little, but how little depends on the
colour count (corrected after the judgment, see below). The direct per-prefix
median of (trivial bound minus W = 4096 bound), which needs no truth, is:
6x6 c = 5: 1 at r = 4, 0 at r = 5; 7x7 c = 6: 1 at r = 4, 0 at r = 5 and 6
(at the critical colour counts the diagram is within one edge of the trivial
bound). At the higher colour counts it removes more: 6x6 c = 7: 3 at r = 4,
1 at r = 5; 7x7 c = 8: 3 at r = 4, 1.5 at r = 5, 0 at r = 6; 8x8 c = 8: 2 at
r = 4; 8x8 c = 10: 3 at r = 4. Where the difference is 0, the merged nodes
carry colour sets that cover almost every colour, so nearly every remaining
edge counts as matched. (At 8x8 with r >= 4 only lower bounds on the gap
exist, coverage 0 to 1.3%.) The bound only works in the last two or three rows.

## Reading the numbers

- **Gap** = bound minus true best completion (matched-edges). The truth is
  exact where the oracle finished; censored prefixes (oracle cap 1e9 nodes for
  r <= 3, 1e8 for r >= 4) enter M_hi at +infinity and M_lo at their lower
  bound (bound minus the oracle's proven upper bound on the truth). Where
  coverage is low (r >= 4 at 7x7 and 8x8) the "finished" quartiles describe
  the easy prefixes only.
- **Population.** Prefixes are partial boards the repo's `BreakSolver` reaches
  on a row-major path, half with a break schedule (one break cell per row),
  half strict (no breaks: perfect so far). Almost all are deeply dead: at the
  decision stratum the median true deficit (best completion below the planted
  maximum) is 10 edges for schedule prefixes and 7 for strict ones, and none
  of the 240 can reach max - 4. The gap was not measured on prefixes close to
  an optimum, which is where a branch and bound with a good incumbent would
  spend its time; a prefix with a small deficit may have a different gap.
- **Pruning shares** count, among prefixes whose truth is below the target T,
  those whose bound is also below T (a valid bound never prunes a prefix that
  can reach T). The targets are relative to the planted maximum, which every
  instance reaches.
- **Work.** DD nodes and transitions are machine-independent; wall times are
  per machine (`x86` x86_64 and `m1` arm64, recorded per row) and per
  thread. At 7x7 r = 3 the W = 4096 bound costs a median 1.3e6 transitions,
  about 0.2 s on one `m1` core: far more than a DFS node. A stage 1 would
  have to show that the pruning pays for that.
- **Frontier relaxation** is common wherever more than one row is left: at
  W = 4096, 12% to 48% of the absorbed nodes come from step-2 merges (colour
  sets relaxed), the rest from merging identical frontiers (pieces relaxed
  only); with one row left nothing is merged at W >= 512.

## What a pass would and would not mean

K1 and K2 not firing says the diagram's bound is within a median of 4 edges
of the truth three rows from the end of a 7x7 board, and 3 edges better than
no bound (the W = 1 control is the trivial bound at the median). It says nothing about search gains (stage 1, not run)
and nothing about the 16x16. The 16x16 question also depends on how much of a
break-tolerant search runs after its first break unlock, which another track
is measuring on the real pieces tonight (`reported`: a referee report on the
lead branch, `memos/partial-match-regime.REVIEW.md`, finds that whether a
score bound pays there turns on that share); these gap numbers per remaining
row and per remaining cell are the other input.

## Budget, machines, coverage

Runs: 20:50 to 00:27 on `m1` (2, then 4, then 7 threads) and `x86` (from
21:24; 2, then 6, then 12 threads), following the lead's quotas. Total compute
30.95 thread-hours (12.45 on `m1`, 18.50 on `x86`, summed from the per-row
wall times), above the 10 to 15 estimated in `PREREG.md` (a budget overrun with
no effect on any count); the excess is the
censored oracle runs at 8x8 and at r >= 4. Not reached in a useful sense:
8x8 with 4 or more rows left (oracle coverage 0 to 1.3%, so only lower bounds
on the gap exist there) and 8x8 with 3 rows left beyond the 36% (critical) and
10% (critical + 2) of prefixes the oracle finished. The r >= 4 strata at 7x7
c = 8 are in the same state (coverage 0.4 to 1.7%).

## Limits

- **No run log.** `results/run-log.jsonl` does not exist: the preregistration
  guard (`prereg_guard.py`) was committed on the lead branch one minute before
  `PREREG.md` and never reached this branch. No test row has a guard log
  line, so the order "preregistration before results" rests on commit order
  and file birth times (the judge checked both).
- **Every decision prefix is deeply dead.** None of the 240 can reach the
  planted maximum minus 4 (the best truth is max - 5). The gap near the
  optimum, where a search with a good incumbent spends its time, was not
  measured.
- **Cost.** One W = 4096 bound costs a median 1.3e6 transitions (about 0.2 s)
  at 7x7 with 3 rows left. Nothing here shows it pays inside a search.
- **Compute overrun.** 30.95 thread-hours against 10 to 15 preregistered.

## Corrections made after the judgment

Wording only, requested by the judge (`JUDGE.md`); no number and no verdict
changed.

1. The sentence "at r >= 4 on 6x6 and 7x7 its median gap is within one edge of
   the trivial bound" held only at the critical colour counts. It is
   restricted above and replaced by the direct per-prefix median difference
   between the trivial bound and the W = 4096 bound: 3 at 6x6 c = 7 and 7x7
   c = 8 with 4 rows left, 2 to 3 at 8x8 with 4 rows left.
2. The Outcome now says that the W = 1 control equals the trivial bound at the
   median on every stratum with 2 or more rows left, so "3 edges better than
   the control" means 3 edges better than no bound.
3. The Limits section above (missing run log, dead population, bound cost,
   compute overrun) was added.

## Reproduce

```
just experiments score-bound-gap            # self-test + analysis of the committed rows
cd research/experiments/score-bound-gap
cargo build --release --manifest-path compute/Cargo.toml
./sample_all.sh                             # test prefixes (deterministic, about 3 min)
./run_remote.sh MACHINE THREADS RUNG:SOURCE:RLIST:CAP ...   # e.g. n7-c6:strict:1,2,3:1000000000
python3 -I analysis/analyze.py              # results/summary.json + tables
```

Gaps, node counts and oracle coverage are machine-independent; wall times
are not (each row records its machine).

## Files

- `compute/src/dd.rs`: the bound and its merge rule; `oracle.rs`: exact best
  completion and an unpruned brute force; `sample.rs`: `BreakSolver` prefix
  sampler; `selftest.rs`: brute-force soundness checks (also a `cargo test`).
- `results/test/prefixes-*.jsonl`: the sampled test prefixes;
  `results/test/measure-RUNG-SOURCE-MACHINE[-qN].jsonl` (the `-qN` files are
  extra processes started when the thread quota rose): one row per prefix (oracle truth or
  censoring, every bound, work counters); `results/summary.json`: every number
  quoted here.
- `results/calibration/`: development rows on calibration seeds 1001..1010
  (not used in any verdict).
