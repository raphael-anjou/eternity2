#!/bin/sh
# runs on m1 in ~/work/d2. args: label csv cap priv bwflags seeds...
# off: frozen Track 15 engine, fix off, legal cap; onl: fix on, legal cap; on: fix on, node cap;
# bw: this branch's blackwood_bt, node cap.
cd ~/work/d2 || exit 1
LABEL=$1; CSV=$2; CAP=$3; PRIV=$4; BWFLAGS=$5; shift 5
BW=head/research/experiments/single-core-benchmark/engine/target/release/blackwood_bt
FCQ=t15ref/research/experiments/five-clue-quota/engine/target/release/fcq
U=201,206,211,216,221,225,229,233,237,239
mkdir -p prop/$LABEL
for s in "$@"; do
  $FCQ bw --csv $CSV --unlock $U --seed $s --cap 1.3e8 --legalcap $CAP --priv $PRIV > prop/$LABEL/off-$s.json
  $FCQ bw --csv $CSV --unlock $U --seed $s --cap 1.3e8 --legalcap $CAP --priv $PRIV --orientfix > prop/$LABEL/onl-$s.json
  $FCQ bw --csv $CSV --unlock $U --seed $s --cap $CAP --priv $PRIV --orientfix > prop/$LABEL/on-$s.json
  $BW $CSV --restarts 1 --seed $s --nodecap $CAP --priv $PRIV $BWFLAGS --hist --emit-dir prop/$LABEL/bw-$s > /dev/null 2> prop/$LABEL/bw-$s.txt
done
echo done $LABEL
