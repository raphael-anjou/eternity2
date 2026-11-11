#!/usr/bin/env bash
# Cut-multiset (Hall matching) pruning on the hard ladder. Deterministic given
# the committed seeds; node counts do not depend on THREADS, wall clock does.
#
#   ./run.sh            # everything (about 15 min on 8 vCPU)
#   ./run.sh check      # build, tests, instance checks, fork-off equivalence
#   ./run.sh calibrate | test | analyze
set -euo pipefail
cd "$(dirname "$0")"
THREADS="${THREADS:-8}"
STAGE="${1:-all}"
BIN=compute/target/release/cut_multiset
export HARD_LADDER_ROOT="$PWD/../hard-ladder"
V1=cmp-hall-frontier

cargo build --release --manifest-path compute/Cargo.toml
"$BIN" check-instances   # committed ladder files == generator
"$BIN" check-c10         # instances/8-c10-*.jsonl == generator

case "$STAGE" in
  check|all)
    cargo test --release --manifest-path compute/Cargo.toml
    "$BIN" run --solver cmp-fork-off --role test --ladder primary --sizes 6,7 --threads "$THREADS" \
      --out results/test/primary/cmp-fork-off.jsonl
    python3 analysis/equivalence.py
    ;;&
  calibrate|all)
    for s in scb-gacolor_ac3_random $V1 cmp-hall-all cmp-hall-frontier-k4 cmp-hall-all-k4; do
      "$BIN" run-c10 --solver "$s" --role calibration --threads "$THREADS"
    done
    for s in $V1 cmp-hall-all cmp-hall-frontier-k4 cmp-hall-all-k4; do
      "$BIN" run --solver "$s" --role calibration --ladder primary --sizes 6,7 --threads "$THREADS" \
        --out "results/calibration/primary/$s.jsonl"
    done
    python3 analysis/analyze.py calibration
    ;;&
  test|all)
    V2=$(python3 -c 'import json;print(json.load(open("results/calibration/selection.json"))["V2"])')
    for s in scb-gacolor_ac3_random hp-dfs; do
      "$BIN" run-c10 --solver "$s" --role test --threads "$THREADS"
    done
    for s in $V1 "$V2"; do
      "$BIN" run --solver "$s" --role test --ladder primary --sizes 6,7 --threads "$THREADS" \
        --out "results/test/primary/$s.jsonl"
      "$BIN" run-c10 --solver "$s" --role test --threads "$THREADS"
    done
    ;;&
  analyze|all)
    V2=$(python3 -c 'import json;print(json.load(open("results/calibration/selection.json"))["V2"])')
    python3 analysis/analyze.py test "$V2"
    ;;
esac
