#!/bin/sh
# Run one job line (id, stage, binary, args) through the prereg guard.
# Usage: scripts/one.sh ID STAGE BIN ARGS...   (from the experiment directory)
set -e
id=$1; stage=$2; b=$3; shift 3
case $b in
  qk) bin=engine/target/release/qk ;;
  rpp) bin=../real-puzzle-profile/engine/target/release/rpp ;;
  *) echo "unknown binary $b" >&2; exit 2 ;;
esac
mkdir -p results/raw
[ -s results/raw/$id.jsonl ] && { echo "skip $id (done)"; exit 0; }
nice -n 10 python3 ../common/prereg_guard.py . "$stage" -- $bin "$@" > results/raw/$id.jsonl.part
mv results/raw/$id.jsonl.part results/raw/$id.jsonl
echo "done $id"
