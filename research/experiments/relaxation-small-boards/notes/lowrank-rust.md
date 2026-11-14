# Low-rank solver in Rust: faithful port, then time to a verified solution at 5x5

Build study, 2026-11-13 to 2026-11-14. Draft, not independently checked. Every board is a generated TOY (5x5, frame 3, inner 4, seeds 0 to 4, first corner pinned, and the broken-board control of `run_unit.py`); nothing here is Eternity II progress. Marks: [M] measured by the crate in `lowrank-rust/`, [D] derived, [memo] taken from a sibling memo, [G] guessed. No 6x6 run was made (project's decision of 22:51).

## 0. The reading first

- **Speed-up.** Best configuration: median **3.3 s** CPU to a verified solution per random start, 90th percentile 12.3 s (successes), **50 of 50** starts solved on the five boards (10 fixed starts each, NumPy's own `init_U` seeds), broken control 0 of 10 [M, `results/s10_nn0_rs1000.jsonl`]. The NumPy baseline rounds only when it stops, at its 480 s limit, and solves 52 of 66 runs (79 percent, the project's tally; 74 of 88 on disk in this study). Against that limit the median start is about 145 times faster. The baseline's own time to a first solution was never recorded, so that ratio is an upper bound on the true gain. The decomposable part: **5.0 times fewer evaluations** for the same success rate (15,188 to 3,017, median, the same port before and after) and **about 10 times cheaper evaluations** (NumPy 11.3 ms at r = 32 [M from its rows: 480 s over 42,306 evaluations]; port 1.14 ms, best configuration on a loaded Mac).
- **With parallel starts**, the lever called threads: 8 independent starts on 8 cores reach a first verified solution in 1.5 to 2.5 s on average per board, 3 starts in 1.9 to 3.6 s [M, `py/portfolio.py`, exact average over all subsets of the 10 measured starts]. Threads inside one evaluation lose at this size (section 3).
- **The port is proved equal before anything changed.** 30 states (6 boards x 5 random V, multipliers, penalties 1 to 1e4, ranks 4 to 64): objective and full gradient (in V and in the sphere coordinates W) agree with NumPy to a worst relative error of **1.46e-13** for all eight kernels, target 1e-9: PASS [M, `results/equiv.txt`].
- **What moved the count:** the inner budget (300 L-BFGS iterations per outer step to 20) and the stopping rule. Rank 16 instead of 32 costs nothing in success. Dropping the nonnegativity multipliers (keeping that inequality as a pure one-sided penalty) is faster at r = 16 and removes the only stored object of size F. Schedule knobs (penalty start, growth, cap, objective weight, L-BFGS memory) all land within about 10 percent [M, sweep on x86].
- **The relaxation is not solved when the rounding is.** At the first valid rounding the largest violation is 0.03, the mass off the solutions 0.18 per cell, and the factor has full rank 16 [M, medians]. This is a heuristic that finds a board while the penalty dynamics still move, not a convergent certificate; theory C reaches the same reading [memo].

## 1. Step table

Same five boards and 10 starts throughout, cap 480 s CPU per start (broken control: cap 60 s, 10 starts). Seconds are thread CPU seconds on the Mac (`clock_gettime`), because the machine carried a load of 15 to 108 from other jobs and wall time was mostly queueing; even CPU seconds drift with the load (the early steps ran at load 50 to 100, the late ones at 15 to 40), so **the evaluation count is the load-independent column**. "p90" is over successful starts. Floor: 79 percent.

| step | change (kept unless marked) | solved | median s | p90 s | median evaluations | ms per evaluation | broken solved |
|---|---|---|---|---|---|---|---|
| NumPy | `bm.py`, r = 32 and 64, rounding at the 480 s limit | 52/66 (79%) | 480 (limit) | 480 | 42,306 (r = 32, whole run) | 11.3 | 0/12 |
| S0 | faithful port (BLAS kernel), NumPy stopping (200 outer steps), first valid rounding recorded, end rounding scored | end: 28/39 (72%); valid at some point: 39/39 | 33 to first, 130 to end | | 16,274 to first | 2.0 | not run |
| S1 | stop at the first exactly verified rounding, rounding after every inner iteration | 50/50 | 36.2 | 55.5 | 15,188 | 2.41 | 0/10 |
| S2 | inner L-BFGS budget 300 to 20 iterations per outer step | 50/50 | 7.1 | 17.8 | 2,695 | 2.65 | 0/10 |
| S3 | violation and multiplier update read from one Gram dgemm (same values to 7e-15) | 50/50 | 6.9 | 15.5 | 2,748 | 2.46 | 0/10 |
| S4 | no nonnegativity multipliers, r = 32 (**rejected** at r = 32: slower) | 49/50 | 8.6 | 25.7 | 3,612 | 2.08 | 0/10 |
| S5 | rank 32 to 16 | 50/50 | 6.3 | 16.1 | 3,231 | 1.96 | 0/10 |
| S6 | structured kernel: no P x P object in the evaluation | 50/50 | 6.2 | 20.0 | 3,274 | 1.94 | 0/10 |
| S7a | restart from a fresh start after 100 outer steps without a rounding (**rejected**) | 50/50 | 9.2 | 42.7 | 4,841 | 1.86 | 0/10 |
| S7b | the same after 250 (**rejected**: p90 doubles) | 50/50 | 6.1 | 39.7 | 3,274 | 1.92 | 0/10 |
| S8 | penalty growth 1.3 to 1.5 (neutral, **not kept**) | 50/50 | 6.6 | 16.2 | 3,446 | 1.89 | 0/10 |
| S9 | no nonnegativity multipliers, r = 16 | 49/50 | 3.9 | 13.6 | 3,010 | 1.32 | 0/10 |
| **S10** | **plus a restart after 1,000 outer steps without a rounding** | **50/50** | **3.3** | **12.3** | **3,017** | **1.14** | **0/10** |

Best configuration (S10), per board [M, `results/best_s10_table.md`]:

| board (solutions) | solved | median s | p90 s | median evaluations | mean first solution, 8 parallel starts |
|---|---|---|---|---|---|
| seed 0 (36) | 10/10 | 3.1 | 9.6 | 2,613 | 1.5 s |
| seed 1 (6) | 10/10 | 4.0 | 6.7 | 3,274 | 2.4 s |
| seed 2 (8) | 10/10 | 5.8 | 12.6 | 4,785 | 2.2 s |
| seed 3 (403) | 10/10 | 2.5 | 3.6 | 2,204 | 1.7 s |
| seed 4 (2) | 10/10 | 7.6 | 14.4 | 6,516 | 2.5 s |
| broken (0) | 0/10 | | | | |

Configuration: `kernel=struct r=16 inner_iters=20 fast_outer=1 mult_mode=1 restart_outer=1000`, penalty from 10, times 1.3 when the violation did not halve, cap 1e5 (bm.py's schedule), L-BFGS memory 10 with a strong Wolfe line search, f64 throughout, entrywise nonnegativity kept on every free pair. Every accepted board was also found in the enumerated solution list [M].

Notes on the table.
- **S0 shows the stopping rule is needed, not only faster.** Run to its end, the faithful algorithm had found a valid rounding in 39 of 39 runs but still held one at the end in only 28: on seed 0 and seed 3 half the runs wander off the solution they had found [M]. NumPy's end-of-run scoring undercounts its own solver for the same reason [D].
- **S2 is the big lever, and it is in evaluations, not seconds:** 5.6 times fewer. More frequent multiplier updates beat solving each subproblem further. x86 sweep, evaluations to a solution (median, 10 starts x 5 boards): inner 300: 16,432; 50: 4,348; 20: 2,756; 10: 2,533 [M]. I kept 20 rather than 10 (within noise, and the outer step costs a Gram product).
- **S4 against S9.** Dropping the nonnegativity multipliers cost r = 32 a third more evaluations and saved r = 16 8 percent (3,274 to 3,010), with one stuck start each time. At r = 16 each evaluation is also cheaper, since the struct kernel no longer patches the 84 percent of free pairs that carried a multiplier; part of the drop from 1.94 to 1.32 ms is the lighter machine load late in the night, which I cannot separate from that saving. The stuck start is a tail, not a trap of the configuration: S10's restart after 1,000 outer steps (about 25,000 evaluations, eight times the median) recovers it and touches no other run (same evaluation counts, deterministic) [M].
- **The broken control cannot pass by construction**: acceptance is an exact check (each cell filled by one of its placements, each tile once, every inner edge matched, every rim side grey) and the broken board has no solution by enumeration. It checks the verifier, and it stayed at 0 of 10 at every step.

## 2. Where the remaining time goes (S10, 50 runs)

| part | share of CPU time | per occurrence |
|---|---|---|
| evaluation (two dgemm of SB x P x r, one elementwise pass over all pairs, conflict patches) | 75% | 1.1 ms in the benchmark at P = 478, r = 16 |
| L-BFGS (two-loop recursion over 10 pairs, line search vectors) | 15% | about 40 vector passes of length P r per iteration |
| outer step (Gram, violation, multiplier update) | 7% | 1.9 ms every 20 inner iterations |
| rounding and exact check after every inner iteration | 2.7% | sort of P values |

Evaluations per inner iteration: 1.30 (line search). Outer steps to a solution: median 117.

**Kernel benchmark** (ms per evaluation, one thread, P = 478, median of 3, [M, `results/kernels.txt`]):

| kernel | r = 8 | r = 16 | r = 32 |
|---|---|---|---|
| ref: literal port of `fV` (pair list for zeros, all ordered pairs for I2) | 4.75 | 6.02 | 11.5 |
| rows: one pass over all pairs, no gather/scatter list | 4.94 | 6.05 | 10.2 |
| blk: rows blocked by 4, transposed factor, branch-free, safe Rust | 1.52 | 2.46 | 4.78 |
| blas: Gram by Accelerate dgemm, one elementwise pass, dgemm back | 0.90 | 1.03 | 0.98 |
| blas2: same, pair kind as one byte instead of two f64 masks (**rejected**) | 1.10 | 1.25 | 1.27 |
| struct: no P x P object, 64-row blocks, conflict and active lists | 1.05 | 1.10 | 1.40 |

The engineering study's two facts both held [memo, section 2 and 3]: reading the pair terms out of the Gram matrix instead of gathering 2 K r numbers is the whole gap between `ref` and `blas` (5 to 12 times), and safe Rust loops stay 1.5 to 5 times slower than the BLAS product even when blocked and vectorised. Unsafe code is therefore used in exactly two isolated places: the Accelerate `dgemm` call (`src/blas.rs`, bounds asserted before the call) and `clock_gettime` for thread CPU time (`src/cputime.rs`). Single precision was not tried: at r = 16 the GEMM part is about a fifth of an evaluation (fit below), so f32 could win at most that fifth.

**Threads [M].** Accelerate with 2 or 4 threads: 1.62 and 1.51 ms against 0.92 ms on one (slower). Rayon across row chunks (deterministic chunk-ordered sums, kernels `par` and `blkpar`): 2 and 3 threads slower than one at P = 478. At this size a core is worth more as another start.

## 3. Cost model and expectations

**Per evaluation [M fit, D].** t = P^2 (a + b r) on one core, with a = 3.9e-9 s per pair (elementwise pass and list patches) and b = 6.7e-11 s per pair and column (the two products), fitted on the struct kernel at r = 8, 16, 32 (`py/costmodel.py`). In flops: 4 P^2 r for the two products, about 8 P^2 for the pass, 2 (K + A) patches (A = 0 in the best configuration), plus 4 m P r for L-BFGS. The P^2 elementwise term is 3.6 times the product term at r = 16; it equals it near r = 58. **Rank below 16 buys little time at any size.**

**Per outer step [D].** 20 inner iterations x 1.30 evaluations, plus one Gram (2 P^2 r), one violation pass and one multiplier update (P^2 each): about 26 P^2 (a + b r) + 3 P^2 a.

**Memory of the best configuration, P^2-free [D].** (2 m + 8) P r doubles of vectors, SB P doubles of scratch per thread, 2 K x 12 bytes of conflict list (index and multiplier). The 5x5 code still holds the pair kind and a dense multiplier matrix for bookkeeping between outer steps; the evaluation does not read them, and at scale the kind of a pair is three structural tests (same cell, same tile, adjacent with mismatched colours).

| board | P | s per evaluation, 1 core | 10 cores (ideal) | GB, best form | GB of nonnegativity multipliers if kept (A = 0.84 F, measured) |
|---|---|---|---|---|---|
| 5x5 | 478 | 0.0011 [M] | | 0.005 | 0.002 |
| 6x6 | 1,290 | 0.0083 | 0.0008 | 0.015 | 0.014 |
| 7x7 | 2,916 | 0.043 | 0.004 | 0.04 | 0.07 |
| 12x12 | 41,616 | 8.7 | 0.87 | 1.4 | 17 |
| 16x16 | 156,816 | 123 | 12 | 9.3 (conflict list 7.9) | 241 |

**Expectation at 6x6 and 7x7 (labelled: not run).** Evaluations to a solution are measured at one size only (median 3,017 at P = 478), so the growth is unknown. With growth P^1 to P^2: 6x6 needs about 8,000 to 22,000 evaluations, **70 to 190 s per start on one core**; 7x7 18,000 to 110,000, **13 to 80 minutes per start** [D, G for the exponent]. One reading of the NumPy 6x6 failures at 1,650 s follows from section 1 [D]: at about 57 ms per evaluation [memo, engineering, r = 64] they made roughly 29,000 evaluations with the 300-iteration inner budget, which at 5x5 needed 16,400 where 20 iterations need 2,800. "Not converged" there is not yet evidence against 6x6; it is consistent with the slow inner budget. Whether the rounding still lands at 6x6 is the open question this memo does not answer.

**16x16 [D].** 123 s per evaluation on one core, about 12 s on 10 cores if memory bandwidth allows; 9.3 GB in the best form. **The term that breaks first is the nonnegativity multiplier store**: with multipliers kept it passes 32 GB at P near 56,000 (between 12x12 and 14x14), because 84 percent of the free pairs carry one at the solve point (there is no small active set, as theory A and B also found [memo]). The best configuration drops that store; then nothing breaks in memory, and the next wall is time: the P^2 elementwise term, 2.5e10 pairs per evaluation, rank-independent. At the 5x5 count of 3,000 evaluations that is 10 hours per start on 10 cores; the count will grow. Theory A's aggregated equalities would replace the 7.9 GB conflict list by 7,952 vector equalities [memo]; they do not touch the P^2 term.

## 4. What did not work

- **Restarts as a strategy** (S7): restarting after 100 or 250 outer steps throws away the penalty already built; the median rises or holds and p90 doubles. Only the long-stall restart (1,000) helps, as insurance for a rare stuck start.
- **Pure quadratic penalty** (no multipliers at all): 0 of 12 on a mini suite [M]. The equality multipliers (sums, conflicts) are needed.
- **Low rank 4 and 8**: 8,667 and 4,831 median evaluations, 49 and 50 of 50 [M, x86]; the count rises faster than the per-evaluation cost falls once the P^2 pass dominates. Rank 64: 3,693 (47 of 47 when stopped) at twice the cost.
- **Schedule knobs, all within about 10 percent of inner 20** [M, x86, evaluations median / p90]: penalty start 1,000 (2,666 / 6,276, 49 of 50), cap 1e4 (2,728 / 6,770), cap 1e6 (2,898 / 6,697), growth 1.5 (2,516 / 5,582), objective weight 0 (3,145 / 5,941), L-BFGS memory 5 (2,918 / 5,434); reference 2,756 / 6,434.
- **Byte mask for the pair kind** (blas2): slower than two f64 masks on this machine.
- **Threads inside an evaluation**, BLAS or rayon: slower at P = 478 (section 2).
- **Not tried, for lack of time:** a truncated-Newton inner step with Hessian-vector products (the inner budget result says inner accuracy is not the bottleneck, so I ranked it low); f32 products; an initialisation-scale sweep (queued on x86, not reached).

## Appendix A. Files and how to reproduce

All under `lowrank-rust/`.
- `py/export.py` (with `bm.py`, `board.py`, `common.py` copied unchanged from `scripts/lowrank-relaxation/`): boards, linear constraints (cell and tile rows), conflict and free pair lists, enumerated solutions, NumPy's starts for ranks 2 to 64 and starts 0 to 9, and the 30 equivalence states. `uv run --with numpy --with scipy python -I py/export.py 5`.
- `src/`: `al.rs` (objective, gradient, eight kernels, violation, multipliers), `lbfgs.rs`, `solver.rs` (outer loop, rounding, exact check, restarts), `board.rs`, `blas.rs`, `cputime.rs`, `main.rs` (`equiv`, `run`, `bench`). `cargo build --release -j 3`.
- `step.sh <tag> <binary> [key=value ...]`: one step row. The binaries used for each step are kept in `bin/` (v5 for S1, v6 for S2 and S3, v7 from S4).
- `results/<step>.jsonl` one row per run; `py/summarise.py`, `py/portfolio.py`, `py/tags.py`, `py/costmodel.py`.
- `results/x86/`: the sweep rows from x86 (Linux, 13 jobs, pure-Rust `blk` kernel, budget in evaluations; `sweep.sh`). Cross-check of kernels: inner 20 at r = 32 needed a median 2,756 evaluations there against 2,695 on the Mac with the BLAS kernel [M]; trajectories differ in rounding only.
- `results/superseded/`: a first S1 whose parallel jobs were serialised by a shared one-thread pool (fixed before every row in the table); `results/s0.jsonl` stops at 39 runs (stopped to stay inside the core limit).
