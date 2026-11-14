B=$HOME/work/d2-rejudge
NEW=$B/head/research/experiments/single-core-benchmark/engine/target/release/blackwood_bt
FV=$HOME/work/d2-judge/five.csv
J=$B/abl.txt
: > $J
for s in 501 502 503 504 505 506 507 508; do
  echo "x-ro-$s $NEW $FV --restarts 1 --seed $s --nodecap 100000000 --hist --order row-major" >> $J
  echo "x-kl-$s $NEW $FV --restarts 1 --seed $s --nodecap 100000000 --hist --keying legacy-port" >> $J
  echo "x-publeg-$s $NEW $FV --restarts 1 --seed $s --nodecap 100000000 --hist --keying legacy-port --legacy-break-path --isolation --legacy-hints --legacy-quota-test" >> $J
done
xargs -P 4 -L 1 sh $B/one.sh < $J
uptime
echo ABL_DONE
