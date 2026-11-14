#!/bin/sh
# TOY BOARDS ONLY. Resumes the three 6x6 leak runs (6, 4, 5, seeds 0 1 2) from their checkpoints in
# runs/, detached, nice 15. THREADS (default 1) sets every BLAS thread count.
# Usage on x86, from ./convex6/resume/: THREADS=1 sh launch.sh
cd "$(dirname "$0")" || exit 1
T=${THREADS:-1}
export OMP_NUM_THREADS=$T OPENBLAS_NUM_THREADS=$T VECLIB_MAXIMUM_THREADS=$T MKL_NUM_THREADS=$T
for s in 0 1 2; do
  setsid nohup nice -n 15 nix shell nixpkgs#uv -c uv run --with numpy --with scipy python -I admm.py 6 4 5 $s leak 300 864000 runs \
    >> "runs/stdout_leak_6_4_5_$s.txt" 2>&1 < /dev/null &
  echo "started seed $s threads $T $(date '+%F %T')"
done
