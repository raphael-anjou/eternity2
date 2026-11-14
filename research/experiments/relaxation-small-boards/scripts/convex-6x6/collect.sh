#!/bin/sh
# TOY BOARDS ONLY. Pulls every PROGRESS file of the 6x6 runs from the lab machines (and the queue
# logs), then prints one summary table with the local 5x5 validation runs.
# Usage: sh collect.sh
cd "$(dirname "$0")" || exit 1
for h in x86 m1; do
  mkdir -p "raw/remote/$h"
  scp -o BatchMode=yes -q "$h:./convex6/runs/PROGRESS_*" "$h:./convex6/runs/queue_*.log" "raw/remote/$h/" \
    || echo "($h: nothing to pull yet or unreachable)"
done
for h in x86 m1; do echo "== $h queue"; cat raw/remote/$h/queue_*.log 2>/dev/null; done
python3 -I collect_table.py raw/val5/PROGRESS_*.txt raw/remote/*/PROGRESS_*.txt
