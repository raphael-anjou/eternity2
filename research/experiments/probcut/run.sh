#!/usr/bin/env bash
# Full ProbCut pipeline (see PREREG.md). Deterministic given the seeds.
set -euo pipefail
cd "$(dirname "$0")"
cargo build --release --manifest-path compute/Cargo.toml
B=compute/target/release/probcut
$B gen
$B check-instances
if [ "${1:-}" != "--test-only" ]; then
  $B stage1 --role calibration --ladder primary --sizes 6,7
  $B stage1 --role calibration --ladder 8-c10
  for u in 100 1000 10000; do
    $B run --solver fc-luby --role calibration --ladder primary --sizes 6,7 --luby-unit $u --out results/calibration/luby/primary-u$u.jsonl
    $B run --solver fc-luby --role calibration --ladder 8-c10 --luby-unit $u --out results/calibration/luby/8-c10-u$u.jsonl
  done
  $B run --solver fc-dfs --role calibration --ladder primary --sizes 6,7
  $B run --solver fc-dfs --role calibration --ladder 8-c10
  python3 analysis/probcut.py calibrate
fi
for s in scb-gacolor_ac3_random hp-dfs; do
  $B run --solver $s --role test --ladder 8-c10
done
for s in fc-dfs fc-luby probcut-v1 probcut-v2; do
  $B run --solver $s --role test --ladder primary --sizes 6,7
  $B run --solver $s --role test --ladder 8-c10
done
$B stage1 --role test --ladder primary --sizes 6,7 --planted-only
$B stage1 --role test --ladder 8-c10 --planted-only
python3 analysis/probcut.py report
