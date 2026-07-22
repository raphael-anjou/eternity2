#!/usr/bin/env bash
# Hardness-peak sweep: solver effort vs interior-color count on generated framed
# boards, using the site's own seeded generator and step-able DFS solver.
#
# For each interior-color count c in COLORS and each seed in SEEDS, solve one
# generated board (fixed 5-color border band, c interior colors, total = c + 5)
# under a fixed DFS node budget and record: solved yes/no, nodes-to-first-
# solution (or the budget spent), depth reached, and wall time. Writes one JSON
# result file for the swept board size.
#
# This topic produces TWO result files, one per regime (see run-both.sh, which
# calls this script twice in order):
#   - results/hardness-peak-8x8.json    (E2_SIZE=8): the traversable solve-rate
#     PEAK, where instances actually get solved and the curve rises and falls.
#   - results/hardness-peak-16x16.json  (E2_SIZE=16, the real board): effort
#     under budget declines monotonically with c, and the interior palette caps
#     at 17 (the format's 22-motif alphabet minus 5 border colors), so no peak is
#     traversable on the full board.
#
# Determinism: every (c, seed) cell is a pure function of its arguments and the
# solver build, so the JSON reproduces bit-for-bit on any machine at the same
# NODE_BUDGET (nodes are hardware-independent; the recorded seconds are not, and
# are informational only).
#
# Resumable: each cell writes a per-cell fragment under results/frags-<size>/. A
# rerun skips any (c, seed) whose fragment already exists, so an interrupted
# sweep picks up where it left off. Delete the frags dir to force a clean rerun.
#
# Parallel: cells run 8 at a time (8 cores max). Each cell is single-threaded.
#
# Env overrides:
#   COLORS       space-separated interior-color counts. Default depends on size:
#                8x8 -> 1..17 (its peak sits low); else 8..26 (spanning the
#                predicted 16x16 peak; values past the 22-color ceiling are
#                clamped by the binary and flagged clampedToCeiling).
#   SEEDS        space-separated seeds (default: 1 .. 30).
#   NODE_BUDGET  max DFS steps per cell before giving up. Default depends on
#                size: 8x8 -> 30000000; else 20000000.
#   E2_SIZE      board edge length (default: 16). Set 8 for the traversable peak.
#   OUT          output JSON path (default: results/hardness-peak-<size>x<size>.json).
#   JOBS         parallel cells (default: 8).
set -euo pipefail
cd "$(dirname "$0")"

BIN=compute/target/release/hardness_peak
[ -x "$BIN" ] || (cd compute && cargo build --release >/dev/null 2>&1)

SIZE=${E2_SIZE:-16}
JOBS=${JOBS:-8}

# Size-dependent defaults: the 8x8 peak lives at low c and needs a larger budget
# to resolve its far flank; the 16x16 sweep spans the higher predicted band.
if [ "$SIZE" -le 8 ]; then
  COLORS=${COLORS:-1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17}
  NODE_BUDGET=${NODE_BUDGET:-30000000}
else
  COLORS=${COLORS:-8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26}
  NODE_BUDGET=${NODE_BUDGET:-20000000}
fi
SEEDS=${SEEDS:-$(seq 1 30)}
OUT=${OUT:-results/hardness-peak-${SIZE}x${SIZE}.json}

export E2_SIZE="$SIZE"

FRAGS="results/frags-${SIZE}"
mkdir -p "$FRAGS"

# solve_one C SEED : run one cell, write $FRAGS/c<C>.s<SEED>.frag unless it
# already exists (resume). The fragment is one JSON object on one line.
solve_one() {
  local c="$1" seed="$2"
  local frag="$FRAGS/c${c}.s${seed}.frag"
  [ -f "$frag" ] && return 0
  local tmp="$frag.partial"
  "$BIN" "$c" "$seed" "$NODE_BUDGET" > "$tmp"
  mv "$tmp" "$frag"          # atomic: a fragment only exists once complete
  echo "size=$SIZE c=$c seed=$seed done" 1>&2
}
export -f solve_one
export BIN NODE_BUDGET FRAGS SIZE

# Fan out (c, seed) cells JOBS at a time. A simple counting scheduler keeps us
# dependency-free (no GNU parallel / xargs -P portability worries).
running=0
for c in $COLORS; do
  for seed in $SEEDS; do
    solve_one "$c" "$seed" &
    running=$((running + 1))
    if [ "$running" -ge "$JOBS" ]; then
      wait -n
      running=$((running - 1))
    fi
  done
done
wait

# Assemble fragments into one JSON object with a rows[] array, in a deterministic
# (c, seed) order.
{
  echo "{"
  echo "  \"note\": \"For each interior-color count c and seed, a generated framed ${SIZE}x${SIZE} board (5-color border band, c interior colors, total c+5) is solved by the engine's plain row-major backtracking DFS under a fixed node budget. 'nodes' is DFS steps (placements + backtracks) to the first full solution, or the budget/exhaustion point when unsolved. Effort is measured in nodes so it is hardware-independent and deterministic; 'seconds' is informational only. The peak in median nodes / solve-rate vs c is the hardness peak.\","
  echo "  \"size\": ${SIZE},"
  echo "  \"borderColors\": 5,"
  echo "  \"nodeBudget\": ${NODE_BUDGET},"
  echo "  \"rows\": ["
  first=1
  for c in $COLORS; do
    for seed in $SEEDS; do
      frag="$FRAGS/c${c}.s${seed}.frag"
      [ -f "$frag" ] || continue
      [ $first -eq 1 ] && first=0 || echo "    ,"
      printf '    '
      cat "$frag"
    done
  done
  echo
  echo "  ]"
  echo "}"
} > "$OUT"

echo "wrote $OUT ($(grep -c '"seed"' "$OUT") rows)" 1>&2
