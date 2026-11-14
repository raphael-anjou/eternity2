#!/bin/bash
# TOY BOARDS ONLY. T1 driver. Usage: bash run.sh JOBFILE PARALLEL
# Each job is one single-threaded process of iso_t (iso.c plus a thread-CPU timer per run),
# re-check schedule T 0.8 -> 0.2, one snapshot. Rows: raw/<tag>_<board>_s<seed>.jsonl, log: raw/log.txt
cd "$(dirname "$0")" || exit 1
mkdir -p raw
[ -x iso_t ] || cc -O2 -Wall -o iso_t iso_t.c -lm || exit 1
one() { local tag=$1 b=$2 runs=$3 steps=$4 seed=$5; local t0=$(date +%s)
  nice -n 10 ./iso_t "boards/b_${b}.txt" anneal "$runs" "$steps" "$seed" 1 0.8 0.2 > "raw/${tag}_${b}_s${seed}.jsonl"
  echo "exit=$? ${tag}_${b}_s${seed} $(( $(date +%s) - t0 ))s $(hostname -s)" >> raw/log.txt; }
export -f one
echo "start $1 $(date +%H:%M:%S) $(hostname -s)" >> raw/log.txt
xargs -P "$2" -L 1 bash -c 'one "$@"' _ < "$1"
echo "done $1 $(date +%H:%M:%S)" >> raw/log.txt
