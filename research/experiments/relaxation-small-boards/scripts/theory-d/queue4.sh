#!/bin/bash
cd "$(dirname "$0")"
until [ -f raw/QUEUE3_DONE ]; do sleep 5; done
PY="uv run --with numpy --with scipy python -I -B"
nice -n 10 $PY admm_var.py 5 3 4 1 300 480 3 99 1 l2 > raw/admm_s1_knn3_kz99_tf1_l2_rho300.jsonl 2>> raw/err.txt
nice -n 10 $PY kills.py > raw/kills.txt 2>> raw/err.txt
echo done > raw/QUEUE4_DONE
