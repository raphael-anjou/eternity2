# Growth exponent of the low-rank solver, 5x5 to 6x6

Build study, 2026-11-14. Draft, not independently checked. Every board is a generated TOY (5x5 frame 3 inner 4; 6x6 frame 4 inner 5; seeds 0 to 4, first corner pinned, the same generator as every other study, plus the broken-board control built as in `run_unit.py`). Nothing here is Eternity II progress. Marks: [M] measured, [D] derived, [X] extrapolation. Configuration: the best one of `lowrank-rust.md` (S10), unchanged. Crate and rows in `growth-exponent/`; the analysis is `py/growth.py`, its full output in `results/growth.txt`.

## 0. The reading first

- **Exponent 4.5, not 1 or 2.** Expected evaluations per verified solution, charging capped starts in full: 5,083 at P = 478, then 460,416 at P = 1,290 (Mac, 50 starts per size). Exponent **4.54**, bootstrap 95 percent over boards **[3.97, 5.31]** [M, D]. The Kaplan-Meier median (capped starts censored) gives the same answer: 3,010 to 270,051, exponent 4.53 [4.00, 4.71]. The x86 replicate (other kernel, 6.6 times fewer evaluations per start) gives 4.86 [4.25, 6.00]. Pooling both machines at 6x6: 4.57.
- **The fit rests on two sizes.** Nothing says the exponent is constant. Two points cannot tell a power law from an exponential, and they cannot show curvature.
- **16x16, extrapolation [X]:** (156,816 / 1,290)^4.54 times 460,416 is about **1.3e15 evaluations**, or **1.9e11 days at 12 s per evaluation**. The interval runs from 8.6e13 to 5.4e16 evaluations (1.2e10 to 7.4e12 days). The preset "near 2 means dead" line is passed by a wide margin. **In this form the solver is of no use at 16x16.** The cost per evaluation also grows as P^2 (1.19 ms to 7.18 ms on the Mac), so CPU seconds per solution grow with exponent 6.3 (6 s to 3,084 s).
- **Verdict on the kill line.** On the Mac, with the 30-minute CPU limit, 22 of 50 6x6 starts solved (44 percent). That is above the 20 percent kill line and below the 50 percent gate for 7x7. On x86, with the same limit in seconds but slower cores and the pure-Rust kernel, 3 of 50 solved. The solve rate in a fixed number of CPU seconds depends on the machine. The evaluation statistics above do not. The broken control gave 0 of 10 on both machines. No 6x6 rounding was accepted that is not in the enumerated solution list.
- **What the 6x6 runs are.** At 6x6 the solver behaves as a restart portfolio. An attempt is 1,000 outer steps, about 25,500 evaluations. It succeeds with probability 0.055 on the Mac (22 solves in 398 attempt-equivalents) and 0.045 on x86 (3 in 66.5). That is about 18 attempts per solution. At 5x5, 49 of 50 starts solved inside their first attempt. No capped start converged. All were still moving at the cap (violation 2.4e-2 to 6.0e-2, full rank 16, 2 to 21 mismatches after rounding), so "converged and wrong" never occurred.

## 1. Table per size

Expected evaluations and CPU seconds per verified solution = everything spent (capped starts in full, solved starts up to their solution) / solutions found. Seconds are thread CPU seconds, on the machine named. Mac: this Mac, `kernel=struct` with Accelerate, 5 to 8 jobs, nice 10, under load from other jobs. x86: Linux laptop chip (2 fast and 12 slow cores), `kernel=blk` in pure Rust, 13 jobs. The objective and gradient are the same to 1e-13; trajectories differ only by rounding.

