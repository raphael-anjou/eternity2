#!/bin/sh
# quota-keying: build, then run one preregistered stage through the prereg guard.
# Usage: quota-keying/run.sh build|gate|main|m3scan|m3 [P] [SEEDS]
#   m3 needs SEEDS: the admitted openings (comma list) printed by the analysis.
# Then: python3 -I quota-keying/scripts/check_gates.py; python3 -I quota-keying/scripts/analyze.py
set -e
cd "$(dirname "$0")"
st=${1:-build}; p=${2:-4}
(cd engine && cargo build --release)
(cd ../real-puzzle-profile/engine && cargo build --release)
python3 -I scripts/make_noclue.py
mkdir -p jobs
case $st in
  build) exit 0 ;;
  m3) python3 -I scripts/jobs.py m3 "$3" > jobs/m3.tsv ;;
  m3scan) python3 -I scripts/jobs.py m3scan | head -40 > jobs/m3scan-5-20-7.tsv; st=m3scan-5-20-7 ;;
  *) python3 -I scripts/jobs.py "$st" > "jobs/$st.tsv" ;;
esac
sh scripts/run_jobs.sh "jobs/$st.tsv" "$p"
