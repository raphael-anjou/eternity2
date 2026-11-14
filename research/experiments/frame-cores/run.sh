#!/bin/sh
# frame-cores driver. Runs on the compute host from the experiment directory
# (research/experiments/frame-cores). Stages:
#   build    cargo build (nix toolchain)
#   stage0   validity checks (generated boards only, no test seeds)
#   cal      calibration set (seeds disjoint from test)
#   test     the preregistered test stage, through prereg_guard
# Threads: J (default 10).
set -e
cd "$(dirname "$0")"
EXP=$(pwd)
B=$EXP/engine/target/release/fc
J=${J:-10}
GUARD="python3 -I $EXP/../common/prereg_guard.py $EXP"
CSV=${CSV:-$EXP/../single-core-benchmark/engine/data/puzzles/size_16_official_eternity.csv}
if [ -z "$PY" ]; then
  PY=$(nix build --no-link --print-out-paths --impure --expr 'with(import<nixpkgs>{});python3.withPackages(p:[p.python-sat])')/bin/python3
fi
export PY

case "$1" in
build)
  cd engine && nix shell nixpkgs#cargo nixpkgs#rustc nixpkgs#gcc -c cargo build --release
  ;;
stage0)
  export J
  sh scripts/stage0.sh
  ;;
cal)
  W=work/cal; mkdir -p $W
  $B real --csv "$CSV" --out $W/real.puz
  $B gen --n 16 --colors 22 --seed 901 --clues five --out $W/null901.puz
  $B sample --puz $W/real.puz --stratum H --count 12 --np 5000 --seed 9101 --prefix realH --out $W/realH.frames
  $B sample --puz $W/real.puz --stratum U --count 6 --np 5000 --seed 9102 --prefix realU --out $W/realU.frames
  $B sample --puz $W/null901.puz --stratum H --count 6 --np 5000 --seed 9103 --prefix n901H --out $W/n901H.frames
  $B sample --puz $W/null901.puz --stratum U --count 3 --np 5000 --seed 9104 --prefix n901U --out $W/n901U.frames
  cat $W/realH.frames $W/realU.frames > $W/real.frames
  cat $W/n901H.frames $W/n901U.frames > $W/null.frames
  { sh scripts/batch.sh jobs $W/real.puz $W/real.frames $W/real 60 60 120 3 60
    sh scripts/batch.sh jobs $W/null901.puz $W/null.frames $W/null901 60 60 120 3 60; } > $W/all.jobs
  J=$J sh scripts/run_jobs.sh $W/all.jobs
  sh scripts/batch.sh post $W/real.puz $W/real.frames $W/real $J
  sh scripts/batch.sh post $W/null901.puz $W/null.frames $W/null901 $J
  ;;
test)
  export CSV
  $GUARD test -- sh scripts/test.sh
  ;;
*)
  echo "usage: run.sh build|stage0|cal|test" >&2; exit 2
  ;;
esac
