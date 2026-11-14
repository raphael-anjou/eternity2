#!/usr/bin/env bash
# Calibration pass 2 (calibration seeds only): step beta at k = 0 and k = 1, n = 6 and 7, cap 20000.
set -u
cd "$(dirname "$0")/.."
B=compute/target/release/dmb
P=../hard-ladder/instances/primary-calibration.jsonl
C=calib/cal2
mkdir -p "$C"
rm -f "$C"/*.jsonl
for beta in 0.1 0.25 0.5 0.8; do
  (
    nice "$B" run --instances "$P" --sizes 6,7 --k 0 --runs 1-1 --cap 20000 --beta "$beta" --out "$C/k0-b$beta.jsonl"
    nice "$B" run --instances "$P" --sizes 6,7 --k 1 --runs 1-1 --cap 20000 --beta "$beta" --out "$C/k1-b$beta.jsonl"
  ) &
done
wait
echo cal2 done
