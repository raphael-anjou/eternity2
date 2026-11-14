#!/bin/sh
# Kernel micro-benchmark, Rust against NumPy, same sizes. At most 2 cores: the Rust runs use
# RAYON_NUM_THREADS=2 with Accelerate held to 1 thread; NumPy runs use 1 BLAS thread.
# Sizes: 5x5 (P 478, K 27203 measured), 6x6 (P 1290, K 154766 measured), synthetic P 20000 with
# K 2e7 (0.05 P^2; the zero share at 10x10 is 0.036 P^2).
cd "$(dirname "$0")"
out=raw/kernel_bench.jsonl
: > $out
for cfg in "478 27203 16 30" "478 27203 64 30" "1290 154766 16 15" "1290 154766 64 15" "20000 20000000 16 2" "20000 20000000 64 2"; do
  RAYON_NUM_THREADS=2 VECLIB_MAXIMUM_THREADS=1 nice -n 10 ./kernel-bench/target/release/kernel-bench $cfg >> $out
  VECLIB_MAXIMUM_THREADS=1 nice -n 10 uv run --with numpy --with scipy python -I kernel_numpy.py $cfg >> $out
done
uptime >> raw/kernel_bench_loadavg.txt
