#!/bin/bash
# one process at a time, nice 10, every run capped under 10 minutes
cd "$(dirname "$0")"
PY="uv run --with numpy --with scipy python -I -B"
for s in 0 1 2; do nice -n 10 $PY rootfilter.py 5 3 4 $s >> raw/rootfilter.jsonl 2>> raw/err.txt; done
for s in 0 1 2; do nice -n 10 $PY wincons.py 5 3 4 $s "2x2,3x3,4x4,3x5,5x3,4x5,5x4" > raw/wincons_$s.jsonl 2>> raw/err.txt; done
for s in 1 2 0; do for m in edge 2x2; do nice -n 10 $PY winlp.py 5 3 4 $s $m 500 >> raw/winlp.jsonl 2>> raw/err.txt; done; done
# weakened lifts: seed 1 first (rho 300 reached the reading at 4,500 iterations in the full form)
run() { nice -n 10 $PY admm_var.py 5 3 4 $1 $2 480 $3 $4 $5 $6 > raw/admm_s$1_knn$3_kz$4_tf$5_$6_rho$2.jsonl 2>> raw/err.txt; }
run 1 300 1 99 1 l2
run 1 300 0 99 1 l2
run 1 300 1 1 0 l2
run 2 300 1 99 1 l2
run 1 300 2 99 1 l2
run 1 300 99 99 1 l2
echo done > raw/QUEUE1_DONE
