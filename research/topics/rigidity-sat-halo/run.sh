#!/usr/bin/env bash
# Reproduce halo-SAT strict-optimality on the public record boards.
#
# For each board and halo radius, emit the CNF, run kissat, and record the
# verdict and wall-time. Writes results/halo-sat.json. Deterministic given the
# same solver build. Requires kissat on PATH.
set -euo pipefail
cd "$(dirname "$0")"

BIN=compute/target/release/halo_cnf
[ -x "$BIN" ] || (cd compute && cargo build --release >/dev/null 2>&1)

BOARDS=(Joshua_Blackwood_470 JBlackwood+PMcGavin_469 Joshua_Blackwood_468 Louis_Verhaard_467)
RADII=(1 2)
KISSAT_TIMEOUT=${KISSAT_TIMEOUT:-600}

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

echo "{" > results/halo-sat.json
echo "  \"note\": \"For each public record board, cells within Chebyshev radius R of a mismatched edge are freed and a SAT solver is asked whether those pieces can be rearranged so every halo edge matches. UNSAT = strict local optimum on that halo. Verdicts from kissat; deterministic.\"," >> results/halo-sat.json
echo "  \"metric\": \"Chebyshev\"," >> results/halo-sat.json
echo "  \"results\": [" >> results/halo-sat.json

first=1
for board in "${BOARDS[@]}"; do
  for R in "${RADII[@]}"; do
    "$BIN" "$board" "$R" > "$tmp/h.cnf" 2> "$tmp/h.json"
    start=$(python3 -c 'import time;print(time.time())')
    set +e
    kissat -q --time="$KISSAT_TIMEOUT" "$tmp/h.cnf" > "$tmp/h.out" 2>&1
    ec=$?
    set -e
    end=$(python3 -c 'import time;print(time.time())')
    case $ec in
      10) verdict=SAT ;;
      20) verdict=UNSAT ;;
      *)  verdict=TIMEOUT ;;
    esac
    secs=$(python3 -c "print(round($end-$start,2))")
    [ $first -eq 1 ] && first=0 || echo "    ," >> results/halo-sat.json
    python3 - "$tmp/h.json" "$verdict" "$secs" >> results/halo-sat.json <<'PY'
import json,sys
d=json.load(open(sys.argv[1])); verdict=sys.argv[2]; secs=float(sys.argv[3])
print("    {")
print(f'      "board": {json.dumps(d["board"])},')
print(f'      "score": {d["score"]},')
print(f'      "mismatches": {d["mismatches"]},')
print(f'      "radius": {d["radius"]},')
print(f'      "freedCells": {d["freed_cells"]},')
print(f'      "vars": {d["vars"]},')
print(f'      "clauses": {d["clauses"]},')
print(f'      "verdict": {json.dumps(verdict)},')
print(f'      "seconds": {secs}')
print("    }", end="")
PY
    echo >> results/halo-sat.json
    echo "$board R=$R -> $verdict (${secs}s)" 1>&2
  done
done

echo "  ]" >> results/halo-sat.json
echo "}" >> results/halo-sat.json
echo "wrote results/halo-sat.json" 1>&2
