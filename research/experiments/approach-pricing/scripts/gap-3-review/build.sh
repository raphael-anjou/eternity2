#!/bin/sh
# Gap 3 review: build r3_shape in a work folder. Usage: build.sh <work folder>
# Exports Track 10's tree.rs and puzzle.rs unchanged from commit dc71ad56.
set -e
W="$1"
HERE="$(cd "$(dirname "$0")" && pwd)"
REPO=.
mkdir -p "$W/src"
for f in tree puzzle; do
  git -C "$REPO" show "dc71ad56:research/experiments/real-puzzle-profile/engine/src/$f.rs" > "$W/src/$f.rs"
done
cp "$HERE/r3_shape.rs" "$W/src/main.rs"
rustc --edition 2021 -A warnings -C opt-level=3 -C panic=abort -o "$W/r3_shape" "$W/src/main.rs"
