#!/bin/sh
# TOY. Local, one process at a time (1 core), nice 10, each under 10 minutes.
cd "$(dirname "$0")"
PY="nice -n 10 uv run --with numpy --with scipy python -I -B"
while pgrep -f "dr_obj.py" > /dev/null; do sleep 5; done
drrun() { timeout 590 $PY dr_obj.py $1 $2 $3 $4 6000 500 raw/dr_obj.jsonl 25 $5 2>&1 | grep final | cut -c1-1200 >> raw/chain_dr.log; }
drrun 1 feas 0 0 3
drrun 1 feas 0 0 0.33
drrun 2 feas 0 0 1
drrun 2 lin 0.1 5 1
drrun 1 feas 0 0 10
drrun 1 lin 0.1 6 1
echo CHAIN_DONE >> raw/chain_dr.log
