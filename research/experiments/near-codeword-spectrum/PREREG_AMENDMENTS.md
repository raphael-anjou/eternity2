# Amendments to PREREG.md

All amendments below were made on 2026-11-11, after timing runs on
development seeds (>= 1000) only and before any reported-seed solve was
started. No reported result had been produced or read. The kill criteria
K1 to K3 and their operationalisation are unchanged.

## A1. Break indicator written as an explicit boolean (model, no change in meaning)

What: per joint j, `sum_k m[j,k] + br_j == 1` with a boolean `br_j`, and the
objective `sum_j br_j`, instead of the objective `sum_j (1 - sum_k m[j,k])`.

Why: the two are equivalent on integer points (each side has exactly one
colour), but with the original form CP-SAT reported a best bound of about
-350 on time-limited 6x6 curve points (dev seed 1000), so no usable LB. The
explicit form gives bounds >= 0 and closed the 6x6 dev bands faster (mid
4.5 s -> 2.8 s). Band optima on the dev instance were identical (near 0,
mid 3, far 2, K2 4).

## A2. Full curve b_min(a) on seeds 1..10 only (prereg: 1..30)

What: the descriptive full curve (every a = 1..N-1, 30 s x 2 workers) is run
on primary seeds 1..10 at 5x5 and 6x6, not 1..30. The band solves that feed
K1, K2 and K3 are unchanged (all 30 seeds).

Why: on dev seeds most curve points do not close in 30 s (5x5: 9 of 24
points hit the limit, about 115 s wall per instance at 4 solves in
parallel; 6x6: nearly all points beyond a = 8 hit the limit, about 260 s per
instance). 60 instances would cost about 2.6 h wall, over the 3 h total. The
curve is descriptive only (it feeds the radius metric r(b) and the band
consistency check), so r(b) is reported on 10 instances per size.

## A3. Stage 3: 16 runs per sampler (prereg: 32)

What: each stage-3 instance gets 16 runs of each sampler (break DFS, SA,
greedy null), with the same per-run budgets and seeds r = 0..15 (+100, +200).

Why: dev timing gives about 34 s per 20,000,000-step break-DFS run at 10x10
(none of 21 dev runs completed a board) and about 10 s per 20,000,000-attempt
SA run, so 32 runs would cost about 80 min wall on 8 cores instead of the
15 min in the prereg. 16 runs costs about 40 min.

## A4. Wall clock

With A2 and A3 the estimate is stage 1 bands about 15 min, curves about
65 min, stage 2 about 2 h (unchanged), stage 3 about 40 min: about 4 h wall,
above the 3 h estimate. Stage 2 (the K1c evidence) is kept at full budget
rather than cut.

## A5. Run stopped by the program lead during stage 2 (2026-11-11, 14:55 UTC)

What: the program lead asked for no further compute. The run was stopped
about 6 min into stage 2. Stage 3 was not run. Done before the stop:
stage 1 in full (320 band solves), the A2 curves in full (590 solves), and 6
of 80 stage-2 solves (7x7 C=15: seed 1 near/mid/far, seed 2 near/mid,
seed 3 near). No 8x8 solve and no contrast stage-2 solve were run.

Consequence (by the prereg's own rules, not a change to them): K1c has 1 of
10 usable 7x7 instances and 0 of 10 at 8x8, so it is undetermined; K1a, K1b,
K2 and K3 use only stage 1 and are evaluated as written. The stage-3 metrics
are not reported.

## Round-1 judge fixes (A6 to A10)

The judge found the run invalid (`JUDGE_ROUND1.md`, committed verbatim).
A6 to A10 were written on 2026-11-11 after that report and before any of
the new solves below was analysed. Timing for them came from development
seed 1000 only. The kill criteria K1 to K3 (thresholds, tests, both-ends
rule, outcome rule) are unchanged; A6 corrects the distance they are
computed on, because the prereg's stated premise for the old one was false.

### A6. Rotation-quotiented distance d* (judge finding PREREG.md:38, blocker; README.md:91 / analyze.py:168-175, blocker)

What: d*(board) = min over k = 0..3 of d_k, where d_k is the colour-tuple
distance to the planted board turned k quarter turns clockwise. The pin is
kept: d* is invariant under turning the board, so with the pin every
rotation class of boards appears exactly once (this is the quotient the
prereg intended; the judge's "no pin" version gives the same optima with a
4-fold symmetric model). Bands are the prereg's bands applied to d*, except
that the far band runs to N (d_k <= N): near* = [1, ceil(N/4)-1],
mid* = [ceil(N/4), floor(3N/4)], far* = [floor(3N/4)+1, N] (the judge's
"distinct" band: boards far from every turned copy), K2* = [ceil(N/3),
floor(3N/4)]. CP-SAT: four linear distance expressions, each >= lo, and a
Boolean disjunction that at least one is <= hi (`scripts/cpsat_quotient.py`).
B* = H*_mid - H*_far.

Instances, budgets and statistics are the prereg's stage-1 ones (5x5 C=13
and 6x6 C=14 seeds 1..30, contrasts seeds 1..10, 4 bands, 120 s x 4
workers, CP-SAT random_seed 1000*n + seed). K1a, K1b, K2 and K3 are
evaluated on d* with exactly the prereg's rules (K1b: the same one-sided
Wilcoxon, on B*). These starred evaluations decide the outcome; the
original-distance evaluations (none fires) stay in the record, labelled as
measuring the planted solution against its own turned image.

