#!/bin/bash
# Pass 3 (after pass 2): local nonnegativity in the convex lift, 4 at a time, 600 s each.
cd "$(dirname "$0")"
while [ ! -f raw/cmp/DONE2 ]; do sleep 10; done
mkdir -p raw/local && date > raw/local/START
PY="nice -n 10 uv run --with numpy --with scipy python -I -B"
printf '%s\n' "1 0" "1 1" "1 2" "2 1" | xargs -P 4 -L 1 sh -c "$PY local_nonneg.py 5 3 4 \$0 \$1 3000 600 > raw/local/ln_s\$0_d\$1.jsonl 2> raw/local/ln_s\$0_d\$1.err"
date > raw/local/DONE
