#!/bin/sh
# five-clue-quota: build, then run one preregistered stage on this machine through the prereg guard.
# Usage: five-clue-quota/run.sh build|gate|calib|test [P] [HOST]
#   test/calib run HOST's job list here (the night split them across x86, m1, local with scripts/launch.sh).
set -e
cd "$(dirname "$0")"
st=${1:-build}; p=${2:-4}; h=${3:-local}
(cd engine && cargo build --release)
mkdir -p jobs
[ "$st" = build ] && exit 0
python3 -I scripts/jobs.py "$st" "$h" > "jobs/$st-$h.tsv"
sh scripts/run_jobs.sh "jobs/$st-$h.tsv" "$p"
[ "$st" = gate ] && python3 -I scripts/check_gates.py
exit 0
