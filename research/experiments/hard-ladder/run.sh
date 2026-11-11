#!/usr/bin/env bash
# Hard-instance ladder: build, verify instances, calibrate, run the baseline
# test ladders, analyse. Deterministic given the committed seeds.
#
#   ./run.sh            # everything (about 30 min on 8 vCPU)
#   ./run.sh check      # build + tests + instance check only (minutes)
#   ./run.sh calibrate | test | analyze
#
# SIZES (default 6,7,8; rungs 9,10 are run by candidate sessions, PREREG_AMENDMENTS.md A2).
# THREADS (default 8) runs that many instances in parallel; node counts do not
# depend on it, wall clock does.
set -euo pipefail
cd "$(dirname "$0")"
THREADS="${THREADS:-8}"
STAGE="${1:-all}"
BIN=compute/target/release/hard_ladder

cargo build --release --manifest-path compute/Cargo.toml
"$BIN" check-instances

case "$STAGE" in
  check)
    cargo test --release --manifest-path compute/Cargo.toml
    ;;
  calibrate|all)
    "$BIN" calibrate --threads "$THREADS"
    python3 analysis/analyze.py calibration
    ;;&
  test|all)
    BEST=$(python3 -c 'import json;print(json.load(open("results/calibration/ranking.json"))["selected"])')
    for ladder in primary secondary; do
      for solver in "$BEST" hp-dfs; do
        "$BIN" run --solver "$solver" --role test --ladder "$ladder" --sizes "${SIZES:-6,7,8}" --threads "$THREADS"
      done
    done
    ;;&
  analyze|all)
    python3 analysis/analyze.py baseline
    ;;
  calibrate|test) ;;
  *) echo "unknown stage $STAGE" >&2; exit 2 ;;
esac
