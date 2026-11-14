#!/bin/sh
# Autonomous driver on the compute host (the ssh link is down until 22:00).
# Each step appends name, start, end and exit code to ~/work/d1/PROGRESS.txt;
# a failed step does not stop later ones. Results are mirrored to
# ~/work/d1/results after every step.
EXP=$HOME/work/d1/research/experiments/frame-cores
P=$HOME/work/d1/PROGRESS.txt
GUARD="python3 -I $EXP/../common/prereg_guard.py $EXP"
cd $EXP
export J=10
export PY=$(nix build --no-link --print-out-paths --impure --expr 'with(import<nixpkgs>{});python3.withPackages(p:[p.python-sat])')/bin/python3
export CSV=$EXP/../single-core-benchmark/engine/data/puzzles/size_16_official_eternity.csv
step() {
  name=$1; shift
  echo "$name start $(date -Is)" >> $P
  "$@"; rc=$?
  echo "$name end $(date -Is) exit $rc" >> $P
  mkdir -p $HOME/work/d1/results
  rsync -a $EXP/results/ $HOME/work/d1/results/
  cp $EXP/*.log $HOME/work/d1/results/ 2>/dev/null
}
wait_stage0() {
  i=0
  while [ ! -s results/stage0/summary.json ] && [ $i -lt 360 ]; do sleep 10; i=$((i+1)); done
  [ -s results/stage0/summary.json ]
}
step stage0-wait wait_stage0
step cal sh run.sh cal
step cal-save sh -c "mkdir -p results/cal && for s in real null901; do cat work/cal/\$s/ref/*.jsonl > results/cal/\$s.refuters.jsonl; cp work/cal/\$s/counts.jsonl results/cal/\$s.counts.jsonl; cp work/cal/\$s/base.jsonl results/cal/\$s.base.jsonl; done; cp work/cal/*.frames work/cal/*.puz results/cal/"
step test $GUARD test -- sh scripts/test.sh
step test-far $GUARD test-far -- sh scripts/far.sh
step analyze sh -c "python3 -I scripts/analyze.py results/test results/stage0/summary.json > results/summary.json && python3 -I scripts/tables.py results/summary.json > results/tables.md"
echo "driver done $(date -Is)" >> $P
