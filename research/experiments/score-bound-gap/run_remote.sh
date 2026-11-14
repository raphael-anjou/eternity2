#!/usr/bin/env bash
# Run a queue of measure jobs on one machine. Usage:
#   ./run_remote.sh MACHINE THREADS JOB...
# where JOB = RUNG:SOURCE:RLIST:CAP  (RUNG in n6-c5 n7-c6 n6-c7 n7-c8 n8-c8 n8-c10;
# SOURCE schedule|strict; RLIST like 1,2,3; CAP oracle node cap).
set -eu
cd "$(dirname "$0")"
machine=$1; threads=$2; shift 2
inst_of() {
  case $1 in
    n6-c5|n7-c6|n8-c8) echo ../hard-ladder/instances/primary-test.jsonl ;;
    n6-c7|n7-c8) echo ../hard-ladder/instances/plus2-test.jsonl ;;
    n8-c10) echo ../hard-ladder/instances/8-c10-test.jsonl ;;
  esac
}
for job in "$@"; do
  IFS=: read rung source rlist cap <<< "$job"
  echo "$(date) start $job"
  ./compute/target/release/sbg measure --instances "$(inst_of $rung)" \
    --prefixes results/test/prefixes-$rung-$source.jsonl --widths 1,64,512,4096 \
    --cap "$cap" --threads "$threads" --machine "$machine" --r "$rlist" --source "$source" \
    --out results/test/measure-$rung-$source-$machine.jsonl
  echo "$(date) done $job"
done
