#!/bin/sh
# Wait until no runner of WAITFILE is left, then run JOBFILE with P parallel jobs.
# Usage: scripts/after.sh WAITFILE JOBFILE P
cd "$(dirname "$0")/.."
while pgrep -f "run_jobs.sh $1" > /dev/null; do sleep 30; done
sh scripts/run_jobs.sh "$2" "$3"
