#!/usr/bin/env bash
# Backdoor-guided branching (PREREG.md). Stages:
#   all | build | stage1 | calibrate | test | analyze
# SEEDS=20 (default, the sample-size rule's outcome, README.md) or SEEDS=30.
set -euo pipefail
cd "$(dirname "$0")"
HERE=$(pwd)
LADDER="$HERE/../hard-ladder"
THREADS=${THREADS:-8}
SEEDS=${SEEDS:-20}
BB="$HERE/compute/target/release/bb_ladder"
HL="$LADDER/compute/target/release/hard_ladder"
stage=${1:-all}

build() {
  cargo build --release --manifest-path "$LADDER/compute/Cargo.toml"
  cargo build --release --manifest-path compute/Cargo.toml
  "$HL" check-instances
}

stage1() {
  "$HERE/compute/target/release/bb_features" --threads "$THREADS"
  python3 analysis/stage1.py
}

calibrate() {
  mkdir -p results/calibration
  for s in bb-mrv bb-impact bb-learned bb-single; do
    "$BB" run --solver "$s" --role calibration --ladder primary --sizes 6,7 --threads "$THREADS" \
      --out "results/calibration/primary-$s.jsonl"
    "$BB" run --solver "$s" --role calibration --ladder 8-c10 --sizes 8 --threads "$THREADS" \
      --out "results/calibration/8-c10-$s.jsonl"
  done
  python3 analysis/project.py
}

# Test instances restricted to seeds 1..SEEDS (same subset for every solver).
seed_root() {
  local root="$HERE/results/.ladder-root-s$SEEDS"
  mkdir -p "$root/instances"
  for f in primary-test 8-c10-test; do
    python3 -c "import json,sys
for l in open(sys.argv[1]):
    if l.strip() and json.loads(l)['seed'] <= int(sys.argv[2]): print(l, end='')" \
      "$LADDER/instances/$f.jsonl" "$SEEDS" > "$root/instances/$f.jsonl"
  done
  echo "$root"
}

test_runs() {
  local root
  root=$(seed_root)
  mkdir -p results/test/primary results/test/8-c10
  for s in scb-gacolor_ac3_random hp-dfs; do
    HARD_LADDER_ROOT="$root" "$HL" run --solver "$s" --role test --ladder 8-c10 --sizes 8 \
      --threads "$THREADS" --out "results/test/8-c10/$s.jsonl"
  done
  for s in bb-mrv bb-impact bb-learned bb-single; do
    HARD_LADDER_ROOT="$root" "$BB" run --solver "$s" --role test --ladder primary --sizes 6,7 \
      --threads "$THREADS" --out "results/test/primary/$s.jsonl"
    HARD_LADDER_ROOT="$root" "$BB" run --solver "$s" --role test --ladder 8-c10 --sizes 8 \
      --threads "$THREADS" --out "results/test/8-c10/$s.jsonl"
  done
}

analyze() { python3 analysis/stage2.py; }

case "$stage" in
  build) build ;;
  stage1) build; stage1 ;;
  calibrate) build; calibrate ;;
  test) build; test_runs ;;
  analyze) analyze ;;
  all) build; stage1; calibrate; test_runs; analyze ;;
  *) echo "unknown stage $stage" >&2; exit 2 ;;
esac
