#!/usr/bin/env bash
# Full near-codeword-spectrum run (see PREREG.md, PREREG_AMENDMENTS.md).
# Resumable: CP-SAT stages skip jobs already in results/raw/*.jsonl.
set -euo pipefail
cd "$(dirname "$0")/.."
CORES="${CORES:-8}"
cargo build --release
B=target/release/ncs
if [ ! -x .venv/bin/python ]; then
    uv venv --python 3.12 .venv
    uv pip install --python .venv/bin/python -r requirements.txt
fi
PY=.venv/bin/python
mkdir -p results/raw results/raw/stage3
if [ ! -s results/instances.jsonl ]; then
    {
        for n in 5 6; do c=$((n == 5 ? 13 : 14)); $B instances $n $c 1 30; done
        $B instances 5 9 1 10; $B instances 6 10 1 10
        $B instances 7 15 1 10; $B instances 8 16 1 20
        $B instances 7 11 1 5; $B instances 8 12 1 5
        $B instances 10 18 1 20
    } > results/instances.jsonl
fi
{
    $B annealed 5 13 1; $B annealed 6 14 1
    $B annealed 5 9 1; $B annealed 6 10 1
} > results/raw/annealed.jsonl
for stage in stage1 curve stage2; do
    start=$(date +%s)
    $PY scripts/cpsat_spectrum.py --instances results/instances.jsonl \
        --stage $stage --out results/raw/$stage.jsonl --cores "$CORES"
    echo "$stage wall $(( $(date +%s) - start )) s" | tee -a results/raw/wall.txt
done
start=$(date +%s)
{
    for s in $(seq 1 20); do echo "8 16 $s"; echo "10 18 $s"; done
} | xargs -P "$CORES" -L 1 sh -c '[ -s results/raw/stage3/$0x$0_C$1_s$2.jsonl ] || '"$B"' stage3 $0 $1 $2 > results/raw/stage3/$0x$0_C$1_s$2.tmp && mv results/raw/stage3/$0x$0_C$1_s$2.tmp results/raw/stage3/$0x$0_C$1_s$2.jsonl'
echo "stage3 wall $(( $(date +%s) - start )) s" | tee -a results/raw/wall.txt
cat $(ls results/raw/stage3/*.jsonl | sort -V) > results/raw/stage3.jsonl
$PY scripts/analyze.py
