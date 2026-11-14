#!/bin/sh
# learning-vs-propagation: one job = one arm on one TOY board, one thread, wall limit 3600 s (internal limits 3500 s).
# usage: sh job.sh ARM BOARD      (BOARD like 6_3; run from this directory)
cd "$(dirname "$0")"
A=$1; B=$2; id=${A}_$B; mkdir -p out
CL="./clasp -q --stats=2 --time-limit=3500"
s=$(date +%s)
case $A in
  T)   timeout 3600 ./rowscan 400000000000000 < boards/$B.rs ;;
  AC)  timeout 3600 ./dyn8 file:boards/$B.dyn AC ROW exact 100000000000000 3500 ;;
  WD)  timeout 3600 ./dyn8 file:boards/$B.dyn AC WDEG exact 100000000000000 3500 ;;
  U0|U0f) N=0; [ $A = U0f ] && N=1; timeout 3600 $CL -n $N --no-lookback --lookahead=no --heuristic=None boards/$B.cnf ;;
  U1|U1f) N=0; [ $A = U1f ] && N=1; timeout 3600 $CL -n $N --heuristic=None boards/$B.cnf ;;
  L0|L0f) N=0; [ $A = L0f ] && N=1; timeout 3600 $CL -n $N --no-lookback boards/$B.cnf ;;
  L1|L1f) N=0; [ $A = L1f ] && N=1; timeout 3600 $CL -n $N --lookahead=atom --heuristic=Unit boards/$B.cnf ;;
  C|Cf)   N=0; [ $A = Cf ] && N=1; timeout 3600 $CL -n $N boards/$B.cnf ;;
  K)   timeout 3600 uv run -q --with python-sat python3 -I cdcl_pysat.py boards/$B.cnf 3500 ;;
esac > out/$id.out 2> out/$id.err
rc=$?
echo "$id rc=$rc sec=$(( $(date +%s) - s )) host=$(hostname -s) end=$(date +%H:%M:%S)" >> done.txt
echo "$(wc -l < done.txt) jobs done; last $id at $(date +%H:%M:%S)" > PROGRESS
