#!/usr/bin/env bash
# Round-2 fix runs (PREREG_AMENDMENTS.md A6 to A9). Resumable.
set -euo pipefail
cd "$(dirname "$0")/.."
CORES="${CORES:-8}"
if [ ! -x .venv/bin/python ]; then
    uv venv --python 3.12 .venv
    uv pip install --python .venv/bin/python -r requirements.txt
fi
PY=.venv/bin/python
for stage in star1 k1c star7; do
    start=$(date +%s)
    $PY scripts/cpsat_quotient.py --instances results/instances.jsonl \
        --stage $stage --out results/raw/$stage.jsonl --cores "$CORES"
    echo "$stage wall $(( $(date +%s) - start )) s" | tee -a results/raw/wall_fix.txt
done
$PY scripts/analyze_fix.py
