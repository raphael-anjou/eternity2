#!/usr/bin/env bash
# A20 pipeline (PREREG.md). stage = build | calibrate | r6 | r7 | r8 | seeds23 | rerun | analyze
set -euo pipefail
cd "$(dirname "$0")"
STAGE="${1:-analyze}"
T="${THREADS:-2}"
export RAYON_NUM_THREADS="$T"
BIN=compute/target/release/blocker_backjump
TABU_FILE=results/calibration/selected_tabu.txt

go() { # outdir, then extra args
  local out="$1"; shift
  [ -s "$out/bj-blocker.jsonl" ] && { echo "skip $out (exists)"; return; }
  mkdir -p "$out"
  nice "$BIN" run --threads "$T" --outdir "$out" "$@" 2>"$out/run.log"
}
tabu() { cat "$TABU_FILE"; }

case "$STAGE" in
  build) cargo build --release -j "$T" --manifest-path compute/Cargo.toml ;;
  calibrate)
    for t in 0 16 128; do
      go results/calibration/t$t --arms bj-blocker,bj-random --ladder primary --role calibration --sizes 6,7 --tabu "$t"
    done
    go results/calibration/base --arms bb-mrv,scb-gacolor_ac3_random,bj-blocker --ladder primary --role calibration --sizes 6,7 --tabu 16
    python3 analysis/analyze.py calibrate ;;
  r6) go results/test/r6 --arms scb-gacolor_ac3_random,bb-mrv,bj-blocker,bj-random --ladder primary --role test --sizes 6 --tabu "$(tabu)" ;;
  r7) go results/test/r7 --arms scb-gacolor_ac3_random,bb-mrv,bj-blocker,bj-random --ladder primary --role test --sizes 7 --tabu "$(tabu)" ;;
  r8)
    go results/test/r8a --arms bb-mrv,scb-gacolor_ac3_random,bj-blocker,bj-random --ladder 8-c10 --role test --seeds 21-30 --tabu "$(tabu)"
    go results/test/r8b --arms bb-mrv,scb-gacolor_ac3_random,bj-blocker,bj-random --ladder 8-c10-fresh --role test --seeds 31-50 --tabu "$(tabu)" ;;
  seeds23)
    for s in 2 3; do
      for n in 6 7; do
        go results/test/r$n-seed$s --arms bj-blocker,bj-random --ladder primary --role test --sizes $n --tabu "$(tabu)" --run-seed $s
      done
    done ;;
  analyze) python3 analysis/analyze.py test ;;
  *) echo "unknown stage $STAGE"; exit 2 ;;
esac
