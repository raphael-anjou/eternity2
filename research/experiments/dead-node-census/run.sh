#!/usr/bin/env bash
# Dead-node refutation census (see PREREG.md). Usage:
#   dead-node-census/run.sh build
#   dead-node-census/run.sh gate  [threads] [machine]   # soundness gate, all 90 trees
#   dead-node-census/run.sh fuzz                         # brute-force checks
#   dead-node-census/run.sh stage <B|A|E|D|C> [threads] [machine]
#   dead-node-census/run.sh analyze
# Run from research/experiments/. Rows go to results/{gate,run}/.
set -euo pipefail
cd "$(dirname "$0")"
BIN=compute/target/release/census
T=${3:-6}
M=${4:-$(hostname -s)}
rungs() { # ladder n
  echo "primary 6"; echo "plus2 7"; echo "primary 7"
}
case "${1:-}" in
  build) cargo build --release --manifest-path compute/Cargo.toml ;;
  fuzz) mkdir -p results/gate; $BIN fuzz --out "results/gate/fuzz-$(hostname -s).jsonl" ;;
  gate)
    T=${2:-6}; M=${3:-$(hostname -s)}
    mkdir -p results/gate
    rungs | while read -r L N; do
      $BIN gate --ladder "$L" --n "$N" --threads "$T" --machine "$M" --out "results/gate/gate-$L-n$N.jsonl"
    done ;;
  stage)
    S=$2
    mkdir -p results/run
    rungs | while read -r L N; do
      $BIN run --ladder "$L" --n "$N" --oracles "$S" --threads "$T" --machine "$M" \
        --out "results/run/$S-$L-n$N.jsonl"
    done ;;
  analyze) python3 analysis/analyze.py ;;
  *) sed -n 2,9p "$0"; exit 1 ;;
esac
