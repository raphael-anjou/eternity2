#!/bin/bash
# up to 4 processes at once (coordinator, 00:20), one thread each, nice 10, 5x5 only, under 15 minutes per run
cd "$(dirname "$0")"
until grep -q final raw/admm_s2_knn2_kz99_tf1_l2_rho300.jsonl 2>/dev/null; do sleep 5; done
cat > raw/pool_jobs.txt <<'JOBS'
admm_var.py 5 3 4 0 300 720 99 99 1 l2|p_s0_full
admm_var.py 5 3 4 0 300 720 2 99 1 l2|p_s0_knn2
admm_var.py 5 3 4 3 300 720 99 99 1 l2|p_s3_full
admm_var.py 5 3 4 3 300 720 2 99 1 l2|p_s3_knn2
admm_var.py 5 3 4 4 300 720 99 99 1 l2|p_s4_full
admm_var.py 5 3 4 4 300 720 2 99 1 l2|p_s4_knn2
admm_var.py 5 3 4 2 300 720 99 99 1 l2|p_s2_full
admm_var.py 5 3 4 2 300 840 2 99 1 l2|p_s2_knn2_long
admm_mask.py 5 3 4 1 300 600 0 99 1 l2 500 manh:2|p_s1_manh2
admm_mask.py 5 3 4 1 300 600 0 99 1 l2 500 manh:3|p_s1_manh3
admm_mask.py 5 3 4 1 300 600 0 99 1 l2 500 rand:63210:1|p_s1_rand63210_a
admm_mask.py 5 3 4 1 300 600 0 99 1 l2 500 rand:63210:2|p_s1_rand63210_b
admm_mask.py 5 3 4 1 300 600 0 99 1 l2 500 randlike:2:1|p_s1_randlike_manh2
admm_var.py 5 3 4 0 300 300 1 99 1 l2|p_s0_knn1
admm_var.py 5 3 4 3 300 300 1 99 1 l2|p_s3_knn1
admm_var.py 5 3 4 4 300 300 1 99 1 l2|p_s4_knn1
admm_var.py 5 3 4 1 300 600 2 99 1 leak|p_s1_knn2_leak
admm_var.py 5 3 4 1 300 600 99 99 1 leak|p_s1_full_leak
admm_mask.py 5 3 4 2 300 720 0 99 1 l2 500 manh:2|p_s2_manh2
admm_var.py 5 3 4 1 300 600 1 99 1 leak|p_s1_knn1_leak
JOBS
cat raw/pool_jobs.txt | xargs -P 4 -I{} sh -c 'j="{}"; cmd="${j%%|*}"; name="${j##*|}"; nice -n 10 uv run --with numpy --with scipy python -I -B $cmd > raw/$name.jsonl 2>> raw/err.txt'
nice -n 10 uv run --with numpy --with scipy python -I -B kills.py > raw/kills.txt 2>> raw/err.txt
echo done > raw/POOL_DONE
