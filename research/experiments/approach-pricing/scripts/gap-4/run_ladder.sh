#!/bin/sh
# toy ladder, one core, nice 10, one process at a time. Output appended to ladder.jsonl.
# Every process is under `timeout 840` (hard limit 15 minutes per run) and a branch cap; lines are flushed per variant,
# so a run cut by the timeout keeps the variants it finished.
# History: TILE-FC ran on 6x6 seeds 1 and 2 only. The 7x7 seed 3 process ran past the 15 minute limit and was
# stopped by hand: its EDGE-AC line is missing (truncated run). 7x7 seeds 4 to 6 were never run.
cd "$(dirname "$0")"
V="ROW-PLAIN ROW-FC Y2-FC Y3-FC Y5-FC DOM-FC MIX-FC ROW-AC Y2-AC Y3-AC Y5-AC DOM-AC MIX-AC TILE-AC EDGE-AC"
V8="ROW-PLAIN ROW-FC Y2-FC ROW-AC Y2-AC Y3-AC DOM-AC MIX-AC TILE-AC EDGE-AC"
case "$1" in
  a) for s in 3 4 5 6 7 8 9 10; do timeout 840 nice -n 10 ./toy_orders 6 4 6 $s 400000000 "$V"; done ;;
  b) for s in 1 2 3; do timeout 840 nice -n 10 ./toy_orders 7 4 8 $s 400000000 "$V"; done ;;
  c1) timeout 840 nice -n 10 ./toy_orders 8 4 11 1 100000000 "$V8" ;;
esac >> ladder.jsonl
