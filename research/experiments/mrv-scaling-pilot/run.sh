#!/usr/bin/env bash
# Track 1 pilot (PREREG.md). stage = build | check | test | analyze | all
# bash 3.2 compatible (macOS /bin/bash): no wait -n, no associative arrays.
set -euo pipefail
cd "$(dirname "$0")"
STAGE="${1:-all}"
HERE="$PWD"
export CARGO_TARGET_DIR="$HERE/target"
A22_BIN="$HERE/target/release/mrv_baseline"
PILOT_BIN="$HERE/target/release/pilot_instances"
CAP=100000000
DEADLINE=1794543300 # 2026-11-13 05:15:00 CET, preregistered stop (epoch, same on every machine)
MACHINE="${MACHINE:-m4}"
SEEDS="${SEEDS:-101 102 103 104 105 106 107 108}"
SLOTS="${SLOTS:-4}"
R=results/raw
HB="results/heartbeat-${MACHINE:-m4}.log"

build() {
  nice -n 5 cargo build --release --locked -j 4 --manifest-path ../mrv-baseline/compute/Cargo.toml
  nice -n 5 cargo build --release --locked -j 4 --manifest-path compute/Cargo.toml
}

check() {
  "$PILOT_BIN" check
  mkdir -p results/check
  for s in bb-mrv scb-gacolor_ac3_random; do
    out="results/check/$s-plus2-n7-s1-10.jsonl"
    [ -s "$out" ] || RAYON_NUM_THREADS=1 nice -n 5 "$A22_BIN" run --solver "$s" --ladder plus2 --role test \
      --sizes 7 --seeds 1-10 --cap 1000000 --guard 600 --threads "$SLOTS" --out "$out" 2>"$out.log"
  done
  python3 -I analysis/analyze.py --build-check
}

# bb-mrv binary for a wave: the A22 binary unless bb-binary.txt names another
# (written only after the lead reports Track 0's judge returned keep).
bb_bin() { if [ -s bb-binary.txt ]; then cat bb-binary.txt; else echo "$A22_BIN"; fi; }

heartbeat() { # pid run
  local pid="$1" run="$2" t0
  t0=$(date +%s)
  while kill -0 "$pid" 2>/dev/null; do
    echo "$(date '+%Y-%m-%dT%H:%M:%S') $run pid=$pid elapsed_s=$(( $(date +%s) - t0 )) cpu=$(ps -o time= -p "$pid" 2>/dev/null | tr -d ' ')" >>"$HB"
    sleep 300
  done
}

run_one() { # solver seed wave bin
  local s="$1" seed="$2" wave="$3" bin="$4"
  local run="$s-s$seed" tmp="$R/runs/$s-s$seed.jsonl"
  local now guard started pid
  [ -s "$tmp" ] && { echo "skip $run (row exists)"; return; }
  now=$(date +%s); guard=$(( DEADLINE - now ))
  [ "$guard" -le 0 ] && { echo "past deadline, not launching $run"; return; }
  started=$(date '+%Y-%m-%dT%H:%M:%S%z')
  HARD_LADDER_ROOT="$HERE" RAYON_NUM_THREADS=1 nice -n 5 "$bin" run --solver "$s" --ladder plus2 --role test \
    --sizes 9 --seeds "$seed-$seed" --cap "$CAP" --guard "$guard" --threads 1 --out "$tmp" 2>"$R/runs/$run.log" &
  pid=$!
  echo "$(date '+%Y-%m-%dT%H:%M:%S') launch $run wave=$wave pid=$pid guard_s=$guard bin=$bin" >>"$HB"
  heartbeat "$pid" "$run" &
  wait "$pid" || echo "$run exited non-zero" >>"$HB"
  python3 -I analysis/append_row.py "$tmp" "$R/wave$wave-$MACHINE.jsonl" "$bin" "$wave" "$MACHINE" "$started" \
    "$(date '+%Y-%m-%dT%H:%M:%S%z')" "$guard"
  echo "$(date '+%Y-%m-%dT%H:%M:%S') done $run $(tail -c 300 "$R/runs/$run.log" | tail -1)" >>"$HB"
}

committer() { # commit results/ every 30 min while the test stage runs
  while sleep 1800; do
    git add results >/dev/null 2>&1 && git commit -q -m "research(mrv-scaling-pilot): progress $(date '+%H:%M')" -- results \
      >/dev/null 2>&1 || true
  done
}

test_stage() {
  mkdir -p "$R/runs"
  committer & local cpid=$!
  local seed wave bin
  local prev=0
  for seed in $SEEDS; do
    wave=$(( (seed - 99) / 2 ))
    [ "$wave" != "$prev" ] && bin=$(bb_bin)  # binary fixed at the start of each wave
    prev=$wave
    while [ "$(jobs -rp | grep -vc "^$cpid\$")" -gt $(( SLOTS - 2 )) ]; do sleep 15; done
    run_one bb-mrv "$seed" "$wave" "$bin" &
    run_one scb-gacolor_ac3_random "$seed" "$wave" "$A22_BIN" &
    sleep 2
  done
  while [ "$(jobs -rp | grep -vc "^$cpid\$")" -gt 0 ]; do sleep 15; done
  kill "$cpid" 2>/dev/null || true
  git add results && git commit -q -m "research(mrv-scaling-pilot): all runs finished" -- results || true
}

analyze() { python3 -I analysis/analyze.py; }

case "$STAGE" in
  build) build ;;
  check) check ;;
  test) test_stage ;;
  analyze) analyze ;;
  all) build; check; test_stage; analyze ;;
  *) echo "unknown stage $STAGE"; exit 2 ;;
esac
