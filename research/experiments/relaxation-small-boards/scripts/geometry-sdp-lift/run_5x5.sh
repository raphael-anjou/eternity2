#!/bin/sh
# TOY. Every 5x5 board, one part per process, each under timeout 900, single-threaded.
cd "$(dirname "$0")"
for s in 0 1 2; do
  for part in lp sdp lpcert sdpcert base; do
    tl=400; [ $part = sdp ] && tl=820
    VECLIB_MAXIMUM_THREADS=1 /usr/bin/time -l timeout 900 nice -n 10 uv run --with scs --with numpy --with scipy python -I run_board.py 5 3 4 $s raw/ctrl_5_3_4_$s.json raw/res_5_3_4_$s.json $part 1e-5 $tl > raw/log_5_3_4_${s}_$part.txt 2>&1
    echo "board 5 3 4 $s part $part exit $? $(grep -E ' real' raw/log_5_3_4_${s}_$part.txt) $(grep 'maximum resident' raw/log_5_3_4_${s}_$part.txt)"
  done
done
