#!/bin/sh
# Ship the committed experiment (results excluded) plus its references, then build.
# Usage: scripts/ship.sh HOST
set -e
h=$1
SSH="ssh -F $SSH_CONFIG -o BatchMode=yes"
cd "$(git rev-parse --show-toplevel)"
git archive HEAD research/experiments/five-clue-quota ':(exclude)research/experiments/five-clue-quota/results' research/experiments/five-clue-keying/results/raw/conf-5-7-20-s401.jsonl research/experiments/common \
  | $SSH "$h" "mkdir -p ~/work/t15 && tar -x -C ~/work/t15"
$SSH "$h" 'cd ~/work/t15/research/experiments/five-clue-quota/engine && nix shell nixpkgs#cargo nixpkgs#rustc nixpkgs#gcc -c cargo build --release 2>&1 | tail -1; ls -la target/release/fcq'
