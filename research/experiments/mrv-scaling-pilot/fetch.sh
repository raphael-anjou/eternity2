#!/usr/bin/env bash
# Fetch remote rows, heartbeats and check files into results/remote/<machine>/
# every 20 min and commit them (PREREG_AMENDMENTS.md A1). Usage: fetch.sh [once]
cd "$(dirname "$0")" || exit 1
SSH="ssh -F $SSH_CONFIG -o BatchMode=yes -o ConnectTimeout=20"
REMOTE_DIR=work/main/research/experiments/mrv-scaling-pilot/results/
fetch() {
  for m in x86 m1; do
    mkdir -p "results/remote/$m"
    rsync -a -e "$SSH" --exclude 'raw/runs/*.jsonl' "$m:$REMOTE_DIR" "results/remote/$m/" \
      || echo "$(date '+%Y-%m-%dT%H:%M:%S') fetch from $m failed" >>results/fetch-errors.log
  done
  git add results >/dev/null 2>&1
  git commit -q -m "research(mrv-scaling-pilot): fetch remote results $(date '+%H:%M')" -- results >/dev/null 2>&1 || true
}
if [ "${1:-}" = once ]; then fetch; exit 0; fi
while true; do fetch; sleep 1200; done
