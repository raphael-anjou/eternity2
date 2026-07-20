#!/usr/bin/env bash
# Hint-geometry study: run producer_trie on each generated variant across many
# seeds, single-core, and record score + wall-time per (variant, seed). The
# variants share ONE solved 16x16 board (E2 recipe, ids scrambled); they differ
# only in which cells are pinned and how many. Output: results.jsonl.
#
# Metric: producer runs a fixed-width beam to completion (no early stop), so the
# per-run number is FINAL SCORE at a fixed beam, plus wall-clock. We report the
# score distribution per geometry; time is context, not the primary axis.
set -euo pipefail
cd "$(dirname "$0")"

PROD=../single-core-benchmark/engine/target/release/producer_trie
VARIANTS=variants
OUT=results.jsonl
BEAM="${BEAM:-20000}"
ORDER="${ORDER:-comb:14}"
SEED_LO="${SEED_LO:-1}"
SEED_HI="${SEED_HI:-20}"

: > "$OUT"
echo "producer beam=$BEAM order=$ORDER seeds=$SEED_LO..$SEED_HI"

for csv in "$VARIANTS"/*.csv; do
  name="$(basename "$csv" .csv)"
  nhints="$(awk -F, 'NR>1 && ($5!=0||$6!=0||$7!=0)' "$csv" | wc -l | tr -d ' ')"
  for seed in $(seq "$SEED_LO" "$SEED_HI"); do
    start=$(python3 -c 'import time;print(time.time())')
    # stdout data row: order,beam,tol,seed,score,layers,peak_layer_len
    row="$(RAYON_NUM_THREADS=1 "$PROD" "$csv" --order "$ORDER" --beam "$BEAM" \
             --tol 0 --seed "$seed" --threads 1 2>/dev/null | tail -1)"
    end=$(python3 -c 'import time;print(time.time())')
    score="$(echo "$row" | cut -d, -f5)"
    layers="$(echo "$row" | cut -d, -f6)"
    secs="$(python3 -c "print(f'{$end-$start:.3f}')")"
    printf '{"variant":"%s","nhints":%s,"beam":%s,"seed":%s,"score":%s,"layers":%s,"secs":%s}\n' \
      "$name" "$nhints" "$BEAM" "$seed" "$score" "$layers" "$secs" >> "$OUT"
  done
  echo "  done $name ($nhints hints)"
done
echo "wrote $OUT"
