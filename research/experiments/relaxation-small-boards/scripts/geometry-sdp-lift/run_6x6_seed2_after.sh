#!/bin/sh
# TOY. Seed 2's SDP was stopped after ~10 min because three 6x6 solves at ~8 GB each pushed the
# machine into 17 GB of swap. It restarts here when one of the two other 6x6 SDPs has ended.
cd "$(dirname "$0")"
while [ "$(pgrep -f 'bin/python -I run_board.py 6 4 5 [01] .*sdp.json sdp' | wc -l)" -ge 2 ]; do sleep 20; done
SCS_VERBOSE=1 VECLIB_MAXIMUM_THREADS=3 OMP_NUM_THREADS=3 /usr/bin/time -l timeout 3600 nice -n 10 uv run --with scs --with numpy --with scipy python -I run_board.py 6 4 5 2 raw/ctrl_6_4_5_2.json raw/res_6_4_5_2.sdp.json sdp 1e-5 3300 > raw/log_6_4_5_2_sdp3600.txt 2>&1
echo "6x6 seed 2 sdp (restart) exit $?" >> raw/run_6x6_status.txt
