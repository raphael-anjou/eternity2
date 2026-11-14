# Is the degree-2 lift still exact at 6x6? (launch record, 2026-11-13)

Draft. TOY BOARDS ONLY: every board is generated (`common.py` painter, frame colours 4, inner colours 5, seeds 0, 1, 2, the same boards as the lift and low-rank studies). Nothing here is Eternity II progress. Scripts and logs in `scripts/convex-6x6/`.

## Reading

**6x6: no reading yet.** The six solves are launched or queued (table below); none has converged. A run that has not converged proves nothing either way.

**5x5 validation (measured, local Mac, load 31 on 10 cores).** On 5x5 (3,4) seed 1 the modified `admm.py` reproduces the memo's trajectory: minimum-norm objective, mass off solutions 4.96e-6 per cell at iteration 4,500 (memo: 4.9585e-6 at 4,500), 2e-11 by 13,500. The new largest-leak objective **converged to residual 1e-7 in 5,985 iterations (294 s)**: mass off solutions 8.3e-9 per cell, largest constraint violation 1.4e-8, rank 6 against 6 solutions. That is the first largest-leak certificate from method B: on this board no feasible point of the lift puts mass on a placement outside the solutions. The leak objective converges about three times faster than the minimum-norm one, whose tail is slow (residual 1.2e-5 at 14,000).

**Kill line, fixed before any 6x6 result:** mass off solutions above 0.05 per cell on 2 of 3 boards (largest-leak objective, converged to 1e-7) means the lift is not exact at 6x6 and all engineering on this line stops. Classes per run: converged and exact; converged and leaking (with the number); not converged (with residuals, proves nothing).

## Runs

| board (6x6, 4, 5) | objective | where | state at 22:20 | s per iteration | expected finish |
|---|---|---|---|---|---|
| seed 0 (7 solutions, face order 1,223) | leak | m1 | running since 22:12 | 1.72 [M, iteration 100] | 06:00 Nov 14 to 08:00 Nov 16 |
| seed 2 | leak | m1 | queued: starts when a core frees (the low-rank pool there ends near midnight) | about 1.7 [G] | 08:00 Nov 14 to 10:00 Nov 16 |
| seed 1 | leak | m1 | queued, same | about 1.7 [G] | same |
| seed 0 | l2 | x86 | queued: x86 is full (3 SCS solves to about 23:05, 10 low-rank units) | 2.4 for the eigendecomposition alone at order 1,220 [M], so 3 or more [G] | days; the l2 tail is slow |
| seed 1 | l2 | x86 | queued | same | same |
| seed 2 | l2 | x86 | queued | same | same |

