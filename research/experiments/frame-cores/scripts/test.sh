#!/bin/sh
# Test stage (PREREG.md). Called through prereg_guard by run.sh test.
set -e
D=$(cd "$(dirname "$0")" && pwd)
EXP=$D/..
B=$EXP/engine/target/release/fc
J=${J:-10}
W=$EXP/work/test
: "${PY:?}" "${CSV:?}"
mkdir -p $W
CAPS="120 600 300 3 600"
$B real --csv "$CSV" --out $W/real.puz
$B sample --puz $W/real.puz --stratum H --count 60 --np 5000 --seed 20261113 --prefix realH --out $W/realH.frames 2> $W/realH.log
$B sample --puz $W/real.puz --stratum U --count 30 --np 5000 --seed 20261114 --prefix realU --out $W/realU.frames 2> $W/realU.log
cat $W/realH.frames $W/realU.frames > $W/real.frames
sh $D/batch.sh jobs $W/real.puz $W/real.frames $W/real $CAPS > $W/all.jobs
i=0
for b in 1101 1102 1103; do
  i=$((i+1))
  $B gen --n 16 --colors 22 --bc 5 --seed $b --clues five --out $W/null$b.puz
  $B sample --puz $W/null$b.puz --stratum H --count 20 --np 5000 --seed $((20261120+i)) --prefix n${b}H --out $W/n${b}H.frames 2> $W/n${b}H.log
  $B sample --puz $W/null$b.puz --stratum U --count 10 --np 5000 --seed $((20261130+i)) --prefix n${b}U --out $W/n${b}U.frames 2> $W/n${b}U.log
  cat $W/n${b}H.frames $W/n${b}U.frames > $W/null$b.frames
  sh $D/batch.sh jobs $W/null$b.puz $W/null$b.frames $W/null$b $CAPS >> $W/all.jobs
done
# bottom arcs of every real H frame: A1 item 4 as changed by A2 (k = 15, 14,
# 13, 12; R2 on each, cap 120 s except 450 s at k = 14; R0-arc once per frame at k = 15)
A=$W/real-arcs
mkdir -p $A/ref $A/split
python3 -I $D/arcs.py $W/realH.frames 16 15 14 13 12 > $W/realH.arcs
while read -r id rest; do
  printf '%s %s\n' "$id" "$rest" > $A/split/$id.frames
  # A2 addendum: k = 13 cap 6400 s (twice the 40-cell budget at 2.5e6 nodes/s)
  # on the first 30 frames, 120 s on the others
  case $id in
    *k14) T=450 ;;
    realH[0-9]k13|realH[12][0-9]k13) T=6400 ;;
    *) T=120 ;;
  esac
  echo "R2 $W/real.puz $A/split/$id.frames $A/ref/$id.R2.jsonl 120 $T 0 0 0" >> $W/all.jobs
  case $id in
    *k15) echo "R0-arc $W/real.puz $A/split/$id.frames $A/ref/$id.R0-arc.jsonl 120 120 0 0 0" >> $W/all.jobs ;;
  esac
done < $W/realH.arcs
# A2 item 1: dead 3-cell and 3x3 clue blocks of every test frame (no solver time)
for s in real null1101 null1102 null1103; do
  $B flags --puz $W/$s.puz --frames $W/$s.frames > $W/$s.flags.jsonl
done
J=$J sh $D/run_jobs.sh $W/all.jobs
{ $B count --puz $W/real.puz --stratum H --np 20000 --reps 3 --seed 51 --subsets $W/realH.arcs
  $B count --puz $W/real.puz --stratum U --np 20000 --reps 3 --seed 52 --subsets $W/realH.arcs; } > $A/counts.jsonl
sh $D/batch.sh post $W/real.puz $W/real.frames $W/real $J
for b in 1101 1102 1103; do
  sh $D/batch.sh post $W/null$b.puz $W/null$b.frames $W/null$b $J
done
# raw results into results/test (frames, refuter rows, nogood counts)
R=$EXP/results/test
mkdir -p $R
for s in real null1101 null1102 null1103; do
  mkdir -p $R/$s
  cp $W/$s.puz $W/$s.frames $W/$s.flags.jsonl $R/$s/
  cat $W/$s/ref/*.jsonl > $R/$s/refuters.jsonl
  cp $W/$s/subsets.txt $W/$s/counts.jsonl $W/$s/counts_h.jsonl $W/$s/base.jsonl $R/$s/
done
mkdir -p $R/real-arcs
cp $W/realH.arcs $R/real-arcs/
cat $A/ref/*.jsonl > $R/real-arcs/refuters.jsonl
cp $A/counts.jsonl $R/real-arcs/
cp $W/*.log $R/
