#!/bin/sh
# TOY BOARDS ONLY. Growth-exponent runs on x86 (Linux, pure-Rust kernel blk: same objective and
# gradient as the struct kernel to 1e-13, trajectories differ in rounding only, as in the
# lowrank-rust x86 sweep). The best configuration of lowrank-rust.md (S10), unchanged unless
# extra key=value arguments are given; one thread per job, JOBS parallel jobs (default 13).
# Usage: setsid nohup ./run.sh <tag> <boards> <starts> <cpu_limit_s> [key=value ...] > logs/<tag>.log 2>&1 &
# Rows in results/<tag>.jsonl, progress lines in PROGRESS.
cd "$(dirname "$0")"
tag=$1; boards=$2; starts=$3; tl=$4; shift 4
mkdir -p results logs
echo "$(date +%T) start $tag $boards $starts tl=$tl $*" >> PROGRESS
nice -n 10 ./target/release/lowrank-rust run kernel=blk r=16 inner_iters=20 fast_outer=1 mult_mode=1 \
  restart_outer=1000 round_every=1 round_inner=1 max_outer=100000000 time_limit=$tl wall_limit=1e9 \
  jobs=${JOBS:-13} boards=$boards starts=$starts out=results/$tag.jsonl tag=$tag "$@"
echo "$(date +%T) done $tag exit $?" >> PROGRESS
