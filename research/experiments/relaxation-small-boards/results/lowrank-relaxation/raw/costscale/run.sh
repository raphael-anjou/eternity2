#!/bin/sh
# TOY. One evaluation-cost measurement per board, one at a time, r = 32 and 64.
cd ./lowrank
export OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 MKL_NUM_THREADS=1
for b in "5 3 4 0" "6 4 5 0" "7 4 7 0" "8 4 9 0"; do for r in 32 64; do
  echo "start $b $r $(date +%T)" >> raw/costscale/PROGRESS.txt
  timeout 1800 nice -n 10 nix shell nixpkgs#uv -c uv run --with numpy --with scipy python -I -B costscale.py $b $r >> raw/costscale/cost.jsonl 2>> raw/costscale/err.txt
  echo "end $b $r exit $? $(date +%T)" >> raw/costscale/PROGRESS.txt
done; done
echo "ALL DONE $(date +%T)" >> raw/costscale/PROGRESS.txt
