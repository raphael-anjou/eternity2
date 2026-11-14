#!/bin/sh
# Fetch finished rows and guard logs from the remote hosts (never ships results/ out).
cd "$(dirname "$0")/.."
E="ssh -F $SSH_CONFIG -o BatchMode=yes"
for h in x86 m1; do
  rsync -a --exclude '*.part' -e "$E" "${h}:work/t15/research/experiments/five-clue-quota/results/raw/" results/raw/
  rsync -a -e "$E" "${h}:work/t15/research/experiments/five-clue-quota/results/run-log.jsonl" "results/run-log-${h}.jsonl" 2>/dev/null || true
done
ls results/raw | grep -vc '\.part$'
