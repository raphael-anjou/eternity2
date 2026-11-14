#!/bin/sh
# Fetch finished rows and guard logs from the remote hosts.
cd "$(dirname "$0")/.."
E="ssh -F $SSH_CONFIG -o BatchMode=yes"
for h in x86 m1; do
  rsync -a --exclude '*.part' -e "$E" "${h}:work/t14/research/experiments/five-clue-keying/results/raw/" results/raw/
  rsync -a -e "$E" "${h}:work/t14/research/experiments/five-clue-keying/results/run-log.jsonl" "results/run-log-${h}.jsonl" 2>/dev/null || true
done
ls results/raw | grep -vc '\.part$'
