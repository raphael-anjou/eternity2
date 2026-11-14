#!/bin/sh
# Stage 0 validity checks on generated boards (no test seeds). See PREREG.md.
set -e
D=$(cd "$(dirname "$0")" && pwd)
EXP=$D/..
B=$EXP/engine/target/release/fc
J=${J:-10}
W=$EXP/work/stage0
mkdir -p $W
: "${PY:?}"
for s in 1 2 3 4; do
  $B gen --n 6 --colors 10 --bc 4 --seed $s --out $W/g6-$s.puz
  $B gen --n 7 --colors 12 --bc 4 --seed $s --out $W/g7-$s.puz
done
# (ii) brute force: every solution of every small board
ls $W/g*.puz | xargs -P 8 -I{} python3 -I $D/brute.py {} {}.brute.json
k=0
: > $W/small.jobs
for s in 1 2 3 4; do
  for g in g6-$s g7-$s; do
    k=$((k+1))
    $B sample --puz $W/$g.puz --stratum U --count 30 --np 2000 --seed $((7000+k)) --prefix ${g}s --out $W/$g.samp 2>/dev/null
    python3 -I $D/rightframes.py brute $W/$g.puz.brute.json ${g}r > $W/$g.right
    cat $W/$g.samp $W/$g.right > $W/$g.frames
    sh $D/batch.sh jobs $W/$g.puz $W/$g.frames $W/$g 60 60 60 5 60 >> $W/small.jobs
  done
done
# partial frames: bottom arcs of every small-board frame, k = 3 and 4 rows
for s in 1 2 3 4; do
  for g in g6-$s g7-$s; do
    n=$(head -1 $W/$g.puz | cut -d' ' -f2)
    mkdir -p $W/$g/arcref $W/$g/arcsplit
    python3 -I $D/arcs.py $W/$g.frames $n 3 4 > $W/$g.arcs
    while read -r id rest; do
      printf '%s %s\n' "$id" "$rest" > $W/$g/arcsplit/$id.frames
      echo "R2 $W/$g.puz $W/$g/arcsplit/$id.frames $W/$g/arcref/$id.R2.jsonl 60 60 0 0 0" >> $W/small.jobs
      echo "R0-arc $W/$g.puz $W/$g/arcsplit/$id.frames $W/$g/arcref/$id.R0-arc.jsonl 60 60 0 0 0" >> $W/small.jobs
    done < $W/$g.arcs
  done
done
J=$J sh $D/run_jobs.sh $W/small.jobs
for s in 1 2 3 4; do
  for g in g6-$s g7-$s; do
    sh $D/batch.sh post $W/$g.puz $W/$g.frames $W/$g $J
  done
done
# exact frame counts for every nogood on the 6x6 boards (compared with SMC)
for s in 1 2 3 4; do
  { echo "empty"; cat $W/g6-$s/subsets.txt; } > $W/g6-$s.subsets
done
ls $W/g6-*.subsets | xargs -P 4 -I{} sh -c "python3 -I $D/framecount.py \$(echo {} | sed 's/.subsets$/.puz/') {} > {}.exact.jsonl"
# (i) true frame of planted boards, 10x10 (no clue) and 16x16 (five clues)
for s in 801 802 803; do
  $B gen --n 10 --colors 14 --seed $s --out $W/t10-$s.puz
  $B gen --n 16 --colors 22 --seed $s --clues five --out $W/t16-$s.puz
done
: > $W/true.jobs
for b in t10-801 t10-802 t10-803 t16-801 t16-802 t16-803; do
  python3 -I $D/rightframes.py planted $W/$b.puz ${b}true > $W/$b.true
  for r in R0-rows R0-spiral R1-rows R1-spiral R2; do
    echo "$r $W/$b.puz $W/$b.true $W/$b.true.$r.jsonl 600 600 0 0 0" >> $W/true.jobs
  done
done
J=$J sh $D/run_jobs.sh $W/true.jobs
mkdir -p $EXP/results/stage0
python3 -I $D/stage0_check.py $W > $EXP/results/stage0/summary.json
