#!/bin/sh
# Fetch finished rows and the guard logs from the remote machines
# (part 1 under ~/work/t13, part 2 under ~/work/t13p2).
cd "$(dirname "$0")/.."
E="ssh -F $SSH_CONFIG -o BatchMode=yes"
for h in x86 m1; do
  for d in t13 t13p2; do
    sfx=$h; [ $d = t13p2 ] && sfx=$h-p2
    rsync -a --exclude '*.part' -e "$E" "${h}:work/$d/research/experiments/quota-keying/results/raw/" results/raw/
    rsync -a -e "$E" "${h}:work/$d/research/experiments/quota-keying/results/run-log.jsonl" "results/run-log-${sfx}.jsonl" 2>/dev/null || true
  done
done
ls results/raw | grep -vc '\.part$'
