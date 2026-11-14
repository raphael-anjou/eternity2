#!/bin/sh
# Unplanned follow-up (not a preregistered reading): share of sampled prefixes
# that arc consistency does not refute, on test boards in seed order, one core,
# nice 10, through the prereg guard. Starts no new board after BUDGET seconds
# (default 2100). Resumable: finished boards are skipped.
#   sh scripts/ac_followup.sh
#   python3 -I scripts/ac_followup.py results/acfollowup > results/ac-followup.txt
set -e
EXP=$(cd "$(dirname "$0")/.." && pwd)
BIN=$EXP/engine/target/release/psx
OUT=$EXP/results/acfollowup
BUDGET=${BUDGET:-2100}
mkdir -p "$OUT"
start=$(date +%s)
for b in $(seq -f %.0f 1113001 1113048); do
  out=$OUT/board-$b.jsonl
  [ -f "$out" ] && continue
  now=$(date +%s)
  [ $((now - start)) -ge "$BUDGET" ] && break
  python3 "$EXP/../common/prereg_guard.py" "$EXP" ac-followup -- sh -c "nice -n 10 $BIN acshare --board $b --np 500000 --m 1000 > $out.part && mv $out.part $out"
done
