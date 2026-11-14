#!/bin/sh
# Second pass on the official 16x16 pieces (the first pass, run16.sh, was stopped by the second analysis after its e2r FC part
# and the lines it had written are kept in real16.jsonl). Same guards: internal cpu limit and `timeout 840` per process.
# Larger populations, because the estimates for the dynamic rules turned out heavy-tailed at M = 3000.
cd "$(dirname "$0")"
E2=research/experiments/single-core-benchmark/engine/data/puzzles/size_16_official_eternity.csv
run() { out=$1; shift; timeout 840 nice -n 10 ./dyn16 "$@" | sed "s#$E2#official#" >> $out; }
case "$1" in
 fc) for r in ROW Y3 Y2 B2; do run real16_fc.jsonl e2r:$E2 FC $r smc 12000 5 31 600; done ;;
 ac) run real16_ac.jsonl e2r:$E2 AC ROW smc 2000 4 21 600; run real16_ac.jsonl e2r:$E2 AC Y3 smc 2000 4 21 600
     run real16_ac.jsonl e2r:$E2 AC B2 smc 2000 4 21 600; run real16_ac.jsonl e2r:$E2 AC Y2 smc 2000 3 21 600
     run real16_ac.jsonl e2r:$E2 AC DOMF smc 500 1 21 600; run real16_ac.jsonl e2r:$E2 AC DOM smc 500 1 21 600 ;;
 y) run real16_fc.jsonl e2r:$E2 FC Y3 smc 12000 8 61 600; run real16_fc.jsonl e2r:$E2 FC Y2 smc 12000 8 61 600 ;;
 la) run real16_fc.jsonl e2r:$E2 FC L2 smc 4000 3 41 600; run real16_ac.jsonl e2r:$E2 AC L2 smc 600 3 41 600 ;;
esac
# Diagnostic for the staircase rules under forward checking: a GENERATED 16x16 toy with the official colour numbers
# (5 frame, 17 interior), once with only cell 0 fixed and once with five more planted tiles in the cells the official
# clues occupy on the turned board. Usage: sh run16b.sh toy
if [ "$1" = toy ]; then
  for b in gen:16:5:17:1:c gen:16:5:17:1:c,34,45,120,210,221; do for r in ROW Y2 Y3; do
    timeout 840 nice -n 10 ./dyn16 $b FC $r smc 12000 4 71 600 >> toy16.jsonl; done; done
fi
