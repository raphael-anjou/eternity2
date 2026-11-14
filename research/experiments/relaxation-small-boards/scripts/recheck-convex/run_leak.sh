#!/bin/sh
# TOY BOARDS ONLY (5x5). re-check runs: the largest-leak objective of ../convex-6x6/admm.py
# (unmodified, read only) on 5x5 (3,4) boards, one process, one thread, nice 10.
# LIMIT is the script's cumulative time limit: 570 for a fresh run; 1140 resumes a 570 s run from
# its checkpoint for 570 s more (the script's own resume), so no process runs over 10 minutes.
# Usage: run_leak.sh LIMIT seed [seed ...]
cd "$(dirname "$0")" || exit 1
limit="$1"; shift
for s in "$@"; do
  VECLIB_MAXIMUM_THREADS=1 OMP_NUM_THREADS=1 nice -n 10 uv run --with numpy --with scipy python -I -B \
    ../convex-6x6/admm.py 5 3 4 "$s" leak 300 "$limit" "raw/leak_s$s" >> "raw/leak_s$s.out" 2>> "raw/leak_s$s.err"
  nice -n 10 uv run --with numpy --with scipy python -I -B verify_point.py "$s" "raw/leak_s$s/ckpt_leak_5_3_4_$s.npy" \
    "raw/x_leak_5_3_4_$s.npy" >> raw/verify.jsonl 2>> raw/verify.err
done
