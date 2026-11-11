#!/usr/bin/env bash
# A15 non-chronological, bond-weighted unassignment on the hard ladder.
# Deterministic in events and nodes given the committed seeds (see PREREG.md).
#
#   ./run.sh            # everything: check, calibrate, test R6/R7 (+R8 if triggered), analyse
#   ./run.sh check      # build, tests, clippy, instance check only
#   ./run.sh calibrate | test | analyze
#   ./run.sh explore    # exploratory equal-CPU check at R7 (PREREG_AMENDMENTS.md E1, non-binding;
#                       # CPU times are machine-dependent, so it is not part of "all")
#
# THREADS (default 8) runs that many instances in parallel; events and nodes
# do not depend on it, wall clock does.
set -euo pipefail
cd "$(dirname "$0")"
THREADS="${THREADS:-8}"
STAGE="${1:-all}"
BIN=compute/target/release/ktam

cargo build --release --manifest-path compute/Cargo.toml
"$BIN" check-instances

case "$STAGE" in
  check|all)
    cargo test --release --manifest-path compute/Cargo.toml
    cargo clippy --release --all-targets --manifest-path compute/Cargo.toml -- -D warnings
    ;;&
  calibrate|all)
    for arm in B3 V1 V2; do "$BIN" calibrate --arm "$arm" --threads "$THREADS"; done
    python3 analysis/analyze.py select
    ;;&
  test|all)
    for rung in R6 R7; do
      for arm in B1 B2 B3 V1 V2; do "$BIN" run --arm "$arm" --rung "$rung" --threads "$THREADS"; done
    done
    python3 analysis/analyze.py summary > /dev/null
    if [ "$(python3 -c 'import json;print(json.load(open("results/summary.json"))["r8_triggered"])')" = "True" ]; then
      for arm in B1 B2 B3 V1 V2; do "$BIN" run --arm "$arm" --rung R8 --threads "$THREADS"; done
    else
      echo "R8 skipped: neither V1 nor V2 is GAIN or PASS against B1 at R6 or R7 (PREREG.md section 3)"
    fi
    ;;&
  analyze|all)
    python3 analysis/analyze.py summary
    ;;
  explore)
    for arm in B1 B2 B3 V1 V2; do "$BIN" explore-cpu --arm "$arm" --rung R7 --threads "$THREADS"; done
    for arm in B1 V2; do "$BIN" explore-cpu --arm "$arm" --rung R7 --threads 1 --tag timing1t; done
    python3 analysis/explore_cpu.py caps | while read -r arm cap; do
      "$BIN" explore-cpu --arm "$arm" --rung R7 --threads "$THREADS" --node-cap "$cap" --tag equalcpu
    done
    python3 analysis/explore_cpu.py summary
    ;;
  check|calibrate|test) ;;
  *) echo "unknown stage $STAGE" >&2; exit 2 ;;
esac
