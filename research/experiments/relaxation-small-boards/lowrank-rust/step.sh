#!/bin/sh
# TOY. One step of the table on the full suite: five 5x5 boards x 10 fixed starts (NumPy's
# init_U seeds), cap time_limit (CPU s, default 480), then the broken control x 10 starts, cap 60 s.
# Usage: ./step.sh <tag> <binary> [key=value ...]   -> results/<tag>.jsonl
cd "$(dirname "$0")"
tag=$1; bin=$2; shift 2
export VECLIB_MAXIMUM_THREADS=1
nice -n 10 $bin run kernel=${KERNEL:-blas} jobs=${JOBS:-3} out=results/$tag.jsonl tag=$tag \
  boards=b5_s0,b5_s1,b5_s2,b5_s3,b5_s4 starts=0-9 round_every=1 round_inner=1 time_limit=${TL:-480} "$@"
nice -n 10 $bin run kernel=${KERNEL:-blas} jobs=${JOBS:-3} out=results/$tag.jsonl tag=$tag \
  boards=b5_broken starts=0-9 round_every=1 round_inner=1 time_limit=60 "$@"
