E=research/experiments/single-core-benchmark/engine/target/release/blackwood_bt
OLD=$HOME/work/d2-judge/old/$E
J1=$HOME/work/d2-judge/new/$E
NEW=$HOME/work/d2-rejudge/head/$E
NC=$HOME/work/d2-judge/noclue.csv
L="--keying legacy-port --order row-major --legacy-break-path --isolation --legacy-hints --legacy-quota-test"
T=$HOME/work/d2-rejudge/timing
mkdir -p $T/scratch
uptime > $T/uptime.txt
for k in 1 2 3; do
  /usr/bin/time -p $OLD $NC --restarts 1 --seed 102 --nodecap 100000000 --emit-dir $T/scratch > /dev/null 2> $T/old-$k.txt
  /usr/bin/time -p $J1 $NC --restarts 1 --seed 102 --nodecap 100000000 $L --emit-dir $T/scratch > /dev/null 2> $T/j1-$k.txt
  /usr/bin/time -p $NEW $NC --restarts 1 --seed 102 --nodecap 100000000 $L --emit-dir $T/scratch > /dev/null 2> $T/new-$k.txt
done
uptime >> $T/uptime.txt
echo TIME_DONE
