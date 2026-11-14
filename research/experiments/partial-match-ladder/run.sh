#!/usr/bin/env bash
# Partial-match ladder stages (see PREREG.md and README.md).
#
#   ./run.sh check        build, unit tests, regenerate and check every committed instance file
#   ./run.sh analyze      recompute probe.json, calibration.json, summary.json and the tables from raw jsonl
#   ./run.sh rerun-test N C THREADS
#                         re-run every entrant on the committed test file of rung (N, C) into
#                         results/rerun/; the work counter makes scores machine-independent, so the
#                         rows must equal the committed ones (wall clock aside)
set -eu
cd "$(dirname "$0")"
stage=${1:-check}
build() { cargo build --release --manifest-path compute/Cargo.toml; }
case "$stage" in
  check)
    build
    cargo test --release --manifest-path compute/Cargo.toml
    compute/target/release/pml check instances/*.jsonl
    ;;
  analyze)
    python3 -I analysis/analyze.py probe
    python3 -I analysis/analyze.py calibrate
    python3 -I analysis/analyze.py determinism
    python3 -I analysis/analyze.py test
    ;;
  rerun-test)
    build
    n=$2; c=$3; threads=${4:-4}
    rho=$(python3 -I -c 'import json; print(json.load(open("results/probe.json"))["rho"])')
    mkdir -p results/rerun
    for e in break-r1 break-r2 sa pt alns; do
      we=$((4000000 * n * n))
      compute/target/release/pml run --entrant "$e" --instances "instances/test-n$n-c$c.jsonl" \
        --budget-evals "$we" --budget-placements "$(awk -v r="$rho" -v w="$we" 'BEGIN { printf "%.0f", r * w }')" --run-seed 1 --threads "$threads" \
        --machine local --out "results/rerun/${e}__test-n$n-c$c.jsonl"
    done
    ;;
  *) echo "unknown stage $stage"; exit 2 ;;
esac
