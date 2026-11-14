#!/bin/bash
# TOY BOARDS ONLY. Second analysis rerun of the memo's own annealer (iso.c, compiled unchanged as iso_ref)
# at larger budgets and one narrower schedule fixed before any run. One process at a time,
# nice -n 10, each call under timeout 590. Rows: raw/<tag>_<board>.jsonl, log: raw/log.txt.
cd "$(dirname "$0")" || exit 1
B=../geometry-isolation/boards
run() { local tag=$1 b=$2; shift 2; local t0=$(date +%s); timeout 590 nice -n 10 ./iso_ref "$B/b_${b}.txt" anneal "$@" > "raw/${tag}_${b}.jsonl"; echo "exit=$? ${tag}_${b} $(( $(date +%s) - t0 ))s" >> raw/log.txt; }
S6="6_4_5_0 6_4_5_1 6_4_5_2 6_4_5_3 6_4_5_4"
# R4: same 2e6 steps, narrower schedule 0.8 -> 0.2 (equal budget, different schedule)
for b in $S6; do run r4_2e6_T0.8-0.2 $b 100 2000000 404 1 0.8 0.2; done
# R1: memo schedule 2.0 -> 0.05, 10x the steps
for b in $S6; do run r1_2e7 $b 30 20000000 111 1 2.0 0.05; done
# R2: memo schedule, 100x the steps
for b in $S6; do run r2_2e8 $b 5 200000000 222 1 2.0 0.05; done
# R5: narrower schedule at 100x the steps
for b in $S6; do run r5_2e8_T0.8-0.2 $b 5 200000000 555 1 0.8 0.2; done
# R6: 7x7, memo schedule, 100x the steps
for b in 7_4_7_0 7_4_7_1; do run r6_2e8 $b 2 200000000 666 1 2.0 0.05; done
# R3: one run of 2e9 steps (a few CPU minutes, inside the relaxation's range) on three boards
for b in 6_4_5_1 6_4_5_3 6_4_5_4; do run r3_2e9 $b 1 2000000000 333 1 2.0 0.05; done
echo "ALL DONE $(date +%H:%M:%S)" >> raw/log.txt
