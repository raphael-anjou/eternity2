#!/bin/sh
# TOY. Exploration on a mini suite (boards s1 and s4, starts 0 and 1) before a full-suite step.
# Usage: ./explore.sh <tag> [key=value ...]   (rows appended to results/explore/explore.jsonl)
cd "$(dirname "$0")"
tag=$1; shift
VECLIB_MAXIMUM_THREADS=1 nice -n 10 ${BIN:-./target/release/lowrank-rust} run kernel=blas jobs=${JOBS:-1} \
  out=results/explore/explore.jsonl tag="$tag" boards=${BOARDS:-b5_s1,b5_s4} starts=${STARTS:-0-1} \
  round_every=1 round_inner=1 time_limit=${TL:-150} "$@"