Why: the pin does not remove the board rotation. A turned planted copy with
the planted corner swapped back into cell 0 is a 2- or 4-break board in the
old far band on all 140 instances, so H_far was the price of a corner swap
on the solution's own image and B, K1a and K1b did not measure a gap
between distinct solutions.

Secondary, reported only: the Wilcoxon on H*_mid alone (the judge's other
suggested fix), and the judge's next-round quantity H*_mid -
max(H*_near, H*_far).

### A7. K1c as written, completed with the free far certificate (judge finding PREREG_AMENDMENTS.md:50, blocker)

What: K1c on the original distance d_0 at 7x7 C=15 and 8x8 C=16, seeds
1..10. Best far = the free certificate (planted board turned k = 1, 2, 3,
planted corner swapped back; min breaks over the k whose d_0 is in the far
band; `free_certificates` in `scripts/cpsat_quotient.py`). Mid: one CP-SAT
solve per seed at the prereg's 300 s x 4 workers on the mid band, with the
extra constraint breaks <= that certificate. INFEASIBLE proves mid > far;
a found board proves mid <= far; UNKNOWN means no mid board at or below
the far certificate was found in the budget, which K1c ("best mid-band board
found") counts as mid > far. The six partial stage-2 solves of the first
run are kept in `results/raw/stage2.jsonl` but not used.

Why: the far certificate needs no search, and using it in place of a
300 s far solve can only make K1c easier to fire (a CP-SAT far UB would be
<= the certificate). The breaks cap focuses the whole mid budget on the
only question K1c asks.

### A8. Stage 3 dropped

What: stage 3 (solver position, hypothesis part 3) is not run. K1c at 8x8
uses CP-SAT only (the prereg also allowed stage-3 per-a minima there).

Why: the fix is capped at about 3 hours wall, and A6, A7 and A9 use all of
it. Hypothesis part 3, and the radius at a solver plateau (part 2), stay
untested and are reported as such.

### A9. K1c on d* at 7x7 (judge's next-round K1c')

What: 7x7 C=15 seeds 1..10, mid* and far* on d*, equal budget 150 s x 4
workers each (the prereg's 300 s halved to fit the 3 h cap). K1c* fires
if >= 3 of 10 seeds have mid* UB <= far* UB (certificates only, as K1c).
K1 fires if K1c (A7) or K1c* fires; K1c at 8x8 is evaluated only on d_0
(A7) because 8x8 d* solves do not fit the budget.

Why: on d_0 the far band at 7x7 and 8x8 holds the 2-break turned copy, so
K1c as written is close to unable to fire; K1c* is the version that tests
distinct solutions.

### A10. Reporting fixes (should-fix and nitpick findings)

- The hypothesis's ">= 2 breaks" part is stated per size; it is not a kill
  criterion (K1a fires at <= 0).
- K2 band cap at floor(3N/4): stated reason in the README (without it the
  turned copy, 2 breaks in the old far band, fires K2 trivially).
- b_ann is reported for the near and mid bands only; the first-moment count
  leaves out the three turned images, so its far value is not a floor
  prediction.
- Radius r(b): certain and uncertain seeds reported separately; part 2 at a
  solver plateau not measured (A8).
- R(k) in the annealed DP counts a 2-fold symmetric interior piece turned by
  a half turn in its own cell as changed (CP-SAT does not). Not fixed:
  there are 1 such piece over the 30 5x5 C=13 instances and 5 over the 30
  6x6 C=14 instances, so the shift in E(a, b) is far below one break.

### A11. Independent witness recheck for the fix run (judge Reruns section)

What: `scripts/check_fix.py` rechecks every witness in
`results/raw/{star1,k1c,star7}.jsonl` with code that shares nothing with the
solver (numpy grid, whole-board quarter turn as `rot90` plus a side shift):
permutation, gray legality, pin, breaks = UB, all four d_k equal to the
recorded ones, band membership on d* (quotient jobs) or d_0 (k1c), and
breaks <= the cap for k1c. Analysis only; no criterion reads it.

Why: the judge's witness check was run on the first run's files only; the
fix adds a new distance, so the new witnesses get the same check.
