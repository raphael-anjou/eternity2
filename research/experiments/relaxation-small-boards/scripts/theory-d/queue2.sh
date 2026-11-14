#!/bin/bash
cd "$(dirname "$0")"
until [ -f raw/QUEUE1_DONE ]; do sleep 5; done
PY="uv run --with numpy --with scipy python -I -B"
nice -n 10 $PY admm_agg.py 5 3 4 1 300 480 0 0 1 l2 > raw/agg_s1_knn0_l2_rho300.jsonl 2>> raw/err.txt
nice -n 10 $PY admm_agg.py 5 3 4 1 300 480 1 0 1 l2 > raw/agg_s1_knn1_l2_rho300.jsonl 2>> raw/err.txt
echo done > raw/QUEUE2_DONE
