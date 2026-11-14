#!/bin/sh
# TOY. Slot-aware runner: launches each line of a jobs file (run_unit.py arguments) as soon as fewer
# than NPROC run_unit.py solver processes are alive on this machine (so it shares slots with units
# still finishing from an earlier batch). One BLAS thread each, nice 10, timeout TIMEOUT (default 1800) per unit.
# PROGRESS.txt in OUT_DIR gets start/end/exit per unit; each start's row is appended to its .jsonl
# as soon as that start finishes.
# Usage: sh run_pool.sh JOBS_FILE OUT_DIR NPROC   (UVCMD="nix shell nixpkgs#uv -c uv" on x86)
JOBS=$1; OUT=$2; NP=$3
cd "$(dirname "$0")"
export PATH=$HOME/.local/bin:$PATH OMP_NUM_THREADS=1 VECLIB_MAXIMUM_THREADS=1 OPENBLAS_NUM_THREADS=1
mkdir -p "$OUT"
grep -v '^#' "$JOBS" | while read -r a b c d e f g h i; do
  while [ "$(pgrep -f 'bin/python3? -I -B run_unit.py' | wc -l)" -ge "$NP" ]; do sleep 15; done
  tag="${a}_${b}_${c}_${d}_${e}_${f}_${g}_${h}_${i}"
  ( echo "start $tag $(date +%T)" >> "$OUT/PROGRESS.txt"
    timeout ${TIMEOUT:-1800} nice -n 10 ${UVCMD:-uv} run --with numpy --with scipy python -I -B run_unit.py $a $b $c $d $e $f $g "$OUT/u_$tag.jsonl" $h $i > "$OUT/log_$tag.txt" 2>&1
    echo "end $tag exit $? $(date +%T)" >> "$OUT/PROGRESS.txt" ) &
  sleep 20
done
wait
echo "ALL DONE $(date +%T)" >> "$OUT/PROGRESS.txt"
