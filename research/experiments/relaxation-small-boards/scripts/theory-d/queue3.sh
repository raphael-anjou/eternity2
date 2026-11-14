#!/bin/bash
cd "$(dirname "$0")"
until grep -q final raw/admm_s1_knn0_kz99_tf1_l2_rho300.jsonl 2>/dev/null; do sleep 5; done
PY="uv run --with numpy --with scipy python -I -B"
nice -n 10 $PY admm_var.py 5 3 4 1 300 420 99 99 1 l2 > raw/admm_s1_knn99_kz99_tf1_l2_rho300.jsonl 2>> raw/err.txt
nice -n 10 $PY admm_agnn.py 5 3 4 1 300 480 1 99 1 l2 500 NW > raw/agnn_s1_NW_l2_rho300.jsonl 2>> raw/err.txt
nice -n 10 $PY admm_agnn.py 5 3 4 1 300 480 1 99 1 l2 500 N > raw/agnn_s1_N_l2_rho300.jsonl 2>> raw/err.txt
nice -n 10 $PY admm_var.py 5 3 4 1 300 480 2 99 1 l2 > raw/admm_s1_knn2_kz99_tf1_l2_rho300.jsonl 2>> raw/err.txt
nice -n 10 $PY admm_var.py 5 3 4 2 300 300 1 99 1 l2 > raw/admm_s2_knn1_kz99_tf1_l2_rho300.jsonl 2>> raw/err.txt
nice -n 10 $PY admm_agg.py 5 3 4 1 300 300 1 0 1 l2 > raw/agg_s1_knn1_l2_rho300.jsonl 2>> raw/err.txt
nice -n 10 $PY admm_var.py 5 3 4 1 300 240 1 1 0 l2 > raw/admm_s1_knn1_kz1_tf0_l2_rho300.jsonl 2>> raw/err.txt
echo done > raw/QUEUE3_DONE
