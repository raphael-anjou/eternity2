#!/bin/bash
# TOY BOARDS ONLY. Runs every measurement sequentially (one process at a time), each under
# nice -n 10 and timeout 900. Raw outputs go to raw/. Usage: bash run_all.sh [stage]
# stages: basin beam anneal count (default: all, in that order)
cd "$(dirname "$0")" || exit 1
mkdir -p raw
cc -O2 -Wall -o iso iso.c -lm || exit 1
BOARDS="5_3_4_0 5_3_4_1 5_3_4_2 5_3_4_3 5_3_4_4 6_4_5_0 6_4_5_1 6_4_5_2 6_4_5_3 6_4_5_4 7_4_7_0 7_4_7_1"
CAP=2000000
run() { local out=$1; shift; timeout 900 nice -n 10 ./iso "$@" > "$out"; echo "exit=$? $out"; }
stage=${1:-all}
for b in $BOARDS; do
  if [ "$stage" = all ] || [ "$stage" = basin ]; then
    for ms in 1 2 3; do run "raw/basin_${b}_m${ms}.jsonl" "boards/b_${b}.txt" basin $ms 4 $CAP; done
  fi
  if [ "$stage" = all ] || [ "$stage" = beam ]; then
    run "raw/beam_${b}.jsonl" "boards/b_${b}.txt" beam 200 30
  fi
  if [ "$stage" = all ] || [ "$stage" = anneal ]; then
    run "raw/anneal_${b}.jsonl" "boards/b_${b}.txt" anneal 300 2000000 1 50
  fi
  if [ "$stage" = all ] || [ "$stage" = count ]; then
    run "raw/count_${b}.jsonl" "boards/b_${b}.txt" count 4 20000000000
  fi
done
