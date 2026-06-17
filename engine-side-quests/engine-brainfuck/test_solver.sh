#!/usr/bin/env bash
# Parity test for the Brainfuck solver: compile it, run it on each puzzle of the
# chosen size, and compare the solved board (raw output bytes) to the board the
# Rust engine produced (data/bf_<n>_<n>_*.board.txt). A match proves the pure-
# Brainfuck explicit-stack backtracking DFS finds the same solution as
# solver.rs — including the backtracking cases.
#
# Usage:
#   ./test_solver.sh         # 3x3 puzzles (fast, ~1s each)
#   ./test_solver.sh 4       # 4x4 puzzles (slower, ~100s each)
set -euo pipefail
cd "$(dirname "$0")"

SIZE="${1:-3}"

E2_BF_SIZE="$SIZE" python3 build.py >/dev/null
if command -v rustc >/dev/null; then
  rustc -O bf.rs -o bf 2>/dev/null
  RUN=(./bf)
else
  echo "(rustc not found; falling back to the slower Python interpreter)"
  RUN=(python3 bf.py)
fi

shopt -s nullglob
puzzles=(data/bf_"${SIZE}"_"${SIZE}"_*.bin)
if [ ${#puzzles[@]} -eq 0 ]; then
  echo "no data/bf_${SIZE}_${SIZE}_*.bin puzzles found" >&2
  exit 1
fi

pass=0
fail=0
for bin in "${puzzles[@]}"; do
  stem="${bin%.bin}"
  exp="$(cat "${stem}.board.txt")"
  got="$("${RUN[@]}" solver.bf < "$bin" \
        | python3 -c 'import sys; print(" ".join(str(b) for b in sys.stdin.buffer.read()))')"
  name="$(basename "$stem")"
  if [ "$got" = "$exp" ]; then
    echo "  ok   $name -> $got"
    pass=$((pass + 1))
  else
    echo "  FAIL $name"
    echo "    expected: $exp"
    echo "    got:      $got"
    fail=$((fail + 1))
  fi
done

echo ""
echo "${SIZE}x${SIZE}: $pass passed, $fail failed"
[ "$fail" -eq 0 ]
