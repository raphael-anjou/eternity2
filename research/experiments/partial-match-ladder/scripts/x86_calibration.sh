#!/usr/bin/env bash
# x86 lane for calibration (2 threads): waits for Track 1's mrv_baseline runs to end,
# then the determinism rows, the rest of the rho probe, rho, and break-r1/break-r2 calibration.
set -eu
cd "$(dirname "$0")/.."
export RAYON_NUM_THREADS=1
while pgrep -f "mrv-scaling-pilot/target/release/mrv_baseline" > /dev/null; do sleep 60; done
date > results/x86_lane_started.txt
B=compute/target/release/pml
head -n 1 instances/cal-n8-c10.jsonl > results/det-instance.jsonl
mkdir -p results/determinism/x86
P="--budget-evals 100000000 --budget-placements 100000000 --run-seed 1 --threads 1 --machine x86"
(
  for e in break-r1 break-r2 sa pt alns; do
    $B run --entrant $e --instances results/det-instance.jsonl --budget-evals 20000000 --budget-placements 20000000 \
      --run-seed 1 --threads 1 --machine x86 --out results/determinism/x86/$e.jsonl 2>> results/determinism/x86/log.txt
  done
  $B run --entrant pt --instances results/probe/probe-n8-instances.jsonl $P --out results/probe/x86/pt.jsonl 2>> results/probe/x86/pt.log
) &
$B run --entrant break-r1 --instances results/probe/probe-n8-instances.jsonl $P --out results/probe/x86/break-r1.jsonl 2>> results/probe/x86/break-r1.log
wait
python3 -I analysis/analyze.py probe > results/probe_stdout.txt
rho=$(python3 -I -c 'import json; print(json.load(open("results/probe.json"))["rho"])')
echo "$rho" > results/rho.txt
scripts/run_jobs.sh x86 calibration "$rho" 2 1 "break-r1 break-r2" \
  instances/cal-n12-c*.jsonl instances/cal-n10-c*.jsonl instances/cal-n8-c*.jsonl
date > results/x86_lane_done.txt