Expected finish: leak iterations scaled from the 5x5 leak run (5,985 to 1e-7) by (P / 478)^a with a = 1 to 3 (the memo's range), so 16,000 to 118,000 iterations [D]. The memo's cost model (0.40 s per iteration at order 1,291) is 4.3 times optimistic on m1 under full load. The leak runs decide the kill line; they all sit on the faster machine.

Collect at any time: `sh scripts/convex-6x6/collect.sh` (pulls every PROGRESS file and prints one table with the classes and the kill-line count). A run interrupted for any reason resumes from its checkpoint by rerunning its line of `jobs_*.txt` (same command).

## Deviations

1. **Penalty rule.** The textbook residual balancing (rho doubled or halved when the primal residual and rho times the step differ by more than 10 times) parked rho at 4.7 and 9.4 on 5x5 seed 1 and stalled: mass off solutions 0.35 and 0.62 at iteration 5,000, where the memo's fixed rho of 300 reads 5e-6 (`raw/val5_scaled_balance_failed/`). I replaced it by balancing the primal residual against the step itself, both in the units of v. On 5x5 seed 1 from rho 300 that rule never fired (the two stayed within a factor 2), so the automatic penalty is in place but untested in action; at 6x6 it moves rho only if the residuals separate by 10 times.
2. **Stop.** Primal residual and step both below 1e-7, the original script's own test. Rho times the step is logged as `r_dual` but not used: at 5x5 it was 2.6e-5 at the stop, and requiring it below 1e-7 would cost tens of thousands of iterations without moving a reading at the 0.05 level.
3. Starting rho 300 (the memo's best on seed 1) for all runs. Anderson acceleration and the path without facial reduction were removed (the memo dropped both).
4. **Threads: one per run.** The two-thread test was not run: no machine had two idle cores (x86 13 to 14 busy, m1 7 of 8). The memo measured 1.1 times for two threads at this order on the local Mac, well under 1.6.
5. **Only one of six runs started at report time.** The other five wait in `queue.sh`, which starts a run when fewer than CAP processes (8 on m1, 14 on x86) use over half a core on two checks 45 s apart, so other jobs' runs are never oversubscribed. The kill-line reading therefore starts around midnight for two of three boards.
6. A first launch on x86 stayed attached to its shell; the queue (which had started nothing) was killed and relaunched detached with `setsid`. No other jobs were touched.

Files in `scripts/convex-6x6/`: `admm.py` (modified copy), `bm.py`, `board.py`, `common.py` (unmodified copies, hashes of all originals in `COPIED_SHA256.txt`), `summarise_admm.py` (unmodified), `queue.sh`, `jobs_m1.txt`, `jobs_x86.txt`, `collect.sh`, `collect_table.py`, `raw/val5/` (validation logs), `raw/val5_scaled_balance_failed/`. Remote work only under `./convex6/` on x86 and m1.

## Resume of the leak runs, 2026-11-14

TOY BOARDS ONLY. Scripts and logs in `scripts/convex-6x6/resume/`, raw files in `resume/raw/`.

**Reading.** No run converged, so none decides anything. The kill line, unchanged: mass off solutions above 0.05 per cell on 2 of 3 boards (largest-leak objective, converged to residual 1e-7) means the lift is not exact at 6x6. A run that has not converged decides nothing under it. x86 rebooted (system, not this study) at 04:30:03, which ended all three runs; they were not relaunched.

**What was resumed.** The three leak runs (6x6, frame 4, inner 5, seeds 0, 1, 2) from their m1 checkpoints at iterations 3,800, 2,900 and 4,200, on x86, nice 15, one BLAS thread each. `resume/admm.py` adds an exact resume (previous Z and residuals saved) and a test iteration cap; on the 5x5 validation board a run resumed at 200 matched an uninterrupted one at 400 with difference 0.0.

**Why they were relaunched at 03:37 (`runs_bad_pinv/`).** After the 02:22 resume the seed 1 residual rose from 5e-4 to 3e-2, impossible for fixed-penalty Douglas-Rachford. Cause: `np.linalg.pinv(C @ C.T)` with numpy's default cutoff of 1e-15. C C^T is rank-deficient; x86's OpenBLAS keeps rounding-level singular values the Macs drop, giving pinv entries near 1e11 on all three 6x6 boards (0.25 on the Macs and on 5x5, so the 5x5 check passed). The fix is a cutoff of 1e-10. With it, x86 and the local Mac agree to 12 digits over 10 iterations from the seed 1 checkpoint. The runs restarted from the original m1 checkpoints; the bad segment is in `runs_bad_pinv/` on x86 and `resume/raw/bad_pinv/`. The 2026-11-13 minimum-norm runs on x86 carry the same fault.

**Last reading (x86, 04:27 to 04:29).** Penalty 300 throughout; the automatic rule never fired, so these are fixed-penalty runs and review row C4's failure mode did not occur.

| board | iteration | primal residual | step | rho times step | mass off solutions per cell | largest violation | penalty |
|---|---|---|---|---|---|---|---|
| seed 0 | 5,400 | 3.4e-4 | 2.6e-4 | 7.9e-2 | 0.843 | 6.6e-6 | 300 |
| seed 1 | 4,400 | 3.5e-4 | 4.0e-4 | 1.2e-1 | 0.865 | 4.6e-6 | 300 |
| seed 2 | 5,900 | 2.8e-4 | 2.8e-4 | 8.4e-2 | 0.730 | 3.9e-6 | 300 |

**Trend over the last 1,000 iterations.** The larger of the two stop residuals fell by a factor of 0.84 (seed 0), 0.71 (seed 1) and 0.73 (seed 2). Mass off solutions moved by under 0.001 per cell on each board over the same window. At those rates, 1e-7 needs about 47,000, 24,000 and 25,000 more iterations, 24, 13.5 and 13 hours at the 1.8 to 2.0 s per iteration measured on the loaded x86. These are lower bounds: the rate keeps slowing, and a power-law fit puts seed 0 near 7e7 iterations.

**Deviations.** The thread test after 03:30 was not run: x86 stayed at load 13 until the reboot. The pinv cutoff in `resume/admm.py` differs from the launch copy, and changes nothing on the Macs.
