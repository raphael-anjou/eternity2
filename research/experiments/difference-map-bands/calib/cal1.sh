#!/usr/bin/env bash
# Calibration pass 1 (calibration seeds 1001..1010 only): iteration counts and cost per iteration.
set -u
cd "$(dirname "$0")/.."
B=compute/target/release/dmb
H=../hard-ladder/instances
C=calib/cal1
mkdir -p "$C"
rm -f "$C"/*.jsonl
P="$H/primary-calibration.jsonl"
E="$H/8-c10-calibration.jsonl"
(
  nice "$B" run --instances "$P" --sizes 6,7,8 --k 0 --runs 1-1 --cap 10000 --out "$C/k0.jsonl"
  nice "$B" run --instances "$E" --k 0 --runs 1-1 --cap 10000 --out "$C/k0-easy.jsonl"
  nice "$B" run --instances "$P" --sizes 6,7,8 --k 1 --runs 1-1 --cap 10000 --out "$C/k1.jsonl"
  nice "$B" run --instances "$E" --k 1 --runs 1-1 --cap 10000 --out "$C/k1-easy.jsonl"
) &
(
  nice "$B" run --instances "$P" --sizes 6,7,8 --k 2 --runs 1-1 --cap 10000 --out "$C/k2.jsonl"
  nice "$B" run --instances "$E" --k 2 --runs 1-1 --cap 10000 --out "$C/k2-easy.jsonl"
) &
(
  nice "$B" run --instances "$P" --sizes 6,7 --k 3 --runs 1-1 --cap 10000 --out "$C/k3.jsonl"
) &
(
  nice "$B" run --instances "$P" --sizes 8 --k 3 --runs 1-1 --cap 2000 --out "$C/k3-n8.jsonl"
  nice "$B" run --instances "$E" --k 3 --runs 1-1 --cap 2000 --out "$C/k3-easy.jsonl"
) &
wait
echo cal1 done
