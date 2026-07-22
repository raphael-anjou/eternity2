#!/usr/bin/env bash
# Reproduce halo-SAT strict-optimality on the public record boards.
#
# For each board and halo radius, emit the CNF, run kissat, and record the
# verdict and wall-time. Writes results/halo-sat.json. Deterministic given the
# same solver build. Requires kissat on PATH.
#
# Radii 1 and 2 are seconds to tens of seconds. Radii 3 and 4 free a block of
# ~110-150 cells and can take many minutes to solve; each solve is capped at
# KISSAT_TIMEOUT seconds (kissat's own --time limit) and a row that hits the cap
# is recorded with verdict "TIMEOUT" and the cap in "capSeconds", so a partial
# radius-4 sweep still reports exactly what it proved and what it did not. The
# five boards are solved in parallel (kissat is single-threaded; 8 cores max).
set -euo pipefail
cd "$(dirname "$0")"

BIN=compute/target/release/halo_cnf
[ -x "$BIN" ] || (cd compute && cargo build --release >/dev/null 2>&1)

# The public record boards, plus the strict five-clue record (Riotte 464).
BOARDS=(Joshua_Blackwood_470 JBlackwood+PMcGavin_469 Joshua_Blackwood_468 Louis_Verhaard_467 Benjamin_Riotte_464)
RADII=(1 2 3 4)
# Per-instance solve cap in seconds. Default 1800 (30 min); the whole sweep with
# five boards in parallel stays inside a couple of hours on 8 cores.
KISSAT_TIMEOUT=${KISSAT_TIMEOUT:-1800}

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

# solve_one BOARD R : emit CNF, run kissat (capped), write a per-row JSON
# fragment to $tmp/$BOARD.$R.frag . Runs standalone so it can be backgrounded.
solve_one() {
  local board="$1" R="$2"
  local cnf="$tmp/$board.$R.cnf" stats="$tmp/$board.$R.stats"
  "$BIN" "$board" "$R" > "$cnf" 2> "$stats"
  local start end ec verdict secs
  start=$(python3 -c 'import time;print(time.time())')
  set +e
  kissat -q --time="$KISSAT_TIMEOUT" "$cnf" > /dev/null 2>&1
  ec=$?
  set -e
  end=$(python3 -c 'import time;print(time.time())')
  case $ec in
    10) verdict=SAT ;;
    20) verdict=UNSAT ;;
    *)  verdict=TIMEOUT ;;
  esac
  secs=$(python3 -c "print(round($end-$start,2))")
  rm -f "$cnf"  # CNFs are hundreds of MB at R>=3; drop as soon as solved
  python3 - "$stats" "$verdict" "$secs" "$KISSAT_TIMEOUT" > "$tmp/$board.$R.frag" <<'PY'
import json,sys
d=json.load(open(sys.argv[1])); verdict=sys.argv[2]; secs=float(sys.argv[3]); cap=int(sys.argv[4])
lines=[]
lines.append("    {")
lines.append(f'      "board": {json.dumps(d["board"])},')
lines.append(f'      "score": {d["score"]},')
lines.append(f'      "mismatches": {d["mismatches"]},')
lines.append(f'      "radius": {d["radius"]},')
lines.append(f'      "freedCells": {d["freed_cells"]},')
lines.append(f'      "vars": {d["vars"]},')
lines.append(f'      "clauses": {d["clauses"]},')
lines.append(f'      "verdict": {json.dumps(verdict)},')
if verdict == "TIMEOUT":
    lines.append(f'      "capSeconds": {cap},')
lines.append(f'      "seconds": {secs}')
lines.append("    }")
open(sys.stdout.fileno(), "w").write("\n".join(lines))
PY
  echo "$board R=$R -> $verdict (${secs}s)" 1>&2
}

# Sweep radius by radius; within a radius run all five boards in parallel.
for R in "${RADII[@]}"; do
  for board in "${BOARDS[@]}"; do
    solve_one "$board" "$R" &
  done
  wait
done

# Assemble results in a deterministic board-major, radius-minor order.
{
  echo "{"
  echo "  \"note\": \"For each public record board, cells within Chebyshev radius R of a mismatched edge are freed and a SAT solver is asked whether those pieces can be rearranged so every halo edge matches. UNSAT = strict local optimum on that halo. A row with verdict TIMEOUT hit the per-instance kissat time cap (capSeconds) and is neither proven nor refuted. Verdicts from kissat; deterministic.\","
  echo "  \"metric\": \"Chebyshev\","
  echo "  \"results\": ["
  first=1
  for board in "${BOARDS[@]}"; do
    for R in "${RADII[@]}"; do
      frag="$tmp/$board.$R.frag"
      [ -f "$frag" ] || continue
      [ $first -eq 1 ] && first=0 || echo "    ,"
      cat "$frag"
      echo
    done
  done
  echo "  ]"
  echo "}"
} > results/halo-sat.json

echo "wrote results/halo-sat.json" 1>&2
