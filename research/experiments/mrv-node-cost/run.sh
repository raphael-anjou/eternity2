#!/usr/bin/env bash
# Track 0 (bb-mrv node cost). Stages: build | gate | supp | time | analyze | all.
# Skips any raw file that already exists; delete results/raw to rerun.
set -euo pipefail
cd "$(dirname "$0")"
BIN=compute/target/release/mrv_node_cost
RAW=results/raw
stage="${1:-all}"
mkdir -p "$RAW"

build() { cargo build --release -j 2 --manifest-path compute/Cargo.toml; }

once() { # once OUT CMD...
  local out=$1; shift
  if [ -s "$out" ]; then echo "skip $out"; else nice "$@" --out "$out.tmp" && mv "$out.tmp" "$out"; fi
}

gate() {
  once $RAW/gate-r6-cal.jsonl $BIN gate --ladder primary --role calibration --sizes 6
  once $RAW/gate-r6-test.jsonl $BIN gate --ladder primary --role test --sizes 6
  once $RAW/gate-r8-cal.jsonl $BIN gate --ladder 8-c10 --role calibration
  once $RAW/gate-r8-test.jsonl $BIN gate --ladder 8-c10 --role test
  once $RAW/gate-r8-fresh.jsonl $BIN gate --ladder 8-c10-fresh --role test
  once $RAW/gate-r7-cal.jsonl $BIN gate --ladder primary --role calibration --sizes 7
  once $RAW/gate-r7-test.jsonl $BIN gate --ladder primary --role test --sizes 7
}

supp() {
  once $RAW/gate-supp-n9.jsonl $BIN gate --ladder plus2 --role calibration --sizes 9 --seeds 1001-1003 --cap 100000
  once $RAW/gate-supp-n10.jsonl $BIN gate --ladder plus2 --role calibration --sizes 10 --seeds 1001-1001 --cap 100000
}

timing() {
  once $RAW/time-r6.jsonl $BIN time --ladder primary --role test --sizes 6
  once $RAW/time-r8.jsonl $BIN time --ladder 8-c10 --role test
  once $RAW/time-r8-fresh.jsonl $BIN time --ladder 8-c10-fresh --role test
  once $RAW/time-r7.jsonl $BIN time --ladder primary --role test --sizes 7
}

analyze() { python3 -I analysis/analyze.py; }

case "$stage" in
  build) build ;;
  gate) gate ;;
  supp) supp ;;
  time) timing ;;
  analyze) analyze ;;
  all) build; gate; supp; timing; analyze ;;
  *) echo "unknown stage $stage"; exit 1 ;;
esac
