#!/usr/bin/env bash
# Solve and verify the recovered Eternity II clue puzzles, and render each
# solution as an SVG. Deterministic given the same kissat build. Requires kissat.
set -euo pipefail
cd "$(dirname "$0")"

BIN=compute/target/release/cluesolve
[ -x "$BIN" ] || (cd compute && cargo build --release >/dev/null 2>&1)

solve() { # <file> <w> <h> <name>
  "$BIN" "data/$1" "$2" "$3" > "results/$4-solution.json"
  "$BIN" "data/$1" "$2" "$3" --svg > "results/$4-solution.svg"
  cp "results/$4-solution.svg" "../../../web/src/assets/research/clue-puzzles/$4_solution.svg"
  echo "$4: $(grep -o '"validated": [a-z]*' "results/$4-solution.json")"
}

solve clue1_shorter36.txt 6 6 clue1
solve clue2_shorter72.txt 12 6 clue2
echo "done"
