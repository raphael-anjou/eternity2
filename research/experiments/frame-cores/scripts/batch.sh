#!/bin/sh
# Two phases, same code path for stage 0, calibration and test.
#   batch.sh jobs PUZ FRAMES OUTDIR TDFS TSAT MINCAP PERCALL CCAP >> JOBFILE
#       splits FRAMES into one file per frame, prints one job line per
#       (frame, refuter); R2 lines are printed first by the caller's sort.
#   batch.sh post PUZ FRAMES OUTDIR THREADS
#       collects nogoods and counts their information (SMC, 20000 particles,
#       3 runs), plus N_F and N_H (10 runs each).
set -e
D=$(cd "$(dirname "$0")" && pwd)
B=$D/../engine/target/release/fc
MODE=$1; P=$2; F=$3; O=$4
mkdir -p "$O/ref" "$O/split" "$O/cnt"
if [ "$MODE" = jobs ]; then
  while read -r id rest; do
    printf '%s %s\n' "$id" "$rest" > "$O/split/$id.frames"
    for r in R2 R0-rows R0-spiral R1-rows R1-spiral; do
      echo "$r $P $O/split/$id.frames $O/ref/$id.$r.jsonl $5 $6 $7 $8 $9"
    done
  done < "$F"
  exit 0
fi
J=${5:-10}
python3 -I "$D/mkcores.py" "$F" "$O/ref" "$O/subsets.txt"
rm -f "$O"/cnt/part.*
split -n l/"$J" -d "$O/subsets.txt" "$O/cnt/part."
ls "$O"/cnt/part.?? | xargs -P "$J" -I{} sh -c "$B count --puz $P --stratum U --np 20000 --reps 3 --seed 31 --subsets {} > {}.jsonl"
cat "$O"/cnt/part.*.jsonl > "$O/counts.jsonl"
# the same nogoods counted inside the clue-block-passing population H
ls "$O"/cnt/part.?? | xargs -P "$J" -I{} sh -c "$B count --puz $P --stratum H --np 20000 --reps 3 --seed 32 --subsets {} > {}.h.jsonl"
cat "$O"/cnt/part.??.h.jsonl > "$O/counts_h.jsonl"
{ $B count --puz "$P" --stratum U --np 20000 --reps 10 --seed 41; $B count --puz "$P" --stratum H --np 20000 --reps 10 --seed 42; } > "$O/base.jsonl"
