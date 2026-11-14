#!/bin/sh
# Every command behind memos/where-a-record-is-paid.md. About 18 core-minutes of engine
# runs (2 at a time), 3 minutes of Python. Run from this folder.
set -e
H=$(cd "$(dirname "$0")" && pwd); cd "$H"
SRC=<repo>/research/experiments/real-puzzle-profile/engine
CSV=<repo>/research/experiments/single-core-benchmark/engine/data/puzzles/size_16_official_eternity.csv
U=201,206,211,216,221,225,229,233,237,239
BIN=$H/engine-copy/target/release/rpp

if [ ! -d engine-copy ]; then
  mkdir -p engine-copy data out
  cp -R $SRC/src $SRC/Cargo.toml $SRC/Cargo.lock engine-copy/
  cp $CSV data/official.csv
  python3 -I patch_engine.py; python3 -I patch_engine2.py; python3 -I patch_engine3.py; python3 -I patch_engine4.py
fi
# data/official-noclue.csv = data/official.csv with the three hint columns set to 0 (see the memo, section 7)
(cd engine-copy && cargo build --release)

# 1. the copy reproduces the committed histogram (checked against results/m1/record-s1.jsonl, 1e9 checkpoint)
$BIN bw --csv data/official.csv --unlock $U --seed 1 --cap 1e9 --label equiv --probe 201 --probeout out/probe201-port-s1.jsonl > out/equiv-port-s1-1e9.jsonl

# 2. one ring colour + two interior colours, 1e8 nodes
for cfg in 1,20,21 1,6,7 3,12,13; do n=$(echo $cfg | tr ',' '-')
  $BIN bw --csv data/official.csv --unlock $U --seed 1 --cap 1e8 --label official-priv$n --priv $cfg > out/e2-official-priv$n-s1-1e8.jsonl
done
for run in "1,20,21 1" "1,6,7 1" "2,14,15 1" "5,18,19 1" "3,12,13 2" "3,12,13 3" "3,12,13 4" "3,12,13 1" "4,16,17 1"; do
  set -- $run; n=$(echo $1 | tr ',' '-')
  $BIN bw --csv data/official-noclue.csv --unlock $U --seed $2 --cap 1e8 --label noclue-priv$n --priv $1 --probe 201 --probeout out/probe201-noclue-priv$n-s$2-1e8.jsonl > out/e2-official-noclue-priv$n-s$2-1e8.jsonl
done
$BIN bw --csv data/official-noclue.csv --unlock $U --seed 1 --cap 3e8 --label noclue-priv4-16-17 --priv 4,16,17 --probe 201 --probeout out/probe201-noclue-priv4-16-17-s1-3e8.jsonl > out/e2-official-noclue-priv4-16-17-s1-3e8.jsonl

# 3. more breaks: early extra unlocks, late extra unlocks
$BIN bw --csv data/official-noclue.csv --unlock 176,181,186,191,196,$U,241 --maxbreaks 16 --seed 1 --cap 1e8 --label K16 --priv 4,16,17 --probe 201 --probeout out/probe201-noclue-priv4-16-17-s1-K16-1e8.jsonl > out/e3-noclue-priv4-16-17-s1-K16-1e8.jsonl
$BIN bw --csv data/official-noclue.csv --unlock $U,240,242,244,246,248,250 --maxbreaks 16 --seed 1 --cap 1e8 --label K16late --priv 4,16,17 > out/e5-K16late-noclue-priv4-16-17-s1-1e8.jsonl
$BIN bw --csv data/official-noclue.csv --unlock $U,240,241,242,243,244,245,246,247,248,249 --maxbreaks 20 --seed 1 --cap 1e8 --label K20late --priv 4,16,17 > out/e5-K20late-noclue-priv4-16-17-s1-1e8.jsonl

# 4. legal-board histograms and legal probe rows (e4 was run before patches 3 and 4; e6 after)
$BIN bw --csv data/official-noclue.csv --unlock $U --seed 1 --cap 1e8 --label legal-4-16-17 --priv 4,16,17 > out/e4-legal-noclue-priv4-16-17-s1-1e8.jsonl
$BIN bw --csv data/official-noclue.csv --unlock $U --seed 1 --cap 1e8 --label legal-3-12-13 --priv 3,12,13 > out/e4-legal-noclue-priv3-12-13-s1-1e8.jsonl
$BIN bw --csv data/official-noclue.csv --unlock $U --seed 1 --cap 3e8 --label legalprobe-4-16-17 --priv 4,16,17 --probe 201 --probeout out/probeL201-noclue-priv4-16-17-s1-3e8.jsonl > out/e6-legalprobe-noclue-priv4-16-17-s1-3e8.jsonl
$BIN bw --csv data/official-noclue.csv --unlock $U --seed 1 --cap 1e8 --label legalprobe-3-12-13 --priv 3,12,13 --probe 201 --probeout out/probeL201-noclue-priv3-12-13-s1-1e8.jsonl > out/e6-legalprobe-noclue-priv3-12-13-s1-1e8.jsonl

# 5. analysis
python3 -I a1_hist.py > out/a1_hist.out
python3 -I a2_tail_model.py > out/a2_tail_model.out
python3 -I a3_probe.py > out/a3_probe.out
python3 -I a4_price.py > out/a4_price.out
python3 -I a5_probe_legal.py > out/a5_probe_legal.out
python3 -I a6_extra.py > out/a6_extra.out
python3 -I a7_legal_hist.py > out/a7_legal_hist.out

python3 -I a8_piece_counts.py > out/a8_piece_counts.out
