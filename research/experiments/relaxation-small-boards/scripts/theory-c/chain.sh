#!/bin/sh
# TOY. One process at a time (1 core), nice 10, each process under 10 minutes.
cd "$(dirname "$0")"
PY="nice -n 10 uv run --with numpy --with scipy python -I -B"
bmrun() { $PY bm_obj.py $1 $2 32 0 2 raw/bm_obj.jsonl 240 25000 >> raw/chain.log 2>&1; }
drrun() { timeout 590 $PY dr_obj.py $1 $2 $3 $4 6000 480 raw/dr_obj.jsonl 25 2>&1 | grep final | cut -c1-1200 >> raw/chain.log; }
bmrun 1 conc
bmrun 1 l2
bmrun 1 lin
drrun 1 feas 0 0
drrun 1 lin 0.1 5
bmrun 2 conc
bmrun 2 l2
bmrun 2 lin
drrun 1 lin 0.01 5
drrun 1 l2 300 0
bmrun 1 none
bmrun 1 linconc
echo CHAIN_DONE >> raw/chain.log
