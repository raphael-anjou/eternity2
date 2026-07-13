#!/usr/bin/env bash
# profile_algo.sh — sample-profile one engine on variant_00 with samply, on a
# QUIET machine (a profiled run needs no core contention or the samples lie).
# Produces a samply profile.json you can open, plus a text top-functions summary.
#
# Usage:
#   profile_algo.sh native   <algo>  <budget_s>   # via run_algo
#   profile_algo.sh producer  <budget_s>          # standalone (comb:14 beam)
#   profile_algo.sh blackwood <budget_s>
#   profile_algo.sh verhaard  <budget_s>
set -uo pipefail
V2_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
VARIANTS_DIR="$(cat /tmp/claude-501/-Users-raphaelanjou-Documents-dev-projects-polytech-eternity2/73206ce1-35b1-47c7-a754-aaa40b0c60b8/scratchpad/variants_dir.txt 2>/dev/null)"
VJSON="${VARIANTS_DIR}/variant_00.json"
VCSV="${VARIANTS_DIR}/variant_00.csv"
OUTDIR="${V2_ROOT}/output/vol-235/profiles_$(date +%Y%m%dT%H%M%S)"
mkdir -p "$OUTDIR"
export RAYON_NUM_THREADS=1 LC_ALL=C

KIND="$1"; shift
case "$KIND" in
  native)
    ALGO="$1"; BUDGET_S="${2:-60}"
    OUT="${OUTDIR}/native_${ALGO}.json"
    samply record --save-only -o "$OUT" -- \
      "${V2_ROOT}/target/release/run_algo" --puzzle "$VJSON" --algo "$ALGO" \
      --seed 1 --budget-ms "$((BUDGET_S*1000))" --emit "${OUTDIR}/${ALGO}.url"
    ;;
  producer)
    BUDGET_S="${1:-60}"
    OUT="${OUTDIR}/producer.json"
    samply record --save-only -o "$OUT" -- \
      "${V2_ROOT}/target/release/vol232_w1_producer_trie" "$VCSV" \
      --order comb:14 --beam 80000 --tol 0 --seed 1 --emit-best "${OUTDIR}/producer.url"
    ;;
  blackwood)
    BUDGET_S="${1:-60}"
    OUT="${OUTDIR}/blackwood.json"
    timeout "$BUDGET_S" samply record --save-only -o "$OUT" -- \
      "${V2_ROOT}/target/release/blackwood_bt" "$VCSV" \
      --nodecap 20000000 --restarts 1000 --seed 1 --emit-dir "$OUTDIR" --emit-prefix bwp || true
    ;;
  verhaard)
    BUDGET_S="${1:-60}"
    OUT="${OUTDIR}/verhaard.json"
    timeout "$BUDGET_S" samply record --save-only -o "$OUT" -- \
      "${V2_ROOT}/target/release/verhaard_faithful_v2" "$VCSV" \
      --nodecap 20000000 --restarts 1000 --seed 1 --emit-dir "$OUTDIR" --emit-prefix vhp || true
    ;;
  *) echo "unknown kind $KIND"; exit 2;;
esac
echo "profile -> $OUT"
echo "open with: samply load $OUT"
