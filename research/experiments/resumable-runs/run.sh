#!/usr/bin/env bash
# Track 9 (resumable runs for bb-mrv-fast). See PREREG.md.
# Stages: build | gate-a | gate-b | gate-c | time | analyze | all, and for the
# fix round (PREREG_AMENDMENTS.md A1): fix-bin | fix-gates | tamper | fix-analyze.
# Every run goes through ../common/prereg_guard.py (logs to results/run-log.jsonl).
# A finished piece (its final output file exists) is skipped; delete it to rerun.
set -euo pipefail
cd "$(dirname "$0")"
BIN=${BIN:-compute/target/release/resumable_runs}
RAW=${RAW:-results/raw}
CK=${CK:-results/ckpt}
FIXBIN=compute/target/gate-bin/resumable_runs
stage="${1:-all}"
mkdir -p "$RAW" "$CK"

g() { # g STAGE CMD...
  local s=$1; shift
  python3 ../common/prereg_guard.py . "$s" -- nice "$@"
}

build() { cargo build --release -j 2 --manifest-path compute/Cargo.toml; }

# Track 0's gate set: name ladder role extra-args
SETS=(
  "r6-cal primary calibration --sizes 6"
  "r6-test primary test --sizes 6"
  "r7-cal primary calibration --sizes 7"
  "r7-test primary test --sizes 7"
  "r8-cal 8-c10 calibration"
  "r8-test 8-c10 test"
  "r8-fresh 8-c10-fresh test"
)
N9="n9 plus2 calibration --sizes 9 --seeds 1001-1010"

ga_set() { # ga_set CAP PROGRESS_EVERY name ladder role [args]
  local cap=$1 pe=$2 name=$3 ladder=$4 role=$5; shift 5
  local out=$RAW/ga-$name.jsonl
  if [ -s "$out" ]; then echo "skip $out"; return; fi
  rm -f "$RAW/progress-ga-$name.jsonl" "$out.tmp"
  g gate-a "$BIN" gate-a --ladder "$ladder" --role "$role" "$@" --cap "$cap" \
    --progress "$RAW/progress-ga-$name.jsonl" --progress-every "$pe" --out "$out.tmp"
  mv "$out.tmp" "$out"
}

gb_set() { # gb_set C1 C2 C3 PROGRESS_EVERY name ladder role [args]
  local c1=$1 c2=$2 c3=$3 pe=$4 name=$5 ladder=$6 role=$7; shift 7
  local fin=$RAW/gb-$name-s3.jsonl
  if [ -e "$fin" ]; then echo "skip $name"; return; fi
  rm -rf "$CK/gb-$name" "$RAW"/gb-"$name"-s?.jsonl "$RAW/progress-gb-$name.jsonl"
  local P=(--progress "$RAW/progress-gb-$name.jsonl" --progress-every "$pe")
  g gate-b "$BIN" run --ladder "$ladder" --role "$role" "$@" --cap "$c1" \
    --checkpoint-dir "$CK/gb-$name/s1" "${P[@]}" --out "$RAW/gb-$name-s1.jsonl"
  mkdir -p "$CK/gb-$name/s1" "$CK/gb-$name/s2"
  : > "$RAW/gb-$name-s2.jsonl.tmp"
  g gate-b "$BIN" resume --from "$CK/gb-$name/s1" --cap "$c2" \
    --checkpoint-dir "$CK/gb-$name/s2" "${P[@]}" --out "$RAW/gb-$name-s2.jsonl.tmp"
  mv "$RAW/gb-$name-s2.jsonl.tmp" "$RAW/gb-$name-s2.jsonl"
  mkdir -p "$CK/gb-$name/s3"
  : > "$RAW/gb-$name-s3.jsonl.tmp"
  g gate-b "$BIN" resume --from "$CK/gb-$name/s2" --cap "$c3" \
    --checkpoint-dir "$CK/gb-$name/s3" "${P[@]}" --out "$RAW/gb-$name-s3.jsonl.tmp"
  mv "$RAW/gb-$name-s3.jsonl.tmp" "$RAW/gb-$name-s3.jsonl"
}

gate_a() { for s in "${SETS[@]}"; do ga_set 1000000 100000 $s; done; }
gate_b() { for s in "${SETS[@]}"; do gb_set 333333 666666 1000000 100000 $s; done; }
gate_c() {
  ga_set 3000000 100000 $N9
  gb_set 1000000 2000000 3000000 100000 $N9
}

timing() {
  for s in "r8-test 8-c10 test" "r8-fresh 8-c10-fresh test"; do
    set -- $s
    local out=$RAW/time-$1.jsonl
    if [ -s "$out" ]; then echo "skip $out"; continue; fi
    rm -f "$out.tmp" "$RAW/progress-time-$1.jsonl"
    g time "$BIN" time --ladder "$2" --role "$3" --cap 1000000 \
      --progress "$RAW/progress-time-$1.jsonl" --out "$out.tmp"
    mv "$out.tmp" "$out"
  done
}

analyze() { python3 -I analysis/analyze.py; }

# ---- fix round (A1) ---------------------------------------------------------
fix_bin() { build; mkdir -p "$(dirname $FIXBIN)"; cp compute/target/release/resumable_runs "$FIXBIN"; }
# One gate stage with the frozen fixed binary into results/fix/.
fix_gate() { BIN=$FIXBIN RAW=results/fix/raw CK=results/fix/ckpt ./run.sh "$1"; }

TAMPER=(
  "primary test --sizes 7 --seeds 1-1 --cap 400000 n7-c6-s1"
  "8-c10 test --seeds 5-5 --cap 333333 n8-c10-s5"
  "plus2 test --sizes 9 --seeds 1-1 --cap 500000 n9-c11-s1"
)
tamper() {
  local T=results/fix/tamper
  mkdir -p "$T"
  for s in "${TAMPER[@]}"; do
    set -- $s
    local ladder=$1 role=$2 id=${!#}
    shift 2
    local out=$T/tamper-$id.jsonl
    if [ -s "$out" ]; then echo "skip $out"; continue; fi
    if [ ! -e "$T/ck/$id.ckpt.json" ]; then
      g tamper "$FIXBIN" run --ladder "$ladder" --role "$role" "${@:1:$#-1}" \
        --checkpoint-dir "$T/ck" --guard 0 --out "$T/make-$id.jsonl"
    fi
    rm -rf "compute/target/tamper-work/$id"
    g tamper "$FIXBIN" tamper --from "$T/ck/$id.ckpt.json" --extra 10000 \
      --work "compute/target/tamper-work/$id" --out "$out.tmp"
    mv "$out.tmp" "$out"
  done
}

case "$stage" in
  build) build ;;
  gate-a) gate_a ;;
  gate-b) gate_b ;;
  gate-c) gate_c ;;
  time) timing ;;
  analyze) analyze ;;
  fix-bin) fix_bin ;;
  fix-gate-a) fix_gate gate-a ;;
  fix-gate-b) fix_gate gate-b ;;
  fix-gate-c) fix_gate gate-c ;;
  tamper) tamper ;;
  fix-analyze) python3 -I analysis/analyze.py results/fix/raw results/fix/summary.json
               python3 -I analysis/fix_compare.py ;;
  all) build; gate_a; gate_b; gate_c; timing; analyze ;;
  *) echo "unknown stage $stage"; exit 1 ;;
esac
