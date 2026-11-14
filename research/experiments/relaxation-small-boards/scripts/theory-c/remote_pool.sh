#!/bin/sh
# TOY, 5x5 only. Runs on m1 under ./theory-c/, 4 processes at most, nice 10,
# one start per process (each capped at 840 s and 12,000 evaluations; the first 33 runs used 240 s and 25,000). A job whose output file
# exists is skipped, so the pool can be restarted with another job order.
# Usage: nohup sh remote_pool.sh jobs.txt > pool.log 2>&1 &
cd "$(dirname "$0")"
mkdir -p raw/suite
export PATH="$HOME/.local/bin:$PATH" VECLIB_MAXIMUM_THREADS=1
xargs -P 4 -L 1 sh -c 'seed=$0; mode=$1; st=$2; o=raw/suite/${mode}_${seed}_${st}.jsonl; [ -s $o ] || nice -n 10 uv run --with numpy --with scipy python -I -B bm_obj.py $seed $mode 32 $st $((st+1)) $o ${TCAP:-840} ${ECAP:-12000} >> raw/suite.log 2>&1' < "$1"
echo POOL_DONE >> raw/suite.log
