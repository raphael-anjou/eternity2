#!/bin/sh
# TOY. Runs every line of a jobs file (the arguments of run_unit.py) with at most NPROC processes,
# one BLAS thread each, nice 10, timeout 1800 per unit. PROGRESS.txt gets start/end/exit per unit.
# Usage: sh run_jobs.sh JOBS_FILE OUT_DIR NPROC
JOBS=$1; OUT=$2; NP=$3
cd "$(dirname "$0")"
export PATH=$HOME/.local/bin:$PATH OMP_NUM_THREADS=1 VECLIB_MAXIMUM_THREADS=1 OPENBLAS_NUM_THREADS=1
mkdir -p "$OUT"
export OUT
grep -v '^#' "$JOBS" | xargs -P "$NP" -L 1 sh -c 'tag=$(echo "$*" | tr " " "_"); echo "start $tag $(date +%T)" >> "$OUT/PROGRESS.txt"; timeout 1800 nice -n 10 uv run --with numpy --with scipy python -I -B run_unit.py $1 $2 $3 $4 $5 $6 $7 "$OUT/u_$tag.jsonl" $8 $9 > "$OUT/log_$tag.txt" 2>&1; echo "end $tag exit $? $(date +%T)" >> "$OUT/PROGRESS.txt"' sh
echo "ALL DONE $(date +%T)" >> "$OUT/PROGRESS.txt"
