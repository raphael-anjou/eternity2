#!/usr/bin/env bash
# hall-on-mrv pipeline (PREREG.md). stage = all | build | check | test | trace | analyze
# test/trace skip any raw file that already exists; delete results/test to rerun.
set -euo pipefail
cd "$(dirname "$0")"
STAGE="${1:-all}"
T="${THREADS:-2}"
export RAYON_NUM_THREADS="$T"
BIN=compute/target/release/hall_on_mrv
ALL=bb-mrv,hall-off,hall-v1,hall-v2,cost-v1,cost-v2,mrv-rand-1,mrv-rand-2,mrv-rand-3

build() { cargo build --release -j "$T" --manifest-path compute/Cargo.toml; }
check() { cargo test --release -j "$T" --manifest-path compute/Cargo.toml; }
rung() { # rung arms reps
  local out="results/test/$1.jsonl"
  [ -s "$out" ] && { echo "skip $out (exists)"; return; }
  mkdir -p results/test
  /usr/bin/time -p nice "$BIN" run --rung "$1" --role test --arms "$2" --reps "$3" --threads "$T" \
    --cap 1000000 --guard 600 --out "$out" 2>"$out.log"
}
test_runs() {
  rung r6 "$ALL,scb" 5
  rung r8 "$ALL" 1
  rung r7 "$ALL,scb" 1
}
trace() {
  mkdir -p results/trace
  for spec in r6:calibration r7:calibration r8:calibration r6:test; do
    for arm in hall-v1 hall-v2; do
      local out="results/trace/${spec%%:*}-${spec#*:}-$arm.jsonl"
      [ -s "$out" ] || nice "$BIN" trace-check --rung "${spec%%:*}" --role "${spec#*:}" --arm "$arm" --out "$out" 2>/dev/null
    done
  done
}
analyze() { python3 analysis/analyze.py; }

case "$STAGE" in
  build) build ;;
  check) check ;;
  test) test_runs ;;
  trace) trace ;;
  analyze) analyze ;;
  all) build; check; test_runs; trace; analyze ;;
  *) echo "unknown stage $STAGE"; exit 2 ;;
esac
