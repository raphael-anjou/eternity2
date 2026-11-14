#!/bin/sh
# Run one job line (id, stage, args) through the prereg guard; skip finished rows.
# Usage: scripts/one.sh ID STAGE ARGS...   (from the experiment directory)
set -e
id=$1; stage=$2; shift 2
mkdir -p results/raw
[ -s results/raw/$id.jsonl ] && { echo "skip $id (done)"; exit 0; }
nice -n 10 python3 ../common/prereg_guard.py . "$stage" -- engine/target/release/fck "$@" > results/raw/$id.jsonl.part
mv results/raw/$id.jsonl.part results/raw/$id.jsonl
echo "done $id"
