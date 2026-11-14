B=$HOME/work/d2-rejudge
E=research/experiments/single-core-benchmark/engine/target/release/blackwood_bt
OLD=$HOME/work/d2-judge/old/$E
J1=$HOME/work/d2-judge/new/$E
NEW=$B/head/$E
FCQ=$HOME/work/d2/t15ref/research/experiments/five-clue-quota/engine/target/release/fcq
NC=$HOME/work/d2-judge/noclue.csv
FV=$HOME/work/d2-judge/five.csv
CE=$HOME/work/d2-judge/centre.csv
L="--keying legacy-port --order row-major --legacy-break-path --isolation --legacy-hints --legacy-quota-test"
U=201,206,211,216,221,225,229,233,237,239
R=$B/runs
mkdir -p $R
J=$B/jobs.txt
: > $J
# supplementary first (longest)
for s in 501 502 503 504 505 506 507 508; do
  echo "c-$s $NEW $FV --restarts 1 --seed $s --nodecap 100000000 --hist" >> $J
done
for s in 501 502 503 504 505 506 507 508; do
  echo "a-$s $OLD $FV --restarts 1 --seed $s --nodecap 100000000" >> $J
  echo "b-$s $NEW $FV --restarts 1 --seed $s --nodecap 100000000 $L --hist" >> $J
done
# regression: legacy vs the pre-branch outputs of the first judgment
for s in 101 102 103 104 108; do echo "leg-noclue-$s $NEW $NC --seed $s --nodecap 100000000 $L --hist" >> $J; done
for s in 501 502 503 504 505 506; do echo "leg-five-$s $NEW $FV --seed $s --nodecap 100000000 $L --hist" >> $J; done
# regression: new default vs first-judgment binary, 1e7, hist
for s in 1 2 3 4; do
  echo "hnew-noclue-$s $NEW $NC --restarts 1 --seed $s --nodecap 10000000 --hist" >> $J
  echo "hj1-noclue-$s $J1 $NC --restarts 1 --seed $s --nodecap 10000000 --hist" >> $J
  echo "hnew-centre-$s $NEW $CE --restarts 1 --seed $s --nodecap 10000000 --hist" >> $J
  echo "hj1-centre-$s $J1 $CE --restarts 1 --seed $s --nodecap 10000000 --hist" >> $J
  echo "hnew-five-$s $NEW $FV --restarts 1 --seed $s --nodecap 10000000 --hist" >> $J
  echo "hj1-five-$s $J1 $FV --restarts 1 --seed $s --nodecap 10000000 --hist" >> $J
done
cat > $B/one.sh <<'EOS'
name=$1; shift
d=$HOME/work/d2-rejudge/runs/$name
mkdir -p $d
/usr/bin/time -p "$@" --emit-dir $d > $d/stdout 2> $d/stderr
EOS
xargs -P 4 -L 1 sh $B/one.sh < $J
# fcq companions for legal accounting of arms a (fix off) and b (fix on)
mkdir -p $R/fcq
for s in 501 502 503 504 505 506 507 508; do
  echo "$s" 
done | xargs -P 4 -I{} sh -c "$FCQ bw --csv $FV --unlock $U --seed {} --cap 100000000 --priv 22,21,20 > $R/fcq/off-{}.json; $FCQ bw --csv $FV --unlock $U --seed {} --cap 100000000 --priv 22,21,20 --orientfix > $R/fcq/on-{}.json"
echo BATCH_DONE
