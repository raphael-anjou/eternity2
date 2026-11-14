#!/bin/sh
# Development sync to x86 (working tree, results excluded), then build.
# The test stage ships committed code with git archive instead (run.sh ship).
set -e
W=.
H=x86
SSHC="ssh -F $SSH_CONFIG -o BatchMode=yes"
$SSHC $H 'mkdir -p ~/work/d1/research/experiments'
rsync -a --delete --exclude target --exclude results --exclude work --exclude "*.log" -e "$SSHC" $W/research/experiments/frame-cores ${H}:work/d1/research/experiments/
rsync -a --exclude target -e "$SSHC" $W/research/experiments/common ${H}:work/d1/research/experiments/
$SSHC $H 'mkdir -p ~/work/d1/research/experiments/single-core-benchmark/engine/data/puzzles'
rsync -a -e "$SSHC" $W/research/experiments/single-core-benchmark/engine/data/puzzles/size_16_official_eternity.csv ${H}:work/d1/research/experiments/single-core-benchmark/engine/data/puzzles/
$SSHC $H 'cd ~/work/d1/research/experiments/frame-cores/engine && nix shell nixpkgs#cargo nixpkgs#rustc nixpkgs#gcc -c cargo build --release 2>&1 | grep -E "^(warning: unused|error|Finished)|-->" | head -30'
