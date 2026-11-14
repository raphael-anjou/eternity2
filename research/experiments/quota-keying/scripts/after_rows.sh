#!/bin/sh
# Wait until RAWDIR holds N finished rows matching PREFIX (no .part), then run JOBFILE with P jobs.
# Usage: scripts/after_rows.sh RAWDIR PREFIX N JOBFILE P
cd "$(dirname "$0")/.."
while [ "$(ls "$1" 2>/dev/null | grep "^$2" | grep -vc '\.part$')" -lt "$3" ]; do sleep 30; done
sh scripts/run_jobs.sh "$4" "$5"
