#!/bin/sh
# Referee runs for where-a-record-is-paid.REVIEW.md. Every run is capped at 1e8 nodes.
# engine/ is the memo's engine-copy (source only) plus one addition: the environment
# variable RV_QSCALE scales the quota array (default 1.0, search otherwise unchanged).
set -e
H=$(cd "$(dirname "$0")" && pwd); cd "$H"
D=../where-a-record-is-paid/data
U=201,206,211,216,221,225,229,233,237,239
BIN=$H/engine/target/release/rpp
(cd engine && cargo build --release 2>/dev/null)
one() { # name csv priv seed [qscale]
  RV_QSCALE=${5:-1.0} $BIN bw --csv $D/$2 --unlock $U --seed $4 --cap 1e8 --label $1 --priv $3 > out/$1.jsonl
}
# A. like-for-like control the memo lacks: the port's default keying with NO clue
one A-noclue-22-21-20-s1 official-noclue.csv 22,21,20 1 &
one A-noclue-22-21-20-s2 official-noclue.csv 22,21,20 2 & wait
one A-noclue-22-21-20-s3 official-noclue.csv 22,21,20 3 &
# B. the published key, Blackwood's {13,16,10} = repo {5,20,7} (v1_colour_map.py), no clue and centre clue
one B-noclue-5-20-7-s1 official-noclue.csv 5,20,7 1 & wait
one B-noclue-5-20-7-s2 official-noclue.csv 5,20,7 2 &
one B-noclue-5-20-7-s3 official-noclue.csv 5,20,7 3 & wait
one B-centre-5-20-7-s1 official-centre.csv 5,20,7 1 &
one B-centre-5-20-7-s2 official-centre.csv 5,20,7 2 & wait
# C. the memo's two openings with the clue the record engine carries (centre piece pinned)
one C-centre-3-12-13-s1 official-centre.csv 3,12,13 1 &
one C-centre-4-16-17-s1 official-centre.csv 4,16,17 1 & wait
# D. three interior colours with the quota scaled to their pool (150/122), as solver-engine's scale_c does
one D-noclue-22-21-20-q1.23-s1 official-noclue.csv 22,21,20 1 1.2295 &
one D-noclue-22-21-20-q1.23-s2 official-noclue.csv 22,21,20 2 1.2295 & wait
# E. same opening as B seed 1, without the port's isolation rule (second referee addition: RV_NOISO)
RV_NOISO=1 $BIN bw --csv $D/official-noclue.csv --unlock $U --seed 1 --cap 1e8 --label E-noiso-noclue-5-20-7-s1 --priv 5,20,7 > out/E-noiso-noclue-5-20-7-s1.jsonl
