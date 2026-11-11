#!/usr/bin/env bash
# Launch the night shift on macOS. Run from the eternity2 checkout root:
#   scripts/night-shift.sh 07:00
set -euo pipefail
END="${1:-07:00}"
git fetch origin
if [ -n "$(git status --porcelain)" ]; then echo "working tree not clean"; exit 1; fi
command -v tmux >/dev/null || { echo "brew install tmux"; exit 1; }
tmux new-session -d -s night \
  "caffeinate -dimsu claude --model fable --permission-mode auto --remote-control --settings .claude/settings.night.json"
echo "Attach with: tmux attach -t night"
echo "Then type:   /goal Follow .claude/commands/night-shift.md for research/program/NIGHT_BRIEF.md until research/program/night/<date>/REPORT.md is pushed with one judged line per track and IDEAS.md exists, or the local time is $END. Start each turn with the local time and the tracks still running."
