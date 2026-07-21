#!/usr/bin/env bash
# Parity test: run the COBOL solver on every data/*.txt puzzle and compare its
# RESULT line, field for field, against golden.txt (captured from the Rust
# engine). Matching down to node/attempt/backtrack counts proves the COBOL
# search explores the tree identically, not merely that it solves.
set -euo pipefail
cd "$(dirname "$0")"

cobc -x -free eternity2.cbl -o eternity2

pass=0
fail=0
while read -r line; do
  [ -z "$line" ] && continue
  # "RESULT <size> <colors> <seed>: ..." -> reconstruct the data file name.
  read -r _ size colors seedc _ <<<"$line"
  seed=${seedc%:}
  file="data/p_${size}_${colors}_${seed}.txt"
  got=$(./eternity2 "$file")
  if [ "$got" = "$line" ]; then
    echo "  ok   ${size}x${size} c=${colors} s=${seed}"
    pass=$((pass + 1))
  else
    echo "  FAIL ${size}x${size} c=${colors} s=${seed}"
    echo "    expected: $line"
    echo "    got:      $got"
    fail=$((fail + 1))
  fi
done < golden.txt

echo ""
echo "$pass passed, $fail failed"
[ "$fail" -eq 0 ]
