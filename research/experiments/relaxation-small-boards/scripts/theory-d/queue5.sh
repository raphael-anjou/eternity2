#!/bin/bash
cd "$(dirname "$0")"
PY="uv run --with numpy --with scipy python -I -B"
nice -n 10 $PY admm_var.py 5 3 4 2 300 480 2 99 1 l2 > raw/admm_s2_knn2_kz99_tf1_l2_rho300.jsonl 2>> raw/err.txt
nice -n 10 $PY admm_mask.py 5 3 4 1 300 480 0 99 1 l2 500 manh:2 > raw/amask_s1_manh2.jsonl 2>> raw/err.txt
M=$(python3 -I -c "import json;print(json.loads(open('raw/amask_s1_manh2.jsonl').readline())['n_nonneg'])")
nice -n 10 $PY admm_mask.py 5 3 4 1 300 480 0 99 1 l2 500 rand:$M:1 > raw/amask_s1_rand_manh2count.jsonl 2>> raw/err.txt
nice -n 10 $PY admm_mask.py 5 3 4 1 300 480 0 99 1 l2 500 rand:63210:1 > raw/amask_s1_rand_cheb2count.jsonl 2>> raw/err.txt
nice -n 10 $PY kills.py > raw/kills.txt 2>> raw/err.txt
echo done > raw/QUEUE5_DONE
