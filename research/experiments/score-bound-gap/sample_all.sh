#!/usr/bin/env bash
# Draw the preregistered test prefixes for every rung (single thread).
set -eu
cd "$(dirname "$0")"
mkdir -p results/test
H=../hard-ladder/instances
for spec in n6-c5:primary-test:6 n7-c6:primary-test:7 n6-c7:plus2-test:6 n7-c8:plus2-test:7 n8-c8:primary-test:8 n8-c10:8-c10-test:8; do
  IFS=: read rung file n <<< "$spec"
  for source in schedule strict; do
    ./compute/target/release/sbg sample --instances $H/$file.jsonl --n $n --source $source \
      --runs 1,2,3,4 --steps 1000000 --k 4 --seed 20261112 \
      --out results/test/prefixes-$rung-$source.jsonl 2> results/test/pools-$rung-$source.log
  done
done
