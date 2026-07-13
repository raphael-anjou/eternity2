#!/usr/bin/env bash
# ab_baseline.sh — capture the bit-identical A/B baseline for every native algo.
#
# For each algorithm, run a FIXED-WORK (node-budgeted, not wall-timed) pass on
# variant_00 seed 1 and record (board_hash, nodes, backtracks, nps). This is the
# reference the optimization campaign checks against: any speedup must reproduce
# the SAME board_hash + SAME nodes, only improving nps. Wall-time runs are NOT
# reproducible (a faster build does more work), so the gate uses --node-budget.
#
# Usage: ab_baseline.sh <variant.json> <node_budget> <out.tsv>
set -uo pipefail
V2_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
RUN_ALGO="${V2_ROOT}/target/release/run_algo"

VARIANT="${1:-}"
NODE_BUDGET="${2:-2000000}"
OUT="${3:-/dev/stdout}"
[[ -z "$VARIANT" ]] && { echo "usage: ab_baseline.sh <variant.json> <node_budget> <out.tsv>"; exit 2; }

export RAYON_NUM_THREADS=1
PAR="${PAR:-6}"   # each algo run is single-core; run PAR of them at once.

# Each algo run is independent single-core work -> fan out PAR-wide with xargs
# so all cores are used (the WORK is parallel; each JOB stays single-core, so
# the fixed-node-budget board_hash is unaffected by contention).
_ab_one() {
    local algo="$1"
    local line
    line="$("$RUN_ALGO" --puzzle "$VARIANT" --algo "$algo" --seed 1 \
        --node-budget "$NODE_BUDGET" --emit "/tmp/ab_${algo}.url" 2>/dev/null)"
    local score nodes bt nps hash
    score=$(sed -nE 's/.* score=([0-9]+).*/\1/p' <<<"$line")
    nodes=$(sed -nE 's/.* nodes=([0-9]+).*/\1/p' <<<"$line")
    bt=$(sed -nE 's/.* backtracks=([0-9]+).*/\1/p' <<<"$line")
    nps=$(sed -nE 's/.* nps=([0-9]+).*/\1/p' <<<"$line")
    hash=$(sed -nE 's/.* board_hash=([0-9a-f]+).*/\1/p' <<<"$line")
    printf '%s\t%s\t%s\t%s\t%s\t%s\n' "$algo" "$score" "$nodes" "$bt" "$nps" "$hash"
}
export -f _ab_one
export RUN_ALGO VARIANT NODE_BUDGET

printf 'algo\tscore\tnodes\tbacktracks\tnps\tboard_hash\n' > "$OUT"
"$RUN_ALGO" --list | xargs -P "$PAR" -I{} bash -c '_ab_one "$@"' _ {} >> "$OUT"
echo "baseline ($PAR-wide) -> $OUT"
sort -t$'\t' -k2 -nr "$OUT" 2>/dev/null | head
