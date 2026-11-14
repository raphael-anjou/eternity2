#!/bin/sh
# TOY. After the 6x6 SDPs (coordinator's second budget): at most 2 processes at a time, one solver
# thread each, nothing longer than 20 minutes (timeout 1200). Three sequential lanes (the third
# coordinator message allowed 3 local processes); the 6x6 LP for seeds 1 to 4 runs on x86.
cd "$(dirname "$0")"
export VECLIB_MAXIMUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 CERT_BUDGET=1000
PY="uv run --with scs --with numpy --with scipy python -I"
job() {  # n cf ci seed outfile part
  tl=1000; [ "$6" = sdp ] && tl=1100
  /usr/bin/time -l timeout 1200 nice -n 10 $PY run_board.py $1 $2 $3 $4 raw/ctrl_$1_$2_$3_$4.json $5 $6 1e-5 $tl > raw/log_$1_$2_$3_$4_$6_p2.txt 2>&1
  echo "$1x$1 seed $4 $6 exit $? $(grep -E ' real' raw/log_$1_$2_$3_$4_$6_p2.txt)" >> raw/phase2_status.txt
}
( for s in 0 1 2; do job 6 4 5 $s raw/res_6_4_5_$s.base.json base; done
  for p in lp sdp sdpcert lpcert base; do job 5 3 4 2 raw/res_5_3_4_2.json $p; done ) &
( for p in lp sdp sdpcert lpcert base; do job 5 3 4 1 raw/res_5_3_4_1.json $p; done ) &
( for p in sdp sdpcert lpcert base; do job 5 3 4 0 raw/res_5_3_4_0.json $p; done ) &
wait
