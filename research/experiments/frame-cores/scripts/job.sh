#!/bin/sh
# One refuter on one frames file, single thread. Called by xargs.
#   job.sh REFUTER PUZ FRAMES OUT TDFS TSAT MINCAP PERCALL CCAP
# REFUTER: R0-rows R0-spiral R1-rows R1-spiral R2, or R0-arc for partial frames
D=$(dirname "$0")
B=$D/../engine/target/release/fc
R=$1; P=$2; F=$3; O=$4; T=$5; TS=$6; M=$7; PC=$8; C=$9
case $R in
  R0-rows)   $B dfs --puz "$P" --frames "$F" --order rows --cap 1e15 --tcap "$T" > "$O" ;;
  R0-spiral) $B dfs --puz "$P" --frames "$F" --order spiral --cap 1e15 --tcap "$T" > "$O" ;;
  R1-rows)   $B dfs --puz "$P" --frames "$F" --order rows --hall --cap 1e15 --tcap "$T" > "$O" ;;
  R1-spiral) $B dfs --puz "$P" --frames "$F" --order spiral --hall --cap 1e15 --tcap "$T" > "$O" ;;
  R0-arc)    $B dfs --puz "$P" --frames "$F" --order rows --partial --cap 1e15 --tcap "$T" > "$O" ;;
  R2)        "$PY" "$D/sat.py" refute --puz "$P" --frames "$F" --tcap "$TS" --mincap "$M" --percall "$PC" --ccap "$C" > "$O" ;;
  *) echo "unknown refuter $R" >&2; exit 2 ;;
esac
