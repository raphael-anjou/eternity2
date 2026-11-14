#!/usr/bin/env bash
# real-puzzle-profile: every stage goes through the prereg guard.
#   ./run.sh build | equiv | gate-gen | gate-a | gate-b | m1 | m2 | frames | paint
# Single-threaded processes; at most 4 run at once (JOBS, default 4).
set -euo pipefail
cd "$(dirname "$0")"
EXP="$(pwd)"
GUARD="python3 $EXP/../common/prereg_guard.py $EXP"
CSV="$EXP/../single-core-benchmark/engine/data/puzzles/size_16_official_eternity.csv"
BIN="$EXP/engine/target/release/rpp"
JOBS="${JOBS:-4}"
R="$EXP/results"
mkdir -p "$R/m1" "$R/m2" "$R/gate"

case "${1:-}" in
  build)
    cargo build --release --manifest-path engine/Cargo.toml
    ;;
  equiv)
    # blackwood_bt itself, built from the single-core-benchmark workspace
    BW="${BW_BIN:?set BW_BIN to the blackwood_bt binary}"
    $GUARD equiv -- sh -c "nice $BW $CSV --stage 4 --restarts 1 --seed 1 --nodecap 100000000 --emit-dir $R/gate/bw-emit > $R/gate/equiv-blackwood_bt.txt 2>&1"
    $GUARD equiv -- sh -c "nice $BIN bw --csv $CSV --unlock 201,206,211,216,221,225,229,233,237,239 --seed 1 --cap 1e8 --label equiv > $R/gate/equiv-port.jsonl"
    ;;
  gate-gen)
    $GUARD gate-gen -- sh -c "nice $BIN gate-gen --np 1000 --reps 100 > $R/gate/gen-np1000.jsonl" &
    $GUARD gate-gen -- sh -c "nice $BIN gate-gen --np 100 --reps 100 > $R/gate/gen-np100.jsonl" &
    wait
    ;;
  gate-a)
    $GUARD gate-a -- sh -c "nice $BIN gate-a --jsonl $EXP/../hard-ladder/instances/primary-test.jsonl --np 200 --reps 200 > $R/gate/gate-a.jsonl"
    ;;
  gate-b)
    $GUARD gate-b -- sh -c "nice $BIN smc --csv $CSV --clues none --order ring --np 100000 --reps 20 --seed0 1 --label gate-b-ring > $R/gate/gate-b.jsonl"
    ;;
  m1)
    REC=201,206,211,216,221,225,229,233,237,239
    G201=201,201,201,201,201,201,201,201,201,201
    G221=221,221,221,221,221,221,221,221,221,221
    {
      if [ -n "${ONLY_NOQUOTA:-}" ]; then
        for s in 1 2 3 4 5 6; do echo "record-noquota $REC $s --noquota"; done
      else
      for s in 1 2 3 4 5 6; do echo "record $REC $s"; done
      for s in 1 2 3 4 5 6; do echo "gate221 $G221 $s"; done
      for s in 1 2 3 4 5 6; do echo "gate201 $G201 $s"; done
      for s in 1 2 3 4 5 6; do echo "record-noquota $REC $s --noquota"; done
      fi
    } | xargs -P "$JOBS" -L 1 sh -c '
      name=$0; unlock=$1; seed=$2; extra=${3:-}
      out='"$R"'/m1/$name-s$seed.jsonl
      [ -s "$out" ] && exit 0
      '"$GUARD"' m1 -- sh -c "nice '"$BIN"' bw --csv '"$CSV"' --unlock $unlock --seed $seed --cap 1e10 --label $name $extra > $out"'
    ;;
  m2)
    # two at a time (M1 holds the other two cores while it runs)
    $GUARD m2 -- sh -c "nice $BIN smc --csv $CSV --clues five --order rows --np 1000000 --reps 5 --seed0 101 --label five-np1e6 > $R/m2/five-np1e6.jsonl" &
    $GUARD m2 -- sh -c "nice $BIN smc --csv $CSV --clues five --order rows --np 100000 --reps 5 --seed0 201 --label five-np1e5 > $R/m2/five-np1e5.jsonl" &
    wait
    $GUARD m2 -- sh -c "nice $BIN smc --csv $CSV --clues centre --order rows --np 100000 --reps 5 --seed0 301 --label centre-np1e5 > $R/m2/centre-np1e5.jsonl" &
    $GUARD m2 -- sh -c "nice $BIN smc --csv $CSV --clues none --order rows --np 100000 --reps 5 --seed0 401 --label none-np1e5 > $R/m2/none-np1e5.jsonl" &
    wait
    ;;
  frames)
    for b in v129-palimpsest-463 v181-keyring-460; do
      f="$R/m2/frame-$b.txt"
      [ -s "$f" ] || continue
      $GUARD frames -- sh -c "nice $BIN enum --csv $CSV --clues five --fixed $f --order spiral --cap 1e10 --label frame-$b > $R/m2/frame-$b-enum.jsonl"
    done
    ;;
  equiv2)
    $GUARD equiv2 -- sh -c "nice $BIN bw --csv $CSV --unlock 201,206,211,216,221,225,229,233,237,239 --seed 1 --cap 1e8 --label equiv2 > $R/gate/equiv2-port.jsonl"
    ;;
  m1b-val)
    $GUARD m1b-val -- sh -c "nice $BIN bwval --csv $CSV --unlock 201,206,211,216,221,225,229,233,237,239 --d0 196 --pool 200000 --stride 1 --need 206 --count 10 --np 200 --reps 100 --cap 2e8 --seed0 7 --label m1b-val > $R/gate/m1b-val.jsonl"
    ;;
  m1b-check)
    $GUARD m1b-check -- sh -c "nice $BIN bwsmc --exact-root --cap 1e8 --csv $CSV --unlock 201,206,211,216,221,225,229,233,237,239 --label m1b-exact-root > $R/gate/m1b-exact-root.jsonl"
    $GUARD m1b-check -- sh -c "nice $BIN bwval --csv $CSV --unlock 201,206,211,216,221,225,229,233,237,239 --d0 196 --pool 200000 --stride 1 --need 206 --count 10 --np 10 --reps 200 --cap 2e8 --seed0 7 --label m1b-val-np10 > $R/gate/m1b-val-np10.jsonl"
    ;;
  m1-1e11)
    REC=201,206,211,216,221,225,229,233,237,239
    for s in ${SEEDS:-1 2 3 4}; do
      $GUARD m1-1e11 -- sh -c "nice $BIN bw --csv $CSV --unlock $REC --seed $s --cap 1e11 --label record-1e11 > $R/m1/record-1e11-s$s.jsonl" &
    done
    wait
    ;;
  m1b)
    REC=201,206,211,216,221,225,229,233,237,239
    G221=221,221,221,221,221,221,221,221,221,221
    $GUARD m1b -- sh -c "nice $BIN bwsmc --csv $CSV --unlock $REC --np 100000 --reps 5 --seed0 11 --label record > $R/m1/m1b-record.jsonl" &
    $GUARD m1b -- sh -c "nice $BIN bwsmc --csv $CSV --unlock $G221 --np 100000 --reps 5 --seed0 21 --label gate221 > $R/m1/m1b-gate221.jsonl" &
    wait
    $GUARD m1b -- sh -c "nice $BIN bwsmc --csv $CSV --unlock $REC --noquota --np 100000 --reps 5 --seed0 31 --label record-noquota > $R/m1/m1b-record-noquota.jsonl"
    ;;
  frames-noclue)
    for b in v129-palimpsest-463 v181-keyring-460; do
      $GUARD frames-noclue -- sh -c "nice $BIN enum --csv $CSV --clues none --fixed $R/m2/frame-$b.txt --order spiral --cap 1e10 --label frame-noclue-$b > $R/m2/frame-noclue-$b-enum.jsonl"
    done
    ;;
  paint)
    $GUARD paint -- sh -c "nice $BIN paint --csv $CSV --count ${COUNT:-20} --reps 2 --np 100000 > $R/m2/paint-np1e5.jsonl"
    ;;
  *) echo "usage: $0 build|equiv|gate-gen|gate-a|gate-b|m1|m2|frames|paint" >&2; exit 2 ;;
esac
