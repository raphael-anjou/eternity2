#!/usr/bin/env bash
# A21 pipeline (PREREG.md). stage = all | build | calibrate | test | check | analyze
# The runner appends and resumes: an interrupted stage can be rerun.
set -euo pipefail
cd "$(dirname "$0")"
STAGE="${1:-all}"
T="${THREADS:-2}"
export RAYON_NUM_THREADS="$T"
BIN=compute/target/release/value_impact
CTRL=vi-probe-rand-s1,vi-probe-rand-s2,vi-probe-rand-s3,mrv-rand-s1,mrv-rand-s2,mrv-rand-s3
SCB=scb-gacolor_ac3_random

build() { cargo build --release -j "$T" --manifest-path compute/Cargo.toml; }
go() { # rung role arms reps out
  mkdir -p "$(dirname "$5")"
  "$BIN" run --rung "$1" --role "$2" --arms "$3" --reps "$4" --threads "$T" --out "$5" 2>>"$5.log"
}
calibrate() { for r in r6 r7 r8; do go "$r" calibration bb-mrv,vi-index,vi-impact 1 "results/calibration/$r.jsonl"; done; }
test_() { # interleaved timed arms; order R8, R7, R6 (PREREG compute budget)
  go r8 test "bb-mrv,vi-impact,$CTRL" 1 results/test/r8.jsonl
  go r7 test "bb-mrv,vi-impact,$CTRL,$SCB" 1 results/test/r7.jsonl
  go r6 test "bb-mrv,vi-impact,$CTRL,$SCB" 5 results/test/r6.jsonl
}
check() { for r in r6 r7 r8; do go "$r" test vi-index 1 "results/check/$r.jsonl"; done; }
analyze() { python3 -I analysis/analyze.py; }

case "$STAGE" in
  build) build ;;
  calibrate) calibrate ;;
  test) test_ ;;
  check) check ;;
  analyze) analyze ;;
  all) build; calibrate; test_; check; analyze ;;
  *) echo "unknown stage $STAGE"; exit 2 ;;
esac
