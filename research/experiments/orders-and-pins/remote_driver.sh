#!/bin/sh
# orders-and-pins, remote stages on m1 (amendment A7). Detached driver:
# a-official (with the stability ladder), a-null, b2. A failed step does not
# stop later ones. Progress in PROGRESS.txt.
cd "$(dirname "$0")"
P=PROGRESS.txt
echo "driver start $(date '+%Y-%m-%d %H:%M:%S %Z') tree=ba0b77b1" >> $P
for st in a-official a-null b2; do
  now=$(date +%H%M)
  if [ "$now" -ge 1250 ]; then J=7; else J=4; fi
  s=$(date '+%H:%M:%S')
  JOBS=$J ./run.sh "$st" 2>> "results/drive-$st.log"
  code=$?
  echo "step $st jobs=$J start $s end $(date '+%H:%M:%S') exit $code" >> $P
done
echo "driver end $(date '+%Y-%m-%d %H:%M:%S %Z')" >> $P
