#!/usr/bin/env bash
# A22 pipeline (PREREG.md). stage = all | build | check | calibrate | q1 | q2 | extended | ablations | implcheck | analyze
set -euo pipefail
cd "$(dirname "$0")"
STAGE="${1:-all}"
T="${THREADS:-8}"
BIN=compute/target/release/mrv_baseline
HL=../hard-ladder/compute/target/release/hard_ladder
R=results/raw

build() {
  cargo build --release --manifest-path compute/Cargo.toml
  cargo build --release --manifest-path ../hard-ladder/compute/Cargo.toml
}
check() {
  "$HL" check-instances
  cargo test --release --manifest-path compute/Cargo.toml
}
arm() { # arm solver set ladder seeds [sizes] [cap] [guard]
  local s="$1" set="$2" lad="$3" seeds="$4" sizes="${5:-}" cap="${6:-1000000}" g="${7:-600}"
  local out="$R/$s/$set.jsonl"
  [ -s "$out" ] && { echo "skip $out (exists)"; return; }
  mkdir -p "$(dirname "$out")"
  "$BIN" run --solver "$s" --ladder "$lad" --role test --seeds "$seeds" ${sizes:+--sizes "$sizes"} \
    --cap "$cap" --guard "$g" --threads "$T" --out "$out" 2>"$out.log"
}
q1() { # Q1 instances: R6/R7 seeds 21..30, R8 = 8-c10 21..30 + 8-c10-fresh 31..50
  local s="$1"
  arm "$s" q1-r6r7 primary 21-30 6,7
  arm "$s" q1-r8a 8-c10 21-30
  arm "$s" q1-r8b 8-c10-fresh 31-50
}
calibrate() {
  mkdir -p results/calibration
  for s in bb-mrv scb-gacolor_ac3_random; do
    out="results/calibration/$s.jsonl"
    [ -s "$out" ] || "$BIN" run --solver "$s" --ladder plus2 --role calibration --cap 2000000 --guard 600 \
      --threads "$T" --out "$out" 2>"$out.log"
  done
}
q2() {
  for s in bb-mrv scb-gacolor_ac3_random hp-dfs; do arm "$s" q2-plus2 plus2 1-30; done
}
extended() { # sizes with fewer than 10/30 solved by both at 1e6 (analysis/analyze.py --need-extended)
  for n in $(python3 analysis/analyze.py --need-extended); do
    for s in bb-mrv scb-gacolor_ac3_random; do
      if [ "$n" = 8 ]; then
        arm "$s" ext-r8a 8-c10 21-30 "" 10000000 180
        arm "$s" ext-r8b 8-c10-fresh 31-50 "" 10000000 180
      else
        arm "$s" "ext-n$n" plus2 1-30 "$n" 10000000 180
      fi
    done
  done
}
ablations() {
  for s in mrv-nohs mrv-bf mrv-rand scb-ad; do q1 "$s"; done
  arm scb-fork-off q1-r6r7 primary 21-30 6,7
}
implcheck() { # bb-mrv on A16's used seeds 1..20: implementation check only, never evidence
  mkdir -p results/check
  local f
  for f in primary:6,7 8-c10:; do
    local lad="${f%%:*}" sz="${f#*:}" out="results/check/bb-mrv-a16-${f%%:*}.jsonl"
    [ -s "$out" ] || "$BIN" run --solver bb-mrv --ladder "$lad" --role test --seeds 1-20 ${sz:+--sizes "$sz"} \
      --threads "$T" --out "$out" 2>"$out.log"
  done
}
analyze() { python3 analysis/analyze.py; }

case "$STAGE" in
  build) build ;;
  check) check ;;
  calibrate) calibrate ;;
  q1) for s in bb-mrv scb-gacolor_ac3_random hp-dfs; do q1 "$s"; done ;;
  q2) q2 ;;
  extended) extended ;;
  ablations) ablations ;;
  implcheck) implcheck ;;
  analyze) analyze ;;
  all)
    build; check; calibrate
    for s in bb-mrv scb-gacolor_ac3_random hp-dfs; do q1 "$s"; done
    q2; extended; ablations; implcheck; analyze ;;
  *) echo "unknown stage $STAGE"; exit 2 ;;
esac
