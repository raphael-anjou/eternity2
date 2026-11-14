set -e
B=~/work/d2-rejudge
cd $B
mkdir -p head
tar -xf head.tar -C head
echo "srcsums:"
shasum ~/work/d2-judge/new/research/experiments/single-core-benchmark/engine/crates/bench-audit/src/bin/blackwood_bt.rs ~/work/d2-judge/old/research/experiments/single-core-benchmark/engine/crates/bench-audit/src/bin/blackwood_bt.rs
ls -la ~/work/d2-judge/new/research/experiments/single-core-benchmark/engine/target/release/blackwood_bt ~/work/d2-judge/old/research/experiments/single-core-benchmark/engine/target/release/blackwood_bt
cd $B/head/research/experiments/single-core-benchmark/engine
nix shell nixpkgs#cargo nixpkgs#rustc -c cargo build --release -j 7 -p eternity2-bench-audit --bin blackwood_bt > $B/build.log 2>&1
echo "build rc=$?"
nix shell nixpkgs#cargo nixpkgs#rustc -c cargo test --release -j 7 -p eternity2-bench-audit --bin blackwood_bt > $B/test.log 2>&1 || echo "test rc nonzero"
tail -30 $B/test.log
echo BUILD_DONE
