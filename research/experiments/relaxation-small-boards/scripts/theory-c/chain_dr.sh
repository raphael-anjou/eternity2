#!/bin/sh
# TOY. Local, one process at a time (1 core), nice 10, each under 10 minutes. Splitting diagnostics.
cd "$(dirname "$0")"
PY="nice -n 10 uv run --with numpy --with scipy python -I -B"
drrun() { timeout 590 $PY dr_obj.py $1 $2 $3 $4 6000 500 raw/dr_obj.jsonl 25 2>&1 | grep final | cut -c1-1200 >> raw/chain_dr.log; }
$PY degen.py 0 1 2 3 4 > raw/degen.jsonl 2>&1
drrun 1 feas 0 0
drrun 1 lin 0.1 5
drrun 1 l2 300 0
drrun 1 lin 0.01 5
drrun 2 feas 0 0
drrun 2 lin 0.1 5
echo CHAIN_DONE >> raw/chain_dr.log