| size, P | machine | board (solutions) | solved | E evaluations per solution | E CPU s per solution | KM median evaluations |
|---|---|---|---|---|---|---|
| 5x5, 478 | Mac (memo's S10 rows) | s0 (36) | 10/10 | 5,452 | 6 | 2,585 |
| | | s1 (6) | 10/10 | 3,903 | 5 | 3,267 |
| | | s2 (8) | 10/10 | 5,973 | 7 | 4,582 |
| | | s3 (403) | 10/10 | 2,479 | 3 | 2,151 |
| | | s4 (2) | 10/10 | 7,607 | 8 | 6,415 |
| | | **all** | **50/50** | **5,083** | **6** | **3,010** |
| | x86 | s0, s4, 5 starts each | 10/10 | 4,514 | 10 | 2,528 |
| 6x6, 1,290 | Mac | s0 (7) | 6/10 | 289,519 | 1,982 | 145,849 |
| | | s1 (4) | 7/10 | 230,330 | 1,608 | 200,647 |
| | | s2 (14) | 5/10 | 377,691 | 2,582 | 164,263 |
| | | s3 (10) | 2/10 | 1,291,811 | 8,118 | not reached |
| | | s4 (3) | 2/10 | 1,153,821 | 7,778 | not reached |
| | | **all** | **22/50** | **460,416** | **3,084** | **270,051** |
| | x86 | s0, s1, s2 | 1/10 each | 315,071 to 347,597 | 16,785 to 17,691 | not reached |
| | | s3, s4 | 0/10 each | no solution | | |
| | | **all** | **3/50** | **563,118** | **29,155** | not reached |

Other figures [M]:
- Median over solved starts only (the first statistic planned): 3,017 at 5x5 and 109,540 at 6x6 (Mac). This drops the censored starts and so understates 6x6. It is given for comparison, not used in the fit.
- Evaluations a start makes in 30 CPU minutes at 6x6: Mac median 228,456, x86 34,699.
- ms per evaluation at 6x6: Mac 7.18 (the memo's model predicted 8.3), x86 49.
- Broken control, 6x6: 0 of 10 valid roundings on each machine (Mac median 226,560 evaluations, x86 38,482).

**The two machines agree** on what does not depend on speed. Success per attempt was 0.055 and 0.045. Boards s3 and s4 were the hardest on both, and s0 to s2 the easiest. Expected evaluations per solution were 460k and 563k, and the x86 figure rests on 3 solves. Seeds 3 and 4 cost four to five times more than seeds 0 to 2, and that is not explained by their solution counts (10 and 3).

**Where solves land inside an attempt (observation only).** At 5x5, solves come early: outer steps 47 to 832 after the start, median about 110, and 49 of 50 in the first attempt. At 6x6 the pattern does not hold. The 25 solves (both machines) land at steps 114 to 976 inside their attempt, median 605, and 16 of 25 come after step 500. A flat hazard would put about 12.5 after step 500, so this leans late, weakly. Implication for the restart interval: shortening it would cut off most 6x6 solves. Whether a longer interval would raise the success per evaluation is untested. The data cannot say, because no attempt ran past 1,000 steps.

## 2. Deviations and what was not run

- **Not run (decided at 02:10):** the three adjustments (rank 32, inner budget 50, slower penalty schedule) and 7x7. With 44 percent solved on the Mac, the 7x7 gate (half) would not have opened anyway. If it had, 7x7 would have used the convention of the other low-rank studies' scripts: frame 4, inner 7 (`scripts/lowrank-relaxation/scale.py`, `construct.py`), not frame 5, inner 6.
- **Two machines, two kernels.** x86 was planned as the main machine. At 00:35 the plan changed to the Mac run with the memo's exact kernel, and x86 was kept as a replicate. On x86, 30 CPU minutes bought 6.6 times fewer evaluations, so its starts are much more heavily censored. The headline fit uses the Mac, whose 5x5 rows are the memo's own S10 rows (`results/s5_memo_s10.jsonl`, copied). The x86 5x5 point rests on only 10 starts on 2 boards, the sanity check.
- **5x5 sanity check:** Mac, 2 boards x 5 starts, reproduced the memo's evaluation counts exactly (10/10). x86 `blk`, 10/10, counts in the same range (`results/check5_*.jsonl`).
- **Solutions:** enumerated at 6x6 by the existing export (17 s, 3 to 14 solutions per board). Every accepted board was in the list.
- **Code changes in the copy:** `main.rs` gained `order=start` (start-major job order, which only reorders the queue). `export.py` writes ranks 16 and 32 only, no equivalence states, and adds the 6x6 and 7x7 boards and broken controls. `run.sh` (x86) and `run_mac.sh` set `max_outer` and `wall_limit` high so that the 30-minute CPU limit is the only cap. The memo's `step.sh` used the default 840 s wall cap.
- **Load:** the Mac carried other jobs' work, so its seconds drift with load. Evaluation counts do not.
