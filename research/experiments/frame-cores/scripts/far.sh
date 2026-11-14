#!/bin/sh
# A2 addendum: extra real H frames with no dead 3x3 clue block (frames away
# from the corner-local deaths). Rejection from H by `fc flags`, seed 20261115.
set -e
D=$(cd "$(dirname "$0")" && pwd)
EXP=$D/..
B=$EXP/engine/target/release/fc
J=${J:-10}
W=$EXP/work/test
: "${PY:?}"
mkdir -p $W
$B sample --puz $W/real.puz --stratum H --count 240 --np 5000 --seed 20261115 --prefix realHx --out $W/realHx.all 2> $W/realHx.log
$B flags --puz $W/real.puz --frames $W/realHx.all > $W/realHx.flags.jsonl
python3 -I - $W/realHx.all $W/realHx.flags.jsonl $W/realfar.frames <<'PY'
import json, sys
fl = {json.loads(l)["id"]: json.loads(l) for l in open(sys.argv[2])}
keep = [l for l in open(sys.argv[1]) if not fl[l.split()[0]]["dead_block3x3"]][:60]
open(sys.argv[3], "w").writelines(keep)
print(len(keep), "kept")
PY
cp $W/real.puz $W/realfar.puz
sh $D/batch.sh jobs $W/realfar.puz $W/realfar.frames $W/realfar 120 600 300 3 600 > $W/far.jobs
J=$J sh $D/run_jobs.sh $W/far.jobs
sh $D/batch.sh post $W/realfar.puz $W/realfar.frames $W/realfar $J
$B flags --puz $W/realfar.puz --frames $W/realfar.frames > $W/realfar.flags.jsonl
R=$EXP/results/test/realfar
mkdir -p $R
cp $W/realfar.puz $W/realfar.frames $W/realfar.flags.jsonl $W/realHx.all $W/realHx.flags.jsonl $R/
cat $W/realfar/ref/*.jsonl > $R/refuters.jsonl
cp $W/realfar/subsets.txt $W/realfar/counts.jsonl $W/realfar/counts_h.jsonl $W/realfar/base.jsonl $R/
