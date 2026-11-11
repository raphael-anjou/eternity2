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
