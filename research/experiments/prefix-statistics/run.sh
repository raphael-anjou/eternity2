#!/usr/bin/env bash
# prefix-statistics: every gate and test command goes through the prereg guard.
#   ./run.sh build | gate-exact | gate-rep | calib | a4check | painter | test | real | analyze
# At most JOBS single-threaded processes at once (default 2), nice 10.
# Resumable: a finished output file is skipped; any other is rerun.
# `./run.sh job <stage> <args>` runs one guarded job (used through xargs).
set -euo pipefail
cd "$(dirname "$0")"
EXP="$(pwd)"
GUARD="python3 $EXP/../common/prereg_guard.py $EXP"
CSV="$EXP/../single-core-benchmark/engine/data/puzzles/size_16_official_eternity.csv"
BIN="$EXP/engine/target/release/psx"
JOBS="${JOBS:-2}"
R="$EXP/results"
G="$R/${GATE_DIR:-gate}"
mkdir -p "$G" "$R/test" "$R/real"

case "${1:-}" in
  build)
    cargo build --release -j 2 --manifest-path engine/Cargo.toml
    ;;
  job)
    stage="$2"
    case "$stage" in
      gate-exact)
        b="$3"; out="$G/exact-$b.jsonl"
        [ -s "$out" ] && exit 0
        $GUARD gate-exact -- sh -c "nice -n 10 $BIN gate --n 6 --colors 9 --border 2 --boards $b --np 500 --reps 20 --m 500 --cap 200000000 > $out.part && mv $out.part $out"
        ;;
      gate-rep)
        b="$3"; tag="$4"; np="$5"; s="$6"; out="$G/rep-b$b-np$tag-s$s.jsonl"
        grep -q '"done"' "$out" 2>/dev/null && exit 0
        $GUARD gate-rep -- sh -c "nice -n 10 $BIN measure --board $b --dirs 0 --depths 96,128,144,160,176 --np $np --smc-seed $s --m 1000 --role gaterep > $out"
        ;;
      calib)
        b="$3"; out="$R/calib/board-$b.jsonl"
        grep -q '"done"' "$out" 2>/dev/null && exit 0
        $GUARD calib -- sh -c "nice -n 10 $BIN measure --board $b --m 1000 --role calib > $out"
        ;;
      test)
        b="$3"; out="$R/test/board-$b.jsonl"
        grep -q '"done"' "$out" 2>/dev/null && exit 0
        $GUARD test -- sh -c "nice -n 10 $BIN measure --board $b --np 500000 --m 1000 --role test > $out"
        ;;
      *) echo "unknown job stage $stage" >&2; exit 2 ;;
    esac
    ;;
  gate-exact)
    printf '%s\n' 9001 9002 9003 9004 9005 9006 9007 9008 9009 9010 | xargs -P "$JOBS" -n 1 ./run.sh job gate-exact
    ;;
  gate-rep)
    {
      for b in 7002 7003; do
        for s in 11 12 13; do echo "$b 1e5 100000 $s"; done
        echo "$b 1e6 1000000 31"
        for s in 21 22 23; do echo "$b 1e4 10000 $s"; done
      done
    } | xargs -P "$JOBS" -L 1 ./run.sh job gate-rep
    ;;
  calib)
    mkdir -p "$R/calib"
    seq -f %.0f 7007 7016 | xargs -P "$JOBS" -n 1 ./run.sh job calib
    ;;
  test)
    seq -f %.0f 1113001 1113096 | xargs -P "$JOBS" -n 1 ./run.sh job test
    ;;
  real)
    out="$R/real/real.jsonl"
    $GUARD real -- sh -c "nice -n 10 $BIN real --csv $CSV --np 500000 --m 1000 --smc-seed 7 > $out"
    ;;
  a4check)
    # A4 change 1: board 7007 rerun with the A4 binary at the calibration
    # settings; every pre-existing non-timing field must be byte-identical.
    mkdir -p "$R/a4check"
    out="$R/a4check/board-7007.jsonl"
    $GUARD a4check -- sh -c "nice -n 10 $BIN measure --board 7007 --m 1000 --role calib > $out"
    python3 -I scripts/identity.py "$R/calib/board-7007.jsonl" "$out"
    ;;
  painter)
    # A4 change 8: 6x6, 8 colours (2 frame), boards 9101 to 9140, exact.
    mkdir -p "$R/painter"
    out="$R/painter/painter.jsonl"
    $GUARD painter -- sh -c "nice -n 10 $BIN painter --n 6 --colors 8 --border 2 --seed0 9101 --count 40 --cap 400000000 > $out"
    python3 -I scripts/painter.py "$out"
    ;;
  analyze)
    uv run --with numpy python3 -I scripts/analyze.py "$R/test" --real "$R/real/real.jsonl" --json "$R/summary.json"
    ;;
  *) echo "usage: $0 build|gate-exact|gate-rep|test|real|analyze" >&2; exit 2 ;;
esac
