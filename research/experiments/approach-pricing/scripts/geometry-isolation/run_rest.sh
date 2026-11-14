#!/bin/bash
# TOY BOARDS ONLY. Second driver, after the scope cut of 18:15 (see the memo): run_all.sh did the
# basin stage for all move sets on the five 5x5 boards and on 6x6 seeds 0 and 1; this script runs,
# in priority order, the annealer with FDC and trap check, the counts, the M1 basins on the
# remaining 6x6 boards, the beam bounds, and last the 7x7 M1 basins. One process at a time, each
# under nice -n 10 and timeout 900.
cd "$(dirname "$0")" || exit 1
mkdir -p raw
cc -O2 -Wall -o iso iso.c -lm || exit 1
ALL="5_3_4_0 5_3_4_1 5_3_4_2 5_3_4_3 5_3_4_4 6_4_5_0 6_4_5_1 6_4_5_2 6_4_5_3 6_4_5_4 7_4_7_0 7_4_7_1"
run() { local out=$1; shift; timeout 900 nice -n 10 ./iso "$@" > "$out"; echo "exit=$? $out $(date +%H:%M:%S)"; }
for b in $ALL; do run "raw/anneal_${b}.jsonl" "boards/b_${b}.txt" anneal 200 2000000 1 50 2.0 0.05 4 200000; done
for b in $ALL; do
  run "raw/count_${b}.jsonl" "boards/b_${b}.txt" count 4 2000000000
  if grep -q '"aborted":1' "raw/count_${b}.jsonl"; then
    run "raw/count_${b}_knuth.jsonl" "boards/b_${b}.txt" knuth 4 2000000 1
  fi
done
for b in 6_4_5_2 6_4_5_3 6_4_5_4; do run "raw/basin_${b}_m1.jsonl" "boards/b_${b}.txt" basin 1 4 2000000; done
for b in $ALL; do run "raw/beam_${b}.jsonl" "boards/b_${b}.txt" beam 200 30; done
for b in 7_4_7_0 7_4_7_1; do run "raw/basin_${b}_m1.jsonl" "boards/b_${b}.txt" basin 1 4 1000000; done
echo "ALL DONE $(date +%H:%M:%S)"
