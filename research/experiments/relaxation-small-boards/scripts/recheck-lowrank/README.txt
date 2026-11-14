TOY BOARDS ONLY. Scripts of the independent re-check memos/relaxation-lowrank-recheck.md (2026-11-14).
Nothing here is Eternity II progress. At most 2 cores, nice -n 10, no run over 10 minutes,
nothing remote, no 6x6 solve. The memos and their folders were read, not written.

01_tally.py               every lowrank-rust result row by file and tag (runs, solved, same
                          (board, start) set, distinct configurations). python3 -I
02_anneal_cost.py [time]  annealing work per solution from the isolation study's rows, and a
                          timing of its annealer binary at 5x5 (20 runs per board). python3 -I
03_validity.py            what "solved" means: placements, grid, solutions and the broken board
                          re-derived from the painter with a DFS and an edge check written here.
04_s10_detail.py          S9 against S10 run by run, stuck starts, work per solution, state at the
                          first valid rounding, roundings rejected on the broken board, S0.
05_equiv_independent.py   the 30 stored equivalence references recomputed with the original bm.py.
                          uv run --with numpy --with scipy python -I
06_fresh_export.py ROOT   starts 10 to 49 and painter seeds 5 to 14 exported into a scratch COPY of
                          the crate (ROOT), never into lowrank-rust.
07_growth_and_cost.py     cost model recomputed; 6x6 expectation against the growth rows; growth
                          with capped runs as censored exposure. Reads raw/s6m1_snapshot.jsonl.
08_fresh_summary.py       summary of raw/fresh_*.jsonl.
09_theory_c_tally.py      theory C's suite re-tallied, paired sign tests.

raw/fresh_boards_a.jsonl, fresh_boards_b.jsonl   S10 unchanged on ten new 5x5 boards (seeds 5 to 14)
raw/fresh_starts.jsonl                           S10 unchanged on starts 10 to 19 of the memo boards
raw/mistake_max_outer200_boards.jsonl            re-check's error, kept: the same job with the crate's
                                                 default of 200 outer steps (the memo's rows used
                                                 max_outer=10000000, which its configuration line
                                                 does not list). Not used in any ruling.
raw/s6m1_snapshot.jsonl                         copy of growth-exponent/results/s6m1.jsonl as read
raw/*.txt                                        outputs of the scripts above

The scratch build: lowrank-rust/{src,Cargo.toml,Cargo.lock,py} and its 5x5 boards copied to
the study scratchpad, `cargo build --release -j 2 --offline`, then
  VECLIB_MAXIMUM_THREADS=1 nice -n 10 ./target/release/lowrank-rust run kernel=struct r=16 \
    inner_iters=20 fast_outer=1 mult_mode=1 restart_outer=1000 round_every=1 round_inner=1 \
    max_outer=10000000 time_limit=120 jobs=2 boards=... starts=... out=... tag=...
and `./target/release/lowrank-rust equiv b5_s0 b5_s1 b5_s2 b5_s3 b5_s4 b5_broken`.

raw/loadcheck.jsonl, loadcheck.txt   the configurations of S3, S6 and S10 alternated on the same six
                                     starts, twice, one job at a time: CPU ms per evaluation under
                                     one load (same scratch build, same command line with
                                     kernel=blas r=32 for S3 and no mult_mode/restart for S3, S6).
raw/equiv_rerun.txt                  last line of the scratch build's `equiv`, and of 05.
