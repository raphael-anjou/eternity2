FCQ=$HOME/work/d2/t15ref/research/experiments/five-clue-quota/engine/target/release/fcq
FV=$HOME/work/d2-judge/five.csv
U=201,206,211,216,221,225,229,233,237,239
R=$HOME/work/d2-rejudge/runs/fcq
mkdir -p $R
s=$1
$FCQ bw --csv $FV --unlock $U --seed $s --cap 100000000 --priv 22,21,20 > $R/off-$s.json
$FCQ bw --csv $FV --unlock $U --seed $s --cap 100000000 --priv 22,21,20 --orientfix > $R/on-$s.json
