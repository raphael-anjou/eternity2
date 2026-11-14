#!/bin/bash
# Pass 2: the baseline runs of pass 1 that hit the 180 s limit on a real (not broken) board are
# rerun from the same start with a 480 s limit (the low-rank study's stage 1 limit), 4 at a time, after pass 1 has ended.
cd "$(dirname "$0")"
while [ ! -f raw/cmp/DONE ]; do sleep 10; done
python3 -I - > jobs_pass2.txt <<'PYEOF'
import json
PY = "nice -n 10 env OMP_NUM_THREADS=1 VECLIB_MAXIMUM_THREADS=1 OPENBLAS_NUM_THREADS=1 uv run --with numpy --with scipy python -I -B"
for l in open("raw/cmp/bm.jsonl"):
    r = json.loads(l)
    if not r["broken"] and not r["solved"]:
        s, st = r["board"][3], r["start"]
        print(f"{PY} baseline_bm.py 5 3 4 {s} 32 {st} 480 raw/cmp/bm_pass2.jsonl > raw/cmp/log_bm2_{s}_{st}.txt 2>&1")
PYEOF
date > raw/cmp/START2
xargs -P 4 -I{} sh -c '{}' < jobs_pass2.txt
for sd in 0 1; do nice -n 10 uv run --with numpy --with scipy python -I -B verify_barrier.py 5 3 4 $sd 100; done > raw/verify_barrier.jsonl 2> raw/verify_barrier.err
date > raw/cmp/DONE2
