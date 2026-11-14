#!/bin/sh
# pin-map: every measured stage goes through the prereg guard, one core, nice.
#   sh run.sh build | validate | <stage> | all | analyze
# stages: boards s8 g8 p8 pairs8 s8c g8c p8c s10 g10 p10 n8 mrv8
set -eu
cd "$(dirname "$0")"
EXP="$(pwd)"
GUARD="python3 $EXP/../common/prereg_guard.py $EXP"
case "${1:-}" in
  build)
    cargo build --release -j 1 --manifest-path engine/Cargo.toml
    ;;
  validate)
    nice -n 10 python3 -I scripts/validate.py calib/boards/n8-f4-c12-s9001.txt 10
    nice -n 10 python3 -I scripts/validate.py calib/boards/n8-f4-c11-s9002.txt 10
    ;;
  boards|s8|g8|p8|pairs8|s8c|g8c|p8c|s10|g10|p10|n8|mrv8)
    nice -n 10 $GUARD "$1" -- python3 -I scripts/drive.py "$1"
    ;;
  all)
    for st in boards s8 g8 p8 pairs8 s8c g8c p8c s10 g10 p10 n8 mrv8; do
      sh "$0" "$st"
    done
    ;;
  analyze)
    uv run --with matplotlib python -I scripts/analyze.py
    ;;
  *)
    echo "usage: sh run.sh build|validate|boards|s8|g8|p8|pairs8|s8c|g8c|p8c|s10|g10|p10|n8|mrv8|all|analyze" >&2
    exit 2
    ;;
esac
