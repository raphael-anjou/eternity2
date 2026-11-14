#!/usr/bin/env bash
# difference-map-bands pipeline (PREREG.md). stage = build | test | merge | analyze | all
# Test jobs go through ../common/prereg_guard.py (job.sh). 4 jobs in parallel, one thread each.
set -euo pipefail
cd "$(dirname "$0")"
STAGE="${1:-all}"
export CAP=20000 RUNS=1-3 BETA=0.25
JOBS="${JOBS:-4}"

build() {
  cargo build --release --manifest-path compute/Cargo.toml
  cargo build --release --manifest-path ../mrv-baseline/compute/Cargo.toml
}

joblist() { # most expensive first
  for k in 3 2; do
    for s in 1-5 6-10 11-15 16-20 21-25 26-30; do echo "dm $k primary 8 $s"; done
    for s in 1-10 11-20 21-30; do echo "dm $k primary 7 $s"; done
  done
  for s in 1-5 6-10 11-15 16-20 21-25 26-30; do echo "dm 2 8-c10 8 $s"; done
  for s in 1-10 11-20 21-30; do echo "base bb-mrv primary 8 $s"; echo "base scb-gacolor_ac3_random primary 8 $s"; done
  for s in 1-10 11-20 21-30; do echo "dm 3 primary 6 $s"; echo "dm 2 primary 6 $s"; done
  for k in 1 0; do
    for s in 1-15 16-30; do echo "dm $k primary 8 $s"; echo "dm $k 8-c10 8 $s"; done
    echo "dm $k primary 7 1-30"
    echo "dm $k primary 6 1-30"
  done
  echo "base bb-mrv primary 6,7 1-30"
  echo "base scb-gacolor_ac3_random primary 6,7 1-30"
  echo "base bb-mrv 8-c10 - 1-30"
  echo "base scb-gacolor_ac3_random 8-c10 - 1-30"
}

test_() { joblist | xargs -P "$JOBS" -L 1 bash job.sh; }

merge() {
  mkdir -p results/raw
  for k in 0 1 2 3; do
    for lad in primary 8-c10; do
      ls results/raw/parts/dm-k$k-$lad-n*.jsonl >/dev/null 2>&1 || continue
      cat results/raw/parts/dm-k$k-$lad-n*.jsonl > results/raw/dm-k$k-$lad.jsonl
    done
  done
  for s in bb-mrv scb-gacolor_ac3_random; do
    cat results/raw/parts/$s-primary-n*.jsonl > results/raw/$s-primary.jsonl
    cat results/raw/parts/$s-8-c10-n*.jsonl > results/raw/$s-8-c10.jsonl
  done
}

analyze() { python3 -I analysis/analyze.py; }

case "$STAGE" in
  build) build ;;
  test) test_ ;;
  merge) merge ;;
  analyze) analyze ;;
  all) build; test_; merge; analyze ;;
  *) echo "unknown stage $STAGE"; exit 2 ;;
esac
