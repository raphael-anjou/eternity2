#!/bin/sh
# Population estimates on the OFFICIAL 16x16 pieces with the five clues fixed. One core, nice 10, one process at a time.
# Every process: internal cpu limit 780 s and `timeout 840` (hard limit under 15 minutes).
# e2r = board turned by a half turn, so ROW there is the scan from the last cell of the official numbering.
cd "$(dirname "$0")"
E2=research/experiments/single-core-benchmark/engine/data/puzzles/size_16_official_eternity.csv
run() { timeout 840 nice -n 10 ./dyn16 "$@" | sed "s#$E2#official#" >> real16.jsonl; }
case "$1" in
 fc) for b in e2r e2; do run $b:$E2 PLAIN ROW smc 3000 5 11 780
       for r in ROW Y2 Y3 Y5 B2 B3 DOMF DOM; do run $b:$E2 FC $r smc 3000 5 11 780; done; done ;;
 ac) for r in ROW Y2 B2 Y3 B3 DOMF DOM; do run e2r:$E2 AC $r smc 1500 4 11 780; done
     for r in ROW Y2 B2; do run e2:$E2 AC $r smc 1500 4 11 780; done ;;
esac
