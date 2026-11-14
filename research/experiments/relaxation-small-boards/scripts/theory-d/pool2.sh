#!/bin/bash
# second wave, same limits: penalty 3000 (the engineering memo's setting for seeds 0 and 2) on the runs that penalty 300 left undecided
cd "$(dirname "$0")"
until [ -f raw/POOL_DONE ]; do sleep 5; done
cat > raw/pool2_jobs.txt <<'JOBS'
admm_var.py 5 3 4 0 3000 840 2 99 1 l2|q_s0_knn2_rho3000
admm_var.py 5 3 4 0 3000 840 99 99 1 l2|q_s0_full_rho3000
admm_var.py 5 3 4 3 3000 840 99 99 1 l2|q_s3_full_rho3000
admm_var.py 5 3 4 2 3000 840 2 99 1 l2|q_s2_knn2_rho3000
JOBS
cat raw/pool2_jobs.txt | xargs -P 4 -I{} sh -c 'j="{}"; cmd="${j%%|*}"; name="${j##*|}"; nice -n 10 uv run --with numpy --with scipy python -I -B $cmd > raw/$name.jsonl 2>> raw/err.txt'
nice -n 10 uv run --with numpy --with scipy python -I -B kills.py > raw/kills.txt 2>> raw/err.txt
echo done > raw/POOL2_DONE
