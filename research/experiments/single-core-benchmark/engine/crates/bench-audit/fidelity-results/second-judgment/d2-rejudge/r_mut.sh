B=$HOME/work/d2-rejudge
mkdir -p $B/mut
tar -xf $B/head.tar -C $B/mut
F=$B/mut/research/experiments/single-core-benchmark/engine/crates/bench-audit/src/bin/blackwood_bt.rs
# revert the legal order to the port's old one (privileged first, then fewer breaks)
sed -i '' 's/cands.sort_by(|a, b| a.2.cmp(\&b.2).then(hc(b).cmp(\&hc(a))));/cands.sort_by(|a, b| hc(b).cmp(\&hc(a)).then(a.2.cmp(\&b.2)));/' $F
grep -n "cands.sort_by" $F
cd $B/mut/research/experiments/single-core-benchmark/engine
nix shell nixpkgs#cargo nixpkgs#rustc -c cargo test --release -j 3 -p eternity2-bench-audit --bin blackwood_bt > $B/mut-test.log 2>&1
echo "mut test rc=$?"
grep -E "^test |test result" $B/mut-test.log
echo MUT_DONE
