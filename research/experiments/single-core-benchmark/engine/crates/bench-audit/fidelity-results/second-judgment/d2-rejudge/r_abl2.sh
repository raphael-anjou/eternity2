B=$HOME/work/d2-rejudge
NEW=$B/head/research/experiments/single-core-benchmark/engine/target/release/blackwood_bt
FV=$HOME/work/d2-judge/five.csv
J=$B/abl2.txt
: > $J
for s in 501 502 503 504 505 506 507 508; do
  echo "x-lh-$s $NEW $FV --restarts 1 --seed $s --nodecap 100000000 --hist --legacy-hints" >> $J
  echo "x-lq-$s $NEW $FV --restarts 1 --seed $s --nodecap 100000000 --hist --legacy-quota-test" >> $J
  echo "x-rolh-$s $NEW $FV --restarts 1 --seed $s --nodecap 100000000 --hist --order row-major --legacy-hints" >> $J
done
xargs -P 4 -L 1 sh $B/one.sh < $J
uptime
echo ABL2_DONE
