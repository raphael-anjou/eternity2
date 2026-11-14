#!/bin/sh
# F1 identity checks on m1, ~/work/d2. base = pre-branch binary (637a42be), c8af01491 = branch before F1, head = after F1
cd ~/work/d2 || exit 1
NC=data/research/program/night/2026-11-12/memos/where-a-record-is-paid/data/official-noclue.csv
CE=data/research/program/night/2026-11-12/memos/where-a-record-is-paid/data/official-centre.csv
OF=base/research/experiments/single-core-benchmark/engine/data/puzzles/size_16_official_eternity.csv
L='--keying legacy-port --order row-major --legacy-break-path --isolation --legacy-hints --legacy-quota-test'
P=research/experiments/single-core-benchmark/engine/target/release/blackwood_bt
BASE=base/$P; PRE=c8af01491/$P; HEAD=head/$P
rm -rf f1 && mkdir -p f1
for s in 101 102 103 104 108; do
  $BASE $NC --restarts 1 --seed $s --nodecap 100000000 --emit-dir f1/b-nc-$s > /dev/null 2> f1/base-nc-$s.txt
  $HEAD $NC --restarts 1 --seed $s --nodecap 100000000 $L --emit-dir f1/h-nc-$s > /dev/null 2> f1/head-nc-$s.txt
done
for s in 501 502 503 504 505 506; do
  $BASE $OF --restarts 1 --seed $s --nodecap 100000000 --emit-dir f1/b-of-$s > /dev/null 2> f1/base-of-$s.txt
  $HEAD $OF --restarts 1 --seed $s --nodecap 100000000 $L --emit-dir f1/h-of-$s > /dev/null 2> f1/head-of-$s.txt
done
for s in 1 2 3 4; do
  $PRE $CE --restarts 1 --seed $s --nodecap 10000000 --hist --emit-dir f1/p-ce-$s > /dev/null 2> f1/pre-ce-$s.txt
  $HEAD $CE --restarts 1 --seed $s --nodecap 10000000 --hist --emit-dir f1/h-ce-$s > /dev/null 2> f1/post-ce-$s.txt
done
# keep the emitted urls of the legacy pairs for comparison
for f in f1/b-* f1/h-*; do b=${f#f1/}; [ -f $f/best_so_far.json ] && grep -o '"url"[^,]*' $f/best_so_far.json > f1/url-$b.txt; done
rm -rf f1/b-* f1/h-* f1/p-*
echo done
