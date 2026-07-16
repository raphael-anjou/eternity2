#!/usr/bin/env bash
# Reproduce the community-engines comparison on both grids, single core, 60s.
#
# Every engine emits a board in its own format; a Python post-step rescores it
# through THE canonical scorer (canonical_rescore.py, validated to reproduce the
# known 469 board) and appends a row to results-unpinned.jsonl /
# results-pinned-collapse.jsonl. Engine self-reports are never trusted; the
# canonical rescore is authoritative.
#
# Throughput is captured per engine in its native unit and NEVER cross-compared:
#   McGavin  -> tiles/s        (its print_state "Rate")
#   Blackwood-> search-nodes/s (its NPS probe)
#   Verhaard -> search-nodes/s (our reimpl, run_algo nps)
#   break-2  -> search-nodes/s (our dfs-study engine, run_dfs nps)
#
# Prereqs on this machine: clang, ~/.dotnet/dotnet (net8), and the prebuilt
# run_algo (single-core-benchmark) and run_dfs (dfs-study) binaries.
#
# The community engines' OWN sources are NOT vendored in this repo (see the
# .gitignore): each stays with its author under its own licence. To rerun, place
# them under engines/ first, from where the lab pages say they live:
#   engines/mcgavin/     genbody.c + e2.puz + *.hnt   (genbody71.zip, groups.io msg 11749)
#   engines/blackwood/   *.cs + *.csproj              (github.com/jblackwood345/EternityII_Solver, GPL-3.0)
#   engines/verhaard/    best_so_far.url              (our reimpl runs via run_algo; original is Win32-only)
# Everything else here (parsers, this script, the rescored results) IS committed.
set -euo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DFS="$(cd "$HERE/.." && pwd)"                              # dfs-study experiment
REPO="$(cd "$DFS/../../.." && pwd)"                        # repo root
SCB="$REPO/research/experiments/single-core-benchmark"
BUDGET_S="${BUDGET_S:-60}"
WORK="${WORK:-$HERE/.work}"                                # scratch build/run dir
mkdir -p "$WORK"

RUN_ALGO="$SCB/engine/target/release/run_algo"
RUN_DFS="$DFS/engine/target/release/run_dfs"
DOTNET="$HOME/.dotnet/dotnet"

UNPINNED_OUT="$HERE/results-unpinned.jsonl"
PINNED_OUT="$HERE/results-pinned-collapse.jsonl"
: > "$UNPINNED_OUT"
: > "$PINNED_OUT"

log() { printf '[run_community] %s\n' "$*" >&2; }

# --- McGavin: codegen body.c for a (puz,hnt), compile, solve BUDGET_S, rescore.
# $1=hnt path  $2=label  $3=out jsonl  $4=grid  $5=instance
run_mcgavin() {
  local hnt="$1" label="$2" out="$3" grid="$4" inst="$5"
  local w="$WORK/mcg_$label"; rm -rf "$w"; mkdir -p "$w/solutions"
  cp "$HERE/engines/mcgavin/genbody.c" "$HERE/engines/mcgavin/e2.puz" "$w/"
  cp "$hnt" "$w/hints.hnt"
  ( cd "$w"
    # Build the solver with the flags McGavin himself documents for ARM
    # (groups.io msg 11751): native tuning plus link-time optimisation. On this
    # M1 that is measurably faster than a plain -Ofast build (median tiles/s up
    # about 1.6x), so his throughput here is not understated by a lazy build.
    # The generator (-DG pass) stays on plain -Ofast; only the solver is hot.
    clang -o genbody_g genbody.c -lm -Ofast -w -DG
    ./genbody_g e2.puz hints.hnt >/dev/null 2>&1
    clang -o gs genbody.c -lm -Ofast -w -mcpu=native -mtune=native -flto
    ( ./gs e2.puz hints.hnt >run.log 2>&1 ) & local pid=$!
    sleep "$((BUDGET_S + 2))"; kill -INT "$pid" 2>/dev/null || true
    sleep 1; kill -9 "$pid" 2>/dev/null || true; wait "$pid" 2>/dev/null || true )
  # Best board: highest-depth solution file if any, else the last stdout dump.
  local best; best="$(ls "$w"/solutions/*.txt 2>/dev/null | sort -t_ -k4 -n | tail -1 || true)"
  local board="${best:-$w/run.log}"
  python3 "$HERE/emit_result.py" --engine mcgavin --board "$board" \
    --run-log "$w/run.log" --grid "$grid" --instance "$inst" \
    --label "mcgavin-c" --display "McGavin (C)" >> "$out"
  log "mcgavin $label done"
}

