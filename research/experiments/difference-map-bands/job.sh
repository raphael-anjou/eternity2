#!/usr/bin/env bash
# One test job, run through the preregistration guard (called by run.sh via xargs).
#   job.sh dm <k> <ladder> <size> <seeds>      difference map, run seeds and cap from run.sh env
#   job.sh base <solver> <ladder> <sizes|-> <seeds>
set -euo pipefail
cd "$(dirname "$0")"
G="python3 ../common/prereg_guard.py ."
H=../hard-ladder/instances
mkdir -p results/raw/parts
if [ "$1" = dm ]; then
  k="$2"; lad="$3"; n="$4"; seeds="$5"
  out="results/raw/parts/dm-k$k-$lad-n$n-s$seeds.jsonl"
  [ -s "$out" ] && { echo "skip $out"; exit 0; }
  $G "test-dm-k$k" -- nice compute/target/release/dmb run --instances "$H/$lad-test.jsonl" \
    --sizes "$n" --seeds "$seeds" --k "$k" --runs "$RUNS" --cap "$CAP" --beta "$BETA" --out "$out"
else
  s="$2"; lad="$3"; sz="$4"; seeds="$5"
  out="results/raw/parts/$s-$lad-n$sz-s$seeds.jsonl"
  [ -s "$out" ] && { echo "skip $out"; exit 0; }
  szarg=()
  [ "$sz" != "-" ] && szarg=(--sizes "$sz")
  $G "test-base-$s" -- nice ../mrv-baseline/compute/target/release/mrv_baseline run --solver "$s" \
    --ladder "$lad" --role test --seeds "$seeds" ${szarg[@]+"${szarg[@]}"} --cap 1000000 --guard 600 --threads 1 --out "$out"
fi
