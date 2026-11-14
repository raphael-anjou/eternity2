#!/bin/sh
# TOY. Driver for the 6x6 solves on the lab machine (Linux, 14 cores, ~25 GB free), run under nohup.
# 6x6 seeds 0..4: the SDP lift (at most 3 at a time, 4 threads each, timeout 10800 per solve, solver
# iteration log line-buffered to its own file), then LP and the cheap baselines. Every start, end and
# exit code goes to raw_x86/PROGRESS.txt.
# usage: nohup sh x86_driver.sh BACKEND > raw_x86/driver.out 2>&1 &
cd "$(dirname "$0")"
BACKEND=${1:-mkl}
R=raw_x86
mkdir -p $R
P=$R/PROGRESS.txt
UV="nix shell nixpkgs#uv -c uv run --with scs --with numpy --with scipy python -I"
log() { echo "$(date '+%Y-%m-%d %H:%M:%S') $*" >> $P; }
log "driver start backend=$BACKEND"
for s in 0 1 2; do cp raw/ctrl_6_4_5_$s.json $R/; done

# near-solution controls for the two new seeds (single-threaded C, alongside the first SDPs)
( for s in 3 4; do
    [ -s $R/ctrl_6_4_5_$s.json ] && { log "ctrl seed $s exists, skipped"; continue; }
    log "ctrl seed $s start"
    CC="nix shell nixpkgs#gcc -c gcc" timeout 3600 nice -n 10 $UV controls_c.py 6 4 5 $s 5000000 900 3000000000 900 $R/ctrl_6_4_5_$s.json > $R/log_ctrl_6_4_5_$s.txt 2>&1
    log "ctrl seed $s end exit $?"
  done ) &

sdp() {
  s=$1
  log "sdp seed $s start"
  SCS_VERBOSE=1 SCS_LINEBUF=1 SCS_LINEAR_SOLVER=$BACKEND OMP_NUM_THREADS=4 OPENBLAS_NUM_THREADS=4 MKL_NUM_THREADS=4 \
    timeout 10800 nice -n 10 $UV run_board.py 6 4 5 $s $R/ctrl_6_4_5_$s.json $R/res_6_4_5_$s.sdp.json sdp 1e-5 10500 \
    > $R/log_6_4_5_${s}_sdp.txt 2>&1
  e=$?
  log "sdp seed $s end exit $e status=$(python3 -I -c "import json;d=json.load(open('$R/res_6_4_5_$s.sdp.json'))['sdp'];print(d['status'],d['iters'],d['res_pri'],d['res_dual'])" 2>/dev/null)"
}
base() {
  s=$1; part=$2
  log "$part seed $s start"
  SCS_LINEAR_SOLVER=$BACKEND OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 MKL_NUM_THREADS=1 \
    timeout 10800 nice -n 10 $UV run_board.py 6 4 5 $s $R/ctrl_6_4_5_$s.json $R/res_6_4_5_$s.base.json $part 1e-5 10500 \
    > $R/log_6_4_5_${s}_$part.txt 2>&1
  log "$part seed $s end exit $?"
}

# three SDP lanes: seeds (0,3), (1,4), (2); the third lane then runs the baselines
( sdp 0; sdp 3 ) &
( sdp 1; sdp 4 ) &
( sdp 2; for s in 0 1 2 3 4; do base $s lp; base $s base; done ) &
wait
log "driver end"