# --- Blackwood: build once, run per clue-mode BUDGET_S, rescore best_board.txt.
BW_BUILT=""
build_blackwood() {
  [ -n "$BW_BUILT" ] && return 0
  local b="$WORK/bw"; rm -rf "$b"; mkdir -p "$b"
  cp "$HERE/engines/blackwood/"*.cs "$HERE/engines/blackwood/EternityII_Solver.csproj" "$b/"
  ( cd "$b" && DOTNET_CLI_TELEMETRY_OPTOUT=1 DOTNET_NOLOGO=1 "$DOTNET" build -c Release -v quiet >/dev/null )
  BW_BUILT="$b"
}
# $1=clue-mode(centre|5clue) $2=label $3=out $4=grid $5=instance
run_blackwood() {
  build_blackwood
  local mode="$1" label="$2" out="$3" grid="$4" inst="$5"
  local b="$BW_BUILT"; local dll; dll="$(find "$b/bin" -name EternityII_Solver.dll -path '*net8.0*' | head -1)"
  ( cd "$b" && rm -f best_board.txt
    BW_CLUES="$mode" BW_BUDGET_MS="$((BUDGET_S * 1000))" "$DOTNET" "$dll" >stdout.log 2>stderr.log )
  python3 "$HERE/emit_result.py" --engine blackwood --board "$b/best_board.txt" \
    --run-log "$b/stderr.log" --grid "$grid" --instance "$inst" \
    --label "blackwood-cs" --display "Blackwood (C#)" >> "$out"
  log "blackwood $mode done"
}

# --- Verhaard reimpl (ours) + break-2 (ours) via run_algo / run_dfs on JSON.
# $1=bin $2=algo $3=variant.json $4=seed $5=label $6=display $7=out $8=grid $9=inst $10=native|dfs
run_ours() {
  local bin="$1" algo="$2" var="$3" seed="$4" label="$5" disp="$6" out="$7" grid="$8" inst="$9" kind="${10}"
  local emit="$WORK/${label}_${inst}_s${seed}.json"
  if [ "$kind" = native ]; then
    RAYON_NUM_THREADS=1 "$bin" --puzzle "$var" --algo "$algo" --seed "$seed" \
      --budget-ms "$((BUDGET_S * 1000))" --emit "$emit" >"$WORK/${label}_${inst}_s${seed}.out" 2>/dev/null
  else
    RAYON_NUM_THREADS=1 "$bin" --puzzle "$var" --algo "$algo" --seed "$seed" \
      --budget-s "$BUDGET_S" --emit "$emit" >"$WORK/${label}_${inst}_s${seed}.out" 2>/dev/null
  fi
  python3 "$HERE/emit_result.py" --engine bucas --board "$emit" \
    --run-log "$WORK/${label}_${inst}_s${seed}.out" --grid "$grid" --instance "$inst" \
    --seed "$seed" --label "$label" --display "$disp" >> "$out"
  log "$label $inst seed=$seed done"
}

log "=== UNPINNED grid (centre clue only) ==="
# McGavin: seed-invariant for a fixed hint set -> one run stands for all seeds.
run_mcgavin "$HERE/engines/mcgavin/e2_1.hnt" "centre" "$UNPINNED_OUT" unpinned "centre-clue"
# Blackwood: unseeded (new Random()); one run (stable across repeats we measured).
run_blackwood centre "centre" "$UNPINNED_OUT" unpinned "centre-clue"
# Our engines on 3 unpinned seeds each (head-to-head baseline + variance).
for s in 1 7 13; do
  V="$DFS/variants-unpinned/unpinned_0$([ $s = 1 ] && echo 0 || { [ $s = 7 ] && echo 1 || echo 2; }).json"
  run_ours "$RUN_ALGO" verhaard_preferred "$V" "$s" verhaard-reimpl "Verhaard (our reimpl)" "$UNPINNED_OUT" unpinned "unpinned_v" native
  run_ours "$RUN_DFS" break-2 "$V" "$s" break-2 "break-2 (ours)" "$UNPINNED_OUT" unpinned "unpinned_v" dfs
done

log "=== PINNED grid (corner pins -> collapse) ==="
# McGavin on 2 pinned variants: arbitrary corner pins dead-end the fixed scan.
for v in 00 01; do
  python3 "$HERE/make_mcgavin_hnt.py" "$DFS/variants/variant_$v.json" "$WORK/pin_$v.hnt"
  run_mcgavin "$WORK/pin_$v.hnt" "pin$v" "$PINNED_OUT" pinned "variant_$v"
done
# Blackwood's constrained analog: the 5 official clues it cannot honour with its
# fixed scan (it hardcodes pieces, so arbitrary corner pins are not expressible).
run_blackwood 5clue "5clue" "$PINNED_OUT" pinned "5-official-clues"

log "DONE. results-unpinned.jsonl / results-pinned-collapse.jsonl written."
