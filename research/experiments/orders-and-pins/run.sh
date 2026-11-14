#!/bin/sh
# orders-and-pins: every stage that runs the estimator goes through the
# prereg guard. Resumable: rerun the same stage after a kill.
#   ./run.sh build | plans | gate6 | gt | gb | g8 | b0 | bl | b1 | bs | b4 | b3 | b2 | a-official | a-null | all
# all = the A4 (M12) order: gb g8 b0 bl b1 bs b4 b3 b2 a-official a-null
# JOBS (default 2) single-threaded processes at a time, each under nice -n 10.
set -eu
cd "$(dirname "$0")"
EXP="$(pwd)"
JOBS="${JOBS:-2}"
GUARD="python3 $EXP/../common/prereg_guard.py $EXP"
mkdir -p results/gate results/a results/b

case "${1:-}" in
  build)
    cargo build --release -j 2 --manifest-path engine/Cargo.toml
    ;;
  plans)
    python3 -I scripts/orders.py
    python3 -I scripts/pins_model.py rows-b-lr
    python3 -I scripts/plan.py
    ;;
  gate6|gt|gb|g8)
    $GUARD "$1" -- python3 -I scripts/drive.py "plans/$1.jsonl" "results/gate/$1.jsonl" --jobs "$JOBS"
    ;;
  a-official|a-null)
    $GUARD "$1" -- python3 -I scripts/drive.py "plans/$1.jsonl" "results/a/$1.jsonl" --jobs "$JOBS"
    ;;
  b0|bl|b1|b2|bs|b3|b4)
    $GUARD "$1" -- python3 -I scripts/drive.py "plans/$1.jsonl" "results/b/$1.jsonl" --jobs "$JOBS"
    ;;
  all)
    for st in gb g8 b0 bl b1 bs b4 b3 b2 a-official a-null; do
      "$0" "$st" 2>> "results/drive-$st.log"
    done
    ;;
  *)
    echo "usage: $0 build|plans|gate6|gt|gb|g8|b0|bl|b1|bs|b4|b3|b2|a-official|a-null|all" >&2
    exit 2
    ;;
esac
