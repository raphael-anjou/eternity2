#!/bin/sh
# TOY. Parameter sweep on the sweep machine (Linux, pure-Rust kernel blk, no BLAS): every config on
# the five 5x5 boards x 10 starts (budget ME evaluations, default 60000) and the broken control x 10
# starts (budget MB evaluations, default 10000; exact verification cannot accept a board there).
# Usage: setsid nohup ./sweep.sh configs.txt out.jsonl > sweep.log 2>&1 &   (one config per line:
# tag key=value ...). Progress in PROGRESS.
cd "$(dirname "$0")"
cf=$1; out=$2
common="kernel=blk fast_outer=1 jobs=13 round_every=1 round_inner=1 max_outer=10000000 time_limit=1e9 wall_limit=840"
while read -r tag rest; do
  [ -z "$tag" ] && continue
  case "$tag" in \#*) continue;; esac
  echo "$(date +%T) start $tag" >> PROGRESS
  nice -n 10 ./target/release/lowrank-rust run $common out=$out tag=$tag \
    boards=b5_s0,b5_s1,b5_s2,b5_s3,b5_s4 starts=0-9 max_evals=${ME:-60000} $rest 2>/dev/null
  nice -n 10 ./target/release/lowrank-rust run $common out=$out tag=$tag \
    boards=b5_broken starts=0-9 max_evals=${MB:-10000} $rest 2>/dev/null
  echo "$(date +%T) done $tag" >> PROGRESS
done < "$cf"
echo "$(date +%T) ALL DONE" >> PROGRESS
