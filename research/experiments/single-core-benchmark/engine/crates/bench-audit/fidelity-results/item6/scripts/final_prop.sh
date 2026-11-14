#!/bin/sh
H=$(dirname "$0")
L='--keying legacy-port --order row-major --legacy-break-path --isolation --legacy-hints --legacy-quota-test'
OF=base/research/experiments/single-core-benchmark/engine/data/puzzles/size_16_official_eternity.csv
NC=data/research/program/night/2026-11-12/memos/where-a-record-is-paid/data/official-noclue.csv
ssh -F $SSH_CONFIG -o BatchMode=yes m1 'rm -rf ~/work/d2/prop'
rm -rf "$H/prop"
sh "$H/prop1.sh" f5 $OF 10000000 22,21,20 "$L" 501 502 503 504 505 506 507 508 509 510 511 512 513 514 515 516
sh "$H/prop1.sh" f5late $OF 100000000 22,21,20 "$L" 501 506 507
sh "$H/prop1.sh" fn13 $NC 10000000 22,21,20 "$L" 101 102 103 104 105 106 107 108 109 110
sh "$H/prop1.sh" fn13m3 $NC 10000000 3,12,13 "$L" 105 108
