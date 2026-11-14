#!/bin/sh
# Ship the committed tree (this experiment, Track 13's experiment for the gate
# rows, the prereg guard) to HOST under ~/work/t14 and build fck there.
# results/ is excluded: shipping it overwrote the remote guard log on the night (README).
# Usage: scripts/ship.sh HOST
set -e
h=$1
SSH="ssh -F $SSH_CONFIG -o BatchMode=yes"
cd "$(git rev-parse --show-toplevel)"
git archive HEAD research/experiments/five-clue-keying ':(exclude)research/experiments/five-clue-keying/results' research/experiments/quota-keying/results/raw/c5-P-bwf-s101.jsonl research/experiments/common \
  | $SSH "$h" "mkdir -p ~/work/t14 && tar -x -C ~/work/t14"
$SSH "$h" 'cd ~/work/t14/research/experiments/five-clue-keying/engine && nix shell nixpkgs#cargo nixpkgs#rustc nixpkgs#gcc -c cargo build --release 2>&1 | tail -1; ls -la target/release/fck'
