#!/usr/bin/env bash
# Cut-keyed replay (see PREREG.md). Run from anywhere. Usage:
#   cut-keyed-replay/run.sh build
#   cut-keyed-replay/run.sh check                  # soundness gate 3 (generated boards, no test seeds)
#   cut-keyed-replay/run.sh test <n> [threads]     # test seeds 1..30 at primary n, through prereg_guard
#   cut-keyed-replay/run.sh test-remote <host> <n> [threads]   # same on an ssh host holding a copy under ~/work/t12
#   cut-keyed-replay/run.sh analyze
set -euo pipefail
cd "$(dirname "$0")"
BIN=compute/target/release/replay
GUARD="python3 ../common/prereg_guard.py ."
case "${1:-}" in
  build) cargo build --release --manifest-path compute/Cargo.toml ;;
  check) mkdir -p results/check; $BIN check --out results/check/check.jsonl ;;
  test)
    N=$2; T=${3:-2}
    mkdir -p results/run
    $GUARD "test-n$N" -- $BIN run --ladder primary --n "$N" --threads "$T" \
      --machine "$(hostname -s)" --out "results/run/primary-n$N.jsonl" ;;
  test-remote)
    H=$2; N=$3; T=${4:-10}
    SSH="ssh -F $SSH_CONFIG -o BatchMode=yes"
    D=work/t12/research/experiments/cut-keyed-replay
    mkdir -p results/run
    $GUARD "test-n$N-$H" -- $SSH "$H" "cd $D && mkdir -p results/run && $BIN run --ladder primary --n $N --threads $T --machine $H --out results/run/primary-n$N.jsonl"
    rsync -a -e "$SSH" "${H}:$D/results/run/primary-n$N.jsonl" results/run/ ;;
  analyze) python3 analysis/analyze.py ;;
  *) sed -n 2,7p "$0"; exit 1 ;;
esac
