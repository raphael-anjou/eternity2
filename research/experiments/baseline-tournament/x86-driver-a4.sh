#!/usr/bin/env bash
# Amendment 4 (PREREG_AMENDMENTS.md): replaces the x86 laptop tier-order driver after
# the fixed quota (x86 laptop 4 threads in total). Keeps the preregistered order for
# what is left: bb-learned R7 (tier 3), then the tier 5 and 6 items that
# m1 can no longer take. Waits for the running bb-impact process first.
# Threads: 3 while the 1-thread bb-learned R8 process runs, so 4 in total.
set -uo pipefail
cd "$(dirname "$0")"
BIN=compute/target/release/tournament
DEADLINE="${DEADLINE:-22:00}"
while pgrep -f "entrant bb-impact" >/dev/null; do sleep 20; done
echo "$(date +%FT%T) bb-impact seeds=1 (finished, driver a4 took over)" >> results/stage-times.txt
go() { # entrant seeds rungs
  if [[ "$(date +%H:%M)" > "$DEADLINE" || "$(date +%H:%M)" == "$DEADLINE" ]]; then
    echo "NOT REACHED (deadline $DEADLINE): $*" >> results/not-reached.txt; return 0; fi
  local t0=$SECONDS
  "$BIN" run --entrant "$1" --seeds "$2" --rungs "$3" --threads 3 --out-dir results/raw 2>>results/log.txt </dev/null
  echo "$(date +%FT%T) $1 seeds=$2 rungs=$3 $((SECONDS - t0)) s (driver a4)" >> results/stage-times.txt
}
go bb-learned 1 R7
for e in cmp-hall-all-k4 cmp-hall-all cmp-hall-frontier-k4; do
  rm -f results/raw/$e/s2-*.jsonl results/raw/$e/s3-*.jsonl   # placeholders (amendment 1)
  go "$e" 2,3 R6,R7,R8
done
for e in scb-fork-off cmp-fork-off; do
  rm -f results/raw/$e/s1-*.jsonl
  go "$e" 1 R6,R7,R8
done
echo "$(date +%FT%T) driver a4 done" >> results/stage-times.txt
