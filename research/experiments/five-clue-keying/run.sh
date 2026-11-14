#!/bin/sh
# five-clue-keying: build, then run one preregistered stage on this machine through the prereg guard.
# Usage: five-clue-keying/run.sh build|gate|screen|confirm [P]
#   screen/confirm run the whole job list here (the night split it across hosts with scripts/launch.sh).
set -e
cd "$(dirname "$0")"
st=${1:-build}; p=${2:-4}
(cd engine && cargo build --release)
mkdir -p jobs
case $st in
  build) exit 0 ;;
  gate) python3 -I scripts/jobs.py gate > jobs/gate.tsv ;;
  *) for h in x86 m1 local; do python3 -I scripts/jobs.py "$st" $h; done > "jobs/$st-all.tsv"; st=$st-all ;;
esac
sh scripts/run_jobs.sh "jobs/$st.tsv" "$p"
