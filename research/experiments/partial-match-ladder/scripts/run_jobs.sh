#!/usr/bin/env bash
# Run (entrant x instance file) jobs with the preregistered budget pair.
#
#   scripts/run_jobs.sh MACHINE PHASE RHO JOBS_PARALLEL THREADS_PER_JOB "ENTRANTS" FILE...
#
# Budget pair for an n x n file: W_eval = 4,000,000 * n^2 move evaluations,
# W_place = RHO * W_eval placements (PREREG.md). Each job appends to
# results/PHASE/MACHINE/<entrant>__<file>.jsonl and is resumable.
# Busy threads = JOBS_PARALLEL * THREADS_PER_JOB. Works with bash 3.2.
set -eu
cd "$(dirname "$0")/.."
machine=$1; phase=$2; rho=$3; par=$4; per=$5; entrants=$6; shift 6
bin=compute/target/release/pml
mkdir -p "results/$phase/$machine"
jobs=""
for f in "$@"; do
  for e in $entrants; do
    jobs="$jobs$e $f
"
  done
done
RAYON_NUM_THREADS=1
export machine phase rho per bin RAYON_NUM_THREADS
printf '%s' "$jobs" | xargs -P "$par" -L 1 sh -c '
  e=$1; f=$2
  base=$(basename "$f" .jsonl)
  n=$(echo "$base" | sed -E "s/.*-n([0-9]+)-c.*/\1/")
  we=$((4000000 * n * n))
  wp=$(awk -v r="$rho" -v w="$we" "BEGIN { printf \"%.0f\", r * w }")
  "$bin" run --entrant "$e" --instances "$f" --budget-evals "$we" --budget-placements "$wp" \
    --run-seed 1 --threads "$per" --machine "$machine" \
    --out "results/$phase/$machine/${e}__${base}.jsonl" 2>>"results/$phase/$machine/log.txt"
' sh
