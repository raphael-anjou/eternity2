#!/bin/sh
# runs on m1 in ~/work/d2: user seconds per 1e8 nodes, seed 102 no clue, 3 repeats each
cd ~/work/d2 || exit 1
NC=data/research/program/night/2026-11-12/memos/where-a-record-is-paid/data/official-noclue.csv
L='--keying legacy-port --order row-major --legacy-break-path --isolation --legacy-hints --legacy-quota-test'
for b in "$@"; do
  BW=$b/research/experiments/single-core-benchmark/engine/target/release/blackwood_bt
  F=$L; [ "$b" = base ] && F=
  for k in 1 2 3; do
    /usr/bin/time -p $BW $NC --restarts 1 --seed 102 --nodecap 100000000 $F --emit-dir scratch/t 2>&1 | grep -E "^user|nodes=" | tr '\n' ' '
    echo " $b"
  done
done
