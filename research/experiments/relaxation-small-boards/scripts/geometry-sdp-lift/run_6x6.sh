#!/bin/sh
# TOY. 6x6 boards under the raised limits (coordinator message: all cores, timeout 3600 per run).
# Three SDP solves in parallel, 3 Accelerate threads each (9 in total); the cheap baselines (LP, arc
# consistency, probe B's flow) run alongside, one board at a time, single-threaded.
cd "$(dirname "$0")"
PY="uv run --with scs --with numpy --with scipy python -I"
for s in 0 1 2; do
  ( SCS_VERBOSE=1 VECLIB_MAXIMUM_THREADS=3 OMP_NUM_THREADS=3 /usr/bin/time -l timeout 3600 nice -n 10 $PY run_board.py 6 4 5 $s raw/ctrl_6_4_5_$s.json raw/res_6_4_5_$s.sdp.json sdp 1e-5 3300 > raw/log_6_4_5_${s}_sdp3600.txt 2>&1
    echo "6x6 seed $s sdp exit $?" >> raw/run_6x6_status.txt ) &
done
( for s in 0 1 2; do
    for part in lp base; do
      VECLIB_MAXIMUM_THREADS=1 OMP_NUM_THREADS=1 /usr/bin/time -l timeout 3600 nice -n 10 $PY run_board.py 6 4 5 $s raw/ctrl_6_4_5_$s.json raw/res_6_4_5_$s.base.json $part 1e-5 3300 > raw/log_6_4_5_${s}_$part.txt 2>&1
      echo "6x6 seed $s $part exit $?" >> raw/run_6x6_status.txt
    done
  done ) &
wait
