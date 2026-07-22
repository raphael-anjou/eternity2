#!/usr/bin/env bash
# Produce BOTH hardness-peak result files, in order:
#   1. results/hardness-peak-8x8.json    (traversable solve-rate peak)
#   2. results/hardness-peak-16x16.json  (real board: monotone decline + ceiling)
#
# Each regime's grid, budget, and parallelism come from run.sh's size-dependent
# defaults; both are resumable, so rerunning after an interruption is safe.
# ~5 min for the 8x8 sweep + ~38 min for the 16x16 sweep on 8 cores.
set -euo pipefail
cd "$(dirname "$0")"

echo "== hardness-peak: 8x8 traversable-peak sweep ==" 1>&2
E2_SIZE=8 ./run.sh

echo "== hardness-peak: 16x16 real-board sweep ==" 1>&2
E2_SIZE=16 ./run.sh

echo "== both result files written ==" 1>&2
ls -1 results/hardness-peak-*.json 1>&2
