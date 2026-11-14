#!/bin/bash
# TOY BOARDS ONLY. Third driver (18:40), after run_rest.sh was stopped at the exact counts: the
# K=4 exact count did not finish 5x5 seed 0 in 2e9 nodes (133 s), and the Knuth and hybrid
# estimators had useless variance. So: exact counts up to j=3 on 5x5, up to j=2 on one 6x6 board,
# then the remaining M1 basins, the beam bounds, and the 7x7 M1 basins last (cap 1e6).
# One process at a time, nice -n 10, every run under timeout (900 or less).
cd "$(dirname "$0")" || exit 1
cc -O2 -Wall -o iso iso.c -lm || exit 1
run() { local t=$1 out=$2; shift 2; timeout $t nice -n 10 ./iso "$@" > "$out"; echo "exit=$? $out $(date +%H:%M:%S)"; }
for s in 0 1 2 3 4; do run 300 "raw/count_5_3_4_${s}.jsonl" "boards/b_5_3_4_${s}.txt" count 3 20000000000; done
run 300 "raw/count_6_4_5_1.jsonl" "boards/b_6_4_5_1.txt" count 2 20000000000
for b in 6_4_5_3 6_4_5_4; do run 900 "raw/basin_${b}_m1.jsonl" "boards/b_${b}.txt" basin 1 4 2000000; done
for b in 5_3_4_0 5_3_4_1 5_3_4_2 5_3_4_3 5_3_4_4 6_4_5_0 6_4_5_1 6_4_5_2 6_4_5_3 6_4_5_4 7_4_7_0 7_4_7_1; do
  run 120 "raw/beam_${b}.jsonl" "boards/b_${b}.txt" beam 200 30
done
for b in 7_4_7_0 7_4_7_1; do run 300 "raw/basin_${b}_m1.jsonl" "boards/b_${b}.txt" basin 1 4 1000000; done
echo "ALL DONE $(date +%H:%M:%S)"
