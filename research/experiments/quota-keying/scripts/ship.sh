#!/bin/sh
# Ship the committed tree (this experiment, the Track 10 engine for the gate,
# the prereg guard) to HOST under ~/work/t13 and build both binaries.
# Usage: scripts/ship.sh HOST [DIR]   (DIR under ~/work, default t13; part 2 used t13p2)
set -e
h=$1; d=${2:-t13}
SSH="ssh -F $SSH_CONFIG -o BatchMode=yes"
root=$(git rev-parse --show-toplevel)
cd "$root"
git archive HEAD research/experiments/quota-keying research/experiments/real-puzzle-profile/engine research/experiments/common \
  | $SSH "$h" "mkdir -p ~/work/$d && tar -x -C ~/work/$d"
$SSH "$h" "cd ~/work/$d/research/experiments"' && for d in quota-keying/engine real-puzzle-profile/engine; do (cd $d && nix shell nixpkgs#cargo nixpkgs#rustc nixpkgs#gcc -c cargo build --release 2>&1 | tail -1); done; ls -la quota-keying/engine/target/release/qk real-puzzle-profile/engine/target/release/rpp'
