#!/bin/sh
# On one host: the one-row port gate, then a stage's job list for this host.
# Usage: scripts/host_run.sh HOST STAGE P   (from anywhere; writes host-STAGE.log)
cd "$(dirname "$0")/.."
h=$1; st=$2; p=$3
mkdir -p jobs
python3 -I scripts/jobs.py gate1 "$h" > "jobs/gate1-$h.tsv"
sh scripts/run_jobs.sh "jobs/gate1-$h.tsv" 1
python3 -I scripts/gate1_check.py "$h" || { echo "GATE FAILED on $h"; exit 1; }
python3 -I scripts/jobs.py "$st" "$h" > "jobs/$st-$h.tsv"
sh scripts/run_jobs.sh "jobs/$st-$h.tsv" "$p"
echo "FINISHED $st on $h"
