#!/bin/sh
# TOY BOARDS ONLY. Mac runs with the memo's best configuration exactly (S10: kernel=struct, Accelerate
# dgemm, one thread per job), start-major job order. Same arguments as run.sh.
cd "$(dirname "$0")"
tag=$1; boards=$2; starts=$3; tl=$4; shift 4
mkdir -p results logs
export VECLIB_MAXIMUM_THREADS=1
echo "$(date +%T) start $tag $boards $starts tl=$tl jobs=${JOBS:-5} $*" >> PROGRESS_mac
nice -n 10 ./target/release/lowrank-rust run kernel=struct r=16 inner_iters=20 fast_outer=1 mult_mode=1 \
  restart_outer=1000 round_every=1 round_inner=1 max_outer=100000000 time_limit=$tl wall_limit=1e9 \
  jobs=${JOBS:-5} order=start boards=$boards starts=$starts out=results/$tag.jsonl tag=$tag "$@"
echo "$(date +%T) done $tag exit $?" >> PROGRESS_mac
