#!/bin/sh
# Toy ladder (GENERATED boards, not Eternity II). One core, nice 10, one process at a time.
# Every process has a branch cap, an internal cpu limit of 780 s and `timeout 840` (hard limit under 15 minutes).
# All boards have the planted tile of cell 0 fixed (breaks the four-fold turn symmetry; cell 0 is the head of every rule).
cd "$(dirname "$0")"
ex() { out=$1; shift; timeout 840 nice -n 10 "$@" exact 2000000000 780 >> $out; }
sm() { out=$1; shift; timeout 840 nice -n 10 "$@" 780 >> $out; }
FCR="ROW Y2 B2 DOMF DOM"; ACR="ROW Y2 B2 DOMF DOM WDEG"
case "$1" in
 x5) for s in 1 2 3 4 5 6 7 8 9 10 11 12; do b=gen:5:4:3:$s:c; ex exact.jsonl ./dyn8 $b PLAIN ROW
       for r in $FCR; do ex exact.jsonl ./dyn8 $b FC $r; done; for r in $ACR; do ex exact.jsonl ./dyn8 $b AC $r; done; done ;;
 x6) for s in 1 2 3 4 5 6 7 8 9 10 11 12; do b=gen:6:4:5:$s:c; ex exact.jsonl ./dyn8 $b PLAIN ROW
       for r in $FCR; do ex exact.jsonl ./dyn8 $b FC $r; done; for r in $ACR; do ex exact.jsonl ./dyn8 $b AC $r; done; done ;;
 x7) for s in $2; do b=gen:7:4:7:$s:c
       for r in ROW DOMF; do ex exact.jsonl ./dyn8 $b FC $r; done; for r in ROW DOMF DOM WDEG B2; do ex exact.jsonl ./dyn8 $b AC $r; done; done ;;
 x7c) for s in $2; do b=gen:7:4:6:$s:c; for r in ROW DOMF; do ex exact.jsonl ./dyn8 $b AC $r; done; done ;;
 s) # population estimates at the ladder colour count of each size: args n ci "seeds" binary
    n=$2; ci=$3; bin=$5
    for s in $4; do b=gen:$n:4:$ci:$s:c; sm smc.jsonl ./$bin $b PLAIN ROW smc 2000 2 5
      for r in ROW Y2 B2 DOMF DOM; do sm smc.jsonl ./$bin $b FC $r smc 2000 2 5; done
      for r in ROW Y2 B2 DOMF DOM; do sm smc.jsonl ./$bin $b AC $r smc 2000 2 5; done; done ;;
 c8) # five clues placed like the official ones, scaled to 8x8 (D5's M8C cells): (2,2) (2,5) (5,2) (5,5) (3,4); no corner pin
    for s in 1 2 3 4 5 6 7 8; do b=gen:8:4:11:$s:18,21,42,45,28; ex clues8.jsonl ./dyn8 $b PLAIN ROW; ex clues8.jsonl ./dyn8 $b FC ROW
      for r in ROW B2 DOMF DOM; do ex clues8.jsonl ./dyn8 $b AC $r; done; done ;;
 c8s) for s in 1 2 3 4 5 6; do b=gen:8:4:7:$s:18,21,42,45,28; sm clues8.jsonl ./dyn8 $b PLAIN ROW smc 2000 2 5
      for r in ROW B2 DOMF DOM; do sm clues8.jsonl ./dyn8 $b AC $r smc 2000 2 5; done; done ;;
 l) # one-step lookahead rule, exact, small sizes
    for s in 1 2 3 4 5 6 7 8 9 10 11 12; do b=gen:6:4:5:$s:c; ex exact.jsonl ./dyn8 $b FC L2; ex exact.jsonl ./dyn8 $b AC L2; done ;;
 v8) # (first pass also ran DOMF: board 1 complete, board 2 stopped by the second analysis after about 4 minutes, no line written)
    # estimator check for the staircase rules: exact against small-population estimates, 8x8, 11 interior colours
    for s in 1 2 3; do b=gen:8:4:11:$s:c; for r in ROW Y2 Y3 B2; do ex valid8.jsonl ./dyn8 $b FC $r; sm valid8.jsonl ./dyn8 $b FC $r smc 100 30 9; done; done ;;
esac
