#!/bin/sh
# learning-real-prefixes: start detached workers, one per core. usage: sh launch.sh QUEUE cpu1 cpu2 ...
# With taskset (Linux) each worker and its children are pinned to the named cpu; elsewhere the names are labels only.
# The file uvpath, when present, holds the path of uv for the K arm.
cd "$(dirname "$0")"
[ -f uvpath ] && UV=$(cat uvpath) && export UV
q=$1; shift
for c in "$@"; do
  if command -v taskset > /dev/null; then setsid nohup taskset -c $c python3 -I worker.py $q cpu$c > /dev/null 2>&1 < /dev/null &
  else nohup python3 -I worker.py $q w$c > /dev/null 2>&1 < /dev/null & fi
done
