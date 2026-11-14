#!/bin/sh
# TOY BOARDS ONLY. Runs on a lab machine from ./convex6/. Starts each line of JOBS
# (n cf ci seed objective rho0 seconds) as soon as the machine has an idle core: fewer than CAP
# processes above 50 percent CPU, on two checks 45 s apart (so a slot that another study's pool
# refills within seconds is not taken). One BLAS thread per run, nice 10, detached.
# Usage: nohup sh queue.sh JOBS CAP > runs/queue_JOBS.log 2>&1 < /dev/null &
#        (on x86: UVCMD="nix shell nixpkgs#uv -c uv")
cd "$(dirname "$0")" || exit 1
JOBS=$1; CAP=$2
export OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 VECLIB_MAXIMUM_THREADS=1 MKL_NUM_THREADS=1 PATH=$HOME/.local/bin:$PATH
mkdir -p runs
busy() { ps -Ao pcpu= | awk '$1 > 50 {c++} END {print c+0}'; }
grep -v '^#' "$JOBS" | while read -r n cf ci seed obj rho secs; do
  while :; do
    if [ "$(busy)" -lt "$CAP" ]; then sleep 45; [ "$(busy)" -lt "$CAP" ] && break; fi
    sleep 60
  done
  echo "start $obj $n $cf $ci $seed busy=$(busy) $(date '+%F %T')"
  nohup nice -n 10 ${UVCMD:-uv} run --with numpy --with scipy python -I admm.py $n $cf $ci $seed $obj $rho $secs runs \
    > "runs/stdout_${obj}_${n}_${cf}_${ci}_${seed}.txt" 2>&1 < /dev/null &
  sleep 120
done
echo "queue empty $(date '+%F %T')"
